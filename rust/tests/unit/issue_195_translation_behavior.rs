use std::fs;
use std::path::PathBuf;
use std::process::Command;

use meta_language::{decode_program_translation, translate_program, TranslationSupport};
use serde_json::Value;

use super::issue_195_observations as observations;

fn corpus() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/fixtures/four-language-conformance.json");
    serde_json::from_str(&fs::read_to_string(path).expect("shared corpus is readable"))
        .expect("shared corpus is valid JSON")
}

#[test]
fn translated_javascript_print_executes_in_rust() {
    let corpus = corpus();
    let fixture = corpus["translationBehaviorCases"]
        .as_array()
        .expect("translation behavior cases")
        .iter()
        .find(|case| case["sourceLanguage"] == "JavaScript" && case["targetLanguage"] == "Rust")
        .expect("JavaScript to Rust case");
    let source_text = fixture["source"].as_str().expect("source");
    let expected_stdout = fixture["expectedStdout"].as_str().expect("stdout");
    let translated = translate_program(source_text, "JavaScript", "Rust")
        .expect("JavaScript to Rust translation");
    let directory = std::env::temp_dir().join(format!(
        "meta-language-translation-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ));
    fs::create_dir(&directory).expect("temporary directory");
    let source = directory.join("translated.rs");
    let executable = directory.join(format!("translated{}", std::env::consts::EXE_SUFFIX));
    fs::write(&source, translated.code()).expect("translated source");
    let mut rustc = Command::new("rustc");
    rustc.args(["--edition", "2024", "--crate-type", "bin"]);
    #[cfg(windows)]
    if let Ok(linker) = std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER") {
        rustc.arg("-C").arg(format!("linker={linker}"));
    }
    let compiler = rustc
        .arg("-o")
        .arg(&executable)
        .arg(&source)
        .output()
        .expect("rustc available");
    assert!(
        compiler.status.success(),
        "{}",
        String::from_utf8_lossy(&compiler.stderr)
    );
    let output = Command::new(&executable)
        .output()
        .expect("translated program runs");
    assert!(output.status.success());
    assert_eq!(output.stdout, expected_stdout.as_bytes());
    fs::remove_dir_all(directory).expect("temporary directory cleanup");
    observations::record(&observations::Observation {
        requirement_id: "I195-TRANSLATE-javascript-to-rust",
        suffix: "positive",
        fixture_id: "planned:translation:JavaScript:Rust",
        fixture_file: observations::FOUR_LANGUAGE_FIXTURE,
        assertions: &[
            "realTargetArtifact",
            "nativeTargetValidation",
            "semanticPreservationChecked",
        ],
        test_name: "translated_javascript_print_executes_in_rust",
    });
}

#[test]
fn translated_rust_function_exports_javascript_behavior() {
    let corpus = corpus();
    let fixture = corpus["translationBehaviorCases"]
        .as_array()
        .expect("translation behavior cases")
        .iter()
        .find(|case| case["sourceLanguage"] == "Rust" && case["targetLanguage"] == "JavaScript")
        .expect("Rust to JavaScript case");
    let source_text = fixture["source"].as_str().expect("source");
    let exported_name = fixture["export"].as_str().expect("exported name");
    let expected_result = fixture["expectedResult"].as_u64().expect("expected result");
    let translated = translate_program(source_text, "Rust", "JavaScript")
        .expect("Rust to JavaScript translation");
    assert!(translated
        .code()
        .contains("export function answer() { return 42; }"));
    assert_eq!(
        decode_program_translation(translated.code(), "JavaScript")
            .expect("envelope still decodes")
            .source(),
        source_text
    );
    let directory = std::env::temp_dir().join(format!(
        "meta-language-javascript-translation-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ));
    fs::create_dir(&directory).expect("temporary directory");
    fs::write(directory.join("translated.mjs"), translated.code()).expect("translated module");
    let name = serde_json::to_string(exported_name).expect("export name is JSON-safe");
    let script =
        format!("const module = await import('./translated.mjs'); console.log(module[{name}]());");
    let output = Command::new("node")
        .args(["--input-type=module", "--eval", &script])
        .current_dir(&directory)
        .output()
        .expect("Node.js available for translated JavaScript");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, format!("{expected_result}\n").as_bytes());
    fs::remove_dir_all(directory).expect("temporary directory cleanup");
}

#[test]
fn rust_identifier_reserved_by_strict_javascript_stays_transport_only() {
    let translated = translate_program("pub fn public() -> u32 { 42 }", "Rust", "JavaScript")
        .expect("translation descriptor");
    assert_eq!(
        translated.contract().support,
        TranslationSupport::PortableEncoding
    );
    assert!(!translated.code().contains("export function public"));
}
