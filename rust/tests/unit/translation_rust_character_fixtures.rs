//! All shared finite character vectors run against the actual generated rules in CI.
#[path = "../../src/translation/rust_character_rules.rs"]
mod rules;
use meta_language::{StructuredDataValue, read_structured_data_lino_ordered};
use std::collections::HashSet;

const SHARED_CHARACTER_FIXTURES: &str =
    include_str!("../../../parity/fixtures/rust-character-literals.lino");
const NATIVE_CHARACTER_FIXTURES: &str = include_str!("../../src/data/rust-character-literals.lino");

#[test]
fn byte_expression_type_and_negation_decisions_preserve_their_source_domains() {
    assert_eq!(rules::read_rust_character_expression_type("byteChar"), "u8");
    for kind in [
        "char",
        "number",
        "identifier",
        "",
        "unknown",
        "byteCharSuffix",
    ] {
        assert_eq!(rules::read_rust_character_expression_type(kind), "");
    }
    for (language, kind, signed, expected) in [
        ("Rust", "fixed", false, false),
        ("Rust", "fixed", true, true),
        ("Rust", "", false, true),
        ("Rust", "int", false, true),
        ("Rust", "float", false, true),
        ("JavaScript", "fixed", false, true),
        ("Lean", "fixed", false, true),
        ("Rocq", "fixed", false, true),
    ] {
        assert_eq!(
            rules::accept_rust_numeric_literal_negation(language, kind, signed),
            expected
        );
    }
}

fn character_fixture_member<'a>(
    value: &'a StructuredDataValue,
    name: &str,
) -> &'a StructuredDataValue {
    let StructuredDataValue::Object(fields) = value else {
        panic!("a shared character record exists")
    };
    fields
        .iter()
        .find_map(|(key, value)| key.iter().copied().eq(name.encode_utf16()).then_some(value))
        .expect("the original character field exists")
}

const fn character_fixture_array(value: &StructuredDataValue) -> &[StructuredDataValue] {
    let StructuredDataValue::Array(values) = value else {
        panic!("a shared character array exists")
    };
    values.as_slice()
}

const fn character_fixture_number(value: &StructuredDataValue) -> f64 {
    let StructuredDataValue::Number(number) = value else {
        panic!("a shared character argument is a primitive Number")
    };
    assert!(
        number.is_finite(),
        "only finite paired Number cells are admitted"
    );
    *number
}

fn character_fixture_text(value: &StructuredDataValue) -> String {
    let StructuredDataValue::String(units) = value else {
        panic!("a shared character label exists")
    };
    String::from_utf16(units)
        .expect("fixture labels, unlike original source units, are scalar text")
}

fn character_fixture_result(tag: &str, fields: &[(&str, f64)]) -> StructuredDataValue {
    let mut values = vec![(
        "$".encode_utf16().collect(),
        StructuredDataValue::String(tag.encode_utf16().collect()),
    )];
    values.extend(fields.iter().map(|(name, value)| {
        (
            name.encode_utf16().collect(),
            StructuredDataValue::Number(*value),
        )
    }));
    StructuredDataValue::Object(values)
}

// Mechanical observation of the actual generated enum fields; this implements
// no lexical decision, escape decoding or expected result calculation.
fn character_fixture_scan_result(result: rules::RustCharacterLiteral) -> StructuredDataValue {
    use rules::RustCharacterLiteral;
    let (tag, fields) = match result {
        RustCharacterLiteral::NotCharacter => ("notCharacter", Vec::new()),
        RustCharacterLiteral::Char(value, content_start, content_end, end) => (
            "char",
            vec![
                ("value", value),
                ("contentStart", content_start),
                ("contentEnd", content_end),
                ("end", end),
            ],
        ),
        RustCharacterLiteral::ByteChar(value, content_start, content_end, end) => (
            "byteChar",
            vec![
                ("value", value),
                ("contentStart", content_start),
                ("contentEnd", content_end),
                ("end", end),
            ],
        ),
        RustCharacterLiteral::Malformed(at, end) => ("malformed", vec![("at", at), ("end", end)]),
        RustCharacterLiteral::Unterminated(at, end) => {
            ("unterminated", vec![("at", at), ("end", end)])
        }
        RustCharacterLiteral::NonScalar(at, end) => ("nonScalar", vec![("at", at), ("end", end)]),
        RustCharacterLiteral::NonAscii(at, end) => ("nonAscii", vec![("at", at), ("end", end)]),
        RustCharacterLiteral::UnicodeByteEscape(at, end) => {
            ("unicodeByteEscape", vec![("at", at), ("end", end)])
        }
        RustCharacterLiteral::UnsupportedSuffix(at, end) => {
            ("unsupportedSuffix", vec![("at", at), ("end", end)])
        }
    };
    character_fixture_result(tag, &fields)
}

