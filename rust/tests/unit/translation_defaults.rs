use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_lean::emit_lean;
use meta_language::translation::emit_rocq::emit_rocq;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::rust::parse_rust;

fn javascript_output(code: &str) -> String {
    let output = std::process::Command::new("node")
        .args(["--input-type=module", "-e", code])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn omitted_arguments_fill_trailing_defaults_and_all_emitters_accept_them() {
    let source = "/** @param {number} n @param {number} amount @returns {number} */\nfunction add(n, amount = 2) { return n + amount; }\nconsole.log(add(3), add(3, 7));\n";
    let surface = parse_javascript(source).unwrap();
    let program = check_program(&surface).unwrap();
    assert_eq!(javascript_output(source), "5 10\n");
    assert_eq!(
        javascript_output(&emit_javascript(&program).unwrap().text),
        "5 10\n"
    );
    assert!(emit_rust(&program).unwrap().text.contains("2f64"));
    emit_lean(&program).unwrap();
    emit_rocq(&program).unwrap();
}

#[test]
fn defaults_reject_parameter_dependencies_and_nondefault_trailing_parameters() {
    for (source, reason) in [
        (
            "function f(a, b = a) { return b; }",
            "default reading parameter a",
        ),
        (
            "function f(a = 1, b) { return b; }",
            "parameter b after a default",
        ),
    ] {
        assert!(
            parse_javascript(source)
                .unwrap_err()
                .message()
                .contains(reason)
        );
    }
}

#[test]
fn unicode_escape_scalars_match_the_javascript_oracle_and_rust_frontend() {
    let source = "console.log('%s', '\\ud83d\\ude00\\u{e9}\\u00e9');\n";
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    assert_eq!(javascript_output(source), "😀éé\n");
    assert_eq!(
        javascript_output(&emit_javascript(&program).unwrap().text),
        "😀éé\n"
    );
    parse_rust("fn main() { println!(\"\\u{1f600}\\u{e9}\"); }").unwrap();
    for source in ["console.log('\\ud83d');", "console.log('\\u{110000}');"] {
        assert!(
            parse_javascript(source)
                .unwrap_err()
                .message()
                .contains("scalar values only")
        );
    }
    assert!(
        parse_javascript("console.log('\\uXYZZ');")
            .unwrap_err()
            .message()
            .contains("malformed unicode escape")
    );
}
