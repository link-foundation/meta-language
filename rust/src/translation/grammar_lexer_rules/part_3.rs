// Generated from the complete shared grammar-lexer-rules.js and recorded source closure.
// Do not edit by hand.
use super::*;

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
                    self::read_required_text_fact(kinds, index),
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
    mut name: self::ml_string::Text,
    mut target: f64,
    mut found: bool,
) -> self::MlReadGrammarLexerLiteralExtensionLoop16Result {
    loop {
        return if target < count {
            {
                let ml_s20 = if ((self::read_grammar_lexer_entry_form(
                    self::read_required_text_fact(kinds, target),
                )) == 7f64)
                    && ((self::read_required_text_fact(names, target)) == name)
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
                match ml_s20 {
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
                    let ml_s22 = if ((self::read_grammar_lexer_entry_form(
                        self::read_required_text_fact(kinds, index),
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
                                            let ml_s23 = self::ml_read_grammar_lexer_literal_extension_loop16(kinds, names, count, identity, renames_2, name.clone(), target, found);
                                            match ml_s23 {
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
                    match ml_s22 {
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
                let form = self::read_grammar_lexer_entry_form(self::read_required_text_fact(
                    kinds, index,
                ));
                {
                    let ml_s24 = if form == 3f64 {
                        {
                            let position = self::read_grammar_lexer_text_position(
                                parser_literals,
                                self::read_required_text_fact(values, index),
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
                    match ml_s24 {
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
    mut owner: self::ml_string::Text,
    mut value: self::ml_string::Text,
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
                let form = self::read_grammar_lexer_entry_form(self::read_required_text_fact(
                    kinds, index,
                ));
                {
                    let name = self::read_required_text_fact(names, index);
                    {
                        let matching_owner_present_3 = if (form == 1f64)
                            && ((self::read_required_text_fact(targets, index)) == owner)
                        {
                            { true }
                        } else {
                            matching_owner_present
                        };
                        if (form == 3f64)
                            && ((self::read_required_text_fact(values, index)) == value)
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
                let form = self::read_grammar_lexer_entry_form(self::read_required_text_fact(
                    kinds, index,
                ));
                {
                    let name = self::read_required_text_fact(names, index);
                    if ((form == 7f64) || (form == 8f64))
                        && self::contains_grammar_lexer_text(originals, name.clone())
                    {
                        self::MlReadGrammarLexerOwnerComparisonLoop20Result::MlReadGrammarLexerOwnerComparisonLoop20Return(vec![-1f64])
                    } else {
                        {
                            let result_3 = if (form == 6f64)
                                && (name != (self::ml_string::Text::from_units(vec![])))
                            {
                                {
                                    let owner = self::read_grammar_lexer_text_position(
                                        originals,
                                        name.clone(),
                                    );
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
                let ml_s25 = if (self::read_grammar_lexer_entry_form(
                    self::read_required_text_fact(kinds, index),
                )) == 1f64
                {
                    {
                        let original = self::read_required_text_fact(names, index);
                        {
                            let target = self::read_required_text_fact(targets, index);
                            if (((original == (self::ml_string::Text::from_units(vec![])))
                                || (target == (self::ml_string::Text::from_units(vec![]))))
                                || self::contains_grammar_lexer_text(
                                    &(originals),
                                    original.clone(),
                                ))
                                || self::contains_grammar_lexer_text(&(publics), target.clone())
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
                match ml_s25 {
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
pub enum MlReadGrammarLexerCommandPlanJoin25 {
    MlReadGrammarLexerCommandPlanJoin25Next(self::ml_string::Text, self::ml_string::Text, bool),
}
