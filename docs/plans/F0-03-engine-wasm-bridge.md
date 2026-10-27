# F0-03 · engine-wasm-bridge ∑

Done when: `packages/engine-wasm` built by wasm-pack, called from a Web Worker pool with typed
TS; the reference set gives identical results natively and in WASM.

Adds the engine's first exported functions, so `/review` runs once. New engine crate
`crates/wasm` (thin bindings, no math) and new dependency `wasm-bindgen` (MIT OR Apache-2.0;
reads and writes nothing, generates the JS glue). New tool: wasm-pack (MIT OR Apache-2.0),
installed like cargo-deny. No table, no cross-module call. Core gains one accessor,
`HandRank::value()`.

## Shape
- `engine/crates/wasm`: `normalizeRange(notation) → string`, `rangeSize(notation) → number`
  (combos counted at their weight), `evaluateHand("As Ks Qs Js Ts") → number` (bigger wins),
  `handCategory(cards) → string`. Each returns `Result<_, String>` with an error code
  (`InvalidToken`, `InvalidWeight`, `InvalidCard`, `InvalidHand` for anything other than 5–7
  distinct cards); nothing panics across the boundary. `examples/reference.rs` runs the same
  functions natively over lines `fn<TAB>arg` from stdin.
- `packages/engine-wasm`: `pkg/` from `wasm-pack build --target web` (gitignored);
  `handle.ts` maps a typed call to `{ ok: true, value } | { ok: false, code }`; `worker.ts`
  runs it in a Web Worker; `pool.ts` round-robins calls over N workers and matches answers
  by id. Its turbo `build` is the wasm-pack build (inputs: `engine/crates/**`), and its
  `typecheck` and `test` depend on it, so `pnpm check` builds the WASM first. `verify` drops
  the separate wasm32 cargo build.

## Scenarios
1. **Parity (reference set).** Given the reference inputs (every range in
   `docs/specs/engine-cards-and-ranges.md`, the invalid notations, the reference hands, invalid
   hands, and 3,000 seeded 5-, 6- and 7-card hands), when they run through the WASM build in
   Node and through the native example, then every answer and error code is identical
   (tolerance: exact, including `f64` bits).
2. **Reference values through WASM.** `rangeSize("QQ+,AKs:0.5")` = 20, `rangeSize("22+")` =
   78, `normalizeRange("AKs,AQs,AJs,ATs")` = `ATs+`, `handCategory("As Ks Qs Js Ts")` =
   `StraightFlush`, `evaluateHand` of the wheel < of `2c 3d 4h 5s 6c`.
3. **Error codes, never a throw (edge case).** `normalizeRange("AKx")` → `{ ok: false, code:
   "InvalidToken" }`; `evaluateHand("As Ks")` and `"As As Kd Qc Jh"` → `InvalidHand`;
   `"As Ks Qs Js 1s"` → `InvalidCard`.
4. **Pool.** Given 2 workers that answer out of order, when 5 calls are made, then each
   caller gets its own answer and the calls alternate between workers.
5. **Budget.** The gzipped `.wasm` stays under 1.5 MB (architecture budget); the size is
   reported in the plan's outcome, no new check.

## End-to-end check
`pnpm verify` green from a clean `pkg/`: wasm-pack build, parity test, pool test, license
check (wasm-bindgen included). The real browser Worker is first exercised by F1-01's page.
