#![cfg(test)]

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::Env;

fn setup(env: &Env) -> (AssetRegistryClient<'_>, Address) {
    env.mock_all_auths();
    let admin = Address::generate(env);
    let contract_id = env.register_contract(None, AssetRegistry);
    let client = AssetRegistryClient::new(env, &contract_id);
    client.initialize(&admin);
    (client, admin)
}

#[test]
fn register_and_query_asset() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin) = setup(&env);

    let anchor = Address::generate(&env);
    let asset_id = Address::generate(&env);
    let code = Symbol::new(&env, "USDC");

    client.register_asset(&asset_id, &anchor, &code);
    assert!(client.is_supported(&asset_id));

    let info = client.get_asset(&asset_id);
    assert_eq!(info.anchor, anchor);
    assert_eq!(info.status, AssetStatus::Active);

    client.set_status(&asset_id, &AssetStatus::Suspended);
    assert!(!client.is_supported(&asset_id));

    let _ = admin;
}

#[test]
fn unknown_asset_is_not_supported() {
    let env = Env::default();
    let (client, _admin) = setup(&env);
    let asset_id = Address::generate(&env);
    assert!(!client.is_supported(&asset_id));
}

#[test]
fn duplicate_registration_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin) = setup(&env);

    let anchor = Address::generate(&env);
    let asset_id = Address::generate(&env);
    let code = Symbol::new(&env, "USDC");

    client.register_asset(&asset_id, &anchor, &code);
    let result = client.try_register_asset(&asset_id, &anchor, &code);
    assert_eq!(result, Err(Ok(Error::AssetAlreadyRegistered)));
}