fn assert_character_fixture_value(
    actual: &StructuredDataValue,
    expected: &StructuredDataValue,
    id: &str,
) {
    match (actual, expected) {
        (StructuredDataValue::Number(actual), StructuredDataValue::Number(expected)) => {
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "{id}: exact binary64 value including signed zero"
            );
        }
        (StructuredDataValue::Object(actual), StructuredDataValue::Object(expected)) => {
            assert_eq!(
                actual.len(),
                expected.len(),
                "{id}: every result field remains present"
            );
            for (name, expected) in expected {
                let actual = actual
                    .iter()
                    .find_map(|(key, value)| (key == name).then_some(value))
                    .expect("every original result field remains present");
                assert_character_fixture_value(actual, expected, id);
            }
        }
        (StructuredDataValue::Array(actual), StructuredDataValue::Array(expected)) => {
            assert_eq!(actual.len(), expected.len(), "{id}: ordered result length");
            for (actual, expected) in actual.iter().zip(expected) {
                assert_character_fixture_value(actual, expected, id);
            }
        }
        _ => assert_eq!(actual, expected, "{id}: exact result shape and labels"),
    }
}

#[test]
fn all_shared_character_vectors_match_independent_premises_and_generated_javascript_baselines() {
    assert_eq!(
        NATIVE_CHARACTER_FIXTURES, SHARED_CHARACTER_FIXTURES,
        "the automatic generator publishes identical fixture Links to both packages"
    );
    let fixture = read_structured_data_lino_ordered(NATIVE_CHARACTER_FIXTURES).unwrap();
    assert_eq!(
        character_fixture_text(character_fixture_member(&fixture, "id")),
        "rust-character-literals"
    );
    assert_eq!(
        character_fixture_array(character_fixture_member(&fixture, "javascriptOnlyCases")).len(),
        9,
        "nonfinite and foreign/hole host controls are not fabricated native values"
    );
    let cases = character_fixture_array(character_fixture_member(&fixture, "cases"));
    assert_eq!(cases.len(), 1976);
    let mut identifiers = HashSet::new();
    let mut counts = [0_usize; 3];
    let mut negative_zero_start = false;
    let mut negative_zero_unit = false;
    for row in cases {
        let id = character_fixture_text(character_fixture_member(row, "id"));
        assert!(
            identifiers.insert(id.clone()),
            "each shared ordinal remains unique: {id}"
        );
        let args = character_fixture_array(character_fixture_member(row, "args"));
        let target = character_fixture_text(character_fixture_member(row, "target"));
        let actual = match target.as_str() {
            "decodeRustCharacterLiteral" => {
                counts[0] += 1;
                assert_eq!(args.len(), 2, "{id}");
                let units: Vec<_> = character_fixture_array(&args[0])
                    .iter()
                    .map(character_fixture_number)
                    .collect();
                let before: Vec<_> = units.iter().map(|unit| unit.to_bits()).collect();
                let start = character_fixture_number(&args[1]);
                if id == "negative-zero-start" {
                    assert_eq!(start.to_bits(), (-0.0_f64).to_bits());
                    negative_zero_start = true;
                }
                if id == "negative-zero-unit" {
                    assert_eq!(units[1].to_bits(), (-0.0_f64).to_bits());
                    negative_zero_unit = true;
                }
                if let StructuredDataValue::String(source) = character_fixture_member(row, "source")
                {
                    assert_eq!(
                        units.len(),
                        source.len(),
                        "{id}: complete original UTF16 length"
                    );
                    for (unit, source) in units.iter().zip(source) {
                        assert_eq!(
                            unit.to_bits(),
                            f64::from(*source).to_bits(),
                            "{id}: original UTF16 code unit"
                        );
                    }
                }
                let result = rules::decode_rust_character_literal(&units, start);
                assert_eq!(
                    units.iter().map(|unit| unit.to_bits()).collect::<Vec<_>>(),
                    before,
                    "{id}: original source vector is not rewritten"
                );
                character_fixture_scan_result(result)
            }
            "readRustCharacterByteEscapeValue" => {
                counts[1] += 1;
                assert_eq!(args.len(), 2, "{id}");
                StructuredDataValue::Number(rules::read_rust_character_byte_escape_value(
                    character_fixture_number(&args[0]),
                    character_fixture_number(&args[1]),
                ))
            }
            "readRustCharacterSimpleEscapeValue" => {
                counts[2] += 1;
                assert_eq!(args.len(), 1, "{id}");
                StructuredDataValue::Number(rules::read_rust_character_simple_escape_value(
                    character_fixture_number(&args[0]),
                ))
            }
            _ => panic!("every original shared target is exercised: {target}"),
        };
        let expected = character_fixture_member(row, "expected");
        match character_fixture_text(character_fixture_member(expected, "mode")).as_str() {
            "exact" => assert_character_fixture_value(
                &actual,
                character_fixture_member(expected, "value"),
                &id,
            ),
            "tagAndBounds" => {
                assert_character_fixture_value(
                    character_fixture_member(&actual, "$"),
                    character_fixture_member(expected, "tag"),
                    &id,
                );
                let at = character_fixture_number(character_fixture_member(&actual, "at"));
                let end = character_fixture_number(character_fixture_member(&actual, "end"));
                let start = character_fixture_number(&args[1]);
                let length =
                    f64::from(u32::try_from(character_fixture_array(&args[0]).len()).unwrap());
                assert_eq!(
                    at.to_bits(),
                    at.trunc().to_bits(),
                    "{id}: integral refusal start"
                );
                assert_eq!(
                    end.to_bits(),
                    end.trunc().to_bits(),
                    "{id}: integral refusal end"
                );
                assert!(
                    at >= start && at <= length && end >= at && end <= length,
                    "{id}: independent original absolute refusal bounds"
                );
            }
            mode => panic!("every original independent expectation is exercised: {mode}"),
        }
        // The exact full result is separately checked as generated baseline,
        // never described as an independent oracle or prior native evidence.
        assert_character_fixture_value(&actual, character_fixture_member(row, "baseline"), &id);
    }
    assert_eq!(identifiers.len(), 1976);
    assert_eq!(counts, [1205, 515, 256]);
    assert!(negative_zero_start && negative_zero_unit);
}

