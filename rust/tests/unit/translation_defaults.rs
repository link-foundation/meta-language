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
