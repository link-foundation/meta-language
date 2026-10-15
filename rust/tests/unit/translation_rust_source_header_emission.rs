//! Source-free primitive reference headers and independent native observations.

use std::fs;
use std::process::Command;

use meta_language::translation::emit_rust::{
    RustEmissionOptions, emit_rust, emit_rust_with_options,
};
use meta_language::translation::ir::{Decl, Program};
use meta_language::{
    LinkId, LinkMetadata, LinkNetwork, LinkType, TypedProgramLinksEmissionOptions,
    create_typed_program_links, create_typed_program_links_with_rust_source_signature_model,
    decode_typed_program_links, emit_typed_program_links, emit_typed_program_links_with_options,
    encode_typed_program_links, percent_encode_links_text,
};
use serde_json::Value;

use super::issue_195_translation_pairs::{TemporaryDirectory, run};

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-source-header-emission.lino"
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

fn function<'a>(program: &'a Program, name: &str) -> &'a meta_language::translation::ir::FnDecl {
    let Some(Decl::Fn(function)) = program.declaration(name) else {
        panic!("the function exists")
    };
    function
}

fn field(network: &LinkNetwork, parent: LinkId, label: &str) -> (LinkId, LinkId) {
    let label = meta_language::typed_program_links_schema()["fields"][label]
        .as_str()
        .unwrap_or(label);
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
fn twelve_owned_headers_keep_order_copies_defaults_and_reimported_facts() {
    let data = fixture();
    let options = options(&data);
    let model = data["model"].as_str().unwrap();
    let capture = data["sourceSignatureModel"].as_str().unwrap();
    assert_eq!(data["cases"].as_array().unwrap().len(), 12);
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let native = create_typed_program_links_with_rust_source_signature_model(
            case["source"].as_str().unwrap(),
            "Rust",
            capture,
            None,
        )
        .unwrap();
        let shared = reload(case);
        for (network, root) in [(&native.network, native.root), (&shared.0, shared.1)] {
            let before = network.to_lino();
            let program = decode_typed_program_links(network, root).unwrap();
            let emitted =
                emit_typed_program_links_with_options(network, root, "Rust", &options).unwrap();
            assert!(
                emitted
                    .code
                    .contains(&format!("{} {{", case["header"].as_str().unwrap())),
                "{id}"
            );
            for copy in case["copies"].as_array().unwrap() {
                assert!(
                    emitted
                        .code
                        .contains(&format!("    {}", copy.as_str().unwrap())),
                    "{id}"
                );
            }
            assert_eq!(network.to_lino(), before, "{id}");
            let default = emit_rust(&program).unwrap();
            assert_eq!(
                emit_typed_program_links(network, root, "Rust")
                    .unwrap()
                    .code,
                default.text,
                "{id}"
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
                default.text,
                "{id}"
            );
            let selected = emit_rust_with_options(
                &program,
                RustEmissionOptions {
                    source_header_model: Some(model),
                },
            )
            .unwrap();
            assert_eq!(selected.text, emitted.code, "{id}");
            let reread = create_typed_program_links_with_rust_source_signature_model(
                &format!("{}\nfn main(){{}}", selected.definitions.join("\n")),
                "Rust",
                capture,
                None,
            )
            .unwrap();
            let restored = decode_typed_program_links(&reread.network, reread.root).unwrap();
            let name = case["functionName"].as_str().unwrap();
            assert_eq!(
                function(&restored, name).source_signature,
                function(&program, name).source_signature,
                "{id}"
            );
        }
    }
}

#[test]
fn unknown_models_wrong_targets_absent_facets_and_eight_interfaces_refuse() {
    let data = fixture();
    let options = options(&data);
    let (network, root) = reload(&data["cases"][1]);
    assert!(
        emit_typed_program_links_with_options(&network, root, "JavaScript", &options)
            .unwrap_err()
            .message
            .contains("requires Rust target")
    );
    let unknown = TypedProgramLinksEmissionOptions {
        rust_source_header_emission_model: Some("unknown".to_owned()),
        ..TypedProgramLinksEmissionOptions::default()
    };
    assert!(
        emit_typed_program_links_with_options(&network, root, "Rust", &unknown)
            .unwrap_err()
            .message
            .contains("Rust source header emission model")
    );
    let absent =
        create_typed_program_links(data["cases"][1]["source"].as_str().unwrap(), "Rust").unwrap();
    assert!(
        emit_typed_program_links_with_options(&absent.network, absent.root, "Rust", &options)
            .unwrap_err()
            .message
            .contains("owned source signature facet")
    );
    assert_eq!(data["refusals"].as_array().unwrap().len(), 8);
    for case in data["refusals"].as_array().unwrap() {
        let (network, root) = reload(case);
        assert!(
            emit_typed_program_links_with_options(&network, root, "Rust", &options)
                .unwrap_err()
                .message
                .contains("Rust source header emission"),
            "{}",
            case["id"]
        );
    }
}

