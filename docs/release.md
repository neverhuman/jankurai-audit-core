# Release process

This document is the release control surface for jankurai-core. It covers the
version source, the release notes, the release automation, integrity and SBOM
evidence, and rollback. Launch gates require every section below to be backed by
a real artifact or command.

## Version source

The single source of truth for the version is the [`VERSION`](../VERSION) file at
the repository root. The crate version in `crates/jankurai/Cargo.toml` and any
release tag MUST match `VERSION`. Tags follow the family pattern
`jankurai-core-v<MAJOR.MINOR.PATCH>-split.<N>` as described in
[`SPLIT.md`](../SPLIT.md).

## Release notes

Release notes are generated when the release is cut, from the commits since the
previous release (their subjects and `Todo:` trailers). A change never adds a
[`CHANGELOG.md`](../CHANGELOG.md) entry by hand. At tag time the generated notes
are written under a heading that matches the new `VERSION`, and the `Unreleased`
section is rolled up into it.

## Release automation

A release of this repository is an immutable tag on a commit that passed the
forge gate. This repository publishes no binaries: the hub
(`root/jankurai`) builds and key-signs the release binaries on our servers from
the tagged family revisions (see the hub's `docs/release.md`).

1. Bump [`VERSION`](../VERSION) and write the generated notes for the commits
   since the previous release into [`CHANGELOG.md`](../CHANGELOG.md), rolling the
   `Unreleased` section up under the new version heading.
2. Run the full gate locally: `bash scripts/ci-local.sh pr-ci` (the exact
   command the forge runs) or `just check`.
3. Open a pull request with the version commit. The forge gate runs the
   `required` lane declared in [`.jeryu/ci.toml`](../.jeryu/ci.toml),
   [`ops/ci/pr-ci.sh`](../ops/ci/pr-ci.sh): format, lint, fast lane, security
   (secrets, dependencies, SBOM, vulnerabilities), tool adoption and the
   ratchet self-audit, then writes the evidence bundle
   `target/jankurai/evidence/<commit>/` (artifacts, `MANIFEST.sha256` and a
   `receipt.json` naming the commit, tree and gate result) on the gate host.
4. After the merge, tag the release commit with
   `jankurai-core-v<version>-split.<N>`. The tag mirror in
   [`.jeryu/repo.toml`](../.jeryu/repo.toml) publishes the immutable tag to the
   public GitHub mirror.

GitHub is a publishing mirror only and runs no workflows for this repository.
The GitHub-era `ci-<sha>` tags are no longer published; the forge's
exact-head `required` status and the evidence bundle record that a commit
passed the gate.


Release builds depend on immutable tags, never branches.

## Integrity, provenance, and SBOM

- **Dependency integrity**: builds are reproducible because `Cargo.lock` is
  committed and every CI lane uses `--locked`.
- **SBOM**: the security lane (`tools/security-lane.sh`, run by the forge gate)
  writes a CycloneDX SBOM with `syft` to `target/jankurai/security/sbom.json`,
  scans it with `grype --fail-on high`, and records the
  `Cargo.lock` + SBOM digests in
  `target/jankurai/security/provenance-inputs.sha256`.
- **Provenance**: the same lane runs `gitleaks detect` for secret scanning and
  `cargo audit` / `cargo deny` for advisories, bans and sources; the audit lane
  writes the `repo-score` report and the digest of the exact auditor binary
  that produced it. The evidence bundle keeps all of these per commit.
- **Tool pinning**: the security tools and workflow linters are pinned in
  [`ops/ci/lib.sh`](../ops/ci/lib.sh) and checked by `scripts/ci-doctor.sh`.
  No GitHub Action is part of the release path: GitHub only receives mirrored
  branches and tags.

## Rollback

If a release regresses:

1. Identify the last known-good tag (`jankurai-core-v<version>-split.<N>`).
2. Re-point consumers at that immutable tag; tags are never moved or deleted.
3. Open a revert commit that restores the previous `VERSION` and `CHANGELOG.md`
   state; its commit subject describes the rollback and the next release's
   generated notes pick it up.
4. Re-run `just check` to confirm the rolled-back tree is green before
   re-publishing.

Because tags are immutable and `Cargo.lock` is committed, any prior release can
be rebuilt bit-for-bit from its tag.
