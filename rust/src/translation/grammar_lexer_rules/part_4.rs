// Generated from the complete shared grammar-lexer-rules.js and recorded source closure.
// Do not edit by hand.
use super::MlReadGrammarLexerOwnerComparisonJoin21;
use super::MlReadGrammarLexerOwnerComparisonLoop19Result;
use super::accept_grammar_lexer_decimal;
use super::accept_grammar_lexer_identifier;
use super::contains_grammar_lexer_text;
use super::ml_array;
use super::ml_string;
use super::read_grammar_lexer_entry_form;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_grammar_lexer_owner_comparison_loop19(
    kinds: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    targets: &[self::ml_string::Text],
    mut originals: Vec<self::ml_string::Text>,
    mut publics: Vec<self::ml_string::Text>,
    mut positions: Vec<f64>,
    mut result: Vec<f64>,
    mut index: f64,
) -> self::MlReadGrammarLexerOwnerComparisonLoop19Result {
    loop {
        return if index < (kinds.len() as f64) {
            {
                let ml_s35 = if (self::read_grammar_lexer_entry_form(
                    &(self::read_required_text_fact(kinds, index)),
                )) == 1f64
                {
                    {
                        let original = self::read_required_text_fact(names, index);
                        {
                            let target = self::read_required_text_fact(targets, index);
                            if (((original == (self::ml_string::Text::from_units(vec![])))
                                || (target == (self::ml_string::Text::from_units(vec![]))))
                                || self::contains_grammar_lexer_text(&(originals), &(original)))
                                || self::contains_grammar_lexer_text(&(publics), &(target))
                            {
                                self::MlReadGrammarLexerOwnerComparisonJoin21::MlReadGrammarLexerOwnerComparisonJoin21Return(vec![-1f64])
                            } else {
                                {
                                    let originals_2 = {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![original.clone()], originals);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let publics_2 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![target.clone()], publics);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let positions_2 = {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![index], positions);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let result_2 = {
                                                    let (ml_append_right, ml_append_left) =
                                                        (vec![index, index], result);
                                                    self::ml_array::append(
                                                        ml_append_left,
                                                        ml_append_right,
                                                    )
                                                };
                                                self::MlReadGrammarLexerOwnerComparisonJoin21::MlReadGrammarLexerOwnerComparisonJoin21Next(originals_2, publics_2, positions_2, result_2)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlReadGrammarLexerOwnerComparisonJoin21::MlReadGrammarLexerOwnerComparisonJoin21Next(originals, publics, positions, result)
                };
                match ml_s35 {
                    self::MlReadGrammarLexerOwnerComparisonJoin21::MlReadGrammarLexerOwnerComparisonJoin21Next(originals_3, publics_3, positions_3, result_3) => {
                        {
                            let index_2 = index + 1f64;
                            {
                                (originals, publics, positions, result, index) = (originals_3, publics_3, positions_3, result_3, index_2);
                                continue;
                            }
                        }
                    }
                    self::MlReadGrammarLexerOwnerComparisonJoin21::MlReadGrammarLexerOwnerComparisonJoin21Return(ml_result21) => {
                        self::MlReadGrammarLexerOwnerComparisonLoop19Result::MlReadGrammarLexerOwnerComparisonLoop19Return(ml_result21)
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerOwnerComparisonLoop19Result::MlReadGrammarLexerOwnerComparisonLoop19Done(originals, positions, result, index)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_binding_names_loop23(
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
                                if (self::contains_grammar_lexer_text(&(bindings), &(name))) {
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
                                        &(legacy),
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
                                                &(emitted),
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
pub enum MlReadGrammarLexerBindingNamesLoop22Result {
    MlReadGrammarLexerBindingNamesLoop22Done(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_binding_names_loop22(
    declared: &[self::ml_string::Text],
    references: &[self::ml_string::Text],
    mut bindings: Vec<self::ml_string::Text>,
    mut index: f64,
) -> self::MlReadGrammarLexerBindingNamesLoop22Result {
    loop {
        if index < (declared.len() as f64) {
            {
                let name = self::read_required_text_fact(declared, index);
                {
                    let bindings_3 = if self::contains_grammar_lexer_text(references, &(name))
                        && !(self::contains_grammar_lexer_text(&(bindings), &(name)))
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
            self::MlReadGrammarLexerBindingNamesLoop22Result::MlReadGrammarLexerBindingNamesLoop22Done(bindings, index)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarLexerCommandPlanLoop24Result {
    MlReadGrammarLexerCommandPlanLoop24Done(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        Vec<self::ml_string::Text>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarLexerCommandPlanJoin26 {
    MlReadGrammarLexerCommandPlanJoin26Next(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        Vec<self::ml_string::Text>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_command_plan_loop27(
    argument: &self::ml_string::Text,
    mut zero: bool,
    mut offset: f64,
) -> bool {
    loop {
        if offset < (argument.length()) {
            {
                let zero_3 = if (argument.slice(offset, (offset + 1f64)))
                    == (self::ml_string::Text::from_units(vec![48]))
                {
                    zero
                } else {
                    { false }
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
pub enum MlReadGrammarLexerCommandPlanJoin25 {
    MlReadGrammarLexerCommandPlanJoin25Next(self::ml_string::Text, self::ml_string::Text, bool),
}

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
                                    let ml_s36 = if (opening >= 0f64)
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
                                    match ml_s36 {
                                        self::MlReadGrammarLexerCommandPlanJoin25::MlReadGrammarLexerCommandPlanJoin25Next(head_3, argument_3, call_3) => {
                                            {
                                                let ml_s37 = if piece == (self::ml_string::Text::from_units(vec![115, 107, 105, 112])) {
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
                                                match ml_s37 {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarLexerCommandEmissionLoop31Result {
    MlReadGrammarLexerCommandEmissionLoop31Done,
    MlReadGrammarLexerCommandEmissionLoop31Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_command_emission_loop31(
    expected: &[self::ml_string::Text],
    mut index: f64,
    replayed: &[self::ml_string::Text],
) -> self::MlReadGrammarLexerCommandEmissionLoop31Result {
    loop {
        return if index < (expected.len() as f64) {
            if (self::read_required_text_fact(replayed, index))
                == (self::read_required_text_fact(expected, index))
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
                self::MlReadGrammarLexerCommandEmissionLoop31Result::MlReadGrammarLexerCommandEmissionLoop31Return(Vec::<self::ml_string::Text>::new())
            }
        } else {
            self::MlReadGrammarLexerCommandEmissionLoop31Result::MlReadGrammarLexerCommandEmissionLoop31Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerCommandEmissionLoop30Result {
    MlReadGrammarLexerCommandEmissionLoop30Done(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        f64,
    ),
    MlReadGrammarLexerCommandEmissionLoop30Return(Vec<self::ml_string::Text>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarLexerCommandEmissionJoin32 {
    MlReadGrammarLexerCommandEmissionJoin32Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerCommandEmissionJoin32Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_command_emission_loop30(
    operations: &[self::ml_string::Text],
    modes: &[self::ml_string::Text],
    mut pieces: Vec<self::ml_string::Text>,
    mut expected: Vec<self::ml_string::Text>,
    mut index: f64,
) -> self::MlReadGrammarLexerCommandEmissionLoop30Result {
    loop {
        return if index < (operations.len() as f64) {
            {
                let operation = self::read_required_text_fact(operations, index);
                {
                    let mode = self::read_required_text_fact(modes, index);
                    {
                        let ml_s38 = if operation
                            == (self::ml_string::Text::from_units(vec![
                                112, 111, 112, 77, 111, 100, 101,
                            ])) {
                            {
                                let ml_library_inequality_1 =
                                    mode != (self::ml_string::Text::from_units(vec![]));
                                if ml_library_inequality_1 {
                                    self::MlReadGrammarLexerCommandEmissionJoin32::MlReadGrammarLexerCommandEmissionJoin32Return(Vec::<self::ml_string::Text>::new())
                                } else {
                                    {
                                        let pieces_2 = {
                                            let (ml_append_right, ml_append_left) = (
                                                vec![self::ml_string::Text::from_units(vec![
                                                    112, 111, 112, 77, 111, 100, 101,
                                                ])],
                                                pieces,
                                            );
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        self::MlReadGrammarLexerCommandEmissionJoin32::MlReadGrammarLexerCommandEmissionJoin32Next(pieces_2)
                                    }
                                }
                            }
                        } else if (operation
                            == (self::ml_string::Text::from_units(vec![
                                112, 117, 115, 104, 77, 111, 100, 101,
                            ])))
                            || (operation
                                == (self::ml_string::Text::from_units(vec![
                                    115, 101, 116, 77, 111, 100, 101,
                                ])))
                        {
                            if (self::accept_grammar_lexer_identifier(&(mode))) {
                                {
                                    let argument = if mode
                                        == (self::ml_string::Text::from_units(vec![
                                            100, 101, 102, 97, 117, 108, 116,
                                        ])) {
                                        self::ml_string::Text::from_units(vec![
                                            68, 69, 70, 65, 85, 76, 84, 95, 77, 79, 68, 69,
                                        ])
                                    } else {
                                        mode.clone()
                                    };
                                    {
                                        let pieces_3 = {
                                            let (ml_append_right, ml_append_left) = (
                                                vec![
                                                    (((if operation
                                                        == (self::ml_string::Text::from_units(
                                                            vec![115, 101, 116, 77, 111, 100, 101],
                                                        ))
                                                    {
                                                        self::ml_string::Text::from_units(vec![
                                                            109, 111, 100, 101,
                                                        ])
                                                    } else {
                                                        self::ml_string::Text::from_units(vec![
                                                            112, 117, 115, 104, 77, 111, 100, 101,
                                                        ])
                                                    })
                                                    .concat(&self::ml_string::Text::from_units(
                                                        vec![40],
                                                    )))
                                                    .concat(&argument))
                                                    .concat(&self::ml_string::Text::from_units(
                                                        vec![41],
                                                    )),
                                                ],
                                                pieces,
                                            );
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        self::MlReadGrammarLexerCommandEmissionJoin32::MlReadGrammarLexerCommandEmissionJoin32Next(pieces_3)
                                    }
                                }
                            } else {
                                self::MlReadGrammarLexerCommandEmissionJoin32::MlReadGrammarLexerCommandEmissionJoin32Return(Vec::<self::ml_string::Text>::new())
                            }
                        } else {
                            self::MlReadGrammarLexerCommandEmissionJoin32::MlReadGrammarLexerCommandEmissionJoin32Return(Vec::<self::ml_string::Text>::new())
                        };
                        match ml_s38 {
                            self::MlReadGrammarLexerCommandEmissionJoin32::MlReadGrammarLexerCommandEmissionJoin32Next(pieces_4) => {
                                {
                                    let expected_2 = { let (ml_append_right, ml_append_left) = (vec![operation, mode], expected); self::ml_array::append(ml_append_left, ml_append_right) };
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (pieces, expected, index) = (pieces_4, expected_2, index_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                            self::MlReadGrammarLexerCommandEmissionJoin32::MlReadGrammarLexerCommandEmissionJoin32Return(ml_result32) => {
                                self::MlReadGrammarLexerCommandEmissionLoop30Result::MlReadGrammarLexerCommandEmissionLoop30Return(ml_result32)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerCommandEmissionLoop30Result::MlReadGrammarLexerCommandEmissionLoop30Done(pieces, expected, index)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarLexerCommandEmissionJoin29 {
    MlReadGrammarLexerCommandEmissionJoin29Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerCommandEmissionJoin29Return(Vec<self::ml_string::Text>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarLexerCommandEmissionJoin28 {
    MlReadGrammarLexerCommandEmissionJoin28Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerCommandEmissionJoin28Return(Vec<self::ml_string::Text>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarLexerEmissionIssueLoop37Result {
    MlReadGrammarLexerEmissionIssueLoop37Done,
    MlReadGrammarLexerEmissionIssueLoop37Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_emission_issue_loop37(
    references: &[self::ml_string::Text],
    mut index: f64,
    locals: &[self::ml_string::Text],
) -> self::MlReadGrammarLexerEmissionIssueLoop37Result {
    loop {
        return if index < (references.len() as f64) {
            if (self::contains_grammar_lexer_text(
                locals,
                &(self::read_required_text_fact(references, index)),
            )) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlReadGrammarLexerEmissionIssueLoop37Result::MlReadGrammarLexerEmissionIssueLoop37Return(self::ml_string::Text::from_units(vec![97, 110, 32, 117, 110, 114, 101, 115, 111, 108, 118, 101, 100, 32, 115, 111, 117, 114, 99, 101, 45, 108, 111, 99, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 114, 101, 109, 97, 105, 110, 115]))
            }
        } else {
            self::MlReadGrammarLexerEmissionIssueLoop37Result::MlReadGrammarLexerEmissionIssueLoop37Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerEmissionIssueLoop36Result {
    MlReadGrammarLexerEmissionIssueLoop36Done(f64),
    MlReadGrammarLexerEmissionIssueLoop36Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_emission_issue_loop36(
    rule_names: &[self::ml_string::Text],
    rule_kinds: &[self::ml_string::Text],
    mut index: f64,
    locals: &[self::ml_string::Text],
    owners: &[self::ml_string::Text],
) -> self::MlReadGrammarLexerEmissionIssueLoop36Result {
    loop {
        return if index < (rule_names.len() as f64) {
            {
                let name = self::read_required_text_fact(rule_names, index);
                {
                    let kind = self::read_required_text_fact(rule_kinds, index);
                    if (kind
                        != (self::ml_string::Text::from_units(vec![110, 111, 114, 109, 97, 108])))
                        && !(self::contains_grammar_lexer_text(locals, &(name)))
                    {
                        self::MlReadGrammarLexerEmissionIssueLoop36Result::MlReadGrammarLexerEmissionIssueLoop36Return(self::ml_string::Text::from_units(vec![105, 109, 112, 108, 105, 99, 105, 116, 32, 111, 114, 32, 100, 101, 99, 108, 97, 114, 101, 100, 45, 111, 110, 108, 121, 32, 116, 111, 107, 101, 110, 115, 32, 110, 101, 101, 100, 32, 97, 110, 32, 117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100, 32, 108, 101, 120, 101, 114, 32, 115, 101, 99, 116, 105, 111, 110]))
                    } else if (kind
                        == (self::ml_string::Text::from_units(vec![110, 111, 114, 109, 97, 108])))
                        && !(self::contains_grammar_lexer_text(owners, &(name)))
                    {
                        self::MlReadGrammarLexerEmissionIssueLoop36Result::MlReadGrammarLexerEmissionIssueLoop36Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 112, 97, 114, 115, 101, 114, 32, 114, 117, 108, 101, 32, 110, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 117, 110, 105, 113, 117, 101, 32, 108, 101, 120, 101, 114, 32, 111, 119, 110, 101, 114]))
                    } else {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerEmissionIssueLoop36Result::MlReadGrammarLexerEmissionIssueLoop36Done(index)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerEmissionIssueLoop35Result {
    MlReadGrammarLexerEmissionIssueLoop35Done(
        f64,
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
    ),
    MlReadGrammarLexerEmissionIssueLoop35Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarLexerEmissionIssueJoin39 {
    MlReadGrammarLexerEmissionIssueJoin39Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerEmissionIssueJoin39Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlReadGrammarLexerEmissionIssueJoin38 {
    MlReadGrammarLexerEmissionIssueJoin38Next(Vec<self::ml_string::Text>),
    MlReadGrammarLexerEmissionIssueJoin38Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_grammar_lexer_emission_issue_loop35(
    kinds: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    entry_modes: &[self::ml_string::Text],
    targets: &[self::ml_string::Text],
    public_kinds: &[self::ml_string::Text],
    public_names: &[self::ml_string::Text],
    mut count: f64,
    mut index: f64,
    mut locals: Vec<self::ml_string::Text>,
    mut owners: Vec<self::ml_string::Text>,
) -> self::MlReadGrammarLexerEmissionIssueLoop35Result {
    loop {
        return if index < count {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let name = self::read_required_text_fact(names, index);
                    {
                        let actual = self::read_required_text_fact(public_kinds, index);
                        {
                            let ml_s39 = if (kind
                                == (self::ml_string::Text::from_units(vec![
                                    108, 101, 120, 105, 99, 97, 108, 79, 119, 110, 101, 114,
                                ])))
                                || (kind
                                    == (self::ml_string::Text::from_units(vec![
                                        108, 101, 120, 105, 99, 97, 108, 66, 105, 110, 100, 105,
                                        110, 103,
                                    ]))) {
                                if ((self::read_required_text_fact(targets, index)) != name)
                                    || (actual == (self::ml_string::Text::from_units(vec![])))
                                {
                                    self::MlReadGrammarLexerEmissionIssueJoin38::MlReadGrammarLexerEmissionIssueJoin38Return(self::ml_string::Text::from_units(vec![112, 117, 98, 108, 105, 99, 32, 108, 101, 120, 105, 99, 97, 108, 32, 98, 105, 110, 100, 105, 110, 103, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 117, 110, 105, 113, 117, 101, 32, 115, 111, 117, 114, 99, 101, 32, 110, 97, 109, 101, 115]))
                                } else if kind
                                    == (self::ml_string::Text::from_units(vec![
                                        108, 101, 120, 105, 99, 97, 108, 79, 119, 110, 101, 114,
                                    ]))
                                {
                                    {
                                        let owners_2 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![name.clone()], owners);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        self::MlReadGrammarLexerEmissionIssueJoin38::MlReadGrammarLexerEmissionIssueJoin38Next(owners_2)
                                    }
                                } else {
                                    self::MlReadGrammarLexerEmissionIssueJoin38::MlReadGrammarLexerEmissionIssueJoin38Next(owners)
                                }
                            } else {
                                self::MlReadGrammarLexerEmissionIssueJoin38::MlReadGrammarLexerEmissionIssueJoin38Next(owners)
                            };
                            match ml_s39 {
                                self::MlReadGrammarLexerEmissionIssueJoin38::MlReadGrammarLexerEmissionIssueJoin38Next(owners_3) => {
                                    {
                                        let ml_s40 = if (kind == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 82, 117, 108, 101]))) || (kind == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 70, 114, 97, 103, 109, 101, 110, 116]))) {
                                            if ((self::contains_grammar_lexer_text(&(locals), &(name)) || ((self::read_required_text_fact(public_names, index)) != name)) || ((kind == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 82, 117, 108, 101]))) && (actual != (self::ml_string::Text::from_units(vec![116, 111, 107, 101, 110]))))) || ((kind == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 70, 114, 97, 103, 109, 101, 110, 116]))) && (actual != (self::ml_string::Text::from_units(vec![115, 105, 108, 101, 110, 116])))) {
                                                self::MlReadGrammarLexerEmissionIssueJoin39::MlReadGrammarLexerEmissionIssueJoin39Return(self::ml_string::Text::from_units(vec![101, 97, 99, 104, 32, 108, 101, 120, 105, 99, 97, 108, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 111, 110, 101, 32, 117, 110, 99, 104, 97, 110, 103, 101, 100, 32, 112, 117, 98, 108, 105, 99, 32, 116, 111, 107, 101, 110, 32, 111, 114, 32, 102, 114, 97, 103, 109, 101, 110, 116]))
                                            } else if (kind == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 82, 117, 108, 101]))) && ((self::read_required_text_fact(entry_modes, index)) != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) {
                                                    self::MlReadGrammarLexerEmissionIssueJoin39::MlReadGrammarLexerEmissionIssueJoin39Return(self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 100, 101, 102, 97, 117, 108, 116, 45, 109, 111, 100, 101, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 117, 108, 101, 115, 32, 97, 114, 101, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100]))
                                                } else {
                                                    {
                                                        let locals_2 = { let (ml_append_right, ml_append_left) = (vec![name.clone()], locals); self::ml_array::append(ml_append_left, ml_append_right) };
                                                        self::MlReadGrammarLexerEmissionIssueJoin39::MlReadGrammarLexerEmissionIssueJoin39Next(locals_2)
                                                    }
                                                }
                                        } else {
                                            self::MlReadGrammarLexerEmissionIssueJoin39::MlReadGrammarLexerEmissionIssueJoin39Next(locals)
                                        };
                                        match ml_s40 {
                                            self::MlReadGrammarLexerEmissionIssueJoin39::MlReadGrammarLexerEmissionIssueJoin39Next(locals_3) => {
                                                {
                                                    let index_2 = index + 1f64;
                                                    {
                                                        (index, locals, owners) = (index_2, locals_3, owners_3);
                                                        continue;
                                                    }
                                                }
                                            }
                                            self::MlReadGrammarLexerEmissionIssueJoin39::MlReadGrammarLexerEmissionIssueJoin39Return(ml_result39) => {
                                                self::MlReadGrammarLexerEmissionIssueLoop35Result::MlReadGrammarLexerEmissionIssueLoop35Return(ml_result39)
                                            }
                                        }
                                    }
                                }
                                self::MlReadGrammarLexerEmissionIssueJoin38::MlReadGrammarLexerEmissionIssueJoin38Return(ml_result39_2) => {
                                    self::MlReadGrammarLexerEmissionIssueLoop35Result::MlReadGrammarLexerEmissionIssueLoop35Return(ml_result39_2)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerEmissionIssueLoop35Result::MlReadGrammarLexerEmissionIssueLoop35Done(index, locals, owners)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerEmissionIssueLoop34Result {
    MlReadGrammarLexerEmissionIssueLoop34Done(f64),
    MlReadGrammarLexerEmissionIssueLoop34Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_lexer_emission_issue_loop34(
    channels: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlReadGrammarLexerEmissionIssueLoop34Result {
    loop {
        return if index < (channels.len() as f64) {
            {
                let channel = self::read_required_text_fact(channels, index);
                if (channel
                    != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])))
                    && (channel
                        != (self::ml_string::Text::from_units(vec![72, 73, 68, 68, 69, 78])))
                {
                    self::MlReadGrammarLexerEmissionIssueLoop34Result::MlReadGrammarLexerEmissionIssueLoop34Return(self::ml_string::Text::from_units(vec![99, 117, 115, 116, 111, 109, 32, 99, 104, 97, 110, 110, 101, 108, 32, 115, 101, 99, 116, 105, 111, 110, 115, 32, 97, 114, 101, 32, 110, 111, 116, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100]))
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarLexerEmissionIssueLoop34Result::MlReadGrammarLexerEmissionIssueLoop34Done(index)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarLexerEmissionIssueLoop33Result {
    MlReadGrammarLexerEmissionIssueLoop33Done(f64),
    MlReadGrammarLexerEmissionIssueLoop33Return(self::ml_string::Text),
}
