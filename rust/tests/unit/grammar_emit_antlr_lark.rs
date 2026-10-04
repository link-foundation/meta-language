//! ANTLR and Lark emitters: every expression variant and rule kind, rename
//! collisions, lossy reports, re-import equivalence through `import_antlr`
//! and `import_lark`, and the emission fixpoint from the first re-import.

use meta_language::{
    CharClassItem, EmitReport, Grammar, GrammarEmitError, GrammarExpr, GrammarFormat,
    GrammarImportError, GrammarParser, GrammarRule, RuleKind, emit_antlr, emit_lark, import_abnf,
    import_antlr, import_bnf, import_ebnf, import_lark, import_pest, import_tree_sitter_json,
};
use serde_json::Value;

type Importer = fn(&str) -> Result<Grammar, GrammarImportError>;
type Emitter = fn(&Grammar) -> Result<(String, EmitReport), GrammarEmitError>;

const FIXTURE: &str = include_str!("../../../parity/fixtures/grammar-importers.json");
const TARGETS: [(&str, Emitter, Importer); 2] = [
    ("antlr", emit_antlr, import_antlr),
    ("lark", emit_lark, import_lark),
];

fn t(value: &str) -> GrammarExpr {
    GrammarExpr::Terminal(value.to_string())
}

fn nt(name: &str) -> GrammarExpr {
    GrammarExpr::NonTerminal(name.to_string())
}

const fn seq(items: Vec<GrammarExpr>) -> GrammarExpr {
    GrammarExpr::Sequence(items)
}

const fn choice(ordered: bool, alternatives: Vec<GrammarExpr>) -> GrammarExpr {
    GrammarExpr::Choice {
        ordered,
        alternatives,
    }
}

const fn class(negated: bool, items: Vec<CharClassItem>) -> GrammarExpr {
    GrammarExpr::CharClass { negated, items }
}

fn rule(name: &str, kind: RuleKind, expr: GrammarExpr) -> GrammarRule {
    GrammarRule::new(name, expr).with_kind(kind)
}

fn grammar(start: &str, rules: Vec<GrammarRule>) -> Grammar {
    rules
        .into_iter()
        .fold(Grammar::new(), Grammar::with_rule)
        .with_start(start)
}

/// Normalises what both importers normalise: nested sequences and choices
/// are flattened, empty sequence items dropped, singletons unwrapped, and
/// choice order is no longer significant.
fn normalize(expr: &GrammarExpr) -> GrammarExpr {
    match expr {
        GrammarExpr::Sequence(items) => {
            let mut flat = Vec::new();
            for item in items {
                match normalize(item) {
                    GrammarExpr::Empty => {}
                    GrammarExpr::Sequence(nested) => flat.extend(nested),
                    item => flat.push(item),
                }
            }
            match flat.len() {
                0 => GrammarExpr::Empty,
                1 => flat.remove(0),
                _ => GrammarExpr::Sequence(flat),
            }
        }
        GrammarExpr::Choice { alternatives, .. } => {
            let mut flat = Vec::new();
            for alternative in alternatives {
                match normalize(alternative) {
                    GrammarExpr::Choice {
                        alternatives: nested,
                        ..
                    } => flat.extend(nested),
                    alternative => flat.push(alternative),
                }
            }
            if flat.iter().all(|item| *item == GrammarExpr::Empty) {
                GrammarExpr::Empty
            } else if flat.len() == 1 {
                flat.remove(0)
            } else {
                choice(false, flat)
            }
        }
        GrammarExpr::Optional(inner) => GrammarExpr::optional(normalize(inner)),
        GrammarExpr::ZeroOrMore(inner) => GrammarExpr::zero_or_more(normalize(inner)),
        GrammarExpr::OneOrMore(inner) => GrammarExpr::one_or_more(normalize(inner)),
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::repeat(normalize(expr), *min, *max),
        GrammarExpr::And(inner) => GrammarExpr::and(normalize(inner)),
        GrammarExpr::Not(inner) => GrammarExpr::not(normalize(inner)),
        GrammarExpr::Capture { label, expr } => GrammarExpr::Capture {
            label: label.clone(),
            expr: Box::new(normalize(expr)),
        },
        expr => expr.clone(),
    }
}

