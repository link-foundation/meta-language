use std::collections::BTreeSet;

use meta_language::{
    CharClassItem, FromLinks, Grammar, GrammarExpr, GrammarFormat, GrammarRule, LinkType,
    LinksDecoder, LinksEncoder, RuleKind, ToLinks,
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
