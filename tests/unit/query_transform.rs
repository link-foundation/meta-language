use meta_language::{
    LinkMetadata, LinkNetwork, LinkQuery, LinkType, ParseConfiguration, ReplacementRule,
    TextReplacement,
};

#[test]
fn jscodeshift_style_identifier_fixture_renames_only_captured_identifiers() {
    let source = "const oldName = call(oldName);\n// oldName in comments stays\n";
    let mut network = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default());
    let query = LinkQuery::from_sexpression(
        r#"
        (identifier) @target
        (#eq? @target "oldName")
        "#,
    )
    .expect("query parses");

    let captures = network.find(&query);

    assert_eq!(captures.len(), 2);

    let report = network.replace(
        &captures,
        &ReplacementRule::captured_text("target", "renamedValue"),
    );
    let output = network.reconstruct_text();

    assert_eq!(report.text_replacements().len(), 2);
    assert_eq!(
        output,
        "const renamedValue = call(renamedValue);\n// oldName in comments stays\n"
    );
    assert_only_reported_ranges_changed(source, &output, report.text_replacements());
}

#[test]
fn recast_style_literal_fixture_changes_only_the_literal_bytes() {
    let source = "const result = 1 + keep; // preserve formatting\n";
    let mut network = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default());
    let query = LinkQuery::from_sexpression(
        r#"
        (number) @literal
        (#eq? @literal "1")
        "#,
    )
    .expect("query parses");

    let captures = network.find(&query);

    assert_eq!(captures.len(), 1);

    let report = network.replace(&captures, &ReplacementRule::captured_text("literal", "2"));
    let output = network.reconstruct_text();

    assert_eq!(report.text_replacements().len(), 1);
    assert_eq!(report.text_replacements()[0].old_text(), "1");
    assert_eq!(output, "const result = 2 + keep; // preserve formatting\n");
    assert_only_reported_ranges_changed(source, &output, report.text_replacements());
}

#[test]
fn replace_can_delegate_structural_rules_to_apply_substitution() {
    let mut network = LinkNetwork::new();
    let one = network.insert_point("1");
    let two = network.insert_point("2");
    let relation = network.insert_link(
        [one, one],
        LinkMetadata::new().with_link_type(LinkType::Relation),
    );
    let query = LinkQuery::by_type(LinkType::Relation);
    let captures = network.find(&query);

    let report = network.replace(
        &captures,
        &ReplacementRule::substitution(meta_language::SubstitutionRule::new(
            [one, one],
            [one, two],
        )),
    );

    assert_eq!(report.substitution().updated(), &[relation]);
    assert_eq!(
        network
            .link(relation)
            .expect("updated relation")
            .references(),
        &[one, two]
    );
}

fn assert_only_reported_ranges_changed(
    source: &str,
    output: &str,
    replacements: &[TextReplacement],
) {
    let mut replacement_ranges = replacements
        .iter()
        .map(|replacement| {
            (
                replacement
                    .span()
                    .expect("text replacement reports a source span")
                    .byte_range(),
                replacement.new_text(),
            )
        })
        .collect::<Vec<_>>();
    replacement_ranges.sort_by_key(|(range, _new_text)| range.start());

    let mut expected = String::new();
    let mut copied_until = 0;
    for (range, new_text) in replacement_ranges {
        assert!(
            copied_until <= range.start(),
            "replacement ranges must not overlap"
        );
        expected.push_str(&source[copied_until..range.start()]);
        expected.push_str(new_text);
        copied_until = range.end();
    }
    expected.push_str(&source[copied_until..]);

    assert_eq!(output, expected);
}
