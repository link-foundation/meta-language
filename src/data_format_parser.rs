use crate::{
    ByteRange, LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration, Point,
    SourceSpan,
};

pub fn parse(text: &str, language: &str, configuration: ParseConfiguration) -> Option<LinkNetwork> {
    let language = canonical_language(language)?;
    let (mut network, document) = LinkNetwork::new_parse_document(text, language);
    parse_into(
        &mut network,
        document,
        text,
        language,
        SpanOffset::zero(),
        configuration,
    );
    network.attach_embedded_regions(document, text, language, configuration);
    Some(network)
}

pub fn parse_embedded_region_into(
    network: &mut LinkNetwork,
    region: LinkId,
    text: &str,
    language: &str,
    span: SourceSpan,
    configuration: ParseConfiguration,
) -> Option<LinkId> {
    let language = canonical_language(language)?;
    Some(parse_into(
        network,
        region,
        text,
        language,
        SpanOffset::new(span.byte_range().start(), span.start_point()),
        configuration,
    ))
}

fn canonical_language(language: &str) -> Option<&'static str> {
    if language.eq_ignore_ascii_case("csv") {
        Some("CSV")
    } else if language.eq_ignore_ascii_case("json5") {
        Some("JSON5")
    } else {
        None
    }
}

fn parse_into(
    network: &mut LinkNetwork,
    owner: LinkId,
    text: &str,
    language: &str,
    offset: SpanOffset,
    configuration: ParseConfiguration,
) -> LinkId {
    let context = ParseContext {
        text,
        language,
        offset,
        configuration,
    };
    let is_valid = match language {
        "CSV" => validate_csv(text),
        "JSON5" => json5_nodes::parse(text).is_ok(),
        _ => true,
    };
    let root = insert_syntax(
        network,
        owner,
        root_term(language),
        ByteRange::new(0, text.len()),
        if is_valid {
            LinkFlags::clean()
        } else {
            LinkFlags::containing_error()
        },
        &context,
    );

    match language {
        "CSV" => insert_csv_records(network, root, &context),
        "JSON5" => {
            let lexical_error = insert_json5_tokens(network, root, &context);
            if lexical_error {
                network.set_flags(root, LinkFlags::containing_error());
            }
        }
        _ => {}
    }

    root
}

const fn root_term(language: &str) -> &'static str {
    match language.as_bytes() {
        b"CSV" => "csv_file",
        _ => "document",
    }
}

fn validate_csv(text: &str) -> bool {
    if has_unclosed_csv_quote(text) {
        return false;
    }

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(text.as_bytes());
    reader.byte_records().all(|record| record.is_ok())
}

fn has_unclosed_csv_quote(text: &str) -> bool {
    let mut field_start = 0;
    let mut index = 0;
    let mut in_quotes = false;

    while index < text.len() {
        let character = next_char(text, index);
        let character_len = character.len_utf8();

        if in_quotes {
            if character == '"' {
                let next_index = index + character_len;
                if text[next_index..].starts_with('"') {
                    index = next_index + 1;
                    continue;
                }
                in_quotes = false;
            }
            index += character_len;
            continue;
        }

        match character {
            '"' if index == field_start => {
                in_quotes = true;
                index += character_len;
            }
            ',' | '\n' => {
                index += character_len;
                field_start = index;
            }
            '\r' => {
                index = if text[index + character_len..].starts_with('\n') {
                    index + character_len + 1
                } else {
                    index + character_len
                };
                field_start = index;
            }
            _ => {
                index += character_len;
            }
        }
    }

    in_quotes
}

fn insert_csv_records(network: &mut LinkNetwork, root: LinkId, context: &ParseContext<'_>) {
    for record in collect_csv_records(context.text) {
        let record_link = insert_syntax(
            network,
            root,
            "record",
            ByteRange::new(record.start, record.end),
            LinkFlags::clean(),
            context,
        );

        for element in record.elements {
            match element.kind {
                CsvElementKind::Field => {
                    let field = insert_syntax(
                        network,
                        record_link,
                        "field",
                        ByteRange::new(element.start, element.end),
                        LinkFlags::clean(),
                        context,
                    );
                    insert_token_if_non_empty(
                        network,
                        field,
                        ByteRange::new(element.start, element.end),
                        LinkFlags::clean(),
                        true,
                        context,
                    );
                }
                CsvElementKind::Delimiter => insert_token_if_non_empty(
                    network,
                    record_link,
                    ByteRange::new(element.start, element.end),
                    LinkFlags::clean(),
                    false,
                    context,
                ),
                CsvElementKind::LineBreak => insert_token_if_non_empty(
                    network,
                    record_link,
                    ByteRange::new(element.start, element.end),
                    LinkFlags::extra(),
                    false,
                    context,
                ),
            }
        }
    }
}

