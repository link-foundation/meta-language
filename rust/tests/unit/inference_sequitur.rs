use std::collections::{BTreeMap, BTreeSet};

use meta_language::{
    run_sequitur, size_symbols, Grammar, GrammarExpr, GrammarFormat, GrammarOracle,
};

#[test]
fn canonical_example_derives_exact_input_and_builds_shared_rules() {
    let input = symbols("abcabdabcabd");
    let grammar = run_sequitur(&input);
    let oracle = GrammarOracle::new(&grammar);

    assert_eq!(grammar.source_format(), Some(GrammarFormat::Inferred));
    assert_eq!(grammar.start(), Some("start"));
    assert!(oracle.accepts("abcabdabcabd"));
    assert!(!oracle.accepts("abcabd"));
    assert!(generated_rules(&grammar).count() > 0, "{grammar:#?}");
    assert!(
        generated_rules(&grammar).any(|(_, body)| body.iter().any(is_nonterminal)),
        "{grammar:#?}"
    );
}

#[test]
fn emitted_grammar_preserves_sequitur_invariants() {
    let grammar = run_sequitur(&symbols("abcabdabcabd"));

    assert_digram_uniqueness(&grammar);
    assert_rule_utility(&grammar);
}

#[test]
fn input_without_repeated_digrams_stays_flat() {
    let input = symbols("abcdef");
    let grammar = run_sequitur(&input);

    assert_eq!(grammar.rule_names(), vec!["start"]);
    assert_eq!(
        grammar.rule("start").map(meta_language::GrammarRule::expr),
        Some(&GrammarExpr::Sequence(
            input.into_iter().map(GrammarExpr::Terminal).collect()
        ))
    );
}

#[test]
fn nested_repetition_builds_multi_level_hierarchy() {
    let grammar = run_sequitur(&symbols("abcabdabcabdabcabdabcabd"));

    assert!(max_nonterminal_depth(&grammar, "start") > 1, "{grammar:#?}");
}

#[test]
fn repetitive_input_is_smaller_than_flat_input() {
    let input = symbols("abcabdabcabdabcabdabcabdabcabdabcabd");
    let grammar = run_sequitur(&input);

    assert!(size_symbols(&grammar) < input.len(), "{grammar:#?}");
}

#[test]
fn sequitur_output_is_deterministic() {
    let input = symbols("abcabdabcabdabcabdabcabd");

    assert_eq!(run_sequitur(&input), run_sequitur(&input));
}

fn symbols(input: &str) -> Vec<String> {
    input.chars().map(|value| value.to_string()).collect()
}

fn generated_rules(grammar: &Grammar) -> impl Iterator<Item = (&str, &[GrammarExpr])> {
    grammar.rules().iter().filter_map(|rule| {
        let GrammarExpr::Sequence(body) = rule.expr() else {
            return None;
        };
        rule.name()
            .starts_with('R')
            .then_some((rule.name(), body.as_slice()))
    })
}

const fn is_nonterminal(expr: &GrammarExpr) -> bool {
    matches!(expr, GrammarExpr::NonTerminal(_))
}

fn assert_digram_uniqueness(grammar: &Grammar) {
    let mut seen = BTreeMap::<(String, String), String>::new();

    for rule in grammar.rules() {
        let GrammarExpr::Sequence(body) = rule.expr() else {
            panic!("Sequitur should emit sequence rules: {rule:#?}");
        };

        for window in body.windows(2) {
            let key = (symbol_key(&window[0]), symbol_key(&window[1]));
            let location = format!("{}:{key:?}", rule.name());
            assert_eq!(
                seen.insert(key, location.clone()),
                None,
                "duplicate digram at {location}; seen map: {seen:#?}"
            );
        }
    }
}

fn assert_rule_utility(grammar: &Grammar) {
    let generated = generated_rules(grammar)
        .map(|(name, _)| name.to_string())
        .collect::<BTreeSet<_>>();
    let mut references = BTreeMap::<String, usize>::new();

    for rule in grammar.rules() {
        let GrammarExpr::Sequence(body) = rule.expr() else {
            panic!("Sequitur should emit sequence rules: {rule:#?}");
        };

        for expr in body {
            if let GrammarExpr::NonTerminal(name) = expr {
                *references.entry(name.clone()).or_default() += 1;
            }
        }
    }

    for name in generated {
        assert!(
            references.get(&name).copied().unwrap_or_default() > 1,
            "generated rule {name} is not useful: {references:#?}"
        );
    }
}

fn max_nonterminal_depth(grammar: &Grammar, rule_name: &str) -> usize {
    let Some(rule) = grammar.rule(rule_name) else {
        return 0;
    };
    let GrammarExpr::Sequence(body) = rule.expr() else {
        return 0;
    };

    body.iter()
        .filter_map(|expr| match expr {
            GrammarExpr::NonTerminal(name) => Some(1 + max_nonterminal_depth(grammar, name)),
            _ => None,
        })
        .max()
        .unwrap_or_default()
}

fn symbol_key(expr: &GrammarExpr) -> String {
    match expr {
        GrammarExpr::Terminal(value) => format!("T:{value}"),
        GrammarExpr::NonTerminal(name) => format!("N:{name}"),
        other => panic!("Sequitur emitted unsupported expression: {other:#?}"),
    }
}
