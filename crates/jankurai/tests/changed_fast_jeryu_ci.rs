//! A source/docs-only diff must retain the unchanged forge CI evidence.
use jankurai::audit::{run_audit, run_audit_with_options, AuditOptions};
use jankurai::model::Report;
use std::{fs, path::Path, process::Command};
use tempfile::tempdir;

fn write(repo: &Path, path: &str, text: &str) {
    let path = repo.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn fixture(repo: &Path) {
    write(repo, "AGENTS.md", "Read agent standard\n");
    write(
        repo,
        "README.md",
        "# Fixture\nRust workspace layout and validation.\n",
    );
    write(
        repo,
        "Cargo.toml",
        "[package]\nname = \"fixture\"\nversion = \"1.0.0\"\n[dev-dependencies]\nproptest = \"1\"\n",
    );
    write(
        repo,
        "Justfile",
        "fast:\n    bash scripts/fast.sh\naudit:\n    bash ops/ci/audit.sh\n",
    );
    write(repo, ".jeryu/ci.toml", "schema_version = \"2\"\nprovider = \"jeryu\"\n[[lane]]\nname = \"required\"\ncommand = \"bash scripts/required.sh\"\n");
    write(
        repo,
        "scripts/required.sh",
        "#!/bin/sh\nset -eu\nbash scripts/fast.sh\nbash ops/ci/audit.sh\n",
    );
    write(repo, "scripts/fast.sh", "#!/bin/sh\nset -eu\ncargo check --locked --offline\ncargo nextest run --locked --offline\n");
    write(
        repo,
        "ops/ci/audit.sh",
        "#!/bin/sh\nset -eu\njankurai audit . --json target/repo-score.json\n",
    );
    write(
        repo,
        "src/lib.rs",
        "pub fn value() -> u8 { 1 }\n#[test] fn value_is_one() { assert_eq!(value(), 1); }\nproptest::proptest! { #[test] fn constant_value(_input in 0u8..=255) { proptest::prop_assert_eq!(value(), 1); } }\n",
    );
    write(repo, "docs/notes.md", "# Notes\n");
    write(
        repo,
        "src/unrelated.rs",
        "fn unrelated() { todo!(\"outside the diff\"); }\n",
    );
}

fn changed(repo: &Path, path: &str) -> Report {
    run_audit_with_options(
        repo,
        &[repo.join(path)],
        AuditOptions {
            changed_fast: true,
            ..AuditOptions::default()
        },
    )
    .unwrap()
}

fn assert_ci_evidence(report: &Report) {
    for cap in ["no-deterministic-fast-lane", "no-jankurai-audit-lane-in-ci"] {
        assert!(
            !report.caps_applied.iter().any(|value| value == cap),
            "{cap}: {:?}",
            report.caps_applied
        );
    }
    assert!(
        !report
            .findings
            .iter()
            .any(|f| f.problem.contains("declared CI lane") && f.problem.contains("resolve")),
        "{:?}",
        report.findings
    );
}

#[test]
fn changed_fast_retains_jeryu_declaration_and_nested_scripts() {
    let dir = tempdir().unwrap();
    fixture(dir.path());
    let full = run_audit(dir.path(), &[]).unwrap();
    assert_ci_evidence(&full);
    for path in ["src/lib.rs", "docs/notes.md"] {
        let report = changed(dir.path(), path);
        assert_ci_evidence(&report);
        let proof = |r: &Report| {
            r.dimensions
                .iter()
                .find(|d| d.name == "Proof lanes and test routing")
                .unwrap()
                .score
        };
        assert_eq!(
            proof(&report),
            proof(&full),
            "full: {:?}; scoped: {:?}",
            full.dimensions
                .iter()
                .find(|d| d.name == "Proof lanes and test routing"),
            report
                .dimensions
                .iter()
                .find(|d| d.name == "Proof lanes and test routing")
        );
        assert_eq!(report.scope.paths, [path]);
        assert!(!report.findings.iter().any(|f| f.path == "src/unrelated.rs"));
    }
    // Merely declaring a lane must not manufacture executable evidence.
    write(dir.path(), "scripts/fast.sh", "#!/bin/sh\nset -eu\n");
    write(dir.path(), "ops/ci/audit.sh", "#!/bin/sh\nset -eu\n");
    let degraded = changed(dir.path(), "docs/notes.md");
    assert!(degraded
        .caps_applied
        .iter()
        .any(|cap| cap == "no-jankurai-audit-lane-in-ci"));
}

fn git(repo: &Path, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .env("GIT_AUTHOR_NAME", "fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@example.com")
        .env("GIT_COMMITTER_NAME", "fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@example.com")
        .env("JERYU_GIT_BYPASS", "1")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn diff_audit_retains_jeryu_ci_for_docs_only_commit() {
    let dir = tempdir().unwrap();
    let repo = dir.path();
    fixture(repo);
    git(repo, &["init", "-q", "-b", "main"]);
    git(repo, &["add", "."]);
    git(repo, &["commit", "-qm", "base"]);
    git(repo, &["checkout", "-qb", "docs"]);
    write(
        repo,
        "docs/notes.md",
        "# Notes\nDocumentation-only change.\n",
    );
    git(repo, &["commit", "-qam", "docs"]);
    let out = repo.join("target/diff");
    let json = out.join("score.json");
    let output = Command::new(env!("CARGO_BIN_EXE_jankurai"))
        .arg("diff-audit")
        .arg(repo)
        .args(["--base-ref", "main", "--skip-proof", "--advisory-only"])
        .arg("--out-dir")
        .arg(out)
        .arg("--json")
        .arg(&json)
        .env("JERYU_GIT_BYPASS", "1")
        .env_remove("JANKURAI_DIFF_BASE")
        .env_remove("GITHUB_BASE_REF")
        .env_remove("CI_MERGE_REQUEST_DIFF_BASE_SHA")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&fs::read(json).unwrap()).unwrap();
    for cap in ["no-deterministic-fast-lane", "no-jankurai-audit-lane-in-ci"] {
        assert!(
            !report["caps_applied"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v == cap),
            "{cap}: {}",
            report["caps_applied"]
        );
    }
    assert_eq!(
        report["scope"]["paths"],
        serde_json::json!(["docs/notes.md"])
    );
}
