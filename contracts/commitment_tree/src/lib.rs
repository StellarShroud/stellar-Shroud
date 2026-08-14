#![no_std]

//! Append-only incremental Merkle tree of shielded-note commitments.
//!
//! This stores commitment roots and lets callers prove a note was inserted
//! by supplying a known root. It does **not** verify Merkle *membership*
//! paths on-chain — that belongs to the ZK circuit (Phase 2). Here we only
//! maintain the tree state the circuit will eventually prove against.

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, BytesN, Env, Vec};

/// Fixed tree depth. 2^20 ≈ 1,048,576 leaves — ample for an MVP shielded pool.
const DEPTH: u32 = 20;
/// How many historical roots stay valid for spend proofs, so a proof built
/// against a slightly stale root (e.g. due to a race with another deposit)
/// still verifies.
const ROOT_HISTORY_SIZE: u32 = 30;

#[contracttype]
enum DataKey {
    Admin,
    Zeros,
    FilledSubtree(u32),
    NextIndex,
    RootHistory(u32),
    CurrentRootIndex,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    TreeFull = 3,
}

#[contract]
pub struct CommitmentTree;

#[contractimpl]
impl CommitmentTree {
    /// `admin` should be the `shroud_pool` contract address — the only
    /// caller authorized to insert new commitments.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();

        // Precompute the "empty subtree" hash at every level, starting from
        // an all-zero leaf, so inserts can fill in blanks without hashing
        // the whole empty tree each time.
        let mut zeros: Vec<BytesN<32>> = Vec::new(&env);
        let mut current = BytesN::from_array(&env, &[0u8; 32]);
        zeros.push_back(current.clone());
        for _ in 0..DEPTH {
            current = Self::hash_pair(&env, &current, &current);
            zeros.push_back(current.clone());
        }
        let empty_root = zeros.get(DEPTH).unwrap();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Zeros, &zeros);
        env.storage().instance().set(&DataKey::NextIndex, &0u32);
        env.storage()
            .instance()
            .set(&DataKey::CurrentRootIndex, &0u32);
        env.storage()
            .persistent()
            .set(&DataKey::RootHistory(0), &empty_root);
        Ok(())
    }

    /// Inserts a new leaf (commitment) and returns the resulting root.
    pub fn insert(env: Env, leaf: BytesN<32>) -> Result<BytesN<32>, Error> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();

        let zeros: Vec<BytesN<32>> = env.storage().instance().get(&DataKey::Zeros).unwrap();
        let mut index: u32 = env
            .storage()
            .instance()
            .get(&DataKey::NextIndex)
            .unwrap_or(0);

        if index >= (1u32 << DEPTH) {
            return Err(Error::TreeFull);
        }

        let mut current = leaf;
        let original_index = index;
        for level in 0..DEPTH {
            if index & 1 == 0 {
                env.storage()
                    .persistent()
                    .set(&DataKey::FilledSubtree(level), &current);
                let zero = zeros.get(level).unwrap();
                current = Self::hash_pair(&env, &current, &zero);
            } else {
                let left: BytesN<32> = env
                    .storage()
                    .persistent()
                    .get(&DataKey::FilledSubtree(level))
                    .unwrap();
                current = Self::hash_pair(&env, &left, &current);
            }
            index >>= 1;
        }

        let new_root = current;
        let next_root_index =
            (env
                .storage()
                .instance()
                .get::<_, u32>(&DataKey::CurrentRootIndex)
                .unwrap_or(0)
                + 1)
                % ROOT_HISTORY_SIZE;
        env.storage()
            .persistent()
            .set(&DataKey::RootHistory(next_root_index), &new_root);
        env.storage()
            .instance()
            .set(&DataKey::CurrentRootIndex, &next_root_index);
        env.storage()
            .instance()
            .set(&DataKey::NextIndex, &(original_index + 1));

        Ok(new_root)
    }

    pub fn current_root(env: Env) -> BytesN<32> {
        let idx: u32 = env
            .storage()
            .instance()
            .get(&DataKey::CurrentRootIndex)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .get(&DataKey::RootHistory(idx))
            .unwrap()
    }

    /// Whether `root` is (still) recognized as a valid recent tree state.
    pub fn is_known_root(env: Env, root: BytesN<32>) -> bool {
        let current_idx: u32 = env
            .storage()
            .instance()
            .get(&DataKey::CurrentRootIndex)
            .unwrap_or(0);
        let mut i = current_idx;
        for _ in 0..ROOT_HISTORY_SIZE {
            if let Some(stored) = env
                .storage()
                .persistent()
                .get::<_, BytesN<32>>(&DataKey::RootHistory(i))
            {
                if stored == root {
                    return true;
                }
            }
            i = if i == 0 { ROOT_HISTORY_SIZE - 1 } else { i - 1 };
        }
        false
    }

    fn hash_pair(env: &Env, left: &BytesN<32>, right: &BytesN<32>) -> BytesN<32> {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(&left.to_array());
        bytes[32..].copy_from_slice(&right.to_array());
        env.crypto()
            .sha256(&soroban_sdk::Bytes::from_array(env, &bytes))
            .into()
    }

    fn require_admin(env: &Env) -> Result<Address, Error> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }
}

#[cfg(test)]
mod test;
