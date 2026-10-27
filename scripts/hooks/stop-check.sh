#!/usr/bin/env bash
# Claude Code Stop hook: when the session has uncommitted, unpushed or not-yet-merged code
# changes, run the fast checks (`pnpm check`: cargo fmt/clippy/test, lint, typecheck, unit
# tests) and block the stop with the failure tail so the session fixes it before handing
# back. Docs and config changes skip it. Skips until the monorepo exists (no package.json).
# Never runs twice in a row for the same stop (`stop_hook_active`), so it cannot loop.
set -uo pipefail
INPUT="$(cat)"
case "$INPUT" in *'"stop_hook_active":true'*) exit 0 ;; esac
cd "$(dirname "${BASH_SOURCE[0]}")/../.." || exit 0
[ -f package.json ] || exit 0

changed="$( {
  git diff --name-only HEAD
  git diff --name-only --cached
  git ls-files --others --exclude-standard
  git log --name-only --format= main..HEAD 2>/dev/null
} | sort -u )"
echo "$changed" | grep -qE '^((apps|packages|e2e|scripts)/.*\.(ts|tsx|js|mjs|cjs|sql)|engine/.*\.(rs|toml))$' || exit 0

if command -v fnm >/dev/null 2>&1; then eval "$(fnm env)" && fnm use >/dev/null 2>&1; fi
OUT="$(pnpm check 2>&1)"
STATUS=$?
if [ "$STATUS" -ne 0 ]; then
  printf 'pnpm check failed (exit %s). Fix before stopping:\n%s\n' "$STATUS" "$(printf '%s\n' "$OUT" | tail -40)" >&2
  exit 2
fi
exit 0
