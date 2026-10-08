use meta_language::self_translate;
use serde_json::Value;
use std::path::Path;
use std::process::Command;

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../parity/fixtures/self-translation-line-endings.json"
    ))
    .unwrap()
}

#[test]
fn source_envelopes_preserve_line_endings_and_match_javascript() {
    let script = "import fs from 'node:fs'; import {selfTranslate} from './js/src/self-translation.js'; const f=JSON.parse(fs.readFileSync('parity/fixtures/self-translation-line-endings.json')); process.stdout.write(JSON.stringify(f.cases.map(e=>selfTranslate(e.source,e.language,e.target).code)));";
    let result = Command::new("node")
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap())
        .args(["--input-type=module", "-e", script])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let expected: Vec<String> = serde_json::from_slice(&result.stdout).unwrap();
    for (index, entry) in fixture()["cases"].as_array().unwrap().iter().enumerate() {
        let source = entry["source"].as_str().unwrap();
        let language = entry["language"].as_str().unwrap();
        let target = entry["target"].as_str().unwrap();
        let translated = self_translate(source, language, target).unwrap();
        assert!(
            translated
                .items
                .iter()
                .any(|item| Some(item.status) == entry["status"].as_str())
        );
        assert_eq!(translated.code, expected[index], "{}", entry["name"]);
        assert_eq!(
            self_translate(&translated.code, target, language)
                .unwrap()
                .code,
            source
        );
        assert_eq!(
            self_translate(source, language, language).unwrap().code,
            source
        );
        assert_eq!(
            self_translate(&expected[index], target, language)
                .unwrap()
                .code,
            source
        );
    }
}

#[test]
fn source_envelopes_reject_edited_bodies_and_corrupt_originals() {
    let fixture = fixture();
    let entry = fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["name"] == "translated-crlf-function")
        .unwrap();
    let source = entry["source"].as_str().unwrap();
    let translated = self_translate(source, "JavaScript", "Rust").unwrap().code;
    let edited = translated.replace("2f64", "9f64");
    assert_ne!(edited, translated);
    let restored = self_translate(&edited, "Rust", "JavaScript").unwrap().code;
    assert_ne!(restored, source);
    assert!(restored.contains("9f64"));
    let envelope_start = translated.find("envelope-sha256=").unwrap() + "envelope-sha256=".len();
    let mut corrupted_hash = translated.clone();
    corrupted_hash.replace_range(envelope_start..envelope_start + 64, &"0".repeat(64));
    for corrupted in [
        translated.replace("original=\"", "original=\"X"),
        corrupted_hash,
        translated.replace("original=\"", "original={"),
    ] {
        assert_ne!(
            self_translate(&corrupted, "Rust", "JavaScript")
                .unwrap()
                .code,
            source
        );
    }
}
