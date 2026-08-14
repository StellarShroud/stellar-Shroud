import type { ShieldedAsset } from "@/lib/types";

export function BalanceCard({ assets }: { assets: ShieldedAsset[] }) {
  return (
    <div className="card">
      <p className="card-title">Balances</p>
      {assets.map((asset) => (
        <div key={asset.code}>
          <div className="balance-row">
            <span className="balance-label">{asset.code} — public</span>
            <span className="balance-amount">{asset.publicBalance.toLocaleString()}</span>
          </div>
          <div className="balance-row">
            <span className="balance-label">{asset.code} — shielded</span>
            <span className="balance-amount">{asset.shieldedBalance.toLocaleString()}</span>
          </div>
        </div>
      ))}
    </div>
  );
}
