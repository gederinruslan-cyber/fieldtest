# Features

*The one list of what to build, in order, with what "done" means. `/next` takes the first
`ready` row whose dependencies are `done`. Statuses: `todo` · `ready` · `in-progress` · `done`
· `later`. `pnpm ship` marks a row `done` with the merged SHA. Product background:
[`product-plan.md`](product-plan.md). Hard rules live in `CLAUDE.md` and `.claude/rules/`.*

**Marks:** `∑` engine math, `🔒` user data or auth, `⚑` poker-site policy (imports, pools,
anything near a live table). They say where to be careful and when `/review` is worth its
cost. Everything merges on a green `verify:quick` gate.

## Milestones

| Milestone | Exit criteria | State |
|---|---|---|
| **M0 · Foundation** | Monorepo, Rust engine workspace, WASM bridge, checks and ship script, reference fixtures | open |
| **M1 · Free tools** | Equity, ICM, push/fold and bounty calculators plus the range analyzer, public and matching HRC/ICMIZER on the reference set | |
| **M2 · Core simulator** | Accounts; sit-and-gos then MTTs against configurable pools; exact ICM grading; leak report; trial and paywall | |
| **M3 · Retention** | Drills from your own mistakes; capped AI coach; PokerStars import; GG upload behind a flag | |
| **M4 · Growth** | Coach seats (after interviews), pools from imported hands, showcases, affiliates | |

## M0 · Foundation

| ID | Feature | Status | Depends | Done when |
|---|---|---|---|---|
| F0-01 | bootstrap-monorepo | done `79cb5d5` | — | pnpm + Turborepo with empty `apps/web` (Next.js), `apps/api` (Fastify), `apps/worker` (BullMQ), `packages/ui`; Cargo workspace `engine/`; `pnpm check`, `verify`, `verify:quick`, `verify:full` and `licenses:check` exist and pass; `cargo deny` configured to ADR 0003; Compose with Postgres and Redis; the Stop hook runs |
| F0-02 | engine-cards-and-eval ∑ | done `bbb9498` | F0-01 | Card, combo and range types; range notation parses and prints round-trip; 7-card evaluator gives the exact category counts over all 133,784,560 hands |
| F0-03 | engine-wasm-bridge ∑ | todo | F0-02 | `packages/engine-wasm` built by wasm-pack, called from a Web Worker pool with typed TS; the reference set gives identical results natively and in WASM |
| F0-04 | reference-fixtures ∑ | todo | F0-02 | `engine/fixtures/` holds published equities and HRC/ICMIZER outputs with source and date (values captured by the human, NH-2) |

## M1 · Free tools

| ID | Feature | Status | Depends | Done when |
|---|---|---|---|---|
| F1-01 | equity-calculator ∑ | todo | F0-03, F0-04 | Exact hand-vs-range and range-vs-range enumeration, Monte Carlo with stated error, up to 9 players; matches the reference equities; static page |
| F1-02 | icm-calculator ∑ | todo | F0-03, F0-04 | Malmuth-Harville ICM, exact; equities sum to the prize pool; matches the reference set |
| F1-03 | push-fold-nash ∑ | todo | F1-02 | Heads-up then multiway push/fold Nash with ICM; matches HRC's free Nash tool on the reference set; unlimited and free |
| F1-04 | bounty-calculators ∑ | todo | F1-03 | PKO and Mystery Bounty adjustments to calling ranges, assumptions stated on the page |
| F1-05 | range-analyzer | todo | F0-03 | 13×13 grid, board picker, made-hand and draw breakdown with combo counts, filters carried to the next street |
| F1-06 | range-advantage-and-textures ∑ | todo | F1-05 | Range and nut advantage per board; texture report across all 1,755 flops |
| F1-07 | free-tools-public | todo | F1-01…F1-06 | Static pages, sitemap, titles; deploy is a human checkpoint |

## M2 · Core simulator

| ID | Feature | Status | Depends | Done when |
|---|---|---|---|---|
| F2-01 | accounts-and-auth 🔒 | todo | F0-01 | Email sign-up/in, sessions, owner-scoped data with the isolation test |
| F2-02 | tournament-engine ∑ | todo | F0-02 | Sit-and-go rules, blinds and antes, payouts; a seed replays identically |
| F2-03 | pool-presets-and-sliders ⚑ | todo | F2-02 | Presets (Fish, Nit, Maniac, Calling Station, TAG) and sliders; pools never exportable |
| F2-04 | simulator-table | todo | F2-03, F0-03 | Play a full sit-and-go in the browser against a pool |
| F2-05 | mtt-fields ∑ | todo | F2-04 | 9 to several hundred players, table balancing, late registration, fast-forward to bubble or final table |
| F2-06 | icm-grading ∑ | todo | F1-03, F2-04 | Preflop and all-in decisions graded exactly; postflop flagged as estimates |
| F2-07 | leak-report | todo | F2-06 | Report by position, stage and spot after each tournament |
| F2-08 | trial-and-paywall 🔒 | todo | F2-01 | 14–30 day full trial, Pro subscription; payment provider is a human checkpoint |

## M3 · Retention

| ID | Feature | Status | Depends | Done when |
|---|---|---|---|---|
| F3-01 | estimate-then-reveal-drills ∑ | todo | F1-05 | Estimate, combo-counting and range-reading drills graded by the engine |
| F3-02 | mistake-driven-drills | todo | F2-06, F3-01 | Mistakes from tournaments become spaced-repetition drills |
| F3-03 | ai-coach 🔒 | todo | F2-06 | Haiku-default explanations citing only engine numbers, monthly cap, works with AI off |
| F3-04 | pokerstars-import 🔒⚑ | todo | F2-01 | Upload PokerStars files after a session; names hashed; leak stats |
| F3-05 | ggpoker-upload ⚑ | todo | F3-04 | Manual PokerCraft upload behind `import.ggpoker`; policy re-checked after 2026-12-01 (human) |

## M4 · Growth

| ID | Feature | Status | Depends | Done when |
|---|---|---|---|---|
| F4-01 | coach-seats 🔒 | later | F2-08 | Built only after 5–10 coach interviews |
| F4-02 | pools-from-imported-hands ⚑ | later | F3-04 | Pool profile suggested from a user's own imported hands |
| F4-03 | showcases ⚑ | later | F2-05 | Server-side pool runs published as content |
