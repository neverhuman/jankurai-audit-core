//! End to end: a forge-gated repository whose lane does not run the audit gets
//! its audit-lane and dimension findings routed at `.jeryu/ci.toml`, never at a
//! workflow file its forge would not execute.

use std::fs;
use std::path::Path;

const DECLARATION: &str = "\
schema_version = \"2\"
provider = \"jeryu\"

[[lane]]
name = \"required\"
command = \"just required\"
runs = [\"jankurai audit\", \"cargo audit\", \"gitleaks\"]
";

const THIN_JUSTFILE: &str = "required:\n    bash scripts/gate.sh\n";
const EMPTY_GATE: &str = "#!/usr/bin/env bash\nset -euo pipefail\ncargo test --workspace\n";

fn write(root: &Path, path: &str, contents: &str) {
    let full = root.join(path);
    fs::create_dir_all(full.parent().unwrap()).unwrap();
    fs::write(full, contents).unwrap();
}

#[test]
fn full_audit_routes_forge_repository_findings_at_the_declaration() {
    let root = tempfile::tempdir().unwrap();
    for (path, contents) in [
        (
            "Cargo.toml",
            "[package]\nname = \"widgetworks-ledger\"\nversion = \"0.1.0\"\n",
        ),
        ("src/lib.rs", "pub fn balance() -> i64 {\n    0\n}\n"),
        (".jeryu/ci.toml", DECLARATION),
        ("Justfile", THIN_JUSTFILE),
        ("scripts/gate.sh", EMPTY_GATE),
    ] {
        write(root.path(), path, contents);
    }
    let report = jankurai::audit::run_audit(root.path(), &[]).unwrap();
    assert!(report
        .caps_applied
        .iter()
        .any(|cap| cap == "no-jankurai-audit-lane-in-ci"));
    let lane = report
        .findings
        .iter()
        .find(|finding| finding.problem == "CI does not run the jankurai audit lane")
        .expect("audit-lane finding");
    assert_eq!(lane.path, ".jeryu/ci.toml");
    assert!(!lane.agent_fix.contains(".github/workflows"));
    let stray: Vec<_> = report
        .findings
        .iter()
        .filter(|finding| finding.path.starts_with(".github/workflows"))
        .map(|finding| (&finding.path, &finding.problem))
        .collect();
    assert!(stray.is_empty(), "{stray:?}");
}
