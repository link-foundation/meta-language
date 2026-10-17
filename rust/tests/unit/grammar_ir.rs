use std::collections::BTreeSet;

use meta_language::{
    CharClassItem, FromLinks, Grammar, GrammarExpr, GrammarFormat, GrammarRule, LinkMetadata,
    LinkNetwork, LinkType, LinksCodecError, LinksDecoder, LinksEncoder, RuleKind, ToLinks,
    import_antlr, render_grammar_links,
};

#[test]
fn grammar_builder_covers_expression_algebra_and_recursive_rules() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .source_format(GrammarFormat::Peg)
        .start("expr")
        .rule(
            "expr",
            expr.seq([
                expr.nt("term"),
                expr.rep0(expr.seq([
                    expr.choice_ordered([expr.term("+"), expr.term("-")]),
                    expr.nt("term"),
                ])),
            ]),
        )
        .rule(
            "term",
            expr.seq([
                expr.nt("factor"),
                expr.rep0(expr.seq([
                    expr.choice_ordered([expr.term("*"), expr.term("/")]),
                    expr.nt("factor"),
                ])),
            ]),
        )
        .rule(
            "factor",
            expr.choice_ordered([
                expr.capture(
                    Some("number"),
                    expr.rep1(expr.char_class(false, [CharClassItem::range('0', '9')])),
                ),
                expr.seq([expr.term("("), expr.nt("expr"), expr.term(")")]),
            ]),
        )
        .rule("optional_space", expr.opt(expr.char(' ')))
        .rule("bounded_ident", expr.repeat(expr.nt("letter"), 1, Some(8)))
        .rule(
            "lookahead",
            expr.seq([
                expr.and(expr.nt("letter")),
                expr.not(expr.term("_")),
                expr.any(),
            ]),
        )
        .rule(
            "letter",
            expr.choice_unordered([
                expr.char_range('a', 'z'),
                expr.char_range('A', 'Z'),
                expr.terminal_insensitive("alpha"),
            ]),
        )
        .build();

    assert_eq!(grammar.source_format(), Some(GrammarFormat::Peg));
    assert_eq!(grammar.start_rule().map(GrammarRule::name), Some("expr"));
    assert_eq!(
        grammar.rule_names(),
        [
            "expr",
            "term",
            "factor",
            "optional_space",
            "bounded_ident",
            "lookahead",
            "letter"
        ]
    );
    assert!(matches!(
        grammar.rule("lookahead").map(GrammarRule::expr),
        Some(GrammarExpr::Sequence(parts)) if parts.len() == 3
    ));
}

#[test]
fn referenced_nonterminals_supports_undefined_rule_detection() {
    let expr = Grammar::expr();
    let grammar = Grammar::new()
        .with_rule(GrammarRule::new(
            "stmt",
            expr.choice_unordered([
                expr.nt("assignment"),
                expr.seq([expr.nt("expr"), expr.term(";")]),
            ]),
        ))
        .with_rule(GrammarRule::new(
            "expr",
            expr.choice_ordered([expr.nt("term"), expr.nt("expr")]),
        ));

    assert_eq!(
        grammar.referenced_nonterminals(),
        BTreeSet::from([
            "assignment".to_string(),
            "expr".to_string(),
            "term".to_string()
        ])
    );
    assert_eq!(
        grammar.undefined_nonterminals(),
        BTreeSet::from(["assignment".to_string(), "term".to_string()])
    );
}

#[test]
fn grammar_round_trips_through_links_for_hand_built_fixtures() {
    for grammar in fixture_grammars() {
        let mut encoder = LinksEncoder::new();
        let root = grammar.to_links(&mut encoder);
        let network = encoder.into_network();

        assert!(
            network
                .links()
                .filter(|link| {
                    link.metadata()
                        .term()
                        .is_some_and(|term| term.starts_with("grammar::"))
                })
                .all(|link| link.metadata().link_type() == Some(LinkType::Grammar)),
            "all grammar-tagged links should carry LinkType::Grammar"
        );

        let mut links_decoder = LinksDecoder::new(&network);
        let decoded_grammar =
            Grammar::from_links(&mut links_decoder, root).expect("grammar decodes");
        assert_eq!(decoded_grammar, grammar);
    }
}

