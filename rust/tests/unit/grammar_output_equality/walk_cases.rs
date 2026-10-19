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
