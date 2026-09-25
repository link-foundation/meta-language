use std::borrow::Cow;

use tree_sitter::{
    InputEdit, Language, Node, Parser, Point as TreeSitterPoint, Range as TreeSitterRange, Tree,
};

#[allow(unsafe_code)]
mod rocq_grammar {
    use tree_sitter_language::LanguageFn;

    unsafe extern "C" {
        fn tree_sitter_rocq() -> *const ();
    }

    // SAFETY: build.rs compiles the generated parser from the pinned revision
    // recorded in vendor/tree-sitter-rocq/NOTICE.md with this exact symbol.
    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_rocq) };
}

#[allow(unsafe_code)]
mod csv_grammar {
    use tree_sitter_language::LanguageFn;

    unsafe extern "C" {
        fn tree_sitter_csv() -> *const ();
    }

    // SAFETY: build.rs compiles the generated parser from the pinned tag
    // recorded in vendor/tree-sitter-csv/NOTICE.md with this exact symbol.
    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_csv) };
}

use crate::line_index::LineIndex;
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
    let lines = LineIndex::new(text);
    let context = ConvertContext::new(
        text,
        &lines,
        language,
        configuration,
        SpanOffset::zero(),
        text.len(),
    );
    let tree_parent =
        if language.eq_ignore_ascii_case("lean") || language.eq_ignore_ascii_case("lean4") {
            network.insert_link(
                [document],
                LinkMetadata::new()
                    .with_link_type(LinkType::Syntax)
                    .with_named(true)
                    .with_term("file")
                    .with_language(language)
                    .with_span(span_for_range(&lines, 0, text.len(), SpanOffset::zero()))
                    .with_flags(flags_for_node(root)),
            )
        } else {
            document
        };
    convert_root(&mut network, tree_parent, root, context);
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
    let lines = LineIndex::new(old_text);
    let start_position = lines.byte_point(range.start());
    let old_end_position = lines.byte_point(range.end());
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
    let lines = LineIndex::new(parse_text.as_ref());
    let context = ConvertContext::new(
        parse_text.as_ref(),
        &lines,
        language,
        configuration,
        SpanOffset::new(span.byte_range().start(), span.start_point()),
        text.len(),
    );
    Some(convert_root(network, region, root, context))
}

/// Converts a grammar root below `parent`. Tree-sitter starts the root after
/// its leading padding, so the text outside the root is retained as gap
/// tokens beside it, mirroring `parseGrammarCst` in
/// `js/src/programming-language-parser.js`.
fn convert_root(
    network: &mut LinkNetwork,
    parent: LinkId,
    root: Node<'_>,
    context: ConvertContext<'_>,
) -> LinkId {
    insert_gap_token(network, parent, 0, root.start_byte(), context);
    let root_id = convert_node(network, parent, root, context);
    insert_gap_token(
        network,
        parent,
        root.end_byte(),
        context.source_len,
        context,
    );
    root_id
}

/// Selects the primary default grammar the language catalog records for a
/// language name or alias.
fn grammar_for_language(language: &str) -> Option<Language> {
    let grammar = crate::language_catalog::language_entry(language)?
        .grammars
        .first()?;
    grammar_by_id(&grammar.id)
}

