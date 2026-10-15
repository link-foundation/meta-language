//! Shared immutable input-lifetime projections; original compilers run in CI.

use std::fs;
use std::path::Path;
use std::process::Command;

use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::rust::parse_rust;
use meta_language::{LinkNetwork, create_typed_program_links, emit_typed_program_links};
use serde_json::Value;

use super::issue_195_translation_pairs::{TemporaryDirectory, run};

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-function-lifetimes.lino"
    ))
    .expect("the shared lifetime fixture is inspectable Links data")
}

#[test]
fn scoped_lifetime_inputs_match_shared_projection_and_source_free_emission() {
    for case in fixture()["cases"].as_array().expect("accepted sources") {
        let id = case["id"].as_str().expect("identity");
        let source = case["source"].as_str().expect("Rust source");
        let surface = parse_rust(source).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_eq!(
            serde_json::to_value(&surface).unwrap(),
            case["surface"],
            "{id}"
        );
        let program = check_program(&surface).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_eq!(
            emit_javascript(&program).unwrap().text,
            case["javascript"].as_str().unwrap(),
            "{id}"
        );
        assert_eq!(
            emit_rust(&program).unwrap().text,
            case["rust"].as_str().unwrap(),
            "{id}"
        );
        let archive = create_typed_program_links(source, "Rust").unwrap();
        let reloaded = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
        for (target, field) in [("JavaScript", "javascript"), ("Rust", "rust")] {
            assert_eq!(
                emit_typed_program_links(&reloaded, archive.root, target)
                    .unwrap()
                    .code,
                case[field].as_str().unwrap(),
                "{id}: {target}"
            );
        }
    }
}

#[test]
fn undeclared_and_unproved_lifetime_scopes_match_shared_obligations() {
    for case in fixture()["invalid"].as_array().expect("refused sources") {
        let id = case["id"].as_str().expect("identity");
        let error = parse_rust(case["source"].as_str().unwrap())
            .and_then(|surface| check_program(&surface))
            .expect_err("unproved lifetime scopes cannot enter the selected model");
        assert_eq!(error.kind.as_str(), case["kind"].as_str().unwrap(), "{id}");
        assert_eq!(error.reason, case["reason"].as_str().unwrap(), "{id}");
        assert_eq!(
            serde_json::to_value(error.span).unwrap(),
            case["span"],
            "{id}"
        );
    }
}

fn run_rust(code: &str, directory: &Path, name: &str) -> Vec<String> {
    let source = directory.join(format!("{name}.rs"));
    let binary = directory.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    fs::write(&source, code).expect("the source writes");
    let mut build_command = Command::new("rustc");
    build_command.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        build_command.arg("-C").arg(format!("linker={linker}"));
    }
    let compiled = run(build_command.arg(&source).arg("-o").arg(&binary));
    assert!(compiled.ok, "{name}: {}", compiled.stderr);
    let executed = run(&mut Command::new(binary));
    assert!(executed.ok, "{name}: {}", executed.stderr);
    executed.stdout.lines().map(str::to_owned).collect()
}

// This independent rustc oracle executes only in native CI.
#[test]
fn original_lifetime_sources_and_emitted_values_have_the_same_outputs() {
    let directory = TemporaryDirectory::new("rust-function-lifetimes");
    for (index, case) in fixture()["cases"].as_array().unwrap().iter().enumerate() {
        let expected = case["output"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let source = case["source"].as_str().unwrap();
        assert_eq!(
            run_rust(source, &directory.0, &format!("original_{index}")),
            expected
        );
        let archive = create_typed_program_links(source, "Rust").unwrap();
        let reloaded = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
        let emitted = emit_typed_program_links(&reloaded, archive.root, "Rust").unwrap();
        assert_eq!(
            run_rust(&emitted.code, &directory.0, &format!("emitted_{index}")),
            expected
        );
    }
}
