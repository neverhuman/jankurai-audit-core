#!/usr/bin/env bash
# Offline proof of scripts/git-source-route.sh: the rewrite covers every locked
# Git source on both host spellings, writes no git config, and leaves the lock
# file untouched.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

fail() { echo "FAIL: $1" >&2; exit 1; }
lock_before="$(sha256sum Cargo.lock)"

if JANKURAI_GIT_MIRROR_BASE= bash scripts/git-source-route.sh env >/dev/null 2>&1; then
  fail "unset mirror base must be refused"
fi
if JANKURAI_GIT_MIRROR_BASE=http://mirror.invalid/g bash scripts/git-source-route.sh env >/dev/null 2>&1; then
  fail "non-https mirror base must be refused"
fi

out="$(JANKURAI_GIT_MIRROR_BASE=https://mirror.invalid/g bash scripts/git-source-route.sh env)"
owners="$(sed -n 's#^source = "git+https://github.com/\([^/]*\)/.*#\1#p' Cargo.lock | sort -u)"
[[ -n "$owners" ]] || fail "no locked git sources found"
for o in $owners; do
  for h in github.com www.github.com; do
    grep -qx "GIT_CONFIG_VALUE_[0-9]*=https://$h/$o/" <<<"$out" || fail "no rewrite for $h/$o"
  done
done
grep -q 'insteadOf' <<<"$out" || fail "rewrite keys missing"

# The route is applied to the child only.
seen="$(JANKURAI_GIT_MIRROR_BASE=https://mirror.invalid/g bash scripts/git-source-route.sh run -- \
  git ls-remote --get-url https://www.github.com/$(printf '%s' "$owners" | head -n1)/x.git)"
[[ "$seen" == "https://mirror.invalid/g/x.git" ]] || fail "run did not rewrite: $seen"
git config --global --get-regexp '^url\..*mirror\.invalid' >/dev/null 2>&1 && fail "global config was written"

[[ "$lock_before" == "$(sha256sum Cargo.lock)" ]] || fail "Cargo.lock changed"
echo "ok: git-source-route"
