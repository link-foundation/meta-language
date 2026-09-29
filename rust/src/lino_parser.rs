//! The links-notation semantics of a `LiNo` document.
//!
//! The links are read from the built-in lossless `LiNo` CST
//! ([`parse_lino_cst`]) with the transformation the official links-notation
//! 0.22 `Parser` applies to its raw parse (`links-notation/src/Parser.js`): an
//! indented id takes the lines below it as its values, a line with values and
//! indented lines is combined with each of them, and a parenthesized group
//! holding one link is that link. Every link becomes a Relation spanning its
//! source, named for `name:`; every reference becomes the link carrying that
//! name, so a name resolves to its definition wherever it appears (shared,
//! recursive and forward references), or else to one Concept point per name.
//! A path element the official parser repeats in several combined links is one
//! shared link. Mirrors `js/src/lino-semantics.js`.

use std::collections::HashSet;

use serde::ser::{Serialize, SerializeSeq, Serializer};

use crate::builtin_grammar::GrammarNode;
use crate::line_index::LineIndex;
use crate::lino_grammar::{parse_lino_cst, quoted_reference_value};
use crate::{
    structured_text_parser, ByteRange, LinkId, LinkMetadata, LinkNetwork, LinkType,
    ParseConfiguration, SourceSpan,
};

pub fn parse(text: &str, language: &str, configuration: ParseConfiguration) -> LinkNetwork {
    parse_with_links(text, language, configuration).0
}

fn parse_with_links(
    text: &str,
    language: &str,
    configuration: ParseConfiguration,
) -> (LinkNetwork, Vec<LinkId>) {
    let mut network = structured_text_parser::parse_lino(text, language, configuration);
    let line_index = LineIndex::new(text);
    let documents = LinoSemantics::new(&mut network, text, &line_index, language).insert_document();
    (network, documents)
}

/// The links-notation reading of one link.
///
/// A reference is its name and a link is its optional name followed by its
/// values, the form of `parity/fixtures/lino-compatibility-matrix.json`. It
/// serializes as that JSON: a string, or an array whose first item is the
/// name or `null`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinksNotationReading {
    /// A reference to a name.
    Reference(String),
    /// A link: its name, if it is named, and its values in order.
    Link {
        name: Option<String>,
        values: Vec<Self>,
    },
}

impl Serialize for LinksNotationReading {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Reference(name) => serializer.serialize_str(name),
            Self::Link { name, values } => {
                let mut sequence = serializer.serialize_seq(Some(values.len() + 1))?;
                sequence.serialize_element(name)?;
                for value in values {
                    sequence.serialize_element(value)?;
                }
                sequence.end()
            }
        }
    }
}

impl LinkNetwork {
    /// Parses a `LiNo` document and returns its network with the ids of its
    /// top-level links in document order, as the official links-notation
    /// parser lists them.
    #[must_use]
    pub fn parse_links_notation(
        text: &str,
        configuration: ParseConfiguration,
    ) -> (Self, Vec<LinkId>) {
        parse_with_links(text, "LiNo", configuration)
    }

    /// The links-notation reading of the link `id` that
    /// [`LinkNetwork::parse_links_notation`] inserted. A named link appearing
    /// as a value outside its own source (a reference to it) reads as its name.
    #[must_use]
    pub fn links_notation_reading(&self, id: LinkId) -> Option<LinksNotationReading> {
        let metadata = self.link(id)?.metadata();
        if metadata.link_type() != Some(LinkType::Relation) {
            return Some(LinksNotationReading::Reference(
                metadata.term().unwrap_or_default().to_owned(),
            ));
        }
        let mut values = Vec::new();
        for &value in self.link(id)?.references() {
            let value_metadata = self.link(value)?.metadata();
            let is_reference = value_metadata.link_type() == Some(LinkType::Relation)
                && value_metadata.is_named()
                && (value == id || !within(value_metadata.span(), metadata.span()));
            values.push(if is_reference {
                LinksNotationReading::Reference(
                    value_metadata.term().unwrap_or_default().to_owned(),
                )
            } else {
                self.links_notation_reading(value)?
            });
        }
        Some(LinksNotationReading::Link {
            name: metadata
                .is_named()
                .then(|| metadata.term().unwrap_or_default().to_owned()),
            values,
        })
    }
}

