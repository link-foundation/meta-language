// Generated from the complete shared grammar-lexer-rules.js and recorded source closure.
// Do not edit by hand.
use super::MlAcceptGrammarLexerIdentifierLoop1Result;
use super::contains_grammar_lexer_text;
use super::ml_array;
use super::ml_string;
use super::read_grammar_lexer_entry_form;
use super::read_grammar_lexer_text_position;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_grammar_lexer_identifier_loop1(
    text: &self::ml_string::Text,
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
                    if ((letter)
                        || ((index > 0f64)
                            && (self::ml_string::Text::from_units(vec![
                                48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
                            ]))
                            .contains(&character)))
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
    text: &self::ml_string::Text,
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
    value: &self::ml_string::Text,
    mut index: f64,
) -> self::MlContainsGrammarLexerTextLoop3Result {
    loop {
        return if index < (values.len() as f64) {
            if (self::read_required_text_fact(values, index)) == (*value) {
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
                        && (((owner_section).partial_cmp(&(section))
                            == Some(std::cmp::Ordering::Equal))
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
    value: &self::ml_string::Text,
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
                let ml_s29 = if (((self::read_required_text_fact(kinds, index))
                    == (self::ml_string::Text::from_units(vec![116, 111, 107, 101, 110])))
                    && self::read_required_boolean_fact(literals, index))
                    && ((self::read_required_text_fact(values, index)) == (*value))
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
                match ml_s29 {
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerTextPositionLoop9Result {
    MlReadGrammarLexerTextPositionLoop9Done,
    MlReadGrammarLexerTextPositionLoop9Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_text_position_loop9(
    values: &[self::ml_string::Text],
    value: &self::ml_string::Text,
    mut index: f64,
) -> self::MlReadGrammarLexerTextPositionLoop9Result {
    loop {
        return if index < (values.len() as f64) {
            if (self::read_required_text_fact(values, index)) == (*value) {
                self::MlReadGrammarLexerTextPositionLoop9Result::MlReadGrammarLexerTextPositionLoop9Return(index)
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
            self::MlReadGrammarLexerTextPositionLoop9Result::MlReadGrammarLexerTextPositionLoop9Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_literal_extension_loop14(
    mut index: f64,
    renames: &[f64],
    mut result: Vec<f64>,
) -> Vec<f64> {
    loop {
        if index < (renames.len() as f64) {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::read_required_number_fact(renames, index)],
                        result,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (index, result) = (index_2, result_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerLiteralExtensionLoop13Result {
    MlReadGrammarLexerLiteralExtensionLoop13Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_literal_extension_loop13(
    kinds: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    mut index: f64,
    mut insertion: f64,
    mut preceding: f64,
) -> self::MlReadGrammarLexerLiteralExtensionLoop13Result {
    loop {
        if index < insertion {
            {
                let preceding_3 = if ((self::read_grammar_lexer_entry_form(
                    &(self::read_required_text_fact(kinds, index)),
                )) == 3f64)
                    && (self::read_required_text_fact(names, index)).starts_with(
                        &self::ml_string::Text::from_units(vec![
                            64, 108, 105, 116, 101, 114, 97, 108, 58,
                        ]),
                    ) {
                    { preceding + 1f64 }
                } else {
                    preceding
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (index, preceding) = (index_2, preceding_3);
                        continue;
                    }
                }
            }
        }
        return {
            self::MlReadGrammarLexerLiteralExtensionLoop13Result::MlReadGrammarLexerLiteralExtensionLoop13Done(index, preceding)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerLiteralExtensionLoop12Result {
    MlReadGrammarLexerLiteralExtensionLoop12Done(f64, Vec<f64>),
    MlReadGrammarLexerLiteralExtensionLoop12Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerLiteralExtensionJoin15 {
    MlReadGrammarLexerLiteralExtensionJoin15Next(f64, Vec<f64>),
    MlReadGrammarLexerLiteralExtensionJoin15Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerLiteralExtensionLoop16Result {
    MlReadGrammarLexerLiteralExtensionLoop16Done(Vec<f64>, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerLiteralExtensionJoin17 {
    MlReadGrammarLexerLiteralExtensionJoin17Next(Vec<f64>, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_grammar_lexer_literal_extension_loop16(
    kinds: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    mut count: f64,
    mut identity: f64,
    mut renames: Vec<f64>,
    name: &self::ml_string::Text,
    mut target: f64,
    mut found: bool,
) -> self::MlReadGrammarLexerLiteralExtensionLoop16Result {
    loop {
        return if target < count {
            {
                let ml_s30 = if ((self::read_grammar_lexer_entry_form(
                    &(self::read_required_text_fact(kinds, target)),
                )) == 7f64)
                    && ((self::read_required_text_fact(names, target)) == (*name))
                {
                    {
                        let renames_2 = {
                            let (ml_append_right, ml_append_left) =
                                (vec![target, (identity + 1f64)], renames);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let found_2 = true;
                            self::MlReadGrammarLexerLiteralExtensionJoin17::MlReadGrammarLexerLiteralExtensionJoin17Next(renames_2, found_2)
                        }
                    }
                } else {
                    self::MlReadGrammarLexerLiteralExtensionJoin17::MlReadGrammarLexerLiteralExtensionJoin17Next(renames, found)
                };
                match ml_s30 {
                    self::MlReadGrammarLexerLiteralExtensionJoin17::MlReadGrammarLexerLiteralExtensionJoin17Next(renames_3, found_3) => {
                        {
                            let target_2 = target + 1f64;
                            {
                                (renames, target, found) = (renames_3, target_2, found_3);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerLiteralExtensionLoop16Result::MlReadGrammarLexerLiteralExtensionLoop16Done(renames, found)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_literal_extension_loop12(
    kinds: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    mut count: f64,
    mut index: f64,
    mut insertion: f64,
    mut identity: f64,
    mut renames: Vec<f64>,
) -> self::MlReadGrammarLexerLiteralExtensionLoop12Result {
    loop {
        return if index < count {
            {
                let name = self::read_required_text_fact(names, index);
                {
                    let ml_s32 = if ((self::read_grammar_lexer_entry_form(
                        &(self::read_required_text_fact(kinds, index)),
                    )) == 3f64)
                        && name.starts_with(&self::ml_string::Text::from_units(vec![
                            64, 108, 105, 116, 101, 114, 97, 108, 58,
                        ])) {
                        if index < insertion {
                            {
                                let identity_2 = identity + 1f64;
                                self::MlReadGrammarLexerLiteralExtensionJoin15::MlReadGrammarLexerLiteralExtensionJoin15Next(identity_2, renames)
                            }
                        } else {
                            {
                                let renames_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![index, (identity + 1f64)], renames);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let target = 0f64;
                                    {
                                        let found = false;
                                        {
                                            let ml_s33 = self::ml_read_grammar_lexer_literal_extension_loop16(kinds, names, count, identity, renames_2, &(name), target, found);
                                            match ml_s33 {
                                                self::MlReadGrammarLexerLiteralExtensionLoop16Result::MlReadGrammarLexerLiteralExtensionLoop16Done(renames_3, found_2) => {
                                                    if found_2 {
                                                        {
                                                            let identity_3 = identity + 1f64;
                                                            self::MlReadGrammarLexerLiteralExtensionJoin15::MlReadGrammarLexerLiteralExtensionJoin15Next(identity_3, renames_3)
                                                        }
                                                    } else {
                                                        self::MlReadGrammarLexerLiteralExtensionJoin15::MlReadGrammarLexerLiteralExtensionJoin15Return(vec![-1f64])
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        self::MlReadGrammarLexerLiteralExtensionJoin15::MlReadGrammarLexerLiteralExtensionJoin15Next(identity, renames)
                    };
                    match ml_s32 {
                        self::MlReadGrammarLexerLiteralExtensionJoin15::MlReadGrammarLexerLiteralExtensionJoin15Next(identity_4, renames_4) => {
                            {
                                let index_2 = index + 1f64;
                                {
                                    (index, identity, renames) = (index_2, identity_4, renames_4);
                                    continue;
                                }
                            }
                        }
                        self::MlReadGrammarLexerLiteralExtensionJoin15::MlReadGrammarLexerLiteralExtensionJoin15Return(ml_result15) => {
                            self::MlReadGrammarLexerLiteralExtensionLoop12Result::MlReadGrammarLexerLiteralExtensionLoop12Return(ml_result15)
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerLiteralExtensionLoop12Result::MlReadGrammarLexerLiteralExtensionLoop12Done(index, renames)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerLiteralExtensionLoop11Result {
    MlReadGrammarLexerLiteralExtensionLoop11Done(f64, f64),
    MlReadGrammarLexerLiteralExtensionLoop11Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerLiteralExtensionJoin18 {
    MlReadGrammarLexerLiteralExtensionJoin18Next(f64),
    MlReadGrammarLexerLiteralExtensionJoin18Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_grammar_lexer_literal_extension_loop11(
    kinds: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    parser_literals: &[self::ml_string::Text],
    mut count: f64,
    mut index: f64,
    mut order: f64,
    mut insertion: f64,
) -> self::MlReadGrammarLexerLiteralExtensionLoop11Result {
    loop {
        return if index < count {
            {
                let form = self::read_grammar_lexer_entry_form(
                    &(self::read_required_text_fact(kinds, index)),
                );
                {
                    let ml_s34 = if form == 3f64 {
                        {
                            let position = self::read_grammar_lexer_text_position(
                                parser_literals,
                                &(self::read_required_text_fact(values, index)),
                            );
                            if position < 0f64 {
                                self::MlReadGrammarLexerLiteralExtensionJoin18::MlReadGrammarLexerLiteralExtensionJoin18Return(vec![-1f64])
                            } else if (position > order) && (insertion == count) {
                                {
                                    let insertion_2 = index;
                                    self::MlReadGrammarLexerLiteralExtensionJoin18::MlReadGrammarLexerLiteralExtensionJoin18Next(insertion_2)
                                }
                            } else {
                                self::MlReadGrammarLexerLiteralExtensionJoin18::MlReadGrammarLexerLiteralExtensionJoin18Next(insertion)
                            }
                        }
                    } else if ((form == 7f64)
                        && !((self::read_required_text_fact(names, index)).starts_with(
                            &self::ml_string::Text::from_units(vec![
                                64, 108, 105, 116, 101, 114, 97, 108, 58,
                            ]),
                        )))
                        && (insertion == count)
                    {
                        {
                            let insertion_3 = index;
                            self::MlReadGrammarLexerLiteralExtensionJoin18::MlReadGrammarLexerLiteralExtensionJoin18Next(insertion_3)
                        }
                    } else {
                        self::MlReadGrammarLexerLiteralExtensionJoin18::MlReadGrammarLexerLiteralExtensionJoin18Next(insertion)
                    };
                    match ml_s34 {
                        self::MlReadGrammarLexerLiteralExtensionJoin18::MlReadGrammarLexerLiteralExtensionJoin18Next(insertion_4) => {
                            {
                                let index_2 = index + 1f64;
                                {
                                    (index, insertion) = (index_2, insertion_4);
                                    continue;
                                }
                            }
                        }
                        self::MlReadGrammarLexerLiteralExtensionJoin18::MlReadGrammarLexerLiteralExtensionJoin18Return(ml_result18) => {
                            self::MlReadGrammarLexerLiteralExtensionLoop11Result::MlReadGrammarLexerLiteralExtensionLoop11Return(ml_result18)
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerLiteralExtensionLoop11Result::MlReadGrammarLexerLiteralExtensionLoop11Done(index, insertion)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerLiteralExtensionLoop10Result {
    MlReadGrammarLexerLiteralExtensionLoop10Done(bool, f64, Vec<self::ml_string::Text>, Vec<bool>),
    MlReadGrammarLexerLiteralExtensionLoop10Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_grammar_lexer_literal_extension_loop10(
    owner: &self::ml_string::Text,
    value: &self::ml_string::Text,
    kinds: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    targets: &[self::ml_string::Text],
    item_kinds: &[self::ml_string::Text],
    mut count: f64,
    mut matching_owner_present: bool,
    mut index: f64,
    mut candidates: Vec<self::ml_string::Text>,
    mut literals: Vec<bool>,
) -> self::MlReadGrammarLexerLiteralExtensionLoop10Result {
    loop {
        return if index < count {
            {
                let form = self::read_grammar_lexer_entry_form(
                    &(self::read_required_text_fact(kinds, index)),
                );
                {
                    let name = self::read_required_text_fact(names, index);
                    {
                        let matching_owner_present_3 = if (form == 1f64)
                            && ((self::read_required_text_fact(targets, index)) == (*owner))
                        {
                            { true }
                        } else {
                            matching_owner_present
                        };
                        if (form == 3f64)
                            && ((self::read_required_text_fact(values, index)) == (*value))
                        {
                            self::MlReadGrammarLexerLiteralExtensionLoop10Result::MlReadGrammarLexerLiteralExtensionLoop10Return(vec![0f64, -1f64, -1f64])
                        } else {
                            {
                                let candidates_2 = {
                                    let (ml_append_right, ml_append_left) = (
                                        vec![if (form == 7f64)
                                            && !(name.starts_with(
                                                &self::ml_string::Text::from_units(vec![
                                                    64, 108, 105, 116, 101, 114, 97, 108, 58,
                                                ]),
                                            )) {
                                            self::ml_string::Text::from_units(vec![
                                                116, 111, 107, 101, 110,
                                            ])
                                        } else {
                                            self::ml_string::Text::from_units(vec![
                                                115, 105, 108, 101, 110, 116,
                                            ])
                                        }],
                                        candidates,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let literals_2 = {
                                        let (ml_append_right, ml_append_left) = (
                                            vec![
                                                ((self::read_required_text_fact(
                                                    item_kinds, index,
                                                )) == (self::ml_string::Text::from_units(vec![
                                                    108, 105, 116, 101, 114, 97, 108,
                                                ]))),
                                            ],
                                            literals,
                                        );
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (matching_owner_present, index, candidates, literals) = (
                                                matching_owner_present_3,
                                                index_2,
                                                candidates_2,
                                                literals_2,
                                            );
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerLiteralExtensionLoop10Result::MlReadGrammarLexerLiteralExtensionLoop10Done(matching_owner_present, index, candidates, literals)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerOwnerComparisonLoop20Result {
    MlReadGrammarLexerOwnerComparisonLoop20Done(Vec<f64>),
    MlReadGrammarLexerOwnerComparisonLoop20Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_owner_comparison_loop20(
    kinds: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    originals: &[self::ml_string::Text],
    positions: &[f64],
    mut result: Vec<f64>,
    mut index: f64,
) -> self::MlReadGrammarLexerOwnerComparisonLoop20Result {
    loop {
        return if index < (kinds.len() as f64) {
            {
                let form = self::read_grammar_lexer_entry_form(
                    &(self::read_required_text_fact(kinds, index)),
                );
                {
                    let name = self::read_required_text_fact(names, index);
                    if ((form == 7f64) || (form == 8f64))
                        && self::contains_grammar_lexer_text(originals, &(name))
                    {
                        self::MlReadGrammarLexerOwnerComparisonLoop20Result::MlReadGrammarLexerOwnerComparisonLoop20Return(vec![-1f64])
                    } else {
                        {
                            let result_3 = if (form == 6f64)
                                && (name != (self::ml_string::Text::from_units(vec![])))
                            {
                                {
                                    let owner =
                                        self::read_grammar_lexer_text_position(originals, &(name));
                                    if owner >= 0f64 {
                                        {
                                            {
                                                let (ml_append_right, ml_append_left) = (
                                                    vec![
                                                        index,
                                                        self::read_required_number_fact(
                                                            positions, owner,
                                                        ),
                                                    ],
                                                    result,
                                                );
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            }
                                        }
                                    } else {
                                        result
                                    }
                                }
                            } else {
                                result
                            };
                            {
                                let index_2 = index + 1f64;
                                {
                                    (result, index) = (result_3, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerOwnerComparisonLoop20Result::MlReadGrammarLexerOwnerComparisonLoop20Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerOwnerComparisonLoop19Result {
    MlReadGrammarLexerOwnerComparisonLoop19Done(
        Vec<self::ml_string::Text>,
        Vec<f64>,
        Vec<f64>,
        f64,
    ),
    MlReadGrammarLexerOwnerComparisonLoop19Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerOwnerComparisonJoin21 {
    MlReadGrammarLexerOwnerComparisonJoin21Next(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        Vec<f64>,
        Vec<f64>,
    ),
    MlReadGrammarLexerOwnerComparisonJoin21Return(Vec<f64>),
}
