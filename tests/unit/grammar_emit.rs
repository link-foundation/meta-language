use meta_language::{
    emit_abnf, emit_bnf, emit_ebnf, import_abnf, import_bnf, import_ebnf, CharClassItem, Grammar,
    GrammarEmitError, GrammarFormat,
};

const LIST_BNF: &str = include_str!("../fixtures/grammar/emit/list.bnf");
const LIST_EBNF: &str = include_str!("../fixtures/grammar/emit/list.ebnf");
const MESSAGE_ABNF: &str = include_str!("../fixtures/grammar/emit/message.abnf");

#[test]
fn emits_bnf_golden_with_helpers_and_lossy_report() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("list")
        .rule(
            "item",
            expr.choice_unordered([expr.term("a"), expr.terminal_insensitive("B")]),
        )
        .rule(
            "list",
            expr.seq([
                expr.nt("item"),
                expr.rep0(expr.seq([expr.term(","), expr.nt("item")])),
                expr.opt(expr.term(",")),
            ]),
        )
        .rule("letter", expr.char_range('a', 'c'))
        .build();

    let (text, report) = emit_bnf(&grammar).expect("BNF emits");

    assert_eq!(text, LIST_BNF);
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("case-insensitive")));
    let reparsed = import_bnf(&text).expect("emitted BNF imports");
    assert_eq!(reparsed.source_format(), Some(GrammarFormat::Bnf));
    assert!(reparsed.undefined_nonterminals().is_empty());
}

#[test]
fn emits_ebnf_golden_with_native_operators_helpers_and_lossy_report() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("list")
        .rule(
            "list",
            expr.seq([expr.term("("), expr.opt(expr.nt("items")), expr.term(")")]),
        )
        .rule(
            "items",
            expr.seq([
                expr.nt("item"),
                expr.rep0(expr.seq([expr.term(","), expr.nt("item")])),
            ]),
        )
        .rule(
            "item",
            expr.choice_ordered([
                expr.term("word"),
                expr.terminal_insensitive("id"),
                expr.char_range('0', '2'),
                expr.capture(Some("wild"), expr.any()),
            ]),
        )
        .build();

    let (text, report) = emit_ebnf(&grammar).expect("EBNF emits");

    assert_eq!(text, LIST_EBNF);
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("ordered choice")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("case-insensitive")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("capture label")));
}

#[test]
fn emits_abnf_golden_with_rfc7405_literals_and_native_repetition() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("message")
        .rule(
            "message",
            expr.seq([
                expr.term("GET"),
                expr.terminal_insensitive("header"),
                expr.opt(expr.nt("tail")),
                expr.rep1(expr.char_class(
                    false,
                    [CharClassItem::range('0', '2'), CharClassItem::char('-')],
                )),
            ]),
        )
        .rule(
            "tail",
            expr.choice_ordered([
                expr.char_range('A', 'F'),
                expr.any(),
                expr.repeat(expr.term("x"), 2, Some(4)),
                expr.capture(Some("bang"), expr.term("!")),
            ]),
        )
        .build();

    let (text, report) = emit_abnf(&grammar).expect("ABNF emits");

    assert_eq!(text, MESSAGE_ABNF);
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("ordered choice")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("capture label")));
    assert!(!report
        .lossy
        .iter()
        .any(|note| note.contains("case-insensitive")));
    let reparsed = import_abnf(&text).expect("emitted ABNF imports");
    assert!(reparsed.undefined_nonterminals().is_empty());
}

#[test]
fn emitted_text_reimports_for_basic_smoke_cases() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .rule(
            "entry",
            expr.seq([expr.nt("letter"), expr.rep0(expr.nt("letter"))]),
        )
        .rule(
            "letter",
            expr.choice_unordered([expr.term("a"), expr.term("b")]),
        )
        .build();

    let (bnf, _) = emit_bnf(&grammar).expect("BNF emits");
    let (ebnf, _) = emit_ebnf(&grammar).expect("EBNF emits");
    let (abnf, _) = emit_abnf(&grammar).expect("ABNF emits");

    assert!(import_bnf(&bnf)
        .expect("emitted BNF imports")
        .undefined_nonterminals()
        .is_empty());
    assert!(import_ebnf(&ebnf)
        .expect("emitted EBNF imports")
        .undefined_nonterminals()
        .is_empty());
    assert!(import_abnf(&abnf)
        .expect("emitted ABNF imports")
        .undefined_nonterminals()
        .is_empty());
}

#[test]
fn unsupported_constructs_report_format_and_construct() {
    let expr = Grammar::expr();

    let bnf_error = emit_bnf(&Grammar::builder().rule("bad", expr.any()).build())
        .expect_err("BNF has no any-char wildcard");
    assert_unsupported(bnf_error, GrammarFormat::Bnf, "AnyChar");

    let ebnf_error = emit_ebnf(
        &Grammar::builder()
            .rule("bad", expr.char_class(true, [CharClassItem::char('x')]))
            .build(),
    )
    .expect_err("EBNF has no negated character class");
    assert_unsupported(ebnf_error, GrammarFormat::Ebnf, "negated CharClass");

    let abnf_error = emit_abnf(
        &Grammar::builder()
            .rule("bad", expr.not(expr.term("x")))
            .build(),
    )
    .expect_err("ABNF has no predicates");
    assert_unsupported(abnf_error, GrammarFormat::Abnf, "Not");
}

fn assert_unsupported(error: GrammarEmitError, format: GrammarFormat, construct: &str) {
    assert!(matches!(
        error,
        GrammarEmitError::Unsupported {
            format: actual_format,
            construct: actual_construct,
        } if actual_format == format && actual_construct.contains(construct)
    ));
}
