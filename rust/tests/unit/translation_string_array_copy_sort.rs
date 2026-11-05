//! Future whole-program controls; native compilation belongs exclusively to CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::{
    check::check_program, emit_rust::emit_rust, javascript::parse_javascript,
};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, emit_typed_program_links,
    encode_link_network_binary, encode_typed_program_links, read_structured_data_lino,
};
use std::{fs, process::Command};
fn emission(value: meta_language::TypedProgramLinksEmission) -> serde_json::Value {
    serde_json::json!({"code":value.code,"language":value.language,"mappings":value.mappings,"assumptions":value.assumptions,"encodings":value.encodings,"theorems":value.theorems})
}

pub(super) fn require_rust_profile() {
    let compiler = run(Command::new("rustc").arg("--version"));
    assert!(compiler.ok, "{}", compiler.stderr);
    let version = compiler
        .stdout
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split('.')
        .collect::<Vec<_>>();
    assert!(
        version[0] == "1" && (90..100).contains(&version[1].parse::<u32>().unwrap()),
        "uncovered Rust {}",
        compiler.stdout
    );
}
#[test]
fn dense_copy_sort_keeps_original_node_units_both_frontends_codecs_and_native_execution() {
    require_rust_profile();
    let produced = run(Command::new("node").args([
        "--max-old-space-size=352",
        "../experiments/javascript-program/string-array-copy-sort.mjs",
    ]));
    assert!(produced.ok, "{}", produced.stderr);
    let product = read_structured_data_lino(&produced.stdout).unwrap();
    let program =
        check_program(&parse_javascript(product["source"].as_str().unwrap()).unwrap()).unwrap();
    let packet = encode_typed_program_links(&program).unwrap();
    assert_eq!(packet.network.to_lino(), product["links"].as_str().unwrap());
    assert_eq!(packet.root.as_u64(), product["root"].as_u64().unwrap());
    let binary =
        encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
    ] {
        for target in ["JavaScript", "Rust"] {
            let emitted = emit_typed_program_links(&network, packet.root, target).unwrap();
            assert_eq!(emission(emitted), product["emissions"][target]);
        }
        for target in ["Lean", "Rocq"] {
            let error = emit_typed_program_links(&network, packet.root, target).unwrap_err();
            assert!(
                error
                    .message
                    .contains(product["formalRefusalReason"].as_str().unwrap()),
                "{}",
                error.message
            );
        }
    }
    for row in product["refusals"].as_array().unwrap() {
        let error = parse_javascript(row["source"].as_str().unwrap())
            .and_then(|surface| check_program(&surface))
            .unwrap_err();
        assert_eq!(error.kind.as_str(), "unsupported");
        assert_eq!(error.reason, product["refusalReason"].as_str().unwrap());
    }
    let mut forged = program.clone();
    let meta_language::translation::ir::Decl::Fn(function) = forged
        .declarations
        .iter_mut()
        .find(|declaration| declaration.name() == "sortStrings")
        .unwrap()
    else {
        panic!("the actual source function is retained")
    };
    function.body.ty = meta_language::translation::types::Type::Bool;
    assert!(encode_typed_program_links(&forged).is_err());
    let emitted = emit_rust(&program).unwrap();
    let header = emitted
        .text
        .lines()
        .find(|line| line.starts_with("#![allow("))
        .unwrap();
    let library = std::iter::once(header.to_owned())
        .chain(emitted.preludes)
        .chain(emitted.definitions)
        .collect::<Vec<_>>()
        .join("\n\n");
    assert_eq!(library, product["nativeLibrary"].as_str().unwrap());
    let target = product["nativeTarget"].as_str().unwrap().replace('.', "::");
    let mut main = "\nfn main(){\n".to_owned();
    for row in product["cases"].as_array().unwrap() {
        let values = |key: &str| {
            row[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|units| {
                    format!(
                        "crate::ml_string::Text::from_units(vec![{}])",
                        units
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|unit| unit.as_u64().unwrap().to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                })
                .collect::<Vec<_>>()
                .join(",")
        };
        let input = values("units");
        let expected = values("expected");
        main.push_str(&format!("{{let input=vec![{input}];let result=crate::{target}(input.clone());assert_eq!(result,vec![{expected}]);assert_eq!(input,vec![{input}]);}}\n"));
    }
    main.push_str("}\n");
    let temporary = TemporaryDirectory::new("string-copy-sort");
    let source = temporary.0.join("main.rs");
    let binary = temporary
        .0
        .join(format!("sort{}", std::env::consts::EXE_SUFFIX));
    fs::write(&source, library + &main).unwrap();
    let compiled = run(Command::new("rustc")
        .arg("--edition=2024")
        .arg(&source)
        .arg("-o")
        .arg(&binary));
    assert!(compiled.ok, "{}", compiled.stderr);
    let executed = run(&mut Command::new(&binary));
    assert!(executed.ok, "{}", executed.stderr);
}
