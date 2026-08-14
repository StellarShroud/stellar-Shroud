/**
 * In-memory stand-in for wallet + chain state.
 *
 * TODO(chain): every function here should eventually become a Soroban RPC
 * call through a Stellar wallet (Freighter) signing a transaction that
 * invokes `shroud_pool` -- see contracts/shroud_pool for the actual
 * deposit/transfer/withdraw logic this mirrors. Until the ZK circuit
 * (PROJECT.md Phase 2) and a testnet deployment exist, there's nothing
 * real to call, so this module exists purely to make the UI demonstrable
 * end-to-end against the same shapes the contracts use.
 */

import type {
  AnchorAssetSummary,
  AuditLogEntry,
  AuditorEntry,
  ShieldedAsset,
  WalletTransaction,
} from "./types";

let assets: ShieldedAsset[] = [
  { code: "USDC", anchor: "GACHOR...ANCHOR1", publicBalance: 4200, shieldedBalance: 1000 },
  { code: "EURC", anchor: "GACHOR...ANCHOR2", publicBalance: 800, shieldedBalance: 0 },
];

let transactions: WalletTransaction[] = [
  {
    id: "tx-1",
    kind: "deposit",
    asset: "USDC",
    amount: 1000,
    timestamp: Date.now() - 1000 * 60 * 60 * 5,
    reference: "0x3f9a...c21",
  },
];

const auditors: AuditorEntry[] = [
  {
    id: "auditor-1",
    anchor: "GACHOR...ANCHOR1",
    publicKeyFingerprint: "9c1a...8e02",
    status: "Active",
  },
];

const anchorSummary: AnchorAssetSummary[] = [
  { code: "USDC", shieldedVolume: 128_400, depositCount: 42, withdrawalCount: 17 },
  { code: "EURC", shieldedVolume: 9_200, depositCount: 6, withdrawalCount: 2 },
];

const auditLog: AuditLogEntry[] = [
  {
    id: "log-1",
    action: "Registered auditor for USDC",
    actor: "GACHOR...ANCHOR1",
    timestamp: Date.now() - 1000 * 60 * 60 * 24 * 3,
  },
  {
    id: "log-2",
    action: "Disclosed transaction 0x3f9a...c21",
    actor: "auditor-1",
    timestamp: Date.now() - 1000 * 60 * 60 * 2,
  },
];

function randomReference(): string {
  const bytes = Array.from({ length: 3 }, () =>
    Math.floor(Math.random() * 256)
      .toString(16)
      .padStart(2, "0"),
  ).join("");
  return `0x${bytes}...${Math.floor(Math.random() * 999)}`;
}

export function getAssets(): ShieldedAsset[] {
  return assets;
}

export function getTransactions(): WalletTransaction[] {
  return [...transactions].sort((a, b) => b.timestamp - a.timestamp);
}

export function getAuditors(): AuditorEntry[] {
  return auditors;
}

export function getAnchorSummary(): AnchorAssetSummary[] {
  return anchorSummary;
}

export function getAuditLog(): AuditLogEntry[] {
  return [...auditLog].sort((a, b) => b.timestamp - a.timestamp);
}

/** Moves `amount` from public to shielded balance for `code`. */
export function deposit(code: ShieldedAsset["code"], amount: number): void {
  const asset = assets.find((a) => a.code === code);
  if (!asset || amount <= 0 || amount > asset.publicBalance) return;

  asset.publicBalance -= amount;
  asset.shieldedBalance += amount;
  transactions = [
    ...transactions,
    {
      id: `tx-${transactions.length + 1}`,
      kind: "deposit",
      asset: code,
      amount,
      timestamp: Date.now(),
      reference: randomReference(),
    },
  ];
}

/** Shielded-to-shielded transfer. Doesn't change this wallet's total, since
 * in the real protocol it consumes one note and creates an output note --
 * this mock just records the event for the transaction history demo. */
export function sendShielded(code: ShieldedAsset["code"], amount: number): void {
  const asset = assets.find((a) => a.code === code);
  if (!asset || amount <= 0 || amount > asset.shieldedBalance) return;

  transactions = [
    ...transactions,
    {
      id: `tx-${transactions.length + 1}`,
      kind: "transfer",
      asset: code,
      amount,
      timestamp: Date.now(),
      reference: randomReference(),
    },
  ];
}

/** Moves `amount` from shielded back to public balance for `code`. */
export function withdraw(
  code: ShieldedAsset["code"],
  amount: number,
  recipient: string,
): void {
  const asset = assets.find((a) => a.code === code);
  if (!asset || amount <= 0 || amount > asset.shieldedBalance) return;

  asset.shieldedBalance -= amount;
  transactions = [
    ...transactions,
    {
      id: `tx-${transactions.length + 1}`,
      kind: "withdraw",
      asset: code,
      amount,
      recipient,
      timestamp: Date.now(),
      reference: randomReference(),
    },
  ];
}
