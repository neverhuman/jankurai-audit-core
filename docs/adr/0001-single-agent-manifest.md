# ADR 0001 — One `agent/jankurai.toml` instead of ten adoption files

Status: proposed
Date: 2026-10-15
Applies to: jankurai-core (auditor and CLI), every repository that adopts the standard
Supersedes: nothing
Implementation: follow-up work; this ADR decides the shape only

## Context

A repository adopting jankurai today is asked for ten files under `agent/`:

| File | Carries repo knowledge? |
| --- | --- |
| `agent/owner-map.json` | yes — path prefixes to owners |
| `agent/test-map.json` | yes — path prefixes to runnable proof commands |
| `agent/boundaries.toml` | partly — domain paths and forbidden imports |
| `agent/proof-lanes.toml` | partly — lane commands and required artifacts are real, the standard lane set is not |
| `agent/audit-policy.toml` | rarely — long, but almost every key repeats the standard's own value |
| `agent/generated-zones.toml` | mostly boilerplate — usually `target/` or `dist/` |
| `agent/tool-adoption.toml` | boilerplate — one `mode` per known tool id |
| `agent/standard-version.toml` | boilerplate — versions the auditor already knows |
| `agent/JANKURAI_STANDARD.md` | boilerplate — a pointer to the published standard |
| `agent/jankurai-install.toml` | generated; `DO NOT EDIT BY HAND` already |

Only owners, test routes, boundaries and a few lane commands hold anything a human
has to think about. `agent/audit-policy.toml` in this repository is 90 lines of
`[caps]`, `[limits]`, `[scan]`, `[canonical]` and `[precommit_gate]`, and nearly all
of it restates the published standard. The rest is a tax charged ten times per
repository, and the family has sixteen of them.

The auditor does not read these files through one loader. `agent/audit-policy.toml`
is read in `crates/jankurai/src/audit/outcome.rs`, `agent/standard-version.toml` in
`crates/jankurai/src/audit/mod.rs`, six of them are digested by
`manifest_fingerprints()` (same file, around line 516) and again by
`repo_manifest_fingerprints()` in `crates/jankurai/src/commands/proof.rs`, a list of
twelve control files is required by `jankurai doctor`, and seven are written from
`crates/jankurai/templates/agent/`. Any consolidation therefore touches several call
sites, which is the main reason to decide the shape before writing code.

There is already a precedent for moving a control file without breaking repositories:
`jankurai doctor` accepts `.jankurai/repo-score.json` *or* the legacy
`agent/repo-score.json` through `push_local_or_legacy_file_check()`.

## Decision

Add one optional file, `agent/jankurai.toml`, with a section per concern:

```toml
schema_version = "1.0.0"

[policy]
minimum_score = 90
fail_on = ["critical", "high"]
advisory_on = ["medium"]

# Nested policy subsections keep today's shape: [policy.caps], [policy.limits],
# [policy.scan], [policy.canonical], [policy.precommit_gate].

[owners]
workspace = "jankurai-core"

[owners.paths]
"crates/jankurai/src/audit" = "tools"
"ops/" = "ops"

[proof.tests]
"crates/jankurai" = "cargo test -p jankurai --locked"

[[proof.lane]]
name = "fast"
command = "just fast"
purpose = "run the jankurai audit quickly with JSON and Markdown on stdout"

[zones]
[[zones.zone]]
path = "target/"
source = "crates/jankurai (Cargo workspace)"
command = "cargo build --workspace --locked"

[boundaries]
stack = { id = "rust-ts-vite-react-postgres-bounded-python", version = "0.5.0" }
[boundaries.rust]
domain_paths = ["crates/jankurai/src/domain"]
forbidden_domain_imports = ["std::fs", "std::env"]
```

Rules that make this safe to land:

