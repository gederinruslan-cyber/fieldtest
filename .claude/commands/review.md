---
description: Fresh-context second opinion on the current diff — wrong numbers and boundary crossings only
argument-hint: "[path to the plan or scenarios]"
---

Invoke the **reviewer** agent on `git diff main...HEAD` with the scenarios at `$ARGUMENTS`
(or the current plan in `docs/plans/`). Give it the diff and the scenarios only. Paste its
verdict back verbatim. Fix a BLOCK and re-run on that finding only. Notes are optional and
never become rules, checks or backlog items.
