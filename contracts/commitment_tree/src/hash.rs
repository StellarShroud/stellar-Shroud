use soroban_sdk::{Bytes, BytesN, Env};

/// `SHA256(left || right)`. Must match `shroud-crypto`'s off-chain
/// `hash_pair` exactly (see `crypto/src/lib.rs`), or roots computed by a
/// wallet will never match roots stored on-chain.
pub(crate) fn hash_pair(env: &Env, left: &BytesN<32>, right: &BytesN<32>) -> BytesN<32> {
    let mut bytes = [0u8; 64];
    bytes[..32].copy_from_slice(&left.to_array());
    bytes[32..].copy_from_slice(&right.to_array());
    env.crypto().sha256(&Bytes::from_array(env, &bytes)).into()
}
