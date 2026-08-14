# StellarShroud frontend

Demo UI for the three roles PROJECT.md's Phase 7 describes: a user
wallet, an anchor dashboard, and an auditor dashboard.

## Status

Most of the UI runs against `src/lib/mockWallet.ts` — in-memory mock
state, not a deployed contract. The exception is the **Live testnet**
card on the wallet page (`/`): it connects a real Freighter wallet and
submits real signed transactions against the contracts deployed in
`../deployments/testnet.json` — `approve` + `deposit` (native XLM into
`shroud_pool`) and `withdraw`, both provable on
[stellar.expert](https://stellar.expert/explorer/testnet).

What's still not wired to anything real:
- **Shielded-to-shielded transfer** — needs constructing an output note
  for a recipient this wallet doesn't hold the secret for; that's
  SDK-level work (PROJECT.md Phase 6), not something to improvise here.
- **Multi-asset** — only native XLM is registered in `asset_registry` on
  testnet (no issuer setup required), so the mock USDC/EURC assets stay
  mock-only.
- **Anchor/Auditor dashboards** — still fully mock data.

## Running

```sh
npm install
npm run dev       # http://localhost:3000
npm run build     # production build
npm run typecheck
```

To use the Live testnet card you'll need the
[Freighter](https://www.freighter.app/) browser extension, set to
Testnet, with a funded account (fund via
[friendbot](https://friendbot.stellar.org)).

## Layout

```text
src/
├── app/
│   ├── page.tsx           user wallet (balance, deposit, send, withdraw, history, live testnet)
│   ├── anchor/page.tsx     anchor dashboard (supported assets, compliance config)
│   └── auditor/page.tsx    auditor dashboard (disclosed transactions, audit log)
├── components/             one component per card/table/form
└── lib/
    ├── types.ts             shapes mirroring the Soroban contracts' data
    ├── mockWallet.ts         in-memory mock state + TODO(chain) markers
    ├── network.ts            testnet RPC URL + deployed contract addresses
    ├── soroban.ts            RPC read/invoke helpers (simulate/sign/submit/poll)
    ├── notes.ts              browser-side commitment/nullifier generation
    └── chain.ts              real approve+deposit / withdraw, composed from the above
```
