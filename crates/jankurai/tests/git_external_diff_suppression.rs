//! Every `git diff` the auditor spawns must pass `--no-ext-diff`.
//!
//! A developer or CI host can set `diff.external` globally. Git then hands
//! patch-producing diffs to that tool, which may print nothing, print a
//! non-unified format, or die. Either way the auditor reads a diff it did not
//! produce and reports changed paths or changed lines that are not the
//! repository's. `--no-ext-diff` is not observable on `--name-only` or
//! `--quiet` today, but the flag belongs on every invocation so a later switch
//! to a patch-producing form cannot silently inherit a user's diff tool.

use std::fs;
use std::path::{Path, PathBuf};

fn crate_src() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_sources(&path, out);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

#[test]
fn every_spawned_git_diff_suppresses_the_external_diff_tool() {
    let mut sources = Vec::new();
    rust_sources(&crate_src(), &mut sources);
    assert!(!sources.is_empty(), "no Rust sources found to inspect");

    let mut offenders = Vec::new();
    let mut checked = 0usize;
    for path in &sources {
        let text = fs::read_to_string(path).unwrap();
        for (index, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            // Argument lists spawn git as `["diff", ...]` or `.arg("diff")`.
            let spawns_diff = trimmed.starts_with("\"diff\",")
                || trimmed.contains("[\"diff\",")
                || trimmed.contains("&[\"diff\",")
                || trimmed.contains(".arg(\"diff\")");
            if !spawns_diff {
                continue;
            }
            checked += 1;
            // The flag may sit on the next line of a multi-line argument list.
            let window: String = text
                .lines()
                .skip(index)
                .take(8)
                .collect::<Vec<_>>()
                .join("\n");
            if !window.contains("--no-ext-diff") {
                offenders.push(format!("{}:{}: {trimmed}", path.display(), index + 1));
            }
        }
    }

    assert!(
        checked > 0,
        "the scan matched no git diff invocation; the detection shape drifted"
    );
    assert!(
        offenders.is_empty(),
        "git diff invocations without --no-ext-diff:\n{}",
        offenders.join("\n")
    );
}
