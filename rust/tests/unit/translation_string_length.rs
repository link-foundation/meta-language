use meta_language::program_translation::translate_program;
use std::process::Command;

#[test]
fn utf16_string_lengths_match_javascript_emission_in_every_target() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let source =
        std::fs::read_to_string(root.join("parity/fixtures/translation-corpus/string-length.mjs"))
            .unwrap();
    let script = "import {translateProgram} from './js/src/program-translation.js'; console.log(JSON.stringify(['Rust','Lean','Rocq'].map(target => translateProgram(process.argv[1], 'JavaScript', target))));";
    let result = Command::new("node")
        .current_dir(root)
        .args(["--input-type=module", "-e", script, &source])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let expected: Vec<serde_json::Value> = serde_json::from_slice(&result.stdout).unwrap();
    for (target, expected) in ["Rust", "Lean", "Rocq"].into_iter().zip(expected) {
        let translated = translate_program(&source, "JavaScript", target).unwrap();
        assert_eq!(translated.diagnostic(), None, "{target}");
        assert_eq!(
            translated.code(),
            expected["code"].as_str().unwrap(),
            "{target}"
        );
    }
}
