// Generated from complete owned Rust CST declarations. Do not edit by hand.

//! Walks over the leaves of a result and the comparisons of results by the
//! trees they build, which the order of results and the parting of conflicts
//! read (see `ordering` and `parting`).

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::OnceLock;

use super::grammar_selection::{AcceptedSelection, SelectionMetadata};
use super::grammar_selection_rules::{
    read_grammar_selection_equality_fields, read_grammar_tree_equality_fields,
    read_grammar_tree_recursion_plan,
};
use super::results::{Children, Tree, TreeType};

/// One step of a walk over the leaves of a result: a part of a child list,
/// not yet opened, one tree, or the end of the children of a node the walk
/// entered (the node in progress for the items above it).
pub(super) enum Walk {
    Part(Children),
    Item(Rc<Tree>),
    End(Rc<Tree>),
}

/// Opens the part on top of a walk: its items, or its two linked halves,
/// without flattening it.
pub(super) fn open_part(walk: &mut Vec<Walk>) {
    let Some(Walk::Part(part)) = walk.pop() else {
        return;
    };
    if let Some(items) = part.flat.get() {
        walk.extend(items.iter().rev().cloned().map(Walk::Item));
    } else if let Some((before, after)) = part.link.borrow().clone() {
        walk.push(Walk::Part(after));
        walk.push(Walk::Part(before));
    }
}

/// The item on top of a walk once the parts above it are opened and the
/// ends of nodes passed, without descending into a node.
pub(super) fn next_item(walk: &mut Vec<Walk>) -> Option<Rc<Tree>> {
    loop {
        match walk.last()? {
            Walk::End(_) => {
                walk.pop();
            }
            Walk::Part(_) => open_part(walk),
            Walk::Item(item) => return Some(item.clone()),
        }
    }
}

/// Replaces the node on top of a walk with its children, marking their end
/// when `mark` is set.
pub(super) fn open_node(walk: &mut Vec<Walk>, mark: bool) {
    if let Some(Walk::Item(node)) = walk.pop() {
        let children = node.children.clone();
        if mark {
            walk.push(Walk::End(node));
        }
        walk.push(Walk::Part(children));
    }
}

/// The items of a child list in order, without flattening it.
pub(super) struct Items(Vec<Walk>);

impl Iterator for Items {
    type Item = Rc<Tree>;

    fn next(&mut self) -> Option<Rc<Tree>> {
        loop {
            match self.0.last()? {
                Walk::Part(_) => open_part(&mut self.0),
                Walk::Item(_) => {
                    let Some(Walk::Item(item)) = self.0.pop() else {
                        return None;
                    };
                    return Some(item);
                }
                Walk::End(_) => {
                    self.0.pop();
                }
            }
        }
    }
}

pub(super) fn items(children: &Children) -> Items {
    Items(vec![Walk::Part(children.clone())])
}

/// The meaningful (not trivia) items of a child list.
pub(super) fn meaningful(children: &Children) -> Vec<Rc<Tree>> {
    items(children).filter(|child| !child.trivia).collect()
}

/// The pairs of nodes already compared by `same_tree_in`, which a caller
/// keeps while the trees it compares live and none of them changes.
pub(super) type TreeMemo = RefCell<HashMap<(*const Tree, *const Tree), bool>>;

/// Whether two subtrees are the same tree over the same text, whichever of
/// them holds the white space around it.
pub(super) fn same_tree(a: &Rc<Tree>, b: &Rc<Tree>) -> bool {
    same_tree_with(a, b, None)
}

/// `same_tree`, remembering the pairs of nodes it compares in `memo`: two
/// leftmost chains compared node by node share their subtrees, which would
/// otherwise be compared again for every pair above them. It mirrors
/// sameTree in js/src/grammar-runtime/executor.js.
pub(super) fn same_tree_in(a: &Rc<Tree>, b: &Rc<Tree>, memo: &TreeMemo) -> bool {
    same_tree_with(a, b, Some(memo))
}