fn collect_csv_records(text: &str) -> Vec<CsvRecord> {
    let mut records = Vec::new();
    let mut current = CsvRecord {
        start: 0,
        end: 0,
        elements: Vec::new(),
    };
    let mut field_start = 0;
    let mut index = 0;
    let mut in_quotes = false;

    while index < text.len() {
        let character = next_char(text, index);
        let character_len = character.len_utf8();

        if in_quotes {
            if character == '"' {
                let next_index = index + character_len;
                if text[next_index..].starts_with('"') {
                    index = next_index + 1;
                    continue;
                }
                in_quotes = false;
            }
            index += character_len;
            continue;
        }

        match character {
            '"' if index == field_start => {
                in_quotes = true;
                index += character_len;
            }
            ',' => {
                push_csv_field_and_separator(
                    &mut current,
                    field_start,
                    index,
                    index,
                    index + character_len,
                    CsvElementKind::Delimiter,
                );
                field_start = index + character_len;
                index += character_len;
            }
            '\n' => {
                finish_csv_record(
                    &mut records,
                    &mut current,
                    field_start,
                    index,
                    index,
                    index + character_len,
                );
                index += character_len;
                field_start = index;
            }
            '\r' => {
                let line_break_end = if text[index + character_len..].starts_with('\n') {
                    index + character_len + 1
                } else {
                    index + character_len
                };
                finish_csv_record(
                    &mut records,
                    &mut current,
                    field_start,
                    index,
                    index,
                    line_break_end,
                );
                index = line_break_end;
                field_start = index;
            }
            _ => {
                index += character_len;
            }
        }
    }

    if current.start < text.len() || field_start < text.len() {
        current.elements.push(CsvElement {
            kind: CsvElementKind::Field,
            start: field_start,
            end: text.len(),
        });
        current.end = text.len();
        records.push(current);
    }

    records
}

fn push_csv_field_and_separator(
    record: &mut CsvRecord,
    field_start: usize,
    field_end: usize,
    separator_start: usize,
    separator_end: usize,
    separator_kind: CsvElementKind,
) {
    record.elements.push(CsvElement {
        kind: CsvElementKind::Field,
        start: field_start,
        end: field_end,
    });
    record.elements.push(CsvElement {
        kind: separator_kind,
        start: separator_start,
        end: separator_end,
    });
}

fn finish_csv_record(
    records: &mut Vec<CsvRecord>,
    current: &mut CsvRecord,
    field_start: usize,
    field_end: usize,
    line_break_start: usize,
    line_break_end: usize,
) {
    push_csv_field_and_separator(
        current,
        field_start,
        field_end,
        line_break_start,
        line_break_end,
        CsvElementKind::LineBreak,
    );
    current.end = line_break_end;
    records.push(std::mem::replace(
        current,
        CsvRecord {
            start: line_break_end,
            end: line_break_end,
            elements: Vec::new(),
        },
    ));
}

fn insert_json5_tokens(
    network: &mut LinkNetwork,
    root: LinkId,
    context: &ParseContext<'_>,
) -> bool {
    let mut index = 0;
    let mut has_error = false;
    while index < context.text.len() {
        let character = next_char(context.text, index);

        if character.is_whitespace() {
            let end = consume_while(context.text, index, char::is_whitespace);
            insert_token(
                network,
                root,
                ByteRange::new(index, end),
                LinkFlags::extra(),
                false,
                context,
            );
            index = end;
        } else if context.text[index..].starts_with("//") {
            let end = context.text[index..]
                .find('\n')
                .map_or(context.text.len(), |relative| index + relative);
            insert_token(
                network,
                root,
                ByteRange::new(index, end),
                LinkFlags::extra(),
                false,
                context,
            );
            index = end;
        } else if context.text[index..].starts_with("/*") {
            let (end, flags) = context.text[index + 2..].find("*/").map_or_else(
                || {
                    has_error = true;
                    (context.text.len(), LinkFlags::error())
                },
                |relative| (index + 2 + relative + 2, LinkFlags::extra()),
            );
            insert_token(
                network,
                root,
                ByteRange::new(index, end),
                flags,
                false,
                context,
            );
            index = end;
        } else if character == '"' || character == '\'' {
            let (end, closed) = consume_quoted_json5_string(context.text, index, character);
            let flags = if closed {
                LinkFlags::clean()
            } else {
                has_error = true;
                LinkFlags::error()
            };
            insert_token(
                network,
                root,
                ByteRange::new(index, end),
                flags,
                true,
                context,
            );
            index = end;
        } else if is_json5_punctuation(character) {
            let end = index + character.len_utf8();
            insert_token(
                network,
                root,
                ByteRange::new(index, end),
                LinkFlags::clean(),
                false,
                context,
            );
            index = end;
        } else {
            let end = consume_json5_atom(context.text, index);
            insert_token(
                network,
                root,
                ByteRange::new(index, end),
                LinkFlags::clean(),
                true,
                context,
            );
            index = end;
        }
    }
    has_error
}

