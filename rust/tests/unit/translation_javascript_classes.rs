//! CI-only complete source/frontend/native product controls, never run locally.
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
        }
    }
}
