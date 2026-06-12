use meta_language::{LinkNetwork, ParseConfiguration};

#[test]
fn render_source_emits_programmatically_constructed_syntax_without_spans() {
    let mut network = LinkNetwork::new();
    let first_tokens = [
        network.insert_source_token("JavaScript", "const"),
        network.insert_source_token("JavaScript", " "),
        network.insert_source_token("JavaScript", "values"),
        network.insert_source_token("JavaScript", " "),
        network.insert_source_token("JavaScript", "="),
        network.insert_source_token("JavaScript", " "),
        network.insert_source_token("JavaScript", "[3, 1, 2]"),
        network.insert_source_token("JavaScript", ";"),
        network.insert_source_token("JavaScript", "\n"),
    ];
    let first_statement =
        network.insert_syntax_node("JavaScript", "lexical_declaration", first_tokens);

    let second_tokens = [
        network.insert_source_token("JavaScript", "values"),
        network.insert_source_token("JavaScript", ".sort();"),
        network.insert_source_token("JavaScript", "\n"),
    ];
    let second_statement =
        network.insert_syntax_node("JavaScript", "expression_statement", second_tokens);

    let third_tokens = [
        network.insert_source_token("JavaScript", "console.log("),
        network.insert_source_token("JavaScript", "values"),
        network.insert_source_token("JavaScript", ".join(\",\")"),
        network.insert_source_token("JavaScript", ");"),
        network.insert_source_token("JavaScript", "\n"),
    ];
    let third_statement =
        network.insert_syntax_node("JavaScript", "expression_statement", third_tokens);
    let program = network.insert_syntax_node(
        "JavaScript",
        "program",
        [first_statement, second_statement, third_statement],
    );

    let expected = "const values = [3, 1, 2];\nvalues.sort();\nconsole.log(values.join(\",\"));\n";

    assert_eq!(network.reconstruct_text(), "");
    assert_eq!(network.render_source_from(program, "JavaScript"), expected);
    assert_eq!(network.render_source("JavaScript"), expected);

    let parsed = LinkNetwork::parse(expected, "JavaScript", ParseConfiguration::default());
    assert_eq!(parsed.reconstruct_text(), expected);
    assert!(
        parsed.verify_full_match(None).is_clean(),
        "rendered source should parse cleanly"
    );
}

#[test]
fn render_source_can_follow_field_only_syntax_children() {
    let mut network = LinkNetwork::new();
    let keyword = network.insert_source_token("JavaScript", "return ");
    let literal = network.insert_source_token("JavaScript", "42");
    let semicolon = network.insert_source_token("JavaScript", ";");
    let statement = network.insert_syntax_node("JavaScript", "return_statement", []);

    network.insert_field(statement, "keyword", keyword);
    network.insert_field(statement, "argument", literal);
    network.insert_field(statement, "terminator", semicolon);

    assert_eq!(
        network.render_source_from(statement, "JavaScript"),
        "return 42;"
    );
}

#[test]
fn render_source_matches_reconstruct_text_for_span_backed_parse_networks() {
    let source = "const answer = 42;\n";
    let network = LinkNetwork::parse(source, "JavaScript", ParseConfiguration::default());

    assert_eq!(network.render_source("JavaScript"), source);
    assert_eq!(
        network.render_source_from_document("JavaScript"),
        Some(source.to_string())
    );
}
