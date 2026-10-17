//! Boolean const decision trees use the shared linked policy; execution is CI-only.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::{Decl, Expr, Node, Program};
use meta_language::translation::rust::{parse_rust, parse_rust_with_const_function_model};
use meta_language::translation::surface::UnaryOp;
use meta_language::translation::types::Type;
use meta_language::{
    BinaryLinksOptions, LinkId, LinkMetadata, LinkNetwork, LinkType, SubstitutionRule,
    TypedProgramLinks, create_typed_program_links_with_rust_const_function_model,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary, encode_typed_program_links, read_structured_data_lino,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::fs;
use std::process::Command;
use std::sync::OnceLock;

fn fixture() -> &'static Value {
    static FIXTURE: OnceLock<Value> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        read_structured_data_lino(include_str!(
            "../../../experiments/rust-const-branch-fixture.lino"
        ))
        .unwrap()
    })
}

fn model() -> &'static str {
    fixture()["model"].as_str().unwrap()
}

fn packet(entry: &Value) -> TypedProgramLinks {
    TypedProgramLinks {
        network: LinkNetwork::from_lino(entry["links"].as_str().unwrap()).unwrap(),
        root: LinkId::from_u64(entry["root"].as_u64().unwrap()),
    }
}

fn program(entry: &Value) -> Program {
    let packet = packet(entry);
    decode_typed_program_links(&packet.network, packet.root).unwrap()
}

fn selected(source: &str) -> Program {
    check_program(&parse_rust_with_const_function_model(source, model(), "").unwrap()).unwrap()
}

fn archives(packet: &TypedProgramLinks) -> [LinkNetwork; 2] {
    [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ]
}

fn observe_javascript(code: &str, entry: &Value, expected: &Value) {
    let script = format!(
        "const assert=require('node:assert/strict'),vm=require('node:vm'),context={{}};vm.runInNewContext({},context,{{timeout:1000}});const fn=context[{}],inputs={},expected={};assert.deepEqual(inputs.map(args=>fn(...args)),expected);for(let position=0;position<inputs[0].length;position++)for(const value of[undefined,null,0,1,'false',{{}},new Boolean(false)]){{const args=[...inputs[0]];args[position]=value;assert.throws(()=>fn(...args),/requires boolean/u);}}",
        json!(code),
        entry["name"],
        entry["inputs"],
        expected
    );
    let result = run(Command::new("node").args(["-e", &script]));
    assert!(result.ok, "{}", result.stderr);
}

fn observe_javascript_archive(packet: &TypedProgramLinks) -> String {
    let directory = TemporaryDirectory::new("const-branch-native-archive");
    let archive = directory.0.join("program.lino");
    fs::write(&archive, packet.network.to_lino()).unwrap();
    let index = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../js/src/index.js");
    let script = format!(
        "import {{pathToFileURL}} from 'node:url';import {{readFileSync}} from 'node:fs';const {{LinkNetwork,decodeTypedProgramLinks,emitTypedProgramLinks}}=await import(pathToFileURL({}));const network=LinkNetwork.fromLino(readFileSync({},'utf8'));decodeTypedProgramLinks(network,{});emitTypedProgramLinks(network,{},'Rust');process.stdout.write(emitTypedProgramLinks(network,{},'JavaScript').code);",
        json!(index.to_string_lossy()),
        json!(archive.to_string_lossy()),
        packet.root.as_u64(),
        packet.root.as_u64(),
        packet.root.as_u64()
    );
    let path = directory.0.join("observe.mjs");
    fs::write(&path, script).unwrap();
    let result = run(Command::new("node").arg(path));
    assert!(result.ok, "{}", result.stderr);
    result.stdout
}

#[test]
fn four_actual_qualified_trees_preserve_every_truth_row_and_owned_archive_field() {
    let entries = fixture()["cases"].as_array().unwrap();
    assert_eq!(entries.len(), 4);
    for entry in entries {
        let source = entry["source"].as_str().unwrap();
        assert_eq!(
            Sha256::digest(source.as_bytes()).iter().fold(
                String::new(),
                |mut hexadecimal, byte| {
                    write!(hexadecimal, "{byte:02x}").expect("writing into a String succeeds");
                    hexadecimal
                },
            ),
            entry["sourceSha256"].as_str().unwrap()
        );
        let actual = program(entry);
        assert_eq!(selected(source), actual);
        assert!(parse_rust(source).is_err());
        let Decl::Fn(function) = &actual.declarations[0] else {
            unreachable!()
        };
        assert!(matches!(function.body.node, Node::If { .. }));
        assert_eq!(
            function.source_const_function.as_ref().unwrap(),
            &entry["proof"]
        );
        assert_eq!(
            serde_json::to_value(function.source_signature.as_ref().unwrap()).unwrap(),
            entry["signature"]
        );
        let emitted = emit_rust(&actual).unwrap();
        assert_eq!(
            emitted.definitions[0],
            entry["rustDefinition"].as_str().unwrap()
        );
        assert!(emitted.definitions[0].starts_with("pub const fn "));
        let native = create_typed_program_links_with_rust_const_function_model(
            source,
            "Rust",
            model(),
            None,
            Some(&json!({"fixture":entry["name"]})),
        )
        .unwrap();
        for network in archives(&native) {
            assert_eq!(
                decode_typed_program_links(&network, native.root).unwrap(),
                actual
            );
            let emitted = emit_typed_program_links(&network, native.root, "JavaScript").unwrap();
            assert_eq!(emitted.code, entry["javascript"].as_str().unwrap());
            observe_javascript(&emitted.code, entry, &entry["expected"]);
        }
        observe_javascript(
            &observe_javascript_archive(&native),
            entry,
            &entry["expected"],
        );
        let mut definitions = emitted.definitions;
        for _ in 0..3 {
            let replay = selected(&(definitions.join("\n\n") + "\nfn main(){}"));
            observe_javascript(
                &emit_javascript(&replay).unwrap().text,
                entry,
                &entry["expected"],
            );
            let emitted = emit_rust(&replay).unwrap();
            assert_eq!(emitted.definitions, definitions);
            assert!(parse_rust_with_const_function_model(&emitted.text, model(), "").is_err());
            definitions = emitted.definitions;
        }
    }
}

