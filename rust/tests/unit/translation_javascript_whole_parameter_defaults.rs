//! Deferred complete source/definition, codecs, emissions and native products.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn whole_parameter_defaults_preserve_actual_intervals_and_initialization_boundaries() {
    assert_nominal_translation_product(
        "../experiments/javascript-program/whole-parameter-defaults.mjs",
        "JavaScript whole parameter defaults",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/whole-parameter-default-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
