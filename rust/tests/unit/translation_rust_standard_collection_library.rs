use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::{
    emit_javascript::emit_javascript,
    emit_rust::emit_rust,
    ir::Decl,
    rust::parse_rust_with_standard_collection_source_model,
    source_signatures::{
        SourceSignature, SourceTypeSignature, validate_source_signature_with_data,
    },
    surface::{SExpr, SNode},
    types::Type,
};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, RustStandardCollectionLibraryLimits,
    RustStandardCollectionSourceHandlers, TypedProgramLinksLimits,
    create_rust_standard_collection_source_library,
    create_typed_program_links_with_rust_exclusive_source_model,
    create_typed_program_links_with_rust_standard_collection_source_model,
    decode_link_network_binary, decode_typed_program_links_with_limits, encode_link_network_binary,
    load_rust_standard_collection_source_library,
};
use serde_json::json;
use std::{fs, process::Command, rc::Rc};

const MODEL: &str = "Rust closed standard collections 1";

#[test]
fn actual_owned_vector_heads_use_the_same_ordered_loop_transport() {
    let rust = run(Command::new("rustc").arg("--version"));
    assert!(rust.ok, "{}", rust.stderr);
    let version = rust.stdout.split_whitespace().nth(1).unwrap();
    let model = "Rust exclusive source loans 1";
    let source = r#"fn main() {
        let values = vec![7u16, 1u16];
        for value in (values.into_iter()) { println!("{}", value); }
        for value in vec![3u16, 9u16] { println!("{}", value); break; }
        for (mut value, label) in vec![(4u16, true)].into_iter() {
            value += 2u16;
            println!("{}", value);
            println!("{}", label);
        }
        for mut text in vec![String::from("z")] {
            text.push_str("!");
            println!("{}", text);
        }
    }"#;
    let packet =
        create_typed_program_links_with_rust_exclusive_source_model(source, model, version, None)
            .unwrap_or_else(|error| panic!("owned vector traversal under Rust {version}: {error}"));
    let checked = decode_typed_program_links_with_limits(
        &packet.network,
        packet.root,
        TypedProgramLinksLimits::default(),
    )
    .unwrap();
    let original = compile_and_observe(source);
    assert_eq!(original, "7\n1\n3\n6\ntrue\nz!\n");
    assert_eq!(
        compile_and_observe(&emit_rust(&checked).unwrap().text),
        original
    );
    let directory = TemporaryDirectory::new("javascript-owned-vector");
    let output = directory.0.join("vector.mjs");
    fs::write(&output, emit_javascript(&checked).unwrap().text).unwrap();
    let execution = run(Command::new("node").arg(output));
    assert!(execution.ok, "{}", execution.stderr);
    assert_eq!(execution.stdout, original);
    for invalid in [
        r#"fn main() { let values = vec![7u16]; for _value in values {} println!("{}", values.len()); }"#,
        r#"fn main() { let values = vec![7u16]; for _value in values.into_iter() { break; } println!("{}", values.len()); }"#,
        r#"fn main() { let values = vec![7u16]; for _value in &values {} }"#,
        r#"fn main() { let values = vec![7u16]; let iter = values.into_iter(); for _value in iter {} }"#,
    ] {
        assert!(
            create_typed_program_links_with_rust_exclusive_source_model(
                invalid, model, version, None
            )
            .is_err(),
            "{invalid}"
        );
    }
    for uncovered in ["1.89.0", "1.100.0"] {
        assert!(
            create_typed_program_links_with_rust_exclusive_source_model(
                source, model, uncovered, None
            )
            .unwrap_err()
            .to_string()
            .contains("Rust >=1.90.0 <1.100.0"),
            "{uncovered}"
        );
    }
}

