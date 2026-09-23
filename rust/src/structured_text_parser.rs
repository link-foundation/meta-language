use unicode_segmentation::UnicodeSegmentation;

use crate::line_index::LineIndex;
use crate::natural_language::annotate_natural_language;
use crate::{
    ByteRange, LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration,
    SourceSpan,
};

pub fn parse_plain(text: &str, language: &str, configuration: ParseConfiguration) -> LinkNetwork {
    parse_lines(
        text,
        language,
        configuration,
        "text_document",
        |_| ("line", false),
        balanced_parentheses(text),
    )
}

pub fn parse_lino(text: &str, language: &str, configuration: ParseConfiguration) -> LinkNetwork {
    parse_lines(
        text,
        language,
        configuration,
        "lino_document",
        lino_line,
        balanced_parentheses(text),
    )
}

pub fn parse_pdf(text: &str, language: &str, configuration: ParseConfiguration) -> LinkNetwork {
    parse_lines(
        text,
        language,
        configuration,
        "pdf_file",
        pdf_line,
        valid_pdf_container(text),
    )
}

pub fn parse_natural(text: &str, language: &str, configuration: ParseConfiguration) -> LinkNetwork {
    let (mut network, document) = LinkNetwork::new_parse_document(text, language);
    let lines = LineIndex::new(text);
    let root_span = span_for_range(&lines, 0, text.len());
    let root = insert_syntax(
        &mut network,
        document,
        "natural_language_document",
        language,
        root_span,
        LinkFlags::clean(),
    );

    for (start, sentence) in text.split_sentence_bound_indices() {
        let end = start + sentence.len();
        let sentence_node = insert_syntax(
            &mut network,
            root,
            "sentence",
            language,
            span_for_range(&lines, start, end),
            LinkFlags::clean(),
        );
        insert_lexical_nodes(
            &mut network,
            sentence_node,
            text,
            start,
            end,
            language,
            &lines,
            configuration,
        );
    }

    network.attach_embedded_regions(document, text, language, configuration);
    annotate_natural_language(&mut network, document, text, language, configuration);
    network
}

fn parse_lines(
    text: &str,
    language: &str,
    configuration: ParseConfiguration,
    root_term: &str,
    classify_line: fn(&str) -> (&'static str, bool),
    document_is_valid: bool,
) -> LinkNetwork {
    let (mut network, document) = LinkNetwork::new_parse_document(text, language);
    let lines = LineIndex::new(text);
    let root_span = span_for_range(&lines, 0, text.len());
    let mut classified = Vec::new();
    let mut start = 0;
    for line in text.split_inclusive('\n') {
        let end = start + line.len();
        classified.push((start, end, classify_line(line)));
        start = end;
    }
    if start < text.len() || text.is_empty() {
        classified.push((start, text.len(), classify_line(&text[start..])));
    }
    let has_line_error = classified.iter().any(|(_, _, (_, error))| *error);
    let root_flags = if document_is_valid && !has_line_error {
        LinkFlags::clean()
    } else {
        LinkFlags::containing_error()
    };
    let root = insert_syntax(
        &mut network,
        document,
        root_term,
        language,
        root_span,
        root_flags,
    );

    for (start, end, (term, error)) in classified {
        let flags = if error {
            LinkFlags::error()
        } else {
            LinkFlags::clean()
        };
        let line = insert_syntax(
            &mut network,
            root,
            term,
            language,
            span_for_range(&lines, start, end),
            flags,
        );
        insert_lexical_nodes(
            &mut network,
            line,
            text,
            start,
            end,
            language,
            &lines,
            configuration,
        );
    }
    network.attach_embedded_regions(document, text, language, configuration);
    network
}

#[allow(clippy::too_many_arguments)]
fn insert_lexical_nodes(
    network: &mut LinkNetwork,
    parent: LinkId,
    text: &str,
    start: usize,
    end: usize,
    language: &str,
    lines: &LineIndex,
    configuration: ParseConfiguration,
) {
    for (relative_start, segment) in text[start..end].split_word_bound_indices() {
        let segment_start = start + relative_start;
        let segment_end = segment_start + segment.len();
        let whitespace = segment.chars().all(char::is_whitespace);
        let term = if whitespace {
            "whitespace"
        } else if segment.chars().any(char::is_alphanumeric) {
            "word"
        } else {
            "punctuation"
        };
        let span = span_for_range(lines, segment_start, segment_end);
        let flags = if whitespace {
            LinkFlags::extra()
        } else {
            LinkFlags::clean()
        };
        let syntax = insert_syntax(network, parent, term, language, span, flags);
        let token = network.insert_link(
            [syntax],
            LinkMetadata::new()
                .with_link_type(LinkType::Token)
                .with_named(!whitespace)
                .with_term(segment)
                .with_language(language)
                .with_span(span)
                .with_flags(flags),
        );
        if whitespace {
            network.attach_trivia(
                parent,
                token,
                span,
                configuration.trivia_attachment_policy(),
            );
        }
    }
}

fn insert_syntax(
    network: &mut LinkNetwork,
    parent: LinkId,
    term: &str,
    language: &str,
    span: SourceSpan,
    flags: LinkFlags,
) -> LinkId {
    network.insert_link(
        [parent],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term(term)
            .with_language(language)
            .with_span(span)
            .with_flags(flags),
    )
}

fn lino_line(line: &str) -> (&'static str, bool) {
    let trimmed = line.trim();
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        ("link", false)
    } else if trimmed.starts_with('(') || trimmed.ends_with(')') {
        ("lino_error", true)
    } else if trimmed.ends_with(':') && !trimmed.starts_with(':') {
        ("definition", false)
    } else if line.starts_with(char::is_whitespace) {
        ("definition_body", false)
    } else if trimmed.split_whitespace().count() > 1 {
        ("link", false)
    } else if trimmed.is_empty() {
        ("blank_line", false)
    } else {
        ("atom", false)
    }
}

fn pdf_line(line: &str) -> (&'static str, bool) {
    let trimmed = line.trim();
    if trimmed.starts_with("%PDF-") {
        ("header", false)
    } else if trimmed == "%%EOF" {
        ("end_of_file", false)
    } else if is_pdf_object_header(trimmed) {
        ("object_header", false)
    } else if trimmed == "endobj" {
        ("object_end", false)
    } else if trimmed == "xref" {
        ("cross_reference_table", false)
    } else if trimmed == "trailer" {
        ("trailer", false)
    } else if matches!(trimmed, "stream" | "endstream") {
        ("stream_boundary", false)
    } else {
        ("pdf_line", false)
    }
}

fn is_pdf_object_header(text: &str) -> bool {
    let mut parts = text.split_whitespace();
    parts.next().is_some_and(|part| part.parse::<u64>().is_ok())
        && parts.next().is_some_and(|part| part.parse::<u64>().is_ok())
        && parts.next() == Some("obj")
        && parts.next().is_none()
}

fn valid_pdf_container(text: &str) -> bool {
    let header = text
        .lines()
        .next()
        .is_some_and(|line| line.starts_with("%PDF-"));
    header && text.trim_end().ends_with("%%EOF")
}

fn balanced_parentheses(text: &str) -> bool {
    let mut depth = 0_usize;
    for character in text.chars() {
        match character {
            '(' => depth += 1,
            ')' if depth == 0 => return false,
            ')' => depth -= 1,
            _ => {}
        }
    }
    depth == 0
}

fn span_for_range(lines: &LineIndex, start: usize, end: usize) -> SourceSpan {
    SourceSpan::new(
        ByteRange::new(start, end),
        lines.char_point(start),
        lines.char_point(end),
    )
}
