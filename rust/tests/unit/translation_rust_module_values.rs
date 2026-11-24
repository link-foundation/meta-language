//! Deferred native public APIs, original products, supplied consumers and codecs.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{
    SelfTranslationOptions, read_structured_data_lino, self_translate_with,
    self_translation_signatures,
};
use std::{fs, process::Command};
#[test]
fn ordinary_module_values_retain_original_native_products_and_complete_definitions() {
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered source usize model in module values"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/module-values.mjs",
        "Rust actual module values",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-module-value-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
#[test]
fn native_public_interfaces_and_consumers_match_the_same_javascript_products() {
    use meta_language::translation::{
        check::check_program,
        emit_javascript::emit_javascript,
        javascript::{ModuleContext, parse_javascript_in},
    };
    let producer =
        run(Command::new("node").arg("../experiments/rust-records/module-publications.mjs"));
    assert!(producer.ok, "{}", producer.stderr);
    let product =
        read_structured_data_lino(&producer.stdout).expect("complete public API products");
    let directory = TemporaryDirectory::new("rust-module-values");
    let options = SelfTranslationOptions {
        semantic_translation: true,
        ..SelfTranslationOptions::default()
    };
    for (index, row) in product["publications"]
        .as_array()
        .expect("source publications")
        .iter()
        .enumerate()
    {
        let source = row["source"].as_str().unwrap();
        let signatures = self_translation_signatures(source, "Rust", &options).unwrap();
        assert_eq!(serde_json::to_value(signatures).unwrap(), row["signatures"]);
        let actual = self_translate_with(source, "Rust", "JavaScript", &options).unwrap();
        assert!(actual.items.iter().all(|item| item.status == "translated"));
        assert_eq!(actual.code, row["code"].as_str().unwrap());
        let module = format!("module_{index}.mjs");
        fs::write(directory.0.join(&module), actual.code).unwrap();
        let mut code = format!(
            "import * as module from './{module}';\nconsole.log(String(module.observe()));\nconsole.log(Object.keys(module).sort().join(','));\n"
        );
        let mut expected = vec![row["value"].as_str().unwrap().to_owned()];
        let mut exports = row["exports"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect::<Vec<_>>();
        exports.sort_unstable();
        expected.push(exports.join(","));
        for check in row["checks"].as_array().unwrap() {
            let name = serde_json::to_string(check["name"].as_str().unwrap()).unwrap();
            code.push_str(&format!("console.log(typeof module[{name}]);\n"));
            expected.push(check["kind"].as_str().unwrap().to_owned());
            let expression = if check["call"].is_null() {
                format!("module[{name}]")
            } else {
                let args = check["call"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("module[{name}]({args})")
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
    for (index, row) in product["consumers"].as_array().unwrap().iter().enumerate() {
        let context = ModuleContext {
            externals: serde_json::from_value(row["externals"].clone()).unwrap(),
            module_directory: None,
        };
        let program =
            check_program(&parse_javascript_in(row["source"].as_str().unwrap(), &context).unwrap())
                .unwrap();
        let emitted = emit_javascript(&program).unwrap();
        assert_eq!(emitted.text, row["code"].as_str().unwrap());
        let target = emitted
            .mappings
            .iter()
            .find(|mapping| mapping.kind == "function" && mapping.source == "result")
            .unwrap()
            .target
            .clone();
        let code = format!(
            "'use strict';\n{}\n{}\nconsole.log(String({target}()));\nconsole.log(JSON.stringify(typeof events==='undefined'?[]:events));\n",
            row["setup"].as_str().unwrap(),
            emitted.text.replace("export function ", "function ")
        );
        let file = directory.0.join(format!("callable_{index}.mjs"));
        fs::write(&file, code).unwrap();
        let observed = run(Command::new("node").arg(file));
        assert!(observed.ok, "{}", observed.stderr);
        assert_eq!(
            observed.stdout,
            format!(
                "{}\n{}\n",
                row["value"].as_str().unwrap(),
                serde_json::to_string(&row["events"]).unwrap()
            )
        );
        let packet = meta_language::encode_typed_program_links(&program).unwrap();
        assert_eq!(packet.root.as_u64(), row["root"].as_u64().unwrap());
        assert_eq!(packet.network.to_lino(), row["links"].as_str().unwrap());
        let bytes = meta_language::encode_link_network_binary(
            &packet.network,
            meta_language::BinaryLinksOptions::default(),
        )
        .unwrap();
        for network in [
            meta_language::LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
            meta_language::decode_link_network_binary(
                &bytes,
                meta_language::BinaryLinksOptions::default(),
            )
            .unwrap(),
        ] {
            for target in ["JavaScript", "Rust", "Lean", "Rocq"] {
                let value =
                    meta_language::emit_typed_program_links(&network, packet.root, target).unwrap();
                assert_eq!(
                    value.code,
                    row["emissions"][target]["code"].as_str().unwrap()
                );
            }
        }
    }
    assert!(
        self_translation_signatures(
            "pub const VALUE:i64=7;fn hidden()->i64{missing_name}",
            "Rust",
            &options
        )
        .is_err()
    );
}
