// Generated from the complete shared grammar-lexer-rules.js and recorded source closure.
// Do not edit by hand.
use super::MlReadGrammarLexerEmissionIssueLoop33Result;
use super::accept_grammar_lexer_decimal;
use super::contains_grammar_lexer_position;
use super::contains_grammar_lexer_text;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_grammar_lexer_entry_form;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_emission_issue_loop33(
    modes: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlReadGrammarLexerEmissionIssueLoop33Result {
    loop {
        return if index < (modes.len() as f64) {
            if (self::read_required_text_fact(modes, index))
                == (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))
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
                self::MlReadGrammarLexerEmissionIssueLoop33Result::MlReadGrammarLexerEmissionIssueLoop33Return(self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 116, 104, 101, 32, 100, 101, 102, 97, 117, 108, 116, 32, 108, 101, 120, 101, 114, 32, 109, 111, 100, 101, 32, 105, 115, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100]))
            }
        } else {
            self::MlReadGrammarLexerEmissionIssueLoop33Result::MlReadGrammarLexerEmissionIssueLoop33Done(index)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueLoop41Result {
    MlReadGrammarLexerDeclarationIssueLoop41Done,
    MlReadGrammarLexerDeclarationIssueLoop41Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin42 {
    MlReadGrammarLexerDeclarationIssueJoin42Next,
    MlReadGrammarLexerDeclarationIssueJoin42Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueLoop43Result {
    MlReadGrammarLexerDeclarationIssueLoop43Done,
    MlReadGrammarLexerDeclarationIssueLoop43Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin44 {
    MlReadGrammarLexerDeclarationIssueJoin44Next,
    MlReadGrammarLexerDeclarationIssueJoin44Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_declaration_issue_loop43(
    action_owners: &[f64],
    actions: &[self::ml_string::Text],
    action_modes: &[self::ml_string::Text],
    declared_modes: &[self::ml_string::Text],
    mut index: f64,
    mut action: f64,
) -> self::MlReadGrammarLexerDeclarationIssueLoop43Result {
    loop {
        return if action < (actions.len() as f64) {
            {
                let ml_s41 = if (self::read_required_number_fact(action_owners, action)) == index {
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
                            self::MlReadGrammarLexerDeclarationIssueJoin44::MlReadGrammarLexerDeclarationIssueJoin44Return(vec![9f64, index, action])
                        } else if (operation
                            != (self::ml_string::Text::from_units(vec![
                                112, 111, 112, 77, 111, 100, 101,
                            ])))
                            && !(self::contains_grammar_lexer_text(
                                declared_modes,
                                &(self::read_required_text_fact(action_modes, action)),
                            ))
                        {
                            self::MlReadGrammarLexerDeclarationIssueJoin44::MlReadGrammarLexerDeclarationIssueJoin44Return(vec![10f64, index, action])
                        } else {
                            self::MlReadGrammarLexerDeclarationIssueJoin44::MlReadGrammarLexerDeclarationIssueJoin44Next
                        }
                    }
                } else {
                    self::MlReadGrammarLexerDeclarationIssueJoin44::MlReadGrammarLexerDeclarationIssueJoin44Next
                };
                match ml_s41 {
                    self::MlReadGrammarLexerDeclarationIssueJoin44::MlReadGrammarLexerDeclarationIssueJoin44Next => {
                        {
                            let action_2 = action + 1f64;
                            {
                                action = action_2;
                                continue;
                            }
                        }
                    }
                    self::MlReadGrammarLexerDeclarationIssueJoin44::MlReadGrammarLexerDeclarationIssueJoin44Return(ml_result44) => {
                        self::MlReadGrammarLexerDeclarationIssueLoop43Result::MlReadGrammarLexerDeclarationIssueLoop43Return(ml_result44)
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerDeclarationIssueLoop43Result::MlReadGrammarLexerDeclarationIssueLoop43Done
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
pub fn ml_read_grammar_lexer_declaration_issue_loop41(
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
) -> self::MlReadGrammarLexerDeclarationIssueLoop41Result {
    loop {
        return if index < count {
            {
                let ml_s43 = if (self::read_grammar_lexer_entry_form(
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
                                    self::MlReadGrammarLexerDeclarationIssueJoin42::MlReadGrammarLexerDeclarationIssueJoin42Return(vec![11f64, index, -1f64])
                                } else {
                                    {
                                        let action = 0f64;
                                        {
                                            let ml_s44 = self::ml_read_grammar_lexer_declaration_issue_loop43(action_owners, actions, action_modes, declared_modes, index, action);
                                            match ml_s44 {
                                                self::MlReadGrammarLexerDeclarationIssueLoop43Result::MlReadGrammarLexerDeclarationIssueLoop43Done => {
                                                    self::MlReadGrammarLexerDeclarationIssueJoin42::MlReadGrammarLexerDeclarationIssueJoin42Next
                                                }
                                                self::MlReadGrammarLexerDeclarationIssueLoop43Result::MlReadGrammarLexerDeclarationIssueLoop43Return(ml_result43) => {
                                                    self::MlReadGrammarLexerDeclarationIssueJoin42::MlReadGrammarLexerDeclarationIssueJoin42Return(ml_result43)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlReadGrammarLexerDeclarationIssueJoin42::MlReadGrammarLexerDeclarationIssueJoin42Return(vec![8f64, index, -1f64])
                        }
                    } else {
                        self::MlReadGrammarLexerDeclarationIssueJoin42::MlReadGrammarLexerDeclarationIssueJoin42Return(vec![7f64, index, -1f64])
                    }
                } else {
                    self::MlReadGrammarLexerDeclarationIssueJoin42::MlReadGrammarLexerDeclarationIssueJoin42Next
                };
                match ml_s43 {
                    self::MlReadGrammarLexerDeclarationIssueJoin42::MlReadGrammarLexerDeclarationIssueJoin42Next => {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                    self::MlReadGrammarLexerDeclarationIssueJoin42::MlReadGrammarLexerDeclarationIssueJoin42Return(ml_result42) => {
                        self::MlReadGrammarLexerDeclarationIssueLoop41Result::MlReadGrammarLexerDeclarationIssueLoop41Return(ml_result42)
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerDeclarationIssueLoop41Result::MlReadGrammarLexerDeclarationIssueLoop41Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueLoop40Result {
    MlReadGrammarLexerDeclarationIssueLoop40Done(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        f64,
    ),
    MlReadGrammarLexerDeclarationIssueLoop40Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin48 {
    MlReadGrammarLexerDeclarationIssueJoin48Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerDeclarationIssueJoin48Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin47 {
    MlReadGrammarLexerDeclarationIssueJoin47Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerDeclarationIssueJoin47Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin46 {
    MlReadGrammarLexerDeclarationIssueJoin46Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerDeclarationIssueJoin46Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerDeclarationIssueJoin45 {
    MlReadGrammarLexerDeclarationIssueJoin45Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerDeclarationIssueJoin45Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_grammar_lexer_declaration_issue_loop40(
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
) -> self::MlReadGrammarLexerDeclarationIssueLoop40Result {
    loop {
        return if index < count {
            {
                let form = self::read_grammar_lexer_entry_form(
                    &(self::read_required_text_fact(kinds, index)),
                );
                if form == 0f64 {
                    self::MlReadGrammarLexerDeclarationIssueLoop40Result::MlReadGrammarLexerDeclarationIssueLoop40Return(vec![2f64, index, -1f64])
                } else {
                    {
                        let ml_s45 = if form == 1f64 {
                            if ((self::read_required_text_fact(items, index))
                                != (self::ml_string::Text::from_units(vec![114, 101, 102])))
                                || self::contains_grammar_lexer_text(
                                    &(owner_names),
                                    &(self::read_required_text_fact(names, index)),
                                )
                            {
                                self::MlReadGrammarLexerDeclarationIssueJoin45::MlReadGrammarLexerDeclarationIssueJoin45Return(vec![3f64, index, -1f64])
                            } else {
                                {
                                    let owner_names_2 = {
                                        let (ml_append_right, ml_append_left) = (
                                            vec![self::read_required_text_fact(names, index)],
                                            owner_names,
                                        );
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    self::MlReadGrammarLexerDeclarationIssueJoin45::MlReadGrammarLexerDeclarationIssueJoin45Next(owner_names_2)
                                }
                            }
                        } else {
                            self::MlReadGrammarLexerDeclarationIssueJoin45::MlReadGrammarLexerDeclarationIssueJoin45Next(owner_names)
                        };
                        match ml_s45 {
                            self::MlReadGrammarLexerDeclarationIssueJoin45::MlReadGrammarLexerDeclarationIssueJoin45Next(owner_names_3) => {
                                {
                                    let ml_s46 = if form == 2f64 {
                                        if ((self::read_required_text_fact(items, index)) != (self::ml_string::Text::from_units(vec![114, 101, 102]))) || self::contains_grammar_lexer_text(&(binding_names), &(self::read_required_text_fact(names, index))) {
                                            self::MlReadGrammarLexerDeclarationIssueJoin46::MlReadGrammarLexerDeclarationIssueJoin46Return(vec![4f64, index, -1f64])
                                        } else {
                                            {
                                                let binding_names_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_text_fact(names, index)], binding_names); self::ml_array::append(ml_append_left, ml_append_right) };
                                                self::MlReadGrammarLexerDeclarationIssueJoin46::MlReadGrammarLexerDeclarationIssueJoin46Next(binding_names_2)
                                            }
                                        }
                                    } else {
                                        self::MlReadGrammarLexerDeclarationIssueJoin46::MlReadGrammarLexerDeclarationIssueJoin46Next(binding_names)
                                    };
                                    match ml_s46 {
                                        self::MlReadGrammarLexerDeclarationIssueJoin46::MlReadGrammarLexerDeclarationIssueJoin46Next(binding_names_3) => {
                                            {
                                                let ml_s47 = if form == 3f64 {
                                                    if self::contains_grammar_lexer_text(&(literal_values), &(self::read_required_text_fact(values, index))) {
                                                        self::MlReadGrammarLexerDeclarationIssueJoin47::MlReadGrammarLexerDeclarationIssueJoin47Return(vec![5f64, index, -1f64])
                                                    } else {
                                                        {
                                                            let literal_values_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_text_fact(values, index)], literal_values); self::ml_array::append(ml_append_left, ml_append_right) };
                                                            self::MlReadGrammarLexerDeclarationIssueJoin47::MlReadGrammarLexerDeclarationIssueJoin47Next(literal_values_2)
                                                        }
                                                    }
                                                } else {
                                                    self::MlReadGrammarLexerDeclarationIssueJoin47::MlReadGrammarLexerDeclarationIssueJoin47Next(literal_values)
                                                };
                                                match ml_s47 {
                                                    self::MlReadGrammarLexerDeclarationIssueJoin47::MlReadGrammarLexerDeclarationIssueJoin47Next(literal_values_3) => {
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
                                                                    let ml_s48 = if (form == 7f64) || (form == 8f64) {
                                                                        if self::contains_grammar_lexer_text(&(local_names), &(self::read_required_text_fact(names, index))) {
                                                                            self::MlReadGrammarLexerDeclarationIssueJoin48::MlReadGrammarLexerDeclarationIssueJoin48Return(vec![6f64, index, -1f64])
                                                                        } else {
                                                                            {
                                                                                let local_names_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_required_text_fact(names, index)], local_names); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                self::MlReadGrammarLexerDeclarationIssueJoin48::MlReadGrammarLexerDeclarationIssueJoin48Next(local_names_2)
                                                                            }
                                                                        }
                                                                    } else {
                                                                        self::MlReadGrammarLexerDeclarationIssueJoin48::MlReadGrammarLexerDeclarationIssueJoin48Next(local_names)
                                                                    };
                                                                    match ml_s48 {
                                                                        self::MlReadGrammarLexerDeclarationIssueJoin48::MlReadGrammarLexerDeclarationIssueJoin48Next(local_names_3) => {
                                                                            {
                                                                                let index_2 = index + 1f64;
                                                                                {
                                                                                    (binding_names, owner_names, literal_values, local_names, declared_modes, declared_channels, index) = (binding_names_3, owner_names_3, literal_values_3, local_names_3, declared_modes_3, declared_channels_3, index_2);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlReadGrammarLexerDeclarationIssueJoin48::MlReadGrammarLexerDeclarationIssueJoin48Return(ml_result48) => {
                                                                            self::MlReadGrammarLexerDeclarationIssueLoop40Result::MlReadGrammarLexerDeclarationIssueLoop40Return(ml_result48)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlReadGrammarLexerDeclarationIssueJoin47::MlReadGrammarLexerDeclarationIssueJoin47Return(ml_result48_2) => {
                                                        self::MlReadGrammarLexerDeclarationIssueLoop40Result::MlReadGrammarLexerDeclarationIssueLoop40Return(ml_result48_2)
                                                    }
                                                }
                                            }
                                        }
                                        self::MlReadGrammarLexerDeclarationIssueJoin46::MlReadGrammarLexerDeclarationIssueJoin46Return(ml_result48_3) => {
                                            self::MlReadGrammarLexerDeclarationIssueLoop40Result::MlReadGrammarLexerDeclarationIssueLoop40Return(ml_result48_3)
                                        }
                                    }
                                }
                            }
                            self::MlReadGrammarLexerDeclarationIssueJoin45::MlReadGrammarLexerDeclarationIssueJoin45Return(ml_result48_4) => {
                                self::MlReadGrammarLexerDeclarationIssueLoop40Result::MlReadGrammarLexerDeclarationIssueLoop40Return(ml_result48_4)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerDeclarationIssueLoop40Result::MlReadGrammarLexerDeclarationIssueLoop40Done(declared_modes, declared_channels, index)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_obligation_positions_loop50(
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
pub enum MlReadGrammarLexerObligationPositionsLoop49Result {
    MlReadGrammarLexerObligationPositionsLoop49Done(Vec<f64>, f64),
    MlReadGrammarLexerObligationPositionsLoop49Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerObligationPositionsJoin51 {
    MlReadGrammarLexerObligationPositionsJoin51Next(Vec<f64>, f64, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerObligationPositionsLoop52Result {
    MlReadGrammarLexerObligationPositionsLoop52Done(f64, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_obligation_positions_loop52(
    edge_sources: &[f64],
    edge_targets: &[f64],
    mut index: f64,
    mut pending: Vec<f64>,
    mut current: f64,
) -> self::MlReadGrammarLexerObligationPositionsLoop52Result {
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
            self::MlReadGrammarLexerObligationPositionsLoop52Result::MlReadGrammarLexerObligationPositionsLoop52Done(index, pending)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_obligation_positions_loop49(
    mut count: f64,
    edge_sources: &[f64],
    edge_targets: &[f64],
    mut reached: Vec<f64>,
    mut index: f64,
    mut pending: Vec<f64>,
    mut cursor: f64,
) -> self::MlReadGrammarLexerObligationPositionsLoop49Result {
    loop {
        return if cursor < (pending.len() as f64) {
            {
                let current = self::read_required_number_fact(&(pending), cursor);
                if (!(self::ml_math::is_safe_integer(current)) || (current < 0f64))
                    || (current >= count)
                {
                    self::MlReadGrammarLexerObligationPositionsLoop49Result::MlReadGrammarLexerObligationPositionsLoop49Return(vec![-1f64])
                } else {
                    {
                        let ml_s50 = if (self::contains_grammar_lexer_position(&(reached), current))
                        {
                            self::MlReadGrammarLexerObligationPositionsJoin51::MlReadGrammarLexerObligationPositionsJoin51Next(reached, index, pending)
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
                                        let ml_s51 =
                                            self::ml_read_grammar_lexer_obligation_positions_loop52(
                                                edge_sources,
                                                edge_targets,
                                                index_2,
                                                pending,
                                                current,
                                            );
                                        match ml_s51 {
                                            self::MlReadGrammarLexerObligationPositionsLoop52Result::MlReadGrammarLexerObligationPositionsLoop52Done(index_3, pending_2) => {
                                                self::MlReadGrammarLexerObligationPositionsJoin51::MlReadGrammarLexerObligationPositionsJoin51Next(reached_2, index_3, pending_2)
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        match ml_s50 {
                            self::MlReadGrammarLexerObligationPositionsJoin51::MlReadGrammarLexerObligationPositionsJoin51Next(reached_3, index_4, pending_3) => {
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
            self::MlReadGrammarLexerObligationPositionsLoop49Result::MlReadGrammarLexerObligationPositionsLoop49Done(reached, index)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_lazy_normalization_loop55(
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
pub fn ml_read_grammar_lexer_lazy_normalization_loop54(
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
                                    self::ml_read_grammar_lexer_lazy_normalization_loop55(
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_grammar_lexer_lazy_normalization_loop53(
    kinds: &[self::ml_string::Text],
    labels: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    inner: &[f64],
    edge_sources: &[f64],
    edge_targets: &[f64],
    mut count: f64,
    mut captures: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < count {
            {
                let captures_3 = if ((self::read_required_text_fact(kinds, index))
                    == (self::ml_string::Text::from_units(vec![99, 97, 112, 116, 117, 114, 101])))
                    && ((self::read_required_text_fact(labels, index))
                        == (self::ml_string::Text::from_units(vec![
                            110, 111, 110, 95, 103, 114, 101, 101, 100, 121,
                        ]))) {
                    {
                        let delimiter = -1f64;
                        {
                            let repeat = self::read_required_number_fact(inner, index);
                            {
                                let delimiter_3 = if ((repeat >= 0f64) && (repeat < count))
                                    && ((self::read_required_text_fact(kinds, repeat))
                                        == (self::ml_string::Text::from_units(vec![
                                            114, 101, 112, 101, 97, 116, 48,
                                        ]))) {
                                    {
                                        let atom = self::read_required_number_fact(inner, repeat);
                                        if ((atom >= 0f64) && (atom < count))
                                            && ((self::read_required_text_fact(kinds, atom))
                                                == (self::ml_string::Text::from_units(vec![
                                                    97, 110, 121,
                                                ])))
                                        {
                                            {
                                                let owner = 0f64;
                                                {
                                                    self::ml_read_grammar_lexer_lazy_normalization_loop54(kinds, texts, edge_sources, edge_targets, count, index, delimiter, owner)
                                                }
                                            }
                                        } else {
                                            delimiter
                                        }
                                    }
                                } else {
                                    delimiter
                                };
                                {
                                    {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![index, delimiter_3], captures);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    }
                                }
                            }
                        }
                    }
                } else {
                    captures
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (captures, index) = (captures_3, index_2);
                        continue;
                    }
                }
            }
        }
        return { captures };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptGrammarParserTokenNegationLoop56Result {
    MlAcceptGrammarParserTokenNegationLoop56Done,
    MlAcceptGrammarParserTokenNegationLoop56Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_grammar_parser_token_negation_loop56(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlAcceptGrammarParserTokenNegationLoop56Result {
    loop {
        return if index < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    if (kind
                        == (self::ml_string::Text::from_units(vec![
                            108, 105, 116, 101, 114, 97, 108,
                        ])))
                        && ((value.length()) > 0f64)
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
                        if ((kind == (self::ml_string::Text::from_units(vec![114, 101, 102])))
                            && ((value.length()) > 0f64))
                            && (self::ml_string::Text::from_units(vec![
                                65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81,
                                82, 83, 84, 85, 86, 87, 88, 89, 90,
                            ]))
                            .contains(&value.slice(0f64, 1f64))
                        {
                            {
                                let index_3 = index + 1f64;
                                {
                                    index = index_3;
                                    continue;
                                }
                            }
                        }
                        {
                            if (kind == (self::ml_string::Text::from_units(vec![110, 111, 116])))
                                && (value
                                    == (self::ml_string::Text::from_units(vec![97, 110, 121])))
                            {
                                {
                                    let index_4 = index + 1f64;
                                    {
                                        index = index_4;
                                        continue;
                                    }
                                }
                            }
                            {
                                self::MlAcceptGrammarParserTokenNegationLoop56Result::MlAcceptGrammarParserTokenNegationLoop56Return(false)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptGrammarParserTokenNegationLoop56Result::MlAcceptGrammarParserTokenNegationLoop56Done
        };
    }
}
