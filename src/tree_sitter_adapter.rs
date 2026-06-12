use std::borrow::Cow;

use tree_sitter::{InputEdit, Language, Node, Parser, Point as TreeSitterPoint, Tree};

use crate::{
    ByteRange, LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration, Point,
    SourceSpan,
};

pub fn parse(text: &str, language: &str, configuration: ParseConfiguration) -> Option<LinkNetwork> {
    let grammar = grammar_for_language(language)?;
    let mut parser = Parser::new();
    parser.set_language(&grammar).ok()?;
    let parsed = parser.parse(text, None)?;

    Some(network_from_tree(text, language, configuration, &parsed))
}

pub fn parse_incremental(
    old_text: &str,
    range: ByteRange,
    replacement: &str,
    language: &str,
    configuration: ParseConfiguration,
) -> Option<LinkNetwork> {
    let grammar = grammar_for_language(language)?;
    let edited_text = apply_text_edit(old_text, range, replacement)?;
    let mut parser = Parser::new();
    parser.set_language(&grammar).ok()?;
    let mut old_tree = parser.parse(old_text, None)?;
    old_tree.edit(&input_edit(old_text, range, replacement));
    let parsed = parser.parse(&edited_text, Some(&old_tree))?;

    Some(network_from_tree(
        &edited_text,
        language,
        configuration,
        &parsed,
    ))
}

fn network_from_tree(
    text: &str,
    language: &str,
    configuration: ParseConfiguration,
    parsed: &Tree,
) -> LinkNetwork {
    let (mut network, document) = LinkNetwork::new_parse_document(text, language);
    let root = parsed.root_node();
    let context = ConvertContext::new(
        text,
        language,
        configuration,
        SpanOffset::zero(),
        text.len(),
    );
    convert_node(&mut network, document, root, context);
    network.attach_embedded_regions(document, text, language, configuration);
    network
}

fn apply_text_edit(old_text: &str, range: ByteRange, replacement: &str) -> Option<String> {
    if range.end() > old_text.len()
        || !old_text.is_char_boundary(range.start())
        || !old_text.is_char_boundary(range.end())
    {
        return None;
    }

    let mut edited =
        String::with_capacity(old_text.len() - (range.end() - range.start()) + replacement.len());
    edited.push_str(&old_text[..range.start()]);
    edited.push_str(replacement);
    edited.push_str(&old_text[range.end()..]);
    Some(edited)
}

fn input_edit(old_text: &str, range: ByteRange, replacement: &str) -> InputEdit {
    let start_position = point_at_byte(old_text, range.start());
    let old_end_position = point_at_byte(old_text, range.end());
    let new_end_position = point_after_text(start_position, replacement);

    InputEdit {
        start_byte: range.start(),
        old_end_byte: range.end(),
        new_end_byte: range.start() + replacement.len(),
        start_position: tree_sitter_point(start_position),
        old_end_position: tree_sitter_point(old_end_position),
        new_end_position: tree_sitter_point(new_end_position),
    }
}

fn point_after_text(start: Point, text: &str) -> Point {
    let mut row = start.row();
    let mut column = start.column();
    for byte in text.bytes() {
        if byte == b'\n' {
            row += 1;
            column = 0;
        } else {
            column += 1;
        }
    }
    Point::new(row, column)
}

const fn tree_sitter_point(point: Point) -> TreeSitterPoint {
    TreeSitterPoint::new(point.row(), point.column())
}

pub fn parse_embedded_region_into(
    network: &mut LinkNetwork,
    region: LinkId,
    text: &str,
    language: &str,
    span: SourceSpan,
    configuration: ParseConfiguration,
) -> Option<LinkId> {
    let grammar = grammar_for_language(language)?;
    let parse_text = embedded_parse_text(text, language);
    let mut parser = Parser::new();
    parser.set_language(&grammar).ok()?;
    let parsed = parser.parse(parse_text.as_ref(), None)?;
    let root = parsed.root_node();
    let context = ConvertContext::new(
        parse_text.as_ref(),
        language,
        configuration,
        SpanOffset::new(span.byte_range().start(), span.start_point()),
        text.len(),
    );
    Some(convert_node(network, region, root, context))
}

