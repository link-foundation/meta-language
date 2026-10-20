//! The complete JavaScript algorithm is generated; native execution is CI-only.
#[path = "../../src/translation/owned_outcome_integer_projection_rules.rs"]
mod rules;

use meta_language::StructuredDataValue;

fn projection_fixture_field<'a>(
    value: &'a StructuredDataValue,
    name: &str,
) -> &'a StructuredDataValue {
    let StructuredDataValue::Object(fields) = value else {
        panic!("expected an ordered fixture object: {value:?}");
    };
    &fields
        .iter()
        .find(|(key, _)| key.iter().copied().eq(name.encode_utf16()))
        .unwrap_or_else(|| panic!("missing fixture field {name}"))
        .1
}

fn projection_fixture_array(value: &StructuredDataValue) -> &[StructuredDataValue] {
    let StructuredDataValue::Array(values) = value else {
        panic!("expected an ordered fixture array: {value:?}");
    };
    values
}

fn projection_fixture_units(value: &StructuredDataValue) -> &[u16] {
    let StructuredDataValue::String(units) = value else {
        panic!("expected exact fixture string units: {value:?}");
    };
    units
}

fn projection_fixture_boolean(value: &StructuredDataValue) -> bool {
    let StructuredDataValue::Boolean(value) = value else {
        panic!("expected a fixture Boolean: {value:?}");
    };
    *value
}

fn projection_fixture_number(value: &StructuredDataValue) -> f64 {
    let StructuredDataValue::Number(value) = value else {
        panic!("expected a fixture number: {value:?}");
    };
    *value
}

#[test]
fn exact_integer_projections_retain_all_original_big_integer_oracle_controls() {
    let fixture = meta_language::read_structured_data_lino_ordered(include_str!(
        "../../../parity/fixtures/grammar-outcome-integer-projections.lino"
    ))
    .unwrap();
    let cases = projection_fixture_array(projection_fixture_field(&fixture, "cases"));
    assert_eq!(cases.len(), 3628);
    assert_eq!(
        projection_fixture_array(projection_fixture_field(
            projection_fixture_field(&fixture, "closure"),
            "inputs"
        ))
        .len(),
        2
    );
    assert!(
        cases.iter().any(
            |row| projection_fixture_units(projection_fixture_field(row, "text")).contains(&0xd800)
        ),
        "the original unpaired surrogate controls must reach the generated algorithm unchanged"
    );
    for row in cases {
        let projected = f64::from_bits(
            u64::from_str_radix(
                &String::from_utf16(projection_fixture_units(projection_fixture_field(
                    row,
                    "projectedBits",
                )))
                .unwrap(),
                16,
            )
            .unwrap(),
        );
        let text = rules::ml_string::Text::from_units(
            projection_fixture_units(projection_fixture_field(row, "text")).to_vec(),
        );
        assert_eq!(
            rules::accept_owned_outcome_integer_projection(
                &text,
                projection_fixture_boolean(projection_fixture_field(row, "signed")),
                projection_fixture_number(projection_fixture_field(row, "width")),
                projected,
            ),
            projection_fixture_boolean(projection_fixture_field(row, "expected")),
            "every original decimal and projected bit: {row:?}"
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
