/**
 * Browser-side mirror of crypto/src/commitments.rs and
 * crypto/src/nullifiers.rs, using Web Crypto instead of the `sha2` crate.
 * Same TODO(zk) caveat applies: SHA-256-based commitments/nullifiers are
 * placeholders pending a real proving system (PROJECT.md Phase 2), kept
 * identical here only so a note built in the browser hashes the same way
 * the Rust side documents.
 */

function toHex(bytes: ArrayBuffer): string {
  return Array.from(new Uint8Array(bytes))
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}

function fromHex(hex: string): Uint8Array {
  const bytes = new Uint8Array(hex.length / 2);
  for (let i = 0; i < bytes.length; i++) {
    bytes[i] = parseInt(hex.slice(i * 2, i * 2 + 2), 16);
  }
  return bytes;
}

function concatBytes(...parts: Uint8Array[]): Uint8Array {
  const total = parts.reduce((sum, p) => sum + p.length, 0);
  const out = new Uint8Array(total);
  let offset = 0;
  for (const p of parts) {
    out.set(p, offset);
    offset += p.length;
  }
  return out;
}

async function sha256(bytes: Uint8Array): Promise<Uint8Array> {
  const digest = await crypto.subtle.digest("SHA-256", bytes as BufferSource);
  return new Uint8Array(digest);
}

function u64BeBytes(amount: number): Uint8Array {
  const buf = new ArrayBuffer(8);
  new DataView(buf).setBigUint64(0, BigInt(amount), false);
  return new Uint8Array(buf);
}

/** Fresh 32-byte randomness for a new note. */
export function randomBytes32Hex(): string {
  const bytes = new Uint8Array(32);
  crypto.getRandomValues(bytes);
  return toHex(bytes.buffer as ArrayBuffer);
}

/** `Commitment = SHA256(asset || amount || recipient || randomness)` --
 * matches `Note::commitment()` in crypto/src/commitments.rs. `asset` and
 * `recipient` are 32-byte hex strings (pad/hash a Stellar address to fit;
 * here we just hash the address string itself for a stand-in 32 bytes). */
export async function buildCommitment(
  assetHex32: string,
  amount: number,
  recipientHex32: string,
  randomnessHex32: string,
): Promise<string> {
  const digest = await sha256(
    concatBytes(
      fromHex(assetHex32),
      u64BeBytes(amount),
      fromHex(recipientHex32),
      fromHex(randomnessHex32),
    ),
  );
  return toHex(digest.buffer as ArrayBuffer);
}

/** `Nullifier = SHA256(secret || note_id)` -- matches
 * `derive_nullifier()` in crypto/src/nullifiers.rs. */
export async function buildNullifier(
  secretHex32: string,
  noteIdHex32: string,
): Promise<string> {
  const digest = await sha256(concatBytes(fromHex(secretHex32), fromHex(noteIdHex32)));
  return toHex(digest.buffer as ArrayBuffer);
}

/** Hashes an arbitrary string down to 32 bytes of hex -- used to turn a
 * Stellar G-address into the fixed-size field `buildCommitment` expects. */
export async function hashToHex32(input: string): Promise<string> {
  const encoded = new TextEncoder().encode(input);
  const digest = await sha256(encoded);
  return toHex(digest.buffer as ArrayBuffer);
}
