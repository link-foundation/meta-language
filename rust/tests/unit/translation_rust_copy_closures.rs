//! Future native values, exact definition, codecs and every target emission.
#[test]
fn immutable_copy_closures_retain_actual_source_ownership() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/copy-closures.mjs",
        "Rust explicitly typed immutable Copy closures",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-copy-closure-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
