#!/usr/bin/env bash
# Proves the forge gate cannot be weakened silently. It replaces the GitHub
# workflow's required-job aggregate and its test, which went with the workflow:
#
#   - .jeryu/ci.toml is a schema 2 jeryu declaration whose only lane is
#     `required`, running exactly `bash ops/ci/pr-ci.sh`;
#   - the Justfile has no `required` recipe, which the forge gate would run
#     in place of ops/ci/pr-ci.sh;
#   - ops/ci/pr-ci.sh runs ops/ci/quality-gates.sh and always collects
#     evidence;
#   - ops/ci/quality-gates.sh runs required, security, tool adoption and the
#     final audit, in that order, with nothing made advisory;
#   - no GitHub workflow is committed (GitHub is a publishing mirror).
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

failures=0
ok()  { printf 'ok: %s\n' "$1"; }
bad() { printf 'FAIL: %s\n' "$1" >&2; failures=$((failures + 1)); }
check() { local name="$1"; shift; if "$@"; then ok "$name"; else bad "$name"; fi; }

decl=.jeryu/ci.toml
# Strip comments and blank lines so only the declaration itself is compared.
body="$(grep -v -E '^[[:space:]]*(#|$)' "$decl")"
want='schema_version = "2"
provider = "jeryu"
[[lane]]
name = "required"
command = "bash ops/ci/pr-ci.sh"'
check "$decl declares exactly one lane: required -> bash ops/ci/pr-ci.sh" \
  test "$body" = "$want"

check "Justfile has no required recipe that would replace ops/ci/pr-ci.sh on the forge" \
  bash -c "! grep -qE '^required([[:space:]]|:)' Justfile"

pr_ci="$(grep -v -E '^[[:space:]]*#' ops/ci/pr-ci.sh)"
check "pr-ci.sh runs the quality gates" \
  grep -qxF 'bash ops/ci/quality-gates.sh' <<<"$pr_ci"
check "pr-ci.sh collects evidence on every exit" \
  grep -qxF 'trap collect EXIT' <<<"$pr_ci"
# shellcheck disable=SC2016 # the literal "$gate_exit" is the text to match
check "pr-ci.sh evidence collection uses the gate's exit code" \
  grep -qF 'bash ops/ci/collect-evidence.sh "$gate_exit"' <<<"$pr_ci"

gates="$(grep -E '^bash ' ops/ci/quality-gates.sh || true)"
want_gates='bash ops/ci/required.sh
bash ops/ci/security.sh
bash ops/ci/tool-adoption.sh
bash ops/ci/audit.sh'
check "quality-gates.sh runs required, security, tool-adoption, audit in order" \
  test "$gates" = "$want_gates"
check "quality-gates.sh keeps errexit and pipefail" \
  grep -qxF 'set -euo pipefail' ops/ci/quality-gates.sh
check "no quality gate is masked with || true or || :" \
  bash -c "! grep -nE '\|\|[[:space:]]*(true|:)([[:space:]]|$)' ops/ci/quality-gates.sh ops/ci/required.sh ops/ci/security.sh ops/ci/audit.sh"

check "no GitHub workflow is committed (GitHub is a publishing mirror)" \
  bash -c "! git ls-files --error-unmatch .github/workflows >/dev/null 2>&1 && [[ ! -d .github/workflows ]]"

if ((failures)); then
  printf '%s: %d check(s) failed\n' "$0" "$failures" >&2
  exit 1
fi
printf '%s: forge gate contract holds\n' "$0"
