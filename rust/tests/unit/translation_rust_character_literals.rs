#[test]
fn opaque_rust_character_tokens_preserve_string_values_and_absolute_utf16_spans() {
    let prefix = "// 😀 original UTF16 source\r\n";
    for (literal, kind, expected) in [
        ("'{'", TokenKind::Character, "123"),
        ("'}'", TokenKind::Character, "125"),
        ("'\"'", TokenKind::Character, "34"),
        ("'α'", TokenKind::Character, "945"),
        ("'😀'", TokenKind::Character, "128512"),
        (r"'\u{1_F600___}'", TokenKind::Character, "128512"),
        (r"'\x7F'", TokenKind::Character, "127"),
        (r"'\''", TokenKind::Character, "39"),
        ("b'}'", TokenKind::ByteCharacter, "125"),
        (r"b'\xFF'", TokenKind::ByteCharacter, "255"),
    ] {
        let source = format!("{prefix}{literal} + following");
        let result = tokenize(&source, Language::Rust).unwrap();
        let token = &result.tokens[0];
        assert_eq!(token.kind, kind, "{literal}");
        assert_eq!(token.value, expected, "{literal}");
        assert_eq!(token.raw, literal);
        assert_eq!(
            token.span(),
            Span::new(
                prefix.encode_utf16().count(),
                prefix.encode_utf16().count() + literal.encode_utf16().count()
            )
        );
        assert_eq!(result.comments[0].text, prefix.strip_suffix('\n').unwrap());
        assert_eq!(
            result
                .tokens
                .iter()
                .skip(1)
                .map(|token| token.raw.as_str())
                .collect::<Vec<_>>(),
            ["+", "following", ""]
        );
    }
}

#[test]
fn character_tokens_protect_original_delimiters_comments_and_complete_owned_rust_source() {
    let source =
        "// 😀\nfn main(){ let a='{'; let b=b'}'; let c='/'; let d='*'; let e='\"'; } // outside";
    let result = tokenize(source, Language::Rust).unwrap();
    assert_eq!(
        result
            .tokens
            .iter()
            .filter(|token| matches!(token.kind, TokenKind::Character | TokenKind::ByteCharacter))
            .map(|token| token.raw.as_str())
            .collect::<Vec<_>>(),
        ["'{'", "b'}'", "'/'", "'*'", "'\"'"]
    );
    assert_eq!(
        result
            .tokens
            .iter()
            .filter(
                |token| token.kind == TokenKind::Punct && matches!(token.value.as_str(), "{" | "}")
            )
            .map(|token| token.value.as_str())
            .collect::<Vec<_>>(),
        ["{", "}"]
    );
    assert_eq!(
        result
            .comments
            .iter()
            .map(|comment| comment.text.as_str())
            .collect::<Vec<_>>(),
        ["// 😀", "// outside"]
    );
    let original = "fn a(){m!('\"')} //\"\ntype A = _;";
    let result = tokenize(original, Language::Rust).unwrap();
    assert_eq!(
        result
            .tokens
            .iter()
            .filter(|token| token.kind == TokenKind::Character)
            .map(|token| token.raw.as_str())
            .collect::<Vec<_>>(),
        ["'\"'"]
    );
    assert_eq!(result.comments[0].text, "//\"");
    assert_eq!(
        result
            .tokens
            .iter()
            .rev()
            .take(6)
            .map(|token| token.raw.as_str())
            .collect::<Vec<_>>(),
        ["", ";", "_", "=", "A", "type"]
    );
    let owned = include_str!("../../src/translation/js_number.rs");
    let result = tokenize(owned, Language::Rust).unwrap();
    assert_eq!(
        result
            .tokens
            .iter()
            .filter(|token| matches!(token.kind, TokenKind::Character | TokenKind::ByteCharacter))
            .map(|token| (token.kind, token.value.as_str(), token.raw.as_str()))
            .collect::<Vec<_>>(),
        [
            (TokenKind::Character, "53", "'5'"),
            (TokenKind::ByteCharacter, "48", "b'0'"),
            (TokenKind::ByteCharacter, "48", "b'0'"),
            (TokenKind::Character, "48", "'0'"),
            (TokenKind::Character, "45", "'-'"),
            (TokenKind::Character, "43", "'+'"),
            (TokenKind::ByteCharacter, "57", "b'9'"),
            (TokenKind::ByteCharacter, "48", "b'0'"),
            (TokenKind::Character, "101", "'e'"),
            (TokenKind::Character, "46", "'.'"),
            (TokenKind::Character, "48", "'0'"),
        ]
    );
    let original = meta_language::translation::lexer::Source::new(owned);
    for token in &result.tokens {
        assert_eq!(original.slice(token.start, token.end), token.raw);
    }
    assert_eq!(
        result.tokens.last().unwrap().end,
        owned.encode_utf16().count()
    );
}

