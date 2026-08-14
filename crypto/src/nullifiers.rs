//! Nullifier derivation: prevents a note from being spent twice.
//!
//! `TODO(zk)`: `Nullifier = SHA256(secret || note_id)` is a placeholder
//! construction, kept simple so it's easy to compute both in the wallet
//! and (eventually) inside a circuit. The final construction must ensure
//! the nullifier reveals nothing about `secret` or which note was spent
//! beyond what the circuit already proves.

use crate::Hash32;
use sha2::{Digest, Sha256};

/// `secret` is the note owner's spending secret; `note_id` uniquely
/// identifies the note being spent (e.g. its commitment, or the index it
/// was inserted at). Deriving the nullifier from both means the same
/// secret produces a different nullifier per note, so spending one note
/// doesn't reveal whether the owner holds others.
pub fn derive_nullifier(secret: &Hash32, note_id: &Hash32) -> Hash32 {
    let mut hasher = Sha256::new();
    hasher.update(secret);
    hasher.update(note_id);
    hasher.finalize().into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nullifier_is_deterministic() {
        let secret = [5u8; 32];
        let note_id = [6u8; 32];
        assert_eq!(
            derive_nullifier(&secret, &note_id),
            derive_nullifier(&secret, &note_id)
        );
    }

    #[test]
    fn different_notes_produce_different_nullifiers() {
        let secret = [5u8; 32];
        assert_ne!(
            derive_nullifier(&secret, &[1u8; 32]),
            derive_nullifier(&secret, &[2u8; 32])
        );
    }

    #[test]
    fn different_secrets_produce_different_nullifiers() {
        let note_id = [1u8; 32];
        assert_ne!(
            derive_nullifier(&[5u8; 32], &note_id),
            derive_nullifier(&[6u8; 32], &note_id)
        );
    }
}
