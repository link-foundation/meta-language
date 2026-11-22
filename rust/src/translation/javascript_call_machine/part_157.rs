// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallArgumentList;
use super::JavaScriptCallFrame;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallIteratorOperation;
use super::JavaScriptCallIteratorRegion;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptIteratorRegionsUpdate;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::accept_java_script_call_argument_lists_update;
use super::accept_java_script_call_argument_rest;
use super::accept_java_script_call_array_iteration;
use super::accept_java_script_call_construction;
use super::accept_java_script_call_definition;
use super::accept_java_script_call_iterator_protocol;
use super::accept_java_script_call_key;
use super::accept_java_script_collection_completion;
use super::accept_java_script_collection_iterator_result_completion;
use super::advance_java_script_call_frame;
use super::append_java_script_call_argument;
use super::begin_java_script_argument_rest;
use super::begin_java_script_array_iterator_next;
use super::begin_java_script_collection_iterator_result;
use super::begin_java_script_construction;
use super::begin_java_script_property_definition;
use super::begin_java_script_property_key_conversion;
use super::begin_java_script_string_iterator_result;
use super::begin_java_script_synchronous_iterator;
use super::begin_java_script_synchronous_iterator_close;
use super::begin_java_script_synchronous_iterator_step;
use super::complete_java_script_property_definition_with_symbol;
use super::copy_java_script_collection;
use super::execute_java_script_call_primitive_instruction;
use super::execute_java_script_collection_operation;
use super::execute_java_script_ordinary_call_property;
use super::execute_java_script_primitive_method;
use super::execute_java_script_weak_collection_operation;
use super::find_java_script_call_argument_list;
use super::find_java_script_iterator_region;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_string;
use super::open_java_script_call_argument_list;
use super::push_java_script_call;
use super::push_java_script_collection_constructor;
use super::push_java_script_error_constructor;
use super::push_java_script_ordinary_class_constructor;
use super::read_java_script_call_iterator_close_completion;
use super::read_java_script_collection_iterator_result_prototypes_error;
use super::read_java_script_descriptor_operands;
use super::read_java_script_error_allocation_directory;
use super::read_java_script_weak_collection_operation_kind;
use super::remove_java_script_iterator_region;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::request_java_script_error_value;
use super::retain_java_script_call_resource_limit_machine;
use super::test_java_script_array_intrinsic_operation;
use super::test_java_script_array_value;
use super::test_java_script_call_strict_equal;
use super::test_java_script_semantic_same_value;
use super::write_java_script_call_iterator_result;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_invocation_instruction(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    instruction: self::JavaScriptCallInstruction,
    realm: self::ml::Big,
) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            _,
            _,
            shared_constructor_field23,
            shared_constructor_field24,
            shared_constructor_field25,
            _,
            _,
            _,
            _,
            _,
        ) => match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(
                shared_constructor_field21,
                shared_constructor_field22,
            ) => {
                let shared_constructor_field21 = *shared_constructor_field21;
                {
                    let next = self::advance_java_script_call_frame(frame.clone());
                    match next.clone() {
                        self::JavaScriptCallFrame::CallFrame(
                            shared_constructor_field11,
                            shared_constructor_field12,
                            shared_constructor_field13,
                            shared_constructor_field14,
                            shared_constructor_field15,
                            shared_constructor_field16,
                            shared_constructor_field17,
                            shared_constructor_field18,
                            shared_constructor_field19,
                            shared_constructor_field20,
                        ) => {
                            let shared_constructor_field16 = *shared_constructor_field16;
                            {
                                let current = self::replace_java_script_call_frame(
                                    shared_constructor_field21.clone(),
                                    shared_constructor_field22.clone(),
                                    next.clone(),
                                );
                                {
                                    let registers = shared_constructor_field23.clone();
                                    match instruction.clone() {
                                            self::JavaScriptCallInstruction::Call(destination, callee, receiver, arguments) => {
                                                {
                                                    let arguments_ = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values1660 = { let ml_reference_value1661 = arguments.borrow().clone(); ml_reference_value1661 }; let ml_callback1660 = { let registers = registers.clone(); std::rc::Rc::new(move |index: f64| -> self::JavaScriptSemanticValue { ({
                                                        let source_ml_source2_array_read_value29782 = registers.clone();
                                                        {
                                                            let source_ml_source2_array_read_index29783 = index;
                                                            { let ml_reference_array1662 = source_ml_source2_array_read_value29782.clone(); let ml_reference_position1662 = source_ml_source2_array_read_index29783; let ml_reference_projection1662 = self::ml_array::optional_at(&*ml_reference_array1662.borrow(), self::ml_array::number_index(ml_reference_position1662)); ml_reference_projection1662 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values1660.iter().cloned().enumerate().map(|(ml_index1660, ml_value1660)| ml_callback1660(ml_value1660)).collect::<Vec<_>>() }));
                                                    self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field21.clone(), shared_constructor_field22.clone(), {
                                                        let source_ml_source2_nominal_field29787 = shared_constructor_field11.clone();
                                                        {
                                                            let source_ml_source2_nominal_field29789 = shared_constructor_field12;
                                                            {
                                                                let source_ml_source2_nominal_field29791 = shared_constructor_field13.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field29793 = shared_constructor_field14.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field29795 = shared_constructor_field15.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field29797 = shared_constructor_field16.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field29799 = shared_constructor_field17.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field29801 = shared_constructor_field18.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field29803 = shared_constructor_field19.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field29805 = shared_constructor_field20.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override29807 = self::JavaScriptCallPending::WaitCall(destination);
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field29787.clone(), source_ml_source2_nominal_field29789, source_ml_source2_nominal_field29791.clone(), source_ml_source2_nominal_field29793.clone(), source_ml_source2_nominal_field29795.clone(), Box::new(source_ml_source2_nominal_override29807.clone()), source_ml_source2_nominal_field29799.clone(), source_ml_source2_nominal_field29801.clone(), source_ml_source2_nominal_field29803.clone(), source_ml_source2_nominal_field29805.clone())
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
                                                    }), ({ let ml_reference_array1663 = registers.clone(); let ml_reference_position1663 = callee; let ml_reference_projection1663 = self::ml_array::optional_at(&*ml_reference_array1663.borrow(), self::ml_array::number_index(ml_reference_position1663)); ml_reference_projection1663 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1664 = registers.clone(); let ml_reference_position1664 = receiver; let ml_reference_projection1664 = self::ml_array::optional_at(&*ml_reference_array1664.borrow(), self::ml_array::number_index(ml_reference_position1664)); ml_reference_projection1664 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), arguments_.clone())
                                                }
                                            }
                                            self::JavaScriptCallInstruction::ArgumentListOpen(slot) => {
                                                self::accept_java_script_call_argument_lists_update(machine.clone(), next.clone(), self::open_java_script_call_argument_list(shared_constructor_field21.clone(), (match frame.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, source_ml_source2_data_property_value29813, _, _) => {
                                                        ((source_ml_source2_data_property_value29813.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>>,))).0).clone()
                                                    }
                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallArgumentList>::new()))), slot, { let ml_reference_array1665 = shared_constructor_field25.clone(); let ml_reference_projection1665 = ml_reference_array1665.borrow().len() as f64; ml_reference_projection1665 }))
                                            }
                                            self::JavaScriptCallInstruction::ArgumentListAppend(slot_2, value) => {
                                                self::accept_java_script_call_argument_lists_update(machine.clone(), next.clone(), self::append_java_script_call_argument(shared_constructor_field21.clone(), (match frame.clone() {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, source_ml_source2_data_property_value29816, _, _) => {
                                                        ((source_ml_source2_data_property_value29816.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>>,))).0).clone()
                                                    }
                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallArgumentList>::new()))), slot_2, { let ml_reference_array1666 = shared_constructor_field25.clone(); let ml_reference_projection1666 = ml_reference_array1666.borrow().len() as f64; ml_reference_projection1666 }, ({ let ml_reference_array1667 = registers.clone(); let ml_reference_position1667 = value; let ml_reference_projection1667 = self::ml_array::optional_at(&*ml_reference_array1667.borrow(), self::ml_array::number_index(ml_reference_position1667)); ml_reference_projection1667 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                            }
                                            self::JavaScriptCallInstruction::ArgumentListCall(destination_2, slot_3, instruction_callee, instruction_receiver, mode) => {
                                                {
                                                    let lists = (match frame.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, source_ml_source2_data_property_value29820, _, _) => {
                                                            ((source_ml_source2_data_property_value29820.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>>,))).0).clone()
                                                        }
                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallArgumentList>::new())));
                                                    {
                                                        let at = self::find_java_script_call_argument_list(lists.clone(), slot_3);
                                                        {
                                                            let list = { let ml_reference_array1668 = lists.clone(); let ml_reference_position1668 = at; let ml_reference_projection1668 = self::ml_array::optional_at(&*ml_reference_array1668.borrow(), self::ml_array::number_index(ml_reference_position1668)); ml_reference_projection1668 };
                                                            if match (list.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field21.clone()), self::ml_string::Text::from_units(vec![97, 32, 118, 97, 114, 105, 97, 100, 105, 99, 32, 67, 97, 108, 108, 32, 99, 111, 110, 115, 117, 109, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 110, 101, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 32, 101, 120, 97, 99, 116, 108, 121, 32, 111, 110, 99, 101]))
                                                            } else {
                                                                {
                                                                    let caller = self::replace_java_script_call_frame(shared_constructor_field21.clone(), shared_constructor_field22.clone(), {
                                                                        let source_ml_source2_nominal_spread29826 = next.clone();
                                                                        match source_ml_source2_nominal_spread29826.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                                                let shared_constructor_field6 = *shared_constructor_field6;
                                                                                {
                                                                                    let source_ml_source2_nominal_field29827 = shared_constructor_field1.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field29829 = shared_constructor_field2;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field29831 = shared_constructor_field3.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field29833 = shared_constructor_field4.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field29835 = shared_constructor_field5.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field29837 = shared_constructor_field6.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field29839 = shared_constructor_field7.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field29841 = shared_constructor_field8.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field29843 = shared_constructor_field9.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field29845 = shared_constructor_field10.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override29847 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values1669 = { let ml_reference_value1670 = lists.borrow().clone(); ml_reference_value1670 }; let ml_callback1669 = { let at = at; std::rc::Rc::new(move |row: self::JavaScriptCallArgumentList, index: f64| -> bool { (index != at) }) }; ml_values1669.iter().cloned().enumerate().filter_map(|(ml_index1669, ml_value1669)| if ml_callback1669(ml_value1669.clone(), (ml_index1669 as f64)) { Some(ml_value1669) } else { None }).collect::<Vec<_>>() }));
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override29848 = self::JavaScriptCallPending::WaitCall(destination_2);
                                                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field29827.clone(), source_ml_source2_nominal_field29829, source_ml_source2_nominal_field29831.clone(), source_ml_source2_nominal_field29833.clone(), source_ml_source2_nominal_field29835.clone(), Box::new(source_ml_source2_nominal_override29848.clone()), source_ml_source2_nominal_field29839.clone(), Some((Some(source_ml_source2_nominal_override29847.clone()),)), source_ml_source2_nominal_field29843.clone(), source_ml_source2_nominal_field29845.clone())
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
                                                                    });
                                                                    {
                                                                        let callee_2 = ({ let ml_reference_array1671 = registers.clone(); let ml_reference_position1671 = instruction_callee; let ml_reference_projection1671 = self::ml_array::optional_at(&*ml_reference_array1671.borrow(), self::ml_array::number_index(ml_reference_position1671)); ml_reference_projection1671 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                        {
                                                                            let receiver_2 = ({ let ml_reference_array1672 = registers.clone(); let ml_reference_position1672 = instruction_receiver; let ml_reference_projection1672 = self::ml_array::optional_at(&*ml_reference_array1672.borrow(), self::ml_array::number_index(ml_reference_position1672)); ml_reference_projection1672 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                            if (mode == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]))) {
                                                                                {
                                                                                    let directory = match program.clone() {
                                                                                        self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value29853, _) => {
                                                                                            ((source_ml_source2_data_property_value29853.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                                        }
                                                                                    };
                                                                                    if match (directory.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field21.clone()), self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 100, 105, 99, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                                                    } else {
                                                                                        self::accept_java_script_call_construction(program.clone(), caller.clone(), destination_2, realm.clone(), self::begin_java_script_construction(program.clone(), shared_constructor_field21.clone(), directory.clone().expect("a proven present value"), callee_2.clone(), receiver_2.clone(), match list.clone().expect("a proven present value") {
                                                                                            self::JavaScriptCallArgumentList::CallArgumentList(_, _, source_ml_source2_data_property_value29856) => {
                                                                                                source_ml_source2_data_property_value29856.clone()
                                                                                            }
                                                                                        }), Some(match list.clone().expect("a proven present value") {
                                                                                            self::JavaScriptCallArgumentList::CallArgumentList(_, _, source_ml_source2_data_property_value29857) => {
                                                                                                source_ml_source2_data_property_value29857.clone()
                                                                                            }
                                                                                        }))
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                {
                                                                                    let transition = if (mode == (self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110]))) {
                                                                                        self::push_java_script_collection_constructor(program.clone(), caller.clone(), callee_2.clone(), match list.clone().expect("a proven present value") {
                                                                                            self::JavaScriptCallArgumentList::CallArgumentList(_, _, source_ml_source2_data_property_value29858) => {
                                                                                                source_ml_source2_data_property_value29858.clone()
                                                                                            }
                                                                                        })
                                                                                    } else {
                                                                                        if (mode == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114, 32, 99, 108, 97, 115, 115]))) {
                                                                                            self::push_java_script_error_constructor(program.clone(), caller.clone(), callee_2.clone(), match list.clone().expect("a proven present value") {
                                                                                                self::JavaScriptCallArgumentList::CallArgumentList(_, _, source_ml_source2_data_property_value29859) => {
                                                                                                    source_ml_source2_data_property_value29859.clone()
                                                                                                }
                                                                                            })
                                                                                        } else {
                                                                                            if (mode == (self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]))) {
                                                                                                self::push_java_script_ordinary_class_constructor(program.clone(), caller.clone(), callee_2.clone(), receiver_2.clone(), match list.clone().expect("a proven present value") {
                                                                                                    self::JavaScriptCallArgumentList::CallArgumentList(_, _, source_ml_source2_data_property_value29860) => {
                                                                                                        source_ml_source2_data_property_value29860.clone()
                                                                                                    }
                                                                                                })
                                                                                            } else {
                                                                                                self::push_java_script_call(program.clone(), caller.clone(), callee_2.clone(), receiver_2.clone(), match list.clone().expect("a proven present value") {
                                                                                                    self::JavaScriptCallArgumentList::CallArgumentList(_, _, source_ml_source2_data_property_value29861) => {
                                                                                                        source_ml_source2_data_property_value29861.clone()
                                                                                                    }
                                                                                                })
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    self::retain_java_script_call_resource_limit_machine(machine.clone(), transition.clone())
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
                                            self::JavaScriptCallInstruction::ConstructErrorClass(destination_3, callee_3, arguments_2) => {
                                                self::push_java_script_error_constructor(program.clone(), self::replace_java_script_call_frame(shared_constructor_field21.clone(), shared_constructor_field22.clone(), {
                                                    let source_ml_source2_nominal_field29865 = shared_constructor_field11.clone();
                                                    {
                                                        let source_ml_source2_nominal_field29867 = shared_constructor_field12;
                                                        {
                                                            let source_ml_source2_nominal_field29869 = shared_constructor_field13.clone();
                                                            {
                                                                let source_ml_source2_nominal_field29871 = shared_constructor_field14.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field29873 = shared_constructor_field15.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field29875 = shared_constructor_field16.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field29877 = shared_constructor_field17.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field29879 = shared_constructor_field18.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field29881 = shared_constructor_field19.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field29883 = shared_constructor_field20.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override29885 = self::JavaScriptCallPending::WaitCall(destination_3);
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field29865.clone(), source_ml_source2_nominal_field29867, source_ml_source2_nominal_field29869.clone(), source_ml_source2_nominal_field29871.clone(), source_ml_source2_nominal_field29873.clone(), Box::new(source_ml_source2_nominal_override29885.clone()), source_ml_source2_nominal_field29877.clone(), source_ml_source2_nominal_field29879.clone(), source_ml_source2_nominal_field29881.clone(), source_ml_source2_nominal_field29883.clone())
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
                                                }), ({ let ml_reference_array1673 = registers.clone(); let ml_reference_position1673 = callee_3; let ml_reference_projection1673 = self::ml_array::optional_at(&*ml_reference_array1673.borrow(), self::ml_array::number_index(ml_reference_position1673)); ml_reference_projection1673 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values1674 = { let ml_reference_value1675 = arguments_2.borrow().clone(); ml_reference_value1675 }; let ml_callback1674 = { let registers = registers.clone(); std::rc::Rc::new(move |index: f64| -> self::JavaScriptSemanticValue { ({
                                                    let source_ml_source2_array_read_value29888 = registers.clone();
                                                    {
                                                        let source_ml_source2_array_read_index29889 = index;
                                                        { let ml_reference_array1676 = source_ml_source2_array_read_value29888.clone(); let ml_reference_position1676 = source_ml_source2_array_read_index29889; let ml_reference_projection1676 = self::ml_array::optional_at(&*ml_reference_array1676.borrow(), self::ml_array::number_index(ml_reference_position1676)); ml_reference_projection1676 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values1674.iter().cloned().enumerate().map(|(ml_index1674, ml_value1674)| ml_callback1674(ml_value1674)).collect::<Vec<_>>() })))
                                            }
                                            self::JavaScriptCallInstruction::CallClassConstructor(destination_4, callee_4, receiver_3, arguments_3) => {
                                                self::push_java_script_ordinary_class_constructor(program.clone(), self::replace_java_script_call_frame(shared_constructor_field21.clone(), shared_constructor_field22.clone(), {
                                                    let source_ml_source2_nominal_field29893 = shared_constructor_field11.clone();
                                                    {
                                                        let source_ml_source2_nominal_field29895 = shared_constructor_field12;
                                                        {
                                                            let source_ml_source2_nominal_field29897 = shared_constructor_field13.clone();
                                                            {
                                                                let source_ml_source2_nominal_field29899 = shared_constructor_field14.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field29901 = shared_constructor_field15.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field29903 = shared_constructor_field16.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field29905 = shared_constructor_field17.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field29907 = shared_constructor_field18.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field29909 = shared_constructor_field19.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field29911 = shared_constructor_field20.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override29913 = self::JavaScriptCallPending::WaitCall(destination_4);
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field29893.clone(), source_ml_source2_nominal_field29895, source_ml_source2_nominal_field29897.clone(), source_ml_source2_nominal_field29899.clone(), source_ml_source2_nominal_field29901.clone(), Box::new(source_ml_source2_nominal_override29913.clone()), source_ml_source2_nominal_field29905.clone(), source_ml_source2_nominal_field29907.clone(), source_ml_source2_nominal_field29909.clone(), source_ml_source2_nominal_field29911.clone())
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
                                                }), ({ let ml_reference_array1677 = registers.clone(); let ml_reference_position1677 = callee_4; let ml_reference_projection1677 = self::ml_array::optional_at(&*ml_reference_array1677.borrow(), self::ml_array::number_index(ml_reference_position1677)); ml_reference_projection1677 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1678 = registers.clone(); let ml_reference_position1678 = receiver_3; let ml_reference_projection1678 = self::ml_array::optional_at(&*ml_reference_array1678.borrow(), self::ml_array::number_index(ml_reference_position1678)); ml_reference_projection1678 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values1679 = { let ml_reference_value1680 = arguments_3.borrow().clone(); ml_reference_value1680 }; let ml_callback1679 = { let registers = registers.clone(); std::rc::Rc::new(move |index: f64| -> self::JavaScriptSemanticValue { ({
                                                    let source_ml_source2_array_read_value29918 = registers.clone();
                                                    {
                                                        let source_ml_source2_array_read_index29919 = index;
                                                        { let ml_reference_array1681 = source_ml_source2_array_read_value29918.clone(); let ml_reference_position1681 = source_ml_source2_array_read_index29919; let ml_reference_projection1681 = self::ml_array::optional_at(&*ml_reference_array1681.borrow(), self::ml_array::number_index(ml_reference_position1681)); ml_reference_projection1681 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values1679.iter().cloned().enumerate().map(|(ml_index1679, ml_value1679)| ml_callback1679(ml_value1679)).collect::<Vec<_>>() })))
                                            }
                                            self::JavaScriptCallInstruction::ConstructCollection(destination_5, callee_5, arguments_4) => {
                                                self::push_java_script_collection_constructor(program.clone(), self::replace_java_script_call_frame(shared_constructor_field21.clone(), shared_constructor_field22.clone(), {
                                                    let source_ml_source2_nominal_field29923 = shared_constructor_field11.clone();
                                                    {
                                                        let source_ml_source2_nominal_field29925 = shared_constructor_field12;
                                                        {
                                                            let source_ml_source2_nominal_field29927 = shared_constructor_field13.clone();
                                                            {
                                                                let source_ml_source2_nominal_field29929 = shared_constructor_field14.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field29931 = shared_constructor_field15.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field29933 = shared_constructor_field16.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field29935 = shared_constructor_field17.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field29937 = shared_constructor_field18.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field29939 = shared_constructor_field19.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field29941 = shared_constructor_field20.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override29943 = self::JavaScriptCallPending::WaitCall(destination_5);
                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field29923.clone(), source_ml_source2_nominal_field29925, source_ml_source2_nominal_field29927.clone(), source_ml_source2_nominal_field29929.clone(), source_ml_source2_nominal_field29931.clone(), Box::new(source_ml_source2_nominal_override29943.clone()), source_ml_source2_nominal_field29935.clone(), source_ml_source2_nominal_field29937.clone(), source_ml_source2_nominal_field29939.clone(), source_ml_source2_nominal_field29941.clone())
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
                                                }), ({ let ml_reference_array1682 = registers.clone(); let ml_reference_position1682 = callee_5; let ml_reference_projection1682 = self::ml_array::optional_at(&*ml_reference_array1682.borrow(), self::ml_array::number_index(ml_reference_position1682)); ml_reference_projection1682 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), std::rc::Rc::new(std::cell::RefCell::new({ let ml_values1683 = { let ml_reference_value1684 = arguments_4.borrow().clone(); ml_reference_value1684 }; let ml_callback1683 = { let registers = registers.clone(); std::rc::Rc::new(move |index: f64| -> self::JavaScriptSemanticValue { ({
                                                    let source_ml_source2_array_read_value29946 = registers.clone();
                                                    {
                                                        let source_ml_source2_array_read_index29947 = index;
                                                        { let ml_reference_array1685 = source_ml_source2_array_read_value29946.clone(); let ml_reference_position1685 = source_ml_source2_array_read_index29947; let ml_reference_projection1685 = self::ml_array::optional_at(&*ml_reference_array1685.borrow(), self::ml_array::number_index(ml_reference_position1685)); ml_reference_projection1685 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values1683.iter().cloned().enumerate().map(|(ml_index1683, ml_value1683)| ml_callback1683(ml_value1683)).collect::<Vec<_>>() })))
                                            }
                                            self::JavaScriptCallInstruction::ArgumentRest(destination_6, prototype, from, realm_2) => {
                                                self::accept_java_script_call_argument_rest(current.clone(), destination_6, realm_2.clone(), self::begin_java_script_argument_rest(shared_constructor_field21.clone(), shared_constructor_field24.clone(), from, ({ let ml_reference_array1686 = registers.clone(); let ml_reference_position1686 = prototype; let ml_reference_projection1686 = self::ml_array::optional_at(&*ml_reference_array1686.borrow(), self::ml_array::number_index(ml_reference_position1686)); ml_reference_projection1686 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                            }
                                            self::JavaScriptCallInstruction::IteratorProtocol(destination_7, done, input, iterator_key, slot_4, operation, realm_3, read_value) => {
                                                {
                                                    let regions = (match frame.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, source_ml_source2_data_property_value29952, _, _, _) => {
                                                            ((source_ml_source2_data_property_value29952.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,))).0).clone()
                                                        }
                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallIteratorRegion>::new())));
                                                    {
                                                        let region = {
                                                            let source_ml_source2_array_read_index29954 = self::find_java_script_iterator_region(regions.clone(), slot_4);
                                                            { let ml_reference_array1687 = regions.clone(); let ml_reference_position1687 = source_ml_source2_array_read_index29954; let ml_reference_projection1687 = self::ml_array::optional_at(&*ml_reference_array1687.borrow(), self::ml_array::number_index(ml_reference_position1687)); ml_reference_projection1687 }
                                                        };
                                                        {
                                                            let pending = self::JavaScriptCallIteratorOperation::IteratorOperation(destination_7, done, slot_4, operation.clone(), realm_3.clone());
                                                            if (operation == (self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]))) {
                                                                if !(match (region.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field21.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 111, 112, 101, 110, 105, 110, 103, 32, 99, 97, 110, 110, 111, 116, 32, 111, 118, 101, 114, 119, 114, 105, 116, 101, 32, 97, 110, 32, 97, 99, 116, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 103, 105, 111, 110]))
                                                                } else {
                                                                    if (({ let ml_reference_array1688 = regions.clone(); let ml_reference_projection1688 = ml_reference_array1688.borrow().len() as f64; ml_reference_projection1688 }) >= (128f64)) {
                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 111, 112, 101, 110, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 97, 109, 101, 32, 114, 101, 103, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::accept_java_script_call_iterator_protocol(program.clone(), current.clone(), Some(pending.clone()), self::begin_java_script_synchronous_iterator(shared_constructor_field21.clone(), ({ let ml_reference_array1689 = registers.clone(); let ml_reference_position1689 = input; let ml_reference_projection1689 = self::ml_array::optional_at(&*ml_reference_array1689.borrow(), self::ml_array::number_index(ml_reference_position1689)); ml_reference_projection1689 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1690 = registers.clone(); let ml_reference_position1690 = iterator_key; let ml_reference_projection1690 = self::ml_array::optional_at(&*ml_reference_array1690.borrow(), self::ml_array::number_index(ml_reference_position1690)); ml_reference_projection1690 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                                                    }
                                                                }
                                                            } else {
                                                                if match (region.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field21.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 110, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 103, 105, 111, 110]))
                                                                } else {
                                                                    if (operation == (self::ml_string::Text::from_units(vec![115, 116, 101, 112]))) {
                                                                        self::accept_java_script_call_iterator_protocol(program.clone(), current.clone(), Some(pending.clone()), self::begin_java_script_synchronous_iterator_step(shared_constructor_field21.clone(), match region.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallIteratorRegion::CallIteratorRegion(_, source_ml_source2_data_property_value29963) => {
                                                                                let source_ml_source2_data_property_value29963 = *source_ml_source2_data_property_value29963;
                                                                                source_ml_source2_data_property_value29963.clone()
                                                                            }
                                                                        }, (((read_value.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()).clone().unwrap_or_else(|| true)))
                                                                    } else {
                                                                        if match match region.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallIteratorRegion::CallIteratorRegion(_, source_ml_source2_data_property_value29964) => {
                                                                                let source_ml_source2_data_property_value29964 = *source_ml_source2_data_property_value29964;
                                                                                source_ml_source2_data_property_value29964.clone()
                                                                            }
                                                                        } {
                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value29965) => {
                                                                                source_ml_source2_data_property_value29965
                                                                            }
                                                                        } {
                                                                            {
                                                                                let removed = self::remove_java_script_iterator_region(shared_constructor_field21.clone(), regions.clone(), slot_4);
                                                                                match removed.clone() {
                                                                                    self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions_2) => {
                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field21.clone(), shared_constructor_field22.clone(), self::write_java_script_call_iterator_result({
                                                                                            let source_ml_source2_nominal_field29970 = shared_constructor_field11.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field29972 = shared_constructor_field12;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field29974 = shared_constructor_field13.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field29976 = shared_constructor_field14.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field29978 = shared_constructor_field15.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field29980 = shared_constructor_field16.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field29982 = shared_constructor_field17.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field29984 = shared_constructor_field18.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field29986 = shared_constructor_field19.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field29988 = shared_constructor_field20.clone();
                                                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field29970.clone(), source_ml_source2_nominal_field29972, source_ml_source2_nominal_field29974.clone(), source_ml_source2_nominal_field29976.clone(), source_ml_source2_nominal_field29978.clone(), Box::new(source_ml_source2_nominal_field29980.clone()), Some((Some(regions_2.clone()),)), source_ml_source2_nominal_field29984.clone(), source_ml_source2_nominal_field29986.clone(), source_ml_source2_nominal_field29988.clone())
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }, destination_7, done, self::JavaScriptSemanticValue::Undefined, true))))
                                                                                    }
                                                                                    _ => {
                                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field21.clone()), self::ml_string::Text::from_units(vec![101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 108, 101, 97, 115, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 111, 110]))
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::accept_java_script_call_iterator_protocol(program.clone(), current.clone(), Some(pending.clone()), self::begin_java_script_synchronous_iterator_close(shared_constructor_field21.clone(), match region.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallIteratorRegion::CallIteratorRegion(_, source_ml_source2_data_property_value29993) => {
                                                                                    let source_ml_source2_data_property_value29993 = *source_ml_source2_data_property_value29993;
                                                                                    source_ml_source2_data_property_value29993.clone()
                                                                                }
                                                                            }, self::read_java_script_call_iterator_close_completion(frame.clone())))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptCallInstruction::CollectionIteratorResult(destination_8, kind, array_prototype, object_prototype) => {
                                                if (kind == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) {
                                                    self::accept_java_script_collection_iterator_result_completion(program.clone(), machine.clone(), frame.clone(), destination_8, realm.clone(), self::begin_java_script_string_iterator_result(shared_constructor_field21.clone(), ({
                                                        let source_ml_source2_array_read_index29996 = 0f64;
                                                        { let ml_reference_array1691 = registers.clone(); let ml_reference_position1691 = source_ml_source2_array_read_index29996; let ml_reference_projection1691 = self::ml_array::optional_at(&*ml_reference_array1691.borrow(), self::ml_array::number_index(ml_reference_position1691)); ml_reference_projection1691 }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1692 = registers.clone(); let ml_reference_position1692 = array_prototype; let ml_reference_projection1692 = self::ml_array::optional_at(&*ml_reference_array1692.borrow(), self::ml_array::number_index(ml_reference_position1692)); ml_reference_projection1692 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1693 = registers.clone(); let ml_reference_position1693 = object_prototype; let ml_reference_projection1693 = self::ml_array::optional_at(&*ml_reference_array1693.borrow(), self::ml_array::number_index(ml_reference_position1693)); ml_reference_projection1693 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                                } else {
                                                    if (kind == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) {
                                                        {
                                                            let error = self::read_java_script_collection_iterator_result_prototypes_error(shared_constructor_field21.clone(), ({ let ml_reference_array1694 = registers.clone(); let ml_reference_position1694 = array_prototype; let ml_reference_projection1694 = self::ml_array::optional_at(&*ml_reference_array1694.borrow(), self::ml_array::number_index(ml_reference_position1694)); ml_reference_projection1694 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1695 = registers.clone(); let ml_reference_position1695 = object_prototype; let ml_reference_projection1695 = self::ml_array::optional_at(&*ml_reference_array1695.borrow(), self::ml_array::number_index(ml_reference_position1695)); ml_reference_projection1695 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field21.clone()), error.clone())
                                                            } else {
                                                                self::accept_java_script_call_array_iteration(program.clone(), machine.clone(), self::begin_java_script_array_iterator_next(shared_constructor_field21.clone(), ({
                                                                    let source_ml_source2_array_read_index30009 = 0f64;
                                                                    { let ml_reference_array1696 = registers.clone(); let ml_reference_position1696 = source_ml_source2_array_read_index30009; let ml_reference_projection1696 = self::ml_array::optional_at(&*ml_reference_array1696.borrow(), self::ml_array::number_index(ml_reference_position1696)); ml_reference_projection1696 }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), match program.clone() {
                                                                    self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value30010, _, _, _) => {
                                                                        let source_ml_source2_data_property_value30010 = *source_ml_source2_data_property_value30010;
                                                                        source_ml_source2_data_property_value30010.clone()
                                                                    }
                                                                }))
                                                            }
                                                        }
                                                    } else {
                                                        self::accept_java_script_collection_iterator_result_completion(program.clone(), machine.clone(), frame.clone(), destination_8, realm.clone(), self::begin_java_script_collection_iterator_result(shared_constructor_field21.clone(), ({
                                                            let source_ml_source2_array_read_index30013 = 0f64;
                                                            { let ml_reference_array1697 = registers.clone(); let ml_reference_position1697 = source_ml_source2_array_read_index30013; let ml_reference_projection1697 = self::ml_array::optional_at(&*ml_reference_array1697.borrow(), self::ml_array::number_index(ml_reference_position1697)); ml_reference_projection1697 }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), &(kind), ({ let ml_reference_array1698 = registers.clone(); let ml_reference_position1698 = array_prototype; let ml_reference_projection1698 = self::ml_array::optional_at(&*ml_reference_array1698.borrow(), self::ml_array::number_index(ml_reference_position1698)); ml_reference_projection1698 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1699 = registers.clone(); let ml_reference_position1699 = object_prototype; let ml_reference_projection1699 = self::ml_array::optional_at(&*ml_reference_array1699.borrow(), self::ml_array::number_index(ml_reference_position1699)); ml_reference_projection1699 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                                    }
                                                }
                                            }
                                            self::JavaScriptCallInstruction::CollectionOperation(destination_9, operation_2) => {
                                                {
                                                    let receiver_4 = ({
                                                        let source_ml_source2_array_read_index30019 = 0f64;
                                                        { let ml_reference_array1700 = registers.clone(); let ml_reference_position1700 = source_ml_source2_array_read_index30019; let ml_reference_projection1700 = self::ml_array::optional_at(&*ml_reference_array1700.borrow(), self::ml_array::number_index(ml_reference_position1700)); ml_reference_projection1700 }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                    if ((self::read_java_script_weak_collection_operation_kind(&(operation_2))) != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::accept_java_script_collection_completion(program.clone(), machine.clone(), frame.clone(), destination_9, self::execute_java_script_weak_collection_operation(shared_constructor_field21.clone(), receiver_4.clone(), &(operation_2), shared_constructor_field24.clone()))
                                                    } else {
                                                        self::accept_java_script_collection_completion(program.clone(), machine.clone(), frame.clone(), destination_9, self::execute_java_script_collection_operation(shared_constructor_field21.clone(), receiver_4.clone(), &(operation_2), shared_constructor_field24.clone()))
                                                    }
                                                }
                                            }
                                            self::JavaScriptCallInstruction::PrimitiveMethod(destination_10, operation_3) => {
                                                if self::test_java_script_array_intrinsic_operation(&(operation_3)) {
                                                    {
                                                        let result = self::test_java_script_array_value(shared_constructor_field21.clone(), ({
                                                            let source_ml_source2_array_read_value30026 = shared_constructor_field24.clone();
                                                            {
                                                                let source_ml_source2_array_read_index30027 = 0f64;
                                                                { let ml_reference_array1701 = source_ml_source2_array_read_value30026.clone(); let ml_reference_position1701 = source_ml_source2_array_read_index30027; let ml_reference_projection1701 = self::ml_array::optional_at(&*ml_reference_array1701.borrow(), self::ml_array::number_index(ml_reference_position1701)); ml_reference_projection1701 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                        if (((({ let ml_reference_value1702 = result.borrow().clone(); ml_reference_value1702 }).0).1) != (self::ml_string::Text::from_units(vec![]))) {
                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field21.clone()), (({ let ml_reference_value1703 = result.borrow().clone(); ml_reference_value1703 }).0).1)
                                                        } else {
                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field21.clone(), shared_constructor_field22.clone(), self::write_java_script_call_register(next.clone(), destination_10, self::JavaScriptSemanticValue::Boolean((({ let ml_reference_value1704 = result.borrow().clone(); ml_reference_value1704 }).0).0)))))
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let completion = self::execute_java_script_primitive_method(({
                                                            let source_ml_source2_array_read_index30032 = 0f64;
                                                            { let ml_reference_array1705 = registers.clone(); let ml_reference_position1705 = source_ml_source2_array_read_index30032; let ml_reference_projection1705 = self::ml_array::optional_at(&*ml_reference_array1705.borrow(), self::ml_array::number_index(ml_reference_position1705)); ml_reference_projection1705 }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), &(operation_3), shared_constructor_field24.clone());
                                                        match completion.clone() {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_2) => {
                                                                let value_2 = *value_2;
                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field21.clone(), shared_constructor_field22.clone(), self::write_java_script_call_register(next.clone(), destination_10, value_2.clone()))))
                                                            }
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(reason) => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field21.clone()), reason.clone())
                                                            }
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveError(error_class, reason_2) => {
                                                                self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason_2))
                                                            }
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(reason_3) => {
                                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_3.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field21.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 102, 97, 109, 105, 108, 121]))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_property_instruction(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    instruction: self::JavaScriptCallInstruction,
    realm: self::ml::Big,
    function_identity: self::ml::Big,
) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            _,
            _,
            shared_constructor_field14,
            shared_constructor_field15,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(
                shared_constructor_field12,
                shared_constructor_field13,
            ) => {
                let shared_constructor_field12 = *shared_constructor_field12;
                match program.clone() {
                    self::JavaScriptCallProgram::CallProgram(
                        _,
                        _,
                        shared_constructor_field11,
                        _,
                        _,
                        _,
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
                                            shared_constructor_field12.clone(),
                                            shared_constructor_field13.clone(),
                                            next.clone(),
                                        );
                                        {
                                            let registers = shared_constructor_field14.clone();
                                            match instruction.clone() {
                                                    self::JavaScriptCallInstruction::CollectionAllocate(_, _, _) => {
                                                        self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone(), instruction.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, _, _, _, _) => {
                                                        self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone(), instruction.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::CollectionCopy(destination, target, input, adder, iterator_key, iterator_method, iterator_prototype, next_method) => {
                                                        self::accept_java_script_collection_completion(program.clone(), machine.clone(), frame.clone(), destination, self::copy_java_script_collection(shared_constructor_field12.clone(), ({ let ml_reference_array1706 = registers.clone(); let ml_reference_position1706 = target; let ml_reference_projection1706 = self::ml_array::optional_at(&*ml_reference_array1706.borrow(), self::ml_array::number_index(ml_reference_position1706)); ml_reference_projection1706 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1707 = registers.clone(); let ml_reference_position1707 = input; let ml_reference_projection1707 = self::ml_array::optional_at(&*ml_reference_array1707.borrow(), self::ml_array::number_index(ml_reference_position1707)); ml_reference_projection1707 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1708 = registers.clone(); let ml_reference_position1708 = adder; let ml_reference_projection1708 = self::ml_array::optional_at(&*ml_reference_array1708.borrow(), self::ml_array::number_index(ml_reference_position1708)); ml_reference_projection1708 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1709 = registers.clone(); let ml_reference_position1709 = iterator_key; let ml_reference_projection1709 = self::ml_array::optional_at(&*ml_reference_array1709.borrow(), self::ml_array::number_index(ml_reference_position1709)); ml_reference_projection1709 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1710 = registers.clone(); let ml_reference_position1710 = iterator_method; let ml_reference_projection1710 = self::ml_array::optional_at(&*ml_reference_array1710.borrow(), self::ml_array::number_index(ml_reference_position1710)); ml_reference_projection1710 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1711 = registers.clone(); let ml_reference_position1711 = iterator_prototype; let ml_reference_projection1711 = self::ml_array::optional_at(&*ml_reference_array1711.borrow(), self::ml_array::number_index(ml_reference_position1711)); ml_reference_projection1711 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1712 = registers.clone(); let ml_reference_position1712 = next_method; let ml_reference_projection1712 = self::ml_array::optional_at(&*ml_reference_array1712.borrow(), self::ml_array::number_index(ml_reference_position1712)); ml_reference_projection1712 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))
                                                    }
                                                    self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, _, _, _, _) => {
                                                        self::request_java_script_error_value(program.clone(), machine.clone(), frame.clone(), instruction.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::ReadDerivedReceiver(destination_2) => {
                                                        {
                                                            let receiver = ({
                                                                let source_ml_source2_array_read_index30103 = 0f64;
                                                                { let ml_reference_array1713 = registers.clone(); let ml_reference_position1713 = source_ml_source2_array_read_index30103; let ml_reference_projection1713 = self::ml_array::optional_at(&*ml_reference_array1713.borrow(), self::ml_array::number_index(ml_reference_position1713)); ml_reference_projection1713 }
                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                            match receiver.clone() {
                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                    self::request_java_script_call_error(current.clone(), realm.clone(), &(self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 116, 104, 105, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 115, 117, 112, 101, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110])))
                                                                }
                                                                _ => {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field12.clone(), shared_constructor_field13.clone(), self::write_java_script_call_register(next.clone(), destination_2, receiver.clone()))))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptCallInstruction::InvokeErrorIntrinsic(destination_3) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvokeErrorIntrinsic(Box::new(self::replace_java_script_call_frame(shared_constructor_field12.clone(), shared_constructor_field13.clone(), {
                                                            let source_ml_source2_nominal_field30109 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field30111 = shared_constructor_field2;
                                                                {
                                                                    let source_ml_source2_nominal_field30113 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field30115 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field30117 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field30119 = shared_constructor_field6.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field30121 = shared_constructor_field7.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field30123 = shared_constructor_field8.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field30125 = shared_constructor_field9.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field30127 = shared_constructor_field10.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override30129 = self::JavaScriptCallPending::WaitErrorIntrinsic(destination_3);
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field30109.clone(), source_ml_source2_nominal_field30111, source_ml_source2_nominal_field30113.clone(), source_ml_source2_nominal_field30115.clone(), source_ml_source2_nominal_field30117.clone(), Box::new(source_ml_source2_nominal_override30129.clone()), source_ml_source2_nominal_field30121.clone(), source_ml_source2_nominal_field30123.clone(), source_ml_source2_nominal_field30125.clone(), source_ml_source2_nominal_field30127.clone())
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
                                                        })), realm.clone(), Box::new(self::JavaScriptSemanticValue::Object(function_identity.clone(), true)), Box::new(({
                                                            let source_ml_source2_array_read_index30131 = 0f64;
                                                            { let ml_reference_array1714 = registers.clone(); let ml_reference_position1714 = source_ml_source2_array_read_index30131; let ml_reference_projection1714 = self::ml_array::optional_at(&*ml_reference_array1714.borrow(), self::ml_array::number_index(ml_reference_position1714)); ml_reference_projection1714 }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)), shared_constructor_field15.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::InvokeHostCapability(destination_4) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvokeHostCapability(Box::new(self::replace_java_script_call_frame(shared_constructor_field12.clone(), shared_constructor_field13.clone(), {
                                                            let source_ml_source2_nominal_field30137 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field30139 = shared_constructor_field2;
                                                                {
                                                                    let source_ml_source2_nominal_field30141 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field30143 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field30145 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field30147 = shared_constructor_field6.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field30149 = shared_constructor_field7.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field30151 = shared_constructor_field8.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field30153 = shared_constructor_field9.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field30155 = shared_constructor_field10.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override30157 = self::JavaScriptCallPending::WaitHostCapability(destination_4, Box::new(self::read_java_script_error_allocation_directory(shared_constructor_field12.clone())));
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field30137.clone(), source_ml_source2_nominal_field30139, source_ml_source2_nominal_field30141.clone(), source_ml_source2_nominal_field30143.clone(), source_ml_source2_nominal_field30145.clone(), Box::new(source_ml_source2_nominal_override30157.clone()), source_ml_source2_nominal_field30149.clone(), source_ml_source2_nominal_field30151.clone(), source_ml_source2_nominal_field30153.clone(), source_ml_source2_nominal_field30155.clone())
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
                                                        })), realm.clone(), Box::new(self::JavaScriptSemanticValue::Object(function_identity.clone(), true)), Box::new(({
                                                            let source_ml_source2_array_read_index30159 = 0f64;
                                                            { let ml_reference_array1715 = registers.clone(); let ml_reference_position1715 = source_ml_source2_array_read_index30159; let ml_reference_projection1715 = self::ml_array::optional_at(&*ml_reference_array1715.borrow(), self::ml_array::number_index(ml_reference_position1715)); ml_reference_projection1715 }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)), shared_constructor_field15.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::NumberBinary(destination_5, instruction_left, instruction_right, operator) => {
                                                        {
                                                            let left = ({ let ml_reference_array1716 = registers.clone(); let ml_reference_position1716 = instruction_left; let ml_reference_projection1716 = self::ml_array::optional_at(&*ml_reference_array1716.borrow(), self::ml_array::number_index(ml_reference_position1716)); ml_reference_projection1716 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                            {
                                                                let right = ({ let ml_reference_array1717 = registers.clone(); let ml_reference_position1717 = instruction_right; let ml_reference_projection1717 = self::ml_array::optional_at(&*ml_reference_array1717.borrow(), self::ml_array::number_index(ml_reference_position1717)); ml_reference_projection1717 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                match left.clone() {
                                                                    self::JavaScriptSemanticValue::Number(left_value) => {
                                                                        match right.clone() {
                                                                            self::JavaScriptSemanticValue::Number(right_value) => {
                                                                                if ((operator == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) || (operator == (self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])))) {
                                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field12.clone(), shared_constructor_field13.clone(), self::write_java_script_call_register(next.clone(), destination_5, self::JavaScriptSemanticValue::Boolean(if (operator == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) {
                                                                                        (left_value < right_value)
                                                                                    } else {
                                                                                        (left_value == right_value)
                                                                                    })))))
                                                                                } else {
                                                                                    {
                                                                                        let value = if (operator == (self::ml_string::Text::from_units(vec![97, 100, 100]))) {
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
                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field12.clone(), shared_constructor_field13.clone(), self::write_java_script_call_register(next.clone(), destination_5, self::JavaScriptSemanticValue::Number(value)))))
                                                                                    }
                                                                                }
                                                                            }
                                                                            _ => {
                                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field12.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                            }
                                                                        }
                                                                    }
                                                                    _ => {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field12.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptCallInstruction::SameValue(destination_6, left_2, right_2) => {
                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field12.clone(), shared_constructor_field13.clone(), self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Boolean(self::test_java_script_semantic_same_value(({ let ml_reference_array1718 = registers.clone(); let ml_reference_position1718 = left_2; let ml_reference_projection1718 = self::ml_array::optional_at(&*ml_reference_array1718.borrow(), self::ml_array::number_index(ml_reference_position1718)); ml_reference_projection1718 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1719 = registers.clone(); let ml_reference_position1719 = right_2; let ml_reference_projection1719 = self::ml_array::optional_at(&*ml_reference_array1719.borrow(), self::ml_array::number_index(ml_reference_position1719)); ml_reference_projection1719 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                    }
                                                    self::JavaScriptCallInstruction::StrictEqual(destination_7, left_3, right_3) => {
                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field12.clone(), shared_constructor_field13.clone(), self::write_java_script_call_register(next.clone(), destination_7, self::JavaScriptSemanticValue::Boolean(self::test_java_script_call_strict_equal(({ let ml_reference_array1720 = registers.clone(); let ml_reference_position1720 = left_3; let ml_reference_projection1720 = self::ml_array::optional_at(&*ml_reference_array1720.borrow(), self::ml_array::number_index(ml_reference_position1720)); ml_reference_projection1720 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1721 = registers.clone(); let ml_reference_position1721 = right_3; let ml_reference_projection1721 = self::ml_array::optional_at(&*ml_reference_array1721.borrow(), self::ml_array::number_index(ml_reference_position1721)); ml_reference_projection1721 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                    }
                                                    self::JavaScriptCallInstruction::RequireWrite(_) => {
                                                        self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone(), instruction.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::Primitive(_, _, _) => {
                                                        self::execute_java_script_call_primitive_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::OrdinaryAllocate(_, _) => {
                                                        self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone(), instruction.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::ArrayAllocate(_, _, _) => {
                                                        self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone(), instruction.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::ClosureAllocate(_, _, _, _, _) => {
                                                        self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone(), instruction.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                                        self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone(), instruction.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::OrdinaryDelete(_, _, _) => {
                                                        self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone(), instruction.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::OrdinaryHas(_, _, _) => {
                                                        self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone(), instruction.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                                        self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone(), instruction.clone())
                                                    }
                                                    self::JavaScriptCallInstruction::OrdinaryDefine(destination_8, target_2, key, fields, realm_2) => {
                                                        {
                                                            let decoded = self::read_java_script_descriptor_operands(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values1722 = { let ml_reference_value1723 = fields.borrow().clone(); ml_reference_value1723 }; let ml_callback1722 = { let registers = registers.clone(); std::rc::Rc::new(move |coordinate: f64| -> self::JavaScriptSemanticValue { ({
                                                                let source_ml_source2_array_read_value30183 = registers.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index30184 = coordinate;
                                                                    { let ml_reference_array1724 = source_ml_source2_array_read_value30183.clone(); let ml_reference_position1724 = source_ml_source2_array_read_index30184; let ml_reference_projection1724 = self::ml_array::optional_at(&*ml_reference_array1724.borrow(), self::ml_array::number_index(ml_reference_position1724)); ml_reference_projection1724 }
                                                                }
                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values1722.iter().cloned().enumerate().map(|(ml_index1722, ml_value1722)| ml_callback1722(ml_value1722)).collect::<Vec<_>>() })));
                                                            if (((({ let ml_reference_value1725 = decoded.borrow().clone(); ml_reference_value1725 }).0).1) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field12.clone()), (({ let ml_reference_value1726 = decoded.borrow().clone(); ml_reference_value1726 }).0).1)
                                                            } else {
                                                                self::accept_java_script_call_definition(program.clone(), current.clone(), destination_8, realm_2.clone(), self::complete_java_script_property_definition_with_symbol(shared_constructor_field12.clone(), ({ let ml_reference_array1727 = registers.clone(); let ml_reference_position1727 = target_2; let ml_reference_projection1727 = self::ml_array::optional_at(&*ml_reference_array1727.borrow(), self::ml_array::number_index(ml_reference_position1727)); ml_reference_projection1727 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1728 = registers.clone(); let ml_reference_position1728 = key; let ml_reference_projection1728 = self::ml_array::optional_at(&*ml_reference_array1728.borrow(), self::ml_array::number_index(ml_reference_position1728)); ml_reference_projection1728 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (({ let ml_reference_value1729 = decoded.borrow().clone(); ml_reference_value1729 }).0).0, Some(shared_constructor_field11.clone()), Some(self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121]))))
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptCallInstruction::PropertyKey(destination_9, input_2, realm_3) => {
                                                        self::accept_java_script_call_key(program.clone(), current.clone(), destination_9, realm_3.clone(), self::begin_java_script_property_key_conversion(shared_constructor_field12.clone(), ({ let ml_reference_array1730 = registers.clone(); let ml_reference_position1730 = input_2; let ml_reference_projection1730 = self::ml_array::optional_at(&*ml_reference_array1730.borrow(), self::ml_array::number_index(ml_reference_position1730)); ml_reference_projection1730 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field11.clone()))
                                                    }
                                                    self::JavaScriptCallInstruction::PropertyDefinition(destination_10, target_3, key_2, attributes, realm_4, operation) => {
                                                        self::accept_java_script_call_definition(program.clone(), current.clone(), destination_10, realm_4.clone(), self::begin_java_script_property_definition(shared_constructor_field12.clone(), ({ let ml_reference_array1731 = registers.clone(); let ml_reference_position1731 = target_3; let ml_reference_projection1731 = self::ml_array::optional_at(&*ml_reference_array1731.borrow(), self::ml_array::number_index(ml_reference_position1731)); ml_reference_projection1731 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1732 = registers.clone(); let ml_reference_position1732 = key_2; let ml_reference_projection1732 = self::ml_array::optional_at(&*ml_reference_array1732.borrow(), self::ml_array::number_index(ml_reference_position1732)); ml_reference_projection1732 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({ let ml_reference_array1733 = registers.clone(); let ml_reference_position1733 = attributes; let ml_reference_projection1733 = self::ml_array::optional_at(&*ml_reference_array1733.borrow(), self::ml_array::number_index(ml_reference_position1733)); ml_reference_projection1733 }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field11.clone(), ((operation.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()))
                                                    }
                                                    _ => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field12.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 102, 97, 109, 105, 108, 121]))
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
        },
    }
}