/// Returns the compiled grammar for a grammar-lock id.
pub fn grammar_by_id(id: &str) -> Option<Language> {
    Some(match id {
        "c" => tree_sitter_c::LANGUAGE.into(),
        "cpp" => tree_sitter_cpp::LANGUAGE.into(),
        "csharp" => tree_sitter_c_sharp::LANGUAGE.into(),
        "css" => tree_sitter_css::LANGUAGE.into(),
        "csv" => csv_grammar::LANGUAGE.into(),
        "dtd" => tree_sitter_xml::LANGUAGE_DTD.into(),
        "go" => tree_sitter_go::LANGUAGE.into(),
        "graphql" => tree_sitter_graphql::LANGUAGE.into(),
        "html" => tree_sitter_html::LANGUAGE.into(),
        "ini" => tree_sitter_ini::LANGUAGE.into(),
        "java" => tree_sitter_java::LANGUAGE.into(),
        "javascript" => tree_sitter_javascript::LANGUAGE.into(),
        "json" => tree_sitter_json::LANGUAGE.into(),
        "json5" => tree_sitter_json5_orchard::LANGUAGE.into(),
        "kotlin" => tree_sitter_kotlin_ng::LANGUAGE.into(),
        "lean" => tree_sitter_lean4::language(),
        "lua" => tree_sitter_lua::LANGUAGE.into(),
        "markdown" => tree_sitter_md_025::LANGUAGE.into(),
        "markdown_inline" => tree_sitter_md_025::INLINE_LANGUAGE.into(),
        "pascal" => tree_sitter_pascal::LANGUAGE.into(),
        "perl" => ts_parser_perl::LANGUAGE.into(),
        "php" => tree_sitter_php::LANGUAGE_PHP.into(),
        "proto" => tree_sitter_proto::LANGUAGE.into(),
        "python" => tree_sitter_python::LANGUAGE.into(),
        "r" => tree_sitter_r::LANGUAGE.into(),
        "rocq" => rocq_grammar::LANGUAGE.into(),
        "ruby" => tree_sitter_ruby::LANGUAGE.into(),
        "rust" => tree_sitter_rust::LANGUAGE.into(),
        "scala" => tree_sitter_scala::LANGUAGE.into(),
        "sql" => tree_sitter_sequel::LANGUAGE.into(),
        "swift" => tree_sitter_swift::LANGUAGE.into(),
        "toml" => tree_sitter_toml_ng::LANGUAGE.into(),
        "tsx" => tree_sitter_typescript::LANGUAGE_TSX.into(),
        "typescript" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "vb" => tree_sitter_vb_dotnet::LANGUAGE.into(),
        "xml" => tree_sitter_xml::LANGUAGE_XML.into(),
        "yaml" => tree_sitter_yaml::LANGUAGE.into(),
        _ => return None,
    })
}

fn convert_node(
    network: &mut LinkNetwork,
    parent: LinkId,
    node: Node<'_>,
    context: ConvertContext<'_>,
) -> LinkId {
    convert_node_with(network, parent, node, &[], context)
}

/// Converts `node`, placing `injected` nodes from another tree (Markdown block
/// continuations inside inline content) among its children. Mirrors
/// `convertGrammarNode` in `js/src/programming-language-parser.js`.
fn convert_node_with<'tree>(
    network: &mut LinkNetwork,
    parent: LinkId,
    node: Node<'tree>,
    injected: &[Node<'tree>],
    context: ConvertContext<'_>,
) -> LinkId {
    let flags = flags_for_node(node);
    let node_id = network.insert_link(
        [parent],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(node.is_named())
            .with_term(node.kind())
            .with_language(context.language)
            .with_span(span_for_node(
                node,
                context.lines,
                context.source_len,
                context.offset,
            ))
            .with_flags(flags),
    );

    let inline_tree = is_markdown_inline_container(node, context.language)
        .then(|| parse_markdown_inline(node, context.text));
    let mut extra_children = injected.to_vec();
    let own_children = inline_tree.as_ref().map_or_else(
        || children_with_fields(node),
        |tree| {
            extra_children.extend(markdown_inline_excluded_children(node));
            children_with_fields(tree.root_node())
        },
    );

    if own_children.is_empty() && extra_children.is_empty() {
        if is_rocq_identifier(node, context.language) {
            let semantic_term =
                rocq_identifier_term(&context.text[node.start_byte()..node.end_byte()]);
            let semantic_id = network.insert_link(
                [node_id],
                LinkMetadata::new()
                    .with_link_type(LinkType::Syntax)
                    .with_named(node.is_named())
                    .with_term(semantic_term)
                    .with_language(context.language)
                    .with_span(span_for_node(
                        node,
                        context.lines,
                        context.source_len,
                        context.offset,
                    ))
                    .with_flags(flags),
            );
            insert_leaf_token(network, semantic_id, node, context);
        } else {
            insert_leaf_token(network, node_id, node, context);
        }
        return node_id;
    }

    let mut covered_until = node.start_byte();
    let mut child_has_error = false;
    for child in distribute_injected_children(own_children, &extra_children) {
        if context.has_synthetic_suffix() && child.node.start_byte() >= context.source_len {
            break;
        }
        insert_gap_token(
            network,
            node_id,
            covered_until,
            child.node.start_byte(),
            context,
        );

        let child_id = convert_node_with(network, node_id, child.node, &child.injected, context);
        if let Some(label) = child.field {
            network.insert_field(node_id, label, child_id);
        }
        child_has_error |= network
            .link(child_id)
            .is_some_and(|link| link.metadata().flags().has_error());
        covered_until = covered_until.max(child.node.end_byte().min(context.source_len));
    }

    insert_gap_token(network, node_id, covered_until, node.end_byte(), context);
    // A Markdown inline tree is parsed separately from its block container, so
    // its errors reach the containing block nodes through their children.
    if child_has_error && !flags.has_error() {
        network.set_flags(node_id, flags.with_containing_error());
    }
    node_id
}

