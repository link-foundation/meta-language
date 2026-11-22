// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptCallTemplate;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510;
use super::accept_java_script_call_array_assignment;
use super::advance_java_script_call_frame;
use super::begin_java_script_array_assignment;
use super::delete_java_script_ordinary_property;
use super::find_java_script_call_template;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::plan_java_script_ordinary_set_with_array_definition;
use super::push_java_script_call;
use super::read_java_script_collection_construction_receiver;
use super::read_java_script_collection_iterator_receiver_error;
use super::read_java_script_executable_call_function;
use super::read_java_script_indexed_property_presence;
use super::read_java_script_ordinary_key_error;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_ordinary_call_property(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    instruction: self::JavaScriptCallInstruction,
) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            shared_constructor_field13,
            _,
            shared_constructor_field14,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    shared_constructor_field11,
                    shared_constructor_field12,
                ) => {
                    let shared_constructor_field11 = *shared_constructor_field11;
                    {
                        let next = self::advance_java_script_call_frame(frame.clone());
                        match next.clone() {
                            self::JavaScriptCallFrame::CallFrame(
                                shared_constructor_field1,
                                shared_constructor_field2,
                                shared_constructor_field3,
                                shared_constructor_field4,
                                shared_constructor_field5,
                                shared_constructor_field6,
                                shared_constructor_field7,
                                shared_constructor_field8,
                                shared_constructor_field9,
                                shared_constructor_field10,
                            ) => {
                                let shared_constructor_field6 = *shared_constructor_field6;
                                {
                                    let registers = shared_constructor_field14.clone();
                                    match instruction.clone() {
                                        self::JavaScriptCallInstruction::CollectionAllocate(destination, instruction_prototype, _) => {
                                            {
                                                let held = self::read_java_script_collection_construction_receiver(program.clone(), machine.clone(), frame.clone(), instruction.clone());
                                                if (((({ let ml_reference_value2322 = held.borrow().clone(); ml_reference_value2322 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), (({ let ml_reference_value2323 = held.borrow().clone(); ml_reference_value2323 }).0).0)
                                                } else {
                                                    {
                                                        let receiver = (({ let ml_reference_value2324 = held.borrow().clone(); ml_reference_value2324 }).0).1;
                                                        if !(match (receiver.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination, receiver.clone().expect("a proven present value")))))
                                                        } else {
                                                            {
                                                                let source_ml_source2_s36024 = self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Next;
                                                                match source_ml_source2_s36024.clone() {
                                                                    self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Next => {
                                                                        {
                                                                            let prototype = ({ let ml_reference_array2325 = registers.clone(); let ml_reference_position2325 = instruction_prototype; let ml_reference_projection2325 = self::ml_array::optional_at(&*ml_reference_array2325.borrow(), self::ml_array::number_index(ml_reference_position2325)); ml_reference_projection2325 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                            if (match prototype.clone() {
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            } && match prototype.clone() {
                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }) {
                                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 108, 108, 32, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                                            } else {
                                                                                if (({ let ml_reference_array2326 = match shared_constructor_field11.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value36036, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value36036.clone()
                                                                                    }
                                                                                }; let ml_reference_projection2326 = ml_reference_array2326.borrow().len() as f64; ml_reference_projection2326 }) >= (4096f64)) {
                                                                                    self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                                } else {
                                                                                    self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(machine.clone()), Box::new(prototype.clone()))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Return(source_ml_source1_temporary_result1510) => {
                                                                        let source_ml_source1_temporary_result1510 = *source_ml_source1_temporary_result1510;
                                                                        source_ml_source1_temporary_result1510.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::CollectionIteratorAllocate(destination_2, instruction_prototype_2, source_ml_source2_f36011, _, source_ml_source2_f36013) => {
                                            {
                                                let held_2 = self::read_java_script_collection_construction_receiver(program.clone(), machine.clone(), frame.clone(), instruction.clone());
                                                if (((({ let ml_reference_value2327 = held_2.borrow().clone(); ml_reference_value2327 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), (({ let ml_reference_value2328 = held_2.borrow().clone(); ml_reference_value2328 }).0).0)
                                                } else {
                                                    {
                                                        let receiver_2 = (({ let ml_reference_value2329 = held_2.borrow().clone(); ml_reference_value2329 }).0).1;
                                                        if !(match (receiver_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination_2, receiver_2.clone().expect("a proven present value")))))
                                                        } else {
                                                            {
                                                                let source_ml_source2_s36064 = if (source_ml_source2_f36011 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) {
                                                                    {
                                                                        let source_ml_source1_shadow_receiver_1 = ({
                                                                            let source_ml_source2_array_read_index36069 = 0f64;
                                                                            { let ml_reference_array2330 = registers.clone(); let ml_reference_position2330 = source_ml_source2_array_read_index36069; let ml_reference_projection2330 = self::ml_array::optional_at(&*ml_reference_array2330.borrow(), self::ml_array::number_index(ml_reference_position2330)); ml_reference_projection2330 }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                        {
                                                                            let owner = self::read_java_script_executable_call_function(program.clone(), shared_constructor_field11.clone(), shared_constructor_field13.clone());
                                                                            if (match source_ml_source1_shadow_receiver_1.clone() {
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    true
                                                                                }
                                                                                _ => {
                                                                                    false
                                                                                }
                                                                            } || match source_ml_source1_shadow_receiver_1.clone() {
                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                    true
                                                                                }
                                                                                _ => {
                                                                                    false
                                                                                }
                                                                            }) {
                                                                                self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Return(Box::new(if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109]))
                                                                                } else {
                                                                                    self::request_java_script_call_error(machine.clone(), match owner.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value36073, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value36073.clone()
                                                                                        }
                                                                                    }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 111, 114, 32, 110, 117, 108, 108, 32, 116, 111, 32, 111, 98, 106, 101, 99, 116])))
                                                                                }))
                                                                            } else {
                                                                                match source_ml_source1_shadow_receiver_1.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                        self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Next
                                                                                    }
                                                                                    _ => {
                                                                                        self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Return(Box::new(self::JavaScriptCallMachineTransition::MachineNeedArrayIteratorBoxing(Box::new(machine.clone()), Box::new(source_ml_source1_shadow_receiver_1.clone()))))
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if (source_ml_source2_f36011 == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) {
                                                                        {
                                                                            let value = ({
                                                                                let source_ml_source2_array_read_index36077 = (((source_ml_source2_f36013.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()).clone().unwrap_or_else(|| (-1f64));
                                                                                { let ml_reference_array2331 = registers.clone(); let ml_reference_position2331 = source_ml_source2_array_read_index36077; let ml_reference_projection2331 = self::ml_array::optional_at(&*ml_reference_array2331.borrow(), self::ml_array::number_index(ml_reference_position2331)); ml_reference_projection2331 }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                            if (match value.clone() {
                                                                                self::JavaScriptSemanticValue::Text(_) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            } && match value.clone() {
                                                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }) {
                                                                                self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Return(Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                                                                            } else {
                                                                                self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Next
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let source_ml_source1_shadow_receiver_2 = self::read_java_script_collection_iterator_receiver_error(shared_constructor_field11.clone(), ({
                                                                                let source_ml_source2_array_read_index36084 = 0f64;
                                                                                { let ml_reference_array2332 = registers.clone(); let ml_reference_position2332 = source_ml_source2_array_read_index36084; let ml_reference_projection2332 = self::ml_array::optional_at(&*ml_reference_array2332.borrow(), self::ml_array::number_index(ml_reference_position2332)); ml_reference_projection2332 }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), &(source_ml_source2_f36011));
                                                                            if (((({ let ml_reference_value2333 = source_ml_source1_shadow_receiver_2.borrow().clone(); ml_reference_value2333 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                {
                                                                                    let owner_2 = self::read_java_script_executable_call_function(program.clone(), shared_constructor_field11.clone(), shared_constructor_field13.clone());
                                                                                    self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Return(Box::new(if ((({ let ml_reference_value2334 = source_ml_source1_shadow_receiver_2.borrow().clone(); ml_reference_value2334 }).0).1 && !(match (owner_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                                        self::request_java_script_call_error(machine.clone(), match owner_2.clone().expect("a proven present value") {
                                                                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value36087, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value36087.clone()
                                                                                            }
                                                                                        }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &((({ let ml_reference_value2335 = source_ml_source1_shadow_receiver_2.borrow().clone(); ml_reference_value2335 }).0).0))
                                                                                    } else {
                                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), (({ let ml_reference_value2336 = source_ml_source1_shadow_receiver_2.borrow().clone(); ml_reference_value2336 }).0).0)
                                                                                    }))
                                                                                }
                                                                            } else {
                                                                                self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Next
                                                                            }
                                                                        }
                                                                    }
                                                                };
                                                                match source_ml_source2_s36064.clone() {
                                                                    self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Next => {
                                                                        {
                                                                            let prototype_2 = ({ let ml_reference_array2337 = registers.clone(); let ml_reference_position2337 = instruction_prototype_2; let ml_reference_projection2337 = self::ml_array::optional_at(&*ml_reference_array2337.borrow(), self::ml_array::number_index(ml_reference_position2337)); ml_reference_projection2337 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                            if (match prototype_2.clone() {
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            } && match prototype_2.clone() {
                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }) {
                                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 108, 108, 32, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                                            } else {
                                                                                if (({ let ml_reference_array2338 = match shared_constructor_field11.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value36097, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value36097.clone()
                                                                                    }
                                                                                }; let ml_reference_projection2338 = ml_reference_array2338.borrow().len() as f64; ml_reference_projection2338 }) >= (4096f64)) {
                                                                                    self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                                } else {
                                                                                    self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(machine.clone()), Box::new(prototype_2.clone()))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Return(source_ml_source1_temporary_result1510_2) => {
                                                                        let source_ml_source1_temporary_result1510_2 = *source_ml_source1_temporary_result1510_2;
                                                                        source_ml_source1_temporary_result1510_2.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::RequireWrite(instruction_success) => {
                                            {
                                                let success = ({ let ml_reference_array2339 = registers.clone(); let ml_reference_position2339 = instruction_success; let ml_reference_projection2339 = self::ml_array::optional_at(&*ml_reference_array2339.borrow(), self::ml_array::number_index(ml_reference_position2339)); ml_reference_projection2339 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                {
                                                    let owner_3 = self::read_java_script_executable_call_function(program.clone(), shared_constructor_field11.clone(), shared_constructor_field13.clone());
                                                    if match (owner_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                    } else {
                                                        match success.clone() {
                                                            self::JavaScriptSemanticValue::Boolean(value_2) => {
                                                                {
                                                                    let current = self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), next.clone());
                                                                    if value_2 {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(current.clone()))
                                                                    } else {
                                                                        self::request_java_script_call_error(current.clone(), match owner_3.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value36105, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value36105.clone()
                                                                            }
                                                                        }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100])))
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryAllocate(destination_3, instruction_prototype_3) => {
                                            {
                                                let held_3 = self::read_java_script_collection_construction_receiver(program.clone(), machine.clone(), frame.clone(), instruction.clone());
                                                if (((({ let ml_reference_value2340 = held_3.borrow().clone(); ml_reference_value2340 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), (({ let ml_reference_value2341 = held_3.borrow().clone(); ml_reference_value2341 }).0).0)
                                                } else {
                                                    {
                                                        let receiver_3 = (({ let ml_reference_value2342 = held_3.borrow().clone(); ml_reference_value2342 }).0).1;
                                                        if !(match (receiver_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination_3, receiver_3.clone().expect("a proven present value")))))
                                                        } else {
                                                            {
                                                                let source_ml_source2_s36113 = self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Next;
                                                                match source_ml_source2_s36113.clone() {
                                                                    self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Next => {
                                                                        {
                                                                            let prototype_3 = ({ let ml_reference_array2343 = registers.clone(); let ml_reference_position2343 = instruction_prototype_3; let ml_reference_projection2343 = self::ml_array::optional_at(&*ml_reference_array2343.borrow(), self::ml_array::number_index(ml_reference_position2343)); ml_reference_projection2343 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                            if (match prototype_3.clone() {
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            } && match prototype_3.clone() {
                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }) {
                                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 108, 108, 32, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                                            } else {
                                                                                if (({ let ml_reference_array2344 = match shared_constructor_field11.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value36125, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value36125.clone()
                                                                                    }
                                                                                }; let ml_reference_projection2344 = ml_reference_array2344.borrow().len() as f64; ml_reference_projection2344 }) >= (4096f64)) {
                                                                                    self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                                } else {
                                                                                    self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(machine.clone()), Box::new(prototype_3.clone()))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Return(source_ml_source1_temporary_result1510_3) => {
                                                                        let source_ml_source1_temporary_result1510_3 = *source_ml_source1_temporary_result1510_3;
                                                                        source_ml_source1_temporary_result1510_3.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::ArrayAllocate(destination_4, instruction_prototype_4, _) => {
                                            {
                                                let held_4 = self::read_java_script_collection_construction_receiver(program.clone(), machine.clone(), frame.clone(), instruction.clone());
                                                if (((({ let ml_reference_value2345 = held_4.borrow().clone(); ml_reference_value2345 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), (({ let ml_reference_value2346 = held_4.borrow().clone(); ml_reference_value2346 }).0).0)
                                                } else {
                                                    {
                                                        let receiver_4 = (({ let ml_reference_value2347 = held_4.borrow().clone(); ml_reference_value2347 }).0).1;
                                                        if !(match (receiver_4.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination_4, receiver_4.clone().expect("a proven present value")))))
                                                        } else {
                                                            {
                                                                let source_ml_source2_s36132 = self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Next;
                                                                match source_ml_source2_s36132.clone() {
                                                                    self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Next => {
                                                                        {
                                                                            let prototype_4 = ({ let ml_reference_array2348 = registers.clone(); let ml_reference_position2348 = instruction_prototype_4; let ml_reference_projection2348 = self::ml_array::optional_at(&*ml_reference_array2348.borrow(), self::ml_array::number_index(ml_reference_position2348)); ml_reference_projection2348 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                            if (match prototype_4.clone() {
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            } && match prototype_4.clone() {
                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }) {
                                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 108, 108, 32, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                                            } else {
                                                                                if (({ let ml_reference_array2349 = match shared_constructor_field11.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value36144, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value36144.clone()
                                                                                    }
                                                                                }; let ml_reference_projection2349 = ml_reference_array2349.borrow().len() as f64; ml_reference_projection2349 }) >= (4096f64)) {
                                                                                    self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                                } else {
                                                                                    self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(machine.clone()), Box::new(prototype_4.clone()))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510::MlSource1TemporaryExecuteJavaScriptOrdinaryCallPropertyJoin1510Return(source_ml_source1_temporary_result1510_4) => {
                                                                        let source_ml_source1_temporary_result1510_4 = *source_ml_source1_temporary_result1510_4;
                                                                        source_ml_source1_temporary_result1510_4.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::ClosureAllocate(_, owner_4, ordinal, _, _) => {
                                            {
                                                let template = {
                                                    let source_ml_source2_array_read_value36146 = (match program.clone() {
                                                        self::JavaScriptCallProgram::CallProgram(_, _, _, source_ml_source2_data_property_value36145, _, _) => {
                                                            ((source_ml_source2_data_property_value36145.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,))).0).clone()
                                                        }
                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallTemplate>::new())));
                                                    {
                                                        let source_ml_source2_array_read_index36147 = self::find_java_script_call_template(program.clone(), owner_4.clone(), ordinal);
                                                        { let ml_reference_array2350 = source_ml_source2_array_read_value36146.clone(); let ml_reference_position2350 = source_ml_source2_array_read_index36147; let ml_reference_projection2350 = self::ml_array::optional_at(&*ml_reference_array2350.borrow(), self::ml_array::number_index(ml_reference_position2350)); ml_reference_projection2350 }
                                                    }
                                                };
                                                if match (template.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 116, 105, 99, 32, 108, 101, 120, 105, 99, 97, 108, 32, 116, 101, 109, 112, 108, 97, 116, 101]))
                                                } else {
                                                    if (({ let ml_reference_array2351 = match shared_constructor_field11.clone() {
                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value36152, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value36152.clone()
                                                        }
                                                    }; let ml_reference_projection2351 = ml_reference_array2351.borrow().len() as f64; ml_reference_projection2351 }) >= (4096f64)) {
                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                    } else {
                                                        self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(machine.clone()), Box::new(match template.clone().expect("a proven present value") {
                                                            self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, _, _, source_ml_source2_data_property_value36153, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value36153 = *source_ml_source2_data_property_value36153;
                                                                source_ml_source2_data_property_value36153.clone()
                                                            }
                                                        }))
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryGet(destination_5, instruction_object, key, receiver_5) => {
                                            {
                                                let object = ({ let ml_reference_array2352 = registers.clone(); let ml_reference_position2352 = instruction_object; let ml_reference_projection2352 = self::ml_array::optional_at(&*ml_reference_array2352.borrow(), self::ml_array::number_index(ml_reference_position2352)); ml_reference_projection2352 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                match object.clone() {
                                                    self::JavaScriptSemanticValue::Object(identity, _) => {
                                                        {
                                                            let plan = self::plan_java_script_ordinary_get(shared_constructor_field11.clone(), identity.clone(), ({ let ml_reference_array2353 = registers.clone(); let ml_reference_position2353 = key; let ml_reference_projection2353 = self::ml_array::optional_at(&*ml_reference_array2353.borrow(), self::ml_array::number_index(ml_reference_position2353)); ml_reference_projection2353 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array2354 = registers.clone(); let ml_reference_position2354 = receiver_5; let ml_reference_projection2354 = self::ml_array::optional_at(&*ml_reference_array2354.borrow(), self::ml_array::number_index(ml_reference_position2354)); ml_reference_projection2354 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                            if (((({ let ml_reference_value2355 = plan.borrow().clone(); ml_reference_value2355 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), (({ let ml_reference_value2356 = plan.borrow().clone(); ml_reference_value2356 }).0).3)
                                                            } else {
                                                                if (((({ let ml_reference_value2357 = plan.borrow().clone(); ml_reference_value2357 }).0).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination_5, (({ let ml_reference_value2358 = plan.borrow().clone(); ml_reference_value2358 }).0).5))))
                                                                } else {
                                                                    if (((({ let ml_reference_value2359 = plan.borrow().clone(); ml_reference_value2359 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                        self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), {
                                                                            let source_ml_source2_nominal_field36168 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field36170 = shared_constructor_field2;
                                                                                {
                                                                                    let source_ml_source2_nominal_field36172 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field36174 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field36176 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field36178 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field36180 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field36182 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field36184 = shared_constructor_field9.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field36186 = shared_constructor_field10.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override36188 = self::JavaScriptCallPending::WaitGet(destination_5);
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field36168.clone(), source_ml_source2_nominal_field36170, source_ml_source2_nominal_field36172.clone(), source_ml_source2_nominal_field36174.clone(), source_ml_source2_nominal_field36176.clone(), Box::new(source_ml_source2_nominal_override36188.clone()), source_ml_source2_nominal_field36180.clone(), source_ml_source2_nominal_field36182.clone(), source_ml_source2_nominal_field36184.clone(), source_ml_source2_nominal_field36186.clone())
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
                                                                        }), (({ let ml_reference_value2360 = plan.borrow().clone(); ml_reference_value2360 }).0).2, (({ let ml_reference_value2361 = plan.borrow().clone(); ml_reference_value2361 }).0).4, (({ let ml_reference_value2362 = plan.borrow().clone(); ml_reference_value2362 }).0).1)
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114]))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryDelete(destination_6, instruction_object_2, key_2) => {
                                            {
                                                let object_2 = ({ let ml_reference_array2363 = registers.clone(); let ml_reference_position2363 = instruction_object_2; let ml_reference_projection2363 = self::ml_array::optional_at(&*ml_reference_array2363.borrow(), self::ml_array::number_index(ml_reference_position2363)); ml_reference_projection2363 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                match object_2.clone() {
                                                    self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                                        {
                                                            let deleted = self::delete_java_script_ordinary_property(shared_constructor_field11.clone(), identity_2.clone(), ({ let ml_reference_array2364 = registers.clone(); let ml_reference_position2364 = key_2; let ml_reference_projection2364 = self::ml_array::optional_at(&*ml_reference_array2364.borrow(), self::ml_array::number_index(ml_reference_position2364)); ml_reference_projection2364 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                            if (((({ let ml_reference_value2365 = deleted.borrow().clone(); ml_reference_value2365 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new((({ let ml_reference_value2366 = deleted.borrow().clone(); ml_reference_value2366 }).0).1), (({ let ml_reference_value2367 = deleted.borrow().clone(); ml_reference_value2367 }).0).0)
                                                            } else {
                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame((({ let ml_reference_value2368 = deleted.borrow().clone(); ml_reference_value2368 }).0).1, shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Boolean((({ let ml_reference_value2369 = deleted.borrow().clone(); ml_reference_value2369 }).0).2)))))
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 100, 101, 108, 101, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]))
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryHas(destination_7, instruction_object_3, key_3) => {
                                            {
                                                let object_3 = ({ let ml_reference_array2370 = registers.clone(); let ml_reference_position2370 = instruction_object_3; let ml_reference_projection2370 = self::ml_array::optional_at(&*ml_reference_array2370.borrow(), self::ml_array::number_index(ml_reference_position2370)); ml_reference_projection2370 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                match object_3.clone() {
                                                    self::JavaScriptSemanticValue::Object(identity_3, _) => {
                                                        {
                                                            let key_error = self::read_java_script_ordinary_key_error(shared_constructor_field11.clone(), ({ let ml_reference_array2371 = registers.clone(); let ml_reference_position2371 = key_3; let ml_reference_projection2371 = self::ml_array::optional_at(&*ml_reference_array2371.borrow(), self::ml_array::number_index(ml_reference_position2371)); ml_reference_projection2371 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                            if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), key_error.clone())
                                                            } else {
                                                                {
                                                                    let presence = self::read_java_script_indexed_property_presence(shared_constructor_field11.clone(), identity_3.clone(), ({ let ml_reference_array2372 = registers.clone(); let ml_reference_position2372 = key_3; let ml_reference_projection2372 = self::ml_array::optional_at(&*ml_reference_array2372.borrow(), self::ml_array::number_index(ml_reference_position2372)); ml_reference_projection2372 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                                    if (((({ let ml_reference_value2373 = presence.borrow().clone(); ml_reference_value2373 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), (({ let ml_reference_value2374 = presence.borrow().clone(); ml_reference_value2374 }).0).0)
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination_7, self::JavaScriptSemanticValue::Boolean((({ let ml_reference_value2375 = presence.borrow().clone(); ml_reference_value2375 }).0).1)))))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 72, 97, 115, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101]))
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::OrdinarySet(destination_8, instruction_object_4, key_4, value_3, instruction_receiver) => {
                                            {
                                                let object_4 = ({ let ml_reference_array2376 = registers.clone(); let ml_reference_position2376 = instruction_object_4; let ml_reference_projection2376 = self::ml_array::optional_at(&*ml_reference_array2376.borrow(), self::ml_array::number_index(ml_reference_position2376)); ml_reference_projection2376 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                match object_4.clone() {
                                                    self::JavaScriptSemanticValue::Object(object_identity, _) => {
                                                        {
                                                            let plan_2 = self::plan_java_script_ordinary_set_with_array_definition(shared_constructor_field11.clone(), object_identity.clone(), ({ let ml_reference_array2377 = registers.clone(); let ml_reference_position2377 = key_4; let ml_reference_projection2377 = self::ml_array::optional_at(&*ml_reference_array2377.borrow(), self::ml_array::number_index(ml_reference_position2377)); ml_reference_projection2377 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array2378 = registers.clone(); let ml_reference_position2378 = value_3; let ml_reference_projection2378 = self::ml_array::optional_at(&*ml_reference_array2378.borrow(), self::ml_array::number_index(ml_reference_position2378)); ml_reference_projection2378 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array2379 = registers.clone(); let ml_reference_position2379 = instruction_receiver; let ml_reference_projection2379 = self::ml_array::optional_at(&*ml_reference_array2379.borrow(), self::ml_array::number_index(ml_reference_position2379)); ml_reference_projection2379 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                            if (((({ let ml_reference_value2380 = plan_2.borrow().clone(); ml_reference_value2380 }).0).4) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), (({ let ml_reference_value2381 = plan_2.borrow().clone(); ml_reference_value2381 }).0).4)
                                                            } else {
                                                                if (((({ let ml_reference_value2382 = plan_2.borrow().clone(); ml_reference_value2382 }).0).0) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))) {
                                                                    {
                                                                        let owner_5 = self::read_java_script_executable_call_function(program.clone(), shared_constructor_field11.clone(), shared_constructor_field13.clone());
                                                                        {
                                                                            let receiver_6 = (({ let ml_reference_value2383 = plan_2.borrow().clone(); ml_reference_value2383 }).0).6;
                                                                            {
                                                                                let definition = ((((({ let ml_reference_value2384 = plan_2.borrow().clone(); ml_reference_value2384 }).0).3).clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>,))).0).clone();
                                                                                if ((match (owner_5.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match receiver_6.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }) || match (definition.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![100, 101, 102, 101, 114, 114, 101, 100, 32, 65, 114, 114, 97, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 114, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 118, 97, 108, 117, 101, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114]))
                                                                                } else {
                                                                                    {
                                                                                        let identity_4 = (((match receiver_6.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value36229, _) => {
                                                                                                Some((source_ml_source2_data_property_value36229.clone(),))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(self::ml::Big,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0).clone();
                                                                                        if {
                                                                                            let source_ml_source2_comparison_operand36231 = None::<()>;
                                                                                            match (identity_4.clone(), source_ml_source2_comparison_operand36231.clone()) { (_ml_left, Some(_ml_right)) => false, _ => false }
                                                                                        } {
                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![100, 101, 102, 101, 114, 114, 101, 100, 32, 65, 114, 114, 97, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 114, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 118, 97, 108, 117, 101, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114]))
                                                                                        } else {
                                                                                            {
                                                                                                let current_2 = self::replace_java_script_call_frame((({ let ml_reference_value2385 = plan_2.borrow().clone(); ml_reference_value2385 }).0).5, shared_constructor_field12.clone(), next.clone());
                                                                                                self::accept_java_script_call_array_assignment(program.clone(), current_2.clone(), destination_8, match owner_5.clone().expect("a proven present value") {
                                                                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value36234, _, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value36234.clone()
                                                                                                    }
                                                                                                }, self::begin_java_script_array_assignment((({ let ml_reference_value2386 = plan_2.borrow().clone(); ml_reference_value2386 }).0).5, identity_4.clone(), ({ let ml_reference_array2387 = registers.clone(); let ml_reference_position2387 = key_4; let ml_reference_projection2387 = self::ml_array::optional_at(&*ml_reference_array2387.borrow(), self::ml_array::number_index(ml_reference_position2387)); ml_reference_projection2387 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), definition.clone().expect("a proven present value"), match program.clone() {
                                                                                                    self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value36237, _, _, _) => {
                                                                                                        let source_ml_source2_data_property_value36237 = *source_ml_source2_data_property_value36237;
                                                                                                        source_ml_source2_data_property_value36237.clone()
                                                                                                    }
                                                                                                }))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if (((({ let ml_reference_value2388 = plan_2.borrow().clone(); ml_reference_value2388 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                        self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), {
                                                                            let source_ml_source2_nominal_field36241 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field36243 = shared_constructor_field2;
                                                                                {
                                                                                    let source_ml_source2_nominal_field36245 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field36247 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field36249 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field36251 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field36253 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field36255 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field36257 = shared_constructor_field9.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field36259 = shared_constructor_field10.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override36261 = self::JavaScriptCallPending::WaitSet(destination_8);
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field36241.clone(), source_ml_source2_nominal_field36243, source_ml_source2_nominal_field36245.clone(), source_ml_source2_nominal_field36247.clone(), source_ml_source2_nominal_field36249.clone(), Box::new(source_ml_source2_nominal_override36261.clone()), source_ml_source2_nominal_field36253.clone(), source_ml_source2_nominal_field36255.clone(), source_ml_source2_nominal_field36257.clone(), source_ml_source2_nominal_field36259.clone())
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
                                                                        }), (({ let ml_reference_value2389 = plan_2.borrow().clone(); ml_reference_value2389 }).0).2, (({ let ml_reference_value2390 = plan_2.borrow().clone(); ml_reference_value2390 }).0).6, (({ let ml_reference_value2391 = plan_2.borrow().clone(); ml_reference_value2391 }).0).1)
                                                                    } else {
                                                                        if (((({ let ml_reference_value2392 = plan_2.borrow().clone(); ml_reference_value2392 }).0).0) == (self::ml_string::Text::from_units(vec![104, 111, 115, 116, 32, 99, 97, 108, 108]))) {
                                                                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), {
                                                                                let source_ml_source2_nominal_field36265 = shared_constructor_field1.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field36267 = shared_constructor_field2;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field36269 = shared_constructor_field3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field36271 = shared_constructor_field4.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field36273 = shared_constructor_field5.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field36275 = shared_constructor_field6.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field36277 = shared_constructor_field7.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field36279 = shared_constructor_field8.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field36281 = shared_constructor_field9.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field36283 = shared_constructor_field10.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override36285 = self::JavaScriptCallPending::WaitHostSet(destination_8);
                                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field36265.clone(), source_ml_source2_nominal_field36267, source_ml_source2_nominal_field36269.clone(), source_ml_source2_nominal_field36271.clone(), source_ml_source2_nominal_field36273.clone(), Box::new(source_ml_source2_nominal_override36285.clone()), source_ml_source2_nominal_field36277.clone(), source_ml_source2_nominal_field36279.clone(), source_ml_source2_nominal_field36281.clone(), source_ml_source2_nominal_field36283.clone())
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
                                                                            }), (({ let ml_reference_value2393 = plan_2.borrow().clone(); ml_reference_value2393 }).0).2, (({ let ml_reference_value2394 = plan_2.borrow().clone(); ml_reference_value2394 }).0).6, (({ let ml_reference_value2395 = plan_2.borrow().clone(); ml_reference_value2395 }).0).1)
                                                                        } else {
                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame((({ let ml_reference_value2396 = plan_2.borrow().clone(); ml_reference_value2396 }).0).5, shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination_8, self::JavaScriptSemanticValue::Boolean((({ let ml_reference_value2397 = plan_2.borrow().clone(); ml_reference_value2397 }).0).7)))))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
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
