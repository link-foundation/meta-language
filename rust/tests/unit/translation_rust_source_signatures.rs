//! Owned actual header facts; local checks never run a native compiler.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::ir::{Decl, Program};
use meta_language::translation::output::thread_output;
use meta_language::translation::rust::{
    parse_rust, parse_rust_with_source_profiles, parse_rust_with_source_signature_model,
};
use meta_language::{
    LinkId, LinkNetwork, LinkType, create_typed_program_links_with_rust_source_profiles,
    create_typed_program_links_with_rust_source_signature_model, decode_typed_program_links,
    emit_typed_program_links,
};
use serde_json::Value;
use std::fs;
use std::process::Command;

fn fixture() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-source-signatures.lino"
    ))
    .unwrap()
}
fn signature(program: &Program, name: &str) -> Value {
    let Some(Decl::Fn(function)) = program.declaration(name) else {
        panic!("an owned function exists")
    };
    serde_json::to_value(function.source_signature.as_ref().unwrap()).unwrap()
}
fn field(network: &LinkNetwork, parent: LinkId, label: &str) -> Option<LinkId> {
    network.links().find_map(|link| match link.references() {
        [owner, name, child]
            if link.metadata().link_type() == Some(LinkType::Field)
                && *owner == parent
                && network.link(*name).and_then(|name| name.metadata().term()) == Some(label) =>
        {
            Some(*child)
        }
        _ => None,
    })
}

#[test]
fn actual_rust_header_owners_qualifiers_and_positions_survive_both_runtime_archives() {
    let data = fixture();
    let model = data["sourceSignatureModel"].as_str().unwrap();
    assert_eq!(data["cases"].as_array().unwrap().len(), 19);
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let source = case["source"].as_str().unwrap();
        let name = case["functionName"].as_str().unwrap();
        let surface = parse_rust_with_source_signature_model(source, model)
            .unwrap_or_else(|error| panic!("{id}: {error}"));
        let checked = check_program(&surface).unwrap();
        assert_eq!(signature(&checked, name), case["expected"], "{id}");
        let user = serde_json::json!({"project":id});
        let native = create_typed_program_links_with_rust_source_signature_model(
            source,
            "Rust",
            model,
            Some(&user),
        )
        .unwrap();
        let native_network = LinkNetwork::from_lino(&native.network.to_lino()).unwrap();
        let shared = LinkNetwork::from_lino(case["archive"].as_str().unwrap()).unwrap();
        for (network, root) in [
            (&native_network, native.root),
            (&shared, LinkId::from_u64(case["root"].as_u64().unwrap())),
        ] {
            assert_eq!(
                signature(&decode_typed_program_links(network, root).unwrap(), name),
                case["expected"],
                "{id}"
            );
            assert!(
                network.links().all(|link| !link
                    .metadata()
                    .definition()
                    .is_some_and(|text| text.contains(source))),
                "{id}"
            );
        }
        let default = check_program(&parse_rust(source).unwrap()).unwrap();
        assert!(
            default.declarations.iter().all(
                |entry| !matches!(entry,Decl::Fn(function) if function.source_signature.is_some())
            ),
            "{id}"
        );
    }
}

#[test]
fn captured_type_owner_and_qualifier_refusals_remain_exact_and_malformed_graphs_refuse() {
    let data = fixture();
    let model = data["sourceSignatureModel"].as_str().unwrap();
    assert_eq!(data["invalid"].as_array().unwrap().len(), 7);
    for case in data["invalid"].as_array().unwrap() {
        let result =
            parse_rust_with_source_signature_model(case["source"].as_str().unwrap(), model)
                .and_then(|surface| check_program(&surface));
        let error = result.unwrap_err();
        assert_eq!(error.kind.as_str(), case["kind"].as_str().unwrap());
        assert_eq!(error.reason, case["reason"].as_str().unwrap());
        assert_eq!(serde_json::to_value(error.span).unwrap(), case["span"]);
    }
    assert_eq!(data["malformed"].as_array().unwrap().len(), 10);
    for case in data["malformed"].as_array().unwrap() {
        let network = LinkNetwork::from_lino(case["archive"].as_str().unwrap()).unwrap();
        let root = LinkId::from_u64(case["root"].as_u64().unwrap());
        for target in ["JavaScript", "Rust"] {
            assert!(
                emit_typed_program_links(&network, root, target).is_err(),
                "{}: {target}",
                case["id"]
            );
        }
    }
}

