//! The public concrete syntax tree of the native grammar executor and its
//! compact, stable text rendering (docs/grammar/feature-union.md#syntax-tree),
//! as `js/src/grammar-runtime/syntax-tree.js`. Every node keeps its byte span,
//! every token its exact text, and trivia, ERROR, MISSING and
//! embedded-language nodes stay in the tree, so the tree reproduces the input
//! byte for byte.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use super::operations::OperationValue;
use super::program::Compiled;
use super::results::{Tree, TreeType};
use super::text::{hex_of, quote_text, text_of};

/// The attributes semantic actions set on a node or token, by name.
pub type SyntaxAttributes = BTreeMap<String, OperationValue>;

/// The text of a leaf: the exact UTF-8 text, or the hexadecimal spelling of
/// bytes that are not well-formed UTF-8.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LeafText {
    /// Well-formed UTF-8 text.
    Text(String),
    /// The lower-case hexadecimal spelling of the bytes.
    Hex(String),
}

impl LeafText {
    fn of(bytes: &[u8], start: usize, end: usize) -> Self {
        text_of(bytes, start, end).map_or_else(|| Self::Hex(hex_of(bytes, start, end)), Self::Text)
    }

    fn render(&self) -> String {
        match self {
            Self::Text(text) => quote_text(text),
            Self::Hex(hex) => format!("<{hex}>"),
        }
    }
}

/// One node of a concrete syntax tree; offsets are byte offsets into the input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyntaxTree {
    /// A node a rule (or an alias) built.
    Node {
        /// The node kind.
        kind: String,
        /// The field the node is captured under.
        field: Option<String>,
        /// Whether the node is trivia (an extra of a rule that builds a node).
        trivia: bool,
        /// The first byte.
        start: usize,
        /// The byte after the last.
        end: usize,
        /// The children, trivia included.
        children: Vec<Self>,
        /// The attributes semantic actions set.
        attributes: Option<SyntaxAttributes>,
    },
    /// A leaf: a literal, a token rule, an external token or trivia.
    Token {
        /// The token kind; `None` for an anonymous literal.
        kind: Option<String>,
        /// The field the token is captured under.
        field: Option<String>,
        /// Whether the token is trivia (an extra or a channel token).
        trivia: bool,
        /// The first byte.
        start: usize,
        /// The byte after the last.
        end: usize,
        /// The attributes semantic actions set.
        attributes: Option<SyntaxAttributes>,
        /// The token text.
        text: LeafText,
        /// Lossless source-local lexer metadata; absent on ordinary tokens.
        lexical: Option<serde_json::Value>,
    },
    /// Input error recovery skipped, or the whole input of a refused parse.
    Error {
        /// The first byte.
        start: usize,
        /// The byte after the last.
        end: usize,
        /// The skipped text.
        text: LeafText,
        /// Why the input is an error node, when it is not skipped input.
        reason: Option<String>,
    },
    /// A zero-width node error recovery inserted for an absent item.
    Missing {
        /// The missing rule kind or literal.
        kind: Option<String>,
        /// The offset.
        start: usize,
        /// The offset again: missing nodes are empty.
        end: usize,
        /// Whether `kind` is a literal.
        literal: bool,
    },
    /// A region another language parsed.
    Embed {
        /// The embedded language.
        language: String,
        /// The first byte.
        start: usize,
        /// The byte after the last.
        end: usize,
        /// The tree of the embedded language.
        root: Box<Self>,
        /// Optional inspectable selection policy, origin metadata and witnesses.
        selection: Option<Box<super::grammar_selection::GrammarSelection>>,
        /// Full original host CST children for an explicitly mapped source.
        source_children: Option<Vec<Self>>,
    },
}

impl SyntaxTree {
    /// The first byte of the node.
    #[must_use]
    pub const fn start(&self) -> usize {
        match self {
            Self::Node { start, .. }
            | Self::Token { start, .. }
            | Self::Error { start, .. }
            | Self::Missing { start, .. }
            | Self::Embed { start, .. } => *start,
        }
    }

    /// The byte after the last byte of the node.
    #[must_use]
    pub const fn end(&self) -> usize {
        match self {
            Self::Node { end, .. }
            | Self::Token { end, .. }
            | Self::Error { end, .. }
            | Self::Missing { end, .. }
            | Self::Embed { end, .. } => *end,
        }
    }

    /// The first ERROR or MISSING node, in preorder.
    #[must_use]
    pub fn first_recovery(&self) -> Option<&Self> {
        match self {
            Self::Error { .. } | Self::Missing { .. } => Some(self),
            Self::Node { children, .. } => children.iter().find_map(Self::first_recovery),
            Self::Embed {
                root,
                source_children,
                ..
            } => source_children
                .as_ref()
                .and_then(|children| children.iter().find_map(Self::first_recovery))
                .or_else(|| root.first_recovery()),
            Self::Token { .. } => None,
        }
    }

