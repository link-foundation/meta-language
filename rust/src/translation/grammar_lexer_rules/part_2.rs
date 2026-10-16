// Generated from the complete shared grammar-lexer-rules.js and recorded source closure.
// Do not edit by hand.
use super::*;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_lexer_command_plan(
    command: self::ml_string::Text,
    name: self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    {
        let body = command.clone();
        {
            let body_3 = if body.starts_with(&self::ml_string::Text::from_units(vec![45, 62])) {
                {
                    let body_2 = body.slice(2f64, f64::INFINITY);
                    body_2
                }
            } else {
                body.clone()
            };
            {
                let type_ = name.clone();
                {
                    let channel =
                        self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]);
                    {
                        let disposition =
                            self::ml_string::Text::from_units(vec![101, 109, 105, 116]);
                        {
                            let actions = Vec::<self::ml_string::Text>::new();
                            {
                                let pieces = body_3.split(
                                    Some(&self::ml_string::Text::from_units(vec![44])),
                                    4_294_967_295f64,
                                );
                                {
                                    let index = 0f64;
                                    {
                                        let ml_s8 = self::ml_read_grammar_lexer_command_plan_loop11(
                                            type_,
                                            channel,
                                            disposition,
                                            actions,
                                            &(pieces),
                                            index,
                                        );
                                        match ml_s8 {
                                            self::MlReadGrammarLexerCommandPlanLoop11Result::MlReadGrammarLexerCommandPlanLoop11Done(type_2, channel_2, disposition_2, actions_2) => {
                                                self::ml_array::append(vec![type_2.clone(), channel_2.clone(), disposition_2.clone()], actions_2)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_lexer_mode(
    candidate: self::ml_string::Text,
    active: self::ml_string::Text,
) -> bool {
    candidate == active
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_lexer_trivia(
    channel: self::ml_string::Text,
    disposition: self::ml_string::Text,
) -> bool {
    (disposition == (self::ml_string::Text::from_units(vec![115, 107, 105, 112])))
        || (channel != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_lexer_trivia_identity(
    source: self::ml_string::Text,
    emitted: self::ml_string::Text,
    disposition: self::ml_string::Text,
) -> self::ml_string::Text {
    if disposition == (self::ml_string::Text::from_units(vec![115, 107, 105, 112])) {
        source.clone()
    } else {
        emitted.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_lexer_expected_token(
    expected: self::ml_string::Text,
    actual: self::ml_string::Text,
    wildcard: bool,
) -> bool {
    wildcard || (expected == actual)
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_grammar_lexer_declaration_issue(
    kinds: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    items: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    modes: &[self::ml_string::Text],
    channels: &[self::ml_string::Text],
    dispositions: &[self::ml_string::Text],
    action_owners: &[f64],
    actions: &[self::ml_string::Text],
    action_modes: &[self::ml_string::Text],
) -> Vec<f64> {
    {
        let count = kinds.len() as f64;
        if ((((((((names.len() as f64) != count) || ((items.len() as f64) != count))
            || ((values.len() as f64) != count))
            || ((modes.len() as f64) != count))
            || ((channels.len() as f64) != count))
            || ((dispositions.len() as f64) != count))
            || ((action_owners.len() as f64) != (actions.len() as f64)))
            || ((action_modes.len() as f64) != (actions.len() as f64))
        {
            vec![1f64, -1f64, -1f64]
        } else {
            {
                let binding_names = Vec::<self::ml_string::Text>::new();
                {
                    let owner_names = Vec::<self::ml_string::Text>::new();
                    {
                        let literal_values = Vec::<self::ml_string::Text>::new();
                        {
                            let local_names = Vec::<self::ml_string::Text>::new();
                            {
                                let declared_modes = vec![self::ml_string::Text::from_units(vec![
                                    100, 101, 102, 97, 117, 108, 116,
                                ])];
                                {
                                    let declared_channels = vec![
                                        self::ml_string::Text::from_units(vec![
                                            100, 101, 102, 97, 117, 108, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            72, 73, 68, 68, 69, 78,
                                        ]),
                                    ];
                                    {
                                        let index = 0f64;
                                        {
                                            let ml_s9 = self::ml_read_grammar_lexer_declaration_issue_loop15(kinds, names, items, values, count, binding_names, owner_names, literal_values, local_names, declared_modes, declared_channels, index);
                                            match ml_s9 {
                                                self::MlReadGrammarLexerDeclarationIssueLoop15Result::MlReadGrammarLexerDeclarationIssueLoop15Done(declared_modes_2, declared_channels_2, index_2) => {
                                                    {
                                                        let index_3 = 0f64;
                                                        {
                                                            let ml_s10 = self::ml_read_grammar_lexer_declaration_issue_loop16(kinds, modes, channels, dispositions, action_owners, actions, action_modes, count, &(declared_modes_2), &(declared_channels_2), index_3);
                                                            match ml_s10 {
                                                                self::MlReadGrammarLexerDeclarationIssueLoop16Result::MlReadGrammarLexerDeclarationIssueLoop16Done => {
                                                                    vec![0f64, -1f64, -1f64]
                                                                }
                                                                self::MlReadGrammarLexerDeclarationIssueLoop16Result::MlReadGrammarLexerDeclarationIssueLoop16Return(ml_result16) => {
                                                                    ml_result16
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlReadGrammarLexerDeclarationIssueLoop15Result::MlReadGrammarLexerDeclarationIssueLoop15Return(ml_result19) => {
                                                    ml_result19
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_lexer_obligation_positions(
    count: f64,
    root: f64,
    edge_sources: &[f64],
    edge_targets: &[f64],
    owners: &[f64],
    categories: &[self::ml_string::Text],
) -> Vec<f64> {
    if (((((!(self::ml_math::is_safe_integer(count)) || (count < 0f64))
        || !(self::ml_math::is_safe_integer(root)))
        || (root < 0f64))
        || (root >= count))
        || ((edge_sources.len() as f64) != (edge_targets.len() as f64)))
        || ((owners.len() as f64) != (categories.len() as f64))
    {
        vec![-1f64]
    } else {
        {
            let reached = Vec::<f64>::new();
            {
                let index = 0f64;
                {
                    let pending = vec![root];
                    {
                        let cursor = 0f64;
                        {
                            let ml_s11 = self::ml_read_grammar_lexer_obligation_positions_loop24(
                                count,
                                edge_sources,
                                edge_targets,
                                reached,
                                index,
                                pending,
                                cursor,
                            );
                            match ml_s11 {
                                self::MlReadGrammarLexerObligationPositionsLoop24Result::MlReadGrammarLexerObligationPositionsLoop24Done(reached_2, index_2) => {
                                    {
                                        let selected = Vec::<f64>::new();
                                        {
                                            let index_3 = 0f64;
                                            {
                                                self::ml_read_grammar_lexer_obligation_positions_loop25(count, root, owners, categories, &(reached_2), index_3, selected)
                                            }
                                        }
                                    }
                                }
                                self::MlReadGrammarLexerObligationPositionsLoop24Result::MlReadGrammarLexerObligationPositionsLoop24Return(ml_result25) => {
                                    ml_result25
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_lexer_lazy_normalization(
    kinds: &[self::ml_string::Text],
    labels: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    inner: &[f64],
    edge_sources: &[f64],
    edge_targets: &[f64],
) -> Vec<f64> {
    {
        let count = kinds.len() as f64;
        if ((((labels.len() as f64) != count) || ((texts.len() as f64) != count))
            || ((inner.len() as f64) != count))
            || ((edge_sources.len() as f64) != (edge_targets.len() as f64))
        {
            vec![-1f64]
        } else {
            {
                let captures = Vec::<f64>::new();
                {
                    let index = 0f64;
                    {
                        self::ml_read_grammar_lexer_lazy_normalization_loop28(
                            kinds,
                            labels,
                            texts,
                            inner,
                            edge_sources,
                            edge_targets,
                            count,
                            captures,
                            index,
                        )
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptGrammarLexerIdentifierLoop1Result {
    MlAcceptGrammarLexerIdentifierLoop1Done,
    MlAcceptGrammarLexerIdentifierLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_grammar_lexer_identifier_loop1(
    mut text: self::ml_string::Text,
    mut index: f64,
) -> self::MlAcceptGrammarLexerIdentifierLoop1Result {
    loop {
        return if index < (text.length()) {
            {
                let character = text.slice(index, (index + 1f64));
                {
                    let letter = (self::ml_string::Text::from_units(vec![
                        65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
                        84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106,
                        107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121,
                        122, 95,
                    ]))
                    .contains(&character);
                    if letter
                        && !((index > 0f64)
                            && (self::ml_string::Text::from_units(vec![
                                48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                            ]))
                            .contains(&character))
                    {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlAcceptGrammarLexerIdentifierLoop1Result::MlAcceptGrammarLexerIdentifierLoop1Return(false)
                    }
                }
            }
        } else {
            self::MlAcceptGrammarLexerIdentifierLoop1Result::MlAcceptGrammarLexerIdentifierLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptGrammarLexerDecimalLoop2Result {
    MlAcceptGrammarLexerDecimalLoop2Done,
    MlAcceptGrammarLexerDecimalLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_grammar_lexer_decimal_loop2(
    mut text: self::ml_string::Text,
    mut index: f64,
) -> self::MlAcceptGrammarLexerDecimalLoop2Result {
    loop {
        return if index < (text.length()) {
            {
                let character = text.slice(index, (index + 1f64));
                if ((self::ml_string::Text::from_units(vec![
                    48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                ]))
                .contains(&character))
                {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlAcceptGrammarLexerDecimalLoop2Result::MlAcceptGrammarLexerDecimalLoop2Return(false)
                }
            }
        } else {
            self::MlAcceptGrammarLexerDecimalLoop2Result::MlAcceptGrammarLexerDecimalLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlContainsGrammarLexerTextLoop3Result {
    MlContainsGrammarLexerTextLoop3Done,
    MlContainsGrammarLexerTextLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_contains_grammar_lexer_text_loop3(
    values: &[self::ml_string::Text],
    mut value: self::ml_string::Text,
    mut index: f64,
) -> self::MlContainsGrammarLexerTextLoop3Result {
    loop {
        return if index < (values.len() as f64) {
            if (self::read_required_text_fact(values, index)) == value {
                self::MlContainsGrammarLexerTextLoop3Result::MlContainsGrammarLexerTextLoop3Return(
                    true,
                )
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlContainsGrammarLexerTextLoop3Result::MlContainsGrammarLexerTextLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlContainsGrammarLexerPositionLoop4Result {
    MlContainsGrammarLexerPositionLoop4Done,
    MlContainsGrammarLexerPositionLoop4Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_contains_grammar_lexer_position_loop4(
    values: &[f64],
    mut value: f64,
    mut index: f64,
) -> self::MlContainsGrammarLexerPositionLoop4Result {
    loop {
        return if index < (values.len() as f64) {
            if (self::read_required_number_fact(values, index)) == value {
                self::MlContainsGrammarLexerPositionLoop4Result::MlContainsGrammarLexerPositionLoop4Return(true)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlContainsGrammarLexerPositionLoop4Result::MlContainsGrammarLexerPositionLoop4Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_grammar_lexer_section_rule_positions_loop6(
    mut section: f64,
    section_kinds: &[self::ml_string::Text],
    section_names: &[self::ml_string::Text],
    token_vocabularies: &[self::ml_string::Text],
    rule_sections: &[f64],
    rule_kinds: &[self::ml_string::Text],
    mut selected: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (rule_sections.len() as f64) {
            {
                let owner_section = self::read_required_number_fact(rule_sections, index);
                {
                    let selected_3 = if ((self::read_required_text_fact(rule_kinds, index))
                        == (self::ml_string::Text::from_units(vec![110, 111, 114, 109, 97, 108])))
                        && ((owner_section == section)
                            || (((owner_section >= 0f64)
                                && ((self::read_required_text_fact(
                                    section_kinds,
                                    owner_section,
                                )) == (self::ml_string::Text::from_units(vec![
                                    112, 97, 114, 115, 101, 114,
                                ]))))
                                && ((self::read_required_text_fact(
                                    token_vocabularies,
                                    owner_section,
                                )) == (self::read_required_text_fact(section_names, section)))))
                    {
                        {
                            {
                                let (ml_append_right, ml_append_left) = (vec![index], selected);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            }
                        }
                    } else {
                        selected
                    };
                    {
                        let index_2 = index + 1f64;
                        {
                            (selected, index) = (selected_3, index_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { selected };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerSectionRulePositionsLoop5Result {
    MlReadGrammarLexerSectionRulePositionsLoop5Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_section_rule_positions_loop5(
    mut section: f64,
    rule_sections: &[f64],
    rule_kinds: &[self::ml_string::Text],
    mut selected: Vec<f64>,
    mut index: f64,
) -> self::MlReadGrammarLexerSectionRulePositionsLoop5Result {
    loop {
        if index < (rule_sections.len() as f64) {
            {
                let kind = self::read_required_text_fact(rule_kinds, index);
                {
                    let selected_3 = if ((self::read_required_number_fact(rule_sections, index))
                        == section)
                        && ((kind
                            == (self::ml_string::Text::from_units(vec![116, 111, 107, 101, 110])))
                            || (kind
                                == (self::ml_string::Text::from_units(vec![
                                    115, 105, 108, 101, 110, 116,
                                ])))) {
                        {
                            {
                                let (ml_append_right, ml_append_left) = (vec![index], selected);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            }
                        }
                    } else {
                        selected
                    };
                    {
                        let index_2 = index + 1f64;
                        {
                            (selected, index) = (selected_3, index_2);
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlReadGrammarLexerSectionRulePositionsLoop5Result::MlReadGrammarLexerSectionRulePositionsLoop5Done(selected, index)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerLiteralRulePositionLoop7Result {
    MlReadGrammarLexerLiteralRulePositionLoop7Done(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerLiteralRulePositionJoin8 {
    MlReadGrammarLexerLiteralRulePositionJoin8Next(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_literal_rule_position_loop7(
    mut value: self::ml_string::Text,
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    literals: &[bool],
    mut selected: f64,
    mut count: f64,
    mut index: f64,
) -> self::MlReadGrammarLexerLiteralRulePositionLoop7Result {
    loop {
        return if index < (kinds.len() as f64) {
            {
                let ml_s12 = if (((self::read_required_text_fact(kinds, index))
                    == (self::ml_string::Text::from_units(vec![116, 111, 107, 101, 110])))
                    && self::read_required_boolean_fact(literals, index))
                    && ((self::read_required_text_fact(values, index)) == value)
                {
                    {
                        let selected_2 = index;
                        {
                            let count_2 = count + 1f64;
                            self::MlReadGrammarLexerLiteralRulePositionJoin8::MlReadGrammarLexerLiteralRulePositionJoin8Next(selected_2, count_2)
                        }
                    }
                } else {
                    self::MlReadGrammarLexerLiteralRulePositionJoin8::MlReadGrammarLexerLiteralRulePositionJoin8Next(selected, count)
                };
                match ml_s12 {
                    self::MlReadGrammarLexerLiteralRulePositionJoin8::MlReadGrammarLexerLiteralRulePositionJoin8Next(selected_3, count_3) => {
                        {
                            let index_2 = index + 1f64;
                            {
                                (selected, count, index) = (selected_3, count_3, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerLiteralRulePositionLoop7Result::MlReadGrammarLexerLiteralRulePositionLoop7Done(selected, count)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_binding_names_loop10(
    names: &[self::ml_string::Text],
    legacy_types: &[self::ml_string::Text],
    emitted_types: &[self::ml_string::Text],
    dispositions: &[self::ml_string::Text],
    mut bindings: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (names.len() as f64) {
            {
                let name = self::read_required_text_fact(names, index);
                {
                    let legacy = self::read_required_text_fact(legacy_types, index);
                    {
                        let emitted = self::read_required_text_fact(emitted_types, index);
                        {
                            let bindings_3 =
                                if (self::contains_grammar_lexer_text(&(bindings), name.clone())) {
                                    bindings
                                } else {
                                    {
                                        {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![name.clone()], bindings);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        }
                                    }
                                };
                            {
                                let bindings_5 = if (legacy
                                    != (self::ml_string::Text::from_units(vec![])))
                                    && !(self::contains_grammar_lexer_text(
                                        &(bindings_3),
                                        legacy.clone(),
                                    )) {
                                    {
                                        {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![legacy.clone()], bindings_3);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        }
                                    }
                                } else {
                                    bindings_3
                                };
                                {
                                    let bindings_7 =
                                        if ((self::read_required_text_fact(dispositions, index))
                                            == (self::ml_string::Text::from_units(vec![
                                                101, 109, 105, 116,
                                            ])))
                                            && !(self::contains_grammar_lexer_text(
                                                &(bindings_5),
                                                emitted.clone(),
                                            ))
                                        {
                                            {
                                                {
                                                    let (ml_append_right, ml_append_left) =
                                                        (vec![emitted.clone()], bindings_5);
                                                    self::ml_array::append(
                                                        ml_append_left,
                                                        ml_append_right,
                                                    )
                                                }
                                            }
                                        } else {
                                            bindings_5
                                        };
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (bindings, index) = (bindings_7, index_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { bindings };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerBindingNamesLoop9Result {
    MlReadGrammarLexerBindingNamesLoop9Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_binding_names_loop9(
    declared: &[self::ml_string::Text],
    references: &[self::ml_string::Text],
    mut bindings: Vec<self::ml_string::Text>,
    mut index: f64,
) -> self::MlReadGrammarLexerBindingNamesLoop9Result {
    loop {
        if index < (declared.len() as f64) {
            {
                let name = self::read_required_text_fact(declared, index);
                {
                    let bindings_3 = if self::contains_grammar_lexer_text(references, name.clone())
                        && !(self::contains_grammar_lexer_text(&(bindings), name.clone()))
                    {
                        {
                            {
                                let (ml_append_right, ml_append_left) =
                                    (vec![name.clone()], bindings);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            }
                        }
                    } else {
                        bindings
                    };
                    {
                        let index_2 = index + 1f64;
                        {
                            (bindings, index) = (bindings_3, index_2);
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlReadGrammarLexerBindingNamesLoop9Result::MlReadGrammarLexerBindingNamesLoop9Done(
                bindings, index,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarLexerCommandPlanLoop11Result {
    MlReadGrammarLexerCommandPlanLoop11Done(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        Vec<self::ml_string::Text>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarLexerCommandPlanJoin13 {
    MlReadGrammarLexerCommandPlanJoin13Next(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        Vec<self::ml_string::Text>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_command_plan_loop14(
    mut argument: self::ml_string::Text,
    mut zero: bool,
    mut offset: f64,
) -> bool {
    loop {
        if offset < (argument.length()) {
            {
                let zero_3 = if (argument.slice(offset, (offset + 1f64)))
                    != (self::ml_string::Text::from_units(vec![48]))
                {
                    { false }
                } else {
                    zero
                };
                {
                    let offset_2 = offset + 1f64;
                    {
                        (zero, offset) = (zero_3, offset_2);
                        continue;
                    }
                }
            }
        }
        return { zero };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarLexerCommandPlanJoin12 {
    MlReadGrammarLexerCommandPlanJoin12Next(self::ml_string::Text, self::ml_string::Text, bool),
}
