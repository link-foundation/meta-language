//! Exact primitive binary64 operations and independent native bit oracles.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::rust::parse_rust;
use meta_language::{
    LinkNetwork, create_typed_program_links, decode_typed_program_links, emit_typed_program_links,
};
use serde_json::Value;

use super::issue_195_translation_pairs::{TemporaryDirectory, run};

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-float-operations.lino"
    ))
    .expect("shared floating operation fixtures are inspectable Links data")
}

#[test]
fn primitive_operations_match_surface_source_free_links_and_actual_definition_reimports() {
    for case in fixture()["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let surface = parse_rust(case["source"].as_str().unwrap()).unwrap();
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
        let reverse = check_program(
            &parse_javascript(case["javascriptDefinitions"].as_str().unwrap()).unwrap(),
        )
        .unwrap();
        assert_eq!(
            emit_rust(&reverse).unwrap().definitions.join("\n\n"),
            definitions,
            "{id}"
        );
        let archive = create_typed_program_links(case["source"].as_str().unwrap(), "Rust").unwrap();
        let network = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
        assert!(
            network
                .links()
                .all(|link| !link.metadata().definition().is_some_and(
                    |definition| definition.contains(case["source"].as_str().unwrap())
                )),
            "{id}"
        );
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
fn unproved_source_receivers_and_separate_numeric_models_keep_exact_refusals() {
    for case in fixture()["invalid"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let error = parse_rust(case["source"].as_str().unwrap())
            .and_then(|surface| check_program(&surface))
            .expect_err("actual primitive source evidence is required");
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

// Batch all bounded definitions into one compilation per direction. Native
// execution runs in CI; local work uses the independent JavaScript bit oracle.
#[test]
fn original_emitted_reparsed_and_reverse_rust_preserve_exact_bits_and_receiver_counts() {
    let data = fixture();
    let cases = data["cases"].as_array().unwrap();
    let expected = cases
        .iter()
        .flat_map(|case| {
            case["observations"]
                .as_array()
                .unwrap()
                .iter()
                .map(|row| row["expectedBits"].as_str().unwrap().to_owned())
        })
        .collect::<Vec<_>>();
    let directory = TemporaryDirectory::new("rust-float-operations");
    for variant in ["original", "emitted", "reparsed", "reverse"] {
        let mut modules = Vec::new();
        for (index, case) in cases.iter().enumerate() {
            let mut definitions = match variant {
                "original" => case["source"].as_str().unwrap().to_owned(),
                "emitted" => case["rustDefinitions"].as_str().unwrap().to_owned(),
                "reparsed" => {
                    let program = check_program(
                        &parse_rust(&format!(
                            "{}\nfn main(){{}}",
                            case["rustDefinitions"].as_str().unwrap()
                        ))
                        .unwrap(),
                    )
                    .unwrap();
                    emit_rust(&program).unwrap().definitions.join("\n\n")
                }
                "reverse" => {
                    let program = check_program(
                        &parse_javascript(case["originalJavaScript"].as_str().unwrap()).unwrap(),
                    )
                    .unwrap();
                    emit_rust(&program).unwrap().definitions.join("\n\n")
                }
                _ => unreachable!(),
            };
            let namespace = format!("ml_case_{index}");
            if variant != "original" && definitions.contains("crate::") {
                let helper = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../experiments/issue-199-generated-rust-namespace.mjs");
                let adapter = "import{pathToFileURL}from'node:url';const{qualifyGeneratedRustRootPaths}=await import(pathToFileURL(process.argv[1]));process.stdout.write(qualifyGeneratedRustRootPaths(process.argv[2],process.argv[3]));";
                let qualified = run(Command::new("node")
                    .args(["--input-type=module", "--eval", adapter])
                    .arg(helper)
                    .arg(&definitions)
                    .arg(&namespace));
                assert!(qualified.ok, "{index}: {variant}: {}", qualified.stderr);
                definitions = qualified.stdout;
            }
            let mut calls = String::new();
            for row in case["observations"].as_array().unwrap() {
                let field = if variant == "original" {
                    "originalRustArguments"
                } else {
                    "emittedRustArguments"
                };
                let observe = if case["surface"]["items"][0]["ret"]["kind"] == "bool" {
                    "observe_bool"
                } else {
                    "observe"
                };
                write!(
                    calls,
                    "crate::{observe}(apply({}));",
                    row[field].as_str().unwrap()
                )
                .expect("formatting a caller into a String succeeds");
            }
            let (instrumented, count) = if let Some(expected_calls) = case["receiverCalls"].as_u64()
            {
                let header = if variant == "original" {
                    "fn provider()->f64{"
                } else {
                    "fn provider() -> f64 {"
                };
                assert_eq!(definitions.matches(header).count(), 1);
                let replaced = definitions.replace(
                    header,
                    &format!(
                        "{header}RECEIVER_CALLS.fetch_add(1,std::sync::atomic::Ordering::SeqCst);"
                    ),
                );
                let source = format!(
                    "static RECEIVER_CALLS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);\n{replaced}"
                );
                (
                    source,
                    format!(
                        "assert_eq!(RECEIVER_CALLS.load(std::sync::atomic::Ordering::SeqCst),{expected_calls});"
                    ),
                )
            } else {
                (definitions, String::new())
            };
            modules.push(format!(
                "mod {namespace}{{{instrumented}\npub fn run(){{{calls}{count}}}}}"
            ));
        }
        let mut calls = String::new();
        for index in 0..cases.len() {
            write!(calls, "ml_case_{index}::run();")
                .expect("formatting a caller into a String succeeds");
        }
        let code = format!(
            "fn observe(value:f64){{if value.is_nan(){{println!(\"NaN\");}}else{{println!(\"{{:016x}}\",value.to_bits());}}}}fn observe_bool(value:bool){{println!(\"{{}}\",value);}}\n{}\nfn main(){{{calls}}}",
            modules.join("\n")
        );
        assert_eq!(
            run_rust(&code, &directory.0, variant),
            expected,
            "{variant}"
        );
    }
    let tag = "function observe(value){if(typeof value==='boolean'){console.log(String(value));return;}if(Number.isNaN(value)){console.log('NaN');return;}const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value,false);console.log(view.getBigUint64(0,false).toString(16).padStart(16,'0'));}";
    for case in cases {
        let expected = case["observations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row["expectedBits"].as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let mut calls = String::new();
        for row in case["observations"].as_array().unwrap() {
            let arguments = row["javascriptArguments"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| {
                    value.as_array().map_or_else(
                        || value.as_str().unwrap().to_owned(),
                        |array| {
                            format!(
                                "[{}]",
                                array
                                    .iter()
                                    .map(|value| value.as_str().unwrap())
                                    .collect::<Vec<_>>()
                                    .join(",")
                            )
                        },
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            write!(calls, "observe(apply({arguments}));")
                .expect("formatting a caller into a String succeeds");
        }
        assert_eq!(
            run_javascript(&format!(
                "{}\n{tag}\n{calls}",
                case["originalJavaScript"].as_str().unwrap()
            )),
            expected
        );
        let archive = create_typed_program_links(case["source"].as_str().unwrap(), "Rust").unwrap();
        let network = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
        let emitted = emit_typed_program_links(&network, archive.root, "JavaScript").unwrap();
        assert_eq!(
            run_javascript(&format!("{}\n{tag}\n{calls}", emitted.code)),
            expected
        );
    }
}

#[test]
fn source_free_operation_edit_changes_the_independent_native_bit_observation() {
    let data = fixture();
    let edit = &data["edit"];
    let case = data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == edit["case"])
        .unwrap();
    let archive = create_typed_program_links(case["source"].as_str().unwrap(), "Rust").unwrap();
    let network = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
    let from = edit["from"].as_str().unwrap();
    let to = edit["to"].as_str().unwrap();
    let operation = network
        .links()
        .find(|link| link.metadata().definition() == Some(from))
        .unwrap()
        .id();
    let text = network
        .to_lino()
        .lines()
        .map(|line| {
            if line.starts_with(&format!("({operation}: ")) {
                line.replace(&format!("(def: {from})"), &format!("(def: {to})"))
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let changed = LinkNetwork::from_lino(&text).unwrap();
    let program = decode_typed_program_links(&changed, archive.root).unwrap();
    let definitions = emit_rust(&program).unwrap().definitions.join("\n\n");
    assert!(definitions.contains(".ceil()"));
    let source = format!(
        "{definitions}\nfn main(){{println!(\"{{:016x}}\",apply({}f64).to_bits());}}",
        edit["input"].as_str().unwrap()
    );
    let directory = TemporaryDirectory::new("rust-float-operation-edit");
    assert_eq!(
        run_rust(&source, &directory.0, "edited"),
        [edit["expectedBits"].as_str().unwrap()]
    );
}
