#!/usr/bin/env rust-script
//! Check for manual version modification in Cargo.toml
//!
//! This script prevents manual version changes in pull requests.
//! Versions should be managed automatically by the CI/CD pipeline
//! using changelog fragments in changelog.d/.
//!
//! Key behavior:
//! - Detects if `version = "..."` line has changed in Cargo.toml
//! - Fails the CI check if manual version change is detected
//! - Skips check for automated release branches (changelog-manual-release-*)
//!
//! Usage: rust-script scripts/check-version-modification.rs
//!
//! Environment variables (set by GitHub Actions):
//!   - GITHUB_HEAD_REF: The head branch name for PRs
//!   - GITHUB_BASE_REF: The base branch name for PRs
//!   - GITHUB_EVENT_NAME: Should be 'pull_request'
//!
//! Exit codes:
//!   - 0: No manual version changes detected (or check skipped)
//!   - 1: Manual version changes detected
//!
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

use std::env;
use std::path::Path;
use std::process::{Command, exit};
use regex::Regex;

fn exec(command: &str, args: &[&str]) -> String {
    match Command::new(command).args(args).output() {
        Ok(output) => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        Err(_) => String::new(),
    }
}

fn exec_ignore_error(command: &str, args: &[&str]) {
    let _ = Command::new(command)
        .args(args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
}

fn should_skip_version_check() -> bool {
    let head_ref = env::var("GITHUB_HEAD_REF").unwrap_or_default();

    // Skip for automated release PRs
    let automated_branch_prefixes = [
        "changelog-manual-release-",
        "changeset-release/",
        "release/",
        "automated-release/",
    ];

    for prefix in &automated_branch_prefixes {
        if head_ref.starts_with(prefix) {
            println!("Skipping version check for automated branch: {}", head_ref);
            return true;
        }
    }

    false
}

fn get_rust_root() -> String {
    if let Ok(root) = env::var("RUST_ROOT") {
        if !root.is_empty() {
            return root;
        }
    }

    if Path::new("./Cargo.toml").exists() {
        return ".".to_string();
    }

    if Path::new("./rust/Cargo.toml").exists() {
        return "rust".to_string();
    }

    ".".to_string()
}

fn get_cargo_toml_path(rust_root: &str) -> String {
    if rust_root == "." {
        "Cargo.toml".to_string()
    } else {
        format!("{}/Cargo.toml", rust_root)
    }
}

/// Extract the `[package]` version value from raw `Cargo.toml` text.
///
/// Anchored at the start of a line so inline `{ version = "..." }` dependency
/// specifications are ignored and only the package version line matches.
fn extract_version(content: &str) -> Option<String> {
    if content.is_empty() {
        return None;
    }
    let pattern = Regex::new(r#"(?m)^version\s*=\s*"([^"]+)""#).unwrap();
    pattern
        .captures(content)
        .map(|caps| caps[1].to_string())
}

/// Read the package version on the base branch.
///
/// The Rust crate moved from the repository root into `rust/`, so a single PR
/// can legitimately have its manifest at a different path than the base branch.
/// We therefore look for the manifest at the new path first and fall back to the
/// historical root path, comparing version *values* rather than diff lines so a
/// pure file move is not mistaken for a manual version bump.
fn base_version(base_ref: &str, cargo_toml_path: &str) -> Option<String> {
    let candidates = [cargo_toml_path, "rust/Cargo.toml", "Cargo.toml"];
    for path in candidates {
        let content = exec("git", &["show", &format!("origin/{}:{}", base_ref, path)]);
        if let Some(version) = extract_version(&content) {
            return Some(version);
        }
    }
    None
}

fn main() {
    println!("Checking for manual version modifications in Cargo.toml...\n");

    // Only run on pull requests
    let event_name = env::var("GITHUB_EVENT_NAME").unwrap_or_default();
    if event_name != "pull_request" {
        println!("Skipping: Not a pull request event (event: {})", event_name);
        exit(0);
    }

    // Skip for automated release branches
    if should_skip_version_check() {
        exit(0);
    }

    let rust_root = get_rust_root();
    let cargo_toml_path = get_cargo_toml_path(&rust_root);
    let base_ref = env::var("GITHUB_BASE_REF").unwrap_or_else(|_| "main".to_string());

    // Ensure we have the base branch available for `git show`.
    exec_ignore_error("git", &["fetch", "origin", &base_ref, "--depth=1"]);

    let head_content = std::fs::read_to_string(&cargo_toml_path).unwrap_or_default();
    let head_version = extract_version(&head_content);
    let base = base_version(&base_ref, &cargo_toml_path);

    match (base, head_version) {
        (Some(base_version), Some(head_version)) if base_version != head_version => {
            eprintln!("Error: Manual version change detected in Cargo.toml!\n");
            eprintln!(
                "  base ({}): {}\n  head:       {}\n",
                base_ref, base_version, head_version
            );
            eprintln!("Versions are managed automatically by the CI/CD pipeline.");
            eprintln!("Please do not modify the version field directly.\n");
            eprintln!("To trigger a release, add a changelog fragment to changelog.d/");
            eprintln!("with the appropriate bump type (major, minor, or patch).\n");
            eprintln!("See changelog.d/README.md for more information.\n");
            eprintln!("If you need to undo your version change, run:");
            eprintln!("  git checkout origin/{} -- {}", base_ref, cargo_toml_path);
            exit(1);
        }
        _ => {
            println!("Package version field was not changed.");
            println!("Version check passed.");
        }
    }
}
