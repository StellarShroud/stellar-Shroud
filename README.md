# StellarShroud

Confidential payments layer for Stellar anchor-issued stablecoins and
regulated digital assets. Full architecture, threat model, and roadmap
live in [`PROJECT.md`](./PROJECT.md).

## Status

Early scaffold. This repository currently implements:

- **Phase 1** — the Soroban contract state machine (asset registry,
  nullifier registry, commitment tree, auditor registry, shielded pool)
  with **zero-knowledge proof verification stubbed out**.
- **Phase 2, partially** — off-chain commitment/nullifier primitives and
  *real* Merkle membership proof generation + verification in `crypto/`
  (the on-chain tree only tracks roots; proving a leaf belongs under one
  is what a wallet/prover needs, and eventually what the ZK circuit
  constrains). The circuit itself, and which proving system it uses, is
  not implemented yet.

Real proof verification is tracked as Phase 2 in `PROJECT.md` and marked
with `TODO(zk)` in code — every commitment/nullifier construction here is
a placeholder pending that decision.

See [`plan.md`](./plan.md) for the current implementation scope.

## Layout

```text
contracts/
├── shroud_pool/          deposit / shielded transfer / withdraw
├── commitment_tree/       incremental Merkle tree of note commitments
├── nullifier_registry/    double-spend guard
├── asset_registry/        which Stellar assets are shieldable
└── auditor_registry/      anchor-authorized auditor public keys

crypto/
├── commitments.rs         shielded note commitment (hash-based, TODO(zk))
├── nullifiers.rs           nullifier derivation (hash-based, TODO(zk))
└── merkle.rs                off-chain tree mirror + membership proofs
```

## Building

`shroud_pool` calls the other four contracts cross-contract, importing their
already-compiled WASM interfaces (`soroban_sdk::contractimport!`) rather than
depending on their Rust source — building multiple `#[contract]` crates into
one WASM binary isn't possible, and this is the standard Soroban pattern for
splitting a protocol across separate deployable contracts. That means the
four leaf contracts must be built before `shroud_pool`:

```sh
# 1. Build the contracts shroud_pool depends on
cargo build -p asset-registry -p nullifier-registry -p commitment-tree -p auditor-registry \
    --target wasm32-unknown-unknown --release

# 2. Build shroud_pool (reads the wasm files built above)
cargo build -p shroud-pool --target wasm32-unknown-unknown --release
```

Native tests don't need the staged build — each crate's tests register the
real contract implementation in an in-memory test environment:

```sh
cargo test
```
