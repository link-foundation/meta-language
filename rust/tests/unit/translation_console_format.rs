//! `console.log` with several arguments prints `util.format` of them: a
//! literal first string reads its directives, and the arguments left over
//! follow after spaces.

use meta_language::{TranslationSupport, translate_program};

const FORMAT: &str = "const x = -0;
console.log('%s=%d items', 'n', 3n, x, true);
";

fn code(source: &str, target: &str) -> String {
    translate_program(source, "JavaScript", target)
        .expect("translation descriptor")
        .code()
        .to_owned()
}

fn refusal(source: &str) -> String {
    translate_program(source, "JavaScript", "Rust")
        .expect("translation descriptor")
        .diagnostic()
        .expect("a diagnostic")
        .message
        .clone()
}

#[test]
fn console_log_with_several_arguments_is_a_semantic_translation_in_every_target() {
    for target in ["Rust", "Lean", "Rocq"] {
        let translated =
            translate_program(FORMAT, "JavaScript", target).expect("translation descriptor");
        assert!(translated.diagnostic().is_none(), "{target}");
        assert_eq!(
            translated.contract().support,
            TranslationSupport::SemanticTranslation,
            "{target}"
        );
    }
}

#[test]
fn the_arguments_print_as_util_format_joins_them() {
    let lean = code(FORMAT, "Lean");
    assert!(
        lean.contains("IO.println (((((((\"n\" ++ \"=\") ++ ((toString (3 : Int)) ++ \"n\")) ++ \" items\") ++ \" \") ++ (ml_js_console x)) ++ \" \") ++ (toString true))"),
        "{lean}"
    );
    let plain = code("console.log('100%%', 1n, '%s');\n", "Lean");
    assert!(
        plain.contains("IO.println ((((\"100%\" ++ \" \") ++ ((toString (1 : Int)) ++ \"n\")) ++ \" \") ++ \"%s\")"),
        "{plain}"
    );
}

#[test]
fn the_util_format_forms_that_are_not_kept_are_refused_with_a_reason() {
    assert_eq!(
        refusal("const s = 'a';\nconsole.log(s, 1n);\n"),
        "console.log with a computed first string: util.format reads the % directives of a first string argument, which only the run knows; pass a literal format string, or print one template literal at 27..28"
    );
    assert_eq!(
        refusal("console.log('%d', 'x');\n"),
        "%d of a string: %d prints a BigInt or a Number; the conversion Number(value) of other values is not kept at 18..21"
    );
    assert_eq!(
        refusal("console.log('%i', 1.5);\n"),
        "%i of a float: %i prints a BigInt; the conversion parseInt(value) of other values is not kept at 18..21"
    );
    assert_eq!(
        refusal("console.log('%j', 1n);\n"),
        "console.log %j directive: the portable directives are %s, %d, %i, %c and %% at 12..16"
    );
    assert_eq!(
        refusal("const s = 'x';\nconsole.log('%c', s);\n"),
        "%c with a computed style: console.log discards the CSS a %c directive takes; pass it as a string literal at 33..34"
    );
}
