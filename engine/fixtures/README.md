# Golden reference fixtures

Reference values the engine is checked against. Each file starts with `#` lines giving its
source, retrieval date, approval and tolerance, then a CSV header row.

**These values are human-approved. Changing one needs a `needs-human` decision; never edit a
fixture to make a test pass.** Approved under NH-2 on 2026-10-28.

| File | What | Tolerance | Used by |
|---|---|---|---|
| `seven-card-categories.csv` | 7-card hand category counts (Wikipedia) | exact | evaluator (F0-02) |
| `hu-equities.csv` | 26 heads-up preflop all-in equities (DeucesCracked, full enumeration) | ±0.005 pp | equity (F1-01) |
| `icm.csv` | Malmuth-Harville ICM (Wikipedia example, symmetry case) | 1e-9 of the pool | ICM (F1-02) |
| `hu-push-fold.csv` | Heads-up push/fold Nash percentages, 2–20 bb, no ante and 10% ante (PokerStrategy/HRC) | ±1 hand class | push/fold (F1-03) |

Still to capture (NH-2): multiway ICM push/fold spots and PKO / Mystery Bounty spots from HRC or
ICMIZER, added with F1-03 and F1-04.
