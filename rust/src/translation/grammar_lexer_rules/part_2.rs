// Generated from the complete shared grammar-lexer-rules.js and recorded source closure.
// Do not edit by hand.
use super::MlAcceptGrammarParserTokenNegationLoop56Result;
use super::MlReadGrammarLexerBindingNamesLoop22Result;
use super::MlReadGrammarLexerCommandEmissionJoin28;
use super::MlReadGrammarLexerCommandEmissionJoin29;
use super::MlReadGrammarLexerCommandEmissionLoop30Result;
use super::MlReadGrammarLexerCommandEmissionLoop31Result;
use super::MlReadGrammarLexerCommandPlanLoop24Result;
use super::MlReadGrammarLexerDeclarationIssueLoop40Result;
use super::MlReadGrammarLexerDeclarationIssueLoop41Result;
use super::MlReadGrammarLexerEmissionIssueLoop33Result;
use super::MlReadGrammarLexerEmissionIssueLoop34Result;
use super::MlReadGrammarLexerEmissionIssueLoop35Result;
use super::MlReadGrammarLexerEmissionIssueLoop36Result;
use super::MlReadGrammarLexerEmissionIssueLoop37Result;
use super::MlReadGrammarLexerLiteralExtensionLoop10Result;
use super::MlReadGrammarLexerLiteralExtensionLoop11Result;
use super::MlReadGrammarLexerLiteralExtensionLoop12Result;
use super::MlReadGrammarLexerLiteralExtensionLoop13Result;
use super::MlReadGrammarLexerObligationPositionsLoop49Result;
use super::MlReadGrammarLexerOwnerComparisonLoop19Result;
use super::MlReadGrammarLexerOwnerComparisonLoop20Result;
use super::accept_grammar_lexer_decimal;
use super::accept_grammar_lexer_identifier;
use super::ml_accept_grammar_parser_token_negation_loop56;
use super::ml_array;
use super::ml_math;
use super::ml_read_grammar_lexer_binding_names_loop22;
use super::ml_read_grammar_lexer_binding_names_loop23;
use super::ml_read_grammar_lexer_command_emission_loop30;
use super::ml_read_grammar_lexer_command_emission_loop31;
use super::ml_read_grammar_lexer_command_plan_loop24;
use super::ml_read_grammar_lexer_declaration_issue_loop40;
use super::ml_read_grammar_lexer_declaration_issue_loop41;
use super::ml_read_grammar_lexer_emission_issue_loop33;
use super::ml_read_grammar_lexer_emission_issue_loop34;
use super::ml_read_grammar_lexer_emission_issue_loop35;
use super::ml_read_grammar_lexer_emission_issue_loop36;
use super::ml_read_grammar_lexer_emission_issue_loop37;
use super::ml_read_grammar_lexer_lazy_normalization_loop53;
use super::ml_read_grammar_lexer_literal_extension_loop10;
use super::ml_read_grammar_lexer_literal_extension_loop11;
use super::ml_read_grammar_lexer_literal_extension_loop12;
use super::ml_read_grammar_lexer_literal_extension_loop13;
use super::ml_read_grammar_lexer_literal_extension_loop14;
use super::ml_read_grammar_lexer_obligation_positions_loop49;
use super::ml_read_grammar_lexer_obligation_positions_loop50;
use super::ml_read_grammar_lexer_owner_comparison_loop19;
use super::ml_read_grammar_lexer_owner_comparison_loop20;
use super::ml_string;
use super::read_grammar_lexer_literal_rule_position;
use super::read_grammar_lexer_text_position;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_grammar_lexer_literal_extension(
    owner: &self::ml_string::Text,
    value: &self::ml_string::Text,
    kinds: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    targets: &[self::ml_string::Text],
    item_kinds: &[self::ml_string::Text],
    item_values: &[self::ml_string::Text],
    parser_literals: &[self::ml_string::Text],
) -> Vec<f64> {
    {
        let count = kinds.len() as f64;
        if (((((names.len() as f64) != count) || ((values.len() as f64) != count))
            || ((targets.len() as f64) != count))
            || ((item_kinds.len() as f64) != count))
            || ((item_values.len() as f64) != count)
        {
            vec![-1f64]
        } else {
            {
                let matching_owner_present = false;
                {
                    let index = 0f64;
                    {
                        let candidates = Vec::<self::ml_string::Text>::new();
                        {
                            let literals = Vec::<bool>::new();
                            {
                                let ml_s8 = self::ml_read_grammar_lexer_literal_extension_loop10(
                                    owner,
                                    value,
                                    kinds,
                                    names,
                                    values,
                                    targets,
                                    item_kinds,
                                    count,
                                    matching_owner_present,
                                    index,
                                    candidates,
                                    literals,
                                );
                                match ml_s8 {
                                    self::MlReadGrammarLexerLiteralExtensionLoop10Result::MlReadGrammarLexerLiteralExtensionLoop10Done(matching_owner_present_2, index_2, candidates_2, literals_2) => {
                                        if !matching_owner_present_2 || ((*value) == (self::ml_string::Text::from_units(vec![]))) {
                                            vec![0f64, -1f64, -1f64]
                                        } else {
                                            {
                                                let order = self::read_grammar_lexer_text_position(parser_literals, value);
                                                if order < 0f64 {
                                                    vec![-1f64]
                                                } else {
                                                    {
                                                        let selected = self::read_grammar_lexer_literal_rule_position(value, &(self::ml_string::Text::from_units(vec![99, 111, 109, 98, 105, 110, 101, 100])), &(candidates_2), item_values, &(literals_2));
                                                        {
                                                            let insertion = count;
                                                            {
                                                                let index_3 = 0f64;
                                                                {
                                                                    let ml_s9 = self::ml_read_grammar_lexer_literal_extension_loop11(kinds, names, values, parser_literals, count, index_3, order, insertion);
                                                                    match ml_s9 {
                                                                        self::MlReadGrammarLexerLiteralExtensionLoop11Result::MlReadGrammarLexerLiteralExtensionLoop11Done(index_4, insertion_2) => {
                                                                            if selected >= 0f64 {
                                                                                vec![2f64, insertion_2, selected]
                                                                            } else if selected == -2f64 {
                                                                                    vec![3f64, insertion_2, -1f64]
                                                                                } else {
                                                                                    {
                                                                                        let identity = 0f64;
                                                                                        {
                                                                                            let renames = Vec::<f64>::new();
                                                                                            {
                                                                                                let index_5 = 0f64;
                                                                                                {
                                                                                                    let ml_s10 = self::ml_read_grammar_lexer_literal_extension_loop12(kinds, names, count, index_5, insertion_2, identity, renames);
                                                                                                    match ml_s10 {
                                                                                                        self::MlReadGrammarLexerLiteralExtensionLoop12Result::MlReadGrammarLexerLiteralExtensionLoop12Done(index_6, renames_2) => {
                                                                                                            {
                                                                                                                let preceding = 0f64;
                                                                                                                {
                                                                                                                    let index_7 = 0f64;
                                                                                                                    {
                                                                                                                        let ml_s11 = self::ml_read_grammar_lexer_literal_extension_loop13(kinds, names, index_7, insertion_2, preceding);
                                                                                                                        match ml_s11 {
                                                                                                                            self::MlReadGrammarLexerLiteralExtensionLoop13Result::MlReadGrammarLexerLiteralExtensionLoop13Done(index_8, preceding_2) => {
                                                                                                                                {
                                                                                                                                    let result = vec![1f64, insertion_2, preceding_2];
                                                                                                                                    {
                                                                                                                                        let index_9 = 0f64;
                                                                                                                                        {
                                                                                                                                            self::ml_read_grammar_lexer_literal_extension_loop14(index_9, &(renames_2), result)
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlReadGrammarLexerLiteralExtensionLoop12Result::MlReadGrammarLexerLiteralExtensionLoop12Return(ml_result14) => {
                                                                                                            ml_result14
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                        }
                                                                        self::MlReadGrammarLexerLiteralExtensionLoop11Result::MlReadGrammarLexerLiteralExtensionLoop11Return(ml_result17) => {
                                                                            ml_result17
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
                                    self::MlReadGrammarLexerLiteralExtensionLoop10Result::MlReadGrammarLexerLiteralExtensionLoop10Return(ml_result18) => {
                                        ml_result18
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
pub fn read_grammar_lexer_owner_comparison(
    kinds: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    targets: &[self::ml_string::Text],
) -> Vec<f64> {
    if ((kinds.len() as f64) != (names.len() as f64))
        || ((kinds.len() as f64) != (targets.len() as f64))
    {
        vec![-1f64]
    } else {
        {
            let originals = Vec::<self::ml_string::Text>::new();
            {
                let publics = Vec::<self::ml_string::Text>::new();
                {
                    let positions = Vec::<f64>::new();
                    {
                        let result = Vec::<f64>::new();
                        {
                            let index = 0f64;
                            {
                                let ml_s12 = self::ml_read_grammar_lexer_owner_comparison_loop19(
                                    kinds, names, targets, originals, publics, positions, result,
                                    index,
                                );
                                match ml_s12 {
                                    self::MlReadGrammarLexerOwnerComparisonLoop19Result::MlReadGrammarLexerOwnerComparisonLoop19Done(originals_2, positions_2, result_2, index_2) => {
                                        {
                                            let index_3 = 0f64;
                                            {
                                                let ml_s13 = self::ml_read_grammar_lexer_owner_comparison_loop20(kinds, names, &(originals_2), &(positions_2), result_2, index_3);
                                                match ml_s13 {
                                                    self::MlReadGrammarLexerOwnerComparisonLoop20Result::MlReadGrammarLexerOwnerComparisonLoop20Done(result_3) => {
                                                        result_3
                                                    }
                                                    self::MlReadGrammarLexerOwnerComparisonLoop20Result::MlReadGrammarLexerOwnerComparisonLoop20Return(ml_result20) => {
                                                        ml_result20
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlReadGrammarLexerOwnerComparisonLoop19Result::MlReadGrammarLexerOwnerComparisonLoop19Return(ml_result20_2) => {
                                        ml_result20_2
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
pub fn read_grammar_lexer_binding_names(
    declared: &[self::ml_string::Text],
    references: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    legacy_types: &[self::ml_string::Text],
    emitted_types: &[self::ml_string::Text],
    dispositions: &[self::ml_string::Text],
) -> Vec<self::ml_string::Text> {
    {
        let bindings = Vec::<self::ml_string::Text>::new();
        {
            let index = 0f64;
            {
                let ml_s14 = self::ml_read_grammar_lexer_binding_names_loop22(
                    declared, references, bindings, index,
                );
                match ml_s14 {
                    self::MlReadGrammarLexerBindingNamesLoop22Result::MlReadGrammarLexerBindingNamesLoop22Done(bindings_2, index_2) => {
                        {
                            let index_3 = 0f64;
                            {
                                self::ml_read_grammar_lexer_binding_names_loop23(names, legacy_types, emitted_types, dispositions, bindings_2, index_3)
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
pub fn read_grammar_lexer_command_plan(
    command: &self::ml_string::Text,
    name: &self::ml_string::Text,
) -> Vec<self::ml_string::Text> {
    {
        let body = command.clone();
        {
            let body_3 = if body.starts_with(&self::ml_string::Text::from_units(vec![45, 62])) {
                { body.slice(2f64, f64::INFINITY) }
            } else {
                body
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
                                        let ml_s15 =
                                            self::ml_read_grammar_lexer_command_plan_loop24(
                                                type_,
                                                channel,
                                                disposition,
                                                actions,
                                                &(pieces),
                                                index,
                                            );
                                        match ml_s15 {
                                            self::MlReadGrammarLexerCommandPlanLoop24Result::MlReadGrammarLexerCommandPlanLoop24Done(type_2, channel_2, disposition_2, actions_2) => {
                                                self::ml_array::append(vec![type_2, channel_2, disposition_2], actions_2)
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
pub fn read_grammar_lexer_command_emission(
    name: &self::ml_string::Text,
    type_: &self::ml_string::Text,
    channel: &self::ml_string::Text,
    disposition: &self::ml_string::Text,
    operations: &[self::ml_string::Text],
    modes: &[self::ml_string::Text],
) -> Vec<self::ml_string::Text> {
    {
        let ml_library_inequality_5 = (operations.len() as f64) != (modes.len() as f64);
        if ml_library_inequality_5 {
            Vec::<self::ml_string::Text>::new()
        } else {
            {
                let pieces = Vec::<self::ml_string::Text>::new();
                {
                    let ml_s16 = if (*disposition)
                        == (self::ml_string::Text::from_units(vec![115, 107, 105, 112]))
                    {
                        {
                            let ml_library_inequality_4 = (*type_)
                                != (self::ml_string::Text::from_units(vec![83, 75, 73, 80]));
                            if ml_library_inequality_4 {
                                self::MlReadGrammarLexerCommandEmissionJoin28::MlReadGrammarLexerCommandEmissionJoin28Return(Vec::<self::ml_string::Text>::new())
                            } else {
                                {
                                    let pieces_2 = {
                                        let (ml_append_right, ml_append_left) = (
                                            vec![self::ml_string::Text::from_units(vec![
                                                115, 107, 105, 112,
                                            ])],
                                            pieces,
                                        );
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    self::MlReadGrammarLexerCommandEmissionJoin28::MlReadGrammarLexerCommandEmissionJoin28Next(pieces_2)
                                }
                            }
                        }
                    } else if (*disposition)
                        == (self::ml_string::Text::from_units(vec![101, 109, 105, 116]))
                    {
                        {
                            let ml_library_inequality_3 = (*type_) != (*name);
                            if ml_library_inequality_3 {
                                if ((self::accept_grammar_lexer_identifier(type_))
                                    || (self::accept_grammar_lexer_decimal(type_)))
                                {
                                    {
                                        let pieces_3 = {
                                            let (ml_append_right, ml_append_left) = (
                                                vec![
                                                    ((self::ml_string::Text::from_units(vec![
                                                        116, 121, 112, 101, 40,
                                                    ]))
                                                    .concat(type_))
                                                    .concat(&self::ml_string::Text::from_units(
                                                        vec![41],
                                                    )),
                                                ],
                                                pieces,
                                            );
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        self::MlReadGrammarLexerCommandEmissionJoin28::MlReadGrammarLexerCommandEmissionJoin28Next(pieces_3)
                                    }
                                } else {
                                    self::MlReadGrammarLexerCommandEmissionJoin28::MlReadGrammarLexerCommandEmissionJoin28Return(Vec::<self::ml_string::Text>::new())
                                }
                            } else {
                                self::MlReadGrammarLexerCommandEmissionJoin28::MlReadGrammarLexerCommandEmissionJoin28Next(pieces)
                            }
                        }
                    } else {
                        self::MlReadGrammarLexerCommandEmissionJoin28::MlReadGrammarLexerCommandEmissionJoin28Return(Vec::<self::ml_string::Text>::new())
                    };
                    match ml_s16 {
                    self::MlReadGrammarLexerCommandEmissionJoin28::MlReadGrammarLexerCommandEmissionJoin28Next(pieces_4) => {
                        {
                            let ml_s17 = {
let ml_library_inequality_2 = (*channel) != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])) ;
if ml_library_inequality_2 {
                                {
                                    let valid_channel = self::accept_grammar_lexer_identifier(channel) || self::accept_grammar_lexer_decimal(channel);
                                    if valid_channel {
                                        {
                                            let pieces_5 = { let (ml_append_right, ml_append_left) = (vec![((self::ml_string::Text::from_units(vec![99, 104, 97, 110, 110, 101, 108, 40])).concat(channel)).concat(&self::ml_string::Text::from_units(vec![41]))], pieces_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                            self::MlReadGrammarLexerCommandEmissionJoin29::MlReadGrammarLexerCommandEmissionJoin29Next(pieces_5)
                                        }
                                    } else {
                                        self::MlReadGrammarLexerCommandEmissionJoin29::MlReadGrammarLexerCommandEmissionJoin29Return(Vec::<self::ml_string::Text>::new())
                                    }
                                }
                            } else {
                                self::MlReadGrammarLexerCommandEmissionJoin29::MlReadGrammarLexerCommandEmissionJoin29Next(pieces_4)
                            }
};
                            match ml_s17 {
                                self::MlReadGrammarLexerCommandEmissionJoin29::MlReadGrammarLexerCommandEmissionJoin29Next(pieces_6) => {
                                    {
                                        let expected = vec![type_.clone(), channel.clone(), disposition.clone()];
                                        {
                                            let index = 0f64;
                                            {
                                                let ml_s18 = self::ml_read_grammar_lexer_command_emission_loop30(operations, modes, pieces_6, expected, index);
                                                match ml_s18 {
                                                    self::MlReadGrammarLexerCommandEmissionLoop30Result::MlReadGrammarLexerCommandEmissionLoop30Done(pieces_7, expected_2, index_2) => {
                                                        {
                                                            let command = if (pieces_7.len() as f64) == 0f64 {
                                                                self::ml_string::Text::from_units(vec![])
                                                            } else {
                                                                (self::ml_string::Text::from_units(vec![45, 62, 32])).concat(&self::ml_string::join(&pieces_7, &self::ml_string::Text::from_units(vec![44, 32])))
                                                            };
                                                            {
                                                                let replayed = self::read_grammar_lexer_command_plan(&(command), name);
                                                                {
let ml_library_inequality_1 = (replayed.len() as f64) != (expected_2.len() as f64) ;
if ml_library_inequality_1 {
                                                                    Vec::<self::ml_string::Text>::new()
                                                                } else {
                                                                    {
                                                                        let index_3 = 0f64;
                                                                        {
                                                                            let ml_s19 = self::ml_read_grammar_lexer_command_emission_loop31(&(expected_2), index_3, &(replayed));
                                                                            match ml_s19 {
                                                                                self::MlReadGrammarLexerCommandEmissionLoop31Result::MlReadGrammarLexerCommandEmissionLoop31Done => {
                                                                                    vec![command]
                                                                                }
                                                                                self::MlReadGrammarLexerCommandEmissionLoop31Result::MlReadGrammarLexerCommandEmissionLoop31Return(ml_result31) => {
                                                                                    ml_result31
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
}
                                                            }
                                                        }
                                                    }
                                                    self::MlReadGrammarLexerCommandEmissionLoop30Result::MlReadGrammarLexerCommandEmissionLoop30Return(ml_result31_2) => {
                                                        ml_result31_2
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlReadGrammarLexerCommandEmissionJoin29::MlReadGrammarLexerCommandEmissionJoin29Return(ml_result32) => {
                                    ml_result32
                                }
                            }
                        }
                    }
                    self::MlReadGrammarLexerCommandEmissionJoin28::MlReadGrammarLexerCommandEmissionJoin28Return(ml_result32_2) => {
                        ml_result32_2
                    }
                }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_grammar_lexer_emission_issue(
    families: f64,
    obligations: f64,
    modes: &[self::ml_string::Text],
    channels: &[self::ml_string::Text],
    types: f64,
    kinds: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    entry_modes: &[self::ml_string::Text],
    targets: &[self::ml_string::Text],
    public_kinds: &[self::ml_string::Text],
    public_names: &[self::ml_string::Text],
    rule_names: &[self::ml_string::Text],
    rule_kinds: &[self::ml_string::Text],
    references: &[self::ml_string::Text],
) -> self::ml_string::Text {
    {
        let ml_library_inequality_3 = families != 1f64;
        if ml_library_inequality_3 {
            self::ml_string::Text::from_units(vec![
                111, 110, 101, 32, 117, 110, 105, 113, 117, 101, 108, 121, 32, 111, 119, 110, 101,
                100, 32, 108, 101, 120, 101, 114, 32, 102, 97, 109, 105, 108, 121, 32, 105, 115,
                32, 114, 101, 113, 117, 105, 114, 101, 100,
            ])
        } else {
            let ml_library_inequality_2 = obligations != 0f64;
            if ml_library_inequality_2 {
                self::ml_string::Text::from_units(vec![
                    117, 110, 114, 101, 115, 111, 108, 118, 101, 100, 32, 108, 101, 120, 105, 99,
                    97, 108, 32, 111, 98, 108, 105, 103, 97, 116, 105, 111, 110, 115, 32, 114, 101,
                    109, 97, 105, 110,
                ])
            } else {
                let ml_library_inequality_1 = types != 0f64;
                if ml_library_inequality_1 {
                    self::ml_string::Text::from_units(vec![
                        116, 111, 107, 101, 110, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111,
                        110, 32, 115, 101, 99, 116, 105, 111, 110, 115, 32, 97, 114, 101, 32, 110,
                        111, 116, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100,
                    ])
                } else {
                    {
                        let count = kinds.len() as f64;
                        if ((((((names.len() as f64) != count)
                            || ((entry_modes.len() as f64) != count))
                            || ((targets.len() as f64) != count))
                            || ((public_kinds.len() as f64) != count))
                            || ((public_names.len() as f64) != count))
                            || ((rule_names.len() as f64) != (rule_kinds.len() as f64))
                        {
                            self::ml_string::Text::from_units(vec![
                                105, 110, 118, 97, 108, 105, 100, 32, 108, 101, 120, 105, 99, 97,
                                108, 32, 101, 109, 105, 115, 115, 105, 111, 110, 32, 102, 97, 99,
                                116, 32, 99, 111, 108, 117, 109, 110, 115,
                            ])
                        } else {
                            {
                                let index = 0f64;
                                {
                                    let ml_s20 = self::ml_read_grammar_lexer_emission_issue_loop33(
                                        modes, index,
                                    );
                                    match ml_s20 {
                                    self::MlReadGrammarLexerEmissionIssueLoop33Result::MlReadGrammarLexerEmissionIssueLoop33Done(index_2) => {
                                        {
                                            let index_3 = 0f64;
                                            {
                                                let ml_s21 = self::ml_read_grammar_lexer_emission_issue_loop34(channels, index_3);
                                                match ml_s21 {
                                                    self::MlReadGrammarLexerEmissionIssueLoop34Result::MlReadGrammarLexerEmissionIssueLoop34Done(index_4) => {
                                                        {
                                                            let locals = Vec::<self::ml_string::Text>::new();
                                                            {
                                                                let owners = Vec::<self::ml_string::Text>::new();
                                                                {
                                                                    let index_5 = 0f64;
                                                                    {
                                                                        let ml_s22 = self::ml_read_grammar_lexer_emission_issue_loop35(kinds, names, entry_modes, targets, public_kinds, public_names, count, index_5, locals, owners);
                                                                        match ml_s22 {
                                                                            self::MlReadGrammarLexerEmissionIssueLoop35Result::MlReadGrammarLexerEmissionIssueLoop35Done(index_6, locals_2, owners_2) => {
                                                                                {
                                                                                    let index_7 = 0f64;
                                                                                    {
                                                                                        let ml_s23 = self::ml_read_grammar_lexer_emission_issue_loop36(rule_names, rule_kinds, index_7, &(locals_2), &(owners_2));
                                                                                        match ml_s23 {
                                                                                            self::MlReadGrammarLexerEmissionIssueLoop36Result::MlReadGrammarLexerEmissionIssueLoop36Done(index_8) => {
                                                                                                {
                                                                                                    let index_9 = 0f64;
                                                                                                    {
                                                                                                        let ml_s24 = self::ml_read_grammar_lexer_emission_issue_loop37(references, index_9, &(locals_2));
                                                                                                        match ml_s24 {
                                                                                                            self::MlReadGrammarLexerEmissionIssueLoop37Result::MlReadGrammarLexerEmissionIssueLoop37Done => {
                                                                                                                self::ml_string::Text::from_units(vec![])
                                                                                                            }
                                                                                                            self::MlReadGrammarLexerEmissionIssueLoop37Result::MlReadGrammarLexerEmissionIssueLoop37Return(ml_result37) => {
                                                                                                                ml_result37
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlReadGrammarLexerEmissionIssueLoop36Result::MlReadGrammarLexerEmissionIssueLoop36Return(ml_result37_2) => {
                                                                                                ml_result37_2
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlReadGrammarLexerEmissionIssueLoop35Result::MlReadGrammarLexerEmissionIssueLoop35Return(ml_result37_3) => {
                                                                                ml_result37_3
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlReadGrammarLexerEmissionIssueLoop34Result::MlReadGrammarLexerEmissionIssueLoop34Return(ml_result39) => {
                                                        ml_result39
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlReadGrammarLexerEmissionIssueLoop33Result::MlReadGrammarLexerEmissionIssueLoop33Return(ml_result39_2) => {
                                        ml_result39_2
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
pub fn read_grammar_lexer_emission_disposition(
    command_recovery: bool,
    issue: &self::ml_string::Text,
) -> f64 {
    if (*issue) == (self::ml_string::Text::from_units(vec![])) {
        1f64
    } else if command_recovery {
        2f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_lexer_mode(
    candidate: &self::ml_string::Text,
    active: &self::ml_string::Text,
) -> bool {
    (*candidate) == (*active)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_lexer_trivia(
    channel: &self::ml_string::Text,
    disposition: &self::ml_string::Text,
) -> bool {
    ((*disposition) == (self::ml_string::Text::from_units(vec![115, 107, 105, 112])))
        || ((*channel)
            != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_lexer_trivia_identity(
    source: &self::ml_string::Text,
    emitted: &self::ml_string::Text,
    disposition: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*disposition) == (self::ml_string::Text::from_units(vec![115, 107, 105, 112])) {
        source.clone()
    } else {
        emitted.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_lexer_expected_token(
    expected: &self::ml_string::Text,
    actual: &self::ml_string::Text,
    wildcard: bool,
) -> bool {
    wildcard || ((*expected) == (*actual))
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
                                            let ml_s25 = self::ml_read_grammar_lexer_declaration_issue_loop40(kinds, names, items, values, count, binding_names, owner_names, literal_values, local_names, declared_modes, declared_channels, index);
                                            match ml_s25 {
                                                self::MlReadGrammarLexerDeclarationIssueLoop40Result::MlReadGrammarLexerDeclarationIssueLoop40Done(declared_modes_2, declared_channels_2, index_2) => {
                                                    {
                                                        let index_3 = 0f64;
                                                        {
                                                            let ml_s26 = self::ml_read_grammar_lexer_declaration_issue_loop41(kinds, modes, channels, dispositions, action_owners, actions, action_modes, count, &(declared_modes_2), &(declared_channels_2), index_3);
                                                            match ml_s26 {
                                                                self::MlReadGrammarLexerDeclarationIssueLoop41Result::MlReadGrammarLexerDeclarationIssueLoop41Done => {
                                                                    vec![0f64, -1f64, -1f64]
                                                                }
                                                                self::MlReadGrammarLexerDeclarationIssueLoop41Result::MlReadGrammarLexerDeclarationIssueLoop41Return(ml_result41) => {
                                                                    ml_result41
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlReadGrammarLexerDeclarationIssueLoop40Result::MlReadGrammarLexerDeclarationIssueLoop40Return(ml_result44) => {
                                                    ml_result44
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
                            let ml_s27 = self::ml_read_grammar_lexer_obligation_positions_loop49(
                                count,
                                edge_sources,
                                edge_targets,
                                reached,
                                index,
                                pending,
                                cursor,
                            );
                            match ml_s27 {
                                self::MlReadGrammarLexerObligationPositionsLoop49Result::MlReadGrammarLexerObligationPositionsLoop49Done(reached_2, index_2) => {
                                    {
                                        let selected = Vec::<f64>::new();
                                        {
                                            let index_3 = 0f64;
                                            {
                                                self::ml_read_grammar_lexer_obligation_positions_loop50(count, root, owners, categories, &(reached_2), index_3, selected)
                                            }
                                        }
                                    }
                                }
                                self::MlReadGrammarLexerObligationPositionsLoop49Result::MlReadGrammarLexerObligationPositionsLoop49Return(ml_result50) => {
                                    ml_result50
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
                        self::ml_read_grammar_lexer_lazy_normalization_loop53(
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_parser_token_negation(
    parser_production: bool,
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
) -> bool {
    if (!parser_production || ((kinds.len() as f64) == 0f64))
        || ((kinds.len() as f64) != (values.len() as f64))
    {
        false
    } else {
        {
            let index = 0f64;
            {
                let ml_s28 =
                    self::ml_accept_grammar_parser_token_negation_loop56(kinds, values, index);
                match ml_s28 {
                    self::MlAcceptGrammarParserTokenNegationLoop56Result::MlAcceptGrammarParserTokenNegationLoop56Done => {
                        true
                    }
                    self::MlAcceptGrammarParserTokenNegationLoop56Result::MlAcceptGrammarParserTokenNegationLoop56Return(ml_result56) => {
                        ml_result56
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
