//! Paired selected type namespaces; native compilation runs only in CI.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::{Decl, Node, Program};
use meta_language::translation::rust::{parse_rust, parse_rust_with_type_namespace_model};
use meta_language::{
    LinkId, LinkMetadata, LinkNetwork, LinkType, TypedProgramLinksEmissionOptions,
    create_typed_program_links_with_rust_type_namespace_model, decode_typed_program_links,
    emit_typed_program_links, emit_typed_program_links_with_options, encode_typed_program_links,
};
use serde_json::{Value, json};
use std::fs;
use std::process::Command;

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-type-namespace.lino"
    ))
    .unwrap()
}
fn function<'a>(program: &'a Program, name: &str) -> &'a meta_language::translation::ir::FnDecl {
    let Some(Decl::Fn(function)) = program.declaration(name) else {
        panic!("the actual function exists")
    };
    function
}
fn archive(entry: &Value) -> (LinkNetwork, LinkId) {
    (
        LinkNetwork::from_lino(entry["archive"].as_str().unwrap()).unwrap(),
        LinkId::from_u64(entry["root"].as_u64().unwrap()),
    )
}
fn field(network: &LinkNetwork, owner: LinkId, label: &str) -> LinkId {
    network
        .links()
        .find_map(|link| match link.references() {
            [parent, name, child]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && *parent == owner
                    && network.link(*name).and_then(|name| name.metadata().term())
                        == Some(label) =>
            {
                Some(*child)
            }
            _ => None,
        })
        .unwrap()
}
fn declaration(network: &LinkNetwork, name: &str) -> LinkId {
    network
        .links()
        .find_map(|link| match link.references() {
            [owner, label, value]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && network
                        .link(*label)
                        .and_then(|label| label.metadata().term())
                        == Some("full name")
                    && network
                        .link(*value)
                        .and_then(|value| value.metadata().definition())
                        == Some(name) =>
            {
                Some(*owner)
            }
            _ => None,
        })
        .unwrap()
}
fn observe(code: &str) -> Vec<String> {
    let observed = run(Command::new("node")
        .arg("--input-type=module")
        .arg("-e")
        .arg(code));
    assert!(observed.ok, "{}", observed.stderr);
    observed.stdout.lines().map(str::to_owned).collect()
}
fn expected(entry: &Value) -> Vec<String> {
    entry["output"]
        .as_array()
        .unwrap()
        .iter()
        .map(|line| line.as_str().unwrap().to_owned())
        .collect()
}

#[test]
fn fifteen_actual_source_headers_retain_distinct_type_and_value_bindings() {
    let data = fixture();
    let model = data["typeNamespaceModel"].as_str().unwrap();
    assert_eq!(data["cases"].as_array().unwrap().len(), 15);
    for entry in data["cases"].as_array().unwrap() {
        let id = entry["id"].as_str().unwrap();
        let source = entry["source"].as_str().unwrap();
        let name = entry["functionName"].as_str().unwrap();
        let checked =
            check_program(&parse_rust_with_type_namespace_model(source, model, "").unwrap())
                .unwrap();
        let actual = function(&checked, name);
        assert_eq!(
            serde_json::to_value(&actual.params[0].ty).unwrap(),
            entry["expected"]["parameterType"],
            "{id}"
        );
        assert_eq!(
            serde_json::to_value(&actual.ret).unwrap(),
            entry["expected"]["resultType"],
            "{id}"
        );
        let signature = actual.source_signature.as_ref().unwrap();
        assert_eq!(
            json!(signature.parameters[0].owner.path),
            entry["expected"]["parameterOwner"],
            "{id}"
        );
        assert_eq!(
            json!(signature.parameters[0].wrappers),
            entry["expected"]["wrappers"],
            "{id}"
        );
        assert_eq!(
            json!(signature.parameters[0].lifetimes),
            entry["expected"]["lifetimes"],
            "{id}"
        );
        assert_eq!(
            json!(signature.result.owner.path),
            entry["expected"]["resultOwner"],
            "{id}"
        );
        assert_eq!(
            actual.source_type_namespace.as_ref().unwrap(),
            &json!({
                "model":model,"language":"Rust","parameters":[entry["expected"]["parameterSpelling"]],"result":entry["expected"]["resultSpelling"]
            }),
            "{id}"
        );
        let native = create_typed_program_links_with_rust_type_namespace_model(
            source,
            "Rust",
            model,
            None,
            Some(&json!({"project":id})),
        )
        .unwrap();
        let native_network = LinkNetwork::from_lino(&native.network.to_lino()).unwrap();
        let (shared, shared_root) = archive(entry);
        for (network, root) in [(&native_network, native.root), (&shared, shared_root)] {
            let decoded = decode_typed_program_links(network, root).unwrap();
            assert_eq!(
                function(&decoded, name).source_type_namespace,
                actual.source_type_namespace,
                "{id}"
            );
            if let Some(caller) = entry["callFunction"].as_str() {
                let Node::Call { func, .. } = &function(&decoded, caller).body.node else {
                    panic!("the value call is retained")
                };
                assert_eq!(func, entry["callTarget"].as_str().unwrap(), "{id}");
            }
            assert_eq!(
                observe(
                    &emit_typed_program_links(network, root, "JavaScript")
                        .unwrap()
                        .code
                ),
                expected(entry),
                "{id}"
            );
            assert!(
                network
                    .links()
                    .all(|link| link.metadata().definition() != Some(source)),
                "{id}"
            );
        }
    }
}

