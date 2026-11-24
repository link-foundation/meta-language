// Generated from complete js/src/translation/rust-generic-type-alias-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result;
use super::group_after;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::ml_string_number;
use super::read_required_text_fact;
use super::read_rust_generic_type_alias_parameter_directory;
use super::source_ml_source1_temporary_read_rust_generic_type_alias_declaration_plan_loop35;

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Result {
    MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Done(f64),
    MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Return(
        Vec<self::ml_string::Text>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_declaration_plan_loop34(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut end: f64,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Result {
    loop {
        return if end < (values.len() as f64) {
            {
                let boundary = self::read_required_text_fact(values, end);
                if ((self::read_required_text_fact(kinds, end))
                    == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                    && (boundary == (self::ml_string::Text::from_units(vec![59])))
                {
                    self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Done(end)
                } else if ((self::read_required_text_fact(kinds, end))
                    == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                    && (((boundary == (self::ml_string::Text::from_units(vec![123])))
                        || (boundary == (self::ml_string::Text::from_units(vec![91]))))
                        || (boundary == (self::ml_string::Text::from_units(vec![40]))))
                {
                    {
                        let after = self::group_after(kinds, values, end);
                        if after < 0f64 {
                            self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![117, 110, 99, 108, 111, 115, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 32, 103, 114, 111, 117, 112])])
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
                    self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![117, 110, 99, 108, 111, 115, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110])])
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
            self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Done(end)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33 {
    MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33Next(
        f64,
        Vec<self::ml_string::Text>,
    ),
    MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33Return(
        Vec<self::ml_string::Text>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_declaration_plan_loop36(
    mut parameters: Vec<self::ml_string::Text>,
    plan: &[self::ml_string::Text],
    mut count: f64,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < count {
            {
                let parameters_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::read_required_text_fact(plan, (4f64 + (at * 4f64)))],
                        parameters,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (parameters, at) = (parameters_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { parameters };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_declaration_plan_loop32(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    raws: &[self::ml_string::Text],
    mut result: Vec<self::ml_string::Text>,
    mut names: Vec<self::ml_string::Text>,
    mut owners: Vec<self::ml_string::Text>,
    mut closes: Vec<self::ml_string::Text>,
    mut position: f64,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result {
    loop {
        return if position < (values.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, position);
                {
                    let value = self::read_required_text_fact(values, position);
                    if ((closes.len() as f64) > 0f64)
                        && (position
                            == (self::ml_string_number::convert_java_script_string_number_units(
                                &(self::read_required_text_fact(
                                    &(closes),
                                    ((closes.len() as f64) - 1f64),
                                ))
                                .0
                                .iter()
                                .copied()
                                .map(f64::from)
                                .collect::<Vec<f64>>(),
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
                                let after = self::group_after(kinds, values, (position + 2f64));
                                if after < 0f64 {
                                    self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Return(result)
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
                                                let (ml_append_right, ml_append_left) = (
                                                    vec![{
                                                        let source_ml_source2_string_argument102 =
                                                            after - 1f64;
                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument102))
                                                    }],
                                                    closes,
                                                );
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
                            if ((position + 3f64) >= (values.len() as f64))
                                || ((self::read_required_text_fact(kinds, (position + 1f64)))
                                    != (self::ml_string::Text::from_units(vec![
                                        105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                                    ])))
                            {
                                self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 44, 32, 99, 111, 110, 115, 116, 114, 97, 105, 110, 101, 100, 32, 111, 114, 32, 109, 97, 108, 102, 111, 114, 109, 101, 100, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110])])
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
                                        self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115])])
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
                                                let first = position + 3f64;
                                                {
                                                    let parameters =
                                                        Vec::<self::ml_string::Text>::new();
                                                    {
                                                        let source_ml_source2_s105 = if (self::read_required_text_fact(values, (position + 2f64))) == (self::ml_string::Text::from_units(vec![60])) {
                                                                {
                                                                    let plan = self::read_rust_generic_type_alias_parameter_directory(kinds, values, raws, (position + 2f64));
                                                                    if {
                                                                        let source_ml_source2_comparison_operand106 = self::ml_array::optional_at(&plan, self::ml_array::number_index(0f64));
                                                                        {
                                                                            let source_ml_source2_comparison_operand107 = self::ml_string::Text::from_units(vec![112, 97, 114, 97, 109, 101, 116, 101, 114, 115]);
                                                                            !(match (source_ml_source2_comparison_operand106, source_ml_source2_comparison_operand107) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                        }
                                                                    } {
                                                                        self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33Return(plan)
                                                                    } else {
                                                                        {
                                                                            let close = self::ml_string_number::convert_java_script_string_number_units(&(self::read_required_text_fact(&(plan), 1f64)).0.iter().copied().map(f64::from).collect::<Vec<f64>>());
                                                                            {
                                                                                let count = self::ml_string_number::convert_java_script_string_number_units(&(self::read_required_text_fact(&(plan), 3f64)).0.iter().copied().map(f64::from).collect::<Vec<f64>>());
                                                                                {
                                                                                    let at = 0f64;
                                                                                    {
                                                                                        let parameters_2 = self::source_ml_source1_temporary_read_rust_generic_type_alias_declaration_plan_loop36(parameters, &(plan), count, at);
                                                                                        if (self::read_required_text_fact(&(plan), 2f64)) == (self::ml_string::Text::from_units(vec![61])) {
                                                                                            {
                                                                                                let first_2 = close + 1f64;
                                                                                                self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33Next(first_2, parameters_2)
                                                                                            }
                                                                                        } else if ((close + 1f64) >= (values.len() as f64)) || ((self::read_required_text_fact(values, (close + 1f64))) != (self::ml_string::Text::from_units(vec![61]))) {
                                                                                                self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 98, 111, 117, 110, 100, 115, 32, 97, 110, 100, 32, 119, 104, 101, 114, 101, 32, 99, 108, 97, 117, 115, 101, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110])])
                                                                                            } else {
                                                                                                {
                                                                                                    let first_3 = close + 2f64;
                                                                                                    self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33Next(first_3, parameters_2)
                                                                                                }
                                                                                            }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
let ml_library_inequality_1 = (self::read_required_text_fact(values, (position + 2f64))) != (self::ml_string::Text::from_units(vec![61])) ;
if ml_library_inequality_1 {
                                                                    self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 44, 32, 99, 111, 110, 115, 116, 114, 97, 105, 110, 101, 100, 32, 111, 114, 32, 109, 97, 108, 102, 111, 114, 109, 101, 100, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116, 97, 116, 105, 111, 110])])
                                                                } else {
                                                                    self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33Next(first, parameters)
                                                                }
};
                                                        match source_ml_source2_s105 {
                                                                self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33Next(first_4, parameters_3) => {
                                                                    {
                                                                        let end = first_4;
                                                                        {
                                                                            let source_ml_source2_s108 = self::source_ml_source1_temporary_read_rust_generic_type_alias_declaration_plan_loop34(kinds, values, end);
                                                                            match source_ml_source2_s108 {
                                                                                self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Done(end_2) => {
                                                                                    if (end_2 >= (values.len() as f64)) || (end_2 == first_4) {
                                                                                        self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![101, 109, 112, 116, 121, 32, 111, 114, 32, 117, 110, 99, 108, 111, 115, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110])])
                                                                                    } else {
                                                                                        {
                                                                                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![name.clone(), {
                                                                                                let source_ml_source2_string_argument109 = position;
                                                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument109))
                                                                                            }, {
                                                                                                let source_ml_source2_string_argument110 = first_4;
                                                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument110))
                                                                                            }, {
                                                                                                let source_ml_source2_string_argument111 = end_2;
                                                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument111))
                                                                                            }, {
                                                                                                let source_ml_source2_string_argument112 = end_2 + 1f64;
                                                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument112))
                                                                                            }, {
                                                                                                let source_ml_source2_string_argument113 = parameters_3.len() as f64;
                                                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument113))
                                                                                            }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                            {
                                                                                                let source_ml_source1_temporary_values31 = parameters_3;
                                                                                                {
                                                                                                    let source_ml_source1_temporary_index31 = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let result_3 = self::source_ml_source1_temporary_read_rust_generic_type_alias_declaration_plan_loop35(result_2, &(source_ml_source1_temporary_values31), source_ml_source1_temporary_index31);
                                                                                                        if (names_2.len() as f64) > 10_000f64 {
                                                                                                            self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 121, 112, 101, 32, 97, 108, 105, 97, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])])
                                                                                                        } else {
                                                                                                            {
                                                                                                                let position_4 = end_2 + 1f64;
                                                                                                                {
                                                                                                                    (result, names, position) = (result_3, names_2, position_4);
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
                                                                                self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop34Return(source_ml_source1_temporary_result35) => {
                                                                                    self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Return(source_ml_source1_temporary_result35)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanJoin33Return(source_ml_source1_temporary_result35_2) => {
                                                                    self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Return(source_ml_source1_temporary_result35_2)
                                                                }
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
                                let after_2 = self::group_after(kinds, values, position);
                                if after_2 < 0f64 {
                                    self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Return(result)
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
            self::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Result::MlSource1TemporaryReadRustGenericTypeAliasDeclarationPlanLoop32Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustGenericTypeAliasExpansionErrorLoop37Result {
    MlSource1TemporaryReadRustGenericTypeAliasExpansionErrorLoop37Done,
    MlSource1TemporaryReadRustGenericTypeAliasExpansionErrorLoop37Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_generic_type_alias_expansion_error_loop37(
    declarations: &[self::ml_string::Text],
    mut at: f64,
) -> self::MlSource1TemporaryReadRustGenericTypeAliasExpansionErrorLoop37Result {
    loop {
        return if at < (declarations.len() as f64) {
            if (self::read_required_text_fact(declarations, at))
                == (self::ml_string::Text::from_units(vec![]))
            {
                self::MlSource1TemporaryReadRustGenericTypeAliasExpansionErrorLoop37Result::MlSource1TemporaryReadRustGenericTypeAliasExpansionErrorLoop37Return(self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99, 32, 97, 108, 105, 97, 115, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114, 115]))
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
            self::MlSource1TemporaryReadRustGenericTypeAliasExpansionErrorLoop37Result::MlSource1TemporaryReadRustGenericTypeAliasExpansionErrorLoop37Done
        };
    }
}
