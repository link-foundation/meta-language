// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallArgumentList;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallIteratorRegion;
use super::JavaScriptCallMachine;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptCallTemplate;
use super::JavaScriptNumberCallState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryCreateJavaScriptCallFrameLoop1467Result;
use super::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460;
use super::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471;
use super::find_java_script_call_template;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_collection_operation_kind;
use super::read_java_script_error_construction_kind_error;
use super::read_java_script_number_protocol_operation_error;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop1467;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop1468;
use super::source_ml_source1_temporary_create_java_script_call_frame_loop1469;
use super::test_java_script_array_callback_operation;
use super::test_java_script_array_creation_operation;
use super::test_java_script_array_intrinsic_operation;
use super::test_java_script_array_length;
use super::test_java_script_call_coordinate;
use super::test_java_script_collection_constructor_mode;
use super::test_java_script_collection_for_each_operation;
use super::test_java_script_collection_intrinsic_mode;
use super::test_java_script_collection_iterator_intrinsic_mode;
use super::test_java_script_error_constructor_mode;
use super::test_java_script_error_intrinsic_mode;
use super::test_java_script_host_capability_mode;
use super::test_java_script_iterator_region_slot;
use super::test_java_script_object_enumeration_operation;
use super::test_java_script_object_observation_operation;
use super::test_java_script_primitive_intrinsic_mode;
use super::test_java_script_primitive_method_operation;
use super::test_java_script_property_definitions_operation;
use super::test_java_script_property_reflection_operation;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_operation_instruction_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    fn_: self::JavaScriptCallFunction,
    instruction: self::JavaScriptCallInstruction,
) -> self::ml_string::Text {
    match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(
            _,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            shared_constructor_field4,
            _,
            _,
            _,
            _,
            _,
        ) => match program.clone() {
            self::JavaScriptCallProgram::CallProgram(_, shared_constructor_field1, _, _, _, _) => {
                let source_ml_source2_s34449 = match instruction.clone() {
                            self::JavaScriptCallInstruction::ArgumentListOpen(slot) => {
                                if !(self::test_java_script_iterator_region_slot(slot)) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![67, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 115, 108, 111, 116, 115, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 107, 105, 110, 100, 115]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::ArgumentListAppend(slot_2, _) => {
                                if !(self::test_java_script_iterator_region_slot(slot_2)) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![67, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 115, 108, 111, 116, 115, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 107, 105, 110, 100, 115]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::ArgumentListCall(_, slot_3, _, _, mode) => {
                                if (!(self::test_java_script_iterator_region_slot(slot_3)) || !(self::ml_array::includes(&{ let ml_reference_value2222 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 97, 108, 108]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])]))).borrow().clone(); ml_reference_value2222 }, &mode.clone(), 0f64, false, |a, b| a == b))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![67, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 115, 108, 111, 116, 115, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 107, 105, 110, 100, 115]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::CollectionAllocate(_, _, kind) => {
                                if (!(self::test_java_script_collection_constructor_mode(&(shared_constructor_field3))) || !(self::ml_array::includes(&{ let ml_reference_value2223 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])]))).borrow().clone(); ml_reference_value2223 }, &kind.clone(), 0f64, false, |a, b| a == b))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 107, 105, 110, 100, 32, 97, 110, 100, 32, 102, 114, 97, 109, 101]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, _, kind_2, iteration_kind, input) => {
                                if ((((!(self::test_java_script_collection_iterator_intrinsic_mode(&(shared_constructor_field3))) || !(self::ml_array::includes(&{ let ml_reference_value2224 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])]))).borrow().clone(); ml_reference_value2224 }, &kind_2.clone(), 0f64, false, |a, b| a == b))) || !(self::ml_array::includes(&{ let ml_reference_value2225 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![107, 101, 121, 115]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])]))).borrow().clone(); ml_reference_value2225 }, &iteration_kind.clone(), 0f64, false, |a, b| a == b))) || ((kind_2 == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) && ((iteration_kind != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]))) || {
                                    let source_ml_source2_comparison_operand34537 = ((input.clone().unwrap_or_else(|| (None::<f64>,))).0).clone();
                                    {
                                        let source_ml_source2_comparison_operand34538 = None::<()>;
                                        match (source_ml_source2_comparison_operand34537.clone(), source_ml_source2_comparison_operand34538.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                    }
                                }))) || ((kind_2 != (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) && {
                                    let source_ml_source2_comparison_operand34539 = ((input.clone().unwrap_or_else(|| (None::<f64>,))).0).clone();
                                    {
                                        let source_ml_source2_comparison_operand34540 = None::<()>;
                                        !(match (source_ml_source2_comparison_operand34539.clone(), source_ml_source2_comparison_operand34540.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                    }
                                })) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 107, 105, 110, 100, 32, 97, 110, 100, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::ArgumentRest(destination, _, from, realm) => {
                                if ((((realm != shared_constructor_field2) || !(self::test_java_script_call_coordinate(from, 1048577f64))) || (destination != ((from + 1f64)))) || (destination > shared_constructor_field4)) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 115, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 111, 114, 109, 97, 108, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 32, 97, 110, 100, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::IteratorProtocol(destination_2, done, _, _, slot_4, operation, realm_2, read_value) => {
                                if ((((((!(self::ml_array::includes(&{ let ml_reference_value2226 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]), self::ml_string::Text::from_units(vec![115, 116, 101, 112]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])]))).borrow().clone(); ml_reference_value2226 }, &operation.clone(), 0f64, false, |a, b| a == b)) || (realm_2 != shared_constructor_field2)) || !(self::test_java_script_iterator_region_slot(slot_4))) || (destination_2 == (0f64))) || (done == (0f64))) || (destination_2 == done)) || ({
                                    let source_ml_source2_comparison_operand34547 = ((read_value.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                    {
                                        let source_ml_source2_comparison_operand34548 = None::<()>;
                                        !(match (source_ml_source2_comparison_operand34547.clone(), source_ml_source2_comparison_operand34548.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                    }
                                } && ((operation != (self::ml_string::Text::from_units(vec![115, 116, 101, 112]))) || ((match ((read_value.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 112, 114, 111, 116, 111, 99, 111, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 103, 105, 111, 110, 115, 44, 32, 114, 101, 97, 108, 109, 115, 32, 97, 110, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 112, 114, 105, 118, 97, 116, 101, 32, 114, 101, 115, 117, 108, 116, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::CollectionIteratorResult(_, kind_3, _, _) => {
                                if (!(self::test_java_script_collection_iterator_intrinsic_mode(&(shared_constructor_field3))) || !(self::ml_array::includes(&{ let ml_reference_value2227 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])]))).borrow().clone(); ml_reference_value2227 }, &kind_3.clone(), 0f64, false, |a, b| a == b))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 107, 105, 110, 100, 32, 97, 110, 100, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::CollectionOperation(_, operation_2) => {
                                if (!(self::test_java_script_collection_intrinsic_mode(&(shared_constructor_field3))) || ((self::read_java_script_collection_operation_kind(&(operation_2))) == (self::ml_string::Text::from_units(vec![])))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 102, 114, 97, 109, 101]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::PrimitiveMethod(_, operation_3) => {
                                if (!(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field3))) || (!(self::test_java_script_primitive_method_operation(&(operation_3))) && !(self::test_java_script_array_intrinsic_operation(&(operation_3))))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 102, 114, 97, 109, 101]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::CollectionForEach(_, operation_4) => {
                                if (!(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field3))) || !(self::test_java_script_collection_for_each_operation(&(operation_4)))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::ArrayCreation(_, operation_5, _, _) => {
                                if (!(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field3))) || !(self::test_java_script_array_creation_operation(&(operation_5)))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::ArrayCallback(_, operation_6, _) => {
                                if (!(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field3))) || !(self::test_java_script_array_callback_operation(&(operation_6)))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 111, 119, 110, 101, 114, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::ObjectEnumeration(_, operation_7, _) => {
                                if (!(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field3))) || !(self::test_java_script_object_enumeration_operation(&(operation_7)))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::PropertyDefinitions(_, operation_8, _, _) => {
                                if (!(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field3))) || !(self::test_java_script_property_definitions_operation(&(operation_8)))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::PropertyReflection(_, operation_9, _, _, _, _) => {
                                if (!(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field3))) || !(self::test_java_script_property_reflection_operation(&(operation_9)))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::InstanceOf(_, _, _, _, operation_10) => {
                                if (!(self::ml_array::includes(&{ let ml_reference_value2228 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 97, 115, 32, 105, 110, 115, 116, 97, 110, 99, 101]), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 111, 110, 116, 97, 105, 110, 115])]))).borrow().clone(); ml_reference_value2228 }, &operation_10.clone(), 0f64, false, |a, b| a == b)) || ((operation_10 != (self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102]))) && !(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field3))))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 111, 114, 32, 111, 114, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 98, 111, 100, 121]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::ObjectObservation(_, operation_11, _, _) => {
                                if (!(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field3))) || !(self::test_java_script_object_observation_operation(&(operation_11)))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::CollectionCopy(_, _, _, _, _, _, _, _) => {
                                if !(self::test_java_script_collection_constructor_mode(&(shared_constructor_field3))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 112, 121, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 114, 97, 109, 101]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, _, realm_3, error_class, bind_receiver) => {
                                if !(self::ml_array::includes(&{ let ml_reference_value2229 = shared_constructor_field1.borrow().clone(); ml_reference_value2229 }, &realm_3.clone(), 0f64, false, |a, b| a == b)) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 118, 97, 108, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                } else {
                                    {
                                        let error = self::read_java_script_error_construction_kind_error(&(error_class), bind_receiver, &(shared_constructor_field3));
                                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                                            self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(error.clone())
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                        }
                                    }
                                }
                            }
                            self::JavaScriptCallInstruction::ReadDerivedReceiver(_) => {
                                if !(self::test_java_script_error_constructor_mode(&(shared_constructor_field3))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 114, 101, 97, 100, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::InvokeErrorIntrinsic(_) => {
                                if !(self::test_java_script_error_intrinsic_mode(&(shared_constructor_field3))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![104, 111, 115, 116, 32, 69, 114, 114, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 110, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::InvokeHostCapability(_) => {
                                if !(self::test_java_script_host_capability_mode(&(shared_constructor_field3))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![104, 111, 115, 116, 32, 99, 97, 112, 97, 98, 105, 108, 105, 116, 121, 32, 99, 97, 108, 108, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 108, 121, 32, 113, 117, 97, 108, 105, 102, 105, 101, 100, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::LiteralAccessor(_, _, _, _, kind_4, realm_4) => {
                                if (!(self::ml_array::includes(&{ let ml_reference_value2230 = shared_constructor_field1.borrow().clone(); ml_reference_value2230 }, &realm_4.clone(), 0f64, false, |a, b| a == b)) || !(self::ml_array::includes(&{ let ml_reference_value2231 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![103, 101, 116]), self::ml_string::Text::from_units(vec![115, 101, 116])]))).borrow().clone(); ml_reference_value2231 }, &kind_4.clone(), 0f64, false, |a, b| a == b))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![108, 105, 116, 101, 114, 97, 108, 32, 97, 99, 99, 101, 115, 115, 111, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 103, 101, 116, 116, 101, 114, 47, 115, 101, 116, 116, 101, 114, 32, 107, 105, 110, 100, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::ObjectFromEntries(_, _, _, _) => {
                                if !(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field3))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 102, 114, 97, 109, 101]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::SymbolRegistry(_, _, operation_12) => {
                                if (!(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field3))) || !(self::ml_array::includes(&{ let ml_reference_value2232 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![102, 111, 114]), self::ml_string::Text::from_units(vec![107, 101, 121, 70, 111, 114])]))).borrow().clone(); ml_reference_value2232 }, &operation_12.clone(), 0f64, false, |a, b| a == b))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 119, 111, 114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 109, 101, 116, 104, 111, 100, 32, 111, 119, 110, 101, 114]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::FunctionName(_, _, _, prefix, realm_5) => {
                                if ((realm_5 != shared_constructor_field2) || !(self::ml_array::includes(&{ let ml_reference_value2233 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![103, 101, 116]), self::ml_string::Text::from_units(vec![115, 101, 116])]))).borrow().clone(); ml_reference_value2233 }, &prefix.clone(), 0f64, false, |a, b| a == b))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109, 32, 97, 110, 100, 32, 112, 114, 101, 102, 105, 120]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::ArrayAllocate(_, _, length) => {
                                if !(self::test_java_script_array_length(length)) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 108, 105, 116, 101, 114, 97, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 115, 105, 103, 110, 101, 100, 51, 50, 32, 115, 112, 97, 114, 115, 101, 32, 108, 101, 110, 103, 116, 104]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::ClosureAllocate(_, owner, ordinal, captures, _) => {
                                if self::test_java_script_error_constructor_mode(&(shared_constructor_field3)) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 116, 104, 105, 115, 32, 99, 108, 111, 115, 117, 114, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 108, 101, 120, 105, 99, 97, 108, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 99, 101, 108, 108]))
                                } else {
                                    {
                                        let template = {
                                            let source_ml_source2_array_read_value34575 = (match program.clone() {
                                                self::JavaScriptCallProgram::CallProgram(_, _, _, source_ml_source2_data_property_value34574, _, _) => {
                                                    ((source_ml_source2_data_property_value34574.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,))).0).clone()
                                                }
                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallTemplate>::new())));
                                            {
                                                let source_ml_source2_array_read_index34576 = self::find_java_script_call_template(program.clone(), owner.clone(), ordinal);
                                                { let ml_reference_array2234 = source_ml_source2_array_read_value34575.clone(); let ml_reference_position2234 = source_ml_source2_array_read_index34576; let ml_reference_projection2234 = self::ml_array::optional_at(&*ml_reference_array2234.borrow(), self::ml_array::number_index(ml_reference_position2234)); ml_reference_projection2234 }
                                            }
                                        };
                                        if ((match (template.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match template.clone().expect("a proven present value") {
                                            self::JavaScriptCallTemplate::CallTemplate(_, _, source_ml_source2_data_property_value34579, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value34579.clone()
                                            }
                                        }) != shared_constructor_field2)) || ((match template.clone().expect("a proven present value") {
                                            self::JavaScriptCallTemplate::CallTemplate(_, _, _, _, _, source_ml_source2_data_property_value34582, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value34582
                                            }
                                        }) != ({ let ml_reference_array2235 = captures.clone(); let ml_reference_projection2235 = ml_reference_array2235.borrow().len() as f64; ml_reference_projection2235 }))) {
                                            self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 116, 101, 109, 112, 108, 97, 116, 101, 44, 32, 114, 101, 97, 108, 109, 32, 97, 110, 100, 32, 99, 97, 112, 116, 117, 114, 101, 32, 118, 101, 99, 116, 111, 114]))
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                        }
                                    }
                                }
                            }
                            self::JavaScriptCallInstruction::ArraySpread(_, _, _, _, _, realm_6) => {
                                if (realm_6 != shared_constructor_field2) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::ObjectSpread(_, _, _, realm_7, excluded, operation_13) => {
                                if (!(self::ml_array::includes(&{ let ml_reference_value2236 = shared_constructor_field1.borrow().clone(); ml_reference_value2236 }, &realm_7.clone(), 0f64, false, |a, b| a == b)) || ({
                                    let source_ml_source2_comparison_operand34587 = ((operation_13.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone();
                                    {
                                        let source_ml_source2_comparison_operand34588 = None::<()>;
                                        !(match (source_ml_source2_comparison_operand34587.clone(), source_ml_source2_comparison_operand34588.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                    }
                                } && (({
                                    let source_ml_source2_comparison_operand34589 = ((operation_13.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone();
                                    !(match (source_ml_source2_comparison_operand34589.clone(), self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                } || !(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field3)))) || (({ let ml_reference_array2237 = (((excluded.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))); let ml_reference_projection2237 = ml_reference_array2237.borrow().len() as f64; ml_reference_projection2237 }) != (0f64))))) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 83, 101, 116, 32, 98, 111, 100, 121, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 107, 101, 121, 115]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            self::JavaScriptCallInstruction::OrdinaryDataProperty(_, _, _, _, realm_8) => {
                                if !(self::ml_array::includes(&{ let ml_reference_value2238 = shared_constructor_field1.borrow().clone(); ml_reference_value2238 }, &realm_8.clone(), 0f64, false, |a, b| a == b)) {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(self::ml_string::Text::from_units(vec![97, 32, 100, 97, 116, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                                }
                            }
                            _ => {
                                self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next
                            }
                        };
                match source_ml_source2_s34449.clone() {
                            self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460::MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(source_ml_source1_temporary_result1460) => {
                                source_ml_source1_temporary_result1460.clone()
                            }
                        }
            }
        },
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn replace_java_script_call_frame(
    heap: self::JavaScriptOrdinaryHeap,
    frames: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFrame>>>,
    frame: self::JavaScriptCallFrame,
) -> self::JavaScriptCallMachine {
    self::JavaScriptCallMachine::CallMachine(
        Box::new(heap.clone()),
        std::rc::Rc::new(std::cell::RefCell::new({
            let ml_values2239 = {
                let ml_reference_value2240 = frames.borrow().clone();
                ml_reference_value2240
            };
            let ml_callback2239 = {
                let frames = frames.clone();
                let frame = frame.clone();
                std::rc::Rc::new(
                    move |entry: self::JavaScriptCallFrame,
                          index: f64|
                          -> self::JavaScriptCallFrame {
                        if (index
                            == ({
                                let ml_reference_array2241 = frames.clone();
                                let ml_reference_projection2241 =
                                    ml_reference_array2241.borrow().len() as f64;
                                ml_reference_projection2241
                            } - 1f64))
                        {
                            frame.clone()
                        } else {
                            entry.clone()
                        }
                    },
                )
            };
            ml_values2239
                .iter()
                .cloned()
                .enumerate()
                .map(|(ml_index2239, ml_value2239)| {
                    ml_callback2239(ml_value2239, (ml_index2239 as f64))
                })
                .collect::<Vec<_>>()
        })),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn write_java_script_call_register(
    frame: self::JavaScriptCallFrame,
    destination: f64,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallFrame {
    {
        let source_ml_source2_nominal_spread34595 = frame.clone();
        match source_ml_source2_nominal_spread34595.clone() {
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
                    let source_ml_source2_nominal_field34596 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_nominal_field34598 = shared_constructor_field2;
                        {
                            let source_ml_source2_nominal_field34600 =
                                shared_constructor_field3.clone();
                            {
                                let source_ml_source2_nominal_field34602 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_nominal_field34604 =
                                        shared_constructor_field5.clone();
                                    {
                                        let source_ml_source2_nominal_field34606 =
                                            shared_constructor_field6.clone();
                                        {
                                            let source_ml_source2_nominal_field34608 =
                                                shared_constructor_field7.clone();
                                            {
                                                let source_ml_source2_nominal_field34610 =
                                                    shared_constructor_field8.clone();
                                                {
                                                    let source_ml_source2_nominal_field34612 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_nominal_field34614 =
                                                            shared_constructor_field10.clone();
                                                        {
                                                            let source_ml_source2_nominal_override34616 =
                                                                std::rc::Rc::new(
                                                                    std::cell::RefCell::new({
                                                                        let ml_values2242 = {
                                                                            let ml_reference_value2243 = (match frame.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value34594, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value34594.clone()
                                                                }
                                                            }).borrow().clone();
                                                                            ml_reference_value2243
                                                                        };
                                                                        let ml_callback2242 = {
                                                                            let destination =
                                                                                destination;
                                                                            let value =
                                                                                value.clone();
                                                                            std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, index: f64| -> self::JavaScriptSemanticValue { if (index == destination) {
                                                                value.clone()
                                                            } else {
                                                                entry.clone()
                                                            } })
                                                                        };
                                                                        ml_values2242.iter().cloned().enumerate().map(|(ml_index2242, ml_value2242)| ml_callback2242(ml_value2242, (ml_index2242 as f64))).collect::<Vec<_>>()
                                                                    }),
                                                                );
                                                            {
                                                                let source_ml_source2_nominal_override34617 = self::JavaScriptCallPending::CallIdle;
                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field34596.clone(), source_ml_source2_nominal_field34598, source_ml_source2_nominal_override34616.clone(), source_ml_source2_nominal_field34602.clone(), source_ml_source2_nominal_field34604.clone(), Box::new(source_ml_source2_nominal_override34617.clone()), source_ml_source2_nominal_field34608.clone(), source_ml_source2_nominal_field34610.clone(), source_ml_source2_nominal_field34612.clone(), source_ml_source2_nominal_field34614.clone())
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
pub fn replace_java_script_call_pending(
    frame: self::JavaScriptCallFrame,
    pending: self::JavaScriptCallPending,
) -> self::JavaScriptCallFrame {
    match frame.clone() {
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
                let source_ml_source2_nominal_field34619 = shared_constructor_field1.clone();
                {
                    let source_ml_source2_nominal_field34621 = shared_constructor_field2;
                    {
                        let source_ml_source2_nominal_field34623 =
                            shared_constructor_field3.clone();
                        {
                            let source_ml_source2_nominal_field34625 =
                                shared_constructor_field4.clone();
                            {
                                let source_ml_source2_nominal_field34627 =
                                    shared_constructor_field5.clone();
                                {
                                    let source_ml_source2_nominal_field34629 =
                                        shared_constructor_field6.clone();
                                    {
                                        let source_ml_source2_nominal_field34631 =
                                            shared_constructor_field7.clone();
                                        {
                                            let source_ml_source2_nominal_field34633 =
                                                shared_constructor_field8.clone();
                                            {
                                                let source_ml_source2_nominal_field34635 =
                                                    shared_constructor_field9.clone();
                                                {
                                                    let source_ml_source2_nominal_field34637 =
                                                        shared_constructor_field10.clone();
                                                    self::JavaScriptCallFrame::CallFrame(
                                                        source_ml_source2_nominal_field34619
                                                            .clone(),
                                                        source_ml_source2_nominal_field34621,
                                                        source_ml_source2_nominal_field34623
                                                            .clone(),
                                                        source_ml_source2_nominal_field34625
                                                            .clone(),
                                                        source_ml_source2_nominal_field34627
                                                            .clone(),
                                                        Box::new(pending.clone()),
                                                        source_ml_source2_nominal_field34631
                                                            .clone(),
                                                        source_ml_source2_nominal_field34633
                                                            .clone(),
                                                        source_ml_source2_nominal_field34635
                                                            .clone(),
                                                        source_ml_source2_nominal_field34637
                                                            .clone(),
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn advance_java_script_call_frame(
    frame: self::JavaScriptCallFrame,
) -> self::JavaScriptCallFrame {
    match frame.clone() {
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
                let source_ml_source2_nominal_field34642 = shared_constructor_field1.clone();
                {
                    let source_ml_source2_nominal_field34644 = shared_constructor_field2;
                    {
                        let source_ml_source2_nominal_field34646 =
                            shared_constructor_field3.clone();
                        {
                            let source_ml_source2_nominal_field34648 =
                                shared_constructor_field4.clone();
                            {
                                let source_ml_source2_nominal_field34650 =
                                    shared_constructor_field5.clone();
                                {
                                    let source_ml_source2_nominal_field34652 =
                                        shared_constructor_field6.clone();
                                    {
                                        let source_ml_source2_nominal_field34654 =
                                            shared_constructor_field7.clone();
                                        {
                                            let source_ml_source2_nominal_field34656 =
                                                shared_constructor_field8.clone();
                                            {
                                                let source_ml_source2_nominal_field34658 =
                                                    shared_constructor_field9.clone();
                                                {
                                                    let source_ml_source2_nominal_field34660 =
                                                        shared_constructor_field10.clone();
                                                    {
                                                        let source_ml_source2_nominal_override34662 =
                                                            (shared_constructor_field2 + 1f64);
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field34642.clone(), source_ml_source2_nominal_override34662, source_ml_source2_nominal_field34646.clone(), source_ml_source2_nominal_field34648.clone(), source_ml_source2_nominal_field34650.clone(), Box::new(source_ml_source2_nominal_field34652.clone()), source_ml_source2_nominal_field34654.clone(), source_ml_source2_nominal_field34656.clone(), source_ml_source2_nominal_field34658.clone(), source_ml_source2_nominal_field34660.clone())
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
pub fn create_java_script_call_frame(
    fn_: self::JavaScriptCallFunction,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallFrame {
    match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(
            shared_constructor_field1,
            _,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            {
                let registers = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                    self::JavaScriptSemanticValue,
                >::new()));
                {
                    let index = 0f64;
                    {
                        let source_ml_source2_s34663 = self::source_ml_source1_temporary_create_java_script_call_frame_loop1467(fn_.clone(), registers.clone(), index);
                        match source_ml_source2_s34663.clone() {
                            self::MlSource1TemporaryCreateJavaScriptCallFrameLoop1467Result::MlSource1TemporaryCreateJavaScriptCallFrameLoop1467Done => {
                                {
                                    let bound = if self::test_java_script_error_constructor_mode(&(shared_constructor_field2)) {
                                        self::JavaScriptSemanticValue::Undefined
                                    } else {
                                        if (shared_constructor_field2 == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108]))) {
                                            shared_constructor_field3.clone()
                                        } else {
                                            receiver.clone()
                                        }
                                    };
                                    {
                                        let registers_2 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values2244 = { let ml_reference_value2245 = registers.borrow().clone(); ml_reference_value2245 }; let ml_callback2244 = { let bound = bound.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, index_2: f64| -> self::JavaScriptSemanticValue { if (index_2 == (0f64)) {
                                            bound.clone()
                                        } else {
                                            entry.clone()
                                        } }) }; ml_values2244.iter().cloned().enumerate().map(|(ml_index2244, ml_value2244)| ml_callback2244(ml_value2244, (ml_index2244 as f64))).collect::<Vec<_>>() }));
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let registers_3 = self::source_ml_source1_temporary_create_java_script_call_frame_loop1468(fn_.clone(), arguments_.clone(), registers_2.clone(), index_2);
                                                {
                                                    let index_3 = 0f64;
                                                    {
                                                        let registers_4 = self::source_ml_source1_temporary_create_java_script_call_frame_loop1469(fn_.clone(), registers_3.clone(), index_3);
                                                        {
                                                            let source_ml_source1_temporary_constructor_field1461 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source1_temporary_constructor_field1462 = 0f64;
                                                                {
                                                                    let source_ml_source1_temporary_constructor_field1465 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallHandler>::new()));
                                                                    {
                                                                        let source_ml_source1_temporary_constructor_field1466 = self::JavaScriptCallPending::CallIdle;
                                                                        if ((shared_constructor_field2 == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108]))) && !(match (match fn_.clone() {
                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value34671, _) => {
                                                                                ((source_ml_source2_data_property_value34671.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                            }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                            {
                                                                                let source_ml_source2_constructor_field34679 = Some((match fn_.clone() {
                                                                                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value34678, _) => {
                                                                                        ((source_ml_source2_data_property_value34678.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value"));
                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source1_temporary_constructor_field1461.clone(), source_ml_source1_temporary_constructor_field1462, registers_4.clone(), arguments_.clone(), source_ml_source1_temporary_constructor_field1465.clone(), Box::new(source_ml_source1_temporary_constructor_field1466.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, Some((source_ml_source2_constructor_field34679.clone(),)))
                                                                            }
                                                                        } else {
                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source1_temporary_constructor_field1461.clone(), source_ml_source1_temporary_constructor_field1462, registers_4.clone(), arguments_.clone(), source_ml_source1_temporary_constructor_field1465.clone(), Box::new(source_ml_source1_temporary_constructor_field1466.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>)
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
pub fn read_java_script_number_call_source_operation(
    fn_: self::JavaScriptCallFunction,
    frame: self::JavaScriptCallFrame,
    destination: f64,
    realm: self::ml::Big,
    exotic_key: self::JavaScriptSemanticValue,
    state: self::JavaScriptNumberCallState,
) -> self::ml_string::Text {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            _,
            shared_constructor_field6,
            shared_constructor_field7,
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
                shared_constructor_field4,
                _,
                _,
                _,
                _,
                _,
                shared_constructor_field5,
                _,
                _,
            ) => {
                if ((shared_constructor_field6 == (0f64)) || (shared_constructor_field4 != realm)) {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    {
                        let instruction = {
                            let source_ml_source2_array_read_value34692 =
                                shared_constructor_field5.clone();
                            {
                                let source_ml_source2_array_read_index34693 =
                                    (shared_constructor_field6 - 1f64);
                                {
                                    let ml_reference_array2246 =
                                        source_ml_source2_array_read_value34692.clone();
                                    let ml_reference_position2246 =
                                        source_ml_source2_array_read_index34693;
                                    let ml_reference_projection2246 = self::ml_array::optional_at(
                                        &*ml_reference_array2246.borrow(),
                                        self::ml_array::number_index(ml_reference_position2246),
                                    );
                                    ml_reference_projection2246
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
                                            let ml_reference_array2247 = arguments.clone();
                                            let ml_reference_projection2247 =
                                                ml_reference_array2247.borrow().len() as f64;
                                            ml_reference_projection2247
                                        }) != (1f64)))
                                    {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        if ((self::read_java_script_number_protocol_operation_error(
                                            &(operation),
                                        ))
                                            != (self::ml_string::Text::from_units(vec![])))
                                        {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            {
                                                let input = ({
                                                        let source_ml_source2_array_read_value34697 = shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_array_read_index34698 = ({
                                                                let source_ml_source2_array_read_index34696 = 0f64;
                                                                { let ml_reference_array2248 = arguments.clone(); let ml_reference_position2248 = source_ml_source2_array_read_index34696; let ml_reference_projection2248 = self::ml_array::optional_at(&*ml_reference_array2248.borrow(), self::ml_array::number_index(ml_reference_position2248)); ml_reference_projection2248 }
                                                            }).clone().unwrap_or_else(|| (-1f64));
                                                            { let ml_reference_array2249 = source_ml_source2_array_read_value34697.clone(); let ml_reference_position2249 = source_ml_source2_array_read_index34698; let ml_reference_projection2249 = self::ml_array::optional_at(&*ml_reference_array2249.borrow(), self::ml_array::number_index(ml_reference_position2249)); ml_reference_projection2249 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                match input.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        let source_ml_source2_s34706 = match state.clone() {
                                                                    self::JavaScriptNumberCallState::NumberProtocolState(state_state) => {
                                                                        let state_state = *state_state;
                                                                        match state_state.clone() {
                                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, _) => {
                                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                                let shared_constructor_field2 = *shared_constructor_field2;
                                                                                if (shared_constructor_field3 != (if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101]))) {
                                                                                    self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])
                                                                                } else {
                                                                                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                                                                                })) {
                                                                                    self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471Return(self::ml_string::Text::from_units(vec![]))
                                                                                } else {
                                                                                    if (!(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), input.clone())) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), exotic_key.clone()))) {
                                                                                        self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471Return(self::ml_string::Text::from_units(vec![]))
                                                                                    } else {
                                                                                        self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471Next
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::JavaScriptNumberCallState::NumberPrimitiveValue(_) => {
                                                                        self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471Next
                                                                    }
                                                                };
                                                        match source_ml_source2_s34706.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471Next => {
                                                                        operation.clone()
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471::MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471Return(source_ml_source1_temporary_result1471) => {
                                                                        source_ml_source1_temporary_result1471.clone()
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
