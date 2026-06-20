use meta_language::{
    evaluate_constraint, mine_semantic_constraints, Grammar, SemanticInferenceConfig, TruthValue,
};

#[test]
fn semantic_fixtures_mine_and_check_constraints() {
    let cases = [
        (
            def_use_grammar(),
            fixture_lines(include_str!(
                "../fixtures/grammar/semantic/def-before-use.txt"
            )),
            "def alpha; use beta;",
        ),
        (
            length_field_grammar(),
            fixture_lines(include_str!(
                "../fixtures/grammar/semantic/length-field.txt"
            )),
            "len=4:abc",
        ),
        (
            paired_delimiter_grammar(),
            fixture_lines(include_str!("../fixtures/grammar/semantic/equal-count.txt")),
            "(()",
        ),
    ];

    for (grammar, positives, invalid) in cases {
        let constraint =
            mine_semantic_constraints(&grammar, &positives, &SemanticInferenceConfig::default());

        assert_eq!(constraint.recall, meta_language::Probability::ONE);
        assert!(positives.iter().all(|positive| {
            evaluate_constraint(&grammar, positive, &constraint) == TruthValue::True
        }));
        assert_eq!(
            evaluate_constraint(&grammar, invalid, &constraint),
            TruthValue::False
        );
    }
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

fn fixture_lines(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect()
}
