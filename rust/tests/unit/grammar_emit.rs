use meta_language::{
    emit_abnf, emit_bnf, emit_ebnf, emit_gbnf, emit_pest, import_abnf, import_bnf, import_ebnf,
    import_gbnf, import_pest, CharClassItem, Grammar, GrammarEmitError, GrammarFormat, GrammarRule,
    RuleKind,
};

const LIST_BNF: &str = include_str!("../fixtures/grammar/emit/list.bnf");
const LIST_EBNF: &str = include_str!("../fixtures/grammar/emit/list.ebnf");
const MESSAGE_ABNF: &str = include_str!("../fixtures/grammar/emit/message.abnf");
const COVERING_PEST: &str = include_str!("../fixtures/grammar/emit/covering.pest");
const COVERING_GBNF: &str = include_str!("../fixtures/grammar/emit/covering.gbnf");
const JSON_OBJECT_GBNF: &str = include_str!("../fixtures/grammar/emit/json-object.gbnf");

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

    assert_eq!(text, normalized_fixture(LIST_BNF));
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

    assert_eq!(text, normalized_fixture(LIST_EBNF));
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

    assert_eq!(text, normalized_fixture(MESSAGE_ABNF));
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
fn emits_pest_golden_with_native_peg_operators_and_rule_modifiers() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("start")
        .grammar_rule(
            GrammarRule::new(
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

    let (text, report) = emit_pest(&grammar).expect("pest emits");

    assert_eq!(text, normalized_fixture(COVERING_PEST));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("unordered choice")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("capture label")));
    assert!(!report
        .lossy
        .iter()
        .any(|note| note.contains("anonymous capture")));

    let reparsed = import_pest(&text).expect("emitted pest imports");
    assert_eq!(reparsed.source_format(), Some(GrammarFormat::Peg));
    assert!(reparsed.undefined_nonterminals().is_empty());
}

#[test]
fn emits_gbnf_golden_with_root_mapping_native_operators_and_lossy_report() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("entry")
        .rule("root", expr.term("shadow"))
        .rule(
            "entry",
            expr.seq([
                expr.term("start"),
                expr.nt("root"),
                expr.char_range('A', 'Z'),
                expr.char_class(
                    false,
                    [
                        CharClassItem::range('a', 'c'),
                        CharClassItem::char('_'),
                        CharClassItem::char('?'),
                    ],
                ),
                expr.char_class(true, [CharClassItem::char('"'), CharClassItem::char('\n')]),
                expr.any(),
                expr.terminal_insensitive("Case"),
                expr.choice_ordered([expr.term("a"), expr.term("b")]),
                expr.opt(expr.nt("item")),
                expr.rep0(expr.nt("item")),
                expr.rep1(expr.nt("item")),
                expr.repeat(expr.nt("item"), 2, Some(4)),
                expr.repeat(expr.nt("item"), 3, None),
                expr.repeat(expr.nt("item"), 2, Some(2)),
                expr.seq([
                    expr.not(expr.char_class(
                        false,
                        [CharClassItem::char('x'), CharClassItem::range('0', '9')],
                    )),
                    expr.any(),
                ]),
                expr.capture(Some("label"), expr.term("cap")),
                expr.empty(),
            ]),
        )
        .rule("item", expr.term("x"))
        .rule("bad name", expr.nt("entry"))
        .build();

    let (text, report) = emit_gbnf(&grammar).expect("GBNF emits");

    assert_eq!(text, normalized_fixture(COVERING_GBNF));
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
        .any(|note| note.contains("capture label")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("renamed rule \"root\"")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("renamed rule \"bad name\"")));

    let reparsed = import_gbnf(&text).expect("emitted GBNF imports");
    assert_eq!(reparsed.source_format(), Some(GrammarFormat::Gbnf));
    assert!(reparsed.undefined_nonterminals().is_empty());
}

