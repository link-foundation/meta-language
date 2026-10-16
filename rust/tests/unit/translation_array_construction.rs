use meta_language::program_translation::translate_program;
use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, javascript::parse_javascript,
};
use std::process::Command;

#[test]
fn array_construction_matches_javascript_and_preserves_order_nesting_and_effects() {
    let fixture: serde_json::Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/translation-array-construction.lino"
    ))
    .unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let script = "import {translateProgram} from './js/src/program-translation.js'; process.stdout.write(JSON.stringify(['Rust','Lean','Rocq'].map(target=>translateProgram(process.argv[1],'JavaScript',target).code)));";
    for entry in fixture["cases"].as_array().unwrap() {
        let source = entry["source"].as_str().unwrap();
        let result = Command::new("node")
            .current_dir(root)
            .args(["--input-type=module", "-e", script, source])
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let expected: Vec<String> = serde_json::from_slice(&result.stdout).unwrap();
        for (index, target) in ["Rust", "Lean", "Rocq"].iter().enumerate() {
            let translated = translate_program(source, "JavaScript", target).unwrap();
            assert_eq!(translated.diagnostic(), None, "{target}: {}", entry["name"]);
            assert_eq!(
                translated.code(),
                expected[index],
                "{target}: {}",
                entry["name"]
            );
        }
        let emitted =
            emit_javascript(&check_program(&parse_javascript(source).unwrap()).unwrap()).unwrap();
        for executed in [source, emitted.text.as_str()] {
            let result = Command::new("node")
                .args(["--input-type=module", "-e", executed])
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
            assert_eq!(
                String::from_utf8(result.stdout).unwrap(),
                entry["expected"].as_str().unwrap()
            );
        }
    }
}

#[test]
fn indexed_slices_and_string_copies_retain_actual_javascript_observations() {
    for (source, expected) in [
        ("function copy() { return [1].slice(0).length; }", "1\n"),
        ("function copy() { return [1].slice(-1, 2).length; }", "1\n"),
        ("function copy() { return 'a'.slice(); }", "a\n"),
    ] {
        let translated = translate_program(source, "JavaScript", "Rust").unwrap();
        assert_eq!(translated.diagnostic(), None, "{source}");
        let runnable = format!("{source}\nconsole.log(copy());");
        let checked = check_program(&parse_javascript(&runnable).unwrap()).unwrap();
        let emitted = emit_javascript(&checked).unwrap();
        for executed in [runnable.as_str(), emitted.text.as_str()] {
            let output = Command::new("node")
                .args(["--input-type=module", "-e", executed])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{source}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert_eq!(
                String::from_utf8(output.stdout).unwrap(),
                expected,
                "{source}"
            );
        }
    }
}

#[test]
fn array_construction_rejects_unrepresented_bounds_iterators_and_mapping() {
    for source in [
        "function copy() { return Array.from().length; }",
        "function copy() { return Array.from([1], 2).length; }",
        "function copy() { return Array.from(2).length; }",
        "function copy() { return Array.of(1, 'a').length; }",
    ] {
        assert!(
            translate_program(source, "JavaScript", "Rust")
                .unwrap()
                .diagnostic()
                .is_some(),
            "{source}"
        );
    }
}
