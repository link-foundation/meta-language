// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptArrayPropertyPlan;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryAcceptStringNumberInfinityLoop157Result;
use super::MlSource1TemporaryConvertStringNumberRationalJoin149;
use super::MlSource1TemporaryConvertStringNumberRationalJoin150;
use super::MlSource1TemporaryConvertStringNumberRationalLoop151Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop152Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop155Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop156Result;
use super::MlSource1TemporaryReadStringNumberBinaryPowerLoop147Result;
use super::convert_java_script_array_unsigned_length;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_array_length_definition;
use super::plan_java_script_array_property_definition;
use super::read_java_script_array_definition_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_source1_temporary_accept_string_number_infinity_loop157;
use super::source_ml_source1_temporary_convert_string_number_rational_loop151;
use super::source_ml_source1_temporary_convert_string_number_rational_loop152;
use super::source_ml_source1_temporary_convert_string_number_rational_loop153;
use super::source_ml_source1_temporary_convert_string_number_rational_loop154;
use super::source_ml_source1_temporary_convert_string_number_rational_loop155;
use super::source_ml_source1_temporary_convert_string_number_rational_loop156;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop147;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop148;
use super::source_ml_source1_temporary_read_string_number_decimal_power_loop145;
use super::source_ml_source1_temporary_read_string_number_exact_mantissa_loop146;
use super::test_java_script_array_length_key;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_property_definition(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayDefinitionState,
) -> self::JavaScriptArrayCompletion {
    {
        let error =
            self::read_java_script_array_definition_state_error(heap.clone(), state.clone());
        {
            let ml_library_inequality_2 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), error.clone())
            } else if (match state.clone() {
                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value17509,
                    _,
                    _,
                ) => source_ml_source2_data_property_value17509.clone(),
            }) != (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))
            {
                self::JavaScriptArrayCompletion::ArrayNumberRequest(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                    Box::new(
                        ({
                            (match state.clone() {
                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value17510,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value17510.clone(),
                            })
                            .borrow()
                            .clone()
                        })
                        .8,
                    ),
                )
            } else {
                {
                    let object = {
                        let source_ml_source2_array_read_value17514 = match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source2_data_property_value17511,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value17511.clone(),
                        };
                        {
                            let source_ml_source2_array_read_index17515 = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value17512, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value17512.clone()
                                }
                            }, match state.clone() {
                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source2_data_property_value17513, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value17513.clone()
                                }
                            });
                            {
                                let ml_reference_array197 = source_ml_source2_array_read_value17514;
                                let ml_reference_position197 =
                                    source_ml_source2_array_read_index17515;
                                self::ml_array::optional_at(
                                    &*ml_reference_array197.borrow(),
                                    self::ml_array::number_index(ml_reference_position197),
                                )
                            }
                        }
                    };
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } || match object.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => false,
                        _ => true,
                    } {
                        self::JavaScriptArrayCompletion::ArrayInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 114, 101, 99, 101, 105, 118, 101, 114,
                                32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105,
                                110, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 103,
                                108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114,
                                121,
                            ]),
                        )
                    } else {
                        {
                            let number_length = match state.clone() {
                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value17611,
                                ) => {
                                    let source_ml_source2_data_property_value17611 =
                                        *source_ml_source2_data_property_value17611;
                                    source_ml_source2_data_property_value17611.clone()
                                }
                            };
                            match number_length {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::Number(value) => {
                                    let plan = if self::test_java_script_array_length_key(match state.clone() {
                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source2_data_property_value17620, _, _, _, _) => {
                                                let source_ml_source2_data_property_value17620 = *source_ml_source2_data_property_value17620;
                                                source_ml_source2_data_property_value17620.clone()
                                            }
                                        }) && ({ (match state.clone() {
                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17621, _, _, _) => {
                                                source_ml_source2_data_property_value17621.clone()
                                            }
                                        }).borrow().clone() }).9 {
                                            self::plan_java_script_array_length_definition(object.clone().expect("a proven present value"), match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17622, _, _, _) => {
                                                    source_ml_source2_data_property_value17622.clone()
                                                }
                                            }, match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, _, source_ml_source2_data_property_value17623, _) => {
                                                    source_ml_source2_data_property_value17623
                                                }
                                            }, value)
                                        } else {
                                            self::plan_java_script_array_property_definition(object.clone().expect("a proven present value"), match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source2_data_property_value17624, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value17624 = *source_ml_source2_data_property_value17624;
                                                    source_ml_source2_data_property_value17624.clone()
                                                }
                                            }, match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17625, _, _, _) => {
                                                    source_ml_source2_data_property_value17625.clone()
                                                }
                                            })
                                        };
                                    if ((match plan.clone() {
                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value17627,
                                            _,
                                        ) => source_ml_source2_data_property_value17627.clone(),
                                    }) == (self::ml_string::Text::from_units(vec![
                                        82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                                    ]))) || ((match plan.clone() {
                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value17629,
                                            _,
                                        ) => source_ml_source2_data_property_value17629.clone(),
                                    }) == (self::ml_string::Text::from_units(vec![
                                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                                    ]))) {
                                        self::JavaScriptArrayCompletion::ArrayError(Box::new(heap.clone()), match plan.clone() {
                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, source_ml_source2_data_property_value17630, _) => {
                                                    source_ml_source2_data_property_value17630.clone()
                                                }
                                            }, match plan.clone() {
                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, source_ml_source2_data_property_value17631, _, _) => {
                                                    source_ml_source2_data_property_value17631.clone()
                                                }
                                            })
                                    } else if (match plan.clone() {
                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value17633,
                                            _,
                                        ) => source_ml_source2_data_property_value17633.clone(),
                                    }) == (self::ml_string::Text::from_units(vec![
                                        114, 101, 115, 111, 117, 114, 99, 101,
                                    ])) {
                                        {
                                            let source_ml_source2_constructor_field17636 = match plan.clone() {
                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, source_ml_source2_data_property_value17635, _, _) => {
                                                            source_ml_source2_data_property_value17635.clone()
                                                        }
                                                    };
                                            {
                                                let source_ml_source2_constructor_field17637 =
                                                    Some(state.clone());
                                                self::JavaScriptArrayCompletion::ArrayResourceLimit(
                                                    Box::new(heap.clone()),
                                                    source_ml_source2_constructor_field17636,
                                                    Some(source_ml_source2_constructor_field17637),
                                                )
                                            }
                                        }
                                    } else if (match plan.clone() {
                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value17639,
                                            _,
                                        ) => source_ml_source2_data_property_value17639.clone(),
                                    }) != (self::ml_string::Text::from_units(vec![]))
                                    {
                                        self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), match plan.clone() {
                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, source_ml_source2_data_property_value17640, _, _) => {
                                                            source_ml_source2_data_property_value17640.clone()
                                                        }
                                                    })
                                    } else {
                                        {
                                            let updated = self::replace_java_script_ordinary_object(heap.clone(), match plan.clone() {
                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source2_data_property_value17641, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value17641 = *source_ml_source2_data_property_value17641;
                                                                source_ml_source2_data_property_value17641.clone()
                                                            }
                                                        });
                                            {
                                                let invalid =
                                                    self::read_java_script_ordinary_heap_error(
                                                        updated.clone(),
                                                    );
                                                {
                                                    let ml_library_inequality_1 = invalid
                                                        != (self::ml_string::Text::from_units(
                                                            vec![],
                                                        ));
                                                    if ml_library_inequality_1 {
                                                        {
                                                            let source_ml_source2_constructor_field17644 =
                                                                Some(state.clone());
                                                            self::JavaScriptArrayCompletion::ArrayResourceLimit(Box::new(heap.clone()), invalid.clone(), Some(source_ml_source2_constructor_field17644,))
                                                        }
                                                    } else {
                                                        self::JavaScriptArrayCompletion::ArrayPropertyComplete(Box::new(updated.clone()), match plan.clone() {
                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, source_ml_source2_data_property_value17645, _, _, _) => {
                                                                        source_ml_source2_data_property_value17645
                                                                    }
                                                                }, match plan.clone() {
                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source2_data_property_value17646) => {
                                                                        source_ml_source2_data_property_value17646.clone()
                                                                    }
                                                                })
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116,
                                            104, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                                            109, 97, 110, 116, 105, 99, 32, 78, 117, 109, 98, 101,
                                            114,
                                        ]),
                                    )
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
pub fn resume_java_script_array_property_number(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayDefinitionState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayCompletion {
    {
        let error =
            self::read_java_script_array_definition_state_error(heap.clone(), state.clone());
        {
            let ml_library_inequality_2 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), error.clone())
            } else {
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    {
                        let ml_library_inequality_1 =
                            invalid != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            self::JavaScriptArrayCompletion::ArrayInvalid(
                                Box::new(heap.clone()),
                                invalid.clone(),
                            )
                        } else {
                            match value.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(value_value) => {
                            if (match state.clone() {
                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, source_ml_source2_data_property_value17656, _, _) => {
                                    source_ml_source2_data_property_value17656.clone()
                                }
                            }) == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])) {
                                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                            } else {
                                {
                                    let next = if (match state.clone() {
                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, source_ml_source2_data_property_value17658, _, _) => {
                                            source_ml_source2_data_property_value17658.clone()
                                        }
                                    }) == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 32, 110, 117, 109, 98, 101, 114])) {
                                        {
                                            let source_ml_source2_nominal_field17660 = match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source2_data_property_value17661, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value17661.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17662 = match state.clone() {
                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source2_data_property_value17663, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value17663 = *source_ml_source2_data_property_value17663;
                                                        source_ml_source2_data_property_value17663.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17664 = match state.clone() {
                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17665, _, _, _) => {
                                                            source_ml_source2_data_property_value17665.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17666 = match state.clone() {
                                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, source_ml_source2_data_property_value17667, _, _) => {
                                                                source_ml_source2_data_property_value17667.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17668 = match state.clone() {
                                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, _, source_ml_source2_data_property_value17669, _) => {
                                                                    source_ml_source2_data_property_value17669
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17670 = match state.clone() {
                                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, _, _, source_ml_source2_data_property_value17671) => {
                                                                        let source_ml_source2_data_property_value17671 = *source_ml_source2_data_property_value17671;
                                                                        source_ml_source2_data_property_value17671.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_override17673 = self::convert_java_script_array_unsigned_length(value_value);
                                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source2_nominal_field17660, Box::new(source_ml_source2_nominal_field17662), source_ml_source2_nominal_field17664, self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 32, 110, 117, 109, 98, 101, 114]), source_ml_source2_nominal_override17673, Box::new(source_ml_source2_nominal_field17670))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let source_ml_source2_nominal_field17675 = match state.clone() {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source2_data_property_value17676, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value17676.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field17677 = match state.clone() {
                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source2_data_property_value17678, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value17678 = *source_ml_source2_data_property_value17678;
                                                        source_ml_source2_data_property_value17678.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field17679 = match state.clone() {
                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value17680, _, _, _) => {
                                                            source_ml_source2_data_property_value17680.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field17681 = match state.clone() {
                                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, source_ml_source2_data_property_value17682, _, _) => {
                                                                source_ml_source2_data_property_value17682.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field17683 = match state.clone() {
                                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, _, source_ml_source2_data_property_value17684, _) => {
                                                                    source_ml_source2_data_property_value17684
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field17685 = match state.clone() {
                                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, _, _, source_ml_source2_data_property_value17686) => {
                                                                        let source_ml_source2_data_property_value17686 = *source_ml_source2_data_property_value17686;
                                                                        source_ml_source2_data_property_value17686.clone()
                                                                    }
                                                                };
                                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source2_nominal_field17675, Box::new(source_ml_source2_nominal_field17677), source_ml_source2_nominal_field17679, self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]), source_ml_source2_nominal_field17683, Box::new(value.clone()))
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    self::JavaScriptArrayCompletion::ArrayDefinitionContinue(Box::new(heap.clone()), Box::new(next))
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 99, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
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
pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    (((((((((((9f64..=13f64).contains(&unit)) || (unit == 32f64)) || (unit == 160f64))
        || (unit == 5760f64))
        || ((8192f64..=8202f64).contains(&unit)))
        || (unit == 8232f64))
        || (unit == 8233f64))
        || (unit == 8239f64))
        || (unit == 8287f64))
        || (unit == 12_288f64))
        || (unit == 65_279f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_numeric_digit_unit(unit: f64) -> f64 {
    if (48f64..=57f64).contains(&unit) {
        unit - 48f64
    } else if (65f64..=70f64).contains(&unit) {
        unit - 55f64
    } else if (97f64..=102f64).contains(&unit) {
        unit - 87f64
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_digit(unit: f64) -> f64 {
    self::read_java_script_numeric_digit_unit(unit)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_decimal_power(exponent: f64) -> self::ml::Big {
    {
        let value = self::ml::Big::from_i128(1);
        {
            let base = self::ml::Big::from_i128(10);
            {
                self::source_ml_source1_temporary_read_string_number_decimal_power_loop145(
                    exponent, value, base,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_exact_mantissa(value: self::ml::Big) -> f64 {
    {
        let result = 0f64;
        {
            let scale = 1f64;
            {
                self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop146(
                    value.clone(),
                    result,
                    scale,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_binary_power(exponent: f64) -> f64 {
    {
        let result = 1f64;
        {
            let source_ml_source2_s17915 =
                self::source_ml_source1_temporary_read_string_number_binary_power_loop147(
                    exponent, result,
                );
            match source_ml_source2_s17915 {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop147Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop147Done(exponent_2, result_2) => {
                    {
                        self::source_ml_source1_temporary_read_string_number_binary_power_loop148(exponent_2, result_2)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_string_number_rational(numerator: self::ml::Big, denominator: self::ml::Big) -> f64 {
    if (numerator < (self::ml::Big::from_i128(0))) || (denominator <= (self::ml::Big::from_i128(0)))
    {
        f64::NAN
    } else if numerator == (self::ml::Big::from_i128(0)) {
        0f64
    } else {
        {
            let exponent = 0f64;
            {
                let source_ml_source2_s17918 = if numerator >= denominator {
                    {
                        let source_ml_source2_s17919 = self::source_ml_source1_temporary_convert_string_number_rational_loop155(numerator.clone(), exponent, denominator.clone());
                        match source_ml_source2_s17919 {
                                self::MlSource1TemporaryConvertStringNumberRationalLoop155Result::MlSource1TemporaryConvertStringNumberRationalLoop155Done(exponent_2) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin149::MlSource1TemporaryConvertStringNumberRationalJoin149Next(exponent_2)
                                }
                                self::MlSource1TemporaryConvertStringNumberRationalLoop155Result::MlSource1TemporaryConvertStringNumberRationalLoop155Return(source_ml_source1_temporary_result155) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin149::MlSource1TemporaryConvertStringNumberRationalJoin149Return(source_ml_source1_temporary_result155)
                                }
                            }
                    }
                } else {
                    {
                        let source_ml_source2_s17920 = self::source_ml_source1_temporary_convert_string_number_rational_loop156(denominator.clone(), exponent, numerator.clone());
                        match source_ml_source2_s17920 {
                                self::MlSource1TemporaryConvertStringNumberRationalLoop156Result::MlSource1TemporaryConvertStringNumberRationalLoop156Done(exponent_3) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin149::MlSource1TemporaryConvertStringNumberRationalJoin149Next(exponent_3)
                                }
                                self::MlSource1TemporaryConvertStringNumberRationalLoop156Result::MlSource1TemporaryConvertStringNumberRationalLoop156Return(source_ml_source1_temporary_result156) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin149::MlSource1TemporaryConvertStringNumberRationalJoin149Return(source_ml_source1_temporary_result156)
                                }
                            }
                    }
                };
                match source_ml_source2_s17918 {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin149::MlSource1TemporaryConvertStringNumberRationalJoin149Next(exponent_4) => {
                            {
                                let shift = if exponent_4 >= -1022f64 {
                                    52f64 - exponent_4
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s17921 = if shift >= 0f64 {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop153(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin150::MlSource1TemporaryConvertStringNumberRationalJoin150Next(numerator_2, denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop154(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin150::MlSource1TemporaryConvertStringNumberRationalJoin150Next(numerator.clone(), denominator_2)
                                            }
                                        }
                                    };
                                    match source_ml_source2_s17921 {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin150::MlSource1TemporaryConvertStringNumberRationalJoin150Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = self::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale = denominator_3.clone();
                                                        {
                                                            let weight = self::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s17922 = self::source_ml_source1_temporary_convert_string_number_rational_loop151(remainder.clone(), scale, weight);
                                                                match source_ml_source2_s17922 {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop151Result::MlSource1TemporaryConvertStringNumberRationalLoop151Done(scale_2, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s17923 = self::source_ml_source1_temporary_convert_string_number_rational_loop152(quotient, remainder.clone(), scale_2.clone(), weight_2.clone());
                                                                            match source_ml_source2_s17923 {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop152Result::MlSource1TemporaryConvertStringNumberRationalLoop152Done(quotient_2, remainder_2) => {
                                                                                    {
                                                                                        let twice = remainder_2.mul(&self::ml::Big::from_i128(2));
                                                                                        {
                                                                                            let quotient_4 = if (twice > denominator_3) || ((twice == denominator_3) && ((self::ml::divide(&quotient_2, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1)))) {
                                                                                                {
                                                                                                    quotient_2.add(&self::ml::Big::from_i128(1))
                                                                                                }
                                                                                            } else {
                                                                                                quotient_2.clone()
                                                                                            };
                                                                                            (self::read_string_number_exact_mantissa(quotient_4) * self::read_string_number_binary_power(-shift))
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
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryConvertStringNumberRationalJoin149::MlSource1TemporaryConvertStringNumberRationalJoin149Return(source_ml_source1_temporary_result154) => {
                            source_ml_source1_temporary_result154
                        }
                    }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_string_number_infinity(
    units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    start: f64,
    end: f64,
) -> bool {
    {
        let ml_library_inequality_1 = (end - start) != 8f64;
        if ml_library_inequality_1 {
            false
        } else {
            {
                let expected = std::rc::Rc::new(std::cell::RefCell::new(vec![
                    73f64, 110f64, 102f64, 105f64, 110f64, 105f64, 116f64, 121f64,
                ]));
                {
                    let index = 0f64;
                    {
                        let source_ml_source2_s17924 =
                            self::source_ml_source1_temporary_accept_string_number_infinity_loop157(
                                units.clone(),
                                start,
                                expected,
                                index,
                            );
                        match source_ml_source2_s17924 {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop157Result::MlSource1TemporaryAcceptStringNumberInfinityLoop157Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop157Result::MlSource1TemporaryAcceptStringNumberInfinityLoop157Return(source_ml_source1_temporary_result157) => {
                            source_ml_source1_temporary_result157
                        }
                    }
                    }
                }
            }
        }
    }
}
