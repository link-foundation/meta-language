//! Recorded native recovery: where a language parses with its native grammar,
//! a malformed input is repaired by the native executor's cost-based
//! recovery, which can place its ERROR and MISSING nodes elsewhere than
//! tree-sitter's LR recovery. Each such case of the conformance and
//! generative suites is recorded in `parity/fixtures/native-recovery.json`
//! with the digests of both trees, the repair sites of both and a category,
//! whose justification the file gives. A recorded case checks that the oracle
//! is malformed and is the recorded one, that the native tree is the recorded
//! one, malformed, lossless and consistent with its diagnostics, and that the
//! category follows from the two trees; a case that matches the oracle, or a
//! clean oracle, may not be recorded. Mirrors
//! `js/tests/support/native-recovery.js`.

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::PathBuf;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use super::cst_lines::{parse_cst_lines, row_offsets};

pub const NATIVE_RECOVERY_FILE: &str = "parity/fixtures/native-recovery.json";

fn sha256(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .fold(String::with_capacity(64), |mut hex, byte| {
            use std::fmt::Write as _;
            let _ = write!(hex, "{byte:02x}");
            hex
        })
}

pub fn read_native_recovery() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(NATIVE_RECOVERY_FILE);
    serde_json::from_slice(&fs::read(&path).expect("native recovery records"))
        .expect("native recovery JSON")
}

/// Whether canonical CST lines hold an ERROR or MISSING node.
pub fn is_malformed(text: &str) -> bool {
    parse_cst_lines(text)
        .iter()
        .any(|node| node.error || node.missing)
}

/// The ERROR spans and MISSING leaves of canonical CST lines, and the bytes
/// the ERROR spans cover.
pub struct RepairSites {
    pub errors: Vec<String>,
    pub missing: Vec<String>,
    pub skipped: usize,
}

fn point((row, column): (usize, usize)) -> String {
    format!("{row}:{column}")
}

pub fn repair_sites(text: &str, source: &str) -> RepairSites {
    let offsets = row_offsets(source);
    let nodes = parse_cst_lines(text);
    let mut covered = BTreeSet::new();
    let mut errors = Vec::new();
    let mut missing = Vec::new();
    for node in &nodes {
        if node.error {
            covered.extend(offsets[node.start.0] + node.start.1..offsets[node.end.0] + node.end.1);
            errors.push(format!("{}-{}", point(node.start), point(node.end)));
        }
        if node.missing {
            let kind = if node.named {
                node.kind.clone()
            } else {
                serde_json::to_string(&node.kind).expect("JSON kind")
            };
            missing.push(format!("{kind} {}", point(node.start)));
        }
    }
    RepairSites {
        errors,
        missing,
        skipped: covered.len(),
    }
}

/// The category of a recovery discrepancy, from the repair sites of the
/// oracle and the native tree.
pub fn recovery_category(oracle: &RepairSites, native: &RepairSites) -> &'static str {
    let at = |missing: &[String]| -> Vec<String> {
        missing
            .iter()
            .map(|entry| entry[entry.rfind(' ').map_or(0, |space| space + 1)..].to_string())
            .collect()
    };
    if oracle.errors == native.errors && at(&oracle.missing) == at(&native.missing) {
        return "same-repair-sites";
    }
    match oracle.skipped.cmp(&native.skipped) {
        std::cmp::Ordering::Greater => "oracle-skips-more",
        std::cmp::Ordering::Less => "native-skips-more",
        std::cmp::Ordering::Equal => "same-skipped-bytes",
    }
}

/// The record of one discrepancy: the digests and repair sites of both trees
/// and its category.
pub fn recovery_record(
    language: &str,
    suite: &str,
    id: &str,
    source: &str,
    oracle: &str,
    native: &str,
) -> Value {
    let (oracle_sites, native_sites) = (repair_sites(oracle, source), repair_sites(native, source));
    let sites = |text: &str, sites: &RepairSites| json!({ "cstSha256": sha256(text), "errors": sites.errors, "missing": sites.missing });
    json!({
        "language": language,
        "suite": suite,
        "case": id,
        "category": recovery_category(&oracle_sites, &native_sites),
        "oracle": sites(oracle, &oracle_sites),
        "native": sites(native, &native_sites),
    })
}

/// The recorded discrepancies of one suite and language.
pub struct NativeRecovery {
    language: String,
    suite: String,
    categories: Value,
    records: HashMap<String, Value>,
    used: RefCell<BTreeSet<String>>,
}

