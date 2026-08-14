use soroban_sdk::{contracttype, Address, Env};

use crate::types::AssetInfo;

#[contracttype]
enum DataKey {
    Admin,
    Asset(Address),
}

pub(crate) fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub(crate) fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&DataKey::Admin)
}

pub(crate) fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub(crate) fn has_asset(env: &Env, asset_id: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Asset(asset_id.clone()))
}

pub(crate) fn get_asset(env: &Env, asset_id: &Address) -> Option<AssetInfo> {
    env.storage()
        .persistent()
        .get(&DataKey::Asset(asset_id.clone()))
}

pub(crate) fn set_asset(env: &Env, asset_id: &Address, info: &AssetInfo) {
    env.storage()
        .persistent()
        .set(&DataKey::Asset(asset_id.clone()), info);
}
