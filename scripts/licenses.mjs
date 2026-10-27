// Fails when a production npm dependency has a license outside ADR 0003's allow-list.
// An SPDX "A OR B" passes when either side is allowed; "A AND B" needs both.
import { execFileSync } from "node:child_process";

const ALLOWED = new Set([
  "MIT",
  "Apache-2.0",
  "BSD-2-Clause",
  "BSD-3-Clause",
  "ISC",
  "Zlib",
  "Unicode-3.0",
  "0BSD",
  "CC-BY-4.0",
]);

function allowed(expression) {
  const spdx = expression.replace(/[()]/g, "").trim();
  if (spdx.includes(" OR ")) return spdx.split(" OR ").some(allowed);
  if (spdx.includes(" AND ")) return spdx.split(" AND ").every(allowed);
  return ALLOWED.has(spdx);
}

const byLicense = JSON.parse(
  execFileSync("pnpm", ["licenses", "list", "--prod", "--json"], { encoding: "utf8" }),
);
const refused = Object.entries(byLicense)
  .filter(([license]) => !allowed(license))
  .flatMap(([license, pkgs]) => pkgs.map((p) => `${p.name}: ${license}`));

if (refused.length > 0) {
  console.error(`licenses: not allowed by ADR 0003:\n  ${refused.join("\n  ")}`);
  process.exit(1);
}
console.log(`licenses: ${Object.values(byLicense).flat().length} production packages ok`);
