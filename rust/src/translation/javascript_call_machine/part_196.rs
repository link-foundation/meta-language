// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallProgram;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::JavaScriptTextCallState;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result;
use super::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_program_error;
use super::read_java_script_text_protocol_operation_error;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop1475;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_text_call_source_operation(
    fn_: self::JavaScriptCallFunction,
    frame: self::JavaScriptCallFrame,
    destination: f64,
    realm: self::ml::Big,
    exotic_key: self::JavaScriptSemanticValue,
    state: self::JavaScriptTextCallState,
) -> self::ml_string::Text {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            _,
            shared_constructor_field5,
            shared_constructor_field6,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(
                _,
                shared_constructor_field3,
                _,
                _,
                _,
                _,
                _,
                shared_constructor_field4,
                _,
                _,
            ) => {
                if ((shared_constructor_field5 == (0f64)) || (shared_constructor_field3 != realm)) {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    {
                        let instruction = {
                            let source_ml_source2_array_read_value34718 =
                                shared_constructor_field4.clone();
                            {
                                let source_ml_source2_array_read_index34719 =
                                    (shared_constructor_field5 - 1f64);
                                {
                                    let ml_reference_array2250 =
                                        source_ml_source2_array_read_value34718.clone();
                                    let ml_reference_position2250 =
                                        source_ml_source2_array_read_index34719;
                                    let ml_reference_projection2250 = self::ml_array::optional_at(
                                        &*ml_reference_array2250.borrow(),
                                        self::ml_array::number_index(ml_reference_position2250),
                                    );
                                    ml_reference_projection2250
                                }
                            }
                        };
                        if match (instruction.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            match instruction.clone().expect("a proven present value") {
                                self::JavaScriptCallInstruction::Primitive(
                                    instruction_destination,
                                    operation,
                                    arguments,
                                ) => {
                                    if ((instruction_destination != destination)
                                        || (({
                                            let ml_reference_array2251 = arguments.clone();
                                            let ml_reference_projection2251 =
                                                ml_reference_array2251.borrow().len() as f64;
                                            ml_reference_projection2251
                                        }) != (1f64)))
                                    {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        if ((self::read_java_script_text_protocol_operation_error(
                                            &(operation),
                                        )) != (self::ml_string::Text::from_units(vec![])))
                                        {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            {
                                                let input = ({
                                                        let source_ml_source2_array_read_value34723 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_array_read_index34724 = ({
                                                                let source_ml_source2_array_read_index34722 = 0f64;
                                                                { let ml_reference_array2252 = arguments.clone(); let ml_reference_position2252 = source_ml_source2_array_read_index34722; let ml_reference_projection2252 = self::ml_array::optional_at(&*ml_reference_array2252.borrow(), self::ml_array::number_index(ml_reference_position2252)); ml_reference_projection2252 }
                                                            }).clone().unwrap_or_else(|| (-1f64));
                                                            { let ml_reference_array2253 = source_ml_source2_array_read_value34723.clone(); let ml_reference_position2253 = source_ml_source2_array_read_index34724; let ml_reference_projection2253 = self::ml_array::optional_at(&*ml_reference_array2253.borrow(), self::ml_array::number_index(ml_reference_position2253)); ml_reference_projection2253 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                match input.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        let source_ml_source2_s34730 = match state.clone() {
                                                                    self::JavaScriptTextCallState::TextProtocolState(state_state) => {
                                                                        let state_state = *state_state;
                                                                        match state_state.clone() {
                                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, _, _) => {
                                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                                let shared_constructor_field2 = *shared_constructor_field2;
                                                                                if (!(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), input.clone())) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), exotic_key.clone()))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472Return(self::ml_string::Text::from_units(vec![]))
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472Next
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::JavaScriptTextCallState::TextPrimitiveValue(_) => {
                                                                        self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472Next
                                                                    }
                                                                };
                                                        match source_ml_source2_s34730.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472Next => {
                                                                        operation.clone()
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472Return(source_ml_source1_temporary_result1472) => {
                                                                        source_ml_source1_temporary_result1472.clone()
                                                                    }
                                                                }
                                                    }
                                                    _ => self::ml_string::Text::from_units(vec![]),
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => self::ml_string::Text::from_units(vec![]),
                            }
                        }
                    }
                }
            }
        },
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_machine_error(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
) -> self::ml_string::Text {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let error = self::read_java_script_call_program_error(
                    program.clone(),
                    shared_constructor_field1.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    if ((({
                        let ml_reference_array2254 = shared_constructor_field2.clone();
                        let ml_reference_projection2254 =
                            ml_reference_array2254.borrow().len() as f64;
                        ml_reference_projection2254
                    }) == (0f64))
                        || (({
                            let ml_reference_array2255 = shared_constructor_field2.clone();
                            let ml_reference_projection2255 =
                                ml_reference_array2255.borrow().len() as f64;
                            ml_reference_projection2255
                        }) > (1024f64)))
                    {
                        self::ml_string::Text::from_units(vec![
                            97, 32, 99, 97, 108, 108, 32, 109, 97, 99, 104, 105, 110, 101, 32, 114,
                            101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109,
                            112, 108, 101, 116, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97,
                            99, 116, 105, 118, 101, 32, 115, 116, 97, 99, 107,
                        ])
                    } else {
                        {
                            let slots = 0f64;
                            {
                                let argument_slots = 0f64;
                                {
                                    let index = 0f64;
                                    {
                                        let source_ml_source2_s34739 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop1475(program.clone(), machine.clone(), slots, argument_slots, index);
                                        match source_ml_source2_s34739.clone() {
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Done => {
                                                self::ml_string::Text::from_units(vec![])
                                            }
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Return(source_ml_source1_temporary_result1475) => {
                                                source_ml_source1_temporary_result1475.clone()
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
