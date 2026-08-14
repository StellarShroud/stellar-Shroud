#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::Env;

fn setup(env: &Env) -> (NullifierRegistryClient<'_>, Address) {
    env.mock_all_auths();
    let admin = Address::generate(env);
    let contract_id = env.register_contract(None, NullifierRegistry);
    let client = NullifierRegistryClient::new(env, &contract_id);
    client.initialize(&admin);
    (client, admin)
}

#[test]
fn spend_marks_nullifier() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup(&env);

    let nullifier = BytesN::from_array(&env, &[7u8; 32]);
    assert!(!client.is_spent(&nullifier));

    client.spend(&nullifier);
    assert!(client.is_spent(&nullifier));
}

#[test]
fn double_spend_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup(&env);

    let nullifier = BytesN::from_array(&env, &[9u8; 32]);
    client.spend(&nullifier);

    let result = client.try_spend(&nullifier);
    assert_eq!(result, Err(Ok(Error::AlreadySpent)));
}
