// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptObjectFromEntriesState;
use super::JavaScriptObjectFromEntriesTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPropertyDefinitionCall;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012;
use super::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorLoop1013Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_definition_getter_arguments;
use super::read_java_script_descriptor_field;
use super::read_java_script_object_from_entries_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_spelling;
use super::read_java_script_property_definition_continuation_error;
use super::source_ml_source1_temporary_read_java_script_property_definition_state_error_loop1013;
use super::test_java_script_semantic_same_value;
use super::throw_java_script_object_from_entries;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_object_from_entries_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectFromEntriesState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptObjectFromEntriesTransition {
    {
        let error = {
            let source_ml_source2_logical21281 =
                self::read_java_script_object_from_entries_state_error(heap.clone(), state.clone());
            if !source_ml_source2_logical21281.0.is_empty() {
                source_ml_source2_logical21281.clone()
            } else {
                self::read_java_script_ordinary_value_error(heap.clone(), value.clone())
            }
        };
        if (((error != (self::ml_string::Text::from_units(vec![])))
            || ((match state.clone() {
                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value21283,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value21283.clone(),
            }) != (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]))))
            || match value.clone() {
                self::JavaScriptSemanticValue::Object(_, _) => false,
                _ => true,
            })
        {
            self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(
                Box::new(heap.clone()),
                if !error.0.is_empty() {
                    error.clone()
                } else {
                    self::ml_string::Text::from_units(vec![
                        79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105,
                        101, 115, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 108, 121,
                        32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108,
                        109, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                        111, 110,
                    ])
                },
            )
        } else {
            self::throw_java_script_object_from_entries(heap.clone(), state.clone(), value.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_definition_heap(
    transition: self::JavaScriptPropertyDefinitionTransition,
    heap: self::JavaScriptOrdinaryHeap,
) -> self::JavaScriptOrdinaryHeap {
    match transition.clone() {
        self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(
            _,
            transition_heap,
        ) => (((transition_heap
            .clone()
            .unwrap_or_else(|| (None::<self::JavaScriptOrdinaryHeap>,)))
        .0)
            .clone())
        .clone()
        .unwrap_or_else(|| heap.clone()),
        self::JavaScriptPropertyDefinitionTransition::DefinitionCall(
            _,
            _,
            _,
            _,
            transition_heap_2,
        ) => (((transition_heap_2
            .clone()
            .unwrap_or_else(|| (None::<self::JavaScriptOrdinaryHeap>,)))
        .0)
            .clone())
        .clone()
        .unwrap_or_else(|| heap.clone()),
        self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(heap_2, _, _) => {
            let heap_2 = *heap_2;
            heap_2.clone()
        }
        self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(_, transition_heap_3) => {
            (((transition_heap_3
                .clone()
                .unwrap_or_else(|| (None::<self::JavaScriptOrdinaryHeap>,)))
            .0)
                .clone())
            .clone()
            .unwrap_or_else(|| heap.clone())
        }
        self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(
            _,
            _,
            transition_heap_4,
            _,
        ) => (((transition_heap_4
            .clone()
            .unwrap_or_else(|| (None::<self::JavaScriptOrdinaryHeap>,)))
        .0)
            .clone())
        .clone()
        .unwrap_or_else(|| heap.clone()),
        self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(_, transition_heap_5) => {
            (((transition_heap_5
                .clone()
                .unwrap_or_else(|| (None::<self::JavaScriptOrdinaryHeap>,)))
            .0)
                .clone())
            .clone()
            .unwrap_or_else(|| heap.clone())
        }
        self::JavaScriptPropertyDefinitionTransition::DefinitionResourceLimit(heap_3, _, _) => {
            let heap_3 = *heap_3;
            heap_3.clone()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn project_java_script_property_key_completion(
    completion: self::JavaScriptSemanticCompletion,
) -> self::JavaScriptPrimitiveConversionCompletion {
    match completion.clone() {
        self::JavaScriptSemanticCompletion::NormalCompletion(value) => {
            let value = *value;
            self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone()))
        }
        self::JavaScriptSemanticCompletion::ThrowCompletion(value_2) => {
            let value_2 = *value_2;
            self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(Box::new(
                value_2.clone(),
            ))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_definition_array_conversion(
    assignment: self::JavaScriptArrayAssignmentState,
) -> Option<self::JavaScriptPrimitiveConversionState> {
    match assignment.clone() {
        self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(_, _, conversion) => {
            let conversion = *conversion;
            Some(conversion.clone())
        }
        _ => None::<self::JavaScriptPrimitiveConversionState>,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_definition_primitive_conversion(
    state: self::JavaScriptPropertyDefinitionState,
) -> Option<self::JavaScriptPrimitiveConversionState> {
    match state.clone() {
        self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, conversion, _, _) => {
            let conversion = *conversion;
            Some(conversion.clone())
        }
        self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
            _,
            _,
            assignment,
            _,
            _,
        ) => {
            let assignment = *assignment;
            self::read_java_script_definition_array_conversion(assignment.clone())
        }
        _ => None::<self::JavaScriptPrimitiveConversionState>,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_definition_primitive_call_error(
    heap: self::JavaScriptOrdinaryHeap,
    call: self::JavaScriptPropertyDefinitionCall,
    conversion: Option<self::JavaScriptPrimitiveConversionState>,
) -> self::ml_string::Text {
    match call.clone() {
        self::JavaScriptPropertyDefinitionCall::PropertyDefinitionCallRecord(
            _,
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            if ((match (conversion.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } || !(self::test_java_script_semantic_same_value(
                shared_constructor_field1.clone(),
                match conversion.clone().expect("a proven present value") {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                        source_ml_source2_data_property_value21312,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value21312 =
                            *source_ml_source2_data_property_value21312;
                        source_ml_source2_data_property_value21312.clone()
                    }
                },
            ))) || !(self::ml_array::includes(
                &{
                    let ml_reference_value1071 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                        self::ml_string::Text::from_units(vec![
                            101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            101, 120, 111, 116, 105, 99, 67, 97, 108, 108,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            102, 105, 114, 115, 116, 67, 97, 108, 108,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            115, 101, 99, 111, 110, 100, 67, 97, 108, 108,
                        ]),
                    ])))
                    .borrow()
                    .clone();
                    ml_reference_value1071
                },
                &match conversion.clone().expect("a proven present value") {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value21313,
                    ) => source_ml_source2_data_property_value21313.clone(),
                },
                0f64,
                false,
                |a, b| a == b,
            ))) {
                self::ml_string::Text::from_units(vec![
                    100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 104, 105, 108, 100,
                    32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101,
                    105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103,
                    32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101,
                    114, 115, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 99, 101, 105, 118,
                    101, 114,
                ])
            } else {
                if ((match conversion.clone().expect("a proven present value") {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value21315,
                    ) => source_ml_source2_data_property_value21315.clone(),
                }) == (self::ml_string::Text::from_units(vec![
                    101, 120, 111, 116, 105, 99, 67, 97, 108, 108,
                ]))) {
                    if ((({ let ml_reference_array1072 = shared_constructor_field2.clone(); let ml_reference_projection1072 = ml_reference_array1072.borrow().len() as f64; ml_reference_projection1072 }) != (1f64)) || !(self::test_java_script_semantic_same_value(({
                        let source_ml_source2_array_read_value21319 = shared_constructor_field2.clone();
                        {
                            let source_ml_source2_array_read_index21320 = 0f64;
                            { let ml_reference_array1073 = source_ml_source2_array_read_value21319.clone(); let ml_reference_position1073 = source_ml_source2_array_read_index21320; let ml_reference_projection1073 = self::ml_array::optional_at(&*ml_reference_array1073.borrow(), self::ml_array::number_index(ml_reference_position1073)); ml_reference_projection1073 }
                        }
                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), self::read_java_script_primitive_conversion_spelling(&(match conversion.clone().expect("a proven present value") {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value21321, _) => {
                            source_ml_source2_data_property_value21321.clone()
                        }
                    }))))) {
                        self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 101, 120, 111, 116, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112, 101, 32, 104, 105, 110, 116])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                } else {
                    if (((match conversion.clone().expect("a proven present value") {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value21323,
                        ) => source_ml_source2_data_property_value21323.clone(),
                    }) == (self::ml_string::Text::from_units(vec![
                        102, 105, 114, 115, 116, 67, 97, 108, 108,
                    ]))) || ((match conversion.clone().expect("a proven present value") {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value21325,
                        ) => source_ml_source2_data_property_value21325.clone(),
                    }) == (self::ml_string::Text::from_units(vec![
                        115, 101, 99, 111, 110, 100, 67, 97, 108, 108,
                    ])))) {
                        if (({
                            let ml_reference_array1074 = shared_constructor_field2.clone();
                            let ml_reference_projection1074 =
                                ml_reference_array1074.borrow().len() as f64;
                            ml_reference_projection1074
                        }) == (0f64))
                        {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            self::ml_string::Text::from_units(vec![
                                100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 111, 114,
                                100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115,
                                105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101,
                                116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 101, 109, 112,
                                116, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105,
                                115, 116,
                            ])
                        }
                    } else {
                        {
                            let first = ((match conversion.clone().expect("a proven present value")
                            {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value21329,
                                ) => source_ml_source2_data_property_value21329.clone(),
                            }) == (self::ml_string::Text::from_units(vec![
                                102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114,
                            ])));
                            {
                                let name = if ((match conversion.clone().expect("a proven present value") {
                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value21331, _) => {
                                        source_ml_source2_data_property_value21331.clone()
                                    }
                                }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                                    if first {
                                        self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103])
                                    } else {
                                        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102])
                                    }
                                } else {
                                    if first {
                                        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102])
                                    } else {
                                        self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103])
                                    }
                                };
                                {
                                    let key = if ((match conversion.clone().expect("a proven present value") {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source2_data_property_value21333) => {
                                            source_ml_source2_data_property_value21333.clone()
                                        }
                                    }) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]))) {
                                        match conversion.clone().expect("a proven present value") {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value21334, _, _) => {
                                                let source_ml_source2_data_property_value21334 = *source_ml_source2_data_property_value21334;
                                                source_ml_source2_data_property_value21334.clone()
                                            }
                                        }
                                    } else {
                                        self::read_java_script_primitive_conversion_spelling(&(name))
                                    };
                                    self::read_java_script_definition_getter_arguments(
                                        heap.clone(),
                                        call.clone(),
                                        key.clone(),
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
pub fn read_java_script_property_definition_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPropertyDefinitionState,
) -> self::ml_string::Text {
    if !(self::ml_array::includes(
        &{
            let ml_reference_value1075 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![
                    112, 114, 111, 112, 101, 114, 116, 121, 68, 101, 102, 105, 110, 105, 116, 105,
                    111, 110, 75, 101, 121,
                ]),
                self::ml_string::Text::from_units(vec![
                    112, 114, 111, 112, 101, 114, 116, 121, 68, 101, 102, 105, 110, 105, 116, 105,
                    111, 110, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114,
                ]),
                self::ml_string::Text::from_units(vec![
                    112, 114, 111, 112, 101, 114, 116, 121, 68, 101, 102, 105, 110, 105, 116, 105,
                    111, 110, 65, 114, 114, 97, 121,
                ]),
                self::ml_string::Text::from_units(vec![
                    112, 114, 111, 112, 101, 114, 116, 121, 68, 101, 102, 105, 110, 105, 116, 105,
                    111, 110, 82, 101, 97, 100, 121,
                ]),
            ])))
            .borrow()
            .clone();
            ml_reference_value1075
        },
        &match state.clone() {
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, _, _, _) => {
                self::ml_string::Text::from_units(vec![
                    112, 114, 111, 112, 101, 114, 116, 121, 68, 101, 102, 105, 110, 105, 116, 105,
                    111, 110, 75, 101, 121,
                ])
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                _,
                _,
                _,
                _,
                _,
                _,
            ) => self::ml_string::Text::from_units(vec![
                112, 114, 111, 112, 101, 114, 116, 121, 68, 101, 102, 105, 110, 105, 116, 105, 111,
                110, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114,
            ]),
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, _, _) => {
                self::ml_string::Text::from_units(vec![
                    112, 114, 111, 112, 101, 114, 116, 121, 68, 101, 102, 105, 110, 105, 116, 105,
                    111, 110, 65, 114, 114, 97, 121,
                ])
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, _, _) => {
                self::ml_string::Text::from_units(vec![
                    112, 114, 111, 112, 101, 114, 116, 121, 68, 101, 102, 105, 110, 105, 116, 105,
                    111, 110, 82, 101, 97, 100, 121,
                ])
            }
        },
        0f64,
        false,
        |a, b| a == b,
    )) {
        self::ml_string::Text::from_units(vec![
            112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105,
            111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
            117, 97, 108, 32, 116, 121, 112, 101, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97,
            116, 105, 111, 110,
        ])
    } else {
        {
            let target = match state.clone() {
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                    source_ml_source2_data_property_value21335,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value21335 =
                        *source_ml_source2_data_property_value21335;
                    source_ml_source2_data_property_value21335.clone()
                }
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                    source_ml_source2_data_property_value21336,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value21336 =
                        *source_ml_source2_data_property_value21336;
                    source_ml_source2_data_property_value21336.clone()
                }
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                    source_ml_source2_data_property_value21337,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value21337 =
                        *source_ml_source2_data_property_value21337;
                    source_ml_source2_data_property_value21337.clone()
                }
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                    source_ml_source2_data_property_value21338,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value21338 =
                        *source_ml_source2_data_property_value21338;
                    source_ml_source2_data_property_value21338.clone()
                }
            };
            {
                let error = {
                    let source_ml_source2_logical21339 =
                        self::read_java_script_ordinary_heap_error(heap.clone());
                    if !source_ml_source2_logical21339.0.is_empty() {
                        source_ml_source2_logical21339.clone()
                    } else {
                        self::read_java_script_ordinary_value_error(heap.clone(), target.clone())
                    }
                };
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    match target.clone() {
                        self::JavaScriptSemanticValue::Object(identity, _) => {
                            if ({
                                let source_ml_source2_comparison_operand21345 = match state.clone() {
                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, _, source_ml_source2_data_property_value21341, _) => {
                                        ((source_ml_source2_data_property_value21341.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                    }
                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(_, _, _, _, source_ml_source2_data_property_value21342, _) => {
                                        ((source_ml_source2_data_property_value21342.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                    }
                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, source_ml_source2_data_property_value21343, _) => {
                                        ((source_ml_source2_data_property_value21343.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                    }
                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, source_ml_source2_data_property_value21344, _) => {
                                        ((source_ml_source2_data_property_value21344.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                    }
                                };
                                {
                                    let source_ml_source2_comparison_operand21346 = None::<()>;
                                    !(match (source_ml_source2_comparison_operand21345.clone(), source_ml_source2_comparison_operand21346.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                }
                            } && !(self::ml_array::includes(&{ let ml_reference_value1076 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121]), self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121])]))).borrow().clone(); ml_reference_value1076 }, &(match state.clone() {
                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, _, source_ml_source2_data_property_value21347, _) => {
                                    ((source_ml_source2_data_property_value21347.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                }
                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(_, _, _, _, source_ml_source2_data_property_value21348, _) => {
                                    ((source_ml_source2_data_property_value21348.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                }
                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, source_ml_source2_data_property_value21349, _) => {
                                    ((source_ml_source2_data_property_value21349.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                }
                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, source_ml_source2_data_property_value21350, _) => {
                                    ((source_ml_source2_data_property_value21350.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                }
                            }).clone().expect("a proven present value"), 0f64, false, |a, b| a == b))) {
                                self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 114, 32, 82, 101, 102, 108, 101, 99, 116, 32, 114, 101, 115, 117, 108, 116, 32, 112, 111, 108, 105, 99, 121])
                            } else {
                                {
                                    let source_ml_source2_s21390 = if !(match (match state.clone() {
                                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, _, _, source_ml_source2_data_property_value21391) => {
                                            ((source_ml_source2_data_property_value21391.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                        }
                                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(_, _, _, _, _, source_ml_source2_data_property_value21392) => {
                                            ((source_ml_source2_data_property_value21392.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                        }
                                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, _, source_ml_source2_data_property_value21393) => {
                                            ((source_ml_source2_data_property_value21393.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                        }
                                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, _, source_ml_source2_data_property_value21394) => {
                                            ((source_ml_source2_data_property_value21394.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                        }
                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        {
                                            let call = (match state.clone() {
                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, _, _, source_ml_source2_data_property_value21395) => {
                                                    ((source_ml_source2_data_property_value21395.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                                }
                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(_, _, _, _, _, source_ml_source2_data_property_value21396) => {
                                                    ((source_ml_source2_data_property_value21396.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                                }
                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, _, source_ml_source2_data_property_value21397) => {
                                                    ((source_ml_source2_data_property_value21397.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                                }
                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, _, source_ml_source2_data_property_value21398) => {
                                                    ((source_ml_source2_data_property_value21398.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value");
                                            match call.clone() {
                                                self::JavaScriptPropertyDefinitionCall::PropertyDefinitionCallRecord(shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                                    let shared_constructor_field4 = *shared_constructor_field4;
                                                    let shared_constructor_field5 = *shared_constructor_field5;
                                                    if ((((match call.clone() {
                                                        self::JavaScriptPropertyDefinitionCall::PropertyDefinitionCallRecord(_, _, _) => {
                                                            false
                                                        }
                                                    } || {
                                                        let source_ml_source2_s21403 = shared_constructor_field4.clone();
                                                        match source_ml_source2_s21403.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) || !(((match shared_constructor_field4.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value21408) => {
                                                            Some((source_ml_source2_data_property_value21408,))
                                                        }
                                                        _ => {
                                                            None::<(bool,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0)) || (({ let ml_reference_array1077 = shared_constructor_field6.clone(); let ml_reference_projection1077 = ml_reference_array1077.borrow().len() as f64; ml_reference_projection1077 }) > (2f64))) || match state.clone() {
                                                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, _, _) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    }) {
                                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012Return(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 108, 121, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
                                                    } else {
                                                        {
                                                            let source_ml_source1_temporary_values1011 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![shared_constructor_field4.clone(), shared_constructor_field5.clone()], { let ml_reference_value1078 = shared_constructor_field6.borrow().clone(); ml_reference_value1078 })));
                                                            {
                                                                let source_ml_source1_temporary_index1011 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let source_ml_source2_s21420 = self::source_ml_source1_temporary_read_java_script_property_definition_state_error_loop1013(heap.clone(), source_ml_source1_temporary_values1011.clone(), source_ml_source1_temporary_index1011.clone());
                                                                    match source_ml_source2_s21420.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorLoop1013Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorLoop1013Done => {
                                                                            match state.clone() {
                                                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(_, _, conversion, _, _, _) => {
                                                                                    let conversion = *conversion;
                                                                                    match conversion.clone() {
                                                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(shared_constructor_field1, shared_constructor_field2, _, shared_constructor_field3) => {
                                                                                            let shared_constructor_field1 = *shared_constructor_field1;
                                                                                            if (!shared_constructor_field3 || !(self::test_java_script_semantic_same_value(shared_constructor_field5.clone(), shared_constructor_field1.clone()))) {
                                                                                                self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012Return(self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                                                            } else {
                                                                                                {
                                                                                                    let invalid = self::read_java_script_definition_getter_arguments(heap.clone(), call.clone(), self::JavaScriptSemanticValue::Text((({ let ml_reference_value1079 = (self::read_java_script_descriptor_field(shared_constructor_field2)).borrow().clone(); ml_reference_value1079 }).0).2));
                                                                                                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012Return(invalid.clone())
                                                                                                    } else {
                                                                                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012Next
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                _ => {
                                                                                    {
                                                                                        let invalid_2 = self::read_java_script_definition_primitive_call_error(heap.clone(), call.clone(), self::read_java_script_definition_primitive_conversion(state.clone()));
                                                                                        if (invalid_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012Return(invalid_2.clone())
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012Next
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorLoop1013Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorLoop1013Return(source_ml_source1_temporary_result1013) => {
                                                                            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012Return(source_ml_source1_temporary_result1013.clone())
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
                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012Next
                                    };
                                    match source_ml_source2_s21390.clone() {
                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012Next => {
                                            self::read_java_script_property_definition_continuation_error(heap.clone(), state.clone(), identity.clone())
                                        }
                                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012Return(source_ml_source1_temporary_result1012) => {
                                            source_ml_source1_temporary_result1012.clone()
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116])
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_definition_keys_error(
    heap: self::JavaScriptOrdinaryHeap,
    key: self::JavaScriptSemanticValue,
    exotic_key: Option<self::JavaScriptSemanticValue>,
) -> self::ml_string::Text {
    {
        let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
        if ((key_error != (self::ml_string::Text::from_units(vec![])))
            || match (exotic_key.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            })
        {
            key_error.clone()
        } else {
            {
                let invalid = self::read_java_script_ordinary_value_error(
                    heap.clone(),
                    exotic_key.clone().expect("a proven present value"),
                );
                if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                    invalid.clone()
                } else {
                    if match exotic_key.clone().expect("a proven present value") {
                        self::JavaScriptSemanticValue::Symbol(_) => true,
                        _ => false,
                    } {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![
                            112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110,
                            105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105,
                            116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                            110, 32, 83, 121, 109, 98, 111, 108,
                        ])
                    }
                }
            }
        }
    }
}
