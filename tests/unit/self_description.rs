use std::collections::BTreeSet;

use meta_language::{LinkNetwork, LinkType};

#[test]
fn self_description_contains_common_roots() {
    let network = LinkNetwork::self_describing();
    let root_terms = self_description_root_terms();
    let root_term_set = root_terms.iter().copied().collect::<BTreeSet<_>>();

    for term in root_terms {
        let id = network.find_term(term).expect("root term exists");
        let link = network.link(id).expect("root link exists");
        assert_eq!(link.references(), &[id], "{term} should be a point link");

        let definition = network.definition_for(id).expect("root definition");
        let referenced_terms = referenced_root_terms(definition, &root_term_set);
        assert!(
            !referenced_terms.is_empty(),
            "{term} definition should reference root terms"
        );
        assert_controlled_definition(definition, &root_term_set);
        for referenced_term in referenced_terms {
            assert!(
                network.find_term(&referenced_term).is_some(),
                "{term} definition references undefined term: {referenced_term}"
            );
        }
    }

    let point = network.find_term("point").expect("point term");
    assert_eq!(network.definition_for(point), Some("(point: point point)"));

    let rml_type = network.find_term("Type").expect("Type term");
    assert_eq!(network.definition_for(rml_type), Some("(Type: Type Type)"));
}

#[test]
fn self_description_definition_links_reference_only_roots() {
    let network = LinkNetwork::self_describing();
    let root_terms = self_description_root_terms();
    let root_ids = root_terms
        .iter()
        .map(|term| network.find_term(term).expect("root term exists"))
        .collect::<BTreeSet<_>>();

    for term in root_terms {
        let root = network.find_term(term).expect("root term exists");
        let expected_references = definition_link_references(&network, term);
        let definition_link = network
            .links()
            .find(|link| {
                link.metadata().link_type() == Some(LinkType::Relation)
                    && link.references() == expected_references.as_slice()
            })
            .unwrap_or_else(|| panic!("missing structural definition link for {term}"));

        assert_eq!(definition_link.references()[0], root);
        assert!(definition_link
            .references()
            .iter()
            .all(|reference| root_ids.contains(reference)));
    }
}

#[test]
fn self_description_term_labels_are_only_roots() {
    let network = LinkNetwork::self_describing();
    let root_terms = self_description_root_terms()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();

    for term in network.links().filter_map(|link| link.metadata().term()) {
        assert!(
            root_terms.contains(term),
            "self-description term label should be a seeded root: {term}"
        );
    }
}

const fn self_description_root_terms() -> &'static [&'static str] {
    &[
        "link",
        "reference",
        "relation link",
        "language",
        "grammar",
        "type",
        "Type",
        "concept",
        "point",
        "field",
        "trivia",
        "region",
        "object",
    ]
}

fn referenced_root_terms(definition: &str, root_terms: &BTreeSet<&str>) -> Vec<String> {
    let (_remaining, references) = remove_controlled_terms(definition, root_terms);
    references
}

fn assert_controlled_definition(definition: &str, root_terms: &BTreeSet<&str>) {
    let (remaining, _references) = remove_controlled_terms(definition, root_terms);
    let external_text = remaining
        .chars()
        .filter(|character| !matches!(character, '(' | ')' | ':' | ' ' | '\t'))
        .collect::<String>();

    assert!(
        external_text.is_empty(),
        "definition contains vocabulary outside seeded roots: {definition}"
    );
}

fn remove_controlled_terms(definition: &str, root_terms: &BTreeSet<&str>) -> (String, Vec<String>) {
    let mut remaining = definition.to_string();
    let mut references = Vec::new();

    for term in root_terms_by_length(root_terms) {
        while let Some(start) = remaining.find(term) {
            references.push(term.to_string());
            remaining.replace_range(start..start + term.len(), &" ".repeat(term.len()));
        }
    }

    (remaining, references)
}

fn root_terms_by_length<'a>(root_terms: &'a BTreeSet<&'a str>) -> Vec<&'a str> {
    let mut terms = root_terms.iter().copied().collect::<Vec<_>>();
    terms.sort_by(|left, right| right.len().cmp(&left.len()).then_with(|| left.cmp(right)));
    terms
}

fn definition_link_references(network: &LinkNetwork, term: &str) -> Vec<meta_language::LinkId> {
    let mut references = Vec::new();
    references.push(network.find_term(term).expect("root term exists"));
    references.extend(expected_definition_terms(term).iter().map(|reference| {
        network
            .find_term(reference)
            .expect("referenced root exists")
    }));
    references
}

fn expected_definition_terms(term: &str) -> &'static [&'static str] {
    match term {
        "link" => &["reference", "reference"],
        "reference" => &["link", "link"],
        "relation link" | "object" => &["link", "reference"],
        "language" => &["grammar", "concept"],
        "grammar" => &["language", "relation link"],
        "type" => &["Type", "concept"],
        "Type" => &["Type", "Type"],
        "concept" => &["link", "language"],
        "point" => &["point", "point"],
        "field" => &["relation link", "reference"],
        "trivia" => &["region", "link"],
        "region" => &["language", "link"],
        unknown => panic!("unexpected self-description root term: {unknown}"),
    }
}
