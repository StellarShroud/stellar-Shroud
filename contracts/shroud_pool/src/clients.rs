//! Cross-contract clients for the other four protocol contracts.
//!
//! These target already-compiled contract WASM rather than depending on
//! the other crates' Rust source: pulling in another `#[contract]` crate
//! as a normal dependency would link its wasm-exported functions (e.g.
//! `initialize`) into this contract's own binary and collide with ours.
//! `contractimport!` only reads the deployed contract's interface, so no
//! implementation code is duplicated here.
//!
//! Build order: `asset_registry`, `nullifier_registry`, and
//! `commitment_tree` must be built to wasm before this crate (see
//! `../../README.md`).

pub(crate) mod asset_registry_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/asset_registry.wasm"
    );
}

pub(crate) mod nullifier_registry_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/nullifier_registry.wasm"
    );
}

pub(crate) mod commitment_tree_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/commitment_tree.wasm"
    );
}