#[test]
fn actual_owner_generated_and_linked_context_premises_cannot_be_erased() {
    let data = fixture();
    let (network, root) = reload(&data["cases"][1]);
    let program = decode_typed_program_links(&network, root).unwrap();
    let selected = RustEmissionOptions {
        source_header_model: data["model"].as_str(),
    };
    for output in [true, false] {
        let mut changed = program.clone();
        changed.output_threaded = output;
        changed.aborts_threaded = !output;
        assert!(emit_rust_with_options(&changed, selected).is_err());
    }
    let mut generated = program.clone();
    let Decl::Fn(function) = &mut generated.declarations[0] else {
        panic!("the function exists")
    };
    function.generated = true;
    assert!(
        emit_rust_with_options(&generated, selected)
            .unwrap_err()
            .reason
            .contains("original leaf")
    );
    for (kind, path) in [
        ("declaration", vec!["Alias".to_owned()]),
        ("primitive", vec!["core".to_owned(), "f64".to_owned()]),
    ] {
        let mut changed = program.clone();
        let Decl::Fn(function) = &mut changed.declarations[0] else {
            panic!("the function exists")
        };
        let owner = &mut function.source_signature.as_mut().unwrap().parameters[0].owner;
        owner.kind = kind.to_owned();
        owner.path = path;
        assert!(encode_typed_program_links(&changed).is_err());
        assert!(emit_rust_with_options(&changed, selected).is_err());
    }
    for label in ["reference semantics", "Unicode code unit strings"] {
        let mut changed = network.clone();
        let body = changed.link(root).unwrap().references()[0];
        let boolean = changed.insert_link(
            [],
            LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_named(true)
                .with_language("meta-language typed program")
                .with_term("boolean value")
                .with_definition("true"),
        );
        changed.insert_field(body, label, boolean);
        let restored = LinkNetwork::from_lino(&changed.to_lino()).unwrap();
        assert!(
            emit_typed_program_links_with_options(&restored, root, "Rust", &options(&data))
                .is_err(),
            "{label}"
        );
        let unknown = TypedProgramLinksEmissionOptions {
            rust_source_header_emission_model: Some("unknown".to_owned()),
            ..TypedProgramLinksEmissionOptions::default()
        };
        assert!(
            emit_typed_program_links_with_options(&restored, root, "Rust", &unknown)
                .unwrap_err()
                .message
                .contains("Rust source header emission model"),
            "an unknown model precedes the forbidden {label} context"
        );
    }
    let body = network.link(root).unwrap().references()[0];
    let directory = field(&network, body, "declarations").1;
    let entry = network.link(directory).unwrap().references()[0];
    let declaration = network.link(entry).unwrap().references()[1];
    let parameters = field(&network, declaration, "parameters").1;
    let parameter = network.link(parameters).unwrap().references()[0];
    for value in [false, true] {
        let mut changed = network.clone();
        let scalar = changed.insert_link(
            [],
            LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_named(true)
                .with_language("meta-language typed program")
                .with_term("boolean value")
                .with_definition(value.to_string()),
        );
        changed.insert_field(parameter, "optional", scalar);
        let restored = LinkNetwork::from_lino(&changed.to_lino()).unwrap();
        assert!(
            emit_typed_program_links_with_options(&restored, root, "Rust", &options(&data))
                .is_err(),
            "an actual linked optional={value} parameter has no admitted source-header interpretation"
        );
    }
    for (kind, definition) in [
        ("null value", None),
        ("number value", Some("0")),
        ("string value", Some("")),
        ("boolean value", Some("true")),
        ("boolean value", Some("false")),
    ] {
        let mut changed = network.clone();
        let mut metadata = LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_language("meta-language typed program")
            .with_term(kind);
        if let Some(value) = definition {
            metadata = metadata.with_definition(value);
        }
        let scalar = changed.insert_link([], metadata);
        changed.insert_field(declaration, "generated", scalar);
        let restored = LinkNetwork::from_lino(&changed.to_lino()).unwrap();
        let emitted =
            emit_typed_program_links_with_options(&restored, root, "Rust", &options(&data));
        if kind == "boolean value" && definition == Some("false") {
            assert!(
                emitted.is_ok(),
                "an explicit linked generated=false declaration remains original"
            );
        } else {
            assert!(
                emitted.is_err(),
                "an actual linked generated={kind}:{definition:?} cannot become original through truthiness"
            );
        }
    }
}

