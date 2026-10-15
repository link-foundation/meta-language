//! Historical fixture source digests remain independent of physical `LiNo` hashes.
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use meta_language::structured_data_links::{
    StructuredDataValue, read_structured_data_lino_ordered,
};
use serde_json::Value;

use super::project_data::{read_project_data, resolve_project_data_path};

fn json_string(units: &[u16]) -> String {
    serde_json::to_string(&String::from_utf16(units).expect("UTF-16 provenance scalar"))
        .expect("JSON provenance scalar")
}

fn canonical_tokens(value: &StructuredDataValue, tokens: &mut Vec<String>) {
    match value {
        StructuredDataValue::Null => tokens.push("null".to_owned()),
        StructuredDataValue::Boolean(value) => tokens.push(value.to_string()),
        StructuredDataValue::Number(value) => tokens.push(if *value == 0.0 {
            "0".to_owned()
        } else {
            value.to_string()
        }),
        StructuredDataValue::String(units) => tokens.push(json_string(units)),
        StructuredDataValue::Array(values) => {
            tokens.push("[".to_owned());
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    tokens.push(",".to_owned());
                }
                canonical_tokens(value, tokens);
            }
            tokens.push("]".to_owned());
        }
        StructuredDataValue::Object(entries) => {
            tokens.push("{".to_owned());
            for (index, (name, value)) in entries.iter().enumerate() {
                if index > 0 {
                    tokens.push(",".to_owned());
                }
                tokens.push(json_string(name));
                tokens.push(":".to_owned());
                canonical_tokens(value, tokens);
            }
            tokens.push("}".to_owned());
        }
    }
}

fn scalar_meaning(token: &str) -> Value {
    let value: Value = serde_json::from_str(token).expect("JSON provenance scalar spelling");
    assert!(!value.is_array() && !value.is_object());
    value
}

/// Recreate the source bytes pinned by unchanged fixture manifest metadata.
pub fn read_fixture_source_bytes(file: impl AsRef<Path>) -> Vec<u8> {
    let logical = file.as_ref();
    let actual = resolve_project_data_path(logical).expect("one fixture storage format");
    if logical
        .extension()
        .is_none_or(|extension| extension != "json")
        || actual
            .extension()
            .is_none_or(|extension| extension != "lino")
    {
        return fs::read(actual).expect("fixture source file");
    }
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let name = logical
        .strip_prefix(&repository)
        .expect("repository fixture source")
        .to_str()
        .expect("UTF-8 fixture source path")
        .replace('\\', "/");
    let layouts: Value = read_project_data(include_str!(
        "../../../parity/fixtures/issue-195-json-source-layouts.lino"
    ))
    .expect("fixture source formatting links");
    let layout = layouts
        .get(&name)
        .expect("registered fixture source byte contract");
    let source = fs::read_to_string(actual).expect("structured fixture Links data");
    let value = read_structured_data_lino_ordered(&source).expect("ordered fixture Links data");
    let mut tokens = Vec::new();
    canonical_tokens(&value, &mut tokens);
    assert_eq!(layout["tokenCount"].as_u64(), Some(tokens.len() as u64));
    let mut whitespace = HashMap::new();
    for entry in layout["whitespace"].as_array().expect("whitespace records") {
        let index = usize::try_from(entry[0].as_u64().expect("whitespace position"))
            .expect("bounded whitespace position");
        let text = entry[1].as_str().expect("source whitespace");
        assert!(index <= tokens.len() && !text.is_empty());
        assert!(
            text.bytes()
                .all(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
        );
        assert!(whitespace.insert(index, text).is_none());
    }
    let mut spellings = HashMap::new();
    for entry in layout["spellings"]
        .as_array()
        .expect("scalar spelling records")
    {
        let index = usize::try_from(entry[0].as_u64().expect("scalar position"))
            .expect("bounded scalar position");
        let text = entry[1].as_str().expect("scalar spelling");
        let canonical = scalar_meaning(tokens.get(index).expect("scalar token position"));
        let original = scalar_meaning(text);
        if canonical.is_number() && original.is_number() {
            assert_eq!(canonical.as_f64(), original.as_f64());
        } else {
            assert_eq!(canonical, original);
        }
        assert!(spellings.insert(index, text).is_none());
    }
    let mut source = String::new();
    for (index, token) in tokens.iter().enumerate() {
        if let Some(text) = whitespace.get(&index) {
            source.push_str(text);
        }
        source.push_str(spellings.get(&index).copied().unwrap_or(token));
    }
    if let Some(text) = whitespace.get(&tokens.len()) {
        source.push_str(text);
    }
    source.into_bytes()
}
