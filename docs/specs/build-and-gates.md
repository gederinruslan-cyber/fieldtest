# Build and gates

*Behaviour of the repository's checks and local stack. Source: F0-01.*

## Gates
- `pnpm check` runs cargo fmt --check, clippy with warnings denied and cargo test over
  `engine/`, then Biome lint, typecheck and unit tests over every package; any failure exits
  non-zero.
- `pnpm verify` is check plus format check, license check, the engine's wasm32 build and the
  production build of every app.
- `pnpm verify:quick` is verify plus Postgres and Valkey from `compose.yaml` reporting healthy,
  then the integration tests. It is the ship gate.
- `pnpm verify:full` is verify:quick plus Playwright (desktop Chromium) against the production
  builds of web and api running on the host against the Compose services: the home page shows
  "Fieldtest" and the study-tool copy, and the api's `/health` answers ok.

## Licenses
- `pnpm licenses:check` refuses any engine crate or production npm package whose license is
  outside ADR 0003's allow-list (as amended by NH-4), naming the package; an SPDX `A OR B`
  passes when either side is allowed, `A AND B` needs both. Unpublished workspace crates are
  ignored. Optional LGPL dependencies are excluded from install, not allowed.

## Api
- `GET /health` answers 200 `{"status":"ok"}`.

## Local stack
- Postgres 18 on 127.0.0.1:25432 and Valkey 8 on 127.0.0.1:26379 (ADR 0004); web on 3200,
  api on 3201.

## Stop hook
- When the session has changed code (`.ts`, `.tsx`, `.js`, `.mjs`, `.cjs`, `.sql`, or engine
  `.rs`/`.toml`), the hook runs `pnpm check` and blocks the stop with the failure tail (exit
  2) when it is red; it finds cargo on a bare PATH (Homebrew's keg-only rustup,
  `~/.cargo/bin`).
