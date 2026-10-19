//! Shared Unit-call archives, actual executable artifacts and source-free edits.
//! Native compilation and toolchain execution run only in CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::{Decl, Effect, Item, Node, Program};
use meta_language::translation::rust::{parse_rust, parse_rust_with_pure_unit_call_model};
use meta_language::{
    LinkId, LinkMetadata, LinkNetwork, LinkType, SubstitutionRule, TypedProgramLinks,
    create_typed_program_links, create_typed_program_links_with_rust_pure_unit_call_model,
    decode_typed_program_links, emit_typed_program_links, encode_typed_program_links,
};
use serde_json::{Value, json};
use std::fs;
use std::process::Command;

fn fixtures() -> [Value; 2] {
    [
        meta_language::read_structured_data_lino(include_str!(
            "../../../parity/fixtures/rust-pure-unit-calls.lino"
        ))
        .unwrap(),
        meta_language::read_structured_data_lino(include_str!(
            "../../../parity/fixtures/rust-public-pure-unit-calls.lino"
        ))
        .unwrap(),
    ]
}
fn archive(entry: &Value) -> TypedProgramLinks {
    TypedProgramLinks {
        network: LinkNetwork::from_lino(entry["links"].as_str().unwrap()).unwrap(),
        root: LinkId::from_u64(entry["root"].as_u64().unwrap()),
    }
}
fn checked(packet: &TypedProgramLinks) -> Program {
    let network = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    decode_typed_program_links(&network, packet.root).unwrap()
}
fn names(program: &Program) -> Value {
    json!(
        program
            .declarations
            .iter()
            .map(Decl::name)
            .collect::<Vec<_>>()
    )
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
                    panic!("each selected statement is a let")
                };
                let Node::Call { func, .. } = &value.node else {
                    panic!("each actual value calls its owner")
                };
                func.as_str()
            })
            .collect::<Vec<_>>()
    )
}
fn execute_javascript(code: &str) -> String {
    let result = run(Command::new("node")
        .arg("--input-type=module")
        .arg("-e")
        .arg(code));
    assert!(result.ok, "{}", result.stderr);
    result.stdout.trim_end_matches('\n').to_owned()
}
fn observe_calls(packet: &TypedProgramLinks) -> Value {
    let program = checked(packet);
    let target = emit_typed_program_links(&packet.network, packet.root, "JavaScript").unwrap();
    let script = format!(
        "const vm=require('node:vm');const context=vm.createContext({{console:{{log(){{}}}}}});vm.runInContext({},context);const calls=[];for(const name of {}){{if(typeof context[name]!=='function')throw Error('missing owned helper');context[name]=()=>{{calls.push(name);return null;}};}}vm.runInContext('main();',context);console.log(JSON.stringify(calls));",
        serde_json::to_string(&target.code).unwrap(),
        names(&program)
    );
    let result = run(Command::new("node").arg("-e").arg(script));
    assert!(result.ok, "{}", result.stderr);
    serde_json::from_str(result.stdout.trim()).unwrap()
}
fn compile_and_execute(code: &str) {
    let directory = TemporaryDirectory::new("pure-unit-calls");
    let source = directory.0.join("main.rs");
    let executable = directory.0.join("main");
    fs::write(&source, code).unwrap();
    let compiled = run(Command::new("rustc")
        .arg("--edition=2024")
        .arg(&source)
        .arg("-o")
        .arg(&executable));
    assert!(compiled.ok, "{}", compiled.stderr);
    let executed = run(&mut Command::new(&executable));
    assert!(executed.ok, "{}", executed.stderr);
    assert_eq!(executed.stdout, "");
}

#[test]
fn complete_private_and_public_source_archives_retain_real_calls_and_execute() {
    let data = fixtures();
    assert_eq!(data[0]["cases"].as_array().unwrap().len(), 12);
    assert_eq!(data[1]["cases"].as_array().unwrap().len(), 9);
    for fixture in &data {
        let model = fixture["model"].as_str().unwrap();
        for entry in fixture["cases"].as_array().unwrap() {
            let id = entry["id"].as_str().unwrap();
            let source = entry["source"].as_str().unwrap();
            let original = archive(entry);
            let from_javascript = checked(&original);
            let native = create_typed_program_links_with_rust_pure_unit_call_model(
                source, "Rust", model, None,
            )
            .unwrap();
            let program = checked(&native);
            assert_eq!(program, from_javascript, "{id}: actual paired IR");
            assert_eq!(names(&program), entry["names"], "{id}");
            assert_eq!(calls(&program), entry["calls"], "{id}");
            assert_eq!(
                program.source_pure_unit_calls,
                json!({"model":model,"language":"Rust"}).into()
            );
            assert_eq!(
                observe_calls(&original),
                entry["calls"],
                "{id}: actual ordered JavaScript execution"
            );
            let javascript =
                emit_typed_program_links(&native.network, native.root, "JavaScript").unwrap();
            assert_eq!(
                execute_javascript(&javascript.code),
                entry["output"].as_str().unwrap()
            );
            let rust = emit_typed_program_links(&native.network, native.root, "Rust").unwrap();
            compile_and_execute(&rust.code);
            assert!(
                !native
                    .network
                    .to_lino()
                    .contains(&meta_language::percent_encode_links_text(source)),
                "{id}: no saved source payload"
            );
        }
    }
}

