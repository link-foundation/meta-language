//! Deferred complete original/JavaScript/native products with independent types.
#[path = "../../src/translation/rust_displayed_string_interface_rules.rs"]
mod displayed_rules;
#[path = "../../src/translation/rust_standard_owned_call_rules.rs"]
mod owned_rules;
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::emit_rust::{RustEmissionOptions, emit_rust_with_options};
use meta_language::translation::ir::{Decl, FnDecl, Program};
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, TypedProgramLinksEmissionOptions,
    create_rust_source_owner_program_links, decode_link_network_binary, decode_typed_program_links,
    emit_typed_program_links, emit_typed_program_links_with_options, encode_link_network_binary,
};
use serde_json::Value;
use std::{fs, process::Command};

fn fixture() -> Value {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered source pointer width"
    );
    meta_language::project_data::read_project_data_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity/fixtures/rust-displayed-string-interface.lino"),
    )
    .unwrap()
}
fn function<'a>(program: &'a Program, name: &str) -> &'a FnDecl {
    let Some(Decl::Fn(function)) = program.declaration(name) else {
        panic!("actual retained function required")
    };
    function
}
fn emission_value(value: &meta_language::TypedProgramLinksEmission) -> Value {
    serde_json::json!({"code":value.code,"language":value.language.as_str(),"mappings":value.mappings,"assumptions":value.assumptions,"encodings":value.encodings,"theorems":value.theorems})
}
fn options(data: &Value) -> TypedProgramLinksEmissionOptions {
    TypedProgramLinksEmissionOptions {
        rust_source_header_emission_model: Some(data["model"].as_str().unwrap().to_owned()),
        ..TypedProgramLinksEmissionOptions::default()
    }
}

