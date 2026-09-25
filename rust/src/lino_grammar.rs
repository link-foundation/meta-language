//! The built-in `LiNo` grammar CST, shared with `js/src/lino-grammar.js`.
//!
//! The recursive descent below mirrors, rule for rule and with the same
//! ordered choices, backtracking and indentation side effects, the official
//! links-notation 0.13 PEG grammar (`links-notation/src/grammar.pegjs`), so
//! every document the official grammar accepts yields a clean CST whose `link`
//! nodes carry the official `id`, `values` and `children` as `id`, `value` and
//! `child` fields.
//!
//! - `lino_document` is the root; every link form is a named `link` node.
//! - References are named `reference` or `quoted_reference` leaves; `(`, `)`
//!   and `:` are anonymous leaves.
//! - `LiNo` whitespace (space, tab, CR, LF) is an anonymous `whitespace` extra
//!   owned by the smallest node that spans it, as tree-sitter places extras.
//! - Where the official grammar rejects the source, an `ERROR` node spans the
//!   rest of that line (with its `(`, `)`, `:` and `reference` tokens) and
//!   parsing restarts on the next line as a new document.
//!
//! Offsets are UTF-8 byte offsets; every delimiter is ASCII, so the
//! JavaScript runtime computes the same tree over string indices.

/// A node of the built-in `LiNo` grammar CST over byte offsets.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct LinoNode {
    pub term: &'static str,
    pub named: bool,
    pub field: Option<&'static str>,
    pub start: usize,
    pub end: usize,
    pub is_error: bool,
    pub has_error: bool,
    pub extra: bool,
    pub children: Vec<Self>,
}

impl LinoNode {
    const fn named(term: &'static str, start: usize, end: usize, children: Vec<Self>) -> Self {
        Self {
            term,
            named: true,
            field: None,
            start,
            end,
            is_error: false,
            has_error: false,
            extra: false,
            children,
        }
    }

    fn anonymous(term: &'static str, start: usize, end: usize) -> Self {
        Self {
            named: false,
            ..Self::named(term, start, end, Vec::new())
        }
    }

    fn link(children: Vec<Self>) -> Self {
        let start = children.first().map_or(0, |child| child.start);
        let end = children.last().map_or(0, |child| child.end);
        Self::named("link", start, end, children)
    }

    const fn with_field(mut self, field: &'static str) -> Self {
        self.field = Some(field);
        self
    }

    /// Whether this node is a leaf token of the grammar.
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty() && self.term != "lino_document"
    }
}

/// Parses `text` into the built-in `LiNo` grammar CST.
pub fn parse_lino_cst(text: &str) -> LinoNode {
    let bytes = text.as_bytes();
    let mut children = Vec::new();
    let mut position = 0;
    while position < bytes.len() {
        let mut parser = LinoGrammarParser::new(bytes, position);
        let document = parser.document();
        children.extend(document.links);
        let Some(error_start) = document.error_start else {
            break;
        };
        let error_end = line_end(bytes, error_start);
        children.push(error_node(bytes, error_start, error_end));
        position = error_end;
    }
    let mut root = LinoNode::named("lino_document", 0, bytes.len(), children);
    root.has_error = root
        .children
        .iter()
        .any(|child| child.is_error || child.has_error);
    fill_whitespace(&mut root, bytes);
    root
}

struct ParsedDocument {
    links: Vec<LinoNode>,
    error_start: Option<usize>,
}

struct LinoGrammarParser<'a> {
    text: &'a [u8],
    position: usize,
    indentation_stack: Vec<usize>,
    base_indentation: Option<usize>,
}

impl<'a> LinoGrammarParser<'a> {
    fn new(text: &'a [u8], position: usize) -> Self {
        Self {
            text,
            position,
            indentation_stack: vec![0],
            base_indentation: None,
        }
    }

    // document = skipEmptyLines links _ eof / _ eof
    // Where neither alternative reaches the end, the links parsed so far are
    // kept and the error starts after the whitespace that follows them.
    fn document(&mut self) -> ParsedDocument {
        self.skip_empty_lines();
        let links = self.links().unwrap_or_default();
        self.whitespace();
        ParsedDocument {
            links,
            error_start: (!self.at_end()).then_some(self.position),
        }
    }

