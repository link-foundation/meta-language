//! Thin fixture boundary: inspectable project data keeps logical JSON aliases.
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

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
