use meta_language::{
    build_structural_prior, ByteSpan, Delimiter, LeafKind, PriorOptions, SeedNode, WhitespacePolicy,
};

#[test]
fn arithmetic_expression_keeps_parenthesized_constituents() {
    let example = "a*(b+c)".to_string();
    let prior = build_structural_prior(&[example], PriorOptions::default());
    let root = prior.trees[0].root_children();

    assert_eq!(leaf(&root[0]), Some((ByteSpan::new(0, 1), LeafKind::Text)));
    assert_eq!(leaf(&root[1]), Some((ByteSpan::new(1, 2), LeafKind::Text)));

    let (delimiter, span, children) = group(&root[2]).expect("third child is paren group");
    assert_eq!(delimiter, Delimiter::Paren);
    assert_eq!(span, ByteSpan::new(2, 7));
    assert_eq!(children.len(), 3);
    assert_eq!(
        leaf(&children[0]),
        Some((ByteSpan::new(3, 4), LeafKind::Text))
    );
    assert_eq!(
        leaf(&children[1]),
        Some((ByteSpan::new(4, 5), LeafKind::Text))
    );
    assert_eq!(
        leaf(&children[2]),
        Some((ByteSpan::new(5, 6), LeafKind::Text))
    );
}

#[test]
fn json_like_input_lowers_braces_brackets_and_opaque_quotes() {
    let example = r#"{"k": [1, 2]}"#.to_string();
    let prior = build_structural_prior(std::slice::from_ref(&example), PriorOptions::default());
    let root = prior.trees[0].root_children();

    let (delimiter, span, children) = group(&root[0]).expect("root child is curly group");
    assert_eq!(delimiter, Delimiter::Curly);
    assert_eq!(span, ByteSpan::new(0, example.len()));
    assert_eq!(
        leaf(&children[0]),
        Some((ByteSpan::new(1, 4), LeafKind::DoubleQuote))
    );
    assert_eq!(
        leaf(&children[1]),
        Some((ByteSpan::new(4, 5), LeafKind::Text))
    );

    let (delimiter, span, square_children) =
        group(&children[2]).expect("third curly child is square group");
    assert_eq!(delimiter, Delimiter::Square);
    assert_eq!(span, ByteSpan::new(6, 12));
    assert_eq!(square_children.len(), 3);
    assert_eq!(
        square_children
            .iter()
            .map(|node| node.slice(&example))
            .collect::<Vec<_>>(),
        vec!["1", ",", "2"]
    );
}

#[test]
fn nested_s_expression_preserves_inside_out_groups() {
    let example = "(f (g x) y)".to_string();
    let prior = build_structural_prior(&[example], PriorOptions::default());
    let root = prior.trees[0].root_children();

    let (delimiter, _span, children) = group(&root[0]).expect("outer paren group");
    assert_eq!(delimiter, Delimiter::Paren);
    assert_eq!(children.len(), 3);

    let (inner_delimiter, inner_span, inner_children) =
        group(&children[1]).expect("nested paren group");
    assert_eq!(inner_delimiter, Delimiter::Paren);
    assert_eq!(inner_span, ByteSpan::new(3, 8));
    assert_eq!(inner_children.len(), 2);
}

#[test]
fn unbalanced_delimiters_fall_back_to_flat_text_leaves() {
    let example = "(a [b".to_string();
    let prior = build_structural_prior(std::slice::from_ref(&example), PriorOptions::default());
    let children = prior.trees[0].root_children();

    assert!(children
        .iter()
        .all(|node| matches!(node, SeedNode::Leaf { .. })));
    assert_eq!(
        children
            .iter()
            .map(|node| node.slice(&example))
            .collect::<Vec<_>>(),
        vec!["(", "a", "[", "b"]
    );
}

#[test]
fn alphabet_is_sorted_deduplicated_leaf_text() {
    let examples = vec!["b(a)".to_string(), "a(b)".to_string()];
    let prior = build_structural_prior(&examples, PriorOptions::default());

    assert_eq!(prior.alphabet, vec!["a", "b"]);
}

#[test]
fn structural_prior_is_deterministic() {
    let examples = vec!["a*(b+c)".to_string(), r#"{"k": [1, 2]}"#.to_string()];
    let opts = PriorOptions::default();

    assert_eq!(
        build_structural_prior(&examples, opts),
        build_structural_prior(&examples, opts)
    );

    let reversed = vec![examples[1].clone(), examples[0].clone()];
    let first = build_structural_prior(&examples, opts);
    let second = build_structural_prior(&reversed, opts);

    assert_eq!(first.alphabet, second.alphabet);
    assert_eq!(first.trees[0], second.trees[1]);
    assert_eq!(first.trees[1], second.trees[0]);
}

#[test]
fn keep_whitespace_policy_preserves_verbatim_text_runs() {
    let example = "(a  b)".to_string();
    let prior = build_structural_prior(
        &[example],
        PriorOptions {
            whitespace: WhitespacePolicy::Keep,
            ..PriorOptions::default()
        },
    );
    let (_, _, children) = group(&prior.trees[0].root_children()[0]).expect("outer group");

    assert_eq!(
        leaf(&children[0]),
        Some((ByteSpan::new(1, 5), LeafKind::Text))
    );
}

trait SeedTreeExt {
    fn root_children(&self) -> &[SeedNode];
}

impl SeedTreeExt for meta_language::SeedTree {
    fn root_children(&self) -> &[SeedNode] {
        let SeedNode::Group {
            delimiter,
            children,
            span,
        } = &self.root
        else {
            panic!("root must be a group");
        };
        assert_eq!(*delimiter, Delimiter::Root);
        assert_eq!(*span, ByteSpan::new(0, self.example.len()));
        children
    }
}

trait SeedNodeExt {
    fn slice<'a>(&self, example: &'a str) -> &'a str;
}

impl SeedNodeExt for SeedNode {
    fn slice<'a>(&self, example: &'a str) -> &'a str {
        let span = match self {
            Self::Leaf { span, .. } | Self::Group { span, .. } => *span,
        };
        &example[span.start..span.end]
    }
}

const fn leaf(node: &SeedNode) -> Option<(ByteSpan, LeafKind)> {
    match node {
        SeedNode::Leaf { span, kind } => Some((*span, *kind)),
        SeedNode::Group { .. } => None,
    }
}

fn group(node: &SeedNode) -> Option<(Delimiter, ByteSpan, &[SeedNode])> {
    match node {
        SeedNode::Group {
            delimiter,
            children,
            span,
        } => Some((*delimiter, *span, children)),
        SeedNode::Leaf { .. } => None,
    }
}
