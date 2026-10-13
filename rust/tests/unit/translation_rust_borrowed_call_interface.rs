//! Independent source oracles and editable Links for the borrowed-call model.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::emit_rust::{
    RustEmissionOptions, emit_rust, emit_rust_with_options,
};
use meta_language::translation::ir::{Decl, FnDecl, Program};
use meta_language::{
    LinkId, LinkMetadata, LinkNetwork, LinkType, TypedProgramLinksEmissionOptions,
    create_typed_program_links, create_typed_program_links_with_rust_source_signature_model,
    decode_typed_program_links, emit_typed_program_links, emit_typed_program_links_with_options,
    encode_typed_program_links, percent_encode_links_text,
};
use serde_json::Value;
use std::fmt::Write;
use std::fs;
use std::process::Command;

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-borrowed-call-interface.lino"
    ))
    .unwrap()
}
fn options(data: &Value) -> TypedProgramLinksEmissionOptions {
    TypedProgramLinksEmissionOptions {
        rust_source_header_emission_model: Some(data["model"].as_str().unwrap().to_owned()),
        ..TypedProgramLinksEmissionOptions::default()
    }
}
fn reload(case: &Value) -> (LinkNetwork, LinkId) {
    (
        LinkNetwork::from_lino(case["links"].as_str().unwrap()).unwrap(),
        LinkId::from_u64(case["root"].as_u64().unwrap()),
    )
}
fn function<'a>(program: &'a Program, name: &str) -> &'a FnDecl {
    let Some(Decl::Fn(function)) = program.declaration(name) else {
        panic!("the actual function exists")
    };
    function
}
fn replay(definitions: &[String], data: &Value) -> meta_language::TypedProgramLinks {
    create_typed_program_links_with_rust_source_signature_model(
        &format!("{}\nfn main(){{}}", definitions.join("\n")),
        "Rust",
        data["sourceSignatureModel"].as_str().unwrap(),
        None,
    )
    .unwrap()
}
fn field(network: &LinkNetwork, parent: LinkId, label: &str) -> (LinkId, LinkId) {
    network
        .links()
        .find_map(|link| match link.references() {
            [owner, name, child]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && *owner == parent
                    && network.link(*name).and_then(|name| name.metadata().term())
                        == Some(label) =>
            {
                Some((link.id(), *child))
            }
            _ => None,
        })
        .unwrap()
}
fn edit_definition(network: &LinkNetwork, id: LinkId, value: &str) -> LinkNetwork {
    let old = network.link(id).unwrap().metadata().definition().unwrap();
    let text = network
        .to_lino()
        .lines()
        .map(|line| {
            if line.starts_with(&format!("({id}: ")) {
                line.replacen(
                    &format!("(def: {})", percent_encode_links_text(old)),
                    &format!("(def: {})", percent_encode_links_text(value)),
                    1,
                )
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    LinkNetwork::from_lino(&text).unwrap()
}
fn edit_references(network: &LinkNetwork, id: LinkId, references: &[LinkId]) -> LinkNetwork {
    let text = network
        .to_lino()
        .lines()
        .map(|line| {
            if line.starts_with(&format!("({id}: ")) {
                let metadata = &line[line.find("(meta:").unwrap()..];
                format!(
                    "({id}: {} {metadata}",
                    references
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    LinkNetwork::from_lino(&text).unwrap()
}

#[test]
fn actual_owned_callers_keep_headers_argument_order_hygiene_and_defaults() {
    let data = fixture();
    assert_eq!(data["cases"].as_array().unwrap().len(), 12);
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let native = create_typed_program_links_with_rust_source_signature_model(
            case["source"].as_str().unwrap(),
            "Rust",
            data["sourceSignatureModel"].as_str().unwrap(),
            None,
        )
        .unwrap();
        let shared = reload(case);
        for (network, root) in [(&native.network, native.root), (&shared.0, shared.1)] {
            let before = network.to_lino();
            let program = decode_typed_program_links(network, root).unwrap();
            let emitted =
                emit_typed_program_links_with_options(network, root, "Rust", &options(&data))
                    .unwrap();
            for header in case["headers"].as_array().unwrap() {
                assert!(
                    emitted
                        .code
                        .contains(&format!("{} {{", header.as_str().unwrap())),
                    "{id}"
                );
            }
            let selected = emit_rust_with_options(
                &program,
                RustEmissionOptions {
                    source_header_model: data["model"].as_str(),
                },
            )
            .unwrap();
            assert_eq!(emitted.code, selected.text, "{id}");
            let caller = selected
                .definitions
                .iter()
                .find(|definition| {
                    definition.contains(&format!("pub fn {}", case["caller"].as_str().unwrap()))
                })
                .unwrap();
            let mut after = 0;
            for binding in case["bindings"].as_array().unwrap() {
                let binding = binding.as_str().unwrap();
                let index = caller.find(binding).unwrap();
                assert!(index >= after, "{id}: source argument order");
                assert_eq!(
                    caller.matches(binding).count(),
                    1,
                    "{id}: one materialization"
                );
                after = index + binding.len();
            }
            let invocation = case["invocation"].as_str().unwrap();
            assert!(
                caller.find(invocation).unwrap() >= after,
                "{id}: borrow after evaluation"
            );
            assert_eq!(
                caller.matches(invocation).count(),
                1,
                "{id}: one invocation"
            );
            assert_eq!(
                network.to_lino(),
                before,
                "{id}: unchanged original archive"
            );
            let default = emit_rust(&program).unwrap();
            assert_eq!(
                emit_typed_program_links(network, root, "Rust")
                    .unwrap()
                    .code,
                default.text
            );
            assert_eq!(
                emit_typed_program_links_with_options(
                    network,
                    root,
                    "Rust",
                    &TypedProgramLinksEmissionOptions::default()
                )
                .unwrap()
                .code,
                default.text
            );
            let previous = TypedProgramLinksEmissionOptions {
                rust_source_header_emission_model: Some(
                    "primitive borrowed Rust function header 1".into(),
                ),
                ..TypedProgramLinksEmissionOptions::default()
            };
            assert!(
                emit_typed_program_links_with_options(network, root, "Rust", &previous)
                    .unwrap_err()
                    .message
                    .contains(
                        "calls and effects require a distinct interface transformation proof"
                    )
            );
            let reread = replay(&selected.definitions, &data);
            let restored = decode_typed_program_links(&reread.network, reread.root).unwrap();
            for original in &program.declarations {
                assert_eq!(
                    function(&restored, original.full_name()).source_signature,
                    function(&program, original.full_name()).source_signature,
                    "{id}"
                );
            }
        }
    }
}

#[test]
fn twelve_original_unsupported_interfaces_and_selected_target_errors_refuse() {
    let data = fixture();
    assert_eq!(data["refusals"].as_array().unwrap().len(), 12);
    for case in data["refusals"].as_array().unwrap() {
        let (network, root) = reload(case);
        let refused =
            emit_typed_program_links_with_options(&network, root, "Rust", &options(&data))
                .unwrap_err();
        assert!(
            refused.message.contains("Rust borrowed call interface")
                || refused.message.contains("Rust source header emission"),
            "{}: {refused}",
            case["id"]
        );
    }
    let (network, root) = reload(&data["cases"][0]);
    assert!(
        emit_typed_program_links_with_options(&network, root, "JavaScript", &options(&data))
            .unwrap_err()
            .message
            .contains("requires Rust target")
    );
    let unknown = TypedProgramLinksEmissionOptions {
        rust_source_header_emission_model: Some("unknown".into()),
        ..TypedProgramLinksEmissionOptions::default()
    };
    assert!(
        emit_typed_program_links_with_options(&network, root, "Rust", &unknown)
            .unwrap_err()
            .message
            .contains("Rust source header emission model")
    );
    let absent =
        create_typed_program_links(data["cases"][0]["source"].as_str().unwrap(), "Rust").unwrap();
    assert!(
        emit_typed_program_links_with_options(
            &absent.network,
            absent.root,
            "Rust",
            &options(&data)
        )
        .unwrap_err()
        .message
        .contains("owned source signature facet")
    );
}

#[test]
fn linked_value_and_nested_reference_edits_change_actual_calls_without_source_replay() {
    let data = fixture();
    let (network, root) = reload(&data["cases"][0]);
    let program = decode_typed_program_links(&network, root).unwrap();
    let scalar = network
        .links()
        .find(|link| {
            link.metadata().term() == Some("string value")
                && link.metadata().definition() == Some("1.0")
        })
        .unwrap()
        .id();
    let edited = edit_definition(&network, scalar, "2.0");
    let changed = decode_typed_program_links(&edited, root).unwrap();
    for original in &program.declarations {
        assert_eq!(
            function(&changed, original.full_name()).source_signature,
            function(&program, original.full_name()).source_signature
        );
    }
    let selected = emit_rust_with_options(
        &changed,
        RustEmissionOptions {
            source_header_model: data["model"].as_str(),
        },
    )
    .unwrap();
    let imported = replay(&selected.definitions, &data);
    let restored = decode_typed_program_links(&imported.network, imported.root).unwrap();
    assert_eq!(
        function(&restored, "leaf").source_signature,
        function(&program, "leaf").source_signature
    );
    let altered = serde_json::json!({"caller":"caller", "inputs":[[3]], "expected":[5]});
    compile_and_observe(&selected.text, &observation_harness(&altered, &changed));

    let mut deep = program.clone();
    let Some(Decl::Fn(leaf)) = deep
        .declarations
        .iter_mut()
        .find(|declaration| declaration.full_name() == "leaf")
    else {
        panic!("the leaf exists")
    };
    let signature = leaf.source_signature.as_mut().unwrap();
    signature.parameters[0].wrappers.push("reference".into());
    signature.parameters[0].lifetimes = vec![Some("outer".into()), Some("inner".into())];
    signature.lifetime_bindings = vec!["outer".into(), "inner".into()];
    let expected_signature = signature.clone();
    let archive = encode_typed_program_links(&deep).unwrap();
    let reloaded = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
    let emitted =
        emit_typed_program_links_with_options(&reloaded, archive.root, "Rust", &options(&data))
            .unwrap();
    assert!(
        emitted
            .code
            .contains("pub fn leaf<'outer, 'inner>(value: &'outer &'inner f64)")
    );
    assert!(
        emitted
            .code
            .contains("crate::leaf(&(&(ml_borrowed_argument)))")
    );
    let selected = emit_rust_with_options(
        &deep,
        RustEmissionOptions {
            source_header_model: data["model"].as_str(),
        },
    )
    .unwrap();
    let imported = replay(&selected.definitions, &data);
    let restored = decode_typed_program_links(&imported.network, imported.root).unwrap();
    assert_eq!(
        function(&restored, "leaf")
            .source_signature
            .as_ref()
            .unwrap(),
        &expected_signature
    );
    compile_and_observe(
        &selected.text,
        &observation_harness(&data["cases"][0], &deep),
    );
}

#[test]
fn redirected_declaration_links_and_unrepresented_raw_call_fields_refuse() {
    let data = fixture();
    let packet = create_typed_program_links_with_rust_source_signature_model(
        "fn leaf(value:&f64)->f64{*value}fn caller(value:f64)->f64{leaf(&value)}fn other(value:&bool)->f64{if *value{1.0}else{2.0}}fn main(){}",
        "Rust", data["sourceSignatureModel"].as_str().unwrap(), None,
    ).unwrap();
    let body = packet.network.link(packet.root).unwrap().references()[0];
    let directory = field(&packet.network, body, "declarations").1;
    let declarations = packet
        .network
        .link(directory)
        .unwrap()
        .references()
        .iter()
        .map(|id| {
            let references = packet.network.link(*id).unwrap().references();
            (
                packet
                    .network
                    .link(references[0])
                    .unwrap()
                    .metadata()
                    .definition()
                    .unwrap(),
                references[1],
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let caller = field(&packet.network, declarations["caller"], "body").1;
    let relation = field(&packet.network, caller, "declaration reference").0;
    for target in ["caller", "other"] {
        let mut references = packet.network.link(relation).unwrap().references().to_vec();
        references[2] = declarations[target];
        let redirected = edit_references(&packet.network, relation, &references);
        let refused = emit_typed_program_links_with_options(
            &redirected,
            packet.root,
            "Rust",
            &options(&data),
        )
        .unwrap_err();
        assert!(
            refused.message.contains(if target == "caller" {
                "owned pure leaf"
            } else {
                "call argument type"
            }),
            "{refused}"
        );
    }
    let (original, root) = reload(&data["cases"][0]);
    let body = original.link(root).unwrap().references()[0];
    let directory = field(&original, body, "declarations").1;
    let caller = original
        .link(directory)
        .unwrap()
        .references()
        .iter()
        .find_map(|id| {
            let row = original.link(*id).unwrap().references();
            (original.link(row[0]).unwrap().metadata().definition() == Some("caller"))
                .then_some(row[1])
        })
        .unwrap();
    for (owner, label) in [
        (field(&original, caller, "body").1, "optional"),
        (caller, "unsafe"),
        (caller, "async"),
    ] {
        let mut edited = original.clone();
        let scalar = edited.insert_link(
            [],
            LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_named(true)
                .with_language("meta-language typed program")
                .with_term("boolean value")
                .with_definition("false"),
        );
        edited.insert_field(owner, label, scalar);
        let restored = LinkNetwork::from_lino(&edited.to_lino()).unwrap();
        assert!(
            emit_typed_program_links_with_options(&restored, root, "Rust", &options(&data))
                .is_err(),
            "{label}"
        );
    }
}

#[test]
fn actual_primitive_expression_records_and_parameter_spans_retain_strict_field_admission() {
    let data = fixture();
    for selected in [
        data["model"].as_str().unwrap(),
        "primitive borrowed Rust function header 1",
    ] {
        let leaf = "fn leaf(value:&f64)->f64{let copy=*value;if copy>0.0{-copy}else{1.0}}";
        let source = if selected == data["model"].as_str().unwrap() {
            format!("{leaf}fn caller(value:f64)->f64{{leaf(&value)}}fn main(){{}}")
        } else {
            format!("{leaf}fn main(){{}}")
        };
        let packet = create_typed_program_links_with_rust_source_signature_model(
            &source,
            "Rust",
            data["sourceSignatureModel"].as_str().unwrap(),
            None,
        )
        .unwrap();
        let original = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
        let root = packet.root;
        let selected_options = TypedProgramLinksEmissionOptions {
            rust_source_header_emission_model: Some(selected.into()),
            ..TypedProgramLinksEmissionOptions::default()
        };
        assert!(
            emit_typed_program_links_with_options(&original, root, "Rust", &selected_options)
                .is_ok()
        );
        for kind in ["lit", "var", "binary", "unary", "let", "if"] {
            let label = meta_language::typed_program_links_schema()["fields"]["k"]
                .as_str()
                .unwrap();
            let node = original
                .links()
                .find_map(|link| match link.references() {
                    [owner, name, value]
                        if link.metadata().link_type() == Some(LinkType::Field)
                            && original.link(*name).and_then(|name| name.metadata().term())
                                == Some(label)
                            && original
                                .link(*value)
                                .and_then(|value| value.metadata().definition())
                                == Some(kind) =>
                    {
                        Some(*owner)
                    }
                    _ => None,
                })
                .unwrap();
            for field in ["borrowEscape", "opaque"] {
                let mut changed = original.clone();
                let false_value = changed.insert_link(
                    [],
                    LinkMetadata::new()
                        .with_link_type(LinkType::Semantic)
                        .with_named(true)
                        .with_language("meta-language typed program")
                        .with_term("boolean value")
                        .with_definition("false"),
                );
                changed.insert_field(node, field, false_value);
                let restored = LinkNetwork::from_lino(&changed.to_lino()).unwrap();
                assert!(
                    emit_typed_program_links_with_options(
                        &restored,
                        root,
                        "Rust",
                        &selected_options
                    )
                    .is_err(),
                    "{selected}: {kind}.{field}"
                );
            }
        }
        let body = original.link(root).unwrap().references()[0];
        let directory = field(&original, body, "declarations").1;
        let declaration = original
            .link(original.link(directory).unwrap().references()[0])
            .unwrap()
            .references()[1];
        let parameters = field(&original, declaration, "parameters").1;
        let parameter = original.link(parameters).unwrap().references()[0];
        for field in ["async", "unsafe", "opaque"] {
            let mut changed = original.clone();
            let scalar = changed.insert_link(
                [],
                LinkMetadata::new()
                    .with_link_type(LinkType::Semantic)
                    .with_named(true)
                    .with_language("meta-language typed program")
                    .with_term("boolean value")
                    .with_definition("false"),
            );
            changed.insert_field(declaration, field, scalar);
            assert!(
                emit_typed_program_links_with_options(
                    &LinkNetwork::from_lino(&changed.to_lino()).unwrap(),
                    root,
                    "Rust",
                    &selected_options
                )
                .is_err(),
                "{selected}: {field}"
            );
        }
        for object in [false, true] {
            let mut changed = original.clone();
            let label = meta_language::typed_program_links_schema()["fields"]["span"]
                .as_str()
                .unwrap();
            let span = if object {
                field(&original, declaration, label).1
            } else {
                changed.insert_link(
                    [],
                    LinkMetadata::new()
                        .with_link_type(LinkType::Semantic)
                        .with_named(true)
                        .with_language("meta-language typed program")
                        .with_term("null value"),
                )
            };
            changed.insert_field(parameter, label, span);
            assert!(
                emit_typed_program_links_with_options(
                    &LinkNetwork::from_lino(&changed.to_lino()).unwrap(),
                    root,
                    "Rust",
                    &selected_options
                )
                .is_err(),
                "{selected}: parameter span object={object}"
            );
        }
    }
}

fn number(value: &Value) -> f64 {
    match value.as_str() {
        Some("-0") => -0.0,
        Some("NaN") => f64::NAN,
        Some("Infinity") => f64::INFINITY,
        _ => value.as_f64().unwrap(),
    }
}
fn rust_scalar(value: &Value) -> String {
    value.as_bool().map_or_else(
        || format!("f64::from_bits(0x{:016x})", number(value).to_bits()),
        |value| value.to_string(),
    )
}
fn observation_harness(case: &Value, program: &Program) -> String {
    let name = case["caller"].as_str().unwrap();
    let signature = function(program, name).source_signature.as_ref().unwrap();
    let mut lines = String::new();
    for (index, values) in case["inputs"].as_array().unwrap().iter().enumerate() {
        let mut arguments = Vec::new();
        for (position, value) in values.as_array().unwrap().iter().enumerate() {
            let variable = format!("INPUT_{index}_{position}");
            let ty = if value.is_boolean() { "bool" } else { "f64" };
            writeln!(
                &mut lines,
                "const {variable}: {ty} = {};",
                rust_scalar(value)
            )
            .unwrap();
            arguments.push(format!(
                "{}{variable}",
                "&".repeat(signature.parameters[position].wrappers.len())
            ));
        }
        writeln!(
            &mut lines,
            "let observed = {name}({});",
            arguments.join(", ")
        )
        .unwrap();
        let expected = &case["expected"][index];
        if expected.is_boolean() {
            writeln!(
                &mut lines,
                "assert_eq!(observed, {});",
                rust_scalar(expected)
            )
            .unwrap();
        } else if expected == "NaN" {
            lines.push_str("assert!(observed.is_nan());\n");
        } else {
            writeln!(
                &mut lines,
                "assert_eq!(observed.to_bits(), {}.to_bits());",
                rust_scalar(expected)
            )
            .unwrap();
        }
    }
    format!("\n#[test]\nfn borrowed_call_observations() {{\n{lines}\n}}\n")
}
fn compile_and_observe(code: &str, harness: &str) {
    let directory = TemporaryDirectory::new("borrowed-call-interface");
    let source = directory.0.join("witness.rs");
    let binary = directory
        .0
        .join(format!("witness{}", std::env::consts::EXE_SUFFIX));
    fs::write(&source, format!("{code}\n{harness}")).unwrap();
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition", "2024", "--test"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    let compiled = run(compiler.arg(&source).arg("-o").arg(&binary));
    assert!(
        compiled.ok,
        "Rust borrowed-call compilation failed\n{}\n{}",
        compiled.stdout, compiled.stderr
    );
    let observed = run(Command::new(&binary).args(["--exact", "borrowed_call_observations"]));
    assert!(
        observed.ok,
        "Rust borrowed-call observations failed\n{}\n{}",
        observed.stdout, observed.stderr
    );
    assert!(observed.stdout.contains("1 passed"), "{}", observed.stdout);
}

fn observe_javascript(code: &str, case: &Value) {
    let scalar = |value: &Value| {
        value
            .as_str()
            .map_or_else(|| value.to_string(), str::to_owned)
    };
    let mut script = format!("const assert = require('node:assert/strict');\n{code}\n");
    for (values, expected) in case["inputs"]
        .as_array()
        .unwrap()
        .iter()
        .zip(case["expected"].as_array().unwrap())
    {
        let arguments = values
            .as_array()
            .unwrap()
            .iter()
            .map(&scalar)
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            &mut script,
            "assert.ok(Object.is({}({arguments}), {}));",
            case["caller"].as_str().unwrap(),
            scalar(expected)
        )
        .unwrap();
    }
    let directory = TemporaryDirectory::new("borrowed-call-javascript-observations");
    let file = directory.0.join("observations.cjs");
    fs::write(&file, script).unwrap();
    let observed = run(Command::new("node").arg(&file));
    assert!(
        observed.ok,
        "JavaScript borrowed-call observations failed\n{}\n{}",
        observed.stdout, observed.stderr
    );
}

#[test]
fn original_and_full_emitted_rust_sources_execute_independent_primitive_observations() {
    let data = fixture();
    for case in data["cases"].as_array().unwrap() {
        let (network, root) = reload(case);
        let program = decode_typed_program_links(&network, root).unwrap();
        let harness = observation_harness(case, &program);
        compile_and_observe(case["source"].as_str().unwrap(), &harness);
        let emitted =
            emit_typed_program_links_with_options(&network, root, "Rust", &options(&data)).unwrap();
        compile_and_observe(&emitted.code, &harness);
        let selected = emit_rust_with_options(
            &program,
            RustEmissionOptions {
                source_header_model: data["model"].as_str(),
            },
        )
        .unwrap();
        let imported = replay(&selected.definitions, &data);
        let reread = decode_typed_program_links(&imported.network, imported.root).unwrap();
        let javascript =
            emit_typed_program_links(&imported.network, imported.root, "JavaScript").unwrap();
        assert_eq!(
            function(&reread, case["caller"].as_str().unwrap()).source_signature,
            function(&program, case["caller"].as_str().unwrap()).source_signature
        );
        observe_javascript(
            &emit_typed_program_links(&network, root, "JavaScript")
                .unwrap()
                .code,
            case,
        );
        observe_javascript(&javascript.code, case);
    }
}
