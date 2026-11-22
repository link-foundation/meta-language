use serde_json::Value;

use super::super::feature_runtime::{FeatureParseOptions, LeafText, ParseRejection, SyntaxTree};
use super::{JsonDocumentIssue, parser, pointer_child};
use crate::line_index::LineIndex;
use crate::{ByteRange, LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType, SourceSpan};

pub(super) struct Node<'source> {
    pub kind: String,
    pub text: &'source str,
    pub start: usize,
    pub end: usize,
    pub field: Option<String>,
    pub link_id: LinkId,
    pub span: SourceSpan,
    pub flags: LinkFlags,
    pub children: Vec<Self>,
    pub pointer: String,
}

impl<'source> Node<'source> {
    pub fn is_value(&self) -> bool {
        !self.flags.is_missing()
            && !self.flags.is_extra()
            && matches!(
                self.kind.as_str(),
                "object" | "array" | "string" | "number" | "true" | "false" | "null"
            )
    }

    pub fn field(&self, name: &str) -> Option<&Self> {
        self.children
            .iter()
            .find(|child| child.field.as_deref() == Some(name))
    }

    pub fn key(&self) -> Option<String> {
        self.field("key")
            .filter(|node| !node.flags.has_error())
            .and_then(|node| serde_json::from_str(node.text).ok())
    }

    pub fn postorder<'node>(&'node self, nodes: &mut Vec<&'node Self>) {
        for child in &self.children {
            child.postorder(nodes);
        }
        nodes.push(self);
    }

    pub fn issue(&self, keyword: &str, expected: Vec<Value>) -> JsonDocumentIssue {
        JsonDocumentIssue {
            keyword: keyword.to_owned(),
            pointer: self.pointer.clone(),
            span: self.span,
            link_id: self.link_id,
            expected,
            reason: None,
            pattern: None,
            alternatives: Vec::new(),
            accepted: None,
        }
    }

    pub fn read(&self) -> Option<Value> {
        (!self.flags.has_error())
            .then(|| serde_json::from_str(self.text).ok())
            .flatten()
    }

    fn set_pointer(&mut self, pointer: &str) {
        self.pointer = pointer.to_owned();
        let mut index = 0;
        for child in &mut self.children {
            let child_pointer = if self.kind == "array" && child.is_value() {
                let item = pointer_child(pointer, &index.to_string());
                index += 1;
                item
            } else if self.kind == "object" && child.kind == "pair" {
                child
                    .key()
                    .map_or_else(|| pointer.to_owned(), |key| pointer_child(pointer, &key))
            } else {
                pointer.to_owned()
            };
            child.set_pointer(&child_pointer);
        }
    }
}

pub(super) struct Document<'source> {
    pub root: Node<'source>,
    pub network: LinkNetwork,
    rejection: Option<ParseRejection>,
    pub source: &'source str,
    pub lines: LineIndex,
}

impl<'source> Document<'source> {
    pub fn read(source: &'source str) -> Self {
        let parser = parser();
        let strict = parser
            .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
            .expect("registered JSON document rule");
        let tree = if strict.ok {
            strict.tree
        } else {
            let recovery = FeatureParseOptions {
                error_recovery: Some(true),
                accept_recovery: Some(true),
                ..FeatureParseOptions::default()
            };
            parser
                .parse_tree(source.as_bytes(), &recovery)
                .expect("registered JSON document rule")
                .tree
        };
        let tree = tree.unwrap_or_else(|| SyntaxTree::Error {
            start: 0,
            end: source.len(),
            text: LeafText::Text(source.to_owned()),
            reason: None,
        });
        let lines = LineIndex::new(source);
        let (mut network, _) = LinkNetwork::new_parse_document(source, "JSON");
        let mut root = project(tree, &mut network, source, &lines);
        root.set_pointer("");
        Self {
            root,
            network,
            rejection: strict.rejection,
            source,
            lines,
        }
    }

