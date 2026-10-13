//! Shared cooked-string continuation corpus, source spans and semantic reload.
use meta_language::translation::lexer::{TokenKind, tokenize};
use meta_language::{
    LinkNetwork, create_typed_program_links, decode_typed_program_links, emit_typed_program_links,
    read_structured_data_lino,
};
use serde_json::Value;

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/translation-rust-string-continuations.lino"
    ))
    .expect("the automatically converted continuation corpus")
}

#[test]
fn rust_continuation_tokens_keep_original_utf16_spans_and_utf8_bytes() {
    let fixture = fixture();
    let cases = fixture["fixtures"].as_array().expect("cases");
    assert_eq!(cases.len(), 19);
    for case in cases {
        let literal = case["literal"].as_str().expect("literal");
        let expected = case["value"].as_str().expect("value");
        let source = format!("// 😀\nfn main(){{println!(\"{{}}\",{literal});}}\n");
        let tokens = tokenize(&source, meta_language::translation::Language::Rust)
            .expect("the original Rust literal is accepted");
        let token = tokens
            .tokens
            .iter()
            .find(|token| token.raw == literal)
            .expect("the literal is one token");
        assert_eq!(token.kind, TokenKind::String);
        assert_eq!(token.value, expected);
        assert_eq!(token.raw, literal);
        let byte_start = source.find(literal).expect("source literal");
        let byte_end = byte_start + literal.len();
        assert_eq!(&source.as_bytes()[byte_start..byte_end], literal.as_bytes());
        assert_eq!(token.start, source[..byte_start].encode_utf16().count());
        assert_eq!(token.end, source[..byte_end].encode_utf16().count());
    }
}

#[test]
fn continued_rust_literals_reload_as_semantic_links_and_emit_both_targets() {
    let fixture = fixture();
    for case in fixture["fixtures"].as_array().expect("cases") {
        let literal = case["literal"].as_str().expect("literal");
        let expected = case["value"].as_str().expect("value");
        let source = format!("fn main(){{println!(\"{{}}\",{literal});}}");
        let created = create_typed_program_links(&source, "Rust").expect("typed Rust import");
        let decoded = decode_typed_program_links(&created.network, created.root)
            .expect("the original semantic graph");
        let reloaded = LinkNetwork::from_lino(&created.network.to_lino())
            .expect("source-free Links Notation reload");
        assert_eq!(
            decode_typed_program_links(&reloaded, created.root).expect("reloaded semantic graph"),
            decoded
        );
        for (target, language) in [
            (
                "JavaScript",
                meta_language::translation::Language::JavaScript,
            ),
            ("Rust", meta_language::translation::Language::Rust),
        ] {
            let emitted = emit_typed_program_links(&reloaded, created.root, target)
                .expect("canonical target emission");
            let tokens = tokenize(&emitted.code, language).expect("target literal tokenization");
            assert!(
                tokens
                    .tokens
                    .iter()
                    .any(|token| { token.kind == TokenKind::String && token.value == expected })
            );
        }
    }
}

#[test]
fn rust_and_javascript_retain_their_distinct_continuation_whitespace() {
    let source = "\"a\\\n \t b\"";
    for (language, expected) in [
        (meta_language::translation::Language::Rust, "ab"),
        (meta_language::translation::Language::JavaScript, "a \t b"),
    ] {
        let tokens = tokenize(source, language).expect("the selected continuation model");
        assert_eq!(tokens.tokens[0].value, expected);
        assert_eq!(tokens.tokens[0].raw, source);
        assert_eq!(tokens.tokens[0].end, source.encode_utf16().count());
    }
}
