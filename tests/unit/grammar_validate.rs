use meta_language::{
    parse_grammar_surface, validate, DiagnosticKind, Grammar, GrammarDiagnostic, GrammarRule,
    Severity,
};

#[test]
fn undefined_nonterminal_reports_rule_and_nearest_name() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("expr")
        .rule("expr", expr.nt("experssion"))
        .rule("expression", expr.term("x"))
        .build();

    let diagnostics = validate(&grammar);
    let diagnostic = find_kind(&diagnostics, |kind| {
        matches!(
            kind,
            DiagnosticKind::UndefinedNonTerminal {
                name,
                referenced_in
            } if name == "experssion" && referenced_in == "expr"
        )
    });

    assert_eq!(diagnostic.severity, Severity::Error);
    assert_eq!(diagnostic.location.rule, "expr");
    assert!(diagnostic.message.contains("did you mean `expression`"));
    assert!(diagnostic.is_error());
}

#[test]
fn detects_direct_and_indirect_left_recursion_with_cycle_paths() {
    let expr = Grammar::expr();
    let direct = Grammar::builder()
        .start("expr")
        .rule("expr", expr.seq([expr.nt("expr"), expr.term("+")]))
        .build();
    let direct_diagnostics = validate(&direct);

    assert!(direct_diagnostics.iter().any(|diagnostic| {
        matches!(
            &diagnostic.kind,
            DiagnosticKind::LeftRecursion { cycle } if cycle == &["expr", "expr"]
        )
    }));

    let indirect = Grammar::builder()
        .start("a")
        .rule("a", expr.nt("b"))
        .rule("b", expr.nt("a"))
        .build();
    let indirect_diagnostics = validate(&indirect);

    assert!(indirect_diagnostics.iter().any(|diagnostic| {
        matches!(
            &diagnostic.kind,
            DiagnosticKind::LeftRecursion { cycle }
                if cycle.first().map(String::as_str) == Some("a")
                    && cycle.last().map(String::as_str) == Some("a")
                    && cycle.iter().any(|name| name == "b")
        )
    }));
}

#[test]
fn right_recursion_and_terminal_guards_are_not_left_recursion() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("a")
        .rule("a", expr.seq([expr.term("x"), expr.nt("a")]))
        .rule("b", expr.seq([expr.nt("c"), expr.nt("b")]))
        .rule("c", expr.term("y"))
        .build();

    let diagnostics = validate(&grammar);

    assert!(!diagnostics
        .iter()
        .any(|diagnostic| matches!(diagnostic.kind, DiagnosticKind::LeftRecursion { .. })));
}

#[test]
fn unreachable_rules_are_warnings_from_the_start_rule() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("expr")
        .rule("expr", expr.nt("term"))
        .rule("term", expr.term("x"))
        .rule("dead", expr.term("z"))
        .build();

    let diagnostics = validate(&grammar);
    let diagnostic = find_kind(
        &diagnostics,
        |kind| matches!(kind, DiagnosticKind::UnreachableRule { name } if name == "dead"),
    );

    assert_eq!(diagnostic.severity, Severity::Warning);
    assert_eq!(diagnostic.location.rule, "dead");
}

#[test]
fn nullable_repetition_flags_nullable_inner_expression_only() {
    let expr = Grammar::expr();
    let bad = Grammar::builder()
        .start("a")
        .rule("a", expr.rep0(expr.nt("b")))
        .rule("b", expr.opt(expr.term("x")))
        .build();

    let bad_diagnostics = validate(&bad);

    assert!(bad_diagnostics.iter().any(|diagnostic| {
        matches!(
            &diagnostic.kind,
            DiagnosticKind::NullableRepetition { rule, detail }
                if rule == "a" && detail.contains("nullable")
        )
    }));

    let good = Grammar::builder()
        .start("a")
        .rule("a", expr.rep0(expr.nt("b")))
        .rule("b", expr.term("x"))
        .build();

    let good_diagnostics = validate(&good);

    assert!(!good_diagnostics
        .iter()
        .any(|diagnostic| matches!(diagnostic.kind, DiagnosticKind::NullableRepetition { .. })));
}

#[test]
fn duplicate_rules_are_errors() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("item")
        .rule("item", expr.term("a"))
        .rule("item", expr.term("b"))
        .build();

    let diagnostics = validate(&grammar);
    let diagnostic = find_kind(
        &diagnostics,
        |kind| matches!(kind, DiagnosticKind::DuplicateRule { name } if name == "item"),
    );

    assert_eq!(diagnostic.severity, Severity::Error);
    assert_eq!(diagnostic.location.rule, "item");
}

#[test]
fn labelled_captures_are_reported_as_unused_warnings() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("item")
        .rule("item", expr.capture(Some("value"), expr.term("x")))
        .build();

    let diagnostics = grammar.validate();
    let diagnostic = find_kind(&diagnostics, |kind| {
        matches!(
            kind,
            DiagnosticKind::UnusedCapture { rule, label }
                if rule == "item" && label == "value"
        )
    });

    assert_eq!(diagnostic.severity, Severity::Warning);
    assert_eq!(diagnostic.location.rule, "item");
}

#[test]
fn valid_arithmetic_grammar_has_no_errors_and_validation_is_deterministic() {
    let grammar = parse_grammar_surface(
        r#"
(expr: term (( "+" / "-" ) term)*)
(term: factor (( "*" / "/" ) factor)*)
(factor: number / "(" expr ")")
(number: [0-9]+)
"#,
    )
    .expect("arithmetic surface parses");

    let first = validate(&grammar);
    let second = validate(&grammar);

    assert_eq!(first, second);
    assert!(!first.iter().any(GrammarDiagnostic::is_error));
}

#[test]
fn empty_grammar_and_single_rule_cycle_do_not_panic() {
    assert!(validate(&Grammar::new()).is_empty());

    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("loop")
        .rule("loop", expr.nt("loop"))
        .build();

    assert!(validate(&grammar).iter().any(|diagnostic| {
        matches!(diagnostic.kind, DiagnosticKind::LeftRecursion { .. })
            && diagnostic.location.rule == "loop"
    }));
}

fn find_kind(
    diagnostics: &[GrammarDiagnostic],
    predicate: impl Fn(&DiagnosticKind) -> bool,
) -> &GrammarDiagnostic {
    diagnostics
        .iter()
        .find(|diagnostic| predicate(&diagnostic.kind))
        .expect("expected diagnostic kind")
}
