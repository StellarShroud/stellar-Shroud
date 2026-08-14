#![no_std]

//! Registry of auditor public keys authorized by anchors for selective
//! disclosure. This crate only tracks *who* is authorized to receive
//! disclosed transaction data — the actual encryption/decryption flow
//! (Phase 4 in PROJECT.md) is not implemented here.

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, BytesN, Env};

#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AuditorStatus {
    Active,
    Revoked,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct AuditorInfo {
    pub anchor: Address,
    pub public_key: BytesN<32>,
    pub status: AuditorStatus,
    pub created_at: u64,
}

#[contracttype]
enum DataKey {
    Auditor(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    AuditorNotFound = 1,
    AuditorAlreadyRegistered = 2,
}

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

        let key = DataKey::Auditor(auditor);
        if env.storage().persistent().has(&key) {
            return Err(Error::AuditorAlreadyRegistered);
        }
        let info = AuditorInfo {
            anchor,
            public_key,
            status: AuditorStatus::Active,
            created_at: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&key, &info);
        Ok(())
    }

    pub fn revoke_auditor(env: Env, anchor: Address, auditor: Address) -> Result<(), Error> {
        anchor.require_auth();

        let key = DataKey::Auditor(auditor);
        let mut info: AuditorInfo = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::AuditorNotFound)?;
        if info.anchor != anchor {
            // Only the registering anchor can revoke; treat mismatched
            // callers the same as "not found" rather than leaking who owns it.
            return Err(Error::AuditorNotFound);
        }
        info.status = AuditorStatus::Revoked;
        env.storage().persistent().set(&key, &info);
        Ok(())
    }

    pub fn get_auditor(env: Env, auditor: Address) -> Result<AuditorInfo, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Auditor(auditor))
            .ok_or(Error::AuditorNotFound)
    }

    pub fn is_authorized(env: Env, auditor: Address, anchor: Address) -> bool {
        match env
            .storage()
            .persistent()
            .get::<_, AuditorInfo>(&DataKey::Auditor(auditor))
        {
            Some(info) => info.anchor == anchor && info.status == AuditorStatus::Active,
            None => false,
        }
    }
}

#[cfg(test)]
mod test;
