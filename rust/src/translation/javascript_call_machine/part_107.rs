// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPropertyDefinitionCall;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptPropertyDefinitionContinuationErrorLoop1015Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_assignment_state_error;
use super::read_java_script_descriptor_conversion_state_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::read_java_script_property_definition_keys_error;
use super::read_java_script_property_descriptor_error;
use super::source_ml_source1_temporary_read_java_script_property_definition_continuation_error_loop1015;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_definition_continuation_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPropertyDefinitionState,
    identity: self::ml::Big,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
            _,
            attributes,
            conversion,
            _,
            _,
        ) => {
            let attributes = *attributes;
            let conversion = *conversion;
            {
                let invalid = {
                    let source_ml_source2_logical21443 =
                        self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            attributes.clone(),
                        );
                    if !source_ml_source2_logical21443.0.is_empty() {
                        source_ml_source2_logical21443.clone()
                    } else {
                        self::read_java_script_primitive_conversion_state_error(
                            heap.clone(),
                            conversion.clone(),
                        )
                    }
                };
                if !invalid.0.is_empty() {
                    invalid.clone()
                } else {
                    if ((match conversion.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                            _,
                            _,
                            source_ml_source2_data_property_value21445,
                            _,
                        ) => source_ml_source2_data_property_value21445.clone(),
                    }) == (self::ml_string::Text::from_units(vec![
                        115, 116, 114, 105, 110, 103,
                    ]))) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::ml_string::Text::from_units(vec![
                            112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 99, 111,
                            110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105,
                            110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                            116, 114, 105, 110, 103, 32, 104, 105, 110, 116,
                        ])
                    }
                }
            }
        }
        self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
            _,
            key,
            conversion_2,
            exotic_key,
            _,
            _,
        ) => {
            let key = *key;
            let conversion_2 = *conversion_2;
            {
                let key_error = self::read_java_script_property_definition_keys_error(
                    heap.clone(),
                    key.clone(),
                    ((exotic_key
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                    .0)
                        .clone(),
                );
                if !key_error.0.is_empty() {
                    key_error.clone()
                } else {
                    self::read_java_script_descriptor_conversion_state_error(
                        heap.clone(),
                        conversion_2.clone(),
                    )
                }
            }
        }
        self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
            _,
            key_2,
            assignment,
            _,
            _,
        ) => {
            let key_2 = *key_2;
            let assignment = *assignment;
            {
                let key_error_2 =
                    self::read_java_script_ordinary_key_error(heap.clone(), key_2.clone());
                if (key_error_2 != (self::ml_string::Text::from_units(vec![]))) {
                    key_error_2.clone()
                } else {
                    if (((match match assignment.clone() {
                        self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
                            source_ml_source2_data_property_value21452,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value21452 =
                                *source_ml_source2_data_property_value21452;
                            source_ml_source2_data_property_value21452.clone()
                        }
                        self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                            source_ml_source2_data_property_value21453,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value21453 =
                                *source_ml_source2_data_property_value21453;
                            source_ml_source2_data_property_value21453.clone()
                        }
                        self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                            source_ml_source2_data_property_value21454,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value21454 =
                                *source_ml_source2_data_property_value21454;
                            source_ml_source2_data_property_value21454.clone()
                        }
                    } {
                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                            source_ml_source2_data_property_value21455,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value21455.clone(),
                    }) != identity)
                        || !(self::test_java_script_semantic_same_value(
                            match match assignment.clone() {
                                self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
                                    source_ml_source2_data_property_value21456,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value21456 =
                                        *source_ml_source2_data_property_value21456;
                                    source_ml_source2_data_property_value21456.clone()
                                }
                                self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                                    source_ml_source2_data_property_value21457,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value21457 =
                                        *source_ml_source2_data_property_value21457;
                                    source_ml_source2_data_property_value21457.clone()
                                }
                                self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                                    source_ml_source2_data_property_value21458,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value21458 =
                                        *source_ml_source2_data_property_value21458;
                                    source_ml_source2_data_property_value21458.clone()
                                }
                            } {
                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                                    _,
                                    source_ml_source2_data_property_value21459,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value21459 =
                                        *source_ml_source2_data_property_value21459;
                                    source_ml_source2_data_property_value21459.clone()
                                }
                            },
                            key_2.clone(),
                        )))
                    {
                        self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111,
                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                            99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 32, 107, 101,
                            121, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100,
                            32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114,
                        ])
                    } else {
                        self::read_java_script_array_assignment_state_error(
                            heap.clone(),
                            assignment.clone(),
                        )
                    }
                }
            }
        }
        self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
            _,
            key_3,
            descriptor,
            exotic_key_2,
            _,
            _,
        ) => {
            let key_3 = *key_3;
            {
                let key_error_3 = self::read_java_script_property_definition_keys_error(
                    heap.clone(),
                    key_3.clone(),
                    ((exotic_key_2
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                    .0)
                        .clone(),
                );
                if (key_error_3 != (self::ml_string::Text::from_units(vec![]))) {
                    key_error_3.clone()
                } else {
                    {
                        let error =
                            self::read_java_script_property_descriptor_error(descriptor.clone());
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            error.clone()
                        } else {
                            {
                                let source_ml_source1_temporary_values1014 =
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        (({
                                            let ml_reference_value1080 =
                                                descriptor.borrow().clone();
                                            ml_reference_value1080
                                        })
                                        .0)
                                            .8,
                                        (({
                                            let ml_reference_value1081 =
                                                descriptor.borrow().clone();
                                            ml_reference_value1081
                                        })
                                        .0)
                                            .4,
                                        (({
                                            let ml_reference_value1082 =
                                                descriptor.borrow().clone();
                                            ml_reference_value1082
                                        })
                                        .0)
                                            .6,
                                    ]));
                                {
                                    let source_ml_source1_temporary_index1014 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s21460 = self::source_ml_source1_temporary_read_java_script_property_definition_continuation_error_loop1015(heap.clone(), source_ml_source1_temporary_values1014.clone(), source_ml_source1_temporary_index1014.clone());
                                        match source_ml_source2_s21460.clone() {
                                            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionContinuationErrorLoop1015Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionContinuationErrorLoop1015Done => {
                                                self::ml_string::Text::from_units(vec![])
                                            }
                                            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionContinuationErrorLoop1015Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionContinuationErrorLoop1015Return(source_ml_source1_temporary_result1015) => {
                                                source_ml_source1_temporary_result1015.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_definition_getter_arguments(
    heap: self::JavaScriptOrdinaryHeap,
    call: self::JavaScriptPropertyDefinitionCall,
    key: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match call.clone() {
        self::JavaScriptPropertyDefinitionCall::PropertyDefinitionCallRecord(
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let owner = if {
                        let source_ml_source2_s21462 = shared_constructor_field3.clone();
                        match source_ml_source2_s21462.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => true,
                            _ => false,
                        }
                    } {
                        {
                            let source_ml_source2_array_read_value21470 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_array_read_index21471 =
                                    self::find_java_script_ordinary_object(
                                        shared_constructor_field1.clone(),
                                        (((match shared_constructor_field3.clone() {
                                            self::JavaScriptSemanticValue::Object(
                                                source_ml_source2_data_property_value21469,
                                                _,
                                            ) => {
                                                Some((source_ml_source2_data_property_value21469
                                                    .clone(),))
                                            }
                                            _ => None::<(self::ml::Big,)>,
                                        })
                                        .clone()
                                        .expect("a proven present value"))
                                        .0)
                                            .clone(),
                                    );
                                {
                                    let ml_reference_array1083 =
                                        source_ml_source2_array_read_value21470.clone();
                                    let ml_reference_position1083 =
                                        source_ml_source2_array_read_index21471;
                                    let ml_reference_projection1083 = self::ml_array::optional_at(
                                        &*ml_reference_array1083.borrow(),
                                        self::ml_array::number_index(ml_reference_position1083),
                                    );
                                    ml_reference_projection1083
                                }
                            }
                        }
                    } else {
                        None::<self::JavaScriptOrdinaryObject>
                    };
                    if (!(match (owner.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && (((match owner.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                            true
                        }
                        _ => false,
                    } || match owner.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => true,
                        _ => false,
                    }) || match owner.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                            true
                        }
                        _ => false,
                    }) || match owner.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => true,
                        _ => false,
                    })) {
                        if ((((({
                            let ml_reference_array1084 = shared_constructor_field4.clone();
                            let ml_reference_projection1084 =
                                ml_reference_array1084.borrow().len() as f64;
                            ml_reference_projection1084
                        }) == (2f64))
                            && self::test_java_script_semantic_same_value(
                                shared_constructor_field2.clone(),
                                (((match owner.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value21503,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value21503 =
                                            *source_ml_source2_data_property_value21503;
                                        Some((source_ml_source2_data_property_value21503.clone(),))
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value21504,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value21504 =
                                            *source_ml_source2_data_property_value21504;
                                        Some((source_ml_source2_data_property_value21504.clone(),))
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value21505,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value21505 =
                                            *source_ml_source2_data_property_value21505;
                                        Some((source_ml_source2_data_property_value21505.clone(),))
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value21506,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value21506 =
                                            *source_ml_source2_data_property_value21506;
                                        Some((source_ml_source2_data_property_value21506.clone(),))
                                    }
                                    _ => None::<(self::JavaScriptSemanticValue,)>,
                                })
                                .clone()
                                .expect("a proven present value"))
                                .0)
                                    .clone(),
                            ))
                            && self::test_java_script_semantic_same_value(
                                ({
                                    let source_ml_source2_array_read_value21508 =
                                        shared_constructor_field4.clone();
                                    {
                                        let source_ml_source2_array_read_index21509 = 0f64;
                                        {
                                            let ml_reference_array1085 =
                                                source_ml_source2_array_read_value21508.clone();
                                            let ml_reference_position1085 =
                                                source_ml_source2_array_read_index21509;
                                            let ml_reference_projection1085 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array1085.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position1085,
                                                    ),
                                                );
                                            ml_reference_projection1085
                                        }
                                    }
                                })
                                .clone()
                                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                                key.clone(),
                            ))
                            && self::test_java_script_semantic_same_value(
                                ({
                                    let source_ml_source2_array_read_value21511 =
                                        shared_constructor_field4.clone();
                                    {
                                        let source_ml_source2_array_read_index21512 = 1f64;
                                        {
                                            let ml_reference_array1086 =
                                                source_ml_source2_array_read_value21511.clone();
                                            let ml_reference_position1086 =
                                                source_ml_source2_array_read_index21512;
                                            let ml_reference_projection1086 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array1086.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position1086,
                                                    ),
                                                );
                                            ml_reference_projection1086
                                        }
                                    }
                                })
                                .clone()
                                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                                shared_constructor_field3.clone(),
                            ))
                        {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            self::ml_string::Text::from_units(vec![
                                100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 111, 112, 97,
                                113, 117, 101, 32, 71, 101, 116, 32, 114, 101, 116, 97, 105, 110,
                                115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114,
                                101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 107, 101, 121, 32,
                                97, 110, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 111,
                                112, 101, 114, 97, 110, 100, 115,
                            ])
                        }
                    } else {
                        if (({
                            let ml_reference_array1087 = shared_constructor_field4.clone();
                            let ml_reference_projection1087 =
                                ml_reference_array1087.borrow().len() as f64;
                            ml_reference_projection1087
                        }) == (0f64))
                        {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            self::ml_string::Text::from_units(vec![
                                100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 111, 114,
                                100, 105, 110, 97, 114, 121, 32, 103, 101, 116, 116, 101, 114, 115,
                                32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32,
                                101, 109, 112, 116, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116,
                                32, 76, 105, 115, 116,
                            ])
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn retain_java_script_property_definition_context(
    state: self::JavaScriptPropertyDefinitionState,
    operation: Option<self::ml_string::Text>,
    call: Option<self::JavaScriptPropertyDefinitionCall>,
) -> self::JavaScriptPropertyDefinitionState {
    {
        let selected = if {
            let source_ml_source2_comparison_operand21517 = None::<()>;
            match (
                operation.clone(),
                source_ml_source2_comparison_operand21517.clone(),
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }
        } {
            state.clone()
        } else {
            match state.clone() {
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                    source_ml_source2_nominal_field21518,
                    source_ml_source2_nominal_field21519,
                    source_ml_source2_nominal_field21520,
                    _,
                    source_ml_source2_nominal_field21522,
                ) => {
                    let source_ml_source2_nominal_field21518 =
                        *source_ml_source2_nominal_field21518;
                    let source_ml_source2_nominal_field21519 =
                        *source_ml_source2_nominal_field21519;
                    let source_ml_source2_nominal_field21520 =
                        *source_ml_source2_nominal_field21520;
                    {
                        let source_ml_source2_nominal_override21523 =
                            Some(operation.clone().expect("a proven present value"));
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                            Box::new(source_ml_source2_nominal_field21518.clone()),
                            Box::new(source_ml_source2_nominal_field21519.clone()),
                            Box::new(source_ml_source2_nominal_field21520.clone()),
                            Some((source_ml_source2_nominal_override21523.clone(),)),
                            source_ml_source2_nominal_field21522.clone(),
                        )
                    }
                }
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                    source_ml_source2_nominal_field21524,
                    source_ml_source2_nominal_field21525,
                    source_ml_source2_nominal_field21526,
                    source_ml_source2_nominal_field21527,
                    _,
                    source_ml_source2_nominal_field21529,
                ) => {
                    let source_ml_source2_nominal_field21524 =
                        *source_ml_source2_nominal_field21524;
                    let source_ml_source2_nominal_field21525 =
                        *source_ml_source2_nominal_field21525;
                    let source_ml_source2_nominal_field21526 =
                        *source_ml_source2_nominal_field21526;
                    {
                        let source_ml_source2_nominal_override21530 =
                            Some(operation.clone().expect("a proven present value"));
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                            Box::new(source_ml_source2_nominal_field21524.clone()),
                            Box::new(source_ml_source2_nominal_field21525.clone()),
                            Box::new(source_ml_source2_nominal_field21526.clone()),
                            source_ml_source2_nominal_field21527.clone(),
                            Some((source_ml_source2_nominal_override21530.clone(),)),
                            source_ml_source2_nominal_field21529.clone(),
                        )
                    }
                }
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                    source_ml_source2_nominal_field21531,
                    source_ml_source2_nominal_field21532,
                    source_ml_source2_nominal_field21533,
                    _,
                    source_ml_source2_nominal_field21535,
                ) => {
                    let source_ml_source2_nominal_field21531 =
                        *source_ml_source2_nominal_field21531;
                    let source_ml_source2_nominal_field21532 =
                        *source_ml_source2_nominal_field21532;
                    let source_ml_source2_nominal_field21533 =
                        *source_ml_source2_nominal_field21533;
                    {
                        let source_ml_source2_nominal_override21536 =
                            Some(operation.clone().expect("a proven present value"));
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                            Box::new(source_ml_source2_nominal_field21531.clone()),
                            Box::new(source_ml_source2_nominal_field21532.clone()),
                            Box::new(source_ml_source2_nominal_field21533.clone()),
                            Some((source_ml_source2_nominal_override21536.clone(),)),
                            source_ml_source2_nominal_field21535.clone(),
                        )
                    }
                }
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                    source_ml_source2_nominal_field21537,
                    source_ml_source2_nominal_field21538,
                    source_ml_source2_nominal_field21539,
                    source_ml_source2_nominal_field21540,
                    _,
                    source_ml_source2_nominal_field21542,
                ) => {
                    let source_ml_source2_nominal_field21537 =
                        *source_ml_source2_nominal_field21537;
                    let source_ml_source2_nominal_field21538 =
                        *source_ml_source2_nominal_field21538;
                    {
                        let source_ml_source2_nominal_override21543 =
                            Some(operation.clone().expect("a proven present value"));
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                            Box::new(source_ml_source2_nominal_field21537.clone()),
                            Box::new(source_ml_source2_nominal_field21538.clone()),
                            source_ml_source2_nominal_field21539.clone(),
                            source_ml_source2_nominal_field21540.clone(),
                            Some((source_ml_source2_nominal_override21543.clone(),)),
                            source_ml_source2_nominal_field21542.clone(),
                        )
                    }
                }
            }
        };
        if match (call.clone(), None::<()>) {
            (Some(_ml_left), Some(_ml_right)) => false,
            (None, None) => true,
            _ => false,
        } {
            match selected.clone() {
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                    source_ml_source2_nominal_field21544,
                    source_ml_source2_nominal_field21545,
                    source_ml_source2_nominal_field21546,
                    source_ml_source2_nominal_field21547,
                    source_ml_source2_nominal_field21548,
                ) => {
                    let source_ml_source2_nominal_field21544 =
                        *source_ml_source2_nominal_field21544;
                    let source_ml_source2_nominal_field21545 =
                        *source_ml_source2_nominal_field21545;
                    let source_ml_source2_nominal_field21546 =
                        *source_ml_source2_nominal_field21546;
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                        Box::new(source_ml_source2_nominal_field21544.clone()),
                        Box::new(source_ml_source2_nominal_field21545.clone()),
                        Box::new(source_ml_source2_nominal_field21546.clone()),
                        source_ml_source2_nominal_field21547.clone(),
                        source_ml_source2_nominal_field21548.clone(),
                    )
                }
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                    source_ml_source2_nominal_field21549,
                    source_ml_source2_nominal_field21550,
                    source_ml_source2_nominal_field21551,
                    source_ml_source2_nominal_field21552,
                    source_ml_source2_nominal_field21553,
                    source_ml_source2_nominal_field21554,
                ) => {
                    let source_ml_source2_nominal_field21549 =
                        *source_ml_source2_nominal_field21549;
                    let source_ml_source2_nominal_field21550 =
                        *source_ml_source2_nominal_field21550;
                    let source_ml_source2_nominal_field21551 =
                        *source_ml_source2_nominal_field21551;
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                        Box::new(source_ml_source2_nominal_field21549.clone()),
                        Box::new(source_ml_source2_nominal_field21550.clone()),
                        Box::new(source_ml_source2_nominal_field21551.clone()),
                        source_ml_source2_nominal_field21552.clone(),
                        source_ml_source2_nominal_field21553.clone(),
                        source_ml_source2_nominal_field21554.clone(),
                    )
                }
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                    source_ml_source2_nominal_field21555,
                    source_ml_source2_nominal_field21556,
                    source_ml_source2_nominal_field21557,
                    source_ml_source2_nominal_field21558,
                    source_ml_source2_nominal_field21559,
                ) => {
                    let source_ml_source2_nominal_field21555 =
                        *source_ml_source2_nominal_field21555;
                    let source_ml_source2_nominal_field21556 =
                        *source_ml_source2_nominal_field21556;
                    let source_ml_source2_nominal_field21557 =
                        *source_ml_source2_nominal_field21557;
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                        Box::new(source_ml_source2_nominal_field21555.clone()),
                        Box::new(source_ml_source2_nominal_field21556.clone()),
                        Box::new(source_ml_source2_nominal_field21557.clone()),
                        source_ml_source2_nominal_field21558.clone(),
                        source_ml_source2_nominal_field21559.clone(),
                    )
                }
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                    source_ml_source2_nominal_field21560,
                    source_ml_source2_nominal_field21561,
                    source_ml_source2_nominal_field21562,
                    source_ml_source2_nominal_field21563,
                    source_ml_source2_nominal_field21564,
                    source_ml_source2_nominal_field21565,
                ) => {
                    let source_ml_source2_nominal_field21560 =
                        *source_ml_source2_nominal_field21560;
                    let source_ml_source2_nominal_field21561 =
                        *source_ml_source2_nominal_field21561;
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                        Box::new(source_ml_source2_nominal_field21560.clone()),
                        Box::new(source_ml_source2_nominal_field21561.clone()),
                        source_ml_source2_nominal_field21562.clone(),
                        source_ml_source2_nominal_field21563.clone(),
                        source_ml_source2_nominal_field21564.clone(),
                        source_ml_source2_nominal_field21565.clone(),
                    )
                }
            }
        } else {
            match selected.clone() {
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                    source_ml_source2_nominal_field21566,
                    source_ml_source2_nominal_field21567,
                    source_ml_source2_nominal_field21568,
                    source_ml_source2_nominal_field21569,
                    _,
                ) => {
                    let source_ml_source2_nominal_field21566 =
                        *source_ml_source2_nominal_field21566;
                    let source_ml_source2_nominal_field21567 =
                        *source_ml_source2_nominal_field21567;
                    let source_ml_source2_nominal_field21568 =
                        *source_ml_source2_nominal_field21568;
                    {
                        let source_ml_source2_nominal_override21571 =
                            Some(call.clone().expect("a proven present value"));
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                            Box::new(source_ml_source2_nominal_field21566.clone()),
                            Box::new(source_ml_source2_nominal_field21567.clone()),
                            Box::new(source_ml_source2_nominal_field21568.clone()),
                            source_ml_source2_nominal_field21569.clone(),
                            Some((source_ml_source2_nominal_override21571.clone(),)),
                        )
                    }
                }
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                    source_ml_source2_nominal_field21572,
                    source_ml_source2_nominal_field21573,
                    source_ml_source2_nominal_field21574,
                    source_ml_source2_nominal_field21575,
                    source_ml_source2_nominal_field21576,
                    _,
                ) => {
                    let source_ml_source2_nominal_field21572 =
                        *source_ml_source2_nominal_field21572;
                    let source_ml_source2_nominal_field21573 =
                        *source_ml_source2_nominal_field21573;
                    let source_ml_source2_nominal_field21574 =
                        *source_ml_source2_nominal_field21574;
                    {
                        let source_ml_source2_nominal_override21578 =
                            Some(call.clone().expect("a proven present value"));
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                            Box::new(source_ml_source2_nominal_field21572.clone()),
                            Box::new(source_ml_source2_nominal_field21573.clone()),
                            Box::new(source_ml_source2_nominal_field21574.clone()),
                            source_ml_source2_nominal_field21575.clone(),
                            source_ml_source2_nominal_field21576.clone(),
                            Some((source_ml_source2_nominal_override21578.clone(),)),
                        )
                    }
                }
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                    source_ml_source2_nominal_field21579,
                    source_ml_source2_nominal_field21580,
                    source_ml_source2_nominal_field21581,
                    source_ml_source2_nominal_field21582,
                    _,
                ) => {
                    let source_ml_source2_nominal_field21579 =
                        *source_ml_source2_nominal_field21579;
                    let source_ml_source2_nominal_field21580 =
                        *source_ml_source2_nominal_field21580;
                    let source_ml_source2_nominal_field21581 =
                        *source_ml_source2_nominal_field21581;
                    {
                        let source_ml_source2_nominal_override21584 =
                            Some(call.clone().expect("a proven present value"));
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                            Box::new(source_ml_source2_nominal_field21579.clone()),
                            Box::new(source_ml_source2_nominal_field21580.clone()),
                            Box::new(source_ml_source2_nominal_field21581.clone()),
                            source_ml_source2_nominal_field21582.clone(),
                            Some((source_ml_source2_nominal_override21584.clone(),)),
                        )
                    }
                }
                self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                    source_ml_source2_nominal_field21585,
                    source_ml_source2_nominal_field21586,
                    source_ml_source2_nominal_field21587,
                    source_ml_source2_nominal_field21588,
                    source_ml_source2_nominal_field21589,
                    _,
                ) => {
                    let source_ml_source2_nominal_field21585 =
                        *source_ml_source2_nominal_field21585;
                    let source_ml_source2_nominal_field21586 =
                        *source_ml_source2_nominal_field21586;
                    {
                        let source_ml_source2_nominal_override21591 =
                            Some(call.clone().expect("a proven present value"));
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                            Box::new(source_ml_source2_nominal_field21585.clone()),
                            Box::new(source_ml_source2_nominal_field21586.clone()),
                            source_ml_source2_nominal_field21587.clone(),
                            source_ml_source2_nominal_field21588.clone(),
                            source_ml_source2_nominal_field21589.clone(),
                            Some((source_ml_source2_nominal_override21591.clone(),)),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_property_definition_call(
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptPropertyDefinitionCall {
    self::JavaScriptPropertyDefinitionCall::PropertyDefinitionCallRecord(
        Box::new(callee.clone()),
        Box::new(receiver.clone()),
        std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(
            &{
                let ml_reference_value1088 = arguments_.borrow().clone();
                ml_reference_value1088
            },
            0f64,
            f64::INFINITY,
        ))),
    )
}
