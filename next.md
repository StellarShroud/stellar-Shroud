# StellarShroud — Next Up

`plan.md` documents what's been built and why. This is the opposite
direction: a backlog of candidate features, grouped by area, each with
the reasoning behind it so a pick can be made without re-deriving
context. Nothing here is committed to — pick an item (or several) and
say so, and it becomes the next `plan.md` step.

## Contracts

- ~~**Pause / circuit breaker on `shroud_pool`**~~ Done: admin-gated
  `pause()`/`unpause()`/`is_paused()`, blocking deposit/transfer/withdraw
  while paused. Required redeploying `shroud_pool` (no upgrade
  mechanism), which cascaded to `nullifier_registry`/`commitment_tree`
  since their admin is the pool's contract address — see
  `deployments/testnet.json`. Verified for real on testnet: paused,
  confirmed a deposit attempt was rejected with `Error::Paused`,
  unpaused, confirmed deposit succeeded again. 5 new tests, 36 total.
- **Second real asset registered on testnet** — register a real
  anchor-issued test asset (not just native XLM) to prove
  `asset_registry` actually gates multi-asset support the way
  PROJECT.md describes, not just in unit tests. Needs issuing a test
  asset + trustline, more setup than XLM but not hard.
- **Admin rotation** — `asset_registry`/`shroud_pool` admin is fixed at
  `initialize` with no way to transfer it. Fine for a testnet demo, a
  real gap for anything longer-lived.
- **Batch withdraw/deposit** — right now each note is one transaction.
  Batching would cut fees for wallets managing many notes. Low
  priority until there's a wallet UX that actually accumulates many
  notes.
- **Events consumed by an indexer, not just the frontend** — `deposit`/
  `transfer`/`withdraw` events exist but nothing currently indexes them
  for e.g. computing total shielded volume without re-simulating.
  Relevant once the Anchor Dashboard needs real numbers (see Frontend
  below).

## Cryptography (Phase 2 completion — blocked on one decision)

- **Pick the proving system.** Still the biggest open decision — see
  `plan.md`'s open question. Groth16 (arkworks) vs. Plonk/Halo2 is a
  real tradeoff (trusted setup + tiny proofs vs. no ceremony + costlier
  on-chain verification), and Soroban's compute budget makes the
  on-chain verification cost matter more than it would on most chains.
  Everything else in this section is blocked on this.
- **Circuit for the withdraw/transfer statement** — once a system is
  picked: constrain "I know a valid unspent note, it's in the current
  Merkle tree, the nullifier is correctly derived, amounts balance."
  PROJECT.md's Proof Circuit section already specifies the constraints;
  this is implementing them in the chosen DSL.
- **On-chain verifier in `shroud_pool`** — replace `ShroudProof.valid:
  bool` with a real proof + public inputs, and replace the `TODO(zk)`
  checks with actual verification. This is the change every other
  `TODO(zk)` marker in the codebase is waiting on.
- **Prover in the browser or a relayer service** — generating a real
  proof client-side (WASM-compiled circuit) vs. via a relayer the
  wallet sends private inputs to. Affects the frontend's `chain.ts`
  significantly either way.

## Frontend

- **Wire shielded transfer (Send) to the real deployment** — currently
  the one flow left mocked even in the "Live testnet" card, because it
  needs constructing an output note for a recipient whose secret this
  wallet doesn't hold. Real version needs either a recipient public key
  exchange or a relayer pattern. Worth scoping once the ZK circuit
  exists, since a real proof will be required for `transfer` too.
- ~~**Persist notes across reloads**~~ Done: `frontend/src/lib/notesStore.ts`,
  localStorage keyed per address. Still stores `note.secret` in the
  clear -- encrypting it with a key derived from a Freighter signature
  is flagged as a `TODO(chain)` there rather than done, since it's a
  real gap, not deferred silently.
