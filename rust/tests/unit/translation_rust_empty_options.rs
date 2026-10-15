//! Paired source owners and empty singleton observations; execution runs in CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::{Decl, FnDecl, Item, Node, Program};
use meta_language::translation::rust::{parse_rust, parse_rust_with_empty_option_source_model};
use meta_language::{
    LinkId, LinkMetadata, LinkNetwork, LinkType, SubstitutionRule,
    TypedProgramLinksEmissionOptions, create_typed_program_links,
    create_typed_program_links_with_rust_empty_option_source_model, decode_typed_program_links,
    emit_typed_program_links, emit_typed_program_links_with_options, encode_typed_program_links,
};
use serde_json::{Value, json};
use std::fmt::Write as _;
use std::fs;
use std::process::Command;

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-empty-options.lino"
    ))
    .unwrap()
}
fn archive(case: &Value) -> (LinkNetwork, LinkId) {
    (
        LinkNetwork::from_lino(case["links"].as_str().unwrap()).unwrap(),
        LinkId::from_u64(case["root"].as_u64().unwrap()),
    )
}
fn function<'a>(program: &'a Program, name: &str) -> &'a FnDecl {
    let Some(Decl::Fn(function)) = program.declaration(name) else {
        panic!("the actual empty function exists")
    };
    function
}
fn empty_type() -> meta_language::translation::types::Type {
    meta_language::translation::types::Type::Option {
        element: Box::new(meta_language::translation::types::Type::Float),
        absence: "undefined".into(),
    }
}
fn field_relation(network: &LinkNetwork, owner: LinkId, name: &str) -> [LinkId; 3] {
    let label = meta_language::typed_program_links_schema()["fields"][name]
        .as_str()
        .unwrap_or(name);
    network
        .links()
        .find_map(|link| match link.references() {
            [parent, field, child]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && *parent == owner
                    && network
                        .link(*field)
                        .and_then(|field| field.metadata().term())
                        == Some(label) =>
            {
                Some([*parent, *field, *child])
            }
            _ => None,
        })
        .unwrap()
}
fn null(network: &mut LinkNetwork) -> LinkId {
    network.insert_link(
        [],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_language("meta-language typed program")
            .with_term("null value"),
    )
}

#[test]
fn actual_owners_arguments_and_sites_survive_native_and_javascript_archives() {
    let data = fixture();
    let model = data["model"].as_str().unwrap();
    assert_eq!(data["cases"].as_array().unwrap().len(), 8);
    for case in data["cases"].as_array().unwrap() {
        let name = case["functionName"].as_str().unwrap();
        let source = case["source"].as_str().unwrap();
        let native = create_typed_program_links_with_rust_empty_option_source_model(
            source,
            "Rust",
            model,
            Some(&json!({"fixture":case["id"]})),
        )
        .unwrap();
        let (shared, shared_root) = archive(case);
        let shared_program = decode_typed_program_links(&shared, shared_root).unwrap();
        let native_network = LinkNetwork::from_lino(&native.network.to_lino()).unwrap();
        assert_eq!(
            decode_typed_program_links(&native_network, native.root).unwrap(),
            shared_program,
            "{} complete factory Program parity",
            case["id"]
        );
        for (network, root) in [(&shared, shared_root), (&native_network, native.root)] {
            let program = decode_typed_program_links(network, root).unwrap();
            let declaration = function(&program, name);
            assert_eq!(
                declaration.source_empty_option.as_ref().unwrap(),
                &case["expected"]
            );
            assert_eq!(declaration.ret, empty_type());
            assert_eq!(declaration.body.ty, empty_type());
            assert_eq!(
                declaration.body.node,
                Node::None {
                    absence: "undefined".into()
                }
            );
            let encoded = encode_typed_program_links(&program).unwrap();
            let restored = LinkNetwork::from_lino(&encoded.network.to_lino()).unwrap();
            assert_eq!(
                decode_typed_program_links(&restored, encoded.root).unwrap(),
                program
            );
            let definitions = emit_rust(&program).unwrap().definitions.join("\n\n");
            let replay = create_typed_program_links_with_rust_empty_option_source_model(
                &format!("{definitions}\nfn main(){{}}"),
                "Rust",
                model,
                None,
            )
            .unwrap();
            let replay = decode_typed_program_links(&replay.network, replay.root).unwrap();
            assert_eq!(
                function(&replay, case["emittedFunctionName"].as_str().unwrap()).ret,
                empty_type()
            );
        }
        let provenance = field_relation(&native_network, native.root, "provenance")[2];
        let profile = field_relation(&native_network, provenance, "source profile")[2];
        let selected =
            field_relation(&native_network, profile, "rust empty option source model")[2];
        assert_eq!(
            native_network
                .link(selected)
                .unwrap()
                .metadata()
                .definition(),
            Some(model)
        );
        let caller = field_relation(&native_network, provenance, "user provenance")[2];
        assert_eq!(
            native_network
                .link(field_relation(&native_network, caller, "fixture")[2])
                .unwrap()
                .metadata()
                .definition(),
            case["id"].as_str()
        );
    }
}