fn same_tree_with(a: &Rc<Tree>, b: &Rc<Tree>, memo: Option<&TreeMemo>) -> bool {
    if Rc::ptr_eq(a, b) {
        return true;
    }
    if a.ty != b.ty || !same_tree_header(a, b, &grammar_tree_equality_plan(a.ty, false).fields) {
        return false;
    }
    let plan = grammar_tree_equality_plan(a.ty, false);
    if plan.recursion == "leaf" {
        return true;
    }
    let key = (Rc::as_ptr(a), Rc::as_ptr(b));
    if let Some(known) = memo.and_then(|memo| memo.borrow().get(&key).copied()) {
        return known;
    }
    let same = if plan.recursion == "full-embedding" {
        same_embedding(a, b, &TreeMemo::default())
    } else {
        same_children_with(&a.children, &b.children, memo)
    };
    if let Some(memo) = memo {
        memo.borrow_mut().insert(key, same);
    }
    same
}

/// Whether two child lists are the same trees over the same text.
pub(super) fn same_children(a: &Children, b: &Children) -> bool {
    same_children_with(a, b, None)
}

fn same_children_with(a: &Children, b: &Children, memo: Option<&TreeMemo>) -> bool {
    if Rc::ptr_eq(a, b) {
        return true;
    }
    let (first, second) = (meaningful(a), meaningful(b));
    first.len() == second.len()
        && first
            .iter()
            .zip(&second)
            .all(|(x, y)| same_tree_with(x, y, memo))
}

/// Whether two child lists build the same trees, trivia, fields and
/// attributes included: two results that reach them in different ways (Rust's
/// `+.` in a token tree, one run of `(precedence 0 right (repeat1 ...))` or
/// two, both flattened by the silent rule) are one parse, not an ambiguity.
/// It mirrors sameOutput in js/src/grammar-runtime/executor.js.
pub(super) fn same_output(a: &Children, b: &Children) -> bool {
    same_output_with(a, b, &TreeMemo::default())
}

/// Cached projections of the generated portable field and recursive-role plan.
struct GrammarTreeEqualityPlan {
    fields: Vec<String>,
    recursion: String,
}

fn grammar_tree_equality_plan(kind: TreeType, output: bool) -> &'static GrammarTreeEqualityPlan {
    static PLANS: OnceLock<Vec<GrammarTreeEqualityPlan>> = OnceLock::new();
    let plans = PLANS.get_or_init(|| {
        let mut plans = Vec::new();
        for kind in ["node", "token", "error", "missing", "embed"] {
            for output in [false, true] {
                plans.push(GrammarTreeEqualityPlan {
                    fields: read_grammar_tree_equality_fields(kind.to_owned(), output)
                        .split('|')
                        .map(str::to_owned)
                        .collect(),
                    recursion: read_grammar_tree_recursion_plan(kind.to_owned(), output),
                });
            }
        }
        plans
    });
    let index = match kind {
        TreeType::Node => 0,
        TreeType::Token => 1,
        TreeType::Error => 2,
        TreeType::Missing => 3,
        TreeType::Embed => 4,
    };
    &plans[index * 2 + usize::from(output)]
}

fn grammar_selection_equality_fields(alternative: bool) -> &'static [String] {
    static FIELDS: OnceLock<[Vec<String>; 2]> = OnceLock::new();
    &FIELDS.get_or_init(|| {
        [false, true].map(|alternative| {
            read_grammar_selection_equality_fields(alternative)
                .split('|')
                .map(str::to_owned)
                .collect()
        })
    })[usize::from(alternative)]
}

/// Metadata keeps array order, exact key presence and linked object-key order.
fn same_tree_metadata(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    use serde_json::Value;
    match (a, b) {
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| same_tree_metadata(a, b))
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter().zip(b).all(|((key, value), (other_key, other))| {
                    key == other_key && same_tree_metadata(value, other)
                })
        }
        _ => a == b,
    }
}

