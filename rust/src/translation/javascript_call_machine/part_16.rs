// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Result;
use super::accept_java_script_call_definition;
use super::accept_java_script_call_key;
use super::accept_java_script_call_number;
use super::accept_java_script_call_text;
use super::begin_java_script_property_definition;
use super::begin_java_script_property_key_conversion;
use super::begin_java_script_scalar_protocol;
use super::convert_java_script_semantic_boolean;
use super::create_java_script_call_frame;
use super::execute_java_script_call_primitive;
use super::execute_java_script_ordinary_call_property;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_call_machine_error;
use super::read_java_script_call_program_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::return_java_script_call_value;
use super::source_ml_source1_temporary_begin_java_script_call_machine_loop155;
use super::step_java_script_call_number;
use super::step_java_script_call_text;
use super::step_java_script_property_definition;
use super::step_java_script_property_key_conversion;
use super::test_java_script_call_strict_equal;
use super::test_java_script_semantic_same_value;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_primitive_instruction(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, frame: self::JavaScriptCallFrame, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
            let shared_constructor_field10 = *shared_constructor_field10;
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(shared_constructor_field3, shared_constructor_field4) => {
                    let shared_constructor_field3 = *shared_constructor_field3;
                    match program.clone() {
                        self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, shared_constructor_field2) => {
                            let shared_constructor_field2 = *shared_constructor_field2;
                            match instruction.clone() {
                                self::JavaScriptCallInstruction::Constant(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Copy(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Argument(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Jump(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Branch(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Return(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Throw(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PushHandler(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PopHandler => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::RequireWrite(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Primitive(destination, operation, arguments) => {
                                    {
                                        let arguments_ = { let ml_values6 = arguments; let ml_callback6 = { let frame = frame.clone(); std::rc::Rc::new(move |index: f64| -> self::JavaScriptSemanticValue { (self::ml_array::optional_at((match &frame { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value1982, _, _, _) => source_ml_source2_data_property_value1982.as_slice() }), self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values6.iter().cloned().enumerate().map(|(ml_index6, ml_value6)| ml_callback6(ml_value6)).collect::<Vec<_>>() };
                                        {
                                            let input = (self::ml_array::optional_at(&arguments_, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                            if (((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) && match input.clone() {
                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            }) {
                                                {
                                                    let owner = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), shared_constructor_field5.clone())));
                                                    if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                    } else {
                                                        {
                                                            let current = self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                let source_ml_source2_nominal_field1997 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field1999 = shared_constructor_field6;
                                                                    {
                                                                        let source_ml_source2_nominal_field2001 = shared_constructor_field7;
                                                                        {
                                                                            let source_ml_source2_nominal_field2003 = shared_constructor_field8;
                                                                            {
                                                                                let source_ml_source2_nominal_field2005 = shared_constructor_field9;
                                                                                {
                                                                                    let source_ml_source2_nominal_field2007 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_override2009 = (shared_constructor_field6 + 1f64);
                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1997.clone(), source_ml_source2_nominal_override2009, source_ml_source2_nominal_field2001, source_ml_source2_nominal_field2003, source_ml_source2_nominal_field2005, Box::new(source_ml_source2_nominal_field2007.clone()))
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            });
                                                            self::accept_java_script_call_number(program.clone(), current.clone(), destination, match owner.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value2010, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value2010.clone()
                                                                }
                                                            }, self::begin_java_script_scalar_protocol(shared_constructor_field3.clone(), input.clone(), shared_constructor_field2.clone(), &(operation)))
                                                        }
                                                    }
                                                }
                                            } else {
                                                if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) && match input.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        true
                                                    }
                                                    _ => {
                                                        false
                                                    }
                                                }) {
                                                    {
                                                        let owner_2 = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), shared_constructor_field5.clone())));
                                                        if match (owner_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                        } else {
                                                            {
                                                                let current_2 = self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                    let source_ml_source2_nominal_field2027 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field2029 = shared_constructor_field6;
                                                                        {
                                                                            let source_ml_source2_nominal_field2031 = shared_constructor_field7;
                                                                            {
                                                                                let source_ml_source2_nominal_field2033 = shared_constructor_field8;
                                                                                {
                                                                                    let source_ml_source2_nominal_field2035 = shared_constructor_field9;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field2037 = shared_constructor_field10.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override2039 = (shared_constructor_field6 + 1f64);
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2027.clone(), source_ml_source2_nominal_override2039, source_ml_source2_nominal_field2031, source_ml_source2_nominal_field2033, source_ml_source2_nominal_field2035, Box::new(source_ml_source2_nominal_field2037.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                });
                                                                self::accept_java_script_call_text(program.clone(), current_2.clone(), destination, match owner_2.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value2040, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value2040.clone()
                                                                    }
                                                                }, self::begin_java_script_scalar_protocol(shared_constructor_field3.clone(), input.clone(), shared_constructor_field2.clone(), &(operation)))
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let result = self::execute_java_script_call_primitive(&(operation), arguments_);
                                                        match result.clone() {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                                                                let value = *value;
                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register({
                                                                    let source_ml_source2_nominal_field2047 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field2049 = shared_constructor_field6;
                                                                        {
                                                                            let source_ml_source2_nominal_field2051 = shared_constructor_field7;
                                                                            {
                                                                                let source_ml_source2_nominal_field2053 = shared_constructor_field8;
                                                                                {
                                                                                    let source_ml_source2_nominal_field2055 = shared_constructor_field9;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field2057 = shared_constructor_field10.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override2059 = (shared_constructor_field6 + 1f64);
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2047.clone(), source_ml_source2_nominal_override2059, source_ml_source2_nominal_field2051, source_ml_source2_nominal_field2053, source_ml_source2_nominal_field2055, Box::new(source_ml_source2_nominal_field2057.clone()))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }, destination, value.clone()))))
                                                            }
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(reason) => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), reason.clone())
                                                            }
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveError(error_class, reason_2) => {
                                                                {
                                                                    let owner_3 = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), shared_constructor_field5.clone())));
                                                                    if match (owner_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 102, 97, 105, 108, 117, 114, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                                    } else {
                                                                        self::request_java_script_call_error(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                            let source_ml_source2_nominal_field2068 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field2070 = shared_constructor_field6;
                                                                                {
                                                                                    let source_ml_source2_nominal_field2072 = shared_constructor_field7;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field2074 = shared_constructor_field8;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field2076 = shared_constructor_field9;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2078 = shared_constructor_field10.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override2080 = (shared_constructor_field6 + 1f64);
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2068.clone(), source_ml_source2_nominal_override2080, source_ml_source2_nominal_field2072, source_ml_source2_nominal_field2074, source_ml_source2_nominal_field2076, Box::new(source_ml_source2_nominal_field2078.clone()))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }), match owner_3.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value2081, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value2081.clone()
                                                                            }
                                                                        }, &(error_class), &(reason_2))
                                                                    }
                                                                }
                                                            }
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(reason_3) => {
                                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_3.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
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
pub fn step_java_script_call_machine(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, shared_constructor_field2) => {
                    let shared_constructor_field2 = *shared_constructor_field2;
                    {
                        let error = self::read_java_script_call_machine_error(program.clone(), machine.clone());
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), error.clone())
                        } else {
                            {
                                let frame = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index(((shared_constructor_field4.len() as f64) - 1f64)));
                                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                                } else {
                                    {
                                        let pending = match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2090) => {
                                                let source_ml_source2_data_property_value2090 = *source_ml_source2_data_property_value2090;
                                                source_ml_source2_data_property_value2090.clone()
                                            }
                                        };
                                        match pending.clone() {
                                            self::JavaScriptCallPending::CallIdle => {
                                                {
                                                    let fn_ = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2098, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value2098.clone()
                                                        }
                                                    })));
                                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                                    } else {
                                                        {
                                                            let instruction = self::ml_array::optional_at(&match fn_.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2100) => {
                                                                    source_ml_source2_data_property_value2100
                                                                }
                                                            }, self::ml_array::number_index(match frame.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2101, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value2101
                                                                }
                                                            }));
                                                            if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::return_java_script_call_value(program.clone(), machine.clone(), self::JavaScriptSemanticValue::Undefined)
                                                            } else {
                                                                {
                                                                    let next = {
                                                                        let source_ml_source2_nominal_spread2103 = frame.clone().expect("a proven present value");
                                                                        {
                                                                            let source_ml_source2_nominal_field2104 = match source_ml_source2_nominal_spread2103.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2105, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value2105.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field2106 = match source_ml_source2_nominal_spread2103.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2107, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value2107
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field2108 = match source_ml_source2_nominal_spread2103.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2109, _, _, _) => {
                                                                                            source_ml_source2_data_property_value2109
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field2110 = match source_ml_source2_nominal_spread2103.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2111, _, _) => {
                                                                                                source_ml_source2_data_property_value2111
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field2112 = match source_ml_source2_nominal_spread2103.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2113, _) => {
                                                                                                    source_ml_source2_data_property_value2113
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2114 = match source_ml_source2_nominal_spread2103.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2115) => {
                                                                                                        let source_ml_source2_data_property_value2115 = *source_ml_source2_data_property_value2115;
                                                                                                        source_ml_source2_data_property_value2115.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override2116 = (match frame.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2102, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value2102
                                                                                                        }
                                                                                                    } + 1f64);
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2104.clone(), source_ml_source2_nominal_override2116, source_ml_source2_nominal_field2108, source_ml_source2_nominal_field2110, source_ml_source2_nominal_field2112, Box::new(source_ml_source2_nominal_field2114.clone()))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    {
                                                                        let current = self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4.clone(), next.clone());
                                                                        {
                                                                            let registers = match frame.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2119, _, _, _) => {
                                                                                    source_ml_source2_data_property_value2119
                                                                                }
                                                                            };
                                                                            match instruction.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallInstruction::Constant(destination, value) => {
                                                                                    let value = *value;
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination, value.clone()))))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Copy(destination_2, source) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination_2, (self::ml_array::optional_at(&registers, self::ml_array::number_index(source))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Argument(destination_3, index) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination_3, (self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2139, _, _) => {
                                                                                            source_ml_source2_data_property_value2139
                                                                                        }
                                                                                    }, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Jump(target) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                        let source_ml_source2_nominal_field2143 = match next.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2144, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value2144.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field2145 = match next.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2146, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value2146
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2147 = match next.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2148, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value2148
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2149 = match next.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2150, _, _) => {
                                                                                                            source_ml_source2_data_property_value2150
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2151 = match next.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2152, _) => {
                                                                                                                source_ml_source2_data_property_value2152
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2153 = match next.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2154) => {
                                                                                                                    let source_ml_source2_data_property_value2154 = *source_ml_source2_data_property_value2154;
                                                                                                                    source_ml_source2_data_property_value2154.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2143.clone(), target, source_ml_source2_nominal_field2147, source_ml_source2_nominal_field2149, source_ml_source2_nominal_field2151, Box::new(source_ml_source2_nominal_field2153.clone()))
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    })))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Branch(condition, yes, no) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                        let source_ml_source2_nominal_field2159 = match next.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2160, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value2160.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field2161 = match next.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2162, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value2162
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2163 = match next.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2164, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value2164
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2165 = match next.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2166, _, _) => {
                                                                                                            source_ml_source2_data_property_value2166
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2167 = match next.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2168, _) => {
                                                                                                                source_ml_source2_data_property_value2168
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2169 = match next.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2170) => {
                                                                                                                    let source_ml_source2_data_property_value2170 = *source_ml_source2_data_property_value2170;
                                                                                                                    source_ml_source2_data_property_value2170.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override2171 = if self::convert_java_script_semantic_boolean((self::ml_array::optional_at(&registers, self::ml_array::number_index(condition))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)) {
                                                                                                                    yes
                                                                                                                } else {
                                                                                                                    no
                                                                                                                };
                                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2159.clone(), source_ml_source2_nominal_override2171, source_ml_source2_nominal_field2163, source_ml_source2_nominal_field2165, source_ml_source2_nominal_field2167, Box::new(source_ml_source2_nominal_field2169.clone()))
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    })))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Call(destination_4, callee, receiver, arguments) => {
                                                                                    {
                                                                                        let arguments_ = { let ml_values7 = arguments; let ml_callback7 = { let registers = registers.clone(); std::rc::Rc::new(move |index_2: f64| -> self::JavaScriptSemanticValue { (self::ml_array::optional_at(&registers, self::ml_array::number_index(index_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values7.iter().cloned().enumerate().map(|(ml_index7, ml_value7)| ml_callback7(ml_value7)).collect::<Vec<_>>() };
                                                                                        self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                            let source_ml_source2_nominal_field2175 = match next.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2176, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value2176.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2177 = match next.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2178, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value2178
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2179 = match next.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2180, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value2180
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2181 = match next.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2182, _, _) => {
                                                                                                                source_ml_source2_data_property_value2182
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2183 = match next.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2184, _) => {
                                                                                                                    source_ml_source2_data_property_value2184
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field2185 = match next.clone() {
                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2186) => {
                                                                                                                        let source_ml_source2_data_property_value2186 = *source_ml_source2_data_property_value2186;
                                                                                                                        source_ml_source2_data_property_value2186.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override2187 = self::JavaScriptCallPending::WaitCall(destination_4);
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2175.clone(), source_ml_source2_nominal_field2177, source_ml_source2_nominal_field2179, source_ml_source2_nominal_field2181, source_ml_source2_nominal_field2183, Box::new(source_ml_source2_nominal_override2187.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }), (self::ml_array::optional_at(&registers, self::ml_array::number_index(callee))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), arguments_)
                                                                                    }
                                                                                }
                                                                                self::JavaScriptCallInstruction::Return(source_2) => {
                                                                                    self::return_java_script_call_value(program.clone(), current.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(source_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Throw(source_3) => {
                                                                                    self::throw_java_script_call_value(current.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(source_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                                }
                                                                                self::JavaScriptCallInstruction::PushHandler(destination_5, target_2) => {
                                                                                    if ((((match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2189, _) => {
                                                                                            source_ml_source2_data_property_value2189
                                                                                        }
                                                                                    }).len() as f64)) >= (1024f64)) {
                                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![104, 97, 110, 100, 108, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                                    } else {
                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                            let source_ml_source2_nominal_field2196 = match next.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2197, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value2197.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2198 = match next.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2199, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value2199
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2200 = match next.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2201, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value2201
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2202 = match next.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2203, _, _) => {
                                                                                                                source_ml_source2_data_property_value2203
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2204 = match next.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2205, _) => {
                                                                                                                    source_ml_source2_data_property_value2205
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field2206 = match next.clone() {
                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2207) => {
                                                                                                                        let source_ml_source2_data_property_value2207 = *source_ml_source2_data_property_value2207;
                                                                                                                        source_ml_source2_data_property_value2207.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override2208 = {
                                                                                                                        let source_ml_source2_concat_value2193 = match frame.clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2192, _) => {
                                                                                                                                source_ml_source2_data_property_value2192
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_concat_value2194 = vec![self::JavaScriptCallHandler::CallHandler(target_2, destination_5)];
                                                                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallHandler>::new(), source_ml_source2_concat_value2193), source_ml_source2_concat_value2194)
                                                                                                                        }
                                                                                                                    };
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2196.clone(), source_ml_source2_nominal_field2198, source_ml_source2_nominal_field2200, source_ml_source2_nominal_field2202, source_ml_source2_nominal_override2208, Box::new(source_ml_source2_nominal_field2206.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        })))
                                                                                    }
                                                                                }
                                                                                self::JavaScriptCallInstruction::PopHandler => {
                                                                                    if ((((match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2210, _) => {
                                                                                            source_ml_source2_data_property_value2210
                                                                                        }
                                                                                    }).len() as f64)) == (0f64)) {
                                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 104, 97, 110, 100, 108, 101, 114, 32, 112, 111, 112, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 104, 97, 110, 100, 108, 101, 114]))
                                                                                    } else {
                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                            let source_ml_source2_nominal_field2217 = match next.clone() {
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2218, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value2218.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2219 = match next.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2220, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value2220
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2221 = match next.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2222, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value2222
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2223 = match next.clone() {
                                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2224, _, _) => {
                                                                                                                source_ml_source2_data_property_value2224
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2225 = match next.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2226, _) => {
                                                                                                                    source_ml_source2_data_property_value2226
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field2227 = match next.clone() {
                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2228) => {
                                                                                                                        let source_ml_source2_data_property_value2228 = *source_ml_source2_data_property_value2228;
                                                                                                                        source_ml_source2_data_property_value2228.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override2229 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2215, _) => {
                                                                                                                            source_ml_source2_data_property_value2215
                                                                                                                        }
                                                                                                                    }, 0f64, (-1f64));
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2217.clone(), source_ml_source2_nominal_field2219, source_ml_source2_nominal_field2221, source_ml_source2_nominal_field2223, source_ml_source2_nominal_override2229, Box::new(source_ml_source2_nominal_field2227.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        })))
                                                                                    }
                                                                                }
                                                                                self::JavaScriptCallInstruction::NumberBinary(destination_6, instruction_left, instruction_right, operator) => {
                                                                                    {
                                                                                        let left = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_left))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                                        {
                                                                                            let right = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_right))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                                            match left.clone() {
                                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Number(left_value) => {
                                                                                                    match right.clone() {
                                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Null => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Number(right_value) => {
                                                                                                            if ((operator == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) || (operator == (self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])))) {
                                                                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Boolean(if (operator == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) {
                                                                                                                    (left_value < right_value)
                                                                                                                } else {
                                                                                                                    (left_value == right_value)
                                                                                                                })))))
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let value_2 = if (operator == (self::ml_string::Text::from_units(vec![97, 100, 100]))) {
                                                                                                                        (left_value + right_value)
                                                                                                                    } else {
                                                                                                                        if (operator == (self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116]))) {
                                                                                                                            (left_value - right_value)
                                                                                                                        } else {
                                                                                                                            if (operator == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121]))) {
                                                                                                                                (left_value * right_value)
                                                                                                                            } else {
                                                                                                                                if (operator == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101]))) {
                                                                                                                                    (left_value / right_value)
                                                                                                                                } else {
                                                                                                                                    if (operator == (self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114]))) {
                                                                                                                                        (left_value % right_value)
                                                                                                                                    } else {
                                                                                                                                        self::ml_big_integer_conversion::power_java_script_number(left_value,right_value)
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    };
                                                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Number(value_2)))))
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Text(_) => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::JavaScriptCallInstruction::SameValue(destination_7, left_2, right_2) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination_7, self::JavaScriptSemanticValue::Boolean(self::test_java_script_semantic_same_value((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                                }
                                                                                self::JavaScriptCallInstruction::StrictEqual(destination_8, left_3, right_3) => {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register(next.clone(), destination_8, self::JavaScriptSemanticValue::Boolean(self::test_java_script_call_strict_equal((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                                }
                                                                                self::JavaScriptCallInstruction::RequireWrite(_) => {
                                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                                }
                                                                                self::JavaScriptCallInstruction::Primitive(_, _, _) => {
                                                                                    self::execute_java_script_call_primitive_instruction(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                                }
                                                                                self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                                }
                                                                                self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                                                                    self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                                }
                                                                                self::JavaScriptCallInstruction::PropertyKey(destination_9, input, realm) => {
                                                                                    self::accept_java_script_call_key(program.clone(), current.clone(), destination_9, realm.clone(), self::begin_java_script_property_key_conversion(shared_constructor_field3.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(input))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2.clone()))
                                                                                }
                                                                                self::JavaScriptCallInstruction::PropertyDefinition(destination_10, target_3, key, attributes, realm_2) => {
                                                                                    self::accept_java_script_call_definition(program.clone(), current.clone(), destination_10, realm_2.clone(), self::begin_java_script_property_definition(shared_constructor_field3.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(target_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(attributes))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2.clone()))
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
                                            self::JavaScriptCallPending::WaitCall(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptCallPending::WaitGet(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptCallPending::WaitSet(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptCallPending::WaitKey(destination_11, realm_3, state) => {
                                                let state = *state;
                                                self::accept_java_script_call_key(program.clone(), machine.clone(), destination_11, realm_3.clone(), self::step_java_script_property_key_conversion(shared_constructor_field3.clone(), state.clone()))
                                            }
                                            self::JavaScriptCallPending::WaitDefinition(destination_12, realm_4, state_2) => {
                                                let state_2 = *state_2;
                                                self::accept_java_script_call_definition(program.clone(), machine.clone(), destination_12, realm_4.clone(), self::step_java_script_property_definition(shared_constructor_field3.clone(), state_2.clone()))
                                            }
                                            self::JavaScriptCallPending::WaitNumber(destination_13, realm_5, state_3) => {
                                                let state_3 = *state_3;
                                                self::step_java_script_call_number(program.clone(), machine.clone(), destination_13, realm_5.clone(), state_3.clone())
                                            }
                                            self::JavaScriptCallPending::WaitText(destination_14, realm_6, state_4) => {
                                                let state_4 = *state_4;
                                                self::step_java_script_call_text(program.clone(), machine.clone(), destination_14, realm_6.clone(), state_4.clone())
                                            }
                                            self::JavaScriptCallPending::WaitError(_, _, _) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
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
pub fn begin_java_script_call_machine(program: self::JavaScriptCallProgram, heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, receiver: self::JavaScriptSemanticValue, arguments_: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallMachineTransition {
    {
        let error = self::read_java_script_call_program_error(program.clone(), heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), error.clone())
        } else {
            {
                let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value2274, _, _) => source_ml_source2_data_property_value2274.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
                if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 101, 110, 116, 114, 121, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))
                } else {
                    if (((arguments_.len() as f64)) > (1048576f64)) {
                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 97, 108, 108, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                    } else {
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), receiver_error.clone())
                            } else {
                                {
                                    let source_ml_source1_temporary_index154 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s2275 = self::source_ml_source1_temporary_begin_java_script_call_machine_loop155(heap.clone(), arguments_.clone(), source_ml_source1_temporary_index154.clone());
                                        match source_ml_source2_s2275.clone() {
                                            self::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Done => {
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(heap.clone()), vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)])))
                                            }
                                            self::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Return(source_ml_source1_temporary_result155) => {
                                                let source_ml_source1_temporary_result155 = *source_ml_source1_temporary_result155;
                                                source_ml_source1_temporary_result155.clone()
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