/// Rule names, kinds and normalised expressions, in rule order.
fn structure(grammar: &Grammar) -> Vec<(String, RuleKind, GrammarExpr)> {
    grammar
        .rules()
        .iter()
        .map(|rule| (rule.name.clone(), rule.kind, normalize(rule.expr())))
        .collect()
}

fn sorted_structure(grammar: &Grammar) -> Vec<(String, RuleKind, GrammarExpr)> {
    let mut rules = structure(grammar);
    rules.sort_by(|left, right| left.0.cmp(&right.0));
    rules
}

/// Emits, re-imports, and checks that emission is a fixpoint from there on.
fn round_trip(grammar: &Grammar, emit: Emitter, import: Importer) -> (String, EmitReport, Grammar) {
    let (text, report) = emit(grammar).unwrap_or_else(|error| panic!("emits: {error}"));
    let reimported = import(&text).unwrap_or_else(|error| panic!("re-imports {text}: {error}"));
    let (again, _) = emit(&reimported).expect("re-imported grammar emits");
    let stable = import(&again).expect("re-emitted text imports");
    let (third, _) = emit(&stable).expect("stable grammar emits");
    assert_eq!(third, again, "emission is a fixpoint:\n{text}");
    assert_eq!(
        sorted_structure(&stable),
        sorted_structure(&reimported),
        "{again}"
    );
    (text, report, reimported)
}

fn assert_note(report: &EmitReport, needle: &str) {
    assert!(
        report.lossy.iter().any(|note| note.contains(needle)),
        "expected a note containing {needle:?} in {:#?}",
        report.lossy
    );
}

fn expr_of<'grammar>(grammar: &'grammar Grammar, name: &str) -> &'grammar GrammarExpr {
    grammar
        .rule(name)
        .unwrap_or_else(|| panic!("rule {name} exists"))
        .expr()
}

#[test]
fn antlr_emits_native_constructs_verbatim() {
    let g = grammar(
        "entry",
        vec![
            rule(
                "entry",
                RuleKind::Normal,
                seq(vec![
                    GrammarExpr::capture("name", nt("ID")),
                    GrammarExpr::zero_or_more(choice(false, vec![nt("item"), t("it's")])),
                    GrammarExpr::optional(seq(vec![t(","), nt("item")])),
                    GrammarExpr::one_or_more(GrammarExpr::optional(nt("item"))),
                    GrammarExpr::capture("non_greedy", GrammarExpr::zero_or_more(nt("item"))),
                    GrammarExpr::Empty,
                ]),
            ),
            rule(
                "item",
                RuleKind::Normal,
                choice(
                    false,
                    vec![
                        GrammarExpr::not(t("x")),
                        GrammarExpr::zero_or_more(GrammarExpr::not(nt("ID"))),
                        GrammarExpr::not(GrammarExpr::zero_or_more(nt("ID"))),
                        GrammarExpr::Empty,
                    ],
                ),
            ),
            rule(
                "ID",
                RuleKind::Token,
                seq(vec![
                    class(
                        false,
                        vec![
                            CharClassItem::Range('a', 'z'),
                            CharClassItem::Char('-'),
                            CharClassItem::Char(']'),
                            CharClassItem::Char('\\'),
                            CharClassItem::Char('\n'),
                        ],
                    ),
                    class(
                        true,
                        vec![CharClassItem::Char('^'), CharClassItem::Char('"')],
                    ),
                    GrammarExpr::CharRange('0', '9'),
                    GrammarExpr::AnyChar,
                    t("tab\there\u{1}"),
                ]),
            ),
            rule(
                "DIGIT",
                RuleKind::Silent,
                class(false, vec![CharClassItem::Range('0', '9')]),
            )
            .with_doc("-> skip"),
        ],
    );
    let g = g.with_source_format(GrammarFormat::Antlr);
    let (text, report, reimported) = round_trip(&g, emit_antlr, import_antlr);

    assert_eq!(
        text,
        "grammar Entry;\n\n\
         entry : name=ID (item | 'it\\'s')* (',' item)? (item?)+ item*? ;\n\
         item : ~'x' | ~ID* | ~(ID*) | ;\n\
         ID : [a-z\\-\\]\\\\\\n] ~[\\^\"] '0'..'9' . 'tab\\there\\u0001' ;\n\
         fragment DIGIT : [0-9] -> skip ;\n"
    );
    assert!(report.lossy.is_empty(), "{:#?}", report.lossy);
    assert_eq!(structure(&reimported), structure(&g));
    assert_eq!(reimported.start(), Some("entry"));
    assert_eq!(
        reimported.rule("DIGIT").and_then(GrammarRule::doc),
        Some("-> skip")
    );
}

