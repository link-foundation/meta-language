//! Tree nodes shared by the built-in grammars (`lino_grammar`, `pdf_grammar`)
//! and mirrored by `js/src/builtin-grammar.js`.
//!
//! A node has a term, whether it is named, an optional field name, a byte
//! range and its children, and is flagged as an `ERROR` node, a missing node,
//! a node containing either, or an extra. A token is a node without children
//! that covers text; a missing node is zero-width and has no token.

/// A node of a built-in grammar CST over byte offsets.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct GrammarNode {
    pub term: &'static str,
    pub named: bool,
    pub field: Option<&'static str>,
    pub start: usize,
    pub end: usize,
    pub is_error: bool,
    pub is_missing: bool,
    pub has_error: bool,
    pub extra: bool,
    pub children: Vec<Self>,
}

impl GrammarNode {
    /// A named node.
    pub const fn node(term: &'static str, start: usize, end: usize, children: Vec<Self>) -> Self {
        Self {
            term,
            named: true,
            field: None,
            start,
            end,
            is_error: false,
            is_missing: false,
            has_error: false,
            extra: false,
            children,
        }
    }

    /// An anonymous token.
    pub const fn anonymous(term: &'static str, start: usize, end: usize) -> Self {
        let mut token = Self::node(term, start, end, Vec::new());
        token.named = false;
        token
    }

    /// An extra token, such as whitespace or a comment.
    pub const fn extra(term: &'static str, named: bool, start: usize, end: usize) -> Self {
        let mut token = Self::node(term, start, end, Vec::new());
        token.named = named;
        token.extra = true;
        token
    }

    /// A zero-width node the grammar requires at `at` but the text lacks.
    pub const fn missing(term: &'static str, named: bool, at: usize) -> Self {
        let mut node = Self::node(term, at, at, Vec::new());
        node.named = named;
        node.is_missing = true;
        node
    }

    /// An `ERROR` node keeping malformed input.
    pub const fn error(start: usize, end: usize, children: Vec<Self>) -> Self {
        let mut node = Self::node("ERROR", start, end, children);
        node.is_error = true;
        node
    }

    /// A named node spanning its first to its last child.
    pub fn spanning(term: &'static str, children: Vec<Self>) -> Self {
        let start = children.first().map_or(0, |child| child.start);
        let end = children.last().map_or(0, |child| child.end);
        Self::node(term, start, end, children)
    }

    /// This node as the child of its parent in `field`.
    pub const fn with_field(mut self, field: &'static str) -> Self {
        self.field = Some(field);
        self
    }

    /// Whether this node is a token: a node without children covering text.
    pub fn is_token(&self) -> bool {
        self.children.is_empty() && self.start < self.end
    }
}

/// Gives `root` and every node below it the text between (and, for the root,
/// around) its children as extras, which `lex_gap(start, end)` returns as
/// nodes covering exactly that range, so each extra is owned by the smallest
/// node that spans it, as tree-sitter places extras.
pub fn fill_extras(
    mut root: GrammarNode,
    lex_gap: &mut impl FnMut(usize, usize) -> Vec<GrammarNode>,
) -> GrammarNode {
    fill(&mut root, true, lex_gap);
    root
}

fn fill(
    parent: &mut GrammarNode,
    is_root: bool,
    lex_gap: &mut impl FnMut(usize, usize) -> Vec<GrammarNode>,
) {
    if !is_root && parent.children.is_empty() {
        return;
    }
    let mut children = Vec::new();
    let mut covered = parent.start;
    for mut child in std::mem::take(&mut parent.children) {
        if child.start > covered {
            children.extend(lex_gap(covered, child.start));
        }
        fill(&mut child, false, lex_gap);
        covered = covered.max(child.end);
        children.push(child);
    }
    if parent.end > covered {
        children.extend(lex_gap(covered, parent.end));
    }
    parent.children = children;
}

/// Sets `has_error` on every node that is, or contains, an error or missing
/// node, and returns whether `tree` does.
pub fn propagate_errors(tree: &mut GrammarNode) -> bool {
    let mut has_error = tree.is_error || tree.is_missing;
    for child in &mut tree.children {
        has_error = propagate_errors(child) || has_error;
    }
    if has_error {
        tree.has_error = true;
    }
    has_error
}
