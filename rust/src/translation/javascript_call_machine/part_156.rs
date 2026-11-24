// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallCompletion;
use super::JavaScriptCallFrame;
use super::JavaScriptCallHandler;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptFunctionTextResult;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::accept_java_script_call_completion;
use super::accept_java_script_call_function_bind;
use super::advance_java_script_call_frame;
use super::begin_java_script_function_bind;
use super::convert_java_script_semantic_boolean;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_function_text;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::request_java_script_call_error;
use super::retain_java_script_call_resource_limit_machine;
use super::return_java_script_call_value;
use super::throw_java_script_call_value;
use super::unwind_java_script_call_completion;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_control_instruction(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    instruction: self::JavaScriptCallInstruction,
    realm: self::ml::Big,
) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            shared_constructor_field13,
            _,
            shared_constructor_field14,
            shared_constructor_field15,
            shared_constructor_field16,
            _,
            _,
            _,
            _,
            _,
        ) => match machine.clone() {
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
                                let current = self::replace_java_script_call_frame(
                                    shared_constructor_field11.clone(),
                                    shared_constructor_field12.clone(),
                                    next.clone(),
                                );
                                {
                                    let registers = shared_constructor_field14.clone();
                                    match instruction.clone() {
                                            self::JavaScriptCallInstruction::Constant(destination, value) => {
                                                let value = *value;
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination, value.clone()))))
                                            }
                                            self::JavaScriptCallInstruction::Copy(destination_2, source) => {
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination_2, ({ let ml_reference_array1631 = registers.clone(); let ml_reference_position1631 = source; let ml_reference_projection1631 = self::ml_array::optional_at(&*ml_reference_array1631.borrow(), self::ml_array::number_index(ml_reference_position1631)); ml_reference_projection1631 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                            }
                                            self::JavaScriptCallInstruction::Argument(destination_3, index) => {
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination_3, ({
                                                    let source_ml_source2_array_read_value29532 = shared_constructor_field15.clone();
                                                    { let ml_reference_array1632 = source_ml_source2_array_read_value29532.clone(); let ml_reference_position1632 = index; let ml_reference_projection1632 = self::ml_array::optional_at(&*ml_reference_array1632.borrow(), self::ml_array::number_index(ml_reference_position1632)); ml_reference_projection1632 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                            }
                                            self::JavaScriptCallInstruction::ArgumentCount(destination_4) => {
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination_4, self::JavaScriptSemanticValue::Number({ let ml_reference_array1633 = shared_constructor_field15.clone(); let ml_reference_projection1633 = ml_reference_array1633.borrow().len() as f64; ml_reference_projection1633 })))))
                                            }
                                            self::JavaScriptCallInstruction::ArgumentAt(destination_5, instruction_index) => {
                                                {
                                                    let index_2 = ({ let ml_reference_array1634 = registers.clone(); let ml_reference_position1634 = instruction_index; let ml_reference_projection1634 = self::ml_array::optional_at(&*ml_reference_array1634.borrow(), self::ml_array::number_index(ml_reference_position1634)); ml_reference_projection1634 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                    if ((((match index_2.clone() {
                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    } || !(self::ml_math::is_safe_integer(((match index_2.clone() {
                                                        self::JavaScriptSemanticValue::Number(source_ml_source2_data_property_value29540) => {
                                                            Some((source_ml_source2_data_property_value29540,))
                                                        }
                                                        _ => {
                                                            None::<(f64,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0))) || {
                                                        let source_ml_source2_same_value_operand29542 = ((match index_2.clone() {
                                                            self::JavaScriptSemanticValue::Number(source_ml_source2_data_property_value29541) => {
                                                                Some((source_ml_source2_data_property_value29541,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0;
                                                        {
                                                            let source_ml_source2_same_value_operand29543 = (-0f64);
                                                            (((source_ml_source2_same_value_operand29542 == source_ml_source2_same_value_operand29543) && ((source_ml_source2_same_value_operand29542 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand29542)) == ((1f64 / source_ml_source2_same_value_operand29543))))) || ((source_ml_source2_same_value_operand29542 != source_ml_source2_same_value_operand29542) && (source_ml_source2_same_value_operand29543 != source_ml_source2_same_value_operand29543)))
                                                        }
                                                    }) || ((((match index_2.clone() {
                                                        self::JavaScriptSemanticValue::Number(source_ml_source2_data_property_value29545) => {
                                                            Some((source_ml_source2_data_property_value29545,))
                                                        }
                                                        _ => {
                                                            None::<(f64,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0) < (0f64))) || ((((match index_2.clone() {
                                                        self::JavaScriptSemanticValue::Number(source_ml_source2_data_property_value29547) => {
                                                            Some((source_ml_source2_data_property_value29547,))
                                                        }
                                                        _ => {
                                                            None::<(f64,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0) >= (1048576f64))) {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![100, 121, 110, 97, 109, 105, 99, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 97, 100, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 67, 97, 108, 108, 32, 76, 105, 115, 116, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101]))
                                                    } else {
                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination_5, ({
                                                            let source_ml_source2_array_read_value29553 = shared_constructor_field15.clone();
                                                            {
                                                                let source_ml_source2_array_read_index29554 = ((match index_2.clone() {
                                                                    self::JavaScriptSemanticValue::Number(source_ml_source2_data_property_value29552) => {
                                                                        Some((source_ml_source2_data_property_value29552,))
                                                                    }
                                                                    _ => {
                                                                        None::<(f64,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0;
                                                                { let ml_reference_array1635 = source_ml_source2_array_read_value29553.clone(); let ml_reference_position1635 = source_ml_source2_array_read_index29554; let ml_reference_projection1635 = self::ml_array::optional_at(&*ml_reference_array1635.borrow(), self::ml_array::number_index(ml_reference_position1635)); ml_reference_projection1635 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                    }
                                                }
                                            }
                                            self::JavaScriptCallInstruction::FunctionBind(destination_6) => {
                                                self::accept_java_script_call_function_bind(program.clone(), current.clone(), destination_6, realm.clone(), self::begin_java_script_function_bind(shared_constructor_field11.clone(), ({
                                                    let source_ml_source2_array_read_index29557 = 0f64;
                                                    { let ml_reference_array1636 = registers.clone(); let ml_reference_position1636 = source_ml_source2_array_read_index29557; let ml_reference_projection1636 = self::ml_array::optional_at(&*ml_reference_array1636.borrow(), self::ml_array::number_index(ml_reference_position1636)); ml_reference_projection1636 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                    let source_ml_source2_array_read_value29559 = shared_constructor_field15.clone();
                                                    {
                                                        let source_ml_source2_array_read_index29560 = 0f64;
                                                        { let ml_reference_array1637 = source_ml_source2_array_read_value29559.clone(); let ml_reference_position1637 = source_ml_source2_array_read_index29560; let ml_reference_projection1637 = self::ml_array::optional_at(&*ml_reference_array1637.borrow(), self::ml_array::number_index(ml_reference_position1637)); ml_reference_projection1637 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1638 = shared_constructor_field15.borrow().clone(); ml_reference_value1638 }, 1f64, f64::INFINITY)))))
                                            }
                                            self::JavaScriptCallInstruction::BoundFunctionCall(destination_7) => {
                                                {
                                                    let bound = {
                                                        let source_ml_source2_array_read_value29568 = match shared_constructor_field11.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value29564, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value29564.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_array_read_index29569 = self::find_java_script_ordinary_object(match shared_constructor_field11.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value29566, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value29566.clone()
                                                                }
                                                            }, shared_constructor_field13.clone());
                                                            { let ml_reference_array1639 = source_ml_source2_array_read_value29568.clone(); let ml_reference_position1639 = source_ml_source2_array_read_index29569; let ml_reference_projection1639 = self::ml_array::optional_at(&*ml_reference_array1639.borrow(), self::ml_array::number_index(ml_reference_position1639)); ml_reference_projection1639 }
                                                        }
                                                    };
                                                    if (match (bound.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match bound.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![97, 32, 98, 111, 117, 110, 100, 32, 99, 97, 108, 108, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 111, 119, 110, 101, 114]))
                                                    } else {
                                                        if ((({ let ml_reference_array1640 = (((match bound.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value29581) => {
                                                                Some((source_ml_source2_data_property_value29581.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone(); let ml_reference_projection1640 = ml_reference_array1640.borrow().len() as f64; ml_reference_projection1640 } + { let ml_reference_array1641 = shared_constructor_field15.clone(); let ml_reference_projection1641 = ml_reference_array1641.borrow().len() as f64; ml_reference_projection1641 })) > (1048576f64)) {
                                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 97, 110, 100, 32, 115, 117, 112, 112, 108, 105, 101, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 99, 97, 108, 108, 32, 76, 105, 115, 116, 32, 98, 117, 100, 103, 101, 116]))
                                                        } else {
                                                            {
                                                                let next_machine = self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::replace_java_script_call_pending(next.clone(), self::JavaScriptCallPending::WaitCall(destination_7)));
                                                                self::retain_java_script_call_resource_limit_machine(machine.clone(), self::push_java_script_call(program.clone(), next_machine.clone(), (((match bound.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, source_ml_source2_data_property_value29585, _, _) => {
                                                                        let source_ml_source2_data_property_value29585 = *source_ml_source2_data_property_value29585;
                                                                        Some((source_ml_source2_data_property_value29585.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::JavaScriptSemanticValue,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone(), (((match bound.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, source_ml_source2_data_property_value29586, _) => {
                                                                        let source_ml_source2_data_property_value29586 = *source_ml_source2_data_property_value29586;
                                                                        Some((source_ml_source2_data_property_value29586.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::JavaScriptSemanticValue,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone(), std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_concat_value29589 = (((match bound.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value29587) => {
                                                                            Some((source_ml_source2_data_property_value29587.clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone();
                                                                    {
                                                                        let source_ml_source2_concat_value29590 = shared_constructor_field15.clone();
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), { let ml_reference_value1642 = source_ml_source2_concat_value29589.borrow().clone(); ml_reference_value1642 }), { let ml_reference_value1643 = source_ml_source2_concat_value29590.borrow().clone(); ml_reference_value1643 })
                                                                    }
                                                                }))))
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptCallInstruction::FunctionToString(destination_8) => {
                                                {
                                                    let input = ({
                                                        let source_ml_source2_array_read_index29592 = 0f64;
                                                        { let ml_reference_array1644 = registers.clone(); let ml_reference_position1644 = source_ml_source2_array_read_index29592; let ml_reference_projection1644 = self::ml_array::optional_at(&*ml_reference_array1644.borrow(), self::ml_array::number_index(ml_reference_position1644)); ml_reference_projection1644 }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                    {
                                                        let result = self::read_java_script_function_text(program.clone(), shared_constructor_field11.clone(), input.clone());
                                                        match result.clone() {
                                                            self::JavaScriptFunctionTextResult::FunctionTextValue(value_2) => {
                                                                let value_2 = *value_2;
                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register(next.clone(), destination_8, value_2.clone()))))
                                                            }
                                                            self::JavaScriptFunctionTextResult::FunctionTextNeedAuthority(kind) => {
                                                                {
                                                                    let retained = self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::replace_java_script_call_pending(next.clone(), self::JavaScriptCallPending::WaitFunctionText(destination_8, realm.clone(), Box::new(input.clone()), kind.clone())));
                                                                    self::JavaScriptCallMachineTransition::MachineNeedFunctionText(Box::new(retained.clone()), Box::new(input.clone()), kind.clone(), realm.clone())
                                                                }
                                                            }
                                                            self::JavaScriptFunctionTextResult::FunctionTextError(error_class, reason) => {
                                                                self::request_java_script_call_error(current.clone(), realm.clone(), &(error_class), &(reason))
                                                            }
                                                            self::JavaScriptFunctionTextResult::FunctionTextInvalid(reason_2) => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), reason_2.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptCallInstruction::PushFinally(target) => {
                                                if (({ let ml_reference_array1645 = shared_constructor_field16.clone(); let ml_reference_projection1645 = ml_reference_array1645.borrow().len() as f64; ml_reference_projection1645 }) >= (1024f64)) {
                                                    self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 105, 122, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 97, 110, 100, 108, 101, 114, 32, 98, 117, 100, 103, 101, 116]))
                                                } else {
                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), {
                                                        let source_ml_source2_nominal_field29607 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_nominal_field29609 = shared_constructor_field2;
                                                            {
                                                                let source_ml_source2_nominal_field29611 = shared_constructor_field3.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field29613 = shared_constructor_field4.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field29615 = shared_constructor_field5.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field29617 = shared_constructor_field6.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field29619 = shared_constructor_field7.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field29621 = shared_constructor_field8.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field29623 = shared_constructor_field9.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field29625 = shared_constructor_field10.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override29627 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                    let source_ml_source2_concat_value29604 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_concat_value29605 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallHandler::CallFinallyHandler(target)]));
                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallHandler>::new(), { let ml_reference_value1646 = source_ml_source2_concat_value29604.borrow().clone(); ml_reference_value1646 }), { let ml_reference_value1647 = source_ml_source2_concat_value29605.borrow().clone(); ml_reference_value1647 })
                                                                                                    }
                                                                                                }));
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field29607.clone(), source_ml_source2_nominal_field29609, source_ml_source2_nominal_field29611.clone(), source_ml_source2_nominal_field29613.clone(), source_ml_source2_nominal_override29627.clone(), Box::new(source_ml_source2_nominal_field29617.clone()), source_ml_source2_nominal_field29619.clone(), source_ml_source2_nominal_field29621.clone(), source_ml_source2_nominal_field29623.clone(), source_ml_source2_nominal_field29625.clone())
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
                                                    })))
                                                }
                                            }
                                            self::JavaScriptCallInstruction::FinishFinally => {
                                                {
                                                    let handler = {
                                                        let source_ml_source2_array_read_value29630 = shared_constructor_field16.clone();
                                                        {
                                                            let source_ml_source2_array_read_index29631 = ({ let ml_reference_array1648 = shared_constructor_field16.clone(); let ml_reference_projection1648 = ml_reference_array1648.borrow().len() as f64; ml_reference_projection1648 } - 1f64);
                                                            { let ml_reference_array1649 = source_ml_source2_array_read_value29630.clone(); let ml_reference_position1649 = source_ml_source2_array_read_index29631; let ml_reference_projection1649 = self::ml_array::optional_at(&*ml_reference_array1649.borrow(), self::ml_array::number_index(ml_reference_position1649)); ml_reference_projection1649 }
                                                        }
                                                    };
                                                    if (match (handler.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match handler.clone().expect("a proven present value") {
                                                        self::JavaScriptCallHandler::CallDeferredCompletion(_) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![102, 105, 110, 97, 108, 105, 122, 101, 114, 32, 101, 120, 105, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 102, 101, 114, 114, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                    } else {
                                                        {
                                                            let resumed = self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), {
                                                                let source_ml_source2_nominal_field29639 = shared_constructor_field1.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field29641 = shared_constructor_field2;
                                                                    {
                                                                        let source_ml_source2_nominal_field29643 = shared_constructor_field3.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field29645 = shared_constructor_field4.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field29647 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field29649 = shared_constructor_field6.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field29651 = shared_constructor_field7.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field29653 = shared_constructor_field8.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field29655 = shared_constructor_field9.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field29657 = shared_constructor_field10.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override29659 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1650 = shared_constructor_field16.borrow().clone(); ml_reference_value1650 }, 0f64, (-1f64))));
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field29639.clone(), source_ml_source2_nominal_field29641, source_ml_source2_nominal_field29643.clone(), source_ml_source2_nominal_field29645.clone(), source_ml_source2_nominal_override29659.clone(), Box::new(source_ml_source2_nominal_field29649.clone()), source_ml_source2_nominal_field29651.clone(), source_ml_source2_nominal_field29653.clone(), source_ml_source2_nominal_field29655.clone(), source_ml_source2_nominal_field29657.clone())
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
                                                            });
                                                            self::accept_java_script_call_completion(program.clone(), self::unwind_java_script_call_completion(resumed.clone(), (((match handler.clone().expect("a proven present value") {
                                                                self::JavaScriptCallHandler::CallDeferredCompletion(source_ml_source2_data_property_value29660) => {
                                                                    let source_ml_source2_data_property_value29660 = *source_ml_source2_data_property_value29660;
                                                                    Some((source_ml_source2_data_property_value29660.clone(),))
                                                                }
                                                                _ => {
                                                                    None::<(self::JavaScriptCallCompletion,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0).clone()))
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptCallInstruction::CompleteJump(target_2, depth) => {
                                                self::accept_java_script_call_completion(program.clone(), self::unwind_java_script_call_completion(current.clone(), self::JavaScriptCallCompletion::CallCompletion(self::ml_string::Text::from_units(vec![106, 117, 109, 112]), Box::new(self::JavaScriptSemanticValue::Undefined), target_2, depth)))
                                            }
                                            self::JavaScriptCallInstruction::Jump(target_3) => {
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), {
                                                    let source_ml_source2_nominal_field29664 = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field29666 = shared_constructor_field2;
                                                        {
                                                            let source_ml_source2_nominal_field29668 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field29670 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field29672 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field29674 = shared_constructor_field6.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field29676 = shared_constructor_field7.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field29678 = shared_constructor_field8.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field29680 = shared_constructor_field9.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field29682 = shared_constructor_field10.clone();
                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field29664.clone(), target_3, source_ml_source2_nominal_field29668.clone(), source_ml_source2_nominal_field29670.clone(), source_ml_source2_nominal_field29672.clone(), Box::new(source_ml_source2_nominal_field29674.clone()), source_ml_source2_nominal_field29676.clone(), source_ml_source2_nominal_field29678.clone(), source_ml_source2_nominal_field29680.clone(), source_ml_source2_nominal_field29682.clone())
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                })))
                                            }
                                            self::JavaScriptCallInstruction::Branch(condition, yes, no) => {
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), {
                                                    let source_ml_source2_nominal_field29690 = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field29692 = shared_constructor_field2;
                                                        {
                                                            let source_ml_source2_nominal_field29694 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field29696 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field29698 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field29700 = shared_constructor_field6.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field29702 = shared_constructor_field7.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field29704 = shared_constructor_field8.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field29706 = shared_constructor_field9.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field29708 = shared_constructor_field10.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override29710 = if self::convert_java_script_semantic_boolean(({ let ml_reference_array1651 = registers.clone(); let ml_reference_position1651 = condition; let ml_reference_projection1651 = self::ml_array::optional_at(&*ml_reference_array1651.borrow(), self::ml_array::number_index(ml_reference_position1651)); ml_reference_projection1651 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)) {
                                                                                                yes
                                                                                            } else {
                                                                                                no
                                                                                            };
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field29690.clone(), source_ml_source2_nominal_override29710, source_ml_source2_nominal_field29694.clone(), source_ml_source2_nominal_field29696.clone(), source_ml_source2_nominal_field29698.clone(), Box::new(source_ml_source2_nominal_field29700.clone()), source_ml_source2_nominal_field29702.clone(), source_ml_source2_nominal_field29704.clone(), source_ml_source2_nominal_field29706.clone(), source_ml_source2_nominal_field29708.clone())
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
                                                })))
                                            }
                                            self::JavaScriptCallInstruction::Return(source_2) => {
                                                self::return_java_script_call_value(program.clone(), current.clone(), ({ let ml_reference_array1652 = registers.clone(); let ml_reference_position1652 = source_2; let ml_reference_projection1652 = self::ml_array::optional_at(&*ml_reference_array1652.borrow(), self::ml_array::number_index(ml_reference_position1652)); ml_reference_projection1652 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                            }
                                            self::JavaScriptCallInstruction::Throw(source_3) => {
                                                self::throw_java_script_call_value(current.clone(), ({ let ml_reference_array1653 = registers.clone(); let ml_reference_position1653 = source_3; let ml_reference_projection1653 = self::ml_array::optional_at(&*ml_reference_array1653.borrow(), self::ml_array::number_index(ml_reference_position1653)); ml_reference_projection1653 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                            }
                                            self::JavaScriptCallInstruction::PushHandler(destination_9, target_4) => {
                                                if (({ let ml_reference_array1654 = shared_constructor_field16.clone(); let ml_reference_projection1654 = ml_reference_array1654.borrow().len() as f64; ml_reference_projection1654 }) >= (1024f64)) {
                                                    self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![104, 97, 110, 100, 108, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                } else {
                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), {
                                                        let source_ml_source2_nominal_field29723 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_nominal_field29725 = shared_constructor_field2;
                                                            {
                                                                let source_ml_source2_nominal_field29727 = shared_constructor_field3.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field29729 = shared_constructor_field4.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field29731 = shared_constructor_field5.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field29733 = shared_constructor_field6.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field29735 = shared_constructor_field7.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field29737 = shared_constructor_field8.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field29739 = shared_constructor_field9.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field29741 = shared_constructor_field10.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override29743 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                    let source_ml_source2_concat_value29720 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_concat_value29721 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallHandler::CallHandler(target_4, destination_9)]));
                                                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallHandler>::new(), { let ml_reference_value1655 = source_ml_source2_concat_value29720.borrow().clone(); ml_reference_value1655 }), { let ml_reference_value1656 = source_ml_source2_concat_value29721.borrow().clone(); ml_reference_value1656 })
                                                                                                    }
                                                                                                }));
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field29723.clone(), source_ml_source2_nominal_field29725, source_ml_source2_nominal_field29727.clone(), source_ml_source2_nominal_field29729.clone(), source_ml_source2_nominal_override29743.clone(), Box::new(source_ml_source2_nominal_field29733.clone()), source_ml_source2_nominal_field29735.clone(), source_ml_source2_nominal_field29737.clone(), source_ml_source2_nominal_field29739.clone(), source_ml_source2_nominal_field29741.clone())
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
                                                    })))
                                                }
                                            }
                                            self::JavaScriptCallInstruction::PopHandler => {
                                                if {
                                                    let source_ml_source2_comparison_operand29749 = {
                                                        let source_ml_source2_optional_chain_receiver29748 = ({
                                                            let source_ml_source2_array_read_value29746 = shared_constructor_field16.clone();
                                                            {
                                                                let source_ml_source2_array_read_index29747 = ({ let ml_reference_array1657 = shared_constructor_field16.clone(); let ml_reference_projection1657 = ml_reference_array1657.borrow().len() as f64; ml_reference_projection1657 } - 1f64);
                                                                { let ml_reference_array1658 = source_ml_source2_array_read_value29746.clone(); let ml_reference_position1658 = source_ml_source2_array_read_index29747; let ml_reference_projection1658 = self::ml_array::optional_at(&*ml_reference_array1658.borrow(), self::ml_array::number_index(ml_reference_position1658)); ml_reference_projection1658 }
                                                            }
                                                        },);
                                                        if match (source_ml_source2_optional_chain_receiver29748.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                            Some({
                                                                let source_ml_source1_temporary_optional_chain1313 = ((source_ml_source2_optional_chain_receiver29748.clone()).0).clone().expect("a proven present value");
                                                                match source_ml_source1_temporary_optional_chain1313.clone() {
                                                                    self::JavaScriptCallHandler::CallHandler(_, _) => {
                                                                        self::ml_string::Text::from_units(vec![99, 97, 108, 108, 72, 97, 110, 100, 108, 101, 114])
                                                                    }
                                                                    self::JavaScriptCallHandler::CallFinallyHandler(_) => {
                                                                        self::ml_string::Text::from_units(vec![99, 97, 108, 108, 70, 105, 110, 97, 108, 108, 121, 72, 97, 110, 100, 108, 101, 114])
                                                                    }
                                                                    self::JavaScriptCallHandler::CallDeferredCompletion(_) => {
                                                                        self::ml_string::Text::from_units(vec![99, 97, 108, 108, 68, 101, 102, 101, 114, 114, 101, 100, 67, 111, 109, 112, 108, 101, 116, 105, 111, 110])
                                                                    }
                                                                }
                                                            })
                                                        } else {
                                                            None::<self::ml_string::Text>
                                                        }
                                                    };
                                                    !(match (source_ml_source2_comparison_operand29749.clone(), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 72, 97, 110, 100, 108, 101, 114])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                } {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![97, 32, 104, 97, 110, 100, 108, 101, 114, 32, 112, 111, 112, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 99, 97, 116, 99, 104, 32, 104, 97, 110, 100, 108, 101, 114]))
                                                } else {
                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), {
                                                        let source_ml_source2_nominal_field29757 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_nominal_field29759 = shared_constructor_field2;
                                                            {
                                                                let source_ml_source2_nominal_field29761 = shared_constructor_field3.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field29763 = shared_constructor_field4.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field29765 = shared_constructor_field5.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field29767 = shared_constructor_field6.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field29769 = shared_constructor_field7.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field29771 = shared_constructor_field8.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field29773 = shared_constructor_field9.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field29775 = shared_constructor_field10.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override29777 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1659 = shared_constructor_field16.borrow().clone(); ml_reference_value1659 }, 0f64, (-1f64))));
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field29757.clone(), source_ml_source2_nominal_field29759, source_ml_source2_nominal_field29761.clone(), source_ml_source2_nominal_field29763.clone(), source_ml_source2_nominal_override29777.clone(), Box::new(source_ml_source2_nominal_field29767.clone()), source_ml_source2_nominal_field29769.clone(), source_ml_source2_nominal_field29771.clone(), source_ml_source2_nominal_field29773.clone(), source_ml_source2_nominal_field29775.clone())
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
                                                    })))
                                                }
                                            }
                                            _ => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 102, 97, 109, 105, 108, 121]))
                                            }
                                        }
                                }
                            }
                        }
                    }
                }
            }
        },
    }
}