    pub(super) fn recovery_diagnostic(
        &self,
    ) -> Result<Option<serde_json::Value>, super::GrammarRuntimeError> {
        use serde_json::json;
        let data = match self {
            Self::Error {
                start,
                end,
                text,
                reason,
            } => {
                let mut value = json!({"type":"error","start":start,"end":end});
                match text {
                    LeafText::Text(text) => value["text"] = json!(text),
                    LeafText::Hex(hex) => {
                        value["text"] = serde_json::Value::Null;
                        value["hex"] = json!(hex);
                    }
                }
                if let Some(reason) = reason {
                    value["reason"] = json!(reason);
                }
                return Ok(Some(value));
            }
            Self::Missing {
                start,
                end,
                kind,
                literal,
            } => {
                let mut value = json!({"type":"missing","kind":kind,"start":start,"end":end});
                if *literal {
                    value["literal"] = json!(true);
                }
                return Ok(Some(value));
            }
            Self::Node { children, .. } => {
                for child in children {
                    if let Some(recovery) = child.recovery_diagnostic()? {
                        return Ok(Some(recovery));
                    }
                }
                return Ok(None);
            }
            Self::Token { .. } => return Ok(None),
            Self::Embed {
                root,
                source_children,
                selection,
                ..
            } => {
                if let Some(children) = source_children {
                    for child in children {
                        if let Some(recovery) = child.recovery_diagnostic()? {
                            return Ok(Some(recovery));
                        }
                    }
                }
                let Some(recovery) = root.recovery_diagnostic()? else {
                    return Ok(None);
                };
                if source_children.is_none() {
                    return Ok(Some(recovery));
                }
                let mapping = selection
                    .as_ref()
                    .and_then(|selection| selection.source_mapping.as_ref())
                    .ok_or_else(|| super::GrammarRuntimeError {
                        reason: "selection",
                        message: "a mapped grammar has no source correspondence".to_owned(),
                    })?;
                (recovery, mapping)
            }
        };
        let (mut recovery, mapping) = data;
        let start = recovery["start"]
            .as_u64()
            .and_then(|value| usize::try_from(value).ok());
        let end = recovery["end"]
            .as_u64()
            .and_then(|value| usize::try_from(value).ok());
        let cover = start
            .zip(end)
            .and_then(|(start, end)| {
                super::selection_source_mapping::mapped_cover(mapping, start, end)
            })
            .ok_or_else(|| super::GrammarRuntimeError {
                reason: "selection",
                message: "a grammar mapping has no source cover for this decoded range".to_owned(),
            })?;
        if recovery.get("source").is_none() {
            recovery["source"] = mapping["output"].clone();
        }
        if recovery.get("sourceStart").is_none() {
            recovery["sourceStart"] = recovery["start"].clone();
        }
        if recovery.get("sourceEnd").is_none() {
            recovery["sourceEnd"] = recovery["end"].clone();
        }
        let mut route = recovery["sourceRoute"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        route.push(json!({"output":mapping["output"],"outputStart":start,"outputEnd":end,
            "input":mapping["input"],"inputStart":cover[1],"inputEnd":cover[2],"displayAnchor":cover[0]}));
        recovery["start"] = json!(cover[1]);
        recovery["end"] = json!(cover[2]);
        recovery["sourceRoute"] = json!(route);
        recovery["offsetSemantics"] = json!("covering-source-spelling");
        Ok(Some(recovery))
    }

    /// The compact text rendering: `(kind {attributes} child ...)` for a
    /// node, `field:` before a captured child, the quoted text of an
    /// anonymous token, `(kind "text")` for a named one, `~` before trivia,
    /// and `(ERROR@S..E "text")`, `(MISSING@P kind)` and
    /// `(EMBED language@S..E root)`.
    #[must_use]
    pub fn render(&self) -> String {
        let mut rendered = String::new();
        self.render_into(&mut rendered);
        rendered
    }

    fn render_into(&self, out: &mut String) {
        match self {
            Self::Node {
                kind,
                field,
                trivia,
                children,
                attributes,
                ..
            } => {
                render_field(field.as_deref(), out);
                if *trivia {
                    out.push('~');
                }
                out.push('(');
                out.push_str(&render_name(kind));
                if let Some(attributes) = attributes {
                    out.push(' ');
                    render_attributes(attributes, out);
                }
                for child in children {
                    out.push(' ');
                    child.render_into(out);
                }
                out.push(')');
            }
            Self::Token {
                kind,
                field,
                trivia,
                attributes,
                text,
                ..
            } => {
                render_field(field.as_deref(), out);
                if *trivia {
                    out.push('~');
                }
                if kind.is_none() && attributes.is_none() {
                    out.push_str(&text.render());
                    return;
                }
                out.push('(');
                out.push_str(&kind.as_deref().map_or_else(|| "_".to_owned(), render_name));
                if let Some(attributes) = attributes {
                    out.push(' ');
                    render_attributes(attributes, out);
                }
                out.push(' ');
                out.push_str(&text.render());
                out.push(')');
            }
            Self::Error {
                start, end, text, ..
            } => {
                let _ = write!(out, "(ERROR@{start}..{end} {})", text.render());
            }
            Self::Missing {
                kind,
                start,
                literal,
                ..
            } => match kind {
                None => {
                    let _ = write!(out, "(MISSING@{start})");
                }
                Some(kind) => {
                    let name = if *literal {
                        quote_text(kind)
                    } else {
                        render_name(kind)
                    };
                    let _ = write!(out, "(MISSING@{start} {name})");
                }
            },
            Self::Embed {
                language,
                start,
                end,
                root,
                selection,
                source_children,
            } => {
                let _ = write!(out, "(EMBED {}@{start}..{end} ", render_name(language));
                if let Some(children) = source_children {
                    out.push_str("(SOURCE ");
                    for (index, child) in children.iter().enumerate() {
                        if index > 0 {
                            out.push(' ');
                        }
                        child.render_into(out);
                    }
                    let output = selection
                        .as_ref()
                        .and_then(|selection| selection.source_mapping.as_ref())
                        .and_then(|mapping| mapping["output"].as_u64());
                    if let Some(output) = output {
                        let _ = write!(out, ") (DECODED {output} ");
                        root.render_into(out);
                        out.push(')');
                    } else {
                        out.push_str(") (INVALID-SOURCE-MAPPING)");
                    }
                } else {
                    root.render_into(out);
                }
                out.push(')');
            }
        }
    }
}

fn render_field(field: Option<&str>, out: &mut String) {
    if let Some(field) = field {
        out.push_str(&render_name(field));
        out.push(':');
    }
}

fn render_name(name: &str) -> String {
    let mut chars = name.chars();
    let plain = chars
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if plain {
        name.to_owned()
    } else {
        quote_text(name)
    }
}

fn render_attributes(attributes: &SyntaxAttributes, out: &mut String) {
    out.push('{');
    for (index, (name, value)) in attributes.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push_str(&render_name(name));
        out.push('=');
        match value {
            OperationValue::Text(text) => out.push_str(&quote_text(text)),
            OperationValue::Integer(integer) => {
                let _ = write!(out, "{integer}");
            }
        }
    }
    out.push('}');
}