#[test]
fn every_actual_emitted_definition_reimports_with_its_projected_datatype_identity() {
    let data = fixture();
    let model = data["typeNamespaceModel"].as_str().unwrap();
    for entry in data["cases"].as_array().unwrap() {
        let (network, root) = archive(entry);
        let emitted = emit_rust(&decode_typed_program_links(&network, root).unwrap()).unwrap();
        let target = |name: &str| {
            emitted
                .mappings
                .iter()
                .find(|mapping| mapping.source == name)
                .map_or_else(|| name.to_owned(), |mapping| mapping.target.clone())
        };
        let source = format!("{}\nfn main(){{}}", emitted.definitions.join("\n"));
        let selected = create_typed_program_links_with_rust_type_namespace_model(
            &source, "Rust", model, None, None,
        )
        .unwrap();
        let loaded = LinkNetwork::from_lino(&selected.network.to_lino()).unwrap();
        let decoded = decode_typed_program_links(&loaded, selected.root).unwrap();
        let actual = function(&decoded, &target(entry["functionName"].as_str().unwrap()));
        let projected = |value: &Value| {
            if value["kind"] == "data" {
                json!({"kind":"data","name":target(value["name"].as_str().unwrap())})
            } else {
                value.clone()
            }
        };
        assert_eq!(
            serde_json::to_value(&actual.params[0].ty).unwrap(),
            projected(&entry["expected"]["parameterType"]),
            "{}",
            entry["id"]
        );
        assert_eq!(
            serde_json::to_value(&actual.ret).unwrap(),
            projected(&entry["expected"]["resultType"]),
            "{}",
            entry["id"]
        );
        if let Some(caller) = entry["callFunction"].as_str() {
            let Node::Call { func, .. } = &function(&decoded, &target(caller)).body.node else {
                panic!("the reimported call is retained")
            };
            assert_eq!(func, &target(entry["callTarget"].as_str().unwrap()));
            let js = emit_typed_program_links(&loaded, selected.root, "JavaScript").unwrap();
            let name = &js
                .mappings
                .iter()
                .find(|mapping| mapping.source == target(caller))
                .unwrap()
                .target;
            assert_eq!(
                observe(&format!("{}\nconsole.log({name}());", js.code)),
                ["true"]
            );
        }
    }
}

