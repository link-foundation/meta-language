// Generated from complete js/src/translation/rust-generic-type-alias-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result;
use super::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Result;
use super::accept_parameter_name;
use super::dependency_function1;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_string;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_type_alias_declaration_plan_loop3(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut end: f64,
) -> self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Result {
    loop {
        return if end < (values.len() as f64) {
            {
                let boundary = self::read_required_text_fact(values, end);
                if ((self::read_required_text_fact(kinds, end))
                    == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                    && (boundary == (self::ml_string::Text::from_units(vec![59])))
                {
                    self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Done(end)
                } else if ((self::read_required_text_fact(kinds, end))
                    == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                    && (((boundary == (self::ml_string::Text::from_units(vec![123])))
                        || (boundary == (self::ml_string::Text::from_units(vec![91]))))
                        || (boundary == (self::ml_string::Text::from_units(vec![40]))))
                {
                    {
                        let after = self::dependency_function1(kinds, values, end);
                        if after < 0f64 {
                            self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![117, 110, 99, 108, 111, 115, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 32, 103, 114, 111, 117, 112])])
                        } else {
                            {
                                let end_2 = after;
                                {
                                    end = end_2;
                                    continue;
                                }
                            }
                        }
                    }
                } else if ((self::read_required_text_fact(kinds, end))
                    == (self::ml_string::Text::from_units(vec![101, 111, 102])))
                    || (((self::read_required_text_fact(kinds, end))
                        == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                        && (boundary == (self::ml_string::Text::from_units(vec![125]))))
                {
                    self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![117, 110, 99, 108, 111, 115, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110])])
                } else {
                    {
                        let end_3 = end + 1f64;
                        {
                            end = end_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Done(end)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_type_alias_declaration_plan_loop2(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut result: Vec<self::ml_string::Text>,
    mut names: Vec<self::ml_string::Text>,
    mut owners: Vec<self::ml_string::Text>,
    mut closes: Vec<f64>,
    mut position: f64,
) -> self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result {
    loop {
        return if position < (values.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, position);
                {
                    let value = self::read_required_text_fact(values, position);
                    if ((closes.len() as f64) > 0f64)
                        && (position
                            == (self::read_required_number_fact(
                                &(closes),
                                ((closes.len() as f64) - 1f64),
                            )))
                    {
                        {
                            let closes_2 = self::ml_array::slice(&closes, 0f64, -1f64);
                            {
                                let owners_2 = self::ml_array::slice(&owners, 0f64, -1f64);
                                {
                                    let position_2 = position + 1f64;
                                    {
                                        (owners, closes, position) =
                                            (owners_2, closes_2, position_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    {
                        if ((((kind
                            == (self::ml_string::Text::from_units(vec![
                                107, 101, 121, 119, 111, 114, 100,
                            ])))
                            && (value
                                == (self::ml_string::Text::from_units(vec![109, 111, 100]))))
                            && ((position + 2f64) < (values.len() as f64)))
                            && ((self::read_required_text_fact(kinds, (position + 1f64)))
                                == (self::ml_string::Text::from_units(vec![
                                    105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                                ]))))
                            && ((self::read_required_text_fact(values, (position + 2f64)))
                                == (self::ml_string::Text::from_units(vec![123])))
                        {
                            {
                                let after =
                                    self::dependency_function1(kinds, values, (position + 2f64));
                                if after < 0f64 {
                                    self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Return(result)
                                } else {
                                    {
                                        let owners_3 = {
                                            let (ml_append_right, ml_append_left) = (
                                                vec![self::read_required_text_fact(
                                                    values,
                                                    (position + 1f64),
                                                )],
                                                owners,
                                            );
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let closes_3 = {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![(after - 1f64)], closes);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let position_3 = position + 3f64;
                                                {
                                                    (owners, closes, position) =
                                                        (owners_3, closes_3, position_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else if (kind
                            == (self::ml_string::Text::from_units(vec![
                                107, 101, 121, 119, 111, 114, 100,
                            ])))
                            && (value
                                == (self::ml_string::Text::from_units(vec![116, 121, 112, 101])))
                        {
                            if (((position + 3f64) >= (values.len() as f64))
                                || ((self::read_required_text_fact(kinds, (position + 1f64)))
                                    != (self::ml_string::Text::from_units(vec![
                                        105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                                    ]))))
                                || ((self::read_required_text_fact(values, (position + 2f64)))
                                    != (self::ml_string::Text::from_units(vec![61])))
                            {
                                self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 44, 32, 99, 111, 110, 115, 116, 114, 97, 105, 110, 101, 100, 32, 111, 114, 32, 109, 97, 108, 102, 111, 114, 109, 101, 100, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110])])
                            } else {
                                {
                                    let name = self::ml_string::join(
                                        &self::ml_array::append(
                                            owners.clone(),
                                            vec![self::read_required_text_fact(
                                                values,
                                                (position + 1f64),
                                            )],
                                        ),
                                        &self::ml_string::Text::from_units(vec![46]),
                                    );
                                    if self::ml_array::includes(
                                        &names,
                                        &name.clone(),
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    ) {
                                        self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115])])
                                    } else {
                                        {
                                            let names_2 = {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![name.clone()], names);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let end = position + 3f64;
                                                {
                                                    let source_ml_source2_s75 = self::source_ml_source1_temporary_read_rust_type_alias_declaration_plan_loop3(kinds, values, end);
                                                    match source_ml_source2_s75 {
                                                            self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Done(end_2) => {
                                                                if (end_2 >= (values.len() as f64)) || (end_2 == (position + 3f64)) {
                                                                    self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![101, 109, 112, 116, 121, 32, 111, 114, 32, 117, 110, 99, 108, 111, 115, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110])])
                                                                } else {
                                                                    {
                                                                        let result_2 = { let (ml_append_right, ml_append_left) = (vec![name.clone(), {
                                                                            let source_ml_source2_string_argument76 = position;
                                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument76))
                                                                        }, {
                                                                            let source_ml_source2_string_argument77 = position + 3f64;
                                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument77))
                                                                        }, {
                                                                            let source_ml_source2_string_argument78 = end_2;
                                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument78))
                                                                        }, {
                                                                            let source_ml_source2_string_argument79 = end_2 + 1f64;
                                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument79))
                                                                        }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        if (names_2.len() as f64) > 10_000f64 {
                                                                            self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])])
                                                                        } else {
                                                                            {
                                                                                let position_4 = end_2 + 1f64;
                                                                                {
                                                                                    (result, names, position) = (result_2, names_2, position_4);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop3Return(source_ml_source1_temporary_result3) => {
                                                                self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Return(source_ml_source1_temporary_result3)
                                                            }
                                                        }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else if (kind
                            == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                            && (((value == (self::ml_string::Text::from_units(vec![123])))
                                || (value == (self::ml_string::Text::from_units(vec![91]))))
                                || (value == (self::ml_string::Text::from_units(vec![40]))))
                        {
                            {
                                let after_2 = self::dependency_function1(kinds, values, position);
                                if after_2 < 0f64 {
                                    self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Return(result)
                                } else {
                                    {
                                        let position_5 = after_2;
                                        {
                                            position = position_5;
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let position_6 = position + 1f64;
                                {
                                    position = position_6;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlSource1TemporaryReadRustTypeAliasDeclarationPlanLoop2Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4 {
    MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4Next(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
    ),
    MlSource1TemporaryReadRustTypeAliasLookupKeysJoin4Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_type_alias_lookup_keys_loop5(
    names: &[self::ml_string::Text],
    mut count: f64,
) -> f64 {
    loop {
        if ((self::ml_array::optional_at(names, self::ml_array::number_index(count)))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
            == (self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]))
        {
            {
                let count_2 = count + 1f64;
                {
                    count = count_2;
                    continue;
                }
            }
        }
        return { count };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_type_alias_access_prefix_lengths_loop6(
    mut result: Vec<f64>,
    mut length: f64,
) -> Vec<f64> {
    loop {
        if length > 0f64 {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (vec![length], result);
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let length_2 = length - 1f64;
                    {
                        (result, length) = (result_2, length_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceDepthLoop8Result {
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop8Done(f64),
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop8Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop8(
    mut depth: f64,
    source_ml_source1_temporary_values7: &[f64],
    mut source_ml_source1_temporary_index7: self::ml::Big,
) -> self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop8Result {
    loop {
        return if source_ml_source1_temporary_index7
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values7.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_source1_temporary_values7,
                    source_ml_source1_temporary_index7.to_index(),
                );
                if (!(self::ml_math::is_integer(child)) || (child < 0f64)) || (child > 16f64) {
                    self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop8Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop8Return(-1f64)
                } else {
                    if child > depth {
                        {
                            let depth_2 = child;
                            {
                                let source_ml_source1_temporary_index7_2 =
                                    source_ml_source1_temporary_index7
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (depth, source_ml_source1_temporary_index7) =
                                        (depth_2, source_ml_source1_temporary_index7_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_source1_temporary_index7_3 =
                                source_ml_source1_temporary_index7
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index7 =
                                    source_ml_source1_temporary_index7_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop8Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop8Done(depth)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop10Result {
    MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop10Done,
    MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop10Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop10(
    source_ml_source1_temporary_values9: &[f64],
    mut source_ml_source1_temporary_index9: self::ml::Big,
) -> self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop10Result {
    loop {
        return if source_ml_source1_temporary_index9
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values9.len() as u128))
        {
            {
                let value = self::ml_array::at(
                    source_ml_source1_temporary_values9,
                    source_ml_source1_temporary_index9.to_index(),
                );
                if !(self::ml_math::is_integer(value)) || (value < 0f64) {
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop10Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop10Return(self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 115, 117, 109, 32, 115, 111, 117, 114, 99, 101, 32, 109, 97, 116, 101, 114, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 99, 111, 117, 110, 116]))
                } else {
                    {
                        let source_ml_source1_temporary_index9_2 =
                            source_ml_source1_temporary_index9.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index9 =
                                source_ml_source1_temporary_index9_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop10Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop11Result {
    MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop11Done,
    MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop11Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop11(
    identities: &[self::ml_string::Text],
    direct_copies: &[bool],
    mut count: f64,
    mut at: f64,
) -> self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop11Result {
    loop {
        return if at < count {
            if (((self::ml_array::optional_at(identities, self::ml_array::number_index(at)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![])))
                || {
                    let source_ml_source2_comparison_operand80 = self::ml_array::optional_at(
                        direct_copies,
                        self::ml_array::number_index(at),
                    );
                    {
                        let source_ml_source2_comparison_operand81 = true;
                        !(match (
                            source_ml_source2_comparison_operand80,
                            source_ml_source2_comparison_operand81,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                }
            {
                self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop11Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop11Return(false)
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop11Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustFunctionPointerOwnedDomainErrorLoop12Result {
    MlSource1TemporaryReadRustFunctionPointerOwnedDomainErrorLoop12Done,
    MlSource1TemporaryReadRustFunctionPointerOwnedDomainErrorLoop12Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_function_pointer_owned_domain_error_loop12(
    parameters: &[self::ml_string::Text],
    mut at: f64,
) -> self::MlSource1TemporaryReadRustFunctionPointerOwnedDomainErrorLoop12Result {
    loop {
        return if at < (parameters.len() as f64) {
            if {
                let source_ml_source2_comparison_operand82 =
                    self::ml_array::optional_at(parameters, self::ml_array::number_index(at));
                {
                    let source_ml_source2_comparison_operand83 =
                        self::ml_string::Text::from_units(vec![99, 111, 112, 121]);
                    !(match (
                        source_ml_source2_comparison_operand82,
                        source_ml_source2_comparison_operand83,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } && {
                let source_ml_source2_comparison_operand84 =
                    self::ml_array::optional_at(parameters, self::ml_array::number_index(at));
                {
                    let source_ml_source2_comparison_operand85 =
                        self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100]);
                    !(match (
                        source_ml_source2_comparison_operand84,
                        source_ml_source2_comparison_operand85,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                self::MlSource1TemporaryReadRustFunctionPointerOwnedDomainErrorLoop12Result::MlSource1TemporaryReadRustFunctionPointerOwnedDomainErrorLoop12Return(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 112, 111, 105, 110, 116, 101, 114, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 110, 101, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 100, 105, 114, 101, 99, 116, 32, 67, 111, 112, 121, 32, 111, 114, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100, 32, 111, 119, 110, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 115]))
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustFunctionPointerOwnedDomainErrorLoop12Result::MlSource1TemporaryReadRustFunctionPointerOwnedDomainErrorLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustFunctionPointerTypeErrorLoop13Result {
    MlSource1TemporaryReadRustFunctionPointerTypeErrorLoop13Done,
    MlSource1TemporaryReadRustFunctionPointerTypeErrorLoop13Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_function_pointer_type_error_loop13(
    parameters: &[bool],
    mut index: f64,
) -> self::MlSource1TemporaryReadRustFunctionPointerTypeErrorLoop13Result {
    loop {
        return if index < (parameters.len() as f64) {
            if (self::read_required_boolean_fact(parameters, index)) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlSource1TemporaryReadRustFunctionPointerTypeErrorLoop13Result::MlSource1TemporaryReadRustFunctionPointerTypeErrorLoop13Return(self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 112, 111, 105, 110, 116, 101, 114, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 114, 101, 99, 116, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 67, 111, 112, 121, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 115]))
            }
        } else {
            self::MlSource1TemporaryReadRustFunctionPointerTypeErrorLoop13Result::MlSource1TemporaryReadRustFunctionPointerTypeErrorLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result {
    MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Done,
    MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Return(Vec<self::ml_string::Text>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15 {
    MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15Next(f64, self::ml_string::Text),
    MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15Return(Vec<self::ml_string::Text>),
    MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15Continue(
        f64,
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_parameter_plan_loop14(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    raws: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    mut at: f64,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result {
    loop {
        return if at < (values.len() as f64) {
            {
                let name = self::read_required_text_fact(values, at);
                if ((self::read_required_text_fact(kinds, at))
                    != (self::ml_string::Text::from_units(vec![
                        105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                    ])))
                    || !(self::accept_parameter_name(
                        &(name),
                        &(self::read_required_text_fact(raws, at)),
                    ))
                {
                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 110, 101, 101, 100, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 117, 110, 99, 111, 110, 115, 116, 114, 97, 105, 110, 101, 100, 32, 116, 121, 112, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115])])
                } else if self::ml_array::includes(&names, &name.clone(), 0f64, false, |a, b| {
                    a == b
                }) {
                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 116, 121, 112, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114])])
                } else if (names.len() as f64) >= 64f64 {
                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])])
                } else {
                    {
                        let names_2 = {
                            let (ml_append_right, ml_append_left) = (vec![name.clone()], names);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let at_2 = at + 1f64;
                            if at_2 >= (values.len() as f64) {
                                self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![117, 110, 99, 108, 111, 115, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 108, 105, 115, 116])])
                            } else {
                                {
                                    let punctuation = self::read_required_text_fact(values, at_2);
                                    {
                                        let ml_library_inequality_2 =
                                            (self::read_required_text_fact(kinds, at_2))
                                                != (self::ml_string::Text::from_units(vec![
                                                    112, 117, 110, 99, 116,
                                                ]));
                                        if ml_library_inequality_2 {
                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 98, 111, 117, 110, 100, 115, 32, 97, 110, 100, 32, 100, 101, 102, 97, 117, 108, 116, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110])])
                                        } else {
                                            {
                                                let source_ml_source2_s86 = if punctuation
                                                    == (self::ml_string::Text::from_units(vec![44]))
                                                {
                                                    {
                                                        let at_3 = at_2 + 1f64;
                                                        if at_3 >= (values.len() as f64) {
                                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![117, 110, 99, 108, 111, 115, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 108, 105, 115, 116])])
                                                        } else {
                                                            {
                                                                let punctuation_2 =
                                                                    self::read_required_text_fact(
                                                                        values, at_3,
                                                                    );
                                                                if (punctuation_2 != (self::ml_string::Text::from_units(vec![62]))) && (punctuation_2 != (self::ml_string::Text::from_units(vec![62, 61]))) {
                                                                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15Continue(at_3, punctuation_2.clone())
                                                                    } else {
let ml_library_inequality_1 = (self::read_required_text_fact(kinds, at_3)) != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) ;
if ml_library_inequality_1 {
                                                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![109, 97, 108, 102, 111, 114, 109, 101, 100, 32, 103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 100, 101, 108, 105, 109, 105, 116, 101, 114])])
                                                                        } else {
                                                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15Next(at_3, punctuation_2.clone())
                                                                        }
}
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15Next(at_2, punctuation.clone())
                                                };
                                                match source_ml_source2_s86 {
                                                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15Next(at_4, punctuation_3) => {
                                                            if (punctuation_3 == (self::ml_string::Text::from_units(vec![62]))) || (punctuation_3 == (self::ml_string::Text::from_units(vec![62, 61]))) {
                                                                self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Return(self::ml_array::append(vec![self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 101, 116, 101, 114, 115]), {
                                                                    let source_ml_source2_string_argument87 = at_4;
                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument87))
                                                                }, if punctuation_3 == (self::ml_string::Text::from_units(vec![62, 61])) {
                                                                    self::ml_string::Text::from_units(vec![61])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                }], names_2))
                                                            } else {
                                                                self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 98, 111, 117, 110, 100, 115, 32, 97, 110, 100, 32, 100, 101, 102, 97, 117, 108, 116, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110])])
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15Return(source_ml_source1_temporary_result15) => {
                                                            self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Return(source_ml_source1_temporary_result15)
                                                        }
                                                        self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanJoin15Continue(at_5, punctuation_4) => {
                                                            {
                                                                (names, at) = (names_2, at_5);
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
        } else {
            self::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Result::MlSource1TemporaryReadRustGenericTypeAliasParameterPlanLoop14Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_parameter_directory_loop16(
    names: &[self::ml_string::Text],
    starts: &[self::ml_string::Text],
    ends: &[self::ml_string::Text],
    offsets: &[self::ml_string::Text],
    mut result: Vec<self::ml_string::Text>,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < (names.len() as f64) {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            self::read_required_text_fact(names, at),
                            self::read_required_text_fact(starts, at),
                            self::read_required_text_fact(ends, at),
                            self::read_required_text_fact(offsets, at),
                        ],
                        result,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Result {
    MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Done,
    MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop17Return(
        Vec<self::ml_string::Text>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19 {
    MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19Next(
        f64,
        self::ml_string::Text,
    ),
    MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19Return(
        Vec<self::ml_string::Text>,
    ),
    MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin19Continue(
        f64,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18 {
    MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18Next(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        f64,
        bool,
        self::ml_string::Text,
    ),
    MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin18Return(
        Vec<self::ml_string::Text>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Result {
    MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Done(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        f64,
        bool,
    ),
    MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop20Return(
        Vec<self::ml_string::Text>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21 {
    MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21Next(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        f64,
    ),
    MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryJoin21Return(
        Vec<self::ml_string::Text>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Result {
    MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Done(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
        f64,
    ),
    MlSource1TemporaryReadRustGenericTypeAliasParameterDirectoryLoop22Return(
        Vec<self::ml_string::Text>,
    ),
}