#[test]
fn grammar_links_retain_ordered_unicode_lexer_feature_plans_without_source() {
    let mut grammar = import_antlr(include_str!("../fixtures/grammar/antlr/covering.g4"))
        .expect("the original covering fixture imports");
    let mut declarations = grammar.declarations().clone();
    assert_eq!(declarations.lexers.len(), 1);
    let mut first = declarations.lexers[0].clone();
    first.name = "漢字😀".to_owned();
    declarations.lexers.insert(0, first);
    grammar.set_declarations(declarations);

    let mut encoder = LinksEncoder::new();
    let root = grammar.to_links(&mut encoder);
    let network = encoder.into_network();
    let lexers = network
        .links()
        .filter(|link| link.metadata().term() == Some("grammar::lexer"))
        .collect::<Vec<_>>();
    assert_eq!(lexers.len(), 2);
    assert!(lexers.iter().all(|link| {
        link.references().len() == 2 && link.metadata().link_type() == Some(LinkType::Grammar)
    }));
    let lexer_ids = lexers.iter().map(|link| link.id()).collect::<Vec<_>>();
    assert_eq!(
        &network.link(root).unwrap().references()[2 + grammar.rules().len()..],
        lexer_ids.as_slice()
    );

    let text = network.to_lino();
    let reloaded = LinkNetwork::from_lino(&text).expect("the source-free network reloads");
    let mut decoder = LinksDecoder::new(&reloaded);
    let restored = Grammar::from_links(&mut decoder, root).expect("the lexer nodes decode");
    assert_eq!(restored, grammar);
    assert_eq!(restored.declarations().lexers[0].name, "漢字😀");
    assert_eq!(restored.declarations().lexers[1].name, "Covering");
    assert_eq!(
        render_grammar_links(&restored),
        render_grammar_links(&grammar)
    );

    let mut encoder = LinksEncoder::new();
    assert_eq!(restored.to_links(&mut encoder), root);
    assert_eq!(encoder.into_network().to_lino(), text);
}

#[test]
fn grammar_lexer_links_refuse_wrong_arities_names_and_expressions() {
    let grammar = import_antlr(include_str!("../fixtures/grammar/antlr/covering.g4"))
        .expect("the original covering fixture imports");
    let mut encoder = LinksEncoder::new();
    let root = grammar.to_links(&mut encoder);
    let network = encoder.into_network();
    let lexer = network
        .links()
        .find(|link| link.metadata().term() == Some("grammar::lexer"))
        .expect("the imported lexer has its transport node");
    let lexer_id = lexer.id();
    let [name, expression] = lexer.references() else {
        panic!("the encoded lexer has exactly two fields");
    };
    let name = *name;
    let expression = *expression;
    for replacement in [
        vec![],
        vec![name],
        vec![name, expression, name],
        vec![expression, expression],
        vec![name, name],
    ] {
        let mut malformed = network.clone();
        let bad_lexer = malformed.insert_dynamic_link(
            &replacement,
            LinkMetadata::new()
                .with_link_type(LinkType::Grammar)
                .with_term("grammar::lexer"),
        );
        let references = network
            .link(root)
            .unwrap()
            .references()
            .iter()
            .map(|reference| {
                if *reference == lexer_id {
                    bad_lexer
                } else {
                    *reference
                }
            })
            .collect::<Vec<_>>();
        let bad_root = malformed.insert_dynamic_link(
            &references,
            LinkMetadata::new()
                .with_link_type(LinkType::Grammar)
                .with_term("grammar::grammar"),
        );
        let error = Grammar::from_links(&mut LinksDecoder::new(&malformed), bad_root)
            .expect_err("every malformed lexer node is refused");
        if replacement.len() != 2 {
            assert_eq!(
                error,
                LinksCodecError::MalformedObject {
                    object: bad_lexer,
                    reason: format!("expected 2 references, found {}", replacement.len()),
                }
            );
        } else if replacement[0] == expression {
            assert!(
                matches!(error, LinksCodecError::InvalidLiteral { object, type_name, .. }
                if object == expression && type_name == "String")
            );
        } else {
            assert!(
                matches!(error, LinksCodecError::MalformedObject { object, .. }
                if object == name)
            );
        }
    }
}

