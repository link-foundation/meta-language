//! Primitive method source, checked nodes and actual target observations.

use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::javascript::parse_javascript;
use std::fs;
use std::process::Command;

fn javascript(source: &str) -> String {
    let output = Command::new("node")
        .args(["--input-type=module", "-e", source])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn rust(source: &str, name: &str) -> String {
    let directory = std::env::temp_dir().join(format!(
        "meta-language-primitive-methods-{}-{name}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).unwrap();
    let file = directory.join("program.rs");
    let executable = directory.join("program");
    fs::write(&file, source).unwrap();
    let compiled = Command::new("rustc")
        .arg("--edition=2024")
        .arg(&file)
        .arg("-o")
        .arg(&executable)
        .output()
        .unwrap();
    assert!(
        compiled.status.success(),
        "{}; source retained at {}",
        String::from_utf8_lossy(&compiled.stderr),
        file.display()
    );
    let output = Command::new(&executable).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = String::from_utf8(output.stdout).unwrap();
    fs::remove_dir_all(directory).unwrap();
    output
}

#[test]
fn primitive_methods_run_with_utf16_defaults_slice_and_same_value_zero() {
    let source = "/** @param {string} text @returns {string} */function crop(text){return text.slice(1,3);}\n/** @param {number[]} values @returns {number[]} */function middle(values){return values.slice(1,-1);}\n/** @param {number[]} values @param {number} sought @returns {boolean} */function has(values,sought){return values.includes(sought);}\nconsole.log(`${crop('A😀B')}`);console.log(middle([1,2,3,4]).length);console.log('abc'.split().length);console.log(`${'x'.padStart(3)}`);console.log(`${'aba'.replaceAll('a','x')}`);console.log(has([NaN],NaN),has([-0],0),has([0],-0),has([1],1),has([1],2));";
    let expected = "😀\n2\n1\n  x\nxbx\ntrue true true true false\n";
    assert_eq!(javascript(source), expected);
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    assert_eq!(
        javascript(&emit_javascript(&checked).unwrap().text),
        expected
    );
    let emitted = emit_rust(&checked).unwrap();
    assert_eq!(rust(&emitted.text, "scalar"), expected);
}

#[test]
fn nullable_membership_retains_nan_and_separate_absences() {
    let source = "/** @param {(number|null|undefined)[]} values @param {number|null|undefined} sought @returns {boolean} */function has(values,sought){return values.includes(sought);}\nconsole.log(has([NaN,null,undefined,-0],NaN),has([null],undefined),has([undefined],null),has([-0],0));";
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    assert_eq!(javascript(source), "true false false true\n");
    assert_eq!(
        javascript(&emit_javascript(&checked).unwrap().text),
        "true false false true\n"
    );
    let emitted = emit_rust(&checked).unwrap();
    assert!(
        emitted
            .text
            .contains("(crate::MlOptional::Null, crate::MlOptional::Null)")
    );
    assert!(emitted.text.contains("a.is_nan() && b.is_nan()"));
    assert_eq!(rust(&emitted.text, "nullable"), "true false false true\n");
}

#[test]
fn slice_argument_errors_keep_the_actual_method_span() {
    let source =
        "/** @param {string} text @returns {string} */function crop(text){return text.slice(1n);}";
    let error = parse_javascript(source).unwrap_err();
    assert_eq!(error.kind.as_str(), "type");
    assert_eq!(
        error.reason,
        "a BigInt is used as a number; JavaScript does not mix BigInt with other types"
    );
    let span = error.span.unwrap();
    assert_eq!(span.start, source.find("text.slice").unwrap());
    assert_eq!(span.end, source.find(';').unwrap());
}

#[test]
fn utf16_method_self_translation_matches_javascript_and_restores_the_actual_source() {
    let source = "/** @param {string} text @returns {string} */\nexport function crop(text){return text.trim().slice(1,3);}\n";
    let script = "import{selfTranslate}from'./js/src/self-translation.js';process.stdout.write(selfTranslate(process.argv[1],'JavaScript','Rust').code);";
    let output = Command::new("node")
        .current_dir(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap(),
        )
        .args(["--input-type=module", "-e", script, source])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let translated = meta_language::self_translate(source, "JavaScript", "Rust").unwrap();
    assert!(
        translated
            .items
            .iter()
            .all(|item| item.status == "translated")
    );
    assert_eq!(translated.code, String::from_utf8(output.stdout).unwrap());
    assert_eq!(
        meta_language::self_translate(&translated.code, "Rust", "JavaScript")
            .unwrap()
            .code,
        source
    );
    let program = translated.code
        + "\nfn main(){println!(\"{}\",crop(crate::ml_string::Text::from_text(\" A😀B \")));}\n";
    assert_eq!(rust(&program, "self-translation"), "😀\n");
}
