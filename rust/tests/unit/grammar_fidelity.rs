use std::fmt::Write as _;

use meta_language::{
    emit_bnf, grammar_format_profile, import_bnf, CharClassItem, Grammar, GrammarEmitError,
    GrammarFidelityLevel, GrammarFormat, GrammarRule, RuleKind, GRAMMAR_CONSTRUCTS,
    GRAMMAR_FORMATS,
};

#[test]
fn every_grammar_format_profile_reports_support_or_a_fallback_for_each_construct() {
    for &format in GRAMMAR_FORMATS {
        let profile = grammar_format_profile(format).expect("known grammar format");
        for &construct in GRAMMAR_CONSTRUCTS {
            let supported = profile.supports_construct(construct);
            let fallback = profile.construct_fallback(construct).is_some();
            assert!(
                supported ^ fallback,
                "{format}: `{construct}` must be either lossless/equivalent or have exactly one documented fallback"
            );
        }
    }
}

#[test]
fn bnf_profile_classifies_native_and_lossy_constructs() {
    assert_eq!(GRAMMAR_FORMATS, ["bnf"]);

    let profile = grammar_format_profile("Backus-Naur Form").expect("BNF profile");

    for construct in [
        "empty",
        "sequence",
        "unordered-choice",
        "terminal",
        "non-terminal",
    ] {
        assert!(profile.supports_construct(construct), "{construct}");
        assert_eq!(
            profile.construct_fidelity(construct),
            Some(GrammarFidelityLevel::Lossless),
            "{construct}"
        );
    }

    for construct in [
        "ordered-choice",
        "optional",
        "zero-or-more",
        "one-or-more",
        "repeat-range",
        "char-range",
        "char-class",
        "any-char",
        "case-insensitive-terminal",
        "and-predicate",
        "not-predicate",
        "capture",
        "rule-kind-atomic",
        "rule-kind-silent",
        "rule-kind-token",
    ] {
        assert!(!profile.supports_construct(construct), "{construct}");
        assert!(
            profile.construct_fallback(construct).is_some(),
            "{construct} should document its BNF fallback"
        );
        assert_eq!(
            profile.construct_fidelity(construct),
            Some(GrammarFidelityLevel::Lossy),
            "{construct}"
        );
    }
}

#[test]
fn lossless_bnf_constructs_round_trip_through_the_ir() {
    let source = r#"
<start> ::= "a" <tail> | "b"
<tail> ::= "c" "d" |
"#;

    let imported = import_bnf(source).expect("source BNF imports");
    let (emitted, report) = emit_bnf(&imported).expect("BNF emits");
    assert!(
        report.lossy.is_empty(),
        "native BNF constructs should not produce lossy notes: {:?}",
        report.lossy
    );
    let reparsed = import_bnf(&emitted).expect("emitted BNF reparses");

    assert_eq!(reparsed.source_format(), Some(GrammarFormat::Bnf));
    assert_eq!(reparsed.rule_names(), imported.rule_names());
    assert_eq!(reparsed.rules(), imported.rules());
}

#[test]
fn lossy_bnf_constructs_use_documented_fallbacks() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("start")
        .rule(
            "start",
            expr.seq([
                expr.choice_ordered([expr.term("a"), expr.term("b")]),
                expr.terminal_insensitive("case"),
                expr.opt(expr.term("?")),
                expr.rep0(expr.term("*")),
                expr.rep1(expr.term("+")),
                expr.repeat(expr.term("r"), 2, Some(4)),
                expr.char_range('0', '2'),
                expr.char_class(
                    false,
                    [CharClassItem::char('_'), CharClassItem::range('x', 'z')],
                ),
                expr.capture(Some("label"), expr.term("cap")),
            ]),
        )
        .rule_with_kind("atomic_rule", expr.term("atomic"), RuleKind::Atomic)
        .rule_with_kind("silent_rule", expr.term("silent"), RuleKind::Silent)
        .rule_with_kind("token_rule", expr.term("token"), RuleKind::Token)
        .build();

    let (emitted, report) = emit_bnf(&grammar).expect("BNF emits with fallbacks");

    for helper_kind in [
        "mlchoice", "mlopt", "mlstar", "mlplus", "mlrange", "mlclass",
    ] {
        assert!(
            emitted.contains(helper_kind),
            "BNF fallback should synthesize a {helper_kind} helper:\n{emitted}"
        );
    }
    for note in [
        "ordered choice",
        "case-insensitive terminal",
        "capture label",
    ] {
        assert!(
            report.lossy.iter().any(|loss| loss.contains(note)),
            "BNF report should mention {note}: {:?}",
            report.lossy
        );
    }

    let reparsed = import_bnf(&emitted).expect("fallback BNF reparses");
    assert!(reparsed.undefined_nonterminals().is_empty());
    for dropped_kind_rule in ["atomic_rule", "silent_rule", "token_rule"] {
        assert_eq!(
            reparsed.rule(dropped_kind_rule).map(GrammarRule::kind),
            Some(RuleKind::Normal),
            "{dropped_kind_rule} should re-import as a normal BNF production"
        );
    }
}

