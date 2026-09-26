//! The built-in PDF grammar CST, shared with `js/src/pdf-grammar.js`.
//!
//! The grammar follows the COS syntax of ISO 32000-2 section 7 (lexical
//! conventions, objects and file structure):
//!
//! - `pdf_file` holds the `header` (`%PDF-x.y`), `indirect_object`,
//!   `cross_reference_table`, `trailer`, `start_cross_reference` and
//!   `end_of_file` (`%%EOF`) items of the body and every incremental update.
//! - Objects are `dictionary` (with `dictionary_entry` key/value pairs),
//!   `array`, `indirect_reference`, and the `integer`, `real`, `boolean`,
//!   `null`, `name`, `literal_string` and `hex_string` tokens.
//! - A `stream` joins its `dictionary` to its `data`: a `content_stream` of
//!   `operation` and `inline_image` nodes when an unfiltered page, form or
//!   pattern stream parses cleanly as content, and a `stream_data` token
//!   otherwise. The stream extent comes from a direct `/Length` that lands on
//!   `endstream`, or else from the next `endstream` keyword.
//! - Comments are named `comment` extras and PDF whitespace (NUL, TAB, LF,
//!   FF, CR, SP) anonymous `whitespace` extras.
//! - Malformed input is kept in `ERROR` nodes, and absent delimiters,
//!   keywords, the header and the `%%EOF` marker as zero-width missing nodes.
//!
//! Offsets are UTF-8 byte offsets; every delimiter is ASCII and a `/Length`
//! is counted in bytes, so the JavaScript parser computes the same tree over
//! string indices.

use crate::builtin_grammar::{fill_extras, propagate_errors, GrammarNode};

/// Parses `text` as a PDF file into a lossless CST.
pub fn parse_pdf_cst(text: &str) -> GrammarNode {
    PdfGrammarParser {
        text,
        bytes: text.as_bytes(),
        position: 0,
        limit: text.len(),
        content: false,
    }
    .file()
}

// Keywords of the COS file structure, kept as anonymous tokens.
const COS_KEYWORDS: [&str; 10] = [
    "obj",
    "endobj",
    "stream",
    "endstream",
    "R",
    "xref",
    "trailer",
    "startxref",
    "n",
    "f",
];

const VALUE_TOKENS: [&str; 7] = [
    "integer",
    "real",
    "boolean",
    "null",
    "name",
    "literal_string",
    "hex_string",
];

// Dictionary keys of the page content, form XObject and pattern streams whose
// data is parsed as a content stream.
const CONTENT_STREAM_KEYS: [&str; 22] = [
    "/Length",
    "/Type",
    "/Subtype",
    "/FormType",
    "/BBox",
    "/Matrix",
    "/Resources",
    "/Group",
    "/Ref",
    "/Metadata",
    "/PieceInfo",
    "/LastModified",
    "/StructParent",
    "/StructParents",
    "/OPI",
    "/OC",
    "/Name",
    "/PatternType",
    "/PaintType",
    "/TilingType",
    "/XStep",
    "/YStep",
];

// The largest integer a JavaScript number holds exactly, bounding `/Length`.
const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

#[derive(Clone, Copy)]
struct Token<'a> {
    kind: &'static str,
    start: usize,
    end: usize,
    text: &'a str,
}

struct PdfGrammarParser<'a> {
    text: &'a str,
    bytes: &'a [u8],
    position: usize,
    limit: usize,
    content: bool,
}

