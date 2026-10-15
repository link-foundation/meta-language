//! Real declaration-owned tuple initialization; native tools run only in CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::{Decl, Expr, FnDecl, Node, Program};
use meta_language::translation::rust::parse_rust_with_owned_local_state_model;
use meta_language::{
    BinaryLinksOptions, LinkId, LinkMetadata, LinkNetwork, LinkType, SubstitutionRule,
    TypedProgramLinks, create_typed_program_links_with_rust_owned_local_state_model,
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
            "../../../experiments/owned-tuple-call-fixture.lino"
        ))
        .unwrap()
    })
}
fn archives() -> &'static Value {
    static ARCHIVES: OnceLock<Value> = OnceLock::new();
    ARCHIVES.get_or_init(|| {
        read_structured_data_lino(include_str!(
            "../../../experiments/owned-tuple-call-native-fixture.lino"
        ))
        .unwrap()
    })
}
fn model() -> &'static str {
    fixture()["model"].as_str().unwrap()
}
fn archive(index: usize) -> TypedProgramLinks {
    let entry = &archives()["cases"][index];
    TypedProgramLinks {
        network: LinkNetwork::from_lino(entry["links"].as_str().unwrap()).unwrap(),
        root: LinkId::from_u64(entry["root"].as_u64().unwrap()),
    }
}
fn program(index: usize) -> Program {
    let packet = archive(index);
    decode_typed_program_links(&packet.network, packet.root).unwrap()
}
fn selected(source: &str) -> Program {
    check_program(&parse_rust_with_owned_local_state_model(source, model(), "").unwrap()).unwrap()
}
fn function_mut<'a>(program: &'a mut Program, name: &str) -> &'a mut FnDecl {
    program
        .declarations
        .iter_mut()
        .find_map(|entry| match entry {
            Decl::Fn(function) if function.full_name == name => Some(function),
            _ => None,
        })
        .unwrap()
}
fn initializer(function: &mut FnDecl) -> &mut Expr {
    let mut node = &mut function.body;
    loop {
        let Node::Let { value, body, .. } = &mut node.node else {
            panic!("an actual call initializer exists")
        };
        if matches!(value.node, Node::Call { .. }) {
            return value;
        }
        node = body;
    }
}
fn retarget(program: &mut Program, call: bool, fact: bool) {
    let function = function_mut(program, "choose");
    if call {
        let Node::Call { func, .. } = &mut initializer(function).node else {
            unreachable!()
        };
        "alternate".clone_into(func);
    }
    if fact {
        function.source_local_state.as_mut().unwrap()["initializers"][0]["target"] =
            json!("alternate");
    }
}
fn observe_javascript(code: &str, rows: &Value) -> String {
    let mut script = format!(
        "const vm=require('node:vm'),assert=require('node:assert/strict'),context={{}};vm.runInNewContext({},context);",
        json!(code)
    );
    for row in rows.as_array().unwrap() {
        write!(
            script,
            "console.log(context.choose({},BigInt({}),{}));",
            row[0], row[1], row[2]
        )
        .unwrap();
    }
    let result = run(Command::new("node").args(["-e", &script]));
    assert!(result.ok, "{}", result.stderr);
    result.stdout
}
fn expected(rows: &Value) -> String {
    rows.as_array()
        .unwrap()
        .iter()
        .fold(String::new(), |mut output, row| {
            writeln!(output, "{}", row[3].as_str().unwrap()).unwrap();
            output
        })
}
fn javascript_archive(packet: &TypedProgramLinks, target: &str) -> String {
    let directory = TemporaryDirectory::new("tuple-call-native-public-archive");
    let archive = directory.0.join("program.lino");
    fs::write(&archive, packet.network.to_lino()).unwrap();
    let index = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../js/src/index.js");
    let script = format!(
        "import {{pathToFileURL}} from 'node:url';import {{readFileSync}} from 'node:fs';const {{LinkNetwork,decodeTypedProgramLinks,emitTypedProgramLinks}}=await import(pathToFileURL({}));const network=LinkNetwork.fromLino(readFileSync({},'utf8'));decodeTypedProgramLinks(network,{});process.stdout.write(emitTypedProgramLinks(network,{},{}).code);",
        json!(index.to_string_lossy()),
        json!(archive.to_string_lossy()),
        packet.root.as_u64(),
        packet.root.as_u64(),
        json!(target)
    );
    let path = directory.0.join("consume.mjs");
    fs::write(&path, script).unwrap();
    let result = run(Command::new("node").arg(path));
    assert!(result.ok, "{}", result.stderr);
    result.stdout
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
                    && network.link(*field).and_then(|link| link.metadata().term())
                        == Some(label) =>
            {
                Some([*owner, *field, *child])
            }
            _ => None,
        })
        .unwrap()
}
fn declaration(network: &LinkNetwork, name: &str) -> LinkId {
    let label = meta_language::typed_program_links_schema()["fields"]["name"]
        .as_str()
        .unwrap_or("name");
    network
        .links()
        .find_map(|link| match link.references() {
            [owner, field, child]
                if network.link(*field).and_then(|link| link.metadata().term()) == Some(label)
                    && network
                        .link(*child)
                        .and_then(|link| link.metadata().definition())
                        == Some(name) =>
            {
                Some(*owner)
            }
            _ => None,
        })
        .unwrap()
}
fn replace_field(network: &mut LinkNetwork, parent: LinkId, field: &str, replacement: LinkId) {
    let from = relation(network, parent, field);
    assert_eq!(
        network
            .apply_substitution(&SubstitutionRule::new(
                from,
                [from[0], from[1], replacement]
            ))
            .updated()
            .len(),
        1
    );
}
fn scalar_metadata(kind: &str, definition: Option<&str>) -> LinkMetadata {
    let schema = meta_language::typed_program_links_schema();
    let metadata = LinkMetadata::new()
        .with_link_type(LinkType::Semantic)
        .with_named(true)
        .with_language(schema["language"].as_str().unwrap())
        .with_term(schema["scalars"][kind].as_str().unwrap());
    if let Some(definition) = definition {
        metadata.with_definition(definition)
    } else {
        metadata
    }
}
fn refuses(program: &Program) {
    assert!(encode_typed_program_links(program).is_err());
    assert!(emit_javascript(program).is_err());
    assert!(emit_rust(program).is_err());
}

