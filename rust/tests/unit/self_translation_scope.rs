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

#[test]
fn selected_utf16_prelude_retains_scalar_source_and_carries_unproved_text_values() {
    let source = "/** @param {number} value @returns {number} */\nexport function keep(value) { return value; }\n";
    let options = SelfTranslationOptions {
        utf16_strings: true,
        ..SelfTranslationOptions::default()
    };
    let default = self_translate(source, "JavaScript", "Rust").unwrap();
    assert!(!default.code.contains("pub mod ml_string {"));
    let selected = self_translate_with(source, "JavaScript", "Rust", &options).unwrap();
    assert_eq!(selected.code.matches("pub mod ml_string {").count(), 1);
    assert!(selected.items.iter().all(|item| item.status != "carried"));
    assert_eq!(
        self_translate(&selected.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
    for source in [
        "/** @param {string} value @returns {string} */\nexport function keep(value) { return value; }\n",
        "/** @param {string[]} values @returns {string[]} */\nexport function keep(values) { return values; }\n",
    ] {
        let carried = self_translate_with(source, "JavaScript", "Rust", &options).unwrap();
        assert!(carried.items.iter().any(|item| {
            item.status == "carried"
                && item.reason.as_deref().is_some_and(|reason| {
                    reason.contains("Text lowering")
                        && reason.contains("independently paired rendering support")
                })
        }));
        assert_eq!(
            self_translate(&carried.code, "Rust", "JavaScript")
                .unwrap()
                .code,
            source
        );
    }
    let external = SelfTranslationOptions {
        imports: BTreeMap::from([(
            "./text.mjs".to_owned(),
            vec![meta_language::translation::surface::External::Function {
                name: "readLabel".to_owned(),
                params: Vec::new(),
                ret: meta_language::translation::types::Type::String,
            }],
        )]),
        utf16_strings: true,
        ..SelfTranslationOptions::default()
    };
    let source = "import { readLabel } from './text.mjs';\n/** @returns {number} */\nexport function count() { return readLabel().length; }\n";
    let carried = self_translate_with(source, "JavaScript", "Rust", &external).unwrap();
    assert!(carried.items.iter().any(|item| {
        item.status == "carried"
            && item.reason.as_deref().is_some_and(|reason| {
                reason.contains("Text lowering")
                    && reason.contains("independently paired rendering support")
            })
    }));
    assert_eq!(
        self_translate(&carried.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
}
