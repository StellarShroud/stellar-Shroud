/**
 * Shared types for the demo wallet UI. These mirror the shapes the
 * `shroud_pool` / `asset_registry` / `auditor_registry` Soroban contracts
 * expose, but nothing here talks to a real chain yet -- see
 * `mockWallet.ts` for the TODO(chain) markers where Freighter + Soroban
 * RPC calls would replace the local mock state.
 */

export type AssetCode = "USDC" | "EURC";

export interface ShieldedAsset {
  code: AssetCode;
  anchor: string;
  /** Public (unshielded) balance available to deposit, in whole units. */
  publicBalance: number;
  /** Shielded balance across all of this wallet's unspent notes. */
  shieldedBalance: number;
}

export type TransactionKind = "deposit" | "transfer" | "withdraw";

export interface WalletTransaction {
  id: string;
  kind: TransactionKind;
  asset: AssetCode;
  amount: number;
  /** Recipient shown only for withdrawals -- transfers hide this by design. */
  recipient?: string;
  timestamp: number;
  /** Truncated commitment or nullifier, shown for realism -- not a real hash. */
  reference: string;
}

export interface AuditorEntry {
  id: string;
  anchor: string;
  publicKeyFingerprint: string;
  status: "Active" | "Revoked";
}

export interface AnchorAssetSummary {
  code: AssetCode;
  shieldedVolume: number;
  depositCount: number;
  withdrawalCount: number;
}