#[test]
fn actual_generated_native_decisions_match_every_boolean_vector() {
    use super::translation::rust::owned_tuple_call_rules::{
        accept_rust_owned_tuple_call_initializer, accept_rust_owned_tuple_call_model,
    };
    assert!(accept_rust_owned_tuple_call_model(model()));
    for model in ["", "unknown", "closed owned text local state 1"] {
        assert!(!accept_rust_owned_tuple_call_model(model));
    }
    for target in [false, true] {
        for arguments in [false, true] {
            for fields in [false, true] {
                assert_eq!(
                    accept_rust_owned_tuple_call_initializer(target, arguments, fields),
                    target && arguments && fields
                );
            }
        }
    }
}

#[test]
fn all_five_complete_sources_match_actual_public_archives_and_both_formats() {
    assert_eq!(fixture()["cases"].as_array().unwrap().len(), 5);
    for (index, case) in fixture()["cases"].as_array().unwrap().iter().enumerate() {
        let expected_program = program(index);
        let actual = selected(case["source"].as_str().unwrap());
        assert_eq!(actual, expected_program, "{}", case["name"]);
        assert_eq!(
            emit_rust(&actual).unwrap().definitions.join("\n\n"),
            archives()["cases"][index]["rustDefinitions"]
                .as_str()
                .unwrap()
        );
        for (declaration, expected) in actual
            .declarations
            .iter()
            .zip(archives()["cases"][index]["proofs"].as_array().unwrap())
        {
            let Decl::Fn(function) = declaration else {
                unreachable!()
            };
            assert_eq!(
                function.source_local_state.as_ref().unwrap(),
                &expected["proof"]
            );
        }
        let packet = create_typed_program_links_with_rust_owned_local_state_model(
            case["source"].as_str().unwrap(),
            "Rust",
            model(),
            None,
            Some(&json!({"caller":"native tuple call"})),
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
            Some(model())
        );
        let user = relation(&packet.network, provenance, "user provenance")[2];
        assert_eq!(
            packet
                .network
                .link(relation(&packet.network, user, "caller")[2])
                .unwrap()
                .metadata()
                .definition(),
            Some("native tuple call")
        );
        assert!(
            packet
                .network
                .links()
                .all(|link| link.metadata().definition() != case["source"].as_str())
        );
        let text = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
        let bytes =
            encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap();
        let binary = decode_link_network_binary(&bytes, BinaryLinksOptions::default()).unwrap();
        for network in [text, binary] {
            assert_eq!(
                decode_typed_program_links(&network, packet.root).unwrap(),
                expected_program
            );
            for target in ["JavaScript", "Rust"] {
                assert_ne!(
                    emit_typed_program_links(&network, packet.root, target)
                        .unwrap()
                        .code,
                    ""
                );
            }
            let code = javascript_archive(
                &TypedProgramLinks {
                    network,
                    root: packet.root,
                },
                "JavaScript",
            );
            assert_eq!(
                observe_javascript(&code, &case["observations"]),
                expected(&case["observations"])
            );
        }
        let code = emit_javascript(&actual).unwrap().text;
        let row = &case["observations"][0];
        let script = format!(
            "const vm=require('node:vm'),assert=require('node:assert/strict'),context={{}};vm.runInNewContext({},context);const seed=context.seed;let calls=0;context.seed=(...args)=>{{calls++;return seed(...args)}};assert.equal(context.choose({},BigInt({}),{}),{});assert.equal(calls,1);",
            json!(code),
            row[0],
            row[1],
            row[2],
            row[3]
        );
        let result = run(Command::new("node").args(["-e", &script]));
        assert!(result.ok, "{}", result.stderr);
    }
}