fn same_tree_header(a: &Tree, b: &Tree, fields: &[String]) -> bool {
    fields.iter().all(|field| match field.as_str() {
        "type" => a.ty == b.ty,
        "kind" => a.kind == b.kind,
        "projectionName" => a.projection_name == b.projection_name,
        "projectionAnonymous" => a.projection_anonymous == b.projection_anonymous,
        "start" => a.start == b.start,
        "end" => a.end == b.end,
        "trivia" => a.trivia == b.trivia,
        "literal" => a.literal == b.literal,
        "field" => a.field == b.field,
        "language" => a.language == b.language,
        "attributes" => a.attributes == b.attributes,
        "lexical" => match (&a.lexical, &b.lexical) {
            (None, None) => true,
            (Some(a), Some(b)) => same_tree_metadata(a, b),
            _ => false,
        },
        _ => false,
    })
}

fn same_output_with(a: &Children, b: &Children, memo: &TreeMemo) -> bool {
    Rc::ptr_eq(a, b)
        || (a.count == b.count
            && items(a)
                .zip(items(b))
                .all(|(a, b)| same_output_tree(&a, &b, memo)))
}

fn same_output_tree(a: &Rc<Tree>, b: &Rc<Tree>, memo: &TreeMemo) -> bool {
    if Rc::ptr_eq(a, b) {
        return true;
    }
    let plan = grammar_tree_equality_plan(a.ty, true);
    if a.ty != b.ty || !same_tree_header(a, b, &plan.fields) {
        return false;
    }
    let key = (Rc::as_ptr(a), Rc::as_ptr(b));
    if let Some(known) = memo.borrow().get(&key).copied() {
        return known;
    }
    let same = match plan.recursion.as_str() {
        "full-children" => same_output_with(&a.children, &b.children, memo),
        "full-embedding" => same_embedding(a, b, memo),
        "leaf" => true,
        _ => false,
    };
    memo.borrow_mut().insert(key, same);
    same
}

fn same_embedding(a: &Tree, b: &Tree, memo: &TreeMemo) -> bool {
    let roots = match (&a.root, &b.root) {
        (None, None) => true,
        (Some(a), Some(b)) => same_output_tree(a, b, memo),
        _ => false,
    };
    roots
        && match (&a.selection, &b.selection) {
            (None, None) => true,
            (Some(a), Some(b)) => same_selection(a, b, memo),
            _ => false,
        }
}

fn same_selection(a: &SelectionMetadata, b: &SelectionMetadata, memo: &TreeMemo) -> bool {
    grammar_selection_equality_fields(false)
        .iter()
        .all(|field| match field.as_str() {
            "mode" => a.mode == b.mode,
            "mapping" => a.mapping == b.mapping,
            "start" => a.start == b.start,
            "end" => a.end == b.end,
            "candidates" => {
                a.candidates.len() == b.candidates.len()
                    && a.candidates
                        .iter()
                        .zip(&b.candidates)
                        .all(|(a, b)| same_tree_metadata(a, b))
            }
            "provenance" => same_tree_metadata(&a.provenance, &b.provenance),
            "ambiguous" => a.ambiguous == b.ambiguous,
            _ => false,
        })
        && a.alternatives.len() == b.alternatives.len()
        && a.alternatives
            .iter()
            .zip(&b.alternatives)
            .all(|(a, b)| same_selection_alternative(a, b, memo))
}

fn same_selection_alternative(
    a: &AcceptedSelection,
    b: &AcceptedSelection,
    memo: &TreeMemo,
) -> bool {
    grammar_selection_equality_fields(true)
        .iter()
        .all(|field| match field.as_str() {
            "language" => a.language == b.language,
            "provenance" => same_tree_metadata(&a.provenance, &b.provenance),
            _ => false,
        })
        && same_output_tree(&a.root, &b.root, memo)
}

/// The first child of a list that is not trivia, without flattening it.
pub(super) fn first_meaningful(children: &Children) -> Option<Rc<Tree>> {
    items(children).find(|child| !child.trivia)
}

