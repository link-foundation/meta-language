//! Deferred actual native rule decisions and complete original source products.
#[path = "../../src/translation/rust_array_index_rules.rs"]
mod index_rules;
#[test]
fn scalar_array_indices_keep_complete_original_products() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/array-indices.mjs",
        "ordinary Rust scalar array reads",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-array-index-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
#[test]
fn every_index_admission_uses_the_actual_shipped_rules() {
    let fixture = meta_language::project_data::read_project_data_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity/fixtures/rust-array-indices.lino"),
    )
    .unwrap();
    for row in fixture["receivers"].as_array().unwrap() {
        assert_eq!(
            index_rules::read_rust_scalar_array_index_receiver_error(
                row["kind"].as_str().unwrap(),
                row["element"].as_str().unwrap(),
                row["model"].as_str().unwrap(),
                row["proven"].as_bool().unwrap(),
                row["layers"].as_bool().unwrap(),
                row["inferredCopy"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["operands"].as_array().unwrap() {
        assert_eq!(
            index_rules::read_rust_scalar_array_index_operand_error(
                row["literal"].as_bool().unwrap(),
                row["kind"].as_str().unwrap(),
                row["bits"].as_f64().unwrap(),
                row["signed"].as_bool().unwrap(),
                row["owner"].as_str().unwrap(),
                row["wrappers"].as_f64().unwrap(),
                row["proven"].as_bool().unwrap()
            ),
            row["expected"].as_str().unwrap()
        );
    }
}
