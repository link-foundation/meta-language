//! Thin fixture boundary: inspectable project data keeps logical JSON aliases.
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

/// Project one host-compatible member while retaining the whole ordered archive.
///
/// The existing codec refuses unpaired UTF-16 in the selected subtree; this
/// boundary does not replace those strings or inspect unrelated members.
pub fn project_data_member<T: DeserializeOwned>(
    archive: &meta_language::StructuredDataValue,
    name: &str,
) -> Result<T, String> {
    let meta_language::StructuredDataValue::Object(fields) = archive else {
        return Err("project data member requires an ordered object".to_owned());
    };
    let mut matches = fields
        .iter()
        .filter(|(key, _)| key.iter().copied().eq(name.encode_utf16()));
    let (_, selected) = matches
        .next()
        .ok_or_else(|| format!("project data member {name:?} is absent"))?;
    if matches.next().is_some() {
        return Err(format!("project data member {name:?} is ambiguous"));
    }
    let linked = meta_language::structured_data_to_lino_ordered(selected)?;
    let value = meta_language::read_structured_data_lino(&linked)?;
    serde_json::from_value(value).map_err(|error| error.to_string())
}

pub fn read_project_data<T: DeserializeOwned>(text: &str) -> Result<T, String> {
    let value = if text.trim_start().starts_with('(') {
        meta_language::structured_data_links::read_structured_data_lino(text)?
    } else {
        serde_json::from_str(text).map_err(|error| error.to_string())?
    };
    serde_json::from_value(value).map_err(|error| error.to_string())
}

pub fn resolve_project_data_path(file: impl AsRef<Path>) -> Result<PathBuf, String> {
    meta_language::project_data::resolve_project_data_path(file)
}

#[test]
fn native_agda_corpus_keeps_logical_identity_and_decoded_sources() {
    let logical = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/fixtures/native-grammars/agda-corpus.json");
    let physical = resolve_project_data_path(&logical).unwrap();
    assert_eq!(physical.extension().unwrap(), "lino");
    assert!(!logical.exists());
    let fixture: serde_json::Value = read_project_data(include_str!(
        "../../../parity/fixtures/native-grammars/agda-corpus.lino"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 75);
    assert_eq!(cases[0]["file"], "agda-stdlib/Algebra.txt");
    assert_eq!(cases[0]["title"], "module imports");
    assert_eq!(
        cases.last().unwrap()["source"],
        "\na = record { MyRecord old; a = 0; c = 5}\n"
    );
}

#[test]
fn selected_owned_cases_retain_the_complete_archive_and_all_exact_utf16_inputs() {
    use meta_language::StructuredDataValue;

    let source = include_str!("../../../experiments/rust-owned-local-state-fixture.lino");
    let archive = meta_language::read_structured_data_lino_ordered(source).unwrap();
    let before = archive.clone();
    let cases: Vec<serde_json::Value> = project_data_member(&archive, "cases").unwrap();
    assert_eq!(cases.len(), 5);
    assert!(meta_language::read_structured_data_lino(source).is_err());
    assert!(
        project_data_member::<serde_json::Value>(&archive, "inputValues")
            .unwrap_err()
            .contains("unpaired UTF-16")
    );
    let StructuredDataValue::Object(fields) = &archive else {
        panic!("the complete ordered fixture remains an object")
    };
    let (_, StructuredDataValue::Object(inputs)) = fields
        .iter()
        .find(|(key, _)| key.iter().copied().eq("inputValues".encode_utf16()))
        .unwrap()
    else {
        panic!("the original input vectors remain ordered")
    };
    for (role, expected) in [("validText", true), ("invalidText", false)] {
        let (_, StructuredDataValue::Array(values)) = inputs
            .iter()
            .find(|(key, _)| key.iter().copied().eq(role.encode_utf16()))
            .unwrap()
        else {
            panic!("each original input role remains an array")
        };
        assert_eq!(values.len(), 7);
        for value in values {
            let StructuredDataValue::String(units) = value else {
                panic!("each input retains exact code units")
            };
            assert_eq!(String::from_utf16(units).is_ok(), expected);
        }
    }
    assert_eq!(archive, before);
}

#[test]
fn member_selection_keeps_exact_keys_null_and_host_domain_refusals_distinct() {
    use meta_language::StructuredDataValue;

    let key = |text: &str| text.encode_utf16().collect::<Vec<_>>();
    let archive = StructuredDataValue::Object(vec![
        (key("é"), StructuredDataValue::Number(-0.0)),
        (key("e\u{301}"), StructuredDataValue::Boolean(true)),
        (key("present"), StructuredDataValue::Null),
        (vec![0xd800], StructuredDataValue::Null),
    ]);
    let negative_zero: serde_json::Value = project_data_member(&archive, "é").unwrap();
    assert!(negative_zero.as_f64().unwrap().is_sign_negative());
    assert!(project_data_member::<bool>(&archive, "e\u{301}").unwrap());
    assert!(
        project_data_member::<serde_json::Value>(&archive, "present")
            .unwrap()
            .is_null()
    );
    assert!(project_data_member::<serde_json::Value>(&archive, "missing").is_err());
    assert!(project_data_member::<Vec<serde_json::Value>>(&archive, "present").is_err());
    assert!(
        project_data_member::<serde_json::Value>(&StructuredDataValue::Null, "present").is_err()
    );
    let ambiguous = StructuredDataValue::Object(vec![
        (key("same"), StructuredDataValue::Null),
        (key("same"), StructuredDataValue::Boolean(false)),
    ]);
    assert!(project_data_member::<serde_json::Value>(&ambiguous, "same").is_err());
}