impl LinkNetwork {
    /// The links-notation text of the links `ids`, one per line, which reads
    /// back as the same links.
    #[must_use]
    pub fn links_notation_text(&self, ids: &[LinkId]) -> String {
        let readings: Vec<LinksNotationReading> = ids
            .iter()
            .filter_map(|&id| self.links_notation_reading(id))
            .collect();
        format_lino_readings(&readings)
    }
}

/// The links-notation text of `readings`, one top-level link per line, which
/// the official parser reads back as the same readings: a link is
/// parenthesized, a top-level reference is `name:`, and a name that is not a
/// plain reference is quoted. Mirrors `formatLinoReadings`.
fn format_lino_readings(readings: &[LinksNotationReading]) -> String {
    // A quoted reference opened by N delimiters closes at the next run R with
    // an odd R / N anywhere after it, so the empty reference is written as an
    // even run of `"` longer than every `"` run of the document, which nothing
    // closes.
    fn collect<'r>(reading: &'r LinksNotationReading, names: &mut Vec<&'r str>) {
        match reading {
            LinksNotationReading::Reference(name) => names.push(name),
            LinksNotationReading::Link { name, values } => {
                names.extend(name.as_deref());
                for value in values {
                    collect(value, names);
                }
            }
        }
    }
    let mut names = Vec::new();
    for reading in readings {
        collect(reading, &mut names);
    }
    let longest = names
        .iter()
        .map(|name| longest_run(&quote_name(name), '"'))
        .max()
        .unwrap_or(0);
    let empty = "\"".repeat(longest + 2 - longest % 2);
    let name_text = |name: &str| {
        if name.is_empty() {
            empty.clone()
        } else {
            quote_name(name)
        }
    };
    let mut text = String::new();
    for reading in readings {
        match reading {
            LinksNotationReading::Reference(name) => {
                text.push_str(&name_text(name));
                text.push(':');
            }
            LinksNotationReading::Link { .. } => format_link(reading, &name_text, &mut text),
        }
        text.push('\n');
    }
    text
}

fn format_link(
    reading: &LinksNotationReading,
    name_text: &dyn Fn(&str) -> String,
    text: &mut String,
) {
    match reading {
        LinksNotationReading::Reference(name) => text.push_str(&name_text(name)),
        LinksNotationReading::Link { name, values } => {
            text.push('(');
            if let Some(name) = name {
                text.push_str(&name_text(name));
                text.push(':');
            }
            for (index, value) in values.iter().enumerate() {
                if index > 0 || name.is_some() {
                    text.push(' ');
                }
                format_link(value, name_text, text);
            }
            text.push(')');
        }
    }
}

const QUOTES: [char; 3] = ['"', '\'', '`'];

// A plain reference is `[^ \t\n\r(:)]+` without delimiters that opens no
// comment. Otherwise the name is quoted with a delimiter it does not start
// with, preferring one it does not contain, and a delimiter inside is doubled.
fn quote_name(name: &str) -> String {
    if name.is_empty() {
        return "\"\"".to_owned();
    }
    let special = |character: char| {
        matches!(character, ' ' | '\t' | '\n' | '\r' | '(' | ')' | ':')
            || QUOTES.contains(&character)
    };
    if !name.contains(special) && !name.starts_with('#') {
        return name.to_owned();
    }
    let quote = QUOTES
        .into_iter()
        .find(|candidate| !name.contains(*candidate))
        .or_else(|| {
            QUOTES
                .into_iter()
                .find(|candidate| !name.starts_with(*candidate))
        })
        .expect("a name starts with at most one delimiter");
    let doubled = name.replace(quote, &format!("{quote}{quote}"));
    format!("{quote}{doubled}{quote}")
}

fn longest_run(text: &str, character: char) -> usize {
    let mut longest = 0;
    let mut run = 0;
    for current in text.chars() {
        run = if current == character { run + 1 } else { 0 };
        longest = longest.max(run);
    }
    longest
}

const fn within(inner: Option<SourceSpan>, outer: Option<SourceSpan>) -> bool {
    let (Some(inner), Some(outer)) = (inner, outer) else {
        return false;
    };
    outer.byte_range().start() <= inner.byte_range().start()
        && inner.byte_range().end() <= outer.byte_range().end()
}

/// The raw parse, as the official PEG grammar returns it.
enum RawItem {
    Reference {
        id: String,
        start: usize,
        end: usize,
    },
    Group {
        nested: Vec<Self>,
        start: usize,
        end: usize,
    },
    Link {
        id: Option<String>,
        values: Vec<Self>,
        children: Vec<Self>,
        start: usize,
        end: usize,
    },
}