impl<'a> PdfGrammarParser<'a> {
    // pdf_file = header (item | end_of_file)*
    fn file(mut self) -> GrammarNode {
        let mut children = Vec::new();
        while is_white(self.byte(self.position)) {
            self.position += 1;
        }
        let header_end = (self.byte(self.position) == Some(b'%'))
            .then(|| self.comment_end(self.position, self.limit))
            .filter(|end| is_header(&self.bytes[self.position..*end]));
        if let Some(end) = header_end {
            children.push(GrammarNode::node("header", self.position, end, Vec::new()));
            self.position = end;
        } else {
            children.push(GrammarNode::missing("header", true, 0));
        }
        loop {
            self.skip_trivia();
            if self.position >= self.limit {
                break;
            }
            if self.at_end_of_file_marker() {
                children.push(GrammarNode::node(
                    "end_of_file",
                    self.position,
                    self.position + 5,
                    Vec::new(),
                ));
                self.position += 5;
                continue;
            }
            let item = self
                .indirect_object()
                .or_else(|| self.cross_reference_table())
                .or_else(|| self.trailer())
                .or_else(|| self.start_cross_reference());
            let item = item.unwrap_or_else(|| self.error_until(Self::at_sync));
            children.push(item);
        }
        let last = children.last().expect("the header or its missing node");
        if last.term != "end_of_file" {
            let end = last.end;
            children.push(GrammarNode::missing("end_of_file", true, end));
        }
        let mut root = GrammarNode::node("pdf_file", 0, self.text.len(), children);
        propagate_errors(&mut root);
        fill_extras(root, &mut |start, end| self.lex_gap(start, end))
    }

    // indirect_object = integer integer "obj" (object | stream)? "endobj"
    fn indirect_object(&mut self) -> Option<GrammarNode> {
        let [Some(number), Some(generation), Some(keyword)] = self.peek_tokens() else {
            return None;
        };
        if !is_unsigned(number) || !is_unsigned(generation) || !is_keyword(keyword, "obj") {
            return None;
        }
        let mut children = vec![
            leaf_for(number).with_field("object_number"),
            leaf_for(generation).with_field("generation"),
            GrammarNode::anonymous("obj", keyword.start, keyword.end),
        ];
        self.position = keyword.end;
        self.skip_trivia();
        let mut value = self.object();
        if let Some(found) = value.take() {
            self.skip_trivia();
            value = Some(if found.term == "dictionary" && self.at_keyword("stream") {
                self.stream(found)
            } else {
                found
            });
        }
        children.push(
            value
                .unwrap_or_else(|| GrammarNode::missing("null", true, keyword.end))
                .with_field("value"),
        );
        loop {
            self.skip_trivia();
            if self.at_keyword("endobj") {
                children.push(GrammarNode::anonymous(
                    "endobj",
                    self.position,
                    self.position + 6,
                ));
                self.position += 6;
                break;
            }
            if self.position >= self.limit || self.at_sync() {
                let end = last_end(&children);
                children.push(GrammarNode::missing("endobj", false, end));
                break;
            }
            let error = self.error_until(|parser| parser.at_keyword("endobj") || parser.at_sync());
            push_error(&mut children, error, false);
        }
        Some(GrammarNode::spanning("indirect_object", children))
    }

    // stream = dictionary "stream" EOL data? EOL? "endstream"
    fn stream(&mut self, dictionary: GrammarNode) -> GrammarNode {
        let keyword_end = self.position + 6;
        let keyword = GrammarNode::anonymous("stream", self.position, keyword_end);
        let mut data_start = keyword_end;
        if self.bytes[data_start..].starts_with(b"\r\n") {
            data_start += 2;
        } else if matches!(self.byte(data_start), Some(b'\n' | b'\r')) {
            data_start += 1;
        }
        let (data_end, endstream) = self.stream_extent(&dictionary, data_start);
        let data = (data_end > data_start).then(|| {
            self.stream_data(&dictionary, data_start, data_end)
                .with_field("data")
        });
        let mut children = vec![dictionary.with_field("dictionary"), keyword];
        children.extend(data);
        if let Some(endstream) = endstream {
            children.push(GrammarNode::anonymous(
                "endstream",
                endstream,
                endstream + 9,
            ));
            self.position = endstream + 9;
        } else {
            children.push(GrammarNode::missing("endstream", false, data_end));
            self.position = data_end;
        }
        GrammarNode::spanning("stream", children)
    }

