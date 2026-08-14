/**
 * Persists shielded notes to localStorage, keyed by wallet address, so
 * refreshing the page doesn't strand funds a note represents.
 *
 * TODO(chain): this stores `note.secret` in the clear. A real wallet
 * would encrypt it at rest -- e.g. deriving a key from a signature
 * Freighter produces (`signMessage`), so the plaintext only ever exists
 * in memory. Skipped here to keep this demo's dependency footprint
 * small; flagged rather than left silent since it's a real gap, not a
 * placeholder like the ShroudProof stub.
 */

import type { ShieldedNote } from "./chain";

function storageKey(address: string): string {
  return `shroud:notes:${address}`;
}

export function loadNotes(address: string): ShieldedNote[] {
  if (typeof window === "undefined") return [];
  try {
    const raw = window.localStorage.getItem(storageKey(address));
    if (!raw) return [];
    const parsed = JSON.parse(raw);
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

export function saveNotes(address: string, notes: ShieldedNote[]): void {
  if (typeof window === "undefined") return;
  window.localStorage.setItem(storageKey(address), JSON.stringify(notes));
}
