//! Exact source-origin premise, shared diagnostic and independent source oracles.

use std::fs;
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
        "../../../parity/fixtures/rust-scalar-parameters.lino"
    ))
    .expect("the unwritten scalar parameter fixture is valid Links data")
}

#[test]
fn unwritten_scalar_parameters_retain_shared_surface_and_linked_emission() {
    for case in fixture()["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let source = case["source"].as_str().unwrap();
        let surface = parse_rust(source).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_eq!(
            serde_json::to_value(&surface).unwrap(),
            case["surface"],
            "{id}"
        );
        let checked = check_program(&surface).unwrap();
        assert_eq!(
            emit_javascript(&checked).unwrap().text,
            case["javascript"].as_str().unwrap(),
            "{id}"
        );
        let definitions = emit_rust(&checked).unwrap().definitions.join("\n\n");
        assert_eq!(definitions, case["definitions"].as_str().unwrap(), "{id}");
        let (_, main) = source.split_once(" fn main()").unwrap();
        let replay =
            check_program(&parse_rust(&format!("{definitions} fn main(){main}")).unwrap()).unwrap();
        assert_eq!(
            emit_javascript(&replay).unwrap().text,
            case["replayJavaScript"].as_str().unwrap(),
            "{id}"
        );
        let archive = create_typed_program_links(source, "Rust").unwrap();
        let network = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
        assert_eq!(
            emit_typed_program_links(&network, archive.root, "JavaScript")
                .unwrap()
                .code,
            case["javascript"].as_str().unwrap(),
            "{id}"
        );
        assert!(
            emit_typed_program_links(&network, archive.root, "Rust")
                .unwrap()
                .code
                .contains(&definitions),
            "{id}"
        );
    }
}

#[test]
fn written_and_unproved_parameters_retain_original_mut_spans() {
    for case in fixture()["invalid"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let error = parse_rust(case["source"].as_str().unwrap())
            .and_then(|surface| check_program(&surface))
            .expect_err("unproved mut declarations remain refused");
        assert_eq!(error.kind.as_str(), case["kind"].as_str().unwrap(), "{id}");
        assert_eq!(error.reason, case["reason"].as_str().unwrap(), "{id}");
        assert_eq!(
            serde_json::to_value(error.span).unwrap(),
            case["span"],
            "{id}"
        );
    }
}

// Compilation belongs to CI. The source is compiled unchanged, separately
// from the real generated Rust program; the JavaScript tests use the same rows.
#[test]
fn original_and_generated_scalar_sources_match_independent_stdout() {
    let directory = TemporaryDirectory::new("rust-unwritten-scalar-parameters");
    for (index, case) in fixture()["cases"].as_array().unwrap().iter().enumerate() {
        if !case["nativeOracle"].as_bool().unwrap() {
            continue;
        }
        let original = case["source"].as_str().unwrap();
        let checked = check_program(&parse_rust(original).unwrap()).unwrap();
        let generated = emit_rust(&checked).unwrap().text;
        for (variant, source) in [("original", original), ("generated", generated.as_str())] {
            let file = directory.0.join(format!("scalar_{index}_{variant}.rs"));
            let binary = directory.0.join(format!(
                "scalar_{index}_{variant}{}",
                std::env::consts::EXE_SUFFIX
            ));
            fs::write(&file, source).unwrap();
            let mut build_command = Command::new("rustc");
            build_command.args(["--edition", "2024"]);
            #[cfg(windows)]
            if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
                build_command.arg("-C").arg(format!("linker={linker}"));
            }
            let compiled = run(build_command.arg(&file).arg("-o").arg(&binary));
            assert!(compiled.ok, "{index}: {variant}: {}", compiled.stderr);
            let observed = run(&mut Command::new(binary));
            assert!(observed.ok, "{index}: {variant}: {}", observed.stderr);
            assert_eq!(
                observed.stdout.lines().collect::<Vec<_>>(),
                case["expectedOutput"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap())
                    .collect::<Vec<_>>(),
                "{index}: {variant}"
            );
        }
    }
}
