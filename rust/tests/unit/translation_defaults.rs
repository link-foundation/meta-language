use meta_language::program_translation::translate_program;
use meta_language::translation::javascript::parse_javascript;

const DEFAULTS: &str = r"function label(name, level = 'grammar-rule', depth = 2) {
  return `${name}:${level}:${depth}`;
}
/**
 * @param {string} term
 * @param {number[]} children
 * @returns {number}
 */
function size(term, children = []) {
  return children.length + (term === '' ? 0 : 1);
}
console.log('%s', label('a'), label('b', 'x'), label('c', 'y', 3), size('ab'), size('', [1, 2]));
";

#[test]
fn omitted_arguments_receive_defaults_in_every_target() {
    for target in ["Rust", "Lean", "Rocq"] {
        let translated = translate_program(DEFAULTS, "JavaScript", target).unwrap();
        assert_eq!(translated.diagnostic(), None, "{target}");
        assert!(translated.semantics().is_some(), "{target}");
    }
    let translated = translate_program(DEFAULTS, "JavaScript", "Rust").unwrap();
    for expected in [
        "pub fn label(name: String, level: String, depth: f64) -> String {",
        "crate::label(String::from(\"a\"), String::from(\"grammar-rule\"), 2f64)",
        "crate::label(String::from(\"b\"), String::from(\"x\"), 2f64)",
        "crate::size(String::from(\"ab\"), Vec::<f64>::new())",
    ] {
        assert!(translated.code().contains(expected), "{expected}");
    }
}

#[test]
fn defaults_refuse_parameter_references_and_required_parameters_after_defaults() {
    for (source, expected) in [
        (
            "function f(a = 1, b) { return b; }",
            "parameter b after a default",
        ),
        (
            "function f(a, b = a) { return b; }",
            "default reading parameter a",
        ),
    ] {
        let error = parse_javascript(source).unwrap_err();
        assert!(error.message().contains(expected), "{error}");
    }
}

#[test]
fn default_calls_resolve_names_outside_the_callers_local_scope() {
    let source = "function base() { return 7; }\nfunction chosen(value = base()) { return value; }\nfunction caller(base) { return chosen(); }\nconsole.log(caller(99));\n";
    let translated = translate_program(source, "JavaScript", "Rust").unwrap();
    assert_eq!(translated.diagnostic(), None);
    assert!(
        translated.code().contains("crate::chosen(crate::base())"),
        "{}",
        translated.code()
    );
}

use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_lean::emit_lean;
use meta_language::translation::emit_rocq::emit_rocq;
use meta_language::translation::emit_rust::emit_rust;
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

#[test]
fn lexical_boundaries_keep_regular_expression_bodies_and_division_distinct() {
    use meta_language::translation::Language;
    use meta_language::translation::diagnostics::ErrorKind;
    use meta_language::translation::lexer::{TokenKind, tokenize};
    let fixtures: serde_json::Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/translation-lexical-boundaries.json"
    ))
    .unwrap();
    for literal in fixtures["regexLiterals"].as_array().unwrap() {
        let literal = literal.as_str().unwrap();
        let source = format!("function matches(text) {{ return {literal}.test(text); }}");
        let tokens = tokenize(&source, Language::JavaScript).unwrap();
        assert!(
            tokens
                .tokens
                .iter()
                .any(|token| token.kind == TokenKind::RegularExpression && token.raw == literal),
            "{literal}"
        );
        let error = parse_javascript(&source).unwrap_err();
        assert_eq!(error.kind, ErrorKind::Unsupported, "{literal}: {error}");
        assert_eq!(error.construct.as_deref(), Some("regular expression"));
    }
    for source in fixtures["divisionPrograms"].as_array().unwrap() {
        let source = source.as_str().unwrap();
        check_program(&parse_javascript(source).unwrap()).unwrap();
        assert!(
            !tokenize(source, Language::JavaScript)
                .unwrap()
                .tokens
                .iter()
                .any(|token| token.kind == TokenKind::RegularExpression)
        );
    }
    for source in fixtures["malformedRegex"].as_array().unwrap() {
        let error = tokenize(source.as_str().unwrap(), Language::JavaScript).unwrap_err();
        assert_eq!(error.kind, ErrorKind::Syntax);
        assert!(error.reason.contains("unterminated regular expression"));
    }
    let count = fixtures["repeatedRegex"]["count"].as_u64().unwrap();
    let literal = fixtures["repeatedRegex"]["literal"].as_str().unwrap();
    let source = (0..count)
        .map(|index| format!("const r{index} = {literal};"))
        .collect::<Vec<_>>()
        .join("\n");
    let encoded = meta_language::translation::lexer::Source::new(&source);
    let tokens = tokenize(&source, Language::JavaScript).unwrap();
    let literals: Vec<_> = tokens
        .tokens
        .iter()
        .filter(|token| token.kind == TokenKind::RegularExpression)
        .collect();
    assert_eq!(u64::try_from(literals.len()).unwrap(), count);
    for token in literals {
        assert_eq!(token.raw, literal);
        assert_eq!(encoded.slice(token.start, token.end), literal);
    }
    let source = fixtures["templateProgram"].as_str().unwrap();
    emit_javascript(&check_program(&parse_javascript(source).unwrap()).unwrap()).unwrap();
    assert!(
        tokenize(r"`\01`", Language::JavaScript)
            .unwrap_err()
            .reason
            .contains("octal")
    );
}

