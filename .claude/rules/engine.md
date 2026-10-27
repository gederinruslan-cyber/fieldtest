---
paths:
  - "engine/**"
  - "packages/engine-wasm/**"
---

# Engine rules (apply when touching these files)

- The engine is the only implementation of poker math. A TS caller that needs a number adds
  an engine function and calls it through `packages/engine-wasm` (browser) or the native
  binding (api, worker); it never computes it itself.
- Cards, ranges and combos use the engine's canonical types; 1,326 starting combos, card
  removal applied before every count. Range notation (`22+`, `ATs+`, weights) parses and
  prints round-trip.
- Exact where the math allows: hand-vs-range and range-vs-range equity enumerate when the
  combo count fits the budget; ICM (Malmuth-Harville first) and push/fold Nash are exact.
  Monte Carlo returns its sample size and standard error with the result.
- Chips are `u64`/`i64`; never `f64` for a stack, blind or pot. Probabilities are `f64`; no
  rounding inside the engine.
- Every random source takes an explicit seed (`rand_chacha`); no thread-local or time-based
  RNG. Iteration order is deterministic (no `HashMap` iteration in results).
- Every public engine function has: a property test (`proptest`) for its invariants, and a
  golden test against `engine/fixtures/` with a stated tolerance. Equities sum to 1 per
  matchup; ICM equities sum to the prize pool; symmetric spots give symmetric results.
- `engine/fixtures/` holds reference values with their source (published equity, HRC or
  ICMIZER output, date). Changing a value needs a `needs-human` decision.
- Native and WASM agree: the reference set runs through both in `pnpm verify`.
- No `unsafe` and no panics across the WASM boundary; return `Result` with an error code.
- Performance budgets live with the function (e.g. a range-vs-range flop equity under 200 ms
  in the browser); a change that misses one says so in the plan.
