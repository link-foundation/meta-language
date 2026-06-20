use meta_language::{
    emit_tree_sitter_grammar_js, emit_tree_sitter_grammar_js_with_report, CharClassItem, Grammar,
    GrammarEmitError, GrammarFormat, RuleKind,
};

const SUM_GRAMMAR_JS: &str = include_str!("../fixtures/grammar/tree-sitter/sum.grammar.js");
const COVERING_GRAMMAR_JS: &str =
    include_str!("../fixtures/grammar/tree-sitter/covering.grammar.js");

#[test]
fn emits_sum_tree_sitter_grammar_js_skeleton() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("sum")
        .rule(
            "sum",
            expr.seq([
                expr.nt("num"),
                expr.rep0(expr.seq([expr.term("+"), expr.nt("num")])),
            ]),
        )
        .rule_with_kind("num", expr.rep1(expr.char_range('0', '9')), RuleKind::Token)
        .build();

    let text = emit_tree_sitter_grammar_js(&grammar).expect("tree-sitter grammar.js emits");
    let (reported_text, report) =
        emit_tree_sitter_grammar_js_with_report(&grammar).expect("tree-sitter report emits");

    assert_eq!(text, normalized_fixture(SUM_GRAMMAR_JS));
    assert_eq!(reported_text, text);
    assert!(report.lossy.is_empty());
}

#[test]
fn emits_covering_tree_sitter_grammar_js_with_reported_lossy_mappings() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("start")
        .rule(
            "start",
            expr.seq([
                expr.term("start"),
                expr.terminal_insensitive("Case"),
                expr.choice_ordered([expr.term("a"), expr.term("b")]),
                expr.opt(expr.seq([expr.term("x"), expr.nt("digit")])),
                expr.rep0(expr.nt("digit")),
                expr.rep1(expr.nt("digit")),
                expr.repeat(expr.term("m"), 2, Some(4)),
                expr.repeat(expr.term("n"), 3, None),
                expr.repeat(expr.term("z"), 0, Some(0)),
                expr.char_class(
                    false,
                    [
                        CharClassItem::char('-'),
                        CharClassItem::range('a', 'f'),
                        CharClassItem::char('/'),
                        CharClassItem::char(']'),
                    ],
                ),
                expr.char_class(
                    true,
                    [CharClassItem::char('q'), CharClassItem::range('0', '9')],
                ),
                expr.char_range('A', 'Z'),
                expr.any(),
                expr.nt("digit"),
                expr.capture(Some("label"), expr.term("cap")),
                expr.capture_unlabeled(expr.term("anon")),
                expr.empty(),
            ]),
        )
        .grammar_rule(
            meta_language::GrammarRule::new(
                "digit",
                expr.char_class(false, [CharClassItem::range('0', '9')]),
            )
            .with_doc("A decimal digit."),
        )
        .rule(
            "ordered",
            expr.choice_ordered([expr.term("left"), expr.term("right")]),
        )
        .rule_with_kind("atomic_rule", expr.term("atomic"), RuleKind::Atomic)
        .rule_with_kind("_silent_rule", expr.term(" "), RuleKind::Silent)
        .rule_with_kind("token_rule", expr.term("token"), RuleKind::Token)
        .rule("empty", expr.empty())
        .build();

    let (text, report) =
        emit_tree_sitter_grammar_js_with_report(&grammar).expect("tree-sitter grammar.js emits");

    assert_eq!(text, normalized_fixture(COVERING_GRAMMAR_JS));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("case-insensitive")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("ordered choice")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("Repeat with min 2")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("RuleKind::Silent")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("anonymous capture")));
    assert!(!report
        .lossy
        .iter()
        .any(|note| note.contains("capture label")));
}

#[test]
fn emits_tree_sitter_precedence_alias_and_immediate_token_helpers() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("start")
        .rule(
            "start",
            expr.seq([
                expr.capture(Some("prec=-1"), expr.nt("term")),
                expr.capture(Some("prec_left=2"), expr.nt("left")),
                expr.capture(Some("prec_right=3"), expr.nt("right")),
                expr.capture(Some("prec_dynamic=4"), expr.nt("dynamic")),
                expr.capture(Some("alias:name"), expr.nt("identifier")),
                expr.capture(Some("immediate_token"), expr.term("!")),
            ]),
        )
        .rule("term", expr.term("t"))
        .rule("left", expr.term("l"))
        .rule("right", expr.term("r"))
        .rule("dynamic", expr.term("d"))
        .rule("identifier", expr.char_range('a', 'z'))
        .build();

    let text = emit_tree_sitter_grammar_js(&grammar).expect("tree-sitter grammar.js emits");

    assert!(text.contains("prec(-1, $.term)"));
    assert!(text.contains("prec.left(2, $.left)"));
    assert!(text.contains("prec.right(3, $.right)"));
    assert!(text.contains("prec.dynamic(4, $.dynamic)"));
    assert!(text.contains("alias($.identifier, $.name)"));
    assert!(text.contains("token.immediate(\"!\")"));
}

#[test]
fn predicates_report_unsupported_instead_of_panicking() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("start")
        .rule("start", expr.not(expr.term("skip")))
        .build();

    let error = emit_tree_sitter_grammar_js(&grammar).expect_err("predicate is unsupported");

    assert!(matches!(
        error,
        GrammarEmitError::Unsupported {
            format: GrammarFormat::TreeSitter,
            construct
        } if construct == "predicate"
    ));
}

fn normalized_fixture(text: &str) -> String {
    text.replace("\r\n", "\n")
}
