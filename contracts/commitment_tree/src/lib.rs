#![no_std]

//! Append-only incremental Merkle tree of shielded-note commitments.
//!
//! This stores commitment roots and lets callers prove a note was inserted
//! by supplying a known root. It does **not** verify Merkle *membership*
//! paths on-chain — that belongs to the ZK circuit (Phase 2). Here we only
//! maintain the tree state the circuit will eventually prove against.

mod errors;
mod hash;
mod storage;

pub use errors::Error;

use hash::hash_pair;
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Vec};
use storage::{DEPTH, ROOT_HISTORY_SIZE};

#[contract]
pub struct CommitmentTree;

#[contractimpl]
impl CommitmentTree {
    /// `admin` should be the `shroud_pool` contract address — the only
    /// caller authorized to insert new commitments.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if storage::has_admin(&env) {
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
            current = hash_pair(&env, &current, &current);
            zeros.push_back(current.clone());
        }
        let empty_root = zeros.get(DEPTH).unwrap();

        storage::set_admin(&env, &admin);
        storage::set_zeros(&env, &zeros);
        storage::set_next_index(&env, 0);
        storage::set_current_root_index(&env, 0);
        storage::set_root_history(&env, 0, &empty_root);
        Ok(())
    }

    /// Inserts a new leaf (commitment) and returns the resulting root.
    pub fn insert(env: Env, leaf: BytesN<32>) -> Result<BytesN<32>, Error> {
        let admin = storage::get_admin(&env).ok_or(Error::NotInitialized)?;
        admin.require_auth();

        let zeros = storage::get_zeros(&env);
        let index = storage::get_next_index(&env);

        if index >= (1u32 << DEPTH) {
            return Err(Error::TreeFull);
        }

        let mut current = leaf;
        let mut cursor = index;
        for level in 0..DEPTH {
            if cursor & 1 == 0 {
                storage::set_filled_subtree(&env, level, &current);
                let zero = zeros.get(level).unwrap();
                current = hash_pair(&env, &current, &zero);
            } else {
                let left = storage::get_filled_subtree(&env, level);
                current = hash_pair(&env, &left, &current);
            }
            cursor >>= 1;
        }

        let new_root = current;
        let next_root_index = (storage::get_current_root_index(&env) + 1) % ROOT_HISTORY_SIZE;
        storage::set_root_history(&env, next_root_index, &new_root);
        storage::set_current_root_index(&env, next_root_index);
        storage::set_next_index(&env, index + 1);

        Ok(new_root)
    }

    pub fn current_root(env: Env) -> BytesN<32> {
        let idx = storage::get_current_root_index(&env);
        storage::get_root_history(&env, idx).unwrap()
    }

    /// Whether `root` is (still) recognized as a valid recent tree state.
    pub fn is_known_root(env: Env, root: BytesN<32>) -> bool {
        let current_idx = storage::get_current_root_index(&env);
        let mut i = current_idx;
        for _ in 0..ROOT_HISTORY_SIZE {
            if let Some(stored) = storage::get_root_history(&env, i) {
                if stored == root {
                    return true;
                }
            }
            i = if i == 0 { ROOT_HISTORY_SIZE - 1 } else { i - 1 };
        }
        false
    }
}

#[cfg(test)]
mod test;