/// Copies an executor tree into the public shape.
pub(super) fn public_tree(node: &Tree, bytes: &[u8]) -> SyntaxTree {
    let name = |value: &Option<super::program::Name>| value.as_deref().map(str::to_owned);
    match node.ty {
        TreeType::Node => SyntaxTree::Node {
            kind: name(&node.projection_name)
                .or_else(|| name(&node.kind))
                .unwrap_or_default(),
            field: name(&node.field),
            trivia: node.trivia,
            start: node.start,
            end: node.end,
            children: node
                .children
                .iter()
                .map(|child| public_tree(child, bytes))
                .collect(),
            attributes: node.attributes.clone(),
        },
        TreeType::Token => SyntaxTree::Token {
            kind: if node.projection_anonymous {
                None
            } else {
                name(&node.projection_name).or_else(|| name(&node.kind))
            },
            field: name(&node.field),
            trivia: node.trivia,
            start: node.start,
            end: node.end,
            attributes: node.attributes.clone(),
            text: LeafText::of(bytes, node.start, node.end),
            lexical: node.lexical.clone(),
        },
        TreeType::Error => error_tree(bytes, node.start, node.end, None),
        TreeType::Missing => SyntaxTree::Missing {
            kind: name(&node.kind),
            start: node.start,
            end: node.end,
            literal: node.literal,
        },
        TreeType::Embed => SyntaxTree::Embed {
            language: name(&node.language).unwrap_or_default(),
            start: node.start,
            end: node.end,
            root: Box::new(node.root.as_ref().map_or_else(
                || error_tree(bytes, node.start, node.end, None),
                |root| {
                    public_tree(
                        root,
                        node.selection
                            .as_ref()
                            .and_then(|selection| selection.source_bytes.as_deref())
                            .unwrap_or(bytes),
                    )
                },
            )),
            source_children: node.source_children.as_ref().map(|children| {
                children
                    .iter()
                    .map(|child| public_tree(child, bytes))
                    .collect()
            }),
            selection: node.selection.as_deref().map(|selection| {
                Box::new(super::grammar_selection::public_selection(selection, bytes))
            }),
        },
    }
}

