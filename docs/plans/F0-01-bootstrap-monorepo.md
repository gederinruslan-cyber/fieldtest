# F0-01 · bootstrap-monorepo

Done when: pnpm + Turborepo with empty `apps/web`, `apps/api`, `apps/worker`, `packages/ui`;
Cargo workspace `engine/`; the four `pnpm` gates and `licenses:check` exist and pass; `cargo
deny` configured to ADR 0003; Compose with Postgres and Redis; the Stop hook runs.

No engine math, no table, no import path, no auth, so `/review` is not needed.

## Shape
- Root: `pnpm-workspace.yaml` (apps/*, packages/*, e2e), `turbo.json` (build, typecheck,
  test, test:integration), `tsconfig.base.json`, Biome for lint and format (one MIT dev
  tool in place of ESLint plus Prettier), `.nvmrc` 26, `.env.example`.
- `engine/`: workspace with one empty crate `crates/core` (`publish = false`); `deny.toml`
  allows ADR 0003's list (plus 0BSD and CC-BY-4.0 data, NH-4) and ignores private crates.
- `apps/web` Next.js 16 static home page; `apps/api` Fastify 5 with `GET /health`;
  `apps/worker` BullMQ worker with no job types yet; `packages/ui` empty.
- `compose.yaml`: Postgres 18 on 25432 and Valkey 8 on 26379 (ADR 0004, since Redis 8 is
  AGPL), both with healthchecks. Ports avoid the other local stacks on 3000/3001/5432/6379;
  web listens on 3200 and api on 3201.
- New dependencies (all MIT unless noted): turbo, typescript, @biomejs/biome, vitest, next,
  react, react-dom, fastify, bullmq, @playwright/test (Apache-2.0), @types/*.

## Scenarios
1. **Gates exist.** Given a fresh clone with Node 26, pnpm 11 and stable Rust, when I run
   `pnpm check`, then cargo fmt --check, clippy -D warnings, cargo test, biome lint,
   typecheck and unit tests all run and exit 0.
2. **License gate (isolation case).** Given `engine/Cargo.toml` depends on a GPL-3.0 crate,
   when I run `pnpm licenses:check`, then it exits non-zero naming the crate; the same holds
   for a production npm dependency whose license is outside the allow-list. With only
   today's dependencies it exits 0.
3. **Api health.** Given the api app, when `GET /health` is injected, then it answers 200
   `{"status":"ok"}` (Vitest, Fastify inject).
4. **Compose.** Given Docker, when `pnpm verify:quick` runs, then Postgres and Valkey report
   healthy (`docker compose up --wait`) and the integration task runs (none exist yet).
5. **Stack starts (e2e).** Given the production builds, when `pnpm verify:full` runs, then
   Playwright (desktop Chromium) loads the home page, which shows "Fieldtest" and study-tool
   copy, and the api's `/health` answers ok.
6. **Stop hook (edge case).** Given a changed `.ts` file, when the hook runs, then it runs
   `pnpm check` (finding cargo even when only Homebrew's keg-only rustup is installed) and
   exits 2 with the failure tail on a lint error, or 0 when the check is green.

Spec amendment: until the app containers exist (deploy work), `verify:full` runs the
production builds of web and api on the host against the Compose services.

## End-to-end check
`pnpm verify:full` green on the branch: check, format, licenses, wasm build (`cargo build
--target wasm32-unknown-unknown` for the engine until F0-03 adds wasm-pack), build,
Compose healthy, e2e smoke passes.
