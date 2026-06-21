use meta_language::{
    default_pattern_catalog, evaluate_atom, evaluate_constraint, evaluate_probabilistic,
    mine_semantic_constraints, ConstraintAtom, ConstraintClause, ConstraintPattern, Grammar,
    LengthUnit, NonTerminalRef, Probability, SemanticConstraint, SemanticInferenceConfig,
    TruthValue,
};

#[test]
fn mines_def_before_use_and_rejects_undefined_use() {
    let grammar = def_use_grammar();
    let positives = strings(["def alpha; use alpha;", "def beta; use beta;"]);

    let constraint =
        mine_semantic_constraints(&grammar, &positives, &SemanticInferenceConfig::default());

    assert!(
        contains_def_before_use(&constraint, "Def", "Use"),
        "{constraint:#?}"
    );
    assert_eq!(
        evaluate_constraint(&grammar, "def alpha; use alpha;", &constraint),
        TruthValue::True
    );
    assert_eq!(
        evaluate_constraint(&grammar, "def alpha; use beta;", &constraint),
        TruthValue::False
    );
}

#[test]
fn mines_length_field_and_rejects_wrong_length() {
    let grammar = length_field_grammar();
    let positives = strings(["len=3:abc", "len=5:hello"]);

    let constraint =
        mine_semantic_constraints(&grammar, &positives, &SemanticInferenceConfig::default());

    assert!(
        contains_length_field(&constraint, "LengthField", "Body"),
        "{constraint:#?}"
    );
    assert_eq!(
        evaluate_constraint(&grammar, "len=3:abc", &constraint),
        TruthValue::True
    );
    assert_eq!(
        evaluate_constraint(&grammar, "len=4:abc", &constraint),
        TruthValue::False
    );
}

#[test]
fn mines_equal_count_and_rejects_mismatched_counts() {
    let grammar = paired_delimiter_grammar();
    let positives = strings(["(())", "()()"]);

    let constraint =
        mine_semantic_constraints(&grammar, &positives, &SemanticInferenceConfig::default());

    assert!(
        contains_equal_count(&constraint, "Open", "Close"),
        "{constraint:#?}"
    );
    assert_eq!(
        evaluate_constraint(&grammar, "(())", &constraint),
        TruthValue::True
    );
    assert_eq!(
        evaluate_constraint(&grammar, "(()", &constraint),
        TruthValue::False
    );
}

#[test]
fn drops_undiscriminated_unique_candidate() {
    let grammar = item_name_grammar();
    let positives = strings(["item alpha"]);
    let config = SemanticInferenceConfig {
        catalog: patterns(["unique"]),
        ..SemanticInferenceConfig::default()
    };

    let constraint = mine_semantic_constraints(&grammar, &positives, &config);

    assert!(constraint.clauses.is_empty(), "{constraint:#?}");
}

#[test]
fn evaluates_atoms_clauses_and_probabilistic_truth() {
    let grammar = paired_delimiter_grammar();
    let atom = ConstraintAtom::EqualCount {
        left: NonTerminalRef::new("Open"),
        right: NonTerminalRef::new("Close"),
    };
    let constraint = SemanticConstraint::new(
        vec![ConstraintClause::new(vec![atom.clone()])],
        1,
        Probability::ONE,
    );

    assert_eq!(evaluate_atom(&grammar, "()", &atom), TruthValue::True);
    assert_eq!(evaluate_atom(&grammar, "(()", &atom), TruthValue::False);
    assert_eq!(
        evaluate_probabilistic(&grammar, "()", &constraint).true_probability(),
        Probability::ONE
    );
    assert_eq!(
        evaluate_probabilistic(&grammar, "(()", &constraint).true_probability(),
        Probability::ZERO
    );
}

#[test]
fn mining_order_is_deterministic() {
    let grammar = length_field_grammar();
    let positives = strings(["len=3:abc", "len=5:hello"]);

    let first =
        mine_semantic_constraints(&grammar, &positives, &SemanticInferenceConfig::default());
    let second =
        mine_semantic_constraints(&grammar, &positives, &SemanticInferenceConfig::default());

    assert_eq!(first, second);
}

#[test]
fn length_unit_elements_counts_delimited_bodies() {
    let grammar = length_field_grammar();
    let atom = ConstraintAtom::LengthField {
        field: NonTerminalRef::new("LengthField"),
        body: NonTerminalRef::new("Body"),
        unit: LengthUnit::Elements,
    };

    assert_eq!(
        evaluate_atom(&grammar, "len=3:a,b,c", &atom),
        TruthValue::True
    );
    assert_eq!(
        evaluate_atom(&grammar, "len=2:a,b,c", &atom),
        TruthValue::False
    );
}

fn def_use_grammar() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start("Program")
        .rule(
            "Program",
            expr.rep1(expr.choice_unordered([expr.nt("Def"), expr.nt("Use")])),
        )
        .rule(
            "Def",
            expr.seq([expr.term("def "), expr.nt("Name"), expr.term(";")]),
        )
        .rule(
            "Use",
            expr.seq([expr.term("use "), expr.nt("Name"), expr.term(";")]),
        )
        .rule("Name", expr.rep1(expr.char_range('a', 'z')))
        .build()
}

fn length_field_grammar() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start("Record")
        .rule(
            "Record",
            expr.seq([
                expr.term("len="),
                expr.nt("LengthField"),
                expr.term(":"),
                expr.nt("Body"),
            ]),
        )
        .rule("LengthField", expr.rep1(expr.char_range('0', '9')))
        .rule("Body", expr.rep1(expr.char_range('a', 'z')))
        .build()
}

fn paired_delimiter_grammar() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start("Pairs")
        .rule(
            "Pairs",
            expr.rep1(expr.choice_unordered([expr.nt("Open"), expr.nt("Close")])),
        )
        .rule("Open", expr.term("("))
        .rule("Close", expr.term(")"))
        .build()
}

fn item_name_grammar() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start("Item")
        .rule("Item", expr.seq([expr.term("item "), expr.nt("ItemName")]))
        .rule("ItemName", expr.rep1(expr.char_range('a', 'z')))
        .build()
}

fn strings<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn patterns<const N: usize>(names: [&str; N]) -> Vec<ConstraintPattern> {
    default_pattern_catalog()
        .into_iter()
        .filter(|pattern| names.contains(&pattern.name))
        .collect()
}

fn contains_def_before_use(
    constraint: &SemanticConstraint,
    expected_def: &str,
    expected_use: &str,
) -> bool {
    constraint
        .clauses
        .iter()
        .flat_map(|clause| &clause.atoms)
        .any(|atom| {
            matches!(
                atom,
                ConstraintAtom::DefBeforeUse { def, use_ }
                    if def.rule == expected_def && use_.rule == expected_use
            )
        })
}

fn contains_length_field(
    constraint: &SemanticConstraint,
    expected_field: &str,
    expected_body: &str,
) -> bool {
    constraint
        .clauses
        .iter()
        .flat_map(|clause| &clause.atoms)
        .any(|atom| {
            matches!(
                atom,
                ConstraintAtom::LengthField { field, body, unit: LengthUnit::Bytes }
                    if field.rule == expected_field && body.rule == expected_body
            )
        })
}

fn contains_equal_count(
    constraint: &SemanticConstraint,
    expected_left: &str,
    expected_right: &str,
) -> bool {
    constraint
        .clauses
        .iter()
        .flat_map(|clause| &clause.atoms)
        .any(|atom| {
            matches!(
                atom,
                ConstraintAtom::EqualCount { left, right }
                    if left.rule == expected_left && right.rule == expected_right
            )
        })
}
