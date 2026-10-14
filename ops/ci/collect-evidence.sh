#!/usr/bin/env bash
# Evidence bundle for one gate attempt.
#
# usage: collect-evidence.sh <gate-exit-code>
#
# Copies the composition-bound evidence set the gate lanes write under
# target/ and .jankurai/ into target/jankurai/evidence/<commit>/ together with
# a SHA-256 manifest and a small JSON receipt naming the commit, the tree and
# the gate result. When the gate passed, every listed artifact must exist and
# a missing one fails the run; when the gate failed, whatever exists is kept
# so the failure can be reviewed.
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"
cd "$REPO_ROOT"

gate_exit="${1:?usage: collect-evidence.sh <gate-exit-code>}"
if [[ ! "$gate_exit" =~ ^[0-9]+$ ]]; then
  printf '[ci] gate exit code must be a number, got: %s\n' "$gate_exit" >&2
  exit 2
fi

commit="$(git rev-parse HEAD)"
tree="$(git rev-parse 'HEAD^{tree}')"
out="target/jankurai/evidence/$commit"
rm -rf "$out"
mkdir -p "$out/files"

# The set the gate must leave behind. Each path is written by one lane:
# fast (audit-fast), security (evidence, sbom), tool adoption (proofbind,
# proofmark, copy-code, vibe and coverage audits, ratchet report) and the
# final audit (repo-score, auditor digests).
evidence=(
  .jankurai/repo-score.json
  .jankurai/repo-score.md
  target/jankurai/repo-score.json
  target/jankurai/repo-score.md
  target/jankurai/auditor-build.jsonl
  target/jankurai/auditor.sha256
  target/jankurai/auditor-reports.sha256
  target/jankurai/audit-fast.json
  target/jankurai/audit-fast.md
  target/jankurai/fast-score.json
  target/jankurai/proofbind/surface-witness.json
  target/jankurai/proofbind/obligations.json
  target/jankurai/proofmark/proofmark-receipt.json
  target/jankurai/proofmark/proof-receipt.json
  target/jankurai/copy-code.json
  target/jankurai/copy-code.md
  target/jankurai/security/evidence.json
  target/jankurai/security/sbom.json
  target/jankurai/security/provenance-inputs.sha256
  target/jankurai/language-bad-behavior.log
  target/jankurai/vibe-coverage.json
  target/jankurai/vibe-coverage.md
  target/jankurai/coverage/coverage-audit.json
  target/jankurai/coverage/coverage-audit.md
)

# Written only when the audit has something to queue; kept when present.
optional=(
  target/jankurai/repair-queue.jsonl
)

keep() {
  mkdir -p "$out/files/$(dirname "$1")"
  cp "$1" "$out/files/$1"
}

missing=()
for path in "${evidence[@]}"; do
  if [[ -f "$path" && ! -L "$path" ]]; then
    keep "$path"
  else
    missing+=("$path")
  fi
done
for path in "${optional[@]}"; do
  if [[ -f "$path" && ! -L "$path" ]]; then
    keep "$path"
  fi
done

(cd "$out/files" && find . -type f -print0 | sort -z | xargs -0 -r sha256sum) > "$out/MANIFEST.sha256"

jq -n \
  --arg commit "$commit" \
  --arg tree "$tree" \
  --argjson gate_exit "$gate_exit" \
  --argjson missing "$(printf '%s\n' "${missing[@]+"${missing[@]}"}" | jq -R 'select(length > 0)' | jq -s .)" \
  --arg manifest_sha256 "$(sha256sum "$out/MANIFEST.sha256" | cut -d' ' -f1)" \
  '{schema: "jankurai-core/gate-evidence/v1", commit: $commit, tree: $tree,
    gate_exit: $gate_exit, gate_passed: ($gate_exit == 0),
    missing: $missing, manifest_sha256: $manifest_sha256}' \
  > "$out/receipt.json"

log "evidence bundle: $out ($(wc -l < "$out/MANIFEST.sha256") files)"

if [[ "$gate_exit" -eq 0 && "${#missing[@]}" -gt 0 ]]; then
  printf '[ci] gate passed but evidence is missing:\n' >&2
  printf '  %s\n' "${missing[@]}" >&2
  exit 1
fi
