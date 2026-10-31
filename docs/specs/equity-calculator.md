# Equity calculator

*Behaviour of `fieldtest-core::equity` and `/tools/equity`. Source: F1-01.*

## Engine
- 2 to 9 players, each a range (a hand is a one-combo range), on a board of 0, 3, 4 or 5
  distinct cards. A deal gives each player one combo from their range, no two sharing a card
  or a board card, weighted by the product of the combos' weights; every completion of the
  board is equally likely. A card in every combo of one player (a single hand) is dead for the
  others before anything is counted.
- Per player: win (takes the pot alone), tie (splits it), equity (win plus split shares; the
  equities sum to 1) and a standard error (0 when exact). The result names its method: exact
  with the number of deals enumerated, or Monte Carlo with the number of samples.
- Exact when completions × (live combos + deals / 8) is at most 15,000,000, otherwise Monte
  Carlo over the requested samples with an explicit seed (ChaCha8); the same seed gives the
  same numbers natively and in WASM. Exact: a hand against a hand or a small range preflop
  (AhAs vs KK), 9 single hands preflop (278,256 deals), most ranges postflop (190 vs 190
  combos on a flop). Wider preflop ranges are sampled.
- Reference: the 26 `hu-equities.csv` matchups match win, win and tie within ±0.005 pp over
  1,712,304 deals (AhAs vs KdKc: 81.06 / 18.55 / 0.38, equity 81.26%). Monte Carlo of the
  same spot at 200,000 samples lands within 4 standard errors (about 0.09 pp each).
- The same range for both players gives each 0.5; AhAs vs `AA` deals only AdAc (0.5 each).
- Refused: `InvalidPlayers` (not 2–9 players or an empty range), `InvalidBoard`,
  `NoValidDeal` (no deal without a shared card; in Monte Carlo, 10,000 rejected deals in a
  row), `InvalidSamples` (fewer than 2).
- Budget: every call under 0.5 s in the browser (exact worst about 0.35 s; 9-way Monte Carlo
  at 200,000 samples about 0.37 s). A 190 vs 190 combo flop takes about 205 ms, just over
  the 200 ms example in the engine rules.

## Page `/tools/equity`
- Static page with a client calculator: 2–9 player rows (a hand or a range each), Add and
  Remove player, a board field (empty for preflop), Calculate. A `;` in a row is refused.
- Results per player: equity, win and tie to 2 decimals, then the typed range. The table is
  named by its method: "Exact: N deals enumerated" or "Estimate: Monte Carlo over 200,000
  sampled deals (seed 1); ± is one standard error", with the margin in percentage points
  under each estimated equity. At 360 px the page does not scroll sideways; the table does.
