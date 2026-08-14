# StellarShroud frontend

Demo UI for the three roles PROJECT.md's Phase 7 describes: a user
wallet, an anchor dashboard, and an auditor dashboard.

## Status

All three pages run against `src/lib/mockWallet.ts` — in-memory mock
state, not a deployed contract. Every function in that file has a
`TODO(chain)` note on what it becomes once a testnet deployment and a
Freighter integration exist to call for real. Nothing here signs a
transaction or talks to Soroban RPC yet.

## Running

```sh
npm install
npm run dev       # http://localhost:3000
npm run build     # production build
npm run typecheck
```

## Layout

```text
src/
├── app/
│   ├── page.tsx           user wallet (balance, deposit, send, withdraw, history)
│   ├── anchor/page.tsx     anchor dashboard (supported assets, compliance config)
│   └── auditor/page.tsx    auditor dashboard (disclosed transactions, audit log)
├── components/             one component per card/table/form
└── lib/
    ├── types.ts             shapes mirroring the Soroban contracts' data
    └── mockWallet.ts         in-memory mock state + TODO(chain) markers
```
