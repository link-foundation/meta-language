//! CI executes the entire automatically translated inverse decision module.
//! These host assertions do not claim complete native DTO reconstruction.
#[path = "../../src/translation/owned_outcome_inverse_rules.rs"]
mod rules;

use serde_json::Value;

#[test]
fn translated_inverse_decisions_match_every_shared_independent_case() {
    let fixture: Value = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/grammar-outcome-inverse-rules.lino"
    ))
    .expect("the complete shared independent inverse cases load");
    for row in fixture["widths"].as_array().unwrap() {
        assert_eq!(
            rules::accept_owned_outcome_target_width(row[0].as_f64().unwrap()),
            row[1].as_bool().unwrap()
        );
    }
    for row in fixture["kinds"].as_array().unwrap() {
        assert_eq!(
            rules::read_owned_outcome_json_kind(
                row[0].as_bool().unwrap(),
                row[1].as_bool().unwrap(),
                row[2].as_str().unwrap()
            ),
            row[3].as_str().unwrap()
        );
    }
    for row in fixture["fields"].as_array().unwrap() {
        let expected: Vec<String> = row[1]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect();
        assert_eq!(
            rules::read_owned_outcome_json_fields(row[0].as_str().unwrap()),
            expected
        );
    }
    for row in fixture["containers"].as_array().unwrap() {
        assert_eq!(
            rules::accept_owned_outcome_container(
                row[0].as_str().unwrap(),
                row[1].as_str().unwrap(),
                row[2].as_bool().unwrap()
            ),
            row[3].as_bool().unwrap()
        );
    }
}
