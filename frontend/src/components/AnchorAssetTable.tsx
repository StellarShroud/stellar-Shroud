import type { AnchorAssetSummary } from "@/lib/types";

export function AnchorAssetTable({ summary }: { summary: AnchorAssetSummary[] }) {
  return (
    <div className="card">
      <p className="card-title">Supported assets</p>
      <table>
        <thead>
          <tr>
            <th>Asset</th>
            <th>Shielded volume</th>
            <th>Deposits</th>
            <th>Withdrawals</th>
          </tr>
        </thead>
        <tbody>
          {summary.map((row) => (
            <tr key={row.code}>
              <td>{row.code}</td>
              <td>{row.shieldedVolume.toLocaleString()}</td>
              <td>{row.depositCount}</td>
              <td>{row.withdrawalCount}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