// The original finite fixture test above remains unchanged. These fields encode
// primitive Number bits explicitly; they do not encode foreign host values.
fn decode_character_fixture_number_words(value: &StructuredDataValue) -> StructuredDataValue {
    if let StructuredDataValue::Array(values) = value {
        return StructuredDataValue::Array(
            values
                .iter()
                .map(decode_character_fixture_number_words)
                .collect(),
        );
    }
    let StructuredDataValue::Object(fields) = value else {
        panic!("an explicit Number word pair exists")
    };
    assert_eq!(
        fields.len(),
        2,
        "the word pair has only its two declared fields"
    );
    let high = character_fixture_text(character_fixture_member(value, "highWord"));
    let low = character_fixture_text(character_fixture_member(value, "lowWord"));
    for word in [&high, &low] {
        assert_eq!(
            word.len(),
            8,
            "each exact u32 word has eight hexadecimal digits"
        );
        assert!(
            word.bytes()
                .all(|unit| unit.is_ascii_digit() || (b'a'..=b'f').contains(&unit)),
            "word transport is canonical lowercase hexadecimal"
        );
    }
    let high = u32::from_str_radix(&high, 16).unwrap();
    let low = u32::from_str_radix(&low, 16).unwrap();
    StructuredDataValue::Number(f64::from_bits((u64::from(high) << 32) | u64::from(low)))
}

const fn character_fixture_word_number(value: &StructuredDataValue) -> f64 {
    let StructuredDataValue::Number(number) = value else {
        panic!("a mechanically decoded primitive binary64 Number exists")
    };
    *number
}

