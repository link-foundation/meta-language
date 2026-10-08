//! Regression checks for combined declaration scopes and source preservation.
use meta_language::{
    SelfTranslationItem, SelfTranslationOptions, self_translate, self_translate_with,
    self_translation_signatures,
};
use std::collections::BTreeMap;

#[test]
fn resource_limited_executor_preserves_source_and_restores_exactly() {
    let source = include_str!("../../../js/src/grammar-runtime/executor.js");
    let translated = self_translate(source, "JavaScript", "Rust").unwrap();
    assert_ne!(translated.items, [] as [SelfTranslationItem; 0]);
    let mut cursor = 0;
    for item in &translated.items {
        assert!(item.start >= cursor && item.end > item.start && item.end <= source.len());
        assert!(
            source[cursor..item.start]
                .encode_utf16()
                .all(meta_language::translation::lexer::is_js_space)
        );
        cursor = item.end;
        if item.term == "ERROR" {
            assert_eq!(item.status, "carried");
        }
    }
    assert!(
        source[cursor..]
            .encode_utf16()
            .all(meta_language::translation::lexer::is_js_space)
    );
    assert_eq!(
        self_translate(&translated.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
}

#[test]
fn source_prefix_restoration_rejects_appended_code() {
    let source = "/** @param {number} value @returns {number} */\nfunction identity(value) { return value; }";
    let translated = self_translate(source, "JavaScript", "Rust").unwrap();
    assert_eq!(
        self_translate(&translated.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
    let edited = self_translate(
        &format!("{}\nconst EXTRA: f64 = 2.0;\n", translated.code),
        "Rust",
        "JavaScript",
    )
    .unwrap();
    assert_ne!(edited.code, source);
}

#[test]
fn scope_retry_exports_signatures_to_importing_modules() {
    let source = include_str!("../../../js/src/translation/frontend-rules.js");
    let expected: Vec<_> = source
        .lines()
        .filter_map(|line| line.strip_prefix("export function "))
        .map(|tail| tail.split('(').next().unwrap())
        .filter(|name| *name != "decodeUnicodeEscape")
        .collect();
    let signatures =
        self_translation_signatures(source, "JavaScript", &SelfTranslationOptions::default())
            .unwrap();
    let names: Vec<_> = signatures
        .iter()
        .map(meta_language::SelfTranslationSignature::name)
        .collect();
    assert_eq!(names, expected);
    let options = SelfTranslationOptions {
        imports: BTreeMap::from([("./frontend-rules.js".to_owned(), signatures)]),
        ..SelfTranslationOptions::default()
    };
    let imported = self_translate_with("import { readArrayMethodForm as form } from './frontend-rules.js';\n/** @returns {string} */\nexport function select() { return form('concat'); }\n", "JavaScript", "Rust", &options).unwrap();
    assert!(
        imported
            .items
            .iter()
            .all(|item| item.status == "translated")
    );
    assert!(
        imported
            .code
            .contains("use crate::frontend_rules::read_array_method_form as form;")
    );
}