#[test]
fn five_actual_complete_javascript_definition_sets_replay_three_times() {
    let data = fixtures();
    let fixture = &data[1];
    let model = fixture["model"].as_str().unwrap();
    assert_eq!(fixture["generated"].as_array().unwrap().len(), 5);
    for entry in fixture["generated"].as_array().unwrap() {
        let original = entry["original"].as_str().unwrap();
        assert_eq!(
            execute_javascript(original),
            entry["output"].as_str().unwrap()
        );
        let input = create_typed_program_links(original, "JavaScript").unwrap();
        let emitted = emit_rust(&checked(&input)).unwrap();
        assert_eq!(
            emitted.definitions.join("\n\n"),
            entry["definitions"].as_str().unwrap()
        );
        assert_eq!(emitted.text, entry["fullEmittedRust"].as_str().unwrap());
        let mut packet = archive(entry);
        for _ in 0..3 {
            let program = checked(&packet);
            assert_eq!(names(&program), entry["names"]);
            assert_eq!(
                program.main.as_ref().unwrap().effects,
                [] as [meta_language::translation::ir::Effect; 0]
            );
            let javascript =
                emit_typed_program_links(&packet.network, packet.root, "JavaScript").unwrap();
            assert_eq!(
                execute_javascript(&javascript.code),
                entry["output"].as_str().unwrap()
            );
            let actual = emit_rust(&program).unwrap().definitions.join("\n\n");
            packet = create_typed_program_links_with_rust_pure_unit_call_model(
                &(actual + "\nfn main(){}"),
                "Rust",
                model,
                None,
            )
            .unwrap();
        }
        let full = entry["fullEmittedRust"].as_str().unwrap();
        assert!(
            parse_rust(full)
                .unwrap_err()
                .reason
                .contains("function without a result: ml_main")
        );
        assert!(
            parse_rust_with_pure_unit_call_model(full, model)
                .unwrap_err()
                .reason
                .contains("attributes require a separate compilation context proof")
        );
        let wrapper = &full[full.rfind("fn main()").unwrap()..];
        assert!(
            parse_rust(wrapper)
                .unwrap_err()
                .reason
                .contains("std::env::args")
        );
        assert!(parse_rust_with_pure_unit_call_model(wrapper, model).is_err());
    }
}

#[test]
fn all_sixty_source_refusals_preserve_the_shared_diagnostics_and_spans() {
    let data = fixtures();
    assert_eq!(data[0]["invalid"].as_array().unwrap().len(), 38);
    assert_eq!(data[1]["invalid"].as_array().unwrap().len(), 22);
    for fixture in &data {
        let model = fixture["model"].as_str().unwrap();
        for entry in fixture["invalid"].as_array().unwrap() {
            let id = entry["id"].as_str().unwrap();
            let result =
                parse_rust_with_pure_unit_call_model(entry["source"].as_str().unwrap(), model)
                    .and_then(|surface| check_program(&surface));
            let failure = result.unwrap_err();
            assert_eq!(
                failure.kind.as_str(),
                entry["kind"].as_str().unwrap(),
                "{id}"
            );
            assert_eq!(failure.reason, entry["reason"].as_str().unwrap(), "{id}");
            assert_eq!(
                serde_json::to_value(failure.span).unwrap(),
                entry["span"],
                "{id}"
            );
        }
    }
    assert!(
        create_typed_program_links_with_rust_pure_unit_call_model(
            "function empty(){}",
            "JavaScript",
            data[0]["model"].as_str().unwrap(),
            None
        )
        .is_err()
    );
    assert!(parse_rust_with_pure_unit_call_model("fn main(){}", "unknown").is_err());
    assert!(
        parse_rust("fn noop(){}fn main(){}")
            .unwrap_err()
            .reason
            .contains("function without a result: noop")
    );
    assert!(parse_rust("fn noop()->(){()}fn main(){noop();}").is_err());
    for entry in data[1]["javaScriptInvalid"].as_array().unwrap() {
        assert!(
            create_typed_program_links(entry["source"].as_str().unwrap(), "JavaScript").is_err()
        );
    }
}

