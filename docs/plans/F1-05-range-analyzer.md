# F1-05 · range-analyzer

Done when: 13×13 grid, board picker, made-hand and draw breakdown with combo counts, filters
carried to the next street. Engine math (classes, draws, card removal), so `/review` runs.
No new dependency, crate, table or cross-module call; the web app gains its first engine use.

## Engine (`core/src/analysis.rs`, exported through `crates/wasm`)
- Board: 3 to 5 distinct cards, else `InvalidBoard`. Combos sharing a card with the board are
  removed before every count (card removal).
- Made class (first match, exclusive): StraightFlush, Quads, FullHouse, Flush, Straight (best
  hand including the board); Set (pocket pair matching a board card); Trips (a hole card
  matching a board pair); TwoPair (both hole ranks on the board); Overpair; TopPair; Underpair
  (pocket pair below the top board card); MiddlePair; WeakPair (paired to the third board rank
  or lower); AceHigh; Nothing.
- Draws on flop and turn only, flags (a combo can have several): FlushDraw (4 to a flush with
  a hole card, no flush made); OpenEnded (≥2 ranks complete a straight that the board alone
  does not, no straight made); Gutshot (exactly 1 such rank).
- `analyzeRange(range, board)`: live weighted combos, and per class and draw the weighted
  combos and their share of the live total (exact enumeration, f64, no rounding).
- `filterRange(range, board, classes)`: the live combos whose class or any draw is listed,
  at their weights, as canonical notation. `gridWeights(range)`: 169 mean weights (row
  AA..22 high card, suited above the diagonal). `setClass(range, "AKs", weight)`.
- The worker call becomes `{ fn, args: string[] }`; the native reference runner and the
  parity set gain the new functions.

## Web (`apps/web/app/tools/range-analyzer`)
- Server-rendered page (title, description, how-it-works copy, empty grid and board) so it
  reads without JS; a client component hydrates and calls the engine pool, never the main
  thread. Grid buttons toggle a class (`aria-pressed`, label and percent text, 44 px targets,
  horizontal scroll under 600 px); board picker buttons show rank and suit symbol; breakdown
  table labelled "Exact: every combo enumerated"; percentages formatted in one function to
  one decimal. "Keep selected and go to the next street" replaces the range with the
  filtered one and asks for the next board card.

## Scenarios (tolerance exact for counts; shares within 1e-12 of the counted ratio)
1. **Reference spots.** `AA` on `Ah 7c 2d` → 3 live combos, all Set. `AK` on `Kh 7c 2d` → 12
   live, all TopPair. `AhQh` on `Kh 7h 2c` → AceHigh + FlushDraw. `98s` on `Tc 7d 2s` → 4
   Nothing + OpenEnded. `J9o` on `Qc 8d 2s` → 12 Nothing + Gutshot. On `Kh 9c 4d`: `Ks2s`
   TopPair, `9s2h` MiddlePair, `4s2h` WeakPair, `QQ` Underpair, `AA` Overpair, `AsQs`
   AceHigh, `K9` TwoPair, `99` Set; `9s8s` on `9c 9d 4h` Trips.
2. **Board-only draws don't count (edge).** `AhAd` on `9c 8d 7s 6h` → Overpair, no draw; any
   combo on a 5-card board has no draw.
3. **Invariants (property).** For any range and board, class weights sum to the live total
   and shares to 1; each live combo is in exactly one class; filtering by every class gives
   the range minus blocked combos; a filtered range analyzed again holds only kept classes.
4. **Grid.** `gridWeights("AKs,QQ:0.5")` has 1 at AKs, 0.5 at QQ, 0 elsewhere;
   `setClass("", "AKo", 1)` = `AKo`.
5. **Parity.** All new functions join the native/WASM reference set, identical answers.
6. **Flow (e2e, the critical path, real browser Worker).** Type `AA,AK`, pick `Ah 7c 2d`:
   15 live combos: Set 3 (20.0%), TopPair 12 (80.0%); keep Set and go to the turn: the range
   reads `AdAc,AsAc,AsAd` and the board asks for a fourth card.

Spec amendment: the CSP header and sitemap entry for static tool pages land with F1-07
(free-tools-public), which owns hosting; this page adds no inline script or style of its own.

## End-to-end check
`pnpm verify:full` green: engine tests, parity, e2e flow in desktop Chromium.
