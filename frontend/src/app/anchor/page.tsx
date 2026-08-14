import { AnchorAssetTable } from "@/components/AnchorAssetTable";
import { AuditorList } from "@/components/AuditorList";
import { getAnchorSummary, getAuditors } from "@/lib/mockWallet";

export default function AnchorPage() {
  const summary = getAnchorSummary();
  const auditors = getAuditors();

  return (
    <main>
      <div className="banner">
        Demo data only. In production this view reads on-chain state from
        `asset_registry` and `auditor_registry`, scoped to assets this
        anchor issues.
      </div>
      <AnchorAssetTable summary={summary} />
      <div style={{ marginTop: 16 }}>
        <AuditorList auditors={auditors} />
      </div>
    </main>
  );
}
