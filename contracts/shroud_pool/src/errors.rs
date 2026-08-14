use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    UnsupportedAsset = 3,
    UnknownRoot = 4,
    InvalidProof = 5,
    AlreadySpent = 6,
    Paused = 7,
}