struct ChildNode<'tree> {
    node: Node<'tree>,
    field: Option<&'static str>,
    injected: Vec<Node<'tree>>,
}

fn children_with_fields(node: Node<'_>) -> Vec<ChildNode<'_>> {
    (0..node.child_count())
        .map(|index| {
            let index_u32 = u32::try_from(index).expect("tree-sitter child index fits in u32");
            ChildNode {
                node: node
                    .child(index)
                    .expect("tree-sitter child index should be valid"),
                field: node.field_name_for_child(index_u32),
                injected: Vec::new(),
            }
        })
        .collect()
}

fn is_markdown_inline_container(node: Node<'_>, language: &str) -> bool {
    (language.eq_ignore_ascii_case("markdown") || language.eq_ignore_ascii_case("md"))
        && matches!(node.kind(), "inline" | "pipe_table_cell")
}

/// Named children of a Markdown inline container after the first, which the
/// inline grammar skips, exactly as upstream's `MarkdownParser` does.
fn markdown_inline_excluded_children(node: Node<'_>) -> Vec<Node<'_>> {
    (1..node.child_count())
        .filter_map(|index| node.child(index))
        .filter(Node::is_named)
        .collect()
}

/// tree-sitter-markdown parses block structure and inline content with two
/// grammars. Like upstream's `MarkdownParser` (`bindings/rust/parser.rs` in
/// tree-sitter-md), every `inline` and `pipe_table_cell` block node is parsed
/// again with the inline grammar over the node's range minus its named
/// children after the first (block continuations such as a quote's `> `).
/// Mirrors `parseMarkdownInline` in `js/src/programming-language-parser.js`.
fn parse_markdown_inline(node: Node<'_>, text: &str) -> Tree {
    let mut range = node.range();
    let mut ranges = Vec::new();
    for child in markdown_inline_excluded_children(node) {
        ranges.push(TreeSitterRange {
            start_byte: range.start_byte,
            start_point: range.start_point,
            end_byte: child.start_byte(),
            end_point: child.start_position(),
        });
        range.start_byte = child.end_byte();
        range.start_point = child.end_position();
    }
    ranges.push(range);
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_md_025::INLINE_LANGUAGE.into())
        .expect("the Markdown inline grammar matches the tree-sitter ABI");
    parser
        .set_included_ranges(&ranges)
        .expect("Markdown inline ranges follow the ordered block children");
    parser
        .parse(text, None)
        .expect("tree-sitter parses without a timeout or cancellation")
}

/// Places nodes from another tree under the deepest child whose range contains
/// them, and orders the rest among the children by start offset, block nodes
/// first on ties.
fn distribute_injected_children<'tree>(
    mut children: Vec<ChildNode<'tree>>,
    injected: &[Node<'tree>],
) -> Vec<ChildNode<'tree>> {
    let mut top = Vec::new();
    for &node in injected {
        if let Some(owner) = children
            .iter_mut()
            .find(|child| contains_node(child.node, node))
        {
            owner.injected.push(node);
        } else {
            top.push(ChildNode {
                node,
                field: None,
                injected: Vec::new(),
            });
        }
    }
    if top.is_empty() {
        return children;
    }
    top.extend(children);
    top.sort_by_key(|child| child.node.start_byte());
    top
}