#[test]
fn twenty_four_precise_source_refusals_and_twenty_five_malformed_graphs_remain_closed() {
    let data = fixture();
    let model = data["typeNamespaceModel"].as_str().unwrap();
    assert_eq!(data["invalid"].as_array().unwrap().len(), 24);
    for entry in data["invalid"].as_array().unwrap() {
        let failure =
            parse_rust_with_type_namespace_model(entry["source"].as_str().unwrap(), model, "")
                .and_then(|surface| check_program(&surface))
                .unwrap_err();
        assert_eq!(
            failure.kind.as_str(),
            entry["kind"].as_str().unwrap(),
            "{}",
            entry["id"]
        );
        assert_eq!(
            failure.reason,
            entry["reason"].as_str().unwrap(),
            "{}",
            entry["id"]
        );
        assert_eq!(
            serde_json::to_value(failure.span).unwrap(),
            entry["span"],
            "{}",
            entry["id"]
        );
    }
    assert_eq!(data["malformed"].as_array().unwrap().len(), 25);
    for entry in data["malformed"].as_array().unwrap() {
        let (network, root) = archive(entry);
        for target in ["JavaScript", "Rust"] {
            assert!(
                emit_typed_program_links(&network, root, target).is_err(),
                "{}:{target}",
                entry["id"]
            );
        }
    }
    let (mut network, root) = archive(&data["cases"][0]);
    let owner = declaration(&network, "nested.header");
    let scalar = network.insert_link(
        [],
        LinkMetadata::new()
            .with_language("meta-language typed program")
            .with_named(true)
            .with_term("boolean value")
            .with_definition("false"),
    );
    network.insert_field(owner, "generated", scalar);
    let reloaded = LinkNetwork::from_lino(&network.to_lino()).unwrap();
    assert!(decode_typed_program_links(&reloaded, root).is_ok());
    assert_eq!(
        observe(
            &emit_typed_program_links(&reloaded, root, "JavaScript")
                .unwrap()
                .code
        ),
        expected(&data["cases"][0])
    );
}

#[test]
fn consistent_source_free_spelling_and_value_edits_preserve_both_binding_namespaces() {
    let data = fixture();
    let (network, root) = archive(&data["spellingEdit"]);
    let program = decode_typed_program_links(&network, root).unwrap();
    let actual = function(&program, "nested.header");
    assert_eq!(
        actual.source_type_namespace.as_ref().unwrap()["parameters"],
        json!([["crate", "f64"]])
    );
    assert_eq!(
        serde_json::to_value(&actual.params[0].ty).unwrap(),
        json!({"kind":"data","name":"f64"})
    );
    assert_eq!(
        actual.source_signature.as_ref().unwrap().parameters[0]
            .owner
            .path,
        ["f64"]
    );
    assert_eq!(
        observe(
            &emit_typed_program_links(&network, root, "JavaScript")
                .unwrap()
                .code
        ),
        expected(&data["spellingEdit"])
    );
    let (network, root) = archive(&data["cases"][0]);
    let value = field(
        &network,
        field(&network, declaration(&network, "nested.f64"), "body"),
        "value",
    );
    let text = network.to_lino();
    let prefix = format!("({}: ", value.as_u64());
    let original = text.lines().find(|line| line.starts_with(&prefix)).unwrap();
    assert!(original.contains("(def: true)"));
    let edited = original.replace("(def: true)", "(def: false)");
    assert_ne!(edited, original);
    let loaded = LinkNetwork::from_lino(&text.replace(original, &edited)).unwrap();
    let decoded = decode_typed_program_links(&loaded, root).unwrap();
    let Node::Call { func, .. } = &function(&decoded, "nested.value").body.node else {
        panic!("the actual edited call exists")
    };
    assert_eq!(func, "nested.f64");
    assert_eq!(
        function(&decoded, "nested.header")
            .source_type_namespace
            .as_ref()
            .unwrap()["parameters"],
        json!([["f64"]])
    );
    assert_eq!(
        observe(
            &emit_typed_program_links(&loaded, root, "JavaScript")
                .unwrap()
                .code
        ),
        ["false", "true"]
    );
    let emitted = emit_rust(&decoded).unwrap();
    let source = format!("{}\nfn main(){{}}", emitted.definitions.join("\n"));
    let selected = create_typed_program_links_with_rust_type_namespace_model(
        &source,
        "Rust",
        data["typeNamespaceModel"].as_str().unwrap(),
        None,
        None,
    )
    .unwrap();
    let name = &emitted
        .mappings
        .iter()
        .find(|mapping| mapping.source == "nested.value")
        .unwrap()
        .target;
    let js = emit_typed_program_links(&selected.network, selected.root, "JavaScript").unwrap();
    let caller = &js
        .mappings
        .iter()
        .find(|mapping| mapping.source == *name)
        .unwrap()
        .target;
    assert_eq!(
        observe(&format!("{}\nconsole.log({caller}());", js.code)),
        ["false"]
    );
}

