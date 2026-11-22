use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::{
    emit_javascript::emit_javascript,
    emit_rust::emit_rust,
    rust::parse_rust_with_standard_collection_source_model,
    surface::{SExpr, SNode},
};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, RustStandardCollectionLibraryLimits,
    RustStandardCollectionSourceHandlers, TypedProgramLinksLimits,
    create_rust_standard_collection_source_library,
    create_typed_program_links_with_rust_standard_collection_source_model,
    decode_link_network_binary, decode_typed_program_links_with_limits, encode_link_network_binary,
    load_rust_standard_collection_source_library,
};
use serde_json::json;
use std::{fs, process::Command, rc::Rc};

const MODEL: &str = "Rust closed standard collections 1";

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
