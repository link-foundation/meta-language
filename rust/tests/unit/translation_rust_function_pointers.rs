//! Future original/native values, full definitions, codecs and all emissions.
#[test]
fn safe_function_pointer_values_retain_actual_source_signatures() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/function-pointers.mjs",
        "Rust safe function pointers and free function values",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-function-pointer-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