#[test]
fn every_display_selector_expression_field_and_complete_operand_has_premises() {
    let data = fixture();
    for row in data["selections"].as_array().unwrap() {
        assert_eq!(
            displayed_rules::accept_rust_displayed_string_interface_model(
                row["model"].as_str().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["expressions"].as_array().unwrap() {
        assert_eq!(
            displayed_rules::accept_rust_displayed_string_expression(row["kind"].as_str().unwrap()),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["fields"].as_array().unwrap() {
        assert_eq!(
            displayed_rules::accept_rust_displayed_string_expression_field(
                row["kind"].as_str().unwrap(),
                row["field"].as_str().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["operands"].as_array().unwrap() {
        assert_eq!(
            displayed_rules::accept_rust_primitive_display_operand(
                row["kind"].as_str().unwrap(),
                row["result"].as_str().unwrap(),
                row["console"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in data["ownedMethodResults"].as_array().unwrap() {
        assert_eq!(
            owned_rules::accept_rust_standard_string_owned_method_result(
                row["method"].as_str().unwrap(),
                row["kind"].as_str().unwrap(),
                row["model"].as_str().unwrap(),
                row["layers"].as_f64().unwrap(),
                row["proven"].as_bool().unwrap(),
                row["references"].as_bool().unwrap(),
                row["owner"].as_str().unwrap(),
                row["path"].as_str().unwrap(),
                row["children"].as_f64().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
}

#[test]
fn the_complete_previous_boolean_input_retains_its_original_interpretation() {
    let data = fixture();
    let row = &data["previousInterpretation"];
    let packet = create_rust_source_owner_program_links(row["source"].as_str().unwrap()).unwrap();
    let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
    let error = emit_rust_with_options(
        &program,
        RustEmissionOptions {
            source_header_model: row["model"].as_str(),
        },
    )
    .unwrap_err();
    assert!(error.reason.contains(row["reason"].as_str().unwrap()));
    let current = emit_typed_program_links_with_options(
        &packet.network,
        packet.root,
        "Rust",
        &options(&data),
    )
    .unwrap();
    assert!(current.code.contains(".to_string()"));
}

#[test]
fn repeated_formatting_fields_retain_one_complete_original_argument_call() {
    use meta_language::translation::ir::Node;
    let data = fixture();
    let row = data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| {
            row["name"]
                .as_str()
                .unwrap()
                .starts_with("repeated formatting")
        })
        .unwrap();
    let packet = create_rust_source_owner_program_links(row["source"].as_str().unwrap()).unwrap();
    let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
    let mut pending = vec![&function(&program, "render").body];
    let mut calls = 0;
    let mut visits = 0;
    while let Some(node) = pending.pop() {
        visits += 1;
        assert!(visits <= 100000);
        if matches!(&node.node,Node::Call{func,..}if func=="input") {
            calls += 1;
        }
        pending.extend(node.children());
    }
    assert_eq!(calls, 1);
}
fn views(packet: &meta_language::TypedProgramLinks) -> Vec<LinkNetwork> {
    vec![
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ]
}
#[test]
fn complete_source_products_keep_independent_scalar_function_types_and_observations() {
    let data = fixture();
    let produced = run(Command::new("node")
        .arg("../experiments/rust-records/displayed-string-interface.mjs")
        .current_dir(env!("CARGO_MANIFEST_DIR")));
    assert!(produced.ok, "{}\n{}", produced.stdout, produced.stderr);
    let javascript = meta_language::read_structured_data_lino(&produced.stdout).unwrap();
    let packet = create_rust_source_owner_program_links(data["source"].as_str().unwrap()).unwrap();
    let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
    let selected = emit_rust_with_options(
        &program,
        RustEmissionOptions {
            source_header_model: data["model"].as_str(),
        },
    )
    .unwrap();
    let expected = emit_typed_program_links_with_options(
        &packet.network,
        packet.root,
        "Rust",
        &options(&data),
    )
    .unwrap();
    assert_eq!(emission_value(&expected), javascript["emission"]);
    let shared = LinkNetwork::from_lino(javascript["links"].as_str().unwrap()).unwrap();
    let shared_root = LinkId::from_u64(javascript["root"].as_u64().unwrap());
    for (network, root) in std::iter::once((&shared, shared_root))
        .chain(views(&packet).iter().map(|network| (network, packet.root)))
    {
        let restored = decode_typed_program_links(network, root).unwrap();
        for entry in &program.declarations {
            assert_eq!(
                function(&restored, entry.full_name()).source_signature,
                function(&program, entry.full_name()).source_signature
            );
        }
        let actual =
            emit_typed_program_links_with_options(network, root, "Rust", &options(&data)).unwrap();
        assert_eq!(emission_value(&actual), emission_value(&expected));
    }
    let native_library = format!(
        "{}\n{}",
        selected.preludes.join("\n"),
        selected.definitions.join("\n")
    );
    assert_native_products(&data, &javascript, &native_library);
}

fn assert_native_products(data: &Value, javascript: &Value, native_library: &str) {
    let directory = TemporaryDirectory::new("retained-displayed-string-interface");
    // Each product is a crate root: its actual absolute crate paths must keep
    // their meaning. Batch all cases per product instead of nesting the code.
    for (product, library) in [
        ("javascript", javascript["nativeLibrary"].as_str().unwrap()),
        ("native", native_library),
    ] {
        let mut source = format!(
            "{library}\nmod original {{\n{}\n}}\n",
            data["source"].as_str().unwrap()
        );
        let mut observations = String::from("#[test]fn retained_scalar_observations(){\n");
        for (index, row) in data["cases"].as_array().unwrap().iter().enumerate() {
            assert_eq!(javascript["observations"][index]["value"], row["expected"]);
            assert_eq!(
                javascript["observations"][index]["unit"].as_bool().unwrap(),
                row["resultType"] == "()"
            );
            let module = row["module"].as_str().unwrap();
            for prefix in [format!("original::{module}"), module.to_owned()] {
                for check in row["checks"].as_array().unwrap() {
                    source.push_str(&check.as_str().unwrap().replace("CASE", &prefix));
                    source.push('\n');
                }
                observations.push_str(&format!(
                    "assert_eq!({prefix}::result().to_string(),{:?});\n",
                    row["expected"].as_str().unwrap()
                ));
            }
        }
        observations.push_str("}\n");
        source.push_str(&observations);
        let input = directory.0.join(format!("{product}.rs"));
        let binary = directory
            .0
            .join(format!("{product}{}", std::env::consts::EXE_SUFFIX));
        fs::write(&input, source).unwrap();
        let mut command = Command::new("rustc");
        command.args(["--edition", "2024", "--test"]);
        #[cfg(windows)]
        if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
            command.arg("-C").arg(format!("linker={linker}"));
        }
        let compiled = run(command.arg(&input).arg("-o").arg(&binary));
        assert!(
            compiled.ok,
            "{product}\n{}\n{}",
            compiled.stdout, compiled.stderr
        );
        let observed = run(Command::new(&binary).args(["--exact", "retained_scalar_observations"]));
        assert!(
            observed.ok,
            "{product}\n{}\n{}",
            observed.stdout, observed.stderr
        );
        assert!(observed.stdout.contains("1 passed"));
    }
}

#[test]
fn previous_interfaces_and_unrepresented_string_interpretations_keep_their_refusals() {
    let data = fixture();
    for row in data["cases"].as_array().unwrap() {
        let packet =
            create_rust_source_owner_program_links(row["source"].as_str().unwrap()).unwrap();
        let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
        let error = emit_rust_with_options(
            &program,
            RustEmissionOptions {
                source_header_model: Some("retained constructed Rust string function interface 1"),
            },
        )
        .unwrap_err();
        assert!(
            error
                .reason
                .contains("additional source interface interpretation")
        );
    }
    for row in data["interfaceRefusals"].as_array().unwrap() {
        let packet =
            create_rust_source_owner_program_links(row["source"].as_str().unwrap()).unwrap();
        let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
        let error = emit_rust_with_options(
            &program,
            RustEmissionOptions {
                source_header_model: data["model"].as_str(),
            },
        )
        .unwrap_err();
        assert!(
            error.reason.contains(row["reason"].as_str().unwrap()),
            "{}: {}",
            row["name"],
            error.reason
        );
    }
}

#[test]
fn source_free_displayed_string_edits_refuse_after_actual_rechecking() {
    use meta_language::translation::ir::Program;
    let data = fixture();
    let packet =
        create_rust_source_owner_program_links(data["cases"][0]["source"].as_str().unwrap())
            .unwrap();
    for row in data["edits"].as_array().unwrap() {
        if row["path"][0] == "position" {
            assert_unknown_displayed_field(&packet, row);
            continue;
        }
        let program = decode_typed_program_links(&packet.network, packet.root).unwrap();
        let mut value = serde_json::to_value(program).unwrap();
        let function = value["declarations"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|entry| entry["fullName"] == "render")
            .unwrap();
        let mut target = &mut function["body"];
        while target["k"] == "let" {
            target = &mut target["body"];
        }
        assert_eq!(target["k"], "toString");
        let path = row["path"].as_array().unwrap();
        for field in &path[..path.len() - 1] {
            target = &mut target[field.as_str().unwrap()];
        }
        target[path.last().unwrap().as_str().unwrap()] = row["value"].clone();
        let accepted = serde_json::from_value::<Program>(value)
            .ok()
            .and_then(|program| meta_language::encode_typed_program_links(&program).ok())
            .is_some_and(|packet| {
                emit_typed_program_links_with_options(
                    &packet.network,
                    packet.root,
                    "Rust",
                    &options(&data),
                )
                .is_ok()
            });
        assert!(!accepted, "{}", row["name"]);
    }
    let mut program = decode_typed_program_links(&packet.network, packet.root).unwrap();
    program.utf16_strings = Some(true);
    let accepted = meta_language::encode_typed_program_links(&program)
        .ok()
        .is_some_and(|packet| {
            emit_typed_program_links_with_options(
                &packet.network,
                packet.root,
                "Rust",
                &options(&data),
            )
            .is_ok()
        });
    assert!(!accepted);
}

fn assert_unknown_displayed_field(packet: &meta_language::TypedProgramLinks, row: &Value) {
    use meta_language::{LinkType, typed_program_links_schema};
    let mut network = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    let schema = typed_program_links_schema();
    let label = schema["fields"]["k"].as_str().unwrap_or("k");
    let parent = network
        .links()
        .find_map(|link| match link.references() {
            [parent, key, child]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && network.link(*key).and_then(|key| key.metadata().term()) == Some(label)
                    && network
                        .link(*child)
                        .and_then(|child| child.metadata().definition())
                        == Some("toString") =>
            {
                Some(*parent)
            }
            _ => None,
        })
        .unwrap();
    let definition = row["value"].to_string();
    let zero = network.insert_link(
        [],
        meta_language::LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_language(schema["language"].as_str().unwrap())
            .with_term(schema["scalars"]["number"].as_str().unwrap())
            .with_definition(definition.as_str()),
    );
    network.insert_field(parent, "position", zero);
    let bytes = encode_link_network_binary(&network, BinaryLinksOptions::default()).unwrap();
    for edited in [
        network.clone(),
        LinkNetwork::from_lino(&network.to_lino()).unwrap(),
        decode_link_network_binary(&bytes, BinaryLinksOptions::default()).unwrap(),
    ] {
        assert!(
            decode_typed_program_links(&edited, packet.root).is_err(),
            "{}",
            row["name"]
        );
    }
}

#[test]
fn actual_emitted_displayed_definitions_reparse_with_independent_native_signatures() {
    let data = fixture();
    let produced = run(Command::new("node")
        .arg("../experiments/rust-records/displayed-string-interface-reparse.mjs")
        .current_dir(env!("CARGO_MANIFEST_DIR")));
    assert!(produced.ok, "{}\n{}", produced.stdout, produced.stderr);
    let javascript = meta_language::read_structured_data_lino(&produced.stdout).unwrap();
    assert_eq!(javascript["value"], data["roundTripValue"]);
    let original =
        create_rust_source_owner_program_links(data["roundTripSource"].as_str().unwrap()).unwrap();
    let program = decode_typed_program_links(&original.network, original.root).unwrap();
    let emission = emit_rust_with_options(
        &program,
        RustEmissionOptions {
            source_header_model: data["model"].as_str(),
        },
    )
    .unwrap();
    let source = format!("{}\nfn main(){{}}", emission.definitions.join("\n\n"));
    assert!(source.contains("::std::borrow::ToOwned::to_owned"));
    assert!(source.contains("::std::string::String::as_str"));
    assert!(source.contains("format!"));
    assert_eq!(source, javascript["source"]);
    let restored = create_rust_source_owner_program_links(&source).unwrap();
    let reparsed = decode_typed_program_links(&restored.network, restored.root).unwrap();
    let signatures = |program: &meta_language::translation::ir::Program| {
        program
            .declarations
            .iter()
            .filter_map(|entry| match entry {
                meta_language::translation::ir::Decl::Fn(function) => Some((
                    function.full_name.clone(),
                    function.source_signature.clone(),
                )),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(signatures(&program), signatures(&reparsed));
    for network in views(&restored) {
        for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
            assert_eq!(
                emission_value(&emit_typed_program_links(&network, restored.root, target).unwrap()),
                javascript["emissions"][target]
            );
        }
    }
    let selected = emit_rust_with_options(
        &reparsed,
        RustEmissionOptions {
            source_header_model: data["model"].as_str(),
        },
    )
    .unwrap();
    let native_library = format!(
        "{}\n{}",
        selected.preludes.join("\n"),
        selected.definitions.join("\n")
    );
    assert_reparsed_native_products(&data, &javascript, &native_library);
}

fn assert_reparsed_native_products(data: &Value, javascript: &Value, native_library: &str) {
    let directory = TemporaryDirectory::new("displayed-string-interface-reparse");
    for (product, library) in [
        ("javascript", javascript["nativeLibrary"].as_str().unwrap()),
        ("native", native_library),
    ] {
        let mut code = format!(
            "{library}\nmod original {{\n{}\n}}\n",
            data["roundTripSource"].as_str().unwrap()
        );
        for prefix in ["original", "crate"] {
            for check in data["roundTripChecks"].as_array().unwrap() {
                code.push_str(&check.as_str().unwrap().replace("CASE", prefix));
                code.push('\n');
            }
        }
        code.push_str(&format!("#[test]fn actual_reparsed_observation(){{assert_eq!(original::result().to_string(),{:?});assert_eq!(crate::result().to_string(),{:?});}}",data["roundTripValue"].as_str().unwrap(),data["roundTripValue"].as_str().unwrap()));
        let input = directory.0.join(format!("{product}.rs"));
        let binary = directory
            .0
            .join(format!("{product}{}", std::env::consts::EXE_SUFFIX));
        fs::write(&input, code).unwrap();
        let mut command = Command::new("rustc");
        command.args(["--edition", "2024", "--test"]);
        #[cfg(windows)]
        if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
            command.arg("-C").arg(format!("linker={linker}"));
        }
        let compiled = run(command.arg(&input).arg("-o").arg(&binary));
        assert!(
            compiled.ok,
            "{product}\n{}\n{}",
            compiled.stdout, compiled.stderr
        );
        let observed = run(Command::new(&binary).args(["--exact", "actual_reparsed_observation"]));
        assert!(
            observed.ok,
            "{product}\n{}\n{}",
            observed.stdout, observed.stderr
        );
        assert!(observed.stdout.contains("1 passed"));
    }
}