#[test]
fn antlr_records_every_lowering() {
    let g = grammar(
        "Main",
        vec![
            rule("lexy", RuleKind::Atomic, t("a")),
            rule(
                "Main",
                RuleKind::Normal,
                choice(
                    true,
                    vec![
                        seq(vec![
                            GrammarExpr::and(nt("lexy")),
                            GrammarExpr::TerminalInsensitive("if1".to_string()),
                        ]),
                        GrammarExpr::repeat(nt("lexy"), 2, Some(4)),
                        GrammarExpr::repeat(nt("lexy"), 2, None),
                        GrammarExpr::capture_unlabeled(nt("lexy")),
                        GrammarExpr::capture("fragment", nt("lexy")),
                        seq(vec![GrammarExpr::not(t("\"")), GrammarExpr::AnyChar]),
                        GrammarExpr::not(class(false, vec![CharClassItem::Char('q')])),
                        class(true, vec![]),
                        t(""),
                    ],
                ),
            ),
            rule("quiet", RuleKind::Silent, nt("Main")).with_doc("hidden\nhelper"),
        ],
    );
    let (text, report, reimported) = round_trip(&g, emit_antlr, import_antlr);

    assert_eq!(
        text,
        "grammar Main;\n\n\
         main : [iI] [fF] '1' | Lexy Lexy (Lexy Lexy?)? | Lexy Lexy+ | Lexy | fragment_=Lexy \
         | ~[\"] | ~[q] | . | '' ;\n\
         Lexy : 'a' ;\n\
         // hidden\n\
         // helper\n\
         fragment Quiet : main ;\n"
    );
    for needle in [
        "ANTLR renamed rule \"lexy\" to \"Lexy\"",
        "ANTLR renamed rule \"Main\" to \"main\"",
        "ANTLR renamed rule \"quiet\" to \"Quiet\"",
        "ANTLR emitted atomic rule \"lexy\" as lexer rule \"Lexy\"",
        "ANTLR emitted silent rule \"quiet\" as lexer fragment \"Quiet\"",
        "ANTLR treats ordered choice as unordered choice in rule \"main\"",
        "ANTLR dropped positive lookahead in rule \"main\"",
        "ANTLR expanded case-insensitive literal \"if1\"",
        "ANTLR expanded counted repetition {2,4} in rule \"main\"",
        "ANTLR expanded counted repetition {2,} in rule \"main\"",
        "ANTLR dropped anonymous capture in rule \"main\"",
        "ANTLR renamed capture label \"fragment\" to \"fragment_\"",
        "ANTLR lowered negative lookahead followed by any character to a complemented set",
        "ANTLR emitted negative lookahead over a character set as a complemented set",
        "ANTLR emitted an empty negated character class as `.`",
        "ANTLR keeps an empty literal",
        "ANTLR lexer rule \"Quiet\" references parser rule \"main\"",
        "ANTLR parser rule \"main\" uses character-level constructs",
        "ANTLR re-imports the documentation of rule \"Quiet\" as Some(\"// hidden; // helper\")",
    ] {
        assert_note(&report, needle);
    }
    assert_eq!(reimported.start(), Some("main"));
    assert_eq!(
        reimported.rule("Lexy").map(|rule| rule.kind),
        Some(RuleKind::Token)
    );
    assert_eq!(
        reimported.rule("Quiet").map(|rule| rule.kind),
        Some(RuleKind::Silent)
    );
}