#[test]
fn optional_documentation_names_preserve_types_and_runtime_defaults() {
    let source = "/** @param {number} [value=99] @returns {number} */\nfunction add(value = 2) { return value + 1; }\nconsole.log(add(), add(7));\n";
    let surface = parse_javascript(source).unwrap();
    let program = check_program(&surface).unwrap();
    assert_eq!(javascript_output(source), "3 8\n");
    assert_eq!(
        javascript_output(&emit_javascript(&program).unwrap().text),
        "3 8\n"
    );
    let rust = emit_rust(&program).unwrap().text;
    assert!(rust.contains("crate::add(2f64)"), "{rust}");
    assert!(rust.contains("crate::add(7f64)"), "{rust}");
    for target in ["Rust", "Lean", "Rocq"] {
        let translated = translate_program(source, "JavaScript", target).unwrap();
        assert_eq!(translated.diagnostic(), None, "{target}");
    }
    for (source, reason) in [
        (
            "/** @param {number} [value=99] @returns {number} */\nfunction add(value) { return value + 1; }\nconsole.log(add());\n",
            "argument",
        ),
        (
            "/** @param {number} [other] */\nfunction identity(value) { return value; }\n",
            "other is not a parameter",
        ),
    ] {
        let translated = translate_program(source, "JavaScript", "Rust").unwrap();
        assert!(translated.diagnostic().unwrap().message.contains(reason));
    }
}

#[test]
fn type_queries_preserve_bound_value_names_without_discarding_evaluation() {
    let source = "/** @param {number} value @returns {string} */\nfunction numberType(value) { return typeof value; }\n/** @param {bigint} value @returns {string} */\nfunction integerType(value) { return typeof value; }\n/** @param {string} value @returns {string} */\nfunction stringType(value) { return typeof value; }\n/** @param {boolean} value @returns {string} */\nfunction booleanType(value) { return typeof value; }\n/** @param {number[]} value @returns {string} */\nfunction arrayType(value) { return typeof value; }\nconsole.log('%s', numberType(7), integerType(7n), stringType('ready'), booleanType(false), arrayType([]), typeof 7, typeof 7n, typeof false, typeof 'ready');\n";
    let expected = "number bigint string boolean object number bigint boolean string\n";
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    assert_eq!(javascript_output(source), expected);
    assert_eq!(
        javascript_output(&emit_javascript(&program).unwrap().text),
        expected
    );
    for target in ["Rust", "Lean", "Rocq"] {
        assert_eq!(
            translate_program(source, "JavaScript", target)
                .unwrap()
                .diagnostic(),
            None
        );
    }
    for (source, reason) in [
        (
            "function fail() { throw new Error('preserve'); } console.log(typeof fail());",
            "typeof operand",
        ),
        (
            "function answer() { return 7; } console.log(typeof answer);",
            "function value",
        ),
        ("console.log(typeof [1]);", "typeof operand"),
    ] {
        let translated = translate_program(source, "JavaScript", "Rust").unwrap();
        assert!(translated.diagnostic().unwrap().message.contains(reason));
    }
}
