//! Off-chain mirror of the on-chain `commitment_tree` contract's
//! incremental Merkle tree, plus the membership-proof generation and
//! verification the contract deliberately does *not* do on-chain (that
//! belongs to the ZK circuit in Phase 2 — see PROJECT.md's "Proof
//! Circuit" section, constraint 2: "Merkle path is valid").
//!
//! A wallet/prover reconstructs this tree from the `commitment_tree`
//! contract's `insert` events (the leaves are public; what they represent
//! is not) and uses `proof()` to build the private witness a withdrawal
//! or transfer proof needs. `hash_pair` here must match the on-chain
//! contract's hashing exactly (SHA-256 of the 64-byte concatenation) or
//! roots computed off-chain will never match on-chain roots.

use crate::{hash_pair, Hash32};

/// Same depth as `commitment_tree`'s Soroban contract. Keep these in sync.
pub const DEPTH: u32 = 20;

pub struct MerkleTree {
    /// Precomputed "empty subtree" hash at each level, `zeros[0]` being an
    /// all-zero leaf. Lets `proof()` treat a not-yet-inserted sibling as
    /// this value instead of needing the whole tree materialized.
    zeros: Vec<Hash32>,
    leaves: Vec<Hash32>,
}

pub struct MerkleProof {
    pub leaf: Hash32,
    pub index: usize,
    /// Sibling hash at each level from leaf to root, in that order.
    pub siblings: Vec<Hash32>,
    pub root: Hash32,
}

impl MerkleTree {
    pub fn new() -> Self {
        let mut zeros = Vec::with_capacity(DEPTH as usize + 1);
        let mut current = [0u8; 32];
        zeros.push(current);
        for _ in 0..DEPTH {
            current = hash_pair(&current, &current);
            zeros.push(current);
        }
        Self {
            zeros,
            leaves: Vec::new(),
        }
    }

    /// Appends a leaf, mirroring the order `commitment_tree::insert` is
    /// called on-chain. Returns the leaf's index.
    pub fn insert(&mut self, leaf: Hash32) -> usize {
        self.leaves.push(leaf);
        self.leaves.len() - 1
    }

    pub fn root(&self) -> Hash32 {
        self.level_at(DEPTH, &self.leaves)
            .first()
            .copied()
            .unwrap_or(self.zeros[DEPTH as usize])
    }

    /// Builds a membership proof for the leaf at `index`. Panics if
    /// `index` is out of range — callers should only prove leaves they
    /// know they inserted.
    pub fn proof(&self, index: usize) -> MerkleProof {
        assert!(index < self.leaves.len(), "index out of range");

        let mut level = self.leaves.clone();
        let mut idx = index;
        let mut siblings = Vec::with_capacity(DEPTH as usize);

        for depth in 0..DEPTH {
            let sibling_idx = idx ^ 1;
            let sibling = level
                .get(sibling_idx)
                .copied()
                .unwrap_or(self.zeros[depth as usize]);
            siblings.push(sibling);
            level = Self::next_level(&level, self.zeros[depth as usize]);
            idx /= 2;
        }

        MerkleProof {
            leaf: self.leaves[index],
            index,
            siblings,
            root: self.root(),
        }
    }

    fn next_level(level: &[Hash32], zero: Hash32) -> Vec<Hash32> {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut i = 0;
        while i < level.len() {
            let left = level[i];
            let right = level.get(i + 1).copied().unwrap_or(zero);
            next.push(hash_pair(&left, &right));
            i += 2;
        }
        next
    }

    fn level_at(&self, target_depth: u32, leaves: &[Hash32]) -> Vec<Hash32> {
        let mut level = leaves.to_vec();
        for depth in 0..target_depth {
            level = Self::next_level(&level, self.zeros[depth as usize]);
        }
        level
    }
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

impl MerkleProof {
    /// Recomputes the root from `leaf` and `siblings` and checks it
    /// matches `root`. This is exactly the check a ZK circuit's "Merkle
    /// path is valid" constraint must enforce (PROJECT.md's Proof
    /// Circuit) — here it runs directly since there's no circuit yet.
    pub fn verify(&self) -> bool {
        let mut current = self.leaf;
        let mut idx = self.index;
        for sibling in &self.siblings {
            current = if idx & 1 == 0 {
                hash_pair(&current, sibling)
            } else {
                hash_pair(sibling, &current)
            };
            idx >>= 1;
        }
        current == self.root
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_tree_root_is_stable() {
        let tree = MerkleTree::new();
        assert_eq!(tree.root(), tree.zeros[DEPTH as usize]);
    }

    #[test]
    fn insert_changes_root() {
        let mut tree = MerkleTree::new();
        let empty_root = tree.root();
        tree.insert([1u8; 32]);
        assert_ne!(tree.root(), empty_root);
    }

    #[test]
    fn proof_verifies_against_current_root() {
        let mut tree = MerkleTree::new();
        let idx = tree.insert([7u8; 32]);
        tree.insert([8u8; 32]);
        tree.insert([9u8; 32]);

        let proof = tree.proof(idx);
        assert_eq!(proof.root, tree.root());
        assert!(proof.verify());
    }

    #[test]
    fn tampered_leaf_fails_verification() {
        let mut tree = MerkleTree::new();
        let idx = tree.insert([1u8; 32]);
        tree.insert([2u8; 32]);

        let mut proof = tree.proof(idx);
        proof.leaf = [0xFFu8; 32];
        assert!(!proof.verify());
    }

    #[test]
    fn tampered_sibling_fails_verification() {
        let mut tree = MerkleTree::new();
        let idx = tree.insert([1u8; 32]);
        tree.insert([2u8; 32]);
        tree.insert([3u8; 32]);

        let mut proof = tree.proof(idx);
        proof.siblings[0] = [0xFFu8; 32];
        assert!(!proof.verify());
    }

    #[test]
    fn all_inserted_leaves_prove_membership() {
        let mut tree = MerkleTree::new();
        let leaves: Vec<Hash32> = (0u8..16).map(|i| [i; 32]).collect();
        for leaf in &leaves {
            tree.insert(*leaf);
        }
        for i in 0..leaves.len() {
            assert!(tree.proof(i).verify(), "leaf {i} failed to verify");
        }
    }
}
