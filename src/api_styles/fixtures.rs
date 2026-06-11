use crate::api_styles::FluentNetworkApi;
use crate::configuration::ParseConfiguration;
use crate::link_network::{LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::query::LinkQuery;
use crate::substitution::SubstitutionRule;
use crate::transform::ReplacementRule;
use crate::translation_rules::{TranslationRule, TranslationRuleSet};

/// Runs one named API-style parity fixture.
///
/// # Errors
///
/// Returns a message when the fixture name is unknown or the behavior under
/// test does not satisfy the expected contract.
pub fn run_api_style_fixture(name: &str) -> Result<(), String> {
    match name {
        "parse.direct" => fixture_parse_direct(),
        "parse.fluent" => fixture_parse_fluent(),
        "parse.lino_text" => fixture_parse_lino_text(),
        "query.direct" => fixture_query_direct(),
        "query.fluent" => fixture_query_fluent(),
        "query.link_cli_read_identity" => fixture_query_link_cli_read_identity(),
        "query.sexpression" => fixture_query_sexpression(),
        "transform.direct" => fixture_transform_direct(),
        "transform.fluent" => fixture_transform_fluent(),
        "transform.link_cli_update" => fixture_transform_link_cli_update(),
        "transform.sexpression" => fixture_transform_sexpression(),
        "substitute.direct" => fixture_substitute_direct(),
        "substitute.fluent" => fixture_substitute_fluent(),
        "substitute.link_cli_crud" | "substitute.lino_text" => fixture_substitute_link_cli_crud(),
        "serialize.direct" | "serialize.lino_roundtrip" => fixture_serialize_direct(),
        "serialize.fluent" => fixture_serialize_fluent(),
        "snapshot.direct" => fixture_snapshot_direct(),
        "snapshot.fluent" => fixture_snapshot_fluent(),
        "translate.direct" => fixture_translate_direct(),
        "translate.fluent" => fixture_translate_fluent(),
        "translate.lino_rules" => fixture_translate_lino_rules(),
        "verify.direct" => fixture_verify_direct(),
        "verify.fluent" => fixture_verify_fluent(),
        other => Err(format!("unknown API-style fixture `{other}`")),
    }
}

fn fixture_parse_direct() -> Result<(), String> {
    let network = LinkNetwork::parse("alpha", "txt", ParseConfiguration::default());
    ensure(
        network.reconstruct_text() == "alpha",
        "direct parse did not round-trip",
    )
}

fn fixture_parse_fluent() -> Result<(), String> {
    let output =
        LinkNetwork::parse_fluent("alpha", "txt", ParseConfiguration::default()).reconstruct();
    ensure(output == "alpha", "fluent parse did not round-trip")
}

fn fixture_parse_lino_text() -> Result<(), String> {
    let network = LinkNetwork::parse("(one two)\n", "LiNo", ParseConfiguration::default());
    let has_relation = network
        .links()
        .any(|link| link.metadata().link_type() == Some(LinkType::Relation));
    ensure(has_relation, "LiNo text parse did not create a relation")
}

fn fixture_query_direct() -> Result<(), String> {
    let network = query_fixture_network();
    let query = LinkQuery::by_type(LinkType::Concept).with_term("needle");
    ensure(
        network.query_links(&query).len() == 1,
        "direct query did not find the concept",
    )
}

fn fixture_query_fluent() -> Result<(), String> {
    let network = query_fixture_network();
    let query = LinkQuery::by_type(LinkType::Concept).with_term("needle");
    let pipeline = network.into_fluent().find(query);
    ensure(
        pipeline.matches.len() == 1,
        "fluent query did not retain one match",
    )
}

fn fixture_query_link_cli_read_identity() -> Result<(), String> {
    let mut network = link_cli_identity_network();
    let report = network
        .apply_link_cli_substitution_text("((1: 1 1)) ((1: 1 1))")
        .map_err(|error| error.to_string())?;
    ensure(
        report.updated() == [LinkId::from_u64(1)],
        "link-cli read identity did not echo the matched link",
    )
}

fn fixture_query_sexpression() -> Result<(), String> {
    let network = LinkNetwork::parse(
        "const value = 1;\n",
        "JavaScript",
        ParseConfiguration::default(),
    );
    let query =
        LinkQuery::from_sexpression("(identifier) @name").map_err(|error| error.to_string())?;
    ensure(
        !network.find(&query).is_empty(),
        "S-expression query did not match identifiers",
    )
}

fn fixture_transform_direct() -> Result<(), String> {
    let output = direct_transform_output()?;
    ensure(
        output == "const renamed = call(renamed);\n",
        "direct transform output mismatch",
    )
}

fn fixture_transform_fluent() -> Result<(), String> {
    let output = fluent_transform_output()?;
    ensure(
        output == "const renamed = call(renamed);\n",
        "fluent transform output mismatch",
    )
}

fn fixture_transform_link_cli_update() -> Result<(), String> {
    let mut network = link_cli_identity_network();
    let report = network
        .apply_link_cli_substitution_text("((1: 1 1)) ((1: 1 2))")
        .map_err(|error| error.to_string())?;
    ensure(
        report.updated() == [LinkId::from_u64(1)]
            && network.link(LinkId::from_u64(1)).is_some_and(|link| {
                link.references() == [LinkId::from_u64(1), LinkId::from_u64(2)]
            }),
        "link-cli update did not rewrite the matched link",
    )
}

fn fixture_transform_sexpression() -> Result<(), String> {
    fixture_transform_direct()
}

fn fixture_substitute_direct() -> Result<(), String> {
    let mut network = LinkNetwork::new();
    let one = network.insert_point("1");
    let two = network.insert_point("2");
    let relation = network.insert_link(
        [one, one],
        LinkMetadata::new().with_link_type(LinkType::Relation),
    );
    let report = network.apply_substitution(&SubstitutionRule::new([one, one], [one, two]));
    ensure(
        report.updated() == [relation],
        "direct substitution did not update the relation",
    )
}

fn fixture_substitute_fluent() -> Result<(), String> {
    let mut network = LinkNetwork::new();
    let one = network.insert_point("1");
    let two = network.insert_point("2");
    let relation = network.insert_link(
        [one, one],
        LinkMetadata::new().with_link_type(LinkType::Relation),
    );
    let pipeline = network
        .into_fluent()
        .substitute(SubstitutionRule::new([one, one], [one, two]));
    ensure(
        pipeline.last_report().substitution().updated() == [relation],
        "fluent substitution did not update the relation",
    )
}

fn fixture_substitute_link_cli_crud() -> Result<(), String> {
    let mut network = LinkNetwork::new();
    let create = network
        .apply_link_cli_substitution_text("() ((1 1))")
        .map_err(|error| error.to_string())?;
    let relation = create.created()[0];
    let update = network
        .apply_link_cli_substitution_text("((1: 1 1)) ((1: 1 2))")
        .map_err(|error| error.to_string())?;
    let delete = network
        .apply_link_cli_substitution_text("((1 2)) ()")
        .map_err(|error| error.to_string())?;
    ensure(
        relation == LinkId::from_u64(1)
            && update.updated() == [relation]
            && delete.deleted() == [relation],
        "link-cli create/update/delete fixture failed",
    )
}

fn fixture_serialize_direct() -> Result<(), String> {
    let network = LinkNetwork::parse("alpha", "txt", ParseConfiguration::default());
    let lino = network.to_lino();
    let restored = LinkNetwork::from_lino(&lino).map_err(|error| error.to_string())?;
    ensure(
        restored.to_lino() == lino,
        "direct LiNo serialization did not round-trip",
    )
}

fn fixture_serialize_fluent() -> Result<(), String> {
    let network = LinkNetwork::parse("alpha", "txt", ParseConfiguration::default());
    let lino = network.into_fluent().serialize();
    ensure(
        LinkNetwork::from_lino(&lino).is_ok(),
        "fluent serialization did not produce loadable LiNo",
    )
}

fn fixture_snapshot_direct() -> Result<(), String> {
    let network = LinkNetwork::parse("alpha", "txt", ParseConfiguration::default());
    let snapshot = network.snapshot(1, "fixture");
    ensure(
        snapshot.version() == 1 && snapshot.network().reconstruct_text() == "alpha",
        "direct snapshot did not preserve the network",
    )
}

fn fixture_snapshot_fluent() -> Result<(), String> {
    let snapshot = LinkNetwork::parse("alpha", "txt", ParseConfiguration::default())
        .into_fluent()
        .snapshot(1, "fixture");
    ensure(
        snapshot.version() == 1 && snapshot.network().reconstruct_text() == "alpha",
        "fluent snapshot did not preserve the network",
    )
}

fn fixture_translate_direct() -> Result<(), String> {
    let (network, rules) = translation_fixture();
    ensure(
        network.reconstruct_text_as_with_rules("Spanish", ParseConfiguration::default(), &rules)
            == "hola",
        "direct translation fixture failed",
    )
}

fn fixture_translate_fluent() -> Result<(), String> {
    let (network, rules) = translation_fixture();
    ensure(
        network
            .into_fluent()
            .translate("Spanish", ParseConfiguration::default(), &rules)
            == "hola",
        "fluent translation fixture failed",
    )
}

fn fixture_translate_lino_rules() -> Result<(), String> {
    let (_network, rules) = translation_fixture();
    let lino = rules.to_lino();
    let restored = TranslationRuleSet::from_lino(&lino).map_err(|error| error.to_string())?;
    ensure(
        restored == rules,
        "LiNo translation rules did not round-trip",
    )
}

fn fixture_verify_direct() -> Result<(), String> {
    let network = LinkNetwork::parse("alpha", "txt", ParseConfiguration::default());
    ensure(
        network.verify_full_match(None).is_clean(),
        "direct verification reported a clean fixture as invalid",
    )
}

fn fixture_verify_fluent() -> Result<(), String> {
    let report = LinkNetwork::parse("alpha", "txt", ParseConfiguration::default())
        .into_fluent()
        .verify(None);
    ensure(
        report.is_clean(),
        "fluent verification reported a clean fixture as invalid",
    )
}

fn direct_transform_output() -> Result<String, String> {
    let mut network = transform_fixture_network();
    let query = transform_query()?;
    let matches = network.find(&query);
    let report = network.replace(
        &matches,
        &ReplacementRule::captured_text("target", "renamed"),
    );
    ensure(!report.is_empty(), "direct transform made no replacements")?;
    Ok(network.reconstruct_text())
}

fn fluent_transform_output() -> Result<String, String> {
    Ok(transform_fixture_network()
        .into_fluent()
        .find(transform_query()?)
        .replace(ReplacementRule::captured_text("target", "renamed"))
        .reconstruct())
}

fn transform_fixture_network() -> LinkNetwork {
    LinkNetwork::parse(
        "const oldName = call(oldName);\n",
        "JavaScript",
        ParseConfiguration::default(),
    )
}

fn transform_query() -> Result<LinkQuery, String> {
    LinkQuery::from_sexpression(
        r#"
        (identifier) @target
        (#eq? @target "oldName")
        "#,
    )
    .map_err(|error| error.to_string())
}

fn query_fixture_network() -> LinkNetwork {
    let mut network = LinkNetwork::new();
    network.insert_point("needle");
    network
}

fn link_cli_identity_network() -> LinkNetwork {
    let mut network = LinkNetwork::new();
    network.insert_link(
        [LinkId::from_u64(1), LinkId::from_u64(1)],
        LinkMetadata::new().with_link_type(LinkType::Relation),
    );
    network
}

fn translation_fixture() -> (LinkNetwork, TranslationRuleSet) {
    let mut network = LinkNetwork::new();
    let concept = network.insert_concept_expression("greeting", "English", "hello");
    network.insert_link(
        [concept],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_term("proposition:greeting"),
    );
    let rules = TranslationRuleSet::new("greeting").with_rule(
        TranslationRule::new(
            "spanish greeting",
            LinkQuery::by_type(LinkType::Semantic).with_term("proposition:greeting"),
        )
        .with_template("Spanish", "hola"),
    );
    (network, rules)
}

fn ensure(condition: bool, message: &str) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}
