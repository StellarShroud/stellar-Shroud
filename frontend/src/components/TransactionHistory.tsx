import type { WalletTransaction } from "@/lib/types";

const kindLabel: Record<WalletTransaction["kind"], string> = {
  deposit: "Deposit",
  transfer: "Send",
  withdraw: "Withdraw",
};

const kindTone: Record<WalletTransaction["kind"], "success" | "danger" | undefined> = {
  deposit: "success",
  transfer: undefined,
  withdraw: "danger",
};

export function TransactionHistory({ transactions }: { transactions: WalletTransaction[] }) {
  return (
    <div className="card">
      <p className="card-title">Transaction history</p>
      {transactions.length === 0 ? (
        <p className="muted">No transactions yet.</p>
      ) : (
        <table>
          <thead>
            <tr>
              <th>Type</th>
              <th>Asset</th>
              <th>Amount</th>
              <th>Reference</th>
              <th>Time</th>
            </tr>
          </thead>
          <tbody>
            {transactions.map((tx) => (
              <tr key={tx.id}>
                <td>
                  <span className="pill" data-tone={kindTone[tx.kind]}>
                    {kindLabel[tx.kind]}
                  </span>
                </td>
                <td>{tx.asset}</td>
                <td>{tx.amount.toLocaleString()}</td>
                <td className="muted">{tx.reference}</td>
                <td className="muted">{new Date(tx.timestamp).toLocaleString()}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
