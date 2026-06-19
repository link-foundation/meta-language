use meta_language::{import_lark, FromLinks, Grammar, LinksDecoder, LinksEncoder, ToLinks};

#[test]
fn imported_lark_grammar_survives_links_round_trip() {
    let text = include_str!("../fixtures/grammar/lark/covering.lark");
    let grammar = import_lark(text).expect("Lark imports");

    let mut encoder = LinksEncoder::new();
    let root = grammar.to_links(&mut encoder);
    let network = encoder.into_network();
    let mut links_decoder = LinksDecoder::new(&network);
    let restored = Grammar::from_links(&mut links_decoder, root).expect("grammar decodes");

    assert_eq!(restored, grammar);
}
