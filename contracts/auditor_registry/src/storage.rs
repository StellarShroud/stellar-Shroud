use soroban_sdk::{contracttype, Address, Env};

use crate::types::AuditorInfo;

#[contracttype]
enum DataKey {
    Auditor(Address),
}

pub(crate) fn has_auditor(env: &Env, auditor: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Auditor(auditor.clone()))
}

pub(crate) fn get_auditor(env: &Env, auditor: &Address) -> Option<AuditorInfo> {
    env.storage()
        .persistent()
        .get(&DataKey::Auditor(auditor.clone()))
}

pub(crate) fn set_auditor(env: &Env, auditor: &Address, info: &AuditorInfo) {
    env.storage()
        .persistent()
        .set(&DataKey::Auditor(auditor.clone()), info);
}
