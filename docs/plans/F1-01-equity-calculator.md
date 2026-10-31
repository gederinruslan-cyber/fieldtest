# F1-01 · equity-calculator

Done when: exact hand-vs-range and range-vs-range enumeration, Monte Carlo with stated error,
up to 9 players; matches the reference equities; static page.

## Engine (`fieldtest-core::equity`)
- `exact(ranges, board) -> Option<Equity>`: every deal (one live combo per player, no shared
  card, weight = product of weights) × every board completion. `None` when the work
  (completions × (Σ live combos + Π live combos)) is over the budget.
- `monte_carlo(ranges, board, samples, seed)`: rejection-samples the same deal distribution
  (ChaCha8, explicit seed); per player the standard error of the mean pot share.
- `equity(...)`: exact when within budget, else Monte Carlo. Per player: win (sole winner),
  tie (splits), equity (win + split shares), std error (0 when exact); plus method and count
  (deals enumerated or samples).
- Errors: `InvalidPlayers` (not 2–9 players, or an empty range), `InvalidBoard` (not 0, 3, 4
  or 5 distinct cards), `NoValidDeal` (no deal without shared cards).
- **New dependency:** `rand_chacha` (MIT/Apache-2.0, default features off) — the seeded RNG
  the engine rules name.
- Budget: every call under 0.5 s in the browser (measured in WASM: exact worst 0.34 s, 9-way
  Monte Carlo at 200,000 samples 0.37 s). The rules' example "range-vs-range flop under 200
  ms" is missed narrowly: 190 vs 190 combos on a flop is exact in 205 ms. Spec gap, one line:
  preflop "exact hand-vs-range" holds for small ranges (AhAs vs KK); wider preflop ranges
  are sampled and labelled.

## Scenarios
1. Reference: given each of the 26 `hu-equities.csv` matchups preflop, when computed exactly,
   then win1, win2 and tie match within ±0.005 pp over 1,712,304 deals (AhAs vs KdKc 81.06 /
   18.55 / 0.38).
2. Sum: for any 2–4 ranges and board (proptest), equities sum to 1 and win + tie ≤ 1, exact
   and Monte Carlo.
3. Symmetry: the same range for both players on any board gives 0.5 each.
4. Card removal: AhAs vs "AA" deals only the one ace pair left (AdAc), so equity is 0.5.
5. Monte Carlo: AhAs vs KdKc with 200,000 samples is within 4 standard errors of 81.25%
   (exact equity), the same seed gives the same numbers, the standard error ≈ 0.09 pp.
6. Multiway: 9 single hands preflop compute exactly (278,256 boards); AhAs vs KdKc vs QhQs
   matches its Monte Carlo within 4 standard errors.
7. Errors: one player, ten players, `AhAs` vs `AhKd` (NoValidDeal), board `Ah 7c`.
8. Parity: the equity spots (exact and seeded Monte Carlo) answer identically through WASM.

## Page `/tools/equity` (static, client component)
- 2–9 player rows, each a range or hand; a board text field; Calculate.
- Result table per player: equity, win, tie to 2 decimals; caption "Exact: N deals
  enumerated" or "Monte Carlo: N sampled deals (seed 1), ± is one standard error" with a ±
  column in percentage points.

## End-to-end check
Playwright: AhAs vs KdKc preflop shows 81.25% equity and an "Exact" caption; AA vs KK shows a
"Monte Carlo" caption with a ± column.
