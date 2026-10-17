use meta_language::program_translation::translate_program;
use std::process::Command;

const SOURCE: &str = r"/** @param {string} value @returns {boolean} */
function allTests(value) { return value.startsWith('👋') && value.endsWith('.js') && value.includes('é'); }
/** @param {string} value @returns {boolean} */
function emptySearch(value) { return value.startsWith('') && value.endsWith('') && value.includes(''); }
console.log(allTests('👋 café.js'), emptySearch(''), emptySearch('你好'), allTests('cafe.js'));
";

#[test]
fn string_predicates_match_javascript_emission_in_every_target() {
    let script = "import {translateProgram} from './js/src/program-translation.js'; import {parseJavaScript} from './js/src/translation/javascript.js'; import {checkProgram} from './js/src/translation/check.js'; import {emitJavaScript} from './js/src/translation/emit-javascript.js'; console.log(JSON.stringify([{code:emitJavaScript(checkProgram(parseJavaScript(process.argv[1]))).text}, ...['Rust','Lean','Rocq'].map(target => translateProgram(process.argv[1], 'JavaScript', target))]));";
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let result = Command::new("node")
        .current_dir(root)
        .args(["--input-type=module", "-e", script, SOURCE])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let expected: Vec<serde_json::Value> = serde_json::from_slice(&result.stdout).unwrap();
    for (target, expected) in ["JavaScript", "Rust", "Lean", "Rocq"]
        .into_iter()
        .zip(expected)
    {
        if target == "JavaScript" {
            let surface = meta_language::translation::javascript::parse_javascript(SOURCE).unwrap();
            let program = meta_language::translation::check::check_program(&surface).unwrap();
            let emitted =
                meta_language::translation::emit_javascript::emit_javascript(&program).unwrap();
            assert_eq!(emitted.text, expected["code"].as_str().unwrap(), "{target}");
            continue;
        }
        let translated = translate_program(SOURCE, "JavaScript", target).unwrap();
        assert_eq!(translated.diagnostic(), None, "{target}");
        assert_eq!(
            translated.code(),
            expected["code"].as_str().unwrap(),
            "{target}"
        );
    }
}

#[test]
fn string_predicates_reject_positions_and_non_string_receivers() {
    for source in [
        "console.log('abc'.includes('a', 1));",
        "console.log(7 .includes('a'));",
        "console.log('abc'.startsWith(7));",
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

#[test]
fn string_predicates_read_rust_method_spellings_and_borrowed_searches() {
    let source = "pub fn all_tests(value: String, search: String) -> bool { value.starts_with(search.as_str()) && value.ends_with(search.as_str()) && value.contains(search.as_str()) } fn main() { println!(\"{}\", all_tests(String::from(\"你好\"), String::from(\"\"))); }";
    for target in ["JavaScript", "Lean", "Rocq"] {
        let translated = translate_program(source, "Rust", target).unwrap();
        assert_eq!(translated.diagnostic(), None, "{target}");
        if target == "JavaScript" {
            let output = Command::new("node")
                .args(["--input-type=module", "-e", translated.code()])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert_eq!(String::from_utf8(output.stdout).unwrap(), "true\n");
        }
    }
}
