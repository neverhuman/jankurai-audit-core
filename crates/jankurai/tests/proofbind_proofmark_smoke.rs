use std::fs;
use std::process::Command;

use jankurai::validation::{self, ArtifactSchema};
use tempfile::tempdir;

fn binary_path() -> &'static str {
    env!("CARGO_BIN_EXE_jankurai")
}

fn seed_catalog(repo: &std::path::Path) {
    fs::create_dir_all(repo.join("agent")).unwrap();
    fs::write(
        repo.join("agent/owner-map.json"),
        r#"{"workspace":"fixture","owners":{"src/":"tools","agent/":"agent","target/":"workspace"}}"#,
    )
    .unwrap();
    fs::write(
        repo.join("agent/test-map.json"),
        r#"{"workspace":"fixture","tests":{"src/":{"command":"cargo test -p fixture","purpose":"rust"},"agent/":{"command":"just score","purpose":"agent"}}}"#,
    )
    .unwrap();
    fs::write(
        repo.join("agent/proof-lanes.toml"),
        r#"[[lane]]
name = "proofmark-rust"
command = "cargo test -p fixture"
purpose = "proofmark"

[[lane]]
name = "audit"
command = "just score"
purpose = "audit"
"#,
    )
    .unwrap();
}

#[test]
fn proofbind_and_proofmark_help_surfaces_exist() {
    for args in [["proofbind", "--help"], ["proofmark", "--help"]] {
        let output = Command::new(binary_path()).args(args).output().unwrap();
        assert!(
            output.status.success(),
            "{args:?} failed\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Usage:"), "{stdout}");
    }
}

#[test]
fn imported_proofmark_and_edited_obligations_cannot_satisfy_current_proof() {
    let repo = tempdir().unwrap();
    seed_catalog(repo.path());
    fs::create_dir_all(repo.path().join("src")).unwrap();
    fs::write(
        repo.path().join("src/lib.rs"),
        "pub fn api() -> bool { true }\n",
    )
    .unwrap();

    let status = Command::new(binary_path())
        .current_dir(repo.path())
        .arg("proofbind")
        .arg("verify")
        .arg(repo.path())
        .arg("--changed")
        .arg("src/lib.rs")
        .status()
        .unwrap();
    assert!(status.success());

    let obligations_path = repo
        .path()
        .join("target/jankurai/proofbind/obligations.json");
    let obligations: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&obligations_path).unwrap()).unwrap();
    validation::validate_value(
        repo.path(),
        ArtifactSchema::ProofBindObligations,
        &obligations,
    )
    .unwrap();
    assert_eq!(obligations["summary"]["missing"], 1);

    fs::write(
        repo.path().join("coverage.lcov"),
        "TN:\nSF:src/lib.rs\nDA:1,1\nend_of_record\n",
    )
    .unwrap();
    let status = Command::new(binary_path())
        .current_dir(repo.path())
        .arg("proofmark")
        .arg("rust")
        .arg(repo.path())
        .arg("--changed")
        .arg("src/lib.rs")
        .arg("--coverage")
        .arg("coverage.lcov")
        .status()
        .unwrap();
    assert!(status.success());

    let proofmark_receipt_path = repo
        .path()
        .join("target/jankurai/proofmark/proofmark-receipt.json");
    let proofmark: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&proofmark_receipt_path).unwrap()).unwrap();
    validation::validate_value(repo.path(), ArtifactSchema::ProofMarkReceipt, &proofmark).unwrap();
    assert_eq!(proofmark["summary"]["satisfied_obligations"], 1);

    let proof_receipt_path = repo
        .path()
        .join("target/jankurai/proofmark/proof-receipt.json");
    let proof_receipt: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&proof_receipt_path).unwrap()).unwrap();
    validation::validate_value(repo.path(), ArtifactSchema::ProofReceipt, &proof_receipt).unwrap();
    assert!(proof_receipt["rules_covered"]
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(
        proof_receipt["extensions"]["trust_status"],
        "unverified-import"
    );
    assert!(proof_receipt["extensions"].get("proofmark").is_none());

    let required = Command::new(binary_path())
        .current_dir(repo.path())
        .args(["proofmark", "rust"])
        .arg(repo.path())
        .args([
            "--changed",
            "src/lib.rs",
            "--coverage",
            "coverage.lcov",
            "--mode",
            "required",
        ])
        .output()
        .unwrap();
    assert!(
        !required.status.success(),
        "authored LCOV manufactured required execution proof"
    );

    let witness_out = repo.path().join("target/jankurai/merge-witness.json");
    let witness_md = repo.path().join("target/jankurai/merge-witness.md");
    let output = Command::new(binary_path())
        .current_dir(repo.path())
        .arg("witness")
        .arg(repo.path())
        .arg("--changed")
        .arg("src/lib.rs")
        .arg("--proof-receipts")
        .arg(&proof_receipt_path)
        .arg("--out")
        .arg(&witness_out)
        .arg("--md")
        .arg(&witness_md)
        .output()
        .unwrap();
    assert!(
        witness_out.exists(),
        "witness JSON is written before any merge gate failure"
    );
    let witness: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&witness_out).unwrap()).unwrap();
    validation::validate_value(repo.path(), ArtifactSchema::MergeWitness, &witness).unwrap();
    assert!(!output.status.success());
    assert_eq!(witness["proofbind"]["missing_obligation_count"], 1);
    assert_eq!(
        witness["available_proof_receipts"][0]["trust_status"],
        "unverified-import"
    );

    // Neither an authored success bit nor deletion of the cached obligation
    // inventory changes classification of the actual source.
    for forged in [
        serde_json::json!({"summary":{"verdict":"pass"},"obligations":[{"satisfied":true}]}),
        serde_json::json!({"summary":{"verdict":"pass"},"obligations":[]}),
    ] {
        fs::write(&obligations_path, serde_json::to_vec(&forged).unwrap()).unwrap();
        let output = Command::new(binary_path())
            .current_dir(repo.path())
            .arg("witness")
            .arg(repo.path())
            .args(["--changed", "src/lib.rs"])
            .arg("--proof-receipts")
            .arg(&proof_receipt_path)
            .arg("--out")
            .arg(&witness_out)
            .arg("--md")
            .arg(&witness_md)
            .output()
            .unwrap();
        assert!(!output.status.success());
        let witness: serde_json::Value =
            serde_json::from_slice(&fs::read(&witness_out).unwrap()).unwrap();
        assert_eq!(witness["proofbind"]["missing_obligation_count"], 1);
        assert_eq!(witness["proofbind"]["satisfied_obligation_count"], 0);
    }
    let output = Command::new(binary_path())
        .current_dir(repo.path())
        .args(["proofbind", "verify"])
        .arg(repo.path())
        .args(["--changed", "src/lib.rs", "--mode", "required"])
        .arg("--proof-receipts")
        .arg(&proof_receipt_path)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let current: serde_json::Value =
        serde_json::from_slice(&fs::read(obligations_path).unwrap()).unwrap();
    assert_eq!(current["summary"]["missing"], 1);
}

