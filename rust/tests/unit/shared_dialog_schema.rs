//! Validates the shared-dialog source-description schema and its examples.
//!
//! The schema is defined once, for every repository that captures or consumes
//! shared AI dialogs, under `docs/schemas/shared-dialog/`. This test gives the
//! Rust reference implementation three guarantees:
//!
//! 1. Every `*.json` example matches `shared-dialog.schema.json` (required
//!    fields, enum membership, and the captured-vs-diagnostic invariant). The
//!    enums and required lists are read from the schema file itself so the test
//!    and schema can never drift apart.
//! 2. Every `*.lino` example round-trips losslessly through `LinkNetwork::parse`
//!    / `reconstruct_text`, proving the meta-language carries the schema with no
//!    loss (the property web-capture relies on to emit either form).
//! 3. The `demo-memory-mapping.json` projection preserves provider, source URL,
//!    turn role, and turn content for every turn, matching the formal-ai
//!    `demo_memory` mapping documented in the schema README.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use meta_language::{LinkNetwork, ParseConfiguration};
use serde_json::Value;

fn schema_dir() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        let candidate = dir.join("docs/schemas/shared-dialog");
        if candidate.is_dir() {
            return candidate;
        }
        assert!(
            dir.pop(),
            "could not locate docs/schemas/shared-dialog from the crate manifest"
        );
    }
}

fn read_json(path: &Path) -> Value {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

fn string_array(value: &Value, pointer: &str) -> Vec<String> {
    value
        .pointer(pointer)
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("schema is missing array at {pointer}"))
        .iter()
        .map(|item| {
            item.as_str()
                .unwrap_or_else(|| panic!("non-string entry in {pointer}"))
                .to_string()
        })
        .collect()
}

/// The parts of `shared-dialog.schema.json` the validator enforces, lifted from
/// the schema file so there is a single source of truth.
struct SchemaRules {
    source_required: Vec<String>,
    capture_status: BTreeSet<String>,
    capture_method: BTreeSet<String>,
    role: BTreeSet<String>,
    visibility: BTreeSet<String>,
    turn_required: Vec<String>,
    diagnostic_required: Vec<String>,
}

impl SchemaRules {
    fn load(schema: &Value) -> Self {
        let set = |pointer: &str| -> BTreeSet<String> {
            string_array(schema, pointer).into_iter().collect()
        };
        Self {
            source_required: string_array(schema, "/required"),
            capture_status: set("/$defs/captureStatus/enum"),
            capture_method: set("/$defs/captureMethod/enum"),
            role: set("/$defs/role/enum"),
            visibility: set("/$defs/visibility/enum"),
            turn_required: string_array(schema, "/$defs/sharedDialogTurn/required"),
            diagnostic_required: string_array(
                schema,
                "/$defs/sharedDialogCaptureDiagnostic/required",
            ),
        }
    }

    fn check_source(&self, label: &str, source: &Value) {
        let object = source
            .as_object()
            .unwrap_or_else(|| panic!("{label}: source must be an object"));
        for field in &self.source_required {
            assert!(
                object.contains_key(field),
                "{label}: missing required field `{field}`"
            );
        }

        let status = source["capture_status"].as_str().unwrap();
        assert!(
            self.capture_status.contains(status),
            "{label}: capture_status `{status}` is not a known status value"
        );
        let method = source["capture_method"].as_str().unwrap();
        assert!(
            self.capture_method.contains(method),
            "{label}: capture_method `{method}` is not a known method"
        );

        if status == "captured" {
            let turns = source["turns"]
                .as_array()
                .unwrap_or_else(|| panic!("{label}: captured source must have a turns array"));
            assert!(!turns.is_empty(), "{label}: captured source has no turns");
            for turn in turns {
                self.check_turn(label, turn);
            }
        } else {
            let diagnostics = source["diagnostics"].as_array().unwrap_or_else(|| {
                panic!("{label}: non-captured source must have a diagnostics array")
            });
            assert!(
                !diagnostics.is_empty(),
                "{label}: non-captured source has no diagnostics"
            );
            for diagnostic in diagnostics {
                self.check_diagnostic(label, diagnostic);
            }
        }
    }

