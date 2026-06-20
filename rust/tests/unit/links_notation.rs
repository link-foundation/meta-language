use meta_language::{LinkNetwork, LinkType, ParseConfiguration};

#[test]
fn lino_doublet_triplet_and_tuple_sources_emit_relation_links() {
    let source = "(papa mama)\n(papa loves mama)\n";
    let network = LinkNetwork::parse(source, "LiNo", ParseConfiguration::default());

    assert_eq!(network.reconstruct_text(), source);
    assert!(network.verify_full_match(None).is_clean());

    assert_relation_references(&network, &["papa", "mama"]);
    assert_relation_references(&network, &["papa", "loves", "mama"]);
}

#[test]
fn lino_named_links_are_reused_by_later_references_and_self_references() {
    let source = "(papa (lovesMama: loves mama))\n(son lovesMama)\n(obj_0: list obj_0)\n";
    let network = LinkNetwork::parse(source, "LiNo", ParseConfiguration::default());

    assert_eq!(network.reconstruct_text(), source);
    assert!(network.verify_full_match(None).is_clean());

    assert_relation_references(&network, &["son", "lovesMama"]);
    assert_eq!(
        network
            .link(named_relation(&network, "lovesMama"))
            .expect("lovesMama relation")
            .references(),
        [
            network.find_term("loves").expect("loves term exists"),
            network.find_term("mama").expect("mama term exists")
        ]
    );

    let obj = named_relation(&network, "obj_0");
    let obj_link = network.link(obj).expect("obj_0 relation exists");
    assert!(
        obj_link.references().contains(&obj),
        "named LiNo relation should be able to reference itself"
    );
}

#[test]
fn lino_indented_id_syntax_emits_named_relation() {
    let source = "greeting:\n  hello\n";
    let network = LinkNetwork::parse(source, "LiNo", ParseConfiguration::default());

    assert_eq!(network.reconstruct_text(), source);
    assert!(network.verify_full_match(None).is_clean());

    assert_relation_references(&network, &["hello"]);
    assert_eq!(
        network
            .link(named_relation(&network, "greeting"))
            .expect("greeting relation")
            .metadata()
            .term(),
        Some("greeting")
    );
}

fn named_relation(network: &LinkNetwork, term: &str) -> meta_language::LinkId {
    let link_id = network.find_term(term).expect("named relation term exists");
    assert_eq!(
        network
            .link(link_id)
            .expect("named relation link")
            .metadata()
            .link_type(),
        Some(LinkType::Relation)
    );
    link_id
}

fn assert_relation_references(network: &LinkNetwork, terms: &[&str]) {
    let references = terms
        .iter()
        .map(|term| network.find_term(term).expect("term exists"))
        .collect::<Vec<_>>();

    assert!(
        network.links().any(|link| {
            link.metadata().link_type() == Some(LinkType::Relation)
                && link.references() == references.as_slice()
        }),
        "missing LiNo relation with references {terms:?}"
    );
}
