#!/usr/bin/env bash
# Cold-host bootstrap for the locked Git sources (see docs/bootstrap.md).
#
#   git-source-route.sh run   -- <command...>   run a command with the route applied
#   git-source-route.sh probe                   anonymous reachability of each locked source
#   git-source-route.sh env                     print the GIT_CONFIG_* assignments
#
# JANKURAI_GIT_MIRROR_BASE names the trusted mirror, e.g. https://host/git/org
# (no trailing slash). It rewrites https://github.com/<owner>/ and
# https://www.github.com/<owner>/ for the process only: no global or repository
# git config is written, and Cargo.toml / Cargo.lock (revs, tags, checksums) are
# not touched, so dependency identity is unchanged. Cargo still verifies the
# locked commit ids after fetching.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

owner_re='^https://(www\.)?github\.com/[A-Za-z0-9_.-]+/'

locked_urls() {
  sed -n 's/^source = "git+\(https:[^?#"]*\).*/\1/p' Cargo.lock | sort -u
}

route_env() {
  local base="${JANKURAI_GIT_MIRROR_BASE:-}"
  [[ -n "$base" ]] || { echo "git-source-route: set JANKURAI_GIT_MIRROR_BASE" >&2; return 2; }
  [[ "$base" =~ ^https://[^[:space:]]+[^/[:space:]]$ ]] || {
    echo "git-source-route: JANKURAI_GIT_MIRROR_BASE must be an https URL without trailing slash" >&2
    return 2
  }
  local n=0 host owner
  # One rewrite per owner seen in the lock file, for both host spellings.
  while read -r owner; do
    for host in github.com www.github.com; do
      printf 'GIT_CONFIG_KEY_%d=url.%s/.insteadOf\nGIT_CONFIG_VALUE_%d=https://%s/%s/\n' \
        "$n" "$base" "$n" "$host" "$owner"
      n=$((n + 1))
    done
  done < <(locked_urls | sed -E "s#^https://(www\.)?github\.com/([^/]+)/.*#\2#" | sort -u)
  printf 'GIT_CONFIG_COUNT=%d\n' "$n"
}

probe_one() { # url -> prints status line; anonymous: no prompt, no credential helper
  local url="$1" out
  if out="$(GIT_TERMINAL_PROMPT=0 git -c credential.helper= ls-remote --exit-code "$url" HEAD 2>&1)"; then
    echo "reachable  $url"
  else
    echo "unreachable  $url  ($(printf '%s' "$out" | tail -n1))"
    return 1
  fi
}

cmd="${1:-}"; shift || true
case "$cmd" in
  env) route_env ;;
  run)
    [[ "${1:-}" == "--" ]] && shift
    [[ $# -gt 0 ]] || { echo "usage: $0 run -- <command...>" >&2; exit 2; }
    while IFS= read -r kv; do export "$kv"; done < <(route_env)
    exec "$@"
    ;;
  probe)
    bad=0
    while read -r url; do
      [[ "$url" =~ $owner_re ]] || continue
      probe_one "$url" || bad=1
      if [[ -n "${JANKURAI_GIT_MIRROR_BASE:-}" ]]; then
        probe_one "${JANKURAI_GIT_MIRROR_BASE}/${url#https://github.com/*/}" || bad=1
      fi
    done < <(locked_urls)
    exit "$bad"
    ;;
  *) echo "usage: $0 {run -- <command...>|probe|env}" >&2; exit 2 ;;
esac
