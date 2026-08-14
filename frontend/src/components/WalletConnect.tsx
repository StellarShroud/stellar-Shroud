"use client";

import { useState } from "react";

const MOCK_ADDRESS = "GDEMO...WALLET7X9";

/**
 * TODO(chain): swap this for the real Freighter flow --
 * `@stellar/freighter-api`'s `isConnected()` / `requestAccess()` /
 * `getAddress()`, called from a click handler (Freighter requires a user
 * gesture). Kept as local-only state until a testnet deployment exists
 * for this to actually connect to.
 */
export function WalletConnect() {
  const [address, setAddress] = useState<string | null>(null);

  if (address) {
    return (
      <div className="card">
        <p className="card-title">Wallet</p>
        <div className="balance-row">
          <span className="balance-label">Connected</span>
          <span className="balance-amount">{address}</span>
        </div>
        <button type="button" className="secondary" onClick={() => setAddress(null)}>
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
      <button type="button" className="primary" onClick={() => setAddress(MOCK_ADDRESS)}>
        Connect Freighter
      </button>
    </div>
  );
}