    fn check_turn(&self, label: &str, turn: &Value) {
        let object = turn
            .as_object()
            .unwrap_or_else(|| panic!("{label}: turn must be an object"));
        for field in &self.turn_required {
            assert!(
                object.contains_key(field),
                "{label}: turn missing required field `{field}`"
            );
        }
        let role = turn["role"].as_str().unwrap();
        assert!(
            self.role.contains(role),
            "{label}: turn role `{role}` is not a known role"
        );
        assert!(
            turn["order"].is_u64() || turn["order"].is_i64(),
            "{label}: turn order must be an integer"
        );
        if let Some(visibility) = turn.get("visibility").and_then(Value::as_str) {
            assert!(
                self.visibility.contains(visibility),
                "{label}: turn visibility `{visibility}` is not a known visibility"
            );
        }
    }

    fn check_diagnostic(&self, label: &str, diagnostic: &Value) {
        let object = diagnostic
            .as_object()
            .unwrap_or_else(|| panic!("{label}: diagnostic must be an object"));
        for field in &self.diagnostic_required {
            assert!(
                object.contains_key(field),
                "{label}: diagnostic missing required field `{field}`"
            );
        }
    }
}

fn example_files(extension: &str) -> Vec<PathBuf> {
    let dir = schema_dir().join("examples");
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().and_then(|e| e.to_str()) == Some(extension))
        .collect();
    files.sort();
    files
}

#[test]
fn json_examples_match_schema() {
    let schema = read_json(&schema_dir().join("shared-dialog.schema.json"));
    let rules = SchemaRules::load(&schema);

    let files = example_files("json");
    assert!(!files.is_empty(), "expected JSON examples to validate");

    let mut captured = 0;
    let mut diagnostics = 0;
    for path in files {
        let label = path.file_name().unwrap().to_string_lossy().into_owned();
        let value = read_json(&path);
        // The mapping example wraps the instance under `source`.
        let source = value.get("source").unwrap_or(&value);
        rules.check_source(&label, source);
        match source["capture_status"].as_str().unwrap() {
            "captured" => captured += 1,
            _ => diagnostics += 1,
        }
    }
    assert!(captured >= 3, "expected at least three captured examples");
    assert!(diagnostics >= 1, "expected at least one diagnostic example");
}

#[test]
fn schema_covers_required_capture_status_values() {
    let schema = read_json(&schema_dir().join("shared-dialog.schema.json"));
    let rules = SchemaRules::load(&schema);
    for required in [
        "captured",
        "unsupported_provider_format",
        "provider_challenge",
        "login_required",
        "expired_or_deleted",
        "no_transcript_found",
    ] {
        assert!(
            rules.capture_status.contains(required),
            "schema is missing required capture_status value `{required}`"
        );
    }
}

#[test]
fn lino_examples_round_trip_losslessly() {
    let files = example_files("lino");
    assert!(!files.is_empty(), "expected LiNo examples to round-trip");
    for path in files {
        let label = path.file_name().unwrap().to_string_lossy().into_owned();
        let text = fs::read_to_string(&path).unwrap();
        let network = LinkNetwork::parse(&text, "LiNo", ParseConfiguration::default());
        assert_eq!(
            network.reconstruct_text(),
            text,
            "{label}: LiNo example did not round-trip losslessly"
        );
    }
}

#[test]
fn demo_memory_mapping_is_lossless() {
    let mapping = read_json(&schema_dir().join("examples/demo-memory-mapping.json"));
    let source = &mapping["source"];
    let events = mapping["events"].as_array().expect("events array");
    let turns = source["turns"].as_array().expect("turns array");

    assert_eq!(
        events.len(),
        turns.len(),
        "mapping must emit one event per turn"
    );

    let provider = source["provider"].as_str().unwrap();
    let source_url = source["source_url"].as_str().unwrap();

    for (turn, event) in turns.iter().zip(events) {
        assert_eq!(
            event["provider"].as_str(),
            Some(provider),
            "event lost provider"
        );
        assert_eq!(
            event["source_url"].as_str(),
            Some(source_url),
            "event lost source_url"
        );
        assert_eq!(event["role"], turn["role"], "event lost turn role");
        assert_eq!(event["content"], turn["content"], "event lost turn content");
    }
}
