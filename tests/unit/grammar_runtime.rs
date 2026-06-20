use meta_language::{
    register_grammar, with_grammar, CharClassItem, Grammar, GrammarParser, LanguageParser,
    LinkNetwork, LinkType, MembershipOracle, ParseConfiguration, ParserRegistry,
};

#[test]
fn accepts_and_parse_source_cover_expression_algebra() {
    let parser = GrammarParser::new(expression_algebra_grammar());
    let input = "AB5yQaabbbccc!";

    assert!(parser.accepts(input));
    assert!(!parser.accepts("AB5yQaabbbccc!?"));
    assert!(oracle_accepts(&parser, input));

    let network = parser.parse_source(input, "runtime-test", ParseConfiguration::default());

    assert_eq!(network.reconstruct_text(), input);
    assert!(has_grammar_term(&network, "grammar::runtime::rule::body"));
    assert!(has_grammar_term(
        &network,
        "grammar::runtime::capture::bang"
    ));
    assert!(network.links().any(|link| {
        link.metadata().link_type() == Some(LinkType::Token) && link.metadata().term() == Some("!")
    }));
}

#[test]
fn ordered_choice_is_peg_style_and_unordered_choice_uses_longest_match() {
    let expr = Grammar::expr();
    let ordered = Grammar::builder()
        .start("start")
        .rule(
            "start",
            expr.seq([
                expr.choice_ordered([expr.term("a"), expr.term("ab")]),
                expr.term("b"),
            ]),
        )
        .build();
    let unordered = Grammar::builder()
        .start("start")
        .rule(
            "start",
            expr.seq([
                expr.choice_unordered([expr.term("a"), expr.term("ab")]),
                expr.term("b"),
            ]),
        )
        .build();

    assert!(GrammarParser::new(ordered).accepts("ab"));
    assert!(
        !GrammarParser::new(unordered).accepts("ab"),
        "unordered choice is determinised by the longest local alternative"
    );
}

#[test]
fn parse_source_falls_back_losslessly_on_rejected_or_malformed_input() {
    let parser = GrammarParser::new(literal_grammar("ok"));
    let configuration = ParseConfiguration::default();
    let fallback = LinkNetwork::parse_lossless_text("nope", "runtime-test", configuration);

    assert_eq!(
        parser.parse_source("nope", "runtime-test", configuration),
        fallback
    );

    let expr = Grammar::expr();
    let left_recursive = Grammar::builder()
        .start("start")
        .rule("start", expr.seq([expr.nt("start"), expr.term("a")]))
        .build();
    let left_recursive_parser = GrammarParser::new(left_recursive);
    assert!(!left_recursive_parser.accepts("a"));
    assert_eq!(
        left_recursive_parser
            .parse_source("a", "runtime-left-recursive", configuration)
            .reconstruct_text(),
        "a"
    );

    let missing_rule = Grammar::builder()
        .start("start")
        .rule("start", expr.nt("missing"))
        .build();
    let missing_rule_parser = GrammarParser::new(missing_rule);
    assert!(!missing_rule_parser.accepts(""));
    assert_eq!(
        missing_rule_parser
            .parse_source("", "runtime-missing-rule", configuration)
            .reconstruct_text(),
        ""
    );
}

#[test]
fn registry_helpers_route_registered_grammars_through_existing_dispatch() {
    let grammar = literal_grammar("ok");
    let configuration = ParseConfiguration::default();
    let mut registry = ParserRegistry::new();
    register_grammar(&mut registry, "runtime-key", grammar.clone());

    assert!(registry.is_registered("runtime-key"));
    let network = LinkNetwork::parse_with_registry(&registry, "ok", "runtime-key", configuration);
    assert_eq!(network.reconstruct_text(), "ok");
    assert!(has_grammar_term(&network, "grammar::runtime::rule::start"));

    let shadowing_registry = with_grammar(ParserRegistry::new(), "plain-text", grammar);
    let shadowed = shadowing_registry.parse("ok", "plain-text", configuration);
    assert!(has_grammar_term(&shadowed, "grammar::runtime::rule::start"));

    let other = shadowing_registry.parse("(x)", "lino", configuration);
    assert_eq!(other, LinkNetwork::parse("(x)", "lino", configuration));
}

fn expression_algebra_grammar() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start("start")
        .rule("start", expr.nt("body"))
        .rule(
            "body",
            expr.seq([
                expr.empty(),
                expr.terminal_insensitive("ab"),
                expr.char_range('0', '9'),
                expr.char_class(
                    false,
                    [CharClassItem::char('x'), CharClassItem::range('y', 'z')],
                ),
                expr.any(),
                expr.opt(expr.term("?")),
                expr.rep0(expr.term("a")),
                expr.rep1(expr.term("b")),
                expr.repeat(expr.term("c"), 2, Some(3)),
                expr.and(expr.term("!")),
                expr.not(expr.term("?")),
                expr.capture(Some("bang"), expr.term("!")),
            ]),
        )
        .build()
}

fn literal_grammar(literal: &str) -> Grammar {
    Grammar::builder()
        .start("start")
        .rule("start", Grammar::expr().term(literal))
        .build()
}

fn has_grammar_term(network: &LinkNetwork, expected: &str) -> bool {
    network.links().any(|link| {
        link.metadata().link_type() == Some(LinkType::Grammar)
            && link.metadata().term() == Some(expected)
    })
}

fn oracle_accepts(oracle: &dyn MembershipOracle, text: &str) -> bool {
    oracle.accepts(text)
}
