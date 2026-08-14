/**
 * Real Soroban RPC calls against the testnet deployment in `network.ts`.
 * Every export here does one of two things:
 *
 * - `readContract`: simulate-only, no signature, no submission -- for
 *   view-style calls like `balance` or `is_supported`.
 * - `invokeContract`: simulate, sign with Freighter, submit, poll until
 *   finalized -- for state-changing calls like `deposit`/`withdraw`.
 *
 * Nothing here is mocked. This is what `mockWallet.ts`'s `TODO(chain)`
 * markers become once there's a deployment to call.
 */

import {
  Address,
  BASE_FEE,
  Contract,
  TransactionBuilder,
  nativeToScVal,
  rpc,
  scValToNative,
  xdr,
} from "@stellar/stellar-sdk";
import { signTransaction } from "@stellar/freighter-api";
import { NETWORK_PASSPHRASE, RPC_URL } from "./network";

const server = new rpc.Server(RPC_URL);

export async function getLatestLedgerSequence(): Promise<number> {
  const { sequence } = await server.getLatestLedger();
  return sequence;
}

export function scAddress(address: string): xdr.ScVal {
  return new Address(address).toScVal();
}

export function scI128(amount: number | bigint): xdr.ScVal {
  return nativeToScVal(amount, { type: "i128" });
}

export function scU32(n: number): xdr.ScVal {
  return nativeToScVal(n, { type: "u32" });
}

export function scBytes32(hex: string): xdr.ScVal {
  return nativeToScVal(Buffer.from(hex, "hex"), { type: "bytes" });
}

/** Matches `ShroudProof { valid: bool }` from contracts/shroud_pool --
 * a `#[contracttype]` struct serializes as a map keyed by field name. */
export function scShroudProof(valid: boolean): xdr.ScVal {
  return xdr.ScVal.scvMap([
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvSymbol("valid"),
      val: xdr.ScVal.scvBool(valid),
    }),
  ]);
}

async function buildCallTx(
  contractId: string,
  method: string,
  args: xdr.ScVal[],
  sourceAddress: string,
) {
  const account = await server.getAccount(sourceAddress);
  const contract = new Contract(contractId);
  return new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call(method, ...args))
    .setTimeout(30)
    .build();
}

export async function readContract(
  contractId: string,
  method: string,
  args: xdr.ScVal[],
  sourceAddress: string,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
): Promise<any> {
  const tx = await buildCallTx(contractId, method, args, sourceAddress);
  const sim = await server.simulateTransaction(tx);

  if (rpc.Api.isSimulationError(sim)) {
    throw new Error(sim.error);
  }
  if (!("result" in sim) || !sim.result) {
    throw new Error(`Simulation of ${method} returned no result`);
  }
  return scValToNative(sim.result.retval);
}

export async function invokeContract(
  contractId: string,
  method: string,
  args: xdr.ScVal[],
  sourceAddress: string,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
): Promise<any> {
  const tx = await buildCallTx(contractId, method, args, sourceAddress);
  const prepared = await server.prepareTransaction(tx);

  const signed = await signTransaction(prepared.toXDR(), {
    networkPassphrase: NETWORK_PASSPHRASE,
    address: sourceAddress,
  });
  if (signed.error) {
    throw new Error(
      `Freighter declined to sign: ${JSON.stringify(signed.error)}`,
    );
  }

  const signedTx = TransactionBuilder.fromXDR(
    signed.signedTxXdr,
    NETWORK_PASSPHRASE,
  );

  const sendResult = await server.sendTransaction(signedTx);
  if (sendResult.status === "ERROR") {
    throw new Error(`Transaction submission failed for ${method}`);
  }

  const result = await server.pollTransaction(sendResult.hash, {
    attempts: 20,
  });
  if (result.status !== "SUCCESS") {
    throw new Error(`${method} transaction finished with status ${result.status}`);
  }

  return result.returnValue ? scValToNative(result.returnValue) : undefined;
}
