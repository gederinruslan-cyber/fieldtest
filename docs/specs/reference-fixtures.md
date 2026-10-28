# Reference fixtures

*Behaviour of `engine/fixtures/`. Source: F0-04 (values approved by the human under NH-2,
2026-10-28).*

- Every fixture file states its source, retrieval date, approval and tolerance before a CSV
  header row; `engine/fixtures/README.md` lists them. A value changes only through a
  `needs-human` decision, never to make a test pass.
- `seven-card-categories.csv`: the 7-card category counts, exact; the evaluator's exhaustive
  test reads it, and a count off by one fails.
- `hu-equities.csv`: 26 heads-up preflop equities (percent, ±0.005 pp); each row sums to 100
  within 0.02.
- `icm.csv`: Malmuth-Harville ICM, exact values at full precision (tolerance 1e-9 of the pool)
  and the source's whole-number approximations (within 1); each exact row sums to the pool.
- `hu-push-fold.csv`: heads-up push/fold Nash percentages for 2, 4, 6, 8, 10, 12, 15 and 20 bb,
  with no ante and a 10% ante (±1 hand class).
- Multiway ICM push/fold and bounty fixtures are not captured yet (NH-2); F1-03 and F1-04 add
  them.
