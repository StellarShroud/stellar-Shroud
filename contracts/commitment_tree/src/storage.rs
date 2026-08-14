use soroban_sdk::{contracttype, Address, BytesN, Env, Vec};

/// Fixed tree depth. 2^20 ≈ 1,048,576 leaves — ample for an MVP shielded pool.
pub(crate) const DEPTH: u32 = 20;
/// How many historical roots stay valid for spend proofs, so a proof built
/// against a slightly stale root (e.g. due to a race with another deposit)
/// still verifies.
pub(crate) const ROOT_HISTORY_SIZE: u32 = 30;

#[contracttype]
enum DataKey {
    Admin,
    Zeros,
    FilledSubtree(u32),
    NextIndex,
    RootHistory(u32),
    CurrentRootIndex,
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

pub(crate) fn get_zeros(env: &Env) -> Vec<BytesN<32>> {
    env.storage().instance().get(&DataKey::Zeros).unwrap()
}

pub(crate) fn set_zeros(env: &Env, zeros: &Vec<BytesN<32>>) {
    env.storage().instance().set(&DataKey::Zeros, zeros);
}

pub(crate) fn get_filled_subtree(env: &Env, level: u32) -> BytesN<32> {
    env.storage()
        .persistent()
        .get(&DataKey::FilledSubtree(level))
        .unwrap()
}

pub(crate) fn set_filled_subtree(env: &Env, level: u32, value: &BytesN<32>) {
    env.storage()
        .persistent()
        .set(&DataKey::FilledSubtree(level), value);
}

pub(crate) fn get_next_index(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::NextIndex)
        .unwrap_or(0)
}

pub(crate) fn set_next_index(env: &Env, index: u32) {
    env.storage().instance().set(&DataKey::NextIndex, &index);
}

pub(crate) fn get_current_root_index(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::CurrentRootIndex)
        .unwrap_or(0)
}

pub(crate) fn set_current_root_index(env: &Env, index: u32) {
    env.storage()
        .instance()
        .set(&DataKey::CurrentRootIndex, &index);
}

pub(crate) fn get_root_history(env: &Env, index: u32) -> Option<BytesN<32>> {
    env.storage().persistent().get(&DataKey::RootHistory(index))
}

pub(crate) fn set_root_history(env: &Env, index: u32, root: &BytesN<32>) {
    env.storage()
        .persistent()
        .set(&DataKey::RootHistory(index), root);
}