#[test]
fn rust_character_selection_preserves_lifetimes_labels_strings_comments_and_foreign_paths() {
    let lifetime = "fn keep<'a>(value: &'a str) -> &'a str { value }";
    let label = "fn run() { 'outer: loop { break 'outer; } }";
    for (source, expected) in [
        (
            lifetime,
            vec![
                "fn", "keep", "<", "'", "a", ">", "(", "value", ":", "&", "'", "a", "str", ")",
                "->", "&", "'", "a", "str", "{", "value", "}", "",
            ],
        ),
        (
            label,
            vec![
                "fn", "run", "(", ")", "{", "'", "outer", ":", "loop", "{", "break", "'", "outer",
                ";", "}", "}", "",
            ],
        ),
    ] {
        let result = tokenize(source, Language::Rust).unwrap();
        assert_eq!(
            result
                .tokens
                .iter()
                .map(|token| token.raw.as_str())
                .collect::<Vec<_>>(),
            expected
        );
        assert!(
            result.tokens.iter().all(|token| !matches!(
                token.kind,
                TokenKind::Character | TokenKind::ByteCharacter
            ))
        );
        let mut end = 0;
        for token in result
            .tokens
            .iter()
            .filter(|token| token.kind != TokenKind::Eof)
        {
            let start = end + source[end..].find(&token.raw).unwrap();
            assert_eq!(token.span(), Span::new(start, start + token.raw.len()));
            end = start + token.raw.len();
        }
        assert_eq!(result.comments.as_slice(), &[]);
    }
    let source = "\"'{' b'x'\" r##\"'}' \" // /* b'y'\"## // 'z'\n/* b'w' */ value";
    let result = tokenize(source, Language::Rust).unwrap();
    assert_eq!(
        result
            .tokens
            .iter()
            .map(|token| (token.kind, token.value.as_str()))
            .collect::<Vec<_>>(),
        [
            (TokenKind::String, "'{' b'x'"),
            (TokenKind::String, "'}' \" // /* b'y'"),
            (TokenKind::Identifier, "value"),
            (TokenKind::Eof, ""),
        ]
    );
    assert_eq!(
        result
            .comments
            .iter()
            .map(|comment| comment.text.as_str())
            .collect::<Vec<_>>(),
        ["// 'z'", "/* b'w' */"]
    );
    for language in [Language::Lean, Language::Rocq] {
        let result = tokenize("'a' b'a' x'", language).unwrap();
        assert_eq!(
            result
                .tokens
                .iter()
                .map(|token| (token.kind, token.value.as_str()))
                .collect::<Vec<_>>(),
            [
                (TokenKind::Punct, "'"),
                (TokenKind::Identifier, "a'"),
                (TokenKind::Identifier, "b'a'"),
                (TokenKind::Identifier, "x'"),
                (TokenKind::Eof, ""),
            ]
        );
    }
    let result = tokenize("'a' b'a' x", Language::JavaScript).unwrap();
    assert_eq!(
        result
            .tokens
            .iter()
            .map(|token| (token.kind, token.value.as_str()))
            .collect::<Vec<_>>(),
        [
            (TokenKind::String, "a"),
            (TokenKind::Identifier, "b"),
            (TokenKind::String, "a"),
            (TokenKind::Identifier, "x"),
            (TokenKind::Eof, ""),
        ]
    );
}

#[test]
fn rust_character_suffixes_and_typed_semantics_remain_explicitly_unsupported() {
    super::translation_string_array_copy_sort::require_rust_profile();
    for literal in ["'a'word", "b'a'u8", "'a'_suffix", "'a'λ"] {
        let source = format!("// 😀\n{literal}");
        let error = tokenize(&source, Language::Rust).unwrap_err();
        let suffix = &literal[literal.rfind('\'').unwrap() + 1..];
        let start = offset(&source, suffix);
        assert_eq!(error.kind, ErrorKind::Unsupported);
        assert_eq!(
            error.reason,
            "Rust character literal suffix is outside the supported lexical profile"
        );
        assert_eq!(error.span, Some(Span::new(start, start + 1)));
    }
    for literal in ["'{'", "b'}'", r"'\u{1F600}'", r"b'\xFF'"] {
        let source = format!("fn value()->u32{{{literal}}}");
        let error = meta_language::translation::rust::parse_rust(&source).unwrap_err();
        if literal.starts_with('b') {
            assert_eq!(error.kind, ErrorKind::Syntax);
            assert_eq!(error.reason, "a Rust program needs fn main");
            let end = source.encode_utf16().count();
            assert_eq!(error.span, Some(Span::new(end, end)));
        } else {
            assert_eq!(error.kind, ErrorKind::Unsupported);
            assert_eq!(
                error.reason,
                "character literal: characters are outside the portable core"
            );
            assert_eq!(
                error.span,
                Some(Span::new(
                    offset(&source, literal),
                    offset(&source, literal) + literal.encode_utf16().count()
                ))
            );
        }
        let source = format!("fn choose(x:u8)->u8{{match x{{{literal}=>1,_=>0}}}}");
        let error = meta_language::translation::rust::parse_rust(&source).unwrap_err();
        if literal.starts_with('b') {
            // Same original input; the actual u8 pattern now reaches required-main.
            assert_eq!(error.kind, ErrorKind::Syntax);
            assert_eq!(error.reason, "a Rust program needs fn main");
            let end = source.encode_utf16().count();
            assert_eq!(error.span, Some(Span::new(end, end)));
        } else {
            assert_eq!(error.kind, ErrorKind::Unsupported);
            assert_eq!(
                error.reason,
                "character pattern: characters are outside the portable core"
            );
        }
    }
    for literal in ["''", "b'é'", r"b'\u{41}'", r"'\x80'"] {
        let error = tokenize(literal, Language::Rust).unwrap_err();
        assert_eq!(error.kind, ErrorKind::Syntax);
        let span = error.span.unwrap();
        assert!(span.start <= span.end && span.end <= literal.encode_utf16().count());
    }
    let error = tokenize(r"'\u{D800}'", Language::Rust).unwrap_err();
    assert_eq!(error.kind, ErrorKind::Unsupported);
    assert_eq!(
        error.reason,
        "Rust character literal is not a Unicode scalar value"
    );
}
