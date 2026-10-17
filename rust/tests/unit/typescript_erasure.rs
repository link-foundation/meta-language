//! Operational TypeScript erasure and provenance share the JavaScript fixture.
use meta_language::{erase_type_script, self_translate};
use serde_json::{Value, json};

const FIXTURE: &str = include_str!("../../../parity/fixtures/typescript-erasure.lino");

#[test]
fn linked_typescript_erasure_matches_javascript_byte_spans_and_refusals() {
    assert_eq!(
        include_str!("../../src/data/typescript-erasure.lino"),
        include_str!("../../../js/src/data/typescript-erasure.lino")
    );
    let fixture: Value = super::project_data::read_project_data(FIXTURE).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap();
        match erase_type_script(source) {
            Ok(erased) => {
                assert_eq!(
                    erased.code,
                    case["erased"]["code"].as_str().unwrap(),
                    "{}",
                    case["id"]
                );
                let ranges: Vec<_> = erased.erasures.into_iter().map(|erasure| json!({"start":erasure.start,"end":erasure.end,"term":erasure.term})).collect();
                assert_eq!(
                    Value::Array(ranges),
                    case["erased"]["erasures"],
                    "{}",
                    case["id"]
                );
            }
            Err(error) => {
                assert_eq!(
                    error.reason,
                    case["error"]["reason"].as_str().unwrap(),
                    "{}",
                    case["id"]
                );
                let span = error
                    .span
                    .map(|span| json!({"start":span.start,"end":span.end}));
                assert_eq!(span.unwrap_or(Value::Null), case["error"]["span"]);
            }
        }
    }
}

#[test]
fn typescript_translations_restore_from_artifacts_without_original_source() {
    let fixture: Value = super::project_data::read_project_data(FIXTURE).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let source = case["source"].as_str().unwrap();
        let translated = self_translate(source, "TypeScript", "JavaScript").unwrap();
        assert_eq!(
            translated.code,
            case["javascript"].as_str().unwrap(),
            "{}",
            case["id"]
        );
        let replay = self_translate(&translated.code, "JavaScript", "TypeScript").unwrap();
        assert_eq!(replay.code, source, "{}", case["id"]);
        if !case["error"].is_null() {
            assert!(translated.items.iter().any(|item| item.status == "carried"));
        }
    }
}

#[test]
fn edited_erasure_artifacts_do_not_restore_the_original_runtime_body() {
    let source = "function add(a: number, b: number): number { return a + b; }\n";
    let translated = self_translate(source, "TypeScript", "JavaScript").unwrap();
    let at = translated.code.rfind("return a + b;").unwrap();
    let edited = format!(
        "{}{}",
        &translated.code[..at],
        translated.code[at..].replacen("return a + b;", "return a - b;", 1)
    );
    let reversed = self_translate(&edited, "JavaScript", "TypeScript").unwrap();
    assert_ne!(reversed.code, source);
    assert!(reversed.code.contains("return a - b;"));
}

#[test]
fn typed_function_translates_to_the_rust_portable_core() {
    let translated = self_translate(
        "function add(a: number, b: number): number { return a + b; }\n",
        "TypeScript",
        "Rust",
    )
    .unwrap();
    assert!(
        translated
            .items
            .iter()
            .all(|item| item.status == "translated")
    );
    assert!(
        translated
            .code
            .contains("pub fn add(a: f64, b: f64) -> f64")
    );
}

#[test]
fn native_typescript_annotations_preserve_string_and_boolean_types() {
    let fixture: Value = super::project_data::read_project_data(FIXTURE).unwrap();
    for case in fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["rust"].is_string())
    {
        let translated =
            self_translate(case["source"].as_str().unwrap(), "TypeScript", "Rust").unwrap();
        assert_eq!(
            translated.code,
            case["rust"].as_str().unwrap(),
            "{}",
            case["id"]
        );
        assert!(
            translated
                .items
                .iter()
                .all(|item| item.status == "translated")
        );
    }
}
