// Generated from the complete shared grammar-lexer-rules.js and recorded source closure.
// Do not edit by hand.
use super::*;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_command_plan_loop11(
    mut type_: self::ml_string::Text,
    mut channel: self::ml_string::Text,
    mut disposition: self::ml_string::Text,
    mut actions: Vec<self::ml_string::Text>,
    pieces: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlReadGrammarLexerCommandPlanLoop11Result {
    loop {
        return if index < (pieces.len() as f64) {
            {
                let piece = (self::read_required_text_fact(pieces, index)).trim(true, true, false);
                {
                    let opening =
                        piece.index_of(&self::ml_string::Text::from_units(vec![40]), 0f64);
                    {
                        let head = self::ml_string::Text::from_units(vec![]);
                        {
                            let argument = self::ml_string::Text::from_units(vec![]);
                            {
                                let call = false;
                                {
                                    let ml_s13 = if (opening >= 0f64)
                                        && piece
                                            .ends_with(&self::ml_string::Text::from_units(vec![41]))
                                    {
                                        {
                                            let head_2 = (piece.slice(0f64, opening))
                                                .trim(true, true, false);
                                            {
                                                let argument_2 = (piece.slice(
                                                    (opening + 1f64),
                                                    (piece.length() - 1f64),
                                                ))
                                                .trim(true, true, false);
                                                {
                                                    let call_2 =
                                                        self::accept_grammar_lexer_identifier(
                                                            head_2.clone(),
                                                        ) && (self::accept_grammar_lexer_identifier(
                                                            argument_2.clone(),
                                                        )
                                                            || self::accept_grammar_lexer_decimal(
                                                                argument_2.clone(),
                                                            ));
                                                    self::MlReadGrammarLexerCommandPlanJoin12::MlReadGrammarLexerCommandPlanJoin12Next(head_2.clone(), argument_2.clone(), call_2)
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlReadGrammarLexerCommandPlanJoin12::MlReadGrammarLexerCommandPlanJoin12Next(head, argument, call)
                                    };
                                    match ml_s13 {
                                        self::MlReadGrammarLexerCommandPlanJoin12::MlReadGrammarLexerCommandPlanJoin12Next(head_3, argument_3, call_3) => {
                                            {
                                                let ml_s14 = if piece == (self::ml_string::Text::from_units(vec![115, 107, 105, 112])) {
                                                    {
                                                        let type_2 = self::ml_string::Text::from_units(vec![83, 75, 73, 80]);
                                                        {
                                                            let disposition_2 = self::ml_string::Text::from_units(vec![115, 107, 105, 112]);
                                                            self::MlReadGrammarLexerCommandPlanJoin13::MlReadGrammarLexerCommandPlanJoin13Next(type_2, channel.clone(), disposition_2, actions)
                                                        }
                                                    }
                                                } else if piece == (self::ml_string::Text::from_units(vec![112, 111, 112, 77, 111, 100, 101])) {
                                                        {
                                                            let actions_2 = { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![112, 111, 112, 77, 111, 100, 101]), self::ml_string::Text::from_units(vec![])], actions); self::ml_array::append(ml_append_left, ml_append_right) };
                                                            self::MlReadGrammarLexerCommandPlanJoin13::MlReadGrammarLexerCommandPlanJoin13Next(type_.clone(), channel.clone(), disposition.clone(), actions_2)
                                                        }
                                                    } else if call_3 && (head_3 == (self::ml_string::Text::from_units(vec![116, 121, 112, 101]))) {
                                                            {
                                                                let type_3 = argument_3.clone();
                                                                {
                                                                    let disposition_3 = self::ml_string::Text::from_units(vec![101, 109, 105, 116]);
                                                                    self::MlReadGrammarLexerCommandPlanJoin13::MlReadGrammarLexerCommandPlanJoin13Next(type_3, channel.clone(), disposition_3, actions)
                                                                }
                                                            }
                                                        } else if call_3 && (head_3 == (self::ml_string::Text::from_units(vec![99, 104, 97, 110, 110, 101, 108]))) {
                                                                {
                                                                    let zero = (argument_3.length()) > 0f64;
                                                                    {
                                                                        let offset = 0f64;
                                                                        {
                                                                            let zero_2 = self::ml_read_grammar_lexer_command_plan_loop14(argument_3.clone(), zero, offset);
                                                                            {
                                                                                let channel_2 = if zero_2 || (argument_3 == (self::ml_string::Text::from_units(vec![68, 69, 70, 65, 85, 76, 84, 95, 84, 79, 75, 69, 78, 95, 67, 72, 65, 78, 78, 69, 76]))) {
                                                                                    self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])
                                                                                } else {
                                                                                    argument_3.clone()
                                                                                };
                                                                                self::MlReadGrammarLexerCommandPlanJoin13::MlReadGrammarLexerCommandPlanJoin13Next(type_.clone(), channel_2, disposition.clone(), actions)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else if call_3 && ((head_3 == (self::ml_string::Text::from_units(vec![112, 117, 115, 104, 77, 111, 100, 101]))) || (head_3 == (self::ml_string::Text::from_units(vec![109, 111, 100, 101])))) {
                                                                    {
                                                                        let actions_3 = { let (ml_append_right, ml_append_left) = (vec![if head_3 == (self::ml_string::Text::from_units(vec![109, 111, 100, 101])) {
                                                                            self::ml_string::Text::from_units(vec![115, 101, 116, 77, 111, 100, 101])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![112, 117, 115, 104, 77, 111, 100, 101])
                                                                        }, if argument_3 == (self::ml_string::Text::from_units(vec![68, 69, 70, 65, 85, 76, 84, 95, 77, 79, 68, 69])) {
                                                                            self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])
                                                                        } else {
                                                                            argument_3.clone()
                                                                        }], actions); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        self::MlReadGrammarLexerCommandPlanJoin13::MlReadGrammarLexerCommandPlanJoin13Next(type_.clone(), channel.clone(), disposition.clone(), actions_3)
                                                                    }
                                                                } else if piece != (self::ml_string::Text::from_units(vec![])) {
                                                                        {
                                                                            let actions_4 = { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100]), piece.clone()], actions); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                            self::MlReadGrammarLexerCommandPlanJoin13::MlReadGrammarLexerCommandPlanJoin13Next(type_.clone(), channel.clone(), disposition.clone(), actions_4)
                                                                        }
                                                                    } else {
                                                                        self::MlReadGrammarLexerCommandPlanJoin13::MlReadGrammarLexerCommandPlanJoin13Next(type_.clone(), channel.clone(), disposition.clone(), actions)
                                                                    };
                                                match ml_s14 {
                                                    self::MlReadGrammarLexerCommandPlanJoin13::MlReadGrammarLexerCommandPlanJoin13Next(type_4, channel_3, disposition_4, actions_5) => {
                                                        {
                                                            let index_2 = index + 1f64;
                                                            {
                                                                (type_, channel, disposition, actions, index) = (type_4.clone(), channel_3.clone(), disposition_4.clone(), actions_5, index_2);
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
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerCommandPlanLoop11Result::MlReadGrammarLexerCommandPlanLoop11Done(
                type_.clone(),
                channel.clone(),
                disposition.clone(),
                actions,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueLoop16Result {
    MlReadGrammarLexerDeclarationIssueLoop16Done,
    MlReadGrammarLexerDeclarationIssueLoop16Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin17 {
    MlReadGrammarLexerDeclarationIssueJoin17Next,
    MlReadGrammarLexerDeclarationIssueJoin17Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueLoop18Result {
    MlReadGrammarLexerDeclarationIssueLoop18Done,
    MlReadGrammarLexerDeclarationIssueLoop18Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin19 {
    MlReadGrammarLexerDeclarationIssueJoin19Next,
    MlReadGrammarLexerDeclarationIssueJoin19Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_declaration_issue_loop18(
    action_owners: &[f64],
    actions: &[self::ml_string::Text],
    action_modes: &[self::ml_string::Text],
    declared_modes: &[self::ml_string::Text],
    mut index: f64,
    mut action: f64,
) -> self::MlReadGrammarLexerDeclarationIssueLoop18Result {
    loop {
        return if action < (actions.len() as f64) {
            {
                let ml_s15 = if (self::read_required_number_fact(action_owners, action)) == index {
                    {
                        let operation = self::read_required_text_fact(actions, action);
                        if ((operation
                            != (self::ml_string::Text::from_units(vec![
                                112, 117, 115, 104, 77, 111, 100, 101,
                            ])))
                            && (operation
                                != (self::ml_string::Text::from_units(vec![
                                    115, 101, 116, 77, 111, 100, 101,
                                ]))))
                            && (operation
                                != (self::ml_string::Text::from_units(vec![
                                    112, 111, 112, 77, 111, 100, 101,
                                ])))
                        {
                            self::MlReadGrammarLexerDeclarationIssueJoin19::MlReadGrammarLexerDeclarationIssueJoin19Return(vec![9f64, index, action])
                        } else if (operation
                            != (self::ml_string::Text::from_units(vec![
                                112, 111, 112, 77, 111, 100, 101,
                            ])))
                            && !(self::contains_grammar_lexer_text(
                                declared_modes,
                                self::read_required_text_fact(action_modes, action),
                            ))
                        {
                            self::MlReadGrammarLexerDeclarationIssueJoin19::MlReadGrammarLexerDeclarationIssueJoin19Return(vec![10f64, index, action])
                        } else {
                            self::MlReadGrammarLexerDeclarationIssueJoin19::MlReadGrammarLexerDeclarationIssueJoin19Next
                        }
                    }
                } else {
                    self::MlReadGrammarLexerDeclarationIssueJoin19::MlReadGrammarLexerDeclarationIssueJoin19Next
                };
                match ml_s15 {
                    self::MlReadGrammarLexerDeclarationIssueJoin19::MlReadGrammarLexerDeclarationIssueJoin19Next => {
                        {
                            let action_2 = action + 1f64;
                            {
                                action = action_2;
                                continue;
                            }
                        }
                    }
                    self::MlReadGrammarLexerDeclarationIssueJoin19::MlReadGrammarLexerDeclarationIssueJoin19Return(ml_result19) => {
                        self::MlReadGrammarLexerDeclarationIssueLoop18Result::MlReadGrammarLexerDeclarationIssueLoop18Return(ml_result19)
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerDeclarationIssueLoop18Result::MlReadGrammarLexerDeclarationIssueLoop18Done
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
pub fn ml_read_grammar_lexer_declaration_issue_loop16(
    kinds: &[self::ml_string::Text],
    modes: &[self::ml_string::Text],
    channels: &[self::ml_string::Text],
    dispositions: &[self::ml_string::Text],
    action_owners: &[f64],
    actions: &[self::ml_string::Text],
    action_modes: &[self::ml_string::Text],
    mut count: f64,
    declared_modes: &[self::ml_string::Text],
    declared_channels: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlReadGrammarLexerDeclarationIssueLoop16Result {
    loop {
        return if index < count {
            {
                let ml_s17 = if (self::read_grammar_lexer_entry_form(
                    self::read_required_text_fact(kinds, index),
                )) == 7f64
                {
                    if (self::contains_grammar_lexer_text(
                        declared_modes,
                        self::read_required_text_fact(modes, index),
                    )) {
                        if (self::contains_grammar_lexer_text(
                            declared_channels,
                            self::read_required_text_fact(channels, index),
                        )) && !(self::accept_grammar_lexer_decimal(
                            self::read_required_text_fact(channels, index),
                        )) {
                            {
                                let disposition =
                                    self::read_required_text_fact(dispositions, index);
                                if (disposition
                                    != (self::ml_string::Text::from_units(vec![
                                        101, 109, 105, 116,
                                    ])))
                                    && (disposition
                                        != (self::ml_string::Text::from_units(vec![
                                            115, 107, 105, 112,
                                        ])))
                                {
                                    self::MlReadGrammarLexerDeclarationIssueJoin17::MlReadGrammarLexerDeclarationIssueJoin17Return(vec![11f64, index, -1f64])
                                } else {
                                    {
                                        let action = 0f64;
                                        {
                                            let ml_s18 = self::ml_read_grammar_lexer_declaration_issue_loop18(action_owners, actions, action_modes, declared_modes, index, action);
                                            match ml_s18 {
                                                self::MlReadGrammarLexerDeclarationIssueLoop18Result::MlReadGrammarLexerDeclarationIssueLoop18Done => {
                                                    self::MlReadGrammarLexerDeclarationIssueJoin17::MlReadGrammarLexerDeclarationIssueJoin17Next
                                                }
                                                self::MlReadGrammarLexerDeclarationIssueLoop18Result::MlReadGrammarLexerDeclarationIssueLoop18Return(ml_result18) => {
                                                    self::MlReadGrammarLexerDeclarationIssueJoin17::MlReadGrammarLexerDeclarationIssueJoin17Return(ml_result18)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlReadGrammarLexerDeclarationIssueJoin17::MlReadGrammarLexerDeclarationIssueJoin17Return(vec![8f64, index, -1f64])
                        }
                    } else {
                        self::MlReadGrammarLexerDeclarationIssueJoin17::MlReadGrammarLexerDeclarationIssueJoin17Return(vec![7f64, index, -1f64])
                    }
                } else {
                    self::MlReadGrammarLexerDeclarationIssueJoin17::MlReadGrammarLexerDeclarationIssueJoin17Next
                };
                match ml_s17 {
                    self::MlReadGrammarLexerDeclarationIssueJoin17::MlReadGrammarLexerDeclarationIssueJoin17Next => {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                    self::MlReadGrammarLexerDeclarationIssueJoin17::MlReadGrammarLexerDeclarationIssueJoin17Return(ml_result17) => {
                        self::MlReadGrammarLexerDeclarationIssueLoop16Result::MlReadGrammarLexerDeclarationIssueLoop16Return(ml_result17)
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerDeclarationIssueLoop16Result::MlReadGrammarLexerDeclarationIssueLoop16Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueLoop15Result {
    MlReadGrammarLexerDeclarationIssueLoop15Done(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        f64,
    ),
    MlReadGrammarLexerDeclarationIssueLoop15Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin23 {
    MlReadGrammarLexerDeclarationIssueJoin23Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerDeclarationIssueJoin23Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin22 {
    MlReadGrammarLexerDeclarationIssueJoin22Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerDeclarationIssueJoin22Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin21 {
    MlReadGrammarLexerDeclarationIssueJoin21Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerDeclarationIssueJoin21Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin20 {
    MlReadGrammarLexerDeclarationIssueJoin20Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerDeclarationIssueJoin20Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_grammar_lexer_declaration_issue_loop15(
    kinds: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    items: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut count: f64,
    mut binding_names: Vec<self::ml_string::Text>,
    mut owner_names: Vec<self::ml_string::Text>,
    mut literal_values: Vec<self::ml_string::Text>,
    mut local_names: Vec<self::ml_string::Text>,
    mut declared_modes: Vec<self::ml_string::Text>,
    mut declared_channels: Vec<self::ml_string::Text>,
    mut index: f64,
) -> self::MlReadGrammarLexerDeclarationIssueLoop15Result {
    loop {
        return if index < count {
            {
                let form = self::read_grammar_lexer_entry_form(self::read_required_text_fact(
                    kinds, index,
                ));
                if form == 0f64 {
                    self::MlReadGrammarLexerDeclarationIssueLoop15Result::MlReadGrammarLexerDeclarationIssueLoop15Return(vec![2f64, index, -1f64])
                } else {
                    {
                        let ml_s19 = if form == 1f64 {
                            if ((self::read_required_text_fact(items, index))
                                != (self::ml_string::Text::from_units(vec![114, 101, 102])))
                                || self::contains_grammar_lexer_text(
                                    &(owner_names),
                                    self::read_required_text_fact(names, index),
                                )
                            {
                                self::MlReadGrammarLexerDeclarationIssueJoin20::MlReadGrammarLexerDeclarationIssueJoin20Return(vec![3f64, index, -1f64])
                            } else {
                                {
                                    let owner_names_2 = {
                                        let (ml_append_right, ml_append_left) = (
                                            vec![self::read_required_text_fact(names, index)],
                                            owner_names,
                                        );
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    self::MlReadGrammarLexerDeclarationIssueJoin20::MlReadGrammarLexerDeclarationIssueJoin20Next(owner_names_2)
                                }
                            }
                        } else {
                            self::MlReadGrammarLexerDeclarationIssueJoin20::MlReadGrammarLexerDeclarationIssueJoin20Next(owner_names)
                        };
                        match ml_s19 {
                            self::MlReadGrammarLexerDeclarationIssueJoin20::MlReadGrammarLexerDeclarationIssueJoin20Next(owner_names_3) => {
                                {
                                    let ml_s20 = if form == 2f64 {
                                        if ((self::read_required_text_fact(items, index)) != (self::ml_string::Text::from_units(vec![114, 101, 102]))) || self::contains_grammar_lexer_text(&(binding_names), self::read_required_text_fact(names, index)) {
                                            self::MlReadGrammarLexerDeclarationIssueJoin21::MlReadGrammarLexerDeclarationIssueJoin21Return(vec![4f64, index, -1f64])
                                        } else {
                                            {
                                                let binding_names_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_text_fact(names, index)], binding_names); self::ml_array::append(ml_append_left, ml_append_right) };
                                                self::MlReadGrammarLexerDeclarationIssueJoin21::MlReadGrammarLexerDeclarationIssueJoin21Next(binding_names_2)
                                            }
                                        }
                                    } else {
                                        self::MlReadGrammarLexerDeclarationIssueJoin21::MlReadGrammarLexerDeclarationIssueJoin21Next(binding_names)
                                    };
                                    match ml_s20 {
                                        self::MlReadGrammarLexerDeclarationIssueJoin21::MlReadGrammarLexerDeclarationIssueJoin21Next(binding_names_3) => {
                                            {
                                                let ml_s21 = if form == 3f64 {
                                                    if self::contains_grammar_lexer_text(&(literal_values), self::read_required_text_fact(values, index)) {
                                                        self::MlReadGrammarLexerDeclarationIssueJoin22::MlReadGrammarLexerDeclarationIssueJoin22Return(vec![5f64, index, -1f64])
                                                    } else {
                                                        {
                                                            let literal_values_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_text_fact(values, index)], literal_values); self::ml_array::append(ml_append_left, ml_append_right) };
                                                            self::MlReadGrammarLexerDeclarationIssueJoin22::MlReadGrammarLexerDeclarationIssueJoin22Next(literal_values_2)
                                                        }
                                                    }
                                                } else {
                                                    self::MlReadGrammarLexerDeclarationIssueJoin22::MlReadGrammarLexerDeclarationIssueJoin22Next(literal_values)
                                                };
                                                match ml_s21 {
                                                    self::MlReadGrammarLexerDeclarationIssueJoin22::MlReadGrammarLexerDeclarationIssueJoin22Next(literal_values_3) => {
                                                        {
                                                            let declared_modes_3 = if form == 4f64 {
                                                                {
                                                                    { let (ml_append_right, ml_append_left) = (vec![self::read_required_text_fact(names, index)], declared_modes); self::ml_array::append(ml_append_left, ml_append_right) }
                                                                }
                                                            } else {
                                                                declared_modes
                                                            };
                                                            {
                                                                let declared_channels_3 = if form == 5f64 {
                                                                    {
                                                                        { let (ml_append_right, ml_append_left) = (vec![self::read_required_text_fact(names, index)], declared_channels); self::ml_array::append(ml_append_left, ml_append_right) }
                                                                    }
                                                                } else {
                                                                    declared_channels
                                                                };
                                                                {
                                                                    let ml_s22 = if (form == 7f64) || (form == 8f64) {
                                                                        if self::contains_grammar_lexer_text(&(local_names), self::read_required_text_fact(names, index)) {
                                                                            self::MlReadGrammarLexerDeclarationIssueJoin23::MlReadGrammarLexerDeclarationIssueJoin23Return(vec![6f64, index, -1f64])
                                                                        } else {
                                                                            {
                                                                                let local_names_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_text_fact(names, index)], local_names); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                self::MlReadGrammarLexerDeclarationIssueJoin23::MlReadGrammarLexerDeclarationIssueJoin23Next(local_names_2)
                                                                            }
                                                                        }
                                                                    } else {
                                                                        self::MlReadGrammarLexerDeclarationIssueJoin23::MlReadGrammarLexerDeclarationIssueJoin23Next(local_names)
                                                                    };
                                                                    match ml_s22 {
                                                                        self::MlReadGrammarLexerDeclarationIssueJoin23::MlReadGrammarLexerDeclarationIssueJoin23Next(local_names_3) => {
                                                                            {
                                                                                let index_2 = index + 1f64;
                                                                                {
                                                                                    (binding_names, owner_names, literal_values, local_names, declared_modes, declared_channels, index) = (binding_names_3, owner_names_3, literal_values_3, local_names_3, declared_modes_3, declared_channels_3, index_2);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlReadGrammarLexerDeclarationIssueJoin23::MlReadGrammarLexerDeclarationIssueJoin23Return(ml_result23) => {
                                                                            self::MlReadGrammarLexerDeclarationIssueLoop15Result::MlReadGrammarLexerDeclarationIssueLoop15Return(ml_result23)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlReadGrammarLexerDeclarationIssueJoin22::MlReadGrammarLexerDeclarationIssueJoin22Return(ml_result23_2) => {
                                                        self::MlReadGrammarLexerDeclarationIssueLoop15Result::MlReadGrammarLexerDeclarationIssueLoop15Return(ml_result23_2)
                                                    }
                                                }
                                            }
                                        }
                                        self::MlReadGrammarLexerDeclarationIssueJoin21::MlReadGrammarLexerDeclarationIssueJoin21Return(ml_result23_3) => {
                                            self::MlReadGrammarLexerDeclarationIssueLoop15Result::MlReadGrammarLexerDeclarationIssueLoop15Return(ml_result23_3)
                                        }
                                    }
                                }
                            }
                            self::MlReadGrammarLexerDeclarationIssueJoin20::MlReadGrammarLexerDeclarationIssueJoin20Return(ml_result23_4) => {
                                self::MlReadGrammarLexerDeclarationIssueLoop15Result::MlReadGrammarLexerDeclarationIssueLoop15Return(ml_result23_4)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerDeclarationIssueLoop15Result::MlReadGrammarLexerDeclarationIssueLoop15Done(declared_modes, declared_channels, index)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_obligation_positions_loop25(
    mut count: f64,
    mut root: f64,
    owners: &[f64],
    categories: &[self::ml_string::Text],
    reached: &[f64],
    mut index: f64,
    mut selected: Vec<f64>,
) -> Vec<f64> {
    loop {
        if index < (owners.len() as f64) {
            {
                let owner = self::read_required_number_fact(owners, index);
                {
                    let selected_3 = if (((self::ml_math::is_safe_integer(owner)
                        && (owner >= 0f64))
                        && (owner < count))
                        && self::contains_grammar_lexer_position(reached, owner))
                        && ((owner == root)
                            || (((self::read_required_text_fact(categories, index))
                                != (self::ml_string::Text::from_units(vec![
                                    97, 99, 116, 105, 111, 110,
                                ])))
                                && ((self::read_required_text_fact(categories, index))
                                    != (self::ml_string::Text::from_units(vec![
                                        99, 111, 109, 109, 97, 110, 100,
                                    ]))))) {
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
                            (index, selected) = (index_2, selected_3);
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
pub enum MlReadGrammarLexerObligationPositionsLoop24Result {
    MlReadGrammarLexerObligationPositionsLoop24Done(Vec<f64>, f64),
    MlReadGrammarLexerObligationPositionsLoop24Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerObligationPositionsJoin26 {
    MlReadGrammarLexerObligationPositionsJoin26Next(Vec<f64>, f64, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerObligationPositionsLoop27Result {
    MlReadGrammarLexerObligationPositionsLoop27Done(f64, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_obligation_positions_loop27(
    edge_sources: &[f64],
    edge_targets: &[f64],
    mut index: f64,
    mut pending: Vec<f64>,
    mut current: f64,
) -> self::MlReadGrammarLexerObligationPositionsLoop27Result {
    loop {
        if index < (edge_sources.len() as f64) {
            {
                let pending_3 = if (self::read_required_number_fact(edge_sources, index)) == current
                {
                    {
                        {
                            let (ml_append_right, ml_append_left) = (
                                vec![self::read_required_number_fact(edge_targets, index)],
                                pending,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        }
                    }
                } else {
                    pending
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (index, pending) = (index_2, pending_3);
                        continue;
                    }
                }
            }
        }
        return {
            self::MlReadGrammarLexerObligationPositionsLoop27Result::MlReadGrammarLexerObligationPositionsLoop27Done(index, pending)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_obligation_positions_loop24(
    mut count: f64,
    edge_sources: &[f64],
    edge_targets: &[f64],
    mut reached: Vec<f64>,
    mut index: f64,
    mut pending: Vec<f64>,
    mut cursor: f64,
) -> self::MlReadGrammarLexerObligationPositionsLoop24Result {
    loop {
        return if cursor < (pending.len() as f64) {
            {
                let current = self::read_required_number_fact(&(pending), cursor);
                if (!(self::ml_math::is_safe_integer(current)) || (current < 0f64))
                    || (current >= count)
                {
                    self::MlReadGrammarLexerObligationPositionsLoop24Result::MlReadGrammarLexerObligationPositionsLoop24Return(vec![-1f64])
                } else {
                    {
                        let ml_s24 = if (self::contains_grammar_lexer_position(&(reached), current))
                        {
                            self::MlReadGrammarLexerObligationPositionsJoin26::MlReadGrammarLexerObligationPositionsJoin26Next(reached, index, pending)
                        } else {
                            {
                                let reached_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![current], reached);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let index_2 = 0f64;
                                    {
                                        let ml_s25 =
                                            self::ml_read_grammar_lexer_obligation_positions_loop27(
                                                edge_sources,
                                                edge_targets,
                                                index_2,
                                                pending,
                                                current,
                                            );
                                        match ml_s25 {
                                            self::MlReadGrammarLexerObligationPositionsLoop27Result::MlReadGrammarLexerObligationPositionsLoop27Done(index_3, pending_2) => {
                                                self::MlReadGrammarLexerObligationPositionsJoin26::MlReadGrammarLexerObligationPositionsJoin26Next(reached_2, index_3, pending_2)
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        match ml_s24 {
                            self::MlReadGrammarLexerObligationPositionsJoin26::MlReadGrammarLexerObligationPositionsJoin26Next(reached_3, index_4, pending_3) => {
                                {
                                    let cursor_2 = cursor + 1f64;
                                    {
                                        (reached, index, pending, cursor) = (reached_3, index_4, pending_3, cursor_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerObligationPositionsLoop24Result::MlReadGrammarLexerObligationPositionsLoop24Done(reached, index)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_lazy_normalization_loop30(
    edge_sources: &[f64],
    edge_targets: &[f64],
    mut owner: f64,
    mut children: Vec<f64>,
    mut edge: f64,
) -> Vec<f64> {
    loop {
        if edge < (edge_sources.len() as f64) {
            {
                let children_3 = if (self::read_required_number_fact(edge_sources, edge)) == owner {
                    {
                        {
                            let (ml_append_right, ml_append_left) = (
                                vec![self::read_required_number_fact(edge_targets, edge)],
                                children,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        }
                    }
                } else {
                    children
                };
                {
                    let edge_2 = edge + 1f64;
                    {
                        (children, edge) = (children_3, edge_2);
                        continue;
                    }
                }
            }
        }
        return { children };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_grammar_lexer_lazy_normalization_loop29(
    kinds: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    edge_sources: &[f64],
    edge_targets: &[f64],
    mut count: f64,
    mut index: f64,
    mut delimiter: f64,
    mut owner: f64,
) -> f64 {
    loop {
        if owner < count {
            {
                let delimiter_3 = if (self::read_required_text_fact(kinds, owner))
                    == (self::ml_string::Text::from_units(vec![115, 101, 113]))
                {
                    {
                        let children = Vec::<f64>::new();
                        {
                            let edge = 0f64;
                            {
                                let children_2 =
                                    self::ml_read_grammar_lexer_lazy_normalization_loop30(
                                        edge_sources,
                                        edge_targets,
                                        owner,
                                        children,
                                        edge,
                                    );
                                if ((children_2.len() as f64) >= 2f64)
                                    && ((self::read_required_number_fact(
                                        &(children_2),
                                        ((children_2.len() as f64) - 2f64),
                                    )) == index)
                                {
                                    {
                                        let last = self::read_required_number_fact(
                                            &(children_2),
                                            ((children_2.len() as f64) - 1f64),
                                        );
                                        if (((last >= 0f64) && (last < count))
                                            && ((self::read_required_text_fact(kinds, last))
                                                == (self::ml_string::Text::from_units(vec![
                                                    108, 105, 116, 101, 114, 97, 108,
                                                ]))))
                                            && ((self::read_required_text_fact(texts, last))
                                                != (self::ml_string::Text::from_units(vec![])))
                                        {
                                            { last }
                                        } else {
                                            delimiter
                                        }
                                    }
                                } else {
                                    delimiter
                                }
                            }
                        }
                    }
                } else {
                    delimiter
                };
                {
                    let owner_2 = owner + 1f64;
                    {
                        (delimiter, owner) = (delimiter_3, owner_2);
                        continue;
                    }
                }
            }
        }
        return { delimiter };
    }
}
