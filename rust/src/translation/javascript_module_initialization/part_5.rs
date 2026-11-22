// Generated from complete js/src/translation/javascript-module-initialization.js. Do not edit by hand.
use super::JavaScriptModuleBindingSlot;
use super::JavaScriptModuleInitializationPlan;
use super::JavaScriptModuleInitializationState;
use super::JavaScriptModuleInitializer;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Result;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_value_error;
use super::test_java_script_source_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_module_initialization_plan_error_loop8(
    mut plan: self::JavaScriptModuleInitializationPlan,
    mut assigned: Vec<f64>,
    mut source_ml_temporary_values6: Vec<self::JavaScriptModuleInitializer>,
    mut source_ml_temporary_index6: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Result {
    loop {
        return match plan.clone() {
            self::JavaScriptModuleInitializationPlan::ModuleInitializationPlan(
                _,
                _,
                _,
                shared_constructor_field1,
                _,
                _,
            ) => {
                if (source_ml_temporary_index6
                    < (self::ml::Big::from_u128(source_ml_temporary_values6.len() as u128)))
                {
                    {
                        let initializer = self::ml_array::at(
                            &source_ml_temporary_values6,
                            source_ml_temporary_index6.to_index(),
                        );
                        if (!(self::ml_array::includes(
                            &shared_constructor_field1,
                            &match initializer.clone() {
                                self::JavaScriptModuleInitializer::ModuleDataLoad(
                                    source_ml_source1_data_property_value706,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value706,
                                self::JavaScriptModuleInitializer::ModuleConstAlias(
                                    source_ml_source1_data_property_value707,
                                    _,
                                ) => source_ml_source1_data_property_value707,
                            },
                            0f64,
                            false,
                            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                        )) || self::ml_array::includes(
                            &assigned,
                            &match initializer.clone() {
                                self::JavaScriptModuleInitializer::ModuleDataLoad(
                                    source_ml_source1_data_property_value708,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value708,
                                self::JavaScriptModuleInitializer::ModuleConstAlias(
                                    source_ml_source1_data_property_value709,
                                    _,
                                ) => source_ml_source1_data_property_value709,
                            },
                            0f64,
                            false,
                            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                        )) {
                            self::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Result::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Return(self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 111, 114, 100, 101, 114, 32, 109, 117, 115, 116, 32, 97, 115, 115, 105, 103, 110, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 99, 101]))
                        } else {
                            {
                                let assigned_2 = {
                                    let (ml_append_right, ml_append_left) = (
                                        vec![match initializer.clone() {
                                            self::JavaScriptModuleInitializer::ModuleDataLoad(
                                                source_ml_source1_data_property_value710,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value710,
                                            self::JavaScriptModuleInitializer::ModuleConstAlias(
                                                source_ml_source1_data_property_value711,
                                                _,
                                            ) => source_ml_source1_data_property_value711,
                                        }],
                                        assigned,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                match initializer.clone() {
                                    self::JavaScriptModuleInitializer::ModuleDataLoad(
                                        _,
                                        specifier,
                                        reader_owner,
                                    ) => {
                                        if ((specifier
                                            == (self::ml_string::Text::from_units(vec![])))
                                            || (reader_owner
                                                == (self::ml_string::Text::from_units(vec![]))))
                                        {
                                            self::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Result::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Return(self::ml_string::Text::from_units(vec![97, 32, 100, 97, 116, 97, 32, 108, 111, 97, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 85, 82, 76, 32, 111, 112, 101, 114, 97, 110, 100, 32, 97, 110, 100, 32, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 114, 101, 97, 100, 101, 114, 32, 111, 119, 110, 101, 114]))
                                        } else {
                                            {
                                                let source_ml_temporary_index6_2 =
                                                    source_ml_temporary_index6
                                                        .add(&self::ml::Big::from_i128(1));
                                                {
                                                    (assigned, source_ml_temporary_index6) = (
                                                        assigned_2,
                                                        source_ml_temporary_index6_2.clone(),
                                                    );
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptModuleInitializer::ModuleConstAlias(
                                        _,
                                        target,
                                    ) => {
                                        if !(self::ml_array::includes(
                                            &shared_constructor_field1,
                                            &target,
                                            0f64,
                                            false,
                                            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                                        )) {
                                            self::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Result::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 32, 97, 108, 105, 97, 115, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 109, 111, 100, 117, 108, 101, 32, 98, 105, 110, 100, 105, 110, 103]))
                                        } else {
                                            {
                                                let source_ml_temporary_index6_3 =
                                                    source_ml_temporary_index6
                                                        .add(&self::ml::Big::from_i128(1));
                                                {
                                                    (assigned, source_ml_temporary_index6) = (
                                                        assigned_2,
                                                        source_ml_temporary_index6_3.clone(),
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
                    self::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Result::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop7Result {
    MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop7Done,
    MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop7Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_module_initialization_plan_error_loop7(
    mut plan: self::JavaScriptModuleInitializationPlan,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop7Result {
    loop {
        return match plan.clone() {
            self::JavaScriptModuleInitializationPlan::ModuleInitializationPlan(
                _,
                _,
                _,
                shared_constructor_field1,
                shared_constructor_field2,
                _,
            ) => {
                if (at < (shared_constructor_field1.len() as f64)) {
                    {
                        let binding = (self::ml_array::optional_at(
                            &shared_constructor_field1,
                            self::ml_array::number_index(at),
                        ))
                        .clone()
                        .unwrap_or_else(|| (-1f64));
                        {
                            let name = (self::ml_array::optional_at(
                                &shared_constructor_field2,
                                self::ml_array::number_index(at),
                            ))
                            .clone()
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                            if (((!(self::test_java_script_source_coordinate(binding, 65536f64))
                                || ((self::ml_array::index_of(
                                    &shared_constructor_field1,
                                    &binding,
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                )) != at))
                                || (name == (self::ml_string::Text::from_units(vec![]))))
                                || ((self::ml_array::index_of(
                                    &shared_constructor_field2,
                                    &name.clone(),
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                )) != at))
                            {
                                self::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop7Result::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop7Return(self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101, 32, 99, 111, 110, 115, 116, 32, 98, 105, 110, 100, 105, 110, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 97, 110, 100, 32, 110, 97, 109, 101, 115]))
                            } else {
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop7Result::MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop7Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10 {
    MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10Next,
    MlTemporaryReadJavaScriptModuleInitializationStateErrorJoin10Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Result {
    MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Done,
    MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_module_initialization_state_error_loop9(
    mut plan: self::JavaScriptModuleInitializationPlan,
    mut state: self::JavaScriptModuleInitializationState,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Result {
    loop {
        return match state.clone() {
            self::JavaScriptModuleInitializationState::ModuleInitializationState(
                _,
                _,
                _,
                _,
                _,
                shared_constructor_field3,
                shared_constructor_field4,
                _,
            ) => {
                match plan.clone() {
                    self::JavaScriptModuleInitializationPlan::ModuleInitializationPlan(
                        _,
                        _,
                        _,
                        shared_constructor_field1,
                        _,
                        shared_constructor_field2,
                    ) => {
                        if (at < (shared_constructor_field4.len() as f64)) {
                            {
                                let slot = self::ml_array::optional_at(
                                    &shared_constructor_field4,
                                    self::ml_array::number_index(at),
                                );
                                {
                                    let binding = (self::ml_array::optional_at(
                                        &shared_constructor_field1,
                                        self::ml_array::number_index(at),
                                    ))
                                    .clone()
                                    .unwrap_or_else(|| (-1f64));
                                    if ((match (slot.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } || (({
                                        let source_ml_source1_nullish_field_receiver735 =
                                            (slot.clone(),);
                                        if match (source_ml_source1_nullish_field_receiver735
                                            .clone())
                                        .0
                                        {
                                            Some(ml_present) => {
                                                let _ = ml_present.clone();
                                                true
                                            }
                                            None => false,
                                        } {
                                            {
                                                let source_ml_source1_nullish_field_present736 =
                                                    ((source_ml_source1_nullish_field_receiver735
                                                        .clone())
                                                    .0)
                                                        .clone()
                                                        .expect("a proven present value");
                                                match source_ml_source1_nullish_field_present736.clone() {
                                                    self::JavaScriptModuleBindingSlot::ModuleBindingSlot(source_ml_source1_data_property_value737, _, _) => {
                                                        source_ml_source1_data_property_value737
                                                    }
                                                }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException {
                                                class_name: String::from("TypeError"),
                                                message_units: (self::ml_string::Text::from_units(
                                                    vec![
                                                        67, 97, 110, 110, 111, 116, 32, 114, 101,
                                                        97, 100, 32, 112, 114, 111, 112, 101, 114,
                                                        116, 105, 101, 115, 32, 111, 102, 32, 117,
                                                        110, 100, 101, 102, 105, 110, 101, 100, 32,
                                                        40, 114, 101, 97, 100, 105, 110, 103, 32,
                                                        39, 98, 105, 110, 100, 105, 110, 103, 39,
                                                        41,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
                                        }
                                    }) != binding))
                                        || ((self::read_java_script_semantic_value_error({
                                            let source_ml_source1_nullish_field_receiver741 =
                                                (slot.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver741
                                                .clone())
                                            .0
                                            {
                                                Some(ml_present) => {
                                                    let _ = ml_present.clone();
                                                    true
                                                }
                                                None => false,
                                            } {
                                                {
                                                    let source_ml_source1_nullish_field_present742 = ((source_ml_source1_nullish_field_receiver741.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present742.clone() {
                                                    self::JavaScriptModuleBindingSlot::ModuleBindingSlot(_, _, source_ml_source1_data_property_value743) => {
                                                        let source_ml_source1_data_property_value743 = *source_ml_source1_data_property_value743;
                                                        source_ml_source1_data_property_value743.clone()
                                                    }
                                                }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException {
                                                    class_name: String::from("TypeError"),
                                                    message_units:
                                                        (self::ml_string::Text::from_units(vec![
                                                            67, 97, 110, 110, 111, 116, 32, 114,
                                                            101, 97, 100, 32, 112, 114, 111, 112,
                                                            101, 114, 116, 105, 101, 115, 32, 111,
                                                            102, 32, 117, 110, 100, 101, 102, 105,
                                                            110, 101, 100, 32, 40, 114, 101, 97,
                                                            100, 105, 110, 103, 32, 39, 118, 97,
                                                            108, 117, 101, 39, 41,
                                                        ]))
                                                        .0,
                                                    message_present: true,
                                                })
                                                .raise()
                                            }
                                        })) != (self::ml_string::Text::from_units(vec![]))))
                                    {
                                        self::MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Result::MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Return(self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101, 32, 115, 108, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 118, 97, 108, 117, 101, 115]))
                                    } else {
                                        {
                                            let position = {
                                                let ml_values10 = shared_constructor_field2;
                                                let ml_callback10 = {
                                                    let binding = binding;
                                                    std::rc::Rc::new(move |row: self::JavaScriptModuleInitializer| -> bool { ((match row.clone() {
                                                self::JavaScriptModuleInitializer::ModuleDataLoad(source_ml_source1_data_property_value747, _, _) => {
                                                    source_ml_source1_data_property_value747
                                                }
                                                self::JavaScriptModuleInitializer::ModuleConstAlias(source_ml_source1_data_property_value748, _) => {
                                                    source_ml_source1_data_property_value748
                                                }
                                            }) == binding) })
                                                };
                                                ml_values10
                                                    .iter()
                                                    .cloned()
                                                    .enumerate()
                                                    .position(|(ml_index10, ml_value10)| {
                                                        ml_callback10(ml_value10)
                                                    })
                                                    .map_or(-1f64, |ml_position| ml_position as f64)
                                            };
                                            if ((({
                                                let source_ml_source1_nullish_field_receiver754 =
                                                    (slot.clone(),);
                                                if match (source_ml_source1_nullish_field_receiver754.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source1_nullish_field_present755 = ((source_ml_source1_nullish_field_receiver754.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source1_nullish_field_present755.clone() {
                                                            self::JavaScriptModuleBindingSlot::ModuleBindingSlot(_, source_ml_source1_data_property_value756, _) => {
                                                                source_ml_source1_data_property_value756
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 39, 41])).0, message_present: true }).raise()
                                                }
                                            }) != (position < shared_constructor_field3))
                                                || (!({
                                                    let source_ml_source1_nullish_field_receiver759 =
                                                        (slot.clone(),);
                                                    if match (source_ml_source1_nullish_field_receiver759.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source1_nullish_field_present760 = ((source_ml_source1_nullish_field_receiver759.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source1_nullish_field_present760.clone() {
                                                            self::JavaScriptModuleBindingSlot::ModuleBindingSlot(_, source_ml_source1_data_property_value761, _) => {
                                                                source_ml_source1_data_property_value761
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 39, 41])).0, message_present: true }).raise()
                                                }
                                                }) && {
                                                    let source_ml_source1_s765 = {
                                                        let source_ml_source1_nullish_field_receiver775 =
                                                            (slot.clone(),);
                                                        if match (source_ml_source1_nullish_field_receiver775.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        {
                                                            let source_ml_source1_nullish_field_present776 = ((source_ml_source1_nullish_field_receiver775.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_source1_nullish_field_present776.clone() {
                                                                self::JavaScriptModuleBindingSlot::ModuleBindingSlot(_, _, source_ml_source1_data_property_value777) => {
                                                                    let source_ml_source1_data_property_value777 = *source_ml_source1_data_property_value777;
                                                                    source_ml_source1_data_property_value777.clone()
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 118, 97, 108, 117, 101, 39, 41])).0, message_present: true }).raise()
                                                    }
                                                    };
                                                    match source_ml_source1_s765.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                                }))
                                            {
                                                self::MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Result::MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Return(self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101, 32, 115, 108, 111, 116, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114, 101, 102, 105, 120, 32, 97, 110, 100, 32, 116, 101, 109, 112, 111, 114, 97, 108, 32, 100, 101, 97, 100, 32, 122, 111, 110, 101, 115]))
                                            } else {
                                                {
                                                    let at_2 = (at + 1f64);
                                                    {
                                                        at = at_2;
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Result::MlTemporaryReadJavaScriptModuleInitializationStateErrorLoop9Done
                        }
                    }
                }
            }
        };
    }
}
