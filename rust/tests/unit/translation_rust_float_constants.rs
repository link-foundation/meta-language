//! Exact primitive associated constants, shared links, and independent bit oracles.

use std::fmt::Write as _;
use std::fs;
use std::process::Command;

use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::rust::parse_rust;
use meta_language::{LinkId, LinkNetwork, create_typed_program_links, emit_typed_program_links};
use serde_json::Value;

use super::issue_195_translation_pairs::{TemporaryDirectory, run};

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-float-constants.lino"
    ))
    .expect("shared constant fixtures are inspectable Links data")
}

#[test]
fn primitive_constants_match_shared_surface_links_and_actual_definition_reimports() {
    for case in fixture()["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let source = case["source"].as_str().unwrap();
        let surface = parse_rust(source).unwrap();
        assert_eq!(
            serde_json::to_value(&surface).unwrap(),
            case["surface"],
            "{id}"
        );
        let program = check_program(&surface).unwrap();
        assert_eq!(
            emit_javascript(&program).unwrap().text,
            case["javascript"],
            "{id}"
        );
        let definitions = emit_rust(&program).unwrap().definitions.join("\n\n");
        assert_eq!(definitions, case["rustDefinitions"], "{id}");
        let replay =
            check_program(&parse_rust(&format!("{definitions}\nfn main(){{}}")).unwrap()).unwrap();
        assert_eq!(
            emit_rust(&replay).unwrap().definitions.join("\n\n"),
            definitions,
            "{id}"
        );
        assert_eq!(
            emit_javascript(&replay).unwrap().text,
            case["replayJavaScript"],
            "{id}"
        );
        let reverse = check_program(
            &parse_javascript(case["javascriptDefinitions"].as_str().unwrap()).unwrap(),
        )
        .unwrap();
        let returned = emit_rust(&reverse).unwrap().definitions.join("\n\n");
        assert_eq!(returned, case["reverseDefinitions"], "{id}");
        let returned =
            check_program(&parse_rust(&format!("{returned}\nfn main(){{}}")).unwrap()).unwrap();
        assert_eq!(
            emit_javascript(&returned).unwrap().text,
            case["reverseReplayJavaScript"],
            "{id}"
        );
        let archive = create_typed_program_links(source, "Rust").unwrap();
        let native = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
        let shared = LinkNetwork::from_lino(case["archive"].as_str().unwrap()).unwrap();
        for (network, root) in [
            (&native, archive.root),
            (&shared, LinkId::from_u64(case["root"].as_u64().unwrap())),
        ] {
            assert!(
                network.links().all(|link| !link
                    .metadata()
                    .definition()
                    .is_some_and(|definition| definition.contains(source))),
                "{id}"
            );
            for (target, field) in [("JavaScript", "javascript"), ("Rust", "rust")] {
                assert_eq!(
                    emit_typed_program_links(network, root, target)
                        .unwrap()
                        .code,
                    case[field],
                    "{id}: {target}"
                );
            }
        }
    }
}

#[test]
fn nonprimitive_shadowed_qualified_and_called_constants_keep_exact_refusals() {
    for case in fixture()["invalid"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let error = parse_rust(case["source"].as_str().unwrap())
            .and_then(|surface| check_program(&surface))
            .expect_err("an exact unshadowed primitive owner is required");
        assert_eq!(error.kind.as_str(), case["kind"].as_str().unwrap(), "{id}");
        assert_eq!(error.reason, case["reason"].as_str().unwrap(), "{id}");
        assert_eq!(
            serde_json::to_value(error.span).unwrap(),
            case["span"],
            "{id}"
        );
    }
}

#[test]
fn source_free_constant_edit_changes_native_emission_and_independent_sign_bits() {
    let data = fixture();
    let case = data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == data["edit"]["case"])
        .unwrap();
    let archive = case["archive"].as_str().unwrap();
    let changed = archive.replace("(def: Infinity)", "(def: -Infinity)");
    assert_ne!(changed, archive);
    let network = LinkNetwork::from_lino(&changed).unwrap();
    let root = LinkId::from_u64(case["root"].as_u64().unwrap());
    let javascript = emit_typed_program_links(&network, root, "JavaScript").unwrap();
    let code = format!(
        "{}\nconst view=new DataView(new ArrayBuffer(8));view.setFloat64(0,apply(),false);console.log(view.getBigUint64(0,false).toString(16));",
        javascript.code
    );
    let observed = run(Command::new("node").args(["--input-type=module", "--eval", &code]));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(
        observed.stdout.trim(),
        data["edit"]["expectedBits"].as_str().unwrap()
    );
    assert!(
        emit_typed_program_links(&network, root, "Rust")
            .unwrap()
            .code
            .contains("f64::NEG_INFINITY")
    );
}

#[test]
fn original_emitted_reparsed_and_reverse_definitions_match_native_bit_oracles() {
    let data = fixture();
    let directory = TemporaryDirectory::new("rust-float-constants");
    let expected = data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| {
            format!(
                "{} {} {}",
                case["expectedBits"].as_str().unwrap(),
                case["expectedNaN"].as_bool().unwrap(),
                case["expectedAbsoluteBits"].as_str().unwrap()
            )
        })
        .collect::<Vec<_>>();
    for (variant, field) in [
        "source",
        "rustDefinitions",
        "reparsed",
        "reverseDefinitions",
    ]
    .iter()
    .enumerate()
    {
        let mut code = String::new();
        let mut calls = String::new();
        for (index, case) in data["cases"].as_array().unwrap().iter().enumerate() {
            let mut definitions = if *field == "reparsed" {
                let surface = parse_rust(&format!(
                    "{}\nfn main(){{}}",
                    case["rustDefinitions"].as_str().unwrap()
                ))
                .unwrap();
                emit_rust(&check_program(&surface).unwrap())
                    .unwrap()
                    .definitions
                    .join("\n\n")
            } else {
                case[*field].as_str().unwrap().replace("fn main(){}", "")
            };
            // Original functions are private; expose their unchanged bodies
            // within this isolated oracle namespace. Emitted functions are public.
            if *field == "source" {
                definitions = definitions
                    .replace("fn apply(", "pub fn apply(")
                    .replace("fn classify(", "pub fn classify(")
                    .replace("fn absolute(", "pub fn absolute(");
            }
            writeln!(code, "mod case_{index} {{ {definitions} }}").unwrap();
            writeln!(calls, "println!(\"{{}} {{}} {{}}\",observe(case_{index}::apply()),case_{index}::classify(),observe(case_{index}::absolute()));").unwrap();
        }
        writeln!(code, "fn observe(value:f64)->String{{if value.is_nan(){{\"NaN\".to_owned()}}else{{format!(\"{{:016x}}\",value.to_bits())}}}}fn main(){{{calls}}}").unwrap();
        let source = directory.0.join(format!("variant_{variant}.rs"));
        let binary = directory
            .0
            .join(format!("variant_{variant}{}", std::env::consts::EXE_SUFFIX));
        fs::write(&source, code).unwrap();
        let mut compiler = Command::new("rustc");
        compiler.args(["--edition", "2024"]);
        #[cfg(windows)]
        if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
            compiler.arg("-C").arg(format!("linker={linker}"));
        }
        let compiled = run(compiler.arg(&source).arg("-o").arg(&binary));
        assert!(compiled.ok, "{field}: {}", compiled.stderr);
        let executed = run(&mut Command::new(binary));
        assert!(executed.ok, "{field}: {}", executed.stderr);
        assert_eq!(
            executed
                .stdout
                .lines()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            expected,
            "{field}"
        );
    }
}
