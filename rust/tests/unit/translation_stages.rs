//! Runs the Rust translation pipeline on every program recorded in
//! `parity/fixtures/translation-stages.json` and requires each stage to
//! produce what the JavaScript pipeline produced: the same canonical JSON
//! (compared by SHA-256) or the same error kind and message.
//!
//! When a stage differs, `node js/experiments/translation-stage-parity.mjs`
//! dumps both pipelines and prints the JSON paths at which they disagree.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use meta_language::translation::check::check_program;
use meta_language::translation::diagnostics::TranslationError;
use meta_language::translation::emit_common::Emitted;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_lean::emit_lean;
use meta_language::translation::emit_rocq::emit_rocq;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::Program;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::lean::parse_lean;
use meta_language::translation::rocq::parse_rocq;
use meta_language::translation::rust::parse_rust;
use meta_language::translation::surface::SProgram;
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

type Emitter = fn(&Program) -> Result<Emitted, TranslationError>;

const EMITTERS: [(&str, Emitter); 4] = [
    ("javascript", emit_javascript),
    ("rust", emit_rust),
    ("lean", emit_lean),
    ("rocq", emit_rocq),
];

fn repository_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(relative)
}

/// The JavaScript builder's `canonicalJson`: object keys in UTF-16 order, no
/// whitespace, strings escaped as `JSON.stringify` escapes them.
fn canonical(value: &Value, out: &mut String) {
    match value {
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                canonical(item, out);
            }
            out.push(']');
        }
        Value::Object(entries) => {
            let mut keys: Vec<&String> = entries.keys().collect();
            keys.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
            out.push('{');
            for (index, key) in keys.into_iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                out.push_str(&serde_json::to_string(key).expect("keys serialise"));
                out.push(':');
                canonical(&entries[key], out);
            }
            out.push('}');
        }
        other => out.push_str(&other.to_string()),
    }
}

fn digest<T: Serialize>(value: &T) -> String {
    let mut text = String::new();
    canonical(
        &serde_json::to_value(value).expect("stage results serialise"),
        &mut text,
    );
    Sha256::digest(text.as_bytes())
        .iter()
        .fold(String::new(), |mut hex, byte| {
            let _ = write!(hex, "{byte:02x}");
            hex
        })
}

/// Compares a stage's result with its record and returns the successful
/// value only when both runtimes succeeded with the same result.
fn compare<'a, T: Serialize>(
    label: &str,
    actual: &'a Result<T, TranslationError>,
    expected: &Value,
    failures: &mut Vec<String>,
) -> Option<&'a T> {
    match (actual, expected.get("sha256")) {
        (Ok(value), Some(sha256)) => {
            if digest(value) == sha256.as_str().expect("digest") {
                return Some(value);
            }
            failures.push(format!("{label}: the result differs from javascript"));
        }
        (Err(error), None) => {
            let recorded = &expected["error"];
            let (kind, message) = (error.kind.as_str(), error.message());
            if recorded["kind"] != kind || recorded["message"] != message.as_str() {
                failures.push(format!(
                    "{label}: javascript fails with {} {} but rust with {kind} {message:?}",
                    recorded["kind"], recorded["message"]
                ));
            }
        }
        (Ok(_), None) => failures.push(format!(
            "{label}: rust succeeds where javascript fails with {}",
            expected["error"]["message"]
        )),
        (Err(error), Some(_)) => failures.push(format!(
            "{label}: rust fails where javascript succeeds: {}",
            error.message()
        )),
    }
    None
}

fn parse(extension: &str, source: &str) -> Result<SProgram, TranslationError> {
    match extension {
        "lean" => parse_lean(source),
        "v" => parse_rocq(source),
        "rs" => parse_rust(source),
        "mjs" | "js" => parse_javascript(source),
        other => panic!("unknown source extension {other}"),
    }
}

#[test]
#[ignore = "enabled once every frontend and emitter is ported"]
fn every_translation_stage_matches_the_javascript_pipeline() {
    let fixtures: Value = serde_json::from_str(
        &fs::read_to_string(repository_path("parity/fixtures/translation-stages.json"))
            .expect("translation stage fixtures are readable"),
    )
    .expect("translation stage fixtures are valid JSON");
    let programs = fixtures["programs"].as_array().expect("programs");
    assert!(
        programs.len() > 50,
        "the stage fixtures cover the corpus and the case sets"
    );
    let mut failures = Vec::new();
    for program in programs {
        let name = program["name"].as_str().expect("name");
        let extension = program["extension"].as_str().expect("extension");
        let source = program["path"].as_str().map_or_else(
            || program["source"].as_str().expect("source").to_owned(),
            |path| fs::read_to_string(repository_path(path)).expect("corpus source is readable"),
        );
        let surface = parse(extension, &source);
        let Some(surface) = compare(
            &format!("{name} parse"),
            &surface,
            &program["parse"],
            &mut failures,
        ) else {
            continue;
        };
        let checked = check_program(surface);
        let Some(checked) = compare(
            &format!("{name} check"),
            &checked,
            &program["check"],
            &mut failures,
        ) else {
            continue;
        };
        for (target, emit) in EMITTERS {
            compare(
                &format!("{name} emit {target}"),
                &emit(checked),
                &program["emit"][target],
                &mut failures,
            );
        }
    }
    assert!(
        failures.is_empty(),
        "{} stage results differ from the javascript pipeline:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