#[test]
fn emits_gbnf_json_object_fixture_for_llm_constraints() {
    let expr = Grammar::expr();
    let comma_member = expr.seq([expr.term(","), expr.nt("ws"), expr.nt("member")]);
    let comma_value = expr.seq([expr.term(","), expr.nt("ws"), expr.nt("value")]);
    let digit = expr.char_range('0', '9');
    let grammar = Grammar::builder()
        .start("object")
        .rule(
            "object",
            expr.seq([
                expr.term("{"),
                expr.nt("ws"),
                expr.opt(expr.seq([expr.nt("member"), expr.rep0(comma_member)])),
                expr.term("}"),
                expr.nt("ws"),
            ]),
        )
        .rule(
            "member",
            expr.seq([
                expr.nt("string"),
                expr.term(":"),
                expr.nt("ws"),
                expr.nt("value"),
            ]),
        )
        .rule(
            "value",
            expr.choice_unordered([
                expr.nt("string"),
                expr.nt("number"),
                expr.nt("object"),
                expr.nt("array"),
                expr.term("true"),
                expr.term("false"),
                expr.term("null"),
            ]),
        )
        .rule(
            "array",
            expr.seq([
                expr.term("["),
                expr.nt("ws"),
                expr.opt(expr.seq([expr.nt("value"), expr.rep0(comma_value)])),
                expr.term("]"),
                expr.nt("ws"),
            ]),
        )
        .rule(
            "string",
            expr.seq([
                expr.term("\""),
                expr.rep0(expr.choice_unordered([
                    expr.char_class(true, [CharClassItem::char('"'), CharClassItem::char('\\')]),
                    expr.seq([
                        expr.term("\\"),
                        expr.char_class(
                            false,
                            [
                                CharClassItem::char('"'),
                                CharClassItem::char('\\'),
                                CharClassItem::char('/'),
                                CharClassItem::char('b'),
                                CharClassItem::char('f'),
                                CharClassItem::char('n'),
                                CharClassItem::char('r'),
                                CharClassItem::char('t'),
                            ],
                        ),
                    ]),
                ])),
                expr.term("\""),
                expr.nt("ws"),
            ]),
        )
        .rule(
            "number",
            expr.seq([
                expr.opt(expr.term("-")),
                expr.nt("int"),
                expr.opt(expr.seq([expr.term("."), expr.rep1(digit.clone())])),
                expr.opt(expr.seq([
                    expr.char_class(false, [CharClassItem::char('e'), CharClassItem::char('E')]),
                    expr.opt(expr.choice_unordered([expr.term("-"), expr.term("+")])),
                    expr.rep1(digit),
                ])),
                expr.nt("ws"),
            ]),
        )
        .rule(
            "int",
            expr.choice_unordered([
                expr.term("0"),
                expr.seq([
                    expr.char_range('1', '9'),
                    expr.rep0(expr.char_range('0', '9')),
                ]),
            ]),
        )
        .rule(
            "ws",
            expr.rep0(expr.char_class(
                false,
                [
                    CharClassItem::char(' '),
                    CharClassItem::char('\t'),
                    CharClassItem::char('\n'),
                    CharClassItem::char('\r'),
                ],
            )),
        )
        .build();

    let (text, report) = emit_gbnf(&grammar).expect("GBNF emits JSON object grammar");

    assert_eq!(text, normalized_fixture(JSON_OBJECT_GBNF));
    assert!(report.lossy.is_empty());
    assert!(import_gbnf(&text)
        .expect("emitted JSON GBNF imports")
        .undefined_nonterminals()
        .is_empty());
}

#[test]
fn gbnf_folds_single_char_negative_predicate_before_any_char() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .rule("entry", expr.seq([expr.not(expr.term(",")), expr.any()]))
        .build();

    let (text, report) = emit_gbnf(&grammar).expect("GBNF emits predicate peephole");

    assert_eq!(text, "root ::= [^,]\n");
    assert!(report.lossy.is_empty());
}

#[test]
fn pest_escapes_literals_and_char_ranges() {
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
                    ],
                ),
            ]),
        )
        .build();

    let (text, report) = emit_pest(&grammar).expect("pest emits escaped literals");

    assert_eq!(
        text,
        "escaped = { \"\\\"\\\\\\n\\t\" ~ '\\n'..'\\n' ~ (\"\\\"\" | \"\\\\\" | '\\t'..'\\t') }\n"
    );
    assert!(report.lossy.is_empty());
    assert!(import_pest(&text)
        .expect("escaped pest imports")
        .undefined_nonterminals()
        .is_empty());
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
    let (pest, _) = emit_pest(&grammar).expect("pest emits");
    let (gbnf, _) = emit_gbnf(&grammar).expect("GBNF emits");

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
    assert!(import_pest(&pest)
        .expect("emitted pest imports")
        .undefined_nonterminals()
        .is_empty());
    assert!(import_gbnf(&gbnf)
        .expect("emitted GBNF imports")
        .undefined_nonterminals()
        .is_empty());
}