#[test]
fn independent_namespace_selection_preserves_defaults_refusals_and_caller_provenance() {
    let model = "resolved Rust type namespace 1";
    let source = "fn keep(value:f64)->f64{value}fn main(){}";
    let legacy = check_program(&parse_rust(source).unwrap()).unwrap();
    assert_eq!(
        check_program(&parse_rust_with_type_namespace_model(source, "", "").unwrap()).unwrap(),
        legacy
    );
    assert!(function(&legacy, "keep").source_type_namespace.is_none());
    assert!(function(&legacy, "keep").source_signature.is_none());
    let selected = create_typed_program_links_with_rust_type_namespace_model(
        source,
        "Rust",
        model,
        None,
        Some(&json!({"project":"namespace"})),
    )
    .unwrap();
    let loaded = LinkNetwork::from_lino(&selected.network.to_lino()).unwrap();
    let provenance = field(&loaded, selected.root, "provenance");
    let profile = field(&loaded, provenance, "source profile");
    assert_eq!(
        loaded
            .link(field(&loaded, profile, "rust type namespace model"))
            .unwrap()
            .metadata()
            .definition(),
        Some(model)
    );
    assert_eq!(
        loaded
            .link(field(
                &loaded,
                field(&loaded, provenance, "user provenance"),
                "project"
            ))
            .unwrap()
            .metadata()
            .definition(),
        Some("namespace")
    );
    assert!(
        parse_rust_with_type_namespace_model(source, "unknown", "")
            .unwrap_err()
            .reason
            .contains("Rust type namespace model")
    );
    assert!(
        create_typed_program_links_with_rust_type_namespace_model(
            source, "Rust", "unknown", None, None
        )
        .unwrap_err()
        .to_string()
        .contains("Rust type namespace model")
    );
    assert!(
        create_typed_program_links_with_rust_type_namespace_model(
            "function keep(value){return value;}",
            "JavaScript",
            model,
            None,
            None
        )
        .unwrap_err()
        .to_string()
        .contains("requires Rust source")
    );
    assert!(
        parse_rust_with_type_namespace_model(source, model, "unknown")
            .unwrap_err()
            .reason
            .contains("Rust source signature model")
    );
    let prior = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-source-signatures.lino"
    ))
    .unwrap();
    for entry in prior["invalid"].as_array().unwrap() {
        let failure = meta_language::translation::rust::parse_rust_with_source_signature_model(
            entry["source"].as_str().unwrap(),
            prior["sourceSignatureModel"].as_str().unwrap(),
        )
        .and_then(|surface| check_program(&surface))
        .unwrap_err();
        assert_eq!(failure.kind.as_str(), entry["kind"].as_str().unwrap());
        assert_eq!(failure.reason, entry["reason"].as_str().unwrap());
        assert_eq!(serde_json::to_value(failure.span).unwrap(), entry["span"]);
    }
}

