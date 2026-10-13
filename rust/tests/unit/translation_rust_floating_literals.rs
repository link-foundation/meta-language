//! Shared decimal boundaries and Rust binary64 frontend values.
use meta_language::translation::Language;
use meta_language::translation::lexer::{TokenKind, tokenize};
use meta_language::translation::rust::parse_rust;
use serde_json::json;

#[test]
fn decimal_tokens_retain_fraction_exponent_suffix_and_canonical_value() {
    for (source, value, suffix) in [
        ("1.25f64", "1.25", "f64"),
        ("2e3", "2000", ""),
        ("1.", "1", ""),
        ("2f64", "2", "f64"),
    ] {
        let tokens = tokenize(source, Language::Rust).expect("decimal literal");
        let token = &tokens.tokens[0];
        assert_eq!(token.kind, TokenKind::Number);
        assert_eq!(token.value, value);
        assert_eq!(token.suffix, suffix);
        assert!(token.floating);
    }
    for source in ["1..2", "1.floor()"] {
        let tokens = tokenize(source, Language::Rust).expect("separate boundary");
        assert_eq!(tokens.tokens[0].raw, "1");
        assert!(!tokens.tokens[0].floating);
    }
    let tokens = tokenize("5usize", Language::Rust).expect("integer suffix");
    assert_eq!(tokens.tokens[0].suffix, "usize");
    assert!(!tokens.tokens[0].floating);
    assert!(tokenize("1e9999f64", Language::Rust).is_err());
}

#[test]
fn annotated_and_suffixed_binary64_values_enter_the_shared_surface_type() {
    let source = "pub fn scale(value: f64) -> f64 { value * 1.25f64 + 2e3 } fn main() {}";
    let value = serde_json::to_value(parse_rust(source).expect("binary64 frontend"))
        .expect("surface serializes");
    assert_eq!(
        value["items"][0]["params"][0]["type"],
        json!({"kind": "float"})
    );
    assert_eq!(value["items"][0]["ret"], json!({"kind": "float"}));
    assert_eq!(value["items"][0]["body"]["left"]["right"]["value"], "1.25");
    assert_eq!(value["items"][0]["body"]["right"]["value"], "2000");
    assert!(parse_rust("fn value() -> f32 { 1.25f32 }").is_err());
}
