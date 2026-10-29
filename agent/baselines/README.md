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
built from release source `0784c619a037360b336c6a2e10bced7aeb932981`. Its SHA-256 is
`d41c73fb2bd766840a09c5b866c72ec46be7c58d0360ecdd7763a9d5e03ad9bb`. The command was
`jankurai audit . --full --mode standard --no-badge --no-score-history --json target/jankurai/accepted-main-176.json --md target/jankurai/accepted-main-176.md`.
The tree was clean before and after. Score and raw score remain 96, with zero caps
and findings; no policy source or score threshold changed. The report embeds its
accepted Git head. This refresh updates the engine identity required by the ratchet.