    // The end of the stream data and the start of `endstream`, if present.
    fn stream_extent(&self, dictionary: &GrammarNode, data_start: usize) -> (usize, Option<usize>) {
        if let Some(end) = direct_length(dictionary, self.text)
            .and_then(|length| data_start.checked_add(length))
            .filter(|end| self.text.is_char_boundary(*end))
        {
            let mut keyword = end;
            while is_white(self.byte(keyword)) {
                keyword += 1;
            }
            if self.at_keyword_at(keyword, "endstream") {
                return (end, Some(keyword));
            }
        }
        let mut keyword = find(self.bytes, b"endstream", data_start);
        while let Some(found) = keyword {
            if !is_regular(self.byte(found + 9)) {
                break;
            }
            keyword = find(self.bytes, b"endstream", found + 1);
        }
        let Some(keyword) = keyword else {
            return (self.text.len(), None);
        };
        let mut end = keyword;
        if self.byte(end - 1) == Some(b'\n') {
            end -= 1;
        }
        if self.byte(end - 1) == Some(b'\r') {
            end -= 1;
        }
        (end.max(data_start), Some(keyword))
    }

    fn stream_data(&mut self, dictionary: &GrammarNode, start: usize, end: usize) -> GrammarNode {
        is_content_stream_dictionary(dictionary, self.text)
            .then(|| self.content_stream(start, end))
            .flatten()
            .unwrap_or_else(|| GrammarNode::node("stream_data", start, end, Vec::new()))
    }

    // content_stream = (operation | inline_image)*, or None unless the whole
    // range parses cleanly.
    fn content_stream(&mut self, start: usize, end: usize) -> Option<GrammarNode> {
        let saved = (self.position, self.limit, self.content);
        (self.position, self.limit, self.content) = (start, end, true);
        let mut children = Vec::new();
        let mut operands = Vec::new();
        let mut clean = true;
        loop {
            self.skip_trivia();
            if self.position >= self.limit {
                break;
            }
            let token = self.lex(self.position);
            if is_keyword(token, "BI") && operands.is_empty() {
                let Some(image) = self.inline_image(token) else {
                    clean = false;
                    break;
                };
                children.push(image);
            } else if token.kind == "keyword" {
                let mut operation = std::mem::take(&mut operands)
                    .into_iter()
                    .map(|operand: GrammarNode| operand.with_field("operand"))
                    .collect::<Vec<_>>();
                operation.push(
                    GrammarNode::node("operator", token.start, token.end, Vec::new())
                        .with_field("operator"),
                );
                children.push(GrammarNode::spanning("operation", operation));
                self.position = token.end;
            } else {
                let Some(mut operand) = self.object() else {
                    clean = false;
                    break;
                };
                if propagate_errors(&mut operand) {
                    clean = false;
                    break;
                }
                operands.push(operand);
            }
        }
        (self.position, self.limit, self.content) = saved;
        (clean && operands.is_empty())
            .then(|| GrammarNode::node("content_stream", start, end, children))
    }

    // inline_image = "BI" dictionary_entry* "ID" WHITE image_data? WHITE "EI"
    fn inline_image(&mut self, begin: Token<'a>) -> Option<GrammarNode> {
        let mut children = vec![GrammarNode::anonymous("BI", begin.start, begin.end)];
        self.position = begin.end;
        loop {
            self.skip_trivia();
            if self.position >= self.limit {
                return None;
            }
            let token = self.lex(self.position);
            if is_keyword(token, "ID") {
                children.push(GrammarNode::anonymous("ID", token.start, token.end));
                self.position = token.end;
                break;
            }
            if token.kind != "name" {
                return None;
            }
            self.position = token.end;
            self.skip_trivia();
            let mut value = self.object()?;
            if propagate_errors(&mut value) {
                return None;
            }
            children.push(GrammarNode::spanning(
                "dictionary_entry",
                vec![leaf_for(token).with_field("key"), value.with_field("value")],
            ));
        }
        if !is_white(self.byte(self.position)) {
            return None;
        }
        let data_start = self.position + 1;
        let mut end = find(self.bytes, b"EI", data_start);
        while let Some(found) = end {
            if found + 2 > self.limit
                || (is_white(self.byte(found - 1))
                    && (found + 2 == self.limit || is_white(self.byte(found + 2))))
            {
                break;
            }
            end = find(self.bytes, b"EI", found + 1);
        }
        let end = end.filter(|end| end + 2 <= self.limit)?;
        if end - 1 > data_start {
            children.push(
                GrammarNode::node("image_data", data_start, end - 1, Vec::new()).with_field("data"),
            );
        }
        children.push(GrammarNode::anonymous("EI", end, end + 2));
        self.position = end + 2;
        Some(GrammarNode::spanning("inline_image", children))
    }

