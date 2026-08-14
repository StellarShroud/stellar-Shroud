"use client";

import { useEffect, useState } from "react";
import { approveAndDeposit, getXlmBalance, withdrawNote, type ShieldedNote } from "@/lib/chain";
import { loadNotes, saveNotes } from "@/lib/notesStore";

const ONE_XLM_IN_STROOPS = 10_000_000;

function explorerTxUrl(hash: string): string {
  return `https://stellar.expert/explorer/testnet/tx/${hash}`;
}

function formatStroops(stroops: bigint | number): string {
  return (Number(stroops) / ONE_XLM_IN_STROOPS).toLocaleString(undefined, {
    maximumFractionDigits: 7,
  });
}

/**
 * The one part of this UI that talks to a real, deployed contract on
 * Stellar testnet -- everything else on this page runs against
 * `mockWallet.ts`. Deposits and withdrawals here submit real signed
 * transactions via Freighter against the address in
 * `deployments/testnet.json`. Notes are persisted per-address via
 * `notesStore.ts`, so they survive a page reload.
 */
export function LiveTestnetPanel({ address }: { address: string | null }) {
  const [balance, setBalance] = useState<bigint | null>(null);
  const [balanceError, setBalanceError] = useState<string | null>(null);
  const [notes, setNotes] = useState<ShieldedNote[]>([]);
  const [busy, setBusy] = useState(false);
  const [lastError, setLastError] = useState<string | null>(null);
  const [lastTxHash, setLastTxHash] = useState<string | null>(null);

  useEffect(() => {
    if (!address) {
      setBalance(null);
      setNotes([]);
      return;
    }
    setNotes(loadNotes(address));

    let cancelled = false;
    getXlmBalance(address)
      .then((b) => {
        if (!cancelled) setBalance(b);
      })
      .catch((err) => {
        if (!cancelled) {
          setBalanceError(err instanceof Error ? err.message : "Failed to load balance");
        }
      });
    return () => {
      cancelled = true;
    };
  }, [address]);

  if (!address) {
    return (
      <div className="card">
        <p className="card-title">Live testnet</p>
        <p className="muted" style={{ marginTop: 0 }}>
          Connect a wallet above to deposit and withdraw real testnet XLM
          against the deployed <code>shroud_pool</code> contract.
        </p>
      </div>
    );
  }

  async function handleDeposit() {
    if (!address) return;
    setBusy(true);
    setLastError(null);
    setLastTxHash(null);
    try {
      const note = await approveAndDeposit(address, ONE_XLM_IN_STROOPS);
      const updated = [...notes, note];
      setNotes(updated);
      saveNotes(address, updated);
      setLastTxHash(note.depositTxHash);
      const fresh = await getXlmBalance(address);
      setBalance(fresh);
    } catch (err) {
      setLastError(err instanceof Error ? err.message : "Deposit failed");
    } finally {
      setBusy(false);
    }
  }

  async function handleWithdraw(note: ShieldedNote) {
    if (!address) return;
    setBusy(true);
    setLastError(null);
    setLastTxHash(null);
    try {
      await withdrawNote(address, note, address);
      const updated = notes.filter((n) => n.commitment !== note.commitment);
      setNotes(updated);
      saveNotes(address, updated);
      const fresh = await getXlmBalance(address);
      setBalance(fresh);
    } catch (err) {
      setLastError(err instanceof Error ? err.message : "Withdraw failed");
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="card">
      <p className="card-title">Live testnet</p>

      <div className="balance-row">
        <span className="balance-label">XLM balance</span>
        <span className="balance-amount">
          {balance !== null ? formatStroops(balance) : balanceError ? "—" : "Loading..."}
        </span>
      </div>
      {balanceError && (
        <p className="muted" style={{ color: "var(--danger)" }}>
          {balanceError}
        </p>
      )}

      <button type="button" className="primary" disabled={busy} onClick={handleDeposit}>
        {busy ? "Working..." : "Deposit 1 XLM (testnet)"}
      </button>

      {lastError && (
        <p className="muted" style={{ color: "var(--danger)" }}>
          {lastError}
        </p>
      )}
      {lastTxHash && (
        <p className="muted">
          <a href={explorerTxUrl(lastTxHash)} target="_blank" rel="noreferrer">
            View transaction
          </a>
        </p>
      )}

      {notes.length > 0 && (
        <table style={{ marginTop: 12 }}>
          <thead>
            <tr>
              <th>Amount</th>
              <th>Commitment</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {notes.map((note) => (
              <tr key={note.commitment}>
                <td>{formatStroops(note.amountStroops)} XLM</td>
                <td className="muted">{note.commitment.slice(0, 12)}...</td>
                <td>
                  <button
                    type="button"
                    className="secondary"
                    disabled={busy}
                    onClick={() => handleWithdraw(note)}
                  >
                    Withdraw
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
