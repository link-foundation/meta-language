use meta_language::{
    FormalizationLevel, LinkMetadata, LinkNetwork, LinkQuery, LinkType, ParseConfiguration,
    TranslationRule, TranslationRuleRegistry, TranslationRuleSet,
};

#[test]
fn user_supplied_rules_drive_cross_language_reconstruction() {
    let mut network = LinkNetwork::new();
    let proposition = network.insert_concept_expression("capital", "English", "capital");
    let france = network.insert_concept_expression("Q142", "English", "France");
    network.insert_concept_expression("Q142", "Spanish", "Francia");
    let paris = network.insert_concept_expression("Q90", "English", "Paris");
    network.insert_concept_expression("Q90", "Spanish", "Paris");
    network.insert_link(
        [proposition, france, paris],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_term("proposition:capital"),
    );
    let rules = TranslationRuleSet::new("capital-demo").with_rule(
        TranslationRule::new(
            "capital sentence",
            LinkQuery::by_type(LinkType::Semantic).with_term("proposition:capital"),
        )
        .with_reference_capture("subject", 1)
        .with_reference_capture("object", 2)
        .with_template("Spanish", "{object} es la capital de {subject}."),
    );

    assert_eq!(
        network.reconstruct_text_as_with_rules("Spanish", ParseConfiguration::default(), &rules,),
        "Paris es la capital de Francia."
    );
}

#[test]
fn translation_rule_registry_can_replace_the_active_rule_set() {
    let mut network = LinkNetwork::new();
    let concept = network.insert_concept_expression("greeting", "English", "hello");
    network.insert_link(
        [concept],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_term("proposition:greeting"),
    );
    let english = TranslationRuleSet::new("greetings").with_rule(
        TranslationRule::new(
            "english greeting",
            LinkQuery::by_type(LinkType::Semantic).with_term("proposition:greeting"),
        )
        .with_template("English", "hello"),
    );
    let spanish = TranslationRuleSet::new("greetings").with_rule(
        TranslationRule::new(
            "spanish greeting",
            LinkQuery::by_type(LinkType::Semantic).with_term("proposition:greeting"),
        )
        .with_template("Spanish", "hola"),
    );
    let mut registry = TranslationRuleRegistry::new().with_rule_set(english);

    assert_eq!(
        network.reconstruct_text_as_with_registry(
            "English",
            ParseConfiguration::default(),
            &registry,
        ),
        "hello"
    );

    registry.replace_rule_set(spanish);

    assert_eq!(
        network.reconstruct_text_as_with_registry(
            "Spanish",
            ParseConfiguration::default(),
            &registry,
        ),
        "hola"
    );
}

#[test]
fn statehood_demo_is_available_as_a_loadable_rule_set() {
    let rules = TranslationRuleSet::from_lino(TranslationRuleSet::statehood_demo_lino())
        .expect("statehood rule set loads from LiNo");
    let network = LinkNetwork::parse(
        "Hawaii is a state.\n",
        "English",
        ParseConfiguration::default(),
    );

    assert_eq!(
        network.reconstruct_text_as_with_rules("Russian", ParseConfiguration::default(), &rules,),
        "Гавайи это штат.\n"
    );
    assert_eq!(
        network.reconstruct_text_as_with_rules(
            "Russian",
            ParseConfiguration::default().with_formalization_level(FormalizationLevel::Concept),
            &rules,
        ),
        "statehood(Q782, Q35657)\n"
    );
}

#[test]
fn missing_translation_rules_record_diagnostic_links() {
    let mut network = LinkNetwork::new();
    let concept = network.insert_point("unmatched");
    let semantic = network.insert_link(
        [concept],
        LinkMetadata::new()
            .with_link_type(LinkType::Semantic)
            .with_named(true)
            .with_term("proposition:unmatched"),
    );
    let rules = TranslationRuleSet::new("empty");

    assert_eq!(
        network.reconstruct_text_as_with_rules_mut(
            "Spanish",
            ParseConfiguration::default(),
            &rules,
        ),
        ""
    );

    let diagnostic = network
        .links()
        .find(|link| {
            link.metadata().link_type() == Some(LinkType::Semantic)
                && link.metadata().term() == Some("translation-rule:missing")
        })
        .expect("missing translation produces a diagnostic link");
    assert_eq!(diagnostic.references(), &[semantic]);
    assert_eq!(diagnostic.metadata().language(), Some("Spanish"));
    assert!(diagnostic
        .metadata()
        .definition()
        .expect("diagnostic names the unmatched structure")
        .contains("proposition:unmatched"));
}

