# Product plan

The full plan lives in the Claude Doc
[Fieldtest — Product Features](https://claude.ai/artifact/5Yno9xms5U23uVwuU3geBk). The
competitor research behind it is in `~/git/reports/Poker training app landscape.md`
(2026-10-27). This file holds only what the code depends on.

## Thesis

A modern browser tournament simulator with opponent pools the user configures, exact ICM
grading of preflop and all-in decisions, and an AI coach that only explains engine numbers. No
competitor combines these: APT simulates tournaments but lets users pick only bot type and
level, BluffCatcher is Apple-only, GTO Wizard drills single spots.

## Decisions that shape the code

- Free calculators and the range analyzer are the acquisition funnel; they run fully in the
  browser and render statically for search. Long-tail edge: unlimited free push/fold with PKO
  and Mystery Bounty.
- Grading is exact only where the math is exact (preflop, push/fold, ICM); postflop is an
  estimate and says so.
- PokerStars import first; GGPoker by manual upload behind a flag (GG third-party ban from
  2026-12-01).
- No public bot arena, no bot export, no bot API; showcases are content we publish.
- Pricing hypothesis: Free; Pro $15–25/month with a 14–30 day full trial; Coach per seat,
  validated with coaches before it is built.
- Web-first; native app stores later, if ever (simulated-gambling review risk).
