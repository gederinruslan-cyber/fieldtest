#!/usr/bin/env bash
# pnpm ship <change> "<type>(<scope>): <summary> [<feature ID>]"
# Merges a finished change in one step (CLAUDE.md "How work flows"):
#   1. refuses unless the working tree is clean and change/<change> exists on origin;
#   2. if origin/main moved, merges it into the branch and pushes (never force);
#   3. runs `pnpm verify:quick` locally on the branch head (the gate); a red run stops here;
#   4. squash-merges it onto origin/main (detached, so it works from any worktree);
#   5. marks the feature row done with the SHA in docs/features.md, promotes todo rows whose
#      dependencies are now all done to ready, and adds a log.md line, in a second commit;
#   6. pushes main and deletes the branch (remote and local).
# Never force-pushes. Bash 3.2 compatible.
set -euo pipefail

CHANGE="${1:?usage: pnpm ship <change> \"<subject> [<ID>]\"}"
SUBJECT="${2:?usage: pnpm ship <change> \"<subject> [<ID>]\"}"
BRANCH="change/${CHANGE}"
ID="$(printf '%s' "$SUBJECT" | sed -n 's/.*\[\([^] []*\)[^]]*\]$/\1/p')"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

fail() { echo "ship: $*" >&2; exit 1; }

[ -z "$(git status --porcelain)" ] || fail "working tree is not clean"
[ -n "$ID" ] || fail "subject must end with [<feature ID>]"
git fetch -q origin
git rev-parse -q --verify "origin/${BRANCH}" >/dev/null || fail "origin/${BRANCH} does not exist"
HEAD_SHA="$(git rev-parse "origin/${BRANCH}")"

if ! git merge-base --is-ancestor origin/main "$HEAD_SHA"; then
  git switch -q --detach "$HEAD_SHA"
  git merge -q --no-edit origin/main || { git merge --abort; fail "origin/main conflicts with ${BRANCH}"; }
  git push -q origin "HEAD:${BRANCH}"
  HEAD_SHA="$(git rev-parse HEAD)"
fi

git switch -q --detach "$HEAD_SHA"
pnpm verify:quick || fail "verify:quick is red on ${HEAD_SHA:0:7}"

git switch -q --detach origin/main
git merge -q --squash "$HEAD_SHA"
git commit -q -m "$SUBJECT" -m "Change: ${CHANGE}
Verify: local verify:quick ✓ (${HEAD_SHA:0:7})"
MERGED="$(git rev-parse --short HEAD)"

DAY="$(date +%Y-%m-%d)"
python3 - "$ID" "$MERGED" "$CHANGE" "$DAY" <<'EOF'
import re, sys
item, sha, change, day = sys.argv[1:]
p = "docs/features.md"
s = open(p).read()
# A row "| <ID> | <feature> | <status> | ..." becomes "| <ID> | <feature> | done `<sha>` | ...".
row = re.compile(r"^(\| " + re.escape(item) + r" \| [^|]*\| )(?:todo|ready|in-progress)( \|[^\n]*)$", re.M)
s, n = row.subn(lambda r: f"{r.group(1)}done `{sha}`{r.group(2)}", s)
if n == 0:
    print(f"ship: no {item} row in {p}; log line only", file=sys.stderr)
# A todo row becomes ready once every row it depends on is done ("F1-01…F1-06" is a span).
rows = re.compile(r"^(\| (F\d-\d\d) \| [^|]*\| )(todo|ready|in-progress|done[^|]*|later)( \| ([^|]*)\|[^\n]*)$", re.M)
done = {r.group(2) for r in rows.finditer(s) if r.group(3).startswith("done")}
def deps(field):
    ids = []
    for a, b in re.findall(r"(F\d-\d\d)(?:…(F\d-\d\d))?", field):
        ids += [f"{a[:3]}{k:02d}" for k in range(int(a[3:]), int((b or a)[3:]) + 1)] if b else [a]
    return ids
def promote(r):
    if r.group(3) == "todo" and all(d in done for d in deps(r.group(5))):
        print(f"ship: {r.group(2)} is now ready", file=sys.stderr)
        return f"{r.group(1)}ready{r.group(4)}"
    return r.group(0)
s = rows.sub(promote, s)
open(p, "w").write(s)
log = "docs/progress/log.md"
lines = open(log).read().split("\n")
entry = f"- {day} · {item} `{change}` merged ({sha}); local verify:quick."
at = next((i for i, l in enumerate(lines) if l.startswith("- ")), len(lines))
lines.insert(at, entry)
open(log, "w").write("\n".join(lines))
EOF
git add -A docs/features.md docs/progress
git commit -q -m "chore(progress): ${CHANGE} done [${ID}]"
git push -q origin HEAD:main
git push -q origin --delete "$BRANCH"
git branch -D "$BRANCH" >/dev/null 2>&1 || true
echo "ship: ${CHANGE} merged as ${MERGED}; main pushed (HEAD is detached at it)"
