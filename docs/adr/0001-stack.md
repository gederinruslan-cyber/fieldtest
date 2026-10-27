# 0001 · Stack: WholeMoney's TypeScript stack plus a Rust engine

- **Status:** accepted · 2026-10-28
- **Context:** Solo developer working with Claude Code, who already runs this stack in
  WholeMoney. The product needs fast poker math in the browser (free tools, bots) and the same
  math on the server (trusted grading, imports, showcases).
- **Decision:** pnpm + Turborepo monorepo; Next.js web, Fastify api, BullMQ worker, Postgres
  with Drizzle and plain SQL migrations, Redis. Poker math in a Rust Cargo workspace
  (`engine/`), compiled to WASM for the browser and to a native Node addon for the server.
- **Alternatives:** Next.js-only with route handlers (fewer processes, but imports and
  showcases need a worker anyway); a TypeScript engine (simpler build, but too slow for
  range-vs-range enumeration and large-field simulation).
- **Consequences:** two toolchains in `pnpm check`; the WASM and native builds must give the
  same results, which the reference set enforces.