/// The offset of the first leaf under a node that is not white space.
pub(super) fn first_leaf_start(node: &Rc<Tree>) -> usize {
    let mut current = node.clone();
    while current.ty == TreeType::Node {
        let Some(first) = first_meaningful(&current.children) else {
            return current.start;
        };
        current = first;
    }
    current.start
}

/// The nodes along the leftmost chain of a node: itself, then its first
/// meaningful child while that is a node.
pub(super) fn leftmost_chain(node: &Rc<Tree>) -> Vec<Rc<Tree>> {
    let mut chain = Vec::new();
    let mut current = Some(node.clone());
    while let Some(next) = current.filter(|next| next.ty == TreeType::Node) {
        current = first_meaningful(&next.children);
        chain.push(next);
    }
    chain
}

mod tests {
    //! Output equality reads persistent child lists without materializing them.
    use std::collections::BTreeMap;
    use std::rc::Rc;

    use super::super::operations::OperationValue;
    use super::super::results::{Children, Tree, TreeType, children_of, join_children};
    use super::same_output;

    fn token(start: usize) -> Rc<Tree> {
        Rc::new(Tree::new(
            TreeType::Token,
            Some("word".into()),
            start,
            start + 1,
        ))
    }

    fn assert_linked(children: &Children, count: usize) {
        assert_eq!(children.count, count);
        assert!(children.flat.get().is_none());
        assert!(children.link.borrow().is_some());
    }

    #[test]
    fn equal_outputs_preserve_different_lazy_partitions_and_shared_aliases() {
        let first = token(0);
        let second = token(1);
        let third = token(2);
        let left = join_children(
            &children_of(vec![first.clone()]),
            &children_of(vec![second.clone(), third.clone()]),
        );
        let right = join_children(&children_of(vec![first, second]), &children_of(vec![third]));
        assert!(same_output(&left, &right));
        assert!(same_output(&left, &left));
        assert_linked(&left, 3);
        assert_linked(&right, 3);

        let original = token(3);
        let copied = Rc::new((*original).clone());
        let shared = join_children(
            &children_of(vec![original.clone()]),
            &children_of(vec![original]),
        );
        let distinct = join_children(&children_of(vec![copied]), &children_of(vec![token(3)]));
        assert!(same_output(&shared, &distinct));
        assert_linked(&shared, 2);
        assert_linked(&distinct, 2);
    }

    #[test]
    fn different_counts_and_order_refuse_without_flattening() {
        let first = token(0);
        let second = token(1);
        let forward = join_children(
            &children_of(vec![first.clone()]),
            &children_of(vec![second.clone()]),
        );
        let longer = join_children(
            &children_of(vec![first.clone()]),
            &children_of(vec![second.clone(), token(2)]),
        );
        let reversed = join_children(&children_of(vec![second]), &children_of(vec![first]));
        assert!(!same_output(&forward, &longer));
        assert!(!same_output(&forward, &reversed));
        assert_linked(&forward, 2);
        assert_linked(&longer, 3);
        assert_linked(&reversed, 2);
    }

    #[test]
    fn nested_output_metadata_remains_distinct_without_materializing_children() {
        let original = token(0);
        let tail = token(1);
        let children = join_children(
            &children_of(vec![original.clone()]),
            &children_of(vec![tail.clone()]),
        );
        let parent_kind = "parent".into();
        let reference = children_of(vec![Rc::new(Tree::node(
            &parent_kind,
            0,
            2,
            children.clone(),
        ))]);
        let changes: [fn(&mut Tree); 8] = [
            |tree| tree.ty = TreeType::Error,
            |tree| tree.kind = Some("other".into()),
            |tree| tree.start = 1,
            |tree| tree.end = 2,
            |tree| tree.trivia = true,
            |tree| tree.field = Some("field".into()),
            |tree| tree.language = Some("language".into()),
            |tree| {
                tree.attributes = Some(BTreeMap::from([(
                    "attribute".to_owned(),
                    OperationValue::Integer(1),
                )]));
            },
        ];
        for change in changes {
            let mut different = (*original).clone();
            change(&mut different);
            let different_children = join_children(
                &children_of(vec![Rc::new(different)]),
                &children_of(vec![tail.clone()]),
            );
            let candidate = children_of(vec![Rc::new(Tree::node(
                &parent_kind,
                0,
                2,
                different_children.clone(),
            ))]);
            assert!(!same_output(&reference, &candidate));
            assert_linked(&children, 2);
            assert_linked(&different_children, 2);
        }
    }