#[test]
fn original_default_refusals_and_each_selected_source_diagnostic_remain_exact() {
    let data = fixture();
    let source = data["cases"][0]["source"].as_str().unwrap();
    let legacy = parse_rust(source).unwrap_err();
    assert_eq!(legacy.kind.as_str(), "unsupported");
    assert_eq!(
        legacy.reason,
        "type Option<…>: generic types are outside the portable core"
    );
    assert_eq!(
        serde_json::to_value(legacy.span).unwrap(),
        json!({"start":12,"end":18})
    );
    assert_eq!(data["invalid"].as_array().unwrap().len(), 32);
    for case in data["invalid"].as_array().unwrap() {
        let failure = parse_rust_with_empty_option_source_model(
            case["source"].as_str().unwrap(),
            data["model"].as_str().unwrap(),
        )
        .and_then(|surface| check_program(&surface))
        .unwrap_err();
        assert_eq!(
            failure.kind.as_str(),
            case["kind"].as_str().unwrap(),
            "{}",
            case["id"]
        );
        assert_eq!(
            failure.reason,
            case["reason"].as_str().unwrap(),
            "{}",
            case["id"]
        );
        assert_eq!(
            serde_json::to_value(failure.span).unwrap(),
            case["span"],
            "{}",
            case["id"]
        );
    }
    let unknown = parse_rust_with_empty_option_source_model(source, "unknown").unwrap_err();
    assert_eq!(
        unknown.reason,
        "Rust empty option source model: select the legacy model or closed empty primitive Rust option 1"
    );
    assert!(unknown.span.is_none());
    assert_eq!(
        create_typed_program_links_with_rust_empty_option_source_model(
            "function empty(){return undefined;}",
            "JavaScript",
            data["model"].as_str().unwrap(),
            None
        )
        .unwrap_err()
        .message,
        "a Rust empty option source model requires Rust source"
    );
}

