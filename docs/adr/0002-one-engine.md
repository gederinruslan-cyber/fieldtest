# 0002 · One engine for all poker math

- **Status:** accepted · 2026-10-28
- **Context:** The product's promise is that every number is right and that the AI only
  explains numbers. Two implementations (say, a TS helper for display and the Rust engine)
  would eventually disagree.
- **Decision:** All evaluation, equity, ICM, Nash, bounty, simulation and grading code lives
  in `engine/`. Every result carries an exact/estimate flag; Monte Carlo carries sample size
  and error. Golden fixtures with sources pin the results; changing a fixture is a human
  decision.
- **Consequences:** Any new number on screen starts as an engine function with property and
  golden tests. TS formats numbers; it never computes them.
