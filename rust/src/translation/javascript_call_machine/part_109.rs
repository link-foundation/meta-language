// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPropertyDefinitionCall;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::complete_java_script_property_definition_with_symbol;
use super::continue_java_script_property_array_definition;
use super::continue_java_script_property_descriptor_definition_with_symbol;
use super::continue_java_script_property_key_definition_with_symbol;
use super::ml_string;
use super::project_java_script_property_key_completion;
use super::read_java_script_property_definition_state_error;
use super::resume_java_script_array_assignment;
use super::resume_java_script_descriptor_conversion;
use super::resume_java_script_property_key_conversion;
use super::step_java_script_array_assignment;
use super::step_java_script_descriptor_conversion;
use super::step_java_script_property_key_conversion;
use super::test_java_script_standard_property_definitions_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_property_definition(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPropertyDefinitionState,
) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error =
            self::read_java_script_property_definition_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                error.clone(),
                None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
            )
        } else {
            if !(match (
                match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value21703,
                    ) => ((source_ml_source2_data_property_value21703
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,)))
                    .0)
                        .clone(),
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value21704,
                    ) => ((source_ml_source2_data_property_value21704
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,)))
                    .0)
                        .clone(),
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value21705,
                    ) => ((source_ml_source2_data_property_value21705
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,)))
                    .0)
                        .clone(),
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value21706,
                    ) => ((source_ml_source2_data_property_value21706
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }) {
                {
                    let source_ml_source2_constructor_field21707 = Some(heap.clone());
                    {
                        let source_ml_source2_constructor_field21714 = match (match state.clone() {
                            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, _, _, source_ml_source2_data_property_value21709) => {
                                ((source_ml_source2_data_property_value21709.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                            }
                            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(_, _, _, _, _, source_ml_source2_data_property_value21710) => {
                                ((source_ml_source2_data_property_value21710.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                            }
                            self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, _, source_ml_source2_data_property_value21711) => {
                                ((source_ml_source2_data_property_value21711.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                            }
                            self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, _, source_ml_source2_data_property_value21712) => {
                                ((source_ml_source2_data_property_value21712.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                            }
                        }).clone().expect("a proven present value") {
                            self::JavaScriptPropertyDefinitionCall::PropertyDefinitionCallRecord(source_ml_source2_data_property_value21713, _, _) => {
                                let source_ml_source2_data_property_value21713 = *source_ml_source2_data_property_value21713;
                                source_ml_source2_data_property_value21713.clone()
                            }
                        };
                        {
                            let source_ml_source2_constructor_field21720 = match (match state.clone() {
                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, _, _, source_ml_source2_data_property_value21715) => {
                                    ((source_ml_source2_data_property_value21715.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                }
                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(_, _, _, _, _, source_ml_source2_data_property_value21716) => {
                                    ((source_ml_source2_data_property_value21716.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                }
                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, _, source_ml_source2_data_property_value21717) => {
                                    ((source_ml_source2_data_property_value21717.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                }
                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, _, source_ml_source2_data_property_value21718) => {
                                    ((source_ml_source2_data_property_value21718.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                }
                            }).clone().expect("a proven present value") {
                                self::JavaScriptPropertyDefinitionCall::PropertyDefinitionCallRecord(_, source_ml_source2_data_property_value21719, _) => {
                                    let source_ml_source2_data_property_value21719 = *source_ml_source2_data_property_value21719;
                                    source_ml_source2_data_property_value21719.clone()
                                }
                            };
                            {
                                let source_ml_source2_constructor_field21726 = match (match state.clone() {
                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(_, _, _, _, source_ml_source2_data_property_value21721) => {
                                        ((source_ml_source2_data_property_value21721.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                    }
                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(_, _, _, _, _, source_ml_source2_data_property_value21722) => {
                                        ((source_ml_source2_data_property_value21722.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                    }
                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(_, _, _, _, source_ml_source2_data_property_value21723) => {
                                        ((source_ml_source2_data_property_value21723.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                    }
                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(_, _, _, _, _, source_ml_source2_data_property_value21724) => {
                                        ((source_ml_source2_data_property_value21724.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value") {
                                    self::JavaScriptPropertyDefinitionCall::PropertyDefinitionCallRecord(_, _, source_ml_source2_data_property_value21725) => {
                                        source_ml_source2_data_property_value21725.clone()
                                    }
                                };
                                self::JavaScriptPropertyDefinitionTransition::DefinitionCall(
                                    Box::new(state.clone()),
                                    Box::new(source_ml_source2_constructor_field21714.clone()),
                                    Box::new(source_ml_source2_constructor_field21720.clone()),
                                    source_ml_source2_constructor_field21726.clone(),
                                    Some((source_ml_source2_constructor_field21707.clone(),)),
                                )
                            }
                        }
                    }
                }
            } else {
                if {
                    let source_ml_source2_s21731 = match state.clone() {
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                            source_ml_source2_data_property_value21734,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value21734 =
                                *source_ml_source2_data_property_value21734;
                            source_ml_source2_data_property_value21734.clone()
                        }
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                            source_ml_source2_data_property_value21735,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value21735 =
                                *source_ml_source2_data_property_value21735;
                            source_ml_source2_data_property_value21735.clone()
                        }
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                            source_ml_source2_data_property_value21736,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value21736 =
                                *source_ml_source2_data_property_value21736;
                            source_ml_source2_data_property_value21736.clone()
                        }
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                            source_ml_source2_data_property_value21737,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value21737 =
                                *source_ml_source2_data_property_value21737;
                            source_ml_source2_data_property_value21737.clone()
                        }
                    };
                    match source_ml_source2_s21731.clone() {
                        self::JavaScriptSemanticValue::Object(_, _) => false,
                        _ => true,
                    }
                } {
                    self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                        self::ml_string::Text::from_units(vec![
                            97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105,
                            110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97,
                            116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                            115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32,
                            116, 97, 114, 103, 101, 116,
                        ]),
                        None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
                    )
                } else {
                    match state.clone() {
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                            target,
                            attributes,
                            conversion,
                            operation,
                            _,
                        ) => {
                            let target = *target;
                            let attributes = *attributes;
                            let conversion = *conversion;
                            self::continue_java_script_property_key_definition_with_symbol(heap.clone(), target.clone(), attributes.clone(), self::step_java_script_property_key_conversion(heap.clone(), conversion.clone()), Some(match conversion.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value21743, _, _) => {
                                    let source_ml_source2_data_property_value21743 = *source_ml_source2_data_property_value21743;
                                    source_ml_source2_data_property_value21743.clone()
                                }
                            }), ((operation.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone())
                        }
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                            target_2,
                            key,
                            conversion_2,
                            exotic_key,
                            operation_2,
                            _,
                        ) => {
                            let target_2 = *target_2;
                            let key = *key;
                            let conversion_2 = *conversion_2;
                            self::continue_java_script_property_descriptor_definition_with_symbol(
                                heap.clone(),
                                target_2.clone(),
                                key.clone(),
                                self::step_java_script_descriptor_conversion(
                                    heap.clone(),
                                    conversion_2.clone(),
                                ),
                                ((exotic_key
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                                ((operation_2
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                .0)
                                    .clone(),
                            )
                        }
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                            target_3,
                            key_2,
                            assignment,
                            operation_3,
                            _,
                        ) => {
                            let target_3 = *target_3;
                            let key_2 = *key_2;
                            let assignment = *assignment;
                            self::continue_java_script_property_array_definition(
                                target_3.clone(),
                                key_2.clone(),
                                self::step_java_script_array_assignment(
                                    heap.clone(),
                                    assignment.clone(),
                                ),
                                ((operation_3
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                .0)
                                    .clone(),
                            )
                        }
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                            target_4,
                            key_3,
                            descriptor,
                            exotic_key_2,
                            operation_4,
                            _,
                        ) => {
                            let target_4 = *target_4;
                            let key_3 = *key_3;
                            self::complete_java_script_property_definition_with_symbol(
                                heap.clone(),
                                target_4.clone(),
                                key_3.clone(),
                                descriptor.clone(),
                                ((exotic_key_2
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                                ((operation_4
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                .0)
                                    .clone(),
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
pub fn resume_java_script_property_definition(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPropertyDefinitionState,
    completion: self::JavaScriptSemanticCompletion,
) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error =
            self::read_java_script_property_definition_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                error.clone(),
                None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
            )
        } else {
            if {
                let source_ml_source2_s21749 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                        source_ml_source2_data_property_value21752,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value21752 =
                            *source_ml_source2_data_property_value21752;
                        source_ml_source2_data_property_value21752.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                        source_ml_source2_data_property_value21753,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value21753 =
                            *source_ml_source2_data_property_value21753;
                        source_ml_source2_data_property_value21753.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                        source_ml_source2_data_property_value21754,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value21754 =
                            *source_ml_source2_data_property_value21754;
                        source_ml_source2_data_property_value21754.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                        source_ml_source2_data_property_value21755,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value21755 =
                            *source_ml_source2_data_property_value21755;
                        source_ml_source2_data_property_value21755.clone()
                    }
                };
                match source_ml_source2_s21749.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => false,
                    _ => true,
                }
            } {
                self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                    self::ml_string::Text::from_units(vec![
                        97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105,
                        110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97,
                        116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                        115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116,
                        97, 114, 103, 101, 116,
                    ]),
                    None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
                )
            } else {
                match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                        target,
                        attributes,
                        conversion,
                        operation,
                        _,
                    ) => {
                        let target = *target;
                        let attributes = *attributes;
                        let conversion = *conversion;
                        self::continue_java_script_property_key_definition_with_symbol(
                            heap.clone(),
                            target.clone(),
                            attributes.clone(),
                            self::resume_java_script_property_key_conversion(
                                heap.clone(),
                                conversion.clone(),
                                self::project_java_script_property_key_completion(
                                    completion.clone(),
                                ),
                            ),
                            Some(match conversion.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                    _,
                                    source_ml_source2_data_property_value21766,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value21766 =
                                        *source_ml_source2_data_property_value21766;
                                    source_ml_source2_data_property_value21766.clone()
                                }
                            }),
                            ((operation
                                .clone()
                                .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                            .0)
                                .clone(),
                        )
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                        target_2,
                        key,
                        conversion_2,
                        exotic_key,
                        operation_2,
                        _,
                    ) => {
                        let target_2 = *target_2;
                        let key = *key;
                        let conversion_2 = *conversion_2;
                        self::continue_java_script_property_descriptor_definition_with_symbol(
                            heap.clone(),
                            target_2.clone(),
                            key.clone(),
                            self::resume_java_script_descriptor_conversion(
                                heap.clone(),
                                conversion_2.clone(),
                                completion.clone(),
                            ),
                            ((exotic_key
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                            .0)
                                .clone(),
                            ((operation_2
                                .clone()
                                .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                            .0)
                                .clone(),
                        )
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                        target_3,
                        key_2,
                        assignment,
                        operation_3,
                        _,
                    ) => {
                        let target_3 = *target_3;
                        let key_2 = *key_2;
                        let assignment = *assignment;
                        self::continue_java_script_property_array_definition(
                            target_3.clone(),
                            key_2.clone(),
                            self::resume_java_script_array_assignment(
                                heap.clone(),
                                assignment.clone(),
                                self::project_java_script_property_key_completion(
                                    completion.clone(),
                                ),
                            ),
                            ((operation_3
                                .clone()
                                .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                            .0)
                                .clone(),
                        )
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_constructor_field21767 = Some(heap.clone());
                        self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                            self::ml_string::Text::from_units(vec![
                                97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 100, 101,
                                115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 116, 111, 114, 97,
                                103, 101, 32, 115, 117, 115, 112, 101, 110, 115, 105, 111, 110, 32,
                                99, 97, 110, 110, 111, 116, 32, 97, 99, 99, 101, 112, 116, 32, 97,
                                32, 99, 104, 105, 108, 100, 32, 99, 111, 109, 112, 108, 101, 116,
                                105, 111, 110,
                            ]),
                            Some((source_ml_source2_constructor_field21767.clone(),)),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_property_definitions_operation(operation: &self::ml_string::Text) -> bool {
    self::test_java_script_standard_property_definitions_operation(operation)
}
