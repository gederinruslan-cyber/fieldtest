---
paths:
  - "packages/db/**"
  - "apps/api/src/**"
  - "apps/worker/src/**"
  - "apps/api/db/**"
---

# Data layer and auth rules (apply when touching these files)

- Every table that holds user data carries `owner_id` (a user, or a coach team once teams
  exist), is under row-level security and uses composite keys `(owner_id, id)`. Every read and
  write goes through a scoped transaction (`withOwner`); the raw pool is confined to the db
  package.
- A request for another owner's row answers **404, never 403**. Every owner-scoped resource
  gets one integration test proving user A cannot read or change user B's data.
- Migrations are plain SQL with a down script each, applied by a migrator role; runtime
  processes connect as an app or worker role, never a superuser.
- Auth tables are the only RLS exemption and are touched only inside the auth module and the
  purge job. No session token, password, reset token, email or hand data in logs.
- Log fields are an allow-list (`reqId`, error codes, counts), never request bodies or file
  contents.
- Every POST/PUT/DELETE passes the origin and content-type gate; nothing reads a cookie or
  header to decide ownership below HTTP.
- Uploaded hand-history files are parsed in the worker with size and row limits, stored as
  parsed hands (names hashed), and the raw file is discarded after parsing.
- Queues: only through the worker's queue factory; every job carries owner ids and expires.
- Billing amounts are integer minor units plus a currency code, never floats.
- If you add a dependency that touches the database, auth, uploads or HTTP parsing, say so in
  the commit message so the reviewer looks at it.
