//! Deferred actual source contexts, native products, full definitions and codecs.
#[test]
fn contextual_copy_closures_retain_actual_types_and_source_capture_constraints() {
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered source usize width in contextual closure fixtures"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/contextual-closures.mjs",
        "Rust contextual immutable Copy closures",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-contextual-closure-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
