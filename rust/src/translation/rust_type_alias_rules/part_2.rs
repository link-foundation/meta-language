// Generated from complete js/src/translation/rust-type-alias-rules.js. Do not edit by hand.
use super::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result;
use super::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result;
use super::group_after;
use super::ml_array;
use super::ml_number;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_temporary_read_rust_type_alias_declaration_plan_loop3;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_type_alias_declaration_plan_loop2(
    kinds: &[String],
    values: &[String],
    mut result: Vec<String>,
    mut names: Vec<String>,
    mut owners: Vec<String>,
    mut closes: Vec<f64>,
    mut position: f64,
) -> self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result {
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
                        if ((((kind == "keyword") && (value == "mod"))
                            && ((position + 2f64) < (values.len() as f64)))
                            && ((self::read_required_text_fact(kinds, (position + 1f64)))
                                == "identifier"))
                            && ((self::read_required_text_fact(values, (position + 2f64))) == "{")
                        {
                            {
                                let after = self::group_after(kinds, values, (position + 2f64));
                                if after < 0f64 {
                                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(result)
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
                        } else if (kind == "keyword") && (value == "type") {
                            if (((position + 3f64) >= (values.len() as f64))
                                || ((self::read_required_text_fact(kinds, (position + 1f64)))
                                    != "identifier"))
                                || ((self::read_required_text_fact(values, (position + 2f64)))
                                    != "=")
                            {
                                self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(vec![String::from("error"), String::from("generic, constrained or malformed type aliases require their complete parameter interpretation")])
                            } else {
                                {
                                    let name = (self::ml_array::append(
                                        owners.clone(),
                                        vec![self::read_required_text_fact(
                                            values,
                                            (position + 1f64),
                                        )],
                                    ))
                                    .join((String::from(".")).as_str());
                                    if self::ml_array::includes(
                                        &names,
                                        &name.clone(),
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    ) {
                                        self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(vec![String::from("error"), String::from("duplicate source type alias")])
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
                                                    let source_ml_source1_s4 = self::source_ml_temporary_read_rust_type_alias_declaration_plan_loop3(kinds, values, end);
                                                    match source_ml_source1_s4 {
                                                            self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Done(end_2) => {
                                                                if (end_2 >= (values.len() as f64)) || (end_2 == (position + 3f64)) {
                                                                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(vec![String::from("error"), String::from("empty or unclosed source type alias declaration")])
                                                                } else {
                                                                    {
                                                                        let result_2 = { let (ml_append_right, ml_append_left) = (vec![name.clone(), {
                                                                            let source_ml_source1_string_argument5 = position;
                                                                            self::ml_number::js_number(source_ml_source1_string_argument5)
                                                                        }, {
                                                                            let source_ml_source1_string_argument6 = position + 3f64;
                                                                            self::ml_number::js_number(source_ml_source1_string_argument6)
                                                                        }, {
                                                                            let source_ml_source1_string_argument7 = end_2;
                                                                            self::ml_number::js_number(source_ml_source1_string_argument7)
                                                                        }, {
                                                                            let source_ml_source1_string_argument8 = end_2 + 1f64;
                                                                            self::ml_number::js_number(source_ml_source1_string_argument8)
                                                                        }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        if (names_2.len() as f64) > 10_000f64 {
                                                                            self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(vec![String::from("error"), String::from("source type alias declaration limit exceeded")])
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
                                                            self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop3Return(source_ml_temporary_result3) => {
                                                                self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(source_ml_temporary_result3)
                                                            }
                                                        }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else if (kind == "punct")
                            && (((value == "{") || (value == "[")) || (value == "("))
                        {
                            {
                                let after_2 = self::group_after(kinds, values, position);
                                if after_2 < 0f64 {
                                    self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Return(result)
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
            self::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Result::MlTemporaryReadRustTypeAliasDeclarationPlanLoop2Done(result)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_type_alias_lookup_keys_loop5(
    names: &[String],
    start: &[String],
    mut absolute: bool,
    mut result: Vec<String>,
    mut depth: f64,
) -> Vec<String> {
    loop {
        return if depth >= 0f64 {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (self::ml_array::append(
                                self::ml_array::slice(start, 0f64, depth),
                                names.to_vec(),
                            ))
                            .join((String::from(".")).as_str()),
                        ],
                        result,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                if absolute {
                    result_2
                } else {
                    {
                        let depth_2 = depth - 1f64;
                        {
                            (result, depth) = (result_2, depth_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustTypeAliasLookupKeysJoin4 {
    MlTemporaryReadRustTypeAliasLookupKeysJoin4Next(Vec<String>, Vec<String>, bool),
    MlTemporaryReadRustTypeAliasLookupKeysJoin4Return(Vec<String>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_type_alias_lookup_keys_loop6(
    names: &[String],
    mut count: f64,
) -> f64 {
    loop {
        if ((self::ml_array::optional_at(names, self::ml_array::number_index(count)))
            .unwrap_or_else(|| String::new()))
            == "super"
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
