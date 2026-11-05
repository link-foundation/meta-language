// Generated from complete js/src/translation/recursive-binding-rules.js. Do not edit by hand.
use super::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40;
use super::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Result;
use super::accept_java_script_source_binding_name;
use super::contains_java_script_recursive_object_binding;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_recursive_bindings;
use super::read_java_script_recursive_object_binding_positions;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_plan_java_script_recursive_binding_formals_loop41;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_java_script_recursive_binding_formals_loop39(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut maximum_depth: f64,
    mut at: f64,
    mut slot: f64,
    mut names: Vec<self::ml_string::Text>,
    mut instructions: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Result {
    loop {
        return if (at < (kinds.len() as f64))
            && !(((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![41]))))
        {
            {
                let source_ml_source2_s200 = if ((self::read_required_text_fact(kinds, at))
                    == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                    && (((self::read_required_text_fact(values, at))
                        == (self::ml_string::Text::from_units(vec![123])))
                        || ((self::read_required_text_fact(values, at))
                            == (self::ml_string::Text::from_units(vec![91]))))
                {
                    {
                        let tree = self::plan_java_script_recursive_bindings(
                            kinds,
                            values,
                            at,
                            reserved,
                            &(names),
                            maximum_depth,
                        );
                        if (self::read_required_number_fact(&(tree), 0f64)) < 0f64 {
                            self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40Return(tree)
                        } else {
                            {
                                let positions =
                                    self::read_java_script_recursive_object_binding_positions(
                                        &(tree),
                                    );
                                {
                                    let index = 0f64;
                                    {
                                        let names_2 = self::source_ml_source1_temporary_plan_java_script_recursive_binding_formals_loop41(values, names, &(positions), index);
                                        {
                                            let instructions_2 = {
                                                let source_ml_source2_concat_value201 =
                                                    instructions;
                                                {
                                                    let source_ml_source2_concat_value202 =
                                                        vec![1f64, slot, (tree.len() as f64)];
                                                    {
                                                        let source_ml_source2_concat_value203 =
                                                            tree.clone();
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value201), source_ml_source2_concat_value202), source_ml_source2_concat_value203)
                                                    }
                                                }
                                            };
                                            {
                                                let at_2 =
                                                    self::read_required_number_fact(&(tree), 0f64);
                                                self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40Next(at_2, names_2, instructions_2)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if (self::accept_java_script_source_binding_name(
                    &(self::read_required_text_fact(kinds, at)),
                    &(self::read_required_text_fact(values, at)),
                    self::ml_array::includes(
                        &names,
                        &self::read_required_text_fact(values, at),
                        0f64,
                        false,
                        |a, b| a == b,
                    ),
                    reserved,
                )) {
                    {
                        let names_3 = {
                            let source_ml_source2_concat_value204 = names;
                            {
                                let source_ml_source2_concat_value205 =
                                    vec![self::read_required_text_fact(values, at)];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source2_concat_value204,
                                    ),
                                    source_ml_source2_concat_value205,
                                )
                            }
                        };
                        {
                            let instructions_3 = {
                                let source_ml_source2_concat_value206 = instructions;
                                {
                                    let source_ml_source2_concat_value207 = vec![0f64, slot, at];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value206,
                                        ),
                                        source_ml_source2_concat_value207,
                                    )
                                }
                            };
                            {
                                let at_3 = at + 1f64;
                                self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40Next(at_3, names_3, instructions_3)
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40Return(vec![-8f64, at])
                };
                match source_ml_source2_s200 {
                    self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40Next(at_4, names_4, instructions_4) => {
                        if at_4 >= (kinds.len() as f64) {
                            self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Result::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Return(vec![-1f64, at_4])
                        } else if ((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![61]))) {
                                self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Result::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Return(vec![-2f64, at_4])
                            } else {
                                {
                                    let slot_2 = slot + 1f64;
                                    if ((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![44]))) {
                                        {
                                            let at_5 = at_4 + 1f64;
                                            {
                                                (at, slot, names, instructions) = (at_5, slot_2, names_4, instructions_4);
                                                continue;
                                            }
                                        }
                                    }
{
                                        if (((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![41])))) {
                                            {
                                                (at, slot, names, instructions) = (at_4, slot_2, names_4, instructions_4);
                                                continue;
                                            }
                                        }
{
                                            self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Result::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Return(vec![-1f64, at_4])
                                        }
                                    }
                                }
                            }
                    }
                    self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40Return(source_ml_source1_temporary_result40) => {
                        self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Result::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Return(source_ml_source1_temporary_result40)
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Result::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Done(at, slot, instructions)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsLoop42Result {
    MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsLoop42Done(bool, bool),
    MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsLoop42Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsJoin43 {
    MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsJoin43Next(bool, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_contains_java_script_recursive_binding_formals_loop42(
    plan: &[f64],
    mut at: f64,
    mut object: bool,
    mut array: bool,
) -> self::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsLoop42Result {
    loop {
        return if at < (plan.len() as f64) {
            if (self::read_required_number_fact(plan, at)) == 0f64 {
                {
                    let at_2 = at + 3f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                {
                    let count = self::read_required_number_fact(plan, (at + 2f64));
                    {
                        let tree = self::ml_array::slice(plan, (at + 3f64), ((at + 3f64) + count));
                        if self::contains_java_script_recursive_object_binding(&(tree)) {
                            self::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsLoop42Result::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsLoop42Return(true)
                        } else {
                            {
                                let source_ml_source2_s208 = if (self::read_required_number_fact(
                                    &(tree),
                                    2f64,
                                )) == 6f64
                                {
                                    {
                                        let array_2 = true;
                                        self::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsJoin43::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsJoin43Next(object, array_2)
                                    }
                                } else {
                                    {
                                        let object_2 = true;
                                        self::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsJoin43::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsJoin43Next(object_2, array)
                                    }
                                };
                                match source_ml_source2_s208 {
                                    self::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsJoin43::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsJoin43Next(object_3, array_3) => {
                                        {
                                            let at_3 = at + (3f64 + count);
                                            {
                                                (at, object, array) = (at_3, object_3, array_3);
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
        } else {
            self::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsLoop42Result::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsLoop42Done(object, array)
        };
    }
}
