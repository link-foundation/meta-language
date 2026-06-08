use crate::{
    ByteRange, LinkId, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration, Point, SourceSpan,
};

pub fn parse(text: &str, language: &str, configuration: ParseConfiguration) -> LinkNetwork {
    let mut network = LinkNetwork::parse_lossless_text(text, language, configuration);
    Parser::new(&mut network, text, language).parse_document();
    network
}

struct Parser<'a> {
    network: &'a mut LinkNetwork,
    text: &'a str,
    language: &'a str,
    cursor: usize,
}

impl<'a> Parser<'a> {
    const fn new(network: &'a mut LinkNetwork, text: &'a str, language: &'a str) -> Self {
        Self {
            network,
            text,
            language,
            cursor: 0,
        }
    }

    fn parse_document(&mut self) {
        while self.cursor < self.text.len() {
            self.skip_horizontal_and_newline_whitespace();
            if self.cursor >= self.text.len() {
                break;
            }

            if self.peek_byte() == Some(b'(') {
                let _ = self.parse_expression();
            } else {
                self.parse_line_form();
            }
        }
    }

    fn parse_line_form(&mut self) {
        let line_start = self.cursor;
        let line_end = self.line_end(self.cursor);
        let line = &self.text[line_start..line_end];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            self.cursor = self.next_line_start(line_end);
            return;
        }

        if !line.starts_with(char::is_whitespace) && trimmed.ends_with(':') {
            self.parse_indented_definition(line_start, line_end, trimmed);
            return;
        }

        let references =
            parse_line_references(self.network, self.text, self.language, line_start, line_end);
        if references.len() > 1 {
            self.insert_relation(&references, None, line_start, line_end);
        }
        self.cursor = self.next_line_start(line_end);
    }

    fn parse_indented_definition(&mut self, line_start: usize, line_end: usize, trimmed: &str) {
        let name = trimmed.trim_end_matches(':').trim();
        let mut child_start = self.next_line_start(line_end);
        let mut definition_end = line_end;
        let mut references = Vec::new();

        while child_start < self.text.len() {
            let child_end = self.line_end(child_start);
            let child_line = &self.text[child_start..child_end];
            if !child_line.starts_with(char::is_whitespace) {
                break;
            }

            references.extend(parse_line_references(
                self.network,
                self.text,
                self.language,
                child_start,
                child_end,
            ));
            definition_end = child_end;
            child_start = self.next_line_start(child_end);
        }

        self.insert_relation(&references, Some(name), line_start, definition_end);
        self.cursor = child_start;
    }

    fn parse_expression(&mut self) -> Option<LinkId> {
        self.skip_inline_whitespace();
        match self.peek_byte()? {
            b'(' => Some(self.parse_parenthesized_relation()),
            b')' => None,
            _ => self.parse_atom_reference().map(|(id, _span)| id),
        }
    }

    fn parse_parenthesized_relation(&mut self) -> LinkId {
        let start = self.cursor;
        self.cursor += 1;
        self.skip_inline_whitespace();

        let mut references = Vec::new();
        let mut relation_id = None;

        if self.peek_byte() != Some(b')') {
            if let Some((candidate, candidate_span)) = self.parse_atom_text() {
                self.skip_inline_whitespace();
                if self.peek_byte() == Some(b':') {
                    self.cursor += 1;
                    let id = self.insert_relation(
                        &[],
                        Some(candidate),
                        start,
                        candidate_span.byte_range().end(),
                    );
                    relation_id = Some(id);
                } else {
                    references.push(self.reference_for_atom(candidate));
                }
            }
        }

        loop {
            self.skip_inline_whitespace();
            match self.peek_byte() {
                Some(b')') => {
                    self.cursor += 1;
                    break;
                }
                Some(_) => {
                    if let Some(reference) = self.parse_expression() {
                        references.push(reference);
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        let end = self.cursor;
        if let Some(id) = relation_id {
            self.network.set_references(id, &references);
            self.network.set_span(id, self.span(start, end));
            id
        } else {
            self.insert_relation(&references, None, start, end)
        }
    }

    fn parse_atom_reference(&mut self) -> Option<(LinkId, SourceSpan)> {
        let (atom, span) = self.parse_atom_text()?;
        Some((self.reference_for_atom(atom), span))
    }

    fn parse_atom_text(&mut self) -> Option<(&'a str, SourceSpan)> {
        self.skip_inline_whitespace();
        let start = self.cursor;
        while self.cursor < self.text.len() {
            let byte = self.text.as_bytes()[self.cursor];
            if byte.is_ascii_whitespace() || matches!(byte, b'(' | b')' | b':') {
                break;
            }
            self.cursor += 1;
        }

        (start != self.cursor).then(|| {
            let span = self.span(start, self.cursor);
            (&self.text[start..self.cursor], span)
        })
    }

    fn reference_for_atom(&mut self, atom: &str) -> LinkId {
        self.network.find_term(atom).unwrap_or_else(|| {
            self.network
                .insert_typed_point(atom, LinkType::Concept, None)
        })
    }

    fn insert_relation(
        &mut self,
        references: &[LinkId],
        name: Option<&str>,
        start: usize,
        end: usize,
    ) -> LinkId {
        let mut metadata = LinkMetadata::new()
            .with_link_type(LinkType::Relation)
            .with_named(name.is_some())
            .with_language(self.language)
            .with_span(self.span(start, end));
        if let Some(name) = name {
            metadata = metadata.with_term(name);
        }
        self.network.insert_dynamic_link(references, metadata)
    }

    fn skip_inline_whitespace(&mut self) {
        while self
            .peek_byte()
            .is_some_and(|byte| byte.is_ascii_whitespace() && byte != b'\n' && byte != b'\r')
        {
            self.cursor += 1;
        }
    }

    fn skip_horizontal_and_newline_whitespace(&mut self) {
        while self
            .peek_byte()
            .is_some_and(|byte| byte.is_ascii_whitespace())
        {
            self.cursor += 1;
        }
    }

    fn peek_byte(&self) -> Option<u8> {
        self.text.as_bytes().get(self.cursor).copied()
    }

    fn line_end(&self, start: usize) -> usize {
        self.text[start..]
            .find('\n')
            .map_or(self.text.len(), |offset| start + offset)
    }

    fn next_line_start(&self, line_end: usize) -> usize {
        if self.text.as_bytes().get(line_end) == Some(&b'\n') {
            line_end + 1
        } else {
            line_end
        }
    }

    fn span(&self, start: usize, end: usize) -> SourceSpan {
        SourceSpan::new(
            ByteRange::new(start, end),
            point_at_byte(self.text, start),
            point_at_byte(self.text, end),
        )
    }
}

fn parse_line_references(
    network: &mut LinkNetwork,
    text: &str,
    language: &str,
    start: usize,
    end: usize,
) -> Vec<LinkId> {
    let mut parser = Parser::new(network, text, language);
    parser.cursor = start;
    let mut references = Vec::new();

    while parser.cursor < end {
        parser.skip_inline_whitespace();
        if parser.cursor >= end {
            break;
        }
        if parser.peek_byte() == Some(b'(') {
            if let Some(reference) = parser.parse_expression() {
                references.push(reference);
            }
        } else if let Some((reference, _span)) = parser.parse_atom_reference() {
            references.push(reference);
        } else {
            break;
        }
    }

    references
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
