# NH-2 · Reference value candidates

*Found on 2026-10-28 from published sources. These are **candidates**: a human approves them
before they become golden fixtures in `engine/fixtures/` (F0-04). "Checked" means Claude
recomputed the value with an independent throwaway script (JS, not the engine); it is not a
second external source.*

## 1. Seven-card hand categories (F0-02)

Source: [Wikipedia, "Poker probability", 7-card hands](https://en.wikipedia.org/wiki/Poker_probability)
(standard combinatorics). Checked: the counts sum to C(52,7) = 133,784,560.

| Category | Hands |
|---|---|
| Royal flush | 4,324 |
| Straight flush (not royal) | 37,260 |
| Four of a kind | 224,848 |
| Full house | 3,473,184 |
| Flush | 4,047,644 |
| Straight | 6,180,020 |
| Three of a kind | 6,461,620 |
| Two pair | 31,433,400 |
| One pair | 58,627,800 |
| High card | 23,294,460 |

## 2. Heads-up preflop all-in equities (F1-01)

Source: [DeucesCracked, "Preflop equity matchups: exact data"](https://www.deucescracked.com/blog/preflop-poker-equity-matchups-exact-data),
2026-10-12, full enumeration of 1,712,304 boards. **Checked: all 26 rows reproduced exactly
by full enumeration (2026-10-28).** Tolerance proposal: ±0.005 percentage points (exact).

| Hand 1 | Hand 2 | Win 1 % | Win 2 % | Tie % |
|---|---|---|---|---|
| A♥A♠ | K♦K♣ | 81.06 | 18.55 | 0.38 |
| A♥A♠ | K♥K♠ | 82.36 | 17.09 | 0.54 |
| Q♥Q♠ | J♦J♣ | 81.10 | 18.54 | 0.36 |
| A♥A♠ | A♦K♦ | 87.23 | 11.51 | 1.26 |
| A♥A♠ | A♦K♣ | 91.95 | 6.80 | 1.25 |
| K♥K♠ | A♦K♣ | 69.61 | 29.60 | 0.79 |
| A♥K♣ | A♦Q♣ | 72.35 | 22.97 | 4.68 |
| A♥K♥ | A♦Q♦ | 69.16 | 26.61 | 4.23 |
| A♥K♣ | K♦Q♣ | 74.30 | 24.53 | 1.17 |
| Q♥Q♠ | A♦K♦ | 53.59 | 46.02 | 0.39 |
| Q♥Q♠ | A♦K♣ | 56.99 | 42.66 | 0.34 |
| 8♥8♠ | A♦K♣ | 55.42 | 44.28 | 0.30 |
| T♥T♠ | A♦Q♣ | 57.20 | 42.50 | 0.30 |
| 2♥2♠ | A♦K♣ | 52.75 | 46.67 | 0.58 |
| 2♥2♠ | A♦K♦ | 49.60 | 49.77 | 0.63 |
| K♥Q♣ | 2♦2♣ | 48.38 | 50.73 | 0.90 |
| A♥A♠ | 7♦2♣ | 87.24 | 12.40 | 0.36 |
| A♥A♠ | 7♦6♦ | 76.81 | 22.87 | 0.32 |
| 9♥8♥ | A♦A♣ | 22.47 | 77.22 | 0.30 |
| A♥K♣ | 3♦2♣ | 65.68 | 33.74 | 0.58 |
| J♥T♥ | A♦K♣ | 40.97 | 58.61 | 0.42 |
| 7♥6♥ | A♦K♣ | 42.06 | 57.55 | 0.38 |
| A♥K♣ | 8♦7♦ | 57.91 | 41.71 | 0.38 |
| 9♥9♠ | 8♦7♦ | 80.77 | 18.64 | 0.58 |
| A♥J♣ | K♦Q♦ | 55.62 | 43.94 | 0.44 |
| A♥5♥ | K♦Q♣ | 60.38 | 39.16 | 0.45 |

## 3. ICM, Malmuth-Harville (F1-02)

Published examples are rounded; exact values are Claude's recomputation. Tolerance proposal:
1e-9 of the prize pool against the exact value.

| Case | Stacks | Payouts | Published | Exact |
|---|---|---|---|---|
| [Wikipedia, "Independent Chip Model"](https://en.wikipedia.org/wiki/Independent_Chip_Model) | 50 / 30 / 20 | 70 / 30 | ≈45 / 32 / 22 | 45.178571 / 32.250000 / 22.571429 |
| VIP Grinders ICM guide (page blocked our fetch; numbers from search snippet) | 50 / 30 / 20 | 50 / 30 / 20 | shortest stack gets more than its chip share | 38.392857 / 32.750000 / 28.857143 |
| Pokerology ICM lesson (page blocked our fetch) | 4,500 / 2,700 / 1,800 | 84 / 36 | not retrieved | 54.214286 / 38.700000 / 27.085714 |
| Symmetry property | 4 × 2,500 | 50 / 30 / 20 | — | 25 each |

Rejected: esports.gg's 6,000/3,000/1,000 example ($4,400 / $3,200) does not match
Malmuth-Harville (exact: 4,123.81 / 3,383.33 / 2,492.86).

## 4. Heads-up push/fold Nash, chip EV (F1-03)

Source: [PokerStrategy, "Nash Ranges for the Push-or-Fold Play"](https://www.pokerstrategy.com/strategy/sit-and-go/1779/),
updated 2026-03-11, computed with HoldemResources' calculator. Percent of hands; the full hand
lists are on the page. The article itself warns calculators differ by about 0.5% or one hand,
so tolerance proposal: ±1 hand class. **Not checked** (needs the engine's Nash solver).

| Eff. stack (bb) | SB push, no ante | BB call, no ante | SB push, 10% ante | BB call, 10% ante |
|---|---|---|---|---|
| 2 | 94.6 | 96.4 | 98.2 | 100.0 |
| 4 | 78.3 | 82.2 | 84.9 | 89.4 |
| 6 | 70.1 | 59.6 | 71.9 | 65.6 |
| 8 | 62.6 | 48.4 | 66.2 | 50.8 |
| 10 | 56.6 | 38.5 | 60.8 | 43.0 |
| 12 | 51.4 | 33.0 | 54.4 | 36.3 |
| 15 | 45.7 | 28.4 | 49.0 | 30.6 |
| 20 | 40.9 | 21.7 | 44.5 | 23.8 |

(Every stack from 1 to 20 bb is on the page.)

## Still needs a human run

- **Multiway push/fold with ICM** (bubble spots, 3–6 players, real payouts): no trustworthy
  published table found. Run about 10 spots through HRC's free Nash tool or ICMIZER and save
  the outputs.
- **PKO and Mystery Bounty** adjustments (F1-04): no published reference values; ICMIZER is the
  usual tool.
- Copy the exact hand lists for the HU Nash rows from the PokerStrategy page if per-hand
  tests are wanted.
