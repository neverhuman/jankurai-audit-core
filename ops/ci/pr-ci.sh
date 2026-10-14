#!/usr/bin/env bash
# Forge CI entrypoint. .jeryu/ci.toml declares this script as the `required`
# lane, and the forge gate runs it for every pull request head (there is no
# `just required` recipe, so the gate falls back to this file).
#
# It runs the complete composition-bound gate, in the same order the pre-push
# hook runs it: required -> security -> tool adoption -> final audit (see
# ops/ci/quality-gates.sh). Whatever the outcome, it then writes one
# digest-bound evidence bundle, so a failing head still leaves its partial
# evidence behind for review.
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"

collect() {
  local gate_exit=$?
  trap - EXIT
  local collect_exit=0
  bash ops/ci/collect-evidence.sh "$gate_exit" || collect_exit=$?
  if [[ "$gate_exit" -ne 0 ]]; then
    exit "$gate_exit"
  fi
  exit "$collect_exit"
}
trap collect EXIT

log "pr-ci: full quality gate (required, security, tool adoption, audit)"
bash ops/ci/quality-gates.sh
