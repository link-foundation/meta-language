use meta_language::{
    emit_bnf, grammar_concept_translation_rules, translate_grammar_surface, Grammar, GrammarFormat,
    GrammarRule,
};

#[test]
fn translated_grammar_round_trips_back_to_the_original_surface() {
    let grammar = arithmetic_grammar();
    let rules = grammar_concept_translation_rules();

    let russian =
        translate_grammar_surface(&grammar, "Russian", &rules).expect("grammar translates to ru");
    let english =
        translate_grammar_surface(&russian, "English", &rules).expect("grammar translates to en");

    assert_eq!(english, grammar);
}

#[test]
fn translated_grammar_can_still_be_consumed_by_emitters() {
    let grammar = arithmetic_grammar();
    let rules = grammar_concept_translation_rules();
    let russian =
        translate_grammar_surface(&grammar, "Russian", &rules).expect("grammar translates");

    let (bnf, report) = emit_bnf(&russian).expect("translated grammar emits as BNF");

    assert!(report.lossy.is_empty());
    assert!(bnf.contains("<выражение> ::= <слагаемое>"));
    assert!(bnf.contains("<множитель>"));
}

fn arithmetic_grammar() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .source_format(GrammarFormat::Peg)
        .start("expression")
        .grammar_rule(
            GrammarRule::new(
                "expression",
                expr.seq([expr.nt("term"), expr.rep0(expr.nt("term"))]),
            )
            .with_concept("grammar.expression")
            .with_doc("expression"),
        )
        .grammar_rule(GrammarRule::new("term", expr.nt("factor")).with_concept("grammar.term"))
        .grammar_rule(GrammarRule::new("factor", expr.term("n")).with_concept("grammar.factor"))
        .build()
}
