//! Mutually recursive functions: a Lean `mutual` block of partial defs, and in
//! Rocq one `ml_fix` over the sum of the functions' parameter tuples.

use meta_language::{TranslationSupport, translate_program};

const EVEN_ODD: &str = "/** @param {bigint} n @returns {boolean} */
function isEven(n) { if (n === 0n) return true; return isOdd(n - 1n); }
/** @param {bigint} n @returns {boolean} */
function isOdd(n) { if (n === 0n) return false; return isEven(n - 1n); }
console.log(isEven(10n));
";

const TURNS: &str = "/** @param {bigint} n @param {bigint} acc @returns {bigint} */
function first(n, acc) { if (n === 0n) return acc; return second(n - 1n, acc + 1n); }
/** @param {bigint} n @param {bigint} acc @returns {bigint} */
function second(n, acc) { if (n === 0n) return acc; return third(n - 1n, acc + 10n); }
/** @param {bigint} n @param {bigint} acc @returns {bigint} */
function third(n, acc) { if (n === 0n) return acc; return first(n - 1n, acc + 100n); }
console.log(first(10n, 0n));
";

#[test]
fn mutually_recursive_functions_are_a_semantic_translation_in_every_target() {
    for target in ["Rust", "Lean", "Rocq"] {
        let translated =
            translate_program(EVEN_ODD, "JavaScript", target).expect("translation descriptor");
        assert_eq!(
            translated.contract().support,
            TranslationSupport::SemanticTranslation,
            "{target}"
        );
    }
}

#[test]
fn a_lean_mutual_group_is_a_mutual_block_of_partial_defs() {
    let translated =
        translate_program(EVEN_ODD, "JavaScript", "Lean").expect("translation descriptor");
    let code = translated.code();
    let start = code
        .find("\nmutual\n\npartial def isEven (n : Int) : Bool :=\n")
        .expect("the mutual block");
    let block = &code[start..];
    let odd = block
        .find("\n\npartial def isOdd (n : Int) : Bool :=\n")
        .expect("the second member");
    assert!(block[odd..].contains("\n\nend\n"), "{block}");
}

#[test]
fn a_rocq_mutual_group_is_one_ml_fix_projected_to_each_function() {
    let translated =
        translate_program(TURNS, "JavaScript", "Rocq").expect("translation descriptor");
    let code = translated.code();
    for expected in [
        "Definition ml_mutual_first : ((Z * Z) + ((Z * Z) + (Z * Z))) -> (Z + (Z + Z)) :=\n  ml_fix 64 (fun (ml_rec : ((Z * Z) + ((Z * Z) + (Z * Z))) -> (Z + (Z + Z))) (ml_args : ((Z * Z) + ((Z * Z) + (Z * Z)))) =>\n    match ml_args with\n    | (inl (n, acc)) => ",
        "(match ml_rec (inr (inl ((Z.sub n 1%Z), (Z.add acc 1%Z)))) with (inr (inl ml_r)) => ml_r | _ => 0%Z end)",
        "(fun ml_args => match ml_args with (inl _) => (inl 0%Z) | (inr (inl _)) => (inr (inl 0%Z)) | (inr (inr _)) => (inr (inr 0%Z)) end).",
        "Definition third (n : Z) (acc : Z) : Z :=\n  match ml_mutual_first (inr (inr (n, acc))) with (inr (inr ml_r)) => ml_r | _ => 0%Z end.",
    ] {
        assert!(code.contains(expected), "{expected}\n{code}");
    }
}
