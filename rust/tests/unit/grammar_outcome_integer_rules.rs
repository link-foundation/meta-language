//! Shared integer admission executes natively only in CI.
#[path = "../../src/translation/owned_outcome_integer_rules.rs"]
mod rules;

#[test]
fn translated_integer_admission_preserves_every_original_big_integer_boundary() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/grammar-outcome-integer-rules.lino"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 1688);
    assert_eq!(fixture["declarations"].as_array().unwrap().len(), 1);
    for row in cases {
        let value = row["value"].as_f64().unwrap();
        let bits = u64::from_str_radix(row["bits"].as_str().unwrap(), 16).unwrap();
        assert_eq!(value.to_bits(), bits, "original binary64 input bits");
        let expected = row["expected"].as_bool().unwrap();
        assert_eq!(
            rules::accept_owned_outcome_integer_value(
                value,
                row["signed"].as_bool().unwrap(),
                row["width"].as_f64().unwrap(),
            ),
            expected,
            "{}",
            row["name"]
        );
        assert_eq!(row["originalRefusal"].is_null(), expected);
        assert_eq!(row["originalOutput"].is_object(), expected);
    }
    for signed in [false, true] {
        for width in [32.0, 64.0] {
            for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.0] {
                assert!(!rules::accept_owned_outcome_integer_value(
                    value, signed, width
                ));
            }
        }
        for width in [
            16.0,
            128.0,
            0.0,
            -1.0,
            0.5,
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ] {
            assert!(!rules::accept_owned_outcome_integer_value(
                0.0, signed, width
            ));
        }
    }
}
