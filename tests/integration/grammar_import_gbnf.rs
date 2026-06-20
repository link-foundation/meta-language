use meta_language::{import_gbnf, FromLinks, Grammar, LinksDecoder, LinksEncoder, ToLinks};

#[test]
fn imported_gbnf_grammar_survives_links_round_trip() {
    let text = include_str!("../fixtures/grammar/gbnf/arithmetic.gbnf");
    let grammar = import_gbnf(text).expect("GBNF imports");

    let mut encoder = LinksEncoder::new();
    let root = grammar.to_links(&mut encoder);
    let network = encoder.into_network();
    let mut links_decoder = LinksDecoder::new(&network);
    let restored = Grammar::from_links(&mut links_decoder, root).expect("grammar decodes");

    assert_eq!(restored, grammar);
}
