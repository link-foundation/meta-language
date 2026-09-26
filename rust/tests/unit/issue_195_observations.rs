//! Shared writer of issue #195 execution records.
//!
//! The evidence runner (`js/scripts/run-issue-195-evidence.mjs`) sets
//! `ISSUE_195_OBSERVATION_FILE` and `ISSUE_195_COMMIT`, runs the suites, and
//! accepts a requirement cell only when every declared assertion has a passed
//! record for every fixture. Tests call [`record`] after their assertions
//! hold, so a failing test never produces a record. Mirrors
//! `js/tests/support/issue-195-observations.js`.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use sha2::{Digest, Sha256};

/// The shared four-language corpus: semantics, transformations, renames,
/// translations, shared concepts, and parity.
pub const FOUR_LANGUAGE_FIXTURE: &str = "parity/fixtures/four-language-conformance.json";
// Suites adopt these as their requirement cells gain evidence.
#[allow(dead_code)]
/// External conformance, generative, and native-validation evidence.
pub const EVIDENCE_FIXTURE: &str = "parity/fixtures/issue-195-evidence.json";
/// The grammar importer corpus.
#[allow(dead_code)]
pub const GRAMMAR_IMPORTER_FIXTURE: &str = "parity/fixtures/grammar-importers.json";

struct Journal {
    digests: HashMap<&'static str, String>,
    recorded: HashSet<(String, String, String)>,
}

fn journal() -> &'static Mutex<Journal> {
    static JOURNAL: OnceLock<Mutex<Journal>> = OnceLock::new();
    JOURNAL.get_or_init(|| {
        Mutex::new(Journal {
            digests: HashMap::new(),
            recorded: HashSet::new(),
        })
    })
}

/// Mirrors `slug` in `js/scripts/issue-195-requirements.mjs` for the ASCII
/// names the requirement catalog uses.
#[allow(dead_code)]
#[must_use]
pub fn slug(value: &str) -> String {
    let normalized = value
        .to_ascii_lowercase()
        .replace('+', "-plus")
        .replace('#', "-sharp");
    let mut slug = String::new();
    for character in normalized.chars() {
        if character.is_ascii_lowercase() || character.is_ascii_digit() {
            slug.push(character);
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    slug.trim_matches('-').to_owned()
}

fn fixture_digest(journal: &mut Journal, fixture_file: &'static str) -> String {
    journal
        .digests
        .entry(fixture_file)
        .or_insert_with(|| {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join(fixture_file);
            let bytes = fs::read(&path)
                .unwrap_or_else(|error| panic!("{} is readable: {error}", path.display()));
            format!("{:x}", Sha256::digest(bytes))
        })
        .clone()
}

/// One verification cell: the records it produces are
/// `${requirement_id}-rust-${suffix}` × `assertions` for `fixture_id`.
pub struct Observation<'a> {
    pub requirement_id: &'a str,
    pub suffix: &'a str,
    pub fixture_id: &'a str,
    pub fixture_file: &'static str,
    pub assertions: &'a [&'a str],
    pub test_name: &'a str,
}

/// Records that the observation's assertions passed. Repeated records of the
/// same cell are written once, because the runner rejects duplicates.
pub fn record(observation: &Observation<'_>) {
    let Some(path) = std::env::var_os("ISSUE_195_OBSERVATION_FILE") else {
        return;
    };
    let commit = std::env::var("ISSUE_195_COMMIT").expect("observation commit");
    let test_id =
        format!("{}-rust-{}", observation.requirement_id, observation.suffix).to_ascii_lowercase();
    let lines = {
        let mut journal = journal().lock().expect("observation journal lock");
        let fixture_digest = fixture_digest(&mut journal, observation.fixture_file);
        let mut lines = String::new();
        for assertion_id in observation.assertions {
            let key = (
                test_id.clone(),
                (*assertion_id).to_owned(),
                observation.fixture_id.to_owned(),
            );
            if !journal.recorded.insert(key) {
                continue;
            }
            let record = serde_json::json!({
                "testId": test_id,
                "assertionId": assertion_id,
                "fixtureId": observation.fixture_id,
                "fixtureDigest": fixture_digest,
                "runtime": "rust",
                "commit": commit,
                "outcome": "passed",
                "testName": observation.test_name,
            });
            lines.push_str(&record.to_string());
            lines.push('\n');
        }
        drop(journal);
        lines
    };
    if lines.is_empty() {
        return;
    }
    // One `write` call, so records of the concurrently running JavaScript and
    // Rust suites never interleave within a line.
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("observation file opens");
    let written = file
        .write(lines.as_bytes())
        .expect("observation records write");
    assert_eq!(written, lines.len(), "observation records write at once");
}