#[test]
fn all_thirteen_source_refusals_retain_exact_diagnostics_and_spans() {
    assert_eq!(fixture()["refusals"].as_array().unwrap().len(), 13);
    for entry in fixture()["refusals"].as_array().unwrap() {
        let error =
            parse_rust_with_owned_local_state_model(entry["source"].as_str().unwrap(), model(), "")
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
    for selected_model in [
        "closed owned text local state 1",
        "canonical owned text local copies 1",
        "",
    ] {
        assert!(
            parse_rust_with_owned_local_state_model(
                fixture()["cases"][0]["source"].as_str().unwrap(),
                selected_model,
                ""
            )
            .and_then(|surface| check_program(&surface))
            .is_err()
        );
    }
}

#[test]
fn coherent_source_free_retargeting_changes_actual_execution_and_one_sided_edits_refuse() {
    let original = program(3);
    let mut changed = original.clone();
    let signature = function_mut(&mut changed, "choose")
        .source_signature
        .clone();
    retarget(&mut changed, true, true);
    let packet = encode_typed_program_links(&changed).unwrap();
    let restored = decode_typed_program_links(
        &LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        packet.root,
    )
    .unwrap();
    assert_eq!(
        function_mut(&mut changed, "choose").source_signature,
        signature
    );
    let rows = json!([
        ["before", "1", false, "second"],
        ["before", "1", true, "selected"]
    ]);
    assert_eq!(
        observe_javascript(&emit_javascript(&restored).unwrap().text, &rows),
        expected(&rows)
    );
    assert_eq!(
        observe_javascript(&javascript_archive(&packet, "JavaScript"), &rows),
        expected(&rows)
    );
    for edit in 0..7 {
        let mut value = original.clone();
        if edit < 2 {
            retarget(&mut value, edit == 0, edit == 1);
        } else {
            let function = function_mut(&mut value, "choose");
            let proof = function.source_local_state.as_mut().unwrap();
            match edit {
                2 => proof["initializers"][0]["fieldModes"]
                    .as_array_mut()
                    .unwrap()
                    .reverse(),
                3 => proof["initializers"][0]["fields"]
                    .as_array_mut()
                    .unwrap()
                    .reverse(),
                4 => proof["reads"] = json!([]),
                5 => {
                    proof["bindings"]
                        .as_array_mut()
                        .unwrap()
                        .iter_mut()
                        .find(|binding| binding["role"] == "tuple field" && binding["index"] == 1)
                        .unwrap()["mutable"] = json!(true);
                }
                _ => proof["initializers"][0]["target"] = json!("choose"),
            }
        }
        refuses(&value);
    }
}

#[test]
fn direct_declaration_composite_and_equality_counterfeits_refuse_before_emission() {
    for edit in 0..6 {
        let mut actual = serde_json::to_value(program(3)).unwrap();
        match edit {
            0 => actual["declarations"][1]["fullName"] = json!("seed"),
            1 => actual["declarations"][1]["name"] = json!("seed"),
            2 => actual["declarations"][1]["modulePath"] = json!(["counterfeit"]),
            3 => {
                let duplicate = actual["items"][1].clone();
                actual["items"].as_array_mut().unwrap().push(duplicate);
            }
            4 => {
                actual["items"].as_array_mut().unwrap().remove(1);
            }
            _ => actual["items"][1]["fullName"] = json!("counterfeit"),
        }
        refuses(&serde_json::from_value(actual).unwrap());
    }
    let mut tuple = program(0);
    let helper = function_mut(&mut tuple, "seed");
    let Node::Tuple { items, .. } = &mut helper.body.node else {
        unreachable!()
    };
    items[0] = items[1].clone();
    helper.source_local_state.as_mut().unwrap()["reads"] = json!([]);
    refuses(&tuple);
    let mut local = serde_json::to_value(program(0)).unwrap();
    let caller = &mut local["declarations"][1];
    caller["body"]["body"]["body"]["body"]["type"] = json!({"kind":"bool"});
    refuses(&serde_json::from_value(local).unwrap());
    let mut equality = serde_json::to_value(program(0)).unwrap();
    let caller = &mut equality["declarations"][1];
    let branch = caller["body"]["body"]["body"]["body"]["value"].clone();
    let pair = json!({"k":"tuple","items":[branch["then"]["value"],{"k":"lit","value":"0","type":{"kind":"fixed","bits":32,"signed":true}}],"type":caller["body"]["value"]["type"]});
    caller["body"]["body"]["body"]["body"]["value"]["cond"] = json!({"k":"binary","op":"eq","left":pair,"right":pair,"type":{"kind":"bool"},"span":branch["cond"]["span"]});
    let constructor = caller["sourceLocalState"]["literalConstructors"][0].clone();
    let mut left = constructor.clone();
    left["path"] = json!(["body", "body", "body", "value", "cond", "left", "items", 0]);
    let mut right = constructor.clone();
    right["path"] = json!(["body", "body", "body", "value", "cond", "right", "items", 0]);
    caller["sourceLocalState"]["literalConstructors"] = json!([left, right, constructor]);
    let equality = serde_json::from_value(equality).unwrap();
    assert!(
        emit_javascript(&equality)
            .unwrap_err()
            .reason
            .contains("actual Boolean/i32 domain")
    );
    assert!(
        emit_rust(&equality)
            .unwrap_err()
            .reason
            .contains("actual Boolean/i32 domain")
    );
    assert!(
        encode_typed_program_links(&equality)
            .unwrap_err()
            .to_string()
            .contains("actual Boolean/i32 domain")
    );
}

#[test]
fn original_graph_header_domains_unknown_fields_and_interpretation_order_are_checked() {
    for (kind, definition, positive) in [
        ("string", Some("32"), false),
        ("boolean", Some("true"), false),
        ("boolean", Some("false"), false),
        ("null", None, false),
        ("number", Some("32"), true),
        ("number", Some("32.0"), true),
        ("number", Some("3.2e1"), true),
    ] {
        let mut packet = archive(0);
        let helper = declaration(&packet.network, "seed");
        let signature = relation(&packet.network, helper, "sourceSignature")[2];
        let result = relation(&packet.network, signature, "result")[2];
        let ty = relation(&packet.network, result, "type")[2];
        let elements = relation(&packet.network, ty, "elements")[2];
        let integer = packet.network.link(elements).unwrap().references()[1];
        let replacement = packet
            .network
            .insert_link([], scalar_metadata(kind, definition));
        replace_field(&mut packet.network, integer, "bits", replacement);
        let restored = decode_typed_program_links(&packet.network, packet.root);
        if positive {
            let restored = restored.unwrap();
            assert_eq!(
                observe_javascript(
                    &emit_javascript(&restored).unwrap().text,
                    &fixture()["cases"][0]["observations"]
                ),
                expected(&fixture()["cases"][0]["observations"])
            );
        } else {
            assert!(restored.is_err());
        }
    }
    for (kind, definition) in [("boolean", "false"), ("number", "0"), ("string", "")] {
        let mut packet = archive(0);
        let caller = declaration(&packet.network, "choose");
        let mut node = relation(&packet.network, caller, "body")[2];
        for _ in 0..3 {
            node = relation(&packet.network, node, "body")[2];
        }
        let branch = relation(&packet.network, node, "value")[2];
        let condition = relation(&packet.network, branch, "cond")[2];
        let replacement = packet
            .network
            .insert_link([], scalar_metadata(kind, Some(definition)));
        replace_field(&mut packet.network, condition, "domain", replacement);
        assert!(decode_typed_program_links(&packet.network, packet.root).is_err());
    }
    let mut packet = archive(0);
    let caller = declaration(&packet.network, "choose");
    let proof = relation(&packet.network, caller, "source local state")[2];
    let meaning = relation(&packet.network, proof, "interpretation")[2];
    let first = relation(&packet.network, meaning, "integer domain");
    let second = relation(&packet.network, meaning, "tuple initialization");
    let temporary = packet.network.insert_link([], LinkMetadata::new());
    let marker = [first[0], first[1], temporary];
    for (from, to) in [(first, marker), (second, first), (marker, second)] {
        assert_eq!(
            packet
                .network
                .apply_substitution(&SubstitutionRule::new(from, to))
                .updated()
                .len(),
            1
        );
    }
    assert!(decode_typed_program_links(&packet.network, packet.root).is_err());
    let mut packet = archive(0);
    let caller = declaration(&packet.network, "choose");
    let body = relation(&packet.network, caller, "body")[2];
    let call = relation(&packet.network, body, "value")[2];
    let extra = packet
        .network
        .insert_link([], scalar_metadata("boolean", Some("false")));
    packet.network.insert_field(call, "borrowEscape", extra);
    assert!(decode_typed_program_links(&packet.network, packet.root).is_err());
}

fn rust_observations(sources: &[(&Value, String)], name: &str) -> String {
    let directory = TemporaryDirectory::new(name);
    let mut calls = String::new();
    let mut libraries = Vec::new();
    for (index, (case, artifact)) in sources.iter().enumerate() {
        // Keep the complete artifact at its actual crate root: emitted calls
        // such as crate::seed must not be relocated into an observation module.
        let mut source = artifact.clone();
        source.push_str("\npub fn ml_observe(){main();\n");
        for row in case["observations"].as_array().unwrap() {
            writeln!(
                source,
                "println!(\"{{}}\",choose({:?}.to_owned(),{}i32,{}));",
                row[0].as_str().unwrap(),
                row[1].as_str().unwrap(),
                row[2].as_bool().unwrap()
            )
            .unwrap();
        }
        source.push_str("}\n");
        assert!(source.starts_with(artifact.as_str()));
        let crate_name = format!("observed_{index}");
        let input = directory.0.join(format!("{crate_name}.rs"));
        let library = directory.0.join(format!("lib{crate_name}.rlib"));
        fs::write(&input, source).unwrap();
        let build = run(Command::new("rustc")
            .args(["--edition", "2024", "--crate-type", "rlib", "--crate-name"])
            .arg(&crate_name)
            .arg(input)
            .arg("-o")
            .arg(&library));
        assert!(build.ok, "{}", build.stderr);
        writeln!(calls, "{crate_name}::ml_observe();").unwrap();
        libraries.push((crate_name, library));
    }
    let consumer = directory.0.join("consumer.rs");
    let binary = directory
        .0
        .join(format!("observed{}", std::env::consts::EXE_SUFFIX));
    fs::write(&consumer, format!("fn main(){{{calls}}}")).unwrap();
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition", "2024"]).arg(consumer);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    for (crate_name, library) in libraries {
        compiler
            .arg("--extern")
            .arg(format!("{crate_name}={}", library.display()));
    }
    let build = run(compiler.arg("-o").arg(&binary));
    assert!(build.ok, "{}", build.stderr);
    let result = run(&mut Command::new(binary));
    assert!(result.ok, "{}", result.stderr);
    result.stdout
}

