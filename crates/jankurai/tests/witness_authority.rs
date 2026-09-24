use jankurai::commands::witness::{build_witness, WitnessArgs};
use serde_json::json;
use std::fs;

#[test]
fn authored_receipts_cannot_establish_required_lane_execution() {
    let repo = tempfile::tempdir().unwrap();
    let evidence = tempfile::tempdir().unwrap();
    fs::create_dir(repo.path().join("agent")).unwrap();
    fs::write(repo.path().join("README.md"), "# Source\n").unwrap();
    fs::write(
        repo.path().join("agent/owner-map.json"),
        json!({"owners":{"README.md":"docs"}}).to_string(),
    )
    .unwrap();
    fs::write(repo.path().join("agent/test-map.json"), json!({"tests":{"README.md":{"command":"echo claimed > executed", "purpose":"exercise documentation"}}}).to_string()).unwrap();
    let receipt = evidence.path().join("receipt.json");
    let args = WitnessArgs {
        repo: repo.path().to_owned(),
        changed: vec!["README.md".into()],
        changed_from: None,
        baseline: None,
        proof_receipts: Some(receipt.to_str().unwrap().into()),
        out: evidence
            .path()
            .join("witness.json")
            .to_str()
            .unwrap()
            .into(),
        md: evidence.path().join("witness.md").to_str().unwrap().into(),
    };
    // A lane name, successful status, plausible identity or copied authority
    // metadata cannot distinguish a file claim from observed execution.
    for lane in ["fast", "required", "security", "renamed-required"] {
        fs::write(repo.path().join("agent/proof-lanes.toml"), format!("[[lane]]\nname = {lane:?}\ncommand = 'echo claimed > executed'\npurpose = 'exercise documentation'\n")).unwrap();
        for exit_code in [0, 73] {
            fs::write(
                &receipt,
                json!({
                    "lane":lane, "command":"echo claimed > executed", "exit_code":exit_code,
                    "elapsed_ms":1, "artifacts":[], "git_head":"0".repeat(40),
                    "changed_paths":["README.md"], "generated_at":"1970-01-01T00:00:00Z",
                    "extensions":{"trust_status":"supervised", "producer_digest":"a".repeat(64)}
                })
                .to_string(),
            )
            .unwrap();
            let witness = build_witness(&args).unwrap();
            assert_eq!(witness.required_lanes, vec![lane]);
            assert_eq!(witness.proofbind.missing_obligation_count, 0);
            assert!(witness.missing_evidence.iter().any(|message| message
                .contains(&format!("`{lane}` has no trusted execution observation"))));
            assert_eq!(witness.decision, "block");
            assert_eq!(witness.available_proof_receipts.len(), 1);
            assert_eq!(witness.available_proof_receipts[0].exit_code, exit_code);
            assert_eq!(
                witness.available_proof_receipts[0].trust_status,
                "unverified-import"
            );
            assert!(!repo.path().join("executed").exists());
        }
    }
}