#[test]
fn namespace_proofs_compose_with_both_strict_headers_and_actual_scoped_emission() {
    for (source, model, caller) in [
        (
            "fn keep(value:&f64)->f64{*value}fn main(){}",
            "primitive borrowed Rust function header 1",
            "keep",
        ),
        (
            "fn keep(value:&f64)->f64{*value}fn call(value:f64)->f64{keep(&value)}fn main(){}",
            "closed primitive borrowed Rust call interface 1",
            "call",
        ),
    ] {
        let selected = create_typed_program_links_with_rust_type_namespace_model(
            source,
            "Rust",
            "resolved Rust type namespace 1",
            None,
            None,
        )
        .unwrap();
        let loaded = LinkNetwork::from_lino(&selected.network.to_lino()).unwrap();
        let program = decode_typed_program_links(&loaded, selected.root).unwrap();
        let headers = emit_typed_program_links_with_options(
            &loaded,
            selected.root,
            "Rust",
            &TypedProgramLinksEmissionOptions {
                rust_source_header_emission_model: Some(model.to_owned()),
                ..TypedProgramLinksEmissionOptions::default()
            },
        )
        .unwrap();
        assert!(headers.code.contains("value: &f64"));
        let scoped = emit_typed_program_links_with_options(
            &loaded,
            selected.root,
            "JavaScript",
            &TypedProgramLinksEmissionOptions {
                java_script_scoped_binding_emission_model: Some(
                    "owned JavaScript scoped bindings 1".to_owned(),
                ),
                ..TypedProgramLinksEmissionOptions::default()
            },
        )
        .unwrap();
        let name = &scoped
            .mappings
            .iter()
            .find(|mapping| mapping.source == caller)
            .unwrap()
            .target;
        assert_eq!(
            observe(&format!("{}\nconsole.log({name}(3));", scoped.code)),
            ["3"]
        );
        let mut malformed = program;
        let Decl::Fn(function) = malformed
            .declarations
            .iter_mut()
            .find(|entry| entry.full_name() == "keep")
            .unwrap()
        else {
            panic!("the actual captured leaf exists")
        };
        function.source_type_namespace = Some(Value::Bool(true));
        assert!(encode_typed_program_links(&malformed).is_err());
    }
}

#[test]
fn native_emitted_modules_execute_all_fifteen_shared_observations_in_ci() {
    let data = fixture();
    let temporary = TemporaryDirectory::new("owned-type-namespace");
    for entry in data["cases"].as_array().unwrap() {
        let id = entry["id"].as_str().unwrap();
        let (network, root) = archive(entry);
        let source = temporary.0.join(format!("{id}.rs"));
        let binary = temporary
            .0
            .join(format!("{id}{}", std::env::consts::EXE_SUFFIX));
        fs::write(
            &source,
            emit_typed_program_links(&network, root, "Rust")
                .unwrap()
                .code,
        )
        .unwrap();
        let mut build_command = Command::new("rustc");
        build_command.args(["--edition", "2024", "--crate-name", "owned_type_namespace"]);
        #[cfg(windows)]
        if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
            build_command.arg("-C").arg(format!("linker={linker}"));
        }
        let compiled = run(build_command.arg(&source).arg("-o").arg(&binary));
        assert!(compiled.ok, "{id}: {}", compiled.stderr);
        let observed = run(&mut Command::new(&binary));
        assert!(observed.ok, "{id}: {}", observed.stderr);
        assert_eq!(
            observed
                .stdout
                .lines()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            expected(entry),
            "{id}"
        );
    }
}

