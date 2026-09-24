# Jankurai rule catalog

Generated from `jankurai rules export`. Regenerate this file from the registry and keep the drift test green.

Language bad-behavior rules can false-positive on idiomatic code. This catalog does not add per-language allowlist goldens.

- rules: `48`
- auditor: `1.7.0`

| Rule | Name | Lane | Severity | Status | Confidence | Repair |
| --- | --- | --- | --- | --- | --- | --- |
| `HLT-001-DEAD-MARKER` | Future-hostile language and placeholder markers | `fast` | `high` | `stable` | `medium` | marker cleanup is scoped but still needs proof that intent was not removed |
| `HLT-002-GENERATED-MUTATION` | Generated zone mutation risk | `contract` | `high` | `stable` | `high` | generated artifacts require source or generator review before repair |
| `HLT-003-OWNERLESS-PATH` | Ownerless path | `fast` | `high` | `stable` | `medium` | ownership map edits are bounded but affect agent routing |
| `HLT-004-UNMAPPED-PROOF` | Unmapped proof lane | `fast` | `high` | `stable` | `medium` | proof routing repairs are bounded but must preserve validation coverage |
| `HLT-005-PYTHON-PRODUCT-TRUTH` | Python product truth spill | `contract` | `high` | `stable` | `high` | product truth boundary changes require architectural review |
| `HLT-006-DIRECT-DB-WRONG-LAYER` | Direct DB access from wrong layer | `db` | `high` | `stable` | `high` | database layer movement can affect durable data behavior |
| `HLT-007-HANDWRITTEN-CONTRACT` | Handwritten contract drift | `contract` | `high` | `stable` | `high` | contract drift repair can alter public API compatibility |
| `HLT-008-FALSE-GREEN-RISK` | False green test risk | `fast` | `high` | `stable` | `medium` | test proof repairs are scoped but can create false confidence |
| `HLT-009-GENERATED-SECURITY` | Generated security gap | `security` | `high` | `stable` | `high` | generated security behavior must be reviewed before repair |
| `HLT-010-SECRET-SPRAWL` | Secret-like content detected | `security` | `critical` | `stable` | `high` | secret exposure requires human-led rotation and incident review |
| `HLT-011-PROMPT-INJECTION` | Prompt injection risk | `security` | `high` | `stable` | `high` | trusted policy changes require human review against prompt injection |
| `HLT-012-OVERBROAD-AGENCY` | Overbroad agent agency | `security` | `high` | `stable` | `high` | agent authority changes require explicit human approval |
| `HLT-013-RENDERED-UX-GAP` | Rendered UX proof gap | `web` | `high` | `stable` | `high` | user-facing UX proof gaps require rendered evidence review |
| `HLT-014-A11Y-GAP` | Accessibility proof gap | `web` | `high` | `stable` | `high` | accessibility fixes require rendered and assistive proof review |
| `HLT-015-CONTEXT-SETUP-GAP` | Context and setup gap | `fast` | `medium` | `stable` | `medium` | context setup repairs are bounded but affect agent bootstrap behavior |
| `HLT-016-SUPPLY-CHAIN-DRIFT` | Supply-chain drift | `security` | `high` | `stable` | `high` | supply-chain changes require provenance and dependency review |
| `HLT-017-OPAQUE-OBSERVABILITY` | Opaque observability | `observability` | `high` | `stable` | `medium` | observability repairs are typically scoped to telemetry and error receipts |
| `HLT-018-PERF-CONCURRENCY-DRIFT` | Performance and concurrency drift | `fast` | `medium` | `stable` | `medium` | performance and concurrency repairs require targeted proof but can be planned narrowly |
| `HLT-019-STREAMING-RUNTIME-DRIFT` | Streaming runtime drift | `db` | `high` | `stable` | `high` | streaming runtime boundaries can affect production integration behavior |
| `HLT-020-CI-HARDENING-GAP` | CI workflow hardening gap | `security` | `high` | `stable` | `high` | CI hardening changes alter release authority and require review |
| `HLT-021-DESTRUCTIVE-MIGRATION` | Destructive SQL migration without safety evidence | `db-migration-analyze` | `high` | `stable` | `high` | destructive migrations require documented rollback, backfill, and lock safety |
| `HLT-022-AUTHZ-ISOLATION-GAP` | Authorization and data-isolation proof gap | `db` | `high` | `stable` | `medium` | authorization and tenant isolation gaps require reviewed negative tests |
| `HLT-023-INPUT-BOUNDARY-GAP` | Input boundary proof gap | `security` | `high` | `stable` | `medium` | unsafe sinks and unvalidated boundaries can become exploitable behavior |
| `HLT-024-AGENT-TOOL-SUPPLY-GAP` | Agent tool supply-chain proof gap | `security` | `high` | `stable` | `medium` | agent tool and MCP trust changes alter execution authority |
| `HLT-025-RELEASE-READINESS-GAP` | Release readiness proof gap | `release` | `high` | `stable` | `medium` | launch gates must prove backups, monitoring, security, and rollback |
| `HLT-026-COST-BUDGET-GAP` | Cost and budget proof gap | `release` | `medium` | `stable` | `medium` | cost controls are bounded but need explicit budgets and stop conditions |
| `HLT-027-HUMAN-REVIEW-EVIDENCE-GAP` | Human review evidence gap | `audit` | `medium` | `stable` | `medium` | human review and proof claims must be backed by reproducible evidence |
| `HLT-028-BOUNDARY-EVIDENCE-GAP` | Audited runtime boundary evidence gap | `contract` | `high` | `stable` | `high` | runtime boundary reclassification can alter target-stack cap pressure and needs deterministic evidence |
| `HLT-029-RUST-BAD-BEHAVIOR` | Rust bad behavior | `fast` | `high` | `stable` | `high` | Rust bad-behavior findings require a local proof or design change before repair |
| `HLT-030-SQL-BAD-BEHAVIOR` | SQL bad behavior | `db` | `high` | `stable` | `high` | SQL bad-behavior findings require query or migration proof before repair |
| `HLT-031-TYPESCRIPT-BAD-BEHAVIOR` | TypeScript bad behavior | `fast` | `high` | `stable` | `high` | TypeScript bad-behavior findings require boundary proof before repair |
| `HLT-032-DOCKER-BAD-BEHAVIOR` | Docker bad behavior | `security` | `high` | `stable` | `high` | Docker bad-behavior findings require a reviewed container or build proof |
| `HLT-033-PYTHON-BAD-BEHAVIOR` | Python bad behavior | `contract` | `high` | `stable` | `high` | Python bad-behavior findings require boundary proof before repair |
| `HLT-034-CI-BAD-BEHAVIOR` | CI bad behavior | `security` | `high` | `stable` | `high` | CI bad-behavior findings require reviewed workflow evidence before repair |
| `HLT-035-GIT-BAD-BEHAVIOR` | Git bad behavior | `audit` | `high` | `stable` | `high` | Git bad-behavior findings require reviewed automation or hook evidence before repair |
| `HLT-036-GITTOOLS-BAD-BEHAVIOR` | GitTools bad behavior | `audit` | `high` | `stable` | `high` | Git tooling bad-behavior findings require reviewed hook, CI, or policy-tool evidence before repair |
| `HLT-037-RELEASE-BAD-BEHAVIOR` | Release bad behavior | `release` | `high` | `stable` | `high` | Release bad-behavior findings require reviewed version, tag, artifact, provenance, and rollback evidence before repair |
| `HLT-038-REFERENCE-PROFILE-STRUCTURE-GAP` | Reference profile structure gap | `fast` | `medium` | `stable` | `medium` | reference-profile structure repairs are scoped routing changes with migration steering |
| `HLT-039-WEB-SECURITY-BAD-BEHAVIOR` | Web security bad behavior | `security` | `high` | `stable` | `high` | web security findings can alter auth, session, CORS, or deployment behavior and require reviewed security proof |
| `HLT-040-REPO-ROT-BAD-BEHAVIOR` | Repository rot bad behavior | `audit` | `medium` | `stable` | `medium` | repo-rot cleanup is usually scoped but must not delete live compatibility, migration, or generated evidence |
| `HLT-041-COMMENT-HYGIENE` | Dangerous comment hygiene | `fast` | `high` | `stable` | `high` | comment cleanup is scoped but must not remove intentional safety documentation |
| `HLT-042-CI-LOCAL-PARITY` | CI lacks local-run parity | `fast` | `high` | `stable` | `high` | extracting workflow steps into ops/ci/*.sh and adding local runners is mechanical |
| `HLT-043-COPY-PASTE-BAD-BEHAVIOR` | Copy-code redundancy in active source | `copy-code` | `high` | `stable` | `high` | high-confidence copy-code classes should be consolidated under one owner before repair |
| `HLT-044-WORKTREE-SPRAWL` | Worktree sprawl across same-origin sibling checkouts | `audit` | `high` | `experimental` | `medium` | consolidating parallel same-origin checkouts moves working state and needs human-led review before repair |
| `HLT-045-GENERATED-ZONE-GOVERNANCE` | Hand-edit inside a declared generated zone | `contract` | `medium` | `experimental` | `medium` | generated-zone hand-edits should be re-derived from the source generator, not patched in place |
| `HLT-046-UNNECESSARY-VARIETY` | Redundant variety where consistency is expected | `copy-code` | `medium` | `experimental` | `medium` | reconciling diverging same-name definitions into one canonical shape is scoped but needs review that callers still agree |
| `HLT-047-CANONICAL-README` | README drifts from the canonical agent-native shape | `fast` | `medium` | `experimental` | `medium` | adding the missing README links, stack statement, badge, or quick-start is mechanical doc editing |
| `HLT-048-CANONICAL-CI-GAP` | CI drifts from the canonical local-parity shape | `audit` | `medium` | `experimental` | `medium` | moving inline CI steps into ops/ci/*.sh, pinning action SHAs, and adding a jankurai audit lane is mechanical but should be proven by a green run |
