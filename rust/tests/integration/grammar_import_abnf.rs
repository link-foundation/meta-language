use meta_language::{import_abnf, FromLinks, Grammar, LinksDecoder, LinksEncoder, ToLinks};

const POSTAL_ADDRESS: &str = include_str!("../fixtures/grammar/abnf/postal-address.abnf");

#[test]
fn imported_abnf_grammar_survives_links_round_trip() {
    let grammar = import_abnf(POSTAL_ADDRESS).expect("ABNF imports");

    let mut encoder = LinksEncoder::new();
    let root = grammar.to_links(&mut encoder);
    let network = encoder.into_network();
    let mut links_decoder = LinksDecoder::new(&network);
    let restored = Grammar::from_links(&mut links_decoder, root).expect("grammar decodes");

    assert_eq!(restored, grammar);
}
