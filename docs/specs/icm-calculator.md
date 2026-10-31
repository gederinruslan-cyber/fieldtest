# ICM calculator

*Behaviour of `fieldtest-icm` and `/tools/icm`. Source: F1-02.*

## Engine
- `icm(stacks, payouts)`: Malmuth-Harville ICM for 2 to 20 players. Stacks are whole chips
  above 0; payouts run from first place down, at most one per player. Each player takes the
  next place with probability equal to their share of the chips still in play; equity is the
  payouts weighted by the chance of each finish. Exact up to `f64` rounding.
- Reference: every `icm.csv` row matches within 1e-9 of the prize pool (50/30/20 chips with
  70/30 → 45.178571…, 32.25, 22.571428…; 300/12,300/900 with 50/30/20 → 22.9603…, 48.1912…,
  28.8485…; four equal stacks share the pool equally).
- Equities sum to the prize pool; equal stacks get equal equity; the input order does not
  matter; with non-increasing payouts a bigger stack never has less equity; winner takes all
  gives pool × stack / total chips.
- Refused: `InvalidStacks` (fewer than 2 or more than 20 players, a stack of 0, or chips that
  overflow), `InvalidPayouts` (none, more than the players, negative or not finite).

## Page `/tools/icm`
- Static page with a client calculator: one stack per player row (2–20, Add and Remove
  player), a payouts field (numbers from first place down), Calculate. A separator inside a
  stack row is refused.
- Results per player: equity to 2 decimals, prize share and chip share to 2 decimals, chips.
  The table is named "Exact: Malmuth-Harville ICM, every finishing order counted".
