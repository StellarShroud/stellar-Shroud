import type { AuditLogEntry } from "@/lib/types";

export function AuditLog({ entries }: { entries: AuditLogEntry[] }) {
  return (
    <div className="card">
      <p className="card-title">Audit history</p>
      {entries.length === 0 ? (
        <p className="muted">No activity yet.</p>
      ) : (
        <table>
          <thead>
            <tr>
              <th>Action</th>
              <th>Actor</th>
              <th>Time</th>
            </tr>
          </thead>
          <tbody>
            {entries.map((entry) => (
              <tr key={entry.id}>
                <td>{entry.action}</td>
                <td className="muted">{entry.actor}</td>
                <td className="muted">{new Date(entry.timestamp).toLocaleString()}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
