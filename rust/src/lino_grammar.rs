//! The built-in `LiNo` grammar CST, shared with `js/src/lino-grammar.js`.
//!
//! The recursive descent below mirrors, rule for rule and with the same
//! ordered choices, backtracking, indentation side effects and depth limit, the
//! official links-notation 0.22 PEG grammar (`links-notation/src/grammar.pegjs`)
//! as the official `Parser` reads a document: comments are blanked first
//! (`links-notation/src/comments.js`) and quoted references follow
//! `links-notation/src/quotes.js`. Every document the official parser accepts
//! yields a clean CST whose `link` nodes carry the official `id`, `values` and
//! `children` as `id`, `value` and `child` fields, and whose `group` nodes
//! carry the links of a parenthesized group as `link` fields.
//!
//! - `lino_document` is the root; a link is a named `link` node and a
//!   parenthesized group, which opens a nested context read like the root, is
//!   a named `group` node.
//! - References are named `reference` or `quoted_reference` leaves; `(`, `)`
//!   and `:` are anonymous leaves.
//! - `LiNo` whitespace (space, tab, CR, LF) is an anonymous `whitespace` extra
//!   and a comment a named `comment` extra, each owned by the smallest node
//!   that spans it, as tree-sitter places extras.
//! - Where the official grammar rejects the source, or links nest deeper than
//!   [`LINO_MAX_DEPTH`], an `ERROR` node spans the rest of that line (with its
//!   `(`, `)`, `:` and `reference` tokens) and parsing restarts on the next
//!   line as a new document.
//!
//! Offsets are UTF-8 byte offsets; every delimiter is ASCII, so the
//! JavaScript runtime computes the same tree over string indices.

use std::collections::{HashMap, HashSet};

use crate::builtin_grammar::{fill_extras, propagate_errors, GrammarNode};

/// How deep `LiNo` links may nest: every parenthesized group and every
/// indentation level is one level. The official parsers of every runtime
/// refuse deeper links with the same limit (`DEFAULT_MAX_DEPTH`).
pub const LINO_MAX_DEPTH: usize = 64;

/// Parses `text` into the built-in `LiNo` grammar CST.
pub fn parse_lino_cst(text: &str) -> GrammarNode {
    let bytes = text.as_bytes();
    let comments = comment_ranges(bytes);
    let blanked = blank_ranges(bytes, &comments);
    let mut references = DelimitedReferences::new(&blanked);
    let mut children = Vec::new();
    let mut position = 0;
    while position < blanked.len() {
        let mut parser = LinoGrammarParser::new(&blanked, position, &mut references);
        let document = parser.document();
        children.extend(document.links);
        let Some(error_start) = document.error_start else {
            break;
        };
        let error_end = line_end(&blanked, error_start);
        children.push(error_node(&blanked, error_start, error_end));
        position = error_end;
    }
    let mut root = GrammarNode::node("lino_document", 0, bytes.len(), children);
    propagate_errors(&mut root);
    let comment_ends: HashMap<usize, usize> = comments.into_iter().collect();
    fill_extras(root, &mut |start, end| {
        let mut extras = Vec::new();
        let mut position = start;
        while position < end {
            if let Some(&comment_end) = comment_ends.get(&position) {
                extras.push(GrammarNode::extra("comment", true, position, comment_end));
                position = comment_end;
                continue;
            }
            let whitespace_start = position;
            while position < end && !comment_ends.contains_key(&position) {
                assert!(
                    is_lino_whitespace(bytes[position]),
                    "LiNo grammar left {:?} outside its CST",
                    String::from_utf8_lossy(&bytes[start..end])
                );
                position += 1;
            }
            extras.push(GrammarNode::extra(
                "whitespace",
                false,
                whitespace_start,
                position,
            ));
        }
        extras
    })
}

/// Links nest deeper than [`LINO_MAX_DEPTH`].
struct NestingTooDeep;

/// A rule's result: its node, `None` where it does not match, or the depth
/// error that abandons the whole document.
type Parsed<T> = Result<Option<T>, NestingTooDeep>;

struct ParsedDocument {
    links: Vec<GrammarNode>,
    error_start: Option<usize>,
}