    // cross_reference_table = "xref" cross_reference_subsection*
    fn cross_reference_table(&mut self) -> Option<GrammarNode> {
        if !self.at_keyword("xref") {
            return None;
        }
        let mut children = vec![GrammarNode::anonymous(
            "xref",
            self.position,
            self.position + 4,
        )];
        self.position += 4;
        while let [Some(first), Some(count), next] = self.peek_tokens() {
            if !is_unsigned(first)
                || !is_unsigned(count)
                || next.is_some_and(|next| {
                    ["obj", "R", "n", "f"]
                        .iter()
                        .any(|keyword| is_keyword(next, keyword))
                })
            {
                break;
            }
            let mut subsection = vec![
                leaf_for(first).with_field("first_object"),
                leaf_for(count).with_field("count"),
            ];
            self.position = count.end;
            while let [Some(offset), Some(generation), Some(kind)] = self.peek_tokens() {
                let entry_type = match kind.text {
                    "n" => "n",
                    "f" => "f",
                    _ => break,
                };
                if !is_unsigned(offset) || !is_unsigned(generation) || kind.kind != "keyword" {
                    break;
                }
                subsection.push(GrammarNode::spanning(
                    "cross_reference_entry",
                    vec![
                        leaf_for(offset).with_field("offset"),
                        leaf_for(generation).with_field("generation"),
                        GrammarNode::anonymous(entry_type, kind.start, kind.end).with_field("type"),
                    ],
                ));
                self.position = kind.end;
            }
            children.push(GrammarNode::spanning(
                "cross_reference_subsection",
                subsection,
            ));
        }
        Some(GrammarNode::spanning("cross_reference_table", children))
    }

    // trailer = "trailer" dictionary
    fn trailer(&mut self) -> Option<GrammarNode> {
        if !self.at_keyword("trailer") {
            return None;
        }
        let keyword = GrammarNode::anonymous("trailer", self.position, self.position + 7);
        self.position += 7;
        self.skip_trivia();
        let dictionary = if self.at_delimiter("<<") {
            self.dictionary()
        } else {
            GrammarNode::missing("dictionary", true, keyword.end)
        };
        Some(GrammarNode::spanning(
            "trailer",
            vec![keyword, dictionary.with_field("dictionary")],
        ))
    }

    // start_cross_reference = "startxref" integer
    fn start_cross_reference(&mut self) -> Option<GrammarNode> {
        if !self.at_keyword("startxref") {
            return None;
        }
        let keyword = GrammarNode::anonymous("startxref", self.position, self.position + 9);
        self.position += 9;
        let [offset] = self.peek_tokens();
        let offset = if let Some(offset) = offset.filter(|offset| is_unsigned(*offset)) {
            self.position = offset.end;
            leaf_for(offset)
        } else {
            GrammarNode::missing("integer", true, keyword.end)
        };
        Some(GrammarNode::spanning(
            "start_cross_reference",
            vec![keyword, offset.with_field("offset")],
        ))
    }