    // Appended by the source-owned test model generator; the original three cases
    // remain byte exact in walk_cases.rs. These use actual private declarations.

    fn selection_equality_root(language: &str) -> Rc<Tree> {
        let mut space = Tree::new(TreeType::Token, None, 0, 1);
        space.trivia = true;
        let mut word = Tree::new(TreeType::Token, Some("word".into()), 1, 3);
        word.field = Some("value".into());
        word.lexical = Some(serde_json::json!({
            "lexer": language, "rule": "word", "type": "word", "channel": "default", "disposition": "emit",
            "future": {"ordered": ["a", "b"]}
        }));
        Rc::new(Tree::node(
            &format!("{language}_root").into(),
            0,
            3,
            join_children(
                &children_of(vec![Rc::new(space)]),
                &children_of(vec![Rc::new(word)]),
            ),
        ))
    }

    fn selection_equality_embedding() -> Tree {
        use super::super::grammar_selection::{AcceptedSelection, SelectionMetadata};
        let provenance = |language: &str| {
            serde_json::json!({
                "source": language, "edition": "authored", "rules": [
                    {"rule":"document", "origin":format!("{language}:document"), "concept":format!("shared:{language}")}
                ]
            })
        };
        let mut tree = Tree::new(TreeType::Embed, None, 0, 3);
        tree.language = Some("first".into());
        tree.root = Some(selection_equality_root("first"));
        tree.selection = Some(Rc::new(SelectionMetadata {
            mode: "all-grammar".into(),
            mapping: "identity".into(),
            start: 0,
            end: 3,
            candidates: ["first", "second", "third"]
                .map(|language| serde_json::json!({"language": language, "relation": "accepted"}))
                .to_vec(),
            provenance: provenance("first"),
            ambiguous: false,
            alternatives: ["second", "third"]
                .map(|language| AcceptedSelection {
                    language: language.into(),
                    root: selection_equality_root(language),
                    provenance: provenance(language),
                    program: 1,
                })
                .to_vec(),
        }));
        tree
    }

    fn assert_selection_difference(reference: &Tree, candidate: Tree) {
        let reference = Rc::new(reference.clone());
        let candidate = Rc::new(candidate);
        assert!(!super::same_tree(&reference, &candidate));
        let left = join_children(&children_of(vec![reference]), &children_of(vec![token(3)]));
        let right = join_children(&children_of(vec![candidate]), &children_of(vec![token(3)]));
        assert!(!same_output(&left, &right));
        assert_linked(&left, 2);
        assert_linked(&right, 2);
    }

