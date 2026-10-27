# 0003 · Permissive licenses only

- **Status:** accepted · 2026-10-28
- **Context:** The best browser postflop solver (b-inary's postflop-solver / wasm-postflop) is
  AGPL-3.0 and unmaintained since 2023; TexasSolver is AGPL and needs a paid license to run
  as a service. Hosting AGPL code would oblige us to publish our source.
- **Decision:** Shipped and hosted code uses only MIT, Apache-2.0, BSD-2/3-Clause, ISC, Zlib
  or Unicode licenses, enforced by `cargo deny check licenses` and `pnpm licenses:check` in
  `pnpm verify`. Candidates: rs-poker (Apache-2.0), PokerHandEvaluator (Apache-2.0), OMPEval
  (ISC), PokerKit (MIT), OpenSpiel (Apache-2.0); verify each LICENSE file before adding it.
  ICM and push/fold Nash are written in-house.
- **Amendment 2026-10-28 (NH-4):** 0BSD (tslib, via BullMQ and Next) and CC-BY-4.0
  (caniuse-lite, Next's browser-support data) are allowed: 0BSD is public-domain-equivalent
  and CC-BY-4.0 asks only for attribution. CC-BY-4.0 is for data packages, not code. Optional
  LGPL dependencies (sharp's libvips) are excluded rather than allowed.
- **Consequences:** No postflop solver at launch; postflop grading is an estimate. Output of
  an AGPL solver run offline might be usable as data, but only after a legal check (human).
