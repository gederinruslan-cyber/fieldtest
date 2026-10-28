# F0-04 · reference-fixtures ∑

Done when: `engine/fixtures/` holds published equities and HRC/ICMIZER outputs with source and
date (values captured by the human, NH-2).

Human decision (NH-2, 2026-10-28): candidates §1–§4 approved. No engine math, so `/review` is
not needed. No dependency, crate or table.

## Shape
- `engine/fixtures/*.csv`, each with `#` lines for source, date and tolerance, then a header
  row: `seven-card-categories.csv`, `hu-equities.csv` (cards as `AhAs`), `icm.csv`,
  `hu-push-fold.csv`. `engine/fixtures/README.md` indexes them and states that a change needs
  a needs-human decision. Rows are extracted by script from the approved candidates file, not
  retyped.
- The 7-card evaluator test reads its counts from the fixture instead of a constant.

## Scenarios
1. **7-card counts from the fixture.** The exhaustive evaluator test passes against
   `seven-card-categories.csv` (exact), and fails if any count in the file is off by one.
2. **Fixtures are internally consistent (transcription guard).** Every HU equity row sums to
   100 within 0.02 pp (published rounding); every exact ICM row sums to the prize pool within
   1e-6 and each published value is within 1 of its exact value (the source truncates); push/fold rows cover 2–20 bb
   with and without ante, percentages in 0–100; the 7-card counts sum to 133,784,560.
3. **Sources stated.** Every fixture file starts with its source URL, retrieval date and
   tolerance.

Spec amendment: multiway ICM push/fold and bounty fixtures arrive with F1-03/F1-04 when the
human has captured them (NH-2 stays open, non-blocking).

## End-to-end check
`pnpm verify` green; `engine/fixtures/` listed in the README with all four files.
