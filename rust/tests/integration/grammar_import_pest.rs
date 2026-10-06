use meta_language::{FromLinks, Grammar, LinksDecoder, LinksEncoder, ToLinks, import_pest};

#[test]
fn imported_pest_grammar_survives_links_round_trip() {
    let text = include_str!("../fixtures/grammar/peg/arithmetic.pest");
    let grammar = import_pest(text).expect("PEG imports");

    let mut encoder = LinksEncoder::new();
    let root = grammar.to_links(&mut encoder);
    let network = encoder.into_network();
    let mut links_decoder = LinksDecoder::new(&network);
    let restored = Grammar::from_links(&mut links_decoder, root).expect("grammar decodes");

    assert_eq!(restored, grammar);
}