    // skipEmptyLines = ([ \t]* [\r\n])*
    fn skip_empty_lines(&mut self) {
        loop {
            let start = self.position;
            self.inline_whitespace();
            if !self.newline_character() {
                self.position = start;
                return;
            }
        }
    }

    // links = firstLine line*   (pops the indentation pushed for it)
    fn links(&mut self) -> Option<Vec<LinoNode>> {
        let first = self.first_line()?;
        let mut links = vec![first];
        while let Some(line) = self.line() {
            links.push(line);
        }
        if self.indentation_stack.len() > 1 {
            self.indentation_stack.pop();
        }
        Some(links)
    }

    // firstLine = SET_BASE_INDENTATION element
    fn first_line(&mut self) -> Option<LinoNode> {
        let start = self.position;
        let spaces = self.spaces();
        if self.base_indentation.is_none() {
            self.base_indentation = Some(spaces);
        }
        let element = self.element();
        if element.is_none() {
            self.position = start;
        }
        element
    }

    // line = CHECK_INDENTATION element
    fn line(&mut self) -> Option<LinoNode> {
        let start = self.position;
        let spaces = self.spaces();
        if self.normalize_indentation(spaces) < self.current_indentation() {
            self.position = start;
            return None;
        }
        let element = self.element();
        if element.is_none() {
            self.position = start;
        }
        element
    }

    // element = anyLink PUSH_INDENTATION links / anyLink
    fn element(&mut self) -> Option<LinoNode> {
        let start = self.position;
        if let Some(mut link) = self.any_link() {
            let spaces = self.spaces();
            let indentation = self.normalize_indentation(spaces);
            if indentation > self.current_indentation() {
                self.indentation_stack.push(indentation);
                if let Some(links) = self.links() {
                    link.end = links.last().map_or(link.end, |child| child.end);
                    link.children
                        .extend(links.into_iter().map(|child| child.with_field("child")));
                    return Some(link);
                }
            }
        }
        self.position = start;
        self.any_link()
    }

    // anyLink = multiLineAnyLink eol / indentedIdLink / singleLineAnyLink
    fn any_link(&mut self) -> Option<LinoNode> {
        let start = self.position;
        if let Some(link) = self.multi_line_any_link() {
            if self.eol() {
                return Some(link);
            }
        }
        self.position = start;
        self.indented_id_link()
            .or_else(|| self.single_line_any_link())
    }

    // multiLineAnyLink = multiLineValueLink / multiLineLink
    fn multi_line_any_link(&mut self) -> Option<LinoNode> {
        self.multi_line_value_link()
            .or_else(|| self.multi_line_link())
    }

    // multiLineValueLink = "(" multiLineValues _ ")"
    fn multi_line_value_link(&mut self) -> Option<LinoNode> {
        let start = self.position;
        let open = self.literal(b'(', "(")?;
        let values = self.multi_line_values();
        self.whitespace();
        let Some(close) = self.literal(b')', ")") else {
            return self.fail(start);
        };
        let mut children = vec![open];
        children.extend(values);
        children.push(close);
        Some(LinoNode::link(children))
    }

    // multiLineLink = "(" _ reference _ ":" multiLineValues _ ")"
    fn multi_line_link(&mut self) -> Option<LinoNode> {
        let start = self.position;
        let open = self.literal(b'(', "(")?;
        self.whitespace();
        let Some(id) = self.reference() else {
            return self.fail(start);
        };
        self.whitespace();
        let Some(colon) = self.literal(b':', ":") else {
            return self.fail(start);
        };
        let values = self.multi_line_values();
        self.whitespace();
        let Some(close) = self.literal(b')', ")") else {
            return self.fail(start);
        };
        let mut children = vec![open, id.with_field("id"), colon];
        children.extend(values);
        children.push(close);
        Some(LinoNode::link(children))
    }

