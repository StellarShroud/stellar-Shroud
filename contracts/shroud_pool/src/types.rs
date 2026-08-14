use soroban_sdk::contracttype;

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
