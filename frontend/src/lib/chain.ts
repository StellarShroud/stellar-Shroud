/**
 * High-level operations against the real testnet deployment -- composes
 * `soroban.ts` (RPC plumbing) and `notes.ts` (commitment/nullifier
 * generation) into the same deposit/withdraw shapes `mockWallet.ts`
 * exposes, but backed by real transactions.
 *
 * Scope: this wires the native-XLM testnet asset end-to-end (the only
 * asset registered in asset_registry -- see deployments/testnet.json).
 * Shielded-to-shielded transfer is not wired here: it would need this
 * wallet to construct an output note for a recipient it doesn't control
 * the secret for, which is real SDK-level work (PROJECT.md Phase 6), not
 * something to improvise in a demo UI.
 */

import { CONTRACTS, XLM_ASSET_ID } from "./network";
import { buildCommitment, buildNullifier, hashToHex32, randomBytes32Hex } from "./notes";
import {
  getLatestLedgerSequence,
  invokeContract,
  readContract,
  scAddress,
  scBytes32,
  scI128,
  scShroudProof,
  scU32,
} from "./soroban";

/** A shielded note this wallet deposited and can still withdraw. Kept
 * only in memory -- reloading the page loses it. A real wallet would
 * persist and encrypt this; this is a demo. */
export interface ShieldedNote {
  amountStroops: number;
  commitment: string;
  secret: string;
  root: string;
  depositTxHash: string;
}

export async function getXlmBalance(address: string): Promise<bigint> {
  return readContract(XLM_ASSET_ID, "balance", [scAddress(address)], address);
}

/** Approves shroud_pool as a spender, then deposits -- two signed
 * transactions, matching the SEP-41 approve-then-transfer_from pattern
 * contracts/shroud_pool/src/lib.rs documents on `deposit`. */
export async function approveAndDeposit(
  address: string,
  amountStroops: number,
): Promise<ShieldedNote> {
  const currentLedger = await getLatestLedgerSequence();
  const expirationLedger = currentLedger + 1000;

  await invokeContract(
    XLM_ASSET_ID,
    "approve",
    [
      scAddress(address),
      scAddress(CONTRACTS.shroudPool),
      scI128(amountStroops),
      scU32(expirationLedger),
    ],
    address,
  );

  const secret = randomBytes32Hex();
  const randomness = randomBytes32Hex();
  const assetHex = await hashToHex32(XLM_ASSET_ID);
  const recipientHex = await hashToHex32(address);
  const commitment = await buildCommitment(assetHex, amountStroops, recipientHex, randomness);

  const deposit = await invokeContract(
    CONTRACTS.shroudPool,
    "deposit",
    [scAddress(address), scAddress(XLM_ASSET_ID), scI128(amountStroops), scBytes32(commitment)],
    address,
  );

  const root = Buffer.from(deposit.value as Uint8Array).toString("hex");

  return {
    amountStroops,
    commitment,
    secret,
    root,
    depositTxHash: deposit.hash,
  };
}

/** Redeems a shielded note back to `recipient`. `proof` is always
 * `{ valid: true }` here -- see ShroudProof's TODO(zk) in
 * contracts/shroud_pool/src/types.rs, there's no real circuit yet. */
export async function withdrawNote(
  address: string,
  note: ShieldedNote,
  recipient: string,
): Promise<void> {
  const nullifier = await buildNullifier(note.secret, note.commitment);

  await invokeContract(
    CONTRACTS.shroudPool,
    "withdraw",
    [
      scAddress(recipient),
      scAddress(XLM_ASSET_ID),
      scI128(note.amountStroops),
      scBytes32(note.root),
      scBytes32(nullifier),
      scShroudProof(true),
    ],
    address,
  );
}