    // multiLineValues = _ (referenceOrLink _)*
    fn multi_line_values(&mut self) -> Vec<LinoNode> {
        self.whitespace();
        let mut values = Vec::new();
        while let Some(value) = self.reference_or_link() {
            values.push(value.with_field("value"));
            self.whitespace();
        }
        values
    }

    // singleLineValues = (__ referenceOrLink)+
    fn single_line_values(&mut self) -> Option<Vec<LinoNode>> {
        let mut values = Vec::new();
        loop {
            let start = self.position;
            self.inline_whitespace();
            let Some(value) = self.reference_or_link() else {
                self.position = start;
                break;
            };
            values.push(value.with_field("value"));
        }
        (!values.is_empty()).then_some(values)
    }

    // referenceOrLink = multiLineAnyLink / reference
    fn reference_or_link(&mut self) -> Option<LinoNode> {
        self.multi_line_any_link().or_else(|| self.reference())
    }

    // singleLineAnyLink = singleLineLink eol / singleLineValueLink eol
    fn single_line_any_link(&mut self) -> Option<LinoNode> {
        let start = self.position;
        if let Some(link) = self.single_line_link() {
            if self.eol() {
                return Some(link);
            }
        }
        self.position = start;
        if let Some(values) = self.single_line_values() {
            if self.eol() {
                return Some(LinoNode::link(values));
            }
        }
        self.fail(start)
    }

    // singleLineLink = __ reference __ ":" singleLineValues
    fn single_line_link(&mut self) -> Option<LinoNode> {
        let start = self.position;
        self.inline_whitespace();
        let Some(id) = self.reference() else {
            return self.fail(start);
        };
        self.inline_whitespace();
        let Some(colon) = self.literal(b':', ":") else {
            return self.fail(start);
        };
        let Some(values) = self.single_line_values() else {
            return self.fail(start);
        };
        let mut children = vec![id.with_field("id"), colon];
        children.extend(values);
        Some(LinoNode::link(children))
    }

    // indentedIdLink = reference __ ":" eol
    fn indented_id_link(&mut self) -> Option<LinoNode> {
        let start = self.position;
        let id = self.reference()?;
        self.inline_whitespace();
        let Some(colon) = self.literal(b':', ":") else {
            return self.fail(start);
        };
        if !self.eol() {
            return self.fail(start);
        }
        Some(LinoNode::link(vec![id.with_field("id"), colon]))
    }

    // reference = quotedReference / simpleReference
    fn reference(&mut self) -> Option<LinoNode> {
        self.quoted_reference().or_else(|| self.simple_reference())
    }

    // N opening quotes (", ' or `), content in which 2N quotes escape N, and
    // exactly N closing quotes not followed by another quote.
    fn quoted_reference(&mut self) -> Option<LinoNode> {
        let start = self.position;
        let quote = *self.text.get(start)?;
        if !matches!(quote, b'"' | b'\'' | b'`') {
            return None;
        }
        let mut position = start;
        while self.text.get(position) == Some(&quote) {
            position += 1;
        }
        let count = position - start;
        let run = |at: usize, length: usize| {
            self.text
                .get(at..at + length)
                .is_some_and(|bytes| bytes.iter().all(|byte| *byte == quote))
        };
        while position < self.text.len() {
            if run(position, count * 2) {
                position += count * 2;
            } else if run(position, count) && self.text.get(position + count) != Some(&quote) {
                self.position = position + count;
                return Some(LinoNode::named(
                    "quoted_reference",
                    start,
                    self.position,
                    Vec::new(),
                ));
            } else {
                position += 1;
            }
        }
        None
    }

    // simpleReference = [^ \t\n\r(:)]+
    fn simple_reference(&mut self) -> Option<LinoNode> {
        let start = self.position;
        while self
            .text
            .get(self.position)
            .is_some_and(|byte| is_reference_byte(*byte))
        {
            self.position += 1;
        }
        (self.position > start)
            .then(|| LinoNode::named("reference", start, self.position, Vec::new()))
    }

    // eol = __ ([\r\n]+ / eof)
    fn eol(&mut self) -> bool {
        let start = self.position;
        self.inline_whitespace();
        if self.at_end() {
            return true;
        }
        if !self.newline_character() {
            self.position = start;
            return false;
        }
        while self.newline_character() {}
        true
    }