#[test]
fn whole_box_assignment_preserves_the_actual_closed_owner_layers() {
    let rust = run(Command::new("rustc").arg("--version"));
    assert!(rust.ok, "{}", rust.stderr);
    let version = rust.stdout.split_whitespace().nth(1).unwrap();
    let source = r#"fn main() {
        let mut value: Box<Box<u16>> = Box::new(Box::new(7u16));
        println!("{}", **value);
        value = Box::new(Box::new(11u16));
        println!("{}", **value);
    }"#;
    let model = "Rust exclusive source loans 1";
    let packet =
        create_typed_program_links_with_rust_exclusive_source_model(source, model, version, None)
            .unwrap_or_else(|error| panic!("closed Box state under Rust {version}: {error}"));
    let checked = decode_typed_program_links_with_limits(
        &packet.network,
        packet.root,
        TypedProgramLinksLimits::default(),
    )
    .unwrap();
    let original = compile_and_observe(source);
    assert_eq!(original, "7\n11\n");
    assert_eq!(
        compile_and_observe(&emit_rust(&checked).unwrap().text),
        original
    );
    let replaced = source.replace("Box::new(Box::new(11u16))", "Box::new(11u16)");
    assert!(
        create_typed_program_links_with_rust_exclusive_source_model(
            &replaced, model, version, None,
        )
        .is_err()
    );
}

fn abort() -> SExpr {
    SExpr::new(
        SNode::Abort {
            message: "invalid standard collection packet".to_owned(),
        },
        None,
    )
}

fn compile_and_observe(source: &str) -> String {
    let directory = TemporaryDirectory::new("rust-standard-collections");
    let input = directory.0.join("collections.rs");
    let binary = directory.0.join("collections");
    fs::write(&input, source).unwrap();
    let build = run(Command::new("rustc")
        .args(["--edition", "2024"])
        .arg(&input)
        .arg("-o")
        .arg(&binary));
    assert!(build.ok, "{}", build.stderr);
    let execution = run(&mut Command::new(binary));
    assert!(execution.ok, "{}", execution.stderr);
    execution.stdout
}

