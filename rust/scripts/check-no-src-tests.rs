#!/usr/bin/env rust-script
//! Reject Rust tests under `src/`.
//!
//! Tests for this repository belong under `tests/` so the library and binary
//! source tree stay free of private unit-test modules.
//!
//! Usage: rust-script scripts/check-no-src-tests.rs
//!
//! ```cargo
//! [dependencies]
//! walkdir = "2"
//! ```

use std::fs;
use std::path::Path;
#[cfg(not(test))]
use std::process::exit;
use walkdir::WalkDir;

#[derive(Debug, PartialEq, Eq)]
struct SrcTestFinding {
    file: String,
    line: usize,
    marker: &'static str,
}

fn is_rust_source(path: &Path) -> bool {
    path.extension().and_then(|extension| extension.to_str()) == Some("rs")
}

fn relative_path(path: &Path, cwd: &Path) -> String {
    let relative = path
        .strip_prefix(cwd)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();

    relative.replace(std::path::MAIN_SEPARATOR, "/")
}

fn contains_test_token(text: &str) -> bool {
    text.split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .any(|token| token == "test")
}

fn test_marker(line: &str) -> Option<&'static str> {
    let trimmed = line.trim_start();

    if trimmed.starts_with("#[test") {
        Some("#[test]")
    } else if trimmed.starts_with("#[tokio::test") {
        Some("#[tokio::test]")
    } else if trimmed.starts_with("#[async_std::test") {
        Some("#[async_std::test]")
    } else if trimmed.starts_with("#[rstest") {
        Some("#[rstest]")
    } else if trimmed.starts_with("#[cfg") && contains_test_token(trimmed) {
        Some("#[cfg(test)]")
    } else if trimmed.starts_with("mod tests") || trimmed.starts_with("pub mod tests") {
        Some("mod tests")
    } else {
        None
    }
}

fn check_file(path: &Path, cwd: &Path) -> Vec<SrcTestFinding> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Warning: Could not read {}: {error}", path.display());
            return Vec::new();
        }
    };

    content
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            test_marker(line).map(|marker| SrcTestFinding {
                file: relative_path(path, cwd),
                line: index + 1,
                marker,
            })
        })
        .collect()
}

fn check_src_directory(cwd: &Path) -> Vec<SrcTestFinding> {
    let src = cwd.join("src");
    if !src.exists() {
        return Vec::new();
    }

    let mut findings = Vec::new();
    for entry in WalkDir::new(&src)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        if is_rust_source(path) {
            findings.extend(check_file(path, cwd));
        }
    }

    findings.sort_by(|left, right| {
        left.file
            .cmp(&right.file)
            .then_with(|| left.line.cmp(&right.line))
            .then_with(|| left.marker.cmp(right.marker))
    });
    findings
}

#[cfg(not(test))]
fn print_findings(findings: &[SrcTestFinding]) {
    if findings.is_empty() {
        return;
    }

    println!("Found Rust tests under src/:\n");
    for finding in findings {
        println!("  {}:{} {}", finding.file, finding.line, finding.marker);
    }
    println!("\nMove these tests to tests/ and keep src/ free of test modules.\n");
}

#[cfg(not(test))]
fn main() {
    println!("Checking that Rust tests live outside src/...\n");

    let cwd = std::env::current_dir().expect("Failed to get current directory");
    let findings = check_src_directory(&cwd);

    if findings.is_empty() {
        println!("No tests found under src/\n");
        exit(0);
    }

    print_findings(&findings);
    exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("check-no-src-tests-{name}-{nanos}"));
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn ignores_tests_outside_src() {
        let repo = temp_dir("outside-src");
        fs::create_dir_all(repo.join("tests")).unwrap();
        fs::write(repo.join("tests/api.rs"), "#[test]\nfn api_contract() {}\n").unwrap();

        assert_eq!(check_src_directory(&repo), Vec::new());
    }

    #[test]
    fn reports_test_attributes_and_modules_under_src() {
        let repo = temp_dir("src-markers");
        fs::create_dir_all(repo.join("src/nested")).unwrap();
        fs::write(
            repo.join("src/lib.rs"),
            "pub fn value() -> u8 { 1 }\n\n#[cfg(test)]\nmod tests {}\n",
        )
        .unwrap();
        fs::write(
            repo.join("src/nested/runtime.rs"),
            "pub fn run() {}\n\n#[tokio::test]\nasync fn runtime_contract() {}\n",
        )
        .unwrap();

        assert_eq!(
            check_src_directory(&repo),
            vec![
                SrcTestFinding {
                    file: "src/lib.rs".to_string(),
                    line: 3,
                    marker: "#[cfg(test)]",
                },
                SrcTestFinding {
                    file: "src/lib.rs".to_string(),
                    line: 4,
                    marker: "mod tests",
                },
                SrcTestFinding {
                    file: "src/nested/runtime.rs".to_string(),
                    line: 3,
                    marker: "#[tokio::test]",
                },
            ]
        );
    }

    #[test]
    fn marker_detection_accepts_supported_test_forms() {
        assert_eq!(test_marker("#[test]"), Some("#[test]"));
        assert_eq!(
            test_marker("  #[async_std::test]"),
            Some("#[async_std::test]")
        );
        assert_eq!(test_marker("#[rstest]"), Some("#[rstest]"));
        assert_eq!(
            test_marker("#[cfg(any(test, feature = \"x\"))]"),
            Some("#[cfg(test)]")
        );
        assert_eq!(test_marker("#[cfg(feature = \"contest\")]"), None);
        assert_eq!(test_marker("pub mod tests {"), Some("mod tests"));
        assert_eq!(test_marker("let test_value = 1;"), None);
    }
}