#[test]
fn actual_unit_clone_derivations_retain_attached_traits_and_reject_foreign_facts() {
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-type-namespace-traits.lino"
    ))
    .unwrap();
    let model = data["typeNamespaceModel"].as_str().unwrap();
    assert_eq!(data["cases"].as_array().unwrap().len(), 2);
    for entry in data["cases"].as_array().unwrap() {
        let selected = create_typed_program_links_with_rust_type_namespace_model(
            entry["source"].as_str().unwrap(),
            "Rust",
            model,
            None,
            None,
        )
        .unwrap();
        let native = LinkNetwork::from_lino(&selected.network.to_lino()).unwrap();
        let (shared, shared_root) = archive(entry);
        for (network, root) in [(&native, selected.root), (&shared, shared_root)] {
            let decoded = decode_typed_program_links(network, root).unwrap();
            let Some(Decl::Data(datatype)) = decoded.declaration("Choice") else {
                panic!("the actual datatype exists");
            };
            assert_eq!(
                datatype.source_unit_clone_derivation.as_ref().unwrap(),
                &entry["expected"]
            );
            assert_eq!(
                observe(
                    &emit_typed_program_links(network, root, "JavaScript")
                        .unwrap()
                        .code
                ),
                expected(entry)
            );
            let emitted = emit_rust(&decoded).unwrap();
            let source = format!("{}\nfn main(){{}}", emitted.definitions.join("\n"));
            let selected = create_typed_program_links_with_rust_type_namespace_model(
                &source, "Rust", model, None, None,
            )
            .unwrap();
            let loaded = decode_typed_program_links(&selected.network, selected.root).unwrap();
            assert!(loaded.declarations.iter().any(|entry| matches!(entry, Decl::Data(data) if data.source_unit_clone_derivation.is_some())));
        }
    }
    assert_eq!(data["invalid"].as_array().unwrap().len(), 6);
    for entry in data["invalid"].as_array().unwrap() {
        assert!(
            parse_rust(entry["source"].as_str().unwrap())
                .and_then(|surface| check_program(&surface))
                .is_ok(),
            "{}: legacy",
            entry["id"]
        );
        let failure =
            parse_rust_with_type_namespace_model(entry["source"].as_str().unwrap(), model, "")
                .and_then(|surface| check_program(&surface))
                .unwrap_err();
        assert_eq!(
            failure.kind.as_str(),
            entry["kind"].as_str().unwrap(),
            "{}",
            entry["id"]
        );
        assert_eq!(
            failure.reason,
            entry["reason"].as_str().unwrap(),
            "{}",
            entry["id"]
        );
        assert_eq!(
            serde_json::to_value(failure.span).unwrap(),
            entry["span"],
            "{}",
            entry["id"]
        );
    }
    assert_eq!(data["malformed"].as_array().unwrap().len(), 3);
    for entry in data["malformed"].as_array().unwrap() {
        let (network, root) = archive(entry);
        for target in ["JavaScript", "Rust"] {
            let failure = emit_typed_program_links(&network, root, target).unwrap_err();
            assert!(
                failure.to_string().contains("unit Clone derivation"),
                "{}:{target}: {failure}",
                entry["id"]
            );
        }
    }
}

fn container_at(network: &LinkNetwork, mut current: LinkId, path: &[Value]) -> LinkId {
    let schema = meta_language::typed_program_links_schema();
    for segment in path {
        if let Some(index) = segment.as_u64() {
            current = network.link(current).unwrap().references()[usize::try_from(index).unwrap()];
        } else if network.link(current).unwrap().metadata().term()
            == schema["containers"]["map"].as_str()
        {
            current = network
                .link(current)
                .unwrap()
                .references()
                .iter()
                .find_map(|pair| {
                    let children = network.link(*pair).unwrap().references();
                    (network.link(children[0]).unwrap().metadata().definition() == segment.as_str())
                        .then_some(children[1])
                })
                .unwrap();
        } else {
            let name = segment.as_str().unwrap();
            current = field(
                network,
                current,
                schema["fields"][name].as_str().unwrap_or(name),
            );
        }
    }
    current
}

#[test]
fn raw_links_preserve_every_shared_container_location_before_rendering() {
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-type-namespace-containers.lino"
    ))
    .unwrap();
    assert_eq!(data["cases"].as_array().unwrap().len(), 2);
    assert_eq!(data["locations"].as_array().unwrap().len(), 27);
    for location in data["locations"].as_array().unwrap() {
        let entry = data["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["id"] == location["caseId"])
            .unwrap();
        let (mut network, root) = archive(entry);
        assert!(decode_typed_program_links(&network, root).is_ok());
        let container = container_at(&network, root, location["path"].as_array().unwrap());
        let kind = location["kind"].as_str().unwrap();
        assert_eq!(
            network.link(container).unwrap().metadata().term(),
            meta_language::typed_program_links_schema()["containers"][kind].as_str()
        );
        let scalar = field(&network, root, "source language");
        network.insert_field(container, "opaque container fact", scalar);
        let loaded = LinkNetwork::from_lino(&network.to_lino()).unwrap();
        for target in ["JavaScript", "Rust"] {
            let failure = emit_typed_program_links(&loaded, root, target).unwrap_err();
            if kind != "record" {
                assert!(
                    failure
                        .to_string()
                        .contains("ordered program container contains named fields"),
                    "{location}:{target}: {failure}"
                );
            }
        }
        assert!(
            decode_typed_program_links(&loaded, root).is_err(),
            "{location}"
        );
    }
}

