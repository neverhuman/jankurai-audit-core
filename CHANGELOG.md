# Changelog

All notable changes to jankurai-core are documented in this file. The format is
based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
The authoritative version string lives in [`VERSION`](VERSION).

## [Unreleased]

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
