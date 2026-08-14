#![no_std]

//! Registry of auditor public keys authorized by anchors for selective
//! disclosure. This crate only tracks *who* is authorized to receive
//! disclosed transaction data — the actual encryption/decryption flow
//! (Phase 4 in PROJECT.md) is not implemented here.

mod errors;
mod storage;
mod types;

pub use errors::Error;
pub use types::{AuditorInfo, AuditorStatus};

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

#[contract]
pub struct AuditorRegistry;

#[contractimpl]
impl AuditorRegistry {
    /// The anchor registers an auditor's public key for its own assets.
    /// Only the anchor may register or revoke its auditors — this keeps
    /// auditor authorization scoped per-anchor rather than protocol-wide.
    pub fn register_auditor(
        env: Env,
        anchor: Address,
        auditor: Address,
        public_key: BytesN<32>,
    ) -> Result<(), Error> {
        anchor.require_auth();

        if storage::has_auditor(&env, &auditor) {
            return Err(Error::AuditorAlreadyRegistered);
        }
        let info = AuditorInfo {
            anchor,
            public_key,
            status: AuditorStatus::Active,
            created_at: env.ledger().timestamp(),
        };
        storage::set_auditor(&env, &auditor, &info);
        Ok(())
    }

    pub fn revoke_auditor(env: Env, anchor: Address, auditor: Address) -> Result<(), Error> {
        anchor.require_auth();

        let mut info = storage::get_auditor(&env, &auditor).ok_or(Error::AuditorNotFound)?;
        if info.anchor != anchor {
            // Only the registering anchor can revoke; treat mismatched
            // callers the same as "not found" rather than leaking who owns it.
            return Err(Error::AuditorNotFound);
        }
        info.status = AuditorStatus::Revoked;
        storage::set_auditor(&env, &auditor, &info);
        Ok(())
    }

    pub fn get_auditor(env: Env, auditor: Address) -> Result<AuditorInfo, Error> {
        storage::get_auditor(&env, &auditor).ok_or(Error::AuditorNotFound)
    }

    pub fn is_authorized(env: Env, auditor: Address, anchor: Address) -> bool {
        storage::get_auditor(&env, &auditor)
            .map(|info| info.anchor == anchor && info.status == AuditorStatus::Active)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod test;