fn contains_node(outer: Node<'_>, inner: Node<'_>) -> bool {
    let (outer_start, outer_end) = (outer.start_byte(), outer.end_byte());
    let (inner_start, inner_end) = (inner.start_byte(), inner.end_byte());
    outer_start <= inner_start
        && inner_end <= outer_end
        && (inner_start < inner_end || (outer_start < inner_start && inner_start < outer_end))
}

fn is_rocq_identifier(node: Node<'_>, language: &str) -> bool {
    (language.eq_ignore_ascii_case("rocq") || language.eq_ignore_ascii_case("coq"))
        && node.kind() == "ident"
}

fn rocq_identifier_term(text: &str) -> &'static str {
    match text {
        "bool" | "nat" | "Prop" | "Set" | "SProp" | "Type" | "Z" => "primitive_type",
        _ => "identifier",
    }
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

    let span = span_for_range(context.lines, start, end, context.offset);
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

/// Inserts the source text between visible tree-sitter children. That text is
/// either lexer extras or text matched by hidden grammar rules (such as VB's
/// `Module` keyword). Leading and trailing whitespace becomes extra trivia, and
/// the text between them a non-extra token, mirroring `pushGapNodes` in
/// `js/src/programming-language-parser.js`.
fn insert_gap_token(
    network: &mut LinkNetwork,
    owner: LinkId,
    start: usize,
    end: usize,
    context: ConvertContext<'_>,
) {
    let start = start.min(context.source_len);
    let end = end.min(context.source_len);
    if start >= end {
        return;
    }
    let gap = &context.text[start..end];
    let content_start = start + (gap.len() - gap.trim_start().len());
    let content_end = content_start + gap.trim().len();
    for (piece_start, piece_end, flags) in [
        (start, content_start, LinkFlags::extra()),
        (content_start, content_end, LinkFlags::clean()),
        (content_end, end, LinkFlags::extra()),
    ] {
        if piece_start >= piece_end {
            continue;
        }
        let span = span_for_range(context.lines, piece_start, piece_end, context.offset);
        let token = network.insert_link(
            [owner],
            LinkMetadata::new()
                .with_link_type(LinkType::Token)
                .with_named(false)
                .with_term(&context.text[piece_start..piece_end])
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
}

fn flags_for_node(node: Node<'_>) -> LinkFlags {
    let mut flags = LinkFlags::clean();
    if node.is_error() {
        flags = flags.with_error();
    }
    // Mirrors tree-sitter's `ts_node_has_error`, which is true for error and
    // missing nodes themselves as well as for their ancestors.
    if node.has_error() || node.is_error() || node.is_missing() {
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

fn span_for_node(
    node: Node<'_>,
    lines: &LineIndex,
    source_len: usize,
    offset: SpanOffset,
) -> SourceSpan {
    let start = node.start_byte().min(source_len);
    let end = node.end_byte().min(source_len);
    span_for_range(lines, start, end, offset)
}

fn span_for_range(lines: &LineIndex, start: usize, end: usize, offset: SpanOffset) -> SourceSpan {
    SourceSpan::new(
        ByteRange::new(offset.byte + start, offset.byte + end),
        offset.point(lines.byte_point(start)),
        offset.point(lines.byte_point(end)),
    )
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

/// Position of a parsed text inside its host document, used to translate
/// region-relative spans into document spans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpanOffset {
    byte: usize,
    point: Point,
}

impl SpanOffset {
    pub const fn new(byte: usize, point: Point) -> Self {
        Self { byte, point }
    }

    pub const fn zero() -> Self {
        Self::new(0, Point::new(0, 0))
    }

    pub const fn byte(self, byte: usize) -> usize {
        self.byte + byte
    }

    pub const fn point(self, point: Point) -> Point {
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
    lines: &'a LineIndex,
    language: &'a str,
    configuration: ParseConfiguration,
    offset: SpanOffset,
    source_len: usize,
}

impl<'a> ConvertContext<'a> {
    const fn new(
        text: &'a str,
        lines: &'a LineIndex,
        language: &'a str,
        configuration: ParseConfiguration,
        offset: SpanOffset,
        source_len: usize,
    ) -> Self {
        Self {
            text,
            lines,
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
