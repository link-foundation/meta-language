//! Future CI native execution of the source-derived shared array range methods.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::{SelfTranslationOptions, read_structured_data_lino, self_translate_with};
use std::fs;
use std::process::Command;

#[test]
fn shared_array_ranges_have_full_source_parity_and_native_behavior() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let version = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = version
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {version}; >=1.90 <1.100"
    );
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/array-state-methods/oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let expected = read_structured_data_lino(&producer.stdout).unwrap();
    let translated = self_translate_with(
        expected["source"].as_str().unwrap(),
        "JavaScript",
        "Rust",
        &SelfTranslationOptions {
            semantic_translation: true,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(translated.items.iter().all(|item| matches!(
        item.status.as_str(),
        "translated" | "comment" | "provenance"
    )));
    assert_eq!(translated.code, expected["code"].as_str().unwrap());
    let mut main = "\nfn main(){".to_owned();
    for (index, value) in expected["expected"].as_array().unwrap().iter().enumerate() {
        let value = value.as_str().unwrap().parse::<f64>().unwrap();
        assert!(value.is_finite());
        main.push_str(&format!(
            "assert_eq!(compute{index}(),{value}f64,\"fixture {index}\");"
        ));
    }
    main.push_str("println!(\"array range methods\");}\n");
    let directory = TemporaryDirectory::new("shared-array-range-methods");
    let source = directory.0.join("product.rs");
    let binary = directory.0.join(if cfg!(windows) {
        "product.exe"
    } else {
        "product"
    });
    fs::write(&source, translated.code + &main).unwrap();
    let mut compiler = Command::new("rustc");
    compiler.args(["--edition", "2024"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        compiler.arg("-C").arg(format!("linker={linker}"));
    }
    let built = run(compiler.arg(&source).arg("-o").arg(&binary));
    assert!(built.ok, "{}", built.stderr);
    let observed = run(&mut Command::new(&binary));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(observed.stdout, "array range methods\n");
}
