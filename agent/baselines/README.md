# Accepted Baselines

This directory holds reviewed score baselines used by ratchet and public badge checks.

Rules:

- Baselines must come from a clean accepted tree, not from a candidate CI run.
- Generate candidate reports under `target/jankurai/` first.
- Review the report, then copy the accepted JSON here in a dedicated baseline update.
- Do not hand-edit baseline JSON.
- CI copies `agent/baselines/main.repo-score.json` to `target/jankurai/accepted-baseline.json` before the final ratchet audit.

## Auditor 1.7.6 baseline refresh

The report was generated over clean accepted main `78010ed7d503c2079917afc7e6c16942c0b23a92`
(tree `26c657a4d154a93bfbd7205c83deab8d8f7cadf5`) using the 1.7.6 executable
built from release source `151c7b25e1f52d9f4f6a10a6c8d383b21b5f5e2e`. Its SHA-256 is
`8357a80cf8dab371a88bf854ef69df09fa977c7fb7976100dcc3cc1144bec8e3`.

Before generation, both `target/` and `.jankurai/` were absent. Existing candidate
outputs and the same lane's build cache were preserved outside the audited
checkout and restored only after generation. No candidate security, proof,
coverage or other generated report was available to the audit. The resulting
`security_evidence` is empty; no security run from another Git tree is embedded.

With that measured executable on PATH, from the clean accepted checkout:

```bash
set -euo pipefail
test "$(git rev-parse HEAD)" = 78010ed7d503c2079917afc7e6c16942c0b23a92
test -z "$(git status --porcelain)"
test ! -e target && test ! -e .jankurai
jankurai audit . --full --mode standard --no-badge --no-score-history \
  --json target/jankurai/accepted-main-176.json \
  --md target/jankurai/accepted-main-176.md
test -z "$(git status --porcelain)"
```

The generated JSON SHA-256 was `7b68af52960106dae57cbb8e033e236123c59fe670ccd8e60604ba5279cc8f6f`.
Generation timestamps and tool/environment identities are evidence of that run,
not a promise of byte-identical reports on a different host. Score and raw score
remain 96, with zero caps and findings; policy source and thresholds are unchanged.
The report embeds its accepted Git head and refreshes the engine identity needed
by the ratchet. Candidate security proof is produced separately by the full gate.

The generator source is an ancestor in this PR, not a separate candidate ref.
Build it with `cargo build --locked --offline -p jankurai` at the exact source
commit above, preserve that executable, and audit the accepted main shown
above from the same clean owned lane. Its kernel source is the canonical forge
URL and commit `bc246e57a7dd784589c250ff7d7237ce7691864e` recorded by Cargo.lock.
