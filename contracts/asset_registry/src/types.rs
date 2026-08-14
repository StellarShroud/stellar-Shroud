use soroban_sdk::{contracttype, Address, Symbol};

#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AssetStatus {
    Active,
    Suspended,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct AssetInfo {
    pub anchor: Address,
    pub code: Symbol,
    pub status: AssetStatus,
}
