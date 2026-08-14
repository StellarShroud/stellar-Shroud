"use client";

import { useState } from "react";
import { BalanceCard } from "@/components/BalanceCard";
import { DepositForm } from "@/components/DepositForm";
import { SendForm } from "@/components/SendForm";
import { TransactionHistory } from "@/components/TransactionHistory";
import { WalletConnect } from "@/components/WalletConnect";
import { WithdrawForm } from "@/components/WithdrawForm";
import * as wallet from "@/lib/mockWallet";
import type { AssetCode } from "@/lib/types";

export default function WalletPage() {
  const [assets, setAssets] = useState(wallet.getAssets());
  const [transactions, setTransactions] = useState(wallet.getTransactions());

  function refresh() {
    setAssets([...wallet.getAssets()]);
    setTransactions(wallet.getTransactions());
  }

  function handleDeposit(code: AssetCode, amount: number) {
    wallet.deposit(code, amount);
    refresh();
  }

  function handleSend(code: AssetCode, amount: number) {
    wallet.sendShielded(code, amount);
    refresh();
  }

  function handleWithdraw(code: AssetCode, amount: number, recipient: string) {
    wallet.withdraw(code, amount, recipient);
    refresh();
  }

  return (
    <main>
      <div className="banner">
        Demo data only — not connected to a deployed contract. See{" "}
        <code>frontend/src/lib/mockWallet.ts</code> for the TODO(chain) markers.
      </div>

      <WalletConnect />

      <div className="grid-2" style={{ marginTop: 16 }}>
        <BalanceCard assets={assets} />
        <div>
          <DepositForm assets={assets} onDeposit={handleDeposit} />
        </div>
      </div>

      <div className="grid-2" style={{ marginTop: 16 }}>
        <SendForm assets={assets} onSend={handleSend} />
        <WithdrawForm assets={assets} onWithdraw={handleWithdraw} />
      </div>

      <div style={{ marginTop: 16 }}>
        <TransactionHistory transactions={transactions} />
      </div>
    </main>
  );
}
