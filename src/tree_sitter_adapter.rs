use tree_sitter::{Language, Node, Parser};

use crate::{
    ByteRange, LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration, Point,
    SourceSpan,
};

pub fn parse(text: &str, language: &str, configuration: ParseConfiguration) -> Option<LinkNetwork> {
    let grammar = grammar_for_language(language)?;
    let mut parser = Parser::new();
    parser.set_language(grammar).ok()?;
    let parsed = parser.parse(text, None)?;

    let (mut network, document) = LinkNetwork::new_parse_document(text, language);
    let root = parsed.root_node();
    convert_node(&mut network, document, root, text, language, configuration);
    network.attach_embedded_regions(
        document,
        text,
        language,
        configuration.region_detection_policy(),
    );
    Some(network)
}

fn grammar_for_language(language: &str) -> Option<Language> {
    if language.eq_ignore_ascii_case("python") {
        Some(tree_sitter_python::language())
    } else if language == "C" || language == "c" {
        Some(tree_sitter_c::language())
    } else if language.eq_ignore_ascii_case("java") {
        Some(tree_sitter_java::language())
    } else if language.eq_ignore_ascii_case("c++") || language.eq_ignore_ascii_case("cpp") {
        Some(tree_sitter_cpp::language())
    } else if language.eq_ignore_ascii_case("c#") || language.eq_ignore_ascii_case("csharp") {
        Some(tree_sitter_c_sharp::language())
    } else if language.eq_ignore_ascii_case("javascript") || language.eq_ignore_ascii_case("js") {
        Some(tree_sitter_javascript::language())
    } else if language == "R" || language == "r" {
        Some(tree_sitter_r::language())
    } else {
        None
    }
}

fn convert_node(
    network: &mut LinkNetwork,
    parent: LinkId,
    node: Node<'_>,
    text: &str,
    language: &str,
    configuration: ParseConfiguration,
) -> LinkId {
    let node_id = network.insert_link(
        [parent],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(node.is_named())
            .with_term(node.kind())
            .with_language(language)
            .with_span(span_for_node(node))
            .with_flags(flags_for_node(node)),
    );

    if node.child_count() == 0 {
        insert_leaf_token(network, node_id, node, text, language, configuration);
        return node_id;
    }

    let mut covered_until = node.start_byte();
    for child_index in 0..node.child_count() {
        let child = node
            .child(child_index)
            .expect("tree-sitter child index should be valid");
        insert_gap_token(
            network,
            node_id,
            text,
            covered_until,
            child.start_byte(),
            language,
            configuration,
        );

        let child_id = convert_node(network, node_id, child, text, language, configuration);
        if let Some(label) = node.field_name_for_child(
            u32::try_from(child_index).expect("tree-sitter child index fits in u32"),
        ) {
            network.insert_field(node_id, label, child_id);
        }
        covered_until = child.end_byte();
    }

    insert_gap_token(
        network,
        node_id,
        text,
        covered_until,
        node.end_byte(),
        language,
        configuration,
    );
    node_id
}

fn insert_leaf_token(
    network: &mut LinkNetwork,
    owner: LinkId,
    node: Node<'_>,
    text: &str,
    language: &str,
    configuration: ParseConfiguration,
) {
    if node.is_missing() || node.start_byte() == node.end_byte() {
        return;
    }

    let span = span_for_node(node);
    let flags = flags_for_node(node);
    let token = network.insert_link(
        [owner],
        LinkMetadata::new()
            .with_link_type(LinkType::Token)
            .with_named(node.is_named())
            .with_term(&text[node.start_byte()..node.end_byte()])
            .with_language(language)
            .with_span(span)
            .with_flags(flags),
    );

    if flags.is_extra() {
        network.attach_trivia(owner, token, span, configuration.trivia_attachment_policy());
    }
}

fn insert_gap_token(
    network: &mut LinkNetwork,
    owner: LinkId,
    text: &str,
    start: usize,
    end: usize,
    language: &str,
    configuration: ParseConfiguration,
) {
    if start == end {
        return;
    }

    let span = SourceSpan::new(
        ByteRange::new(start, end),
        point_at_byte(text, start),
        point_at_byte(text, end),
    );
    let token = network.insert_link(
        [owner],
        LinkMetadata::new()
            .with_link_type(LinkType::Token)
            .with_named(false)
            .with_term(&text[start..end])
            .with_language(language)
            .with_span(span)
            .with_flags(LinkFlags::extra()),
    );
    network.attach_trivia(owner, token, span, configuration.trivia_attachment_policy());
}

fn flags_for_node(node: Node<'_>) -> LinkFlags {
    let mut flags = LinkFlags::clean();
    if node.is_error() {
        flags = flags.with_error();
    }
    if node.has_error() && !node.is_error() && !node.is_missing() {
        flags = flags.with_containing_error();
    }
    if node.is_missing() {
        flags = flags.with_missing();
    }
    if node.is_extra() {
        flags = flags.with_extra();
    }
    flags
}

fn span_for_node(node: Node<'_>) -> SourceSpan {
    SourceSpan::new(
        ByteRange::new(node.start_byte(), node.end_byte()),
        point_from_tree_sitter(node.start_position()),
        point_from_tree_sitter(node.end_position()),
    )
}

const fn point_from_tree_sitter(point: tree_sitter::Point) -> Point {
    Point::new(point.row, point.column)
}

fn point_at_byte(text: &str, byte: usize) -> Point {
    let mut row = 0;
    let mut line_start = 0;
    for (index, value) in text.bytes().enumerate().take(byte) {
        if value == b'\n' {
            row += 1;
            line_start = index + 1;
        }
    }
    Point::new(row, byte - line_start)
}
