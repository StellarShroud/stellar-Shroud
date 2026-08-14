use soroban_sdk::{contracttype, Address, BytesN, Env};

#[contracttype]
enum DataKey {
    Admin,
    Spent(BytesN<32>),
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

pub(crate) fn is_spent(env: &Env, nullifier: &BytesN<32>) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Spent(nullifier.clone()))
}

pub(crate) fn mark_spent(env: &Env, nullifier: &BytesN<32>) {
    env.storage()
        .persistent()
        .set(&DataKey::Spent(nullifier.clone()), &true);
}