    // object = dictionary | array | indirect_reference | value token, or None
    // (consuming nothing) where no object starts.
    fn object(&mut self) -> Option<GrammarNode> {
        if self.position >= self.limit || self.at_end_of_file_marker() {
            return None;
        }
        let token = self.lex(self.position);
        match token.kind {
            "<<" => return Some(self.dictionary()),
            "[" => return Some(self.array()),
            _ => {}
        }
        if is_unsigned(token) && !self.content {
            if let [_, Some(generation), Some(keyword)] = self.peek_tokens() {
                if is_unsigned(generation) && is_keyword(keyword, "R") {
                    self.position = keyword.end;
                    return Some(GrammarNode::spanning(
                        "indirect_reference",
                        vec![
                            leaf_for(token).with_field("object_number"),
                            leaf_for(generation).with_field("generation"),
                            GrammarNode::anonymous("R", keyword.start, keyword.end),
                        ],
                    ));
                }
            }
        }
        if !VALUE_TOKENS.contains(&token.kind) {
            return None;
        }
        self.position = token.end;
        Some(leaf_for(token))
    }

    // dictionary = "<<" dictionary_entry* ">>"
    fn dictionary(&mut self) -> GrammarNode {
        let mut children = vec![GrammarNode::anonymous(
            "<<",
            self.position,
            self.position + 2,
        )];
        self.position += 2;
        loop {
            self.skip_trivia();
            if self.at_delimiter(">>") {
                children.push(GrammarNode::anonymous(
                    ">>",
                    self.position,
                    self.position + 2,
                ));
                self.position += 2;
                break;
            }
            if self.at_terminator() || self.byte(self.position) == Some(b']') {
                let end = last_end(&children);
                children.push(GrammarNode::missing(">>", false, end));
                break;
            }
            let token = self.lex(self.position);
            if token.kind != "name" {
                let error = self.error_item();
                push_error(&mut children, error, false);
                continue;
            }
            self.position = token.end;
            self.skip_trivia();
            let key = leaf_for(token);
            let value = if self.at_dictionary_end() {
                None
            } else {
                self.object()
            };
            if let Some(value) = value {
                children.push(GrammarNode::spanning(
                    "dictionary_entry",
                    vec![key.with_field("key"), value.with_field("value")],
                ));
            } else {
                push_error(&mut children, key, false);
                if !self.at_dictionary_end() {
                    let error = self.error_item();
                    push_error(&mut children, error, false);
                }
            }
        }
        GrammarNode::spanning("dictionary", children)
    }

    fn at_dictionary_end(&mut self) -> bool {
        self.at_delimiter(">>") || self.byte(self.position) == Some(b']') || self.at_terminator()
    }

    // array = "[" object* "]"
    fn array(&mut self) -> GrammarNode {
        let mut children = vec![GrammarNode::anonymous(
            "[",
            self.position,
            self.position + 1,
        )];
        self.position += 1;
        loop {
            self.skip_trivia();
            if self.byte(self.position) == Some(b']') {
                children.push(GrammarNode::anonymous(
                    "]",
                    self.position,
                    self.position + 1,
                ));
                self.position += 1;
                break;
            }
            if self.at_terminator() || self.at_delimiter(">>") {
                let end = last_end(&children);
                children.push(GrammarNode::missing("]", false, end));
                break;
            }
            let item = self.error_item();
            push_error(&mut children, item, true);
        }
        GrammarNode::spanning("array", children)
    }

    // An object, or a single token where no object starts.
    fn error_item(&mut self) -> GrammarNode {
        self.object().unwrap_or_else(|| self.error_token())
    }

    fn error_token(&mut self) -> GrammarNode {
        let token = self.lex(self.position);
        self.position = token.end;
        leaf_for(token)
    }

    // Tokens up to where `stop` holds, the next `%%EOF` or the end, as one
    // ERROR node (at least one token).
    fn error_until(&mut self, stop: impl Fn(&mut Self) -> bool) -> GrammarNode {
        let mut tokens = Vec::new();
        loop {
            tokens.push(self.error_token());
            self.skip_trivia();
            if self.position >= self.limit || self.at_end_of_file_marker() || stop(self) {
                break;
            }
        }
        GrammarNode::error(tokens[0].start, last_end(&tokens), tokens)
    }