#[test]
fn proofbind_required_mode_fails_when_any_obligation_is_missing() {
    let repo = tempdir().unwrap();
    seed_catalog(repo.path());
    fs::create_dir_all(repo.path().join("src")).unwrap();
    fs::write(
        repo.path().join("src/lib.rs"),
        "pub fn api() -> bool { true }\n",
    )
    .unwrap();

    let output = Command::new(binary_path())
        .current_dir(repo.path())
        .arg("proofbind")
        .arg("verify")
        .arg(repo.path())
        .arg("--changed")
        .arg("src/lib.rs")
        .arg("--mode")
        .arg("required")
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "required mode should fail when any obligation is unresolved"
    );
}

fn git(repo: &std::path::Path, args: &[&str]) {
    let output = Command::new("git")
        .args([
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .current_dir(repo)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// A committed fixture with an authorization module and an ordinary module,
/// with `base` naming the commit before the change under test.
fn seed_committed(repo: &std::path::Path) {
    seed_catalog(repo);
    fs::create_dir_all(repo.join("src")).unwrap();
    fs::write(
        repo.join("src/auth.rs"),
        "pub fn authorize(tenant_id: &str) -> bool { !tenant_id.is_empty() }\n",
    )
    .unwrap();
    fs::write(repo.join("src/lib.rs"), "pub fn api() -> bool { true }\n").unwrap();
    git(repo, &["init", "-q", "-b", "main"]);
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-q", "-m", "base"]);
    git(repo, &["branch", "base"]);
}

fn proofbind_verify(repo: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(binary_path())
        .current_dir(repo)
        .arg("proofbind")
        .arg("verify")
        .arg(repo)
        .args(args)
        .output()
        .unwrap()
}

fn read_artifacts(repo: &std::path::Path) -> (serde_json::Value, serde_json::Value) {
    let read = |name: &str| -> serde_json::Value {
        serde_json::from_str(
            &fs::read_to_string(repo.join("target/jankurai/proofbind").join(name)).unwrap(),
        )
        .unwrap()
    };
    let witness = read("surface-witness.json");
    let obligations = read("obligations.json");
    validation::validate_value(repo, ArtifactSchema::ProofBindWitness, &witness).unwrap();
    validation::validate_value(repo, ArtifactSchema::ProofBindObligations, &obligations).unwrap();
    (witness, obligations)
}

fn surface_keys(witness: &serde_json::Value, path: &str) -> Vec<String> {
    witness["surfaces"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|surface| surface["path"] == path)
        .map(|surface| {
            format!(
                "{}:{}",
                surface["surface_type"].as_str().unwrap(),
                surface["symbol"].as_str().unwrap()
            )
        })
        .collect()
}

// Shipped in 1.7.3 (proof): a deletion-only change set is analyzable, and the
// deleted path still binds obligations. It is classified from its pre-image
// (so deleting an authz module binds HLT-022) and binds a removal obligation,
// so Required mode cannot pass on zero obligations. Before 1.7.3 the same
// change failed with "required input missing".
#[test]
fn proofbind_deletion_only_change_binds_obligations_and_fails_required_mode() {
    let repo = tempdir().unwrap();
    seed_committed(repo.path());
    git(repo.path(), &["rm", "-q", "src/auth.rs"]);
    git(repo.path(), &["commit", "-q", "-m", "remove auth"]);

    let output = proofbind_verify(
        repo.path(),
        &["--changed-from", "base", "--mode", "required"],
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "a removal is never a free pass");
    assert!(stderr.contains("missing obligation(s)"), "{stderr}");
    assert!(!stderr.contains("required input missing"), "{stderr}");

    let (witness, obligations) = read_artifacts(repo.path());
    assert_eq!(witness["changed_paths"], serde_json::json!(["src/auth.rs"]));
    let auth = surface_keys(&witness, "src/auth.rs");
    assert!(
        auth.contains(&"authz_boundary:authz".to_string()),
        "{auth:?}"
    );
    assert!(
        auth.contains(&"business_invariant:removal".to_string()),
        "{auth:?}"
    );
    assert!(obligations["obligations"]
        .as_array()
        .unwrap()
        .iter()
        .any(|obligation| obligation["path"] == "src/auth.rs"
            && obligation["risk_tags"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!("removal"))));
    assert!(obligations["summary"]["total"].as_u64().unwrap() >= 2);
    assert_eq!(
        obligations["summary"]["missing"],
        obligations["summary"]["total"]
    );
    assert_eq!(obligations["summary"]["verdict"], "block");
}

// A deletion next to an ordinary edit binds both.
#[test]
fn proofbind_deletion_alongside_an_edit_binds_both() {
    let repo = tempdir().unwrap();
    seed_committed(repo.path());
    git(repo.path(), &["rm", "-q", "src/auth.rs"]);
    fs::write(
        repo.path().join("src/lib.rs"),
        "pub fn api() -> bool { false }\n",
    )
    .unwrap();
    git(
        repo.path(),
        &["commit", "-q", "-am", "remove auth, edit api"],
    );

    let output = proofbind_verify(repo.path(), &["--changed-from", "base"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let (witness, _) = read_artifacts(repo.path());
    assert!(surface_keys(&witness, "src/lib.rs").contains(&"rust_public_api:api".to_string()));
    let auth = surface_keys(&witness, "src/auth.rs");
    assert!(
        auth.contains(&"authz_boundary:authz".to_string()),
        "{auth:?}"
    );
    assert!(
        auth.contains(&"business_invariant:removal".to_string()),
        "{auth:?}"
    );
}

// The typo guard: a path named with --changed carries no deletion status, so
// a missing one is still a required input, deleted or misspelled.
#[test]
fn proofbind_explicit_missing_path_is_still_a_required_input() {
    let repo = tempdir().unwrap();
    seed_committed(repo.path());
    git(repo.path(), &["rm", "-q", "src/auth.rs"]);
    for path in ["src/auth.rs", "src/atuh.rs"] {
        let output = proofbind_verify(repo.path(), &["--changed", path]);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!output.status.success(), "{path}");
        assert!(
            stderr.contains(&format!("required input missing: {path}")),
            "{stderr}"
        );
    }
}
