// Generated from complete js/src/translation/whole-parameter-default-rules.js. Do not edit by hand.
use super::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin45;
use super::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin46;
use super::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Result;
use super::accept_java_script_source_binding_name;
use super::find_java_script_pattern_default_expression_end;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_recursive_bindings;
use super::read_java_script_recursive_object_binding_positions;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_plan_java_script_whole_parameter_defaults_loop47;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_java_script_whole_parameter_defaults_loop44(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut depth: f64,
    mut at: f64,
    mut slot: f64,
    mut names: Vec<self::ml_string::Text>,
    mut rows: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Result {
    loop {
        return if (at < (kinds.len() as f64))
            && !(((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![41]))))
        {
            {
                let token = at;
                {
                    let tree = Vec::<f64>::new();
                    {
                        let pattern = false;
                        {
                            let source_ml_source2_s218 = if ((self::read_required_text_fact(
                                kinds, at,
                            ))
                                == (self::ml_string::Text::from_units(vec![
                                    112, 117, 110, 99, 116,
                                ])))
                                && (((self::read_required_text_fact(values, at))
                                    == (self::ml_string::Text::from_units(vec![123])))
                                    || ((self::read_required_text_fact(values, at))
                                        == (self::ml_string::Text::from_units(vec![91]))))
                            {
                                {
                                    let pattern_2 = true;
                                    {
                                        let tree_2 = self::plan_java_script_recursive_bindings(
                                            kinds,
                                            values,
                                            at,
                                            reserved,
                                            &(names),
                                            depth,
                                        );
                                        if (self::read_required_number_fact(&(tree_2), 0f64)) < 0f64
                                        {
                                            self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin45::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin45Return(tree_2)
                                        } else {
                                            {
                                                let positions = self::read_java_script_recursive_object_binding_positions(&(tree_2));
                                                {
                                                    let index = 0f64;
                                                    {
                                                        let names_2 = self::source_ml_source1_temporary_plan_java_script_whole_parameter_defaults_loop47(values, names, &(positions), index);
                                                        {
                                                            let at_2 =
                                                                self::read_required_number_fact(
                                                                    &(tree_2),
                                                                    0f64,
                                                                );
                                                            self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin45::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin45Next(at_2, names_2, tree_2, pattern_2)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let name = self::read_required_text_fact(values, at);
                                    if (self::accept_java_script_source_binding_name(
                                        &(self::read_required_text_fact(kinds, at)),
                                        &(name),
                                        self::ml_array::includes(
                                            &names,
                                            &name.clone(),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        ),
                                        reserved,
                                    )) {
                                        {
                                            let names_3 = {
                                                let source_ml_source2_concat_value219 = names;
                                                {
                                                    let source_ml_source2_concat_value220 =
                                                        vec![name.clone()];
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<self::ml_string::Text>::new(),
                                                            source_ml_source2_concat_value219,
                                                        ),
                                                        source_ml_source2_concat_value220,
                                                    )
                                                }
                                            };
                                            {
                                                let at_3 = at + 1f64;
                                                self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin45::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin45Next(at_3, names_3, tree, pattern)
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin45::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin45Return(vec![-8f64, at])
                                    }
                                }
                            };
                            match source_ml_source2_s218 {
                                self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin45::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin45Next(at_4, names_4, tree_3, pattern_3) => {
                                    if at_4 >= (kinds.len() as f64) {
                                        self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Result::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Return(vec![-1f64, at_4])
                                    } else {
                                        {
                                            let first = -1f64;
                                            {
                                                let last = -1f64;
                                                {
                                                    let source_ml_source2_s221 = if ((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                        {
                                                            let first_2 = at_4 + 1f64;
                                                            {
                                                                let last_2 = self::find_java_script_pattern_default_expression_end(kinds, values, first_2, &(self::ml_string::Text::from_units(vec![41])));
                                                                if last_2 < 0f64 {
                                                                    self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin46::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin46Return(vec![-1f64, first_2])
                                                                } else {
                                                                    {
                                                                        let at_5 = last_2;
                                                                        self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin46::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin46Next(at_5, first_2, last_2)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin46::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin46Next(at_4, first, last)
                                                    };
                                                    match source_ml_source2_s221 {
                                                        self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin46::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin46Next(at_6, first_3, last_3) => {
                                                            {
                                                                let rows_4 = if pattern_3 {
                                                                    {
                                                                        {
                                                                            let source_ml_source2_concat_value222 = rows;
                                                                            {
                                                                                let source_ml_source2_concat_value223 = vec![if first_3 < 0f64 {
                                                                                    1f64
                                                                                } else {
                                                                                    3f64
                                                                                }, slot, (tree_3.len() as f64)];
                                                                                {
                                                                                    let source_ml_source2_concat_value224 = tree_3;
                                                                                    {
                                                                                        let source_ml_source2_concat_value225 = if first_3 < 0f64 {
                                                                                            Vec::<f64>::new()
                                                                                        } else {
                                                                                            vec![first_3, last_3]
                                                                                        };
                                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value222), source_ml_source2_concat_value223), source_ml_source2_concat_value224), source_ml_source2_concat_value225)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    {
                                                                        {
                                                                            let source_ml_source2_concat_value226 = rows;
                                                                            {
                                                                                let source_ml_source2_concat_value227 = if first_3 < 0f64 {
                                                                                    vec![0f64, slot, token]
                                                                                } else {
                                                                                    vec![2f64, slot, token, first_3, last_3]
                                                                                };
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value226), source_ml_source2_concat_value227)
                                                                            }
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let slot_2 = slot + 1f64;
                                                                    if at_6 >= (kinds.len() as f64) {
                                                                        self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Result::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Return(vec![-1f64, at_6])
                                                                    } else {
                                                                        if ((self::read_required_text_fact(kinds, at_6)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_6)) == (self::ml_string::Text::from_units(vec![44]))) {
                                                                            {
                                                                                let at_7 = at_6 + 1f64;
                                                                                {
                                                                                    (at, slot, names, rows) = (at_7, slot_2, names_4, rows_4);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
{
                                                                            if (((self::read_required_text_fact(kinds, at_6)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_6)) == (self::ml_string::Text::from_units(vec![41])))) {
                                                                                {
                                                                                    (at, slot, names, rows) = (at_6, slot_2, names_4, rows_4);
                                                                                    continue;
                                                                                }
                                                                            }
{
                                                                                self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Result::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Return(vec![-1f64, at_6])
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin46::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin46Return(source_ml_source1_temporary_result46) => {
                                                            self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Result::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Return(source_ml_source1_temporary_result46)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin45::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsJoin45Return(source_ml_source1_temporary_result46_2) => {
                                    self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Result::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Return(source_ml_source1_temporary_result46_2)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Result::MlSource1TemporaryPlanJavaScriptWholeParameterDefaultsLoop44Done(at, slot, rows)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryContainsJavaScriptWholeParameterDefaultLoop48Result {
    MlSource1TemporaryContainsJavaScriptWholeParameterDefaultLoop48Done(bool, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_contains_java_script_whole_parameter_default_loop48(
    plan: &[f64],
    mut at: f64,
    mut pattern: bool,
    mut defaulted: bool,
) -> self::MlSource1TemporaryContainsJavaScriptWholeParameterDefaultLoop48Result {
    loop {
        if at < (plan.len() as f64) {
            {
                let form = self::read_required_number_fact(plan, at);
                if form == 0f64 {
                    {
                        let at_2 = at + 3f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
                {
                    if form == 2f64 {
                        {
                            let defaulted_2 = true;
                            {
                                let at_3 = at + 5f64;
                                {
                                    (at, defaulted) = (at_3, defaulted_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let pattern_2 = true;
                            {
                                let defaulted_4 = if form == 3f64 { { true } } else { defaulted };
                                {
                                    let at_4 = at
                                        + ((3f64
                                            + self::read_required_number_fact(plan, (at + 2f64)))
                                            + if form == 3f64 { 2f64 } else { 0f64 });
                                    {
                                        (at, pattern, defaulted) = (at_4, pattern_2, defaulted_4);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryContainsJavaScriptWholeParameterDefaultLoop48Result::MlSource1TemporaryContainsJavaScriptWholeParameterDefaultLoop48Done(pattern, defaulted)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_whole_parameter_binding_positions_loop49(
    plan: &[f64],
    mut at: f64,
    mut result: Vec<f64>,
) -> Vec<f64> {
    loop {
        if at < (plan.len() as f64) {
            {
                let form = self::read_required_number_fact(plan, at);
                if (form == 0f64) || (form == 2f64) {
                    {
                        let result_2 = {
                            let source_ml_source2_concat_value228 = result;
                            {
                                let source_ml_source2_concat_value229 =
                                    vec![self::read_required_number_fact(plan, (at + 2f64))];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source2_concat_value228,
                                    ),
                                    source_ml_source2_concat_value229,
                                )
                            }
                        };
                        {
                            let at_2 = at + if form == 0f64 { 3f64 } else { 5f64 };
                            {
                                (at, result) = (at_2, result_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let length = self::read_required_number_fact(plan, (at + 2f64));
                        {
                            let result_3 = {
                                let source_ml_source2_concat_value230 = result;
                                {
                                    let source_ml_source2_concat_value231 =
                                        self::read_java_script_recursive_object_binding_positions(
                                            &(self::ml_array::slice(
                                                plan,
                                                (at + 3f64),
                                                ((at + 3f64) + length),
                                            )),
                                        );
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value230,
                                        ),
                                        source_ml_source2_concat_value231,
                                    )
                                }
                            };
                            {
                                let at_3 =
                                    at + ((3f64 + length) + if form == 3f64 { 2f64 } else { 0f64 });
                                {
                                    (at, result) = (at_3, result_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptJavaScriptWholeParameterInitializationOrderLoop50Result {
    MlSource1TemporaryAcceptJavaScriptWholeParameterInitializationOrderLoop50Done,
    MlSource1TemporaryAcceptJavaScriptWholeParameterInitializationOrderLoop50Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_whole_parameter_initialization_order_loop50(
    plan: &[f64],
    mut at: f64,
    mut pattern: bool,
) -> self::MlSource1TemporaryAcceptJavaScriptWholeParameterInitializationOrderLoop50Result {
    loop {
        return if at < (plan.len() as f64) {
            {
                let form = self::read_required_number_fact(plan, at);
                if form == 0f64 {
                    {
                        let at_2 = at + 3f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
                {
                    if form == 2f64 {
                        if pattern {
                            self::MlSource1TemporaryAcceptJavaScriptWholeParameterInitializationOrderLoop50Result::MlSource1TemporaryAcceptJavaScriptWholeParameterInitializationOrderLoop50Return(false)
                        } else {
                            {
                                let at_3 = at + 5f64;
                                {
                                    at = at_3;
                                    continue;
                                }
                            }
                        }
                    } else if (form == 3f64) && pattern {
                        self::MlSource1TemporaryAcceptJavaScriptWholeParameterInitializationOrderLoop50Result::MlSource1TemporaryAcceptJavaScriptWholeParameterInitializationOrderLoop50Return(false)
                    } else {
                        {
                            let pattern_2 = true;
                            {
                                let at_4 = at
                                    + ((3f64 + self::read_required_number_fact(plan, (at + 2f64)))
                                        + if form == 3f64 { 2f64 } else { 0f64 });
                                {
                                    (at, pattern) = (at_4, pattern_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptJavaScriptWholeParameterInitializationOrderLoop50Result::MlSource1TemporaryAcceptJavaScriptWholeParameterInitializationOrderLoop50Done
        };
    }
}
