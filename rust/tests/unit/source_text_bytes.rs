//! Actual internal generated algorithms and public host facades. Execution is CI-only.
use crate::translation::source_text_byte_rules as rules;
use meta_language::{
    read_source_text_byte, read_source_text_byte_length, read_source_text_units,
    slice_source_text_bytes,
};
use serde_json::Value;

fn fixture() -> Value {
    super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/source-text-bytes.lino"
    ))
    .unwrap()
}
fn count(value: usize) -> f64 {
    f64::from(u32::try_from(value).unwrap())
}
fn number(value: &str) -> f64 {
    match value {
        "NaN" => f64::NAN,
        "Infinity" => f64::INFINITY,
        "-Infinity" => f64::NEG_INFINITY,
        other => other.parse().unwrap(),
    }
}
fn units(row: &Value) -> Vec<f64> {
    row["units"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_f64().unwrap())
        .collect()
}
fn text(row: &Value) -> String {
    String::from_utf16(
        &row["units"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| u16::try_from(value.as_u64().unwrap()).unwrap())
            .collect::<Vec<_>>(),
    )
    .unwrap()
}

#[test]
fn actual_shared_algorithms_and_facades_match_the_frozen_independent_encodings() {
    let fixture = fixture();
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 18);
    for row in fixture["cases"].as_array().unwrap() {
        let text = text(row);
        let units = units(row);
        let bytes: Vec<u8> = row["bytes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|byte| u8::try_from(byte.as_u64().unwrap()).unwrap())
            .collect();
        assert_eq!(text.as_bytes(), bytes);
        assert_eq!(read_source_text_units(&text).unwrap(), units);
        assert_eq!(read_source_text_byte_length(&text).unwrap(), bytes.len());
        assert!(rules::accept_source_text_scalar_units(&units));
        assert_eq!(
            rules::read_source_text_byte_length(&units).to_bits(),
            count(bytes.len()).to_bits()
        );
        for offset in 0..=bytes.len() + 1 {
            let expected = bytes.get(offset).copied();
            assert_eq!(
                read_source_text_byte(&text, count(offset)).unwrap(),
                expected
            );
            assert_eq!(
                rules::read_source_text_byte(&units, count(offset)).to_bits(),
                expected.map_or(-1.0, f64::from).to_bits()
            );
            let position = row["positions"][offset].as_f64().unwrap_or(-1.0);
            assert_eq!(
                rules::read_source_text_code_unit_offset(&units, count(offset)).to_bits(),
                position.to_bits()
            );
            for end in 0..=bytes.len() + 1 {
                let expected = text.get(offset..end);
                assert_eq!(
                    rules::accept_source_text_byte_slice(&units, count(offset), count(end)),
                    expected.is_some()
                );
                assert_eq!(
                    slice_source_text_bytes(&text, count(offset), count(end)).unwrap(),
                    expected
                );
            }
        }
    }
}

#[test]
fn complete_invalid_unit_vectors_and_nonfinite_scalars_refuse_without_replacement() {
    let fixture = fixture();
    assert_eq!(fixture["invalidUnits"].as_array().unwrap().len(), 13);
    for row in fixture["invalidUnits"].as_array().unwrap() {
        let units: Vec<f64> = row
            .as_array()
            .unwrap()
            .iter()
            .map(|value| number(value.as_str().unwrap()))
            .collect();
        assert!(!rules::accept_source_text_scalar_units(&units));
        assert_eq!(
            rules::read_source_text_byte_length(&units).to_bits(),
            (-1.0_f64).to_bits()
        );
        assert_eq!(
            rules::read_source_text_byte(&units, 0.0).to_bits(),
            (-1.0_f64).to_bits()
        );
        assert_eq!(
            rules::read_source_text_code_unit_offset(&units, 0.0).to_bits(),
            (-1.0_f64).to_bits()
        );
        assert!(!rules::accept_source_text_byte_slice(&units, 0.0, 0.0));
    }
    for value in fixture["invalidScalars"].as_array().unwrap() {
        assert_eq!(
            rules::read_source_text_scalar_byte_width(number(value.as_str().unwrap())).to_bits(),
            (-1.0_f64).to_bits()
        );
    }
}

#[test]
fn invalid_offsets_and_signed_zero_preserve_the_shared_result_model() {
    let fixture = fixture();
    for row in fixture["cases"].as_array().unwrap() {
        let text = text(row);
        for spelling in fixture["invalidOffsets"].as_array().unwrap() {
            let offset = number(spelling.as_str().unwrap());
            assert_eq!(read_source_text_byte(&text, offset).unwrap(), None);
            assert_eq!(slice_source_text_bytes(&text, offset, 0.0).unwrap(), None);
        }
        for spelling in fixture["zeroOffsets"].as_array().unwrap() {
            let offset = number(spelling.as_str().unwrap());
            assert_eq!(
                slice_source_text_bytes(&text, offset, offset).unwrap(),
                Some("")
            );
            assert_eq!(
                read_source_text_byte(&text, offset).unwrap(),
                text.as_bytes().first().copied()
            );
        }
    }
    assert!(rules::accept_source_text_unit_count(f64::from(u32::MAX)));
    assert!(!rules::accept_source_text_unit_count(
        f64::from(u32::MAX) + 1.0
    ));
    for value in [-1.0, 0.5, f64::NAN, f64::INFINITY] {
        assert!(!rules::accept_source_text_unit_count(value));
    }
}
