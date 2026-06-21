use meta_language::{import_bnf, FromLinks, Grammar, LinksDecoder, LinksEncoder, ToLinks};

const ARITHMETIC: &str = include_str!("../fixtures/grammar/bnf/arithmetic.bnf");

#[test]
fn imported_bnf_grammar_survives_links_round_trip() {
    let grammar = import_bnf(ARITHMETIC).expect("BNF imports");

    let mut encoder = LinksEncoder::new();
    let root = grammar.to_links(&mut encoder);
    let network = encoder.into_network();
    let mut links_decoder = LinksDecoder::new(&network);
    let restored = Grammar::from_links(&mut links_decoder, root).expect("grammar decodes");

    assert_eq!(restored, grammar);
}