    // Where a file-level item starts: `N G obj`, `xref`, `trailer`,
    // `startxref`, `%%EOF` or the end of the text.
    fn at_sync(&mut self) -> bool {
        if self.content {
            return false;
        }
        if self.position >= self.limit || self.at_end_of_file_marker() {
            return true;
        }
        if ["xref", "trailer", "startxref"]
            .iter()
            .any(|keyword| self.at_keyword(keyword))
        {
            return true;
        }
        let [number, generation, keyword] = self.peek_tokens();
        number.is_some_and(is_unsigned)
            && generation.is_some_and(is_unsigned)
            && keyword.is_some_and(|keyword| is_keyword(keyword, "obj"))
    }

    // Where an unclosed dictionary or array ends.
    fn at_terminator(&mut self) -> bool {
        if self.position >= self.limit {
            return true;
        }
        if self.content {
            return false;
        }
        ["endobj", "stream", "endstream"]
            .iter()
            .any(|keyword| self.at_keyword(keyword))
            || self.at_sync()
    }

    // Up to `N` tokens ahead, skipping trivia, without consuming them.
    fn peek_tokens<const N: usize>(&mut self) -> [Option<Token<'a>>; N] {
        let saved = self.position;
        let mut tokens = [None; N];
        for slot in &mut tokens {
            self.skip_trivia();
            if self.position >= self.limit || self.at_end_of_file_marker() {
                break;
            }
            let token = self.lex(self.position);
            *slot = Some(token);
            self.position = token.end;
        }
        self.position = saved;
        tokens
    }

    // Skips whitespace and comments; outside content streams it stops at a
    // `%%EOF` marker, which is a file-level item.
    fn skip_trivia(&mut self) {
        loop {
            while is_white(self.byte(self.position)) {
                self.position += 1;
            }
            if self.byte(self.position) != Some(b'%') || self.at_end_of_file_marker() {
                return;
            }
            self.position = self.comment_end(self.position, self.limit);
        }
    }

    fn at_end_of_file_marker(&self) -> bool {
        !self.content
            && self.bytes[self.position..].starts_with(b"%%EOF")
            && self.comment_end(self.position, self.limit) == self.position + 5
    }

    fn at_keyword(&self, keyword: &str) -> bool {
        self.at_keyword_at(self.position, keyword)
    }

    fn at_keyword_at(&self, position: usize, keyword: &str) -> bool {
        position + keyword.len() <= self.limit
            && self.bytes[position..].starts_with(keyword.as_bytes())
            && !is_regular(self.byte(position + keyword.len()))
    }

    fn at_delimiter(&self, delimiter: &str) -> bool {
        self.position + delimiter.len() <= self.limit
            && self.bytes[self.position..].starts_with(delimiter.as_bytes())
    }

    // The byte at `position`, or None outside the current limit.
    fn byte(&self, position: usize) -> Option<u8> {
        self.bytes[..self.limit].get(position).copied()
    }

    // A comment runs to the end of its line, without trailing whitespace.
    const fn comment_end(&self, start: usize, bound: usize) -> usize {
        let mut end = start + 1;
        while end < bound && !matches!(self.bytes[end], b'\n' | b'\r') {
            end += 1;
        }
        while end > start + 1 && is_white(Some(self.bytes[end - 1])) {
            end -= 1;
        }
        end
    }

