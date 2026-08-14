#![no_std]

//! Tracks spent shielded notes. A nullifier is derived from a note's secret
//! and, once recorded here, that note can never be spent again — this is
//! the double-spend guard for the whole protocol.

mod errors;
mod storage;

pub use errors::Error;

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

#[contract]
pub struct NullifierRegistry;

#[contractimpl]
impl NullifierRegistry {
    /// `admin` should be the `shroud_pool` contract address — it is the
    /// only caller authorized to mark nullifiers as spent.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if storage::has_admin(&env) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
        Ok(())
    }

    pub fn is_spent(env: Env, nullifier: BytesN<32>) -> bool {
        storage::is_spent(&env, &nullifier)
    }

    /// Marks `nullifier` as spent. Fails if it was already spent.
    pub fn spend(env: Env, nullifier: BytesN<32>) -> Result<(), Error> {
        let admin = storage::get_admin(&env).ok_or(Error::NotInitialized)?;
        admin.require_auth();

        if storage::is_spent(&env, &nullifier) {
            return Err(Error::AlreadySpent);
        }
        storage::mark_spent(&env, &nullifier);
        Ok(())
    }
}

#[cfg(test)]
mod test;