#[test]
fn antlr_renames_collisions_reserved_words_and_references() {
    let g = grammar(
        "Foo",
        vec![
            rule(
                "Foo",
                RuleKind::Normal,
                seq(vec![nt("foo"), nt("grammar"), nt("x-y")]),
            ),
            rule("foo", RuleKind::Normal, t("f")),
            rule("grammar", RuleKind::Normal, t("g")),
            rule("EOF", RuleKind::Token, t("e")),
        ],
    );
    let (text, report, reimported) = round_trip(&g, emit_antlr, import_antlr);

    assert_eq!(
        text,
        "grammar Foo2;\n\nfoo_2 : foo grammar_ x_y ;\nfoo : 'f' ;\ngrammar_ : 'g' ;\nEOF_ : 'e' ;\n"
    );
    for needle in [
        "ANTLR renamed rule \"Foo\" to \"foo_2\"",
        "ANTLR renamed rule \"grammar\" to \"grammar_\"",
        "ANTLR renamed rule \"EOF\" to \"EOF_\"",
        "ANTLR renamed non-terminal reference \"x-y\" to \"x_y\"",
    ] {
        assert_note(&report, needle);
    }
    assert_eq!(reimported.start(), Some("foo_2"));
    assert!(reimported.undefined_nonterminals().contains("x_y"));
}

#[test]
fn antlr_orders_the_start_rule_first_and_reports_lexer_only_grammars() {
    let g = grammar(
        "b",
        vec![
            rule("a", RuleKind::Normal, t("a")),
            rule("b", RuleKind::Normal, nt("a")),
        ],
    );
    let (text, _, reimported) = round_trip(&g, emit_antlr, import_antlr);
    assert!(
        text.starts_with("grammar B;\n\nb : a ;\na : 'a' ;\n"),
        "{text}"
    );
    assert_eq!(reimported.start(), Some("b"));

    let lexer_only = grammar(
        "B",
        vec![
            rule("A", RuleKind::Token, t("a")),
            rule("B", RuleKind::Token, nt("A")),
        ],
    );
    let (text, report, reimported) = round_trip(&lexer_only, emit_antlr, import_antlr);
    assert_eq!(text, "lexer grammar B;\n\nB : A ;\nA : 'a' ;\n");
    assert!(report.lossy.is_empty(), "{:#?}", report.lossy);
    assert_eq!(reimported.start(), Some("B"));

    let token_start = grammar(
        "A",
        vec![
            rule("A", RuleKind::Token, t("a")),
            rule("b", RuleKind::Normal, nt("A")),
        ],
    );
    let (_, report, _) = round_trip(&token_start, emit_antlr, import_antlr);
    assert_note(
        &report,
        "ANTLR re-imports start rule as \"b\" instead of \"A\"",
    );
}

#[test]
fn antlr_rejects_constructs_without_a_faithful_form() {
    for (expr, needle) in [
        (class(false, vec![]), "empty character class"),
        (
            GrammarExpr::CharRange('z', 'a'),
            "descending character range",
        ),
        (
            class(false, vec![CharClassItem::Range('z', 'a')]),
            "descending character class range",
        ),
        (GrammarExpr::repeat(t("a"), 3, Some(2)), "below minimum"),
        (
            GrammarExpr::repeat(t("a"), 0, Some(300)),
            "expands to 300 copies",
        ),
    ] {
        let g = grammar("a", vec![rule("a", RuleKind::Token, expr)]);
        let error = emit_antlr(&g).expect_err("unsupported");
        assert!(error.to_string().contains(needle), "{error}");
        if needle.contains("copies") {
            let (text, _) = emit_lark(&g).expect("Lark counts repetition natively");
            assert_eq!(text, "A: \"a\" ~ 0..300\n");
        } else {
            let error = emit_lark(&g).expect_err("unsupported");
            assert!(
                error.to_string().starts_with("lark emit unsupported"),
                "{error}"
            );
        }
    }
}