#[test]
fn malformed_header_facts_cross_function_binders_and_injected_lifetimes_refuse() {
    let data = fixture();
    let options = options(&data);
    let original = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-source-signatures.lino"
    ))
    .unwrap();
    for case in original["malformed"].as_array().unwrap() {
        let network = LinkNetwork::from_lino(case["archive"].as_str().unwrap()).unwrap();
        assert!(
            emit_typed_program_links_with_options(
                &network,
                LinkId::from_u64(case["root"].as_u64().unwrap()),
                "Rust",
                &options
            )
            .is_err(),
            "{}",
            case["id"]
        );
    }
    let archive = create_typed_program_links_with_rust_source_signature_model(
        "fn subtract(first:&f64,second:&f64)->f64{*first-*second}fn other(first:&f64)->f64{*first}fn main(){}",
        "Rust", data["sourceSignatureModel"].as_str().unwrap(), None,
    ).unwrap();
    let body = archive.network.link(archive.root).unwrap().references()[0];
    let directory = field(&archive.network, body, "declarations").1;
    let declarations = archive
        .network
        .link(directory)
        .unwrap()
        .references()
        .iter()
        .map(|id| {
            let references = archive.network.link(*id).unwrap().references();
            (
                archive
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
    let subtraction = field(&archive.network, declarations["subtract"], "body").1;
    let left = field(&archive.network, subtraction, "left").1;
    let binding = field(&archive.network, left, "binding reference").0;
    let other = archive
        .network
        .link(field(&archive.network, declarations["other"], "parameters").1)
        .unwrap()
        .references()[0];
    let mut references = archive.network.link(binding).unwrap().references().to_vec();
    references[2] = other;
    let changed = edit_references(&archive.network, binding, &references);
    assert!(
        emit_typed_program_links_with_options(&changed, archive.root, "Rust", &options)
            .unwrap_err()
            .message
            .contains("outside its scope")
    );
    let named = data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == "named-lifetime")
        .unwrap();
    for text in ["", "a>\nfn injected()", "1bad", "non ASCII α"] {
        let (network, root) = reload(named);
        let mut program = decode_typed_program_links(&network, root).unwrap();
        let Decl::Fn(function) = &mut program.declarations[0] else {
            panic!("the function exists")
        };
        let signature = function.source_signature.as_mut().unwrap();
        signature.lifetime_bindings[0] = text.to_owned();
        signature.parameters[0].lifetimes[0] = Some(text.to_owned());
        let graph = encode_typed_program_links(&program).unwrap();
        let network = LinkNetwork::from_lino(&graph.network.to_lino()).unwrap();
        assert!(
            emit_typed_program_links_with_options(&network, graph.root, "Rust", &options)
                .unwrap_err()
                .message
                .contains("ASCII identifier"),
            "{text:?}"
        );
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
    let name = case["functionName"].as_str().unwrap();
    let signature = function(program, name).source_signature.as_ref().unwrap();
    let mut lines = Vec::new();
    for (index, inputs) in case["inputs"].as_array().unwrap().iter().enumerate() {
        let mut arguments = Vec::new();
        for (position, value) in inputs.as_array().unwrap().iter().enumerate() {
            let variable = format!("INPUT_{index}_{position}");
            let ty = if value.is_boolean() { "bool" } else { "f64" };
            lines.push(format!("const {variable}: {ty} = {};", rust_scalar(value)));
            arguments.push(format!(
                "{}{variable}",
                "&".repeat(signature.parameters[position].wrappers.len())
            ));
        }
        lines.push(format!("let observed = {name}({});", arguments.join(", ")));
        let expected = &case["expected"][index];
        if expected.is_boolean() {
            lines.push(format!("assert_eq!(observed, {});", rust_scalar(expected)));
        } else if expected.as_str() == Some("NaN") {
            lines.push("assert!(observed.is_nan());".to_owned());
        } else {
            lines.push(format!(
                "assert_eq!(observed.to_bits(), {}.to_bits());",
                rust_scalar(expected)
            ));
        }
    }
    format!(
        "\n#[test]\nfn source_header_observations() {{\n{}\n}}\n",
        lines.join("\n")
    )
}

fn compile_and_observe(code: &str, harness: &str) {
    let directory = TemporaryDirectory::new("source-header-emission");
    let source = directory.0.join("witness.rs");
    let binary = directory
        .0
        .join(format!("witness{}", std::env::consts::EXE_SUFFIX));
    fs::write(&source, format!("{code}\n{harness}")).unwrap();
    let mut build_command = Command::new("rustc");
    build_command.args(["--edition", "2024", "--test"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        build_command.arg("-C").arg(format!("linker={linker}"));
    }
    let compiled = run(build_command.arg(&source).arg("-o").arg(&binary));
    assert!(compiled.ok, "{}", compiled.stderr);
    let observed = run(Command::new(&binary).args(["--exact", "source_header_observations"]));
    assert!(observed.ok, "{}\n{}", observed.stdout, observed.stderr);
    assert!(observed.stdout.contains("1 passed"), "{}", observed.stdout);
}

#[test]
fn actual_original_and_selected_rust_interfaces_execute_independent_bit_and_boolean_oracles() {
    let data = fixture();
    let options = options(&data);
    for case in data["cases"].as_array().unwrap() {
        let (network, root) = reload(case);
        let program = decode_typed_program_links(&network, root).unwrap();
        let harness = observation_harness(case, &program);
        compile_and_observe(case["source"].as_str().unwrap(), &harness);
        let emitted =
            emit_typed_program_links_with_options(&network, root, "Rust", &options).unwrap();
        compile_and_observe(&emitted.code, &harness);
    }
}

#[test]
fn source_free_body_edits_change_observations_without_changing_reference_facts() {
    let data = fixture();
    let options = options(&data);
    let archive = create_typed_program_links_with_rust_source_signature_model(
        "fn add_one(value:&f64)->f64{*value+1.0}fn main(){}",
        "Rust",
        data["sourceSignatureModel"].as_str().unwrap(),
        None,
    )
    .unwrap();
    let before = decode_typed_program_links(&archive.network, archive.root).unwrap();
    let body = archive.network.link(archive.root).unwrap().references()[0];
    let declarations = field(&archive.network, body, "declarations").1;
    let declaration = archive
        .network
        .link(declarations)
        .unwrap()
        .references()
        .iter()
        .find_map(|entry| {
            let pair = archive.network.link(*entry).unwrap().references();
            (archive
                .network
                .link(pair[0])
                .unwrap()
                .metadata()
                .definition()
                == Some("add_one"))
            .then_some(pair[1])
        })
        .unwrap();
    let addition = field(&archive.network, declaration, "body").1;
    let right = field(&archive.network, addition, "right").1;
    let literal = field(&archive.network, right, "value").1;
    assert_eq!(
        archive
            .network
            .link(literal)
            .unwrap()
            .metadata()
            .definition(),
        Some("1")
    );
    let edited = edit_definition(&archive.network, literal, "2.0");
    let program = decode_typed_program_links(&edited, archive.root).unwrap();
    assert_eq!(
        function(&program, "add_one").source_signature,
        function(&before, "add_one").source_signature
    );
    let emitted =
        emit_typed_program_links_with_options(&edited, archive.root, "Rust", &options).unwrap();
    assert!(emitted.code.contains("value: &f64"));
    compile_and_observe(
        &emitted.code,
        "#[test] fn source_header_observations(){let input=3.0;assert_eq!(add_one(&input).to_bits(),5.0_f64.to_bits());}",
    );
    let javascript = emit_typed_program_links(&edited, archive.root, "JavaScript").unwrap();
    let directory = TemporaryDirectory::new("source-header-edit");
    let script = directory.0.join("witness.js");
    fs::write(
        &script,
        format!(
            "{}\nif(!Object.is(add_one(3),5))throw new Error('source-free edit lost');\n",
            javascript.code
        ),
    )
    .unwrap();
    let observed = run(Command::new("node").arg(&script));
    assert!(observed.ok, "{}", observed.stderr);
}