#[test]
fn malformed_source_free_facts_and_actual_raw_nulls_refuse_before_emission() {
    let data = fixture();
    let (network, root) = archive(&data["cases"][0]);
    let original = decode_typed_program_links(&network, root).unwrap();
    for (field, replacement) in [
        ("model", json!("unknown")),
        ("sourceName", json!("other")),
        ("construction", Value::Null),
        ("result", Value::Null),
        ("namespace", json!({})),
        ("parameters", json!([{}])),
    ] {
        let mut forged = original.clone();
        let Decl::Fn(function) = &mut forged.declarations[0] else {
            unreachable!()
        };
        function.source_empty_option.as_mut().unwrap()[field] = replacement;
        assert!(encode_typed_program_links(&forged).is_err(), "{field}");
    }
    for role in ["fn", "proof", "owner", "main", "program"] {
        let (mut network, root) = archive(&data["cases"][0]);
        let program = network.link(root).unwrap().references()[0];
        let directory = field_relation(&network, program, "declarations")[2];
        let pair = network.link(directory).unwrap().references()[0];
        let function = network.link(pair).unwrap().references()[1];
        let proof = field_relation(&network, function, "sourceEmptyOption")[2];
        let result = field_relation(&network, proof, "result")[2];
        let owner = field_relation(&network, result, "owner")[2];
        let main = field_relation(&network, program, "main")[2];
        let (parent, field) = match role {
            "fn" => (function, "sourceEmptyOption"),
            "proof" => (proof, "construction"),
            "owner" => (owner, "source path"),
            "main" => (main, "span"),
            _ => (program, "main"),
        };
        let old = field_relation(&network, parent, field);
        let empty = null(&mut network);
        let updated =
            network.apply_substitution(&SubstitutionRule::new(old, [old[0], old[1], empty]));
        assert_eq!(updated.updated().len(), 1);
        let restored = LinkNetwork::from_lino(&network.to_lino()).unwrap();
        assert!(
            decode_typed_program_links(&restored, root).is_err(),
            "{role}"
        );
        for target in ["Rust", "JavaScript"] {
            assert!(
                emit_typed_program_links(&restored, root, target).is_err(),
                "{role}:{target}"
            );
        }
    }
    let (mut network, root) = archive(&data["cases"][0]);
    let program = network.link(root).unwrap().references()[0];
    let main = field_relation(&network, program, "main")[2];
    let empty = null(&mut network);
    network.insert_field(main, "borrowEscape", empty);
    let restored = LinkNetwork::from_lino(&network.to_lino()).unwrap();
    assert!(decode_typed_program_links(&restored, root).is_err());
}

#[test]
fn actual_javascript_to_rust_definitions_and_complete_javascript_replays_execute_in_ci() {
    let data = fixture();
    let javascript =
        create_typed_program_links("function empty(){return undefined;}", "JavaScript").unwrap();
    let original = decode_typed_program_links(&javascript.network, javascript.root).unwrap();
    let actual = emit_rust(&original).unwrap().definitions.join("\n\n");
    assert_eq!(
        actual,
        "pub fn empty() -> Option<f64> {\n    None::<f64>\n}"
    );
    let failure = parse_rust(&actual).unwrap_err();
    assert_eq!(
        failure.reason,
        "type Option<…>: generic types are outside the portable core"
    );
    assert_eq!(
        serde_json::to_value(failure.span).unwrap(),
        json!({"start":18,"end":24})
    );
    let model = data["model"].as_str().unwrap();
    assert_eq!(
        parse_rust_with_empty_option_source_model(&actual, model)
            .unwrap_err()
            .reason,
        "a Rust program needs fn main"
    );
    let mut packet = create_typed_program_links_with_rust_empty_option_source_model(
        &format!("{actual}\nfn main(){{}}"),
        "Rust",
        model,
        None,
    )
    .unwrap();
    let options = TypedProgramLinksEmissionOptions {
        java_script_scoped_binding_emission_model: Some(
            "owned JavaScript scoped bindings 1".into(),
        ),
        ..TypedProgramLinksEmissionOptions::default()
    };
    for cycle in 0..4 {
        let restored = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
        let code =
            emit_typed_program_links_with_options(&restored, packet.root, "JavaScript", &options)
                .unwrap()
                .code;
        let observed=run(Command::new("node").arg("-e").arg(format!("{code}\nif(empty()!==undefined)throw new Error('wrong empty value');console.log('empty');")));
        assert!(
            observed.ok,
            "complete JavaScript replay {cycle} failed\n{}\n{}",
            observed.stdout, observed.stderr
        );
        assert_eq!(observed.stdout, "empty\n");
        packet = create_typed_program_links(&code, "JavaScript").unwrap();
        assert_eq!(
            function(
                &decode_typed_program_links(&packet.network, packet.root).unwrap(),
                "empty"
            )
            .ret,
            empty_type()
        );
    }
}

