//! Deferred shared lexical controls, including independently authored octets.
use meta_language::read_structured_data_lino;
use meta_language::translation::{
    Language,
    lexer::{TokenKind, tokenize},
};
#[test]
fn actual_byte_tokens_keep_original_spans_and_exact_octets() {
    let fixture = read_structured_data_lino(include_str!(
        "../../parity/fixtures/rust-byte-string-literals.lino"
    ))
    .unwrap();
    let prefix = "// 😀 absolute source\r\n";
    for row in fixture["positives"].as_array().unwrap() {
        let raw = row["source"].as_str().unwrap();
        let source = format!("{prefix}{raw} + following");
        let actual = tokenize(&source, Language::Rust).unwrap();
        let token = &actual.tokens[0];
        assert_eq!(token.kind, TokenKind::ByteString);
        assert_eq!(token.value, row["value"].as_str().unwrap());
        assert_eq!(token.suffix, row["suffix"].as_str().unwrap_or(""));
        assert_eq!(token.raw, raw);
        assert_eq!(token.start, prefix.encode_utf16().count());
        assert_eq!(
            token.end,
            prefix.encode_utf16().count() + raw.encode_utf16().count()
        );
        assert_eq!(
            actual.tokens[1..]
                .iter()
                .map(|token| token.raw.as_str())
                .collect::<Vec<_>>(),
            vec!["+", "following", ""]
        );
    }
    for row in fixture["negatives"].as_array().unwrap() {
        let error = tokenize(row["source"].as_str().unwrap(), Language::Rust).unwrap_err();
        assert!(
            error.to_string().contains(row["reason"].as_str().unwrap()),
            "{error}"
        );
    }
}