#[test]
fn lark_emits_native_constructs_verbatim() {
    let g = grammar(
        "start",
        vec![
            rule(
                "start",
                RuleKind::Normal,
                seq(vec![
                    nt("item"),
                    GrammarExpr::zero_or_more(seq(vec![t(","), nt("item")])),
                    GrammarExpr::optional(choice(false, vec![nt("trailer"), t("say \"hi\"\n")])),
                    GrammarExpr::one_or_more(GrammarExpr::zero_or_more(nt("WORD"))),
                    GrammarExpr::Empty,
                ]),
            ),
            rule(
                "item",
                RuleKind::Silent,
                choice(
                    false,
                    vec![
                        nt("WORD"),
                        GrammarExpr::repeat(nt("NUMBER"), 2, Some(2)),
                        GrammarExpr::repeat(nt("NUMBER"), 1, Some(3)),
                        GrammarExpr::zero_or_more(GrammarExpr::repeat(nt("WORD"), 0, Some(1))),
                        GrammarExpr::Empty,
                    ],
                ),
            )
            .with_doc("// items; inline; priority 2"),
            rule(
                "trailer",
                RuleKind::Normal,
                GrammarExpr::capture("regex", t("[a-z]+")),
            ),
            rule(
                "WORD",
                RuleKind::Token,
                class(
                    true,
                    vec![
                        CharClassItem::Range('A', 'Z'),
                        CharClassItem::Char('/'),
                        CharClassItem::Char(']'),
                        CharClassItem::Char('^'),
                        CharClassItem::Char('\u{7}'),
                        CharClassItem::Char('\u{1F600}'),
                    ],
                ),
            ),
            rule(
                "NUMBER",
                RuleKind::Token,
                class(false, vec![CharClassItem::Range('0', '9')]),
            ),
            rule("_SEP", RuleKind::Normal, t(";")),
            rule("_ignore", RuleKind::Silent, nt("WS")).with_doc("%ignore"),
            rule(
                "WS",
                RuleKind::Token,
                class(
                    false,
                    vec![CharClassItem::Char(' '), CharClassItem::Char('\t')],
                ),
            ),
        ],
    )
    .with_source_format(GrammarFormat::Lark);
    let (text, report, reimported) = round_trip(&g, emit_lark, import_lark);

    assert_eq!(
        text,
        "start: item (\",\" item)* [trailer | \"say \\\"hi\\\"\\n\"] (WORD*)+\n\
         // items\n\
         ?item.2: WORD | NUMBER ~ 2 | NUMBER ~ 1..3 | (WORD ~ 0..1)* |\n\
         trailer: /[a-z]+/\n\
         WORD: /[^A-Z\\/\\]\\^\\x07\\U0001F600]/\n\
         NUMBER: /[0-9]/\n\
         _SEP: \";\"\n\
         WS: /[ \\t]/\n\
         %ignore WS\n"
    );
    assert!(report.lossy.is_empty(), "{:#?}", report.lossy);
    let mut expected = structure(&g);
    let ignore = expected.remove(6);
    expected.push(ignore);
    assert_eq!(structure(&reimported), expected);
    assert_eq!(
        reimported.rule("item").and_then(GrammarRule::doc),
        Some("// items; inline; priority 2")
    );
    assert_eq!(reimported.start(), Some("start"));
}