/// The grammar link tags renamed to readable English names, longest first so
/// no replacement rewrites part of a longer tag, with the tag each replaced.
const RENAMED_GRAMMAR_TAGS: &[(&str, &str)] = &[
    (
        "grammar::character-class-item::character-range",
        "grammar::char-class-item::range",
    ),
    (
        "grammar::character-class-item::character",
        "grammar::char-class-item::char",
    ),
    (
        "grammar::expression::case-insensitive-terminal",
        "grammar::expr::terminal-insensitive",
    ),
    (
        "grammar::expression::zero-or-more-repetition",
        "grammar::expr::zero-or-more",
    ),
    (
        "grammar::expression::one-or-more-repetition",
        "grammar::expr::one-or-more",
    ),
    (
        "grammar::expression::counted-repetition",
        "grammar::expr::repeat",
    ),
    (
        "grammar::expression::optional-expression",
        "grammar::expr::optional",
    ),
    (
        "grammar::expression::positive-predicate",
        "grammar::expr::and",
    ),
    (
        "grammar::expression::negative-predicate",
        "grammar::expr::not",
    ),
    (
        "grammar::expression::empty-expression",
        "grammar::expr::empty",
    ),
    (
        "grammar::expression::character-range",
        "grammar::expr::char-range",
    ),
    (
        "grammar::expression::character-class",
        "grammar::expr::char-class",
    ),
    (
        "grammar::expression::any-character",
        "grammar::expr::any-char",
    ),
    (
        "grammar::expression::nonterminal",
        "grammar::expr::non-terminal",
    ),
    ("grammar::expression::", "grammar::expr::"),
    ("grammar::value::absent-value", "grammar::value::none"),
    ("grammar::value::present-value", "grammar::value::some"),
    ("grammar::value::character::", "grammar::value::char::"),
    ("grammar::value::boolean-value::", "grammar::value::bool::"),
    (
        "grammar::value::natural-number::",
        "grammar::value::usize::",
    ),
];

#[test]
fn grammar_networks_written_with_the_former_tags_still_decode() {
    for grammar in fixture_grammars() {
        let mut encoder = LinksEncoder::new();
        let root = grammar.to_links(&mut encoder);
        // Links Notation text percent-encodes the `:` of a term.
        let written = |tag: &str| tag.replace(':', "%3A");
        let mut text = encoder.into_network().to_lino();
        for (current, former) in RENAMED_GRAMMAR_TAGS {
            text = text.replace(&written(current), &written(former));
        }
        assert!(!text.contains(&written("grammar::expression::")), "{text}");
        assert!(text.contains(&written("grammar::expr::")), "{text}");

        let network = LinkNetwork::from_lino(&text).expect("former network reads");
        let mut links_decoder = LinksDecoder::new(&network);
        let decoded_grammar =
            Grammar::from_links(&mut links_decoder, root).expect("former grammar decodes");
        assert_eq!(decoded_grammar, grammar);
    }
}

fn fixture_grammars() -> Vec<Grammar> {
    let expr = Grammar::expr();
    vec![
        Grammar::builder()
            .source_format(GrammarFormat::Bnf)
            .rule("literal", expr.term("fn"))
            .build(),
        Grammar::builder()
            .source_format(GrammarFormat::Abnf)
            .rule(
                "identifier",
                expr.seq([
                    expr.char_class(
                        false,
                        [
                            CharClassItem::range('a', 'z'),
                            CharClassItem::range('A', 'Z'),
                        ],
                    ),
                    expr.rep0(expr.char_class(
                        false,
                        [
                            CharClassItem::range('a', 'z'),
                            CharClassItem::range('0', '9'),
                        ],
                    )),
                ]),
            )
            .build(),
        Grammar::builder()
            .source_format(GrammarFormat::Ebnf)
            .start("list")
            .rule(
                "list",
                expr.seq([
                    expr.nt("item"),
                    expr.rep0(expr.seq([expr.term(","), expr.nt("item")])),
                    expr.opt(expr.term(",")),
                ]),
            )
            .rule(
                "item",
                expr.choice_unordered([expr.term("a"), expr.term("b")]),
            )
            .build(),
        Grammar::builder()
            .source_format(GrammarFormat::Peg)
            .rule(
                "guarded",
                expr.seq([
                    expr.and(expr.terminal_insensitive("select")),
                    expr.capture_unlabeled(expr.any()),
                    expr.not(expr.term(";")),
                ]),
            )
            .build(),
        Grammar::builder()
            .source_format(GrammarFormat::MetaLanguage)
            .rule("empty", expr.empty())
            .rule("range", expr.repeat(expr.char_range('0', '9'), 2, Some(4)))
            .build(),
        Grammar::builder()
            .source_format(GrammarFormat::Inferred)
            .start("expr")
            .grammar_rule(
                GrammarRule::new(
                    "expr",
                    expr.choice_ordered([expr.nt("term"), expr.nt("expr")]),
                )
                .with_kind(RuleKind::Normal)
                .with_doc("recursive expression"),
            )
            .grammar_rule(
                GrammarRule::new(
                    "term",
                    expr.capture(Some("value"), expr.rep1(expr.nt("digit"))),
                )
                .with_kind(RuleKind::Token)
                .with_concept("concept:number"),
            )
            .rule("digit", expr.char_class(true, [CharClassItem::Char('_')]))
            .build(),
    ]
}
