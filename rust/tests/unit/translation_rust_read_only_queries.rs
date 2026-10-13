//! Shared source projections; original Rust compilation executes only in CI.

use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::rust::parse_rust;
use meta_language::{LinkNetwork, create_typed_program_links, emit_typed_program_links};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

use super::issue_195_translation_pairs::{TemporaryDirectory, run};

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-read-only-queries.lino"
    ))
    .expect("readonly query fixtures are inspectable Links data")
}

#[test]
fn standard_readonly_queries_match_shared_surface_and_source_free_emission() {
    for case in fixture()["cases"].as_array().expect("source cases") {
        let id = case["id"].as_str().unwrap();
        let source = case["source"].as_str().unwrap();
        let surface = parse_rust(source).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_eq!(
            serde_json::to_value(&surface).unwrap(),
            case["surface"],
            "{id}"
        );
        let program = check_program(&surface).unwrap();
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
fn unproved_query_owners_and_pointer_layers_keep_exact_refusals() {
    for case in fixture()["invalid"].as_array().expect("source refusals") {
        let id = case["id"].as_str().unwrap();
        let error = parse_rust(case["source"].as_str().unwrap())
            .and_then(|surface| check_program(&surface))
            .expect_err("only the declared standard readonly queries can enter the model");
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
    fs::write(&source, code).unwrap();
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

#[test]
fn independent_original_and_emitted_rust_queries_match_shared_observations() {
    let directory = TemporaryDirectory::new("rust-read-only-queries");
    for (index, case) in fixture()["cases"].as_array().unwrap().iter().enumerate() {
        let expected = case["output"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            run_rust(
                case["source"].as_str().unwrap(),
                &directory.0,
                &format!("original_{index}")
            ),
            expected
        );
        assert_eq!(
            run_rust(
                case["rust"].as_str().unwrap(),
                &directory.0,
                &format!("emitted_{index}")
            ),
            expected
        );
    }
}