#[test]
fn translation_rule_sets_round_trip_through_lino() {
    let rules = TranslationRuleSet::new("capital-demo").with_rule(
        TranslationRule::new(
            "capital sentence",
            LinkQuery::by_type(LinkType::Semantic)
                .with_term("proposition:capital")
                .with_named(true),
        )
        .with_reference_capture("subject", 1)
        .with_reference_capture("object", 2)
        .with_template("English", "{object} is the capital of {subject}.")
        .with_template("Spanish", "{object} es la capital de {subject}."),
    );

    let lino = rules.to_lino();
    let restored =
        TranslationRuleSet::from_lino(&lino).expect("serialized rule set must load from LiNo");

    assert_eq!(restored, rules);
    assert_eq!(restored.to_lino(), lino);
}

#[test]
fn translation_rules_recursively_render_captured_child_links() {
    let mut network = LinkNetwork::new();
    let left = insert_shell_command(&mut network, "cd /tmp");
    let right = insert_shell_command(&mut network, "ls -la");
    let root = insert_shell_syntax(&mut network, "and", [left, right]);
    let rules = TranslationRuleSet::new("shell-to-js")
        .with_rule(
            TranslationRule::new("and", shell_syntax_query("and"))
                .with_reference_capture("left", 0)
                .with_reference_capture("right", 1)
                .with_template("JavaScript", concat!("{", "left}\n{", "right}")),
        )
        .with_rule(shell_command_rule());

    assert_eq!(
        rules.render_link(&network, root, "JavaScript", ParseConfiguration::default()),
        "await $`cd /tmp`;\nawait $`ls -la`;"
    );
}

#[test]
fn different_rules_compose_across_sibling_rendering_roots() {
    let mut network = LinkNetwork::new();
    let comment_text = insert_shell_token(&mut network, "build step");
    insert_shell_syntax(&mut network, "comment", [comment_text]);
    insert_shell_command(&mut network, "make all");
    let rules = TranslationRuleSet::new("shell-to-js")
        .with_rule(
            TranslationRule::new("comment", shell_syntax_query("comment"))
                .with_reference_capture("body", 0)
                .with_template("JavaScript", "// {body}"),
        )
        .with_rule(shell_command_rule());

    assert_eq!(
        network.reconstruct_text_as_with_rules("JavaScript", ParseConfiguration::default(), &rules),
        "// build step\nawait $`make all`;"
    );
}

#[test]
fn variadic_placeholders_recursively_render_and_join_child_references() {
    let mut network = LinkNetwork::new();
    let first = insert_shell_command(&mut network, "ls");
    let second = insert_shell_command(&mut network, "grep test");
    let third = insert_shell_command(&mut network, "wc -l");
    let root = insert_shell_syntax(&mut network, "pipeline", [first, second, third]);
    let rules = TranslationRuleSet::new("shell-to-js")
        .with_rule(
            TranslationRule::new("pipeline", shell_syntax_query("pipeline"))
                .with_template("JavaScript", "await $`{*.:command| | }`;")
                .with_template("JavaScript:command", "{*.:command| | }"),
        )
        .with_rule(
            TranslationRule::new("command", shell_syntax_query("command"))
                .with_reference_capture("body", 0)
                .with_template("JavaScript:command", "{body:text}"),
        );

    assert_eq!(
        rules.render_link(&network, root, "JavaScript", ParseConfiguration::default()),
        "await $`ls | grep test | wc -l`;"
    );
}

