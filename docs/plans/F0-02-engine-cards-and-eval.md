# F0-02 · engine-cards-and-eval ∑

Done when: card, combo and range types; range notation parses and prints round-trip; the
7-card evaluator gives the exact category counts over all 133,784,560 hands.

Adds engine math, so `/review` runs once. No new crate (all in `fieldtest-core`), no table, no
cross-module call. New dev-dependency only: `proptest` (MIT/Apache-2.0). `[profile.test]`
gets `opt-level = 3` so the full enumeration runs in `pnpm check` in seconds.

## Shape (`engine/crates/core/src`)
- `card.rs`: `Card(u8)` = rank·4 + suit (ranks 2..A, suits c d h s); parses/prints `Ah`.
- `combo.rs`: `Combo` of two distinct cards, high card first; `index()` in 0..1326.
- `range.rs`: `Range` = a weight in [0, 1] per combo. Notation: `AA`, `22+`, `77-55`, `AKs`,
  `AKo`, `AK`, `ATs+`, `KTs-K7s`, `AhKh`, `:w` suffix for weight, comma separated; a later
  token overrides an earlier one. Printing is canonical: pairs high to low, then suited, then
  offsuit, runs compressed (`QQ+`, `77-55`, `ATs+`, `KTs-K7s`), a class with mixed weights
  printed as single combos. Weight printed with Rust's shortest round-trip `f64` form.
- `eval.rs`: `evaluate(&[Card])` for 5 to 7 cards → `HandRank(u32)` (category then kickers,
  bigger wins) with `category()`. Bitmask per suit; no lookup table (WASM size).
- Errors are `Result<_, ParseError>` with a code; no panics on any input string.

## Scenarios (reference spots, tolerance exact)
1. **Category counts.** Given all C(52,7) hands, when each is evaluated, then the counts are
   straight flush 41,584 (royal 4,324 + other 37,260), quads 224,848, full house 3,473,184,
   flush 4,047,644, straight 6,180,020, trips 6,461,620, two pair 31,433,400, pair 58,627,800,
   high card 23,294,460 (Wikipedia "Poker probability", NH-2 §1). The values sit in the test
   with their source until F0-04 moves them into `engine/fixtures/` after human approval.
2. **Best five of seven (property).** Given any 7 distinct cards, the 7-card rank equals the
   max over its 21 five-card subsets; it is unchanged by card order and by any permutation of
   suits (symmetry).
3. **Reference hands.** `A♠K♠Q♠J♠T♠` is a straight flush above `9♠K♠Q♠J♠T♠`; the wheel
   `A♣2♦3♥4♠5♣` is a straight below `2♣3♦4♥5♠6♣`; `A♠A♥K♣K♦Q♠` beats `A♣A♦K♥K♠J♣`; equal
   hands in different suits tie.
4. **Range counts.** `22+` = 78 combos, `ATs+` = 16, `AKo` = 12, `AK` = 16, `KTs-K7s` = 16,
   `77-55` = 18, `AhKh` = 1, `QQ+,AKs:0.5` = 18 combos at 1 and 4 at 0.5; 1,326 combos total
   for `22+,A2+,K2+,Q2+,J2+,T2+,92+,82+,72+,62+,52+,42+,32`.
5. **Round trip (property).** For any range (classes full, partial or weighted at any `f64` in
   (0, 1]), `parse(print(r)) == r`; for any canonical string `print(parse(s)) == s`; cards
   round-trip for all 52.
6. **Edge case.** `AA:0`, `AA:1.5`, `AKx`, `AA-KK` written with a non-pair end, `AhAh`, and an
   empty token return a `ParseError` code, never a panic (property: arbitrary strings never
   panic). The empty string parses to the empty range.

## End-to-end check
`pnpm verify` green: scenario 1 runs inside `cargo test` (release-optimised test profile) and
the crate still builds for wasm32.
