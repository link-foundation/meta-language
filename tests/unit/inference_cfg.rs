use meta_language::{
    infer_cfg, GrammarExpr, GrammarFormat, GrammarOracle, InferenceOptions, MembershipOracle,
    Oracle, PositiveOnlyOracle,
};

#[test]
fn cfg_inference_preserves_positives_and_delimiter_nonterminal() {
    let examples = strings(["a*(b+c)", "x*(y+z)"]);
    let result = infer_cfg(&examples, &PositiveOnlyOracle, InferenceOptions::default());
    let oracle = GrammarOracle::new(&result.grammar);

    assert_eq!(
        result.grammar.source_format(),
        Some(GrammarFormat::Inferred)
    );
    assert_eq!(result.report.rules, result.grammar.rules().len());
    assert!(examples.iter().all(|example| oracle.accepts(example)));
    assert!(
        result
            .grammar
            .rules()
            .iter()
            .any(|rule| rule.name() != "Root" && contains_terminal(rule.expr(), "(")),
        "{:#?}",
        result.grammar
    );
    assert!(
        result
            .grammar
            .rule("Root")
            .is_some_and(|rule| contains_non_terminal(rule.expr())),
        "{:#?}",
        result.grammar
    );
}

#[test]
fn recursive_list_fixture_introduces_self_reference() {
    let examples = strings(["[]", "[a]", "[a,b]", "[a,b,c]"]);
    let result = infer_cfg(&examples, &PositiveOnlyOracle, InferenceOptions::default());
    let oracle = GrammarOracle::new(&result.grammar);

    assert!(examples.iter().all(|example| oracle.accepts(example)));
    assert!(
        result
            .grammar
            .rules()
            .iter()
            .any(|rule| references_rule(rule.expr(), rule.name())),
        "{:#?}",
        result.grammar
    );
}

#[test]
fn inference_is_deterministic_and_input_order_invariant() {
    let examples = strings(["[b,a]", "[]", "[a]"]);
    let mut reversed = examples.clone();
    reversed.reverse();
    let oracle = PositiveOnlyOracle;

    let first = infer_cfg(&examples, &oracle, InferenceOptions::default());
    let second = infer_cfg(&examples, &oracle, InferenceOptions::default());
    let reordered = infer_cfg(&reversed, &oracle, InferenceOptions::default());

    assert_eq!(first, second);
    assert_eq!(first.grammar, reordered.grammar);
}

#[test]
fn membership_oracle_can_tighten_over_general_recursive_candidate() {
    let examples = strings(["[]", "[a]", "[a,b]"]);
    let loose = infer_cfg(&examples, &PositiveOnlyOracle, InferenceOptions::default());
    let strict_oracle = BoundedListOracle;
    let strict = infer_cfg(&examples, &strict_oracle, InferenceOptions::default());

    assert!(GrammarOracle::new(&loose.grammar).accepts("[a,b,a]"));
    assert!(!GrammarOracle::new(&strict.grammar).accepts("[a,b,a]"));
    assert!(examples
        .iter()
        .all(|example| GrammarOracle::new(&strict.grammar).accepts(example)));
    assert!(strict.report.merges_rejected >= loose.report.merges_rejected);
}

#[test]
fn empty_input_returns_empty_inferred_grammar_without_panicking() {
    let result = infer_cfg(&[], &PositiveOnlyOracle, InferenceOptions::default());

    assert_eq!(
        result.grammar.source_format(),
        Some(GrammarFormat::Inferred)
    );
    assert!(result.grammar.rules().is_empty());
    assert_eq!(result.report.rules, 0);
}

struct BoundedListOracle;

impl MembershipOracle for BoundedListOracle {
    fn accepts(&self, text: &str) -> bool {
        matches!(
            text,
            "[]" | "[a]" | "[b]" | "[a,a]" | "[a,b]" | "[b,a]" | "[b,b]"
        )
    }
}

impl Oracle for BoundedListOracle {
    fn membership(&self) -> Option<&dyn MembershipOracle> {
        Some(self)
    }
}

fn strings<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn contains_terminal(expr: &GrammarExpr, expected: &str) -> bool {
    match expr {
        GrammarExpr::Terminal(value) | GrammarExpr::TerminalInsensitive(value) => value == expected,
        GrammarExpr::Choice { alternatives, .. } => alternatives
            .iter()
            .any(|alternative| contains_terminal(alternative, expected)),
        GrammarExpr::Sequence(items) => items.iter().any(|item| contains_terminal(item, expected)),
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::Capture { expr: inner, .. } => contains_terminal(inner, expected),
        GrammarExpr::Empty
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar
        | GrammarExpr::NonTerminal(_) => false,
    }
}

fn contains_non_terminal(expr: &GrammarExpr) -> bool {
    match expr {
        GrammarExpr::NonTerminal(_) => true,
        GrammarExpr::Choice { alternatives, .. } => alternatives.iter().any(contains_non_terminal),
        GrammarExpr::Sequence(items) => items.iter().any(contains_non_terminal),
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::Capture { expr: inner, .. } => contains_non_terminal(inner),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => false,
    }
}

fn references_rule(expr: &GrammarExpr, expected: &str) -> bool {
    match expr {
        GrammarExpr::NonTerminal(name) => name == expected,
        GrammarExpr::Choice { alternatives, .. } => alternatives
            .iter()
            .any(|alternative| references_rule(alternative, expected)),
        GrammarExpr::Sequence(items) => items.iter().any(|item| references_rule(item, expected)),
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::Capture { expr: inner, .. } => references_rule(inner, expected),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => false,
    }
}
