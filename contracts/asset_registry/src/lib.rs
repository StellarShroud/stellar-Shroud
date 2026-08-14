#![no_std]

//! Tracks which Stellar/anchor-issued assets are approved to enter the
//! shielded pool. Only `shroud_pool` should treat an asset as spendable
//! after checking `is_supported` here.

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AssetStatus {
    Active,
    Suspended,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct AssetInfo {
    pub anchor: Address,
    pub code: Symbol,
    pub status: AssetStatus,
}

#[contracttype]
enum DataKey {
    Admin,
    Asset(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    AssetNotFound = 3,
    AssetAlreadyRegistered = 4,
}

#[contract]
pub struct AssetRegistry;

#[contractimpl]
impl AssetRegistry {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    /// Register a new supported asset. Only the registry admin may call this.
    pub fn register_asset(
        env: Env,
        asset_id: Address,
        anchor: Address,
        code: Symbol,
    ) -> Result<(), Error> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();

        let key = DataKey::Asset(asset_id);
        if env.storage().persistent().has(&key) {
            return Err(Error::AssetAlreadyRegistered);
        }
        let info = AssetInfo {
            anchor,
            code,
            status: AssetStatus::Active,
        };
        env.storage().persistent().set(&key, &info);
        Ok(())
    }

    /// Suspend or reactivate a previously registered asset.
    pub fn set_status(env: Env, asset_id: Address, status: AssetStatus) -> Result<(), Error> {
        let admin = Self::require_admin(&env)?;
        admin.require_auth();

        let key = DataKey::Asset(asset_id);
        let mut info: AssetInfo = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::AssetNotFound)?;
        info.status = status;
        env.storage().persistent().set(&key, &info);
        Ok(())
    }

    pub fn get_asset(env: Env, asset_id: Address) -> Result<AssetInfo, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Asset(asset_id))
            .ok_or(Error::AssetNotFound)
    }

    pub fn is_supported(env: Env, asset_id: Address) -> bool {
        match env
            .storage()
            .persistent()
            .get::<_, AssetInfo>(&DataKey::Asset(asset_id))
        {
            Some(info) => info.status == AssetStatus::Active,
            None => false,
        }
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