- **Wire Anchor/Auditor dashboards to real chain reads** — both still
  show `mockWallet.ts` data. `asset_registry`/`auditor_registry` reads
  are simple (`readContract`, already have the pattern from
  `LiveTestnetPanel`); the "shielded volume" numbers need the indexer
  idea above, or a simpler running-total read from `shroud_pool` if one
  gets added there.
- **Real transaction history from chain events** — replace
  `mockWallet.getTransactions()` with `server.getEvents()` filtered to
  the connected address, for the Live testnet section specifically.
- **Multi-asset live support** — once a second asset is registered on
  testnet (see Contracts), extend `network.ts`/`chain.ts` beyond the
  single `XLM_ASSET_ID` constant.

## SDK (Phase 6 — not started)

- **Extract `frontend/src/lib/{chain,notes,soroban}.ts` into a
  standalone package** — this is already a rough draft of the shape
  PROJECT.md's SDK section describes (`ShroudWallet::deposit()`,
  `.generate_proof()`, `.submit()`). Pulling it out of `frontend/` into
  its own package (`sdk/typescript` or similar) makes it reusable by
  anything other than this demo UI, and is a natural checkpoint once
  the note-persistence and real-proof pieces above land.
- **Rust SDK crate** (`sdk/rust`, per PROJECT.md's Project Structure) —
  mirrors the TypeScript one for native/CLI wallets. Lower priority
  than the TypeScript version since nothing in this repo currently
  needs it.

## Tooling / DX

- ~~**CI**~~ Done: `.github/workflows/ci.yml`, two jobs -- `contracts`
  (`cargo test --workspace`, then the staged `wasm32v1-none` build) and
  `frontend` (`npm run typecheck`, `npm run build`). Runs on every push
  and PR to `main`. Doesn't catch the class of bug that only surfaced
  on real testnet deployment (see the pause-feature and earlier
  commits) -- that still needs an actual deployment to check.
- ~~**`CONTRIBUTING.md`**~~ Done: build/test workflow (including the
  wasm-before-tests ordering CI's first run caught), the
  types/errors/storage split convention, `TODO(zk)`/`TODO(chain)`
  markers, and the redeploy-cascade note for `shroud_pool` changes.
  The scoped-issues half of this item is still open -- no GitHub API
  token/`gh` CLI available in this environment to file them directly,
  so they're drafted (not filed) as of this note; see the conversation
  this was done in for the draft text if it's not yet on GitHub.

## Docs (per PROJECT.md's `docs/` layout — none of these exist yet)

- `docs/architecture.md` — PROJECT.md already has the content in prose
  form (Core Idea, Final Architecture sections); this would be the
  pulled-out, versioned reference.
- `docs/threat-model.md` — PROJECT.md's Threat Model section, expanded
  with what's now concretely true post-deployment (e.g. the admin/auth
  bugs found are a real data point for "what can go wrong").
- `docs/cryptography.md` — write once the proving system is chosen;
  premature before then.
- `docs/auditor-disclosure.md` — write alongside the Phase 4 disclosure
  work; premature now since `auditor_registry` only tracks keys, not
  disclosure.

## If you're prioritizing for Stellar Wave specifically

Given the earlier conversation about applying to Stellar Wave, the
highest-leverage items to do *first* are the ones that make this repo
look like a real project outside contributors can productively land
PRs in: **CI**, **`CONTRIBUTING.md`**, and turning a handful of the
items above into actual scoped GitHub issues (the pause/circuit-breaker
contract feature and the note-persistence frontend feature are both
good "Medium" complexity candidates — self-contained, testable, not
blocked on the ZK decision).

## Suggested order

1. ~~Pause/circuit breaker on `shroud_pool`~~ Done.
2. ~~CI~~ Done (and its own first run caught a real ordering bug — see
   the "CI reorder" fix in the commit history: native tests need the
   leaf contracts' wasm built first too, not just the release wasm step).
3. ~~Note persistence in the frontend~~ Done.
4. `CONTRIBUTING.md` + a first batch of scoped GitHub issues
5. Proving-system decision, then the rest of Cryptography/Contracts/SDK
   unblocks from there
