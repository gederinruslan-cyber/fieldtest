---
description: Take the next ready feature from docs/features.md through plan, build, review-on-risk and ship
argument-hint: "[feature ID]"
---

Take one feature end to end, following `CLAUDE.md` "How work flows".

Feature: `$ARGUMENTS` if given; otherwise the first `ready` row in `docs/features.md` whose
dependencies are `done`. Say which feature you picked and why before starting.

1. Preconditions: working tree clean on `main`, `git pull --ff-only` succeeded, no `blocking`
   item in `docs/progress/needs-human.md`. Mark the row `in-progress`.
2. Plan in plan mode (read the "done when", related specs, ADRs and the code you will touch).
   Write the plan as scenarios plus the end-to-end check, under 60 lines, into
   `docs/plans/<ID>-<name>.md`. Engine features name their reference spots and tolerance. If a
   real architecture decision is needed, write the ADR first and stop for the human only if it
   costs money, locks in a vendor, adds a non-permissive license, or touches legal, privacy or
   poker-site policy.
3. Branch `change/<name>` from `main`. Build test-first against the scenarios; commit per
   coherent step; `pnpm check` as you go; `pnpm verify` before pushing. Push the branch.
4. If this feature adds engine math, a table, an import path or changes auth, run `/review`
   once. Fix any BLOCK; re-run on that finding only. Otherwise skip this step and say so.
5. `pnpm ship <name> "<type>(<scope>): <summary> [<ID>]"`. If the gate is red, read the
   failure, fix, push, ship again. The same failure three times → add a `needs-human` item and
   stop. After a merge that touches screens or start-up, run `pnpm verify:full` on `main`
   once; if it is red, fixing it is the next task.
6. Fold the scenarios into the matching `docs/specs/<capability>.md` (add or modify
   requirements; keep it behaviour only) and commit that to `main` directly as
   `docs(specs): <name>`. Delete the plan file. Add one line to `docs/progress/log.md`.
7. Report: feature, merged SHA, gate result, review verdict (or "not needed"), test lines vs
   production lines, anything left open.

Never force-push, deploy, spend money, touch secrets or edit a golden fixture without a
`needs-human` decision. Report a failure as a failure.
