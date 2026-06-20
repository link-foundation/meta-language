use meta_language::{
    LinkId, LinkMetadata, LinkNetwork, LinkQuery, LinkRule, LinkRuleRegistry, LinkRuleSnapshotCase,
    LinkRuleSnapshotExpectation, LinkRuleSnapshotSuite, LinkType, ParseConfiguration,
    QuasiquoteTemplate, ReplacementRule, TraversalStrategy,
};

#[test]
fn ast_grep_style_rule_algebra_supports_relations_booleans_and_named_refs() {
    let mut network = LinkNetwork::new();
    let root = syntax(&mut network, [], "root");
    let block = syntax(&mut network, [root], "block");
    let first_identifier = syntax(&mut network, [block], "identifier");
    let number = syntax(&mut network, [block], "number");
    let second_identifier = syntax(&mut network, [block], "identifier");

    let mut registry = LinkRuleRegistry::new();
    registry.insert(
        "container",
        LinkRule::from_sexpression("(kind block)").unwrap(),
    );
    let rule = LinkRule::from_sexpression(
        r"
        (all
          (meta target identifier)
          (inside (kind identifier) (ref container))
          (precedes (kind identifier) (kind number))
          (not (follows (kind identifier) (kind number))))
        ",
    )
    .expect("rule parses");

    let matches = rule.matches(&network, &registry);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].link_id(), first_identifier);
    assert_eq!(
        matches[0].captures().first("target"),
        Some(first_identifier)
    );
    assert!(matches
        .iter()
        .all(|rule_match| rule_match.link_id() != second_identifier));
    assert!(matches
        .iter()
        .all(|rule_match| rule_match.link_id() != number));
}

#[test]
fn semgrep_coccinelle_style_ellipsis_and_typed_metavariables_match_gaps() {
    let mut network = LinkNetwork::new();
    let root = syntax(&mut network, [], "root");
    let call = syntax(&mut network, [root], "call_expression");
    let first = syntax(&mut network, [call], "identifier");
    let gap = syntax(&mut network, [call], "comment");
    let second = syntax(&mut network, [call], "identifier");

    let rule = LinkRule::from_sexpression(
        r"
        (all
          (kind call_expression)
          (ellipsis (meta first identifier) (meta second identifier)))
        ",
    )
    .expect("rule parses");

    let matches = rule.matches(&network, &LinkRuleRegistry::new());

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].link_id(), call);
    assert_eq!(matches[0].captures().first("first"), Some(first));
    assert_eq!(matches[0].captures().first("second"), Some(second));
    assert_ne!(matches[0].captures().first("first"), Some(gap));
}

#[test]
fn comby_style_text_pattern_matches_plain_text_fallback_tokens() {
    let network = LinkNetwork::parse(
        "alpha beta gamma",
        "UnwiredPlainText",
        ParseConfiguration::default(),
    );
    let rule = LinkRule::from_sexpression(r#"(text "alpha {{gap}} gamma")"#).unwrap();

    let matches = rule.matches(&network, &LinkRuleRegistry::new());

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].captures().text("gap"), Some("beta"));
}

#[test]
fn babel_recast_style_quasiquote_replacement_checks_placeholders_and_parentheses() {
    let source = "const result = (oldValue);\n";
    let mut network = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default());
    let query = LinkQuery::from_sexpression(
        r"
        (parenthesized_expression
          (identifier) @arg) @target
        ",
    )
    .expect("query parses");
    let matches = network.find(&query);
    assert_eq!(matches.len(), 1);

    let missing = QuasiquoteTemplate::parse("{{missing}} + 1").unwrap();
    let missing_report = network.replace(&matches, &ReplacementRule::quasiquote("target", missing));
    assert_eq!(missing_report.template_errors().len(), 1);
    assert_eq!(network.reconstruct_text(), source);

    let template = QuasiquoteTemplate::parse("{{arg}} + 1").unwrap();
    let report = network.replace(&matches, &ReplacementRule::quasiquote("target", template));

    assert_eq!(report.text_replacements().len(), 1);
    assert!(report.template_errors().is_empty());
    assert_eq!(
        network.reconstruct_text(),
        "const result = (oldValue + 1);\n"
    );
}

#[test]
fn stratego_rascal_style_traversal_orders_and_fixpoint_are_available() {
    let mut network = LinkNetwork::new();
    let root = syntax(&mut network, [], "root");
    let outer = syntax(&mut network, [root], "call_expression");
    let inner = syntax(&mut network, [outer], "call_expression");
    let rule = LinkRule::from_sexpression("(kind call_expression)").unwrap();
    let registry = LinkRuleRegistry::new();

    let topdown = TraversalStrategy::TopDown.matches(&network, &rule, &registry);
    let bottomup = TraversalStrategy::BottomUp.matches(&network, &rule, &registry);
    let innermost = TraversalStrategy::Innermost.matches(&network, &rule, &registry);

    assert_eq!(ids(&topdown), vec![outer, inner]);
    assert_eq!(ids(&bottomup), vec![inner, outer]);
    assert_eq!(ids(&innermost), vec![inner]);

    let report = TraversalStrategy::Fixpoint { max_iterations: 4 }.apply_mut(
        &mut network,
        &rule,
        &registry,
        |network, _rule_match| {
            if network.find_term("fixpoint:done").is_none() {
                network.insert_point("fixpoint:done");
                true
            } else {
                false
            }
        },
    );

    assert_eq!(report.iterations(), 2);
    assert_eq!(report.changed(), 1);
}

#[test]
fn ast_grep_style_rule_snapshot_harness_verifies_valid_and_invalid_cases() {
    let suite = LinkRuleSnapshotSuite::new(
        LinkRule::from_sexpression(r#"(text "replace {{name}}")"#).unwrap(),
    )
    .with_case(LinkRuleSnapshotCase::new(
        "valid replacement target",
        "replace me",
        "UnwiredPlainText",
        LinkRuleSnapshotExpectation::Valid,
    ))
    .with_case(LinkRuleSnapshotCase::new(
        "invalid replacement target",
        "ignore me",
        "UnwiredPlainText",
        LinkRuleSnapshotExpectation::Invalid,
    ));

    let report = suite.run(&LinkRuleRegistry::new(), ParseConfiguration::default());

    assert!(report.is_success(), "{report:?}");
    assert_eq!(report.cases().len(), 2);
}

fn syntax<const N: usize>(
    network: &mut LinkNetwork,
    references: [LinkId; N],
    kind: &str,
) -> LinkId {
    network.insert_link(
        references,
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term(kind),
    )
}

fn ids(matches: &[meta_language::LinkRuleMatch]) -> Vec<LinkId> {
    matches
        .iter()
        .map(meta_language::LinkRuleMatch::link_id)
        .collect()
}