1. **Section present wins.** For each concern, a present section of
   `agent/jankurai.toml` is the only source read; the matching separate file is
   ignored, not merged. Per-key merging across the two layouts would make the
   effective policy unreadable and the fingerprints ambiguous. Filling unset keys of
   a present section from the built-in defaults is not that kind of merge — it is
   resolution, and it already happens for `agent/audit-policy.toml` today.
2. **Section absent falls back.** A missing section falls back to the separate file,
   and a missing separate file falls back to the built-in default (see the companion
   defaults proposal). Existing repositories that never add the new file behave
   exactly as they do today, byte for byte and digest for digest.
3. **One loader.** Introduce `crates/jankurai/src/config/` returning a single
   `RepoConfig` with, per concern, the resolved value *and* its provenance
   (`Unified`, `File(path)`, `Default`). Every present call site — `audit::outcome`,
   `audit::mod`, `commands::doctor`, `commands::proof`, `init` — goes through it. No
   second reader of `agent/*.toml` survives the change; that is the acceptance test
   for the implementation, enforceable with a grep test in CI.
4. **Schema.** Add `schemas/repo-config.schema.json` and validate the new file in
   `jankurai doctor` and at audit load, as `agent/boundaries.toml` is validated
   today. A malformed `agent/jankurai.toml` is an error, never a silent fallback to
   the old files — silent fallback would let a typo quietly lower a gate.
5. **`jankurai-install.toml` stays separate.** It is generated output, not
   configuration; folding generated state into the hand-edited file invites the hand
   edits its header forbids.

## Fingerprints and ratchet baselines

This is the part that can break gates, so it is specified rather than left to the
implementation.

`policy_fingerprint` is today a SHA-256 over a JSON document tagged
`jankurai-effective-audit-policy-v1` whose `source_sha256` is the digest of the raw
bytes of `agent/audit-policy.toml`. If that became the digest of the whole
`agent/jankurai.toml`, then adding one owner prefix would change the policy
fingerprint, and `compare_report_to_baseline()` would report `policy_changed` — a
ratchet failure caused by an unrelated edit. Unacceptable.

Decision: when policy comes from `agent/jankurai.toml`, `source_sha256` is the digest
of the **canonical serialization of the resolved `[policy]` section** (keys sorted,
defaults filled in), not of the file. Tag the document
`jankurai-effective-audit-policy-v2` so the two derivations can never collide.

The same rule applies to `manifest_fingerprints`: the six keys (`owner_map`,
`test_map`, `generated_zones`, `boundaries`, `proof_lanes`, `standard_version`) keep
their names and their place in the report and in the evidence index, but each is
computed from the canonical serialization of the resolved concern. A repository with
no `agent/jankurai.toml` keeps file-byte digests, so its existing reports, baselines
and evidence indexes are bit-identical.

For a repository that does adopt the new file, the fingerprints change once. Handle
that like any other baseline move, not with a compatibility fudge:

- The migration command refreshes `agent/baselines/*.repo-score.json` in the same
  commit that writes `agent/jankurai.toml`, by re-running the audit and writing the
  new report, so the ratchet compares like with like.
- `matches_legacy_policy()` is **not** extended to accept the pre-migration digest
  under the new derivation. It exists for reports written by older auditors, and
  teaching it to accept a fingerprint over bytes the change under test controls would
  weaken the ratchet exactly where it matters.
- `jankurai audit` emits one advisory finding when a baseline's
  `policy_fingerprint` is a v1 derivation while the run resolved policy from
  `agent/jankurai.toml` and the baseline was not refreshed — a readable diagnostic
  instead of a bare `policy_changed`.

`input_fingerprint` hashes audited source paths and prose text only
(`audit/mod.rs`), so it is unaffected. `report_fingerprint` is a digest of the report
and moves with the manifest keys, as it already does whenever a manifest is edited.

## Migration path

`jankurai migrate` is taken: it is the legacy-codebase migration planner
(`MigrateCommand::VerifyPrompt`, `MigrateCommand::SliceRisk` in `main.rs`). Reusing
that verb for configuration would collide with a shipped CLI surface. Add instead:

