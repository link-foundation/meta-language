use meta_language::{
    emit_javascript_parser, emit_peggy, CharClassItem, Grammar, JsParserArtifacts, RuleKind,
};

const COVERING_PEGGY: &str = include_str!("../fixtures/grammar/emit/covering.peggy");

#[test]
fn emits_peggy_golden_with_native_operators_rule_kind_notes_and_report() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("start")
        .grammar_rule(
            meta_language::GrammarRule::new(
                "digit",
                expr.char_class(false, [CharClassItem::range('0', '9')]),
            )
            .with_doc("A decimal digit."),
        )
        .rule(
            "start",
            expr.seq([
                expr.and(expr.term("pre")),
                expr.not(expr.term("skip")),
                expr.choice_ordered([expr.terminal_insensitive("hello"), expr.term("hi")]),
                expr.opt(expr.seq([expr.term("a"), expr.nt("digit")])),
                expr.rep0(expr.nt("digit")),
                expr.rep1(expr.nt("digit")),
                expr.repeat(expr.term("x"), 2, Some(4)),
                expr.repeat(expr.term("y"), 3, None),
                expr.repeat(expr.term("z"), 2, Some(2)),
                expr.char_class(
                    false,
                    [CharClassItem::char('-'), CharClassItem::range('a', 'f')],
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
        .rule(
            "unordered",
            expr.choice_unordered([expr.term("left"), expr.term("right")]),
        )
        .rule_with_kind("atomic_rule", expr.term("atomic"), RuleKind::Atomic)
        .rule_with_kind("silent_rule", expr.term(" "), RuleKind::Silent)
        .rule_with_kind("token_rule", expr.term("token"), RuleKind::Token)
        .rule("empty", expr.empty())
        .build();

    let (text, report) = emit_peggy(&grammar).expect("Peggy emits");

    let expected = COVERING_PEGGY.replace("\r\n", "\n");
    assert_eq!(text, expected);
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("unordered choice")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("RuleKind::Atomic")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("RuleKind::Silent")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("RuleKind::Token")));
    assert!(!report
        .lossy
        .iter()
        .any(|note| note.contains("capture label")));
}

#[test]
fn emits_javascript_parser_bundle_for_sum_grammar() {
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
        .rule("num", expr.rep1(expr.char_range('0', '9')))
        .build();

    let (artifacts, report) =
        emit_javascript_parser(&grammar).expect("JavaScript parser codegen emits");

    assert!(report.lossy.is_empty());
    assert_eq!(
        artifacts,
        JsParserArtifacts {
            peggy_grammar: "sum = num (\"+\" num)*\nnum = [0-9]+\n".to_string(),
            module: concat!(
                "import peggy from \"peggy\";\n",
                "\n",
                "const GRAMMAR = \"sum = num (\\\"+\\\" num)*\\nnum = [0-9]+\\n\";\n",
                "export const parser = peggy.generate(GRAMMAR);\n",
            )
            .to_string(),
        }
    );
}

#[test]
fn peggy_escapes_literals_character_classes_and_module_string() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .rule(
            "escaped",
            expr.seq([
                expr.term("\"\\\n\t"),
                expr.char_range('\n', '\n'),
                expr.char_class(
                    false,
                    [
                        CharClassItem::char('"'),
                        CharClassItem::char('\\'),
                        CharClassItem::range('\t', '\t'),
                        CharClassItem::char(']'),
                        CharClassItem::char('^'),
                    ],
                ),
            ]),
        )
        .build();

    let (artifacts, report) =
        emit_javascript_parser(&grammar).expect("JavaScript parser codegen emits escapes");

    assert!(report.lossy.is_empty());
    assert_eq!(
        artifacts.peggy_grammar,
        "escaped = \"\\\"\\\\\\n\\t\" [\\n-\\n] [\"\\\\\\t-\\t\\]\\^]\n"
    );
    assert!(artifacts
        .module
        .contains("const GRAMMAR = \"escaped = \\\"\\\\\\\"\\\\\\\\\\\\n\\\\t\\\""));
}
