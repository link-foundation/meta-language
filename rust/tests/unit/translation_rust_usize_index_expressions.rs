//! Deferred actual shared/native admission and complete original products.
#[path = "../../src/translation/rust_usize_index_expression_rules.rs"]
mod expression_rules;
#[test]
fn contextual_indices_keep_complete_original_products() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/usize-index-expressions.mjs",
        "contextual Rust usize index expressions",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-usize-index-expression-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
#[test]
fn every_admission_uses_the_actual_generated_rules() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let fixture = meta_language::project_data::read_project_data_file(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../parity/fixtures/rust-usize-index-expressions.lino"),
    )
    .unwrap();
    for row in fixture["forms"].as_array().unwrap() {
        assert_eq!(
            expression_rules::accept_rust_usize_index_expression_node(
                row["kind"].as_str().unwrap(),
                row["operator"].as_str().unwrap(),
                row["atom"].as_bool().unwrap(),
                row["literal"].as_bool().unwrap(),
                row["count"].as_f64().unwrap()
            ),
            row["expected"].as_bool().unwrap()
        );
    }
    for row in fixture["decisions"].as_array().unwrap() {
        let texts = |name: &str| {
            row[name]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        let flags = |name: &str| {
            row[name]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_bool().unwrap())
                .collect::<Vec<_>>()
        };
        let parents = row["parents"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            expression_rules::read_rust_usize_index_expression_error(
                &texts("kinds"),
                &texts("operators"),
                &flags("atoms"),
                &flags("literals"),
                &parents
            ),
            row["expected"].as_str().unwrap()
        );
    }
}
#[test]
fn actual_source_expression_budgets_remain_bounded() {
    let kinds = vec!["match".to_owned(); 130];
    let operators = vec![String::new(); 130];
    let flags = vec![false; 130];
    let parents = (0..130)
        .map(|index| f64::from(index) - 1.0)
        .collect::<Vec<_>>();
    assert_eq!(
        expression_rules::read_rust_usize_index_expression_error(
            &kinds, &operators, &flags, &flags, &parents
        ),
        "the index expression exceeds its source depth limit"
    );
    assert_eq!(
        expression_rules::read_rust_usize_index_expression_error(
            &vec!["num".to_owned(); 4097],
            &[],
            &[],
            &[],
            &[]
        ),
        "the index expression exceeds its source node limit"
    );
}
