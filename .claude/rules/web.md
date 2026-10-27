---
paths:
  - "apps/web/**"
  - "packages/ui/**"
---

# Web rules (apply when touching these files)

- `apps/web` holds no business logic and no poker math. Engine calls go through
  `packages/engine-wasm` in a Web Worker pool (never on the main thread); api calls go through
  the generated client only.
- Free tools (calculators, range analyzer, texture pages) render statically for search: real
  HTML content, titles, descriptions and a sitemap entry; the engine hydrates after load.
  They work without an account.
- Every number on screen shows whether it is exact or an estimate; Monte Carlo shows its
  margin. Percentages are formatted in one place with a stated precision.
- The simulator table is keyboard usable (fold/call/raise shortcuts), works at 360 px width,
  and respects reduced motion for card animations.
- Copy says "study" and "practice", never "use while you play"; no feature is presented as a
  live-table aid.
- No inline script or style; CSP with a per-request nonce on dynamic routes;
  `wasm-unsafe-eval` only where the engine loads.
- Accessibility: real buttons, links and labels; 4.5:1 contrast; 44 px targets; screen reader
  usable; light and dark; colour is never the only signal (suits carry a symbol, grades a
  word).