impl NativeRecovery {
    pub fn new(suite: &str, language: &str) -> Self {
        let file = read_native_recovery();
        let records = file["cases"]
            .as_array()
            .expect("recorded cases")
            .iter()
            .filter(|record| record["suite"] == suite && record["language"] == language)
            .map(|record| {
                let id = record["case"].as_str().expect("case id").to_string();
                (id, record.clone())
            })
            .collect();
        Self {
            language: language.to_string(),
            suite: suite.to_string(),
            categories: file["categories"].clone(),
            records,
            used: RefCell::new(BTreeSet::new()),
        }
    }

    /// The problems of case `id`: `problems` compare its native tree `text`
    /// with the oracle tree `oracle`, and `check(cst)` gives the problems of
    /// the same parse against other CST lines. Without a record they stand;
    /// with one, they are replaced by the record's checks.
    pub fn resolve(
        &self,
        id: &str,
        source: &str,
        oracle: &str,
        text: &str,
        problems: Vec<String>,
        check: impl FnOnce(&str) -> Vec<String>,
    ) -> Vec<String> {
        let Some(record) = self.records.get(id) else {
            return problems;
        };
        self.used.borrow_mut().insert(id.to_string());
        if problems.is_empty() {
            return vec![
                "the native tree equals the oracle, so the recorded recovery discrepancy is stale"
                    .to_string(),
            ];
        }
        let expected = recovery_record(&self.language, &self.suite, id, source, oracle, text);
        let mut found = Vec::new();
        if !is_malformed(oracle) {
            found.push(
                "a clean oracle tree may not be recorded as a recovery discrepancy".to_string(),
            );
        }
        if !is_malformed(text) {
            found.push("the native tree is clean, so it is no recovery".to_string());
        }
        let category = record["category"].as_str().unwrap_or_default();
        if self.categories.get(category).is_none() {
            found.push(format!("category {category} has no justification"));
        }
        if &expected != record {
            found.push(format!(
                "the recorded recovery differs: {expected} (regenerate with node scripts/generate-native-recovery.mjs after checking it)"
            ));
        }
        found.extend(
            check(text)
                .into_iter()
                .map(|problem| format!("against the native tree: {problem}")),
        );
        found
    }

    /// The records no case met.
    pub fn unused(&self) -> Vec<String> {
        let used = self.used.borrow();
        let mut unused: Vec<String> = self
            .records
            .keys()
            .filter(|id| !used.contains(*id))
            .cloned()
            .collect();
        unused.sort();
        unused
    }
}

#[test]
fn native_recovery_records_have_justified_categories_that_follow_from_their_sites() {
    let file = read_native_recovery();
    let categories = file["categories"].as_object().expect("categories");
    for (name, justification) in categories {
        assert!(
            justification.as_str().is_some_and(|text| text.len() > 80),
            "{name} is justified"
        );
    }
    let cases = file["cases"].as_array().expect("cases");
    assert_ne!(cases.len(), 0);
    for record in cases {
        let category = record["category"].as_str().expect("category");
        assert!(categories.contains_key(category), "{record}");
        assert!(
            record["suite"] == "conformance" || record["suite"] == "generative",
            "{record}"
        );
        // The category follows from the recorded sites alone where the
        // skipped bytes do not decide it.
        let same_sites = record["oracle"]["errors"] == record["native"]["errors"];
        if category == "same-repair-sites" {
            assert!(same_sites, "{record}");
        }
    }
}

#[test]
fn native_recovery_category_compares_the_repair_sites() {
    let source = "ab\ncd\n";
    let error = |span: &str| format!("•source_file 0:0-2:0\n  •ERROR {span}");
    let missing = "•source_file 0:0-2:0\n  MISSING \";\" 0:1-0:1";
    let wide = repair_sites(&error("0:0-1:2"), source);
    let narrow = repair_sites(&error("0:0-0:2"), source);
    assert_eq!(wide.skipped, 5);
    assert_eq!(recovery_category(&wide, &narrow), "oracle-skips-more");
    assert_eq!(recovery_category(&narrow, &wide), "native-skips-more");
    assert_eq!(recovery_category(&narrow, &narrow), "same-repair-sites");
    let shifted = repair_sites(&error("1:0-1:2"), source);
    assert_eq!(recovery_category(&narrow, &shifted), "same-skipped-bytes");
    let inserted = repair_sites(missing, source);
    assert_eq!(inserted.missing, ["\";\" 0:1"]);
    assert_eq!(inserted.skipped, 0);
    assert!(is_malformed(missing));
    assert!(!is_malformed("source_file 0:0-0:0"));
}