impl RawItem {
    const fn span(&self) -> (usize, usize) {
        match self {
            Self::Reference { start, end, .. }
            | Self::Group { start, end, .. }
            | Self::Link { start, end, .. } => (*start, *end),
        }
    }

    fn children(&self) -> &[Self] {
        match self {
            Self::Link { children, .. } => children,
            _ => &[],
        }
    }

    // An indented id: `name:` with no values and the lines below it.
    const fn is_indented_id(&self) -> bool {
        matches!(self, Self::Link { id: Some(_), values, children, .. }
            if values.is_empty() && !children.is_empty())
    }
}

/// A link as the official parser builds it, kept in an arena so a path
/// element shared by several combined links stays one link. No values and a
/// name is a reference. `start` and `end` are byte offsets of its source.
struct Official {
    id: Option<String>,
    values: Vec<usize>,
    start: usize,
    end: usize,
}

struct LinoSemantics<'a> {
    network: &'a mut LinkNetwork,
    text: &'a str,
    lines: &'a LineIndex,
    language: &'a str,
    links: Vec<Official>,
    ids: Vec<Option<LinkId>>,
}

impl<'a> LinoSemantics<'a> {
    const fn new(
        network: &'a mut LinkNetwork,
        text: &'a str,
        lines: &'a LineIndex,
        language: &'a str,
    ) -> Self {
        Self {
            network,
            text,
            lines,
            language,
            links: Vec::new(),
            ids: Vec::new(),
        }
    }

    fn insert_document(mut self) -> Vec<LinkId> {
        let document = parse_lino_cst(self.text);
        let mut links = Vec::new();
        for child in &document.children {
            if child.term == "link" || child.term == "group" {
                let item = self.raw_item(child);
                self.collect_links(&item, &[], &mut links);
            }
        }
        self.ids = vec![None; self.links.len()];
        for &link in &links {
            self.allocate(link);
        }
        let mut connected = HashSet::new();
        for &link in &links {
            self.connect(link, &mut connected);
        }
        links.into_iter().map(|link| self.id_for(link)).collect()
    }

    fn raw_item(&self, node: &GrammarNode) -> RawItem {
        let (start, end) = (node.start, node.end);
        match node.term {
            "reference" => RawItem::Reference {
                id: self.text[start..end].to_owned(),
                start,
                end,
            },
            "quoted_reference" => RawItem::Reference {
                id: quoted_reference_value(self.text, start, end),
                start,
                end,
            },
            "group" => RawItem::Group {
                nested: self.fields(node, "link"),
                start,
                end,
            },
            _ => RawItem::Link {
                id: self
                    .fields(node, "id")
                    .into_iter()
                    .next()
                    .map(|id| match id {
                        RawItem::Reference { id, .. } => id,
                        RawItem::Group { .. } | RawItem::Link { .. } => String::new(),
                    }),
                values: self.fields(node, "value"),
                children: self.fields(node, "child"),
                start,
                end,
            },
        }
    }

    fn fields(&self, node: &GrammarNode, field: &str) -> Vec<RawItem> {
        node.children
            .iter()
            .filter(|child| child.field == Some(field))
            .map(|child| self.raw_item(child))
            .collect()
    }

    // ---- The official transformation (`links-notation/src/Parser.js`) --------

    fn official(
        &mut self,
        id: Option<String>,
        values: Vec<usize>,
        start: usize,
        end: usize,
    ) -> usize {
        self.links.push(Official {
            id,
            values,
            start,
            end,
        });
        self.links.len() - 1
    }

    fn collect_links(&mut self, item: &RawItem, parent_path: &[usize], result: &mut Vec<usize>) {
        if let (
            true,
            RawItem::Link {
                id,
                children,
                start,
                end,
                ..
            },
        ) = (item.is_indented_id(), item)
        {
            let values = children
                .iter()
                .map(|child| self.transform_indented_value(child))
                .collect();
            let current = self.official(id.clone(), values, *start, *end);
            let combined = self.combine_path_elements(parent_path, current);
            result.push(combined);
            return;
        }
        let current = self.transform_link(item);
        let combined = self.combine_path_elements(parent_path, current);
        result.push(combined);
        if item.children().is_empty() {
            return;
        }
        let mut path = parent_path.to_vec();
        path.push(current);
        for child in item.children() {
            self.collect_links(child, &path, result);
        }
    }

