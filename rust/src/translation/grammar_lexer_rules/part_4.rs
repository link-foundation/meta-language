// Generated from the complete shared grammar-lexer-rules.js and recorded source closure.
// Do not edit by hand.
use super::MlReadGrammarLexerCommandPlanJoin25;
use super::MlReadGrammarLexerCommandPlanJoin26;
use super::MlReadGrammarLexerCommandPlanLoop24Result;
use super::accept_grammar_lexer_decimal;
use super::accept_grammar_lexer_identifier;
use super::contains_grammar_lexer_position;
use super::contains_grammar_lexer_text;
use super::ml_array;
use super::ml_math;
use super::ml_read_grammar_lexer_command_plan_loop27;
use super::ml_string;
use super::read_grammar_lexer_entry_form;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_command_plan_loop24(
    mut type_: self::ml_string::Text,
    mut channel: self::ml_string::Text,
    mut disposition: self::ml_string::Text,
    mut actions: Vec<self::ml_string::Text>,
    pieces: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlReadGrammarLexerCommandPlanLoop24Result {
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
                                    let ml_s26 = if (opening >= 0f64)
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
                                                            &(head_2),
                                                        ) && (self::accept_grammar_lexer_identifier(
                                                            &(argument_2),
                                                        )
                                                            || self::accept_grammar_lexer_decimal(
                                                                &(argument_2),
                                                            ));
                                                    self::MlReadGrammarLexerCommandPlanJoin25::MlReadGrammarLexerCommandPlanJoin25Next(head_2.clone(), argument_2.clone(), call_2)
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlReadGrammarLexerCommandPlanJoin25::MlReadGrammarLexerCommandPlanJoin25Next(head, argument, call)
                                    };
                                    match ml_s26 {
                                        self::MlReadGrammarLexerCommandPlanJoin25::MlReadGrammarLexerCommandPlanJoin25Next(head_3, argument_3, call_3) => {
                                            {
                                                let ml_s27 = if piece == (self::ml_string::Text::from_units(vec![115, 107, 105, 112])) {
                                                    {
                                                        let type_2 = self::ml_string::Text::from_units(vec![83, 75, 73, 80]);
                                                        {
                                                            let disposition_2 = self::ml_string::Text::from_units(vec![115, 107, 105, 112]);
                                                            self::MlReadGrammarLexerCommandPlanJoin26::MlReadGrammarLexerCommandPlanJoin26Next(type_2, channel, disposition_2, actions)
                                                        }
                                                    }
                                                } else if piece == (self::ml_string::Text::from_units(vec![112, 111, 112, 77, 111, 100, 101])) {
                                                        {
                                                            let actions_2 = { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![112, 111, 112, 77, 111, 100, 101]), self::ml_string::Text::from_units(vec![])], actions); self::ml_array::append(ml_append_left, ml_append_right) };
                                                            self::MlReadGrammarLexerCommandPlanJoin26::MlReadGrammarLexerCommandPlanJoin26Next(type_, channel, disposition, actions_2)
                                                        }
                                                    } else if call_3 && (head_3 == (self::ml_string::Text::from_units(vec![116, 121, 112, 101]))) {
                                                            {
                                                                let type_3 = argument_3.clone();
                                                                {
                                                                    let disposition_3 = self::ml_string::Text::from_units(vec![101, 109, 105, 116]);
                                                                    self::MlReadGrammarLexerCommandPlanJoin26::MlReadGrammarLexerCommandPlanJoin26Next(type_3, channel, disposition_3, actions)
                                                                }
                                                            }
                                                        } else if call_3 && (head_3 == (self::ml_string::Text::from_units(vec![99, 104, 97, 110, 110, 101, 108]))) {
                                                                {
                                                                    let zero = (argument_3.length()) > 0f64;
                                                                    {
                                                                        let offset = 0f64;
                                                                        {
                                                                            let zero_2 = self::ml_read_grammar_lexer_command_plan_loop27(&(argument_3), zero, offset);
                                                                            {
                                                                                let channel_2 = if zero_2 || (argument_3 == (self::ml_string::Text::from_units(vec![68, 69, 70, 65, 85, 76, 84, 95, 84, 79, 75, 69, 78, 95, 67, 72, 65, 78, 78, 69, 76]))) {
                                                                                    self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])
                                                                                } else {
                                                                                    argument_3.clone()
                                                                                };
                                                                                self::MlReadGrammarLexerCommandPlanJoin26::MlReadGrammarLexerCommandPlanJoin26Next(type_, channel_2, disposition, actions)
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
                                                                        self::MlReadGrammarLexerCommandPlanJoin26::MlReadGrammarLexerCommandPlanJoin26Next(type_, channel, disposition, actions_3)
                                                                    }
                                                                } else {
let ml_library_inequality_1 = piece != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                                        {
                                                                            let actions_4 = { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100]), piece.clone()], actions); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                            self::MlReadGrammarLexerCommandPlanJoin26::MlReadGrammarLexerCommandPlanJoin26Next(type_, channel, disposition, actions_4)
                                                                        }
                                                                    } else {
                                                                        self::MlReadGrammarLexerCommandPlanJoin26::MlReadGrammarLexerCommandPlanJoin26Next(type_, channel, disposition, actions)
                                                                    }
};
                                                match ml_s27 {
                                                    self::MlReadGrammarLexerCommandPlanJoin26::MlReadGrammarLexerCommandPlanJoin26Next(type_4, channel_3, disposition_4, actions_5) => {
                                                        {
                                                            let index_2 = index + 1f64;
                                                            {
                                                                (type_, channel, disposition, actions, index) = (type_4, channel_3, disposition_4, actions_5, index_2);
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
            self::MlReadGrammarLexerCommandPlanLoop24Result::MlReadGrammarLexerCommandPlanLoop24Done(
                type_,
                channel,
                disposition,
                actions,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueLoop29Result {
    MlReadGrammarLexerDeclarationIssueLoop29Done,
    MlReadGrammarLexerDeclarationIssueLoop29Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin30 {
    MlReadGrammarLexerDeclarationIssueJoin30Next,
    MlReadGrammarLexerDeclarationIssueJoin30Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueLoop31Result {
    MlReadGrammarLexerDeclarationIssueLoop31Done,
    MlReadGrammarLexerDeclarationIssueLoop31Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin32 {
    MlReadGrammarLexerDeclarationIssueJoin32Next,
    MlReadGrammarLexerDeclarationIssueJoin32Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_declaration_issue_loop31(
    action_owners: &[f64],
    actions: &[self::ml_string::Text],
    action_modes: &[self::ml_string::Text],
    declared_modes: &[self::ml_string::Text],
    mut index: f64,
    mut action: f64,
) -> self::MlReadGrammarLexerDeclarationIssueLoop31Result {
    loop {
        return if action < (actions.len() as f64) {
            {
                let ml_s28 = if (self::read_required_number_fact(action_owners, action)) == index {
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
                            self::MlReadGrammarLexerDeclarationIssueJoin32::MlReadGrammarLexerDeclarationIssueJoin32Return(vec![9f64, index, action])
                        } else if (operation
                            != (self::ml_string::Text::from_units(vec![
                                112, 111, 112, 77, 111, 100, 101,
                            ])))
                            && !(self::contains_grammar_lexer_text(
                                declared_modes,
                                &(self::read_required_text_fact(action_modes, action)),
                            ))
                        {
                            self::MlReadGrammarLexerDeclarationIssueJoin32::MlReadGrammarLexerDeclarationIssueJoin32Return(vec![10f64, index, action])
                        } else {
                            self::MlReadGrammarLexerDeclarationIssueJoin32::MlReadGrammarLexerDeclarationIssueJoin32Next
                        }
                    }
                } else {
                    self::MlReadGrammarLexerDeclarationIssueJoin32::MlReadGrammarLexerDeclarationIssueJoin32Next
                };
                match ml_s28 {
                    self::MlReadGrammarLexerDeclarationIssueJoin32::MlReadGrammarLexerDeclarationIssueJoin32Next => {
                        {
                            let action_2 = action + 1f64;
                            {
                                action = action_2;
                                continue;
                            }
                        }
                    }
                    self::MlReadGrammarLexerDeclarationIssueJoin32::MlReadGrammarLexerDeclarationIssueJoin32Return(ml_result32) => {
                        self::MlReadGrammarLexerDeclarationIssueLoop31Result::MlReadGrammarLexerDeclarationIssueLoop31Return(ml_result32)
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerDeclarationIssueLoop31Result::MlReadGrammarLexerDeclarationIssueLoop31Done
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
pub fn ml_read_grammar_lexer_declaration_issue_loop29(
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
) -> self::MlReadGrammarLexerDeclarationIssueLoop29Result {
    loop {
        return if index < count {
            {
                let ml_s30 = if (self::read_grammar_lexer_entry_form(
                    &(self::read_required_text_fact(kinds, index)),
                )) == 7f64
                {
                    if (self::contains_grammar_lexer_text(
                        declared_modes,
                        &(self::read_required_text_fact(modes, index)),
                    )) {
                        if ((self::contains_grammar_lexer_text(
                            declared_channels,
                            &(self::read_required_text_fact(channels, index)),
                        )) || (self::accept_grammar_lexer_decimal(
                            &(self::read_required_text_fact(channels, index)),
                        ))) {
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
                                    self::MlReadGrammarLexerDeclarationIssueJoin30::MlReadGrammarLexerDeclarationIssueJoin30Return(vec![11f64, index, -1f64])
                                } else {
                                    {
                                        let action = 0f64;
                                        {
                                            let ml_s31 = self::ml_read_grammar_lexer_declaration_issue_loop31(action_owners, actions, action_modes, declared_modes, index, action);
                                            match ml_s31 {
                                                self::MlReadGrammarLexerDeclarationIssueLoop31Result::MlReadGrammarLexerDeclarationIssueLoop31Done => {
                                                    self::MlReadGrammarLexerDeclarationIssueJoin30::MlReadGrammarLexerDeclarationIssueJoin30Next
                                                }
                                                self::MlReadGrammarLexerDeclarationIssueLoop31Result::MlReadGrammarLexerDeclarationIssueLoop31Return(ml_result31) => {
                                                    self::MlReadGrammarLexerDeclarationIssueJoin30::MlReadGrammarLexerDeclarationIssueJoin30Return(ml_result31)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlReadGrammarLexerDeclarationIssueJoin30::MlReadGrammarLexerDeclarationIssueJoin30Return(vec![8f64, index, -1f64])
                        }
                    } else {
                        self::MlReadGrammarLexerDeclarationIssueJoin30::MlReadGrammarLexerDeclarationIssueJoin30Return(vec![7f64, index, -1f64])
                    }
                } else {
                    self::MlReadGrammarLexerDeclarationIssueJoin30::MlReadGrammarLexerDeclarationIssueJoin30Next
                };
                match ml_s30 {
                    self::MlReadGrammarLexerDeclarationIssueJoin30::MlReadGrammarLexerDeclarationIssueJoin30Next => {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                    self::MlReadGrammarLexerDeclarationIssueJoin30::MlReadGrammarLexerDeclarationIssueJoin30Return(ml_result30) => {
                        self::MlReadGrammarLexerDeclarationIssueLoop29Result::MlReadGrammarLexerDeclarationIssueLoop29Return(ml_result30)
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerDeclarationIssueLoop29Result::MlReadGrammarLexerDeclarationIssueLoop29Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueLoop28Result {
    MlReadGrammarLexerDeclarationIssueLoop28Done(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        f64,
    ),
    MlReadGrammarLexerDeclarationIssueLoop28Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin36 {
    MlReadGrammarLexerDeclarationIssueJoin36Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerDeclarationIssueJoin36Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin35 {
    MlReadGrammarLexerDeclarationIssueJoin35Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerDeclarationIssueJoin35Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin34 {
    MlReadGrammarLexerDeclarationIssueJoin34Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerDeclarationIssueJoin34Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin33 {
    MlReadGrammarLexerDeclarationIssueJoin33Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerDeclarationIssueJoin33Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_grammar_lexer_declaration_issue_loop28(
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
) -> self::MlReadGrammarLexerDeclarationIssueLoop28Result {
    loop {
        return if index < count {
            {
                let form = self::read_grammar_lexer_entry_form(
                    &(self::read_required_text_fact(kinds, index)),
                );
                if form == 0f64 {
                    self::MlReadGrammarLexerDeclarationIssueLoop28Result::MlReadGrammarLexerDeclarationIssueLoop28Return(vec![2f64, index, -1f64])
                } else {
                    {
                        let ml_s32 = if form == 1f64 {
                            if ((self::read_required_text_fact(items, index))
                                != (self::ml_string::Text::from_units(vec![114, 101, 102])))
                                || self::contains_grammar_lexer_text(
                                    &(owner_names),
                                    &(self::read_required_text_fact(names, index)),
                                )
                            {
                                self::MlReadGrammarLexerDeclarationIssueJoin33::MlReadGrammarLexerDeclarationIssueJoin33Return(vec![3f64, index, -1f64])
                            } else {
                                {
                                    let owner_names_2 = {
                                        let (ml_append_right, ml_append_left) = (
                                            vec![self::read_required_text_fact(names, index)],
                                            owner_names,
                                        );
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    self::MlReadGrammarLexerDeclarationIssueJoin33::MlReadGrammarLexerDeclarationIssueJoin33Next(owner_names_2)
                                }
                            }
                        } else {
                            self::MlReadGrammarLexerDeclarationIssueJoin33::MlReadGrammarLexerDeclarationIssueJoin33Next(owner_names)
                        };
                        match ml_s32 {
                            self::MlReadGrammarLexerDeclarationIssueJoin33::MlReadGrammarLexerDeclarationIssueJoin33Next(owner_names_3) => {
                                {
                                    let ml_s33 = if form == 2f64 {
                                        if ((self::read_required_text_fact(items, index)) != (self::ml_string::Text::from_units(vec![114, 101, 102]))) || self::contains_grammar_lexer_text(&(binding_names), &(self::read_required_text_fact(names, index))) {
                                            self::MlReadGrammarLexerDeclarationIssueJoin34::MlReadGrammarLexerDeclarationIssueJoin34Return(vec![4f64, index, -1f64])
                                        } else {
                                            {
                                                let binding_names_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_text_fact(names, index)], binding_names); self::ml_array::append(ml_append_left, ml_append_right) };
                                                self::MlReadGrammarLexerDeclarationIssueJoin34::MlReadGrammarLexerDeclarationIssueJoin34Next(binding_names_2)
                                            }
                                        }
                                    } else {
                                        self::MlReadGrammarLexerDeclarationIssueJoin34::MlReadGrammarLexerDeclarationIssueJoin34Next(binding_names)
                                    };
                                    match ml_s33 {
                                        self::MlReadGrammarLexerDeclarationIssueJoin34::MlReadGrammarLexerDeclarationIssueJoin34Next(binding_names_3) => {
                                            {
                                                let ml_s34 = if form == 3f64 {
                                                    if self::contains_grammar_lexer_text(&(literal_values), &(self::read_required_text_fact(values, index))) {
                                                        self::MlReadGrammarLexerDeclarationIssueJoin35::MlReadGrammarLexerDeclarationIssueJoin35Return(vec![5f64, index, -1f64])
                                                    } else {
                                                        {
                                                            let literal_values_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_text_fact(values, index)], literal_values); self::ml_array::append(ml_append_left, ml_append_right) };
                                                            self::MlReadGrammarLexerDeclarationIssueJoin35::MlReadGrammarLexerDeclarationIssueJoin35Next(literal_values_2)
                                                        }
                                                    }
                                                } else {
                                                    self::MlReadGrammarLexerDeclarationIssueJoin35::MlReadGrammarLexerDeclarationIssueJoin35Next(literal_values)
                                                };
                                                match ml_s34 {
                                                    self::MlReadGrammarLexerDeclarationIssueJoin35::MlReadGrammarLexerDeclarationIssueJoin35Next(literal_values_3) => {
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
                                                                    let ml_s35 = if (form == 7f64) || (form == 8f64) {
                                                                        if self::contains_grammar_lexer_text(&(local_names), &(self::read_required_text_fact(names, index))) {
                                                                            self::MlReadGrammarLexerDeclarationIssueJoin36::MlReadGrammarLexerDeclarationIssueJoin36Return(vec![6f64, index, -1f64])
                                                                        } else {
                                                                            {
                                                                                let local_names_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_text_fact(names, index)], local_names); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                self::MlReadGrammarLexerDeclarationIssueJoin36::MlReadGrammarLexerDeclarationIssueJoin36Next(local_names_2)
                                                                            }
                                                                        }
                                                                    } else {
                                                                        self::MlReadGrammarLexerDeclarationIssueJoin36::MlReadGrammarLexerDeclarationIssueJoin36Next(local_names)
                                                                    };
                                                                    match ml_s35 {
                                                                        self::MlReadGrammarLexerDeclarationIssueJoin36::MlReadGrammarLexerDeclarationIssueJoin36Next(local_names_3) => {
                                                                            {
                                                                                let index_2 = index + 1f64;
                                                                                {
                                                                                    (binding_names, owner_names, literal_values, local_names, declared_modes, declared_channels, index) = (binding_names_3, owner_names_3, literal_values_3, local_names_3, declared_modes_3, declared_channels_3, index_2);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlReadGrammarLexerDeclarationIssueJoin36::MlReadGrammarLexerDeclarationIssueJoin36Return(ml_result36) => {
                                                                            self::MlReadGrammarLexerDeclarationIssueLoop28Result::MlReadGrammarLexerDeclarationIssueLoop28Return(ml_result36)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlReadGrammarLexerDeclarationIssueJoin35::MlReadGrammarLexerDeclarationIssueJoin35Return(ml_result36_2) => {
                                                        self::MlReadGrammarLexerDeclarationIssueLoop28Result::MlReadGrammarLexerDeclarationIssueLoop28Return(ml_result36_2)
                                                    }
                                                }
                                            }
                                        }
                                        self::MlReadGrammarLexerDeclarationIssueJoin34::MlReadGrammarLexerDeclarationIssueJoin34Return(ml_result36_3) => {
                                            self::MlReadGrammarLexerDeclarationIssueLoop28Result::MlReadGrammarLexerDeclarationIssueLoop28Return(ml_result36_3)
                                        }
                                    }
                                }
                            }
                            self::MlReadGrammarLexerDeclarationIssueJoin33::MlReadGrammarLexerDeclarationIssueJoin33Return(ml_result36_4) => {
                                self::MlReadGrammarLexerDeclarationIssueLoop28Result::MlReadGrammarLexerDeclarationIssueLoop28Return(ml_result36_4)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerDeclarationIssueLoop28Result::MlReadGrammarLexerDeclarationIssueLoop28Done(declared_modes, declared_channels, index)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_obligation_positions_loop38(
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
                        && (((owner).partial_cmp(&(root)) == Some(std::cmp::Ordering::Equal))
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
pub enum MlReadGrammarLexerObligationPositionsLoop37Result {
    MlReadGrammarLexerObligationPositionsLoop37Done(Vec<f64>, f64),
    MlReadGrammarLexerObligationPositionsLoop37Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerObligationPositionsJoin39 {
    MlReadGrammarLexerObligationPositionsJoin39Next(Vec<f64>, f64, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerObligationPositionsLoop40Result {
    MlReadGrammarLexerObligationPositionsLoop40Done(f64, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_obligation_positions_loop40(
    edge_sources: &[f64],
    edge_targets: &[f64],
    mut index: f64,
    mut pending: Vec<f64>,
    mut current: f64,
) -> self::MlReadGrammarLexerObligationPositionsLoop40Result {
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
            self::MlReadGrammarLexerObligationPositionsLoop40Result::MlReadGrammarLexerObligationPositionsLoop40Done(index, pending)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_obligation_positions_loop37(
    mut count: f64,
    edge_sources: &[f64],
    edge_targets: &[f64],
    mut reached: Vec<f64>,
    mut index: f64,
    mut pending: Vec<f64>,
    mut cursor: f64,
) -> self::MlReadGrammarLexerObligationPositionsLoop37Result {
    loop {
        return if cursor < (pending.len() as f64) {
            {
                let current = self::read_required_number_fact(&(pending), cursor);
                if (!(self::ml_math::is_safe_integer(current)) || (current < 0f64))
                    || (current >= count)
                {
                    self::MlReadGrammarLexerObligationPositionsLoop37Result::MlReadGrammarLexerObligationPositionsLoop37Return(vec![-1f64])
                } else {
                    {
                        let ml_s37 = if (self::contains_grammar_lexer_position(&(reached), current))
                        {
                            self::MlReadGrammarLexerObligationPositionsJoin39::MlReadGrammarLexerObligationPositionsJoin39Next(reached, index, pending)
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
                                        let ml_s38 =
                                            self::ml_read_grammar_lexer_obligation_positions_loop40(
                                                edge_sources,
                                                edge_targets,
                                                index_2,
                                                pending,
                                                current,
                                            );
                                        match ml_s38 {
                                            self::MlReadGrammarLexerObligationPositionsLoop40Result::MlReadGrammarLexerObligationPositionsLoop40Done(index_3, pending_2) => {
                                                self::MlReadGrammarLexerObligationPositionsJoin39::MlReadGrammarLexerObligationPositionsJoin39Next(reached_2, index_3, pending_2)
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        match ml_s37 {
                            self::MlReadGrammarLexerObligationPositionsJoin39::MlReadGrammarLexerObligationPositionsJoin39Next(reached_3, index_4, pending_3) => {
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
            self::MlReadGrammarLexerObligationPositionsLoop37Result::MlReadGrammarLexerObligationPositionsLoop37Done(reached, index)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_lazy_normalization_loop43(
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
pub fn ml_read_grammar_lexer_lazy_normalization_loop42(
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
                                    self::ml_read_grammar_lexer_lazy_normalization_loop43(
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
