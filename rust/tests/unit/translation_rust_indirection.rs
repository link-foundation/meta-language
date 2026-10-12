//! The original immutable Rust ownership contract survives value projection.

use std::fs;
use std::path::Path;
use std::process::Command;

use meta_language::translation::check::check_program;
use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::rust::parse_rust;
use serde_json::Value;

use super::issue_195_translation_pairs::{TemporaryDirectory, run};

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../parity/fixtures/rust-source-indirection.json"
    ))
    .expect("the shared Rust indirection fixture is valid JSON")
}

#[test]
fn immutable_source_indirection_matches_shared_surface_and_emitters() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().expect("accepted sources") {
        let id = case["id"].as_str().expect("fixture identity");
        let source = case["source"].as_str().expect("original Rust source");
        let parsed = parse_rust(source).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_eq!(
            serde_json::to_value(&parsed).expect("the surface serializes"),
            case["surface"],
            "{id}: the original source has the same value projection and spans"
        );
        let program = check_program(&parsed).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_eq!(
            emit_javascript(&program).expect("JavaScript emission").text,
            case["javascript"]
                .as_str()
                .expect("shared JavaScript target"),
            "{id}: the value projection emits the shared JavaScript target"
        );
        assert_eq!(
            emit_rust(&program).expect("Rust emission").text,
            case["rust"].as_str().expect("shared Rust target"),
            "{id}: the recursive data fields are boxed and unboxed by the emitter"
        );
    }
}

#[test]
fn unproven_source_indirection_matches_shared_refusal_and_span() {
    let fixture = fixture();
    for case in fixture["invalid"].as_array().expect("refused sources") {
        let id = case["id"].as_str().expect("fixture identity");
        let source = case["source"].as_str().expect("original Rust source");
        let error = parse_rust(source).expect_err("unproven ownership must be refused");
        assert_eq!(error.kind, ErrorKind::Unsupported, "{id}");
        assert_eq!(
            error.reason,
            case["reason"].as_str().expect("shared refusal"),
            "{id}"
        );
        assert_eq!(
            serde_json::to_value(error.span).expect("the source span serializes"),
            case["span"],
            "{id}"
        );
    }
}

fn run_rust(code: &str, directory: &Path, name: &str) -> Vec<String> {
    let source = directory.join(format!("{name}.rs"));
    let binary = directory.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    fs::write(&source, code).expect("the original or emitted Rust source writes");
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    let compiled = run(compiler.arg(&source).arg("-o").arg(&binary));
    assert!(compiled.ok, "{name}: {}", compiled.stderr);
    let executed = run(&mut Command::new(binary));
    assert!(executed.ok, "{name}: {}", executed.stderr);
    executed.stdout.lines().map(str::to_owned).collect()
}

// This compiler oracle runs in CI. Local validation uses the JavaScript
// runtime and formatting; it never executes this native compilation test.
#[test]
fn original_and_emitted_rust_preserve_source_indirection_outputs() {
    let fixture = fixture();
    let directory = TemporaryDirectory::new("rust-source-indirection");
    for (index, case) in fixture["cases"]
        .as_array()
        .expect("accepted sources")
        .iter()
        .enumerate()
    {
        let id = case["id"].as_str().expect("fixture identity");
        let source = case["source"].as_str().expect("original Rust source");
        let parsed = parse_rust(source).unwrap_or_else(|error| panic!("{id}: {error}"));
        let program = check_program(&parsed).unwrap_or_else(|error| panic!("{id}: {error}"));
        let emitted = emit_rust(&program).unwrap_or_else(|error| panic!("{id}: {error}"));
        let expected = case["output"]
            .as_array()
            .expect("observed source output")
            .iter()
            .map(|line| line.as_str().expect("output line").to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            run_rust(source, &directory.0, &format!("original_{index}")),
            expected,
            "{id}: the original Rust ownership is accepted by rustc and preserves behavior"
        );
        assert_eq!(
            run_rust(&emitted.text, &directory.0, &format!("emitted_{index}")),
            expected,
            "{id}: emitted enum boxing and pattern unboxing preserve behavior"
        );
    }
}