#[test]
fn actual_standard_collection_source_survives_both_links_transports() {
    let node = run(Command::new("node").args(["-p", "process.versions.node"]));
    assert!(node.ok, "{}", node.stderr);
    let mut node_parts = node.stdout.trim().split('.');
    let node_major = node_parts.next().unwrap().parse::<u32>().unwrap();
    let node_minor = node_parts.next().unwrap().parse::<u32>().unwrap();
    assert!(
        node_major == 22 && node_minor >= 15 || node_major == 24,
        "uncovered Node {}; supported >=22.15 <23 or >=24 <25",
        node.stdout.trim()
    );
    let rust = run(Command::new("rustc").arg("--version"));
    assert!(rust.ok, "{}", rust.stderr);
    let version = rust.stdout.split_whitespace().nth(1).unwrap();
    let source = include_str!("../../../examples/rust-standard-collections.rs");
    let library = Rc::new(
        load_rust_standard_collection_source_library(
            RustStandardCollectionSourceHandlers {
                invalid: |_| abort(),
                resource: |_, _| abort(),
            },
            Default::default(),
        )
        .unwrap(),
    );
    let packet = create_typed_program_links_with_rust_standard_collection_source_model(
        source,
        MODEL,
        version,
        64,
        Rc::clone(&library),
        None,
    )
    .unwrap_or_else(|error| {
        panic!("actual standard collection source under Rust {version}: {error:?}")
    });
    let limits = TypedProgramLinksLimits {
        node_limit: 1_000_000,
        ..TypedProgramLinksLimits::default()
    };
    let checked =
        decode_typed_program_links_with_limits(&packet.network, packet.root, limits).unwrap();
    let collection = checked
        .declarations
        .iter()
        .find_map(|declaration| match declaration {
            Decl::Data(data)
                if data
                    .name
                    .starts_with("RustStandardCollectionBTreeMap_u8_u64_") =>
            {
                Some(data)
            }
            _ => None,
        })
        .expect("source fixture's actual map carrier");
    let ty = Type::Data {
        name: collection.full_name.clone(),
    };
    let carrier = serde_json::to_value(&collection.source_standard_collection_carrier).unwrap();
    let mut signature = SourceSignature {
        model: "Rust source function signature 2".into(),
        language: "Rust".into(),
        parameters: Vec::new(),
        lifetime_bindings: Vec::new(),
        result: SourceTypeSignature {
            ty: ty.clone(),
            wrappers: Vec::new(),
            lifetimes: Vec::new(),
            owner: serde_json::from_value(carrier["origin"]["sourceOwner"].clone()).unwrap(),
            array_element_position: None,
        },
    };
    let validate_signature = |signature: &SourceSignature| {
        validate_source_signature_with_data(
            signature,
            &[],
            &ty,
            "Rust",
            |ty| Ok(ty.clone()),
            |ty| match ty {
                Type::Data { name } => match checked.declaration(name) {
                    Some(Decl::Data(data)) => Some(data),
                    _ => None,
                },
                _ => None,
            },
        )
    };
    validate_signature(&signature).unwrap();
    signature.result.owner.arguments.swap(0, 1);
    assert!(validate_signature(&signature).is_err());
    let options = BinaryLinksOptions::default();
    let binary = encode_link_network_binary(&packet.network, options).unwrap();
    let original = compile_and_observe(source);
    assert_eq!(original, "1\n");
    for transport in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(&binary, options).unwrap(),
    ] {
        let restored =
            decode_typed_program_links_with_limits(&transport, packet.root, limits).unwrap();
        assert_eq!(restored, checked);
        let directory = TemporaryDirectory::new("javascript-standard-collections");
        let input = directory.0.join("collections.mjs");
        fs::write(&input, emit_javascript(&restored).unwrap().text).unwrap();
        let execution = run(Command::new("node").arg(input));
        assert!(execution.ok, "{}", execution.stderr);
        assert_eq!(execution.stdout, original);
        assert_eq!(
            compile_and_observe(&emit_rust(&restored).unwrap().text),
            original
        );
    }
    for (source, expected) in [
        (
            include_str!("../../../examples/rust-consuming-ordered-collections.rs"),
            "a first\nz last\n1\ndone\n",
        ),
        (
            include_str!("../../../examples/rust-borrowed-ordered-collections.rs"),
            "1\n1\n3\n7\n1 8\n2 7\n1\n2\n8\n7\n2\n",
        ),
    ] {
        let packet = create_typed_program_links_with_rust_standard_collection_source_model(
            source,
            MODEL,
            version,
            64,
            Rc::clone(&library),
            None,
        )
        .unwrap_or_else(|error| {
            panic!("actual ordered BTree source under Rust {version}: {error:?}")
        });
        let checked =
            decode_typed_program_links_with_limits(&packet.network, packet.root, limits).unwrap();
        let original = compile_and_observe(source);
        assert_eq!(original, expected);
        assert_eq!(
            compile_and_observe(&emit_rust(&checked).unwrap().text),
            original
        );
        let directory = TemporaryDirectory::new("javascript-ordered-collections");
        let output = directory.0.join("ordered.mjs");
        fs::write(&output, emit_javascript(&checked).unwrap().text).unwrap();
        let execution = run(Command::new("node").arg(output));
        assert!(execution.ok, "{}", execution.stderr);
        assert_eq!(execution.stdout, original);
    }
    for rejected in [
        "use std::collections::BTreeSet; fn main() { let set:BTreeSet<u8> = BTreeSet::new(); for _value in set {} println!(\"{}\", set.len()); }",
        "use std::collections::BTreeSet; fn main() { let mut set:BTreeSet<u8> = BTreeSet::new(); set.insert(1u8); for _value in set.into_iter() { break; } println!(\"{}\", set.len()); }",
        "use std::collections::BTreeSet; fn main() { let set:BTreeSet<u8> = BTreeSet::new(); for item in (*(&set)).into_iter() { println!(\"{}\", item); } }",
        "use std::collections::BTreeSet; fn main() { let mut set:BTreeSet<u8> = BTreeSet::new(); for &item in set.iter() { set.insert(item); } }",
        "use std::collections::BTreeSet; fn main() { let set:BTreeSet<String> = BTreeSet::new(); for &item in &set { println!(\"{}\", item); } }",
        "use std::collections::BTreeSet; fn main() { let set:BTreeSet<u8> = BTreeSet::new(); for item in &set.iter() { println!(\"{}\", item); } }",
    ] {
        assert!(
            parse_rust_with_standard_collection_source_model(
                rejected,
                MODEL,
                version,
                64,
                Rc::clone(&library)
            )
            .is_err(),
            "{rejected}"
        );
    }
    for (input, input_version, width) in [
        (source, "1.89.0", 64),
        (source, version, 32),
        (
            "use std::collections::BTreeSet; fn f(values: &BTreeSet<u8>) -> bool { values.insert(1u8) } fn main() {}",
            version,
            64,
        ),
        (
            "use std::collections::HashSet; fn f(left: HashSet<u8>, right: HashSet<u8>) -> bool { left < right } fn main() {}",
            version,
            64,
        ),
        (
            "use std::collections::BTreeSet; fn main() { let mut values: BTreeSet<u8> = BTreeSet::new(); values.insert(&1u8); }",
            version,
            64,
        ),
        (
            "use std::collections::BTreeSet; fn main() { let values: BTreeSet<u8> = BTreeSet::new(); println!(\"{:?}\", values); }",
            version,
            64,
        ),
        (
            "use std::collections::BTreeSet; struct Wrapped { values: BTreeSet<u8> } fn f(left: Wrapped, right: Wrapped) -> bool { left == right } fn main() {}",
            version,
            64,
        ),
    ] {
        assert!(
            parse_rust_with_standard_collection_source_model(
                input,
                MODEL,
                input_version,
                width,
                Rc::clone(&library)
            )
            .is_err(),
            "{input}"
        );
    }
}

