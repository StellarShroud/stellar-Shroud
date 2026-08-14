#![no_std]

//! The shielded pool: deposits, private transfers, and withdrawals.
//!
//! Delegates supported-asset checks to `asset_registry`, spent-note
//! tracking to `nullifier_registry`, and commitment/root state to
//! `commitment_tree`. Real ZK proof verification is not implemented yet
//! (see `ShroudProof` below) — this crate exists to validate the on-chain
//! state machine ahead of the cryptography work in PROJECT.md Phase 2.

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, token, Address, BytesN, Env, Symbol,
};

// Cross-contract calls target already-compiled contract WASM rather than
// depending on the other crates' Rust source: pulling in another
// `#[contract]` crate as a normal dependency would link its wasm-exported
// functions (e.g. `initialize`) into this contract's own binary and
// collide with ours. `contractimport!` only reads the deployed contract's
// interface, so no implementation code is duplicated here.
//
// Build order: `asset_registry`, `nullifier_registry`, and
// `commitment_tree` must be built to wasm before this crate (see
// `../../README.md`).
mod asset_registry_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/asset_registry.wasm"
    );
}

mod nullifier_registry_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/nullifier_registry.wasm"
    );
}

mod commitment_tree_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/commitment_tree.wasm"
    );
}

/// Placeholder for a real zero-knowledge proof. `TODO(zk)`: replace with an
/// actual proof type + on-chain verifier once the proving system is chosen
/// (PROJECT.md Phase 2). Every call site that checks `proof.valid` is a
/// marker for where real verification must be wired in.
#[contracttype]
#[derive(Clone, Debug)]
pub struct ShroudProof {
    /// TODO(zk): stand-in for "this transaction is cryptographically
    /// valid." Never set this from untrusted input in anything but tests.
    pub valid: bool,
}

#[contracttype]
enum DataKey {
    Admin,
    AssetRegistry,
    NullifierRegistry,
    CommitmentTree,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    UnsupportedAsset = 3,
    UnknownRoot = 4,
    InvalidProof = 5,
    AlreadySpent = 6,
}

#[contract]
pub struct ShroudPool;

#[contractimpl]
impl ShroudPool {
    pub fn initialize(
        env: Env,
        admin: Address,
        asset_registry: Address,
        nullifier_registry: Address,
        commitment_tree: Address,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::AssetRegistry, &asset_registry);
        env.storage()
            .instance()
            .set(&DataKey::NullifierRegistry, &nullifier_registry);
        env.storage()
            .instance()
            .set(&DataKey::CommitmentTree, &commitment_tree);
        Ok(())
    }

    /// Deposit a supported Stellar asset into the shielded pool, recording
    /// `commitment` as the resulting private note.
    pub fn deposit(
        env: Env,
        depositor: Address,
        asset_id: Address,
        amount: i128,
        commitment: BytesN<32>,
    ) -> Result<BytesN<32>, Error> {
        let registries = Self::registries(&env)?;

        let asset_client = asset_registry_contract::Client::new(&env, &registries.asset_registry);
        if !asset_client.is_supported(&asset_id) {
            return Err(Error::UnsupportedAsset);
        }

        let token_client = token::TokenClient::new(&env, &asset_id);
        token_client.transfer(&depositor, &env.current_contract_address(), &amount);

        let tree_client = commitment_tree_contract::Client::new(&env, &registries.commitment_tree);
        let new_root = tree_client.insert(&commitment);

        env.events().publish(
            (Symbol::new(&env, "deposit"), asset_id),
            (depositor, amount, commitment, new_root.clone()),
        );

        Ok(new_root)
    }

    /// Shielded-to-shielded transfer: consumes one input note and creates
    /// one output commitment. No Stellar asset moves — this is purely
    /// internal pool bookkeeping.
    pub fn transfer(
        env: Env,
        root: BytesN<32>,
        nullifier: BytesN<32>,
        output_commitment: BytesN<32>,
        proof: ShroudProof,
    ) -> Result<BytesN<32>, Error> {
        let registries = Self::registries(&env)?;

        let tree_client = commitment_tree_contract::Client::new(&env, &registries.commitment_tree);
        if !tree_client.is_known_root(&root) {
            return Err(Error::UnknownRoot);
        }

        // TODO(zk): replace with real proof verification.
        if !proof.valid {
            return Err(Error::InvalidProof);
        }

        let nullifier_client =
            nullifier_registry_contract::Client::new(&env, &registries.nullifier_registry);
        if nullifier_client.is_spent(&nullifier) {
            return Err(Error::AlreadySpent);
        }
        nullifier_client.spend(&nullifier);

        let new_root = tree_client.insert(&output_commitment);

        env.events().publish(
            (Symbol::new(&env, "transfer"), nullifier),
            (output_commitment, new_root.clone()),
        );

        Ok(new_root)
    }

    /// Redeem a shielded note back into the underlying Stellar asset.
    pub fn withdraw(
        env: Env,
        recipient: Address,
        asset_id: Address,
        amount: i128,
        root: BytesN<32>,
        nullifier: BytesN<32>,
        proof: ShroudProof,
    ) -> Result<(), Error> {
        let registries = Self::registries(&env)?;

        let asset_client = asset_registry_contract::Client::new(&env, &registries.asset_registry);
        if !asset_client.is_supported(&asset_id) {
            return Err(Error::UnsupportedAsset);
        }

        let tree_client = commitment_tree_contract::Client::new(&env, &registries.commitment_tree);
        if !tree_client.is_known_root(&root) {
            return Err(Error::UnknownRoot);
        }

        // TODO(zk): replace with real proof verification.
        if !proof.valid {
            return Err(Error::InvalidProof);
        }

        let nullifier_client =
            nullifier_registry_contract::Client::new(&env, &registries.nullifier_registry);
        if nullifier_client.is_spent(&nullifier) {
            return Err(Error::AlreadySpent);
        }
        nullifier_client.spend(&nullifier);

        let token_client = token::TokenClient::new(&env, &asset_id);
        token_client.transfer(&env.current_contract_address(), &recipient, &amount);

        env.events().publish(
            (Symbol::new(&env, "withdraw"), asset_id),
            (recipient, amount, nullifier),
        );

        Ok(())
    }

    fn registries(env: &Env) -> Result<Registries, Error> {
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
}

struct Registries {
    asset_registry: Address,
    nullifier_registry: Address,
    commitment_tree: Address,
}

#[cfg(test)]
mod test;