#[test]
fn original_nonfinite_character_numbers_match_shared_ieee754_words_and_exact_generated_baselines() {
    assert_eq!(NATIVE_CHARACTER_FIXTURES, SHARED_CHARACTER_FIXTURES);
    let fixture = read_structured_data_lino_ordered(NATIVE_CHARACTER_FIXTURES).unwrap();
    let historical =
        character_fixture_array(character_fixture_member(&fixture, "javascriptOnlyCases"));
    let cases = character_fixture_array(character_fixture_member(&fixture, "numberWordCases"));
    assert_eq!(
        historical.len(),
        9,
        "the original direct-cell descriptor identities remain present"
    );
    assert_eq!(cases.len(), 9);
    assert_eq!(
        character_fixture_array(character_fixture_member(&fixture, "cases")).len(),
        1976,
        "every original finite paired vector remains present"
    );
    let historical_ids: HashSet<_> = historical
        .iter()
        .map(|row| character_fixture_text(character_fixture_member(row, "id")))
        .collect();
    let mut ids = HashSet::new();
    let mut classes = [0_usize; 3];
    let mut targets = [0_usize; 2];
    for row in cases {
        let id = character_fixture_text(character_fixture_member(row, "id"));
        assert!(
            ids.insert(id.clone()),
            "every nonfinite original ordinal stays distinct: {id}"
        );
        let decoded =
            decode_character_fixture_number_words(character_fixture_member(row, "argumentWords"));
        let args = character_fixture_array(&decoded);
        let mut nonfinite = Vec::new();
        for argument in args {
            match argument {
                StructuredDataValue::Array(units) => {
                    nonfinite.extend(
                        units
                            .iter()
                            .map(character_fixture_word_number)
                            .filter(|number| !number.is_finite()),
                    );
                }
                StructuredDataValue::Number(number) => {
                    if !number.is_finite() {
                        nonfinite.push(*number);
                    }
                }
                _ => {
                    panic!("the original arguments remain primitive Numbers or dense Number arrays")
                }
            }
        }
        assert_eq!(
            nonfinite.len(),
            1,
            "each of the original nine inputs has one nonfinite Number: {id}"
        );
        let number = nonfinite[0];
        if number.is_nan() {
            classes[0] += 1;
        } else if number.is_sign_positive() {
            classes[1] += 1;
        } else {
            classes[2] += 1;
        }
        let target = character_fixture_text(character_fixture_member(row, "target"));
        let descriptor = historical
            .iter()
            .find(|descriptor| {
                character_fixture_text(character_fixture_member(descriptor, "id")) == id
            })
            .unwrap();
        assert_eq!(
            character_fixture_text(character_fixture_member(descriptor, "target")),
            target
        );
        assert_eq!(args.len(), 2, "{id}");
        let actual = match target.as_str() {
            "decodeRustCharacterLiteral" => {
                targets[0] += 1;
                let units: Vec<_> = character_fixture_array(&args[0])
                    .iter()
                    .map(character_fixture_word_number)
                    .collect();
                let before: Vec<_> = units.iter().map(|number| number.to_bits()).collect();
                let start = character_fixture_word_number(&args[1]);
                let result = rules::decode_rust_character_literal(&units, start);
                assert_eq!(
                    units
                        .iter()
                        .map(|number| number.to_bits())
                        .collect::<Vec<_>>(),
                    before,
                    "{id}: source unit bits including the original NaN or infinity are retained"
                );
                character_fixture_scan_result(result)
            }
            "readRustCharacterByteEscapeValue" => {
                targets[1] += 1;
                StructuredDataValue::Number(rules::read_rust_character_byte_escape_value(
                    character_fixture_word_number(&args[0]),
                    character_fixture_word_number(&args[1]),
                ))
            }
            _ => panic!("every one of the original nine target functions is exercised: {target}"),
        };
        let expected = character_fixture_member(row, "expected");
        match character_fixture_text(character_fixture_member(expected, "mode")).as_str() {
            "exact" => assert_character_fixture_value(
                &actual,
                character_fixture_member(expected, "value"),
                &id,
            ),
            "tagAndBounds" => {
                assert_character_fixture_value(
                    character_fixture_member(&actual, "$"),
                    character_fixture_member(expected, "tag"),
                    &id,
                );
                let at = character_fixture_number(character_fixture_member(&actual, "at"));
                let end = character_fixture_number(character_fixture_member(&actual, "end"));
                let start = character_fixture_word_number(&args[1]);
                let length =
                    f64::from(u32::try_from(character_fixture_array(&args[0]).len()).unwrap());
                assert_eq!(
                    at.to_bits(),
                    at.trunc().to_bits(),
                    "{id}: integral refusal start"
                );
                assert_eq!(
                    end.to_bits(),
                    end.trunc().to_bits(),
                    "{id}: integral refusal end"
                );
                assert!(
                    at >= start && at <= length && end >= at && end <= length,
                    "{id}: unchanged independent original refusal bounds"
                );
            }
            mode => panic!("the unchanged independent expectation exists: {mode}"),
        }
        assert_character_fixture_value(&actual, character_fixture_member(row, "baseline"), &id);
    }
    assert_eq!(ids, historical_ids);
    assert_eq!(classes, [3, 3, 3]);
    assert_eq!(targets, [6, 3]);
}
