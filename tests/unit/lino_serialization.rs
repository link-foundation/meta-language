use std::collections::BTreeSet;

use links_notation::parse_lino_to_links;
use meta_language::{
    Link, LinkNetwork, LinoSerializationError, ParseConfiguration, LANGUAGE_FIXTURES,
};

/// Asserts that two networks carry the same links and term registrations.
fn assert_isomorphic(original: &LinkNetwork, restored: &LinkNetwork) {
    let original_links: Vec<&Link> = original.links().collect();
    let restored_links: Vec<&Link> = restored.links().collect();
    assert_eq!(
        original_links, restored_links,
        "round-trip changed the link set"
    );

    let mut terms: BTreeSet<&str> = BTreeSet::new();
    for link in original.links() {
        if let Some(term) = link.metadata().term() {
            terms.insert(term);
        }
    }
    for term in terms {
        assert_eq!(
            original.find_term(term),
            restored.find_term(term),
            "round-trip changed term registration for {term:?}"
        );
    }
}

/// Round-trips a network through `to_lino`/`from_lino` and checks isomorphism,
/// crate-parser acceptance, and serializer idempotence.
fn assert_round_trips(network: &LinkNetwork) {
    let lino = network.to_lino();

    let parsed = parse_lino_to_links(&lino)
        .expect("to_lino output must be accepted by the links-notation crate parser");
    assert_eq!(
        parsed.len(),
        network.links().count(),
        "crate parser must see one statement per link"
    );

    let restored = LinkNetwork::from_lino(&lino).expect("from_lino must reconstruct the network");
    assert_isomorphic(network, &restored);

    assert_eq!(
        restored.to_lino(),
        lino,
        "re-serializing the reconstruction must be byte-for-byte stable"
    );
}

#[test]
fn to_lino_from_lino_round_trips_every_language_fixture() {
    for fixture in LANGUAGE_FIXTURES {
        let network = LinkNetwork::parse(
            fixture.source(),
            fixture.language(),
            ParseConfiguration::default(),
        );
        assert!(
            network.links().count() > 0,
            "{} fixture produced an empty network",
            fixture.language()
        );
        assert_round_trips(&network);
    }
}

#[test]
fn to_lino_from_lino_round_trips_synthetic_networks() {
    // Empty network.
    assert_round_trips(&LinkNetwork::new());

    // Self-description roots exercise types, terms, definitions, and languages.
    assert_round_trips(&LinkNetwork::self_describing());

    // Hand-authored links-notation relations exercise named relation links.
    assert_round_trips(&LinkNetwork::parse(
        "(papa loves mama)\n(son lovesMama)\n",
        "LiNo",
        ParseConfiguration::default(),
    ));

    // Error recovery exercises error/has-error/missing parse flags.
    assert_round_trips(&LinkNetwork::parse(
        "abcd)",
        "LibCST",
        ParseConfiguration::default(),
    ));

    // Source code exercises tokens, trivia, spans, and language labels.
    assert_round_trips(&LinkNetwork::parse(
        "const a = 1; // c\n",
        "JavaScript",
        ParseConfiguration::default(),
    ));
}

#[test]
fn to_lino_output_is_accepted_by_the_links_notation_crate_parser() {
    let network = LinkNetwork::self_describing();
    let lino = network.to_lino();

    let parsed = parse_lino_to_links(&lino).expect("links-notation crate parses to_lino output");
    assert_eq!(parsed.len(), network.links().count());
    assert!(
        parsed.iter().all(links_notation::LiNo::is_link),
        "every serialized statement is an identified link"
    );
}

#[test]
fn empty_input_reconstructs_an_empty_network() {
    let network = LinkNetwork::from_lino("").expect("empty input is a valid empty network");
    assert_eq!(network.links().count(), 0);
    assert_eq!(network, LinkNetwork::new());
}

#[test]
fn malformed_serialization_text_is_reported_as_an_error() {
    let error = LinkNetwork::from_lino("(7: (meta: (t: nonsense) (n: 1)))")
        .expect_err("unknown link type must fail");
    assert!(matches!(error, LinoSerializationError::Structure(_)));
}
