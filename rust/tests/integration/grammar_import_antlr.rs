use meta_language::{FromLinks, Grammar, LinksDecoder, LinksEncoder, ToLinks, import_antlr};

#[test]
fn imported_antlr_grammar_survives_links_round_trip() {
    let grammar =
        import_antlr(include_str!("../fixtures/grammar/antlr/covering.g4")).expect("imports");

    let mut encoder = LinksEncoder::new();
    let root = grammar.to_links(&mut encoder);
    let network = encoder.into_network();
    let mut links_decoder = LinksDecoder::new(&network);
    let restored = Grammar::from_links(&mut links_decoder, root).expect("grammar decodes");

    assert_eq!(restored, grammar);
}
