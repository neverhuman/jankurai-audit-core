pub struct Template {
    pub path: &'static str,
    pub body: &'static str,
}

pub fn template_for_path(path: &str) -> Option<&'static Template> {
    TEMPLATES.iter().find(|t| t.path == path)
}

pub fn body_for_path(path: &str, level: &str, cargo_repo: bool) -> Option<&'static str> {
    if path == "Justfile" {
        if cargo_repo && level == "full" {
            return Some(RUST_FULL_JUSTFILE);
        }
        if matches!(level, "agents" | "score") {
            return Some(MINIMAL_JUSTFILE);
        }
    }
    template_for_path(path).map(|template| template.body)
}

const ADAPTER_POINTER: &str = "<!-- jankurai generated adapter -->\n<!-- jankurai agent request v1 sha256:REPLACE_WITH_HASH -->\nRead `AGENTS.md` first. Use `agent/JANKURAI_STANDARD.md` as the canonical jankurai standard.\nWhen a user provides a paper, release, implementation, or handoff plan in the conversation, treat that plan as the controlling plan. Do not route such plans through the separate local phase workflow unless the user explicitly names MASTER_PLAN phase work.\nFor explicit MASTER_PLAN/phase work only, read `agent/MASTER_PLAN.md`, then `tips/phases/00-phase-index.md`, then the active `tips/phases/*.md` phase file. Log explicit phase work in `tips/phases/logs/`.\nFor explicit MASTER_PLAN/phase planning only, follow `agent/MASTER_PLAN.md#detailed-planner-protocol`.\nIf jankurai is installed, run `jankurai update --client-start --quiet` before work; do not apply updates unless the user asks.\n";
const PROOF_ADAPTER_POINTER: &str = "---\nname: jankurai\ndescription: Jankurai workspace guidance for Codex. Read repo instructions and standard first; phase files are only for explicit phase work.\n---\n\n# jankurai\n\n<!-- jankurai generated adapter -->\n<!-- jankurai agent request v1 sha256:REPLACE_WITH_HASH -->\nRead `AGENTS.md` first. Use `agent/JANKURAI_STANDARD.md` as the canonical jankurai standard.\nWhen a user provides a paper, release, implementation, or handoff plan in the conversation, treat that plan as the controlling plan. Do not route such plans through the separate local phase workflow unless the user explicitly names MASTER_PLAN phase work.\nFor explicit MASTER_PLAN/phase work only, read `agent/MASTER_PLAN.md`, then `tips/phases/00-phase-index.md`, then the active `tips/phases/*.md` phase file. Log explicit phase work in `tips/phases/logs/`.\nFor explicit MASTER_PLAN/phase planning only, follow `agent/MASTER_PLAN.md#detailed-planner-protocol`.\nRun the proof lane in `agent/test-map.json` for changed paths.\nIf jankurai is installed, run `jankurai update --client-start --quiet` before work; do not apply updates unless the user asks.\n";
macro_rules! workspace_boundary {
    () => {
        "## Workspace Boundary\n\n- Work only in the user-named active repo/worktree.\n- Never switch to sibling clones, archives, backups, resolved symlink targets, `/tmp` worktrees, or duplicate roots.\n- Never create repo copies or side folders outside the active repo; preserve work with git branches.\n- Before edits, report `pwd`, `git rev-parse --show-toplevel`, and `git status --short --branch`.\n- Use Jeryu APIs/CLI for forge remote and merge-request work; no credential scraping or raw forge API calls.\n\n"
    };
}
macro_rules! workflow_adapter {
    ($title:literal, $usage:literal, $receipts:literal, $next:literal, $stop:literal) => {
        concat!(
            "# ",
            $title,
            "\n\n",
            "<!-- jankurai generated adapter -->\n",
            "<!-- jankurai agent request v1 sha256:REPLACE_WITH_HASH -->\n",
            "Read `AGENTS.md` first. Use `agent/JANKURAI_STANDARD.md` as the canonical jankurai standard.\n",
            "When a user provides a paper, release, implementation, or handoff plan in the conversation, treat that plan as the controlling plan. Do not route such plans through the separate local phase workflow unless the user explicitly names MASTER_PLAN phase work.\n",
            "For explicit MASTER_PLAN/phase work only, read `agent/MASTER_PLAN.md`, then `tips/phases/00-phase-index.md`, then the active `tips/phases/*.md` phase file. Log explicit phase work in `tips/phases/logs/`.\n",
            "For explicit MASTER_PLAN/phase planning only, follow `agent/MASTER_PLAN.md#detailed-planner-protocol`.\n",
            $usage,
            "\nExpected receipts: ",
            $receipts,
            ".\n",
            "Next command: ",
            $next,
            ".\n",
            "Stop: ",
            $stop,
            ".\n",
            "If jankurai is installed, run `jankurai update --client-start --quiet` before work; do not apply updates unless the user asks.\n"
        )
    };
}
macro_rules! cell_agents_template {
    ($title:literal, $owner:literal, $forbidden:literal, $proof_lane:literal) => {
        concat!(
            "# ",
            $title,
            "\n\n",
            "<!-- jankurai generated adapter -->\n",
            "<!-- jankurai agent request v1 sha256:REPLACE_WITH_HASH -->\n",
            "Read `AGENTS.md` first. Use `agent/JANKURAI_STANDARD.md` as the canonical jankurai standard.\n",
            workspace_boundary!(),
            "When a user provides a paper, release, implementation, or handoff plan in the conversation, treat that plan as the controlling plan. Do not route such plans through the separate local phase workflow unless the user explicitly names MASTER_PLAN phase work.\n",
            "Owns `",
            $owner,
            "`.\n",
            "Forbidden: ",
            $forbidden,
            ".\n",
            "Proof lane: `",
            $proof_lane,
            "`.\n",
            "If jankurai is installed, run `jankurai update --client-start --quiet` before work; do not apply updates unless the user asks.\n"
        )
    };
}
const WEB_AGENTS: &str = cell_agents_template!(
    "apps/web/AGENTS.md",
    "apps/web/",
    "product truth, backend authority, and direct DB writes",
    "rendered UX / Playwright"
);
const API_AGENTS: &str = cell_agents_template!(
    "apps/api/AGENTS.md",
    "apps/api/",
    "UI-only concerns, direct DB writes, and contract generation",
    "edge handler / contract tests"
);
const DOMAIN_AGENTS: &str = cell_agents_template!(
    "crates/domain/AGENTS.md",
    "crates/domain/",
    "I/O glue, transport routing, and persistence code",
    "unit / property tests"
);
const APPLICATION_AGENTS: &str = cell_agents_template!(
    "crates/application/AGENTS.md",
    "crates/application/",
    "transport handlers, persistence code, and UI concerns",
    "use-case / authz tests"
);
const ADAPTERS_AGENTS: &str = cell_agents_template!(
    "crates/adapters/AGENTS.md",
    "crates/adapters/",
    "domain policy, web UI, and direct persistence truth",
    "adapter integration tests"
);
const WORKERS_AGENTS: &str = cell_agents_template!(
    "crates/workers/AGENTS.md",
    "crates/workers/",
    "request handling, UI behavior, and direct user flow ownership",
    "workflow / replay tests"
);
const CONTRACTS_AGENTS: &str = cell_agents_template!(
    "contracts/AGENTS.md",
    "contracts/",
    "generated clients, handwritten transport glue, and product truth",
    "generation / drift checks"
);
const DB_AGENTS: &str = cell_agents_template!(
    "db/AGENTS.md",
    "db/",
    "application logic, transport routing, and UI concerns",
    "migration / constraint tests"
);
const OPS_AGENTS: &str = cell_agents_template!(
    "ops/AGENTS.md",
    "ops/",
    "product feature code, domain policy, and direct DB writes",
    "security lane / workflow lint"
);
const PYTHON_AI_AGENTS: &str = cell_agents_template!(
    "python/ai-service/AGENTS.md",
    "python/ai-service/",
    "product truth, authorization, repo tooling, and direct DB writes",
    "eval / contract tests"
);
const KICKOFF_WORKFLOW: &str = workflow_adapter!(
    "jankurai kickoff",
    "Use `jankurai kickoff . --intent \"<change request>\" --out target/jankurai/kickoff.json --md target/jankurai/kickoff.md` to turn user intent into a no-write handoff. If changed paths are missing, keep the result planning-safe and ask bounded questions before any mutable command runs.",
    "`target/jankurai/kickoff.json`, `target/jankurai/kickoff.md`",
    "`jankurai context-pack`",
    "the task crosses owners, touches generated zones without source regeneration, or needs a broader proof lane than the receipt can justify"
);
const CONTEXT_PACK_WORKFLOW: &str = workflow_adapter!(
    "jankurai context-pack",
    "Use `jankurai context-pack . --changed <path> --max-tokens 6000 --out target/jankurai/context-pack.json --md target/jankurai/context-pack.md` to turn a bounded change set into a repo-aware context bundle.",
    "`target/jankurai/context-pack.json`, `target/jankurai/context-pack.md`",
    "`jankurai audit`",
    "the task is too broad, owner/test routing is unclear, or generated-zone work needs source regeneration first"
);
const PROVE_WORKFLOW: &str = workflow_adapter!(
    "jankurai proof",
    "Use `jankurai proof . --changed <path> --out target/jankurai/proof-plan.json --md target/jankurai/proof-plan.md` to build a **plan** of required lanes. Imported receipts and this plan are not execution proof; supervised command execution is unavailable.",
    "`target/jankurai/proof-plan.json`, `target/jankurai/proof-plan.md`",
    "`jankurai audit`",
    "the plan would imply unsigned commands, missing test-map lanes, or generated-zone mutation without a source contract"
);
const WITNESS_WORKFLOW: &str = workflow_adapter!(
    "jankurai witness",
    "Use `jankurai witness . --changed-from origin/main --baseline agent/baselines/main.repo-score.json --out target/jankurai/merge-witness.json --md target/jankurai/merge-witness.md` to compare the current branch against the accepted baseline.",
    "`target/jankurai/merge-witness.json`, `target/jankurai/merge-witness.md`",
    "`jankurai repair-plan`",
    "changed-path routing, generated-zone touches, baseline score delta, or proof coverage cannot be justified"
);
const REPAIR_PLAN_WORKFLOW: &str = workflow_adapter!(
    "jankurai repair-plan",
    "Use `jankurai repair-plan . --from .jankurai/repo-score.json --out target/jankurai/repair-plan.json --md target/jankurai/repair-plan.md` to turn the latest report into bounded repair packets.",
    "`target/jankurai/repair-plan.json`, `target/jankurai/repair-plan.md`",
    "`jankurai repair`",
    "the repair broadens scope, touches generated zones without a source contract, or requires a migration, secret rotation, or external service change"
);
const MINIMAL_JUSTFILE: &str = "# jankurai scaffold Justfile\n\nfast:\n\tjankurai doctor --fail-on critical\n\nscore:\n\tjankurai audit . --mode advisory --json .jankurai/repo-score.json --md .jankurai/repo-score.md --score-history .jankurai/score-history.jsonl --score-history-csv .jankurai/score-history.csv\n\ndoctor:\n\tjankurai doctor --fail-on high\n\ncheck: fast score\n";
const RUST_FULL_JUSTFILE: &str = "# jankurai scaffold Justfile\n\nfast:\n\tjankurai doctor --fail-on critical\n\nscore:\n\tjankurai audit . --mode advisory --json .jankurai/repo-score.json --md .jankurai/repo-score.md --score-history .jankurai/score-history.jsonl --score-history-csv .jankurai/score-history.csv\n\ndoctor:\n\tjankurai doctor --fail-on high\n\nsecurity:\n\tjankurai security run . --out target/jankurai/security/evidence.json\n\nrust-map:\n\tjankurai rust map .\n\nrust-witness:\n\tjankurai rust witness build .\n\nrust-diagnose:\n\tjankurai rust diagnose .\n\ncheck: fast score security rust-map rust-witness rust-diagnose\n";
pub const PRE_COMMIT_HOOK: &str = r#"#!/usr/bin/env bash
# JANKURAI MANAGED HOOK: pre-commit
# Developer check: scan staged + worktree paths vs HEAD only.
# This is not release, ratchet, or README-badge authority. CI runs a full audit.
# Caps / issue markers / hard findings in the touched set fail the commit.
set -euo pipefail