#[test]
fn bnf_unrepresentable_constructs_return_documented_errors() {
    let profile = grammar_format_profile("bnf").expect("BNF profile");
    let expr = Grammar::expr();

    for (construct, grammar, expected_error) in [
        (
            "any-char",
            Grammar::builder().rule("start", expr.any()).build(),
            "AnyChar",
        ),
        (
            "and-predicate",
            Grammar::builder()
                .rule("start", expr.and(expr.term("peek")))
                .build(),
            "And",
        ),
        (
            "not-predicate",
            Grammar::builder()
                .rule("start", expr.not(expr.term("skip")))
                .build(),
            "Not",
        ),
    ] {
        assert!(
            profile
                .construct_fallback(construct)
                .is_some_and(|fallback| fallback.contains("unsupported")),
            "{construct} should document the unsupported-emission fallback"
        );
        let error = emit_bnf(&grammar).expect_err("construct is not representable in BNF");
        assert!(matches!(
            error,
            GrammarEmitError::Unsupported {
                format: GrammarFormat::Bnf,
                ref construct
            } if construct == expected_error
        ));
    }
}

#[test]
fn fidelity_doc_matrix_matches_generated_matrix() {
    let doc = std::fs::read_to_string(fidelity_doc_path())
        .expect("grammar fidelity documentation should exist");
    let doc = doc.replace("\r\n", "\n");

    assert!(
        doc.contains(&generated_matrix()),
        "committed fidelity doc should contain the generated support matrix"
    );
    assert!(
        doc.contains(&generated_fallback_table()),
        "committed fidelity doc should contain the generated fallback table"
    );
}

fn fidelity_doc_path() -> std::path::PathBuf {
    // `docs/` is shared across languages and lives at the repository root, while
    // the Rust crate now lives under `rust/`. Resolve the doc relative to the
    // first ancestor of the crate manifest that owns the shared `docs/grammar`.
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if dir.join("docs/grammar").is_dir() {
            return dir.join("docs/grammar/fidelity.md");
        }
        if !dir.pop() {
            return std::path::PathBuf::from("docs/grammar/fidelity.md");
        }
    }
}

fn generated_matrix() -> String {
    let mut output = String::from("| Construct | BNF |\n| --- | :---: |\n");
    let profile = grammar_format_profile("bnf").expect("BNF profile");

    for &construct in GRAMMAR_CONSTRUCTS {
        let cell = match profile
            .construct_fidelity(construct)
            .expect("known construct")
        {
            GrammarFidelityLevel::Lossless => "✅",
            GrammarFidelityLevel::Equivalent => "≈",
            GrammarFidelityLevel::Lossy => "⚠️",
        };
        writeln!(&mut output, "| {construct} | {cell} |").expect("writing to a string cannot fail");
    }

    output
}

fn generated_fallback_table() -> String {
    let mut output = String::from("| Format | Construct | Fallback |\n| --- | --- | --- |\n");
    for &format in GRAMMAR_FORMATS {
        let profile = grammar_format_profile(format).expect("known format");
        for &construct in GRAMMAR_CONSTRUCTS {
            if let Some(fallback) = profile.construct_fallback(construct) {
                writeln!(
                    &mut output,
                    "| {} | {construct} | {fallback} |",
                    profile.format()
                )
                .expect("writing to a string cannot fail");
            }
        }
    }
    output
}
