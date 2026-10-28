# Needs Human

*Open escalations from the working session. Newest first. The human answers inline under
**Decision**, sets **Status** to `resolved`, and commits (or tells Claude). Items marked
**blocking** stop the loop until resolved; others let it continue with unrelated work.*

<!-- Template
## NH-<n> · <title> · <blocking | non-blocking> · Status: open
- **Raised:** YYYY-MM-DD during <feature ID> (<plan | build | ship>)
- **Needed:** what decision or action is required
- **Why:** money · public · legal/privacy/naming · poker-site policy · golden fixture · repeated failure · milestone
- **Options:** A … / B … — **Recommendation:** …
- **Decision:** _(human)_
-->

## NH-4 · npm licenses outside ADR 0003 · blocking · Status: resolved
- **Raised:** 2026-10-28 during F0-01 (build)
- **Needed:** `pnpm licenses:check` refused tslib (0BSD, via bullmq and Next's @swc/helpers)
  and caniuse-lite (CC-BY-4.0, via Next); both are unavoidable with the ADR 0001 stack.
- **Why:** legal (license allow-list)
- **Options:** A allow both / B allow 0BSD and exempt caniuse-lite by name / C stop —
  **Recommendation:** A
- **Decision:** 2026-10-28: A, allow 0BSD and CC-BY-4.0 (data) in both allow-lists; ADR 0003
  amended. Sharp's LGPL libvips excluded as an optional dependency instead.

## NH-3 · Domain and trademark for "Fieldtest" · non-blocking · Status: open
- **Raised:** 2026-10-28 during setup
- **Needed:** pick and buy a domain (fieldtest.poker, fieldtest.gg, getfieldtest.com); run a
  USPTO (and EUIPO if launching in the EU) search in classes 9 and 41.
- **Why:** naming, money
- **Decision:** _(human)_

## NH-2 · Reference values from HRC and ICMIZER · non-blocking · Status: open
- **Raised:** 2026-10-28 during setup (needed by F0-04)
- **Needed:** about 20 benchmark spots run through HRC's free Nash/ICM tools and ICMIZER,
  with outputs saved as plain text (spot, stacks, payouts, result, tool, date).
- **Why:** golden fixtures are human-owned; capturing them uses third-party tools.
- **Progress 2026-10-28:** published candidates collected in
  [`nh2-reference-candidates.md`](nh2-reference-candidates.md): 7-card category counts, 26
  exact HU equities (reproduced by enumeration), ICM examples, HU push/fold Nash from HRC.
  Still open: multiway ICM push/fold and bounty spots from HRC/ICMIZER.
- **Progress 2026-10-28 (F0-04):** human approved candidates §1 (7-card counts), §2 (26 HU
  equities), §3 (ICM: Wikipedia example and symmetry case only; the two unretrieved sources are
  excluded) and §4 (HU push/fold Nash) as golden fixtures in `engine/fixtures/`. Multiway ICM
  push/fold and bounty spots are added with F1-03/F1-04 once captured.
- **Progress 2026-10-28 (search):** new candidates for approval in the candidates file: §5
  3-player winner-takes-all Nash (HRC via PokerStrategy), §6 peer-reviewed ICM example
  (Ganzfried & Sandholm, checked), §7 bounty arithmetic (CardPlayer PKO, GTO Wizard mystery
  bounty, both checked). Still unpublished anywhere we found: multiway ICM push/fold with stated
  payouts, and solver bounty calling ranges; those still need an HRC/ICMIZER run.
- **Progress 2026-10-28:** human approved §5–§7; added to `engine/fixtures/`.
- **Decision:** _(human)_

## NH-1 · GitHub repository and first commit · blocking · Status: resolved
- **Raised:** 2026-10-28 during setup
- **Needed:** create a private GitHub repo `fieldtest`, add it as `origin`, and push the
  initial commit on `main`. `pnpm ship` needs `origin`.
- **Why:** public/outward-facing action
- **Decision:** 2026-10-28: repo https://github.com/gederinruslan-cyber/fieldtest, kept **public** by the
  human's choice; first commit 91abd6e pushed to `main`.
