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
        source_mapping: None,
        source_bytes: None,
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
            Rc::make_mut(tree.selection.as_mut().unwrap()).alternatives[0].language = "other".into()
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
