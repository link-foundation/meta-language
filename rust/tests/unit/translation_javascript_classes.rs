//! CI-only complete source/frontend/native product controls, never run locally.
#[path = "../../src/translation/javascript_class_source_rules.rs"]
mod public_field_rules;
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, emit_rust::emit_rust,
    javascript::parse_javascript,
};
use meta_language::{SelfTranslationOptions, read_structured_data_lino, self_translate_with};
use std::{fs, process::Command};

#[test]
fn closed_javascript_classes_keep_both_frontends_and_generated_products() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let compiler_version = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = compiler_version
        .split('.')
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {compiler_version}; >=1.90 <1.100"
    );
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/javascript-classes/oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let fixture = read_structured_data_lino(&producer.stdout).unwrap();
    for (index, row) in fixture["rows"].as_array().unwrap().iter().enumerate() {
        let source = row["source"].as_str().unwrap();
        let surface = parse_javascript(source).unwrap();
        assert_eq!(
            serde_json::to_value(&surface).unwrap(),
            row["surface"],
            "complete source shape {}",
            row["name"]
        );
        let program = check_program(&surface).unwrap();
        let javascript = emit_javascript(&program).unwrap();
        let native = emit_rust(&program).unwrap();
        assert_eq!(serde_json::to_value(javascript).unwrap(), row["javascript"]);
        assert_eq!(serde_json::to_value(&native).unwrap(), row["native"]);
        let translation = self_translate_with(
            source,
            "JavaScript",
            "Rust",
            &SelfTranslationOptions {
                semantic_translation: true,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(
            translation.source_language,
            row["translation"]["sourceLanguage"].as_str().unwrap()
        );
        assert_eq!(
            translation.target_language,
            row["translation"]["targetLanguage"].as_str().unwrap()
        );
        assert_eq!(
            translation.code,
            row["translation"]["code"].as_str().unwrap()
        );
        let expected_items = row["translation"]["items"].as_array().unwrap();
        assert_eq!(translation.items.len(), expected_items.len());
        for (item, expected) in translation.items.iter().zip(expected_items) {
            assert_eq!(item.term, expected["term"].as_str().unwrap());
            assert_eq!(
                item.start,
                usize::try_from(expected["start"].as_u64().unwrap()).unwrap()
            );
            assert_eq!(
                item.end,
                usize::try_from(expected["end"].as_u64().unwrap()).unwrap()
            );
            assert_eq!(item.status, expected["status"].as_str().unwrap());
            assert_eq!(item.reason.as_deref(), expected["reason"].as_str());
        }
        let name = native
            .mappings
            .iter()
            .find(|mapping| mapping.kind == "function" && mapping.source == "compute")
            .unwrap()
            .target
            .clone();
        let expected = row["expected"].as_str().unwrap();
        let assertion = match row["resultType"].as_str().unwrap() {
            "number" => format!("assert_eq!({name}(),{expected}f64);"),
            "boolean" => format!("assert_eq!({name}(),{expected});"),
            "string" => format!("assert_eq!({name}().to_string(),{:?});", expected),
            "error" => {
                let class = row["error"]["errorClass"].as_str().unwrap();
                let units = row["error"]["messageUnits"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| format!("{}u16", value.as_u64().unwrap()))
                    .collect::<Vec<_>>();
                let units = if units.is_empty() {
                    "Vec::<u16>::new()".to_owned()
                } else {
                    format!("vec![{}]", units.join(","))
                };
                let present = row["error"]["messagePresent"].as_bool().unwrap();
                format!(
                    "std::panic::set_hook(Box::new(|_|{{}}));let panic=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{{let _={name}();}})).expect_err(\"the original constructor throws\");let actual=panic.downcast_ref::<(String,Vec<u16>,bool)>().expect(\"the complete source exception payload\");assert_eq!(actual,&({class:?}.to_owned(),{units},{present}));"
                )
            }
            other => panic!("uncovered independent class result {other}"),
        };
        let directory = TemporaryDirectory::new(&format!("closed-class-{index}"));
        for (product, code) in [
            (
                "native",
                native.preludes.join("\n") + "\n" + &native.definitions.join("\n"),
            ),
            (
                "javascript",
                row["native"]["preludes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap())
                    .collect::<Vec<_>>()
                    .join("\n")
                    + "\n"
                    + &row["native"]["definitions"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|value| value.as_str().unwrap())
                        .collect::<Vec<_>>()
                        .join("\n"),
            ),
        ] {
            let file = directory.0.join(format!("{product}.rs"));
            let binary = directory.0.join(if cfg!(windows) {
                format!("{product}.exe")
            } else {
                product.to_owned()
            });
            fs::write(
                &file,
                code + "\nfn main(){" + &assertion + "println!(\"class passed\");}\n",
            )
            .unwrap();
            let mut compiler = Command::new("rustc");
            compiler.args(["--edition", "2024"]);
            #[cfg(windows)]
            if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
                compiler.arg("-C").arg(format!("linker={linker}"));
            }
            let built = run(compiler.arg(&file).arg("-o").arg(&binary));
            assert!(built.ok, "{}: {}", row["name"], built.stderr);
            let observed = run(&mut Command::new(binary));
            assert!(observed.ok, "{}", observed.stderr);
            assert_eq!(observed.stdout, "class passed\n");
            if row["resultType"] == "error" {
                assert_eq!(observed.stderr, "");
            }
        }
    }
}

#[test]
fn public_field_definitions_keep_initializer_boundaries_and_constructor_continuations() {
    super::unsigned_source_documents::require_versions();
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/javascript-closed-classes.lino"
    ))
    .unwrap();
    assert_eq!(
        fixture["sourceModelVersion"], 4,
        "uncovered class source fixture model"
    );
    for row in fixture["fieldDocumentation"].as_array().unwrap() {
        assert_eq!(
            public_field_rules::render_java_script_closed_class_field_documentation(
                &public_field_rules::ml_string::Text::from_text(row["value"].as_str().unwrap())
            )
            .to_string(),
            row["expected"].as_str().unwrap()
        );
    }
    for row in fixture["fieldEnds"].as_array().unwrap() {
        let values = row["values"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| public_field_rules::ml_string::Text::from_text(value.as_str().unwrap()))
            .collect::<Vec<_>>();
        assert_eq!(
            public_field_rules::find_java_script_closed_class_field_end(
                &values,
                row["first"].as_f64().unwrap(),
                row["last"].as_f64().unwrap()
            ),
            row["expected"].as_f64().unwrap()
        );
    }
    for row in fixture["fieldPrefixEnds"].as_array().unwrap() {
        let values = row["row"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            public_field_rules::read_java_script_closed_class_prefix_end(&values),
            row["expected"].as_f64().unwrap()
        );
    }
}
