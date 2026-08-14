#![no_std]

//! Tracks which Stellar/anchor-issued assets are approved to enter the
//! shielded pool. Only `shroud_pool` should treat an asset as spendable
//! after checking `is_supported` here.

mod errors;
mod storage;
mod types;

pub use errors::Error;
pub use types::{AssetInfo, AssetStatus};

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct AssetRegistry;

#[contractimpl]
impl AssetRegistry {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if storage::has_admin(&env) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
        Ok(())
    }

    /// Register a new supported asset. Only the registry admin may call this.
    pub fn register_asset(
        env: Env,
        asset_id: Address,
        anchor: Address,
        code: Symbol,
    ) -> Result<(), Error> {
        let admin = storage::get_admin(&env).ok_or(Error::NotInitialized)?;
        admin.require_auth();

        if storage::has_asset(&env, &asset_id) {
            return Err(Error::AssetAlreadyRegistered);
        }
        let info = AssetInfo {
            anchor,
            code,
            status: AssetStatus::Active,
        };
        storage::set_asset(&env, &asset_id, &info);
        Ok(())
    }

    /// Suspend or reactivate a previously registered asset.
    pub fn set_status(env: Env, asset_id: Address, status: AssetStatus) -> Result<(), Error> {
        let admin = storage::get_admin(&env).ok_or(Error::NotInitialized)?;
        admin.require_auth();

        let mut info = storage::get_asset(&env, &asset_id).ok_or(Error::AssetNotFound)?;
        info.status = status;
        storage::set_asset(&env, &asset_id, &info);
        Ok(())
    }

    pub fn get_asset(env: Env, asset_id: Address) -> Result<AssetInfo, Error> {
        storage::get_asset(&env, &asset_id).ok_or(Error::AssetNotFound)
    }

    pub fn is_supported(env: Env, asset_id: Address) -> bool {
        storage::get_asset(&env, &asset_id)
            .map(|info| info.status == AssetStatus::Active)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod test;
