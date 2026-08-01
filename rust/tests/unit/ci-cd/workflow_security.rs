use regex::Regex;
use std::{fs, path::PathBuf};

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn workflow_files() -> Vec<PathBuf> {
    let mut paths = fs::read_dir(repository_root().join(".github/workflows"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("yml" | "yaml")
            )
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn run_blocks(workflow: &str) -> Vec<String> {
    let lines = workflow.lines().collect::<Vec<_>>();
    let mut blocks = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        let indentation = line.len() - line.trim_start().len();
        if !line.trim_start().starts_with("run:") {
            index += 1;
            continue;
        }
        let mut block = vec![line];
        index += 1;
        while index < lines.len() {
            let next = lines[index];
            let next_indentation = next.len() - next.trim_start().len();
            if !next.trim().is_empty() && next_indentation <= indentation {
                break;
            }
            block.push(next);
            index += 1;
        }
        blocks.push(block.join("\n"));
    }
    blocks
}

fn job_block<'a>(workflow: &'a str, job_name: &str) -> &'a str {
    let marker = format!("  {job_name}:\n");
    let start = workflow.find(&marker).unwrap();
    let rest = &workflow[start + marker.len()..];
    let next = rest
        .lines()
        .scan(0usize, |offset, line| {
            let current = *offset;
            *offset += line.len() + 1;
            Some((current, line))
        })
        .find_map(|(offset, line)| {
            (line.starts_with("  ") && !line.starts_with("    ") && line.trim_end().ends_with(':'))
                .then_some(offset)
        });
    next.map_or_else(
        || &workflow[start..],
        |end| &workflow[start..start + marker.len() + end],
    )
}

#[test]
fn run_blocks_do_not_interpolate_untrusted_workflow_inputs() {
    let untrusted =
        Regex::new(r"\$\{\{\s*(?:inputs\.|github\.event\.inputs\.|github\.head_ref)[^}]*\}\}")
            .unwrap();
    for path in workflow_files() {
        let workflow = fs::read_to_string(&path).unwrap().replace("\r\n", "\n");
        for run_block in run_blocks(&workflow) {
            assert!(
                !untrusted.is_match(&run_block),
                "{} interpolates an untrusted value into a run block:\n{run_block}",
                path.display()
            );
        }
    }
}

#[test]
fn workflows_default_to_read_only_permissions() {
    for path in workflow_files() {
        let workflow = fs::read_to_string(&path).unwrap().replace("\r\n", "\n");
        let header = workflow.split("\njobs:\n").next().unwrap();
        assert!(
            header.contains("permissions:\n  contents: read"),
            "{} must declare a read-only token default",
            path.display()
        );
    }
}

#[test]
fn workflows_separate_cancellable_checks_from_serialized_writes() {
    for path in workflow_files() {
        let workflow = fs::read_to_string(&path).unwrap().replace("\r\n", "\n");
        assert!(!workflow
            .split("\njobs:\n")
            .next()
            .unwrap()
            .contains("\nconcurrency:\n"));
    }

    let rust = fs::read_to_string(repository_root().join(".github/workflows/rust.yml")).unwrap();
    for job_name in [
        "detect-changes",
        "changelog",
        "version-check",
        "secrets-scan",
        "fresh-merge",
        "cargo-lock",
        "lint",
        "coverage",
        "build",
    ] {
        let job = job_block(&rust, job_name);
        assert!(
            job.contains("cancel-in-progress: true"),
            "{job_name} must be cancellable"
        );
    }
    assert!(job_block(&rust, "test").contains("test-${{ matrix.os }}"));

    let write_queue = concat!(
        "    concurrency:\n",
        "      group: release-${{ github.repository }}-main-write\n",
        "      cancel-in-progress: false\n",
    );
    for job_name in [
        "auto-release",
        "manual-release",
        "changelog-pr",
        "deploy-docs",
    ] {
        assert!(
            job_block(&rust, job_name).contains(write_queue),
            "{job_name} must use the write queue"
        );
    }

    let js = fs::read_to_string(repository_root().join(".github/workflows/js.yml")).unwrap();
    assert!(job_block(&js, "test").contains("cancel-in-progress: true"));
    assert!(job_block(&js, "publish").contains(write_queue));
}
