//! Deferred ordinary source guards and automatically shared nonreturning roles.
#[test]
fn source_exception_branches_keep_actual_values_errors_operands_and_codecs() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/javascript-program/abrupt-results.mjs",
        "nonreturning source result roles",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/abrupt-result-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