#[test]
fn selected_module_datatypes_and_functions_recheck_without_relaxing_legacy_modules() {
    let source = "mod nested{enum Choice{Only}fn keep(value:Choice)->Choice{value}}fn main(){}";
    let legacy = check_program(&parse_rust(source).unwrap()).unwrap();
    assert!(
        encode_typed_program_links(&legacy)
            .unwrap_err()
            .to_string()
            .contains("does not yet recheck module items")
    );
    let selected = create_typed_program_links_with_rust_type_namespace_model(
        source,
        "Rust",
        "resolved Rust type namespace 1",
        None,
        None,
    )
    .unwrap();
    let loaded = LinkNetwork::from_lino(&selected.network.to_lino()).unwrap();
    let decoded = decode_typed_program_links(&loaded, selected.root).unwrap();
    assert!(matches!(
        decoded.declaration("nested.Choice"),
        Some(Decl::Data(_))
    ));
    assert!(
        function(&decoded, "nested.keep")
            .source_type_namespace
            .is_some()
    );
}

#[test]
fn present_namespace_record_facts_refuse_normalization_and_retain_genuine_optional_absence() {
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-type-namespace-records.lino"
    ))
    .unwrap();
    assert_eq!(data["malformed"].as_array().unwrap().len(), 26);
    for entry in data["malformed"].as_array().unwrap() {
        let (network, root) = archive(entry);
        assert!(
            decode_typed_program_links(&network, root).is_err(),
            "{}",
            entry["id"]
        );
        for target in ["JavaScript", "Rust"] {
            assert!(
                emit_typed_program_links(&network, root, target).is_err(),
                "{}:{target}",
                entry["id"]
            );
        }
    }
    if let Ok(end) = usize::try_from(9_007_199_254_740_992_u64) {
        let (network, root) = archive(&data["base"]);
        let mut program = decode_typed_program_links(&network, root).unwrap();
        let Some(Decl::Fn(function)) = program
            .declarations
            .iter_mut()
            .find(|entry| entry.name() == "keep")
        else {
            panic!("the selected function exists")
        };
        function.span = Some(meta_language::translation::Span::new(0, end));
        assert!(encode_typed_program_links(&program).is_err());
    }
    for entry in [&data["base"], &data["optionalAbsence"]] {
        let (network, root) = archive(entry);
        let program = decode_typed_program_links(&network, root).unwrap();
        let rebuilt = encode_typed_program_links(&program).unwrap();
        let loaded = LinkNetwork::from_lino(&rebuilt.network.to_lino()).unwrap();
        assert_eq!(
            observe(
                &emit_typed_program_links(&loaded, rebuilt.root, "JavaScript")
                    .unwrap()
                    .code
            ),
            expected(entry)
        );
    }
}

#[test]
fn direct_source_free_namespace_emitters_reject_forged_owned_header_facts() {
    let data = fixture();
    let entry = &data["cases"][0];
    let (network, root) = archive(entry);
    let original = decode_typed_program_links(&network, root).unwrap();
    assert!(emit_rust(&original).is_ok());
    assert!(emit_javascript(&original).is_ok());
    for mutation in 0..4 {
        let mut program = original.clone();
        let Some(Decl::Fn(function)) = program
            .declarations
            .iter_mut()
            .find(|declaration| declaration.full_name() == entry["functionName"].as_str().unwrap())
        else {
            panic!("the actual selected function exists")
        };
        match mutation {
            0 => function.source_type_namespace = Some(Value::Null),
            1 => {
                function.source_type_namespace.as_mut().unwrap()["result"] = json!(["unknown"]);
            }
            2 => {
                function.source_type_namespace.as_mut().unwrap()["parameters"][0] = json!(["bool"]);
            }
            _ => function.generated = true,
        }
        assert!(emit_rust(&program).is_err(), "Rust mutation {mutation}");
        assert!(
            emit_javascript(&program).is_err(),
            "JavaScript mutation {mutation}"
        );
    }
}
