# Architecture

Stack per ADR 0001. Planned layout; F0-01 creates it.

```
engine/                 Cargo workspace — all poker math (ADR 0002)
  crates/core           cards, combos, ranges, evaluator, equity
  crates/icm            ICM, push/fold Nash, bounty models
  crates/sim            tournament rules, pools and bots, seeded simulation
  crates/grade          decision grading, leak aggregation
  crates/wasm           wasm-bindgen bindings for the browser
  crates/node           native bindings for api and worker (napi-rs)
  fixtures/             golden reference values with source and date
  deny.toml             license allow-list (ADR 0003)
packages/
  engine-wasm           wasm-pack output + typed TS wrapper + Web Worker pool
  ui                    design tokens and components
  db                    Drizzle schema, plain SQL migrations, withOwner()
  api-client            generated from the api's OpenAPI document
  ai-coach              the only path to an LLM; prompts, cap, number post-check
apps/
  web                   Next.js: static free-tool pages, simulator, account
  api                   Fastify: auth, pools, tournaments, imports, coach, billing
  worker                BullMQ: hand-history parsing, showcases, purges
e2e/                    Playwright
docs/                   features, specs, ADRs, plans, progress
```

## Where work runs

- **Browser (WASM):** calculators, range analyzer, the simulator's bot decisions, drills.
  Free-tier compute costs nothing on the server.
- **Server (native):** grading that must be trusted (stored results), imports, showcases,
  coach prompts. Same engine code, same results.

## Budgets

- WASM bundle: under 1.5 MB gzipped for the engine; lazy-loaded per tool.
- Range-vs-range flop equity: under 200 ms in the browser on a mid-range laptop.
- A 9-player sit-and-go against bots: under 5 s of compute to fast-forward to the end.
