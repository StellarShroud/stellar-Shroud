use soroban_sdk::{contracttype, Address, BytesN};

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
