//! JavaScript arrays the program reads: a Rust `Vec`, a Lean `Array` and a
//! Rocq list, read through an optional bounds-checked helper that retains
//! JavaScript's observable absence outside an array.

use meta_language::translation::javascript::parse_javascript;
use meta_language::{TranslationSupport, translate_program};

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
fn arrays_the_program_reads_are_a_semantic_translation_in_every_target_with_observable_absence() {
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
        assert!(!contract.assumptions.contains(&IN_BOUNDS), "{target}");
    }
}

#[test]
fn an_array_is_a_vec_an_array_and_a_list_read_through_an_optional_bounds_checked_helper() {
    let rust = code(ARRAYS, "Rust");
    for expected in [
        "let xs = vec![1f64, 2f64];",
        "match crate::ml_array::optional_at(&xs, crate::ml_array::number_index(1f64)) { Some(ml_present) => ml_present, None => f64::NAN } + (xs.len() as f64)",
        "crate::ml_array::append(vec![crate::ml::Big::from_i128(1)], vec![crate::ml::Big::from_i128(2)])",
    ] {
        assert!(rust.contains(expected), "{rust}");
    }
    let lean = code(ARRAYS, "Lean");
    for expected in [
        "let xs := (#[(1 : Float), (2 : Float)] : (Array Float))",
        "xs[ml_index.toUInt64.toNat]? else none",
        "| none => (0 / 0 : Float)) + (Float.ofNat xs.size)",
        "((#[(1 : Int)] : (Array Int)) ++ (#[(2 : Int)] : (Array Int)))",
    ] {
        assert!(lean.contains(expected), "{lean}");
    }
    let rocq = code(ARRAYS, "Rocq");
    for expected in [
        "let xs := (1%float :: 2%float :: @nil float) in",
        "List.nth_error xs (Z.to_nat ml_index)",
        "| None => PrimFloat.nan end)",
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
        (
            "const xs = [1, 2];\nconsole.log(xs);\n",
            "text of an array: console.log lays an array out with util.inspect and String joins its elements with commas, which the translation does not reproduce yet; print the elements one by one at 31..33",
        ),
        (
            "const xs = [1, 2];\nxs[0] = 3;\n",
            "reference mutation: this mutation requires its shared reference and slot operations to be represented at 19..28",
        ),
        (
            "const xs = [1, 2];\nconsole.log(xs === xs);\n",
            "comparison of arrays: === of two arrays compares which array each is, which a value translation does not keep; compare their elements at 31..40",
        ),
    ] {
        assert_eq!(refusal(source), message, "{source}");
    }
    // This original input was historically refused; the current shared
    // JavaScript model preserves UTF-16 length and therefore admits it.
    for target in ["Rust", "Lean", "Rocq"] {
        code("const s = 'ab';\nconsole.log(s.length);\n", target);
    }
    assert!(refusal("const xs = [1, , 2];\n").starts_with("array hole"));
    assert!(refusal("for (const k in { a: 1 }) console.log(k);\n").starts_with("for…in loop"));
}

#[test]
fn utf16_string_length_retains_supplementary_characters_in_each_target() {
    let source = "const s = 'a😀b';\nconsole.log(s.length);\n";
    assert!(code(source, "Rust").contains("encode_utf16().count() as f64"));
    assert!(code(source, "Lean").contains("if c.toNat > 65535 then 2 else 1"));
    assert!(code(source, "Rocq").contains("ml_utf16_length"));
}

#[test]
fn out_of_bounds_array_reads_retain_absence_instead_of_an_in_bounds_assumption() {
    let source = "function read(xs, i) { return xs[i]; }\nconsole.log(read([1], 4) ?? 9);\n";
    for target in ["Rust", "Lean", "Rocq"] {
        let translated =
            translate_program(source, "JavaScript", target).expect("translation descriptor");
        assert!(
            translated.diagnostic().is_none(),
            "{target}: {:?}",
            translated.diagnostic()
        );
        assert!(
            !translated.contract().assumptions.contains(&IN_BOUNDS),
            "{target}"
        );
        let expected = match target {
            "Rust" => "crate::ml_array::optional_at",
            "Lean" => "else none",
            _ => "List.nth_error",
        };
        assert!(
            translated.code().contains(expected),
            "{}",
            translated.code()
        );
    }
}

#[test]
fn source_iteration_retains_its_actual_iterable_wrapper_and_generated_read_proof() {
    fn count(value: &serde_json::Value, kind: &str, proven: bool) -> usize {
        match value {
            serde_json::Value::Object(fields) => {
                if fields.get("k").and_then(serde_json::Value::as_str) == Some("forOfIterable") {
                    assert_eq!(
                        fields.get("type"),
                        Some(&serde_json::json!({"kind":"array","element":{"kind":"int"}}))
                    );
                }
                usize::from(
                    fields.get("k").and_then(serde_json::Value::as_str) == Some(kind)
                        && (!proven
                            || fields.get("provenInBounds")
                                == Some(&serde_json::Value::Bool(true))),
                ) + fields
                    .values()
                    .map(|value| count(value, kind, proven))
                    .sum::<usize>()
            }
            serde_json::Value::Array(values) => {
                values.iter().map(|value| count(value, kind, proven)).sum()
            }
            _ => 0,
        }
    }
    let program = parse_javascript(ARRAYS).expect("the original array program parses");
    let raw = serde_json::to_value(program).expect("raw source tree serialization");
    assert_eq!(count(&raw, "forOfIterable", false), 1);
    assert_eq!(count(&raw, "index", true), 1);
}

#[test]
fn string_iteration_uses_code_points_while_length_uses_utf16_units() {
    let source = "function count(s) { let n = 0; for (const point of s) n += 1; return n; }\nconsole.log(count('a😀b'));\n";
    assert!(code(source, "Rust").contains(".chars().map(|ml_character| ml_character.to_string())"));
    assert!(
        code(source, "Lean")
            .contains(".toList.map (fun ml_character => String.singleton ml_character)")
    );
    let rocq = translate_program(source, "JavaScript", "Rocq").expect("translation descriptor");
    assert!(
        rocq.diagnostic()
            .expect("explicit unsupported iterator encoding")
            .message
            .starts_with("string iteration:")
    );
}
