//! Paired declaration-directed loop draft; native execution belongs to CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::javascript::parse_javascript_with_scalar_loop_model;
use meta_language::translation::rust::parse_rust_with_scalar_loop_model;
use meta_language::{
    LinkNetwork, create_typed_program_links_with_javascript_scalar_loop_model,
    create_typed_program_links_with_rust_scalar_loop_model, decode_typed_program_links,
    emit_typed_program_links, encode_typed_program_links,
};
use serde_json::Value;
use std::fmt::Write;
use std::fs;
use std::process::Command;

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-scalar-loop-declarations.lino"
    ))
    .expect("declaration-directed loop fixtures are structured Links data")
}
fn literal(value: &Value) -> String {
    match value.as_str().unwrap() {
        "NaN" => "f64::NAN".into(),
        "Infinity" => "f64::INFINITY".into(),
        "-Infinity" => "f64::NEG_INFINITY".into(),
        value => format!("{value}f64"),
    }
}
#[test]
fn actual_captured_names_and_emitted_definitions_replay_with_owned_targets() {
    let data = fixture();
    let model = data["model"].as_str().unwrap();
    assert_eq!(data["cases"].as_array().unwrap().len(), 6);
    let mut caller = String::new();
    let mut variant = 0;
    for case in data["cases"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap();
        let language = case["language"].as_str().unwrap();
        let name = case["name"].as_str().unwrap();
        let surface = if language == "Rust" {
            parse_rust_with_scalar_loop_model(source, model)
        } else {
            parse_javascript_with_scalar_loop_model(source, model)
        }
        .unwrap();
        assert_eq!(serde_json::to_value(&surface).unwrap(), case["surface"]);
        let program = check_program(&surface).unwrap();
        let definitions = emit_rust(&program).unwrap().definitions.join("\n\n");
        assert_eq!(definitions, case["rustDefinitions"].as_str().unwrap());
        let javascript = emit_javascript(&program).unwrap().definitions.join("\n\n");
        assert_eq!(javascript, case["javaScriptDefinitions"].as_str().unwrap());
        let reverse_surface = parse_javascript_with_scalar_loop_model(&javascript, model).unwrap();
        assert_eq!(
            serde_json::to_value(&reverse_surface).unwrap(),
            case["javaScriptReverseSurface"]
        );
        let reverse = check_program(&reverse_surface).unwrap();
        let reverse_definitions = emit_rust(&reverse).unwrap().definitions.join("\n\n");
        assert_eq!(
            reverse_definitions,
            case["reverseRustDefinitions"].as_str().unwrap()
        );
        let replay = check_program(
            &parse_rust_with_scalar_loop_model(
                &format!("{reverse_definitions}\nfn main(){{}}"),
                model,
            )
            .unwrap(),
        )
        .unwrap();
        let replay_definitions = emit_rust(&replay).unwrap().definitions.join("\n\n");
        let provenance = serde_json::json!({"caller":"declaration target test"});
        let imported = if language == "Rust" {
            create_typed_program_links_with_rust_scalar_loop_model(
                source,
                language,
                model,
                Some(&provenance),
            )
        } else {
            create_typed_program_links_with_javascript_scalar_loop_model(
                source,
                language,
                model,
                Some(&provenance),
            )
        }
        .unwrap();
        let network = LinkNetwork::from_lino(&imported.network.to_lino()).unwrap();
        let restored = decode_typed_program_links(&network, imported.root).unwrap();
        assert_eq!(restored, program);
        assert_eq!(
            emit_typed_program_links(&network, imported.root, "JavaScript")
                .unwrap()
                .code,
            emit_javascript(&program).unwrap().text
        );
        let mut codes = vec![
            definitions.as_str(),
            reverse_definitions.as_str(),
            replay_definitions.as_str(),
        ];
        if language == "Rust" {
            codes.push(source);
        }
        for code in codes {
            writeln!(
                &mut caller,
                "mod case_{variant} {{\n{code}\n#[test] fn exact_scalar_observations() {{"
            )
            .unwrap();
            for observation in case["observations"].as_array().unwrap() {
                let arguments = observation["arguments"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(literal)
                    .collect::<Vec<_>>()
                    .join(",");
                let expected = literal(&observation["expected"]);
                if observation["expected"] == "NaN" {
                    writeln!(&mut caller, "assert!({name}({arguments}).is_nan());").unwrap();
                } else {
                    writeln!(
                        &mut caller,
                        "assert_eq!({name}({arguments}).to_bits(),({expected}).to_bits());"
                    )
                    .unwrap();
                }
            }
            caller.push_str("}\n}\n");
            variant += 1;
        }
    }
    let directory = TemporaryDirectory::new("scalar-loop-declaration-observations");
    let source = directory.0.join("observe.rs");
    let executable = directory.0.join("observe");
    fs::write(&source, caller).unwrap();
    let compiled = run(Command::new("rustc")
        .arg("--edition=2024")
        .arg("--test")
        .arg(&source)
        .arg("-o")
        .arg(&executable));
    assert!(
        compiled.ok,
        "native declaration loop compilation failed\n{}\n{}",
        compiled.stdout, compiled.stderr
    );
    let observed = run(&mut Command::new(&executable));
    assert!(
        observed.ok,
        "native declaration loop assertions failed\n{}\n{}",
        observed.stdout, observed.stderr
    );
}
#[test]
fn legacy_capture_refusal_and_absolute_surface_target_checks_stay_distinct() {
    let data = fixture();
    let model = data["model"].as_str().unwrap();
    for entry in data["legacyCaptureRefusals"].as_array().unwrap() {
        let source = entry["source"].as_str().unwrap();
        let surface = if entry["language"] == "Rust" {
            parse_rust_with_scalar_loop_model(source, data["legacyModel"].as_str().unwrap())
        } else {
            parse_javascript_with_scalar_loop_model(source, data["legacyModel"].as_str().unwrap())
        }
        .unwrap();
        let error = check_program(&surface).unwrap_err();
        assert_eq!(error.reason, entry["reason"].as_str().unwrap());
        assert_eq!(serde_json::to_value(error.span).unwrap(), entry["span"]);
    }
    let source = data["cases"][0]["source"].as_str().unwrap();
    let original = parse_rust_with_scalar_loop_model(source, model).unwrap();
    for path in [
        serde_json::json!(["run"]),
        serde_json::json!(["self", "run"]),
        serde_json::json!(["crate"]),
        serde_json::json!(["crate", "other"]),
        serde_json::json!(["crate", "nested", "run"]),
        serde_json::json!(["crate.run"]),
    ] {
        let mut surface = serde_json::to_value(&original).unwrap();
        visit(&mut surface, &mut |node| {
            if node["k"] == "app" {
                node["fn"]["path"] = path.clone();
            }
        });
        let forged = serde_json::from_value(surface).unwrap();
        let error = check_program(&forged).unwrap_err();
        assert!(error.reason.contains("owned tail continuation"));
    }
}
fn visit(value: &mut Value, callback: &mut impl FnMut(&mut Value)) {
    callback(value);
    match value {
        Value::Array(values) => {
            for child in values {
                visit(child, callback);
            }
        }
        Value::Object(values) => {
            for (key, child) in values {
                if !["type", "domain", "span"].contains(&key.as_str()) {
                    visit(child, callback);
                }
            }
        }
        _ => {}
    }
}
#[test]
fn actual_qualified_owner_path_survives_raw_codecs_and_rejects_forged_scope() {
    let data = fixture();
    let row = &data["qualifiedLoopGraph"];
    let network = LinkNetwork::from_lino(row["network"].as_str().unwrap()).unwrap();
    let root = meta_language::LinkId::from_u64(row["root"].as_str().unwrap().parse().unwrap());
    let program = decode_typed_program_links(&network, root).unwrap();
    assert_eq!(
        emit_javascript(&program).unwrap().definitions.join("\n\n"),
        row["javaScriptDefinitions"].as_str().unwrap()
    );
    let encoded = encode_typed_program_links(&program).unwrap();
    let reloaded = LinkNetwork::from_lino(&encoded.network.to_lino()).unwrap();
    assert_eq!(
        decode_typed_program_links(&reloaded, encoded.root).unwrap(),
        program
    );
    for path in [vec!["run"], vec!["other", "run"], vec!["nested", "other"]] {
        let mut forged = program.clone();
        let meta_language::translation::ir::Decl::Fn(entry) = &mut forged.declarations[0] else {
            panic!("fixture is an owned loop function")
        };
        entry
            .source_loop
            .as_mut()
            .unwrap()
            .continuation
            .as_mut()
            .unwrap()
            .path = path.into_iter().map(str::to_owned).collect();
        assert!(encode_typed_program_links(&forged).is_err());
    }
}

fn field_relation(
    network: &LinkNetwork,
    parent: meta_language::LinkId,
    label: &str,
) -> [meta_language::LinkId; 3] {
    network
        .links()
        .find_map(|link| match link.references() {
            [owner, name, child]
                if link.metadata().link_type() == Some(meta_language::LinkType::Field)
                    && *owner == parent
                    && network.link(*name).and_then(|name| name.metadata().term())
                        == Some(label) =>
            {
                Some([*owner, *name, *child])
            }
            _ => None,
        })
        .expect("the exact owned field exists")
}
#[test]
fn explicit_null_continuation_cannot_normalize_to_legacy_omission() {
    let data = fixture();
    for selected in [
        data["model"].as_str().unwrap(),
        data["legacyModel"].as_str().unwrap(),
    ] {
        let source = if selected == data["model"].as_str().unwrap() {
            data["cases"][0]["source"].as_str().unwrap()
        } else {
            "fn plain(mut value:f64,limit:f64)->f64{loop{return if value<limit{(value,)=(value+1f64,);continue;}else{value};}}fn main(){}"
        };
        let packet =
            create_typed_program_links_with_rust_scalar_loop_model(source, "Rust", selected, None)
                .unwrap();
        let mut network = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
        let owner = network
            .links()
            .find_map(|link| match link.references() {
                [parent, label, _]
                    if link.metadata().link_type() == Some(meta_language::LinkType::Field)
                        && network
                            .link(*label)
                            .and_then(|label| label.metadata().term())
                            == Some("source loop proof") =>
                {
                    Some(*parent)
                }
                _ => None,
            })
            .unwrap();
        let proof = field_relation(&network, owner, "source loop proof")[2];
        let schema = meta_language::typed_program_links_schema();
        let null = network.insert_link(
            [],
            meta_language::LinkMetadata::new()
                .with_link_type(meta_language::LinkType::Semantic)
                .with_named(true)
                .with_language(schema["language"].as_str().unwrap())
                .with_term(schema["scalars"]["null"].as_str().unwrap()),
        );
        if selected == data["model"].as_str().unwrap() {
            let original = field_relation(&network, proof, "continuation");
            let updated = network.apply_substitution(&meta_language::SubstitutionRule::new(
                original,
                [original[0], original[1], null],
            ));
            assert_eq!(updated.updated().len(), 1);
        } else {
            network.insert_field(proof, "continuation", null);
        }
        let restored = LinkNetwork::from_lino(&network.to_lino()).unwrap();
        assert!(decode_typed_program_links(&restored, packet.root).is_err());
    }
}

#[test]
fn continuation_root_path_and_unknown_raw_fields_cannot_disappear() {
    let data = fixture();
    let row = &data["qualifiedLoopGraph"];
    let root = meta_language::LinkId::from_u64(row["root"].as_str().unwrap().parse().unwrap());
    for field in ["root", "path", "extra"] {
        let mut network = LinkNetwork::from_lino(row["network"].as_str().unwrap()).unwrap();
        let proof = network
            .links()
            .find_map(|link| match link.references() {
                [_, label, child]
                    if link.metadata().link_type() == Some(meta_language::LinkType::Field)
                        && network
                            .link(*label)
                            .and_then(|label| label.metadata().term())
                            == Some("source loop proof") =>
                {
                    Some(*child)
                }
                _ => None,
            })
            .unwrap();
        let continuation = field_relation(&network, proof, "continuation")[2];
        let schema = meta_language::typed_program_links_schema();
        let null = network.insert_link(
            [],
            meta_language::LinkMetadata::new()
                .with_link_type(meta_language::LinkType::Semantic)
                .with_named(true)
                .with_language(schema["language"].as_str().unwrap())
                .with_term(schema["scalars"]["null"].as_str().unwrap()),
        );
        if field == "extra" {
            network.insert_field(continuation, field, null);
        } else {
            let original = field_relation(&network, continuation, field);
            let updated = network.apply_substitution(&meta_language::SubstitutionRule::new(
                original,
                [original[0], original[1], null],
            ));
            assert_eq!(updated.updated().len(), 1);
        }
        assert!(
            decode_typed_program_links(&LinkNetwork::from_lino(&network.to_lino()).unwrap(), root)
                .is_err()
        );
    }
}
