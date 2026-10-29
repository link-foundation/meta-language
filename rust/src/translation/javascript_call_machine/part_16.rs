// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptNumberCallState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::JavaScriptTextCallState;
use super::MlSource1TemporaryPushJavaScriptCallLoop156Result;
use super::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151;
use super::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin152;
use super::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin153;
use super::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Result;
use super::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop149Result;
use super::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop150Result;
use super::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result;
use super::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin143;
use super::MlSource1TemporaryThrowJavaScriptCallValueLoop154Result;
use super::create_java_script_call_frame;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_program_error;
use super::read_java_script_number_call_source_operation;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::read_java_script_source_exception_class;
use super::read_java_script_text_protocol_operation_error;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::source_ml_source1_temporary_push_java_script_call_loop156;
use super::source_ml_source1_temporary_read_java_script_call_frame_error_loop148;
use super::source_ml_source1_temporary_read_java_script_call_frame_error_loop149;
use super::source_ml_source1_temporary_read_java_script_call_frame_error_loop150;
use super::source_ml_source1_temporary_read_java_script_call_machine_error_loop144;
use super::source_ml_source1_temporary_throw_java_script_call_value_loop154;
use super::test_java_script_call_coordinate;
use super::test_java_script_semantic_same_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_text_call_source_operation(fn_: self::JavaScriptCallFunction, frame: self::JavaScriptCallFrame, destination: f64, realm: self::ml::Big, exotic_key: self::JavaScriptSemanticValue, state: self::JavaScriptTextCallState) -> self::ml_string::Text {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(_, shared_constructor_field5, shared_constructor_field6, _, _, _) => {
            match fn_.clone() {
                self::JavaScriptCallFunction::CallFunction(_, shared_constructor_field3, _, _, _, _, _, shared_constructor_field4) => {
                    if ((shared_constructor_field5 == (0f64)) || (shared_constructor_field3 != realm)) {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        {
                            let instruction = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index((shared_constructor_field5 - 1f64)));
                            if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                match instruction.clone().expect("a proven present value") {
                                    self::JavaScriptCallInstruction::Primitive(instruction_destination, operation, arguments) => {
                                        if ((instruction_destination != destination) || (((arguments.len() as f64)) != (1f64))) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            if ((self::read_java_script_text_protocol_operation_error(&(operation))) != (self::ml_string::Text::from_units(vec![]))) {
                                                self::ml_string::Text::from_units(vec![])
                                            } else {
                                                {
                                                    let input = (self::ml_array::optional_at(&shared_constructor_field6, self::ml_array::number_index((self::ml_array::optional_at(&arguments, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| (-1f64))))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                    match input.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            {
                                                                let source_ml_source2_s1493 = match state.clone() {
                                                                    self::JavaScriptTextCallState::TextProtocolState(state_state) => {
                                                                        let state_state = *state_state;
                                                                        match state_state.clone() {
                                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, _, _) => {
                                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                                let shared_constructor_field2 = *shared_constructor_field2;
                                                                                if (!(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), input.clone())) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), exotic_key.clone()))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin143::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin143Return(self::ml_string::Text::from_units(vec![]))
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin143::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin143Next
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::JavaScriptTextCallState::TextPrimitiveValue(_) => {
                                                                        self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin143::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin143Next
                                                                    }
                                                                };
                                                                match source_ml_source2_s1493.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin143::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin143Next => {
                                                                        operation.clone()
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin143::MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin143Return(source_ml_source1_temporary_result143) => {
                                                                        source_ml_source1_temporary_result143.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        self::ml_string::Text::from_units(vec![])
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
pub fn read_java_script_call_machine_error(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine) -> self::ml_string::Text {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let error = self::read_java_script_call_program_error(program.clone(), shared_constructor_field1.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    if ((((shared_constructor_field2.len() as f64)) == (0f64)) || (((shared_constructor_field2.len() as f64)) > (1024f64))) {
                        self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 109, 97, 99, 104, 105, 110, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 105, 118, 101, 32, 115, 116, 97, 99, 107])
                    } else {
                        {
                            let slots = 0f64;
                            {
                                let argument_slots = 0f64;
                                {
                                    let index = 0f64;
                                    {
                                        let source_ml_source2_s1502 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop144(program.clone(), machine.clone(), slots, argument_slots, index);
                                        match source_ml_source2_s1502.clone() {
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Done => {
                                                self::ml_string::Text::from_units(vec![])
                                            }
                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop144Return(source_ml_source1_temporary_result144) => {
                                                source_ml_source1_temporary_result144.clone()
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
pub fn test_java_script_call_pending_destination(pending: self::JavaScriptCallPending, registers: f64) -> bool {
    match pending.clone() {
        self::JavaScriptCallPending::WaitCall(destination) => {
            self::test_java_script_call_coordinate(destination, registers)
        }
        self::JavaScriptCallPending::WaitGet(destination_2) => {
            self::test_java_script_call_coordinate(destination_2, registers)
        }
        self::JavaScriptCallPending::WaitSet(destination_3) => {
            self::test_java_script_call_coordinate(destination_3, registers)
        }
        self::JavaScriptCallPending::WaitKey(destination_4, _, _) => {
            self::test_java_script_call_coordinate(destination_4, registers)
        }
        self::JavaScriptCallPending::WaitDefinition(destination_5, _, _) => {
            self::test_java_script_call_coordinate(destination_5, registers)
        }
        self::JavaScriptCallPending::WaitNumber(destination_6, _, _) => {
            self::test_java_script_call_coordinate(destination_6, registers)
        }
        self::JavaScriptCallPending::WaitText(destination_7, _, _) => {
            self::test_java_script_call_coordinate(destination_7, registers)
        }
        _ => {
            true
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_frame_error(program: self::JavaScriptCallProgram, heap: self::JavaScriptOrdinaryHeap, frame: self::JavaScriptCallFrame, fn_: self::JavaScriptCallFunction, suspended: bool) -> self::ml_string::Text {
    match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(_, shared_constructor_field8, _, _, _, shared_constructor_field9, _, shared_constructor_field10) => {
            match frame.clone() {
                self::JavaScriptCallFrame::CallFrame(_, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7) => {
                    let shared_constructor_field7 = *shared_constructor_field7;
                    match program.clone() {
                        self::JavaScriptCallProgram::CallProgram(_, shared_constructor_field1, shared_constructor_field2) => {
                            let shared_constructor_field2 = *shared_constructor_field2;
                            {
                                let source_ml_source1_temporary_values145 = shared_constructor_field4.clone();
                                {
                                    let source_ml_source1_temporary_index145 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s1519 = self::source_ml_source1_temporary_read_java_script_call_frame_error_loop148(heap.clone(), source_ml_source1_temporary_values145, source_ml_source1_temporary_index145.clone());
                                        match source_ml_source2_s1519.clone() {
                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Done => {
                                                {
                                                    let source_ml_source1_temporary_values146 = shared_constructor_field5;
                                                    {
                                                        let source_ml_source1_temporary_index146 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s1521 = self::source_ml_source1_temporary_read_java_script_call_frame_error_loop149(heap.clone(), source_ml_source1_temporary_values146, source_ml_source1_temporary_index146.clone());
                                                            match source_ml_source2_s1521.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop149Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop149Done => {
                                                                    {
                                                                        let source_ml_source1_temporary_values147 = shared_constructor_field6;
                                                                        {
                                                                            let source_ml_source1_temporary_index147 = self::ml::Big::from_i128(0);
                                                                            {
                                                                                let source_ml_source2_s1523 = self::source_ml_source1_temporary_read_java_script_call_frame_error_loop150(fn_.clone(), source_ml_source1_temporary_values147, source_ml_source1_temporary_index147.clone());
                                                                                match source_ml_source2_s1523.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop150Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop150Done => {
                                                                                        {
                                                                                            let pending = shared_constructor_field7.clone();
                                                                                            if !(self::test_java_script_call_pending_destination(pending.clone(), shared_constructor_field9)) {
                                                                                                self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114])
                                                                                            } else {
                                                                                                {
                                                                                                    let source_ml_source2_s1629 = match pending.clone() {
                                                                                                        self::JavaScriptCallPending::CallIdle => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Next
                                                                                                        }
                                                                                                        self::JavaScriptCallPending::WaitCall(_) => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Next
                                                                                                        }
                                                                                                        self::JavaScriptCallPending::WaitGet(_) => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Next
                                                                                                        }
                                                                                                        self::JavaScriptCallPending::WaitSet(_) => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Next
                                                                                                        }
                                                                                                        self::JavaScriptCallPending::WaitKey(_, realm, _) => {
                                                                                                            if !(self::ml_array::includes(&shared_constructor_field1, &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                            } else {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Next
                                                                                                            }
                                                                                                        }
                                                                                                        self::JavaScriptCallPending::WaitDefinition(_, realm_2, _) => {
                                                                                                            if !(self::ml_array::includes(&shared_constructor_field1, &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                            } else {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Next
                                                                                                            }
                                                                                                        }
                                                                                                        self::JavaScriptCallPending::WaitNumber(destination, realm_3, state) => {
                                                                                                            let state = *state;
                                                                                                            if !(self::ml_array::includes(&shared_constructor_field1, &realm_3.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let state_error = self::read_java_script_number_call_state_error(heap.clone(), state.clone());
                                                                                                                    if (state_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(state_error.clone())
                                                                                                                    } else {
                                                                                                                        if ((self::read_java_script_number_call_source_operation(fn_.clone(), frame.clone(), destination, realm_3.clone(), shared_constructor_field2.clone(), state.clone())) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let source_ml_source2_s1649 = if suspended {
                                                                                                                                    match state.clone() {
                                                                                                                                        self::JavaScriptNumberCallState::NumberProtocolState(_) => {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin152::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin152Next
                                                                                                                                        }
                                                                                                                                        self::JavaScriptNumberCallState::NumberPrimitiveValue(_) => {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin152::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin152Return(self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                } else {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin152::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin152Next
                                                                                                                                };
                                                                                                                                match source_ml_source2_s1649.clone() {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin152::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin152Next => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Next
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin152::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin152Return(source_ml_source1_temporary_result152) => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(source_ml_source1_temporary_result152.clone())
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::JavaScriptCallPending::WaitText(destination_2, realm_4, state_2) => {
                                                                                                            let state_2 = *state_2;
                                                                                                            if !(self::ml_array::includes(&shared_constructor_field1, &realm_4.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let state_error_2 = self::read_java_script_text_call_state_error(heap.clone(), state_2.clone());
                                                                                                                    if (state_error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(state_error_2.clone())
                                                                                                                    } else {
                                                                                                                        if ((self::read_java_script_text_call_source_operation(fn_.clone(), frame.clone(), destination_2, realm_4.clone(), shared_constructor_field2.clone(), state_2.clone())) == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let source_ml_source2_s1658 = if suspended {
                                                                                                                                    match state_2.clone() {
                                                                                                                                        self::JavaScriptTextCallState::TextProtocolState(_) => {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin153::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin153Next
                                                                                                                                        }
                                                                                                                                        self::JavaScriptTextCallState::TextPrimitiveValue(_) => {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin153::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin153Return(self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 115, 116, 114, 105, 110, 103, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 97, 110, 110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                } else {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin153::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin153Next
                                                                                                                                };
                                                                                                                                match source_ml_source2_s1658.clone() {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin153::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin153Next => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Next
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin153::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin153Return(source_ml_source1_temporary_result153) => {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(source_ml_source1_temporary_result153.clone())
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::JavaScriptCallPending::WaitSourceError(realm_5, error_class, pending_message, message_present) => {
                                                                                                            let pending_message = *pending_message;
                                                                                                            if (!(self::ml_array::includes(&shared_constructor_field1, &realm_5.clone(), 0f64, false, |a, b| a == b)) || (shared_constructor_field8 != realm_5)) {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109]))
                                                                                                            } else {
                                                                                                                if (match pending_message.clone() {
                                                                                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                                                                                        false
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        true
                                                                                                                    }
                                                                                                                } || ((self::read_java_script_ordinary_value_error(heap.clone(), pending_message.clone())) != (self::ml_string::Text::from_units(vec![])))) {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 120, 97, 99, 116, 32, 85, 84, 70, 49, 54, 32, 109, 101, 115, 115, 97, 103, 101, 32, 97, 110, 100, 32, 66, 111, 111, 108, 101, 97, 110, 32, 112, 114, 101, 115, 101, 110, 99, 101]))
                                                                                                                } else {
                                                                                                                    {
                                                                                                                        let instruction = if (shared_constructor_field3 > (0f64)) {
                                                                                                                            self::ml_array::optional_at(&shared_constructor_field10, self::ml_array::number_index((shared_constructor_field3 - 1f64)))
                                                                                                                        } else {
                                                                                                                            None::<self::JavaScriptCallInstruction>
                                                                                                                        };
                                                                                                                        if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                        } else {
                                                                                                                            match instruction.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptCallInstruction::Constant(_, _) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::Copy(_, _) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::Argument(_, _) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::Jump(_) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::Branch(_, _, _) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::Return(_) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::Throw(_) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::PushHandler(_, _) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::PopHandler => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::RequireWrite(_) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::Primitive(_, operation, arguments) => {
                                                                                                                                    if ((((self::read_java_script_source_exception_class(&(operation))) != error_class) || (error_class == (self::ml_string::Text::from_units(vec![])))) || (((arguments.len() as f64)) != (2f64))) {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                    } else {
                                                                                                                                        {
                                                                                                                                            let message = (self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index((self::ml_array::optional_at(&arguments, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| (-1f64))))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                                                                                            {
                                                                                                                                                let present = (self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index((self::ml_array::optional_at(&arguments, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| (-1f64))))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                                                                                                if !(self::test_java_script_semantic_same_value(message.clone(), pending_message.clone())) {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                                } else {
                                                                                                                                                    match present.clone() {
                                                                                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                                        }
                                                                                                                                                        self::JavaScriptSemanticValue::Null => {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                                        }
                                                                                                                                                        self::JavaScriptSemanticValue::Boolean(value) => {
                                                                                                                                                            if (value != message_present) {
                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                                            } else {
                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Next
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                                        }
                                                                                                                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                                        }
                                                                                                                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                                        }
                                                                                                                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                                        }
                                                                                                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                                self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::JavaScriptCallPending::WaitError(realm_6, _, _) => {
                                                                                                            if !(self::ml_array::includes(&shared_constructor_field1, &realm_6.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                                                                            } else {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Next
                                                                                                            }
                                                                                                        }
                                                                                                    };
                                                                                                    match source_ml_source2_s1629.clone() {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Next => {
                                                                                                            if (suspended && ((match pending.clone() {
                                                                                                                self::JavaScriptCallPending::CallIdle => {
                                                                                                                    true
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    false
                                                                                                                }
                                                                                                            } || match pending.clone() {
                                                                                                                self::JavaScriptCallPending::WaitError(_, _, _) => {
                                                                                                                    true
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    false
                                                                                                                }
                                                                                                            }) || match pending.clone() {
                                                                                                                self::JavaScriptCallPending::WaitSourceError(_, _, _, _) => {
                                                                                                                    true
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    false
                                                                                                                }
                                                                                                            })) {
                                                                                                                self::ml_string::Text::from_units(vec![97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108])
                                                                                                            } else {
                                                                                                                self::ml_string::Text::from_units(vec![])
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(source_ml_source1_temporary_result151) => {
                                                                                                            source_ml_source1_temporary_result151.clone()
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop150Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop150Return(source_ml_source1_temporary_result153_2) => {
                                                                                        source_ml_source1_temporary_result153_2.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop149Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop149Return(source_ml_source1_temporary_result153_3) => {
                                                                    source_ml_source1_temporary_result153_3.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Return(source_ml_source1_temporary_result153_4) => {
                                                source_ml_source1_temporary_result153_4.clone()
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
pub fn read_java_script_number_call_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptNumberCallState) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptNumberCallState::NumberProtocolState(state_state) => {
            let state_state = *state_state;
            if ((match state_state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1800, _) => {
                    source_ml_source2_data_property_value1800.clone()
                }
            }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 104, 105, 110, 116])
            } else {
                self::read_java_script_primitive_conversion_state_error(heap.clone(), state_state.clone())
            }
        }
        self::JavaScriptNumberCallState::NumberPrimitiveValue(value) => {
            let value = *value;
            if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101])
            } else {
                if match value.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        true
                    }
                    _ => {
                        false
                    }
                } {
                    self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_text_call_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptTextCallState) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptTextCallState::TextProtocolState(state_state) => {
            let state_state = *state_state;
            if ((match state_state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1809, _) => {
                    source_ml_source2_data_property_value1809.clone()
                }
            }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                self::ml_string::Text::from_units(vec![84, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 110, 103, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 104, 105, 110, 116])
            } else {
                self::read_java_script_primitive_conversion_state_error(heap.clone(), state_state.clone())
            }
        }
        self::JavaScriptTextCallState::TextPrimitiveValue(value) => {
            let value = *value;
            if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101])
            } else {
                if match value.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        true
                    }
                    _ => {
                        false
                    }
                } {
                    self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn throw_java_script_call_value(machine: self::JavaScriptCallMachine, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frames = shared_constructor_field2;
                {
                    let source_ml_source2_s1818 = self::source_ml_source1_temporary_throw_java_script_call_value_loop154(machine.clone(), value.clone(), frames);
                    match source_ml_source2_s1818.clone() {
                        self::MlSource1TemporaryThrowJavaScriptCallValueLoop154Result::MlSource1TemporaryThrowJavaScriptCallValueLoop154Done => {
                            self::JavaScriptCallMachineTransition::MachineThrow(Box::new(shared_constructor_field1.clone()), Box::new(value.clone()))
                        }
                        self::MlSource1TemporaryThrowJavaScriptCallValueLoop154Result::MlSource1TemporaryThrowJavaScriptCallValueLoop154Return(source_ml_source1_temporary_result154) => {
                            let source_ml_source1_temporary_result154 = *source_ml_source1_temporary_result154;
                            source_ml_source1_temporary_result154.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_java_script_call_error(machine: self::JavaScriptCallMachine, realm: self::ml::Big, error_class: &self::ml_string::Text, reason: &self::ml_string::Text) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(((shared_constructor_field2.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    {
                        let next = self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitError(realm.clone(), error_class.clone(), reason.clone())));
                        self::JavaScriptCallMachineTransition::MachineAllocateError(Box::new(next.clone()), realm.clone(), error_class.clone(), reason.clone())
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn push_java_script_call(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, callee: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue, arguments_: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field2, shared_constructor_field3) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, _) => {
                    {
                        let frame = self::ml_array::optional_at(&shared_constructor_field3, self::ml_array::number_index(((shared_constructor_field3.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field2.clone()), self::ml_string::Text::from_units(vec![97, 32, 110, 101, 115, 116, 101, 100, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 97, 114, 101, 110, 116]))
                        } else {
                            {
                                let owner = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value1829, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1829.clone()
                                    }
                                })));
                                if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field2.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    match callee.clone() {
                                        self::JavaScriptSemanticValue::Undefined => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1836, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1836.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Null => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1837, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1837.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1838, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1838.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Number(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1839, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1839.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Integer(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1840, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1840.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Text(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1841, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1841.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                            self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1842, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1842.clone()
                                                }
                                            }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                        }
                                        self::JavaScriptSemanticValue::Object(identity, callable) => {
                                            if !callable {
                                                self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1843, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value1843.clone()
                                                    }
                                                }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])))
                                            } else {
                                                {
                                                    let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
                                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field2.clone()), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))
                                                    } else {
                                                        {
                                                            let slots = match fn_.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value1846, _, _) => {
                                                                    source_ml_source2_data_property_value1846
                                                                }
                                                            };
                                                            {
                                                                let argument_slots = (arguments_.len() as f64);
                                                                {
                                                                    let source_ml_source1_temporary_values155 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source1_temporary_index155 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source2_s1848 = self::source_ml_source1_temporary_push_java_script_call_loop156(slots, argument_slots, source_ml_source1_temporary_values155, source_ml_source1_temporary_index155.clone());
                                                                            match source_ml_source2_s1848.clone() {
                                                                                self::MlSource1TemporaryPushJavaScriptCallLoop156Result::MlSource1TemporaryPushJavaScriptCallLoop156Done(slots_2, argument_slots_2) => {
                                                                                    if (((((shared_constructor_field3.len() as f64)) >= (1024f64)) || (slots_2 > (1048576f64))) || (argument_slots_2 > (1048576f64))) {
                                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                                    } else {
                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(shared_constructor_field2.clone()), {
                                                                                            let source_ml_source2_concat_value1853 = shared_constructor_field3;
                                                                                            {
                                                                                                let source_ml_source2_concat_value1854 = vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)];
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), source_ml_source2_concat_value1853), source_ml_source2_concat_value1854)
                                                                                            }
                                                                                        })))
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
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_key(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptPropertyKeyConversionTransition) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(((shared_constructor_field2.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state.clone()))))))
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(state_2, callee, receiver, arguments) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state_2.clone())))), callee.clone(), receiver.clone(), arguments)
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(key) => {
                            let key = *key;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, key.clone()))))
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(value) => {
                            let value = *value;
                            self::throw_java_script_call_value(machine.clone(), value.clone())
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(error_class, reason) => {
                            self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), reason_2.clone())
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_definition(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptPropertyDefinitionTransition) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(((shared_constructor_field2.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state.clone()))))))
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionCall(state_2, callee, receiver, arguments) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state_2.clone())))), callee.clone(), receiver.clone(), arguments)
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(heap, value, _) => {
                            let heap = *heap;
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), shared_constructor_field2, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(machine.clone(), value_2.clone())
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(error_class, reason) => {
                            self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                        }
                        self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), reason_2.clone())
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_number(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, destination: f64, realm: self::ml::Big, transition: self::JavaScriptScalarConversionTransition) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(((shared_constructor_field2.len() as f64) - 1f64)));
                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                } else {
                    match transition.clone() {
                        self::JavaScriptScalarConversionTransition::ScalarContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state.clone()))))))))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarCall(state_2, callee, receiver, arguments) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state_2.clone())))))), callee.clone(), receiver.clone(), arguments)
                        }
                        self::JavaScriptScalarConversionTransition::ScalarCompleted(value) => {
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(machine.clone(), value_2.clone())
                        }
                        self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class, reason) => {
                            self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), reason_2.clone())
                        }
                        self::JavaScriptScalarConversionTransition::ScalarResourceLimit(value_3, reason_3) => {
                            let value_3 = *value_3;
                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberPrimitiveValue(Box::new(value_3.clone()))))))), reason_3.clone())
                        }
                    }
                }
            }
        }
    }
}
