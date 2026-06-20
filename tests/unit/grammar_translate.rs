use std::collections::BTreeMap;

use meta_language::{
    grammar_concept_translation_rules, translate_grammar_surface, CharClassItem, Grammar,
    GrammarExpr, GrammarFormat, GrammarRule, GrammarTranslateError, LinkQuery, LinkType,
    TranslationRule, TranslationRuleSet,
};

#[test]
fn translates_concept_aligned_rule_names_and_nonterminal_references() {
    let grammar = arithmetic_grammar();
    let rules = grammar_concept_translation_rules();

    let translated =
        translate_grammar_surface(&grammar, "Russian", &rules).expect("grammar translates");

    assert_eq!(
        translated.rule_names(),
        ["выражение", "слагаемое", "множитель"]
    );
    assert_eq!(translated.start(), Some("выражение"));
    assert_eq!(translated.source_format(), Some(GrammarFormat::Peg));
    assert_eq!(
        translated.rule("выражение").and_then(GrammarRule::doc),
        Some("выражение")
    );
    assert!(translated.undefined_nonterminals().is_empty());
    assert_same_structure_after_rename(&grammar, &translated);
}

#[test]
fn leaves_unaligned_rules_without_known_surface_concepts_unchanged() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("expression")
        .grammar_rule(
            GrammarRule::new(
                "expression",
                expr.seq([expr.nt("spacing"), expr.term("value")]),
            )
            .with_concept("grammar.expression"),
        )
        .rule("spacing", expr.rep0(expr.char(' ')))
        .build();

    let translated =
        translate_grammar_surface(&grammar, "Russian", &grammar_concept_translation_rules())
            .expect("grammar translates with deterministic fallback");

    assert_eq!(translated.rule_names(), ["выражение", "spacing"]);
    assert!(translated.undefined_nonterminals().is_empty());
    assert_same_structure_after_rename(&grammar, &translated);
}

#[test]
fn infers_concepts_from_known_rule_name_surfaces() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("expression")
        .rule("expression", expr.nt("term"))
        .rule("term", expr.term("n"))
        .build();

    let translated =
        translate_grammar_surface(&grammar, "Russian", &grammar_concept_translation_rules())
            .expect("known rule surfaces translate");

    assert_eq!(translated.rule_names(), ["выражение", "слагаемое"]);
    assert_eq!(translated.start(), Some("выражение"));
    assert!(translated.undefined_nonterminals().is_empty());
    assert_same_structure_after_rename(&grammar, &translated);
}

#[test]
fn reports_name_collisions_instead_of_overwriting_rules() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .grammar_rule(
            GrammarRule::new("left", expr.term("l")).with_concept("grammar.collision.left"),
        )
        .grammar_rule(
            GrammarRule::new("right", expr.term("r")).with_concept("grammar.collision.right"),
        )
        .build();
    let rules = TranslationRuleSet::new("collision")
        .with_rule(concept_rule("grammar.collision.left", "left", "узел"))
        .with_rule(concept_rule("grammar.collision.right", "right", "узел"));

    assert_eq!(
        translate_grammar_surface(&grammar, "Russian", &rules),
        Err(GrammarTranslateError::NameCollision {
            language: "Russian".to_string(),
            name: "узел".to_string()
        })
    );
}

#[test]
fn reports_explicit_concepts_missing_from_the_rule_set() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .grammar_rule(
            GrammarRule::new("mystery", expr.term("?")).with_concept("grammar.missing-concept"),
        )
        .build();

    assert_eq!(
        translate_grammar_surface(&grammar, "Russian", &grammar_concept_translation_rules(),),
        Err(GrammarTranslateError::UnknownConcept {
            rule: "mystery".to_string(),
            concept: "grammar.missing-concept".to_string()
        })
    );
}

#[test]
fn translates_doc_comments_through_existing_reconstruction_rules() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .grammar_rule(
            GrammarRule::new("expression", expr.term("state"))
                .with_concept("grammar.expression")
                .with_doc("Hawaii is a state.\n"),
        )
        .build();
    let mut rules = grammar_concept_translation_rules();
    for rule in TranslationRuleSet::statehood_demo().rules() {
        rules.add_rule(rule.clone());
    }

    let translated =
        translate_grammar_surface(&grammar, "Russian", &rules).expect("grammar translates");

    assert_eq!(
        translated.rule("выражение").and_then(GrammarRule::doc),
        Some("Гавайи это штат.\n")
    );
}

