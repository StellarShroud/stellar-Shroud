//! Shielded note commitments.
//!
//! `TODO(zk)`: `Commitment = SHA256(asset || amount || recipient || randomness)`
//! is a stand-in construction. It is binding and hiding under SHA-256's
//! standard assumptions, but it is not necessarily the primitive the
//! eventual ZK circuit will want (a circuit-friendly hash or Pedersen
//! commitment is typical, chosen to minimize constraint count).

use crate::Hash32;
use sha2::{Digest, Sha256};

/// A private shielded note. `randomness` must be sampled fresh per note —
/// reusing it breaks the commitment's hiding property.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Note {
    pub asset: Hash32,
    pub amount: u64,
    pub recipient: Hash32,
    pub randomness: Hash32,
}

impl Note {
    pub fn new(asset: Hash32, amount: u64, recipient: Hash32, randomness: Hash32) -> Self {
        Self {
            asset,
            amount,
            recipient,
            randomness,
        }
    }

    pub fn commitment(&self) -> Hash32 {
        let mut hasher = Sha256::new();
        hasher.update(self.asset);
        hasher.update(self.amount.to_be_bytes());
        hasher.update(self.recipient);
        hasher.update(self.randomness);
        hasher.finalize().into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn commitment_is_deterministic() {
        let note = Note::new([1u8; 32], 1_000, [2u8; 32], [3u8; 32]);
        assert_eq!(note.commitment(), note.commitment());
    }

    #[test]
    fn different_randomness_changes_commitment() {
        let a = Note::new([1u8; 32], 1_000, [2u8; 32], [3u8; 32]);
        let b = Note::new([1u8; 32], 1_000, [2u8; 32], [4u8; 32]);
        assert_ne!(a.commitment(), b.commitment());
    }

    #[test]
    fn different_amount_changes_commitment() {
        let a = Note::new([1u8; 32], 1_000, [2u8; 32], [3u8; 32]);
        let b = Note::new([1u8; 32], 2_000, [2u8; 32], [3u8; 32]);
        assert_ne!(a.commitment(), b.commitment());
    }
}
