use soroban_sdk::{contracttype, Address, Env};

use crate::errors::Error;

#[contracttype]
enum DataKey {
    Admin,
    AssetRegistry,
    NullifierRegistry,
    CommitmentTree,
    Paused,
}

pub(crate) struct Registries {
    pub(crate) asset_registry: Address,
    pub(crate) nullifier_registry: Address,
    pub(crate) commitment_tree: Address,
}

pub(crate) fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub(crate) fn get_admin(env: &Env) -> Result<Address, Error> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotInitialized)
}

pub(crate) fn is_paused(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&DataKey::Paused)
        .unwrap_or(false)
}

pub(crate) fn set_paused(env: &Env, paused: bool) {
    env.storage().instance().set(&DataKey::Paused, &paused);
}

pub(crate) fn set_config(env: &Env, admin: &Address, registries: &Registries) {
    env.storage().instance().set(&DataKey::Admin, admin);
    env.storage()
        .instance()
        .set(&DataKey::AssetRegistry, &registries.asset_registry);
    env.storage()
        .instance()
        .set(&DataKey::NullifierRegistry, &registries.nullifier_registry);
    env.storage()
        .instance()
        .set(&DataKey::CommitmentTree, &registries.commitment_tree);
}

pub(crate) fn get_registries(env: &Env) -> Result<Registries, Error> {
    let asset_registry = env
        .storage()
        .instance()
        .get(&DataKey::AssetRegistry)
        .ok_or(Error::NotInitialized)?;
    let nullifier_registry = env
        .storage()
        .instance()
        .get(&DataKey::NullifierRegistry)
        .ok_or(Error::NotInitialized)?;
    let commitment_tree = env
        .storage()
        .instance()
        .get(&DataKey::CommitmentTree)
        .ok_or(Error::NotInitialized)?;
    Ok(Registries {
        asset_registry,
        nullifier_registry,
        commitment_tree,
    })
}
