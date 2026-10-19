// Generated from complete owned Rust CST declarations. Do not edit by hand.
use super::forking::Lead;
use super::operations::OperationValue;
use super::program::{Name, PrecedenceTag, TokenRank};
use std::cell::{OnceCell, RefCell};
use std::collections::BTreeMap;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
/// The type of a syntax tree node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TreeType {
    Node,
    Token,
    Error,
    Missing,
    Embed,
}

/// The children of a node, shared between results.
pub(super) type Children = Rc<ChildList>;

/// A list of children. Copying them on every join made a repetition of n
/// items cost O(n²) time and memory, as each iteration (and every prefix the
/// generalized repetition keeps) copied all the children before it. A join
/// instead links its two parts with their child count, and the list is
/// flattened, once, when it is first read; the link is then dropped. A node
/// built from a result shares its list, so the nodes of the prefixes of a
/// repetition are not flattened either.
#[derive(Default)]
pub(super) struct ChildList {
    pub(super) count: usize,
    pub(super) flat: OnceCell<Vec<Rc<Tree>>>,
    pub(super) link: RefCell<Option<(Children, Children)>>,
}

impl ChildList {
    fn flatten(&self) -> Vec<Rc<Tree>> {
        let Some((left, right)) = self.link.borrow_mut().take() else {
            return Vec::new();
        };
        let mut flat = Vec::with_capacity(self.count);
        let mut pending = vec![right, left];
        while let Some(part) = pending.pop() {
            if let Some(items) = part.flat.get() {
                flat.extend_from_slice(items);
            } else if let Some((before, after)) = part.link.borrow().clone() {
                pending.push(after);
                pending.push(before);
            }
        }
        flat
    }
}

impl Deref for ChildList {
    type Target = [Rc<Tree>];

    fn deref(&self) -> &[Rc<Tree>] {
        self.flat.get_or_init(|| self.flatten())
    }
}

impl fmt::Debug for ChildList {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

// Unlinks a long chain one link at a time instead of recursively.
impl Drop for ChildList {
    fn drop(&mut self) {
        let mut next = self.link.get_mut().take();
        while let Some((left, _)) = next {
            next = Rc::try_unwrap(left)
                .ok()
                .and_then(|mut list| list.link.get_mut().take());
        }
    }
}

/// A flat list of children.
pub(super) fn children_of(items: Vec<Rc<Tree>>) -> Children {
    Rc::new(ChildList {
        count: items.len(),
        flat: OnceCell::from(items),
        link: RefCell::new(None),
    })
}

/// The children of `left` followed by those of `right`, linked.
pub(super) fn join_children(left: &Children, right: &Children) -> Children {
    if right.count == 0 {
        return left.clone();
    }
    if left.count == 0 {
        return right.clone();
    }
    Rc::new(ChildList {
        count: left.count + right.count,
        flat: OnceCell::new(),
        link: RefCell::new(Some((left.clone(), right.clone()))),
    })
}

/// One node of the executor's syntax tree.
#[derive(Clone, Debug)]
#[allow(clippy::struct_excessive_bools)] // the flags of one node, as on the JavaScript tree
pub(super) struct Tree {
    pub(super) ty: TreeType,
    pub(super) kind: Option<Name>,
    /// An extra's outer alias; internal lexer decisions keep `kind` unchanged.
    pub(super) projection_name: Option<Name>,
    pub(super) projection_anonymous: bool,
    pub(super) rule: Option<Name>,
    pub(super) field: Option<Name>,
    pub(super) trivia: bool,
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) children: Children,
    pub(super) attributes: Option<BTreeMap<String, OperationValue>>,
    /// Actual own-key order, separate from historical unordered equality.
    pub(super) attribute_order: Option<Vec<String>>,
    /// Lossless source-local lexer metadata, including unknown linked keys.
    pub(super) lexical: Option<serde_json::Value>,
    pub(super) precedence: Option<PrecedenceTag>,
    /// The innermost precedence over the node's last part, which its rule
    /// reduces with (see `reduction`).
    pub(super) tail: Option<PrecedenceTag>,
    /// A token leaf's lexical precedence where it was matched under one
    /// (see `token_rank`).
    pub(super) priority: Option<i64>,
    /// The precedence a token leaf ending a silent rule was reduced with
    /// (see `lone_reduction`), or a node reduced alone under one (see
    /// `shift_order`).
    pub(super) reduced: Option<PrecedenceTag>,
    /// The precedence a node ending a silent rule was reduced with (see
    /// `lone_reduction`).
    pub(super) closes: Option<PrecedenceTag>,
    /// The silent rules the precedence orders name that reduced the item
    /// alone, innermost first (see `child_parting`).
    pub(super) reduced_to: Vec<Name>,
    /// The silent rules a declared conflict names that reduced the item
    /// alone, innermost first (see `forked_order`).
    pub(super) forked_to: Vec<Name>,
    /// A token leaf a silent rule reduced alone (see `preferred_tokens`).
    pub(super) alone: bool,
    /// The token a node's rule was reduced before, where its optional parts
    /// could begin with it (see `reduced_early`).
    pub(super) before: Option<Lead>,
    /// The dynamic precedence of a node's subtree (see `leading_dynamic`).
    pub(super) dynamic: i64,
    pub(super) ambiguous: bool,
    pub(super) literal: bool,
    /// A token of an external scanner, whatever an alias names it (see
    /// `preferred_tokens`).
    pub(super) scanned: bool,
    /// Under keyword lexing, the kind of the token rule that built the leaf,
    /// which an alias keeps.
    pub(super) lexed: Option<Name>,
    /// Under keyword lexing, a plain literal leaf whose text the grammar also
    /// takes as an immediate token (see `KeywordLexing`).
    pub(super) plain: bool,
    /// The rank of the token an alias named the leaf (see `expressions` of
    /// `TokenRanks`).
    pub(super) rank: Option<TokenRank>,
    pub(super) language: Option<Name>,
    pub(super) root: Option<Rc<Self>>,
    pub(super) program: usize,
    pub(super) selection: Option<Rc<super::grammar_selection::SelectionMetadata>>,
    pub(super) source_children: Option<Children>,
}