#[test]
fn default_translation_rules_round_trip_through_lino() {
    let rules = grammar_concept_translation_rules();
    let restored = TranslationRuleSet::from_lino(&rules.to_lino())
        .expect("grammar rule set should round-trip through LiNo");

    assert_eq!(restored, rules);
}

fn arithmetic_grammar() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .source_format(GrammarFormat::Peg)
        .start("expression")
        .grammar_rule(
            GrammarRule::new(
                "expression",
                expr.seq([
                    expr.nt("term"),
                    expr.rep0(expr.seq([
                        expr.choice_ordered([expr.term("+"), expr.term("-")]),
                        expr.nt("term"),
                    ])),
                ]),
            )
            .with_concept("grammar.expression")
            .with_doc("expression"),
        )
        .grammar_rule(
            GrammarRule::new(
                "term",
                expr.seq([
                    expr.nt("factor"),
                    expr.rep0(expr.seq([
                        expr.choice_ordered([expr.term("*"), expr.term("/")]),
                        expr.nt("factor"),
                    ])),
                ]),
            )
            .with_concept("grammar.term"),
        )
        .grammar_rule(
            GrammarRule::new(
                "factor",
                expr.choice_ordered([
                    expr.rep1(expr.char_class(false, [CharClassItem::range('0', '9')])),
                    expr.seq([expr.term("("), expr.nt("expression"), expr.term(")")]),
                ]),
            )
            .with_concept("grammar.factor"),
        )
        .build()
}

fn concept_rule(concept: &str, english: &str, russian: &str) -> TranslationRule {
    TranslationRule::new(
        concept,
        LinkQuery::by_type(LinkType::Concept).with_term(concept),
    )
    .with_template("English", english)
    .with_template("Russian", russian)
}

fn assert_same_structure_after_rename(source: &Grammar, translated: &Grammar) {
    assert_eq!(source.rules().len(), translated.rules().len());

    let translated_to_source_names = source
        .rules()
        .iter()
        .zip(translated.rules())
        .map(|(source_rule, translated_rule)| {
            (
                translated_rule.name().to_string(),
                source_rule.name().to_string(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    for (source_rule, translated_rule) in source.rules().iter().zip(translated.rules()) {
        assert_eq!(source_rule.kind(), translated_rule.kind());
        assert_eq!(source_rule.concept(), translated_rule.concept());
        assert_eq!(
            source_rule.expr(),
            &rename_nonterminals(translated_rule.expr(), &translated_to_source_names)
        );
    }
}

fn rename_nonterminals(expr: &GrammarExpr, names: &BTreeMap<String, String>) -> GrammarExpr {
    match expr {
        GrammarExpr::NonTerminal(name) => {
            GrammarExpr::NonTerminal(names.get(name).cloned().unwrap_or_else(|| name.clone()))
        }
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => GrammarExpr::Choice {
            ordered: *ordered,
            alternatives: alternatives
                .iter()
                .map(|expr| rename_nonterminals(expr, names))
                .collect(),
        },
        GrammarExpr::Sequence(items) => GrammarExpr::Sequence(
            items
                .iter()
                .map(|expr| rename_nonterminals(expr, names))
                .collect(),
        ),
        GrammarExpr::Optional(expr) => {
            GrammarExpr::Optional(Box::new(rename_nonterminals(expr, names)))
        }
        GrammarExpr::ZeroOrMore(expr) => {
            GrammarExpr::ZeroOrMore(Box::new(rename_nonterminals(expr, names)))
        }
        GrammarExpr::OneOrMore(expr) => {
            GrammarExpr::OneOrMore(Box::new(rename_nonterminals(expr, names)))
        }
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::Repeat {
            expr: Box::new(rename_nonterminals(expr, names)),
            min: *min,
            max: *max,
        },
        GrammarExpr::And(expr) => GrammarExpr::And(Box::new(rename_nonterminals(expr, names))),
        GrammarExpr::Not(expr) => GrammarExpr::Not(Box::new(rename_nonterminals(expr, names))),
        GrammarExpr::Capture { label, expr } => GrammarExpr::Capture {
            label: label.clone(),
            expr: Box::new(rename_nonterminals(expr, names)),
        },
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => expr.clone(),
    }
}