fn field(network: &LinkNetwork, parent: LinkId, name: &str) -> (LinkId, LinkId) {
    let label = meta_language::typed_program_links_schema()["fields"][name]
        .as_str()
        .map_or_else(
            || {
                let mut text = String::new();
                for character in name.chars() {
                    if character.is_ascii_uppercase() {
                        text.push(' ');
                        text.push(character.to_ascii_lowercase());
                    } else {
                        text.push(character);
                    }
                }
                text
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
fn replace_scalar(network: &mut LinkNetwork, parent: LinkId, name: &str, value: &Value) {
    let term = if value.is_null() {
        "null value"
    } else if value.is_boolean() {
        "boolean value"
    } else if value.is_string() {
        "string value"
    } else {
        "number value"
    };
    let mut metadata = LinkMetadata::new()
        .with_link_type(LinkType::Semantic)
        .with_named(true)
        .with_language("meta-language typed program")
        .with_term(term);
    if !value.is_null() {
        metadata = metadata.with_definition(
            value
                .as_str()
                .map_or_else(|| value.to_string(), str::to_owned),
        );
    }
    let child = network.insert_link([], metadata);
    let existing = network.links().find_map(|link| match link.references() {
        [owner, key, _]
            if *owner == parent
                && link.metadata().link_type() == Some(LinkType::Field)
                && network.link(*key).and_then(|key| key.metadata().term()) == Some(name) =>
        {
            Some(link.id())
        }
        _ => None,
    });
    if let Some(relation) = existing {
        let refs = network.link(relation).unwrap().references().to_vec();
        let old: [LinkId; 3] = refs.try_into().unwrap();
        let report =
            network.apply_substitution(&SubstitutionRule::new(old, [old[0], old[1], child]));
        assert_eq!(report.updated().len(), 1);
    } else {
        network.insert_field(parent, name, child);
    }
}
fn rejects(packet: &TypedProgramLinks) {
    let network = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    assert!(decode_typed_program_links(&network, packet.root).is_err());
    for target in ["Rust", "JavaScript"] {
        assert!(emit_typed_program_links(&network, packet.root, target).is_err());
    }
}

#[test]
fn raw_null_unknown_fields_and_public_visibility_edits_refuse_before_deserialization() {
    let data = fixtures();
    for fixture in &data {
        let entry = fixture["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["id"].as_str().unwrap().ends_with("ordered-repeated"))
            .unwrap();
        for (role, name, value) in [
            ("program", "source pure unit calls", Value::Null),
            ("marker", "language", Value::Null),
            ("marker", "unknown", json!(false)),
            ("fn", "source pure unit", Value::Null),
            ("proof", "header form", Value::Null),
            ("proof", "body span", Value::Null),
            ("effect", "source pure unit call", Value::Null),
            ("statement", "path form", Value::Null),
            ("statement", "unknown", json!(false)),
            ("effect", "name", json!("alpha")),
            ("main", "sequential async", json!(false)),
            ("effect", "constant", json!(false)),
        ] {
            let mut packet = archive(entry);
            let network = &mut packet.network;
            let program = network.link(packet.root).unwrap().references()[0];
            let marker = field(network, program, "sourcePureUnitCalls").1;
            let main = field(network, program, "main").1;
            let effect = network
                .link(field(network, main, "effects").1)
                .unwrap()
                .references()[0];
            let statement = field(network, effect, "sourcePureUnitCall").1;
            let declaration = network
                .links()
                .find_map(|link| match link.references() {
                    [parent, key, _]
                        if network.link(*key).and_then(|key| key.metadata().term())
                            == Some("source pure unit") =>
                    {
                        Some(*parent)
                    }
                    _ => None,
                })
                .unwrap();
            let proof = field(network, declaration, "sourcePureUnit").1;
            let owner = match role {
                "program" => program,
                "marker" => marker,
                "fn" => declaration,
                "proof" => proof,
                "effect" => effect,
                "statement" => statement,
                "main" => main,
                _ => unreachable!(),
            };
            replace_scalar(network, owner, name, &value);
            rejects(&packet);
        }
    }
    let entry = &data[1]["cases"][1];
    for (role, name, value) in [
        ("proof", "visibility", Value::Null),
        ("visibility", "form", Value::Null),
        ("visibility", "span", Value::Null),
        ("visibility", "unsafe", json!(false)),
        ("span", "start", json!(1)),
        ("span", "end", json!(2)),
    ] {
        let mut packet = archive(entry);
        let network = &mut packet.network;
        let proof = network
            .links()
            .find_map(|link| match link.references() {
                [_, key, child]
                    if network.link(*key).and_then(|key| key.metadata().term())
                        == Some("source pure unit") =>
                {
                    Some(*child)
                }
                _ => None,
            })
            .unwrap();
        let visibility = field(network, proof, "visibility").1;
        let site = field(network, visibility, "span").1;
        replace_scalar(
            network,
            match role {
                "proof" => proof,
                "visibility" => visibility,
                _ => site,
            },
            name,
            &value,
        );
        rejects(&packet);
    }
}

#[test]
fn source_free_renaming_reordering_and_reference_retargeting_change_actual_calls() {
    let data = fixtures();
    for fixture in &data {
        let entry = fixture["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["id"].as_str().unwrap().ends_with("ordered-repeated"))
            .unwrap();
        let mut program = checked(&archive(entry));
        let Decl::Fn(function) = program
            .declarations
            .iter_mut()
            .find(|entry| entry.name() == "alpha")
            .unwrap()
        else {
            panic!("actual function")
        };
        function.name = "renamed".into();
        function.full_name = "renamed".into();
        for item in &mut program.items {
            if let Item::Decl { full_name } = item
                && full_name == "alpha"
            {
                *full_name = "renamed".into();
            }
        }
        for effect in &mut program.main.as_mut().unwrap().effects {
            if let Effect::Let { value, .. } = effect
                && let Node::Call { func, .. } = &mut value.node
                && func == "alpha"
            {
                *func = "renamed".into();
            }
        }
        let renamed = encode_typed_program_links(&program).unwrap();
        assert_eq!(
            observe_calls(&renamed),
            json!(["renamed", "beta", "renamed"])
        );
        program.main.as_mut().unwrap().effects.rotate_right(1);
        let Effect::Let { value, .. } = &mut program.main.as_mut().unwrap().effects[1] else {
            panic!("call statement")
        };
        let Node::Call { func, .. } = &mut value.node else {
            panic!("owned call")
        };
        *func = "beta".into();
        let changed = encode_typed_program_links(&program).unwrap();
        assert_eq!(observe_calls(&changed), json!(["renamed", "beta", "beta"]));
        let actual = emit_rust(&checked(&changed))
            .unwrap()
            .definitions
            .join("\n\n");
        let replay = create_typed_program_links_with_rust_pure_unit_call_model(
            &(actual + "\nfn main(){}"),
            "Rust",
            data[1]["model"].as_str().unwrap(),
            None,
        )
        .unwrap();
        assert!(checked(&replay).declaration("renamed").is_some());
    }
}

#[test]
fn labeled_span_offsets_survive_reordered_raw_records_and_reject_negative_zero() {
    let data = fixtures();
    for fixture in &data {
        let entry = &fixture["cases"][1];
        let mut packet = archive(entry);
        let network = &mut packet.network;
        let program = network.link(packet.root).unwrap().references()[0];
        let main = field(network, program, "main").1;
        let effect = network
            .link(field(network, main, "effects").1)
            .unwrap()
            .references()[0];
        let call = field(network, effect, "value").1;
        let (relation, original_span) = field(network, call, "span");
        let start = field(network, original_span, "start").1;
        let end = field(network, original_span, "end").1;
        let metadata = network.link(original_span).unwrap().metadata().clone();
        let reordered = network.insert_link([], metadata);
        network.insert_field(reordered, "end", end);
        network.insert_field(reordered, "start", start);
        let old: [LinkId; 3] = network
            .link(relation)
            .unwrap()
            .references()
            .try_into()
            .unwrap();
        assert_eq!(
            network
                .apply_substitution(&SubstitutionRule::new(old, [old[0], old[1], reordered]))
                .updated()
                .len(),
            1
        );
        assert_eq!(observe_calls(&packet), json!(["empty"]));
        for target in ["Rust", "JavaScript"] {
            assert!(emit_typed_program_links(&packet.network, packet.root, target).is_ok());
        }
        for spelling in ["-0", "-0.0", "9007199254740992"] {
            let mut malformed = archive(entry);
            let network = &mut malformed.network;
            let program = network.link(malformed.root).unwrap().references()[0];
            let main = field(network, program, "main").1;
            let site = field(network, main, "span").1;
            let relation = field(network, site, "start").0;
            let replacement = network.insert_link(
                [],
                LinkMetadata::new()
                    .with_link_type(LinkType::Semantic)
                    .with_named(true)
                    .with_language("meta-language typed program")
                    .with_term("number value")
                    .with_definition(spelling),
            );
            let old: [LinkId; 3] = network
                .link(relation)
                .unwrap()
                .references()
                .try_into()
                .unwrap();
            assert_eq!(
                network
                    .apply_substitution(&SubstitutionRule::new(old, [old[0], old[1], replacement]))
                    .updated()
                    .len(),
                1
            );
            rejects(&malformed);
        }
        for invalid in [
            json!(-0.0),
            json!(-1),
            json!(1.5),
            json!(9_007_199_254_740_992_u64),
        ] {
            let mut malformed = checked(&archive(entry));
            let Decl::Fn(function) = &mut malformed.declarations[0] else {
                panic!("actual helper")
            };
            function.source_pure_unit.as_mut().unwrap()["headerSpan"]["start"] = invalid;
            assert!(encode_typed_program_links(&malformed).is_err());
        }
        let mut maximum = checked(&archive(entry));
        maximum.main.as_mut().unwrap().span.as_mut().unwrap().end =
            usize::try_from(9_007_199_254_740_991_u64).unwrap();
        assert!(encode_typed_program_links(&maximum).is_ok());
    }
    let packet = create_typed_program_links(
        "/** @returns {number} */function negative(){return -0;}console.log(negative());",
        "JavaScript",
    )
    .unwrap();
    let program = checked(&packet);
    let Some(Decl::Fn(function)) = program.declaration("negative") else {
        panic!("actual primitive helper")
    };
    assert_eq!(serde_json::to_value(&function.body).unwrap()["value"], "-0");
    let code = emit_typed_program_links(&packet.network, packet.root, "JavaScript")
        .unwrap()
        .code;
    assert_eq!(
        execute_javascript(&(code + "\nconsole.log(Object.is(negative(),-0));")),
        "-0\ntrue"
    );
}

#[test]
fn numeric_source_span_aliases_preserve_owned_unit_proofs_and_complete_artifacts() {
    for fixture in fixtures() {
        let entry = &fixture["cases"][1];
        let original = archive(entry);
        let expected_rust = emit_typed_program_links(&original.network, original.root, "Rust")
            .unwrap()
            .code;
        let expected_javascript =
            emit_typed_program_links(&original.network, original.root, "JavaScript")
                .unwrap()
                .code;
        let mut roles = vec!["headerSpan", "bodySpan"];
        if fixture["model"] == "closed public pure Rust unit calls 1" {
            roles.push("visibility");
        }
        for role in roles {
            for suffix in [".0", "e0", "E+0", ".00e+0"] {
                let mut packet = archive(entry);
                let network = &mut packet.network;
                let proof = network
                    .links()
                    .find_map(|link| match link.references() {
                        [_, key, child]
                            if network.link(*key).and_then(|key| key.metadata().term())
                                == Some("source pure unit") =>
                        {
                            Some(*child)
                        }
                        _ => None,
                    })
                    .unwrap();
                let site = if role == "visibility" {
                    field(network, field(network, proof, "visibility").1, "span").1
                } else {
                    field(network, proof, role).1
                };
                for name in ["start", "end"] {
                    let (relation, value) = field(network, site, name);
                    let spelling = format!(
                        "{}{suffix}",
                        network
                            .link(value)
                            .unwrap()
                            .metadata()
                            .definition()
                            .unwrap()
                    );
                    let metadata = network
                        .link(value)
                        .unwrap()
                        .metadata()
                        .clone()
                        .with_definition(&spelling);
                    let replacement = network.insert_link([], metadata);
                    let old: [LinkId; 3] = network
                        .link(relation)
                        .unwrap()
                        .references()
                        .try_into()
                        .unwrap();
                    assert_eq!(
                        network
                            .apply_substitution(&SubstitutionRule::new(
                                old,
                                [old[0], old[1], replacement]
                            ))
                            .updated()
                            .len(),
                        1
                    );
                    assert_eq!(
                        network.link(replacement).unwrap().metadata().definition(),
                        Some(spelling.as_str())
                    );
                }
                let raw = LinkNetwork::from_lino(&network.to_lino()).unwrap();
                let program = decode_typed_program_links(&raw, packet.root).unwrap();
                assert_eq!(calls(&program), entry["calls"]);
                assert_eq!(
                    emit_typed_program_links(&raw, packet.root, "Rust")
                        .unwrap()
                        .code,
                    expected_rust,
                    "{role} {suffix}"
                );
                assert_eq!(
                    emit_typed_program_links(&raw, packet.root, "JavaScript")
                        .unwrap()
                        .code,
                    expected_javascript,
                    "{role} {suffix}"
                );
                assert!(encode_typed_program_links(&program).is_ok());
            }
        }
    }
}
