# Range analyzer

*Behaviour of `/tools/range-analyzer` and the engine's range analysis. Source: F1-05.*

## Engine
- A board is 3 to 5 distinct cards, else `InvalidBoard` (or `InvalidCard`). Combos sharing a
  card with the board are removed before every count.
- Each live combo takes one made class, the first that applies: StraightFlush, Quads,
  FullHouse, Flush, Straight (best hand including the board); Set (pocket pair matching a
  board card); Trips (a hole card matching a board pair); TwoPair (both hole ranks on the
  board); Overpair; TopPair; Underpair (pocket pair below the top board card); MiddlePair;
  WeakPair (paired to the third board rank or lower); AceHigh; Nothing.
- Draws, flop and turn only, any number per combo: FlushDraw (four to a flush including a
  hole card, no flush made); OpenEnded (two or more ranks complete a straight the board alone
  would not, no straight made); Gutshot (exactly one such rank). No draws on the river.
- The breakdown gives the live weighted combos and, per class and draw, the weighted combos
  and their share of the live total. Exact: every live combo is enumerated. Class weights sum
  to the live total and shares to 1.
- Reference spots: `AA` on `Ah 7c 2d` → 3 live, all Set; `AK` on `Kh 7c 2d` → 12 TopPair;
  `AhQh` on `Kh 7h 2c` → AceHigh + FlushDraw; `98s` on `Tc 7d 2s` → 4 Nothing + OpenEnded;
  `J9o` on `Qc 8d 2s` → 12 Nothing + Gutshot; `AhAd` on `9c 8d 7s 6h` → Overpair, no draw;
  on `Kh 9c 4d`: `Ks2s` TopPair, `9s2h` MiddlePair, `4s2h` WeakPair, `QQ` Underpair, `AA`
  Overpair, `AsQs` AceHigh, `K9` TwoPair, `99` Set; `9s8s` on `9c 9d 4h` Trips.
- Filtering keeps the live combos whose class or any draw is chosen, at their weights;
  filtering by every class gives the range minus blocked combos.
- Grid weights are the mean weight of each of the 169 classes (pairs on the diagonal, suited
  above, offsuit below); setting a class sets all its combos.

## Page
- Renders statically (title, description, how-the-counts-work copy, empty grid and board) and
  runs the engine in the Web Worker pool after load; works without an account.
- The range is typed or toggled on the 13×13 grid (a cell is full, partial with its percentage
  printed and hatched, or empty; `aria-pressed` true, mixed or false). Board cards show rank
  and suit symbol in a four-colour deck.
- The breakdown table is captioned "Exact: every live combo enumerated" with the live combo
  count; shares show one decimal, formatted in one place; weighted combos show two decimals.
- "Keep selected and go to the turn/river" replaces the range with the filtered one, locks the
  current board and asks for the next card. Example: `AA,AK` on `Ah 7c 2d` → 15 live: Set 3
  (20.0%), Top pair 12 (80.0%); keeping Set gives `AdAc,AsAc,AsAd` and "Pick the turn card".
- Invalid notation shows a message and leaves the applied range unchanged.
- CSP header and sitemap entry for tool pages come with F1-07.
