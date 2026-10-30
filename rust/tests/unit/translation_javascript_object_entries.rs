//! Future source-language, native product and source-free definition parity.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn own_entry_reads_retain_actual_key_order_payloads_and_source_definitions() {
    assert_nominal_translation_product(
        "../experiments/javascript-program/javascript-object-entries.mjs",
        "JavaScript own-entry value reads",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/object-entry-pair-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
