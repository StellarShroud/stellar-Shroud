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

## Step 1 — Workspace scaffold
- Root `Cargo.toml` as a workspace
- `contracts/` with 5 member crates per project.md's Project Structure section:
  `shroud_pool`, `commitment_tree`, `nullifier_registry`, `asset_registry`, `auditor_registry`
- Each crate: `Cargo.toml` + `src/lib.rs` wired to `soroban-sdk`
- `README.md` (short, points to `PROJECT.md` for full spec)
- `LICENSE` — leave as open question (spec flags contract/crypto licensing needs review)

## Step 2 — Data types & storage
- `asset_registry`: struct for supported asset (Stellar asset id, anchor, code, status) + admin-gated registration
- `nullifier_registry`: spent-nullifier set + `is_spent` / `mark_spent` with double-spend error
- `commitment_tree`: minimal Merkle tree storage (root history, leaf insertion) — real membership-proof verification deferred to Phase 2
- `auditor_registry`: auditor id, pubkey, anchor, status

## Step 3 — Shroud pool contract
- Deposit: pull supported asset in, create commitment, insert into tree, emit event
- Withdrawal: given a (stubbed) proof + nullifier, check nullifier unspent, mark spent, release asset
- Wire pool → the other four contracts via cross-contract calls
- Events for deposit/withdraw/transfer per project.md's Contract Responsibilities section

## Step 4 — Contract tests
- Per project.md's Testing Strategy: deposit, withdrawal, double-spend rejection, unsupported-asset rejection, unauthorized-auditor rejection
- Use `soroban-sdk`'s test utilities, no testnet needed yet

## Explicitly out of scope for this pass
- ZK circuit / proof system selection (Phase 2) — real proof verification stays a stub
- `crypto/`, `sdk/`, `frontend/` directories
- Auditor encryption/disclosure logic (Phase 4)
- Testnet deployment

## Open questions for the user before/while building
- Soroban SDK version / toolchain already pinned anywhere, or start fresh?
- Preferred placeholder for "proof" in Step 3 (e.g., a bool flag admin can toggle) so it's obviously not production-ready and easy to grep for later?
- ~~Should `project.md` be renamed to `PROJECT.md`?~~ Done.