#[test]
fn lark_records_every_lowering() {
    let g = grammar(
        "Main",
        vec![
            rule(
                "Main",
                RuleKind::Normal,
                choice(
                    true,
                    vec![
                        GrammarExpr::TerminalInsensitive("a.b".to_string()),
                        GrammarExpr::CharRange('a', 'f'),
                        GrammarExpr::AnyChar,
                        GrammarExpr::repeat(nt("tok"), 0, None),
                        GrammarExpr::repeat(nt("tok"), 1, None),
                        GrammarExpr::repeat(nt("tok"), 3, None),
                        seq(vec![
                            GrammarExpr::and(nt("tok")),
                            GrammarExpr::not(nt("tok")),
                            nt("tok"),
                        ]),
                        seq(vec![
                            GrammarExpr::not(class(false, vec![CharClassItem::Char('"')])),
                            GrammarExpr::AnyChar,
                        ]),
                        GrammarExpr::capture("name", nt("tok")),
                        GrammarExpr::capture_unlabeled(nt("tok")),
                        GrammarExpr::capture("regex", t("a/b\n")),
                        GrammarExpr::capture("regex", t("[xy]")),
                        GrammarExpr::capture("regex", t("")),
                        class(true, vec![]),
                        t(""),
                    ],
                ),
            ),
            rule("tok", RuleKind::Atomic, nt("Main")).with_doc("line one\nline two; priority 3"),
            rule("_ignore", RuleKind::Silent, t(" ")).with_doc("%ignore"),
            rule("Quiet", RuleKind::Silent, t("q")),
        ],
    );
    let (text, report, reimported) = round_trip(&g, emit_lark, import_lark);

    assert_eq!(
        text,
        "main: /(?i:a\\.b)/ | /[a-f]/ | /[\\x00-\\U0010FFFF]/ | TOK* | TOK+ | TOK ~ 3 TOK* | TOK \
         | /[^\"]/ | TOK | TOK | /a\\/b\\n/ | /[xy]/ | /(?:)/ | /[\\x00-\\U0010FFFF]/ | \"\"\n\
         // line one\n\
         // line two\n\
         TOK.3: main\n\
         ?quiet: \"q\"\n\
         %ignore \" \"\n"
    );
    for needle in [
        "Lark renamed rule \"Main\" to \"main\"",
        "Lark renamed rule \"tok\" to \"TOK\"",
        "Lark renamed rule \"Quiet\" to \"quiet\"",
        "Lark emitted atomic rule \"tok\" as terminal \"TOK\"",
        "Lark treats ordered choice as unordered choice in rule \"main\"",
        "Lark emitted case-insensitive literal \"a.b\" as a (?i:...) regex",
        "Lark emitted character range as a regex character class",
        "Lark emitted any character as a regex character class",
        "Lark emitted unbounded repetition {0,} as `*`",
        "Lark emitted unbounded repetition {1,} as `+`",
        "Lark emitted unbounded repetition {3,} as `~ 3` followed by `*`",
        "Lark dropped positive lookahead",
        "Lark dropped negative lookahead",
        "Lark lowered negative lookahead followed by any character",
        "Lark dropped capture label \"name\"",
        "Lark dropped anonymous capture",
        "Lark escaped regex /a/b\n/ as /a\\/b\\n/",
        "Lark regex /[xy]/ re-imports as a character class",
        "Lark emitted an empty regex as /(?:)/",
        "Lark emitted an empty negated character class as any character",
        "Lark keeps an empty literal",
        "Lark terminal \"TOK\" references rule \"main\"",
        "Lark re-imports the documentation of rule \"TOK\" as Some(\"// line one; // line two; priority 3\")",
        "Lark re-imports the documentation of rule \"quiet\" as Some(\"inline\")",
        "Lark re-imports start rule as \"main\" instead of \"main\"",
    ]
    .into_iter()
    .filter(|needle| !needle.contains("instead of \"main\""))
    {
        assert_note(&report, needle);
    }
    assert_eq!(reimported.start(), Some("main"));
    assert_eq!(expr_of(&reimported, "_ignore"), &t(" "));
    assert_eq!(
        reimported.rule("TOK").map(|rule| rule.kind),
        Some(RuleKind::Token)
    );
}

#[test]
fn lark_renames_collisions_and_keeps_ignore_names() {
    let g = grammar(
        "Foo",
        vec![
            rule(
                "Foo",
                RuleKind::Normal,
                seq(vec![nt("foo"), nt("_ignore"), nt("Some-Ref"), nt("other")]),
            ),
            rule("foo", RuleKind::Normal, t("f")),
            rule("_ignore", RuleKind::Normal, t("i")),
            rule("ws", RuleKind::Silent, t(" ")).with_doc("%ignore"),
            rule("1st", RuleKind::Token, t("1")),
            rule("start", RuleKind::Normal, t("s")),
        ],
    );
    let (text, report, reimported) = round_trip(&g, emit_lark, import_lark);

    assert_eq!(
        text,
        "foo_2: foo _ignore_2 SOME_REF other\nfoo: \"f\"\n_ignore_2: \"i\"\nT_1ST: \"1\"\nstart: \"s\"\n%ignore \" \"\n"
    );
    for needle in [
        "Lark renamed rule \"Foo\" to \"foo_2\"",
        "Lark renamed rule \"_ignore\" to \"_ignore_2\"",
        "Lark renamed rule \"ws\" to \"_ignore\"",
        "Lark renamed rule \"1st\" to \"T_1ST\"",
        "Lark renamed non-terminal reference \"Some-Ref\" to \"SOME_REF\"",
        "Lark re-imports start rule as \"start\" instead of \"foo_2\"",
    ] {
        assert_note(&report, needle);
    }
    assert_eq!(reimported.start(), Some("start"));
    assert_eq!(
        reimported.rule("_ignore").and_then(GrammarRule::doc),
        Some("%ignore")
    );
}

