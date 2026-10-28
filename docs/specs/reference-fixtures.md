# Reference fixtures

*Behaviour of `engine/fixtures/`. Source: F0-04 (values approved by the human under NH-2 §1–§7,
2026-10-28).*

- Every fixture file states its source, retrieval date, approval and tolerance before a CSV
  header row; `engine/fixtures/README.md` lists them. A value changes only through a
  `needs-human` decision, never to make a test pass.
- `seven-card-categories.csv`: the 7-card category counts, exact; the evaluator's exhaustive
  test reads it, and a count off by one fails.
- `hu-equities.csv`: 26 heads-up preflop equities (percent, ±0.005 pp); each row sums to 100
  within 0.02.
- `icm.csv`: Malmuth-Harville ICM, exact values at full precision (tolerance 1e-9 of the pool)
  and the sources' printed approximations (within 1): Wikipedia's 50/30/20 example, a symmetry
  case, and Ganzfried & Sandholm 2008 (stacks 300/12,300/900, 50/30/20 → 22.96 / 48.19 /
  28.848); each exact row sums to the pool.
- `hu-push-fold.csv`: heads-up push/fold Nash percentages for 2, 4, 6, 8, 10, 12, 15 and 20 bb,
  with no ante and a 10% ante (±1 hand class).
- `three-player-push-fold.csv`: six 3-player Nash ranges (push, call, overcall) for BU 3,200 ·
  SB 5,000 · BB 5,000 at 200/400, winner takes all (±1 hand class).
- `bounty.csv`: PKO required equity with and without a bounty (31.4% / 44.4%) and a mystery
  bounty's average value (19 bb), each matching its exact value within the printed rounding.
- Multiway push/fold with ICM pressure and solver bounty calling ranges are not captured yet
  (NH-2).