    fn literal(&mut self, byte: u8, term: &'static str) -> Option<LinoNode> {
        if self.text.get(self.position) != Some(&byte) {
            return None;
        }
        self.position += 1;
        Some(LinoNode::anonymous(term, self.position - 1, self.position))
    }

    fn newline_character(&mut self) -> bool {
        if matches!(self.text.get(self.position), Some(b'\n' | b'\r')) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    // " "*
    fn spaces(&mut self) -> usize {
        let start = self.position;
        while self.text.get(self.position) == Some(&b' ') {
            self.position += 1;
        }
        self.position - start
    }

    // __ = [ \t]*
    fn inline_whitespace(&mut self) {
        while matches!(self.text.get(self.position), Some(b' ' | b'\t')) {
            self.position += 1;
        }
    }

    // _ = [ \t\n\r]*
    fn whitespace(&mut self) {
        while self
            .text
            .get(self.position)
            .is_some_and(|byte| is_lino_whitespace(*byte))
        {
            self.position += 1;
        }
    }

    const fn at_end(&self) -> bool {
        self.position >= self.text.len()
    }

    fn fail(&mut self, start: usize) -> Option<LinoNode> {
        self.position = start;
        None
    }

    fn normalize_indentation(&self, spaces: usize) -> usize {
        self.base_indentation
            .map_or(spaces, |base| spaces.saturating_sub(base))
    }

    fn current_indentation(&self) -> usize {
        self.indentation_stack.last().copied().unwrap_or(0)
    }
}

fn error_node(text: &[u8], start: usize, end: usize) -> LinoNode {
    let mut children = Vec::new();
    let mut position = start;
    while position < end {
        match text[position] {
            byte if is_lino_whitespace(byte) => position += 1,
            b'(' => {
                children.push(LinoNode::anonymous("(", position, position + 1));
                position += 1;
            }
            b')' => {
                children.push(LinoNode::anonymous(")", position, position + 1));
                position += 1;
            }
            b':' => {
                children.push(LinoNode::anonymous(":", position, position + 1));
                position += 1;
            }
            _ => {
                let reference_start = position;
                while position < end && is_reference_byte(text[position]) {
                    position += 1;
                }
                children.push(LinoNode::named(
                    "reference",
                    reference_start,
                    position,
                    Vec::new(),
                ));
            }
        }
    }
    let mut error = LinoNode::named("ERROR", start, end, children);
    error.is_error = true;
    error
}

// The rest of the line after `start`, without its line terminator.
const fn line_end(text: &[u8], start: usize) -> usize {
    let mut end = start;
    while end < text.len() && !matches!(text[end], b'\n' | b'\r') {
        end += 1;
    }
    end
}

// Gives every node the whitespace between (and, for the root, around) its
// children as anonymous `whitespace` extras.
fn fill_whitespace(parent: &mut LinoNode, text: &[u8]) {
    if parent.is_leaf() {
        return;
    }
    let mut children = Vec::new();
    let mut covered = parent.start;
    let gap = |children: &mut Vec<LinoNode>, covered: usize, end: usize| {
        if end <= covered {
            return;
        }
        assert!(
            text[covered..end]
                .iter()
                .all(|byte| is_lino_whitespace(*byte)),
            "LiNo grammar left {:?} outside its CST",
            String::from_utf8_lossy(&text[covered..end])
        );
        let mut whitespace = LinoNode::anonymous("whitespace", covered, end);
        whitespace.extra = true;
        children.push(whitespace);
    };
    for mut child in std::mem::take(&mut parent.children) {
        gap(&mut children, covered, child.start);
        fill_whitespace(&mut child, text);
        covered = child.end;
        children.push(child);
    }
    gap(&mut children, covered, parent.end);
    parent.children = children;
}

const fn is_lino_whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r')
}

const fn is_reference_byte(byte: u8) -> bool {
    !is_lino_whitespace(byte) && !matches!(byte, b'(' | b':' | b')')
}