    // The token at `start`, which is neither whitespace nor a comment.
    fn lex(&self, start: usize) -> Token<'a> {
        let text = self.text;
        let token = |kind, end| Token {
            kind,
            start,
            end,
            text: &text[start..end],
        };
        match self.byte(start) {
            Some(b'(') => {
                let mut depth = 0_usize;
                let mut position = start;
                while position < self.limit {
                    match self.bytes[position] {
                        b'\\' => position += 1,
                        b'(' => depth += 1,
                        b')' => {
                            depth -= 1;
                            if depth == 0 {
                                return token("literal_string", position + 1);
                            }
                        }
                        _ => {}
                    }
                    position += 1;
                }
                token("(", start + 1)
            }
            Some(b'<') => {
                if self.byte(start + 1) == Some(b'<') {
                    return token("<<", start + 2);
                }
                let mut end = start + 1;
                while self.byte(end).is_some_and(|byte| byte.is_ascii_hexdigit())
                    || is_white(self.byte(end))
                {
                    end += 1;
                }
                if self.byte(end) == Some(b'>') {
                    token("hex_string", end + 1)
                } else {
                    token("<", start + 1)
                }
            }
            Some(b'>') => {
                if self.byte(start + 1) == Some(b'>') {
                    token(">>", start + 2)
                } else {
                    token(">", start + 1)
                }
            }
            Some(b')') => token(")", start + 1),
            Some(b'[') => token("[", start + 1),
            Some(b']') => token("]", start + 1),
            Some(b'{') => token("{", start + 1),
            Some(b'}') => token("}", start + 1),
            first => {
                let mut end = start + 1;
                while is_regular(self.byte(end)) {
                    end += 1;
                }
                let run = &self.bytes[start..end];
                let kind = if first == Some(b'/') {
                    "name"
                } else if is_integer(run) {
                    "integer"
                } else if is_real(run) {
                    "real"
                } else if run == b"true" || run == b"false" {
                    "boolean"
                } else if run == b"null" {
                    "null"
                } else {
                    "keyword"
                };
                token(kind, end)
            }
        }
    }

    // The whitespace and comment extras covering a gap between CST nodes.
    fn lex_gap(&self, start: usize, end: usize) -> Vec<GrammarNode> {
        let mut extras = Vec::new();
        let mut position = start;
        while position < end {
            if self.bytes[position] == b'%' {
                let comment_end = self.comment_end(position, end);
                extras.push(GrammarNode::extra("comment", true, position, comment_end));
                position = comment_end;
            } else if is_white(Some(self.bytes[position])) {
                let whitespace_start = position;
                while position < end && is_white(Some(self.bytes[position])) {
                    position += 1;
                }
                extras.push(GrammarNode::extra(
                    "whitespace",
                    false,
                    whitespace_start,
                    position,
                ));
            } else {
                panic!(
                    "PDF grammar left {:?} outside its CST",
                    String::from_utf8_lossy(&self.bytes[position..end])
                );
            }
        }
        extras
    }
}

fn last_end(children: &[GrammarNode]) -> usize {
    children.last().map_or(0, |child| child.end)
}

// Appends `error` to `children`, extending a preceding ERROR node; with
// `keep_objects`, well-formed objects are appended as they are.
fn push_error(children: &mut Vec<GrammarNode>, error: GrammarNode, keep_objects: bool) {
    if keep_objects && !error.is_error && error.named && error.term != "keyword" {
        children.push(error);
        return;
    }
    let (start, end) = (error.start, error.end);
    let nodes = if error.is_error {
        error.children
    } else {
        vec![error]
    };
    match children.last_mut() {
        Some(last) if last.is_error => {
            last.children.extend(nodes);
            last.end = end;
        }
        _ => children.push(GrammarNode::error(start, end, nodes)),
    }
}

fn leaf_for(token: Token<'_>) -> GrammarNode {
    if VALUE_TOKENS.contains(&token.kind) {
        return GrammarNode::node(token.kind, token.start, token.end, Vec::new());
    }
    if token.kind == "keyword" {
        return COS_KEYWORDS
            .iter()
            .find(|keyword| **keyword == token.text)
            .map_or_else(
                || GrammarNode::node("keyword", token.start, token.end, Vec::new()),
                |keyword| GrammarNode::anonymous(keyword, token.start, token.end),
            );
    }
    GrammarNode::anonymous(token.kind, token.start, token.end)
}

// Object, generation, cross-reference and offset numbers are unsigned digits.
fn is_unsigned(token: Token<'_>) -> bool {
    token.kind == "integer" && token.text.bytes().all(|byte| byte.is_ascii_digit())
}

