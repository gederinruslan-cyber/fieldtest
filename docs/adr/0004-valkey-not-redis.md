# 0004 · Valkey as the Redis server

- **Status:** accepted · 2026-10-28
- **Context:** ADR 0001 picks Redis for BullMQ queues and caching. Redis 8 is licensed
  RSALv2 / SSPLv1 / AGPLv3, none of which ADR 0003 allows in anything we host. Redis 7.2 was
  the last BSD release and is past its support window.
- **Decision:** Run Valkey (BSD-3-Clause, the Linux Foundation fork of Redis 7.2) wherever we
  run the server ourselves: local Compose, CI and self-hosted environments. Clients speak the
  Redis protocol unchanged (BullMQ supports Valkey); code and docs keep saying "Redis" for the
  protocol and `REDIS_URL` for the setting.
- **Alternatives:** Redis 7.2 (BSD, but unsupported); Redis 8 under AGPL (ruled out by ADR
  0003); a managed Redis-compatible service (a vendor and money decision, taken at deploy).
- **Consequences:** The managed service chosen at deploy must be Redis-protocol compatible;
  that choice is a human checkpoint.
