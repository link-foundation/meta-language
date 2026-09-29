//! JavaScript arrays the program reads: a Rust `Vec`, a Lean `Array` and a
//! Rocq list, read through a bounds-checked helper, under the assumption that
//! every read is in bounds.

use meta_language::{translate_program, TranslationSupport};

const ARRAYS: &str = "const xs = [1, 2];
console.log(xs[1] + xs.length);
function sum(values) {
  let s = 0n;
  for (const v of values) s += v;
  return s;
}
console.log(sum([1n, ...[2n]]));
";

const IN_BOUNDS: &str = "the translation agrees with the source on executions whose array reads are in bounds; JavaScript reads undefined at an index outside an array, where the target aborts";

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
fn arrays_the_program_reads_are_a_semantic_translation_in_every_target_assuming_reads_in_bounds() {
    for target in ["Rust", "Lean", "Rocq"] {
        let translated =
            translate_program(ARRAYS, "JavaScript", target).expect("translation descriptor");
        assert!(translated.diagnostic().is_none(), "{target}");
        let contract = translated.contract();
        assert_eq!(
            contract.support,
            TranslationSupport::SemanticTranslation,
            "{target}"
        );
        assert!(contract.assumptions.contains(&IN_BOUNDS), "{target}");
    }
}

#[test]
fn an_array_is_a_vec_an_array_and_a_list_read_through_a_bounds_checked_helper() {
    let rust = code(ARRAYS, "Rust");
    for expected in [
        "let xs = vec![1f64, 2f64];",
        "crate::ml_array::at(&xs, crate::ml_array::number_index(1f64)) + (xs.len() as f64)",
        "crate::ml_array::append(vec![crate::ml::Big::from_i128(1)], vec![crate::ml::Big::from_i128(2)])",
    ] {
        assert!(rust.contains(expected), "{rust}");
    }
    let lean = code(ARRAYS, "Lean");
    for expected in [
        "let xs := (#[(1 : Float), (2 : Float)] : (Array Float))",
        "(ml_array_at_float xs (1 : Float)) + (Float.ofNat xs.size)",
        "((#[(1 : Int)] : (Array Int)) ++ (#[(2 : Int)] : (Array Int)))",
    ] {
        assert!(lean.contains(expected), "{lean}");
    }
    let rocq = code(ARRAYS, "Rocq");
    for expected in [
        "let xs := (1%float :: 2%float :: @nil float) in",
        "(ml_list_at xs (ml_float_index 1%float) 0%float)",
        "(List.app (1%Z :: @nil Z) (2%Z :: @nil Z))",
    ] {
        assert!(rocq.contains(expected), "{rocq}");
    }
}

#[test]
fn inference_fixes_the_element_type_of_an_empty_array_from_its_later_use() {
    let source = "function range(n) {\n  let out = [];\n  for (let i = 0n; i < n; i++) out = [...out, i];\n  return out;\n}\nconsole.log(range(3n).length);\n";
    let rust = code(source, "Rust");
    assert!(rust.contains("Vec::<crate::ml::Big>::new()"), "{rust}");
}

#[test]
fn a_typedef_field_may_hold_an_array_of_data_too() {
    let source = "/** @typedef {{ $: 'leaf', items: bigint[] } | { $: 'node', kids: Array<Tree> }} Tree */\nfunction size(t) {\n  if (t.$ === 'leaf') return t.items.length;\n  let n = 0;\n  for (const kid of t.kids) n += size(kid);\n  return n;\n}\nconsole.log(size({ $: 'node', kids: [{ $: 'leaf', items: [1n, 2n] }] }));\n";
    code(source, "Rust");
    // Lean derives no DecidableEq through an Array of data, and Rocq would warn of every nested definition.
    assert!(code(source, "Lean").contains("  deriving Repr, Inhabited"));
    assert!(code(source, "Rocq").contains("#[warnings=\"-register-all\"]\nInductive Tree"));
}

#[test]
fn the_array_forms_that_are_not_kept_are_refused_with_a_reason() {
    for (source, message) in [
        ("const xs = [1, 2];\nconsole.log(xs);\n", "text of an array: console.log lays an array out with util.inspect and String joins its elements with commas, which the translation does not reproduce yet; print the elements one by one at 31..33"),
        ("const xs = [1, 2];\nxs[0] = 3;\n", "assignment of an array element: the portable core reads arrays and does not mutate them; build a new array with [...xs, value] at 19..24"),
        ("const xs = [1, 2];\nconsole.log(xs === xs);\n", "comparison of arrays: === of two arrays compares which array each is, which a value translation does not keep; compare their elements at 31..40"),
        ("const s = 'ab';\nconsole.log(s.length);\n", "length of a string: String.prototype.length counts UTF-16 code units, which the portable string types do not keep at 28..36"),
    ] {
        assert_eq!(refusal(source), message, "{source}");
    }
    assert!(refusal("const xs = [1, , 2];\n").starts_with("array hole"));
    assert!(refusal("for (const k in { a: 1 }) console.log(k);\n").starts_with("for…in loop"));
}
