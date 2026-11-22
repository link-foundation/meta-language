// Generated from complete js/src/translation/javascript-module-initialization.js. Do not edit by hand.
use super::JavaScriptModuleBindingRead;
use super::JavaScriptModuleBindingSlot;
use super::JavaScriptModuleInitializationPlan;
use super::JavaScriptModuleInitializationState;
use super::JavaScriptSemanticValue;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_module_binding_read;
use super::read_java_script_module_initialization_state_error;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_module_binding(
    plan: self::JavaScriptModuleInitializationPlan,
    state: self::JavaScriptModuleInitializationState,
    binding: f64,
) -> self::JavaScriptModuleBindingRead {
    match state.clone() {
        self::JavaScriptModuleInitializationState::ModuleInitializationState(
            _,
            _,
            _,
            _,
            shared_constructor_field3,
            _,
            shared_constructor_field4,
            shared_constructor_field5,
        ) => {
            let shared_constructor_field5 = *shared_constructor_field5;
            match plan.clone() {
                self::JavaScriptModuleInitializationPlan::ModuleInitializationPlan(
                    _,
                    _,
                    _,
                    shared_constructor_field1,
                    shared_constructor_field2,
                    _,
                ) => {
                    let error = self::read_java_script_module_initialization_state_error(
                        plan.clone(),
                        state.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptModuleBindingRead::ModuleBindingInvalid(error.clone())
                    } else {
                        {
                            let slot = {
                                let ml_values7 = shared_constructor_field4;
                                let ml_callback7 = {
                                    let binding = binding;
                                    std::rc::Rc::new(
                                        move |slot: self::JavaScriptModuleBindingSlot| -> bool {
                                            ((match slot.clone() {
                                    self::JavaScriptModuleBindingSlot::ModuleBindingSlot(source_ml_source1_data_property_value609, _, _) => {
                                        source_ml_source1_data_property_value609
                                    }
                                }) == binding)
                                        },
                                    )
                                };
                                ml_values7.iter().cloned().enumerate().find_map(
                                    |(ml_index7, ml_value7)| {
                                        if ml_callback7(ml_value7.clone()) {
                                            Some(ml_value7)
                                        } else {
                                            None
                                        }
                                    },
                                )
                            };
                            if match (slot.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::JavaScriptModuleBindingRead::ModuleBindingInvalid(
                                    self::ml_string::Text::from_units(vec![
                                        97, 32, 109, 111, 100, 117, 108, 101, 32, 114, 101, 97,
                                        100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105,
                                        116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110,
                                        101, 100, 32, 98, 105, 110, 100, 105, 110, 103,
                                    ]),
                                )
                            } else {
                                {
                                    let decision = self::read_java_script_module_binding_read(&(shared_constructor_field3), match slot.clone().expect("a proven present value") {
                                            self::JavaScriptModuleBindingSlot::ModuleBindingSlot(_, source_ml_source1_data_property_value611, _) => {
                                                source_ml_source1_data_property_value611
                                            }
                                        });
                                    if (decision < (0f64)) {
                                        self::JavaScriptModuleBindingRead::ModuleBindingFailure(
                                            Box::new(shared_constructor_field5.clone()),
                                        )
                                    } else {
                                        if (decision == (0f64)) {
                                            self::JavaScriptModuleBindingRead::ModuleBindingUninitialized(binding, (self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(self::ml_array::index_of(&shared_constructor_field1, &binding, 0f64, false, |a, b| a == b)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                                        } else {
                                            self::JavaScriptModuleBindingRead::ModuleBindingValue(Box::new(match slot.clone().expect("a proven present value") {
                                                    self::JavaScriptModuleBindingSlot::ModuleBindingSlot(_, _, source_ml_source1_data_property_value616) => {
                                                        let source_ml_source1_data_property_value616 = *source_ml_source1_data_property_value616;
                                                        source_ml_source1_data_property_value616.clone()
                                                    }
                                                }))
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
pub fn test_java_script_module_binding_alias(
    state: self::JavaScriptModuleInitializationState,
    first: f64,
    second: f64,
) -> bool {
    match state.clone() {
        self::JavaScriptModuleInitializationState::ModuleInitializationState(
            _,
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field1,
            _,
        ) => {
            let left = {
                let ml_values8 = shared_constructor_field1.clone();
                let ml_callback8 = {
                    let first = first;
                    std::rc::Rc::new(move |slot: self::JavaScriptModuleBindingSlot| -> bool {
                        ((match slot.clone() {
                            self::JavaScriptModuleBindingSlot::ModuleBindingSlot(
                                source_ml_source1_data_property_value619,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value619,
                        }) == first)
                    })
                };
                ml_values8
                    .iter()
                    .cloned()
                    .enumerate()
                    .find_map(|(ml_index8, ml_value8)| {
                        if ml_callback8(ml_value8.clone()) {
                            Some(ml_value8)
                        } else {
                            None
                        }
                    })
            };
            {
                let right = {
                    let ml_values9 = shared_constructor_field1;
                    let ml_callback9 = {
                        let second = second;
                        std::rc::Rc::new(move |slot: self::JavaScriptModuleBindingSlot| -> bool {
                            ((match slot.clone() {
                                self::JavaScriptModuleBindingSlot::ModuleBindingSlot(
                                    source_ml_source1_data_property_value622,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value622,
                            }) == second)
                        })
                    };
                    ml_values9
                        .iter()
                        .cloned()
                        .enumerate()
                        .find_map(|(ml_index9, ml_value9)| {
                            if ml_callback9(ml_value9.clone()) {
                                Some(ml_value9)
                            } else {
                                None
                            }
                        })
                };
                ((((!(match (left.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && !(match (right.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                })) && {
                    let source_ml_source1_nullish_field_receiver623 = (left.clone(),);
                    if match (source_ml_source1_nullish_field_receiver623.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_present624 =
                                ((source_ml_source1_nullish_field_receiver623.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            match source_ml_source1_nullish_field_present624.clone() {
                                self::JavaScriptModuleBindingSlot::ModuleBindingSlot(
                                    _,
                                    source_ml_source1_data_property_value625,
                                    _,
                                ) => source_ml_source1_data_property_value625,
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units: (self::ml_string::Text::from_units(vec![
                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114,
                                111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110,
                                100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105,
                                110, 103, 32, 39, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101,
                                100, 39, 41,
                            ]))
                            .0,
                            message_present: true,
                        })
                        .raise()
                    }
                }) && {
                    let source_ml_source1_nullish_field_receiver626 = (right.clone(),);
                    if match (source_ml_source1_nullish_field_receiver626.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_present627 =
                                ((source_ml_source1_nullish_field_receiver626.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            match source_ml_source1_nullish_field_present627.clone() {
                                self::JavaScriptModuleBindingSlot::ModuleBindingSlot(
                                    _,
                                    source_ml_source1_data_property_value628,
                                    _,
                                ) => source_ml_source1_data_property_value628,
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units: (self::ml_string::Text::from_units(vec![
                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114,
                                111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110,
                                100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105,
                                110, 103, 32, 39, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101,
                                100, 39, 41,
                            ]))
                            .0,
                            message_present: true,
                        })
                        .raise()
                    }
                }) && self::test_java_script_semantic_same_value(
                    {
                        let source_ml_source1_nullish_field_receiver629 = (left.clone(),);
                        if match (source_ml_source1_nullish_field_receiver629.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present630 =
                                    ((source_ml_source1_nullish_field_receiver629.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_nullish_field_present630.clone() {
                                    self::JavaScriptModuleBindingSlot::ModuleBindingSlot(
                                        _,
                                        _,
                                        source_ml_source1_data_property_value631,
                                    ) => {
                                        let source_ml_source1_data_property_value631 =
                                            *source_ml_source1_data_property_value631;
                                        source_ml_source1_data_property_value631.clone()
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 118, 97, 108, 117, 101, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    },
                    {
                        let source_ml_source1_nullish_field_receiver632 = (right.clone(),);
                        if match (source_ml_source1_nullish_field_receiver632.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present633 =
                                    ((source_ml_source1_nullish_field_receiver632.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_nullish_field_present633.clone() {
                                    self::JavaScriptModuleBindingSlot::ModuleBindingSlot(
                                        _,
                                        _,
                                        source_ml_source1_data_property_value634,
                                    ) => {
                                        let source_ml_source1_data_property_value634 =
                                            *source_ml_source1_data_property_value634;
                                        source_ml_source1_data_property_value634.clone()
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 118, 97, 108, 117, 101, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    },
                ))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(
    units: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < (units.len() as f64)) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at)))
                    .clone()
                    .unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64)))
                {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done,
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_text_equal_loop2(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut at: f64,
) -> self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    loop {
        return if (at < length) {
            {
                let a = if match left.clone() {
                    self::JavaScriptSemanticValue::Text(_) => true,
                    _ => false,
                } {
                    ({
                        let source_ml_source1_nullish_field_receiver645 = (
                            match left.clone() {
                                self::JavaScriptSemanticValue::Undefined => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Null => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Boolean(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Number(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Integer(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Text(
                                    source_ml_source1_data_property_value644,
                                ) => Some(source_ml_source1_data_property_value644),
                                self::JavaScriptSemanticValue::SourceText(_, _) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Symbol(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Object(_, _) => None::<Vec<f64>>,
                            },
                            at,
                        );
                        {
                            let source_ml_source1_computed_property_key646 =
                                (source_ml_source1_nullish_field_receiver645.clone()).1;
                            if match (source_ml_source1_nullish_field_receiver645.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source1_nullish_field_present647 =
                                        ((source_ml_source1_nullish_field_receiver645.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    self::ml_array::optional_at(
                                        &source_ml_source1_nullish_field_present647,
                                        self::ml_array::number_index(
                                            source_ml_source1_computed_property_key646,
                                        ),
                                    )
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (((self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39,
                                    ]))
                                    .concat(&self::ml_string::Text::from_text(
                                        &self::ml_number::js_number(
                                            (source_ml_source1_nullish_field_receiver645.clone()).1,
                                        ),
                                    )))
                                    .concat(&self::ml_string::Text::from_units(vec![39, 41])))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| (-1f64))
                } else {
                    if match left.clone() {
                        self::JavaScriptSemanticValue::SourceText(_, _) => true,
                        _ => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_receiver658 =
                                (match left.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Number(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Integer(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Text(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::SourceText(
                                        _,
                                        source_ml_source1_data_property_value657,
                                    ) => Some(source_ml_source1_data_property_value657.clone()),
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        None::<self::ml_string::Text>
                                    }
                                },);
                            if ((match (source_ml_source1_nullish_field_receiver658.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present;
                                    self::ml_string::Text::from_units(vec![
                                        115, 116, 114, 105, 110, 103,
                                    ])
                                }
                                None => self::ml_string::Text::from_units(vec![
                                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                                ]),
                            }) == (self::ml_string::Text::from_units(vec![
                                115, 116, 114, 105, 110, 103,
                            ]))) {
                                {
                                    let source_ml_source1_nullish_field_present659 =
                                        ((source_ml_source1_nullish_field_receiver658.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    {
                                        let source_ml_source1_string_unit_operand660 =
                                            source_ml_source1_nullish_field_present659.clone();
                                        {
                                            let source_ml_source1_string_unit_operand661 = at;
                                            (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if ((index < (0f64)) || (index >= (text.length()))) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                        let source_ml_source1_string_unit_operand667 = {
                                                            let source_ml_source1_string_range_operand662 = text.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand663 = index;
                                                                {
                                                                    let source_ml_source1_string_range_operand664 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source1_string_range_position665 = source_ml_source1_string_range_operand663;
                                                                        {
                                                                            let source_ml_source1_string_range_position666 = source_ml_source1_string_range_operand664;
                                                                            source_ml_source1_string_range_operand662.slice(source_ml_source1_string_range_position665, source_ml_source1_string_range_position666)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_string_unit_operand668 = 0f64;
                                                            source_ml_source1_string_unit_operand667.code_point_at(source_ml_source1_string_unit_operand668)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source1_string_unit_operand660.clone(), source_ml_source1_string_unit_operand661)
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 104, 97, 114,
                                        67, 111, 100, 101, 65, 116, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        }
                    } else {
                        (-1f64)
                    }
                };
                {
                    let b = if match right.clone() {
                        self::JavaScriptSemanticValue::Text(_) => true,
                        _ => false,
                    } {
                        ({
                            let source_ml_source1_nullish_field_receiver679 = (
                                match right.clone() {
                                    self::JavaScriptSemanticValue::Undefined => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Null => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Boolean(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Number(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Integer(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Text(
                                        source_ml_source1_data_property_value678,
                                    ) => Some(source_ml_source1_data_property_value678),
                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                        None::<Vec<f64>>
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Object(_, _) => None::<Vec<f64>>,
                                },
                                at,
                            );
                            {
                                let source_ml_source1_computed_property_key680 =
                                    (source_ml_source1_nullish_field_receiver679.clone()).1;
                                if match (source_ml_source1_nullish_field_receiver679.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source1_nullish_field_present681 =
                                            ((source_ml_source1_nullish_field_receiver679.clone())
                                                .0)
                                                .clone()
                                                .expect("a proven present value");
                                        self::ml_array::optional_at(
                                            &source_ml_source1_nullish_field_present681,
                                            self::ml_array::number_index(
                                                source_ml_source1_computed_property_key680,
                                            ),
                                        )
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (((self::ml_string::Text::from_units(
                                            vec![
                                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100,
                                                32, 112, 114, 111, 112, 101, 114, 116, 105, 101,
                                                115, 32, 111, 102, 32, 117, 110, 100, 101, 102,
                                                105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105,
                                                110, 103, 32, 39,
                                            ],
                                        ))
                                        .concat(&self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                (source_ml_source1_nullish_field_receiver679
                                                    .clone())
                                                .1,
                                            ),
                                        )))
                                        .concat(&self::ml_string::Text::from_units(vec![39, 41])))
                                        .0,
                                        message_present: true,
                                    })
                                    .raise()
                                }
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| (-1f64))
                    } else {
                        if match right.clone() {
                            self::JavaScriptSemanticValue::SourceText(_, _) => true,
                            _ => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_receiver692 =
                                    (match right.clone() {
                                        self::JavaScriptSemanticValue::Undefined => {
                                            None::<self::ml_string::Text>
                                        }
                                        self::JavaScriptSemanticValue::Null => {
                                            None::<self::ml_string::Text>
                                        }
                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                            None::<self::ml_string::Text>
                                        }
                                        self::JavaScriptSemanticValue::Number(_) => {
                                            None::<self::ml_string::Text>
                                        }
                                        self::JavaScriptSemanticValue::Integer(_) => {
                                            None::<self::ml_string::Text>
                                        }
                                        self::JavaScriptSemanticValue::Text(_) => {
                                            None::<self::ml_string::Text>
                                        }
                                        self::JavaScriptSemanticValue::SourceText(
                                            _,
                                            source_ml_source1_data_property_value691,
                                        ) => Some(source_ml_source1_data_property_value691.clone()),
                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                            None::<self::ml_string::Text>
                                        }
                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                            None::<self::ml_string::Text>
                                        }
                                    },);
                                if ((match (source_ml_source1_nullish_field_receiver692.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present;
                                        self::ml_string::Text::from_units(vec![
                                            115, 116, 114, 105, 110, 103,
                                        ])
                                    }
                                    None => self::ml_string::Text::from_units(vec![
                                        117, 110, 100, 101, 102, 105, 110, 101, 100,
                                    ]),
                                }) == (self::ml_string::Text::from_units(vec![
                                    115, 116, 114, 105, 110, 103,
                                ]))) {
                                    {
                                        let source_ml_source1_nullish_field_present693 =
                                            ((source_ml_source1_nullish_field_receiver692.clone())
                                                .0)
                                                .clone()
                                                .expect("a proven present value");
                                        {
                                            let source_ml_source1_string_unit_operand694 =
                                                source_ml_source1_nullish_field_present693.clone();
                                            {
                                                let source_ml_source1_string_unit_operand695 = at;
                                                (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                                    let index = if position.is_nan() {
                                                        0f64
                                                    } else {
                                                        position.trunc()
                                                    };
                                                    if ((index < (0f64)) || (index >= (text.length()))) {
                                                        f64::NAN
                                                    } else {
                                                        ({
                                                            let source_ml_source1_string_unit_operand701 = {
                                                                let source_ml_source1_string_range_operand696 = text.clone();
                                                                {
                                                                    let source_ml_source1_string_range_operand697 = index;
                                                                    {
                                                                        let source_ml_source1_string_range_operand698 = (index + 1f64);
                                                                        {
                                                                            let source_ml_source1_string_range_position699 = source_ml_source1_string_range_operand697;
                                                                            {
                                                                                let source_ml_source1_string_range_position700 = source_ml_source1_string_range_operand698;
                                                                                source_ml_source1_string_range_operand696.slice(source_ml_source1_string_range_position699, source_ml_source1_string_range_position700)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_string_unit_operand702 = 0f64;
                                                                source_ml_source1_string_unit_operand701.code_point_at(source_ml_source1_string_unit_operand702)
                                                            }
                                                        }).clone().unwrap_or_else(|| f64::NAN)
                                                    }
                                                } }))(source_ml_source1_string_unit_operand694.clone(), source_ml_source1_string_unit_operand695)
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (self::ml_string::Text::from_units(vec![
                                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32,
                                            112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32,
                                            111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101,
                                            100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39,
                                            99, 104, 97, 114, 67, 111, 100, 101, 65, 116, 39, 41,
                                        ]))
                                        .0,
                                        message_present: true,
                                    })
                                    .raise()
                                }
                            }
                        } else {
                            (-1f64)
                        }
                    };
                    if (a != b) {
                        self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(false)
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
            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Result {
    MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Done,
    MlTemporaryReadJavaScriptModuleInitializationPlanErrorLoop8Return(self::ml_string::Text),
}
