//! Confirms the off-chain `MerkleTree` in this crate computes the exact
//! same root as the on-chain `commitment_tree` Soroban contract for the
//! same sequence of inserted leaves. If this ever fails, a wallet
//! building a withdrawal proof against its locally-tracked root would be
//! rejected on-chain — so this is load-bearing, not just documentation.

use commitment_tree::{CommitmentTree, CommitmentTreeClient};
use shroud_crypto::merkle::MerkleTree;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, BytesN, Env};

#[test]
fn roots_agree_after_several_inserts() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract_id = env.register_contract(None, CommitmentTree);
    let onchain = CommitmentTreeClient::new(&env, &contract_id);
    onchain.initialize(&admin);

    let mut offchain = MerkleTree::new();

    for i in 0u8..8 {
        let leaf = [i; 32];
        onchain.insert(&BytesN::from_array(&env, &leaf));
        offchain.insert(leaf);
    }

    assert_eq!(onchain.current_root().to_array(), offchain.root());
}
