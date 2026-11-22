use meta_language::translation::surface::SExpr;
use meta_language::{
    RustStandardCollectionLibraryLimits, RustStandardCollectionSourceHandlers,
    create_rust_standard_collection_source_library, load_rust_standard_collection_source_library,
};
use serde_json::json;

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
