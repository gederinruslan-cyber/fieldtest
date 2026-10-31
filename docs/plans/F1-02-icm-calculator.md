# F1-02 · icm-calculator

Done when: Malmuth-Harville ICM, exact; equities sum to the prize pool; matches the
reference set.

## Engine
- **New crate** `engine/crates/icm` (`fieldtest-icm`), as the architecture map lays out; no
  dependencies. `fieldtest-wasm` depends on it.
- `icm(stacks: &[u64], payouts: &[f64]) -> Result<Vec<f64>, Error>`: each player's expected
  prize. Malmuth-Harville: a player takes the next place with probability stack / chips still
  in play. Dynamic programme over the set of players already placed (only sets smaller than
  the number of paid places), so 20 players with 20 places is 2^20 states.
- Chips are integers (`u64`); payouts and equities `f64`, never rounded in the engine.
- Errors: `InvalidStacks` (fewer than 2 or more than 20 players, or a stack of 0),
  `InvalidPayouts` (none, more places than players, negative or not finite).
- Budget: 20 players, 20 places under 50 ms in the browser.

## Scenarios
1. Reference: every `icm.csv` row matches `exact` within 1e-9 of the prize pool
   (50/30/20 chips, 70/30 → 45.178571…, 32.25, 22.571428…; Ganzfried & Sandholm
   300/12,300/900, 50/30/20 → 22.9603…, 48.1912…, 28.8485…).
2. Sum: for any 2–12 positive stacks and up to that many payouts (proptest), equities sum to
   the prize pool within 1e-9 of it.
3. Symmetry: equal stacks get equal equity; order of the input does not matter (a permuted
   input gives the permuted output).
4. Monotone: with non-increasing payouts, a bigger stack never has less equity.
5. Winner takes all: equity = pool × stack / total chips (ICM equals chip EV).
6. Errors: one player, 21 players, a 0 stack, no payouts, 4 payouts for 3 players, a
   negative payout.
7. Parity: ICM spots answer identically through WASM and natively.

## WASM and page
- `icm(stacks, payouts)` with numbers separated by spaces, commas or slashes; returns per
  player equity, share of the prize pool and share of the chips (the engine computes the
  shares; the page only formats).
- `/tools/icm`, static: player rows with a stack each (2–20, add and remove), a payouts field,
  Calculate. Table per player: stack, chip share, ICM equity (2 decimals) and prize share;
  caption "Exact: Malmuth-Harville ICM".

## End-to-end check
Playwright: stacks 50 / 30 / 20 with payouts 70, 30 show 45.18, 32.25 and 22.57 under an
"Exact" caption.
