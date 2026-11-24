// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallCompletion;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionState;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::JavaScriptSpeciesConstructionState;
use super::JavaScriptTextCallState;
use super::MlSource1TemporaryFinishJavaScriptCallReturnJoin1507;
use super::MlSource1TemporaryFinishJavaScriptCallReturnJoin1508;
use super::MlSource1TemporaryFinishJavaScriptCallReturnJoin1509;
use super::accept_java_script_call_array_assignment;
use super::accept_java_script_call_array_callback;
use super::accept_java_script_call_array_concatenation;
use super::accept_java_script_call_array_construction;
use super::accept_java_script_call_array_creation;
use super::accept_java_script_call_array_iteration;
use super::accept_java_script_call_array_search;
use super::accept_java_script_call_array_slice;
use super::accept_java_script_call_array_spread;
use super::accept_java_script_call_array_transform;
use super::accept_java_script_call_collection_for_each;
use super::accept_java_script_call_completion;
use super::accept_java_script_call_construction;
use super::accept_java_script_call_definition;
use super::accept_java_script_call_function_bind;
use super::accept_java_script_call_instance_of;
use super::accept_java_script_call_iterator_protocol;
use super::accept_java_script_call_key;
use super::accept_java_script_call_object_enumeration;
use super::accept_java_script_call_object_from_entries;
use super::accept_java_script_call_object_spread;
use super::accept_java_script_call_property_definitions;
use super::accept_java_script_call_species_construction;
use super::accept_java_script_call_text;
use super::finish_java_script_scalar_primitive;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::project_java_script_call_iterator_operation;
use super::read_java_script_call_array_transform_directory;
use super::read_java_script_executable_call_function;
use super::read_java_script_text_call_source_operation;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::resume_java_script_array_assignment;
use super::resume_java_script_array_callback;
use super::resume_java_script_array_concatenation_call;
use super::resume_java_script_array_creation_call;
use super::resume_java_script_array_iterator_call;
use super::resume_java_script_array_search;
use super::resume_java_script_array_slice_call;
use super::resume_java_script_array_spread;
use super::resume_java_script_array_transform_call;
use super::resume_java_script_call_number;
use super::resume_java_script_collection_for_each;
use super::resume_java_script_construction_call;
use super::resume_java_script_function_bind_call;
use super::resume_java_script_instance_of;
use super::resume_java_script_iterator_protocol_call;
use super::resume_java_script_object_enumeration_getter;
use super::resume_java_script_object_from_entries_call;
use super::resume_java_script_object_spread;
use super::resume_java_script_property_definition;
use super::resume_java_script_property_definitions_call;
use super::resume_java_script_property_key_conversion;
use super::resume_java_script_scalar_protocol;
use super::resume_java_script_species_construction_call;
use super::step_java_script_scalar_protocol;
use super::test_java_script_error_constructor_mode;
use super::test_java_script_ordinary_class_receiver_result;
use super::unwind_java_script_call_completion;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_call_text(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    state: self::JavaScriptTextCallState,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = {
                    let source_ml_source2_array_read_value35813 = shared_constructor_field2.clone();
                    {
                        let source_ml_source2_array_read_index35814 = ({
                            let ml_reference_array2311 = shared_constructor_field2.clone();
                            let ml_reference_projection2311 =
                                ml_reference_array2311.borrow().len() as f64;
                            ml_reference_projection2311
                        } - 1f64);
                        {
                            let ml_reference_array2312 =
                                source_ml_source2_array_read_value35813.clone();
                            let ml_reference_position2312 = source_ml_source2_array_read_index35814;
                            let ml_reference_projection2312 = self::ml_array::optional_at(
                                &*ml_reference_array2312.borrow(),
                                self::ml_array::number_index(ml_reference_position2312),
                            );
                            ml_reference_projection2312
                        }
                    }
                };
                if match (frame.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(shared_constructor_field1.clone()),
                        self::ml_string::Text::from_units(vec![
                            116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                            110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                            117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109,
                            101,
                        ]),
                    )
                } else {
                    {
                        let fn_ = self::read_java_script_executable_call_function(
                            program.clone(),
                            shared_constructor_field1.clone(),
                            match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(
                                    source_ml_source2_data_property_value35817,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value35817.clone(),
                            },
                        );
                        if match (fn_.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                self::ml_string::Text::from_units(vec![
                                    116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105,
                                    111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32,
                                    97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32,
                                    102, 117, 110, 99, 116, 105, 111, 110,
                                ]),
                            )
                        } else {
                            {
                                let operation = self::read_java_script_text_call_source_operation(
                                    fn_.clone().expect("a proven present value"),
                                    frame.clone().expect("a proven present value"),
                                    destination,
                                    realm.clone(),
                                    match program.clone() {
                                        self::JavaScriptCallProgram::CallProgram(
                                            _,
                                            _,
                                            source_ml_source2_data_property_value35819,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let source_ml_source2_data_property_value35819 =
                                                *source_ml_source2_data_property_value35819;
                                            source_ml_source2_data_property_value35819.clone()
                                        }
                                    },
                                    state.clone(),
                                );
                                if (operation == (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field1.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            112, 101, 110, 100, 105, 110, 103, 32, 116, 101, 120,
                                            116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111,
                                            117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117,
                                            99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105,
                                            118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32,
                                            97, 110, 100, 32, 114, 101, 97, 108, 109,
                                        ]),
                                    )
                                } else {
                                    match state.clone() {
                                        self::JavaScriptTextCallState::TextProtocolState(
                                            state_state,
                                        ) => {
                                            let state_state = *state_state;
                                            self::accept_java_script_call_text(
                                                program.clone(),
                                                machine.clone(),
                                                destination,
                                                realm.clone(),
                                                self::step_java_script_scalar_protocol(
                                                    shared_constructor_field1.clone(),
                                                    state_state.clone(),
                                                    &(operation),
                                                ),
                                            )
                                        }
                                        self::JavaScriptTextCallState::TextPrimitiveValue(
                                            value,
                                        ) => {
                                            let value = *value;
                                            self::accept_java_script_call_text(
                                                program.clone(),
                                                machine.clone(),
                                                destination,
                                                realm.clone(),
                                                self::finish_java_script_scalar_primitive(
                                                    &(operation),
                                                    value.clone(),
                                                ),
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_text(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    state: self::JavaScriptTextCallState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = {
                    let source_ml_source2_array_read_value35824 = shared_constructor_field2.clone();
                    {
                        let source_ml_source2_array_read_index35825 = ({
                            let ml_reference_array2313 = shared_constructor_field2.clone();
                            let ml_reference_projection2313 =
                                ml_reference_array2313.borrow().len() as f64;
                            ml_reference_projection2313
                        } - 1f64);
                        {
                            let ml_reference_array2314 =
                                source_ml_source2_array_read_value35824.clone();
                            let ml_reference_position2314 = source_ml_source2_array_read_index35825;
                            let ml_reference_projection2314 = self::ml_array::optional_at(
                                &*ml_reference_array2314.borrow(),
                                self::ml_array::number_index(ml_reference_position2314),
                            );
                            ml_reference_projection2314
                        }
                    }
                };
                if match (frame.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(shared_constructor_field1.clone()),
                        self::ml_string::Text::from_units(vec![
                            116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                            110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                            117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109,
                            101,
                        ]),
                    )
                } else {
                    {
                        let fn_ = self::read_java_script_executable_call_function(
                            program.clone(),
                            shared_constructor_field1.clone(),
                            match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(
                                    source_ml_source2_data_property_value35828,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value35828.clone(),
                            },
                        );
                        if match (fn_.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                self::ml_string::Text::from_units(vec![
                                    116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105,
                                    111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32,
                                    97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32,
                                    102, 117, 110, 99, 116, 105, 111, 110,
                                ]),
                            )
                        } else {
                            {
                                let operation = self::read_java_script_text_call_source_operation(
                                    fn_.clone().expect("a proven present value"),
                                    frame.clone().expect("a proven present value"),
                                    destination,
                                    realm.clone(),
                                    match program.clone() {
                                        self::JavaScriptCallProgram::CallProgram(
                                            _,
                                            _,
                                            source_ml_source2_data_property_value35830,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let source_ml_source2_data_property_value35830 =
                                                *source_ml_source2_data_property_value35830;
                                            source_ml_source2_data_property_value35830.clone()
                                        }
                                    },
                                    state.clone(),
                                );
                                if (operation == (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field1.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            112, 101, 110, 100, 105, 110, 103, 32, 116, 101, 120,
                                            116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                                            110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111,
                                            117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117,
                                            99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105,
                                            118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32,
                                            97, 110, 100, 32, 114, 101, 97, 108, 109,
                                        ]),
                                    )
                                } else {
                                    match state.clone() {
                                        self::JavaScriptTextCallState::TextProtocolState(
                                            state_state,
                                        ) => {
                                            let state_state = *state_state;
                                            self::accept_java_script_call_text(program.clone(), machine.clone(), destination, realm.clone(), self::resume_java_script_scalar_protocol(shared_constructor_field1.clone(), state_state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone())), &(operation)))
                                        }
                                        self::JavaScriptTextCallState::TextPrimitiveValue(_) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                                Box::new(shared_constructor_field1.clone()),
                                                self::ml_string::Text::from_units(vec![
                                                    97, 32, 99, 111, 109, 112, 108, 101, 116, 101,
                                                    100, 32, 115, 116, 114, 105, 110, 103, 32, 112,
                                                    114, 105, 109, 105, 116, 105, 118, 101, 32, 99,
                                                    97, 110, 110, 111, 116, 32, 104, 97, 118, 101,
                                                    32, 97, 32, 115, 117, 115, 112, 101, 110, 100,
                                                    101, 100, 32, 99, 104, 105, 108, 100, 32, 99,
                                                    97, 108, 108,
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn return_java_script_call_value(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallMachineTransition {
    {
        let transition = self::unwind_java_script_call_completion(
            machine.clone(),
            self::JavaScriptCallCompletion::CallCompletion(
                self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]),
                Box::new(value.clone()),
                (-1f64),
                0f64,
            ),
        );
        self::accept_java_script_call_completion(program.clone(), transition.clone())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_call_return(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let active = {
                    let source_ml_source2_array_read_value35837 = shared_constructor_field2.clone();
                    {
                        let source_ml_source2_array_read_index35838 = ({
                            let ml_reference_array2315 = shared_constructor_field2.clone();
                            let ml_reference_projection2315 =
                                ml_reference_array2315.borrow().len() as f64;
                            ml_reference_projection2315
                        } - 1f64);
                        {
                            let ml_reference_array2316 =
                                source_ml_source2_array_read_value35837.clone();
                            let ml_reference_position2316 = source_ml_source2_array_read_index35838;
                            let ml_reference_projection2316 = self::ml_array::optional_at(
                                &*ml_reference_array2316.borrow(),
                                self::ml_array::number_index(ml_reference_position2316),
                            );
                            ml_reference_projection2316
                        }
                    }
                };
                {
                    let owner = if match (active.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        None::<self::JavaScriptCallFunction>
                    } else {
                        self::read_java_script_executable_call_function(
                            program.clone(),
                            shared_constructor_field1.clone(),
                            match active.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(
                                    source_ml_source2_data_property_value35840,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value35840.clone(),
                            },
                        )
                    };
                    {
                        let source_ml_source2_s35848 = if ((!(match (active.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }) && !(match (
                            owner.clone(),
                            None::<()>,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }))
                            && self::test_java_script_ordinary_class_receiver_result(
                                &(match owner.clone().expect("a proven present value") {
                                    self::JavaScriptCallFunction::CallFunction(
                                        _,
                                        _,
                                        source_ml_source2_data_property_value35849,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value35849.clone(),
                                }),
                                &(match value.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::ml_string::Text::from_units(vec![
                                            117, 110, 100, 101, 102, 105, 110, 101, 100,
                                        ])
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                                    }
                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                        self::ml_string::Text::from_units(vec![
                                            98, 111, 111, 108, 101, 97, 110,
                                        ])
                                    }
                                    self::JavaScriptSemanticValue::Number(_) => {
                                        self::ml_string::Text::from_units(vec![
                                            110, 117, 109, 98, 101, 114,
                                        ])
                                    }
                                    self::JavaScriptSemanticValue::Integer(_) => {
                                        self::ml_string::Text::from_units(vec![
                                            105, 110, 116, 101, 103, 101, 114,
                                        ])
                                    }
                                    self::JavaScriptSemanticValue::Text(_) => {
                                        self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                                    }
                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                        self::ml_string::Text::from_units(vec![
                                            115, 111, 117, 114, 99, 101, 84, 101, 120, 116,
                                        ])
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        self::ml_string::Text::from_units(vec![
                                            115, 121, 109, 98, 111, 108,
                                        ])
                                    }
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        self::ml_string::Text::from_units(vec![
                                            111, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                }),
                            )) {
                            {
                                let receiver = ({
                                    let source_ml_source2_array_read_value35851 =
                                        match active.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(
                                                _,
                                                _,
                                                source_ml_source2_data_property_value35850,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source2_data_property_value35850.clone(),
                                        };
                                    {
                                        let source_ml_source2_array_read_index35852 = 0f64;
                                        {
                                            let ml_reference_array2317 =
                                                source_ml_source2_array_read_value35851.clone();
                                            let ml_reference_position2317 =
                                                source_ml_source2_array_read_index35852;
                                            let ml_reference_projection2317 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array2317.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position2317,
                                                    ),
                                                );
                                            ml_reference_projection2317
                                        }
                                    }
                                })
                                .clone()
                                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                match receiver.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1507::MlSource1TemporaryFinishJavaScriptCallReturnJoin1507Next(Box::new(receiver.clone()))
                                    }
                                    _ => {
                                        self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1507::MlSource1TemporaryFinishJavaScriptCallReturnJoin1507Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1507::MlSource1TemporaryFinishJavaScriptCallReturnJoin1507Next(Box::new(value.clone()))
                        };
                        match source_ml_source2_s35848.clone() {
                            self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1507::MlSource1TemporaryFinishJavaScriptCallReturnJoin1507Next(value_2) => {
                                let value_2 = *value_2;
                                {
                                    let source_ml_source2_s35869 = if ((!(match (active.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) && self::test_java_script_error_constructor_mode(&(match owner.clone().expect("a proven present value") {
                                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value35870, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value35870.clone()
                                        }
                                    }))) {
                                        {
                                            let source_ml_source2_s35875 = match value_2.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    {
                                                        let receiver_2 = ({
                                                            let source_ml_source2_array_read_value35877 = match active.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value35876, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value35876.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_array_read_index35878 = 0f64;
                                                                { let ml_reference_array2318 = source_ml_source2_array_read_value35877.clone(); let ml_reference_position2318 = source_ml_source2_array_read_index35878; let ml_reference_projection2318 = self::ml_array::optional_at(&*ml_reference_array2318.borrow(), self::ml_array::number_index(ml_reference_position2318)); ml_reference_projection2318 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                        match receiver_2.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1509::MlSource1TemporaryFinishJavaScriptCallReturnJoin1509Return(Box::new(self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value35879, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value35879.clone()
                                                                    }
                                                                }, &(self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![97, 32, 100, 101, 114, 105, 118, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 109, 117, 115, 116, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 32, 116, 104, 105, 115, 32, 116, 104, 114, 111, 117, 103, 104, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 101, 114, 32, 99, 97, 108, 108])))))
                                                            }
                                                            _ => {
                                                                self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1509::MlSource1TemporaryFinishJavaScriptCallReturnJoin1509Next(Box::new(receiver_2.clone()))
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1509::MlSource1TemporaryFinishJavaScriptCallReturnJoin1509Next(Box::new(value_2.clone()))
                                                }
                                            };
                                            match source_ml_source2_s35875.clone() {
                                                self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1509::MlSource1TemporaryFinishJavaScriptCallReturnJoin1509Next(value_3) => {
                                                    let value_3 = *value_3;
                                                    match value_3.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1508::MlSource1TemporaryFinishJavaScriptCallReturnJoin1508Next(Box::new(value_3.clone()))
                                                        }
                                                        _ => {
                                                            self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1508::MlSource1TemporaryFinishJavaScriptCallReturnJoin1508Return(Box::new(self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value35882, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value35882.clone()
                                                                }
                                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![97, 32, 100, 101, 114, 105, 118, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 116, 117, 114, 110, 32, 97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101])))))
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1509::MlSource1TemporaryFinishJavaScriptCallReturnJoin1509Return(source_ml_source1_temporary_result1509) => {
                                                    let source_ml_source1_temporary_result1509 = *source_ml_source1_temporary_result1509;
                                                    self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1508::MlSource1TemporaryFinishJavaScriptCallReturnJoin1508Return(Box::new(source_ml_source1_temporary_result1509.clone()))
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1508::MlSource1TemporaryFinishJavaScriptCallReturnJoin1508Next(Box::new(value_2.clone()))
                                    };
                                    match source_ml_source2_s35869.clone() {
                                        self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1508::MlSource1TemporaryFinishJavaScriptCallReturnJoin1508Next(value_4) => {
                                            let value_4 = *value_4;
                                            {
                                                let frames = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value2319 = shared_constructor_field2.borrow().clone(); ml_reference_value2319 }, 0f64, (-1f64))));
                                                {
                                                    let parent = {
                                                        let source_ml_source2_array_read_index35886 = ({ let ml_reference_array2320 = frames.clone(); let ml_reference_projection2320 = ml_reference_array2320.borrow().len() as f64; ml_reference_projection2320 } - 1f64);
                                                        { let ml_reference_array2321 = frames.clone(); let ml_reference_position2321 = source_ml_source2_array_read_index35886; let ml_reference_projection2321 = self::ml_array::optional_at(&*ml_reference_array2321.borrow(), self::ml_array::number_index(ml_reference_position2321)); ml_reference_projection2321 }
                                                    };
                                                    if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptCallMachineTransition::MachineCompleted(Box::new(shared_constructor_field1.clone()), Box::new(value_4.clone()))
                                                    } else {
                                                        {
                                                            let current = self::JavaScriptCallMachine::CallMachine(Box::new(shared_constructor_field1.clone()), frames.clone());
                                                            {
                                                                let pending = match parent.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value35889, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value35889 = *source_ml_source2_data_property_value35889;
                                                                        source_ml_source2_data_property_value35889.clone()
                                                                    }
                                                                };
                                                                match pending.clone() {
                                                                    self::JavaScriptCallPending::CallIdle => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitCall(destination) => {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), frames.clone(), self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination, value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitGet(destination_2) => {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), frames.clone(), self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_2, value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitSet(destination_3) => {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), frames.clone(), self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_3, self::JavaScriptSemanticValue::Boolean(true)))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitHostSet(destination_4) => {
                                                                        match value_4.clone() {
                                                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), frames.clone(), self::write_java_script_call_register(parent.clone().expect("a proven present value"), destination_4, value_4.clone()))))
                                                                            }
                                                                            _ => {
                                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 83, 101, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66, 111, 111, 108, 101, 97, 110, 32, 114, 101, 115, 117, 108, 116]))
                                                                            }
                                                                        }
                                                                    }
                                                                    self::JavaScriptCallPending::WaitArgumentRest(_, _, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitIteratorProtocol(_, _, _, _, _, state) => {
                                                                        let state = *state;
                                                                        self::accept_java_script_call_iterator_protocol(program.clone(), current.clone(), self::project_java_script_call_iterator_operation(pending.clone()), self::resume_java_script_iterator_protocol_call(shared_constructor_field1.clone(), state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitArraySpread(destination_5, realm, state_2) => {
                                                                        let state_2 = *state_2;
                                                                        self::accept_java_script_call_array_spread(program.clone(), current.clone(), destination_5, realm.clone(), self::resume_java_script_array_spread(shared_constructor_field1.clone(), state_2.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone())), false))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitArrayIteratorNext(_, _, state_3) => {
                                                                        let state_3 = *state_3;
                                                                        self::accept_java_script_call_array_iteration(program.clone(), current.clone(), self::resume_java_script_array_iterator_call(shared_constructor_field1.clone(), state_3.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitCollectionIteratorResult(_, _, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitArrayConcatenation(destination_6, realm_2, state_4, pending_construction) => {
                                                                        let state_4 = *state_4;
                                                                        {
                                                                            let construction = ((pending_construction.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone();
                                                                            {
                                                                                let directory = self::read_java_script_call_array_transform_directory(program.clone(), current.clone(), destination_6, realm_2.clone());
                                                                                if match (directory.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 117, 114, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 101, 99, 105, 101, 115, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                                                } else {
                                                                                    if !(match (construction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                        self::accept_java_script_call_species_construction(program.clone(), current.clone(), self::resume_java_script_species_construction_call(program.clone(), shared_constructor_field1.clone(), directory.clone().expect("a proven present value"), construction.clone().expect("a proven present value"), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                                    } else {
                                                                                        self::accept_java_script_call_array_concatenation(program.clone(), current.clone(), destination_6, realm_2.clone(), self::resume_java_script_array_concatenation_call(shared_constructor_field1.clone(), directory.clone().expect("a proven present value"), state_4.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::JavaScriptCallPending::WaitArraySlice(destination_7, realm_3, state_5, pending_construction_2) => {
                                                                        let state_5 = *state_5;
                                                                        {
                                                                            let construction_2 = ((pending_construction_2.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone();
                                                                            {
                                                                                let directory_2 = self::read_java_script_call_array_transform_directory(program.clone(), current.clone(), destination_7, realm_3.clone());
                                                                                if match (directory_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 108, 105, 99, 101, 32, 114, 101, 116, 117, 114, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 101, 99, 105, 101, 115, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                                                } else {
                                                                                    if !(match (construction_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                        self::accept_java_script_call_species_construction(program.clone(), current.clone(), self::resume_java_script_species_construction_call(program.clone(), shared_constructor_field1.clone(), directory_2.clone().expect("a proven present value"), construction_2.clone().expect("a proven present value"), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                                    } else {
                                                                                        self::accept_java_script_call_array_slice(program.clone(), current.clone(), destination_7, realm_3.clone(), self::resume_java_script_array_slice_call(shared_constructor_field1.clone(), directory_2.clone().expect("a proven present value"), state_5.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::JavaScriptCallPending::WaitArrayTransform(destination_8, realm_4, state_6, pending_construction_3) => {
                                                                        let state_6 = *state_6;
                                                                        {
                                                                            let construction_3 = ((pending_construction_3.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone();
                                                                            {
                                                                                let directory_3 = self::read_java_script_call_array_transform_directory(program.clone(), current.clone(), destination_8, realm_4.clone());
                                                                                if match (directory_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 114, 101, 116, 117, 114, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                                                                } else {
                                                                                    if !(match (construction_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                        self::accept_java_script_call_species_construction(program.clone(), current.clone(), self::resume_java_script_species_construction_call(program.clone(), shared_constructor_field1.clone(), directory_3.clone().expect("a proven present value"), construction_3.clone().expect("a proven present value"), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                                    } else {
                                                                                        self::accept_java_script_call_array_transform(program.clone(), current.clone(), destination_8, realm_4.clone(), self::resume_java_script_array_transform_call(shared_constructor_field1.clone(), directory_3.clone().expect("a proven present value"), state_6.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::JavaScriptCallPending::WaitArraySearch(destination_9, realm_5, state_7) => {
                                                                        let state_7 = *state_7;
                                                                        self::accept_java_script_call_array_search(program.clone(), current.clone(), destination_9, realm_5.clone(), self::resume_java_script_array_search(shared_constructor_field1.clone(), state_7.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitCollectionForEach(destination_10, realm_6, state_8) => {
                                                                        let state_8 = *state_8;
                                                                        self::accept_java_script_call_collection_for_each(program.clone(), current.clone(), destination_10, realm_6.clone(), self::resume_java_script_collection_for_each(shared_constructor_field1.clone(), state_8.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitArrayCallback(destination_11, realm_7, state_9) => {
                                                                        let state_9 = *state_9;
                                                                        self::accept_java_script_call_array_callback(program.clone(), current.clone(), destination_11, realm_7.clone(), self::resume_java_script_array_callback(shared_constructor_field1.clone(), state_9.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitObjectSpread(destination_12, realm_8, state_10) => {
                                                                        let state_10 = *state_10;
                                                                        self::accept_java_script_call_object_spread(program.clone(), current.clone(), destination_12, realm_8.clone(), self::resume_java_script_object_spread(shared_constructor_field1.clone(), state_10.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitObjectEnumeration(destination_13, realm_9, state_11) => {
                                                                        let state_11 = *state_11;
                                                                        self::accept_java_script_call_object_enumeration(program.clone(), current.clone(), destination_13, realm_9.clone(), self::resume_java_script_object_enumeration_getter(shared_constructor_field1.clone(), state_11.clone(), self::JavaScriptSemanticCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitPropertyDefinitions(destination_14, realm_10, state_12) => {
                                                                        let state_12 = *state_12;
                                                                        self::accept_java_script_call_property_definitions(program.clone(), current.clone(), destination_14, realm_10.clone(), self::resume_java_script_property_definitions_call(shared_constructor_field1.clone(), state_12.clone(), self::JavaScriptSemanticCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitPropertyReflection(_, _, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitInstanceOf(destination_15, realm_11, state_13) => {
                                                                        let state_13 = *state_13;
                                                                        self::accept_java_script_call_instance_of(program.clone(), current.clone(), destination_15, realm_11.clone(), self::resume_java_script_instance_of(shared_constructor_field1.clone(), state_13.clone(), self::JavaScriptSemanticCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitObjectIntegrity(_, _, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitObjectPrototype(_, _, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitArrayAssignment(destination_16, realm_12, state_14) => {
                                                                        let state_14 = *state_14;
                                                                        self::accept_java_script_call_array_assignment(program.clone(), current.clone(), destination_16, realm_12.clone(), self::resume_java_script_array_assignment(shared_constructor_field1.clone(), state_14.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitKey(destination_17, realm_13, state_15) => {
                                                                        let state_15 = *state_15;
                                                                        self::accept_java_script_call_key(program.clone(), current.clone(), destination_17, realm_13.clone(), self::resume_java_script_property_key_conversion(shared_constructor_field1.clone(), state_15.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitDefinition(destination_18, realm_14, state_16) => {
                                                                        let state_16 = *state_16;
                                                                        self::accept_java_script_call_definition(program.clone(), current.clone(), destination_18, realm_14.clone(), self::resume_java_script_property_definition(shared_constructor_field1.clone(), state_16.clone(), self::JavaScriptSemanticCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitNumber(destination_19, realm_15, state_17) => {
                                                                        let state_17 = *state_17;
                                                                        self::resume_java_script_call_number(program.clone(), current.clone(), destination_19, realm_15.clone(), state_17.clone(), value_4.clone())
                                                                    }
                                                                    self::JavaScriptCallPending::WaitText(destination_20, realm_16, state_18) => {
                                                                        let state_18 = *state_18;
                                                                        self::resume_java_script_call_text(program.clone(), current.clone(), destination_20, realm_16.clone(), state_18.clone(), value_4.clone())
                                                                    }
                                                                    self::JavaScriptCallPending::WaitArrayCreation(destination_21, realm_17, state_19, pending_construction_4, _, _) => {
                                                                        let state_19 = *state_19;
                                                                        {
                                                                            let construction_4 = ((pending_construction_4.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone();
                                                                            if !(match (construction_4.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                {
                                                                                    let directory_4 = match program.clone() {
                                                                                        self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value35989, _) => {
                                                                                            ((source_ml_source2_data_property_value35989.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                                        }
                                                                                    };
                                                                                    if match (directory_4.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                                                    } else {
                                                                                        self::accept_java_script_call_array_construction(program.clone(), current.clone(), self::resume_java_script_construction_call(program.clone(), shared_constructor_field1.clone(), directory_4.clone().expect("a proven present value"), construction_4.clone().expect("a proven present value"), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                self::accept_java_script_call_array_creation(program.clone(), current.clone(), destination_21, realm_17.clone(), self::resume_java_script_array_creation_call(shared_constructor_field1.clone(), state_19.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                            }
                                                                        }
                                                                    }
                                                                    self::JavaScriptCallPending::WaitConstruction(destination_22, realm_18, state_20, _) => {
                                                                        let state_20 = *state_20;
                                                                        {
                                                                            let directory_5 = match program.clone() {
                                                                                self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value35993, _) => {
                                                                                    ((source_ml_source2_data_property_value35993.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                                }
                                                                            };
                                                                            if match (directory_5.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                                            } else {
                                                                                self::accept_java_script_call_construction(program.clone(), current.clone(), destination_22, realm_18.clone(), self::resume_java_script_construction_call(program.clone(), shared_constructor_field1.clone(), directory_5.clone().expect("a proven present value"), state_20.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))), None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>)
                                                                            }
                                                                        }
                                                                    }
                                                                    self::JavaScriptCallPending::WaitFunctionBind(destination_23, realm_19, state_21) => {
                                                                        let state_21 = *state_21;
                                                                        self::accept_java_script_call_function_bind(program.clone(), current.clone(), destination_23, realm_19.clone(), self::resume_java_script_function_bind_call(shared_constructor_field1.clone(), state_21.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitFunctionText(_, _, _, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitConstructorCheck(_, _, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitObjectFromEntries(destination_24, realm_20, state_22, _) => {
                                                                        let state_22 = *state_22;
                                                                        self::accept_java_script_call_object_from_entries(program.clone(), current.clone(), destination_24, realm_20.clone(), self::resume_java_script_object_from_entries_call(shared_constructor_field1.clone(), state_22.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value_4.clone()))))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitPrimitiveWrapper(_, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitSymbolAllocation(_, _, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitSymbolDescription(_, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitFunctionNameSymbol(_, _, _, _, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitSourceError(_, _, _, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitErrorValue(_, _, _, _, _, _, _, _, _, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitErrorIntrinsic(_) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitHostCapability(_, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                    self::JavaScriptCallPending::WaitError(_, _, _) => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 32, 114, 101, 116, 117, 114, 110, 105, 110, 103, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1508::MlSource1TemporaryFinishJavaScriptCallReturnJoin1508Return(source_ml_source1_temporary_result1508) => {
                                            let source_ml_source1_temporary_result1508 = *source_ml_source1_temporary_result1508;
                                            source_ml_source1_temporary_result1508.clone()
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryFinishJavaScriptCallReturnJoin1507::MlSource1TemporaryFinishJavaScriptCallReturnJoin1507Return(source_ml_source1_temporary_result1509_2) => {
                                let source_ml_source1_temporary_result1509_2 = *source_ml_source1_temporary_result1509_2;
                                source_ml_source1_temporary_result1509_2.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}
