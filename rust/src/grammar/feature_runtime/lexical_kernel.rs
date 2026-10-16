//! Primitive column transport to the complete self-translated lexical policy.
//! These conversions contain no lexical decisions. Generated ABI changes are
//! confined here; the JavaScript source and its Links receipt own the policy.

use crate::translation::grammar_lexer_rules as rules;

fn text(value: &str) -> rules::ml_string::Text {
    rules::ml_string::Text::from_text(value)
}

fn texts(values: &[String]) -> Vec<rules::ml_string::Text> {
    values.iter().map(|value| text(value)).collect()
}

pub fn number(value: usize) -> f64 {
    serde_json::Number::from(value)
        .as_f64()
        .expect("a host index has a JavaScript Number representation")
}

pub fn position(value: f64) -> usize {
    rules::ml_array::number_index(value)
        .expect("a generated position is a non-negative exact array index")
}

pub fn entry_form(kind: &str) -> usize {
    position(rules::read_grammar_lexer_entry_form(text(kind)))
}

pub fn command_plan(command: &str, name: &str) -> Vec<String> {
    rules::read_grammar_lexer_command_plan(text(command), text(name))
        .iter()
        .map(ToString::to_string)
        .collect()
}

pub fn section_positions(
    section: usize,
    kinds: &[String],
    names: &[String],
    vocabularies: &[String],
    rule_sections: &[f64],
    rule_kinds: &[String],
) -> Vec<f64> {
    rules::read_grammar_lexer_section_rule_positions(
        number(section),
        &texts(kinds),
        &texts(names),
        &texts(vocabularies),
        rule_sections,
        &texts(rule_kinds),
    )
}

pub fn literal_rule(
    value: &str,
    section_kind: &str,
    kinds: &[String],
    values: &[String],
    literals: &[bool],
) -> f64 {
    rules::read_grammar_lexer_literal_rule_position(
        text(value),
        text(section_kind),
        &texts(kinds),
        &texts(values),
        literals,
    )
}

pub fn literal_extension(
    owner: &str,
    value: &str,
    columns: &[Vec<String>; 6],
    parser_literals: &[String],
) -> Vec<f64> {
    rules::read_grammar_lexer_literal_extension(
        text(owner),
        text(value),
        &texts(&columns[0]),
        &texts(&columns[1]),
        &texts(&columns[2]),
        &texts(&columns[3]),
        &texts(&columns[4]),
        &texts(&columns[5]),
        &texts(parser_literals),
    )
}

pub fn owner_comparison(kinds: &[String], names: &[String], targets: &[String]) -> Vec<f64> {
    rules::read_grammar_lexer_owner_comparison(&texts(kinds), &texts(names), &texts(targets))
}

pub fn binding_names(
    declared: &[String],
    references: &[String],
    names: &[String],
    legacy: &[String],
    emitted: &[String],
    dispositions: &[String],
) -> Vec<String> {
    rules::read_grammar_lexer_binding_names(
        &texts(declared),
        &texts(references),
        &texts(names),
        &texts(legacy),
        &texts(emitted),
        &texts(dispositions),
    )
    .iter()
    .map(ToString::to_string)
    .collect()
}

pub(super) fn declaration_issue(
    columns: &[Vec<String>; 7],
    owners: &[f64],
    actions: &[String],
    modes: &[String],
) -> Vec<f64> {
    rules::read_grammar_lexer_declaration_issue(
        &texts(&columns[0]),
        &texts(&columns[1]),
        &texts(&columns[2]),
        &texts(&columns[3]),
        &texts(&columns[4]),
        &texts(&columns[5]),
        &texts(&columns[6]),
        owners,
        &texts(actions),
        &texts(modes),
    )
}

pub(super) fn normalization(
    kinds: &[String],
    labels: &[String],
    values: &[String],
    inner: &[f64],
    sources: &[f64],
    targets: &[f64],
) -> Vec<f64> {
    rules::read_grammar_lexer_lazy_normalization(
        &texts(kinds),
        &texts(labels),
        &texts(values),
        inner,
        sources,
        targets,
    )
}

pub(super) fn obligations(
    count: usize,
    root: usize,
    sources: &[f64],
    targets: &[f64],
    owners: &[f64],
    categories: &[String],
) -> Vec<f64> {
    rules::read_grammar_lexer_obligation_positions(
        number(count),
        number(root),
        sources,
        targets,
        owners,
        &texts(categories),
    )
}

pub(super) fn active_mode(candidate: &str, active: &str) -> bool {
    rules::accept_grammar_lexer_mode(text(candidate), text(active))
}

pub(super) fn trivia(channel: &str, disposition: &str) -> bool {
    rules::accept_grammar_lexer_trivia(text(channel), text(disposition))
}

pub(super) fn trivia_identity(source: &str, emitted: &str, disposition: &str) -> String {
    rules::read_grammar_lexer_trivia_identity(text(source), text(emitted), text(disposition))
        .to_string()
}

pub(super) fn expected_token(expected: &str, actual: &str, wildcard: bool) -> bool {
    rules::accept_grammar_lexer_expected_token(text(expected), text(actual), wildcard)
}

pub(super) fn winner(ends: &[f64], eligible: &[bool]) -> f64 {
    crate::translation::frontend_rules::read_lexical_winner_index(ends, eligible)
}