if [ "${JANKURAI_SKIP_HOOKS:-}" = "1" ]; then
  exit 0
fi

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
git_dir="$(git rev-parse --git-dir 2>/dev/null || printf '%s/.git' "$repo_root")"
case "$git_dir" in
  /*) ;;
  *) git_dir="$repo_root/$git_dir" ;;
esac
jankurai_dir="$git_dir/jankurai"
mkdir -p "$jankurai_dir"

env_file="$jankurai_dir/env"
if [ -f "$env_file" ]; then
  # shellcheck disable=SC1090
  . "$env_file"
fi

if [ -n "${JANKURAI_PRE_COMMIT_CHAIN:-}" ] && [ -x "$JANKURAI_PRE_COMMIT_CHAIN" ] && [ -z "${JANKURAI_CHAINED_HOOK:-}" ]; then
  JANKURAI_CHAINED_HOOK=1 "$JANKURAI_PRE_COMMIT_CHAIN" "$@"
fi

if [ -n "${JANKURAI_BIN:-}" ] && [ -x "$JANKURAI_BIN" ]; then
  jankurai_cmd="$JANKURAI_BIN"
else
  jankurai_cmd="${JANKURAI_FALLBACK_BIN:-jankurai}"
fi

cd "$repo_root"
report_dir="${JANKURAI_HOOK_REPORT_DIR:-target/jankurai/diff}"
mkdir -p "$report_dir"

# Compare to HEAD so a feature branch is not re-audited against origin/main.
# Incomplete Git inventories must fail closed (Fleet collect contract).
if ! "$jankurai_cmd" diff-audit --base-ref HEAD --out-dir "$report_dir"; then
  echo "jankurai pre-commit: diff-audit blocked this commit (hard findings, caps, or incomplete Git)." >&2
  echo "jankurai pre-commit: this is not a passing score badge. Set JANKURAI_SKIP_HOOKS=1 to bypass once." >&2
  exit 1
fi
"#;

/// Consumer CI: verified Action + public v1.7.0 auditor. Full-tree score lives here;
/// the local pre-commit hook only scans the diff.
pub const CONSUMER_CI_WORKFLOW: &str = r#"name: jankurai

on:
  pull_request:
  push:
    branches: [main]

permissions:
  contents: read

jobs:
  audit:
    runs-on: ubuntu-latest
    permissions:
      contents: read
    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683
        with:
          fetch-depth: 0
          persist-credentials: false
      - uses: neverhuman/jankurai-action@4a45526ac904315f96e6bbebda4e088268023afa
        id: quality
        with:
          release-tag: v1.7.0
          fail-under: "85"
      - uses: actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02
        if: always()
        with:
          name: jankurai-report
          if-no-files-found: ignore
          path: ${{ steps.quality.outputs.report-directory }}
"#;
pub const PREPARE_COMMIT_MSG_HOOK: &str = r#"#!/usr/bin/env bash
# JANKURAI MANAGED HOOK: prepare-commit-msg
set -euo pipefail

if [ "${JANKURAI_SKIP_HOOKS:-}" = "1" ]; then
  exit 0
fi

message_file="${1:?commit message file is required}"
repo_root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
git_dir="$(git rev-parse --git-dir 2>/dev/null || printf '%s/.git' "$repo_root")"
case "$git_dir" in
  /*) ;;
  *) git_dir="$repo_root/$git_dir" ;;
esac
jankurai_dir="$git_dir/jankurai"

env_file="$jankurai_dir/env"
if [ -f "$env_file" ]; then
  # shellcheck disable=SC1090
  . "$env_file"
fi

if [ -n "${JANKURAI_PREPARE_COMMIT_MSG_CHAIN:-}" ] && [ -x "$JANKURAI_PREPARE_COMMIT_MSG_CHAIN" ] && [ -z "${JANKURAI_CHAINED_HOOK:-}" ]; then
  JANKURAI_CHAINED_HOOK=1 "$JANKURAI_PREPARE_COMMIT_MSG_CHAIN" "$@"
fi

last_score="$jankurai_dir/last-score.env"
if [ ! -f "$last_score" ]; then
  exit 0
fi

# shellcheck disable=SC1090
. "$last_score"

if [ -z "${JANKURAI_SCORE:-}" ] || grep -q '^Jankurai-Score:' "$message_file"; then
  exit 0
fi

{
  printf '\n'
  printf 'Jankurai-Score: %s\n' "$JANKURAI_SCORE"
  printf 'Jankurai-Raw-Score: %s\n' "${JANKURAI_RAW_SCORE:-$JANKURAI_SCORE}"
  printf 'Jankurai-Findings: %s\n' "${JANKURAI_FINDINGS:-0}"
  printf 'Jankurai-Hard-Findings: %s\n' "${JANKURAI_HARD_FINDINGS:-0}"
  printf 'Jankurai-Decision: %s\n' "${JANKURAI_DECISION:-unknown}"
  printf 'Jankurai-Report: %s\n' "${JANKURAI_REPORT:-target/jankurai/hooks/pre-commit-score.json}"
} >> "$message_file"
"#;

pub const TEMPLATES: &[Template] = &[
    Template {
        path: "AGENTS.md",
        body: concat!(
            "# Agent Instructions\n\n",
            "Read `agent/JANKURAI_STANDARD.md` first. For explicit phase or MASTER_PLAN work only, read `agent/MASTER_PLAN.md` before `tips/phases/00-phase-index.md`. Keep generated artifacts under their declared source commands.\n\n",
            workspace_boundary!(),
        ),
    },
    Template {
        path: "apps/web/AGENTS.md",
        body: WEB_AGENTS,
    },
    Template {
        path: "apps/api/AGENTS.md",
        body: API_AGENTS,
    },
    Template {
        path: ".cursor/rules/jankurai.mdc",
        body: "---\nalwaysApply: true\n---\n\n<!-- jankurai generated adapter -->\n<!-- jankurai agent request v1 sha256:REPLACE_WITH_HASH -->\nRead `AGENTS.md` first. Use `agent/JANKURAI_STANDARD.md` as the canonical jankurai standard.\nWhen a user provides a paper, release, implementation, or handoff plan in the conversation, treat that plan as the controlling plan. Do not route such plans through the separate local phase workflow unless the user explicitly names MASTER_PLAN phase work.\nFor explicit MASTER_PLAN/phase work only, read `agent/MASTER_PLAN.md`, then `tips/phases/00-phase-index.md`, then the active `tips/phases/*.md` phase file. Log explicit phase work in `tips/phases/logs/`.\nFor explicit MASTER_PLAN/phase planning only, follow `agent/MASTER_PLAN.md#detailed-planner-protocol`.\nIf jankurai is installed, run `jankurai update --client-start --quiet` before work; do not apply updates unless the user asks.\n",
    },
    Template {
        path: "CLAUDE.md",
        body: ADAPTER_POINTER,
    },
    Template {
        path: "GEMINI.md",
        body: ADAPTER_POINTER,
    },
    Template {
        path: "Justfile",
        body: "# jankurai scaffold Justfile\n\nfast:\n\tjankurai doctor --fail-on critical\n\nscore:\n\tjankurai audit . --mode advisory --json .jankurai/repo-score.json --md .jankurai/repo-score.md --score-history .jankurai/score-history.jsonl --score-history-csv .jankurai/score-history.csv\n\ndoctor:\n\tjankurai doctor --fail-on high\n\nsecurity:\n\tjankurai security run . --out target/jankurai/security/evidence.json\n\ncheck: fast score security\n",
    },
    Template {
        path: ".gitignore",
        body: "# jankurai scaffold .gitignore\n\n# Keep Jankurai receipts local without hiding the rest of target/.\ntarget/jankurai/\n.jankurai/\n",
    },
    Template {
        path: ".github/copilot-instructions.md",
        body: ADAPTER_POINTER,
    },
    Template {
        path: ".github/instructions/jankurai.instructions.md",
        body: ADAPTER_POINTER,
    },
    Template {
        path: ".github/instructions/jankurai-rust.instructions.md",
        body: "---\napplyTo: \"**/*.rs\"\n---\n\n<!-- jankurai generated adapter -->\n<!-- jankurai agent request v1 sha256:REPLACE_WITH_HASH -->\nRead `AGENTS.md` first. Use `agent/JANKURAI_STANDARD.md` as the canonical jankurai standard.\nWhen a user provides a paper, release, implementation, or handoff plan in the conversation, treat that plan as the controlling plan. Do not route such plans through the separate local phase workflow unless the user explicitly names MASTER_PLAN phase work.\nFor explicit MASTER_PLAN/phase work only, read `agent/MASTER_PLAN.md`, then `tips/phases/00-phase-index.md`, then the active `tips/phases/*.md` phase file. Log explicit phase work in `tips/phases/logs/`.\nFor explicit MASTER_PLAN/phase planning only, follow `agent/MASTER_PLAN.md#detailed-planner-protocol`.\nIf jankurai is installed, run `jankurai update --client-start --quiet` before work; do not apply updates unless the user asks.\n",
    },
    Template {
        path: ".github/instructions/jankurai-web.instructions.md",
        body: "---\napplyTo: \"**/*.{ts,tsx,js,jsx,css}\"\n---\n\n<!-- jankurai generated adapter -->\n<!-- jankurai agent request v1 sha256:REPLACE_WITH_HASH -->\nRead `AGENTS.md` first. Use `agent/JANKURAI_STANDARD.md` as the canonical jankurai standard.\nWhen a user provides a paper, release, implementation, or handoff plan in the conversation, treat that plan as the controlling plan. Do not route such plans through the separate local phase workflow unless the user explicitly names MASTER_PLAN phase work.\nFor explicit MASTER_PLAN/phase work only, read `agent/MASTER_PLAN.md`, then `tips/phases/00-phase-index.md`, then the active `tips/phases/*.md` phase file. Log explicit phase work in `tips/phases/logs/`.\nFor explicit MASTER_PLAN/phase planning only, follow `agent/MASTER_PLAN.md#detailed-planner-protocol`.\nIf jankurai is installed, run `jankurai update --client-start --quiet` before work; do not apply updates unless the user asks.\n",
    },
    Template {
        path: ".github/instructions/jankurai-python-ai.instructions.md",
        body: "---\napplyTo: \"python/ai-service/**/*.py\"\n---\n\n<!-- jankurai generated adapter -->\n<!-- jankurai agent request v1 sha256:REPLACE_WITH_HASH -->\nRead `AGENTS.md` first. Use `agent/JANKURAI_STANDARD.md` as the canonical jankurai standard.\nDo not create or expand Python unless a dated advanced-ML/data exception explicitly approves this path. Python must not own product truth, authorization, repo tools, proof lanes, backend glue, or direct production DB writes.\nIf jankurai is installed, run `jankurai update --client-start --quiet` before work; do not apply updates unless the user asks.\n",
    },
    Template {
        path: "contracts/AGENTS.md",
        body: CONTRACTS_AGENTS,
    },
    Template {
        path: ".agents/agents.md",
        body: ADAPTER_POINTER,
    },
    Template {
        path: ".agents/skills/jankurai/SKILL.md",
        body: PROOF_ADAPTER_POINTER,
    },
    Template {
        path: ".agents/workflows/jankurai-audit.md",
        body: "# jankurai audit\n\n<!-- jankurai generated adapter -->\n<!-- jankurai agent request v1 sha256:REPLACE_WITH_HASH -->\nRead `AGENTS.md` first. Use `agent/JANKURAI_STANDARD.md` as the canonical jankurai standard.\nWhen a user provides a paper, release, implementation, or handoff plan in the conversation, treat that plan as the controlling plan. Do not route such plans through the separate local phase workflow unless the user explicitly names MASTER_PLAN phase work.\nFor explicit MASTER_PLAN/phase work only, read `agent/MASTER_PLAN.md`, then `tips/phases/00-phase-index.md`, then the active `tips/phases/*.md` phase file. Log explicit phase work in `tips/phases/logs/`.\nFor explicit MASTER_PLAN/phase planning only, follow `agent/MASTER_PLAN.md#detailed-planner-protocol`.\nRun `jankurai audit . --mode advisory --json .jankurai/repo-score.json --md .jankurai/repo-score.md` for audit.\nIf jankurai is installed, run `jankurai update --client-start --quiet` before work; do not apply updates unless the user asks.\n",
    },
    Template {
        path: ".agents/workflows/jankurai-kickoff.md",
        body: KICKOFF_WORKFLOW,
    },
    Template {
        path: ".agents/workflows/jankurai-context-pack.md",
        body: CONTEXT_PACK_WORKFLOW,
    },
    Template {
        path: ".agents/workflows/jankurai-prove.md",
        body: PROVE_WORKFLOW,
    },
    Template {
        path: ".agents/workflows/jankurai-witness.md",
        body: WITNESS_WORKFLOW,
    },
    Template {
        path: ".agents/workflows/jankurai-repair-plan.md",
        body: REPAIR_PLAN_WORKFLOW,
    },
    Template {
        path: ".claude/skills/jankurai/SKILL.md",
        body: PROOF_ADAPTER_POINTER,
    },
    Template {
        path: "agent/JANKURAI_STANDARD.md",
        body: "# jankurai Standard Agent Bootstrap\n\nStandard version: `0.9.0`\n\nRead `docs/agent-native-standard.md` when policy detail matters. Use `agent/owner-map.json`, `agent/test-map.json`, `agent/generated-zones.toml`, `agent/proof-lanes.toml`, `agent/tool-adoption.toml`, and `agent/boundaries.toml` before editing.\n",
    },
    Template {
        path: "crates/domain/AGENTS.md",
        body: DOMAIN_AGENTS,
    },
    Template {
        path: "crates/application/AGENTS.md",
        body: APPLICATION_AGENTS,
    },
    Template {
        path: "crates/adapters/AGENTS.md",
        body: ADAPTERS_AGENTS,
    },
    Template {
        path: "crates/workers/AGENTS.md",
        body: WORKERS_AGENTS,
    },
    Template {
        path: "agent/MASTER_PLAN.md",
        body: "# jankurai Master Plan\n\nRead `agent/JANKURAI_STANDARD.md`, then this file, before phase or audit work.\n\nFor phase work, read `tips/phases/00-phase-index.md`, then the active `tips/phases/*.md` phase file. Pick the earliest incomplete phase whose dependencies can be advanced unless the user names a phase.\n\nWhen asked for planning, produce a worker-ready plan with objective, read-first files, ownership, current state, implementation steps, hard parts, validation, logging, and safe parallel work packets.\n\nAppend start, progress, and finish entries to `tips/phases/logs/<phase>.log`. Keep proof receipts and generated evidence under `target/jankurai/`.\n\nUse `agent/test-map.json` to choose the smallest credible proof lane. For audit, run `jankurai audit . --mode advisory --json .jankurai/repo-score.json --md .jankurai/repo-score.md`.\n",
    },
    Template {
        path: "agent/boundaries.toml",
        body: include_str!("../../templates/agent/boundaries.toml"),
    },
    Template {
        path: "agent/generated-zones.toml",
        body: include_str!("../../templates/agent/generated-zones.toml"),
    },
    Template {
        path: "agent/owner-map.json",
        body: include_str!("../../templates/agent/owner-map.json"),
    },
    Template {
        path: "agent/proof-lanes.toml",
        body: include_str!("../../templates/agent/proof-lanes.toml"),
    },
    Template {
        path: "agent/audit-policy.toml",
        body: "minimum_score = 85\nfail_on = [\"critical\", \"high\"]\nadvisory_on = [\"medium\", \"low\"]\n\n[history]\nmax_rows = 500\nmax_bytes = 1048576\ndedupe = \"consecutive-equivalent\"\nmirror_env = \"JANKURAI_HISTORY_MIRROR\"\nmirror_required = false\nmirror_max_rows = 5000\n\n[scan]\nexcluded_paths = [\"tips/\"]\n\n[smart_scan]\n# After a clean full scan, only scan git-status changed files by default.\nfull_scan_interval_secs = 3600\nroulette_rate = 0.10\n\n# Full-repo `jankurai gate` stays advisory on new repos (never-freeze).\n# The installed pre-commit hook still runs diff-audit vs HEAD and fails on\n# hard findings or caps in touched files. That hook is not badge authority.\n[precommit_gate]\nblocking = false\n",
    },
    Template {
        path: "agent/badge.toml",
        body: "enabled = true\nsvg = \"agent/jankurai-badge.svg\"\njson = \"agent/jankurai-badge.json\"\nreadme = \"README.md\"\nlink = \"agent/jankurai-badge.json\"\nupdate_readme = true\nlabel = \"jankurai\"\n",
    },
    Template {
        path: ".pre-commit-config.yaml",
        body: "# Optional Python pre-commit framework wrapper.\n# `jankurai hooks install` writes native git hooks and does not need this file.\nrepos:\n  - repo: local\n    hooks:\n      - id: jankurai-diff-audit\n        name: jankurai diff-audit\n        entry: jankurai diff-audit --base-ref HEAD\n        language: system\n        pass_filenames: false\n        always_run: true\n",
    },
    Template {
        path: "agent/security-policy.toml",
        body: "schema_version = \"1.0.0\"\nenabled_tools = [\"gitleaks\", \"cargo audit\", \"npm audit\"]\nrequired_tools = []\nadvisory_tools = [\"gitleaks\", \"cargo audit\", \"npm audit\"]\n\n[severity_thresholds]\nfail_lane_on = \"high\"\n",
    },
    Template {
        path: "agent/tool-adoption.toml",
        body: "schema_version = \"1.0.0\"\n\n[[tools]]\nid = \"audit-ci\"\nmode = \"auto\"\n\n[[tools]]\nid = \"proof-routing\"\nmode = \"auto\"\n\n[[tools]]\nid = \"security\"\nmode = \"auto\"\n\n[[tools]]\nid = \"ux-qa\"\nmode = \"auto\"\n\n[[tools]]\nid = \"db-migration-analyze\"\nmode = \"auto\"\n\n[[tools]]\nid = \"contract-drift\"\nmode = \"auto\"\n\n[[tools]]\nid = \"rust-witness\"\nmode = \"auto\"\n",
    },
    Template {
        path: "agent/standard-version.toml",
        body: include_str!("../../templates/agent/standard-version.toml"),
    },
    Template {
        path: "agent/test-map.json",
        body: include_str!("../../templates/agent/test-map.json"),
    },
    Template {
        path: "docs/install.md",
        body: "# Install jankurai\n\n```sh\nbash -o pipefail -c 'curl --proto \"=https\" --tlsv1.2 -fsSL https://raw.githubusercontent.com/neverhuman/jankurai/v1.7.0/jankurai-installer.sh | bash -s -- --tag v1.7.0'\nexport PATH=\"$HOME/.local/bin:$PATH\"\njankurai init --yes\njankurai hooks install --yes\n```\n\nPre-commit then runs `jankurai diff-audit --base-ref HEAD` on staged and worktree files. That is a developer check, not a README score. Run `jankurai audit .` (full tree) in CI or before merge for the SVG badge. Upgrade with `jankurai upgrade` (same signed installer).\n",
    },
    Template {
        path: "docs/agent-native-standard.md",
        body: "# Agent-Native Standard\n\nKeep product truth in Rust, SQL, and generated contracts. Agents must not add Python except for rare dated advanced-ML/data exceptions under `python/ai-service`. Route every path to an owner and proof lane.\n",
    },
    Template {
        path: "docs/ide-integrations.md",
        body: "# IDE Integrations\n\nAll IDE adapters are thin pointers to `agent/JANKURAI_STANDARD.md`; keep durable policy there or in `docs/`.\n",
    },
    Template {
        path: "docs/exceptions/README.md",
        body: "# jankurai Exceptions\n\nDocument dated exceptions with owner, expiry, migration path, and proof lane.\n",
    },
    Template {
        path: "README-jankurai-scaffold.md",
        body: "# Greenfield scaffold (non-production)\n\n<!-- jankurai-badge:start -->\n<!-- Run `jankurai audit .` then `jankurai badge` (or keep agent/badge.toml enabled) to fill this score. -->\n<!-- jankurai-badge:end -->\n\nThis tree was bootstrapped with `jankurai init`. Replace this file with a real product README when you have one. Copy the badge markers into `README.md` if you rename this file, and set `readme` in `agent/badge.toml`.\n",
    },
    Template {
        path: "contracts/README.md",
        body: "# Contracts\n\nPut OpenAPI, JSON Schema, or protobuf **sources** here. Generated clients and bindings must live only under paths declared in `agent/generated-zones.toml`.\n",
    },
    Template {
        path: "db/AGENTS.md",
        body: DB_AGENTS,
    },
    Template {
        path: "db/README.md",
        body: "# Database\n\nMigrations live in `db/migrations/`. Optional constraint scripts in `db/constraints/`.\n",
    },
    Template {
        path: "db/migrations/README.md",
        body: "# Migrations\n\nAdd versioned SQL migrations. Regenerate any derived artifacts with the recorded command in `agent/generated-zones.toml`.\n",
    },
    Template {
        path: "db/constraints/README.md",
        body: "# Constraints\n\nDeclare durable database truth (checks, FKs) appropriate to your stack.\n",
    },
    Template {
        path: "docs/architecture/README.md",
        body: "# Architecture\n\nDocument boundaries, owners, proof lanes, and data flow. This stub is not production architecture.\n",
    },
    Template {
        path: "docs/decisions/README.md",
        body: "# Architecture Decision Records\n\nRecord significant decisions with date, status, context, and consequences.\n",
    },
    Template {
        path: "docs/auth/README.md",
        body: "# Authentication\n\nDocument authentication mechanisms, session lifecycles, and token issuance boundaries here.\n",
    },
    Template {
        path: "docs/orgs/README.md",
        body: "# Organizations\n\nDocument tenant isolation, RBAC, and cross-organization boundaries here.\n",
    },
    Template {
        path: "docs/admin/README.md",
        body: "# Admin Tools\n\nDocument elevated privileges, support masquerading, and internal operational routes here.\n",
    },
    Template {
        path: "docs/ai/README.md",
        body: "# AI Product Boundary\n\nPrefer Rust/TypeScript service boundaries. Add Python only for rare dated advanced-ML/data exceptions under `python/ai-service`.\n\nAI services may classify, retrieve, rank, generate, summarize, or recommend. They may not silently own durable product truth. Version prompts and attach eval receipts to releases.\n",
    },
    Template {
        path: "docs/product/README.md",
        body: "# Product Intent\n\nDocument product intent, user-visible guarantees, non-goals, risk tolerance, and proof expectations here.\n",
    },
    Template {
        path: "docs/backups/README.md",
        body: "# Backup And Restore\n\nDocument backup scope, restore proof, retention, RPO/RTO targets, and test evidence here.\n",
    },
    Template {
        path: "docs/compliance/README.md",
        body: "# Compliance Evidence\n\nThis is an evidence shell, not a compliance claim.\n\nMap controls to durable, machine-readable evidence before claiming readiness.\n",
    },
    Template {
        path: "docs/migration/README.md",
        body: "# Migration Plan\n\nDocument legacy inventory, boundary map, migration slices, equivalence proof, rollback, and containment policy here.\n",
    },
    Template {
        path: "docs/migration/boundary-map.md",
        body: "# Boundary Map\n\nMap legacy surfaces to target owners, contracts, databases, generated zones, and proof lanes.\n",
    },
    Template {
        path: "docs/migration/slices/README.md",
        body: "# Migration Slices\n\nEach slice needs intent, owner, changed paths, proof lane, equivalence evidence, rollback plan, and residual risk.\n",
    },
    Template {
        path: "docs/observability/README.md",
        body: "# Observability\n\nDocument logs, metrics, traces, audit events, SLOs, and evidence retention boundaries here.\n",
    },
    Template {
        path: "docs/privacy/README.md",
        body: "# Privacy And PII\n\nClassify data, document retention, access boundaries, deletion flows, and proof lanes here.\n",
    },
    Template {
        path: "docs/security/README.md",
        body: "# Security\n\nDocument threat model, secret policy, dependency scanning, provenance, SBOM, and security evidence here.\n",
    },
    Template {
        path: "ops/AGENTS.md",
        body: OPS_AGENTS,
    },
    Template {
        path: "evals/README.md",
        body: "# Evals\n\nStore eval harness docs and receipt conventions here. Generated eval outputs belong under `target/jankurai/` unless explicitly declared.\n",
    },
    Template {
        path: "evals/golden/README.md",
        body: "# Golden Eval Cases\n\nKeep small, reviewed eval cases here. Do not store secrets or production data.\n",
    },
    Template {
        path: "prompts/README.md",
        body: "# Prompts\n\nVersion prompts here. Each prompt change needs an eval note and owner.\n",
    },
    Template {
        path: "python/ai-service/README.md",
        body: "# Exception-Only AI/Data Service\n\nScaffold only. Do not add Python here unless a dated advanced-ML/data exception exists. Keep retrieval, ranking, and generation behind explicit contracts; do not treat model output as source of truth.\n",
    },
    Template {
        path: "python/ai-service/AGENTS.md",
        body: PYTHON_AI_AGENTS,
    },
    Template {
        path: "tools/security-lane.sh",
        body: "#!/usr/bin/env bash\nset -euo pipefail\n# Scaffold stub: replace with real secret/dependency/SBOM checks before treating this lane as proof.\necho \"security-lane scaffold requires project-specific checks\" >&2\nexit 2\n",
    },
    Template {
        path: "tools/jankurai-rust/witness.sh",
        body: "#!/usr/bin/env bash\nset -euo pipefail\nrepo_root=\"${1:-.}\"\nexec jankurai rust witness build \"$repo_root\"\n",
    },
    Template {
        path: "tools/jankurai-hooks/pre-commit",
        body: PRE_COMMIT_HOOK,
    },
    Template {
        path: "tools/jankurai-hooks/prepare-commit-msg",
        body: PREPARE_COMMIT_MSG_HOOK,
    },
    Template {
        path: "agent/ux-qa.toml",
        body: "outputRoot = \".\"\nartifactRoot = \"target/jankurai/ux-qa\"\nreadyState = \"domcontentloaded\"\ntimeoutMs = 15000\nscreenshotRequired = true\nariaSnapshotRequired = true\naccessibilityScanRequired = true\nrequiredStates = [\"loading\", \"empty\", \"error\", \"success\", \"permission-denied\"]\n",
    },
    Template {
        path: "agent/jankurai-install.toml",
        body: "# jankurai install manifest\n# Generated by jankurai init/update.\n# DO NOT EDIT BY HAND.\n",
    },
    Template {
        path: ".github/workflows/jankurai.yml",
        body: CONSUMER_CI_WORKFLOW,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_commit_hook_scans_the_diff_without_jq_or_full_tree() {
        assert!(
            PRE_COMMIT_HOOK.contains("diff-audit --base-ref HEAD"),
            "pre-commit must diff vs HEAD, not origin/main"
        );
        assert!(
            !PRE_COMMIT_HOOK.contains("--skip-proof"),
            "pre-commit must not skip Fleet's Git/proof inventory contract"
        );
        assert!(
            !PRE_COMMIT_HOOK.contains("audit ."),
            "pre-commit must not run a full `jankurai audit .`"
        );
        assert!(
            !PRE_COMMIT_HOOK.contains("jq "),
            "pre-commit must not require jq"
        );
        assert!(PRE_COMMIT_HOOK.contains("JANKURAI_SKIP_HOOKS"));
        assert_eq!(
            template_for_path("tools/jankurai-hooks/pre-commit").map(|t| t.body),
            Some(PRE_COMMIT_HOOK)
        );
    }

    #[test]
    fn consumer_ci_uses_verified_action_not_cargo_install() {
        assert!(CONSUMER_CI_WORKFLOW.contains("neverhuman/jankurai-action@"));
        assert!(CONSUMER_CI_WORKFLOW.contains("release-tag: v1.7.0"));
        assert!(CONSUMER_CI_WORKFLOW.contains("fail-under: \"85\""));
        assert!(
            !CONSUMER_CI_WORKFLOW.contains("cargo install"),
            "consumer CI must not cargo-install the split auditor"
        );
        assert!(
            !CONSUMER_CI_WORKFLOW.contains("|| true"),
            "consumer CI must not advisory-wash the badge check"
        );
        assert_eq!(
            template_for_path(".github/workflows/jankurai.yml").map(|t| t.body),
            Some(CONSUMER_CI_WORKFLOW)
        );
    }

    #[test]
    fn init_registers_badge_and_precommit_framework_templates() {
        assert!(template_for_path("agent/badge.toml").is_some());
        assert!(template_for_path(".pre-commit-config.yaml").is_some());
        let policy = template_for_path("agent/audit-policy.toml")
            .expect("audit-policy template")
            .body;
        assert!(policy.contains("[precommit_gate]"));
        assert!(policy.contains("blocking = false"));
        assert!(
            !PROVE_WORKFLOW.contains("jankurai prove"),
            "init must not advertise supervised prove/receipts as execution"
        );
    }
}
