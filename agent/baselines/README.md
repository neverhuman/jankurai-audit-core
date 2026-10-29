# Accepted Baselines

This directory holds reviewed score baselines used by ratchet and public badge checks.

Rules:

- Baselines must come from a clean accepted tree, not from a candidate CI run.
- Generate candidate reports under `target/jankurai/` first.
- Review the report, then copy the accepted JSON here in a dedicated baseline update.
- Do not hand-edit baseline JSON.
- CI copies `agent/baselines/main.repo-score.json` to `target/jankurai/accepted-baseline.json` before the final ratchet audit.

## Auditor 1.7.6 baseline refresh

The report was generated over clean accepted main `c1aaefada648b8af1879c30f6461fa44219eef83`
(tree `1d26bcb75cdc4339ac134c3846a32fa73d4b0bc2`) using the 1.7.6 executable
built from release source `44c4a9c0b41c4e14d657bfa74a7977ef1b9a01de`. Its SHA-256 is
`3341aaef5e946e4a56eb9f72765f569d0ca185e61593019c4d21b1fae6b81df6`.

Before generation, both `target/` and `.jankurai/` were absent. Existing candidate
outputs and the same lane's build cache were preserved outside the audited
checkout and restored only after generation. No candidate security, proof,
coverage or other generated report was available to the audit. The resulting
`security_evidence` is empty; no security run from another Git tree is embedded.

With that measured executable on PATH, from the clean accepted checkout:

```bash
set -euo pipefail
test "$(git rev-parse HEAD)" = c1aaefada648b8af1879c30f6461fa44219eef83
test -z "$(git status --porcelain)"
test ! -e target && test ! -e .jankurai
jankurai audit . --full --mode standard --no-badge --no-score-history \
  --json target/jankurai/accepted-main-176.json \
  --md target/jankurai/accepted-main-176.md
test -z "$(git status --porcelain)"
```

The generated JSON SHA-256 was `162fd41258f30c5096ba40bf6a3c2f65c75dd66c7114e754cb3e5448fea4e41a`.
Generation timestamps and tool/environment identities are evidence of that run,
not a promise of byte-identical reports on a different host. Score and raw score
remain 96, with zero caps and findings; policy source and thresholds are unchanged.
The report embeds its accepted Git head and refreshes the engine identity needed
by the ratchet. Candidate security proof is produced separately by the full gate.
