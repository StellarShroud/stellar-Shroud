import type { AuditorEntry } from "@/lib/types";

export function AuditorList({ auditors }: { auditors: AuditorEntry[] }) {
  return (
    <div className="card">
      <p className="card-title">Compliance configuration</p>
      {auditors.length === 0 ? (
        <p className="muted">No auditors registered.</p>
      ) : (
        <table>
          <thead>
            <tr>
              <th>Anchor</th>
              <th>Public key</th>
              <th>Status</th>
            </tr>
          </thead>
          <tbody>
            {auditors.map((auditor) => (
              <tr key={auditor.id}>
                <td className="muted">{auditor.anchor}</td>
                <td className="muted">{auditor.publicKeyFingerprint}</td>
                <td>
                  <span
                    className="pill"
                    data-tone={auditor.status === "Active" ? "success" : "danger"}
                  >
                    {auditor.status}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