/// The indentation context of a group around the line being read.
struct NestedContext {
    indentation_stack: Vec<usize>,
    base_indentation: Option<usize>,
    context_depth: usize,
}

struct LinoGrammarParser<'a, 'r> {
    text: &'a [u8],
    position: usize,
    references: &'r mut DelimitedReferences<'a>,
    indentation_stack: Vec<usize>,
    base_indentation: Option<usize>,
    context_stack: Vec<NestedContext>,
    context_depth: usize,
    // Lines already found unreadable, keyed by whether they are inside a group
    // and where they start, since reading a line depends on nothing else.
    unreadable_lines: HashSet<(bool, usize)>,
    root_line_start: usize,
}

impl<'a, 'r> LinoGrammarParser<'a, 'r> {
    fn new(text: &'a [u8], position: usize, references: &'r mut DelimitedReferences<'a>) -> Self {
        Self {
            text,
            position,
            references,
            indentation_stack: vec![0],
            base_indentation: None,
            context_stack: Vec::new(),
            context_depth: 0,
            unreadable_lines: HashSet::new(),
            root_line_start: position,
        }
    }

    // document = skipEmptyLines links _ eof / _ eof
    // Where neither alternative reaches the end, the links parsed so far are
    // kept and the error starts after the whitespace that follows them; where
    // links nest too deeply, it starts at the root line holding them.
    fn document(&mut self) -> ParsedDocument {
        self.skip_empty_lines();
        let mut links = Vec::new();
        if self.links(&mut links, true).is_err() {
            return ParsedDocument {
                links,
                error_start: Some(self.root_line_start),
            };
        }
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
    // The links read are appended to `links` as they are read, so the root
    // keeps them when a later line nests too deeply.
    fn links(
        &mut self,
        links: &mut Vec<GrammarNode>,
        at_root: bool,
    ) -> Result<bool, NestingTooDeep> {
        let Some(first) = self.first_line(at_root)? else {
            return Ok(false);
        };
        links.push(first);
        while let Some(line) = self.line(at_root)? {
            links.push(line);
        }
        if self.indentation_stack.len() > 1 {
            self.indentation_stack.pop();
        }
        Ok(true)
    }

    fn nested_links(&mut self) -> Parsed<Vec<GrammarNode>> {
        let mut links = Vec::new();
        Ok(self.links(&mut links, false)?.then_some(links))
    }

    // firstLine = SET_BASE_INDENTATION element
    fn first_line(&mut self, at_root: bool) -> Parsed<GrammarNode> {
        let start = self.position;
        let spaces = self.spaces();
        if self.base_indentation.is_none() {
            self.base_indentation = Some(spaces);
        }
        if at_root {
            self.root_line_start = self.position;
        }
        let element = self.element()?;
        if element.is_none() {
            self.position = start;
        }
        Ok(element)
    }

    // line = CHECK_INDENTATION element
    fn line(&mut self, at_root: bool) -> Parsed<GrammarNode> {
        let start = self.position;
        let spaces = self.spaces();
        if self.normalize_indentation(spaces) < self.current_indentation() {
            self.position = start;
            return Ok(None);
        }
        if at_root {
            self.root_line_start = self.position;
        }
        let element = self.element()?;
        if element.is_none() {
            self.position = start;
        }
        Ok(element)
    }

    // element = anyLink CHECK_DEPTH (PUSH_INDENTATION links)?
    // A line is read once whether or not indented children follow it, and a
    // line that cannot be read is remembered as unreadable.
    fn element(&mut self) -> Parsed<GrammarNode> {
        let start = self.position;
        let key = (!self.context_stack.is_empty(), start);
        let link = if self.unreadable_lines.contains(&key) {
            None
        } else {
            self.any_link()?
        };
        let Some(mut link) = link else {
            self.unreadable_lines.insert(key);
            self.position = start;
            return Ok(None);
        };
        check_depth(self.depth())?;
        let saved = self.indentation_stack.clone();
        let after_link = self.position;
        let spaces = self.spaces();
        let indentation = self.normalize_indentation(spaces);
        if indentation > self.current_indentation() {
            self.indentation_stack.push(indentation);
            if let Some(children) = self.nested_links()? {
                link.end = children.last().map_or(link.end, |child| child.end);
                link.children
                    .extend(children.into_iter().map(|child| child.with_field("child")));
                return Ok(Some(link));
            }
        }
        self.indentation_stack = saved;
        self.position = after_link;
        Ok(Some(link))
    }

    // anyLink = &"(" groupLink / !"(" (indentedIdLink / singleLineAnyLink)
    fn any_link(&mut self) -> Parsed<GrammarNode> {
        if self.text.get(self.position) == Some(&b'(') {
            return self.group_link();
        }
        if let Some(link) = self.indented_id_link() {
            return Ok(Some(link));
        }
        self.single_line_any_link()
    }

    // groupLink = nestedGroup (eol / singleLineValueAndWhitespace* eol)
    // A group that ends its line is the whole link; values after it make it the
    // first value of a value link.
    fn group_link(&mut self) -> Parsed<GrammarNode> {
        let start = self.position;
        let Some(group) = self.nested_group()? else {
            return Ok(None);
        };
        if self.eol() {
            return Ok(Some(group));
        }
        let mut values = vec![group.with_field("value")];
        values.extend(self.values_while_any()?);
        if self.eol() {
            return Ok(Some(GrammarNode::spanning("link", values)));
        }
        Ok(self.fail(start))
    }

    // nestedGroup = "(" CHECK_DEPTH ENTER_NESTED_CONTEXT nestedGroupBody
    fn nested_group(&mut self) -> Parsed<GrammarNode> {
        let start = self.position;
        let Some(open) = self.literal(b'(', "(") else {
            return Ok(None);
        };
        check_depth(self.depth() + 1)?;
        self.enter_nested_context();
        let body = self.nested_group_body();
        self.exit_nested_context();
        let Some(body) = body? else {
            return Ok(self.fail(start));
        };
        let mut children = vec![open];
        children.extend(body);
        Ok(Some(GrammarNode::node(
            "group",
            start,
            self.position,
            children,
        )))
    }

    // nestedGroupBody = skipEmptyLines links _ ")" / _ ")"
    fn nested_group_body(&mut self) -> Parsed<Vec<GrammarNode>> {
        let start = self.position;
        self.skip_empty_lines();
        if let Some(links) = self.nested_links()? {
            self.whitespace();
            if let Some(close) = self.literal(b')', ")") {
                let mut children: Vec<GrammarNode> = links
                    .into_iter()
                    .map(|link| link.with_field("link"))
                    .collect();
                children.push(close);
                return Ok(Some(children));
            }
        }
        self.position = start;
        self.whitespace();
        Ok(self.literal(b')', ")").map(|close| vec![close]))
    }

    // singleLineAnyLink = singleLineLink eol / singleLineValueLink eol
    fn single_line_any_link(&mut self) -> Parsed<GrammarNode> {
        let start = self.position;
        if let Some(link) = self.single_line_link()? {
            if self.eol() {
                return Ok(Some(link));
            }
        }
        self.position = start;
        if let Some(values) = self.single_line_values()? {
            if self.eol() {
                return Ok(Some(GrammarNode::spanning("link", values)));
            }
        }
        Ok(self.fail(start))
    }

    // singleLineLink = __ reference __ ":" singleLineValues
    fn single_line_link(&mut self) -> Parsed<GrammarNode> {
        let start = self.position;
        self.inline_whitespace();
        let Some(id) = self.reference() else {
            return Ok(self.fail(start));
        };
        self.inline_whitespace();
        let Some(colon) = self.literal(b':', ":") else {
            return Ok(self.fail(start));
        };
        let Some(values) = self.single_line_values()? else {
            return Ok(self.fail(start));
        };
        let mut children = vec![id.with_field("id"), colon];
        children.extend(values);
        Ok(Some(GrammarNode::spanning("link", children)))
    }

    // singleLineValues = (__ referenceOrLink)+
    fn single_line_values(&mut self) -> Parsed<Vec<GrammarNode>> {
        let values = self.values_while_any()?;
        Ok((!values.is_empty()).then_some(values))
    }

    // (__ referenceOrLink)*
    fn values_while_any(&mut self) -> Result<Vec<GrammarNode>, NestingTooDeep> {
        let mut values = Vec::new();
        loop {
            let start = self.position;
            self.inline_whitespace();
            let Some(value) = self.reference_or_link()? else {
                self.position = start;
                return Ok(values);
            };
            values.push(value.with_field("value"));
        }
    }

    // referenceOrLink = nestedGroup / reference
    fn reference_or_link(&mut self) -> Parsed<GrammarNode> {
        if let Some(group) = self.nested_group()? {
            return Ok(Some(group));
        }
        Ok(self.reference())
    }

    // indentedIdLink = reference __ ":" eol
    fn indented_id_link(&mut self) -> Option<GrammarNode> {
        let start = self.position;
        let id = self.reference()?;
        self.inline_whitespace();
        let Some(colon) = self.literal(b':', ":") else {
            return self.fail(start);
        };
        if !self.eol() {
            return self.fail(start);
        }
        Some(GrammarNode::spanning(
            "link",
            vec![id.with_field("id"), colon],
        ))
    }

    // reference = quotedReference / simpleReference
    fn reference(&mut self) -> Option<GrammarNode> {
        self.quoted_reference().or_else(|| self.simple_reference())
    }

    // A reference between delimiters, read as `links-notation/src/quotes.js`
    // reads it.
    fn quoted_reference(&mut self) -> Option<GrammarNode> {
        let start = self.position;
        if !is_quote(*self.text.get(start)?) {
            return None;
        }
        let length = self.references.length_at(start)?;
        self.position = start + length;
        Some(GrammarNode::node(
            "quoted_reference",
            start,
            self.position,
            Vec::new(),
        ))
    }

    // simpleReference = [^ \t\n\r(:)]+
    fn simple_reference(&mut self) -> Option<GrammarNode> {
        let start = self.position;
        while self
            .text
            .get(self.position)
            .is_some_and(|byte| is_reference_byte(*byte))
        {
            self.position += 1;
        }
        (self.position > start)
            .then(|| GrammarNode::node("reference", start, self.position, Vec::new()))
    }

    // eol = __ (lineBreaks / eof / nestedGroupEnd)
    // lineBreaks = [\r\n]+ ([ \t]+ [\r\n]+)*
    // nestedGroupEnd = inside a group, before ")"
    fn eol(&mut self) -> bool {
        let start = self.position;
        self.inline_whitespace();
        if self.at_end() {
            return true;
        }
        if self.newline_character() {
            while self.newline_character() {}
            loop {
                let blank_line = self.position;
                let indented = self.inline_whitespace();
                if indented == 0 || !self.newline_character() {
                    self.position = blank_line;
                    return true;
                }
                while self.newline_character() {}
            }
        }
        if !self.context_stack.is_empty() && self.text.get(self.position) == Some(&b')') {
            return true;
        }
        self.position = start;
        false
    }

    fn literal(&mut self, byte: u8, term: &'static str) -> Option<GrammarNode> {
        if self.text.get(self.position) != Some(&byte) {
            return None;
        }
        self.position += 1;
        Some(GrammarNode::anonymous(
            term,
            self.position - 1,
            self.position,
        ))
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
    fn inline_whitespace(&mut self) -> usize {
        let start = self.position;
        while matches!(self.text.get(self.position), Some(b' ' | b'\t')) {
            self.position += 1;
        }
        self.position - start
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

    fn fail(&mut self, start: usize) -> Option<GrammarNode> {
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

    // Every enclosing group and every indentation level is one level of depth.
    fn depth(&self) -> usize {
        self.context_depth + self.indentation_stack.len() - 1
    }

    // A group opens a context that starts fresh at indentation level zero.
    fn enter_nested_context(&mut self) {
        let context_depth = self.depth() + 1;
        self.context_stack.push(NestedContext {
            indentation_stack: std::mem::replace(&mut self.indentation_stack, vec![0]),
            base_indentation: self.base_indentation.take(),
            context_depth: self.context_depth,
        });
        self.context_depth = context_depth;
    }

    fn exit_nested_context(&mut self) {
        let context = self
            .context_stack
            .pop()
            .expect("every nested context entered is exited");
        self.indentation_stack = context.indentation_stack;
        self.base_indentation = context.base_indentation;
        self.context_depth = context.context_depth;
    }
}

const fn check_depth(levels: usize) -> Result<(), NestingTooDeep> {
    if levels > LINO_MAX_DEPTH {
        Err(NestingTooDeep)
    } else {
        Ok(())
    }
}

// ---- Comments, as `links-notation/src/comments.js` finds them ---------------

/// The `(start, end)` ranges of the comments of `text`: a `#` that opens a
/// token starts a comment that runs to the end of its line, and a `#` inside
/// a token or a quoted reference is an ordinary character.
fn comment_ranges(text: &[u8]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut references = DelimitedReferences::new(text);
    let mut position = 0;
    while position < text.len() {
        let byte = text[position];
        if is_quote(byte) && follows(text, position, b" \t\n\r(:") {
            position += references.length_at(position).unwrap_or(1);
        } else if byte == b'#' && follows(text, position, b" \t\n\r") {
            let start = position;
            position = line_end(text, position);
            ranges.push((start, position));
        } else {
            position += 1;
        }
    }
    ranges
}

fn follows(text: &[u8], position: usize, allowed: &[u8]) -> bool {
    position == 0 || allowed.contains(&text[position - 1])
}

/// `text` with every byte of the given ranges replaced by a space.
fn blank_ranges(text: &[u8], ranges: &[(usize, usize)]) -> Vec<u8> {
    let mut blanked = text.to_vec();
    for &(start, end) in ranges {
        blanked[start..end].fill(b' ');
    }
    blanked
}

// ---- Quoted references, as `links-notation/src/quotes.js` reads them --------

/// Every quoted reference of one document, read on demand. A reference opened
/// by a run of N delimiters closes at the next run whose length R has an odd
/// `R / N`; inside it a run of 2N is an escaped N. An even opening run that
/// encloses nothing substantive is the empty reference.
struct DelimitedReferences<'a> {
    text: &'a [u8],
    runs_by_quote: HashMap<u8, DelimiterRuns>,
    lengths: HashMap<usize, Option<usize>>,
}

impl<'a> DelimitedReferences<'a> {
    fn new(text: &'a [u8]) -> Self {
        Self {
            text,
            runs_by_quote: HashMap::new(),
            lengths: HashMap::new(),
        }
    }

    /// The length of the reference opened at `start`, if one opens there.
    fn length_at(&mut self, start: usize) -> Option<usize> {
        if let Some(length) = self.lengths.get(&start) {
            return *length;
        }
        let length = self.read(start);
        self.lengths.insert(start, length);
        length
    }

    fn read(&mut self, start: usize) -> Option<usize> {
        let text = self.text;
        let quote = *text.get(start)?;
        if !is_quote(quote) {
            return None;
        }
        let runs = self
            .runs_by_quote
            .entry(quote)
            .or_insert_with(|| DelimiterRuns::new(text, quote));
        let opening = runs.index_of(start);
        let count = runs.end(opening) - start;
        let mut closing = opening + 1;
        while closing < runs.count() {
            let length = runs.lengths[closing];
            if length < count {
                closing = runs.next_longer[closing];
            } else if (length / count) % 2 == 1 {
                break;
            } else {
                closing += 1;
            }
        }
        let end = (closing < runs.count()).then(|| runs.end(closing));
        read_quoted(text, start, quote, count, end)
    }
}

/// The length of the reference opened by `count` delimiters at `start` and
/// closed at `end`.
fn read_quoted(
    text: &[u8],
    start: usize,
    quote: u8,
    count: usize,
    end: Option<usize>,
) -> Option<usize> {
    let empty_reference = (count % 2 == 0).then_some(count);
    let Some(end) = end else {
        return empty_reference;
    };
    let Some(empty_reference) = empty_reference else {
        return Some(end - start);
    };
    let value = quoted_body(text, start, quote, count, end);
    Some(if is_substantive_body(&value) {
        end - start
    } else {
        empty_reference
    })
}

/// The value of the reference opened by `count` delimiters at `start` and
/// closed at `end`: a run of 2N delimiters inside is an escaped N.
fn quoted_body(text: &[u8], start: usize, quote: u8, count: usize, end: usize) -> Vec<u8> {
    let mut value = Vec::new();
    let mut position = start + count;
    while position < end {
        if text[position] != quote {
            value.push(text[position]);
            position += 1;
            continue;
        }
        let length = run_length(text, position, quote);
        let escaped = length / (2 * count) * count;
        let closes = if position + length == end { count } else { 0 };
        value.resize(value.len() + length - escaped - closes, quote);
        position += length;
    }
    value
}

/// The value of the `quoted_reference` node spanning `start..end` of `text`:
/// the empty reference when the node is its opening run alone.
pub fn quoted_reference_value(text: &str, start: usize, end: usize) -> String {
    let bytes = text.as_bytes();
    let quote = bytes[start];
    let count = run_length(bytes, start, quote).min(end - start);
    if end - start == count {
        return String::new();
    }
    String::from_utf8_lossy(&quoted_body(bytes, start, quote, count, end)).into_owned()
}

fn run_length(text: &[u8], start: usize, quote: u8) -> usize {
    text[start..]
        .iter()
        .take_while(|byte| **byte == quote)
        .count()
}

/// The maximal runs of one delimiter, each linked to the next longer run.
struct DelimiterRuns {
    starts: Vec<usize>,
    lengths: Vec<usize>,
    next_longer: Vec<usize>,
}

impl DelimiterRuns {
    fn new(text: &[u8], quote: u8) -> Self {
        let mut starts = Vec::new();
        let mut lengths = Vec::new();
        let mut position = 0;
        while position < text.len() {
            if text[position] == quote {
                let length = run_length(text, position, quote);
                starts.push(position);
                lengths.push(length);
                position += length;
            } else {
                position += 1;
            }
        }
        let count = starts.len();
        let mut next_longer = vec![count; count];
        let mut longer: Vec<usize> = Vec::new();
        for run in (0..count).rev() {
            while longer
                .last()
                .is_some_and(|&last| lengths[last] <= lengths[run])
            {
                longer.pop();
            }
            next_longer[run] = longer.last().copied().unwrap_or(count);
            longer.push(run);
        }
        Self {
            starts,
            lengths,
            next_longer,
        }
    }

    fn count(&self) -> usize {
        self.starts.len()
    }

    fn end(&self, run: usize) -> usize {
        self.starts[run] + self.lengths[run]
    }

    /// The run that holds the delimiter at `position`.
    fn index_of(&self, position: usize) -> usize {
        self.starts
            .partition_point(|&start| start <= position)
            .saturating_sub(1)
    }
}

/// Whether a body holds a visible character and no parenthesis it straddles.
fn is_substantive_body(content: &[u8]) -> bool {
    let mut depth = 0_usize;
    let mut has_visible = false;
    for &byte in content {
        if byte == b'(' {
            depth += 1;
        } else if byte == b')' {
            let Some(outer) = depth.checked_sub(1) else {
                return false;
            };
            depth = outer;
        }
        if !is_lino_whitespace(byte) {
            has_visible = true;
        }
    }
    has_visible && depth == 0
}

fn error_node(text: &[u8], start: usize, end: usize) -> GrammarNode {
    let mut children = Vec::new();
    let mut position = start;
    while position < end {
        match text[position] {
            byte if is_lino_whitespace(byte) => position += 1,
            b'(' => {
                children.push(GrammarNode::anonymous("(", position, position + 1));
                position += 1;
            }
            b')' => {
                children.push(GrammarNode::anonymous(")", position, position + 1));
                position += 1;
            }
            b':' => {
                children.push(GrammarNode::anonymous(":", position, position + 1));
                position += 1;
            }
            _ => {
                let reference_start = position;
                while position < end && is_reference_byte(text[position]) {
                    position += 1;
                }
                children.push(GrammarNode::node(
                    "reference",
                    reference_start,
                    position,
                    Vec::new(),
                ));
            }
        }
    }
    GrammarNode::error(start, end, children)
}

// The rest of the line after `start`, without its line terminator.
const fn line_end(text: &[u8], start: usize) -> usize {
    let mut end = start;
    while end < text.len() && !matches!(text[end], b'\n' | b'\r') {
        end += 1;
    }
    end
}

const fn is_quote(byte: u8) -> bool {
    matches!(byte, b'"' | b'\'' | b'`')
}

const fn is_lino_whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r')
}

const fn is_reference_byte(byte: u8) -> bool {
    !is_lino_whitespace(byte) && !matches!(byte, b'(' | b':' | b')')
}
