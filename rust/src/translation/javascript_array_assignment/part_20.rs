// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayAssignmentTransition;
use super::JavaScriptArrayCompletion;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::apply_java_script_primitive_conversion_method;
use super::begin_java_script_array_property_definition;
use super::begin_java_script_primitive_conversion;
use super::continue_java_script_primitive_conversion;
use super::execute_java_script_call_scalar_conversion;
use super::ml;
use super::ml_string;
use super::read_java_script_array_definition_state_error;
use super::read_java_script_call_scalar_conversion_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::read_java_script_semantic_value_error;
use super::resume_java_script_array_property_number;
use super::step_java_script_primitive_conversion;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_primitive_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let error =
            self::read_java_script_primitive_conversion_state_error(heap.clone(), state.clone());
        {
            let ml_library_inequality_2 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
            } else {
                {
                    let getter = (((match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value18228,
                        ) => source_ml_source2_data_property_value18228.clone(),
                    }) == (self::ml_string::Text::from_units(vec![
                        101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114,
                    ]))) || ((match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value18230,
                        ) => source_ml_source2_data_property_value18230.clone(),
                    }) == (self::ml_string::Text::from_units(vec![
                        102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114,
                    ]))))
                        || ((match state.clone() {
                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value18232,
                            ) => source_ml_source2_data_property_value18232.clone(),
                        }) == (self::ml_string::Text::from_units(vec![
                            115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114,
                        ])));
                    {
                        let call = (((match state.clone() {
                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value18234,
                            ) => source_ml_source2_data_property_value18234.clone(),
                        }) == (self::ml_string::Text::from_units(vec![
                            101, 120, 111, 116, 105, 99, 67, 97, 108, 108,
                        ]))) || ((match state.clone() {
                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value18236,
                            ) => source_ml_source2_data_property_value18236.clone(),
                        }) == (self::ml_string::Text::from_units(vec![
                            102, 105, 114, 115, 116, 67, 97, 108, 108,
                        ]))))
                            || ((match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value18238,
                                ) => source_ml_source2_data_property_value18238.clone(),
                            }) == (self::ml_string::Text::from_units(vec![
                                115, 101, 99, 111, 110, 100, 67, 97, 108, 108,
                            ])));
                        if ((getter) || (call)) {
                            {
                                let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value18239) => {
                                    let source_ml_source2_data_property_value18239 = *source_ml_source2_data_property_value18239;
                                    source_ml_source2_data_property_value18239.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value18240) => {
                                    let source_ml_source2_data_property_value18240 = *source_ml_source2_data_property_value18240;
                                    source_ml_source2_data_property_value18240.clone()
                                }
                            });
                                {
                                    let ml_library_inequality_1 =
                                        value_error != (self::ml_string::Text::from_units(vec![]));
                                    if ml_library_inequality_1 {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(value_error.clone())
                                    } else {
                                        match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        if getter {
                                            self::apply_java_script_primitive_conversion_method(state.clone(), value.clone())
                                        } else if match value.clone() {
                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            } {
                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(value.clone()))
                                            } else if (match state.clone() {
                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source2_data_property_value18251) => {
                                                        source_ml_source2_data_property_value18251.clone()
                                                    }
                                                }) == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108])) {
                                                    self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                                } else {
                                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), if (match state.clone() {
                                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source2_data_property_value18253) => {
                                                            source_ml_source2_data_property_value18253.clone()
                                                        }
                                                    }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108])) {
                                                        self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 117, 115, 116, 32, 114, 101, 116, 117, 114, 110, 32, 97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101])
                                                    } else {
                                                        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115])
                                                    })
                                                }
                                    }
                                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                        let value_2 = *value_2;
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(Box::new(value_2.clone()))
                                    }
                                }
                                    }
                                }
                            }
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 114, 101, 115,
                                    117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115,
                                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101,
                                    110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114, 32,
                                    111, 114, 32, 109, 101, 116, 104, 111, 100,
                                ]),
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_scalar_protocol_operation_error(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    self::read_java_script_call_scalar_conversion_error(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_scalar_protocol_hint(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
            116, 101, 120, 116,
        ]))
    {
        self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
            118, 97, 108, 117, 101,
        ]))
    {
        self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])
    } else {
        self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_scalar_protocol(
    operation: &self::ml_string::Text,
    transition: self::JavaScriptPrimitiveConversionTransition,
) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
            } else {
                match transition.clone() {
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(state) => {
                        let state = *state;
                        self::JavaScriptScalarConversionTransition::ScalarContinuation(Box::new(
                            state.clone(),
                        ))
                    }
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(
                        state_2,
                        callee,
                        receiver,
                        arguments,
                    ) => {
                        let state_2 = *state_2;
                        let callee = *callee;
                        let receiver = *receiver;
                        self::JavaScriptScalarConversionTransition::ScalarCall(
                            Box::new(state_2.clone()),
                            Box::new(callee.clone()),
                            Box::new(receiver.clone()),
                            arguments.clone(),
                        )
                    }
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(value) => {
                        let value = *value;
                        self::finish_java_script_scalar_primitive(operation, value.clone())
                    }
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(value_2) => {
                        let value_2 = *value_2;
                        self::JavaScriptScalarConversionTransition::ScalarThrow(Box::new(
                            value_2.clone(),
                        ))
                    }
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                        error_class,
                        reason,
                    ) => self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(
                        error_class.clone(),
                        reason.clone(),
                    ),
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(reason_2) => {
                        self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2.clone())
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_scalar_primitive(
    operation: &self::ml_string::Text,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        {
            let ml_library_inequality_2 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
            } else {
                {
                    let invalid = self::read_java_script_semantic_value_error(value.clone());
                    {
                        let ml_library_inequality_1 =
                            invalid != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            self::JavaScriptScalarConversionTransition::ScalarInvalid(
                                invalid.clone(),
                            )
                        } else {
                            {
                                let result = self::execute_java_script_call_scalar_conversion(
                                    operation,
                                    value.clone(),
                                );
                                match result {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_2) => {
                                let value_2 = *value_2;
                                self::JavaScriptScalarConversionTransition::ScalarCompleted(Box::new(value_2.clone()))
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(reason) => {
                                self::JavaScriptScalarConversionTransition::ScalarInvalid(reason.clone())
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(error_class, reason_2) => {
                                self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class.clone(), reason_2.clone())
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(reason_3) => {
                                self::JavaScriptScalarConversionTransition::ScalarResourceLimit(Box::new(value.clone()), reason_3.clone())
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
pub fn begin_java_script_scalar_protocol(
    heap: self::JavaScriptOrdinaryHeap,
    input: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
    operation: &self::ml_string::Text,
) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if error == (self::ml_string::Text::from_units(vec![])) {
            self::finish_java_script_scalar_protocol(
                operation,
                self::begin_java_script_primitive_conversion(
                    heap.clone(),
                    input.clone(),
                    exotic_key.clone(),
                    &(self::read_java_script_scalar_protocol_hint(operation)),
                ),
            )
        } else {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_scalar_protocol(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
    operation: &self::ml_string::Text,
) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
            } else if (match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    source_ml_source2_data_property_value18255,
                    _,
                ) => source_ml_source2_data_property_value18255.clone(),
            }) != (self::read_java_script_scalar_protocol_hint(operation))
            {
                self::JavaScriptScalarConversionTransition::ScalarInvalid(
                    self::ml_string::Text::from_units(vec![
                        115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                        110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 104, 105,
                        110, 116, 32, 111, 102, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                        32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111,
                        110,
                    ]),
                )
            } else {
                self::finish_java_script_scalar_protocol(
                    operation,
                    self::step_java_script_primitive_conversion(heap.clone(), state.clone()),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_scalar_protocol(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
    operation: &self::ml_string::Text,
) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
            } else if (match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    source_ml_source2_data_property_value18257,
                    _,
                ) => source_ml_source2_data_property_value18257.clone(),
            }) != (self::read_java_script_scalar_protocol_hint(operation))
            {
                self::JavaScriptScalarConversionTransition::ScalarInvalid(
                    self::ml_string::Text::from_units(vec![
                        115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                        110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 104, 105,
                        110, 116, 32, 111, 102, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                        32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111,
                        110,
                    ]),
                )
            } else {
                self::finish_java_script_scalar_protocol(
                    operation,
                    self::resume_java_script_primitive_conversion(
                        heap.clone(),
                        state.clone(),
                        completion.clone(),
                    ),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_assignment_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayAssignmentState,
) -> self::ml_string::Text {
    {
        let error = self::read_java_script_array_definition_state_error(
            heap.clone(),
            match state.clone() {
                self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
                    source_ml_source2_data_property_value18258,
                    _,
                ) => {
                    let source_ml_source2_data_property_value18258 =
                        *source_ml_source2_data_property_value18258;
                    source_ml_source2_data_property_value18258.clone()
                }
                self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                    source_ml_source2_data_property_value18259,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value18259 =
                        *source_ml_source2_data_property_value18259;
                    source_ml_source2_data_property_value18259.clone()
                }
                self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                    source_ml_source2_data_property_value18260,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value18260 =
                        *source_ml_source2_data_property_value18260;
                    source_ml_source2_data_property_value18260.clone()
                }
            },
        );
        {
            let ml_library_inequality_2 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                error.clone()
            } else {
                {
                    let key_error = self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        match state.clone() {
                            self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
                                _,
                                source_ml_source2_data_property_value18261,
                            ) => {
                                let source_ml_source2_data_property_value18261 =
                                    *source_ml_source2_data_property_value18261;
                                source_ml_source2_data_property_value18261.clone()
                            }
                            self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                                _,
                                source_ml_source2_data_property_value18262,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value18262 =
                                    *source_ml_source2_data_property_value18262;
                                source_ml_source2_data_property_value18262.clone()
                            }
                            self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                                _,
                                source_ml_source2_data_property_value18263,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value18263 =
                                    *source_ml_source2_data_property_value18263;
                                source_ml_source2_data_property_value18263.clone()
                            }
                        },
                    );
                    {
                        let ml_library_inequality_1 =
                            key_error != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            key_error.clone()
                        } else if {
                            let source_ml_source2_s18267 = match state.clone() {
                                self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
                                    _,
                                    source_ml_source2_data_property_value18277,
                                ) => {
                                    let source_ml_source2_data_property_value18277 =
                                        *source_ml_source2_data_property_value18277;
                                    source_ml_source2_data_property_value18277.clone()
                                }
                                self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                                    _,
                                    source_ml_source2_data_property_value18278,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value18278 =
                                        *source_ml_source2_data_property_value18278;
                                    source_ml_source2_data_property_value18278.clone()
                                }
                                self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                                    _,
                                    source_ml_source2_data_property_value18279,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value18279 =
                                        *source_ml_source2_data_property_value18279;
                                    source_ml_source2_data_property_value18279.clone()
                                }
                            };
                            match source_ml_source2_s18267 {
                                self::JavaScriptSemanticValue::Symbol(_) => false,
                                _ => true,
                            }
                        } {
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101,
                                110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115,
                                32, 97, 99, 116, 117, 97, 108, 32, 119, 101, 108, 108, 45, 107,
                                110, 111, 119, 110, 32, 112, 114, 105, 109, 105, 116, 105, 118,
                                101, 45, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83,
                                121, 109, 98, 111, 108,
                            ])
                        } else {
                            match state.clone() {
                                self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
                                    _,
                                    _,
                                ) => self::ml_string::Text::from_units(vec![]),
                                self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                                    definition,
                                    exotic_key,
                                    conversion,
                                ) => {
                                    let definition = *definition;
                                    let exotic_key = *exotic_key;
                                    let conversion = *conversion;
                                    if ((((match definition.clone() {
                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, source_ml_source2_data_property_value18284, _, _) => {
                                        source_ml_source2_data_property_value18284.clone()
                                    }
                                }) == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))) || ((match conversion.clone() {
                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value18286, _) => {
                                        source_ml_source2_data_property_value18286.clone()
                                    }
                                }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) || !(self::test_java_script_semantic_same_value(match conversion.clone() {
                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value18287, _, _, _) => {
                                        let source_ml_source2_data_property_value18287 = *source_ml_source2_data_property_value18287;
                                        source_ml_source2_data_property_value18287.clone()
                                    }
                                }, ({ (match definition.clone() {
                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value18288, _, _, _) => {
                                        source_ml_source2_data_property_value18288.clone()
                                    }
                                }).borrow().clone() }).8))) || !(self::test_java_script_semantic_same_value(match conversion.clone() {
                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value18289, _, _) => {
                                        let source_ml_source2_data_property_value18289 = *source_ml_source2_data_property_value18289;
                                        source_ml_source2_data_property_value18289.clone()
                                    }
                                }, exotic_key.clone())) {
                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 112, 117, 116, 44, 32, 112, 104, 97, 115, 101, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 110, 117, 109, 98, 101, 114, 32, 104, 105, 110, 116])
                                } else {
                                    self::read_java_script_primitive_conversion_state_error(heap.clone(), conversion.clone())
                                }
                                }
                                self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                                    definition_2,
                                    _,
                                    value,
                                ) => {
                                    let definition_2 = *definition_2;
                                    let value = *value;
                                    if ((match definition_2.clone() {
                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, _, source_ml_source2_data_property_value18291, _, _) => {
                                        source_ml_source2_data_property_value18291.clone()
                                    }
                                }) == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))) || match value.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                } {
                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 115, 117, 115, 112, 101, 110, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101])
                                } else {
                                    self::read_java_script_ordinary_value_error(heap.clone(), value.clone())
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
pub fn continue_java_script_array_assignment_number(
    heap: self::JavaScriptOrdinaryHeap,
    definition: self::JavaScriptArrayDefinitionState,
    exotic_key: self::JavaScriptSemanticValue,
    transition: self::JavaScriptScalarConversionTransition,
) -> self::JavaScriptArrayAssignmentTransition {
    match transition.clone() {
        self::JavaScriptScalarConversionTransition::ScalarContinuation(state) => {
            let state = *state;
            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentContinue(
                Box::new(heap.clone()),
                Box::new(self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                    Box::new(definition.clone()),
                    Box::new(exotic_key.clone()),
                    Box::new(state.clone()),
                )),
            )
        }
        self::JavaScriptScalarConversionTransition::ScalarCall(
            state_2,
            callee,
            receiver,
            arguments,
        ) => {
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCall(
                Box::new(heap.clone()),
                Box::new(self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                    Box::new(definition.clone()),
                    Box::new(exotic_key.clone()),
                    Box::new(state_2.clone()),
                )),
                Box::new(callee.clone()),
                Box::new(receiver.clone()),
                arguments.clone(),
            )
        }
        self::JavaScriptScalarConversionTransition::ScalarCompleted(value) => {
            let value = *value;
            self::continue_java_script_array_assignment_definition(
                exotic_key.clone(),
                self::resume_java_script_array_property_number(
                    heap.clone(),
                    definition.clone(),
                    value.clone(),
                ),
            )
        }
        self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
            let value_2 = *value_2;
            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentThrow(
                Box::new(heap.clone()),
                Box::new(value_2.clone()),
            )
        }
        self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class, reason) => {
            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentError(
                Box::new(heap.clone()),
                error_class.clone(),
                reason.clone(),
            )
        }
        self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                Box::new(heap.clone()),
                reason_2.clone(),
            )
        }
        self::JavaScriptScalarConversionTransition::ScalarResourceLimit(value_3, reason_3) => {
            let value_3 = *value_3;
            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentResourceLimit(
                Box::new(heap.clone()),
                Box::new(
                    self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                        Box::new(definition.clone()),
                        Box::new(exotic_key.clone()),
                        Box::new(value_3.clone()),
                    ),
                ),
                reason_3.clone(),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_array_assignment_definition(
    exotic_key: self::JavaScriptSemanticValue,
    transition: self::JavaScriptArrayCompletion,
) -> self::JavaScriptArrayAssignmentTransition {
    match transition.clone() {
        self::JavaScriptArrayCompletion::ArrayNormal(heap, _) => {
            let heap = *heap;
            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                Box::new(heap.clone()),
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32,
                    99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116,
                    101, 32, 97, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99,
                    111, 109, 112, 108, 101, 116, 105, 111, 110,
                ]),
            )
        }
        self::JavaScriptArrayCompletion::ArrayPropertyComplete(heap_2, success, writes) => {
            let heap_2 = *heap_2;
            {
                let source_ml_source2_constructor_field18304 = Some(writes.clone());
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCompleted(
                    Box::new(heap_2.clone()),
                    success,
                    Some(source_ml_source2_constructor_field18304),
                )
            }
        }
        self::JavaScriptArrayCompletion::ArrayNumberRequest(heap_3, state, input) => {
            let heap_3 = *heap_3;
            let state = *state;
            let input = *input;
            self::continue_java_script_array_assignment_number(
                heap_3.clone(),
                state.clone(),
                exotic_key.clone(),
                self::begin_java_script_scalar_protocol(
                    heap_3.clone(),
                    input.clone(),
                    exotic_key.clone(),
                    &(self::ml_string::Text::from_units(vec![
                        99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118,
                        101, 32, 110, 117, 109, 98, 101, 114,
                    ])),
                ),
            )
        }
        self::JavaScriptArrayCompletion::ArrayDefinitionContinue(heap_4, state_2) => {
            let heap_4 = *heap_4;
            let state_2 = *state_2;
            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentContinue(
                Box::new(heap_4.clone()),
                Box::new(
                    self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
                        Box::new(state_2.clone()),
                        Box::new(exotic_key.clone()),
                    ),
                ),
            )
        }
        self::JavaScriptArrayCompletion::ArrayInvalid(heap_5, reason) => {
            let heap_5 = *heap_5;
            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                Box::new(heap_5.clone()),
                reason.clone(),
            )
        }
        self::JavaScriptArrayCompletion::ArrayError(heap_6, error_class, reason_2) => {
            let heap_6 = *heap_6;
            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentError(
                Box::new(heap_6.clone()),
                error_class.clone(),
                reason_2.clone(),
            )
        }
        self::JavaScriptArrayCompletion::ArrayResourceLimit(heap_7, reason_3, state_3) => {
            let heap_7 = *heap_7;
            {
                let definition = ((state_3
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptArrayDefinitionState>,)))
                .0)
                    .clone();
                if match (definition.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                        Box::new(heap_7.clone()),
                        self::ml_string::Text::from_units(vec![
                            97, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114,
                            101, 115, 111, 117, 114, 99, 101, 32, 111, 117, 116, 99, 111, 109, 101,
                            32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116,
                            117, 116, 101, 32, 97, 110, 32, 65, 114, 114, 97, 121, 32, 100, 101,
                            102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105,
                            110, 117, 97, 116, 105, 111, 110,
                        ]),
                    )
                } else {
                    self::JavaScriptArrayAssignmentTransition::ArrayAssignmentResourceLimit(
                        Box::new(heap_7.clone()),
                        Box::new(
                            self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
                                Box::new(definition.clone().expect("a proven present value")),
                                Box::new(exotic_key.clone()),
                            ),
                        ),
                        reason_3.clone(),
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn begin_java_script_array_assignment(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
            bool,
            bool,
            bool,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            bool,
            bool,
        )>,
    >,
    exotic_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayAssignmentTransition {
    {
        let transition = self::begin_java_script_array_property_definition(
            heap.clone(),
            identity.clone(),
            key.clone(),
            descriptor.clone(),
        );
        self::continue_java_script_array_assignment_definition(exotic_key.clone(), transition)
    }
}
