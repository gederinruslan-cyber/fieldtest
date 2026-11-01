# 0005 · Push/fold Nash: exact heads-up table, multiway split out

- **Status:** accepted · 2026-10-31
- **Context:** Push/fold Nash weighs every all-in outcome (win, tie and loss, kept apart because
  ICM is not linear) for every pair of the 169 hand classes. Enumerating a preflop matchup
  takes about 0.1 s natively, so a solver can't enumerate as it goes. Three-way all-ins need
  class triples: about 800,000 of them, each over up to 4,096 combo triples and 1.37 million
  boards. That is out of reach both at run time and as a one-off build.
- **Decision:**
  - **Heads-up table.** The heads-up win and tie counts for every ordered class pair are
    enumerated once by `examples/hu_table.rs`, using the engine's own `exact` equity and
    treating suit relabellings as the same matchup. The counts are committed as
    `crates/icm/data/hu-preflop.bin` (169 × 169 × 2 little-endian `u32`, 228 KB) and compiled
    in. Card removal between the two hands is exact.
  - **Solver.** Fictitious play: each side best-responds to the other's average strategy.
    Terminal values are the ICM of every stack at the table. The result states its
    exploitability ε: the most either player could gain by deviating.
  - **Scope.** F1-03 covers small blind versus big blind, with any other players at the table
    folded and still counted in ICM. Multiway (three or more players acting) moves to its own
    row, which must first decide how to get three-way equities: a labelled Monte Carlo
    estimate, or a larger table.
- **Alternatives:**
  - Monte Carlo equities inside the solver: noisy and slower, and an estimate where exact is
    possible.
  - A runtime 169 × 169 enumeration: about 30 minutes.
  - Pairwise approximations of three-way equity: no stated error.
- **Consequences:**
  - The WASM bundle grows by about 230 KB.
  - Regenerating the table is deterministic. A test recomputes sampled entries from `exact`,
    so a stale table fails `pnpm check`.
  - The three-player fixture (`three-player-push-fold.csv`) waits for the multiway row.
