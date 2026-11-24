// Generated from complete js/src/translation/binding-initialization-rules.js. Do not edit by hand.
use super::accept_java_script_source_binding_name;
use super::contains_java_script_recursive_object_binding;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::plan_java_script_recursive_bindings;
use super::read_java_script_recursive_object_binding_positions;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_recursive_array_element_index_loop37(
    plan: &[f64],
    mut row: f64,
    mut owner: f64,
    mut position: f64,
    mut index: f64,
) -> f64 {
    loop {
        if index < row {
            {
                let at = 2f64 + (index * 8f64);
                {
                    let role = self::read_required_number_fact(plan, at);
                    if ((self::read_required_number_fact(plan, (at + 1f64))) == owner)
                        && (((role == 3f64) || (role == 4f64)) || (role == 5f64))
                    {
                        {
                            let position_2 = position + 1f64;
                            {
                                let index_2 = index + 1f64;
                                {
                                    (position, index) = (position_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let index_3 = index + 1f64;
                            {
                                index = index_3;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { position };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Result {
    MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Done,
    MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_contains_java_script_advanced_array_binding_loop38(
    plan: &[f64],
    mut index: f64,
) -> self::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Result {
    loop {
        return if index < (self::read_required_number_fact(plan, 1f64)) {
            {
                let at = 2f64 + (index * 8f64);
                {
                    let role = self::read_required_number_fact(plan, at);
                    if (((((role == 1f64) || (role == 2f64)) || (role == 4f64)) || (role == 5f64))
                        || ((self::read_required_number_fact(plan, (at + 4f64))) >= 0f64))
                        || ((self::read_required_number_fact(plan, (at + 6f64))) >= 0f64)
                    {
                        self::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Result::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Return(true)
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
            self::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Result::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Result {
    MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Done(f64, f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40 {
    MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40Next(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsJoin40Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_recursive_binding_formals_loop41(
    values: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    positions: &[f64],
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (positions.len() as f64) {
            {
                let names_2 = {
                    let source_ml_source2_concat_value196 = names;
                    {
                        let source_ml_source2_concat_value197 =
                            vec![self::read_required_text_fact(
                                values,
                                self::read_required_number_fact(positions, index),
                            )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source2_concat_value196,
                            ),
                            source_ml_source2_concat_value197,
                        )
                    }
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (names, index) = (names_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { names };
    }
}

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
                let source_ml_source2_s205 = if ((self::read_required_text_fact(kinds, at))
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
                                                let source_ml_source2_concat_value206 =
                                                    instructions;
                                                {
                                                    let source_ml_source2_concat_value207 =
                                                        vec![1f64, slot, (tree.len() as f64)];
                                                    {
                                                        let source_ml_source2_concat_value208 =
                                                            tree.clone();
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value206), source_ml_source2_concat_value207), source_ml_source2_concat_value208)
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
                            let source_ml_source2_concat_value209 = names;
                            {
                                let source_ml_source2_concat_value210 =
                                    vec![self::read_required_text_fact(values, at)];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source2_concat_value209,
                                    ),
                                    source_ml_source2_concat_value210,
                                )
                            }
                        };
                        {
                            let instructions_3 = {
                                let source_ml_source2_concat_value211 = instructions;
                                {
                                    let source_ml_source2_concat_value212 = vec![0f64, slot, at];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value211,
                                        ),
                                        source_ml_source2_concat_value212,
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
                match source_ml_source2_s205 {
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
                                let source_ml_source2_s213 = if (self::read_required_number_fact(
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
                                match source_ml_source2_s213 {
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptPatternInitializerAnchorLoop44Result {
    MlSource1TemporaryReadJavaScriptPatternInitializerAnchorLoop44Done,
    MlSource1TemporaryReadJavaScriptPatternInitializerAnchorLoop44Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_pattern_initializer_anchor_loop44(
    mut open: f64,
    mut close: f64,
    positions: &[f64],
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptPatternInitializerAnchorLoop44Result {
    loop {
        return if index < (positions.len() as f64) {
            {
                let position = self::read_required_number_fact(positions, index);
                if (position > open) && (position < close) {
                    self::MlSource1TemporaryReadJavaScriptPatternInitializerAnchorLoop44Result::MlSource1TemporaryReadJavaScriptPatternInitializerAnchorLoop44Return(position)
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
            self::MlSource1TemporaryReadJavaScriptPatternInitializerAnchorLoop44Result::MlSource1TemporaryReadJavaScriptPatternInitializerAnchorLoop44Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPartitionJavaScriptPatternInitializerPositionsLoop45Result {
    MlSource1TemporaryPartitionJavaScriptPatternInitializerPositionsLoop45Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_partition_java_script_pattern_initializer_positions_loop45(
    positions: &[f64],
    mut anchor: f64,
    mut available: Vec<f64>,
    mut uninitialized: Vec<f64>,
    mut previous: f64,
    mut index: f64,
) -> self::MlSource1TemporaryPartitionJavaScriptPatternInitializerPositionsLoop45Result {
    loop {
        return if index < (positions.len() as f64) {
            {
                let position = self::read_required_number_fact(positions, index);
                if !(self::ml_math::is_safe_integer(position)) || (position <= previous) {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("RangeError"),
                        message_units: (self::ml_string::Text::from_units(vec![
                            97, 99, 116, 117, 97, 108, 32, 98, 105, 110, 100, 105, 110, 103, 32,
                            112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 109, 117, 115, 116,
                            32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 115,
                            116, 114, 105, 99, 116, 32, 115, 111, 117, 114, 99, 101, 32, 111, 114,
                            100, 101, 114,
                        ]))
                        .0,
                        message_present: true,
                    })
                    .raise()
                } else {
                    {
                        let previous_2 = position;
                        if position < anchor {
                            {
                                let available_2 = {
                                    let source_ml_source2_concat_value214 = available;
                                    {
                                        let source_ml_source2_concat_value215 = vec![position];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source2_concat_value214,
                                            ),
                                            source_ml_source2_concat_value215,
                                        )
                                    }
                                };
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        (available, previous, index) =
                                            (available_2, previous_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let uninitialized_2 = {
                                    let source_ml_source2_concat_value216 = uninitialized;
                                    {
                                        let source_ml_source2_concat_value217 = vec![position];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<f64>::new(),
                                                source_ml_source2_concat_value216,
                                            ),
                                            source_ml_source2_concat_value217,
                                        )
                                    }
                                };
                                {
                                    let index_3 = index + 1f64;
                                    {
                                        (uninitialized, previous, index) =
                                            (uninitialized_2, previous_2, index_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPartitionJavaScriptPatternInitializerPositionsLoop45Result::MlSource1TemporaryPartitionJavaScriptPatternInitializerPositionsLoop45Done(available, uninitialized)
        };
    }
}
