# jankurai-core

Status: initial split-family extraction
Owner: Jankurai maintainers
Last reviewed: 2026-06-12
Applies to: jankurai-core

## Role

Rust package and binary source for the jankurai auditor and merge control plane.

## Repositories

- Local authoritative repo: `root/jankurai-core`
- Public mirror: `neverhuman/jankurai-audit-core`
- Release tag pattern: `jankurai-core-v1.7.0-split.1`
- Source extraction commit: `cea83b0cbe204be276a2f0299cd760f6812ea2b0`

## Split Rules

- Jeryu remains authoritative; GitHub is the public mirror.
- Release builds compose immutable member tags from the forge or its public mirror, never branches.
- Standalone Cargo uses the kernel commit reviewed and merged on the forge.
  Its canonical source URL is `https://git.neverhuman.org/git/root/jankurai-tools-kernel.git`;
  authenticated forge access may be needed. The hub release fusion replaces this
  route with the corresponding locked local member.
- Local development uses the hub `scripts/fuse.sh` output under `.fusion/`.
- Committed manifests must not depend on sibling checkout paths.
- Generated outputs are regenerated from their source contracts or build commands.

## Required Local Check

```bash
bash scripts/ci-local.sh required
```
