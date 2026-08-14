"use client";

import { useState } from "react";
import type { AssetCode, ShieldedAsset } from "@/lib/types";

export function DepositForm({
  assets,
  onDeposit,
}: {
  assets: ShieldedAsset[];
  onDeposit: (code: AssetCode, amount: number) => void;
}) {
  const [code, setCode] = useState<AssetCode>(assets[0]?.code ?? "USDC");
  const [amount, setAmount] = useState("");

  const parsed = Number(amount);
  const asset = assets.find((a) => a.code === code);
  const valid = asset !== undefined && parsed > 0 && parsed <= asset.publicBalance;

  return (
    <div className="card">
      <p className="card-title">Deposit &amp; shield</p>
      <div className="field">
        <label htmlFor="deposit-asset">Asset</label>
        <select
          id="deposit-asset"
          value={code}
          onChange={(e) => setCode(e.target.value as AssetCode)}
        >
          {assets.map((a) => (
            <option key={a.code} value={a.code}>
              {a.code} ({a.publicBalance.toLocaleString()} public)
            </option>
          ))}
        </select>
      </div>
      <div className="field">
        <label htmlFor="deposit-amount">Amount</label>
        <input
          id="deposit-amount"
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
          onDeposit(code, parsed);
          setAmount("");
        }}
      >
        Deposit &amp; shield
      </button>
    </div>
  );
}