fn is_keyword(token: Token<'_>, keyword: &str) -> bool {
    token.kind == "keyword" && token.text == keyword
}

fn is_header(bytes: &[u8]) -> bool {
    let Some(version) = bytes.strip_prefix(b"%PDF-") else {
        return false;
    };
    let mut parts = version.splitn(2, |byte| *byte == b'.');
    let is_digits = |part: &[u8]| !part.is_empty() && part.iter().all(u8::is_ascii_digit);
    matches!((parts.next(), parts.next()), (Some(major), Some(minor)) if is_digits(major) && is_digits(minor))
}

fn is_integer(run: &[u8]) -> bool {
    let digits = run
        .strip_prefix(b"+")
        .or_else(|| run.strip_prefix(b"-"))
        .unwrap_or(run);
    !digits.is_empty() && digits.iter().all(u8::is_ascii_digit)
}

fn is_real(run: &[u8]) -> bool {
    let unsigned = run
        .strip_prefix(b"+")
        .or_else(|| run.strip_prefix(b"-"))
        .unwrap_or(run);
    let Some(point) = unsigned.iter().position(|byte| *byte == b'.') else {
        return false;
    };
    let (whole, fraction) = (&unsigned[..point], &unsigned[point + 1..]);
    whole.iter().all(u8::is_ascii_digit)
        && fraction.iter().all(u8::is_ascii_digit)
        && !(whole.is_empty() && fraction.is_empty())
}

fn dictionary_entries(dictionary: &GrammarNode) -> impl Iterator<Item = &GrammarNode> {
    dictionary
        .children
        .iter()
        .filter(|child| child.term == "dictionary_entry")
}

fn entry_key<'a>(entry: &GrammarNode, text: &'a str) -> &'a str {
    &text[entry.children[0].start..entry.children[0].end]
}

// The value of a direct, non-negative integer `/Length` that JavaScript holds
// exactly, if any.
fn direct_length(dictionary: &GrammarNode, text: &str) -> Option<usize> {
    let entry =
        dictionary_entries(dictionary).find(|candidate| entry_key(candidate, text) == "/Length")?;
    let value = entry
        .children
        .get(1)
        .filter(|value| value.term == "integer")?;
    let run = &text.as_bytes()[value.start..value.end];
    let (negative, digits) = match run.split_first() {
        Some((b'-', digits)) => (true, digits),
        Some((b'+', digits)) => (false, digits),
        _ => (false, run),
    };
    let length = digits.iter().try_fold(0_u64, |length, digit| {
        length.checked_mul(10)?.checked_add(u64::from(digit - b'0'))
    })?;
    if length > MAX_SAFE_INTEGER || (negative && length != 0) {
        return None;
    }
    usize::try_from(length).ok()
}

fn is_content_stream_dictionary(dictionary: &GrammarNode, text: &str) -> bool {
    if dictionary
        .children
        .iter()
        .any(|child| child.is_error || child.is_missing)
    {
        return false;
    }
    dictionary_entries(dictionary).all(|entry| {
        let key = entry_key(entry, text);
        let value = &entry.children[1];
        let name = (value.term == "name").then(|| &text[value.start..value.end]);
        if !CONTENT_STREAM_KEYS.contains(&key) {
            return false;
        }
        match key {
            "/Type" => matches!(name, Some("/XObject" | "/Pattern")),
            "/Subtype" => name == Some("/Form"),
            _ => true,
        }
    })
}

// The first index of `needle` in `haystack` at or after `from`.
fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    haystack
        .get(from..)?
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|index| index + from)
}

const fn is_white(byte: Option<u8>) -> bool {
    matches!(byte, Some(b' ' | b'\n' | b'\r' | b'\t' | 0x0c | 0))
}

fn is_regular(byte: Option<u8>) -> bool {
    byte.is_some_and(|byte| !is_white(Some(byte)) && !b"()<>[]{}/%".contains(&byte))
}