```
jankurai config migrate [--repo DIR] [--write] [--remove-old] [--refresh-baseline]
```

- Default is a dry run printing the `agent/jankurai.toml` it would write and, per
  concern, whether the value came from a file or from a default.
- `--write` writes the file. Concerns whose current content equals the built-in
  default are **omitted** from the output rather than copied into it; copying
  boilerplate forward would defeat the purpose.
- `--remove-old` deletes only the separate files whose content the new file now
  reproduces exactly, and refuses if any of them differs from what was read.
- `--refresh-baseline` re-runs the audit and rewrites the ratchet baseline, as above.
  It is a separate flag so that writing the file never silently moves a gate's
  expectations.
- The command is idempotent and crash-safe: write to a temporary file in `agent/`,
  `fsync`, rename into place, and only then remove old files. A crash leaves either
  the old set or the new file, never a half-written one and never neither.

`jankurai init` keeps writing the old layout until the follow-up implementation lands,
then writes `agent/jankurai.toml` plus `AGENTS.md` and the generated install manifest.

## Which files can simply disappear

With built-in defaults (companion proposal) plus this file, an adopting repository is
asked for `agent/jankurai.toml` and `AGENTS.md`:

- **Gone, covered by a default:** `agent/tool-adoption.toml` (every tool at its
  default mode), `agent/standard-version.toml` (the auditor already knows its own
  standard, schema, paper edition and auditor version — the file only pins the split
  member and source commit, which belong in the generated install manifest), and
  `agent/audit-policy.toml` for a repository that keeps the standard's floor,
  severity lists, caps, limits and canonical toggles, which is the common case. A
  repository that tunes any of them writes only the keys it tunes under `[policy]`;
  the rest resolve from the default.
- **Gone, covered by detection:** `agent/generated-zones.toml` for the common cases —
  `target/` when a `Cargo.toml` is present, `dist/` for a Vite app — with the section
  kept for anything unusual. Detection must only ever *add* a zone a default knows
  about; it must not drop a zone the repository declared.
- **Gone, replaced by a pointer:** `agent/JANKURAI_STANDARD.md`, whose body is a link
  to the published standard plus the version the repository tracks. `AGENTS.md` can
  carry that one line.
- **Stays, as a section:** owners, proof routes (the lane catalog's commands,
  `required_artifacts` and `rules_covered` are repository-specific; only the standard
  lane *names* come from a default) and boundaries — the concerns that hold real
  knowledge about the repository.
- **Stays, as a separate generated file:** `agent/jankurai-install.toml`,
  `.jankurai/repo-score.{json,md}`, `agent/baselines/`.

That is ten files down to one hand-written file with three sections that matter.

## Consequences

- Adoption cost drops from ten files to one, and the boilerplate stops being a place
  where sixteen repositories drift apart.
- The auditor gains one loader and one schema; it must keep reading the old layout for
  as long as any repository uses it, which this ADR treats as indefinite. There is no
  removal date and no deprecation warning that fails a build.
- Repositories that adopt the new file take a one-time baseline refresh, visible in
  the same commit.
- The forge runs a governed build (`jeryu/jankurai`), so none of this changes a live
  score until a person re-cuts the auditor and re-installs it.

## Alternatives rejected

- **Per-key merge of both layouts.** Cheapest to implement, worst to reason about: the
  effective policy would be spread over two files and the fingerprint over both.
- **A hard cutover with `jankurai migrate` required.** Breaks every repository that
  has not run it, including ones outside the family, for no benefit the fallback does
  not already give.
- **JSON or YAML for the unified file.** The majority of the files being replaced are
  already TOML, `[sections]` match the concerns, and TOML tolerates comments, which
  owner maps need.
- **Keeping `agent/owner-map.json` and `agent/test-map.json` separate and collapsing
  only the boilerplate.** Tempting, and it would be a smaller change, but it leaves
  three files instead of one and still asks a newcomer which of them to read first.
