#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::Env;

#[test]
fn register_and_authorize() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, AuditorRegistry);
    let client = AuditorRegistryClient::new(&env, &contract_id);

    let anchor = Address::generate(&env);
    let auditor = Address::generate(&env);
    let pubkey = BytesN::from_array(&env, &[3u8; 32]);

    client.register_auditor(&anchor, &auditor, &pubkey);
    assert!(client.is_authorized(&auditor, &anchor));

    let other_anchor = Address::generate(&env);
    assert!(!client.is_authorized(&auditor, &other_anchor));
}

#[test]
fn revoke_removes_authorization() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, AuditorRegistry);
    let client = AuditorRegistryClient::new(&env, &contract_id);

    let anchor = Address::generate(&env);
    let auditor = Address::generate(&env);
    let pubkey = BytesN::from_array(&env, &[4u8; 32]);

    client.register_auditor(&anchor, &auditor, &pubkey);
    client.revoke_auditor(&anchor, &auditor);
    assert!(!client.is_authorized(&auditor, &anchor));
}

#[test]
fn unknown_auditor_not_authorized() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AuditorRegistry);
    let client = AuditorRegistryClient::new(&env, &contract_id);

    let anchor = Address::generate(&env);
    let auditor = Address::generate(&env);
    assert!(!client.is_authorized(&auditor, &anchor));
}