#[test]
fn selected_models_and_caller_provenance_are_independently_preserved() {
    let data = fixture();
    let model = data["sourceSignatureModel"].as_str().unwrap();
    let source = "fn values(flag:bool)->Vec<f64>{if flag{return vec![1.0];}vec![2.0]}fn main(){println!(\"{}\",values(true).len() as f64==1.0);}";
    let array = "immutable fresh scalar vector results 1";
    let completion = "closed guarded function returns 1";
    let surface = parse_rust_with_source_profiles(source, array, completion, model).unwrap();
    assert!(check_program(&surface).is_ok());
    let user = serde_json::json!({"project":"selected header"});
    let archive = create_typed_program_links_with_rust_source_profiles(
        source,
        "Rust",
        Some(array),
        Some(completion),
        Some(model),
        Some(&user),
    )
    .unwrap();
    let network = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
    let provenance = field(&network, archive.root, "provenance").unwrap();
    let profile = field(&network, provenance, "source profile").unwrap();
    for (label, expected) in [
        ("rust source signature model", model),
        ("rust completion model", completion),
        ("rust array result model", array),
    ] {
        assert_eq!(
            network
                .link(field(&network, profile, label).unwrap())
                .unwrap()
                .metadata()
                .definition(),
            Some(expected)
        );
    }
    let user = field(&network, provenance, "user provenance").unwrap();
    assert_eq!(
        network
            .link(field(&network, user, "project").unwrap())
            .unwrap()
            .metadata()
            .definition(),
        Some("selected header")
    );
    assert!(parse_rust_with_source_signature_model("fn main(){}", "unknown").is_err());
    assert!(
        parse_rust_with_source_profiles("fn main(){}", "unknown", "", "unknown")
            .unwrap_err()
            .reason
            .contains("Rust array result model")
    );
    assert!(
        parse_rust_with_source_profiles("fn main(){}", "", "unknown", "unknown")
            .unwrap_err()
            .reason
            .contains("Rust completion model")
    );
    assert!(
        create_typed_program_links_with_rust_source_signature_model(
            "function keep(value){return value;}",
            "JavaScript",
            model,
            None
        )
        .is_err()
    );
    let effectful = "fn keep(value:bool)->bool{if value{true}else{panic!(\"message\")}}fn main(){}";
    let checked =
        check_program(&parse_rust_with_source_signature_model(effectful, model).unwrap()).unwrap();
    assert!(
        thread_output(&checked)
            .unwrap_err()
            .reason
            .contains("effect threading changes the captured source header")
    );
    assert!(thread_output(&check_program(&parse_rust(effectful).unwrap()).unwrap()).is_ok());
}

#[test]
fn source_free_body_edit_changes_the_independent_runtime_witness_without_erasing_borrow_facts() {
    let data = fixture();
    let original = data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["id"] == "borrowed-float")
        .unwrap();
    let edited = &data["valueEdit"];
    let original_network = LinkNetwork::from_lino(original["archive"].as_str().unwrap()).unwrap();
    let edited_network = LinkNetwork::from_lino(edited["archive"].as_str().unwrap()).unwrap();
    let root = LinkId::from_u64(edited["root"].as_u64().unwrap());
    assert_eq!(
        signature(
            &decode_typed_program_links(&edited_network, root).unwrap(),
            "keep"
        ),
        original["expected"]
    );
    let directory = TemporaryDirectory::new("rust-source-signature");
    let path = &directory.0;
    let source = path.join("original.rs");
    let binary = path.join(format!("original{}", std::env::consts::EXE_SUFFIX));
    fs::write(&source, original["source"].as_str().unwrap()).unwrap();
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    compiler.arg(&source).arg("-o").arg(&binary);
    let compiled = run(&mut compiler);
    assert!(compiled.ok, "{}", compiled.stderr);
    let original_result = run(&mut Command::new(&binary));
    assert!(original_result.ok, "{}", original_result.stderr);
    assert_eq!(original_result.stdout, "true\n");
    for (network, expected) in [(&original_network, "true\n"), (&edited_network, "false\n")] {
        let code = emit_typed_program_links(network, root, "JavaScript")
            .unwrap()
            .code;
        let script = path.join("linked.js");
        fs::write(&script, code).unwrap();
        let result = run(Command::new("node").arg(&script));
        assert!(result.ok, "{}", result.stderr);
        assert_eq!(result.stdout, expected);
    }
}

#[test]
fn untrusted_expression_fields_consume_the_shared_closed_field_policy() {
    let data = fixture();
    let model = data["sourceSignatureModel"].as_str().unwrap();
    let archive = create_typed_program_links_with_rust_source_signature_model(
        "fn keep()->(f64,bool){(5.0,true)}fn main(){}",
        "Rust",
        model,
        None,
    )
    .unwrap();
    let mut network = LinkNetwork::from_lino(&archive.network.to_lino()).unwrap();
    let expression = network
        .links()
        .find_map(|link| match link.references() {
            [owner, name, child]
                if link.metadata().link_type() == Some(LinkType::Field)
                    && network.link(*name).and_then(|name| name.metadata().term())
                        == Some("construct kind")
                    && network
                        .link(*child)
                        .and_then(|child| child.metadata().definition())
                        == Some("tuple") =>
            {
                Some(*owner)
            }
            _ => None,
        })
        .unwrap();
    let existing = field(&network, expression, "constructor immutability").unwrap();
    network.insert_field(expression, "unrecognizedSemanticField", existing);
    let reloaded = LinkNetwork::from_lino(&network.to_lino()).unwrap();
    let error = decode_typed_program_links(&reloaded, archive.root).unwrap_err();
    assert!(error.to_string().contains("an unrecognized semantic field"));
}