fn consume_quoted_json5_string(text: &str, start: usize, quote: char) -> (usize, bool) {
    let mut index = start + quote.len_utf8();
    let mut escaped = false;
    while index < text.len() {
        let character = next_char(text, index);
        index += character.len_utf8();
        if escaped {
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character == quote {
            return (index, true);
        }
    }
    (text.len(), false)
}

fn consume_json5_atom(text: &str, start: usize) -> usize {
    let mut index = start;
    while index < text.len() {
        let character = next_char(text, index);
        if character.is_whitespace()
            || is_json5_punctuation(character)
            || text[index..].starts_with("//")
            || text[index..].starts_with("/*")
        {
            break;
        }
        index += character.len_utf8();
    }
    index
}

const fn is_json5_punctuation(character: char) -> bool {
    matches!(character, '{' | '}' | '[' | ']' | ':' | ',')
}

fn consume_while(text: &str, start: usize, predicate: impl Fn(char) -> bool) -> usize {
    let mut index = start;
    while index < text.len() {
        let character = next_char(text, index);
        if !predicate(character) {
            break;
        }
        index += character.len_utf8();
    }
    index
}

fn insert_syntax(
    network: &mut LinkNetwork,
    owner: LinkId,
    term: &str,
    range: ByteRange,
    flags: LinkFlags,
    context: &ParseContext<'_>,
) -> LinkId {
    network.insert_link(
        [owner],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term(term)
            .with_language(context.language)
            .with_span(span_for_range(context.text, range, context.offset))
            .with_flags(flags),
    )
}

fn insert_token_if_non_empty(
    network: &mut LinkNetwork,
    owner: LinkId,
    range: ByteRange,
    flags: LinkFlags,
    named: bool,
    context: &ParseContext<'_>,
) {
    if range.start() == range.end() {
        return;
    }
    let token = insert_token(network, owner, range, flags, named, context);
    if flags.is_extra() {
        network.attach_trivia(
            owner,
            token,
            span_for_range(context.text, range, context.offset),
            context.configuration.trivia_attachment_policy(),
        );
    }
}

fn insert_token(
    network: &mut LinkNetwork,
    owner: LinkId,
    range: ByteRange,
    flags: LinkFlags,
    named: bool,
    context: &ParseContext<'_>,
) -> LinkId {
    network.insert_link(
        [owner],
        LinkMetadata::new()
            .with_link_type(LinkType::Token)
            .with_named(named)
            .with_term(&context.text[range.start()..range.end()])
            .with_language(context.language)
            .with_span(span_for_range(context.text, range, context.offset))
            .with_flags(flags),
    )
}

fn span_for_range(text: &str, range: ByteRange, offset: SpanOffset) -> SourceSpan {
    SourceSpan::new(
        ByteRange::new(offset.byte + range.start(), offset.byte + range.end()),
        offset.point(point_at_byte(text, range.start())),
        offset.point(point_at_byte(text, range.end())),
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

fn next_char(text: &str, index: usize) -> char {
    text[index..]
        .chars()
        .next()
        .expect("index is inside source text")
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CsvRecord {
    start: usize,
    end: usize,
    elements: Vec<CsvElement>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CsvElement {
    kind: CsvElementKind,
    start: usize,
    end: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CsvElementKind {
    Field,
    Delimiter,
    LineBreak,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ParseContext<'source> {
    text: &'source str,
    language: &'source str,
    offset: SpanOffset,
    configuration: ParseConfiguration,
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
