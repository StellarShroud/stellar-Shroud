# StellarShroud

Confidential payments layer for Stellar anchor-issued stablecoins and
regulated digital assets. Full architecture, threat model, and roadmap
live in [`PROJECT.md`](./PROJECT.md).

## Status

Early scaffold. This repository currently implements:

- **Phase 1** — the Soroban contract state machine (asset registry,
  nullifier registry, commitment tree, auditor registry, shielded pool)
  with **zero-knowledge proof verification stubbed out**. All 5 contracts
  are deployed to **Stellar testnet** (see [`deployments/testnet.json`](./deployments/testnet.json))
  and a full `approve → deposit → withdraw` round trip has been verified
  end-to-end against real testnet XLM — not just unit tests.
- **Phase 2, partially** — off-chain commitment/nullifier primitives and
  *real* Merkle membership proof generation + verification in `crypto/`
  (the on-chain tree only tracks roots; proving a leaf belongs under one
  is what a wallet/prover needs, and eventually what the ZK circuit
  constrains). The circuit itself, and which proving system it uses, is
  not implemented yet.

Testnet deployment surfaced two real bugs unit tests couldn't catch,
because Soroban's test-mode auth mocking (`mock_all_auths*`) papers over
distinctions that matter on a real network — see the commit history for
`nullifier_registry`/`commitment_tree` (an admin address with no private
key can't satisfy `require_auth`) and `shroud_pool::deposit` (a nested
`require_auth` needs the standard SEP-41 approve-then-`transfer_from`
pattern, not a direct `transfer`). Both are fixed.

Real proof verification is tracked as Phase 2 in `PROJECT.md` and marked
with `TODO(zk)` in code — every commitment/nullifier construction here is
a placeholder pending that decision.

Each contract is split into `types.rs` / `errors.rs` / `storage.rs` (and
`events.rs` / `hash.rs` / `clients.rs` where relevant) per PROJECT.md's
Project Structure section, with `lib.rs` left as thin orchestration.

There's also a demo frontend (`frontend/`) — user wallet, anchor
dashboard, and auditor dashboard, per PROJECT.md's Phase 7 — currently
running against in-memory mock state rather than a deployed contract.

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

frontend/                  Next.js demo UI -- see frontend/README.md

deployments/
└── testnet.json            deployed contract addresses + verified tx hashes
```

## Building

Contracts build against **`wasm32v1-none`**, not `wasm32-unknown-unknown`.
That's the target `stellar contract build` actually uses under the hood
(confirmed via `--print-commands-only`) — it's pinned to the WASM MVP
feature set, which Soroban's host wasmi runtime requires. Newer Rust
toolchains enable extra WASM features (e.g. `reference-types`) by default
for `wasm32-unknown-unknown` that the host rejects at deploy time, and
which contract crate's codegen happens to trip that varies per-crate, so
this can pass for some contracts and fail for others with no code change.

`shroud_pool` calls the other four contracts cross-contract, importing their
already-compiled WASM interfaces (`soroban_sdk::contractimport!`) rather than
depending on their Rust source — building multiple `#[contract]` crates into
one WASM binary isn't possible, and this is the standard Soroban pattern for
splitting a protocol across separate deployable contracts. That means the
four leaf contracts must be built before `shroud_pool`:

```sh
# 1. Build the contracts shroud_pool depends on
cargo build -p asset-registry -p nullifier-registry -p commitment-tree -p auditor-registry \
    --target wasm32v1-none --release

# 2. Build shroud_pool (reads the wasm files built above)
cargo build -p shroud-pool --target wasm32v1-none --release
```

Native tests don't need the staged build — each crate's tests register the
real contract implementation in an in-memory test environment:

```sh
cargo test
```

## Deploying

Deployed to Stellar testnet already — see
[`deployments/testnet.json`](./deployments/testnet.json) for addresses.
To redeploy (dependency order matters, since each contract's admin is set
during `initialize` and can't be changed afterward):

```sh
stellar contract deploy --wasm target/wasm32v1-none/release/asset_registry.wasm --source <key> --network testnet --alias shroud_asset_registry
stellar contract deploy --wasm target/wasm32v1-none/release/nullifier_registry.wasm --source <key> --network testnet --alias shroud_nullifier_registry
stellar contract deploy --wasm target/wasm32v1-none/release/commitment_tree.wasm --source <key> --network testnet --alias shroud_commitment_tree
stellar contract deploy --wasm target/wasm32v1-none/release/auditor_registry.wasm --source <key> --network testnet --alias shroud_auditor_registry
stellar contract deploy --wasm target/wasm32v1-none/release/shroud_pool.wasm --source <key> --network testnet --alias shroud_pool

stellar contract invoke --id shroud_asset_registry --source <key> --network testnet -- initialize --admin <admin address>
stellar contract invoke --id shroud_nullifier_registry --source <key> --network testnet -- initialize --admin <shroud_pool address>
stellar contract invoke --id shroud_commitment_tree --source <key> --network testnet -- initialize --admin <shroud_pool address>
stellar contract invoke --id shroud_pool --source <key> --network testnet -- initialize \
    --admin <admin address> --asset_registry <address> --nullifier_registry <address> --commitment_tree <address>
```

## Frontend

```sh
cd frontend
npm install
npm run dev
```

See [`frontend/README.md`](./frontend/README.md).
