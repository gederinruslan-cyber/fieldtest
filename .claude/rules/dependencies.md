---
paths:
  - "package.json"
  - "pnpm-lock.yaml"
  - "pnpm-workspace.yaml"
  - "apps/*/package.json"
  - "packages/*/package.json"
  - "e2e/package.json"
  - ".nvmrc"
  - "engine/Cargo.toml"
  - "engine/Cargo.lock"
  - "engine/crates/*/Cargo.toml"
  - "engine/deny.toml"
---

# Dependency changes (apply when touching these files)

- Licenses: MIT, Apache-2.0, BSD-2/3-Clause, ISC, Zlib, Unicode only (ADR 0003). `cargo deny
  check licenses` and `pnpm licenses:check` run in `pnpm verify`; never widen their allow-list
  without a `needs-human` decision. AGPL/GPL solvers (wasm-postflop, TexasSolver) are never a
  dependency.
- Any new runtime dependency: say in the commit why it is needed, its license, and what it
  reads or writes. Prefer writing a small function over adding a crate or package.
- Engine crates: no dependency that pulls `std::time` or an unseeded RNG into results; WASM
  size budget stated in `docs/architecture.md`, and a change that grows the bundle more than
  10% says so.
- `next`, `react`, Node: re-run the free-tool prerender check and the CSP scenario.
- `drizzle-orm`, `pg`: re-run the isolation integration tests.
