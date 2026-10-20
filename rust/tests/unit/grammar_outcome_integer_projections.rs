//! The complete JavaScript algorithm is generated; native execution is CI-only.
#[path = "../../src/translation/owned_outcome_integer_projection_rules.rs"]
mod rules;

#[test]
fn exact_integer_projections_retain_all_original_big_integer_oracle_controls() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/grammar-outcome-integer-projections.lino"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 3628);
    assert_eq!(fixture["closure"]["inputs"].as_array().unwrap().len(), 2);
    for row in cases {
        let projected = f64::from_bits(
            u64::from_str_radix(row["projectedBits"].as_str().unwrap(), 16).unwrap(),
        );
        let text = rules::ml_string::Text::from_text(row["text"].as_str().unwrap());
        assert_eq!(
            rules::accept_owned_outcome_integer_projection(
                &text,
                row["signed"].as_bool().unwrap(),
                row["width"].as_f64().unwrap(),
                projected,
            ),
            row["expected"].as_bool().unwrap(),
            "every original decimal and projected bit: {row}"
        );
    }
    for projected in [-0.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.5] {
        assert!(!rules::accept_owned_outcome_integer_projection(
            &rules::ml_string::Text::from_text("0"),
            false,
            64.0,
            projected,
        ));
    }
}