fn handlers()
-> RustStandardCollectionSourceHandlers<impl Fn(SExpr) -> SExpr, impl Fn(SExpr, SExpr) -> SExpr> {
    RustStandardCollectionSourceHandlers {
        invalid: |reason| reason,
        resource: |_state, reason| reason,
    }
}

#[test]
fn installed_collection_source_rechecks_complete_owned_bodies() {
    let library =
        load_rust_standard_collection_source_library(handlers(), Default::default()).unwrap();
    assert_eq!(
        library.callable_names(),
        [
            "executeRustStandardCollectionValuePacket",
            "executeRustStandardCollectionOperation",
            "compareRustStandardCollectionValuePackets",
        ]
    );
    for name in library.callable_names() {
        assert!(library.program().declaration(name).is_some());
    }
    let (name, constructors) = library.transition_type();
    assert!(library.program().declaration(name).is_some());
    assert_eq!(
        constructors,
        [
            "rustCollectionPacketNormal",
            "rustCollectionInvalid",
            "rustCollectionResourceLimit",
        ]
    );
}

#[test]
fn source_library_rejects_missing_modules_and_incomplete_budgets() {
    let receipt = json!({
        "model": "Complete source library asset receipt 1",
        "schemaVersion": 1,
        "entryPath": "js/src/translation/rust-standard-collection-value-packet.js",
        "referenceSemantics": false,
        "programAsset": {
            "path": "js/src/data/rust-standard-collection-value-packet-program.lino",
            "bytes": 0,
            "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        },
        "sourceClosure": { "inputs": [] },
    });
    assert!(
        create_rust_standard_collection_source_library(
            "",
            &receipt,
            handlers(),
            Default::default()
        )
        .err()
        .is_some_and(|error| error.contains("at most 128 actual source modules"))
    );
    let limits = RustStandardCollectionLibraryLimits {
        reference_limit: 0,
        ..Default::default()
    };
    assert!(
        create_rust_standard_collection_source_library("", &receipt, handlers(), limits)
            .err()
            .is_some_and(|error| error.contains("positive budgets"))
    );
}