#[test]
fn bnf_emits_helpers_for_nested_choices_repetition_classes_and_captures() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("entry")
        .rule(
            "entry",
            expr.seq([
                expr.choice_ordered([expr.term("x"), expr.term("y")]),
                expr.rep1(expr.term("!")),
                expr.repeat(expr.term("?"), 1, Some(3)),
                expr.repeat(expr.term("#"), 2, None),
                expr.char_class(
                    false,
                    [CharClassItem::range('0', '1'), CharClassItem::char('_')],
                ),
                expr.capture(Some("label"), expr.term("z")),
            ]),
        )
        .build();

    let (text, report) = emit_bnf(&grammar).expect("BNF emits");

    assert_eq!(
        text,
        concat!(
            "<entry> ::= <mlchoice0> <mlplus1> \"?\" <mlopt2> <mlopt2> ",
            "\"#\" \"#\" <mlstar3> <mlclass4> \"z\"\n",
            "<mlchoice0> ::= \"x\" | \"y\"\n",
            "<mlplus1> ::= \"!\" <mlplus1> | \"!\"\n",
            "<mlopt2> ::= \"?\" |\n",
            "<mlstar3> ::= \"#\" <mlstar3> |\n",
            "<mlclass4> ::= \"0\" | \"1\" | \"_\"\n",
        )
    );
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("ordered choice")));
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("capture label")));
    assert!(import_bnf(&text)
        .expect("emitted BNF imports")
        .undefined_nonterminals()
        .is_empty());
}

#[test]
fn ebnf_emits_grouping_repetition_classes_quotes_and_captures() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("entry")
        .rule(
            "entry",
            expr.seq([
                expr.choice_unordered([expr.term("a"), expr.term("b")]),
                expr.rep1(expr.term("c")),
                expr.repeat(expr.term("d"), 2, Some(4)),
                expr.repeat(expr.term("e"), 0, Some(2)),
                expr.char_class(
                    false,
                    [CharClassItem::range('x', 'z'), CharClassItem::char('"')],
                ),
                expr.term("has \"quote"),
                expr.capture_unlabeled(expr.term("cap")),
            ]),
        )
        .build();

    let (text, report) = emit_ebnf(&grammar).expect("EBNF emits");

    assert_eq!(
        text,
        concat!(
            "entry = (\"a\" | \"b\") , \"c\" , { \"c\" } , \"d\" , \"d\" , ",
            "[ \"d\" ] , [ \"d\" ] , [ \"e\" ] , [ \"e\" ] , mlclass0 , ",
            "'has \"quote' , \"cap\" ;\n",
            "mlclass0 = \"x\" | \"y\" | \"z\" | '\"' ;\n",
        )
    );
    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("anonymous capture")));
    assert!(import_ebnf(&text)
        .expect("emitted EBNF imports")
        .undefined_nonterminals()
        .is_empty());
}

