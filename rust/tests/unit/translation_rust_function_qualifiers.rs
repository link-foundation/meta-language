//! Deferred original compiler products, complete definitions and public APIs.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{
    SelfTranslationOptions, read_structured_data_lino, self_translate_with,
    self_translation_signatures,
};
use std::{fs, process::Command};

#[test]
fn ordinary_const_functions_and_associated_values_keep_original_products() {
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered source usize model"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/function-qualifiers.mjs",
        "ordinary Rust const functions and associated values",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-function-qualifier-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}

#[test]
fn ordinary_const_publication_keeps_actual_signatures_and_values() {
    let output =
        run(Command::new("node").arg("../experiments/rust-records/const-publications.mjs"));
    assert!(output.ok, "{}", output.stderr);
    let product = read_structured_data_lino(&output.stdout).unwrap();
    let directory = TemporaryDirectory::new("ordinary-const-publications");
    let options = SelfTranslationOptions {
        semantic_translation: true,
        ..SelfTranslationOptions::default()
    };
    for (index, row) in product["publications"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        let source = row["source"].as_str().unwrap();
        assert_eq!(
            serde_json::to_value(self_translation_signatures(source, "Rust", &options).unwrap())
                .unwrap(),
            row["signatures"]
        );
        let result = self_translate_with(source, "Rust", "JavaScript", &options).unwrap();
        assert!(result.items.iter().all(|item| item.status == "translated"));
        assert_eq!(result.code, row["code"].as_str().unwrap());
        fs::write(directory.0.join(format!("module_{index}.mjs")), result.code).unwrap();
        let mut code = format!(
            "import * as module from './module_{index}.mjs';\nconsole.log(String(module.observe()));\nconsole.log(Object.keys(module).sort().join(','));\n"
        );
        let mut exports = row["exports"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect::<Vec<_>>();
        exports.sort_unstable();
        let mut expected = vec![row["value"].as_str().unwrap().to_owned(), exports.join(",")];
        for check in row["checks"].as_array().unwrap() {
            let name = serde_json::to_string(check["name"].as_str().unwrap()).unwrap();
            let expression = if check["call"].is_null() {
                format!("module[{name}]")
            } else {
                format!(
                    "module[{name}]({})",
                    check["call"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|value| value.as_str().unwrap())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            };
            code.push_str(&format!("console.log(String({expression}));\n"));
            expected.push(check["value"].as_str().unwrap().to_owned());
        }
        let file = directory.0.join(format!("consumer_{index}.mjs"));
        fs::write(&file, code).unwrap();
        let observed = run(Command::new("node").arg(file));
        assert!(observed.ok, "{}", observed.stderr);
        assert_eq!(
            observed.stdout.lines().collect::<Vec<_>>(),
            expected.iter().map(String::as_str).collect::<Vec<_>>()
        );
    }
}
