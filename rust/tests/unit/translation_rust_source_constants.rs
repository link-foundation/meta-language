//! Deferred actual original/native constant values, complete definitions and codecs.
#[test]
fn source_constant_values_remain_distinct_from_function_items() {
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered source usize model in constant fixtures"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/source-constants.mjs",
        "Rust source constant values and actual callable constants",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-source-constant-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
