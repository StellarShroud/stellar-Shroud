# StellarShroud — Working Plan

Source: `PROJECT.md`. Repo currently contains only that spec — no code, no Cargo workspace.
Full roadmap in the spec has 8 phases (Research → Demo App); this plan scopes just the
first slice that's actually buildable right now: **Phase 0 wrap-up + Phase 1 (Soroban
Foundation), with ZK proof verification stubbed out.**

Rationale for scope: Phase 2 (ZK circuit + proving system selection) is a distinct,
heavy cryptography effort that shouldn't block getting the on-chain data model compiling,
tested, and reviewable. Building the shielded-pool contracts first with a placeholder
"proof always valid" check lets the state machine (deposits, commitments, nullifiers,
withdrawals) get validated independently of circuit work.

## Step 1 — Workspace scaffold ✅
- Root `Cargo.toml` as a workspace
- `contracts/` with 5 member crates per project.md's Project Structure section:
  `shroud_pool`, `commitment_tree`, `nullifier_registry`, `asset_registry`, `auditor_registry`
- Each crate: `Cargo.toml` + `src/lib.rs` wired to `soroban-sdk` 21.7.7
- `README.md` (short, points to `PROJECT.md` for full spec)
- `LICENSE` — still open (spec flags contract/crypto licensing needs review)

## Step 2 — Data types & storage ✅
- `asset_registry`: struct for supported asset (Stellar asset id, anchor, code, status) + admin-gated registration
- `nullifier_registry`: spent-nullifier set + `is_spent` / `spend` with double-spend error
- `commitment_tree`: incremental Merkle tree (depth 20, sha256, 30-entry root history) — real membership-proof *verification* deferred to Phase 2, this crate only maintains state
- `auditor_registry`: auditor id/pubkey/anchor/status, anchor-gated register/revoke

## Step 3 — Shroud pool contract ✅
- Deposit: check supported asset, pull token in via SEP-41 transfer, insert commitment, emit event
- Withdrawal: check known root + stub proof + unspent nullifier, mark spent, release asset, emit event
- Transfer: shielded-to-shielded, consumes one nullifier + inserts one output commitment, no token movement
- Wired pool → the other four contracts via cross-contract calls using `soroban_sdk::contractimport!`
  against pre-built WASM (see README "Building" — importing another `#[contract]` crate's *Rust source*
  doesn't work: its wasm-exported functions get linked into your own binary and collide with same-named
  exports from other imported contracts. `contractimport!` avoids this by reading only the compiled
  interface.)
- Events for deposit/withdraw/transfer per project.md's Contract Responsibilities section

## Step 4 — Contract tests ✅
- 18 tests across all 5 crates: asset registration/suspension, nullifier double-spend, Merkle root
  history, auditor register/revoke, and pool deposit/withdraw/transfer including rejection paths
  (unsupported asset, invalid proof, double-spend, unknown root)
- All pass natively (`cargo test`); all 5 contracts also build cleanly to `wasm32v1-none --release`
  via the staged build in README.md (see Step 8 -- this was `wasm32-unknown-unknown` until testnet
  deployment surfaced why that target is wrong)

## Step 5 — Off-chain crypto primitives (Phase 2, partial) ✅
- New `crypto/` crate (`shroud-crypto`), plain `std` Rust — this is wallet/prover-side code, not a contract
- `commitments.rs`: `Note::commitment()`, SHA-256-based (`TODO(zk)`, same caveat as PROJECT.md's Cryptographic Design section)
- `nullifiers.rs`: `derive_nullifier(secret, note_id)`, SHA-256-based (`TODO(zk)`)
- `merkle.rs`: off-chain incremental tree mirroring `commitment_tree`'s algorithm (same depth/hash), plus
  `MerkleTree::proof()` / `MerkleProof::verify()` — the actual membership-proof generation and verification
  PROJECT.md's Phase 2 calls for, which the on-chain contract intentionally skips (it only tracks roots)
- Integration test (`crypto/tests/onchain_root_agreement.rs`) proves the off-chain root computation matches
  the on-chain `commitment_tree` contract's root for an identical insertion sequence — this is the property
  a wallet's locally-tracked root depends on to build proofs the contract will accept
- 13 new tests, all passing; 31 total across the workspace

## Step 6 — Split each contract into types/errors/storage/lib ✅
- Per PROJECT.md's Project Structure (`lib.rs`, `storage.rs`, `errors.rs`, `events.rs`, `types.rs`),
  applied to all 5 contracts. Crates without a meaningful piece skip that file rather than carrying an
  empty placeholder (e.g. `nullifier_registry` has no custom `contracttype`, so no `types.rs`; only
  `shroud_pool` publishes events, so it's the only crate with `events.rs`; `shroud_pool` also gets a
  `clients.rs` for its three `contractimport!` cross-contract client modules)
- `lib.rs` in every crate is now just the `#[contract]` impl calling into the other modules
- Verified after each crate: `cargo test -p <crate>` and `cargo build -p <crate> --target
  wasm32-unknown-unknown --release` both still pass; full workspace re-run at the end (31/31 tests,
  staged wasm build for all 5 contracts) confirms the refactor changed no behavior

## Step 7 — Frontend demo (Phase 7, partial) ✅
- Next.js 16 + TypeScript app in `frontend/`, per PROJECT.md's stated stack (bumped off the initial
  ^15.0.0 pin after `npm audit` flagged high-severity postcss/sharp CVEs in that version range)
