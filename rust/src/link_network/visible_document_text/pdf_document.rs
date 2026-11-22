use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use miniz_oxide::inflate::decompress_to_vec_zlib_with_limit;

use super::document_container::{
    BinaryStringCoordinates, binary_string, binary_string_bytes, byte_source_span,
    insert_derived_document,
};
use super::document_container_policy::{
    document_stream_fits_budget, pdf_drawing_operator, pdf_lzw_code_width, pdf_stream_filter_kind,
    pdf_text_operator_kind,
};
use super::pdf_text_fonts::{
    PdfFont, decode_font_text, glyph_name, hex_bytes, pdf_whitespace, predefined_encoding,
    read_unicode_map, string_bytes,
};
use crate::builtin_grammar::GrammarNode;
use crate::pdf_grammar::{parse_binary_pdf_cst, parse_pdf_content_cst};
use crate::structured_text_parser::pdf_content_network;
use crate::{
    LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration, SourceSpan,
};

const MAXIMUM_STREAM_BYTES: usize = 16 * 1024 * 1024;
const MAXIMUM_DECODED_BYTES: usize = 32 * 1024 * 1024;
const MAXIMUM_CMAP_BYTES: usize = 2 * 1024 * 1024;

pub(crate) fn attach_binary_pdf(
    network: &mut LinkNetwork,
    parent: LinkId,
    bytes: &[u8],
    language: &str,
    configuration: ParseConfiguration,
) {
    let text = binary_string(bytes);
    let coordinates = BinaryStringCoordinates::new(bytes);
    let span =
        |start, end| byte_source_span(coordinates.byte_offset(start), coordinates.byte_offset(end));
    let tree = parse_binary_pdf_cst(&text);
    let root = network.insert_link(
        [parent],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_term("pdf_file")
            .with_language(language)
            .with_named(true)
            .with_span(span(0, text.len()))
            .with_flags(if tree.has_error {
                LinkFlags::containing_error()
            } else {
                LinkFlags::clean()
            }),
    );
    let mut pending = vec![&tree];
    while let Some(node) = pending.pop() {
        if node.is_error || node.is_missing {
            let flags = if node.is_missing {
                LinkFlags::missing()
            } else {
                LinkFlags::error()
            };
            network.insert_link(
                [root],
                LinkMetadata::new()
                    .with_link_type(LinkType::Syntax)
                    .with_term(node.term)
                    .with_language(language)
                    .with_named(node.named)
                    .with_span(span(node.start, node.end))
                    .with_flags(flags),
            );
        }
        if node.has_error {
            pending.extend(&node.children);
        }
    }
    attach_pdf_content(network, root, &text, &tree, span, configuration, "bytes");
}

pub(crate) fn attach_pdf_content(
    network: &mut LinkNetwork,
    parent: LinkId,
    text: &str,
    tree: &GrammarNode,
    span: impl Fn(usize, usize) -> SourceSpan,
    configuration: ParseConfiguration,
    encoding: &str,
) {
    let document = PdfReader::new(text, tree, configuration.decode_document_streams()).read();
    for (stream, tree) in document.streams {
        let content = pdf_content_network(&stream.source, &tree, configuration);
        let metadata = LinkMetadata::new()
            .with_term("content_stream")
            .with_language("PDF")
            .with_span(span(stream.data.start, stream.data.end));
        insert_derived_document(
            network,
            parent,
            metadata,
            encoding,
            &stream.filter,
            &content,
        );
    }
    for run in document.text_runs {
        network.insert_link(
            [parent],
            LinkMetadata::new()
                .with_link_type(LinkType::Semantic)
                .with_term(if run.separated {
                    "PDF text line"
                } else {
                    "PDF text run"
                })
                .with_language("PDF")
                .with_span(span(run.start, run.end))
                .with_definition(run.text),
        );
    }
    for (data, error) in document.errors {
        network.insert_link(
            [parent],
            LinkMetadata::new()
                .with_link_type(LinkType::Object)
                .with_term("ERROR")
                .with_language("PDF")
                .with_span(span(data.start, data.end))
                .with_named(true)
                .with_definition(error)
                .with_flags(LinkFlags::error()),
        );
    }
}

