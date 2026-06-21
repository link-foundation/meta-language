use meta_language::{LinkId, LinkNetwork, ParseConfiguration};

fn token(network: &mut LinkNetwork, text: &str) -> LinkId {
    network.insert_source_token("JavaScript", text)
}

fn main() {
    let mut network = LinkNetwork::new();

    let declaration_tokens = [
        token(&mut network, "const values = "),
        token(&mut network, "[3, 1, 2]"),
        token(&mut network, ";\n"),
    ];
    let declaration =
        network.insert_syntax_node("JavaScript", "lexical_declaration", declaration_tokens);

    let sort_tokens = [
        token(&mut network, "values"),
        token(&mut network, ".sort();\n"),
    ];
    let sort_call = network.insert_syntax_node("JavaScript", "expression_statement", sort_tokens);

    let print_tokens = [
        token(&mut network, "console.log("),
        token(&mut network, "values.join(\",\")"),
        token(&mut network, ");\n"),
    ];
    let print_call = network.insert_syntax_node("JavaScript", "expression_statement", print_tokens);

    let _program = network.insert_syntax_node(
        "JavaScript",
        "program",
        [declaration, sort_call, print_call],
    );

    let source = network.render_source("JavaScript");
    assert_eq!(
        source,
        "const values = [3, 1, 2];\nvalues.sort();\nconsole.log(values.join(\",\"));\n"
    );

    let parsed = LinkNetwork::parse(&source, "JavaScript", ParseConfiguration::default());
    assert!(parsed.verify_full_match(None).is_clean());
}
