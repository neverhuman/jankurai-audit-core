#!/usr/bin/env bash
# Cold-host bootstrap for the locked Git sources (see docs/bootstrap.md).
#
#   git-source-route.sh run   -- <command...>   run a command with the route applied
#   git-source-route.sh probe                   reachability of each locked source
#   git-source-route.sh verify                  cold-cache fetch of every locked source
#   git-source-route.sh env                     print the GIT_CONFIG_* assignments
#
# JANKURAI_GIT_MIRROR_BASE names the trusted mirror, e.g. https://host/git/org
# (no trailing slash). It rewrites https://github.com/<owner>/ and
# https://www.github.com/<owner>/ for the process only: no global or repository
# git config is written, and Cargo.toml / Cargo.lock (revs, tags, checksums) are
# not touched, so dependency identity is unchanged. Cargo still verifies the
# locked commit ids after fetching.
set -euo pipefail
self="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/$(basename "${BASH_SOURCE[0]}")"
cd "$(dirname "$self")/.."

owner_re='^https://(www\.)?github\.com/[A-Za-z0-9_.-]+/'

locked_urls() {
  sed -n 's/^source = "git+\(https:[^?#"]*\).*/\1/p' Cargo.lock | sort -u
}

locked_revs() {
  sed -n 's/^source = "git+[^#]*#\([0-9a-f]\{40\}\)".*/\1/p' Cargo.lock | sort -u
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

mirror_url() { # locked source url -> same repository on the trusted mirror
  # The mirror flattens the GitHub owner away, exactly as the rewrite does, so
  # both host spellings land on the same mirror repository.
  printf '%s/%s\n' "${JANKURAI_GIT_MIRROR_BASE%/}" \
    "$(sed -E 's#^https://(www\.)?github\.com/[^/]+/##' <<<"$1")"
}

probe_one() { # mode url -> prints status line
  # anonymous: no prompt and no credential helper, so it reports what the public
  # can fetch. authenticated: the host's normal helpers, which is what the forge
  # route uses. The two are reported separately and never conflated.
  local mode="$1" url="$2" out
  local -a auth=()
  [[ "$mode" == anonymous ]] && auth=(-c credential.helper=)
  if out="$(GIT_TERMINAL_PROMPT=0 git "${auth[@]}" ls-remote --exit-code "$url" HEAD 2>&1)"; then
    echo "reachable    $mode  $url"
  else
    echo "unreachable  $mode  $url  ($(printf '%s' "$out" | tail -n1))"
    return 1
  fi
}

verify_cold_cache() { # fetch every locked source into an empty CARGO_HOME
  # Deliberately not under /tmp: it is cleared at boot and may be small.
  local root home
  root="${JANKURAI_COLD_CACHE_DIR:-$PWD/.agent}"
  mkdir -p "$root"
  home="$(mktemp -d "$root/jankurai-coldcache.XXXXXX")" ||
    { echo "git-source-route: could not create a cold cache directory under $root" >&2; return 1; }
  trap 'rm -rf "$home"' RETURN
  local lock_before lock_after rc=0
  lock_before="$(sha256sum Cargo.lock)"

  CARGO_HOME="$home" CARGO_NET_GIT_FETCH_WITH_CLI=true \
    bash "$self" run -- cargo fetch --locked >&2 || rc=1

  # Every locked commit must now exist in the cold cache, fetched through the
  # route: that is what proves the pins, not just the repositories, are served.
  local rev found db
  while read -r rev; do
    found=
    for db in "$home"/git/db/*; do
      [[ -d "$db" ]] || continue
      if git -C "$db" cat-file -e "${rev}^{commit}" 2>/dev/null; then found="$db"; break; fi
    done
    if [[ -n "$found" ]]; then
      echo "fetched  $rev  $(basename "$found")"
    else
      echo "MISSING  $rev"; rc=1
    fi
  done < <(locked_revs)

  lock_after="$(sha256sum Cargo.lock)"
  [[ "$lock_before" == "$lock_after" ]] || { echo "Cargo.lock changed" >&2; rc=1; }
  return "$rc"
}

cmd="${1:-}"; shift || true
case "$cmd" in
  env) route_env ;;
  run)
    if [[ "${1:-}" == "--" ]]; then shift; fi
    [[ $# -gt 0 ]] || { echo "usage: ${0##*/} run -- <command...>" >&2; exit 2; }
    while IFS= read -r kv; do export "$kv"; done < <(route_env)
    exec "$@"
    ;;
  probe)
    bad=0
    while read -r url; do
      [[ "$url" =~ $owner_re ]] || continue
      probe_one anonymous "$url" || bad=1
      if [[ -n "${JANKURAI_GIT_MIRROR_BASE:-}" ]]; then
        probe_one authenticated "$(mirror_url "$url")" || bad=1
      fi
    done < <(locked_urls)
    exit "$bad"
    ;;
  mirror-url)
    [[ $# -eq 1 ]] || { echo "usage: ${0##*/} mirror-url <locked-source-url>" >&2; exit 2; }
    [[ -n "${JANKURAI_GIT_MIRROR_BASE:-}" ]] || { echo "git-source-route: set JANKURAI_GIT_MIRROR_BASE" >&2; exit 2; }
    [[ "$1" =~ $owner_re ]] || { echo "git-source-route: not a GitHub source URL: $1" >&2; exit 2; }
    mirror_url "$1"
    ;;
  verify)
    route_env >/dev/null   # refuses an unset or non-https mirror base
    verify_cold_cache
    ;;
  *) echo "usage: ${0##*/} {run -- <command...>|probe|verify|mirror-url <url>|env}" >&2; exit 2 ;;
esac
