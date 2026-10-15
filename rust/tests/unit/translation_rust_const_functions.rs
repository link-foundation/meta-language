//! Actual styled const predicates and source-free archives; native tools run in CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::{Decl, LitValue, Node, Program};
use meta_language::translation::rust::{parse_rust, parse_rust_with_const_function_model};
use meta_language::translation::surface::BinaryOp;
use meta_language::{
    BinaryLinksOptions, LinkId, LinkMetadata, LinkNetwork, LinkType, SubstitutionRule,
    TypedProgramLinks, create_typed_program_links_with_rust_const_function_model,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary, encode_typed_program_links, read_structured_data_lino,
};
use serde_json::{Value, json};
use std::fmt::Write as _;
use std::fs;
use std::process::Command;
use std::sync::OnceLock;

fn fixture() -> &'static Value {
    static FIXTURE: OnceLock<Value> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        read_structured_data_lino(include_str!(
            "../../../experiments/rust-const-function-fixture.lino"
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
    check_program(&parse_rust_with_const_function_model(source, model()).unwrap()).unwrap()
}
fn observe_javascript(code: &str, entry: &Value, edited: bool) {
    let name = entry["name"].as_str().unwrap();
    let oracle = match name {
        "accept_rust_owned_local_record_value" => {
            if edited {
                "for(const a of[false,true])for(const b of[false,true])assert.equal(fn(a,b),!a&&b);"
            } else {
                "for(const a of[false,true])for(const b of[false,true])assert.equal(fn(a,b),!a||b);"
            }
        }
        "accept_rust_owned_local_source_span" => {
            "for(const a of[false,true])for(const b of[false,true])for(const c of[false,true])assert.equal(fn(a,b,c),!a||b&&c);"
        }
        _ => {
            "for(const a of[-Infinity,-1,-0,0,1,Infinity,NaN])for(const b of[-Infinity,-1,-0,0,1,Infinity,NaN])assert.equal(fn(a,b),a<=b);"
        }
    };
    let script = format!(
        "const assert=require('node:assert/strict'),vm=require('node:vm'),context={{}};vm.runInNewContext({},context);const fn=context[{}];{oracle}",
        json!(code),
        json!(name)
    );
    let result = run(Command::new("node").args(["-e", &script]));
    assert!(result.ok, "{}", result.stderr);
}
fn javascript_public_archive(packet: &TypedProgramLinks) -> String {
    let directory = TemporaryDirectory::new("const-predicate-native-archive");
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
    let result = run(Command::new("node").arg(&path));
    assert!(result.ok, "{}", result.stderr);
    result.stdout
}

#[test]
fn three_actual_complete_styled_sources_and_archives_retain_const_qualification() {
    assert_eq!(fixture()["cases"].as_array().unwrap().len(), 3);
    for entry in fixture()["cases"].as_array().unwrap() {
        let actual = selected(entry["source"].as_str().unwrap());
        assert_eq!(actual, program(entry));
        let Decl::Fn(function) = &actual.declarations[0] else {
            unreachable!()
        };
        assert_eq!(
            function.source_const_function.as_ref().unwrap(),
            &entry["proof"]
        );
        assert_eq!(
            serde_json::to_value(function.source_signature.as_ref().unwrap()).unwrap(),
            entry["signature"]
        );
        let original = parse_rust(entry["source"].as_str().unwrap()).unwrap_err();
        assert_eq!(
            original.reason,
            entry["originalRefusal"]["reason"].as_str().unwrap()
        );
        let emitted = emit_rust(&actual).unwrap();
        assert_eq!(
            emitted.definitions[0],
            entry["rustDefinition"].as_str().unwrap()
        );
        assert!(emitted.definitions[0].starts_with("pub const fn "));
        let replay = selected(&format!(
            "{}\nfn main(){{}}",
            emitted.definitions.join("\n\n")
        ));
        assert_eq!(emit_rust(&replay).unwrap().definitions, emitted.definitions);
        assert!(parse_rust_with_const_function_model(&emitted.text, model()).is_err());
        let native = create_typed_program_links_with_rust_const_function_model(
            entry["source"].as_str().unwrap(),
            "rs",
            model(),
            Some(&json!({"fixture":entry["name"]})),
        )
        .unwrap();
        let text = LinkNetwork::from_lino(&native.network.to_lino()).unwrap();
        let binary = decode_link_network_binary(
            &encode_link_network_binary(&native.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap();
        for network in [&text, &binary] {
            assert_eq!(
                decode_typed_program_links(network, native.root).unwrap(),
                actual
            );
            let code = emit_typed_program_links(network, native.root, "JavaScript")
                .unwrap()
                .code;
            assert_eq!(code, entry["javascript"].as_str().unwrap());
            observe_javascript(&code, entry, false);
        }
        let code = javascript_public_archive(&native);
        observe_javascript(&code, entry, false);
        assert_eq!(
            emit_javascript(&actual).unwrap().text,
            entry["javascript"].as_str().unwrap()
        );
    }
    assert!(
        create_typed_program_links_with_rust_const_function_model("", "JavaScript", model(), None)
            .is_err()
    );
    assert!(parse_rust_with_const_function_model("", "unknown").is_err());
}

#[test]
fn all_twenty_seven_source_refusals_keep_their_frozen_diagnostics() {
    let entries = fixture()["invalid"].as_array().unwrap();
    assert_eq!(entries.len(), 27);
    for entry in entries {
        let failure =
            parse_rust_with_const_function_model(entry["source"].as_str().unwrap(), model())
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
}

#[test]
fn source_free_body_edits_change_execution_without_changing_owned_headers() {
    let entry = &fixture()["cases"][0];
    let mut actual = program(entry);
    let Decl::Fn(function) = &mut actual.declarations[0] else {
        unreachable!()
    };
    let proof = function.source_const_function.clone();
    let signature = function.source_signature.clone();
    let Node::Binary { op, .. } = &mut function.body.node else {
        unreachable!()
    };
    assert_eq!(*op, BinaryOp::Or);
    *op = BinaryOp::And;
    let encoded = encode_typed_program_links(&actual).unwrap();
    let restored = decode_typed_program_links(
        &LinkNetwork::from_lino(&encoded.network.to_lino()).unwrap(),
        encoded.root,
    )
    .unwrap();
    let Decl::Fn(function) = &restored.declarations[0] else {
        unreachable!()
    };
    assert_eq!(function.source_const_function, proof);
    assert_eq!(function.source_signature, signature);
    observe_javascript(&emit_javascript(&restored).unwrap().text, entry, true);
    observe_javascript(&javascript_public_archive(&encoded), entry, true);
    let rust = emit_rust(&restored).unwrap();
    assert!(rust.definitions[0].starts_with("pub const fn "));
    assert!(rust.definitions[0].contains("&&"));
    let replay = selected(&(rust.definitions.join("\n\n") + "\nfn main(){}"));
    observe_javascript(&emit_javascript(&replay).unwrap().text, entry, true);
}

fn relation(network: &LinkNetwork, parent: LinkId, label: &str) -> [LinkId; 3] {
    network
        .links()
        .find_map(|link| match link.references() {
            [owner, field, child]
                if *owner == parent
                    && network
                        .link(*field)
                        .and_then(|field| field.metadata().term())
                        == Some(label) =>
            {
                Some([*owner, *field, *child])
            }
            _ => None,
        })
        .unwrap()
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
fn source_free_unknown_qualifications_order_and_intervals_cannot_be_erased() {
    let entry = &fixture()["cases"][0];
    for edit in 0..7 {
        let mut actual = program(entry);
        let Decl::Fn(function) = &mut actual.declarations[0] else {
            unreachable!()
        };
        let proof = function.source_const_function.as_mut().unwrap();
        match edit {
            0 => proof["keyword"] = json!("static"),
            1 => proof["keywordSpan"]["end"] = json!(0),
            2 => proof["interpretation"]["excludedObservations"] = json!([]),
            3 => proof["borrowEscape"] = json!(false),
            4 => function.generated = true,
            5 => proof["keywordSpan"]["start"] = json!(-0.0),
            _ => {
                let record = proof["interpretation"].as_object_mut().unwrap();
                *record = std::mem::take(record).into_iter().rev().collect();
            }
        }
        assert!(encode_typed_program_links(&actual).is_err(), "{edit}");
        assert!(emit_rust(&actual).is_err(), "{edit}");
        assert!(emit_javascript(&actual).is_err(), "{edit}");
    }
    for edit in 0..5 {
        let mut archive = packet(entry);
        let body = archive.network.link(archive.root).unwrap().references()[0];
        let declarations = relation(&archive.network, body, "declarations")[2];
        let pair = archive.network.link(declarations).unwrap().references()[0];
        let function = archive.network.link(pair).unwrap().references()[1];
        let proof = relation(&archive.network, function, "source const function")[2];
        match edit {
            0 => replace(
                &mut archive.network,
                proof,
                "keyword",
                "string value",
                "static",
            ),
            1 => {
                let site = relation(&archive.network, proof, "keyword span")[2];
                replace(&mut archive.network, site, "start", "number value", "-0");
            }
            2 => {
                let interpretation = relation(&archive.network, proof, "interpretation")[2];
                let a = relation(&archive.network, interpretation, "input domain");
                let b = relation(&archive.network, interpretation, "observations");
                let temporary = archive.network.insert_link([], LinkMetadata::new());
                let marker = [a[0], a[1], temporary];
                for (from, to) in [(a, marker), (b, a), (marker, b)] {
                    assert_eq!(
                        archive
                            .network
                            .apply_substitution(&SubstitutionRule::new(from, to))
                            .updated()
                            .len(),
                        1
                    );
                }
            }
            3 => {
                let main = relation(&archive.network, body, "main")[2];
                let original = relation(&archive.network, function, "source const function");
                assert_eq!(
                    archive
                        .network
                        .apply_substitution(&SubstitutionRule::new(
                            original,
                            [main, original[1], proof]
                        ))
                        .updated()
                        .len(),
                    1
                );
            }
            _ => {
                let main = relation(&archive.network, body, "main")[2];
                replace(&mut archive.network, main, "span", "boolean value", "false");
            }
        }
        let network = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
        for target in ["Rust", "JavaScript"] {
            assert!(
                emit_typed_program_links(&network, archive.root, target).is_err(),
                "{edit}/{target}"
            );
        }
    }
}

#[test]
fn primitive_literals_numeric_interval_aliases_and_ordered_multiple_predicates_keep_their_boundaries()
 {
    let source = fixture()["cases"]
        .as_array()
        .unwrap()
        .iter()
        .rev()
        .map(|entry| entry["definition"].as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n\n")
        + "\nfn main(){}";
    let actual = selected(&source);
    assert_eq!(actual.declarations.len(), 3);
    let archive = encode_typed_program_links(&actual).unwrap();
    assert_eq!(
        decode_typed_program_links(&archive.network, archive.root).unwrap(),
        actual
    );
    let code = javascript_public_archive(&archive);
    for entry in fixture()["cases"].as_array().unwrap() {
        observe_javascript(&code, entry, false);
    }
    let mut reversed = archive;
    let body = reversed.network.link(reversed.root).unwrap().references()[0];
    let mapping = relation(&reversed.network, body, "declarations")[2];
    let original: [LinkId; 3] = reversed
        .network
        .link(mapping)
        .unwrap()
        .references()
        .try_into()
        .unwrap();
    let mut changed = original;
    changed.reverse();
    assert_eq!(
        reversed
            .network
            .apply_substitution(&SubstitutionRule::new(original, changed))
            .updated()
            .len(),
        1
    );
    assert!(
        decode_typed_program_links(
            &LinkNetwork::from_lino(&reversed.network.to_lino()).unwrap(),
            reversed.root
        )
        .is_err()
    );
    let entry = &fixture()["cases"][0];
    let mut actual = program(entry);
    let Decl::Fn(function) = &mut actual.declarations[0] else {
        unreachable!()
    };
    let proof = function.source_const_function.as_mut().unwrap();
    for role in [
        "keywordSpan",
        "functionKeywordSpan",
        "headerSpan",
        "bodySpan",
    ] {
        for coordinate in ["start", "end"] {
            proof[role][coordinate] = Value::Number(
                serde_json::Number::from_f64(proof[role][coordinate].as_f64().unwrap()).unwrap(),
            );
        }
    }
    let archive = encode_typed_program_links(&actual).unwrap();
    observe_javascript(&javascript_public_archive(&archive), entry, false);
    for spelling in [
        "1e999",
        "-1e999",
        "NaN",
        "Infinity",
        "-Infinity",
        "1e308",
        "-0",
    ] {
        let mut actual = selected("const fn predicate(value:f64)->bool{value==1.0}fn main(){}");
        let Decl::Fn(function) = &mut actual.declarations[0] else {
            unreachable!()
        };
        let Node::Binary { right, .. } = &mut function.body.node else {
            unreachable!()
        };
        right.node = Node::Lit {
            value: LitValue::Text(spelling.into()),
        };
        if spelling.ends_with("e999") {
            assert!(encode_typed_program_links(&actual).is_err());
            assert!(emit_rust(&actual).is_err());
            assert!(emit_javascript(&actual).is_err());
        } else {
            let packet = encode_typed_program_links(&actual).unwrap();
            assert!(!javascript_public_archive(&packet).is_empty());
            assert!(emit_rust(&actual).unwrap().definitions[0].starts_with("pub const fn "));
        }
    }
}

#[test]
fn complete_original_and_actual_emitted_rust_artifacts_execute_in_ci() {
    let directory = TemporaryDirectory::new("const-predicate-full-artifacts");
    let mut source = String::new();
    let mut calls = String::new();
    for (index, entry) in fixture()["cases"].as_array().unwrap().iter().enumerate() {
        let actual = program(entry);
        let emitted = emit_rust(&actual).unwrap();
        for (phase, artifact) in [entry["source"].as_str().unwrap(), emitted.text.as_str()]
            .into_iter()
            .enumerate()
        {
            let name = entry["name"].as_str().unwrap();
            writeln!(
                source,
                "mod observed_{index}_{phase}{{\n{artifact}\npub fn observe(){{main();"
            )
            .unwrap();
            let observation = match index {
                0 => format!(
                    "for a in[false,true]{{for b in[false,true]{{assert_eq!({name}(a,b),!a||b);}}}}const OBSERVED:bool={name}(false,false);assert!(OBSERVED);"
                ),
                1 => format!(
                    "for a in[false,true]{{for b in[false,true]{{for c in[false,true]{{assert_eq!({name}(a,b,c),!a||b&&c);}}}}}}const OBSERVED:bool={name}(false,false,false);assert!(OBSERVED);"
                ),
                _ => format!(
                    "for a in[f64::NEG_INFINITY,-1f64,-0f64,0f64,1f64,f64::INFINITY,f64::NAN]{{for b in[f64::NEG_INFINITY,-1f64,-0f64,0f64,1f64,f64::INFINITY,f64::NAN]{{assert_eq!({name}(a,b),a<=b);}}}}const OBSERVED:bool={name}(0f64,1f64);assert!(OBSERVED);"
                ),
            };
            writeln!(source, "{observation}\n}}\n}}").unwrap();
            writeln!(calls, "observed_{index}_{phase}::observe();").unwrap();
        }
    }
    writeln!(source, "fn main(){{{calls}}}").unwrap();
    let input = directory.0.join("observed.rs");
    let binary = directory
        .0
        .join(format!("observed{}", std::env::consts::EXE_SUFFIX));
    fs::write(&input, source).unwrap();
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    let compiled = run(compiler.arg(&input).arg("-o").arg(&binary));
    assert!(compiled.ok, "{}", compiled.stderr);
    let result = run(&mut Command::new(binary));
    assert!(result.ok, "{}", result.stderr);
    assert!(result.stdout.is_empty());
}
