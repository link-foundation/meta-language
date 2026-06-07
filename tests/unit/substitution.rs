use meta_language::{
    LinkMetadata, LinkNetwork, LinkType, SubstitutionValue, VariableSubstitutionRule,
};

#[test]
fn link_cli_style_substitution_binds_index_source_and_target_variables() {
    let mut network = LinkNetwork::new();
    let one = network.insert_point("1");
    let two = network.insert_point("2");
    let relation = network.insert_link(
        [one, two],
        LinkMetadata::new().with_link_type(LinkType::Relation),
    );
    let rule = VariableSubstitutionRule::new(
        [
            SubstitutionValue::variable("$source"),
            SubstitutionValue::variable("$target"),
        ],
        [
            SubstitutionValue::variable("$target"),
            SubstitutionValue::variable("$source"),
        ],
    )
    .with_index_variable("$index");

    let report = network.apply_variable_substitution(&rule);

    assert_eq!(report.updated(), &[relation]);
    assert_eq!(
        network.link(relation).expect("swapped link").references(),
        &[two, one]
    );
    let bindings = &report.bindings()[0];
    assert_eq!(bindings.get("index"), Some(relation));
    assert_eq!(bindings.get("source"), Some(one));
    assert_eq!(bindings.get("target"), Some(two));
}
