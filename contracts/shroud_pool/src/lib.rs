#![no_std]

//! The shielded pool: deposits, private transfers, and withdrawals.
//!
//! Delegates supported-asset checks to `asset_registry`, spent-note
//! tracking to `nullifier_registry`, and commitment/root state to
//! `commitment_tree`. Real ZK proof verification is not implemented yet
//! (see `ShroudProof` below) — this crate exists to validate the on-chain
//! state machine ahead of the cryptography work in PROJECT.md Phase 2.

mod clients;
mod errors;
mod events;
mod storage;
mod types;

pub use errors::Error;
pub use types::ShroudProof;

use clients::{asset_registry_contract, commitment_tree_contract, nullifier_registry_contract};
use soroban_sdk::{contract, contractimpl, token, Address, BytesN, Env};
use storage::Registries;

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
        if storage::has_admin(&env) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();

        storage::set_config(
            &env,
            &admin,
            &Registries {
                asset_registry,
                nullifier_registry,
                commitment_tree,
            },
        );
        Ok(())
    }

    /// Circuit breaker: blocks deposit/transfer/withdraw. Admin-only.
    /// There's no upgrade mechanism on this contract yet, so if a bug
    /// like the two found during testnet deployment (see git history)
    /// turns up again, pausing is the only mitigation short of every
    /// user racing to withdraw before an exploit does.
    pub fn pause(env: Env) -> Result<(), Error> {
        let admin = storage::get_admin(&env)?;
        admin.require_auth();

        storage::set_paused(&env, true);
        events::publish_paused(&env, true);
        Ok(())
    }

    pub fn unpause(env: Env) -> Result<(), Error> {
        let admin = storage::get_admin(&env)?;
        admin.require_auth();

        storage::set_paused(&env, false);
        events::publish_paused(&env, false);
        Ok(())
    }

    pub fn is_paused(env: Env) -> bool {
        storage::is_paused(&env)
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
        if storage::is_paused(&env) {
            return Err(Error::Paused);
        }
        let registries = storage::get_registries(&env)?;

        let asset_client = asset_registry_contract::Client::new(&env, &registries.asset_registry);
        if !asset_client.is_supported(&asset_id) {
            return Err(Error::UnsupportedAsset);
        }

        // Uses transfer_from (authorized by the *spender*, i.e. this
        // contract, which self-authorizes) rather than transfer
        // (authorized by depositor directly): depositor's require_auth
        // would be a "non-root" authorization -- required deep inside this
        // call rather than at the top-level invocation -- which the
        // network's default authorization recording does not support.
        // depositor must call token.approve(depositor, pool_address,
        // amount, ...) first so the allowance exists for this to draw on.
        let token_client = token::TokenClient::new(&env, &asset_id);
        token_client.transfer_from(
            &env.current_contract_address(),
            &depositor,
            &env.current_contract_address(),
            &amount,
        );

        let tree_client = commitment_tree_contract::Client::new(&env, &registries.commitment_tree);
        let new_root = tree_client.insert(&commitment);

        events::publish_deposit(&env, asset_id, depositor, amount, commitment, new_root.clone());

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
        if storage::is_paused(&env) {
            return Err(Error::Paused);
        }
        let registries = storage::get_registries(&env)?;

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

        events::publish_transfer(&env, nullifier, output_commitment, new_root.clone());

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
        if storage::is_paused(&env) {
            return Err(Error::Paused);
        }
        let registries = storage::get_registries(&env)?;

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

        events::publish_withdraw(&env, asset_id, recipient, amount, nullifier);

        Ok(())
    }
}

#[cfg(test)]
mod test;
