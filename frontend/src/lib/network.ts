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
  nullifierRegistry: "CDALY6GLLIZA2PCOTUIMNDAUZFR4J5GT2OT44OUY7LJ4E777IIXSXIHK",
  commitmentTree: "CD7ZS5OWIHD57HNKXZDJLHJ6DTZNFXS3ZPIQI5VFXTR42YEULLBRZLP3",
  auditorRegistry: "CBXGYPAV4LXSLZ7CAS4FTMMEV4HXU5DIKNDBTVNQU62LX7KKJGGZ5Y5G",
  shroudPool: "CDJBRZV6HTMLO4U4VPK5SF3GBEJV77CZXALWAGJXTTFKADM3ABTRXWJQ",
} as const;

/** Native XLM's Stellar Asset Contract -- the one asset registered in
 * asset_registry on testnet, chosen because it needs no issuer setup. */
export const XLM_ASSET_ID = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC";