#[test]
fn optional_segments_and_unresolved_captures_render_as_empty() {
    let mut network = LinkNetwork::new();
    let without_value = insert_shell_syntax(&mut network, "return", []);
    let value = insert_shell_token(&mut network, "result");
    let with_value = insert_shell_syntax(&mut network, "return", [value]);
    let rules = TranslationRuleSet::new("shell-to-js").with_rule(
        TranslationRule::new("return", shell_syntax_query("return"))
            .with_reference_capture("value", 0)
            .with_template(
                "JavaScript",
                "return{?value} {value:text}{/value};{missing}",
            ),
    );

    assert_eq!(
        rules.render_link(
            &network,
            without_value,
            "JavaScript",
            ParseConfiguration::default()
        ),
        "return;"
    );
    assert_eq!(
        rules.render_link(
            &network,
            with_value,
            "JavaScript",
            ParseConfiguration::default()
        ),
        "return result;"
    );
}

#[test]
fn placeholder_contexts_use_serializable_target_language_fallbacks() {
    let mut network = LinkNetwork::new();
    let text = insert_shell_token(&mut network, "hello");
    let word = insert_shell_syntax(&mut network, "word", [text]);
    let root = insert_shell_syntax(&mut network, "assignment", [word]);
    let rules = TranslationRuleSet::new("shell-to-js")
        .with_rule(
            TranslationRule::new("assignment", shell_syntax_query("assignment"))
                .with_reference_capture("value", 0)
                .with_template(
                    "JavaScript",
                    concat!("const value = `", "{", "value:value}", "`;"),
                ),
        )
        .with_rule(
            TranslationRule::new("word", shell_syntax_query("word"))
                .with_reference_capture("body", 0)
                .with_template("JavaScript:command", "{body:text}"),
        )
        .with_language_fallback("JavaScript:value", "JavaScript:command");

    assert_eq!(
        rules.render_link(&network, root, "JavaScript", ParseConfiguration::default()),
        "const value = `hello`;"
    );
    let restored = TranslationRuleSet::from_lino(&rules.to_lino())
        .expect("language fallback rule set must load from LiNo");
    assert_eq!(restored, rules);
}

#[test]
fn multiline_substitutions_inherit_the_placeholder_indentation() {
    let mut network = LinkNetwork::new();
    let first = insert_shell_command(&mut network, "first");
    let second = insert_shell_command(&mut network, "second");
    let body = insert_shell_syntax(&mut network, "block", [first, second]);
    let root = insert_shell_syntax(&mut network, "if", [body]);
    let rules = TranslationRuleSet::new("shell-to-js")
        .with_rule(
            TranslationRule::new("if", shell_syntax_query("if"))
                .with_reference_capture("body", 0)
                .with_template("JavaScript", "if (ready) {\n  {body}\n}"),
        )
        .with_rule(
            TranslationRule::new("block", shell_syntax_query("block"))
                .with_template("JavaScript", "{*.|\\n}"),
        )
        .with_rule(shell_command_rule());

    assert_eq!(
        rules.render_link(&network, root, "JavaScript", ParseConfiguration::default()),
        "if (ready) {\n  await $`first`;\n  await $`second`;\n}"
    );
}

fn shell_syntax_query(term: &str) -> LinkQuery {
    LinkQuery::by_type(LinkType::Syntax)
        .with_language("Shell")
        .with_term(term)
}

fn insert_shell_token(network: &mut LinkNetwork, text: &str) -> meta_language::LinkId {
    network.insert_link(
        [],
        LinkMetadata::new()
            .with_link_type(LinkType::Token)
            .with_language("Shell")
            .with_term(text),
    )
}

fn insert_shell_syntax<const N: usize>(
    network: &mut LinkNetwork,
    term: &str,
    children: [meta_language::LinkId; N],
) -> meta_language::LinkId {
    network.insert_link(
        children,
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_language("Shell")
            .with_named(true)
            .with_term(term),
    )
}

fn insert_shell_command(network: &mut LinkNetwork, text: &str) -> meta_language::LinkId {
    let token = insert_shell_token(network, text);
    insert_shell_syntax(network, "command", [token])
}

fn shell_command_rule() -> TranslationRule {
    TranslationRule::new("command", shell_syntax_query("command"))
        .with_reference_capture("body", 0)
        .with_template("JavaScript", "await $`{body}`;")
}
