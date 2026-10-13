//! Isolated paired draft; local checks never execute the native compiler.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::javascript::parse_javascript_with_scalar_loop_model;
use meta_language::translation::rust::{parse_rust, parse_rust_with_scalar_loop_model};
use meta_language::{
    LinkNetwork, create_typed_program_links_with_javascript_scalar_loop_model,
    create_typed_program_links_with_rust_scalar_loop_model, decode_typed_program_links,
    emit_typed_program_links,
};
use serde_json::Value;
use std::fmt::Write;
use std::fs;
use std::process::Command;

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-scalar-loops.lino"
    ))
    .expect("the scalar loop fixture is structured Links data")
}
fn literal(value: &Value) -> String {
    if let Some(value) = value.as_bool() {
        return value.to_string();
    }
    match value.as_str().unwrap() {
        "NaN" => "f64::NAN".into(),
        "Infinity" => "f64::INFINITY".into(),
        "-Infinity" => "f64::NEG_INFINITY".into(),
        value => format!("{value}f64"),
    }
}
#[test]
fn complete_original_names_and_ordered_state_replay_through_both_frontends_and_archives() {
    let data = fixture();
    let model = data["model"].as_str().unwrap();
    assert_eq!(data["cases"].as_array().unwrap().len(), 4);
    assert_eq!(data["actualSource"].as_str().unwrap().len(), 512);
    let mut caller = String::new();
    let mut variant = 0;
    for case in data["cases"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap();
        let name = case["name"].as_str().unwrap();
        let surface = parse_rust_with_scalar_loop_model(source, model).unwrap();
        assert_eq!(serde_json::to_value(&surface).unwrap(), case["surface"]);
        let checked = check_program(&surface).unwrap();
        assert_eq!(
            emit_javascript(&checked).unwrap().definitions.join("\n\n"),
            case["javaScriptDefinitions"].as_str().unwrap()
        );
        let definitions = emit_rust(&checked).unwrap().definitions.join("\n\n");
        assert_eq!(definitions, case["rustDefinitions"].as_str().unwrap());
        let java_script = case["javaScriptDefinitions"].as_str().unwrap();
        let reverse_surface = parse_javascript_with_scalar_loop_model(java_script, model).unwrap();
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
            &parse_rust_with_scalar_loop_model(&format!("{definitions}\nfn main(){{}}"), model)
                .unwrap(),
        )
        .unwrap();
        let replay_definitions = emit_rust(&replay).unwrap().definitions.join("\n\n");
        for (language, input) in [("Rust", source), ("JavaScript", java_script)] {
            let provenance = serde_json::json!({"caller":"native scalar loop fixture"});
            let imported = if language == "Rust" {
                create_typed_program_links_with_rust_scalar_loop_model(
                    input,
                    language,
                    model,
                    Some(&provenance),
                )
            } else {
                create_typed_program_links_with_javascript_scalar_loop_model(
                    input,
                    language,
                    model,
                    Some(&provenance),
                )
            }
            .unwrap();
            let network = LinkNetwork::from_lino(&imported.network.to_lino()).unwrap();
            let restored = decode_typed_program_links(&network, imported.root).unwrap();
            let javascript = emit_javascript(&restored).unwrap().text;
            assert_eq!(
                emit_typed_program_links(&network, imported.root, "JavaScript")
                    .unwrap()
                    .code,
                javascript
            );
            assert!(
                emit_typed_program_links(&network, imported.root, "Rust")
                    .unwrap()
                    .code
                    .contains(&format!("pub fn {name}("))
            );
        }
        for code in [
            source,
            definitions.as_str(),
            replay_definitions.as_str(),
            reverse_definitions.as_str(),
        ] {
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
                if observation["expected"].is_boolean() {
                    writeln!(&mut caller, "assert_eq!({name}({arguments}),{expected});").unwrap();
                } else if observation["expected"] == "NaN" {
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
    let directory = TemporaryDirectory::new("scalar-loop-observations");
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
        "native loop oracle compilation failed\n{}\n{}",
        compiled.stdout, compiled.stderr
    );
    let observed = run(&mut Command::new(&executable));
    assert!(
        observed.ok,
        "native loop oracle assertions failed\n{}\n{}",
        observed.stdout, observed.stderr
    );
}

#[test]
fn original_default_and_domain_refusals_stay_explicit() {
    let data = fixture();
    let model = data["model"].as_str().unwrap();
    let source = format!("{}\nfn main(){{}}", data["actualSource"].as_str().unwrap());
    let error = parse_rust(&source).unwrap_err();
    assert_eq!(
        error.reason,
        "mutable parameter: mutation is outside the portable core"
    );
    assert_eq!(
        error.span,
        Some(meta_language::translation::Span::new(54, 57))
    );
    assert!(
        create_typed_program_links_with_rust_scalar_loop_model(
            "function keep(){return 1;}",
            "JavaScript",
            model,
            None
        )
        .is_err()
    );
    assert!(
        create_typed_program_links_with_javascript_scalar_loop_model(&source, "Rust", model, None)
            .is_err()
    );
    let unknown = parse_rust_with_scalar_loop_model(&source, "another model").unwrap_err();
    assert_eq!(
        unknown.reason,
        "Rust scalar loop model: select the legacy model or closed primitive scalar loop 1"
    );
    assert_eq!(unknown.span, None);
    assert!(
        parse_javascript_with_scalar_loop_model("function keep(){return 1;}", "another model")
            .is_err()
    );
}

#[test]
fn authentic_emitted_helper_refuses_current_and_future_math_lexical_bindings() {
    let data = fixture();
    let model = data["model"].as_str().unwrap();
    let negatives = data["javaScriptLexicalRefusals"].as_array().unwrap();
    assert_eq!(negatives.len(), 2);
    for case in negatives {
        let source = case["source"].as_str().unwrap();
        let error = parse_javascript_with_scalar_loop_model(source, model).unwrap_err();
        assert_eq!(error.reason, case["reason"].as_str().unwrap());
        assert_eq!(serde_json::to_value(error.span).unwrap(), case["span"]);
        assert!(
            create_typed_program_links_with_javascript_scalar_loop_model(
                source,
                "JavaScript",
                model,
                None
            )
            .is_err()
        );
    }
}

#[test]
fn owned_structured_module_relocation_checks_actual_function_and_namespace_bindings() {
    let data = fixture();
    let row = &data["qualifiedLoopGraph"];
    let network = LinkNetwork::from_lino(row["network"].as_str().unwrap()).unwrap();
    let root = meta_language::LinkId::from_u64(row["root"].as_str().unwrap().parse().unwrap());
    let program = decode_typed_program_links(&network, root).unwrap();
    let Some(meta_language::translation::ir::Decl::Fn(function)) =
        program.declaration("nested.swap")
    else {
        panic!("the actual qualified loop declaration exists");
    };
    assert_eq!(function.full_name, "nested.swap");
    assert_eq!(function.module_path, ["nested"]);
    assert_eq!(function.source_loop.as_ref().unwrap().source_name, "swap");
    assert_eq!(
        emit_javascript(&program).unwrap().definitions.join("\n\n"),
        row["javaScriptDefinitions"].as_str().unwrap()
    );
    assert_eq!(
        emit_rust(&program).unwrap().definitions.join("\n\n"),
        row["rustDefinitions"].as_str().unwrap()
    );
    assert_eq!(
        emit_typed_program_links(&network, root, "Rust")
            .unwrap()
            .code,
        emit_rust(&program).unwrap().text
    );
    for altered in ["other.swap", "swap"] {
        let mut forged = program.clone();
        let meta_language::translation::ir::Decl::Fn(function) = &mut forged.declarations[0] else {
            unreachable!();
        };
        function.full_name = altered.into();
        assert!(meta_language::encode_typed_program_links(&forged).is_err());
    }
    let syntax = format!(
        "{}\nfn main(){{}}",
        row["rustDefinitions"].as_str().unwrap()
    );
    let refused =
        parse_rust_with_scalar_loop_model(&syntax, data["model"].as_str().unwrap()).unwrap_err();
    assert!(
        refused
            .reason
            .contains("the selected scalar state model requires a flat function")
    );
}

#[test]
fn a_parameter_capturing_its_continuation_declaration_requires_an_anchored_reference() {
    let data = fixture();
    let model = data["model"].as_str().unwrap();
    for row in data["bindingCaptureRefusals"].as_array().unwrap() {
        let language = row["language"].as_str().unwrap();
        let source = row["source"].as_str().unwrap();
        let checked = if language == "Rust" {
            parse_rust_with_scalar_loop_model(source, model)
                .and_then(|surface| check_program(&surface))
        } else {
            parse_javascript_with_scalar_loop_model(source, model)
                .and_then(|surface| check_program(&surface))
        };
        let error = checked.unwrap_err();
        assert_eq!(error.reason, row["reason"].as_str().unwrap());
        assert_eq!(serde_json::to_value(error.span).unwrap(), row["span"]);
    }
}