    #[test]
    fn embeddings_compare_full_nested_roots_and_every_selection_witness() {
        let reference = selection_equality_embedding();
        let mut equivalent = reference.clone();
        equivalent.program = 92;
        let selection = Rc::make_mut(equivalent.selection.as_mut().unwrap());
        selection.alternatives[0].program = 93;
        assert!(same_output(
            &children_of(vec![Rc::new(reference.clone())]),
            &children_of(vec![Rc::new(equivalent.clone())])
        ));
        assert!(super::same_tree(
            &Rc::new(reference.clone()),
            &Rc::new(equivalent)
        ));
        let changes: [fn(&mut Tree); 16] = [
            |tree| tree.language = Some("other".into()),
            |tree| Rc::make_mut(tree.root.as_mut().unwrap()).kind = Some("other_root".into()),
            |tree| Rc::make_mut(tree.root.as_mut().unwrap()).field = Some("other".into()),
            |tree| Rc::make_mut(tree.root.as_mut().unwrap()).start = 1,
            |tree| Rc::make_mut(tree.root.as_mut().unwrap()).projection_name = Some("other".into()),
            |tree| Rc::make_mut(tree.selection.as_mut().unwrap()).mode = "any-grammar".into(),
            |tree| Rc::make_mut(tree.selection.as_mut().unwrap()).mapping = "quoted-escape".into(),
            |tree| Rc::make_mut(tree.selection.as_mut().unwrap()).start = 1,
            |tree| Rc::make_mut(tree.selection.as_mut().unwrap()).end = 2,
            |tree| Rc::make_mut(tree.selection.as_mut().unwrap()).ambiguous = true,
            |tree| {
                Rc::make_mut(tree.selection.as_mut().unwrap())
                    .candidates
                    .reverse()
            },
            |tree| {
                Rc::make_mut(tree.selection.as_mut().unwrap()).provenance["edition"] =
                    serde_json::json!("other")
            },
            |tree| {
                Rc::make_mut(tree.selection.as_mut().unwrap()).alternatives[0].language =
                    "other".into()
            },
            |tree| {
                Rc::make_mut(tree.selection.as_mut().unwrap()).alternatives[0].root =
                    selection_equality_root("other")
            },
            |tree| {
                Rc::make_mut(tree.selection.as_mut().unwrap())
                    .alternatives
                    .reverse()
            },
            |tree| tree.selection = None,
        ];
        for change in changes {
            let mut candidate = reference.clone();
            change(&mut candidate);
            assert_selection_difference(&reference, candidate);
        }
        let mut candidate = reference.clone();
        Rc::make_mut(candidate.selection.as_mut().unwrap())
            .alternatives
            .pop();
        assert_selection_difference(&reference, candidate);
    }

    #[test]
    fn lexical_unknown_keys_null_presence_and_linked_metadata_order_remain_observable() {
        let mut first = Tree::new(TreeType::Token, Some("word".into()), 0, 1);
        first.lexical = Some(serde_json::json!({"lexer":"first", "future": null}));
        let mut other = first.clone();
        other.lexical = Some(serde_json::json!({"lexer":"first"}));
        assert!(!same_output(
            &children_of(vec![Rc::new(first.clone())]),
            &children_of(vec![Rc::new(other.clone())])
        ));
        other.lexical = Some(serde_json::json!({"future":null, "lexer":"first"}));
        assert!(!same_output(
            &children_of(vec![Rc::new(first.clone())]),
            &children_of(vec![Rc::new(other.clone())])
        ));
        first.lexical = None;
        other.lexical = Some(serde_json::Value::Null);
        assert!(!same_output(
            &children_of(vec![Rc::new(first)]),
            &children_of(vec![Rc::new(other)])
        ));
        let missing = Tree::new(TreeType::Missing, Some("word".into()), 1, 1);
        let mut literal = missing.clone();
        literal.literal = true;
        assert!(!same_output(
            &children_of(vec![Rc::new(missing)]),
            &children_of(vec![Rc::new(literal)])
        ));
        let reference = selection_equality_embedding();
        let mut candidate = reference.clone();
        Rc::make_mut(candidate.selection.as_mut().unwrap()).provenance["future"] =
            serde_json::Value::Null;
        assert_selection_difference(&reference, candidate);
    }

    #[test]
    fn structural_node_trivia_stays_flexible_but_embedded_trivia_is_concrete() {
        let first = selection_equality_root("first");
        let mut shifted = (*first).clone();
        shifted.start = 1;
        shifted.children = children_of(vec![first.children[1].clone()]);
        assert!(super::same_tree(&first, &Rc::new(shifted)));
        let reference = selection_equality_embedding();
        let mut candidate = reference.clone();
        let root = Rc::make_mut(candidate.root.as_mut().unwrap());
        root.children = children_of(vec![root.children[1].clone()]);
        assert_selection_difference(&reference, candidate);
    }
}
