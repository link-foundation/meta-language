//! Exact primitive receiver proofs and independent original compiler oracles.
//! Rust and Node execution occurs in native CI; fixture storage is shared LiNo.

use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::rust::parse_rust;
use meta_language::{LinkId, LinkNetwork, create_typed_program_links, emit_typed_program_links};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-float-predicates.lino"
    ))
    .expect("shared predicate fixtures are inspectable Links data")
}
#[test]
fn primitive_float_predicates_match_shared_surface_and_source_free_links() {
    for case in fixture()["cases"].as_array().unwrap() {
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
        let imported = create_typed_program_links(source, "Rust").unwrap();
        let native = LinkNetwork::from_lino(&imported.network.to_lino()).unwrap();
        let shared = LinkNetwork::from_lino(case["archive"].as_str().unwrap()).unwrap();
        for (network, root) in [
            (&native, imported.root),
            (&shared, LinkId::from_u64(case["root"].as_u64().unwrap())),
        ] {
            for (target, field) in [("JavaScript", "javascript"), ("Rust", "rust")] {
                assert_eq!(
                    emit_typed_program_links(network, root, target)
                        .unwrap()
                        .code,
                    case[field].as_str().unwrap(),
                    "{id}: {target}"
                );
            }
        }
    }
}
#[test]
fn f32_custom_shadowed_ambiguous_receivers_and_wrong_arity_keep_exact_refusals() {
    for case in fixture()["invalid"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let error = parse_rust(case["source"].as_str().unwrap())
            .and_then(|surface| check_program(&surface))
            .expect_err("the method needs actual primitive source evidence");
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
fn run_javascript(code: &str) -> Vec<String> {
    let result = run(Command::new("node").args(["--input-type=module", "--eval", code]));
    assert!(result.ok, "{}", result.stderr);
    result.stdout.lines().map(str::to_owned).collect()
}
#[test]
fn original_rust_and_javascript_predicates_and_both_emitted_directions_match_independent_oracles() {
    let directory = TemporaryDirectory::new("rust-float-predicates");
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
        assert_eq!(
            run_rust(
                case["reverseRust"].as_str().unwrap(),
                &directory.0,
                &format!("reverse_{index}")
            ),
            expected
        );
        assert_eq!(
            run_javascript(case["originalJavaScript"].as_str().unwrap()),
            expected
        );
        let network = LinkNetwork::from_lino(case["archive"].as_str().unwrap()).unwrap();
        let emitted = emit_typed_program_links(
            &network,
            LinkId::from_u64(case["root"].as_u64().unwrap()),
            "JavaScript",
        )
        .unwrap();
        assert_eq!(run_javascript(&emitted.code), expected);
    }
}
#[test]
fn linked_predicate_edit_changes_native_emission_without_source_replay() {
    let data = fixture();
    let case = data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == "explicit-f64-literal")
        .unwrap();
    let network = LinkNetwork::from_lino(case["archive"].as_str().unwrap()).unwrap();
    let operation = network
        .links()
        .find(|link| link.metadata().definition() == Some("isFinite"))
        .unwrap()
        .id();
    let text = network
        .to_lino()
        .lines()
        .map(|line| {
            if line.starts_with(&format!("({operation}: ")) {
                line.replace("(def: isFinite)", "(def: isNaN)")
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let changed = LinkNetwork::from_lino(&text).unwrap();
    let root = LinkId::from_u64(case["root"].as_u64().unwrap());
    let javascript = emit_typed_program_links(&changed, root, "JavaScript").unwrap();
    assert_eq!(run_javascript(&javascript.code), ["false"]);
    assert!(
        emit_typed_program_links(&changed, root, "Rust")
            .unwrap()
            .code
            .contains(".is_nan()")
    );
}
