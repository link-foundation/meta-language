//! Deferred original/generated JavaScript and native source products; CI only.
#[test]
fn typed_array_slice_allocations_preserve_input_and_copy_values() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/javascript-program/fresh-array-slices.mjs",
        "typed standard array slice allocations",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/owned-array-allocation-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