#[test]
fn abnf_emits_numeric_literals_empty_and_repeat_prefix_forms() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("entry")
        .rule(
            "entry",
            expr.seq([
                expr.term("\""),
                expr.term("\n"),
                expr.rep0(expr.term("a")),
                expr.repeat(expr.term("b"), 0, Some(2)),
                expr.repeat(expr.term("c"), 2, None),
            ]),
        )
        .rule("empty", expr.empty())
        .build();

    let (text, report) = emit_abnf(&grammar).expect("ABNF emits");

    assert_eq!(
        text,
        concat!(
            "entry = %x22 %x0A *( %s\"a\" ) *2( %s\"b\" ) 2*( %s\"c\" )\n",
            "empty = \"\"\n",
        )
    );
    assert!(report.lossy.is_empty());
    assert!(import_abnf(&text)
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

    let bnf_error = emit_bnf(
        &Grammar::builder()
            .rule("bad", expr.and(expr.term("x")))
            .build(),
    )
    .expect_err("BNF has no predicates");
    assert_unsupported(bnf_error, GrammarFormat::Bnf, "And");

    let bnf_error = emit_bnf(
        &Grammar::builder()
            .rule("bad", expr.char_class(true, [CharClassItem::char('x')]))
            .build(),
    )
    .expect_err("BNF has no negated character class");
    assert_unsupported(bnf_error, GrammarFormat::Bnf, "negated CharClass");

    let bnf_error = emit_bnf(
        &Grammar::builder()
            .rule("bad", expr.char_range('\u{0}', char::MAX))
            .build(),
    )
    .expect_err("BNF does not expand huge character ranges");
    assert_unsupported(bnf_error, GrammarFormat::Bnf, "expands to");

    let ebnf_error = emit_ebnf(
        &Grammar::builder()
            .rule("bad", expr.char_class(true, [CharClassItem::char('x')]))
            .build(),
    )
    .expect_err("EBNF has no negated character class");
    assert_unsupported(ebnf_error, GrammarFormat::Ebnf, "negated CharClass");

    let ebnf_error = emit_ebnf(
        &Grammar::builder()
            .rule("bad", expr.not(expr.term("x")))
            .build(),
    )
    .expect_err("EBNF has no predicates");
    assert_unsupported(ebnf_error, GrammarFormat::Ebnf, "Not");

    let ebnf_error = emit_ebnf(
        &Grammar::builder()
            .rule("bad", expr.char_range('z', 'a'))
            .build(),
    )
    .expect_err("EBNF does not expand descending ranges");
    assert_unsupported(ebnf_error, GrammarFormat::Ebnf, "descending bounds");

    let abnf_error = emit_abnf(
        &Grammar::builder()
            .rule("bad", expr.not(expr.term("x")))
            .build(),
    )
    .expect_err("ABNF has no predicates");
    assert_unsupported(abnf_error, GrammarFormat::Abnf, "Not");

    let abnf_error = emit_abnf(
        &Grammar::builder()
            .rule("bad", expr.char_class(true, [CharClassItem::char('x')]))
            .build(),
    )
    .expect_err("ABNF has no negated character class");
    assert_unsupported(abnf_error, GrammarFormat::Abnf, "negated CharClass");

    let abnf_error = emit_abnf(
        &Grammar::builder()
            .rule("bad", expr.char_class(false, []))
            .build(),
    )
    .expect_err("ABNF has no empty character class");
    assert_unsupported(abnf_error, GrammarFormat::Abnf, "empty CharClass");

    let abnf_error = emit_abnf(
        &Grammar::builder()
            .rule("bad", expr.repeat(expr.term("x"), 3, Some(2)))
            .build(),
    )
    .expect_err("ABNF rejects invalid repeat bounds");
    assert_unsupported(abnf_error, GrammarFormat::Abnf, "greater than max");

    let pest_error = emit_pest(
        &Grammar::builder()
            .rule("bad", expr.choice_ordered([]))
            .build(),
    )
    .expect_err("pest rejects empty choices");
    assert_unsupported(pest_error, GrammarFormat::Peg, "empty Choice");

    let pest_error = emit_pest(
        &Grammar::builder()
            .rule("bad", expr.char_class(false, []))
            .build(),
    )
    .expect_err("pest rejects empty character classes");
    assert_unsupported(pest_error, GrammarFormat::Peg, "empty CharClass");

    let pest_error = emit_pest(
        &Grammar::builder()
            .rule("bad", expr.repeat(expr.term("x"), 3, Some(2)))
            .build(),
    )
    .expect_err("pest rejects invalid repeat bounds");
    assert_unsupported(pest_error, GrammarFormat::Peg, "greater than max");

    let gbnf_error = emit_gbnf(
        &Grammar::builder()
            .rule("bad", expr.and(expr.term("x")))
            .build(),
    )
    .expect_err("GBNF has no positive lookahead");
    assert_unsupported(gbnf_error, GrammarFormat::Gbnf, "and-predicate");

    let gbnf_error = emit_gbnf(
        &Grammar::builder()
            .rule("bad", expr.not(expr.term("xy")))
            .build(),
    )
    .expect_err("GBNF rejects general negative lookahead");
    assert_unsupported(gbnf_error, GrammarFormat::Gbnf, "not-predicate");

    let gbnf_error = emit_gbnf(
        &Grammar::builder()
            .rule("bad", expr.char_class(false, []))
            .build(),
    )
    .expect_err("GBNF rejects empty character classes");
    assert_unsupported(gbnf_error, GrammarFormat::Gbnf, "empty CharClass");

    let gbnf_error = emit_gbnf(
        &Grammar::builder()
            .rule("bad", expr.repeat(expr.term("x"), 3, Some(2)))
            .build(),
    )
    .expect_err("GBNF rejects invalid repeat bounds");
    assert_unsupported(gbnf_error, GrammarFormat::Gbnf, "greater than max");
}

fn assert_unsupported(error: GrammarEmitError, format: GrammarFormat, construct: &str) {
    let display = error.to_string();
    assert!(matches!(
        error,
        GrammarEmitError::Unsupported {
            format: actual_format,
            construct: actual_construct,
        } if actual_format == format && actual_construct.contains(construct)
    ));
    assert!(display.contains(format.as_str()));
    assert!(display.contains(construct));
}

fn normalized_fixture(value: &str) -> String {
    value.replace("\r\n", "\n")
}
