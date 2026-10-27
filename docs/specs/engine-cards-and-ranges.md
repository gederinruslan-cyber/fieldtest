# Engine: cards, ranges and hand evaluation

*Behaviour of `engine/crates/core`. Source: F0-02.*

## Cards and combos
- 52 cards written `Ah`, `Tc` (ranks `23456789TJQKA`, suits `cdhs`); every card prints and
  parses back to itself; anything else is `InvalidCard`.
- 1,326 combos of two distinct cards, each with a stable index in 0..1326; a combo is the same
  whichever order its cards are given.

## Range notation
- Tokens `AA`, `22+`, `77-55`, `AKs`, `AKo`, `AK`, `ATs+`, `KTs-K7s` and single combos
  `AhKh`, each with an optional `:weight` in (0, 1], comma separated; a later token overrides
  an earlier one; the empty string is the empty range.
- Reference counts: `22+` 78, `ATs+` 16, `AKo` 12, `AK` 16, `KTs-K7s` 16, `77-55` 18, `AhKh`
  1, all 169 classes 1,326; `QQ+,AKs:0.5` is 18 combos at 1 and 4 at 0.5.
- A zero, negative, above-1 or non-numeric weight is `InvalidWeight`; an unknown token, a
  run whose ends differ in kind or high card, a repeated card or an empty token is
  `InvalidToken`. No input string panics.
- Printing is canonical (pairs high to low, then suited, then offsuit; runs compressed to
  `QQ+`, `77-55`, `ATs+`, `KTs-K7s`; a class with mixed weights as single combos; weights in
  the shortest form that parses back to the same `f64`). Any range prints and parses back to
  itself, and a printed range prints the same again.

## Hand evaluation
- `evaluate` ranks the best five-card hand among 5 to 7 cards; a bigger rank wins and equal
  ranks tie. A 7-card rank equals the best of its 21 five-card subsets and does not depend on
  card order or on renaming suits.
- Over all 133,784,560 seven-card hands the category counts are exact (tolerance 0): straight
  flush 41,584 (royal 4,324), quads 224,848, full house 3,473,184, flush 4,047,644, straight
  6,180,020, trips 6,461,620, two pair 31,433,400, pair 58,627,800, high card 23,294,460
  (Wikipedia "Poker probability"; moves into `engine/fixtures/` with F0-04).
- The wheel `A2345` is the lowest straight; `AKQJT` suited is the highest hand.