#[test]
fn all_seventeen_refusals_and_the_original_conditional_boundary_remain_exact() {
    let entries = fixture()["invalid"].as_array().unwrap();
    assert_eq!(entries.len(), 17);
    for entry in entries {
        let failure =
            parse_rust_with_const_function_model(entry["source"].as_str().unwrap(), model(), "")
                .and_then(|surface| check_program(&surface))
                .unwrap_err();
        assert_eq!(
            failure.kind.as_str(),
            entry["kind"].as_str().unwrap(),
            "{}",
            entry["name"]
        );
        assert_eq!(
            failure.reason,
            entry["reason"].as_str().unwrap(),
            "{}",
            entry["name"]
        );
        assert_eq!(
            serde_json::to_value(failure.span).unwrap(),
            entry["span"],
            "{}",
            entry["name"]
        );
    }
    let original = read_structured_data_lino(include_str!(
        "../../../experiments/rust-const-function-fixture.lino"
    ))
    .unwrap();
    assert_eq!(original["cases"].as_array().unwrap().len(), 3);
    assert_eq!(original["invalid"].as_array().unwrap().len(), 27);
    let conditional = original["invalid"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["name"] == "conditional")
        .unwrap();
    let source = conditional["source"].as_str().unwrap();
    let failure =
        parse_rust_with_const_function_model(source, original["model"].as_str().unwrap(), "")
            .and_then(|surface| check_program(&surface))
            .unwrap_err();
    assert_eq!(failure.reason, conditional["reason"].as_str().unwrap());
    assert_eq!(
        serde_json::to_value(failure.span).unwrap(),
        conditional["span"]
    );
    assert!(matches!(selected(source).declarations[0], Decl::Fn(_)));
}

#[test]
fn source_free_condition_and_branch_edits_change_results_without_replaying_source() {
    let entry = &fixture()["cases"][0];
    for (edit, expected) in [
        (0, json!([false, true, true, false])),
        (1, json!([true, false, true, false])),
    ] {
        let mut actual = program(entry);
        let Decl::Fn(function) = &mut actual.declarations[0] else {
            unreachable!()
        };
        let proof = function.source_const_function.clone();
        let signature = function.source_signature.clone();
        let Node::If { cond, then, .. } = &mut function.body.node else {
            unreachable!()
        };
        let child = if edit == 0 { cond } else { then };
        **child = Expr::new(
            Node::Unary {
                op: UnaryOp::Not,
                arg: child.clone(),
                semantics: None,
            },
            Type::Bool,
        );
        let encoded = encode_typed_program_links(&actual).unwrap();
        for network in archives(&encoded) {
            let restored = decode_typed_program_links(&network, encoded.root).unwrap();
            let Decl::Fn(function) = &restored.declarations[0] else {
                unreachable!()
            };
            assert_eq!(function.source_const_function, proof);
            assert_eq!(function.source_signature, signature);
            observe_javascript(&emit_javascript(&restored).unwrap().text, entry, &expected);
            let emitted = emit_rust(&restored).unwrap();
            assert!(emitted.definitions[0].starts_with("pub const fn "));
            assert_ne!(
                emitted.definitions[0],
                entry["rustDefinition"].as_str().unwrap()
            );
            let replay = selected(&(emitted.definitions.join("\n\n") + "\nfn main(){}"));
            observe_javascript(&emit_javascript(&replay).unwrap().text, entry, &expected);
        }
        observe_javascript(&observe_javascript_archive(&encoded), entry, &expected);
    }
}