impl Tree {
    pub(super) fn new(ty: TreeType, kind: Option<Name>, start: usize, end: usize) -> Self {
        Self {
            ty,
            kind,
            projection_name: None,
            projection_anonymous: false,
            rule: None,
            field: None,
            trivia: false,
            start,
            end,
            children: no_children(),
            attributes: None,
            attribute_order: None,
            lexical: None,
            precedence: None,
            tail: None,
            priority: None,
            reduced: None,
            closes: None,
            reduced_to: Vec::new(),
            forked_to: Vec::new(),
            alone: false,
            before: None,
            dynamic: 0,
            ambiguous: false,
            literal: false,
            scanned: false,
            lexed: None,
            plain: false,
            rank: None,
            language: None,
            root: None,
            program: 0,
            selection: None,
            source_children: None,
        }
    }

    /// A node of `kind` built by rule `kind`.
    pub(super) fn node(kind: &Name, start: usize, end: usize, children: Children) -> Self {
        let mut node = Self::new(TreeType::Node, Some(kind.clone()), start, end);
        node.rule = Some(kind.clone());
        node.children = children;
        node
    }

    pub(super) fn trivia_leaf(kind: Option<Name>, start: usize, end: usize) -> Self {
        let mut leaf = Self::new(TreeType::Token, kind, start, end);
        leaf.trivia = true;
        leaf
    }
}

pub(super) fn no_children() -> Children {
    children_of(Vec::new())
}
// Bind every copied declaration without relaxing native lints.
const _: fn() = || {
    let _ = self::TreeType::Node;
    let _ = self::TreeType::Token;
    let _ = self::TreeType::Error;
    let _ = self::TreeType::Missing;
    let _ = self::TreeType::Embed;
    let _ = |value: &self::ChildList| {
        let _ = (&value.count, &value.flat, &value.link);
    };
    let _ = self::children_of;
    let _ = self::join_children;
    let _ = |value: &self::Tree| {
        let _ = (
            &value.ty,
            &value.kind,
            &value.projection_name,
            &value.projection_anonymous,
            &value.rule,
            &value.field,
            &value.trivia,
            &value.start,
            &value.end,
            &value.children,
            &value.attributes,
            &value.attribute_order,
            &value.lexical,
            &value.precedence,
            &value.tail,
            &value.priority,
            &value.reduced,
            &value.closes,
            &value.reduced_to,
            &value.forked_to,
            &value.alone,
            &value.before,
            &value.dynamic,
            &value.ambiguous,
            &value.literal,
            &value.scanned,
            &value.lexed,
            &value.plain,
            &value.rank,
            &value.language,
            &value.root,
            &value.program,
            &value.selection,
            &value.source_children,
        );
    };
    let _ = self::Tree::new;
    let _ = self::Tree::node;
    let _ = self::Tree::trivia_leaf;
    let _ = self::no_children;
};
