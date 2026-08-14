#![cfg(test)]

use super::*;
use asset_registry::{AssetRegistry, AssetRegistryClient};
use commitment_tree::{CommitmentTree, CommitmentTreeClient};
use nullifier_registry::{NullifierRegistry, NullifierRegistryClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{StellarAssetClient, TokenClient};
use soroban_sdk::Env;

struct TestSetup<'a> {
    env: Env,
    pool: ShroudPoolClient<'a>,
    asset_registry: AssetRegistryClient<'a>,
    token: TokenClient<'a>,
    token_admin: StellarAssetClient<'a>,
    asset_id: Address,
    anchor: Address,
    depositor: Address,
}

fn setup<'a>() -> TestSetup<'a> {
    let env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();

    let admin = Address::generate(&env);
    let anchor = Address::generate(&env);
    let depositor = Address::generate(&env);

    let asset_registry_id = env.register_contract(None, AssetRegistry);
    let nullifier_registry_id = env.register_contract(None, NullifierRegistry);
    let commitment_tree_id = env.register_contract(None, CommitmentTree);
    let pool_id = env.register_contract(None, ShroudPool);

    let asset_registry = AssetRegistryClient::new(&env, &asset_registry_id);
    asset_registry.initialize(&admin);

    let nullifier_registry = NullifierRegistryClient::new(&env, &nullifier_registry_id);
    nullifier_registry.initialize(&pool_id);

    let commitment_tree = CommitmentTreeClient::new(&env, &commitment_tree_id);
    commitment_tree.initialize(&pool_id);

    let pool = ShroudPoolClient::new(&env, &pool_id);
    pool.initialize(
        &admin,
        &asset_registry_id,
        &nullifier_registry_id,
        &commitment_tree_id,
    );

    let token_admin_addr = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(token_admin_addr.clone());
    let asset_id = sac.address();
    let token = TokenClient::new(&env, &asset_id);
    let token_admin = StellarAssetClient::new(&env, &asset_id);
    token_admin.mint(&depositor, &1_000_000);

    let code = Symbol::new(&env, "USDC");
    asset_registry.register_asset(&asset_id, &anchor, &code);

    TestSetup {
        env,
        pool,
        asset_registry,
        token,
        token_admin,
        asset_id,
        anchor,
        depositor,
    }
}

#[test]
fn deposit_transfers_tokens_and_inserts_commitment() {
    let s = setup();

    let commitment = BytesN::from_array(&s.env, &[1u8; 32]);
    let new_root = s
        .pool
        .deposit(&s.depositor, &s.asset_id, &1_000, &commitment);

    assert_eq!(s.token.balance(&s.depositor), 1_000_000 - 1_000);
    assert_eq!(s.token.balance(&s.pool.address), 1_000);
    assert_ne!(new_root, BytesN::from_array(&s.env, &[0u8; 32]));

    let _ = (&s.asset_registry, &s.token_admin, &s.anchor);
}

#[test]
fn deposit_rejects_unsupported_asset() {
    let s = setup();
    let admin = Address::generate(&s.env);
    let other_sac = s.env.register_stellar_asset_contract_v2(admin);
    let other_asset = other_sac.address();

    let commitment = BytesN::from_array(&s.env, &[2u8; 32]);
    let result =
        s.pool
            .try_deposit(&s.depositor, &other_asset, &1_000, &commitment);
    assert_eq!(result, Err(Ok(Error::UnsupportedAsset)));
}

#[test]
fn withdraw_full_roundtrip() {
    let s = setup();

    let commitment = BytesN::from_array(&s.env, &[3u8; 32]);
    let root = s
        .pool
        .deposit(&s.depositor, &s.asset_id, &5_000, &commitment);

    let recipient = Address::generate(&s.env);
    let nullifier = BytesN::from_array(&s.env, &[4u8; 32]);
    let proof = ShroudProof { valid: true };

    s.pool.withdraw(
        &recipient,
        &s.asset_id,
        &5_000,
        &root,
        &nullifier,
        &proof,
    );

    assert_eq!(s.token.balance(&recipient), 5_000);
    assert_eq!(s.token.balance(&s.pool.address), 0);
}

#[test]
fn withdraw_rejects_invalid_proof() {
    let s = setup();

    let commitment = BytesN::from_array(&s.env, &[5u8; 32]);
    let root = s
        .pool
        .deposit(&s.depositor, &s.asset_id, &1_000, &commitment);

    let recipient = Address::generate(&s.env);
    let nullifier = BytesN::from_array(&s.env, &[6u8; 32]);
    let proof = ShroudProof { valid: false };

    let result = s.pool.try_withdraw(
        &recipient,
        &s.asset_id,
        &1_000,
        &root,
        &nullifier,
        &proof,
    );
    assert_eq!(result, Err(Ok(Error::InvalidProof)));
}

#[test]
fn withdraw_rejects_double_spend() {
    let s = setup();

    let commitment = BytesN::from_array(&s.env, &[7u8; 32]);
    let root = s
        .pool
        .deposit(&s.depositor, &s.asset_id, &2_000, &commitment);

    let recipient = Address::generate(&s.env);
    let nullifier = BytesN::from_array(&s.env, &[8u8; 32]);
    let proof = ShroudProof { valid: true };

    s.pool.withdraw(
        &recipient,
        &s.asset_id,
        &1_000,
        &root,
        &nullifier,
        &proof,
    );

    let result = s.pool.try_withdraw(
        &recipient,
        &s.asset_id,
        &1_000,
        &root,
        &nullifier,
        &proof,
    );
    assert_eq!(result, Err(Ok(Error::AlreadySpent)));
}

#[test]
fn withdraw_rejects_unknown_root() {
    let s = setup();

    let recipient = Address::generate(&s.env);
    let nullifier = BytesN::from_array(&s.env, &[9u8; 32]);
    let bogus_root = BytesN::from_array(&s.env, &[0xFFu8; 32]);
    let proof = ShroudProof { valid: true };

    let result = s.pool.try_withdraw(
        &recipient,
        &s.asset_id,
        &1_000,
        &bogus_root,
        &nullifier,
        &proof,
    );
    assert_eq!(result, Err(Ok(Error::UnknownRoot)));
}

#[test]
fn shielded_transfer_moves_note_without_touching_token_balance() {
    let s = setup();

    let commitment = BytesN::from_array(&s.env, &[10u8; 32]);
    let root = s
        .pool
        .deposit(&s.depositor, &s.asset_id, &3_000, &commitment);

    let nullifier = BytesN::from_array(&s.env, &[11u8; 32]);
    let output_commitment = BytesN::from_array(&s.env, &[12u8; 32]);
    let proof = ShroudProof { valid: true };

    let new_root = s
        .pool
        .transfer(&root, &nullifier, &output_commitment, &proof);

    assert_ne!(new_root, root);
    assert_eq!(s.token.balance(&s.pool.address), 3_000);
}
