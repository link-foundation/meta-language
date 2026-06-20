use meta_language::{
    categorise, infer_lexical_classes, ByteRange, CharCategory, GrammarExpr, LexicalConfig,
    RuleKind,
};

#[test]
fn categorise_maps_representative_characters() {
    assert_eq!(categorise('a'), CharCategory::Letter);
    assert_eq!(categorise('Z'), CharCategory::Letter);
    assert_eq!(categorise('é'), CharCategory::Letter);
    assert_eq!(categorise('7'), CharCategory::Digit);
    assert_eq!(categorise(' '), CharCategory::Whitespace);
    assert_eq!(categorise('\n'), CharCategory::Whitespace);
    assert_eq!(categorise('+'), CharCategory::Punctuation);
    assert_eq!(categorise('😀'), CharCategory::Other);

    for delimiter in ['(', ')', '[', ']', '{', '}', '\'', '"', '`'] {
        assert_eq!(categorise(delimiter), CharCategory::Delimiter);
    }
}

#[test]
fn tokenize_segments_by_category_and_preserves_spans() {
    let model = infer_lexical_classes(&["foo123 + bar;"], &LexicalConfig::default());
    let tokens = model.tokenize("foo123 + bar;");
    let actual = tokens
        .iter()
        .map(|token| (token.text.as_str(), token.category, token.span))
        .collect::<Vec<_>>();

    assert_eq!(
        actual,
        vec![
            ("foo123", CharCategory::Letter, ByteRange::new(0, 6)),
            (" ", CharCategory::Whitespace, ByteRange::new(6, 7)),
            ("+", CharCategory::Punctuation, ByteRange::new(7, 8)),
            (" ", CharCategory::Whitespace, ByteRange::new(8, 9)),
            ("bar", CharCategory::Letter, ByteRange::new(9, 12)),
            (";", CharCategory::Punctuation, ByteRange::new(12, 13)),
        ]
    );
}

#[test]
fn delimiters_are_atomic_tokens() {
    let input = "([{'\"`x`\"'}])";
    let model = infer_lexical_classes(&[input], &LexicalConfig::default());
    let tokens = model.tokenize(input);
    let actual = tokens
        .iter()
        .map(|token| (token.text.as_str(), token.category))
        .collect::<Vec<_>>();

    assert_eq!(
        actual,
        vec![
            ("(", CharCategory::Delimiter),
            ("[", CharCategory::Delimiter),
            ("{", CharCategory::Delimiter),
            ("'", CharCategory::Delimiter),
            ("\"", CharCategory::Delimiter),
            ("`", CharCategory::Delimiter),
            ("x", CharCategory::Letter),
            ("`", CharCategory::Delimiter),
            ("\"", CharCategory::Delimiter),
            ("'", CharCategory::Delimiter),
            ("}", CharCategory::Delimiter),
            ("]", CharCategory::Delimiter),
            (")", CharCategory::Delimiter),
        ]
    );
}

#[test]
fn class_formation_generalises_identifiers_and_numbers_and_keeps_keywords_literals() {
    let corpus = [
        "if alpha1 100;",
        "else beta2 200;",
        "if gamma3 300;",
        "else delta4 400;",
    ];
    let model = infer_lexical_classes(&corpus, &LexicalConfig::default());

    assert_token_terminal(&model, "if");
    assert_token_terminal(&model, "else");
    assert_token_terminal(&model, ";");

    let identifier = model
        .classes
        .iter()
        .find(|rule| rule.name == "identifier")
        .expect("identifier rule is inferred");
    assert_eq!(identifier.kind, RuleKind::Token);
    assert!(matches!(
        identifier.expr,
        GrammarExpr::Sequence(ref parts)
            if matches!(parts.as_slice(), [
                GrammarExpr::CharClass { .. },
                GrammarExpr::ZeroOrMore(_)
            ])
    ));

    let integer = model
        .classes
        .iter()
        .find(|rule| rule.name == "integer")
        .expect("integer rule is inferred");
    assert_eq!(integer.kind, RuleKind::Token);
    assert_eq!(
        integer.expr,
        GrammarExpr::OneOrMore(Box::new(GrammarExpr::CharRange('0', '9')))
    );
}

#[test]
fn max_closed_forms_controls_literal_vs_open_letter_classes() {
    let closed = infer_lexical_classes(
        &["red blue"],
        &LexicalConfig {
            max_closed_forms: 2,
        },
    );
    assert_token_terminal(&closed, "red");
    assert_token_terminal(&closed, "blue");
    assert!(closed.classes.iter().all(|rule| rule.name != "identifier"));

    let open = infer_lexical_classes(
        &["red blue"],
        &LexicalConfig {
            max_closed_forms: 1,
        },
    );
    assert!(open.classes.iter().all(
        |rule| !matches!(&rule.expr, GrammarExpr::Terminal(text) if text == "red" || text == "blue")
    ));
    assert!(open.classes.iter().any(|rule| rule.name == "identifier"));
}

#[test]
fn inference_is_deterministic_and_tokenize_is_lossless() {
    let corpus = ["if alpha1 100;", "else beta2 200;"];
    let config = LexicalConfig {
        max_closed_forms: 2,
    };
    let first = infer_lexical_classes(&corpus, &config);
    let second = infer_lexical_classes(&corpus, &config);

    assert_eq!(first, second);

    let text = "if gamma3 300;";
    let reconstructed = first
        .tokenize(text)
        .into_iter()
        .map(|token| token.text)
        .collect::<String>();
    assert_eq!(reconstructed, text);
}

fn assert_token_terminal(model: &meta_language::LexicalModel, expected: &str) {
    assert!(
        model.classes.iter().any(|rule| {
            rule.kind == RuleKind::Token
                && matches!(&rule.expr, GrammarExpr::Terminal(text) if text == expected)
        }),
        "missing token terminal {expected:?} in {:#?}",
        model.classes
    );
}