fn field<'tree>(node: Option<&'tree GrammarNode>, name: &str) -> Option<&'tree GrammarNode> {
    node?
        .children
        .iter()
        .find(|child| child.field == Some(name))
}

fn array_elements(node: Option<&GrammarNode>) -> impl DoubleEndedIterator<Item = &GrammarNode> {
    node.into_iter()
        .flat_map(|node| &node.children)
        .filter(|child| child.named && !child.extra)
}

fn text_of<'source>(source: &'source str, node: Option<&GrammarNode>) -> &'source str {
    node.map_or("", |node| &source[node.start..node.end])
}

fn name_of<'source>(source: &'source str, node: Option<&GrammarNode>) -> Cow<'source, str> {
    let Some(node) = node.filter(|node| node.term == "name") else {
        return Cow::Borrowed("");
    };
    let name = &source[node.start + 1..node.end];
    if !name.contains('#') {
        return Cow::Borrowed(name);
    }
    let mut decoded = String::new();
    let mut remaining = name;
    while let Some(position) = remaining.find('#') {
        decoded.push_str(&remaining[..position]);
        remaining = &remaining[position + 1..];
        if let Some(byte) = remaining
            .get(..2)
            .and_then(|hex| u8::from_str_radix(hex, 16).ok())
        {
            decoded.push(char::from(byte));
            remaining = &remaining[2..];
        } else {
            decoded.push('#');
        }
    }
    decoded.push_str(remaining);
    Cow::Owned(decoded)
}

type ObjectReference = (u64, u64);

fn object_reference(text: &str, node: &GrammarNode) -> Option<ObjectReference> {
    Some((
        text_of(text, field(Some(node), "object_number"))
            .parse()
            .ok()?,
        text_of(text, field(Some(node), "generation"))
            .parse()
            .ok()?,
    ))
}

struct PdfStream<'source> {
    source: Cow<'source, str>,
    data: &'source GrammarNode,
    filter: String,
}

struct TextRun {
    text: String,
    separated: bool,
    start: usize,
    end: usize,
    position: Option<TextPosition>,
}

#[derive(Clone, Copy)]
struct TextPosition {
    x: f64,
    y: f64,
    horizontal_scale: f64,
    vertical_scale: f64,
}

impl Default for TextPosition {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            horizontal_scale: 1.0,
            vertical_scale: 1.0,
        }
    }
}

#[derive(Default)]
struct PdfDocument<'source> {
    streams: Vec<(Rc<PdfStream<'source>>, Rc<GrammarNode>)>,
    text_runs: Vec<TextRun>,
    errors: Vec<(&'source GrammarNode, &'static str)>,
}

struct TextState {
    active: HashSet<usize>,
    separated: bool,
    font: Option<Rc<PdfFont>>,
    ordinate: Option<f64>,
    position: Option<TextPosition>,
    leading: f64,
    orderable: bool,
}

impl TextState {
    fn new() -> Self {
        Self {
            active: HashSet::new(),
            separated: true,
            font: None,
            ordinate: None,
            position: None,
            leading: 0.0,
            orderable: true,
        }
    }

    fn advance_line(&mut self) {
        self.position = self
            .position
            .filter(|_| self.leading.is_finite())
            .map(|mut position| {
                position.y -= self.leading * position.vertical_scale;
                position
            });
    }
}

fn order_page_text(runs: &mut Vec<TextRun>, start: usize) {
    let mut positions = Vec::new();
    for run in runs[start..].iter().filter(|run| run.separated) {
        let Some(position) = run
            .position
            .filter(|position| position.x.is_finite() && position.y.is_finite())
        else {
            return;
        };
        positions.push(position);
    }
    let mut lines = Vec::<(TextPosition, Vec<TextRun>)>::new();
    let mut positions = positions.into_iter();
    for run in runs.drain(start..) {
        if run.separated {
            lines.push((positions.next().expect("validated text line"), Vec::new()));
        }
        lines
            .last_mut()
            .expect("each page starts with a text line")
            .1
            .push(run);
    }
    lines.sort_by(|(left, _), (right, _)| {
        right
            .y
            .partial_cmp(&left.y)
            .expect("finite vertical positions")
            .then_with(|| {
                left.x
                    .partial_cmp(&right.x)
                    .expect("finite horizontal positions")
            })
    });
    runs.extend(lines.into_iter().flat_map(|(_, runs)| runs));
}

