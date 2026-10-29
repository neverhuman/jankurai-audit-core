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

locked_sources() { sed -n 's/^\(source = "git+.*\)/\1/p' Cargo.lock | sort -u; }

out="$(JANKURAI_GIT_MIRROR_BASE=https://mirror.invalid/g bash scripts/git-source-route.sh env)"
owners="$(sed -n 's#^source = "git+https://github.com/\([^/]*\)/.*#\1#p' Cargo.lock | sort -u)"
[[ -n "$owners" ]] || fail "no locked git sources found"
for o in $owners; do
  for h in github.com www.github.com; do
    grep -qx "GIT_CONFIG_VALUE_[0-9]*=https://$h/$o/" <<<"$out" || fail "no rewrite for $h/$o"
  done
done
grep -q 'insteadOf' <<<"$out" || fail "rewrite keys missing"

# mirror-url must agree with the rewrite for both host spellings: the owner is
# dropped, so a `www.` pin and its plain twin name one mirror repository.
export JANKURAI_GIT_MIRROR_BASE=https://mirror.invalid/g
while read -r url; do
  want="https://mirror.invalid/g/$(sed -E 's#^https://(www\.)?github\.com/[^/]+/##' <<<"$url")"
  got="$(bash scripts/git-source-route.sh mirror-url "$url")"
  [[ "$got" == "$want" ]] || fail "mirror-url $url gave $got, want $want"
  [[ "$got" != *"github.com"* ]] || fail "mirror-url left the github host in $got"
done < <(sed -n 's/^source = "git+\(https:[^?#"]*\).*/\1/p' Cargo.lock | sort -u)
# A URL that is not a locked GitHub source must be refused, not rewritten.
for bad in https://evil.example/x.git 'https://github.com.evil.example/o/x.git' ../../etc/passwd; do
  if bash scripts/git-source-route.sh mirror-url "$bad" >/dev/null 2>&1; then
    fail "mirror-url accepted $bad"
  fi
done
unset JANKURAI_GIT_MIRROR_BASE

# Every locked rev is a 40-hex commit id, which is what `verify` looks up.
revs="$(sed -n 's/^source = "git+[^#]*#\([0-9a-f]\{40\}\)".*/\1/p' Cargo.lock | sort -u)"
[[ "$(wc -l <<<"$revs")" -eq "$(grep -c . <<<"$(locked_sources)")" ]] ||
  fail "a locked git source has no 40-hex commit id"

# verify refuses to run without a mirror base rather than fetching from GitHub.
if JANKURAI_GIT_MIRROR_BASE= bash scripts/git-source-route.sh verify >/dev/null 2>&1; then
  fail "verify without a mirror base must be refused"
fi

# The route is applied to the child only.
seen="$(JANKURAI_GIT_MIRROR_BASE=https://mirror.invalid/g bash scripts/git-source-route.sh run -- \
  git ls-remote --get-url https://www.github.com/$(printf '%s' "$owners" | head -n1)/x.git)"
[[ "$seen" == "https://mirror.invalid/g/x.git" ]] || fail "run did not rewrite: $seen"
git config --global --get-regexp '^url\..*mirror\.invalid' >/dev/null 2>&1 && fail "global config was written"

[[ "$lock_before" == "$(sha256sum Cargo.lock)" ]] || fail "Cargo.lock changed"
echo "ok: git-source-route"
