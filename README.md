# StellarShroud

Confidential payments layer for Stellar anchor-issued stablecoins and
regulated digital assets. Private by default, auditable when authorized.
Full architecture, threat model, and roadmap live in
[`PROJECT.md`](./PROJECT.md).

## Status

Early scaffold. This repository currently implements:

- **Phase 1** — the Soroban contract state machine (asset registry,
  nullifier registry, commitment tree, auditor registry, shielded pool)
  with **zero-knowledge proof verification stubbed out**. All 5 contracts
  are deployed to **Stellar testnet** (see [Live on testnet](#live-on-testnet)
  below) and a full `approve → deposit → withdraw` round trip has been
  verified end-to-end against real testnet XLM — not just unit tests.
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
dashboard, and auditor dashboard, per PROJECT.md's Phase 7. Most of it
runs against in-memory mock state, but the wallet page's "Live testnet"
card is real: it connects Freighter and submits real signed
`approve`/`deposit`/`withdraw` transactions against the testnet
deployment below.

See [`plan.md`](./plan.md) for the current implementation scope, step by
step, including what's explicitly out of scope and why. See
[`next.md`](./next.md) for the backlog of candidate next features.

## Live on testnet

Addresses below are also machine-readable in
[`deployments/testnet.json`](./deployments/testnet.json), which the
frontend reads from (via `frontend/src/lib/network.ts`, kept in sync by
hand).

| Contract            | Address                                                                                                                             |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `shroud_pool`         | [`CDJBRZV6...ABTRXWJQ`](https://stellar.expert/explorer/testnet/contract/CDJBRZV6HTMLO4U4VPK5SF3GBEJV77CZXALWAGJXTTFKADM3ABTRXWJQ) |
| `asset_registry`      | [`CDASR4RB...A7B64DS5`](https://stellar.expert/explorer/testnet/contract/CDASR4RBML7PNEG3XVN2BD7FNR5XX3NZMFNWGMILJQXCPTWSA7B64DS5) |
| `nullifier_registry`  | [`CDALY6GL...IIXSXIHK`](https://stellar.expert/explorer/testnet/contract/CDALY6GLLIZA2PCOTUIMNDAUZFR4J5GT2OT44OUY7LJ4E777IIXSXIHK) |
| `commitment_tree`     | [`CD7ZS5OW...LLBRZLP3`](https://stellar.expert/explorer/testnet/contract/CD7ZS5OWIHD57HNKXZDJLHJ6DTZNFXS3ZPIQI5VFXTR42YEULLBRZLP3) |
| `auditor_registry`    | [`CBXGYPAV...JGGZ5Y5G`](https://stellar.expert/explorer/testnet/contract/CBXGYPAV4LXSLZ7CAS4FTMMEV4HXU5DIKNDBTVNQU62LX7KKJGGZ5Y5G) |

The only asset registered in `asset_registry` right now is native XLM's
Stellar Asset Contract
([`CDLZFC3S...2HHGCYSC`](https://stellar.expert/explorer/testnet/contract/CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC)) —
chosen because it needs no issuer setup, not because this is an
XLM-specific protocol.

Verified round trip (2026-08-14):
[`approve → deposit`](https://stellar.expert/explorer/testnet/tx/ffdad6ea5bca045174e6fb13f41f56cfa3413ebafe465c40ff4d3ad60a0af646) →
[`withdraw`](https://stellar.expert/explorer/testnet/tx/f900cc6fcaed239c4221a0d590a295ce54e70a1cca973ac5f81b157ec5c073f5).

## Roadmap

PROJECT.md lays out 8 phases end to end (Research → Demo Application).
Where this repo stands against them — see `plan.md` for the step-by-step
detail behind each line:

| Phase                        | Status         | Notes                                                                                |
| ----------------------------- | --------------- | --------------------------------------------------------------------------------------- |
| 0 — Research                  | Done            | architecture decisions made, recorded in `plan.md`                                    |
| 1 — Soroban Foundation        | Done            | 5 contracts, tested, deployed + verified on testnet                                   |
| 2 — Cryptographic Layer       | In progress     | commitments/nullifiers/Merkle proofs done; ZK circuit not started                     |
| 3 — ZK + Soroban Integration  | Not started     | blocked on Phase 2's proving-system choice                                            |
| 4 — Auditor Disclosure        | Not started     | registry exists (`auditor_registry`); encryption/disclosure flow doesn't              |
| 5 — Anchor Integration        | In progress     | asset registration works; no real anchor onboarding flow                              |
| 6 — SDK                       | Not started     | frontend's `chain.ts`/`notes.ts` are a preview of the shape                           |
| 7 — Demo Application          | In progress     | wallet/anchor/auditor UI exists; wallet's live testnet actions are real, most data is still mock |

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

## Prerequisites

- Rust (stable) with the `wasm32v1-none` target: `rustup target add wasm32v1-none`
- `stellar-cli` for deploying/invoking contracts: `cargo install --locked stellar-cli`
- Node.js 20+ for the frontend

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

## Testing

Native tests don't need the staged wasm build above — each crate's tests
register the real contract implementation in an in-memory test
environment:

```sh
cargo test
```

31 tests across the workspace: per-contract unit tests, plus an
integration test (`crypto/tests/onchain_root_agreement.rs`) that deploys
`commitment_tree` into a test environment and asserts its on-chain root
matches the off-chain `MerkleTree`'s root for an identical insertion
sequence — the property a wallet's locally-tracked root depends on.

## Deploying

Deployed to Stellar testnet already — see [Live on testnet](#live-on-testnet)
above, or [`deployments/testnet.json`](./deployments/testnet.json) for the
machine-readable version. To redeploy (dependency order matters, since
each contract's admin is set during `initialize` and can't be changed
afterward):

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

If you redeploy, update `deployments/testnet.json` and
`frontend/src/lib/network.ts` with the new addresses — they're not read
from a shared source, so they drift silently otherwise.

## Frontend

```sh
cd frontend
npm install
npm run dev
```

See [`frontend/README.md`](./frontend/README.md).

## License

Dual-licensed under [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE),
at your option — the same permissive pattern `soroban-sdk` itself uses, and
what PROJECT.md's License section suggests for the SDK/application layer.
Apache-2.0's explicit patent grant is worth having given the crypto/ZK
surface area here.

This covers the code as it stands today — everything currently marked
`TODO(zk)` is a placeholder, not production cryptography. PROJECT.md still
flags that the contract and cryptography components should get a dedicated
legal review before anything built on a real proving system ships to
mainnet; this license doesn't preempt that review, it just unblocks
contribution and experimentation in the meantime.
