//! The exactly specified `Math` and `Number` functions and constants: each
//! target's own function where it agrees with JavaScript, and a generated
//! helper where it does not.

use meta_language::{TranslationSupport, translate_program};

const MATH: &str = "/**
 * @param {number[]} xs
 * @returns {number}
 */
function spread(xs) {
  return Math.max(...xs) - Math.min(...xs);
}
console.log(spread([3, 1]), Math.round(-0.5), Math.max(1, 2, 3), Number.isInteger(Math.PI), Math.abs(-1));
";

fn code(source: &str, target: &str) -> String {
    let translated =
        translate_program(source, "JavaScript", target).expect("translation descriptor");
    assert!(
        translated.diagnostic().is_none(),
        "{target}: {:?}",
        translated.diagnostic()
    );
    translated.code().to_owned()
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
fn the_exactly_specified_math_and_number_functions_are_a_semantic_translation_in_every_target() {
    for target in ["Rust", "Lean", "Rocq"] {
        let translated =
            translate_program(MATH, "JavaScript", target).expect("translation descriptor");
        assert!(translated.diagnostic().is_none(), "{target}");
        assert_eq!(
            translated.contract().support,
            TranslationSupport::SemanticTranslation,
            "{target}"
        );
    }
}

#[test]
fn math_functions_are_the_targets_own_where_they_agree_and_generated_helpers_where_not() {
    let rust = code(MATH, "Rust");
    for expected in [
        "(crate::ml_math::max_of(&xs) - crate::ml_math::min_of(&xs))",
        "crate::ml_math::round((-0.5f64))",
        "crate::ml_math::max(crate::ml_math::max(1f64, 2f64), 3f64)",
        "crate::ml_math::is_integer(3.141592653589793f64)",
        "((-1f64)).abs()",
    ] {
        assert!(rust.contains(expected), "{rust}");
    }
    let lean = code(MATH, "Lean");
    for expected in [
        "((xs.foldl ml_max (-1.0 / 0.0 : Float)) - (xs.foldl ml_min (1.0 / 0.0 : Float)))",
        "(ml_round (-0.5 : Float))",
        "(Float.abs (-1 : Float))",
    ] {
        assert!(lean.contains(expected), "{lean}");
    }
    let rocq = code(MATH, "Rocq");
    for expected in [
        "(PrimFloat.sub (List.fold_left ml_max xs PrimFloat.neg_infinity) (List.fold_left ml_min xs PrimFloat.infinity))",
        "Definition ml_trunc (x : float) : float :=\n  match Prim2SF x with",
        "(PrimFloat.abs (PrimFloat.opp 1%float))",
    ] {
        assert!(rocq.contains(expected), "{rocq}");
    }
}

#[test]
fn inference_makes_a_parameter_a_math_function_reads_a_number() {
    let source = "function half(n) {\n  return Math.floor(n / 2);\n}\nconsole.log(half(7));\n";
    assert!(code(source, "Rust").contains("pub fn half(n: f64) -> f64 {"));
}

#[test]
fn the_math_and_number_forms_that_are_not_kept_are_refused_with_a_reason() {
    for (source, message) in [
        (
            "console.log(Math.floor(1n));\n",
            "Math.floor of int; it takes Numbers, and converts or rejects anything else at 23..25",
        ),
        (
            "console.log(Math.sin(1));\n",
            "Math.sin: the JavaScript standard library is outside the portable core at 12..23",
        ),
        (
            "console.log(Math.abs(1, 2));\n",
            "Math.abs with 2 arguments: Math.abs takes one Number at 12..26",
        ),
        (
            "const xs = [1, 2];\nconsole.log(Math.abs(...xs));\n",
            "spread argument: pass Math.abs its argument at 43..45",
        ),
        (
            "const m = Math.max;\n",
            "function value Math.max: functions and namespaces are only portable when a function is called at 10..18",
        ),
        (
            "function isNaN(x) {\n  return x;\n}\n",
            "declaration of isNaN: it shadows the JavaScript global isNaN, which the translation reads as the built-in; rename it at 9..14",
        ),
    ] {
        assert_eq!(refusal(source), message, "{source}");
    }
}
