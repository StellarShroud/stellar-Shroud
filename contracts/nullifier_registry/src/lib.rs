#![no_std]

//! Tracks spent shielded notes. A nullifier is derived from a note's secret
//! and, once recorded here, that note can never be spent again — this is
//! the double-spend guard for the whole protocol.

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, BytesN, Env};

#[contracttype]
enum DataKey {
    Admin,
    Spent(BytesN<32>),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    AlreadySpent = 3,
}

#[contract]
pub struct NullifierRegistry;

#[contractimpl]
impl NullifierRegistry {
    /// `admin` should be the `shroud_pool` contract address — it is the
    /// only caller authorized to mark nullifiers as spent.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    pub fn is_spent(env: Env, nullifier: BytesN<32>) -> bool {
        env.storage().persistent().has(&DataKey::Spent(nullifier))
    }

    /// Marks `nullifier` as spent. Fails if it was already spent.
    pub fn spend(env: Env, nullifier: BytesN<32>) -> Result<(), Error> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        admin.require_auth();

        let key = DataKey::Spent(nullifier);
        if env.storage().persistent().has(&key) {
            return Err(Error::AlreadySpent);
        }
        env.storage().persistent().set(&key, &true);
        Ok(())
    }
}

#[cfg(test)]
mod test;