pub(super) fn error_tree(
    bytes: &[u8],
    start: usize,
    end: usize,
    reason: Option<&str>,
) -> SyntaxTree {
    SyntaxTree::Error {
        start,
        end,
        text: LeafText::of(bytes, start, end),
        reason: reason.map(str::to_owned),
    }
}

/// An ambiguous node outside the declared conflicts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ambiguity {
    /// The rule (or alias) that built the node.
    pub rule: String,
    /// The first byte.
    pub start: usize,
    /// The byte after the last.
    pub end: usize,
    /// Explicit accepted grammar alternatives for a language selection.
    pub languages: Option<Vec<String>>,
    /// Original decoded source coordinates and covering physical route.
    pub source: Option<usize>,
    pub source_start: Option<usize>,
    pub source_end: Option<usize>,
    pub source_route: Option<Vec<serde_json::Value>>,
    pub offset_semantics: Option<String>,
}

/// The ambiguous nodes of an executor tree, in preorder, outside the declared conflicts.
pub(super) fn collect_ambiguities(
    node: &Tree,
    compiled: &Compiled,
    program: usize,
    found: &mut Vec<Ambiguity>,
) {
    match node.ty {
        TreeType::Node => {
            let conflicts = &compiled.programs[program].conflicts;
            if node.ambiguous
                && !node
                    .rule
                    .as_deref()
                    .is_some_and(|rule| conflicts.contains(rule))
            {
                found.push(Ambiguity {
                    rule: node
                        .rule
                        .as_deref()
                        .or(node.kind.as_deref())
                        .unwrap_or_default()
                        .to_owned(),
                    start: node.start,
                    end: node.end,
                    languages: None,
                    source: None,
                    source_start: None,
                    source_end: None,
                    source_route: None,
                    offset_semantics: None,
                });
            }
            for child in node.children.iter() {
                collect_ambiguities(child, compiled, program, found);
            }
        }
        TreeType::Embed => {
            if let Some(selection) = &node.selection
                && selection.ambiguous
            {
                found.push(Ambiguity {
                    rule: "any-grammar".to_owned(),
                    start: node.start,
                    end: node.end,
                    languages: Some(
                        selection
                            .candidates
                            .iter()
                            .filter(|candidate| candidate["relation"] == "accepted")
                            .filter_map(|candidate| {
                                candidate["language"].as_str().map(str::to_owned)
                            })
                            .collect(),
                    ),
                    source: None,
                    source_start: None,
                    source_end: None,
                    source_route: None,
                    offset_semantics: None,
                });
            }
            if let Some(children) = &node.source_children {
                for child in children.iter() {
                    collect_ambiguities(child, compiled, program, found);
                }
            }
            let mut decoded = Vec::new();
            let interpreted = if node.source_children.is_some() {
                &mut decoded
            } else {
                &mut *found
            };
            if let Some(root) = &node.root {
                collect_ambiguities(root, compiled, node.program, interpreted);
            }
            if let Some(selection) = &node.selection {
                for alternative in &selection.alternatives {
                    collect_ambiguities(
                        &alternative.root,
                        compiled,
                        alternative.program,
                        interpreted,
                    );
                }
                if let Some(mapping) = &selection.source_mapping {
                    for mut ambiguity in decoded {
                        if let Some(cover) = super::selection_source_mapping::mapped_cover(
                            mapping,
                            ambiguity.start,
                            ambiguity.end,
                        ) {
                            ambiguity.source.get_or_insert_with(|| {
                                mapping["output"]
                                    .as_u64()
                                    .and_then(|source| usize::try_from(source).ok())
                                    .unwrap_or_default()
                            });
                            ambiguity.source_start.get_or_insert(ambiguity.start);
                            ambiguity.source_end.get_or_insert(ambiguity.end);
                            ambiguity.source_route.get_or_insert_with(Vec::new).push(serde_json::json!({"output":mapping["output"],"outputStart":ambiguity.start,"outputEnd":ambiguity.end,
                                "input":mapping["input"],"inputStart":cover[1],"inputEnd":cover[2],"displayAnchor":cover[0]}));
                            ambiguity.start = cover[1];
                            ambiguity.end = cover[2];
                            ambiguity.offset_semantics =
                                Some("covering-source-spelling".to_owned());
                        }
                        found.push(ambiguity);
                    }
                }
            }
        }
        _ => {}
    }
}
