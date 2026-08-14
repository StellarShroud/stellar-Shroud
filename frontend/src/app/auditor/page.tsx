import { AuditLog } from "@/components/AuditLog";
import { TransactionHistory } from "@/components/TransactionHistory";
import { getAuditLog, getTransactions } from "@/lib/mockWallet";

export default function AuditorPage() {
  const transactions = getTransactions();
  const log = getAuditLog();

  return (
    <main>
      <div className="banner">
        Demo data only. Real disclosure requires the auditor to decrypt
        transaction metadata with their registered private key --
        PROJECT.md Phase 4, not implemented yet. These transactions are
        shown as if disclosure already happened.
      </div>
      <TransactionHistory transactions={transactions} title="Disclosed transactions" />
      <div style={{ marginTop: 16 }}>
        <AuditLog entries={log} />
      </div>
    </main>
  );
}
