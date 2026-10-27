---
paths:
  - "engine/crates/sim/**"
  - "apps/api/src/modules/hand-import/**"
  - "apps/api/src/modules/pools/**"
  - "apps/api/src/modules/showcases/**"
  - "apps/worker/src/jobs/**"
  - "apps/web/app/**/import/**"
  - "apps/web/app/**/pools/**"
---

# Table safety and fair play (apply when touching these files)

Fieldtest is a study tool. Poker rooms ban real-time assistance and treat bot tooling as a
threat; GGPoker bans third-party software from 2026-12-01. Nothing we build may work at a
live table.

- Hand histories arrive only as files the user uploads in the browser after a session. No
  desktop agent, no folder watcher, no browser extension, no clipboard or screen reading, no
  "paste the current hand" flow.
- PokerStars import is supported. GGPoker (PokerCraft) import is manual upload only, behind
  the `import.ggpoker` feature flag, which must stay switchable off without a deploy.
- Opponent names are hashed with a per-user salt at import; raw names are never stored,
  logged, shown to other users or used in showcases.
- Pools and bot strategies run only inside our simulator. No export of a pool, bot, strategy
  or range set in any machine-usable form; no bot API; no user code execution.
- Showcases (pool runs published as content) run server-side and use only synthetic or
  consenting users' pools.
- Play money only: no purchasable chips, no prizes of value, no leaderboards with rewards.
- A feature that changes any of this is a `needs-human` item (poker-site policy), not a plan
  decision.
