#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::Env;

fn setup(env: &Env) -> (CommitmentTreeClient<'_>, Address) {
    env.mock_all_auths();
    let admin = Address::generate(env);
    let contract_id = env.register_contract(None, CommitmentTree);
    let client = CommitmentTreeClient::new(env, &contract_id);
    client.initialize(&admin);
    (client, admin)
}

#[test]
fn insert_changes_root_and_is_known() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup(&env);

    let empty_root = client.current_root();

    let leaf = BytesN::from_array(&env, &[1u8; 32]);
    let new_root = client.insert(&leaf);

    assert_ne!(new_root, empty_root);
    assert_eq!(client.current_root(), new_root);
    assert!(client.is_known_root(&new_root));
    assert!(client.is_known_root(&empty_root));
}

#[test]
fn unknown_root_is_rejected() {
    let env = Env::default();
    let (client, _admin) = setup(&env);

    let bogus_root = BytesN::from_array(&env, &[0xAB; 32]);
    assert!(!client.is_known_root(&bogus_root));
}

#[test]
fn sequential_inserts_produce_distinct_roots() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup(&env);

    let leaf1 = BytesN::from_array(&env, &[1u8; 32]);
    let leaf2 = BytesN::from_array(&env, &[2u8; 32]);

    let root1 = client.insert(&leaf1);
    let root2 = client.insert(&leaf2);

    assert_ne!(root1, root2);
    assert!(client.is_known_root(&root1));
    assert!(client.is_known_root(&root2));
}