#[test]
fn every_rule_kind_round_trips_to_its_target_kind() {
    let g = grammar(
        "normal",
        vec![
            rule(
                "normal",
                RuleKind::Normal,
                seq(vec![nt("TOKEN"), nt("atomic")]),
            ),
            rule("TOKEN", RuleKind::Token, t("t")),
            rule("atomic", RuleKind::Atomic, t("a")),
            rule("silent", RuleKind::Silent, t("s")),
        ],
    );
    let (_, _, antlr) = round_trip(&g, emit_antlr, import_antlr);
    let kinds = |grammar: &Grammar| {
        grammar
            .rules()
            .iter()
            .map(|rule| (rule.name.clone(), rule.kind))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        kinds(&antlr),
        [
            ("normal".to_string(), RuleKind::Normal),
            ("TOKEN".to_string(), RuleKind::Token),
            ("Atomic".to_string(), RuleKind::Token),
            ("Silent".to_string(), RuleKind::Silent),
        ]
    );
    let (_, _, lark) = round_trip(&g, emit_lark, import_lark);
    assert_eq!(
        kinds(&lark),
        [
            ("normal".to_string(), RuleKind::Normal),
            ("TOKEN".to_string(), RuleKind::Token),
            ("ATOMIC".to_string(), RuleKind::Token),
            ("silent".to_string(), RuleKind::Silent),
        ]
    );
}

#[test]
fn antlr_sources_round_trip_with_their_rule_structure() {
    for source in [
        include_str!("../fixtures/grammar/antlr/arithmetic.g4"),
        include_str!("../fixtures/grammar/antlr/covering.g4"),
        include_str!("../fixtures/grammar/antlr/lexer-mode.g4"),
        "grammar Comments;\n start : 'a' // first branch\n | ('b' /* group end */) ;",
        "grammar Missing; start : missing ;",
    ] {
        let g = import_antlr(source).expect("fixture imports");
        let (text, report, reimported) = round_trip(&g, emit_antlr, import_antlr);
        assert_eq!(structure(&reimported), structure(&g), "{text}");
        assert_eq!(reimported.start(), g.start(), "{text}");
        assert!(
            report
                .lossy
                .iter()
                .all(|note| note.contains("re-imports the documentation")),
            "{:#?}",
            report.lossy
        );
    }
    let covering =
        import_antlr(include_str!("../fixtures/grammar/antlr/covering.g4")).expect("imports");
    let (text, _, reimported) = round_trip(&covering, emit_antlr, import_antlr);
    assert!(
        text.contains("COMMENT : '//' ~[\\r\\n]* -> channel(HIDDEN) ;"),
        "{text}"
    );
    assert!(
        text.contains("entry : name=ID values=item*? literalRange item ;"),
        "{text}"
    );
    assert_eq!(
        reimported.rule("COMMENT").and_then(GrammarRule::doc),
        Some("-> channel(HIDDEN)")
    );
}

#[test]
fn lark_sources_round_trip_with_their_rule_structure() {
    for source in [
        include_str!("../fixtures/grammar/lark/covering.lark"),
        "other: \"x\"\nstart: other\n",
        "a.3: \"\\x41\" /b+/i?\n",
    ] {
        let g = import_lark(source).expect("fixture imports");
        let (text, report, reimported) = round_trip(&g, emit_lark, import_lark);
        assert_eq!(structure(&reimported), structure(&g), "{text}");
        assert_eq!(reimported.start(), g.start(), "{text}");
        assert_eq!(
            reimported
                .rules()
                .iter()
                .map(GrammarRule::doc)
                .collect::<Vec<_>>(),
            g.rules().iter().map(GrammarRule::doc).collect::<Vec<_>>(),
            "{text}"
        );
        assert!(report.lossy.is_empty(), "{:#?}", report.lossy);
    }
}

