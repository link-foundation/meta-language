//! Complete owned-text source and editable-graph observations. Native tools run in CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::{Decl, Node, Program};
use meta_language::translation::rust::{parse_rust, parse_rust_with_owned_local_state_model};
use meta_language::{
    BinaryLinksOptions, LinkId, LinkMetadata, LinkNetwork, LinkType, StructuredDataValue,
    SubstitutionRule, create_typed_program_links_with_rust_owned_local_state_model,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary, encode_typed_program_links, read_structured_data_lino,
    read_structured_data_lino_ordered, structured_data_to_lino_ordered,
};
use serde_json::{Value, json};
use std::fmt::Write as _;
use std::fs;
use std::process::Command;
use std::sync::OnceLock;

const SOURCE: &str = include_str!("../../../experiments/rust-owned-local-state-fixture.lino");
fn member<'a>(value: &'a StructuredDataValue, name: &str) -> &'a StructuredDataValue {
    let StructuredDataValue::Object(fields) = value else {
        panic!("an owned record exists")
    };
    fields
        .iter()
        .find_map(|(key, value)| key.iter().copied().eq(name.encode_utf16()).then_some(value))
        .unwrap()
}
fn fixture() -> &'static Value {
    static FIXTURE: OnceLock<Value> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        // Only scalar-valid records cross the JSON host boundary. The actual
        // unpaired UTF-16 inputs are extracted losslessly in the final test.
        let StructuredDataValue::Object(fields) =
            read_structured_data_lino_ordered(SOURCE).unwrap()
        else {
            panic!("an owned fixture exists")
        };
        let selected = StructuredDataValue::Object(
            fields
                .into_iter()
                .filter(|(key, _)| !key.iter().copied().eq("inputValues".encode_utf16()))
                .collect(),
        );
        read_structured_data_lino(&structured_data_to_lino_ordered(&selected).unwrap()).unwrap()
    })
}
fn model(copies: bool) -> &'static str {
    fixture()["models"][if copies { "copies" } else { "moves" }]
        .as_str()
        .unwrap()
}
fn program(case: &Value, replay: bool) -> Program {
    let network = LinkNetwork::from_lino(
        case[if replay { "replayLinks" } else { "links" }]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    decode_typed_program_links(
        &network,
        LinkId::from_u64(
            case[if replay { "replayRoot" } else { "root" }]
                .as_u64()
                .unwrap(),
        ),
    )
    .unwrap()
}
fn selected(source: &str, copies: bool) -> Program {
    check_program(&parse_rust_with_owned_local_state_model(source, model(copies), "").unwrap())
        .unwrap()
}
fn javascript_public_archive(network: &LinkNetwork, root: LinkId) -> String {
    let directory = TemporaryDirectory::new("owned-text-native-archive");
    let archive = directory.0.join("program.lino");
    fs::write(&archive, network.to_lino()).unwrap();
    let index = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../js/src/index.js");
    let script = format!(
        "import {{pathToFileURL}} from 'node:url';import {{readFileSync}} from 'node:fs';const {{LinkNetwork,decodeTypedProgramLinks,emitTypedProgramLinks}}=await import(pathToFileURL({}));const network=LinkNetwork.fromLino(readFileSync({},'utf8'));decodeTypedProgramLinks(network,{});emitTypedProgramLinks(network,{},'Rust');process.stdout.write(emitTypedProgramLinks(network,{},'JavaScript').code);",
        json!(index.to_string_lossy()),
        json!(archive.to_string_lossy()),
        root.as_u64(),
        root.as_u64(),
        root.as_u64()
    );
    let path = directory.0.join("observe.mjs");
    fs::write(&path, script).unwrap();
    let result = run(Command::new("node").arg(&path));
    assert!(result.ok, "{}", result.stderr);
    result.stdout
}

fn functions(program: &Program) -> impl Iterator<Item = &meta_language::translation::ir::FnDecl> {
    program.declarations.iter().map(|decl| {
        let Decl::Fn(function) = decl else {
            panic!("the selected declaration is a function")
        };
        function
    })
}

#[test]
fn all_five_actual_sources_match_javascript_programs_and_both_archive_formats() {
    let cases = fixture()["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 5);
    for pattern in ["((mut digits),n)", "((mut digits,n))"] {
        let source = cases[0]["source"]
            .as_str()
            .unwrap()
            .replace("(mut digits,n)", pattern);
        let actual = selected(&source, false);
        assert_eq!(
            functions(&actual)
                .next()
                .unwrap()
                .source_local_state
                .as_ref()
                .unwrap()["initializers"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
    }
    for case in cases {
        let expected = program(case, false);
        let actual = selected(case["source"].as_str().unwrap(), false);
        assert_eq!(actual, expected, "{}", case["name"]);
        let packet = create_typed_program_links_with_rust_owned_local_state_model(
            case["source"].as_str().unwrap(),
            "Rust",
            model(false),
            None,
            Some(&json!({"fixture":case["name"]})),
        )
        .unwrap();
        let provenance = relation(&packet.network, packet.root, "provenance")[2];
        let profile = relation(&packet.network, provenance, "source profile")[2];
        assert_eq!(
            packet
                .network
                .link(relation(&packet.network, profile, "rust owned local state model")[2])
                .unwrap()
                .metadata()
                .definition(),
            Some(model(false))
        );
        let user = relation(&packet.network, provenance, "user provenance")[2];
        assert_eq!(
            packet
                .network
                .link(relation(&packet.network, user, "fixture")[2])
                .unwrap()
                .metadata()
                .definition(),
            case["name"].as_str()
        );
        let public_code = javascript_public_archive(&packet.network, packet.root);
        let expected_output = case["observations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| format!("{}\n", row[3].as_str().unwrap()))
            .collect::<String>();
        assert_eq!(javascript_observations(&public_code, case), expected_output);
        let text = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
        let bytes =
            encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap();
        let binary = decode_link_network_binary(&bytes, BinaryLinksOptions::default()).unwrap();
        for network in [&text, &binary] {
            assert_eq!(
                decode_typed_program_links(network, packet.root).unwrap(),
                expected
            );
            for target in ["Rust", "JavaScript"] {
                assert!(
                    !emit_typed_program_links(network, packet.root, target)
                        .unwrap()
                        .code
                        .is_empty()
                );
            }
        }
        assert!(functions(&actual).all(|function| {
            function.source_local_state.as_ref().unwrap()["literalConstructors"].is_array()
        }));
    }
}

#[test]
fn actual_emitted_definitions_reimport_with_their_independent_copy_interpretation() {
    for case in fixture()["cases"].as_array().unwrap() {
        let original = program(case, false);
        let emitted = emit_rust(&original).unwrap();
        assert_eq!(
            emitted.definitions.join("\n\n"),
            case["rustDefinitions"].as_str().unwrap()
        );
        let source = format!("{}\nfn main(){{}}", emitted.definitions.join("\n\n"));
        assert_eq!(source, case["replaySource"].as_str().unwrap());
        let actual = selected(&source, true);
        assert_eq!(actual, program(case, true));
        for function in functions(&actual) {
            let proof = function.source_local_state.as_ref().unwrap();
            assert_eq!(proof["model"], model(true));
            assert_eq!(proof["initializers"], json!([]));
            assert!(
                proof["reads"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|read| read["mode"] == "copy")
            );
        }
        assert!(
            parse_rust_with_owned_local_state_model(&source, model(false), "")
                .and_then(|surface| check_program(&surface))
                .is_err()
        );
        // The real generated runtime remains a separate source obligation.
        assert!(parse_rust_with_owned_local_state_model(&emitted.text, model(true), "").is_err());
    }
}

#[test]
fn every_frozen_source_refusal_keeps_its_actual_diagnostic_and_span() {
    let fixture = fixture();
    assert_eq!(fixture["invalid"].as_array().unwrap().len(), 18);
    assert_eq!(
        fixture["borrowedStringRefusals"].as_array().unwrap().len(),
        7
    );
    let operations = read_structured_data_lino(include_str!(
        "../../../experiments/rust-owned-local-source-operation-fixture.lino"
    ))
    .unwrap();
    assert_eq!(operations["cases"].as_array().unwrap().len(), 6);
    for entry in fixture["invalid"]
        .as_array()
        .unwrap()
        .iter()
        .chain(fixture["borrowedStringRefusals"].as_array().unwrap())
        .chain(operations["cases"].as_array().unwrap())
        .chain(operations["borrowedMethodRefusals"].as_array().unwrap())
    {
        let selected_model = entry["model"].as_str().unwrap_or(model(false));
        let error = parse_rust_with_owned_local_state_model(
            entry["source"].as_str().unwrap(),
            selected_model,
            "",
        )
        .and_then(|surface| check_program(&surface))
        .unwrap_err();
        assert_eq!(
            error.kind.as_str(),
            entry["kind"].as_str().unwrap(),
            "{}",
            entry["name"]
        );
        assert_eq!(
            error.reason,
            entry["reason"].as_str().unwrap(),
            "{}",
            entry["name"]
        );
        assert_eq!(
            serde_json::to_value(error.span).unwrap(),
            entry["span"],
            "{}",
            entry["name"]
        );
    }
    for case in fixture["cases"].as_array().unwrap() {
        assert!(
            parse_rust(case["source"].as_str().unwrap())
                .and_then(|surface| check_program(&surface))
                .unwrap_err()
                .reason
                .contains("mutable tuple binding")
        );
    }
    let nested = "fn bad(text:String,count:i32,flag:bool)->String{let value={let local=0i32;String::from(\"x\")};value}fn main(){}";
    assert_eq!(
        parse_rust_with_owned_local_state_model(nested, model(true), "")
            .and_then(|surface| check_program(&surface))
            .unwrap_err()
            .reason,
        "Rust owned local state proof: a local version contradicts its actual binding, order or value domain"
    );
    let source = fixture["cases"][0]["source"].as_str().unwrap();
    assert!(parse_rust_with_owned_local_state_model(source, "unknown", "").is_err());
    assert!(
        create_typed_program_links_with_rust_owned_local_state_model(
            source,
            "JavaScript",
            model(false),
            None,
            None
        )
        .is_err()
    );
    assert!(
        parse_rust_with_owned_local_state_model(
            &format!("extern crate alloc;{source}"),
            model(false),
            ""
        )
        .unwrap_err()
        .reason
        .contains("external crate declarations")
    );
}

fn relation(network: &LinkNetwork, parent: LinkId, label: &str) -> [LinkId; 3] {
    let label = meta_language::typed_program_links_schema()["fields"][label]
        .as_str()
        .unwrap_or(label);
    network
        .links()
        .find_map(|link| match link.references() {
            [owner, field, child]
                if *owner == parent
                    && network
                        .link(*field)
                        .and_then(|label| label.metadata().term())
                        == Some(label) =>
            {
                Some([*owner, *field, *child])
            }
            _ => None,
        })
        .unwrap()
}
fn reverse_record(value: &mut Value) {
    let record = value.as_object_mut().unwrap();
    *record = std::mem::take(record).into_iter().rev().collect();
}
fn graph_occurrence(network: &LinkNetwork, mut node: LinkId, path: LinkId) -> LinkId {
    for part in network.link(path).unwrap().references() {
        let part = network.link(*part).unwrap();
        let definition = part.metadata().definition().unwrap();
        node = if part.metadata().term()
            == meta_language::typed_program_links_schema()["scalars"]["number"].as_str()
        {
            network.link(node).unwrap().references()[definition.parse::<usize>().unwrap()]
        } else {
            relation(network, node, definition)[2]
        };
    }
    node
}

fn reorder_two_fields(network: &mut LinkNetwork, owner: LinkId, first: &str, second: &str) {
    let a = relation(network, owner, first);
    let b = relation(network, owner, second);
    let temporary = network.insert_link([], LinkMetadata::new());
    let marker = [a[0], a[1], temporary];
    for (from, to) in [(a, marker), (b, a), (marker, b)] {
        assert_eq!(
            network
                .apply_substitution(&SubstitutionRule::new(from, to))
                .updated()
                .len(),
            1
        );
    }
}

#[test]
fn source_free_owned_state_edits_and_raw_unknown_expression_fields_refuse() {
    let case = &fixture()["cases"][0];
    for edit in 0..18 {
        let mut actual = program(case, false);
        let Decl::Fn(function) = &mut actual.declarations[0] else {
            unreachable!()
        };
        let proof = function.source_local_state.as_mut().unwrap();
        match edit {
            0 => proof["bindings"].as_array_mut().unwrap().reverse(),
            1 => proof["reads"].as_array_mut().unwrap().reverse(),
            2 => proof["initializers"][0]["fields"]
                .as_array_mut()
                .unwrap()
                .reverse(),
            3 => {
                proof["bindings"]
                    .as_array_mut()
                    .unwrap()
                    .iter_mut()
                    .find(|binding| binding["role"] == "replacement")
                    .unwrap()["previous"] = json!("count")
            }
            4 => proof["reads"][0]["mode"] = json!("copy"),
            5 => proof["interpretation"]["integerDomain"]["bits"] = json!(64),
            6 => proof["model"] = json!(model(true)),
            7 => proof["sourceName"] = json!("counterfeit"),
            8 => proof["literalConstructors"] = json!([]),
            9 => proof["literalConstructors"][0]["callSpan"] = json!({"start":0,"end":1000000}),
            10 => proof["literalConstructors"][0]["owner"]["path"] = json!(["String"]),
            11 => proof["bindings"][0]["borrowEscape"] = json!(false),
            12 => function.span = None,
            13 => reverse_record(&mut proof["interpretation"]),
            14 => reverse_record(&mut proof["literalConstructors"][0]["owner"]),
            15 => reverse_record(&mut proof["reads"][0]["span"]),
            16 => reverse_record(&mut proof["literalConstructors"][0]["span"]),
            _ => reverse_record(&mut proof["interpretation"]["integerDomain"]),
        }
        assert!(encode_typed_program_links(&actual).is_err(), "edit {edit}");
    }
    let mut numeric = program(case, false);
    let Decl::Fn(function) = &mut numeric.declarations[0] else {
        unreachable!()
    };
    let proof = function.source_local_state.as_mut().unwrap();
    for role in ["reads", "literalConstructors"] {
        let span = &mut proof[role][0][if role == "reads" { "span" } else { "callSpan" }];
        for coordinate in ["start", "end"] {
            span[coordinate] = Value::Number(
                serde_json::Number::from_f64(span[coordinate].as_f64().unwrap()).unwrap(),
            );
        }
    }
    proof["interpretation"]["integerDomain"]["bits"] = json!(32.0);
    let packet = encode_typed_program_links(&numeric).unwrap();
    let code = javascript_public_archive(&packet.network, packet.root);
    assert!(!javascript_observations(&code, case).is_empty());
    let mut negative_zero = program(case, false);
    let Decl::Fn(function) = &mut negative_zero.declarations[0] else {
        unreachable!()
    };
    function.source_local_state.as_mut().unwrap()["reads"][0]["span"]["start"] = json!(-0.0);
    let Node::Let { value, .. } = &mut function.body.node else {
        unreachable!()
    };
    let Node::Tuple { items, .. } = &mut value.node else {
        unreachable!()
    };
    items[0].span.as_mut().unwrap().start = 0;
    assert!(encode_typed_program_links(&negative_zero).is_err());
    for role in ["interpretation", "integer domain", "constructor owner"] {
        let mut network = LinkNetwork::from_lino(case["links"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(case["root"].as_u64().unwrap());
        let body = network.link(root).unwrap().references()[0];
        let declarations = relation(&network, body, "declarations")[2];
        let pair = network.link(declarations).unwrap().references()[0];
        let function = network.link(pair).unwrap().references()[1];
        let proof = relation(&network, function, "source local state")[2];
        let interpretation = relation(&network, proof, "interpretation")[2];
        let (owner, first, second) = match role {
            "interpretation" => (interpretation, "text owner", "text domain"),
            "integer domain" => (
                relation(&network, interpretation, "integer domain")[2],
                "kind",
                "bits",
            ),
            _ => {
                let constructors = relation(&network, proof, "literal constructors")[2];
                let constructor = network.link(constructors).unwrap().references()[0];
                (relation(&network, constructor, "owner")[2], "kind", "path")
            }
        };
        reorder_two_fields(&mut network, owner, first, second);
        let restored = LinkNetwork::from_lino(&network.to_lino()).unwrap();
        for target in ["Rust", "JavaScript"] {
            assert!(
                emit_typed_program_links(&restored, root, target).is_err(),
                "{role}"
            );
        }
    }
    let mut matching = LinkNetwork::from_lino(case["links"].as_str().unwrap()).unwrap();
    let root = LinkId::from_u64(case["root"].as_u64().unwrap());
    let program_root = matching.link(root).unwrap().references()[0];
    let declarations = relation(&matching, program_root, "declarations")[2];
    let pair = matching.link(declarations).unwrap().references()[0];
    let function = matching.link(pair).unwrap().references()[1];
    let proof = relation(&matching, function, "source local state")[2];
    let reads = relation(&matching, proof, "reads")[2];
    let read = matching.link(reads).unwrap().references()[0];
    let body = relation(&matching, function, "body")[2];
    let occurrence = graph_occurrence(&matching, body, relation(&matching, read, "path")[2]);
    let actual_span = relation(&matching, occurrence, "span")[2];
    let proof_span = relation(&matching, read, "span")[2];
    reorder_two_fields(&mut matching, actual_span, "start", "end");
    if proof_span != actual_span {
        reorder_two_fields(&mut matching, proof_span, "start", "end");
    }
    let restored = LinkNetwork::from_lino(&matching.to_lino()).unwrap();
    for target in ["Rust", "JavaScript"] {
        assert!(
            !emit_typed_program_links(&restored, root, target)
                .unwrap()
                .code
                .is_empty()
        );
    }
    assert!(!javascript_public_archive(&restored, root).is_empty());
    let mut network = LinkNetwork::from_lino(case["links"].as_str().unwrap()).unwrap();
    let variable = network
        .links()
        .find_map(|link| match link.references() {
            [parent, label, child]
                if network
                    .link(*label)
                    .and_then(|label| label.metadata().term())
                    == meta_language::typed_program_links_schema()["fields"]["k"].as_str()
                    && network
                        .link(*child)
                        .and_then(|child| child.metadata().definition())
                        == Some("var") =>
            {
                Some(*parent)
            }
            _ => None,
        })
        .unwrap();
    let child = network.insert_link(
        [],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_language("meta-language typed program")
            .with_named(true)
            .with_term("boolean value")
            .with_definition("true"),
    );
    network.insert_field(variable, "borrow escape", child);
    let loaded = LinkNetwork::from_lino(&network.to_lino()).unwrap();
    let root = LinkId::from_u64(case["root"].as_u64().unwrap());
    for target in ["Rust", "JavaScript"] {
        assert!(emit_typed_program_links(&loaded, root, target).is_err());
    }
}

fn javascript_observations(code: &str, case: &Value) -> String {
    let mut script = code.to_owned();
    for row in case["observations"].as_array().unwrap() {
        writeln!(
            script,
            "console.log({}({},BigInt({}),{}));",
            case["name"].as_str().unwrap(),
            row[0],
            row[1],
            row[2]
        )
        .unwrap();
    }
    let result = run(Command::new("node").args(["--input-type=module", "--eval", &script]));
    assert!(result.ok, "{}", result.stderr);
    result.stdout
}
fn rust_observations(sources: &[(&Value, String)], name: &str) -> String {
    let directory = TemporaryDirectory::new(name);
    let mut source = String::new();
    let mut calls = String::new();
    for (index, (case, artifact)) in sources.iter().enumerate() {
        // Each entire artifact is byte-preserved inside its own source module.
        writeln!(
            source,
            "mod observed_{index} {{\n{artifact}\npub fn observe(){{main();"
        )
        .unwrap();
        for row in case["observations"].as_array().unwrap() {
            writeln!(
                source,
                "println!(\"{{}}\",{}({:?}.to_owned(),{}i32,{}));",
                case["name"].as_str().unwrap(),
                row[0].as_str().unwrap(),
                row[1].as_str().unwrap(),
                row[2].as_bool().unwrap()
            )
            .unwrap();
        }
        writeln!(source, "}}\n}}").unwrap();
        writeln!(calls, "observed_{index}::observe();").unwrap();
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
    result.stdout
}
#[test]
fn complete_original_and_actual_generated_artifacts_execute_before_and_after_value_edit() {
    let mut originals = Vec::new();
    let mut translated = Vec::new();
    let mut expected = String::new();
    for case in fixture()["cases"].as_array().unwrap() {
        let actual = program(case, false);
        let javascript = emit_javascript(&actual).unwrap();
        let rows = case["observations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| format!("{}\n", row[3].as_str().unwrap()))
            .collect::<String>();
        assert_eq!(javascript_observations(&javascript.text, case), rows);
        expected.push_str(&rows);
        originals.push((case, case["source"].as_str().unwrap().to_owned()));
        translated.push((case, emit_rust(&actual).unwrap().text));
    }
    assert_eq!(
        rust_observations(&originals, "owned-text-originals"),
        expected
    );
    assert_eq!(
        rust_observations(&translated, "owned-text-translated"),
        expected
    );
    let case = &fixture()["cases"][0];
    let mut edited = program(case, false);
    let Decl::Fn(function) = &mut edited.declarations[0] else {
        unreachable!()
    };
    let signature = function.source_signature.clone();
    let proof = function.source_local_state.clone();
    let mut node = &mut function.body;
    loop {
        let Node::Let { value, body, .. } = &mut node.node else {
            panic!("a state continuation exists")
        };
        if let Node::If { then, .. } = &mut value.node {
            let Node::Let { value, .. } = &mut then.node else {
                panic!("an actual replacement exists")
            };
            assert!(matches!(value.node, Node::Lit { .. }));
            value.node = Node::Lit {
                value: meta_language::translation::ir::LitValue::Text("edited".into()),
            };
            break;
        }
        node = body;
    }
    assert_eq!(function.source_signature, signature);
    assert_eq!(function.source_local_state, proof);
    let archive = encode_typed_program_links(&edited).unwrap();
    let text = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
    let loaded = decode_typed_program_links(&text, archive.root).unwrap();
    let code = emit_typed_program_links(&text, archive.root, "JavaScript")
        .unwrap()
        .code;
    let mut observation = case.clone();
    observation["observations"] = json!([["seed", "0", true, "edited"], ["seed", "0", false, "7"]]);
    assert_eq!(javascript_observations(&code, &observation), "edited\n7\n");
    let emitted = emit_rust(&loaded).unwrap();
    assert!(emitted.text.contains(&emitted.definitions[0]));
    let definitions = format!("{}\nfn main(){{}}", emitted.definitions.join("\n\n"));
    let replay = selected(&definitions, true);
    assert_eq!(
        javascript_observations(&emit_javascript(&replay).unwrap().text, &observation),
        "edited\n7\n"
    );
    assert_eq!(
        rust_observations(&[(&observation, emitted.text)], "owned-text-edited"),
        "edited\n7\n"
    );
}

#[test]
fn actual_unpaired_utf16_fixture_units_reach_the_generated_kernel_without_loss() {
    let source = "fn hygiene_input(ml_ownedTextInput:String,count:i32,flag:bool)->String{let(mut digits,n)=(ml_ownedTextInput,count);digits=String::from(\"hygiene\");digits}fn main(){}";
    let actual = selected(source, false);
    let emitted = emit_javascript(&actual).unwrap();
    assert!(
        emitted
            .assumptions
            .iter()
            .any(|assumption| assumption.id == "owned-local-text-input-realm")
    );
    let target = &emitted
        .mappings
        .iter()
        .find(|mapping| mapping.source == "hygiene_input")
        .unwrap()
        .target;
    let script = format!(
        "{}\nif({}(\"😀\",0n,true)!==\"hygiene\")throw new Error('wrong original Unicode value');for(const args of [[String.fromCharCode(0xd800),0n,true],[0,0n,true],[\"text\",0,true],[\"text\",0n,1],[\"text\",2147483648n,true],[\"text\",-2147483649n,true]]){{let refused=false;try{{{}(...args);}}catch(error){{if(!(error instanceof RangeError))throw error;refused=true;}}if(!refused)throw new Error('selected input domain was not checked');}}console.log('owned-inputs');",
        emitted.text, target, target
    );
    let result = run(Command::new("node").args(["--input-type=module", "--eval", &script]));
    assert!(result.ok, "{}", result.stderr);
    assert_eq!(result.stdout, "owned-inputs\n");
    let data = read_structured_data_lino_ordered(SOURCE).unwrap();
    let inputs = member(&data, "inputValues");
    for (role, expected) in [("validText", true), ("invalidText", false)] {
        let StructuredDataValue::Array(values) = member(inputs, role) else {
            panic!("actual input vectors exist")
        };
        assert_eq!(values.len(), 7);
        for value in values {
            let StructuredDataValue::String(units) = value else {
                panic!("an exact UTF16 string exists")
            };
            assert_eq!(String::from_utf16(units).is_ok(), expected);
            let numeric = units
                .iter()
                .map(|unit| f64::from(*unit))
                .collect::<Vec<_>>();
            assert_eq!(crate::translation::rust::owned_local_state_rules::accept_rust_owned_local_text_units(&numeric),expected);
        }
    }
}
