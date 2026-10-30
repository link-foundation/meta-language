// Generated from complete js/src/translation/binding-initialization-rules.js. Do not edit by hand.
use super::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Result;
use super::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsLoop42Result;
use super::MlSource1TemporaryContainsJavaScriptRecursiveObjectBindingLoop25Result;
use super::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Result;
use super::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop15Result;
use super::MlSource1TemporaryPartitionJavaScriptFormalInitializerBindingPositionsLoop14Result;
use super::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Result;
use super::MlSource1TemporaryPlanJavaScriptRecursiveBindingsLoop36Result;
use super::MlSource1TemporaryPlanJavaScriptRecursiveObjectBindingsLoop22Result;
use super::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Result;
use super::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Result;
use super::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result;
use super::MlSource1TemporaryReadJavaScriptPatternInitializerAnchorLoop44Result;
use super::MlSource1TemporaryReadJavaScriptRecursiveObjectInitializerBindingLoop24Result;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_array_iterable_form;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_contains_java_script_advanced_array_binding_loop38;
use super::source_ml_source1_temporary_contains_java_script_recursive_binding_formals_loop42;
use super::source_ml_source1_temporary_contains_java_script_recursive_object_binding_loop25;
use super::source_ml_source1_temporary_contains_java_script_recursive_object_formals_loop30;
use super::source_ml_source1_temporary_find_java_script_array_formal_initializer_assignment_loop15;
use super::source_ml_source1_temporary_partition_java_script_formal_initializer_binding_positions_loop14;
use super::source_ml_source1_temporary_plan_java_script_recursive_binding_formals_loop39;
use super::source_ml_source1_temporary_plan_java_script_recursive_bindings_loop36;
use super::source_ml_source1_temporary_plan_java_script_recursive_object_bindings_loop22;
use super::source_ml_source1_temporary_plan_java_script_recursive_object_formals_loop26;
use super::source_ml_source1_temporary_read_java_script_array_binding_tree_loop31;
use super::source_ml_source1_temporary_read_java_script_array_formal_default_binding_positions_loop12;
use super::source_ml_source1_temporary_read_java_script_object_binding_tree_loop17;
use super::source_ml_source1_temporary_read_java_script_pattern_initializer_anchor_loop44;
use super::source_ml_source1_temporary_read_java_script_recursive_array_element_index_loop37;
use super::source_ml_source1_temporary_read_java_script_recursive_object_binding_positions_loop21;
use super::source_ml_source1_temporary_read_java_script_recursive_object_excluded_positions_loop23;
use super::source_ml_source1_temporary_read_java_script_recursive_object_formal_binding_positions_loop29;
use super::source_ml_source1_temporary_read_java_script_recursive_object_initializer_binding_loop24;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_formal_default_binding_positions(plan: &[f64]) -> Vec<f64> {
    if ((plan.len() as f64) < 2f64) || ((self::read_required_number_fact(plan, 0f64)) < 0f64) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                97, 110, 32, 97, 114, 114, 97, 121, 32, 100, 101, 102, 97, 117, 108, 116, 32, 98,
                105, 110, 100, 105, 110, 103, 32, 99, 101, 110, 115, 117, 115, 32, 110, 101, 101,
                100, 115, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 100, 109, 105,
                116, 116, 101, 100, 32, 112, 108, 97, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let cursor = 2f64;
            {
                let names = Vec::<f64>::new();
                {
                    self::source_ml_source1_temporary_read_java_script_array_formal_default_binding_positions_loop12(plan, cursor, names)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn partition_java_script_array_formal_initializer_bindings(
    plan: &[f64],
    binding: f64,
) -> Vec<f64> {
    self::partition_java_script_formal_initializer_binding_positions(
        &(self::read_java_script_array_formal_default_binding_positions(plan)),
        binding,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn partition_java_script_formal_initializer_binding_positions(
    positions: &[f64],
    binding: f64,
) -> Vec<f64> {
    {
        let available = Vec::<f64>::new();
        {
            let uninitialized = Vec::<f64>::new();
            {
                let found = false;
                {
                    let index = 0f64;
                    {
                        let source_ml_source2_s66 = self::source_ml_source1_temporary_partition_java_script_formal_initializer_binding_positions_loop14(positions, binding, available, uninitialized, found, index);
                        match source_ml_source2_s66 {
                            self::MlSource1TemporaryPartitionJavaScriptFormalInitializerBindingPositionsLoop14Result::MlSource1TemporaryPartitionJavaScriptFormalInitializerBindingPositionsLoop14Done(available_2, uninitialized_2, found_2) => {
                                if found_2 {
                                    {
                                        let source_ml_source2_concat_value67 = vec![(available_2.len() as f64)];
                                        {
                                            let source_ml_source2_concat_value68 = available_2;
                                            {
                                                let source_ml_source2_concat_value69 = uninitialized_2;
                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value67), source_ml_source2_concat_value68), source_ml_source2_concat_value69)
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 100, 109, 105, 116, 116, 101, 100, 32, 66, 111, 117, 110, 100, 78, 97, 109, 101, 32, 112, 111, 115, 105, 116, 105, 111, 110])).0, message_present: true }).raise()
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
pub fn find_java_script_array_formal_initializer_assignment(
    assignments: &[self::ml_string::Text],
    parameters: &[self::ml_string::Text],
) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s70 = self::source_ml_source1_temporary_find_java_script_array_formal_initializer_assignment_loop15(assignments, parameters, index);
            match source_ml_source2_s70 {
                self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop15Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop15Done => {
                    -1f64
                }
                self::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop15Result::MlSource1TemporaryFindJavaScriptArrayFormalInitializerAssignmentLoop15Return(source_ml_source1_temporary_result15) => {
                    source_ml_source1_temporary_result15
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_formal_iterable_operation() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        114, 101, 97, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 65, 114, 114, 97, 121, 70,
        111, 114, 109, 97, 108, 73, 116, 101, 114, 97, 98, 108, 101,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_formal_iterable_form(
    kind: &self::ml_string::Text,
    reference: bool,
) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])) {
        self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])
    } else {
        self::read_array_iterable_form(kind, reference)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_formal_inference_form(
    kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*kind) == (self::ml_string::Text::from_units(vec![118, 97, 114])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![108, 105, 116, 101, 114, 97, 108])))
    {
        self::ml_string::Text::from_units(vec![119, 97, 105, 116])
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))
    {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else if (((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
    {
        self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 101])
    } else {
        self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_binding_depth_limit() -> f64 {
    256f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_binding_tree(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
    depth: f64,
    maximum_depth: f64,
    arrays: bool,
) -> Vec<f64> {
    if depth >= maximum_depth {
        vec![-9f64, start]
    } else if ((arrays && (start < (kinds.len() as f64)))
        && ((self::read_required_text_fact(kinds, start))
            == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))))
        && ((self::read_required_text_fact(values, start))
            == (self::ml_string::Text::from_units(vec![91])))
    {
        self::read_java_script_array_binding_tree(
            kinds,
            values,
            start,
            reserved,
            depth,
            maximum_depth,
        )
    } else if ((start >= (kinds.len() as f64))
        || ((self::read_required_text_fact(kinds, start))
            != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))))
        || ((self::read_required_text_fact(values, start))
            != (self::ml_string::Text::from_units(vec![123])))
    {
        vec![-1f64, start]
    } else {
        {
            let at = start + 1f64;
            {
                let rows = vec![0f64, -1f64, start, -1f64, -1f64, -1f64, start, -1f64];
                {
                    let properties = 0f64;
                    {
                        let source_ml_source2_s71 = self::source_ml_source1_temporary_read_java_script_object_binding_tree_loop17(kinds, values, reserved, depth, maximum_depth, arrays, at, rows, properties);
                        match source_ml_source2_s71 {
                                    self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Done(at_2, rows_2, properties_2) => {
                                        {
let ml_product_condition_1 = { let ml_value = properties_2; ml_value != 0f64 && !ml_value.is_nan() };
if ml_product_condition_1 {if ((at_2 >= (kinds.len() as f64)) || ((self::read_required_text_fact(kinds, at_2)) != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))) || ((self::read_required_text_fact(values, at_2)) != (self::ml_string::Text::from_units(vec![125]))) {
                                                vec![-1f64, at_2]
                                            } else {
                                                {
                                                    let rows_3 = {
                                                        let source_ml_source2_concat_value72 = self::ml_array::slice(&rows_2, 0f64, 7f64);
                                                        {
                                                            let source_ml_source2_concat_value73 = vec![at_2];
                                                            {
                                                                let source_ml_source2_concat_value74 = self::ml_array::slice(&rows_2, 8f64, f64::INFINITY);
                                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value72), source_ml_source2_concat_value73), source_ml_source2_concat_value74)
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_concat_value75 = vec![(at_2 + 1f64), ((rows_3.len() as f64) / 8f64)];
                                                        {
                                                            let source_ml_source2_concat_value76 = rows_3;
                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value75), source_ml_source2_concat_value76)
                                                        }
                                                    }
                                                }
                                            }} else {
                                            vec![-4f64, start]
                                        }
}
                                    }
                                    self::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Result::MlSource1TemporaryReadJavaScriptObjectBindingTreeLoop17Return(source_ml_source1_temporary_result17) => {
                                        source_ml_source1_temporary_result17
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
pub fn read_java_script_recursive_object_binding_positions(plan: &[f64]) -> Vec<f64> {
    if ((plan.len() as f64) < 2f64) || ((self::read_required_number_fact(plan, 0f64)) < 0f64) {
        Vec::<f64>::new()
    } else {
        {
            let result = Vec::<f64>::new();
            {
                let index = 0f64;
                {
                    self::source_ml_source1_temporary_read_java_script_recursive_object_binding_positions_loop21(plan, result, index)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_recursive_object_bindings(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
    previous: &[self::ml_string::Text],
    maximum_depth: f64,
) -> Vec<f64> {
    if ((((((kinds.len() as f64) != (values.len() as f64))
        || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64)))
        || !(self::ml_math::is_safe_integer(maximum_depth)))
        || (maximum_depth < 1f64)
    {
        vec![-1f64, start]
    } else {
        {
            let plan = self::read_java_script_object_binding_tree(
                kinds,
                values,
                start,
                reserved,
                0f64,
                maximum_depth,
                false,
            );
            if (self::read_required_number_fact(&(plan), 0f64)) < 0f64 {
                plan
            } else {
                {
                    let positions =
                        self::read_java_script_recursive_object_binding_positions(&(plan));
                    {
                        let names = previous.to_vec();
                        {
                            let index = 0f64;
                            {
                                let source_ml_source2_s77 = self::source_ml_source1_temporary_plan_java_script_recursive_object_bindings_loop22(kinds, values, reserved, &(positions), names, index);
                                match source_ml_source2_s77 {
                                    self::MlSource1TemporaryPlanJavaScriptRecursiveObjectBindingsLoop22Result::MlSource1TemporaryPlanJavaScriptRecursiveObjectBindingsLoop22Done => {
                                        plan
                                    }
                                    self::MlSource1TemporaryPlanJavaScriptRecursiveObjectBindingsLoop22Result::MlSource1TemporaryPlanJavaScriptRecursiveObjectBindingsLoop22Return(source_ml_source1_temporary_result22) => {
                                        source_ml_source1_temporary_result22
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
pub fn read_java_script_recursive_object_excluded_positions(plan: &[f64], owner: f64) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let index = 0f64;
            {
                self::source_ml_source1_temporary_read_java_script_recursive_object_excluded_positions_loop23(plan, owner, result, index)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_recursive_object_initializer_binding(plan: &[f64], row: f64) -> f64 {
    {
        let at = 2f64 + (row * 8f64);
        {
            let binding = self::read_required_number_fact(plan, (at + 3f64));
            if binding >= 0f64 {
                binding
            } else {
                {
                    let open = self::read_required_number_fact(plan, (at + 6f64));
                    {
                        let close = self::read_required_number_fact(plan, (at + 7f64));
                        {
                            let positions =
                                self::read_java_script_recursive_object_binding_positions(plan);
                            {
                                let index = 0f64;
                                {
                                    let source_ml_source2_s78 = self::source_ml_source1_temporary_read_java_script_recursive_object_initializer_binding_loop24(open, close, &(positions), index);
                                    match source_ml_source2_s78 {
                                        self::MlSource1TemporaryReadJavaScriptRecursiveObjectInitializerBindingLoop24Result::MlSource1TemporaryReadJavaScriptRecursiveObjectInitializerBindingLoop24Done => {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 32, 114, 101, 99, 117, 114, 115, 105, 118, 101, 32, 111, 98, 106, 101, 99, 116, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 105, 114, 115, 116, 32, 100, 101, 115, 99, 101, 110, 100, 97, 110, 116, 32, 98, 105, 110, 100, 105, 110, 103])).0, message_present: true }).raise()
                                        }
                                        self::MlSource1TemporaryReadJavaScriptRecursiveObjectInitializerBindingLoop24Result::MlSource1TemporaryReadJavaScriptRecursiveObjectInitializerBindingLoop24Return(source_ml_source1_temporary_result24) => {
                                            source_ml_source1_temporary_result24
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
pub fn contains_java_script_recursive_object_binding(plan: &[f64]) -> bool {
    if ((plan.len() as f64) < 2f64) || ((self::read_required_number_fact(plan, 0f64)) < 0f64) {
        false
    } else {
        {
            let index = 1f64;
            {
                let source_ml_source2_s79 = self::source_ml_source1_temporary_contains_java_script_recursive_object_binding_loop25(plan, index);
                match source_ml_source2_s79 {
                    self::MlSource1TemporaryContainsJavaScriptRecursiveObjectBindingLoop25Result::MlSource1TemporaryContainsJavaScriptRecursiveObjectBindingLoop25Done => {
                        false
                    }
                    self::MlSource1TemporaryContainsJavaScriptRecursiveObjectBindingLoop25Result::MlSource1TemporaryContainsJavaScriptRecursiveObjectBindingLoop25Return(source_ml_source1_temporary_result25) => {
                        source_ml_source1_temporary_result25
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_recursive_object_formals(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
    maximum_depth: f64,
) -> Vec<f64> {
    if ((((kinds.len() as f64) != (values.len() as f64)) || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64))
    {
        vec![-1f64, start]
    } else {
        {
            let at = start;
            {
                let slot = 0f64;
                {
                    let names = Vec::<self::ml_string::Text>::new();
                    {
                        let instructions = Vec::<f64>::new();
                        {
                            let source_ml_source2_s80 = self::source_ml_source1_temporary_plan_java_script_recursive_object_formals_loop26(kinds, values, reserved, maximum_depth, at, slot, names, instructions);
                            match source_ml_source2_s80 {
                                self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Result::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Done(at_2, slot_2, instructions_2) => {
                                    if at_2 >= (kinds.len() as f64) {
                                        vec![-1f64, at_2]
                                    } else {
                                        {
                                            let source_ml_source2_concat_value81 = vec![(at_2 + 1f64), slot_2];
                                            {
                                                let source_ml_source2_concat_value82 = instructions_2;
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value81), source_ml_source2_concat_value82)
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Result::MlSource1TemporaryPlanJavaScriptRecursiveObjectFormalsLoop26Return(source_ml_source1_temporary_result26) => {
                                    source_ml_source1_temporary_result26
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
pub fn read_java_script_recursive_object_formal_binding_positions(plan: &[f64]) -> Vec<f64> {
    if ((plan.len() as f64) < 2f64) || ((self::read_required_number_fact(plan, 0f64)) < 0f64) {
        Vec::<f64>::new()
    } else {
        {
            let at = 2f64;
            {
                let result = Vec::<f64>::new();
                {
                    self::source_ml_source1_temporary_read_java_script_recursive_object_formal_binding_positions_loop29(plan, at, result)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn contains_java_script_recursive_object_formals(plan: &[f64]) -> bool {
    if ((plan.len() as f64) < 2f64) || ((self::read_required_number_fact(plan, 0f64)) < 0f64) {
        false
    } else {
        {
            let at = 2f64;
            {
                let source_ml_source2_s83 = self::source_ml_source1_temporary_contains_java_script_recursive_object_formals_loop30(plan, at);
                match source_ml_source2_s83 {
                    self::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Result::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Done => {
                        false
                    }
                    self::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Result::MlSource1TemporaryContainsJavaScriptRecursiveObjectFormalsLoop30Return(source_ml_source1_temporary_result30) => {
                        source_ml_source1_temporary_result30
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_binding_tree(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
    depth: f64,
    maximum_depth: f64,
) -> Vec<f64> {
    if depth >= maximum_depth {
        vec![-9f64, start]
    } else {
        {
            let at = start + 1f64;
            {
                let rows = vec![6f64, -1f64, start, -1f64, -1f64, -1f64, start, -1f64];
                {
                    let source_ml_source2_s84 = self::source_ml_source1_temporary_read_java_script_array_binding_tree_loop31(kinds, values, reserved, depth, maximum_depth, at, rows);
                    match source_ml_source2_s84 {
                        self::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Result::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Done(at_2, rows_2) => {
                            if ((at_2 >= (kinds.len() as f64)) || ((self::read_required_text_fact(kinds, at_2)) != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))) || ((self::read_required_text_fact(values, at_2)) != (self::ml_string::Text::from_units(vec![93]))) {
                                vec![-1f64, at_2]
                            } else {
                                {
                                    let rows_3 = {
                                        let source_ml_source2_concat_value85 = self::ml_array::slice(&rows_2, 0f64, 7f64);
                                        {
                                            let source_ml_source2_concat_value86 = vec![at_2];
                                            {
                                                let source_ml_source2_concat_value87 = self::ml_array::slice(&rows_2, 8f64, f64::INFINITY);
                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value85), source_ml_source2_concat_value86), source_ml_source2_concat_value87)
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_concat_value88 = vec![(at_2 + 1f64), ((rows_3.len() as f64) / 8f64)];
                                        {
                                            let source_ml_source2_concat_value89 = rows_3;
                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value88), source_ml_source2_concat_value89)
                                        }
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Result::MlSource1TemporaryReadJavaScriptArrayBindingTreeLoop31Return(source_ml_source1_temporary_result31) => {
                            source_ml_source1_temporary_result31
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_recursive_bindings(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
    previous: &[self::ml_string::Text],
    maximum_depth: f64,
) -> Vec<f64> {
    if ((((((kinds.len() as f64) != (values.len() as f64))
        || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64)))
        || !(self::ml_math::is_safe_integer(maximum_depth)))
        || (maximum_depth < 1f64)
    {
        vec![-1f64, start]
    } else {
        {
            let plan = self::read_java_script_object_binding_tree(
                kinds,
                values,
                start,
                reserved,
                0f64,
                maximum_depth,
                true,
            );
            if (self::read_required_number_fact(&(plan), 0f64)) < 0f64 {
                plan
            } else {
                {
                    let positions =
                        self::read_java_script_recursive_object_binding_positions(&(plan));
                    {
                        let names = previous.to_vec();
                        {
                            let index = 0f64;
                            {
                                let source_ml_source2_s90 = self::source_ml_source1_temporary_plan_java_script_recursive_bindings_loop36(kinds, values, reserved, &(positions), names, index);
                                match source_ml_source2_s90 {
                                    self::MlSource1TemporaryPlanJavaScriptRecursiveBindingsLoop36Result::MlSource1TemporaryPlanJavaScriptRecursiveBindingsLoop36Done => {
                                        plan
                                    }
                                    self::MlSource1TemporaryPlanJavaScriptRecursiveBindingsLoop36Result::MlSource1TemporaryPlanJavaScriptRecursiveBindingsLoop36Return(source_ml_source1_temporary_result36) => {
                                        source_ml_source1_temporary_result36
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
pub fn read_java_script_recursive_array_element_index(plan: &[f64], row: f64) -> f64 {
    {
        let owner = self::read_required_number_fact(plan, ((2f64 + (row * 8f64)) + 1f64));
        {
            let position = 0f64;
            {
                let index = 1f64;
                {
                    self::source_ml_source1_temporary_read_java_script_recursive_array_element_index_loop37(plan, row, owner, position, index)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn contains_java_script_advanced_array_binding(plan: &[f64]) -> bool {
    if (((plan.len() as f64) < 2f64) || ((self::read_required_number_fact(plan, 0f64)) < 0f64))
        || ((self::read_required_number_fact(plan, 2f64)) != 6f64)
    {
        false
    } else if (self::read_required_number_fact(plan, 1f64)) == 1f64 {
        true
    } else {
        {
            let index = 1f64;
            {
                let source_ml_source2_s91 = self::source_ml_source1_temporary_contains_java_script_advanced_array_binding_loop38(plan, index);
                match source_ml_source2_s91 {
                        self::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Result::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Done => {
                            false
                        }
                        self::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Result::MlSource1TemporaryContainsJavaScriptAdvancedArrayBindingLoop38Return(source_ml_source1_temporary_result38) => {
                            source_ml_source1_temporary_result38
                        }
                    }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_recursive_binding_formals(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
    maximum_depth: f64,
) -> Vec<f64> {
    if ((((kinds.len() as f64) != (values.len() as f64)) || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64))
    {
        vec![-1f64, start]
    } else {
        {
            let at = start;
            {
                let slot = 0f64;
                {
                    let names = Vec::<self::ml_string::Text>::new();
                    {
                        let instructions = Vec::<f64>::new();
                        {
                            let source_ml_source2_s92 = self::source_ml_source1_temporary_plan_java_script_recursive_binding_formals_loop39(kinds, values, reserved, maximum_depth, at, slot, names, instructions);
                            match source_ml_source2_s92 {
                                self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Result::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Done(at_2, slot_2, instructions_2) => {
                                    if at_2 >= (kinds.len() as f64) {
                                        vec![-1f64, at_2]
                                    } else {
                                        {
                                            let source_ml_source2_concat_value93 = vec![(at_2 + 1f64), slot_2];
                                            {
                                                let source_ml_source2_concat_value94 = instructions_2;
                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value93), source_ml_source2_concat_value94)
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Result::MlSource1TemporaryPlanJavaScriptRecursiveBindingFormalsLoop39Return(source_ml_source1_temporary_result39) => {
                                    source_ml_source1_temporary_result39
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
pub fn contains_java_script_recursive_binding_formals(plan: &[f64]) -> bool {
    if ((plan.len() as f64) < 2f64) || ((self::read_required_number_fact(plan, 0f64)) < 0f64) {
        false
    } else {
        {
            let at = 2f64;
            {
                let object = false;
                {
                    let array = false;
                    {
                        let source_ml_source2_s95 = self::source_ml_source1_temporary_contains_java_script_recursive_binding_formals_loop42(plan, at, object, array);
                        match source_ml_source2_s95 {
                            self::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsLoop42Result::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsLoop42Done(object_2, array_2) => {
                                object_2 && array_2
                            }
                            self::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsLoop42Result::MlSource1TemporaryContainsJavaScriptRecursiveBindingFormalsLoop42Return(source_ml_source1_temporary_result42) => {
                                source_ml_source1_temporary_result42
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
pub fn read_java_script_recursive_binding_iterable_operation() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        114, 101, 97, 100, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 99, 117, 114, 115,
        105, 118, 101, 66, 105, 110, 100, 105, 110, 103, 73, 116, 101, 114, 97, 98, 108, 101,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_recursive_binding_initializer_context(
    context: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*context)
        == (self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 114, 101, 99, 117, 114, 115, 105, 118, 101,
            32, 97, 114, 114, 97, 121, 32, 98, 105, 110, 100, 105, 110, 103, 32, 100, 101, 99, 108,
            97, 114, 97, 116, 105, 111, 110,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 111, 98, 106, 101, 99, 116, 32, 98, 105,
            110, 100, 105, 110, 103, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110,
        ])
    } else {
        context.clone()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_recursive_binding_iterable_form(
    kind: &self::ml_string::Text,
    element: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])) {
        if (*element) == (self::ml_string::Text::from_units(vec![118, 97, 114])) {
            self::ml_string::Text::from_units(vec![119, 97, 105, 116])
        } else if (((((*element)
            == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))
            || ((*element) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))))
            || ((*element)
                == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
            || ((*element) == (self::ml_string::Text::from_units(vec![115, 101, 116]))))
            || ((*element) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
        {
            self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
        } else {
            self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
        }
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![118, 97, 114])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![108, 105, 116, 101, 114, 97, 108])))
    {
        self::ml_string::Text::from_units(vec![119, 97, 105, 116])
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))
    {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else if (((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112])))
    {
        self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 101])
    } else {
        self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_recursive_binding_iterable_error(
    absence: &self::ml_string::Text,
) -> self::ml_string::Text {
    (if (*absence) == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])) {
        self::ml_string::Text::from_units(vec![110, 117, 108, 108])
    } else {
        self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
    })
    .concat(&self::ml_string::Text::from_units(vec![
        32, 105, 115, 32, 110, 111, 116, 32, 105, 116, 101, 114, 97, 98, 108, 101, 32, 40, 99, 97,
        110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32,
        83, 121, 109, 98, 111, 108, 40, 83, 121, 109, 98, 111, 108, 46, 105, 116, 101, 114, 97,
        116, 111, 114, 41, 41,
    ]))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_pattern_initializer_anchor(plan: &[f64], row: f64) -> f64 {
    {
        let at = 2f64 + (row * 8f64);
        {
            let binding = self::read_required_number_fact(plan, (at + 3f64));
            if binding >= 0f64 {
                binding
            } else {
                {
                    let open = self::read_required_number_fact(plan, (at + 6f64));
                    {
                        let close = self::read_required_number_fact(plan, (at + 7f64));
                        {
                            let positions =
                                self::read_java_script_recursive_object_binding_positions(plan);
                            {
                                let index = 0f64;
                                {
                                    let source_ml_source2_s96 = self::source_ml_source1_temporary_read_java_script_pattern_initializer_anchor_loop44(open, close, &(positions), index);
                                    match source_ml_source2_s96 {
                                        self::MlSource1TemporaryReadJavaScriptPatternInitializerAnchorLoop44Result::MlSource1TemporaryReadJavaScriptPatternInitializerAnchorLoop44Done => {
                                            if (open >= 0f64) && (close > open) {
                                                open
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 105, 110, 100, 101, 114, 32, 111, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 97, 116, 116, 101, 114, 110, 32, 105, 110, 116, 101, 114, 118, 97, 108])).0, message_present: true }).raise()
                                            }
                                        }
                                        self::MlSource1TemporaryReadJavaScriptPatternInitializerAnchorLoop44Result::MlSource1TemporaryReadJavaScriptPatternInitializerAnchorLoop44Return(source_ml_source1_temporary_result44) => {
                                            source_ml_source1_temporary_result44
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