    fn transform_indented_value(&mut self, item: &RawItem) -> usize {
        let (start, end) = item.span();
        if let (true, RawItem::Link { id, children, .. }) = (item.is_indented_id(), item) {
            let values = children
                .iter()
                .map(|child| self.transform_indented_value(child))
                .collect();
            return self.official(id.clone(), values, start, end);
        }
        let current = self.transform_link(item);
        if !item.children().is_empty() {
            let mut values = self.links[current].values.clone();
            for child in item.children() {
                values.push(self.transform_indented_value(child));
            }
            let id = self.links[current].id.clone();
            return self.official(id, values, start, end);
        }
        if matches!(item, RawItem::Link { id: None, .. }) && self.links[current].values.len() == 1 {
            return self.links[current].values[0];
        }
        current
    }

    fn combine_path_elements(&mut self, path: &[usize], current: usize) -> usize {
        let Some((&last, rest)) = path.split_last() else {
            return current;
        };
        let parent = if rest.is_empty() {
            last
        } else {
            self.combine_path_elements(rest, last)
        };
        let start = self.links[parent].start.min(self.links[current].start);
        let end = self.links[parent].end.max(self.links[current].end);
        self.official(None, vec![parent, current], start, end)
    }

    fn transform_nested(&mut self, nested: &[RawItem], start: usize, end: usize) -> usize {
        let mut links = Vec::new();
        for item in nested {
            self.collect_links(item, &[], &mut links);
        }
        let wraps_single_group = nested.len() == 1 && matches!(nested[0], RawItem::Group { .. });
        if links.len() == 1 && !wraps_single_group {
            return links[0];
        }
        self.official(None, links, start, end)
    }

    fn transform_link(&mut self, item: &RawItem) -> usize {
        match item {
            RawItem::Group { nested, start, end } => self.transform_nested(nested, *start, *end),
            RawItem::Reference { id, start, end } => {
                self.official(Some(id.clone()), Vec::new(), *start, *end)
            }
            RawItem::Link {
                id,
                values,
                start,
                end,
                ..
            } => {
                let values = values
                    .iter()
                    .map(|value| self.transform_link(value))
                    .collect();
                self.official(id.clone(), values, *start, *end)
            }
        }
    }

    // ---- Links into the network ----------------------------------------------

    fn is_reference(&self, link: usize) -> bool {
        self.links[link].values.is_empty() && self.links[link].id.is_some()
    }

    // Every link becomes a Relation and every name is registered before any
    // value is resolved, so a name used before its definition, or inside it,
    // still resolves to it.
    fn allocate(&mut self, link: usize) {
        if self.is_reference(link) || self.ids[link].is_some() {
            return;
        }
        let Official { id, start, end, .. } = &self.links[link];
        let mut metadata = LinkMetadata::new()
            .with_link_type(LinkType::Relation)
            .with_named(id.is_some())
            .with_language(self.language)
            .with_span(self.span(*start, *end));
        if let Some(name) = id {
            metadata = metadata.with_term(name);
        }
        self.ids[link] = Some(self.network.insert_dynamic_link(&[], metadata));
        for value in self.links[link].values.clone() {
            self.allocate(value);
        }
    }

    fn connect(&mut self, link: usize, connected: &mut HashSet<usize>) {
        if self.is_reference(link) || !connected.insert(link) {
            return;
        }
        let values = self.links[link].values.clone();
        for &value in &values {
            self.connect(value, connected);
        }
        let references: Vec<LinkId> = values.into_iter().map(|value| self.id_for(value)).collect();
        let id = self.ids[link].expect("an allocated link");
        self.network.set_references(id, &references);
    }

    fn id_for(&mut self, link: usize) -> LinkId {
        if !self.is_reference(link) {
            return self.ids[link].expect("an allocated link");
        }
        let name = self.links[link].id.clone().unwrap_or_default();
        self.network.find_term(&name).unwrap_or_else(|| {
            self.network
                .insert_typed_point(&name, LinkType::Concept, None)
        })
    }

    fn span(&self, start: usize, end: usize) -> SourceSpan {
        SourceSpan::new(
            ByteRange::new(start, end),
            self.lines.byte_point(start),
            self.lines.byte_point(end),
        )
    }
}
