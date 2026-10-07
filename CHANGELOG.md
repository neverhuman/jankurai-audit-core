# Changelog

All notable changes to jankurai-core are documented in this file. The format is
based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
The authoritative version string lives in [`VERSION`](VERSION).

## [Unreleased]

## [1.7.3] - 2026-10-06

1.7.3 carries the governed split.6 scoring (provider-aware CI credit), except
where the owner chose 1.7 behaviour (an omitted `fail_on` means critical+high,
and conformance blockers block).

### Added

- A lane declared in `.jeryu/ci.toml` that resolves to no repository content
  (no `just` recipe, `make` target, script or package.json script) is reported
  as a soft (`low`) finding at `.jeryu/ci.toml` naming the lane, as split.6
  does. Such a lane already earned no credit; it used to be dropped silently.

### Fixed

- `jankurai coverage` accepts complete cargo-llvm-cov 0.8.7 LCOV, whose
  `LF`/`LH` can exceed its `DA` records (analyzers `bbd7db9`). Declared totals
  below the captured records, `LH` above `LF`, a record with `LF` but no `DA`,
  and a missing `end_of_record` are still rejected.
- `jankurai proofbind` no longer fails with "required input missing" when the
  change deletes a file: a path Git reports as deleted is a removal, not a
  required input (proof `2f187f1`, now pinned by `rev`).
- `HLT-022-AUTHZ-ISOLATION-GAP` matches its terms (`owner_id`, `tenant_id`,
  `org_id`, `rls`, `admin`, ...) as whole identifier tokens instead of
  substrings, so `rls` inside `urls` or `cdpUrlSetting` no longer applies the
  authz cap (kernel `e66b94c`). `RLS`, `enable_rls`, `tenantId`, `isAdmin`,
  `ROW LEVEL SECURITY` and SQL `CREATE POLICY ... USING (...)` still match.

### Changed

- Public package version and `AUDITOR_VERSION` are `1.7.3`. Standard `0.9.0`
  and schema `1.9.0` are unchanged.
- The kernel pin is `e66b94c1f0c7ceeef85a0dd206d072b19e80d832` and the
  analyzers pin is `8f44aeb385011eeee0601db18cbc644f1913e392`, the same
  revisions `family.lock` names. They carry the split.6 provider-aware CI
  credit (proof CI presence, speed cache hint and security workflow lint read
  the resolved `.jeryu/ci.toml` lane; npm/pnpm/yarn lanes resolve through
  package.json scripts) and the HLT-031 fix that stops reading prose `as` in
  comments as a TypeScript cast.
- `docs/rule-catalog.md` is regenerated for auditor `1.7.3`.
- The bundled workspace-boundary template says "forge remote and merge-request
  work" instead of naming this site's local GitLab and `glab`, and a new
  `init_golden` test rejects any bundled template body carrying a host, home
  path or login. Regenerated cell `AGENTS.md` adapters match.
- The CI and paper tests assert that no home path (`/home/`, `/Users/`, `~/`)
  reaches the audit lane or a rendered paper table, instead of naming one site's
  paths.

## [1.7.2] - 2026-10-02

1.7.2 carries the governed split.5 scoring, except where the owner chose 1.7
behaviour (an omitted `fail_on` means critical+high, and conformance blockers
block).

### Changed

- Public package version and `AUDITOR_VERSION` are `1.7.2`. Standard `0.9.0` and schema `1.9.0` are unchanged.
- The kernel pin is `b4ef74d9d59651375dd5db1d1e9b7d953e447d17` and the
  analyzers pin is `d9a57332c3d1951b2de0d63f877416834be9d04a`, the same
  revisions `family.lock` names. 1.7.1 pinned analyzers `eeda8c0` here while
  `family.lock` named `6cd5551`.
- CI-cap findings (no audit lane, no security lane, no secret or dependency
  scanning) and below-floor dimension findings point a forge-gated repository at
  its `.jeryu/ci.toml` declaration instead of `.github/workflows`. GitHub
  Actions and no-CI repositories keep their paths and wording.
- Tool adoption credits CI evidence as split.5 does; HLT-001 ignores Rust
  comments as split.5 does; HLT-047 checks CLAUDE.md/GEMINI.md reference
  AGENTS.md; ZYAL placement (HLT-024) and repo-rot names (HLT-040) are scoped to
  real signals. These live in the kernel and analyzers.
- `docs/rule-catalog.md` is regenerated for auditor `1.7.2`.

## [1.7.1]

### Changed

- Public package version and `AUDITOR_VERSION` are `1.7.1`. Standard `0.9.0` and schema `1.9.0` are unchanged.
- The kernel pin is `ci-cd94f46d3b4ba3b60780b996ed5f7d1f559dfaef`.
- The accepted ratchet baseline is a fresh full audit at auditor `1.7.1`. Score stays 91 with no hard findings. The previous baseline was auditor `1.7.0`, and ratchet treats that identity change as policy drift.

### Added

- `docs/rule-catalog.md` lists every registry rule with name, lane, severity,
  status, confidence, and repair reason. `committed_rule_catalog_matches_registry`
  fails when the file drifts from `jankurai rules export`.
- Root `Justfile` command surface with `setup`, `fast`, `check`, `security`, and
  `audit` lanes for one-command setup and validation.
- GitHub Actions CI (`.github/workflows/ci.yml`) with build, security, and
  jankurai audit jobs, all third-party actions pinned to commit SHAs.
- Agent-readable documentation: `README.md`, `docs/boundaries.md`,
  `docs/release.md`, and `docs/exceptions.md`.

### Changed

- Re-scoped `agent/boundaries.toml`, `agent/owner-map.json`,
  `agent/test-map.json`, `agent/generated-zones.toml`, and
  `agent/proof-lanes.toml` to the paths that exist in this single-purpose repo.

## [1.6.10] - 2026-06-12

### Added

- Initial split-family extraction of the jankurai auditor Rust crate.
