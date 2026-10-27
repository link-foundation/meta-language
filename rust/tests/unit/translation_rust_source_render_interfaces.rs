//! Deferred source-interface controls and complete original/generated products.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::read_structured_data_lino;
use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, emit_rust::emit_rust, rust::parse_rust,
};
use std::{fs, process::Command};
#[path = "../../src/translation/rust_record_rules.rs"]
mod rules;

#[test]
fn ordinary_calls_and_constructor_layouts_keep_complete_products() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let actual = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = actual
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {actual}; >=1.90 <1.100"
    );
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/rust-source-render/oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let observations = read_structured_data_lino(&producer.stdout).unwrap();
    let text =
        |value: &serde_json::Value| rules::ml_string::Text::from_text(value.as_str().unwrap());
    let texts = |value: &serde_json::Value| {
        value
            .as_array()
            .unwrap()
            .iter()
            .map(text)
            .collect::<Vec<_>>()
    };
    for row in observations["calls"].as_array().unwrap() {
        let rendered =
            rules::render_rust_function_call(&text(&row["head"]), &texts(&row["values"]));
        assert_eq!(
            String::from_utf16(&rendered.0).unwrap(),
            row["expected"].as_str().unwrap()
        );
    }
    for row in observations["constructors"].as_array().unwrap() {
        let language = text(&row["language"]);
        let names = texts(&row["names"]);
        let rendered = rules::render_rust_constructor_fields(
            &language,
            &text(&row["head"]),
            &names,
            &texts(&row["values"]),
        );
        assert_eq!(
            String::from_utf16(&rendered.0).unwrap(),
            row["expected"].as_str().unwrap()
        );
        assert_eq!(
            rules::accept_rust_named_constructor_fields(&language, &names),
            row["named"].as_bool().unwrap()
        );
    }
    for (index, row) in observations["rows"].as_array().unwrap().iter().enumerate() {
        let surface = parse_rust(row["source"].as_str().unwrap()).unwrap();
        assert_eq!(serde_json::to_value(&surface).unwrap(), row["surface"]);
        let program = check_program(&surface).unwrap();
        let native = emit_rust(&program).unwrap();
        assert_eq!(serde_json::to_value(&native).unwrap(), row["native"]);
        assert_eq!(
            serde_json::to_value(emit_javascript(&program).unwrap()).unwrap(),
            row["javascript"]
        );
        let entry = native
            .mappings
            .iter()
            .find(|mapping| mapping.kind == "function" && mapping.source == "compute")
            .unwrap()
            .target
            .clone();
        let javascript_product = row["native"]["preludes"]
            .as_array()
            .unwrap()
            .iter()
            .chain(row["native"]["definitions"].as_array().unwrap())
            .map(|part| part.as_str().unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        for (owner, code, call) in [
            (
                "original",
                row["source"].as_str().unwrap().to_owned(),
                "compute".to_owned(),
            ),
            (
                "native",
                native.preludes.join("\n") + "\n" + &native.definitions.join("\n"),
                entry.clone(),
            ),
            ("javascript", javascript_product, entry.clone()),
        ] {
            let directory = TemporaryDirectory::new(&format!("source-render-{index}-{owner}"));
            let file = directory.0.join("product.rs");
            let binary = directory.0.join(if cfg!(windows) {
                "product.exe"
            } else {
                "product"
            });
            fs::write(
                &file,
                code + &format!("\nfn main(){{println!(\"{{}}\",{call}());}}\n"),
            )
            .unwrap();
            let mut compiler = Command::new("rustc");
            compiler.args(["--edition", "2024"]);
            #[cfg(windows)]
            if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
                compiler.arg("-C").arg(format!("linker={linker}"));
            }
            let built = run(compiler.arg(file).arg("-o").arg(&binary));
            assert!(built.ok, "{} {owner}: {}", row["name"], built.stderr);
            let observed = run(&mut Command::new(binary));
            assert!(observed.ok, "{} {owner}: {}", row["name"], observed.stderr);
            assert_eq!(
                observed.stdout.replace("\r\n", "\n"),
                row["expected"].as_str().unwrap().to_owned() + "\n"
            );
        }
    }
}