fn grammar_for_language(language: &str) -> Option<Language> {
    if language.eq_ignore_ascii_case("python") {
        Some(tree_sitter_python::LANGUAGE.into())
    } else if language == "C" || language == "c" {
        Some(tree_sitter_c::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("java") {
        Some(tree_sitter_java::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("c++") || language.eq_ignore_ascii_case("cpp") {
        Some(tree_sitter_cpp::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("c#") || language.eq_ignore_ascii_case("csharp") {
        Some(tree_sitter_c_sharp::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("javascript") || language.eq_ignore_ascii_case("js") {
        Some(tree_sitter_javascript::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("tsx") {
        Some(tree_sitter_typescript::LANGUAGE_TSX.into())
    } else if language.eq_ignore_ascii_case("typescript") || language.eq_ignore_ascii_case("ts") {
        Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
    } else if language.eq_ignore_ascii_case("visual basic")
        || language.eq_ignore_ascii_case("vb")
        || language.eq_ignore_ascii_case("vb.net")
        || language.eq_ignore_ascii_case("vbnet")
    {
        Some(tree_sitter_vb_dotnet::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("delphi/object pascal")
        || language.eq_ignore_ascii_case("delphi")
        || language.eq_ignore_ascii_case("object pascal")
        || language.eq_ignore_ascii_case("pascal")
    {
        Some(tree_sitter_pascal::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("rust") {
        Some(tree_sitter_rust::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("go") || language.eq_ignore_ascii_case("golang") {
        Some(tree_sitter_go::LANGUAGE.into())
    } else if language == "R" || language == "r" {
        Some(tree_sitter_r::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("ruby") || language.eq_ignore_ascii_case("rb") {
        Some(tree_sitter_ruby::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("sql-ansi") {
        Some(tree_sitter_sequel::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("html") {
        Some(tree_sitter_html::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("css") {
        Some(tree_sitter_css::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("json") {
        Some(tree_sitter_json::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("yaml") || language.eq_ignore_ascii_case("yml") {
        Some(tree_sitter_yaml::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("toml") {
        Some(tree_sitter_toml_ng::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("xml") {
        Some(tree_sitter_xml::LANGUAGE_XML.into())
    } else if language.eq_ignore_ascii_case("dtd") {
        Some(tree_sitter_xml::LANGUAGE_DTD.into())
    } else if language.eq_ignore_ascii_case("ini") {
        Some(tree_sitter_ini::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("protobuf")
        || language.eq_ignore_ascii_case("proto")
        || language.eq_ignore_ascii_case("protocol buffers")
    {
        Some(tree_sitter_proto::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("graphql") || language.eq_ignore_ascii_case("gql") {
        Some(tree_sitter_graphql::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("php") {
        Some(tree_sitter_php::LANGUAGE_PHP.into())
    } else if language.eq_ignore_ascii_case("swift") {
        Some(tree_sitter_swift::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("kotlin") || language.eq_ignore_ascii_case("kt") {
        Some(tree_sitter_kotlin_ng::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("scala") {
        Some(tree_sitter_scala::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("lua") {
        Some(tree_sitter_lua::LANGUAGE.into())
    } else if language.eq_ignore_ascii_case("perl") || language.eq_ignore_ascii_case("pl") {
        Some(ts_parser_perl::LANGUAGE.into())
    } else {
        None
    }
}

fn convert_node(
    network: &mut LinkNetwork,
    parent: LinkId,
    node: Node<'_>,
    context: ConvertContext<'_>,
) -> LinkId {
    let node_id = network.insert_link(
        [parent],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(node.is_named())
            .with_term(node.kind())
            .with_language(context.language)
            .with_span(span_for_node(
                node,
                context.text,
                context.source_len,
                context.offset,
            ))
            .with_flags(flags_for_node(node)),
    );

    if node.child_count() == 0 {
        insert_leaf_token(network, node_id, node, context);
        return node_id;
    }

    let mut covered_until = node.start_byte();
    for child_index in 0..node.child_count() {
        let child_index_u32 =
            u32::try_from(child_index).expect("tree-sitter child index fits in u32");
        let child = node
            .child(child_index)
            .expect("tree-sitter child index should be valid");
        if context.has_synthetic_suffix() && child.start_byte() >= context.source_len {
            break;
        }
        insert_gap_token(network, node_id, covered_until, child.start_byte(), context);

        let child_id = convert_node(network, node_id, child, context);
        if let Some(label) = node.field_name_for_child(child_index_u32) {
            network.insert_field(node_id, label, child_id);
        }
        covered_until = child.end_byte().min(context.source_len);
    }

    insert_gap_token(network, node_id, covered_until, node.end_byte(), context);
    node_id
}

fn insert_leaf_token(
    network: &mut LinkNetwork,
    owner: LinkId,
    node: Node<'_>,
    context: ConvertContext<'_>,
) {
    let start = node.start_byte();
    let end = node.end_byte().min(context.source_len);
    if node.is_missing() || start >= end {
        return;
    }

    let span = span_for_range(context.text, start, end, context.offset);
    let flags = flags_for_node(node);
    let token = network.insert_link(
        [owner],
        LinkMetadata::new()
            .with_link_type(LinkType::Token)
            .with_named(node.is_named())
            .with_term(&context.text[start..end])
            .with_language(context.language)
            .with_span(span)
            .with_flags(flags),
    );

    if flags.is_extra() {
        network.attach_trivia(
            owner,
            token,
            span,
            context.configuration.trivia_attachment_policy(),
        );
    }
}

fn insert_gap_token(
    network: &mut LinkNetwork,
    owner: LinkId,
    start: usize,
    end: usize,
    context: ConvertContext<'_>,
) {
    let start = start.min(context.source_len);
    let end = end.min(context.source_len);
    if start == end {
        return;
    }

    let span = span_for_range(context.text, start, end, context.offset);
    let token = network.insert_link(
        [owner],
        LinkMetadata::new()
            .with_link_type(LinkType::Token)
            .with_named(false)
            .with_term(&context.text[start..end])
            .with_language(context.language)
            .with_span(span)
            .with_flags(LinkFlags::extra()),
    );
    network.attach_trivia(
        owner,
        token,
        span,
        context.configuration.trivia_attachment_policy(),
    );
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

fn span_for_node(node: Node<'_>, text: &str, source_len: usize, offset: SpanOffset) -> SourceSpan {
    let start = node.start_byte().min(source_len);
    let end = node.end_byte().min(source_len);
    span_for_range(text, start, end, offset)
}

fn span_for_range(text: &str, start: usize, end: usize, offset: SpanOffset) -> SourceSpan {
    SourceSpan::new(
        ByteRange::new(offset.byte + start, offset.byte + end),
        offset.point(point_at_byte(text, start)),
        offset.point(point_at_byte(text, end)),
    )
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

fn embedded_parse_text<'a>(text: &'a str, language: &str) -> Cow<'a, str> {
    if language.eq_ignore_ascii_case("css") && css_declaration_list_needs_semicolon(text) {
        Cow::Owned(format!("{text};"))
    } else {
        Cow::Borrowed(text)
    }
}

fn css_declaration_list_needs_semicolon(text: &str) -> bool {
    let trimmed = text.trim_end();
    !trimmed.is_empty()
        && !trimmed.ends_with(';')
        && !trimmed.ends_with('}')
        && !trimmed.contains('{')
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SpanOffset {
    byte: usize,
    point: Point,
}

impl SpanOffset {
    const fn new(byte: usize, point: Point) -> Self {
        Self { byte, point }
    }

    const fn zero() -> Self {
        Self::new(0, Point::new(0, 0))
    }

    const fn point(self, point: Point) -> Point {
        let row = self.point.row() + point.row();
        let column = if point.row() == 0 {
            self.point.column() + point.column()
        } else {
            point.column()
        };
        Point::new(row, column)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ConvertContext<'a> {
    text: &'a str,
    language: &'a str,
    configuration: ParseConfiguration,
    offset: SpanOffset,
    source_len: usize,
}

impl<'a> ConvertContext<'a> {
    const fn new(
        text: &'a str,
        language: &'a str,
        configuration: ParseConfiguration,
        offset: SpanOffset,
        source_len: usize,
    ) -> Self {
        Self {
            text,
            language,
            configuration,
            offset,
            source_len,
        }
    }

    const fn has_synthetic_suffix(self) -> bool {
        self.source_len < self.text.len()
    }
}
