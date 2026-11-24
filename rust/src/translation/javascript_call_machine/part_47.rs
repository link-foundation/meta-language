// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayAssignmentTransition;
use super::JavaScriptArrayCompletion;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryTestDuplicateJavaScriptObjectSpreadKeyNameLoop298Result;
use super::begin_java_script_array_property_definition;
use super::begin_java_script_scalar_protocol;
use super::finish_java_script_scalar_primitive;
use super::ml;
use super::ml_string;
use super::read_java_script_array_definition_state_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::read_java_script_semantic_text_string;
use super::resume_java_script_array_property_number;
use super::resume_java_script_scalar_protocol;
use super::source_ml_source1_temporary_test_duplicate_java_script_object_spread_key_name_loop298;
use super::step_java_script_array_property_definition;
use super::step_java_script_scalar_protocol;
use super::test_java_script_semantic_same_value;

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
                    source_ml_source2_data_property_value7454,
                    _,
                ) => {
                    let source_ml_source2_data_property_value7454 =
                        *source_ml_source2_data_property_value7454;
                    source_ml_source2_data_property_value7454.clone()
                }
                self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                    source_ml_source2_data_property_value7455,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value7455 =
                        *source_ml_source2_data_property_value7455;
                    source_ml_source2_data_property_value7455.clone()
                }
                self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                    source_ml_source2_data_property_value7456,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value7456 =
                        *source_ml_source2_data_property_value7456;
                    source_ml_source2_data_property_value7456.clone()
                }
            },
        );
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            {
                let key_error = self::read_java_script_ordinary_value_error(
                    heap.clone(),
                    match state.clone() {
                        self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
                            _,
                            source_ml_source2_data_property_value7457,
                        ) => {
                            let source_ml_source2_data_property_value7457 =
                                *source_ml_source2_data_property_value7457;
                            source_ml_source2_data_property_value7457.clone()
                        }
                        self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                            _,
                            source_ml_source2_data_property_value7458,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value7458 =
                                *source_ml_source2_data_property_value7458;
                            source_ml_source2_data_property_value7458.clone()
                        }
                        self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                            _,
                            source_ml_source2_data_property_value7459,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value7459 =
                                *source_ml_source2_data_property_value7459;
                            source_ml_source2_data_property_value7459.clone()
                        }
                    },
                );
                if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                    key_error.clone()
                } else {
                    if {
                        let source_ml_source2_s7463 = match state.clone() {
                            self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
                                _,
                                source_ml_source2_data_property_value7465,
                            ) => {
                                let source_ml_source2_data_property_value7465 =
                                    *source_ml_source2_data_property_value7465;
                                source_ml_source2_data_property_value7465.clone()
                            }
                            self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                                _,
                                source_ml_source2_data_property_value7466,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value7466 =
                                    *source_ml_source2_data_property_value7466;
                                source_ml_source2_data_property_value7466.clone()
                            }
                            self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                                _,
                                source_ml_source2_data_property_value7467,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value7467 =
                                    *source_ml_source2_data_property_value7467;
                                source_ml_source2_data_property_value7467.clone()
                            }
                        };
                        match source_ml_source2_s7463.clone() {
                            self::JavaScriptSemanticValue::Symbol(_) => false,
                            _ => true,
                        }
                    } {
                        self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110,
                            116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                            99, 116, 117, 97, 108, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119,
                            110, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 99, 111, 110,
                            118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108,
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
                                match conversion.clone() {
                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, _) => {
                                        let shared_constructor_field3 = *shared_constructor_field3;
                                        let shared_constructor_field4 = *shared_constructor_field4;
                                        match definition.clone() {
                                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, shared_constructor_field1, shared_constructor_field2, _, _) => {
                                                if ((((shared_constructor_field2 == (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))) || (shared_constructor_field5 != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) || !(self::test_java_script_semantic_same_value(shared_constructor_field3.clone(), (({ let ml_reference_value603 = shared_constructor_field1.borrow().clone(); ml_reference_value603 }).0).8))) || !(self::test_java_script_semantic_same_value(shared_constructor_field4.clone(), exotic_key.clone()))) {
                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 112, 117, 116, 44, 32, 112, 104, 97, 115, 101, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 110, 117, 109, 98, 101, 114, 32, 104, 105, 110, 116])
                                                } else {
                                                    self::read_java_script_primitive_conversion_state_error(heap.clone(), conversion.clone())
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                                definition_2,
                                _,
                                value,
                            ) => {
                                let definition_2 = *definition_2;
                                let value = *value;
                                if (((match definition_2.clone() {
                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value7479,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value7479.clone(),
                                }) == (self::ml_string::Text::from_units(vec![
                                    97, 112, 112, 108, 121,
                                ]))) || match value.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => true,
                                    _ => false,
                                }) {
                                    self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 32, 99, 111, 110, 118, 101, 114,
                                        115, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99,
                                        101, 32, 115, 117, 115, 112, 101, 110, 115, 105, 111, 110,
                                        32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115,
                                        32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 112,
                                        114, 105, 109, 105, 116, 105, 118, 101,
                                    ])
                                } else {
                                    self::read_java_script_ordinary_value_error(
                                        heap.clone(),
                                        value.clone(),
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
                let source_ml_source2_constructor_field7485 = Some(writes.clone());
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCompleted(
                    Box::new(heap_2.clone()),
                    success,
                    Some((source_ml_source2_constructor_field7485.clone(),)),
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
pub fn begin_java_script_array_assignment(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
            (
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
            ),
            Vec<f64>,
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
        self::continue_java_script_array_assignment_definition(
            exotic_key.clone(),
            transition.clone(),
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_assignment(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayAssignmentState,
) -> self::JavaScriptArrayAssignmentTransition {
    {
        let error =
            self::read_java_script_array_assignment_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            match state.clone() {
                self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
                    definition,
                    exotic_key,
                ) => {
                    let definition = *definition;
                    let exotic_key = *exotic_key;
                    {
                        let transition = self::step_java_script_array_property_definition(
                            heap.clone(),
                            definition.clone(),
                        );
                        match transition.clone() {
                            self::JavaScriptArrayCompletion::ArrayResourceLimit(
                                heap_2,
                                reason,
                                _,
                            ) => {
                                let heap_2 = *heap_2;
                                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentResourceLimit(Box::new(heap_2.clone()), Box::new(state.clone()), reason.clone())
                            }
                            _ => self::continue_java_script_array_assignment_definition(
                                exotic_key.clone(),
                                transition.clone(),
                            ),
                        }
                    }
                }
                self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                    definition_2,
                    exotic_key_2,
                    conversion,
                ) => {
                    let definition_2 = *definition_2;
                    let exotic_key_2 = *exotic_key_2;
                    let conversion = *conversion;
                    self::continue_java_script_array_assignment_number(
                        heap.clone(),
                        definition_2.clone(),
                        exotic_key_2.clone(),
                        self::step_java_script_scalar_protocol(
                            heap.clone(),
                            conversion.clone(),
                            &(self::ml_string::Text::from_units(vec![
                                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105,
                                118, 101, 32, 110, 117, 109, 98, 101, 114,
                            ])),
                        ),
                    )
                }
                self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                    definition_3,
                    exotic_key_3,
                    value,
                ) => {
                    let definition_3 = *definition_3;
                    let exotic_key_3 = *exotic_key_3;
                    let value = *value;
                    self::continue_java_script_array_assignment_number(
                        heap.clone(),
                        definition_3.clone(),
                        exotic_key_3.clone(),
                        self::finish_java_script_scalar_primitive(
                            &(self::ml_string::Text::from_units(vec![
                                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105,
                                118, 101, 32, 110, 117, 109, 98, 101, 114,
                            ])),
                            value.clone(),
                        ),
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_array_assignment(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayAssignmentState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArrayAssignmentTransition {
    {
        let error =
            self::read_java_script_array_assignment_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            match state.clone() {
                self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                    definition,
                    exotic_key,
                    conversion,
                ) => {
                    let definition = *definition;
                    let exotic_key = *exotic_key;
                    let conversion = *conversion;
                    self::continue_java_script_array_assignment_number(
                        heap.clone(),
                        definition.clone(),
                        exotic_key.clone(),
                        self::resume_java_script_scalar_protocol(
                            heap.clone(),
                            conversion.clone(),
                            completion.clone(),
                            &(self::ml_string::Text::from_units(vec![
                                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105,
                                118, 101, 32, 110, 117, 109, 98, 101, 114,
                            ])),
                        ),
                    )
                }
                _ => self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 65,
                        114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 97, 108,
                        108, 98, 97, 99, 107, 32, 99, 97, 110, 32, 114, 101, 115, 117, 109, 101,
                        32, 119, 105, 116, 104, 32, 97, 32, 99, 104, 105, 108, 100, 32, 99, 111,
                        109, 112, 108, 101, 116, 105, 111, 110,
                    ]),
                ),
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_spread_key_name(
    key: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match key.clone() {
        self::JavaScriptSemanticValue::Symbol(identity) => {
            (self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108, 58]))
                .concat(&self::ml_string::Text::from_text(&identity.to_string()))
        }
        _ => (self::ml_string::Text::from_units(vec![116, 101, 120, 116, 58]))
            .concat(&self::read_java_script_semantic_text_string(key.clone())),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_duplicate_java_script_object_spread_key_name(
    names: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
) -> bool {
    {
        let ordered = std::rc::Rc::new(std::cell::RefCell::new({
            let mut ml_sorted_values = {
                let ml_reference_value604 = names.borrow().clone();
                ml_reference_value604
            };
            ml_sorted_values
                .sort_by(|ml_sort_left, ml_sort_right| ml_sort_left.0.cmp(&ml_sort_right.0));
            ml_sorted_values
        }));
        {
            let at = 1f64;
            {
                let source_ml_source2_s7488 = self::source_ml_source1_temporary_test_duplicate_java_script_object_spread_key_name_loop298(ordered.clone(), at);
                match source_ml_source2_s7488.clone() {
                    self::MlSource1TemporaryTestDuplicateJavaScriptObjectSpreadKeyNameLoop298Result::MlSource1TemporaryTestDuplicateJavaScriptObjectSpreadKeyNameLoop298Done => {
                        false
                    }
                    self::MlSource1TemporaryTestDuplicateJavaScriptObjectSpreadKeyNameLoop298Result::MlSource1TemporaryTestDuplicateJavaScriptObjectSpreadKeyNameLoop298Return(source_ml_source1_temporary_result298) => {
                        source_ml_source1_temporary_result298
                    }
                }
            }
        }
    }
}
