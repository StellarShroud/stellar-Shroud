# Contributing to StellarShroud

Thanks for looking at this. The project's what/why lives in
[`PROJECT.md`](./PROJECT.md); current status and layout in
[`README.md`](./README.md). This file is the how.

## Finding something to work on

[`next.md`](./next.md) is a prioritized backlog — pick anything marked
not-done there. If it's not already a GitHub Issue, open one first
(briefly) so effort doesn't collide, then reference it in your PR.

If you want something smaller to start with, look for issues labeled
`good first issue` — see the batch referenced below.

## Prerequisites

See [`README.md`'s Prerequisites section](./README.md#prerequisites):
Rust with the `wasm32v1-none` target, `stellar-cli`, Node.js 20+.

## Build & test

```sh
# Leaf contracts must build before shroud_pool -- and before `cargo test`
# too, not just the release wasm build. shroud_pool's contractimport!
# calls (see contracts/shroud_pool/src/clients.rs) run unconditionally
# as part of compiling its lib, which the test binary also depends on.
# This bit CI itself on its first run -- see the git history if you want
# the full story.
cargo build -p asset-registry -p nullifier-registry -p commitment-tree -p auditor-registry \
    --target wasm32v1-none --release

cargo test --workspace

# Only needed if you touched shroud_pool itself:
cargo build -p shroud-pool --target wasm32v1-none --release
```

For the frontend:

```sh
cd frontend
npm install
npm run typecheck
npm run build
```

CI (`.github/workflows/ci.yml`) runs all of the above on every push and
PR. It has to pass before a PR merges.

## Conventions this repo already follows

Match these rather than introducing a new pattern, unless you have a
good reason to change the pattern itself (and if so, say why in the PR):

- **Each contract crate is split into `types.rs` / `errors.rs` /
  `storage.rs`** (plus `events.rs`, `hash.rs`, or `clients.rs` where a
  crate actually needs one), with `lib.rs` left as thin orchestration —
  the `#[contract]` impl calling into the other modules, not containing
  logic itself. Don't add a file that would be empty; e.g.
  `nullifier_registry` has no `types.rs` because it has no custom
  `contracttype` beyond its storage key.
- **`TODO(zk)`** marks a placeholder waiting on the ZK proving-system
  decision (see `next.md`'s Cryptography section) — e.g. `ShroudProof`,
  the SHA-256-based commitment/nullifier constructions in `crypto/`.
  **`TODO(chain)`** marks a placeholder in the frontend waiting on real
  chain integration for something not yet wired up. Use these markers
  rather than leaving a silent gap or a vague comment.
- **New contract functionality needs tests**, and if it changes
  `shroud_pool`'s behavior, ideally a real verification against the
  testnet deployment too (not just unit tests) — this repo has hit two
  real bugs that only testnet deployment caught, because Soroban's
  `mock_all_auths*` test helpers paper over exactly the auth
  distinctions that mattered. See the git history around the pause
  feature and the two testnet-deployment fixes for what that looked
  like in practice.
- **Redeploying `shroud_pool` cascades**: it has no upgrade mechanism,
  so a new deployment gets a new address, which means
  `nullifier_registry`/`commitment_tree` (whose admin is the pool's
  contract address) need fresh deployments too. If your change touches
  `shroud_pool`, update `deployments/testnet.json` and
  `frontend/src/lib/network.ts` together — they're not read from a
  shared source.
- **Keep `plan.md`/`next.md`/`README.md` in sync** with what you build:
  mark the `next.md` item you finished, and if `plan.md`'s scope
  changed, update it too. Future contributors (and future you) rely on
  these matching reality.

## Commit style

Small, atomic commits with a message explaining *why*, not just what
changed — the existing git history is the reference for the level of
detail expected. Don't squash unrelated changes into one commit.

## Pull requests

- Reference the issue you're addressing.
- Make sure CI is green before requesting review.
- If you touched a contract, mention in the PR description whether you
  redeployed to testnet and verified there, or why that wasn't
  necessary for this change.

## License

By contributing, you agree your contribution is licensed under the same
dual MIT/Apache-2.0 terms as the rest of the project — see
[`LICENSE-MIT`](./LICENSE-MIT) and [`LICENSE-APACHE`](./LICENSE-APACHE).