#[test]
fn entire_original_and_emitted_rust_artifacts_execute_before_and_after_target_edit() {
    let mut originals = Vec::new();
    let mut native = Vec::new();
    let mut javascript = Vec::new();
    let mut output = String::new();
    for (index, case) in fixture()["cases"].as_array().unwrap().iter().enumerate() {
        let program = program(index);
        let packet = encode_typed_program_links(&program).unwrap();
        output.push_str(&expected(&case["observations"]));
        originals.push((case, case["source"].as_str().unwrap().to_owned()));
        native.push((case, emit_rust(&program).unwrap().text));
        javascript.push((case, javascript_archive(&packet, "Rust")));
    }
    assert_eq!(
        rust_observations(&originals, "tuple-call-originals"),
        output
    );
    assert_eq!(
        rust_observations(&native, "tuple-call-native-emitted"),
        output
    );
    assert_eq!(
        rust_observations(&javascript, "tuple-call-javascript-emitted"),
        output
    );
    let mut changed = program(3);
    retarget(&mut changed, true, true);
    let mut case = fixture()["cases"][3].clone();
    case["observations"] = json!([
        ["before", "1", false, "second"],
        ["before", "1", true, "selected"]
    ]);
    let packet = encode_typed_program_links(&changed).unwrap();
    for artifact in [
        emit_rust(&changed).unwrap().text,
        javascript_archive(&packet, "Rust"),
    ] {
        assert_eq!(
            rust_observations(&[(&case, artifact)], "tuple-call-retarget"),
            expected(&case["observations"])
        );
    }
    let emitted = javascript_archive(&encode_typed_program_links(&program(0)).unwrap(), "Rust");
    for reader in fixture()["emittedArtifact"]["readers"].as_array().unwrap() {
        let error = parse_rust_with_owned_local_state_model(
            &emitted,
            reader["model"].as_str().unwrap(),
            "",
        )
        .and_then(|surface| check_program(&surface))
        .unwrap_err();
        assert_eq!(error.kind.as_str(), reader["kind"].as_str().unwrap());
        assert_eq!(error.reason, reader["reason"].as_str().unwrap());
        assert_eq!(serde_json::to_value(error.span).unwrap(), reader["span"]);
    }
    assert_eq!(fixture()["authentic"]["combined"]["bytes"], 132_002);
    assert_eq!(
        fixture()["authentic"]["initializer"]["originalText"],
        concat!(
            "let (mut digits, n) = decimal(&format!(\"{",
            ":e}\", x.abs()));"
        )
    );
}
