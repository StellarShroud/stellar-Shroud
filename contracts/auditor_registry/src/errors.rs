use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    AuditorNotFound = 1,
    AuditorAlreadyRegistered = 2,
}
