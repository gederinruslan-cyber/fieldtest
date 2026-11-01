# F1-03 · push-fold-nash

Done when: heads-up push/fold Nash with ICM matches HRC on the reference set; free page.
Decision: ADR 0005. Small blind against big blind is exact; with three or more players acting
it moves to a new row, F1-08 push-fold-nash-multiway, because three-way equity needs its own
decision. Amendment: the done-when for F1-03 now reads heads-up; F1-08 depends on F1-03, and
F2-06 depends on F1-08.

New: the `fieldtest-icm` crate depends on `fieldtest-core` (a cross-crate call). It also gains
a generated data file, `data/hu-preflop.bin`, about 228 KB, built by `examples/hu_table.rs`.

## Scenarios
1. **HRC reference.** Given every row of `hu-push-fold.csv` from 12 to 20 bb (2 players,
   winner takes all, blinds 50/100, ante 0 or 10, both stacks `stack_bb` × 100), when
   push/fold is solved, the small blind's push % and the big blind's call % are each within
   ±3 pp of the published value. Amended (NH-5): the 2–8 bb rows are not an equilibrium of
   the stated game, so they are not pass/fail. In their place, at 10 bb, every pure call or
   fold by the big blind agrees with core-engine Monte Carlo equity within 4σ.
2. **Table matches enumeration.** Given AKs v QQ and T9s v 22, the table's win and
   tie counts equal `exact` summed over their combo pairs. For any classes a and b, ties(a,b)
   = ties(b,a), and wins(a,b) + ties + wins(b,a) = combo pairs × 1,712,304.
3. **Nash within ε.** Given any spot with 2 to 6 players, stacks 1 to 60 bb and
   non-increasing payouts, when solved, neither player gains more than the reported ε (at
   most 1e-4 of the prize pool, measured on the strategies returned; hands with a clear
   edge are pure) by switching any hand to push, fold or call. The push and
   call weights lie in [0, 1].
4. **ICM pressure.** Given SB 2,000, BB 2,000, and a third player with 300 folded, blinds
   50/100 and payouts 50/30/20, the big blind calls fewer hands than with the same two stacks
   heads-up winner-takes-all.
5. **Edges.** Payouts with one place and 2 players give chip-EV Nash. A stack that can't
   cover its blind plus ante gives `InvalidStacks`. A small blind of 0, or one not smaller
   than the big blind, gives `InvalidBlinds`. Bad payouts give `InvalidPayouts`.
6. **WASM parity.** `pushFold` returns the same numbers natively and in WASM for a set of
   spots: 10 bb, 15 bb with ante, the ICM spot above, and the errors.

## Engine
- `push_fold(stacks, small_blind, big_blind, ante, payouts)`: `stacks[0]` is the small blind,
  `stacks[1]` the big blind, the rest are folded players who post antes. It returns 169 push
  weights, 169 call weights, the push and call shares of the 1,326 combos, and ε.
- Fictitious play over the exact table. Outcomes are kept separate (fold, steal, win, tie,
  loss), each valued by ICM over all the stacks. A busted player takes the last paid place.
  An odd chip in a tie goes to the small blind.
- Budget: under 200 ms in the browser.

## Page `/tools/push-fold`
- Inputs: stacks (small blind, big blind, others), blinds and ante, payouts.
- Output: two read-only 13×13 grids with mixed hands shown as a share, each with its push or
  call %, and the line "Nash equilibrium: neither player can gain more than ε by changing a
  hand (exact heads-up equities)".

## End-to-end
Run `/tools/push-fold` with the defaults: 1,000 / 1,000, blinds 50/100, no ante, payout 100.
It shows push 58.5% and call 37.3%, the engine's values, which are pinned in the e2e test.
