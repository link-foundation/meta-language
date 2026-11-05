//! Deferred actual generated rules and complete source products; CI only.
#[path = "../../src/translation/rust_fresh_vector_argument_rules.rs"]
mod vector_rules;

#[test]
fn fresh_scalar_vector_arguments_keep_complete_original_products() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/fresh-vector-arguments.mjs",
        "ordinary fresh Rust scalar vector arguments",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-fresh-vector-argument-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}

#[test]
fn every_declared_freshness_decision_uses_the_actual_shipped_rules() {
    let fixture = meta_language::project_data::read_project_data_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity/fixtures/rust-fresh-vector-arguments.lino"),
    )
    .unwrap();
    for row in fixture["forms"].as_array().unwrap() {
        assert_eq!(
            vector_rules::read_rust_fresh_vector_argument_form(
                row["kind"].as_str().unwrap(),
                row["diverging"].as_bool().unwrap(),
                row["model"].as_str().unwrap(),
                row["wrappers"].as_f64().unwrap(),
                row["constructor"].as_bool().unwrap(),
            ),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["admissions"].as_array().unwrap() {
        assert_eq!(
            vector_rules::accept_rust_fresh_owned_vector_argument(
                row["ordinary"].as_bool().unwrap(),
                row["model"].as_str().unwrap(),
                row["wrappers"].as_f64().unwrap(),
                row["element"].as_str().unwrap(),
                row["fresh"].as_bool().unwrap(),
            ),
            row["expected"].as_bool().unwrap()
        );
    }
}
