# Engine in the browser (WASM)

*Behaviour of `engine/crates/wasm` and `packages/engine-wasm`. Source: F0-03.*

## Functions
- `normalizeRange(notation)` gives the canonical notation; `rangeSize(notation)` the combos
  counted at their weight (an empty range is 0, never -0); `evaluateHand(cards)` a number
  where bigger wins and equal ties; `handCategory(cards)` the category name, e.g.
  `StraightFlush`. Cards are written `As Ks Qs Js Ts`.
- `analysisLabels()`, `analyzeRange(range, board)` (live weight, then weight and share per
  class and per draw, in label order), `filterRange(range, board, keep)`,
  `gridWeights(range)` (169 values) and `setClass(range, class, weight)`; see
  `range-analyzer.md`. Calls take any number of typed arguments.
- `equity(players, board, samples, seed)`: ranges separated by `;`; gives 1 (exact) or 0
  (Monte Carlo), the deals or samples, then per player win, tie, equity and standard error;
  see `equity-calculator.md`.
- `icm(stacks, payouts)`: numbers separated by spaces, commas or slashes; gives per player
  the expected prize, its share of the prize pool and the player's share of the chips; see
  `icm-calculator.md`.
- Reference values: `rangeSize("QQ+,AKs:0.5")` = 20, `rangeSize("22+")` = 78,
  `normalizeRange("AKs,AQs,AJs,ATs")` = `ATs+`, the wheel ranks below `2c 3d 4h 5s 6c`.
- Bad input never throws past the bridge: it answers `{ ok: false, code }` with
  `InvalidToken`, `InvalidWeight`, `InvalidCard`, `InvalidHand` (anything other than 5 to 7
  distinct cards), `InvalidBoard`, `InvalidPlayers`, `NoValidDeal`, `InvalidSamples`, `InvalidStacks` or `InvalidPayouts`. Success is `{ ok: true, value }`.

## Parity
- The reference set (every range and hand in `engine-cards-and-ranges.md`, the invalid inputs,
  3,000 seeded 5- to 7-card hands, and the range-analyzer spots plus 300 seeded spots with
  card removal, 12 equity spots, exact and seeded Monte Carlo, and 11 ICM spots) gives identical answers and error codes through WASM
  and natively, compared exactly (including the sign of zero). It runs in `pnpm check`.

## Worker pool
- Engine calls run in module Web Workers, never on the main thread; the pool spreads calls
  round-robin over its workers (one per core, at most four) and each caller gets its own
  answer however the workers' replies interleave.

## Budget
- The gzipped `.wasm` is 46.6 KB against the 1.5 MB budget (F0-03).