- Three pages matching PROJECT.md's Phase 7 Demo Application: `/` (User Wallet — balance, deposit,
  shield, send, withdraw, transaction history), `/anchor` (Anchor Dashboard — supported assets,
  shielded volume, compliance configuration), `/auditor` (Auditor Dashboard — disclosed transactions,
  audit history)
- Backed by `src/lib/mockWallet.ts`, in-memory mock state with `TODO(chain)` markers everywhere a real
  Freighter + Soroban RPC call would go instead — there's no testnet deployment or ZK circuit yet for
  a real integration to call
- Verified: `npm run build` succeeds for all three routes with zero warnings; SSR output checked via
  `curl` against `next dev` for all three pages confirms they render their mock data correctly

## Step 8 — Deploy to Stellar testnet ✅
- All 5 contracts deployed and initialized on testnet, addresses recorded in `deployments/testnet.json`
- Registered native XLM's Stellar Asset Contract as the one supported asset in `asset_registry`
  (needs no issuer setup, and the deploy/relayer identities already hold real testnet XLM)
- Found and fixed two real bugs that unit tests structurally could not have caught, because
  `mock_all_auths*` bypasses exactly the distinctions that broke on a real network:
  - `nullifier_registry`/`commitment_tree::initialize` required `admin.require_auth()`, but `admin` is
    the `shroud_pool` *contract* address — no private key exists to satisfy that. Fixed: removed the
    auth check, relying on the existing one-shot `has_admin` guard instead.
  - `shroud_pool::deposit` called `token.transfer(depositor, pool, amount)`, which needs depositor's auth
    for a call *nested* inside `deposit` — the network's default auth recording only covers the root
    invocation. Fixed: switched to the standard SEP-41 `approve` + `transfer_from` pattern, where
    `transfer_from` is authorized by *spender* (the pool itself, self-authorizing) instead of `from`.
- Also found: newer Rust toolchains default to `wasm32-unknown-unknown` + `reference-types` enabled,
  which Soroban's host wasmi doesn't support -- only surfaced on `auditor_registry` since it's a
  per-crate codegen decision, not something a target-feature flag reliably fixes. Real fix: build
  against `wasm32v1-none` (confirmed via `stellar contract build --print-commands-only`), not
  `wasm32-unknown-unknown` -- README.md updated accordingly, all 5 contracts rebuilt and redeployed.
- Verified a full `approve → deposit → withdraw` round trip via `stellar contract invoke` against real
  testnet XLM -- transaction hashes recorded in `deployments/testnet.json`

## Step 9 — Wire the frontend to the real deployment (partial) ✅
- Added `@stellar/stellar-sdk` + `@stellar/freighter-api`; new `src/lib/soroban.ts` (RPC
  simulate/sign/submit/poll helpers), `src/lib/notes.ts` (browser-side commitment/nullifier generation
  mirroring `crypto/`'s Rust logic via Web Crypto), `src/lib/chain.ts` (real `approveAndDeposit` /
  `withdrawNote`, composed from the above)
- `WalletConnect` now calls real Freighter (`isConnected`/`requestAccess`/`getAddress`) instead of
  faking a connected address
- New `LiveTestnetPanel` on the wallet page: shows the connected wallet's real XLM balance, and can
  submit real `approve`+`deposit`/`withdraw` transactions against the deployed `shroud_pool`, sitting
  alongside (not replacing) the existing mock USDC/EURC demo
- Scoped deliberately tight: shielded-to-shielded transfer stays mocked (constructing a real output note
  for a recipient this wallet doesn't hold the secret for is genuine SDK work, PROJECT.md Phase 6, not
  something to improvise here), and the Anchor/Auditor dashboards stay fully mock
- Verified: `npm run build` + typecheck clean; SSR output checked via `curl` against `next dev` for the
  not-connected states of both `WalletConnect` and `LiveTestnetPanel`

## Explicitly out of scope for this pass
- The actual ZK circuit and proving-system selection (Groth16/Plonk/etc., arkworks/circom/halo2, trusted
  setup vs. transparent, on-chain verifier cost) — this is a consequential, hard-to-reverse architectural
  choice with real security/cost tradeoffs and belongs to the user, not a default I should pick unilaterally
- `sdk/` directory
- Shielded-to-shielded transfer wired to the real deployment (needs real note/key management -- Phase 6)
- Auditor encryption/disclosure logic (Phase 4)
- Anchor/Auditor dashboards wired to real chain data

## Open questions for the user before/while building
- Soroban SDK version / toolchain already pinned anywhere, or start fresh? — Resolved: 21.7.7, fresh.
- Preferred placeholder for "proof" in Step 3 — Resolved: `ShroudProof { valid: bool }`, `TODO(zk)` markers.
- ~~Should `project.md` be renamed to `PROJECT.md`?~~ Done.
- **Proving system for the actual ZK circuit** (blocks the rest of Phase 2): Groth16 needs a trusted setup
  but has tiny, cheap-to-verify proofs; Plonk/Halo2 are transparent (no ceremony) but proofs cost more to
  verify; on-chain verification cost matters a lot here since it runs inside a Soroban contract's compute
  budget. Need the user's call before writing circuit code.
