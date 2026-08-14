//! Off-chain cryptographic primitives for StellarShroud: commitments,
//! nullifiers, and Merkle membership proofs.
//!
//! `TODO(zk)`: everything here is hash-based (SHA-256), matching the
//! placeholder used by the `commitment_tree` Soroban contract so that
//! roots computed here agree with on-chain roots for the same leaf
//! sequence. Once a proving system is selected (PROJECT.md Phase 2), the
//! commitment and nullifier constructions must be replaced with whatever
//! primitives that circuit expects (e.g. a Pedersen commitment over the
//! circuit's native field), and this crate's Merkle proof format becomes
//! the private witness a circuit consumes rather than something verified
//! directly in Rust.

pub mod commitments;
pub mod merkle;
pub mod nullifiers;

pub type Hash32 = [u8; 32];

pub(crate) fn hash_pair(left: &Hash32, right: &Hash32) -> Hash32 {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}
