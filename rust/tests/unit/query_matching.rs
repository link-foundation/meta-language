use meta_language::{
    LinkId, LinkMetadata, LinkNetwork, LinkQuery, LinkType, QueryCaptures, QueryPredicate,
    QueryPredicateHost,
};

#[test]
fn by_type_query_selects_links_of_the_requested_link_type() {
    let network = LinkNetwork::self_describing();
    let query = LinkQuery::by_type(LinkType::Type);

    let matches = network.query_links(&query);
    let terms = matches
        .iter()
        .filter_map(|link| link.metadata().term())
        .collect::<Vec<_>>();

    assert_eq!(terms, vec!["type", "Type"]);
}

#[test]
fn s_expression_query_matches_structure_captures_and_host_predicates() {
    let mut network = LinkNetwork::new();
    let root = network.insert_link(
        [],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term("root")
            .with_language("test"),
    );
    let parent = network.insert_link(
        [root],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term("parent")
            .with_language("test"),
    );
    let name = network.insert_link(
        [parent],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term("identifier")
            .with_language("test"),
    );
    network.insert_link(
        [name],
        LinkMetadata::new()
            .with_link_type(LinkType::Token)
            .with_named(true)
            .with_term("main")
            .with_language("test"),
    );
    let argument = network.insert_link(
        [parent],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term("argument")
            .with_language("test"),
    );
    network.insert_field(parent, "name", name);

    let query = LinkQuery::from_sexpression(
        r#"
        (parent
            .
            name: [(identifier) (property_identifier)] @name
            (argument)*
            .
            !decorator) @parent
        (#eq? @name "main")
        "#,
    )
    .expect("query parses");
    let matches = network.query_matches_with(&query, &TokenTextPredicate);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].link_id(), parent);
    assert_eq!(matches[0].captures().first("parent"), Some(parent));
    assert_eq!(matches[0].captures().first("name"), Some(name));
    assert_eq!(
        network.link(argument).expect("argument").references(),
        &[parent]
    );
}

struct TokenTextPredicate;

impl QueryPredicateHost for TokenTextPredicate {
    fn evaluate(
        &self,
        predicate: &QueryPredicate,
        captures: &QueryCaptures,
        network: &LinkNetwork,
    ) -> bool {
        let [capture_argument, literal_argument] = predicate.arguments() else {
            return false;
        };
        predicate.name() == "eq?"
            && capture_argument.capture_name() == Some("name")
            && literal_argument.literal() == token_text(network, captures.first("name"))
    }
}

fn token_text(network: &LinkNetwork, link_id: Option<LinkId>) -> Option<&str> {
    let link_id = link_id?;
    network
        .links()
        .find(|link| {
            link.metadata().link_type() == Some(LinkType::Token)
                && link.references().first().copied() == Some(link_id)
        })
        .and_then(|link| link.metadata().term())
}