#[test]
fn singleton_binding_rename_and_authentic_original_emitted_reverse_sources_execute_in_ci() {
    let data = fixture();
    let model = data["model"].as_str().unwrap();
    let mut batch = String::new();
    let mut assertions = String::new();
    for (index, case) in data["cases"].as_array().unwrap().iter().enumerate() {
        let source = case["source"].as_str().unwrap();
        let (network, root) = archive(case);
        let program = decode_typed_program_links(&network, root).unwrap();
        let emitted = emit_rust(&program).unwrap().definitions.join("\n\n");
        let reverse = check_program(
            &parse_rust_with_empty_option_source_model(&format!("{emitted}\nfn main(){{}}"), model)
                .unwrap(),
        )
        .unwrap();
        let reverse = emit_rust(&reverse).unwrap().definitions.join("\n\n");
        for (variant, source, name) in [
            ("original", source, case["functionName"].as_str().unwrap()),
            (
                "emitted",
                emitted.as_str(),
                case["emittedFunctionName"].as_str().unwrap(),
            ),
            (
                "reverse",
                reverse.as_str(),
                case["emittedFunctionName"].as_str().unwrap(),
            ),
        ] {
            assert_ne!(name, "observe");
            writeln!(&mut batch, "mod case_{index}_{variant}{{{source}\npub(super) fn observe(){{assert!({name}().is_none());}}}}").unwrap();
            writeln!(&mut assertions, "case_{index}_{variant}::observe();").unwrap();
        }
    }
    let (network, root) = archive(&data["cases"][0]);
    let mut renamed = decode_typed_program_links(&network, root).unwrap();
    let Decl::Fn(entry) = &mut renamed.declarations[0] else {
        unreachable!()
    };
    entry.name = "vacant".into();
    entry.full_name = "vacant".into();
    let proof = entry.source_empty_option.as_mut().unwrap();
    proof["sourceName"] = json!("vacant");
    proof["namespace"]["declarationNames"] = json!(["vacant"]);
    renamed.items = vec![Item::Decl {
        full_name: "vacant".into(),
    }];
    let encoded = encode_typed_program_links(&renamed).unwrap();
    let restored = LinkNetwork::from_lino(&encoded.network.to_lino()).unwrap();
    let changed = decode_typed_program_links(&restored, encoded.root).unwrap();
    assert!(changed.declaration("vacant").is_some());
    assert!(changed.declaration("empty").is_none());
    let emitted = emit_rust(&changed).unwrap().definitions.join("\n\n");
    writeln!(&mut batch, "mod renamed{{{emitted}}}").unwrap();
    writeln!(&mut assertions, "assert!(renamed::vacant().is_none());").unwrap();
    let code = emit_typed_program_links(&restored, encoded.root, "JavaScript")
        .unwrap()
        .code;
    let observed=run(Command::new("node").arg("-e").arg(format!("{code}\nif(typeof empty!=='undefined'||vacant()!==undefined)throw new Error('wrong owned binding');console.log('vacant');")));
    assert!(
        observed.ok,
        "renamed binding execution failed\n{}\n{}",
        observed.stdout, observed.stderr
    );
    assert_eq!(observed.stdout, "vacant\n");
    let directory = TemporaryDirectory::new("rust-empty-option-source");
    let source = directory.0.join("empty.rs");
    let executable = directory.0.join("empty");
    fs::write(
        &source,
        format!("{batch}\n#[test] fn exact_empty_observations(){{{assertions}}}\n"),
    )
    .unwrap();
    let compiled = run(Command::new("rustc")
        .arg("--edition=2024")
        .arg("--test")
        .arg(&source)
        .arg("-o")
        .arg(&executable));
    assert!(
        compiled.ok,
        "empty source compilation failed\n{}\n{}",
        compiled.stdout, compiled.stderr
    );
    let observed = run(Command::new(&executable).args(["--exact", "exact_empty_observations"]));
    assert!(
        observed.ok,
        "empty source observations failed\n{}\n{}",
        observed.stdout, observed.stderr
    );
    assert!(observed.stdout.contains("1 passed"), "{}", observed.stdout);
}
