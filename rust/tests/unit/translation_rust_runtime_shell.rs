//! Complete actual emitter artifacts and real cross-runtime graph identities.
//! Rust compilation and execution happen only in CI, never local development.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::ir::{Decl, Effect, Node, Program};
use meta_language::translation::rust::parse_rust_with_runtime_shell_model;
use meta_language::{
    LinkId, LinkMetadata, LinkNetwork, LinkType, SubstitutionRule, TypedProgramLinks,
    create_typed_program_links_with_rust_runtime_shell_model, decode_typed_program_links,
    emit_typed_program_links, encode_typed_program_links,
};
use serde_json::{Value, json};
use std::{fs, path::PathBuf, process::Command};
const MODEL: &str = "closed pure Rust unit runtime shell 1";
fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-runtime-shell.lino"
    ))
    .unwrap()
}
fn api() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../js/src/index.js")
}
fn javascript_packet(source: &str) -> TypedProgramLinks {
    let result=run(Command::new("node").arg("--input-type=module").arg("-e").arg("import{pathToFileURL}from'node:url';const api=await import(pathToFileURL(process.argv[1]));const packet=api.createTypedProgramLinks(process.argv[2],'Rust',{rustRuntimeShellModel:process.argv[3]});process.stdout.write(String(packet.root.asU64())+'\\n'+packet.network.toLino());").arg(api()).arg(source).arg(MODEL));
    assert!(result.ok, "{}", result.stderr);
    let (root, links) = result.stdout.split_once('\n').unwrap();
    TypedProgramLinks {
        network: LinkNetwork::from_lino(links).unwrap(),
        root: LinkId::from_u64(root.parse().unwrap()),
    }
}
fn javascript_refusal(source: &str) -> Value {
    let result=run(Command::new("node").arg("--input-type=module").arg("-e").arg("import{pathToFileURL}from'node:url';const api=await import(pathToFileURL(process.argv[1]));try{api.createTypedProgramLinks(process.argv[2],'Rust',{rustRuntimeShellModel:process.argv[3]});throw Error('source unexpectedly admitted');}catch(error){if(!error.kind)throw error;console.log(JSON.stringify({kind:error.kind,reason:error.reason,span:error.span??null}));}").arg(api()).arg(source).arg(MODEL));
    assert!(result.ok, "{}", result.stderr);
    serde_json::from_str(result.stdout.trim()).unwrap()
}
fn archive(packet: &TypedProgramLinks) -> TypedProgramLinks {
    TypedProgramLinks {
        network: LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        root: packet.root,
    }
}
fn checked(packet: &TypedProgramLinks) -> Program {
    decode_typed_program_links(&packet.network, packet.root).unwrap()
}
fn calls(program: &Program) -> Value {
    json!(
        program
            .main
            .as_ref()
            .unwrap()
            .effects
            .iter()
            .map(|effect| {
                let Effect::Let { value, .. } = effect else {
                    panic!("owned worker lets")
                };
                let Node::Call { func, .. } = &value.node else {
                    panic!("owned worker calls")
                };
                func.as_str()
            })
            .collect::<Vec<_>>()
    )
}
fn execute_javascript(source: &str) -> String {
    let result = run(Command::new("node")
        .arg("--input-type=module")
        .arg("-e")
        .arg(source));
    assert!(result.ok, "{}", result.stderr);
    result.stdout.trim_end_matches('\n').to_owned()
}
fn observe_in_javascript(packet: &TypedProgramLinks) -> Value {
    let directory = TemporaryDirectory::new("whole-runtime-cross-host");
    let file = directory.0.join("program.lino");
    fs::write(&file, packet.network.to_lino()).unwrap();
    let result=run(Command::new("node").arg("--input-type=module").arg("-e").arg("import{pathToFileURL}from'node:url';import{readFileSync}from'node:fs';import vm from'node:vm';const api=await import(pathToFileURL(process.argv[1]));const network=api.LinkNetwork.fromLino(readFileSync(process.argv[2],'utf8')),root=api.LinkId.fromU64(Number(process.argv[3])),program=api.decodeTypedProgramLinks(network,root),emitted=api.emitTypedProgramLinks(network,root,'JavaScript'),context=vm.createContext({console:{log(){}}});vm.runInContext(emitted.code,context);const calls=[];for(const name of program.declarations.keys())context[name]=()=>{calls.push(name);return null;};vm.runInContext('main();',context);const rust=api.emitTypedProgramLinks(network,root,'Rust');api.createTypedProgramLinks(rust.code,'Rust',{rustRuntimeShellModel:process.argv[4]});console.log(JSON.stringify({calls,code:emitted.code,assumptions:emitted.assumptions,encodings:emitted.encodings,rust:rust.code,rustAssumptions:rust.assumptions,rustEncodings:rust.encodings}));").arg(api()).arg(file).arg(packet.root.as_u64().to_string()).arg(MODEL));
    assert!(result.ok, "{}", result.stderr);
    serde_json::from_str(result.stdout.trim()).unwrap()
}
fn compile_actual_artifact(source: &str) {
    let directory = TemporaryDirectory::new("whole-runtime-actual-artifact");
    let input = directory.0.join("main.rs");
    let output = directory.0.join("program");
    fs::write(&input, source).unwrap();
    let compiled = run(Command::new("rustc")
        .arg("--edition=2024")
        .arg(input)
        .arg("-o")
        .arg(&output));
    assert!(compiled.ok, "{}", compiled.stderr);
    let executed = run(&mut Command::new(output));
    assert!(executed.ok, "{}", executed.stderr);
    assert_eq!(executed.stdout, "");
}
fn example() -> TypedProgramLinks {
    let data = fixture();
    let entry = data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == "ordered-repeated")
        .unwrap();
    create_typed_program_links_with_rust_runtime_shell_model(
        entry["fullEmittedRust"].as_str().unwrap(),
        "Rust",
        MODEL,
        None,
    )
    .unwrap()
}
fn field(network: &LinkNetwork, parent: LinkId, name: &str) -> (LinkId, LinkId) {
    let label = meta_language::typed_program_links_schema()["fields"][name]
        .as_str()
        .map_or_else(
            || {
                name.chars()
                    .flat_map(|character| {
                        if character.is_ascii_uppercase() {
                            vec![' ', character.to_ascii_lowercase()]
                        } else {
                            vec![character]
                        }
                    })
                    .collect::<String>()
            },
            str::to_owned,
        );
    network
        .links()
        .find_map(|link| match link.references() {
            [owner, key, child]
                if *owner == parent
                    && link.metadata().link_type() == Some(LinkType::Field)
                    && network.link(*key).and_then(|key| key.metadata().term())
                        == Some(label.as_str()) =>
            {
                Some((link.id(), *child))
            }
            _ => None,
        })
        .unwrap()
}
fn replace(network: &mut LinkNetwork, parent: LinkId, name: &str, child: LinkId) {
    let relation = field(network, parent, name).0;
    let mut references = network.link(relation).unwrap().references().to_vec();
    references[2] = child;
    let old: [LinkId; 3] = network
        .link(relation)
        .unwrap()
        .references()
        .try_into()
        .unwrap();
    let report = network.apply_substitution(&SubstitutionRule::new(
        old,
        [references[0], references[1], child],
    ));
    assert_eq!(report.updated().len(), 1);
}
fn scalar(network: &mut LinkNetwork, value: &Value) -> LinkId {
    let (kind, text) = match value {
        Value::Null => ("null", None),
        Value::Bool(value) => ("boolean", Some(value.to_string())),
        Value::Number(value) => ("number", Some(value.to_string())),
        Value::String(value) => ("string", Some(value.clone())),
        _ => panic!("scalar"),
    };
    let mut metadata = LinkMetadata::new()
        .with_link_type(LinkType::Semantic)
        .with_language("meta-language typed program")
        .with_named(true)
        .with_term(format!("{kind} value"));
    if let Some(text) = text {
        metadata = metadata.with_definition(text);
    }
    network.insert_link([], metadata)
}
fn clone_record(network: &mut LinkNetwork, original: LinkId) -> LinkId {
    let link = network.link(original).unwrap();
    let metadata = link.metadata().clone();
    let references = link.references().to_vec();
    let fields = network
        .links()
        .filter_map(|link| match link.references() {
            [owner, label, child]
                if *owner == original && link.metadata().link_type() == Some(LinkType::Field) =>
            {
                Some((
                    network
                        .link(*label)
                        .unwrap()
                        .metadata()
                        .term()
                        .unwrap()
                        .to_owned(),
                    *child,
                ))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let cloned = network.insert_dynamic_link(&references, metadata);
    for (label, child) in fields {
        network.insert_field(cloned, &label, child);
    }
    cloned
}
fn reject(packet: &TypedProgramLinks) {
    let packet = archive(packet);
    assert!(decode_typed_program_links(&packet.network, packet.root).is_err());
    for target in ["Rust", "JavaScript"] {
        assert!(emit_typed_program_links(&packet.network, packet.root, target).is_err());
    }
}

#[test]
fn eight_real_complete_emitter_artifacts_replay_between_both_hosts_and_execute() {
    let data = fixture();
    assert_eq!(data["cases"].as_array().unwrap().len(), 8);
    for entry in data["cases"].as_array().unwrap() {
        let source = entry["fullEmittedRust"].as_str().unwrap();
        assert_eq!(
            execute_javascript(entry["original"].as_str().unwrap()),
            entry["output"].as_str().unwrap()
        );
        let from_javascript = javascript_packet(source);
        let mut packet =
            create_typed_program_links_with_rust_runtime_shell_model(source, "Rust", MODEL, None)
                .unwrap();
        assert_eq!(checked(&from_javascript), checked(&packet));
        compile_actual_artifact(source);
        for cycle in 0..3 {
            packet = archive(&packet);
            let program = checked(&packet);
            assert_eq!(calls(&program), entry["calls"]);
            let observed = observe_in_javascript(&packet);
            assert_eq!(observed["calls"], entry["calls"]);
            assert_eq!(observed["assumptions"].as_array().unwrap().len(), 4);
            let javascript =
                emit_typed_program_links(&packet.network, packet.root, "JavaScript").unwrap();
            assert_eq!(
                execute_javascript(&javascript.code),
                entry["output"].as_str().unwrap()
            );
            assert_eq!(javascript.code, observed["code"].as_str().unwrap());
            assert_eq!(
                serde_json::to_value(&javascript.encodings).unwrap(),
                observed["encodings"]
            );
            assert_eq!(
                serde_json::to_value(&javascript.assumptions).unwrap(),
                observed["assumptions"]
            );
            let rust = emit_typed_program_links(&packet.network, packet.root, "Rust").unwrap();
            assert_eq!(rust.code, observed["rust"].as_str().unwrap());
            assert_eq!(
                serde_json::to_value(&rust.assumptions).unwrap(),
                observed["rustAssumptions"]
            );
            assert_eq!(
                serde_json::to_value(&rust.encodings).unwrap(),
                observed["rustEncodings"]
            );
            for operation in [
                "std::env::args().any",
                "std::thread::Builder::new()",
                ".stack_size(1 << 28)",
                ".spawn(move ||",
                ".expect(\"spawn the program thread\")",
                "worker.join().is_err()",
                "std::process::exit(101);",
            ] {
                assert!(rust.code.contains(operation), "{operation}");
            }
            if cycle == 0 {
                compile_actual_artifact(&rust.code);
            }
            packet = create_typed_program_links_with_rust_runtime_shell_model(
                &rust.code, "Rust", MODEL, None,
            )
            .unwrap();
        }
    }
}
#[test]
fn coherent_real_declaration_retarget_and_resource_edits_preserve_actual_graph_ownership() {
    let packet = example();
    let mut program = checked(&packet);
    let alpha = program
        .declarations
        .iter()
        .position(|entry| entry.name() == "alpha")
        .unwrap();
    if let Decl::Fn(function) = &mut program.declarations[alpha] {
        function.name = "gamma".into();
        function.full_name = "gamma".into();
    }
    for item in &mut program.items {
        if let meta_language::translation::ir::Item::Decl { full_name } = item
            && full_name == "alpha"
        {
            *full_name = "gamma".into();
        }
    }
    for effect in &mut program.main.as_mut().unwrap().effects {
        if let Effect::Let { value, .. } = effect
            && let Node::Call { func, .. } = &mut value.node
            && func == "alpha"
        {
            *func = "gamma".into();
        }
    }
    let proof = program.source_runtime_shell.as_mut().unwrap();
    let tokens = proof["tokens"].as_array_mut().unwrap();
    for token in tokens {
        if token["kind"] == "number" && token["value"] == "28" {
            token["value"] = json!("20");
        }
        if token["kind"] == "identifier" && token["value"] == "worker" {
            token["value"] = json!("runner");
        }
    }
    let packet = archive(&encode_typed_program_links(&program).unwrap());
    let observed = observe_in_javascript(&packet);
    assert_eq!(observed["calls"], json!(["gamma", "beta", "gamma"]));
    let rust = emit_typed_program_links(&packet.network, packet.root, "Rust").unwrap();
    assert!(rust.code.contains("1 << 20"));
    assert!(rust.code.contains("runner.join()"));
    compile_actual_artifact(&rust.code);
}
#[test]
fn strict_actual_link_identity_refuses_cloned_targets_tokens_and_assumption_operations() {
    for change in 0..5 {
        let mut packet = example();
        let body = packet.network.link(packet.root).unwrap().references()[0];
        let proof = field(&packet.network, body, "sourceRuntimeShell").1;
        let tokens = field(&packet.network, proof, "tokens").1;
        let occurrences = packet.network.link(tokens).unwrap().references().to_vec();
        match change {
            0 => {
                let mut changed = occurrences.clone();
                changed[1] = changed[0];
                let metadata = packet.network.link(tokens).unwrap().metadata().clone();
                let duplicate = packet.network.insert_dynamic_link(&changed, metadata);
                replace(&mut packet.network, proof, "tokens", duplicate);
            }
            1 | 2 => {
                let role = if change == 1 {
                    "declaration"
                } else {
                    "binding"
                };
                let (token, target) = occurrences
                    .iter()
                    .find_map(|token| {
                        packet
                            .network
                            .links()
                            .find_map(|link| match link.references() {
                                [owner, key, target]
                                    if owner == token
                                        && packet
                                            .network
                                            .link(*key)
                                            .and_then(|key| key.metadata().term())
                                            == Some(role) =>
                                {
                                    Some((*token, *target))
                                }
                                _ => None,
                            })
                    })
                    .unwrap();
                let cloned = clone_record(&mut packet.network, target);
                replace(&mut packet.network, token, role, cloned);
            }
            3 => {
                let assumptions = field(&packet.network, proof, "assumptions").1;
                let assumption = packet.network.link(assumptions).unwrap().references()[0];
                let target = field(&packet.network, assumption, "operationNode").1;
                let cloned = clone_record(&mut packet.network, target);
                replace(&mut packet.network, assumption, "operationNode", cloned);
            }
            _ => {
                let relation = field(&packet.network, body, "sourceRuntimeShell").0;
                let old: [LinkId; 3] = packet
                    .network
                    .link(relation)
                    .unwrap()
                    .references()
                    .try_into()
                    .unwrap();
                packet
                    .network
                    .apply_substitution(&SubstitutionRule::delete(old));
            }
        }
        reject(&packet);
    }
}
#[test]
#[expect(
    clippy::unreadable_literal,
    reason = "The JavaScript lexer test reads this complete file and requires this exact suffixed literal."
)]
fn malformed_runtime_models_namespace_spans_and_unknown_raw_fields_refuse_before_serde() {
    for (role, name, value) in [
        ("proof", "model", json!("unknown")),
        ("proof", "model", Value::Null),
        ("proof", "unknown", json!(false)),
        ("worker", "observationModel", json!("unconditional")),
        ("worker", "unknown", json!(false)),
        ("main", "sourceRuntimeWorker", Value::Null),
        ("span", "start", json!(-0.0)),
        ("span", "end", json!(9007199254740992_u64)),
    ] {
        let mut packet = example();
        let body = packet.network.link(packet.root).unwrap().references()[0];
        let proof = field(&packet.network, body, "sourceRuntimeShell").1;
        let main = field(&packet.network, body, "main").1;
        let target = match role {
            "proof" => proof,
            "worker" => field(&packet.network, main, "sourceRuntimeWorker").1,
            "span" => field(&packet.network, proof, "span").1,
            _ => main,
        };
        let child = scalar(&mut packet.network, &value);
        if name == "unknown" {
            packet.network.insert_field(target, name, child);
        } else {
            replace(&mut packet.network, target, name, child);
        }
        reject(&packet);
    }
}
#[test]
fn orphan_worker_and_semantic_position_corruption_cannot_authorize_either_target() {
    for change in 0..7 {
        let mut program = checked(&example());
        match change {
            0 => program.source_runtime_shell = None,
            1 => program.main.as_mut().unwrap().source_runtime_worker = None,
            2 => {
                if let Effect::Let { value, .. } = &mut program.main.as_mut().unwrap().effects[0]
                    && let Node::Call { func, .. } = &mut value.node
                {
                    *func = "beta".into();
                }
            }
            3 => program.output_threaded = true,
            4 => {
                program.source_runtime_shell.as_mut().unwrap()["assumptions"]
                    .as_array_mut()
                    .unwrap()
                    .pop();
            }
            5 => {
                let token = program.source_runtime_shell.as_mut().unwrap()["tokens"]
                    .as_array_mut()
                    .unwrap()
                    .iter_mut()
                    .find(|token| token.get("declaration").is_some())
                    .unwrap();
                token["alias"] = json!("a b c");
            }
            _ => {
                let token = program.source_runtime_shell.as_mut().unwrap()["tokens"]
                    .as_array_mut()
                    .unwrap()
                    .iter_mut()
                    .find(|token| token["kind"] == "string")
                    .unwrap();
                token["span"]["start"] = json!(token["span"]["end"].as_u64().unwrap() - 1);
            }
        }
        assert!(encode_typed_program_links(&program).is_err());
        assert!(meta_language::translation::emit_rust::emit_rust(&program).is_err());
        assert!(meta_language::translation::emit_javascript::emit_javascript(&program).is_err());
    }
}
#[test]
fn whole_source_refusals_and_default_models_remain_independent() {
    let data = fixture();
    let source = data["cases"][2]["fullEmittedRust"].as_str().unwrap();
    for (before, after) in [
        ("std::env::args", "other::env::args"),
        ("crate::empty()", "&crate::empty()"),
        ("move ||", "||"),
        ("move || { ml_main() }", "move || { check }"),
        ("1 << 28", "1 < < 28"),
        ("1 << 28", "1 << 64"),
        ("exit(101)", "exit(0)"),
        ("pub fn empty", "pub fn std"),
        ("fn main()", "pub fn main()"),
    ] {
        let changed = source.replacen(before, after, 1);
        assert_ne!(changed, source);
        let error = parse_rust_with_runtime_shell_model(&changed, MODEL).unwrap_err();
        assert_eq!(
            json!({"kind":error.kind.as_str(),"reason":error.reason,"span":error.span}),
            javascript_refusal(&changed),
            "{before}"
        );
    }
    assert!(parse_rust_with_runtime_shell_model(source, "unknown").is_err());
    assert!(meta_language::translation::rust::parse_rust(source).is_err());
    for model in [
        "closed pure Rust unit calls 1",
        "closed public pure Rust unit calls 1",
    ] {
        assert!(
            meta_language::translation::rust::parse_rust_with_pure_unit_call_model(source, model)
                .is_err()
        );
    }
    assert!(
        create_typed_program_links_with_rust_runtime_shell_model(
            "function empty(){}",
            "JavaScript",
            MODEL,
            None
        )
        .is_err()
    );
}

#[test]
fn direct_native_source_span_number_aliases_reuse_the_shared_label_based_normalizer() {
    let mut program = checked(&example());
    let proof = program.source_runtime_shell.as_mut().unwrap();
    proof["span"]["start"] = json!(0.0);
    let token = proof["tokens"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|token| token["value"] == "<")
        .unwrap();
    token["span"]["end"] = json!(token["span"]["end"].as_f64().unwrap());
    let emitted = meta_language::translation::emit_rust::emit_rust(&program).unwrap();
    assert!(emitted.text.contains("1 << 28"));
    let packet = encode_typed_program_links(&program).unwrap();
    assert_eq!(
        observe_in_javascript(&packet)["calls"],
        json!(["alpha", "beta", "alpha"])
    );
}
