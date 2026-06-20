use meta_language::{
    import_bnf, register_grammar, LinkNetwork, LinkType, ParseConfiguration, ParserRegistry,
};

const ARITHMETIC: &str = include_str!("../fixtures/grammar/bnf/arithmetic.bnf");
const LIST: &str = include_str!("../fixtures/grammar/bnf/list.bnf");

#[test]
fn imported_bnf_grammar_registers_and_parses_new_input_losslessly() {
    let grammar = import_bnf(ARITHMETIC).expect("arithmetic BNF imports");
    let mut registry = ParserRegistry::new();
    register_grammar(&mut registry, "arith-bnf", grammar);

    let network = LinkNetwork::parse_with_registry(
        &registry,
        "1+2*3",
        "arith-bnf",
        ParseConfiguration::default(),
    );

    assert_eq!(network.reconstruct_text(), "1+2*3");
    assert!(has_grammar_term(&network, "grammar::runtime::rule::expr"));
    assert!(has_grammar_term(&network, "grammar::runtime::rule::digit"));
}

#[test]
fn imported_recursive_bnf_list_accepts_empty_and_nested_items() {
    let grammar = import_bnf(LIST).expect("list BNF imports");
    let mut registry = ParserRegistry::new();
    register_grammar(&mut registry, "list-bnf", grammar);

    for sample in ["", "a", "b,a,b"] {
        let network = LinkNetwork::parse_with_registry(
            &registry,
            sample,
            "list-bnf",
            ParseConfiguration::default(),
        );

        assert_eq!(network.reconstruct_text(), sample);
        assert!(has_grammar_term(&network, "grammar::runtime::rule::list"));
    }
}

fn has_grammar_term(network: &LinkNetwork, expected: &str) -> bool {
    network.links().any(|link| {
        link.metadata().link_type() == Some(LinkType::Grammar)
            && link.metadata().term() == Some(expected)
    })
}
