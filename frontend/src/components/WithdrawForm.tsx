"use client";

import { useState } from "react";
import type { AssetCode, ShieldedAsset } from "@/lib/types";

export function WithdrawForm({
  assets,
  onWithdraw,
}: {
  assets: ShieldedAsset[];
  onWithdraw: (code: AssetCode, amount: number, recipient: string) => void;
}) {
  const [code, setCode] = useState<AssetCode>(assets[0]?.code ?? "USDC");
  const [amount, setAmount] = useState("");
  const [recipient, setRecipient] = useState("");

  const parsed = Number(amount);
  const asset = assets.find((a) => a.code === code);
  const valid =
    asset !== undefined &&
    parsed > 0 &&
    parsed <= asset.shieldedBalance &&
    recipient.trim().length > 0;

  return (
    <div className="card">
      <p className="card-title">Withdraw</p>
      <div className="field">
        <label htmlFor="withdraw-asset">Asset</label>
        <select
          id="withdraw-asset"
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
        <label htmlFor="withdraw-amount">Amount</label>
        <input
          id="withdraw-amount"
          type="number"
          min="0"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          placeholder="0.00"
        />
      </div>
      <div className="field">
        <label htmlFor="withdraw-recipient">Recipient (Stellar address)</label>
        <input
          id="withdraw-recipient"
          type="text"
          value={recipient}
          onChange={(e) => setRecipient(e.target.value)}
          placeholder="G..."
        />
      </div>
      <button
        type="button"
        className="primary"
        disabled={!valid}
        onClick={() => {
          onWithdraw(code, parsed, recipient);
          setAmount("");
          setRecipient("");
        }}
      >
        Withdraw
      </button>
    </div>
  );
}