struct PdfReader<'source> {
    text: &'source str,
    decode_streams: bool,
    objects: HashMap<ObjectReference, &'source GrammarNode>,
    order: Vec<ObjectReference>,
    attached: HashSet<usize>,
    stream_cache: HashMap<usize, Option<Rc<PdfStream<'source>>>>,
    font_cache: HashMap<usize, Rc<PdfFont>>,
    decoded_bytes: usize,
    output: PdfDocument<'source>,
}

impl<'source> PdfReader<'source> {
    fn new(text: &'source str, tree: &'source GrammarNode, decode_streams: bool) -> Self {
        let mut order = Vec::new();
        let mut objects = HashMap::new();
        for object in tree
            .children
            .iter()
            .filter(|object| object.term == "indirect_object")
        {
            if let (Some(key), Some(value)) =
                (object_reference(text, object), field(Some(object), "value"))
                && objects.insert(key, value).is_none()
            {
                order.push(key);
            }
        }
        Self {
            text,
            decode_streams,
            objects,
            order,
            attached: HashSet::new(),
            stream_cache: HashMap::new(),
            font_cache: HashMap::new(),
            decoded_bytes: 0,
            output: PdfDocument::default(),
        }
    }

    fn resolve(&self, node: &'source GrammarNode) -> Option<&'source GrammarNode> {
        if node.term == "indirect_reference" {
            self.objects
                .get(&object_reference(self.text, node)?)
                .copied()
        } else {
            Some(node)
        }
    }

    fn get(&self, node: Option<&'source GrammarNode>, key: &str) -> Option<&'source GrammarNode> {
        let dictionary = self.resolve(node?)?;
        let entry = dictionary.children.iter().find(|entry| {
            entry.term == "dictionary_entry" && name_of(self.text, field(Some(entry), "key")) == key
        })?;
        self.resolve(field(Some(entry), "value")?)
    }

    fn kind(&self, node: Option<&'source GrammarNode>, key: &str) -> Cow<'source, str> {
        name_of(self.text, self.get(node, key))
    }

    fn stream(&mut self, node: Option<&'source GrammarNode>) -> Option<Rc<PdfStream<'source>>> {
        let node = node.filter(|node| node.term == "stream")?;
        let data = field(Some(node), "data")?;
        let dictionary = field(Some(node), "dictionary");
        let filter = self.get(dictionary, "Filter");
        let source = text_of(self.text, Some(data));
        let Some(filter) = filter else {
            return Some(Rc::new(PdfStream {
                source: Cow::Borrowed(source),
                data,
                filter: String::new(),
            }));
        };
        if !self.decode_streams {
            return None;
        }
        if let Some(cached) = self.stream_cache.get(&node.start) {
            return cached.clone();
        }
        let names = if filter.term == "array" {
            array_elements(Some(filter))
                .map(|item| name_of(self.text, self.resolve(item)))
                .collect::<Vec<_>>()
        } else {
            vec![name_of(self.text, Some(filter))]
        };
        let kinds = names
            .iter()
            .map(|name| pdf_stream_filter_kind(name))
            .collect::<Vec<_>>();
        if kinds.is_empty() || kinds.contains(&0) {
            return None;
        }
        let parameters = self.get(dictionary, "DecodeParms");
        let options = if parameters.is_some_and(|node| node.term == "array") {
            array_elements(parameters)
                .map(|parameter| self.resolve(parameter))
                .collect::<Vec<_>>()
        } else {
            vec![parameters]
        };
        if options.iter().any(|&option| {
            text_of(self.text, self.get(option, "Predictor"))
                .parse::<f64>()
                .is_ok_and(|value| value > 1.0)
        }) {
            return None;
        }
        let stream = match self.decode(source, &kinds, &options) {
            Ok(source) => Some(Rc::new(PdfStream {
                source: Cow::Owned(source),
                data,
                filter: names.join(","),
            })),
            Err(error) => {
                self.output.errors.push((data, error));
                None
            }
        };
        self.stream_cache.insert(node.start, stream.clone());
        stream
    }

    fn decode(
        &mut self,
        source: &str,
        kinds: &[u8],
        options: &[Option<&GrammarNode>],
    ) -> Result<String, &'static str> {
        const SIZE_LIMIT: &str = "PDF stream exceeds the decoded size limit";
        let encoded_size = source.chars().count();
        if self.decoded_bytes >= MAXIMUM_DECODED_BYTES
            || !document_stream_fits_budget(encoded_size, 0, self.decoded_bytes)
            || kinds.len() > 8
        {
            return Err(SIZE_LIMIT);
        }
        let mut encoded = binary_string_bytes(source).ok_or("invalid binary PDF stream")?;
        for (index, &kind) in kinds.iter().enumerate() {
            let limit = MAXIMUM_STREAM_BYTES.min(MAXIMUM_DECODED_BYTES - self.decoded_bytes);
            if limit == 0 {
                return Err(SIZE_LIMIT);
            }
            encoded = match kind {
                1 => decompress_to_vec_zlib_with_limit(&encoded, limit)
                    .map_err(|_| "invalid or oversized FlateDecode PDF stream")?,
                2 => decode_ascii_hex(&encoded)?,
                3 => decode_ascii85(&encoded, limit)?,
                _ => {
                    let parameter = self.get(options.get(index).copied().flatten(), "EarlyChange");
                    let early_change = if let Some(node) = parameter {
                        let change = text_of(self.text, Some(node)).parse::<i64>();
                        match (node.term, change) {
                            ("integer", Ok(0)) => 0,
                            ("integer", Ok(1)) => 1,
                            _ => return Err("invalid LZWDecode PDF EarlyChange"),
                        }
                    } else {
                        1
                    };
                    decode_lzw(&encoded, early_change, limit)?
                }
            };
            if !document_stream_fits_budget(0, encoded.len(), self.decoded_bytes) {
                return Err(SIZE_LIMIT);
            }
            self.decoded_bytes += encoded.len();
        }
        Ok(binary_string(&encoded))
    }

    fn font(&mut self, node: Option<&'source GrammarNode>) -> Option<Rc<PdfFont>> {
        let node = node?;
        if let Some(cached) = self.font_cache.get(&node.start) {
            return Some(Rc::clone(cached));
        }
        let encoding_node = self.get(Some(node), "Encoding");
        let mut encoding_name = name_of(self.text, encoding_node);
        if encoding_name.is_empty() {
            encoding_name = self.kind(encoding_node, "BaseEncoding");
        }
        let fallback = match self.kind(Some(node), "BaseFont").as_ref() {
            "Symbol" => "SymbolSetEncoding",
            "ZapfDingbats" => "ZapfDingbatsEncoding",
            _ => "StandardEncoding",
        };
        let encoding = predefined_encoding(if encoding_name.is_empty() {
            fallback
        } else {
            &encoding_name
        });
        let composite =
            self.kind(Some(node), "Subtype") == "Type0" || encoding_name.starts_with("Identity-");
        let mut differences = HashMap::new();
        let mut code = None;
        for member in array_elements(self.get(encoding_node, "Differences")) {
            if member.term == "integer" {
                code = text_of(self.text, Some(member)).parse::<u32>().ok();
            } else if member.term == "name"
                && let Some(value) = code.filter(|value| *value <= 255)
            {
                differences.insert(value, glyph_name(&name_of(self.text, Some(member))));
                code = Some(value + 1);
            }
        }
        let cmap = self
            .stream(self.get(Some(node), "ToUnicode"))
            .filter(|stream| {
                stream.source.chars().take(MAXIMUM_CMAP_BYTES + 1).count() <= MAXIMUM_CMAP_BYTES
            })
            .map(|stream| read_unicode_map(&stream.source));
        let font = Rc::new(PdfFont {
            encoding,
            differences,
            composite,
            cmap,
        });
        self.font_cache.insert(node.start, Rc::clone(&font));
        Some(font)
    }

    fn visit_content(
        &mut self,
        node: Option<&'source GrammarNode>,
        resources: Option<&'source GrammarNode>,
        state: &mut TextState,
        depth: usize,
    ) {
        let Some(node) = node.filter(|node| depth <= 16 && !state.active.contains(&node.start))
        else {
            return;
        };
        let Some(stream) = self.stream(Some(node)) else {
            return;
        };
        let Some(tree) = parse_pdf_content_cst(&stream.source) else {
            return;
        };
        if !tree.children.iter().any(|operation| {
            let operator = text_of(&stream.source, field(Some(operation), "operator"));
            operation.term == "operation"
                && (pdf_text_operator_kind(operator) != 0 || pdf_drawing_operator(operator))
        }) {
            return;
        }
        let tree = Rc::new(tree);
        if !stream.filter.is_empty() && self.attached.insert(node.start) {
            self.output
                .streams
                .push((Rc::clone(&stream), Rc::clone(&tree)));
        }
        state.active.insert(node.start);
        for operation in tree
            .children
            .iter()
            .filter(|operation| operation.term == "operation")
        {
            self.operation(&stream, operation, resources, state, depth);
        }
        state.active.remove(&node.start);
    }

    fn operation(
        &mut self,
        stream: &PdfStream<'_>,
        operation: &GrammarNode,
        resources: Option<&'source GrammarNode>,
        state: &mut TextState,
        depth: usize,
    ) {
        let source = &stream.source;
        let operator = text_of(source, field(Some(operation), "operator"));
        let operands = operation
            .children
            .iter()
            .filter(|child| child.field == Some("operand"));
        let first = operands.clone().next();
        let last = operands.clone().next_back();
        let numeric_operand = |operand: &GrammarNode| {
            if !matches!(operand.term, "integer" | "real") {
                return None;
            }
            text_of(source, Some(operand))
                .parse::<f64>()
                .ok()
                .filter(|value| value.is_finite())
        };
        match pdf_text_operator_kind(operator) {
            1 => {
                state.font =
                    self.font(self.get(self.get(resources, "Font"), &name_of(source, first)))
            }
            2 => {
                let form = self.get(self.get(resources, "XObject"), &name_of(source, first));
                let dictionary = field(form, "dictionary");
                if self.kind(dictionary, "Subtype") == "Form" {
                    let font = state.font.clone();
                    let ordinate = state.ordinate;
                    let position = state.position;
                    let leading = state.leading;
                    state.orderable = false;
                    self.visit_content(
                        form,
                        self.get(dictionary, "Resources").or(resources),
                        state,
                        depth + 1,
                    );
                    state.font = font;
                    state.ordinate = ordinate;
                    state.position = position;
                    state.leading = leading;
                }
            }
            3 => self.show_text(stream, last, state),
            4 => {
                for item in array_elements(last) {
                    if matches!(item.term, "integer" | "real") {
                        let space = text_of(source, Some(item))
                            .parse::<f64>()
                            .is_ok_and(|value| value <= -250.0);
                        if space
                            && !state.separated
                            && self
                                .output
                                .text_runs
                                .last()
                                .is_some_and(|run| !run.text.ends_with(' '))
                        {
                            self.show_run(stream, item, state, " ".into());
                        }
                    } else {
                        self.show_text(stream, Some(item), state);
                    }
                }
            }
            5 => {
                state.separated = true;
                state.advance_line();
                self.show_text(stream, last, state);
            }
            6 => {
                state.separated = true;
                if operator == "BT" {
                    state.ordinate = None;
                    state.position = None;
                }
                if operator == "T*" {
                    state.advance_line();
                }
            }
            7 => {
                let ordinate = text_of(source, last).parse::<f64>().unwrap_or(0.0);
                if (operator == "Tm" && Some(ordinate) != state.ordinate)
                    || (operator != "Tm" && ordinate != 0.0)
                {
                    state.separated = true;
                }
                state.ordinate = Some(if operator == "Tm" {
                    ordinate
                } else {
                    state.ordinate.unwrap_or_default() + ordinate
                });
                let values = operands.map(numeric_operand).collect::<Option<Vec<_>>>();
                match (operator, values.as_deref()) {
                    ("Tm", Some(&[a, b, c, d, x, y]))
                        if a > 0.0 && b == 0.0 && c == 0.0 && d > 0.0 =>
                    {
                        state.position = Some(TextPosition {
                            x,
                            y,
                            horizontal_scale: a,
                            vertical_scale: d,
                        });
                    }
                    ("Td" | "TD", Some(&[x, y])) => {
                        let mut position = state.position.unwrap_or_default();
                        position.x += x * position.horizontal_scale;
                        position.y += y * position.vertical_scale;
                        state.position = Some(position);
                        if operator == "TD" {
                            state.leading = -y;
                        }
                    }
                    _ => state.orderable = false,
                }
            }
            _ if operator == "TL" => {
                state.leading = first
                    .filter(|_| operands.count() == 1)
                    .and_then(numeric_operand)
                    .unwrap_or(f64::NAN);
            }
            _ if operator == "cm" => state.orderable = false,
            _ => {}
        }
    }

    fn show_text(
        &mut self,
        stream: &PdfStream<'_>,
        node: Option<&GrammarNode>,
        state: &mut TextState,
    ) {
        let Some(node) = node.filter(|node| matches!(node.term, "literal_string" | "hex_string"))
        else {
            return;
        };
        let text = decode_font_text(
            &string_bytes(text_of(&stream.source, Some(node))),
            state.font.as_deref(),
        );
        if !text.is_empty() {
            self.show_run(stream, node, state, text);
        }
    }

    fn show_run(
        &mut self,
        stream: &PdfStream<'_>,
        node: &GrammarNode,
        state: &mut TextState,
        text: String,
    ) {
        let (start, end) = if stream.filter.is_empty() {
            (stream.data.start + node.start, stream.data.start + node.end)
        } else {
            (stream.data.start, stream.data.end)
        };
        self.output.text_runs.push(TextRun {
            text,
            separated: state.separated,
            start,
            end,
            position: state.position,
        });
        state.separated = false;
    }

    fn read(mut self) -> PdfDocument<'source> {
        let objects = self
            .order
            .iter()
            .filter_map(|reference| self.objects.get(reference).copied())
            .collect::<Vec<_>>();
        let catalog = objects
            .iter()
            .copied()
            .find(|node| self.kind(Some(node), "Type") == "Catalog");
        if let Some(root) = self.get(catalog, "Pages") {
            let mut frames = vec![(root, None)];
            let mut visited = HashSet::new();
            while let Some((node, inherited)) = frames.pop() {
                if !visited.insert(node.start) {
                    continue;
                }
                let resources = self.get(Some(node), "Resources").or(inherited);
                match self.kind(Some(node), "Type").as_ref() {
                    "Pages" => {
                        for kid in array_elements(self.get(Some(node), "Kids"))
                            .rev()
                            .filter_map(|kid| self.resolve(kid))
                        {
                            frames.push((kid, resources));
                        }
                    }
                    "Page" => {
                        let contents = self.get(Some(node), "Contents");
                        let mut state = TextState::new();
                        let page_start = self.output.text_runs.len();
                        if contents.is_some_and(|node| node.term == "array") {
                            for item in array_elements(contents) {
                                self.visit_content(self.resolve(item), resources, &mut state, 0);
                            }
                        } else {
                            self.visit_content(contents, resources, &mut state, 0);
                        }
                        if state.orderable {
                            order_page_text(&mut self.output.text_runs, page_start);
                        }
                    }
                    _ => {}
                }
            }
        } else {
            for node in objects {
                let dictionary = field(Some(node), "dictionary");
                if node.term == "stream"
                    && matches!(self.kind(dictionary, "Type").as_ref(), "" | "XObject")
                    && matches!(self.kind(dictionary, "Subtype").as_ref(), "" | "Form")
                {
                    self.visit_content(
                        Some(node),
                        self.get(dictionary, "Resources"),
                        &mut TextState::new(),
                        0,
                    );
                }
            }
        }
        self.output
    }
}

fn decode_lzw(source: &[u8], early_change: usize, limit: usize) -> Result<Vec<u8>, &'static str> {
    const INVALID: &str = "invalid or oversized LZWDecode PDF stream";
    const DICTIONARY_SIZE: usize = 4096;
    let mut prefix = [0_u16; DICTIONARY_SIZE];
    let mut suffix = [0_u8; DICTIONARY_SIZE];
    let mut sequence = [0_u8; DICTIONARY_SIZE];
    let mut output = Vec::new();
    let mut next = 258;
    let mut width = 9;
    let mut previous = None;
    let mut bits = 0;
    let mut accumulator = 0_u32;
    let mut bytes = source.iter();
    loop {
        while bits < width {
            accumulator = (accumulator << 8) | u32::from(*bytes.next().ok_or(INVALID)?);
            bits += 8;
        }
        bits -= width;
        let code = ((accumulator >> bits) & ((1 << width) - 1)) as usize;
        if code == 256 {
            next = 258;
            width = 9;
            previous = None;
            continue;
        }
        if code == 257 {
            return Ok(output);
        }
        if code > next || (code >= 258 && previous.is_none()) {
            return Err(INVALID);
        }
        // The next entry is the previous entry followed by its first byte.
        let next_entry = code == next;
        let mut entry = if next_entry {
            previous.ok_or(INVALID)?
        } else {
            code
        };
        let mut length = usize::from(next_entry);
        while entry >= 258 {
            sequence[length] = suffix[entry];
            length += 1;
            entry = usize::from(prefix[entry]);
        }
        sequence[length] = entry as u8;
        length += 1;
        if next_entry {
            sequence[0] = entry as u8;
        }
        if output.len() + length > limit {
            return Err(INVALID);
        }
        output.extend(sequence[..length].iter().rev().copied());
        if let Some(previous) = previous.filter(|_| next < DICTIONARY_SIZE) {
            prefix[next] = previous as u16;
            suffix[next] = entry as u8;
            next += 1;
            width = pdf_lzw_code_width(width, next, early_change);
        }
        previous = Some(code);
    }
}

fn decode_ascii_hex(source: &[u8]) -> Result<Vec<u8>, &'static str> {
    let digits = source
        .iter()
        .copied()
        .filter(|byte| !pdf_whitespace(*byte))
        .map(char::from)
        .collect::<String>();
    let digits = digits.strip_suffix('>').unwrap_or(&digits);
    if !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("invalid ASCIIHexDecode PDF stream");
    }
    Ok(hex_bytes(digits))
}

