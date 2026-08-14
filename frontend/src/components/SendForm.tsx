"use client";

import { useState } from "react";
import type { AssetCode, ShieldedAsset } from "@/lib/types";

/**
 * Shielded-to-shielded transfer. No recipient address field on purpose --
 * in the real protocol the recipient is encoded in the output commitment
 * the sender builds off-chain (see crypto/src/commitments.rs), not passed
 * to the contract in the clear. TODO(chain): this needs a Merkle proof +
 * ShroudProof built via a Freighter-signed flow once the ZK circuit
 * exists (PROJECT.md Phase 2) -- shroud_pool.transfer takes exactly the
 * root/nullifier/output_commitment/proof shape this form stands in for.
 */
export function SendForm({
  assets,
  onSend,
}: {
  assets: ShieldedAsset[];
  onSend: (code: AssetCode, amount: number) => void;
}) {
  const [code, setCode] = useState<AssetCode>(assets[0]?.code ?? "USDC");
  const [amount, setAmount] = useState("");

  const parsed = Number(amount);
  const asset = assets.find((a) => a.code === code);
  const valid = asset !== undefined && parsed > 0 && parsed <= asset.shieldedBalance;

  return (
    <div className="card">
      <p className="card-title">Send (shielded)</p>
      <div className="field">
        <label htmlFor="send-asset">Asset</label>
        <select
          id="send-asset"
          value={code}
          onChange={(e) => setCode(e.target.value as AssetCode)}
        >
          {assets.map((a) => (
            <option key={a.code} value={a.code}>
              {a.code} ({a.shieldedBalance.toLocaleString()} shielded)
            </option>
          ))}
        </select>
      </div>
      <div className="field">
        <label htmlFor="send-amount">Amount</label>
        <input
          id="send-amount"
          type="number"
          min="0"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          placeholder="0.00"
        />
      </div>
      <button
        type="button"
        className="primary"
        disabled={!valid}
        onClick={() => {
          onSend(code, parsed);
          setAmount("");
        }}
      >
        Send privately
      </button>
    </div>
  );
}
