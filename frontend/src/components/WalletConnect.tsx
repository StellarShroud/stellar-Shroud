"use client";

import { useState } from "react";
import { getAddress, isConnected, requestAccess } from "@stellar/freighter-api";

export function WalletConnect({
  onAddressChange,
}: {
  onAddressChange?: (address: string | null) => void;
}) {
  const [address, setAddress] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [connecting, setConnecting] = useState(false);

  function setConnectedAddress(next: string | null) {
    setAddress(next);
    onAddressChange?.(next);
  }

  async function connect() {
    setError(null);
    setConnecting(true);
    try {
      const connectedCheck = await isConnected();
      if (connectedCheck.error) {
        throw new Error(connectedCheck.error.message ?? "Freighter is not installed");
      }

      const access = await requestAccess();
      if (access.error) {
        throw new Error(access.error.message ?? "Freighter access request failed");
      }
      if (access.address) {
        setConnectedAddress(access.address);
        return;
      }

      const current = await getAddress();
      if (current.error) {
        throw new Error(current.error.message ?? "Could not read Freighter address");
      }
      setConnectedAddress(current.address);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to connect Freighter");
    } finally {
      setConnecting(false);
    }
  }

  if (address) {
    return (
      <div className="card">
        <p className="card-title">Wallet</p>
        <div className="balance-row">
          <span className="balance-label">Connected</span>
          <span className="balance-amount">
            {address.slice(0, 6)}...{address.slice(-4)}
          </span>
        </div>
        <button type="button" className="secondary" onClick={() => setConnectedAddress(null)}>
          Disconnect
        </button>
      </div>
    );
  }

  return (
    <div className="card">
      <p className="card-title">Wallet</p>
      <p className="muted" style={{ marginTop: 0 }}>
        Not connected.
      </p>
      {error && (
        <p className="muted" style={{ color: "var(--danger)" }}>
          {error}
        </p>
      )}
      <button type="button" className="primary" disabled={connecting} onClick={connect}>
        {connecting ? "Connecting..." : "Connect Freighter"}
      </button>
    </div>
  );
}
