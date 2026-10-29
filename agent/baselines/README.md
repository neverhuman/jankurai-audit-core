# Accepted Baselines

This directory holds reviewed score baselines used by ratchet and public badge checks.

Rules:

- Baselines must come from a clean accepted tree, not from a candidate CI run.
- Generate candidate reports under `target/jankurai/` first.
- Review the report, then copy the accepted JSON here in a dedicated baseline update.
- Do not hand-edit baseline JSON.
- CI copies `agent/baselines/main.repo-score.json` to `target/jankurai/accepted-baseline.json` before the final ratchet audit.

## Auditor 1.7.6 baseline regeneration (2026-10-29)

This report audits accepted core main `78010ed7d503c2079917afc7e6c16942c0b23a92` (tree
`26c657a4d154a93bfbd7205c83deab8d8f7cadf5`), with a clean working tree and no
prior `target/` or `.jankurai/` artifacts. It is not a candidate-tree report.

Generator source is core commit `8ff4a992ba508ece2bf8c81f47cb53ffbfc3de4b` in this
branch, built with `CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only
cargo build --locked --offline -p jankurai --bin jankurai`. Its lock resolves
the kernel through the forge at immutable tag
`ci-65e8344ff3718ede0e05c55ee27003d9673c17f0`. The generator binary SHA-256 is
`8e660488654dd1bdd9f5026587dcf2c0c4888019a18098161003d55e387a972c`.

With that binary outside the audited tree, the regeneration command was:

```sh
/path/to/jankurai-176 audit . --full --mode standard --no-badge --no-score-history \
  --json target/jankurai/accepted-main-176.json --md target/jankurai/accepted-main-176.md
```

The report SHA-256 is `747989a85a15f8fd8b8a25350a8c2d10ac0ff76f026f969d5a6bc1e662776a2c`.
Score and raw score remain 96, findings/caps remain empty, the minimum remains
85 and ratchet allowed drop remains zero. The policy fingerprint changes because
`effective_policy_fingerprint` includes the auditor version. The badge is
regenerated from this accepted-main report by `jankurai badge`.
