//! Deferred controls for automatically translated UTF16 expansion source.
#[path = "../../src/translation/utf16_text_expansion_rules.rs"]
mod native;
use meta_language::read_structured_data_lino;

fn number(value: &serde_json::Value) -> f64 {
    match value.as_str().unwrap() {
        "NaN" => f64::NAN,
        "Infinity" => f64::INFINITY,
        "-Infinity" => f64::NEG_INFINITY,
        text => text.parse().unwrap(),
    }
}
fn units(value: &serde_json::Value) -> Vec<f64> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|unit| unit.as_f64().unwrap())
        .collect()
}
#[test]
fn shared_literal_repetition_and_padding_controls() {
    super::translation_javascript_object_binding_declarations::require_supported_rust_version();
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/utf16-text-expansion.lino"
    ))
    .unwrap();
    for row in fixture["repetition"].as_array().unwrap() {
        let text = units(&row["text"]);
        let expected = row["length"].as_f64().unwrap();
        assert_eq!(
            native::read_utf16_text_repetition_length(text.len() as f64, number(&row["count"])),
            expected
        );
        if expected >= 0.0 {
            let result = native::construct_utf16_text_repetition(&text, expected);
            assert_eq!(result.len() as f64, expected);
            if row.get("expected").is_some() {
                assert_eq!(result, units(&row["expected"]));
            } else {
                assert_eq!(result.len() as u64, row["expectedLength"].as_u64().unwrap());
                assert!(result.iter().all(|unit| *unit == 97.0));
            }
        }
    }
    for row in fixture["padding"].as_array().unwrap() {
        let text = units(&row["text"]);
        let filler = units(&row["filler"]);
        let expected = row["length"].as_f64().unwrap();
        assert_eq!(
            native::read_utf16_text_padding_length(
                text.len() as f64,
                number(&row["maximum"]),
                filler.len() as f64
            ),
            expected
        );
        if expected >= 0.0 {
            let result = native::construct_utf16_text_padding(
                &text,
                &filler,
                expected,
                row["start"].as_bool().unwrap(),
            );
            assert_eq!(result.len() as f64, expected);
            if row.get("expected").is_some() {
                assert_eq!(result, units(&row["expected"]));
            } else {
                assert_eq!(result.len() as u64, row["expectedLength"].as_u64().unwrap());
                assert!(result.iter().all(|unit| *unit == 120.0));
            }
        }
    }
}
