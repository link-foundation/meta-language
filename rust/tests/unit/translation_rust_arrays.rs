//! Bounded read-only Rust array source projection and actual definition replay.

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
        "../../../parity/fixtures/rust-array-source.lino"
    ))
    .expect("the shared source array fixture is valid Links data")
}

#[test]
fn source_arrays_retain_checked_surface_and_reparse_actual_emitted_definitions() {
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
        let definitions = emit_rust(&program).unwrap().definitions.join("\n\n");
        assert_eq!(
            definitions,
            case["rustDefinitions"].as_str().unwrap(),
            "{id}"
        );
        let replay =
            check_program(&parse_rust(&format!("{definitions}\nfn main(){{}}")).unwrap()).unwrap();
        assert_eq!(
            emit_rust(&replay).unwrap().definitions.join("\n\n"),
            definitions,
            "{id}"
        );
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
fn unsupported_array_ownership_and_borrowing_match_shared_diagnostics() {
    for case in fixture()["invalid"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let error = parse_rust(case["source"].as_str().unwrap())
            .and_then(|surface| check_program(&surface))
            .expect_err("unproved source array semantics remain explicit");
        assert_eq!(error.kind.as_str(), case["kind"].as_str().unwrap(), "{id}");
        assert_eq!(error.reason, case["reason"].as_str().unwrap(), "{id}");
        assert_eq!(
            serde_json::to_value(error.span).unwrap(),
            case["span"],
            "{id}"
        );
    }
}

// Native execution runs only in CI; the parser's input and both actual
// definitions are compiled with an independent caller outside the projection.
#[test]
fn original_and_reparsed_array_policies_observe_the_same_bounded_inputs() {
    let directory = TemporaryDirectory::new("rust-array-source");
    for (case_index, case) in fixture()["cases"].as_array().unwrap().iter().enumerate() {
        let original = case["source"]
            .as_str()
            .unwrap()
            .strip_suffix(" fn main(){}")
            .unwrap();
        let generated = case["rustDefinitions"].as_str().unwrap();
        let replay =
            check_program(&parse_rust(&format!("{generated}\nfn main(){{}}")).unwrap()).unwrap();
        let replayed = emit_rust(&replay).unwrap().definitions.join("\n\n");
        let borrowed = original.contains("values:&");
        let floating =
            case["surface"]["items"][0]["params"][0]["type"]["element"]["kind"] == "float";
        for (variant, definition) in [
            ("original", original),
            ("generated", generated),
            ("replayed", replayed.as_str()),
        ] {
            let calls = case["inputs"]
                .as_array()
                .unwrap()
                .iter()
                .map(|input| {
                    let values = input
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|value| {
                            if let Some(text) = value.as_str() {
                                format!("String::from({text:?})")
                            } else if floating {
                                if value.as_i64().is_some() {
                                    format!("{value}.0f64")
                                } else {
                                    format!("{value}f64")
                                }
                            } else {
                                value.to_string()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(",");
                    let borrow = if variant == "original" && borrowed {
                        "&"
                    } else {
                        ""
                    };
                    format!(
                        "println!(\"{{}}\",{}({borrow}vec![{values}]));",
                        case["name"].as_str().unwrap()
                    )
                })
                .collect::<String>();
            let source = directory.0.join(format!("array_{case_index}_{variant}.rs"));
            let binary = directory.0.join(format!(
                "array_{case_index}_{variant}{}",
                std::env::consts::EXE_SUFFIX
            ));
            fs::write(&source, format!("{definition}\nfn main(){{{calls}}}")).unwrap();
            let mut compiler = Command::new("rustc");
            compiler.args(["--edition", "2024"]);
            #[cfg(windows)]
            if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
                compiler.arg("-C").arg(format!("linker={linker}"));
            }
            let compiled = run(compiler.arg(&source).arg("-o").arg(&binary));
            assert!(compiled.ok, "{case_index}: {variant}: {}", compiled.stderr);
            let observed = run(&mut Command::new(binary));
            assert!(observed.ok, "{case_index}: {variant}: {}", observed.stderr);
            assert_eq!(
                observed.stdout.lines().collect::<Vec<_>>(),
                case["expected"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(Value::to_string)
                    .collect::<Vec<_>>(),
                "{case_index}: {variant}"
            );
        }
    }
}