fn relation(network: &LinkNetwork, parent: LinkId, label: &str) -> [LinkId; 3] {
    let label = meta_language::typed_program_links::label_of(label);
    network
        .links()
        .find_map(|link| match link.references() {
            [owner, field, child]
                if *owner == parent
                    && network
                        .link(*field)
                        .and_then(|field| field.metadata().term())
                        == Some(label.as_str()) =>
            {
                Some([*owner, *field, *child])
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("missing {label} relation for {parent:?}"))
}

fn replace(network: &mut LinkNetwork, parent: LinkId, label: &str, term: &str, value: &str) {
    let old = relation(network, parent, label);
    let child = network.insert_link(
        [],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_language("meta-language typed program")
            .with_named(true)
            .with_term(term)
            .with_definition(value),
    );
    assert_eq!(
        network
            .apply_substitution(&SubstitutionRule::new(old, [old[0], old[1], child]))
            .updated()
            .len(),
        1
    );
}

#[test]
fn raw_unknown_models_kinds_domains_and_unrepresented_branches_refuse_both_emitters() {
    for edit in 0..6 {
        let mut archive = packet(&fixture()["cases"][0]);
        let body = archive.network.link(archive.root).unwrap().references()[0];
        let directory = relation(&archive.network, body, "declarations")[2];
        let pair = archive.network.link(directory).unwrap().references()[0];
        let function = archive.network.link(pair).unwrap().references()[1];
        let proof = relation(&archive.network, function, "sourceConstFunction")[2];
        let expression = relation(&archive.network, function, "body")[2];
        match edit {
            0 => replace(
                &mut archive.network,
                proof,
                "model",
                "string value",
                "unknown",
            ),
            1 => replace(
                &mut archive.network,
                expression,
                "k",
                "string value",
                "call",
            ),
            2 => {
                let condition = relation(&archive.network, expression, "cond")[2];
                let ty = relation(&archive.network, condition, "type")[2];
                replace(&mut archive.network, ty, "kind", "string value", "float");
            }
            3 => {
                let interpretation = relation(&archive.network, proof, "interpretation")[2];
                replace(
                    &mut archive.network,
                    interpretation,
                    "inputDomain",
                    "string value",
                    "primitive Boolean and binary64 values",
                );
            }
            4 => {
                let alternative = relation(&archive.network, expression, "else")[2];
                let old = relation(&archive.network, alternative, "k");
                archive
                    .network
                    .insert_field(alternative, "borrowEscape", old[2]);
            }
            _ => {
                let condition = relation(&archive.network, expression, "cond")[2];
                replace(
                    &mut archive.network,
                    condition,
                    "name",
                    "string value",
                    "foreign",
                );
            }
        }
        for network in archives(&archive) {
            for target in ["Rust", "JavaScript"] {
                assert!(
                    emit_typed_program_links(&network, archive.root, target).is_err(),
                    "{edit}/{target}"
                );
            }
        }
    }
}

fn rust_arguments(row: &Value) -> String {
    row.as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_bool().unwrap().to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[test]
fn actual_original_and_whole_emitted_rust_execute_all_truth_rows_and_const_invocations() {
    let directory = TemporaryDirectory::new("const-branch-full-artifacts");
    let mut source = String::new();
    let mut calls = String::new();
    for (index, entry) in fixture()["cases"].as_array().unwrap().iter().enumerate() {
        let emitted = emit_rust(&program(entry)).unwrap();
        let ordinary = format!("{}\nfn main(){{}}", entry["ordinary"].as_str().unwrap());
        for (phase, artifact) in [
            ordinary.as_str(),
            entry["source"].as_str().unwrap(),
            emitted.text.as_str(),
        ]
        .into_iter()
        .enumerate()
        {
            writeln!(
                source,
                "mod observed_{index}_{phase}{{\n{artifact}\npub fn observe(){{main();"
            )
            .unwrap();
            let name = entry["name"].as_str().unwrap();
            for (row, expected) in entry["inputs"]
                .as_array()
                .unwrap()
                .iter()
                .zip(entry["expected"].as_array().unwrap())
            {
                writeln!(
                    source,
                    "assert_eq!({name}({}),{});",
                    rust_arguments(row),
                    expected.as_bool().unwrap()
                )
                .unwrap();
            }
            if phase != 0 {
                writeln!(
                    source,
                    "const OBSERVED:bool={name}({});assert_eq!(OBSERVED,{});",
                    rust_arguments(&entry["inputs"][0]),
                    entry["expected"][0].as_bool().unwrap()
                )
                .unwrap();
            }
            writeln!(source, "}}\n}}").unwrap();
            writeln!(calls, "observed_{index}_{phase}::observe();").unwrap();
        }
    }
    writeln!(source, "fn main(){{{calls}}}").unwrap();
    let input = directory.0.join("observed.rs");
    let binary = directory
        .0
        .join(format!("observed{}", std::env::consts::EXE_SUFFIX));
    fs::write(&input, source).unwrap();
    let mut build = Command::new("rustc");
    build.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        build.arg("-C").arg(format!("linker={linker}"));
    }
    let compiled = run(build.arg(input).arg("-o").arg(&binary));
    assert!(compiled.ok, "{}", compiled.stderr);
    let observed = run(&mut Command::new(binary));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(observed.stdout, "");
}
