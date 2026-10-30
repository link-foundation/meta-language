//! Deferred complete API parity and independent source-std behavior controls.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{
    SelfTranslationOptions, read_structured_data_lino, self_translate_with,
    self_translate_with_diagnostics, self_translation_signatures,
};
use serde_json::{Value, json};
use std::{fs, process::Command};
#[test]
fn selected_source_std_reaches_every_self_translation_interpretation() {
    super::unsigned_source_documents::require_versions();
    let product =
        run(Command::new("node")
            .arg("../experiments/self-translation-source-versions/products.mjs"));
    assert!(product.ok, "{}", product.stderr);
    let product = read_structured_data_lino(&product.stdout).unwrap();
    let directory = TemporaryDirectory::new("self-translation-source-std");
    let exported = regex::Regex::new(r"export \{ (\w+) as result \};").unwrap();
    let mut driver = String::new();
    let mut expected = Vec::new();
    for (index, row) in product["rows"].as_array().unwrap().iter().enumerate() {
        let options = SelfTranslationOptions {
            semantic_translation: row["semanticTranslation"].as_bool().unwrap(),
            rust_source_version: Some(row["version"].as_str().unwrap().to_owned()),
            ..SelfTranslationOptions::default()
        };
        let source = row["source"].as_str().unwrap();
        let actual = self_translate_with(source, "Rust", "JavaScript", &options).unwrap();
        assert_eq!(actual.code, row["translation"]["code"].as_str().unwrap());
        assert!(actual.items.iter().all(|item|["translated","comment","provenance"].contains(&item.status.as_str())));
        let items=actual.items.iter().map(|item|json!({"term":item.term,"start":item.start,"end":item.end,"status":item.status,"reason":item.reason})).collect::<Vec<_>>();
        assert_eq!(json!(items), row["translation"]["items"]);
        let diagnostic =
            self_translate_with_diagnostics(source, "Rust", "JavaScript", &options).unwrap();
        assert_eq!(diagnostic.translation, actual);
        assert_eq!(json!(diagnostic.diagnostics), row["report"]["diagnostics"]);
        if options.semantic_translation {
            assert_eq!(
                serde_json::to_value(
                    self_translation_signatures(source, "Rust", &options).unwrap()
                )
                .unwrap(),
                row["signatures"]
            );
        }
        let target = exported
            .captures(&actual.code)
            .map_or("result", |capture| capture.get(1).unwrap().as_str());
        let mut code = actual.code.clone();
        code.push_str(&format!("\ntry{{console.log(JSON.stringify({{kind:'value',value:String({target}())}}));}}catch(error){{console.log(JSON.stringify({{kind:'abort',message:error.message}}));}}\n"));
        let file = format!("case_{index}.mjs");
        fs::write(directory.0.join(&file), code).unwrap();
        driver.push_str(&format!("await import('./{file}');\n"));
        expected.push(row["expected"].clone());
    }
    let path = directory.0.join("observe.mjs");
    fs::write(&path, driver).unwrap();
    let observed = run(Command::new("node").arg(path));
    assert!(observed.ok, "{}", observed.stderr);
    let actual = observed
        .stdout
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}
#[test]
fn invalid_or_wrong_language_std_selectors_cannot_publish_a_translation() {
    super::unsigned_source_documents::require_versions();
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-source-versions.lino"
    ))
    .unwrap();
    for version in fixture["invalidVersions"].as_array().unwrap() {
        for semantic_translation in [false, true] {
            let options = SelfTranslationOptions {
                semantic_translation,
                rust_source_version: Some(version.as_str().unwrap().to_owned()),
                ..SelfTranslationOptions::default()
            };
            for target in ["JavaScript", "Rust"] {
                let error =
                    self_translate_with("pub fn result()->bool{true}", "Rust", target, &options)
                        .unwrap_err();
                assert!(
                    error
                        .message
                        .contains(fixture["versionError"].as_str().unwrap()),
                    "{}",
                    error.message
                );
            }
            if semantic_translation {
                assert!(
                    self_translation_signatures("pub fn result()->bool{true}", "Rust", &options)
                        .unwrap_err()
                        .message
                        .contains(fixture["versionError"].as_str().unwrap())
                );
            }
        }
    }
    for language in ["JavaScript", "TypeScript"] {
        for version in ["", "1.90.0"] {
            let options = SelfTranslationOptions {
                rust_source_version: Some(version.to_owned()),
                ..SelfTranslationOptions::default()
            };
            assert_eq!(
                self_translate_with(
                    "function result(){return true;}",
                    language,
                    "Rust",
                    &options
                )
                .unwrap_err()
                .message,
                "Rust source version requires Rust source"
            );
            assert_eq!(
                self_translation_signatures("function result(){return true;}", language, &options)
                    .unwrap_err()
                    .message,
                "Rust source version requires Rust source"
            );
        }
    }
}
