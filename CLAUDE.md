# Fieldtest — tournament poker practice against your own field

Web app where a player configures an opponent pool, plays full tournaments against it in the
browser and gets ICM-graded feedback in plain language. Free calculators and a range analyzer
bring players in; the simulator is the product. Study tool only: never used at a real table.
("Fieldtest" is the chosen name; domain and trademark checks are open.)

Read `docs/features.md` for what to build next. Product background: `docs/product-plan.md`
(links the full plan and the competitor research). Stack and module map:
`docs/architecture.md`. Binding decisions: `docs/adr/`. Behaviour specs, kept current after
each merge: `docs/specs/`. Path-scoped rules for the engine, table safety, the AI coach, data,
web and dependencies load from `.claude/rules/` when you touch those files.

## Invariants (never bend these)

- **One engine.** All poker math (evaluation, equity, ICM, Nash, bounties, simulation,
  grading) lives in the Rust workspace `engine/`. Browser and server run the same code (WASM
  and native). TypeScript never reimplements it, not even "just for display".
- **Every number is exact or labelled.** Enumeration, ICM and push/fold Nash are exact.
  Monte Carlo states its sample size and error. Postflop grading is shown as an estimate.
  Nothing approximate is presented as exact.
- **The engine is checked against references.** Known equities and HRC/ICMIZER outputs are
  golden fixtures with a stated tolerance; a fixture change is a reviewed decision, never a
  fix to make a test pass.
- **Simulations are deterministic.** A tournament is its config plus a seed; the same seed
  replays the same hands everywhere (replays, bug reports, showcases).
- **Chips are integers.** Stacks, blinds and pots are integer chips; probabilities and ICM
  equities are `f64` and are rounded once, at display.
- **Study away from the table.** No overlays, no screen reading, no watching poker-site
  folders, no live hand ingestion, no bot export, no bot API. Hand histories arrive only as
  files a user uploads after a session.
- **No gambling.** Play money only: no purchasable chips, no prizes of value, no cash-out.
- **AI never invents numbers.** The coach receives engine output and may only cite numbers
  present in it; it goes only through `packages/ai-coach` with a small default model, a
  per-user cap and a fallback. The app works with AI off.
- **Permissive licenses only.** MIT, Apache-2.0, BSD, ISC, Zlib. No AGPL or GPL code in
  anything we ship or host (ADR 0003).
- **User isolation below HTTP.** Every query is scoped to the user (or coach team) in the
  data layer; another user's id answers 404.
- **Privacy.** Hand histories are personal data. Opponent names are hashed on import and never
  published. No hand data, emails or tokens in logs. Export and delete always available.
  Synthetic fixtures only.

## How work flows

One session takes one feature from `docs/features.md` end to end:

1. **Plan** in plan mode: read the feature's "done when", the related specs and ADRs, and the
   code you will touch. Write the plan as scenarios (Given/When/Then with real numbers: a
   reference spot, the isolation case, the edge case) plus the end-to-end check that proves
   it. Under 60 lines. If the plan needs a decision with a real alternative, write a short ADR
   first.
2. **Build** on a `change/<name>` branch, test-first for new behaviour, smallest change that
   satisfies the scenarios. Commit per coherent step with explicit paths. Run `pnpm check`
   often and `pnpm verify` before pushing.
3. **Second opinion, when it is cheap insurance.** `/review` runs a fresh-context reviewer over
   the diff with two questions: can a user see a wrong number, and does anything cross a
   boundary (another user's data, a live table, gambling). Use it the first time a feature
   adds engine math, a table, an import path or an auth change. Fix a BLOCK; ignore notes.
4. **Ship.** `pnpm ship <name> "<type>(<scope>): <summary> [<ID>]"` runs `pnpm verify:quick`
   on the branch head, squash-merges to `main` on green, marks the feature done and deletes the
   branch. Run `pnpm verify:full` on `main` after a merge that touches screens or start-up.
   Then fold the scenarios into `docs/specs/`.
5. **Milestone.** At the end of a milestone run the verifier and ask the human for the demo.

`/next` runs steps 1 to 4 for the next ready feature. Human checkpoints: milestone demos,
anything costing money, deploys and anything public, legal, privacy, poker-site policy and
naming, a change to a golden fixture, and the same failure three times. Open items live in
`docs/progress/needs-human.md`.

## Change discipline

- Make the smallest change that satisfies the scenarios. Do not refactor, rename or "improve"
  code you were not asked to touch. Removing code counts as progress.
- No abstraction for a single use. Inline first; extract at three real call sites.
- No defensive handling for inputs that cannot occur; types and the API gate refuse them.
- No new repo check, lint rule or fixture unless a scenario in the feature needs it.
- A spec gap (a scenario cannot hold) is a one-line amendment in the plan, not a process.
- If a change needs a new dependency, a new table, a new engine crate or a cross-module call,
  say so in the plan.

## Tests

- One test per scenario at the cheapest layer that proves it: Rust unit and property tests
  (`proptest`) over TS unit (Vitest) over integration (real Postgres and Redis) over e2e
  (Playwright, desktop Chromium, production build).
- Always: engine math as property tests (equities sum to 1, ICM sums to the prize pool,
  symmetric spots give symmetric results) plus golden reference tests; native and WASM give
  the same result on the reference set; a seeded tournament replays identically; cross-user
  isolation as an integration test per owner-scoped resource; the critical flows in e2e.
- Never: tests that restate config, constants or library behaviour; e2e for static layout.
- Ceiling: test lines at most equal production lines in a change (golden fixture data does
  not count). Red-first for new behaviour.

```bash
pnpm check         # cargo fmt/clippy/test · lint · typecheck · unit (fast; the Stop hook runs this)
pnpm verify        # check + format · license check · wasm build · build (before pushing)
pnpm verify:quick  # verify + Postgres/Redis containers · integration tests (the ship gate)
pnpm verify:full   # verify:quick + full Compose stack · e2e (on main)
```

## Conventions

- Rust (stable) for `engine/`; TypeScript everywhere else; pnpm + Turborepo; Next.js web,
  Fastify api, BullMQ worker, Postgres with Drizzle and plain SQL migrations, Redis. Details
  in `docs/architecture.md`.
- Conventional commits on branches; `main` gets one squashed commit per feature. Never
  force-push. Never commit secrets; `.env` is gitignored.
- Default coach model: `claude-haiku-5-5`; a stronger model only for Coach-tier session
  reviews; every provider behind `packages/ai-coach`.