    pub fn value(&self) -> Option<&Node<'source>> {
        self.root.children.iter().find(|node| node.is_value())
    }

    pub fn nodes(&self) -> Vec<&Node<'source>> {
        let mut nodes = Vec::new();
        self.root.postorder(&mut nodes);
        nodes
    }

    pub fn syntax_issue(&self) -> Option<JsonDocumentIssue> {
        let rejection = self.rejection.as_ref()?;
        let offset = rejection.offset.unwrap_or(self.root.start);
        let nodes = self.nodes();
        let errors: Vec<_> = nodes
            .into_iter()
            .filter(|node| {
                node.start <= offset
                    && offset <= node.end
                    && (node.flags.is_error() || node.flags.is_missing())
            })
            .collect();
        let node = errors
            .iter()
            .copied()
            .find(|node| node.flags.is_missing() && node.start == offset)
            .or_else(|| errors.iter().copied().find(|node| offset < node.end))
            .or_else(|| errors.first().copied())
            .unwrap_or(&self.root);
        let expected = rejection
            .expected
            .iter()
            .flatten()
            .cloned()
            .map(Value::String)
            .collect();
        let mut issue = node.issue("syntax", expected);
        issue.reason = Some(rejection.reason);
        Some(issue)
    }
}

pub(super) fn span(lines: &LineIndex, start: usize, end: usize) -> SourceSpan {
    SourceSpan::new(
        ByteRange::new(start, end),
        lines.byte_point(start),
        lines.byte_point(end),
    )
}

fn project<'source>(
    tree: SyntaxTree,
    network: &mut LinkNetwork,
    source: &'source str,
    lines: &LineIndex,
) -> Node<'source> {
    let start = tree.start();
    let end = tree.end();
    let text = &source[start..end];
    let (kind, field, flags, children, named) = match tree {
        SyntaxTree::Node {
            kind,
            field,
            trivia,
            children,
            ..
        } => {
            let children: Vec<_> = children
                .into_iter()
                .map(|node| project(node, network, source, lines))
                .collect();
            let flags = if children.iter().any(|child| child.flags.has_error()) {
                LinkFlags::containing_error()
            } else if trivia {
                LinkFlags::extra()
            } else {
                LinkFlags::clean()
            };
            (kind, field, flags, children, true)
        }
        SyntaxTree::Token {
            kind,
            field,
            trivia,
            ..
        } => {
            let named = kind.is_some();
            (
                kind.unwrap_or_else(|| text.to_owned()),
                field,
                if trivia {
                    LinkFlags::extra()
                } else {
                    LinkFlags::clean()
                },
                Vec::new(),
                named,
            )
        }
        SyntaxTree::Missing { kind, literal, .. } => {
            let named = kind.is_some() && !literal;
            (
                kind.unwrap_or_default(),
                None,
                LinkFlags::missing(),
                Vec::new(),
                named,
            )
        }
        SyntaxTree::Error { .. } => (
            "ERROR".to_owned(),
            None,
            LinkFlags::error(),
            Vec::new(),
            false,
        ),
        SyntaxTree::Embed { .. } => unreachable!("JSON document grammar has no embedded rules"),
    };
    let first = children
        .iter()
        .find(|child| !child.flags.is_extra())
        .map_or(start, |child| child.span.byte_range().start());
    let last = children
        .iter()
        .rfind(|child| !child.flags.is_extra())
        .map_or(end, |child| child.span.byte_range().end());
    let span = span(lines, first, last);
    let mut references: Vec<_> = children.iter().map(|child| child.link_id).collect();
    let metadata = |link_type, term: &str, named| {
        LinkMetadata::new()
            .with_link_type(link_type)
            .with_language("JSON")
            .with_term(term)
            .with_named(named)
            .with_span(span)
            .with_flags(flags)
    };
    if children.is_empty() && start < end {
        references.push(network.insert_link([], metadata(LinkType::Token, text, named)));
    }
    let link_id =
        network.insert_dynamic_link(&references, metadata(LinkType::Syntax, &kind, named));
    for child in &children {
        if let Some(field) = &child.field {
            network.insert_field(link_id, field, child.link_id);
        }
    }
    Node {
        kind,
        field,
        text,
        start,
        end,
        link_id,
        span,
        flags,
        children,
        pointer: String::new(),
    }
}
