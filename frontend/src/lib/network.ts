/**
 * Testnet network + contract configuration.
 *
 * Mirrors ../../../deployments/testnet.json at the repo root -- kept as a
 * plain TS module rather than importing that file directly, since it
 * lives outside this Next.js project's root. Update both if you redeploy.
 */

export const NETWORK_PASSPHRASE = "Test SDF Network ; September 2015";
export const RPC_URL = "https://soroban-testnet.stellar.org/";

export const CONTRACTS = {
  assetRegistry: "CDASR4RBML7PNEG3XVN2BD7FNR5XX3NZMFNWGMILJQXCPTWSA7B64DS5",
  nullifierRegistry: "CAGMMCCSHZNDI4EI7GLGGWCTKHUGVJEOSAQ7H2S7KJOIW7BNWADSJRH5",
  commitmentTree: "CDQG33EO6C6KE7ZXXHQJ57OIAPYCVAXRG4R65RZTNCCBDMXVT7JXPLOZ",
  auditorRegistry: "CBXGYPAV4LXSLZ7CAS4FTMMEV4HXU5DIKNDBTVNQU62LX7KKJGGZ5Y5G",
  shroudPool: "CBXDMFY3YZM2SDOGC2DYN2B7DSKPZ7VBQTUAGLVEIOI27FEQFRJ7EJTK",
} as const;

/** Native XLM's Stellar Asset Contract -- the one asset registered in
 * asset_registry on testnet, chosen because it needs no issuer setup. */
export const XLM_ASSET_ID = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC";