fn decode_ascii85(source: &[u8], limit: usize) -> Result<Vec<u8>, &'static str> {
    let digits = source
        .iter()
        .copied()
        .filter(|byte| !pdf_whitespace(*byte))
        .collect::<Vec<_>>();
    let digits = digits.strip_prefix(b"<~").unwrap_or(&digits);
    let digits = digits.strip_suffix(b"~>").unwrap_or(digits);
    let mut result = Vec::new();
    let mut value = 0u64;
    let mut count = 0;
    let mut append = |value, count| {
        let value =
            u32::try_from(value).map_err(|_| "invalid or oversized ASCII85Decode PDF stream")?;
        if result.len() + count > limit + 1 {
            return Err("invalid or oversized ASCII85Decode PDF stream");
        }
        result.extend_from_slice(&value.to_be_bytes()[..count]);
        Ok(())
    };
    for &digit in digits {
        if digit == b'z' && count == 0 {
            append(0, 4)?;
        } else {
            if !(b'!'..=b'u').contains(&digit) {
                return Err("invalid ASCII85Decode PDF stream");
            }
            value = value * 85 + u64::from(digit - 33);
            count += 1;
            if count == 5 {
                append(value, 4)?;
                value = 0;
                count = 0;
            }
        }
    }
    if count == 1 {
        return Err("incomplete ASCII85Decode PDF stream");
    }
    if count > 1 {
        for _ in count..5 {
            value = value * 85 + 84;
        }
        append(value, count - 1)?;
    }
    Ok(result)
}
