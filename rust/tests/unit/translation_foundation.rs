use meta_language::translation::decimal::Decimal;
use meta_language::translation::lexer::{tokenize, TokenCursor};
use meta_language::translation::types::{data, fixed, fixed_bounds, rust_fixed_type, NAT};
use meta_language::translation::Language;

#[test]
fn decimals_parse_and_compare() {
    assert_eq!(Decimal::parse("0x1f").unwrap().to_string(), "31");
    assert_eq!(Decimal::parse("0b101").unwrap().to_string(), "5");
    assert_eq!(Decimal::parse("007").unwrap().to_string(), "7");
    assert_eq!(Decimal::parse("-0").unwrap().to_string(), "0");
    let big = Decimal::parse("1219326311370217952237463801111263526900").unwrap();
    assert_eq!(big.to_string(), "1219326311370217952237463801111263526900");
    assert!(big > Decimal::from_u128(u128::MAX));
    assert!(Decimal::parse("-5").unwrap() < Decimal::parse("-4").unwrap());
    assert!(Decimal::parse("-5").unwrap() < Decimal::parse("0").unwrap());
    assert_eq!(Decimal::parse("12a"), None);
}

fn values(text: &str, language: Language) -> Vec<String> {
    tokenize(text, language)
        .unwrap()
        .tokens
        .into_iter()
        .map(|token| token.raw)
        .collect()
}

#[test]
fn tokens_match_the_javascript_runtime() {
    assert_eq!(
        values("def f (n : Nat) : Nat := Nat.succ n -- c", Language::Lean),
        ["def", "f", "(", "n", ":", "Nat", ")", ":", "Nat", ":=", "Nat.succ", "n", ""]
    );
    assert_eq!(
        values("println!(\"{}\", x != 1_0u8)", Language::Rust)[0],
        "println!"
    );
    assert_eq!(
        values("a >>>= 5n", Language::JavaScript),
        ["a", ">>>=", "5n", ""]
    );
    let tokens = tokenize("`a${b}c`", Language::JavaScript).unwrap().tokens;
    assert_eq!(tokens[0].parts[0].expression.as_ref().unwrap().offset, 4);
    let rocq = tokenize("\"a\"\"b\" (* (* x *) *) x", Language::Rocq).unwrap();
    assert_eq!(rocq.tokens[0].value, "a\"b");
    assert_eq!(rocq.comments.len(), 1);
    let emoji = tokenize("😀x", Language::Lean).unwrap().tokens;
    assert_eq!((emoji[0].start, emoji[0].end, emoji[1].start), (0, 2, 2));
    let error = tokenize("1_", Language::Rust).unwrap_err();
    assert_eq!(error.to_string(), "malformed number 1_ at 0..2");
    let mut cursor = TokenCursor::new(
        tokenize("x", Language::Rust).unwrap().tokens,
        Language::Rust,
    );
    assert_eq!(
        cursor.expect("(", Some("call")).unwrap_err().to_string(),
        "expected ( in call but found \"x\" at 0..1"
    );
}

#[test]
fn type_keys_and_bounds() {
    assert_eq!(fixed(32, true).key(), "i32");
    assert_eq!(data("A.T").key(), "data:A.T");
    assert_eq!(NAT.key(), "nat");
    assert_eq!(fixed_bounds(8, true), (-128, 127));
    assert_eq!(fixed_bounds(8, false), (0, 255));
    assert_eq!(fixed_bounds(128, false), (0, u128::MAX));
    assert_eq!(fixed_bounds(128, true).0, i128::MIN);
    assert_eq!(rust_fixed_type("usize"), Some(fixed(64, false)));
    assert_eq!(rust_fixed_type("u7"), None);
    assert_eq!(
        serde_json::to_string(&fixed(16, false)).unwrap(),
        r#"{"kind":"fixed","bits":16,"signed":false}"#
    );
}