#[test]
fn cross_format_sources_keep_rule_structure() {
    let antlr =
        import_antlr(include_str!("../fixtures/grammar/antlr/arithmetic.g4")).expect("imports");
    let (_, _, lark) = round_trip(&antlr, emit_lark, import_lark);
    let (_, _, back) = round_trip(&lark, emit_antlr, import_antlr);
    assert_eq!(
        back.rules()
            .iter()
            .map(|rule| rule.kind)
            .collect::<Vec<_>>(),
        antlr
            .rules()
            .iter()
            .map(|rule| rule.kind)
            .collect::<Vec<_>>()
    );

    let lark =
        import_lark(include_str!("../fixtures/grammar/lark/covering.lark")).expect("imports");
    let (text, report, _) = round_trip(&lark, emit_antlr, import_antlr);
    assert!(
        text.contains("trailer : [a-z]+ ;") || text.contains("regex="),
        "{text}"
    );
    assert_ne!(report.lossy, [] as [String; 0]);
}

fn fixture_importer(format: &str) -> Importer {
    match format {
        "abnf" => import_abnf,
        "bnf" => import_bnf,
        "ebnf" => import_ebnf,
        "pest" => import_pest,
        "tree-sitter-json" => import_tree_sitter_json,
        format => panic!("no importer for {format}"),
    }
}

#[test]
fn shared_importer_fixtures_round_trip_through_antlr_and_lark() {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("fixture is JSON");
    let cases = fixture["cases"].as_array().expect("cases");
    assert_ne!(cases.as_slice(), [] as [Value; 0]);
    for case in cases {
        let id = case["id"].as_str().expect("id");
        let format = case["format"].as_str().expect("format");
        let source = case["source"].as_str().expect("source");
        let grammar =
            fixture_importer(format)(source).unwrap_or_else(|error| panic!("{id}: {error}"));
        for (target, emit, import) in TARGETS {
            let (text, report, reimported) = round_trip(&grammar, emit, import);
            assert_eq!(
                reimported.rules().len(),
                grammar.rules().len(),
                "{id} -> {target}:\n{text}"
            );
            let renamed = report.lossy.iter().any(|note| note.contains(" renamed "));
            if !renamed {
                let names = |grammar: &Grammar| {
                    grammar
                        .rules()
                        .iter()
                        .map(|rule| rule.name.clone())
                        .collect::<Vec<_>>()
                };
                let mut expected = names(&grammar);
                let mut actual = names(&reimported);
                expected.sort();
                actual.sort();
                assert_eq!(actual, expected, "{id} -> {target}");
            }
            for accepted in case["accepts"].as_array().into_iter().flatten() {
                let accepted = accepted.as_str().expect("corpus entry");
                if report
                    .lossy
                    .iter()
                    .all(|note| note.contains(" renamed ") || note.contains("ordered choice"))
                {
                    assert!(
                        GrammarParser::new(reimported.clone()).accepts(accepted),
                        "{id} -> {target} accepts {accepted:?}:\n{text}"
                    );
                }
            }
        }
    }
}

#[test]
fn unusual_rule_names_reach_a_naming_fixpoint() {
    let names = [
        "1st", "x-y", "__", "_", "Über", "aB", "AB_c", "T_1ST", "_WS", "r_", "start", "EOF",
        "fragment", "mode",
    ];
    for kind in [
        RuleKind::Normal,
        RuleKind::Token,
        RuleKind::Atomic,
        RuleKind::Silent,
    ] {
        let rules = names
            .iter()
            .map(|name| rule(name, kind, t(name)))
            .chain([rule(
                "root",
                RuleKind::Normal,
                seq(names.iter().map(|name| nt(name)).collect()),
            )])
            .collect();
        let g = grammar("root", rules);
        for (target, emit, import) in TARGETS {
            let (text, _, reimported) = round_trip(&g, emit, import);
            assert_eq!(
                reimported.rules().len(),
                g.rules().len(),
                "{target}:\n{text}"
            );
            assert!(
                reimported.undefined_nonterminals().is_empty(),
                "{target}:\n{text}"
            );
        }
    }
}
