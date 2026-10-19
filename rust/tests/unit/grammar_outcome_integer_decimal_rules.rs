//! Exact decimal admission runs against standard integer parsing only in CI.
#[path = "../../src/translation/owned_outcome_integer_decimal_rules.rs"]
mod rules;

fn accept_standard_integer_decimal(text: &str, signed: bool, width: u32) -> bool {
    if ![32, 64].contains(&width) {
        return false;
    }
    if signed {
        match text.parse::<i64>() {
            Ok(value) => value.to_string() == text,
            Err(_) => false,
        }
    } else {
        match text.parse::<u64>() {
            Ok(value) => value.to_string() == text && (width == 64 || value <= u64::from(u32::MAX)),
            Err(_) => false,
        }
    }
}

#[test]
fn translated_exact_decimal_admission_preserves_native_integer_limits_and_original_spelling() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/grammar-outcome-integer-decimal-rules.lino"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 3660);
    assert_eq!(fixture["declarations"].as_array().unwrap().len(), 5);
    for row in cases {
        let text = row["text"].as_str().unwrap();
        let signed = row["signed"].as_bool().unwrap();
        let width = row["width"].as_f64().unwrap();
        let native_width = if width == 32.0 {
            32
        } else {
            assert_eq!(width, 64.0);
            64
        };
        let expected = row["expected"].as_bool().unwrap();
        assert_eq!(
            accept_standard_integer_decimal(text, signed, native_width),
            expected,
            "independent standard integer parsing: {text:?}, signed={signed}, width={width}"
        );
        assert_eq!(
            rules::accept_owned_outcome_integer_decimal(
                &rules::ml_string::Text::from_text(text),
                signed,
                width
            ),
            expected,
            "{text:?}, signed={signed}, width={width}"
        );
    }
    for signed in [false, true] {
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
            assert!(!rules::accept_owned_outcome_integer_decimal(
                &rules::ml_string::Text::from_text("0"),
                signed,
                width
            ));
            assert!(
                rules::read_owned_outcome_integer_decimal_limit(false, signed, width)
                    .0
                    .is_empty()
            );
        }
        for width in [32.0, 64.0] {
            assert!(!rules::accept_owned_outcome_integer_decimal(
                &rules::ml_string::Text::from_text(&"9".repeat(4096)),
                signed,
                width
            ));
            for unit in [0xd800, 0xdfff] {
                assert!(!rules::accept_owned_outcome_integer_decimal(
                    &rules::ml_string::Text::from_units(vec![unit]),
                    signed,
                    width
                ));
            }
        }
    }
}
