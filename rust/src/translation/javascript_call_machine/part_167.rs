// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArgumentRestState;
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayCallbackCall;
use super::JavaScriptArrayCallbackState;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptArraySearchCall;
use super::JavaScriptArraySearchState;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptCollectionForEachCall;
use super::JavaScriptCollectionForEachState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_argument_rest_state_error;
use super::read_java_script_array_assignment_state_error;
use super::read_java_script_array_callback_state_error;
use super::read_java_script_array_search_state_error;
use super::read_java_script_collection_for_each_state_error;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_pending_array_callback_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    frame: self::JavaScriptCallFrame,
    fn_: self::JavaScriptCallFunction,
    suspended: bool,
    pending: self::JavaScriptCallPending,
) -> self::ml_string::Text {
    match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(
            _,
            shared_constructor_field11,
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field12,
            _,
            _,
        ) => {
            match frame.clone() {
                self::JavaScriptCallFrame::CallFrame(
                    _,
                    shared_constructor_field8,
                    shared_constructor_field9,
                    shared_constructor_field10,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    match program.clone() {
                        self::JavaScriptCallProgram::CallProgram(
                            _,
                            _,
                            shared_constructor_field7,
                            _,
                            _,
                            _,
                        ) => {
                            let shared_constructor_field7 = *shared_constructor_field7;
                            match pending.clone() {
                                self::JavaScriptCallPending::WaitArgumentRest(
                                    destination,
                                    realm,
                                    state,
                                ) => {
                                    let state = *state;
                                    match state.clone() {
                                        self::JavaScriptArgumentRestState::ArgumentRestState(
                                            shared_constructor_field1,
                                            shared_constructor_field2,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let shared_constructor_field1 =
                                                *shared_constructor_field1;
                                            if (shared_constructor_field8 == (0f64)) {
                                                self::ml_string::Text::from_units(vec![
                                                    97, 114, 103, 117, 109, 101, 110, 116, 32, 114,
                                                    101, 115, 116, 32, 114, 101, 116, 97, 105, 110,
                                                    115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                                    97, 108, 32, 115, 111, 117, 114, 99, 101, 32,
                                                    105, 110, 115, 116, 114, 117, 99, 116, 105,
                                                    111, 110, 32, 97, 110, 100, 32, 112, 114, 111,
                                                    116, 111, 116, 121, 112, 101,
                                                ])
                                            } else {
                                                {
                                                    let instruction = {
                                                        let source_ml_source2_array_read_value31644 =
                                                            shared_constructor_field12.clone();
                                                        {
                                                            let source_ml_source2_array_read_index31645 =
                                                                (shared_constructor_field8 - 1f64);
                                                            {
                                                                let ml_reference_array1938 = source_ml_source2_array_read_value31644.clone();
                                                                let ml_reference_position1938 = source_ml_source2_array_read_index31645;
                                                                let ml_reference_projection1938 = self::ml_array::optional_at(&*ml_reference_array1938.borrow(), self::ml_array::number_index(ml_reference_position1938));
                                                                ml_reference_projection1938
                                                            }
                                                        }
                                                    };
                                                    if (match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::ArgumentRest(_, _, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 115, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                    } else {
                                                        if (((((((match instruction.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ArgumentRest(source_ml_source2_data_property_value31651, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value31651,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0) != destination) || (((((match instruction.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ArgumentRest(_, _, _, source_ml_source2_data_property_value31653) => {
                                                                Some((source_ml_source2_data_property_value31653.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml::Big,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()) != realm)) || ((((match instruction.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ArgumentRest(_, _, source_ml_source2_data_property_value31656, _) => {
                                                                Some((source_ml_source2_data_property_value31656,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0) != shared_constructor_field2)) || !(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), ({
                                                            let source_ml_source2_array_read_value31661 = shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_array_read_index31662 = ((match instruction.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallInstruction::ArgumentRest(_, source_ml_source2_data_property_value31660, _, _) => {
                                                                        Some((source_ml_source2_data_property_value31660,))
                                                                    }
                                                                    _ => {
                                                                        None::<(f64,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0;
                                                                { let ml_reference_array1939 = source_ml_source2_array_read_value31661.clone(); let ml_reference_position1939 = source_ml_source2_array_read_index31662; let ml_reference_projection1939 = self::ml_array::optional_at(&*ml_reference_array1939.borrow(), self::ml_array::number_index(ml_reference_position1939)); ml_reference_projection1939 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) {
                                                            self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 115, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                        } else {
                                                            {
                                                                let error = self::read_java_script_argument_rest_state_error(heap.clone(), shared_constructor_field10.clone(), state.clone());
                                                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                    error.clone()
                                                                } else {
                                                                    if suspended {
                                                                        self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 99, 111, 112, 121, 105, 110, 103, 32, 99, 97, 110, 110, 111, 116, 32, 104, 105, 100, 101, 32, 97, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 99, 97, 108, 108, 98, 97, 99, 107])
                                                                    } else {
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
                                self::JavaScriptCallPending::WaitArraySearch(
                                    destination_2,
                                    realm_2,
                                    state_2,
                                ) => {
                                    let state_2 = *state_2;
                                    match state_2.clone() {
                                        self::JavaScriptArraySearchState::ArraySearchState(
                                            shared_constructor_field1_2,
                                            shared_constructor_field2_2,
                                            shared_constructor_field3,
                                            shared_constructor_field4,
                                            shared_constructor_field5,
                                            shared_constructor_field6,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let shared_constructor_field2_2 =
                                                *shared_constructor_field2_2;
                                            let shared_constructor_field3 =
                                                *shared_constructor_field3;
                                            let shared_constructor_field4 =
                                                *shared_constructor_field4;
                                            let shared_constructor_field6 =
                                                *shared_constructor_field6;
                                            if !(shared_constructor_field8 > (0f64)) {
                                                self::ml_string::Text::from_units(vec![
                                                    65, 114, 114, 97, 121, 32, 115, 101, 97, 114,
                                                    99, 104, 32, 99, 111, 110, 116, 105, 110, 117,
                                                    97, 116, 105, 111, 110, 32, 114, 101, 116, 97,
                                                    105, 110, 115, 32, 105, 116, 115, 32, 97, 99,
                                                    116, 117, 97, 108, 32, 115, 111, 117, 114, 99,
                                                    101, 32, 111, 112, 101, 114, 97, 116, 105, 111,
                                                    110, 44, 32, 100, 101, 115, 116, 105, 110, 97,
                                                    116, 105, 111, 110, 32, 97, 110, 100, 32, 114,
                                                    101, 97, 108, 109,
                                                ])
                                            } else {
                                                {
                                                    let instruction_2 = {
                                                        let source_ml_source2_array_read_value31668 =
                                                            shared_constructor_field12.clone();
                                                        {
                                                            let source_ml_source2_array_read_index31669 =
                                                                (shared_constructor_field8 - 1f64);
                                                            {
                                                                let ml_reference_array1940 = source_ml_source2_array_read_value31668.clone();
                                                                let ml_reference_position1940 = source_ml_source2_array_read_index31669;
                                                                let ml_reference_projection1940 = self::ml_array::optional_at(&*ml_reference_array1940.borrow(), self::ml_array::number_index(ml_reference_position1940));
                                                                ml_reference_projection1940
                                                            }
                                                        }
                                                    };
                                                    if (match (instruction_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction_2.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::ArraySearch(_, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 44, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                    } else {
                                                        if ((((((match instruction_2.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ArraySearch(source_ml_source2_data_property_value31674, _, _) => {
                                                                Some((source_ml_source2_data_property_value31674,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0) != destination_2) || (realm_2 != shared_constructor_field11)) || (shared_constructor_field1_2 != ((((match instruction_2.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ArraySearch(_, source_ml_source2_data_property_value31680, _) => {
                                                                Some((source_ml_source2_data_property_value31680.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml_string::Text,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()))) {
                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 44, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                        } else {
                                                            {
                                                                let error_2 = self::read_java_script_array_search_state_error(heap.clone(), state_2.clone());
                                                                if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                    error_2.clone()
                                                                } else {
                                                                    {
                                                                        let at = (((((match instruction_2.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallInstruction::ArraySearch(_, source_ml_source2_data_property_value31682, _) => {
                                                                                Some((source_ml_source2_data_property_value31682.clone(),))
                                                                            }
                                                                            _ => {
                                                                                None::<(self::ml_string::Text,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0).clone()) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 97, 116])));
                                                                        {
                                                                            let search = if at {
                                                                                self::JavaScriptSemanticValue::Undefined
                                                                            } else {
                                                                                ({
                                                                                    let source_ml_source2_array_read_value31684 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_array_read_index31685 = 0f64;
                                                                                        { let ml_reference_array1941 = source_ml_source2_array_read_value31684.clone(); let ml_reference_position1941 = source_ml_source2_array_read_index31685; let ml_reference_projection1941 = self::ml_array::optional_at(&*ml_reference_array1941.borrow(), self::ml_array::number_index(ml_reference_position1941)); ml_reference_projection1941 }
                                                                                    }
                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)
                                                                            };
                                                                            {
                                                                                let from = ({
                                                                                    let source_ml_source2_array_read_value31687 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_array_read_index31688 = if at {
                                                                                            0f64
                                                                                        } else {
                                                                                            1f64
                                                                                        };
                                                                                        { let ml_reference_array1942 = source_ml_source2_array_read_value31687.clone(); let ml_reference_position1942 = source_ml_source2_array_read_index31688; let ml_reference_projection1942 = self::ml_array::optional_at(&*ml_reference_array1942.borrow(), self::ml_array::number_index(ml_reference_position1942)); ml_reference_projection1942 }
                                                                                    }
                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                                {
                                                                                    let present = (({ let ml_reference_array1943 = shared_constructor_field10.clone(); let ml_reference_projection1943 = ml_reference_array1943.borrow().len() as f64; ml_reference_projection1943 }) >= (if at {
                                                                                        1f64
                                                                                    } else {
                                                                                        2f64
                                                                                    }));
                                                                                    if ((((!(self::test_java_script_semantic_same_value(shared_constructor_field2_2.clone(), ({
                                                                                        let source_ml_source2_array_read_value31694 = shared_constructor_field9.clone();
                                                                                        {
                                                                                            let source_ml_source2_array_read_index31695 = ((((match instruction_2.clone().expect("a proven present value") {
                                                                                                self::JavaScriptCallInstruction::ArraySearch(_, _, source_ml_source2_data_property_value31693) => {
                                                                                                    Some((((source_ml_source2_data_property_value31693.clone().unwrap_or_else(|| (None::<f64>,))).0).clone(),))
                                                                                                }
                                                                                                _ => {
                                                                                                    None::<(Option<f64>,)>
                                                                                                }
                                                                                            }).clone().expect("a proven present value")).0).clone()).clone().unwrap_or_else(|| 0f64);
                                                                                            { let ml_reference_array1944 = source_ml_source2_array_read_value31694.clone(); let ml_reference_position1944 = source_ml_source2_array_read_index31695; let ml_reference_projection1944 = self::ml_array::optional_at(&*ml_reference_array1944.borrow(), self::ml_array::number_index(ml_reference_position1944)); ml_reference_projection1944 }
                                                                                        }
                                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) || !(self::test_java_script_semantic_same_value(shared_constructor_field3.clone(), search.clone()))) || !(self::test_java_script_semantic_same_value(shared_constructor_field4.clone(), from.clone()))) || (shared_constructor_field5 != present)) || !(self::test_java_script_semantic_same_value(shared_constructor_field6.clone(), shared_constructor_field7.clone()))) {
                                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 115, 101, 97, 114, 99, 104, 32, 118, 97, 108, 117, 101, 44, 32, 105, 110, 100, 101, 120, 32, 97, 114, 103, 117, 109, 101, 110, 116, 44, 32, 112, 114, 101, 115, 101, 110, 99, 101, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108])
                                                                                    } else {
                                                                                        if (suspended && match (match state_2.clone() {
                                                                                            self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value31702) => {
                                                                                                ((source_ml_source2_data_property_value31702.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySearchCall>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                            self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 97, 110, 32, 114, 101, 116, 97, 105, 110, 32, 97, 32, 99, 104, 105, 108, 100, 32, 102, 114, 97, 109, 101])
                                                                                        } else {
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
                                                }
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptCallPending::WaitCollectionForEach(
                                    destination_3,
                                    realm_3,
                                    state_3,
                                ) => {
                                    let state_3 = *state_3;
                                    match state_3.clone() {
                                        self::JavaScriptCollectionForEachState::CollectionForEachState(shared_constructor_field1_3, shared_constructor_field2_3, shared_constructor_field3_2, shared_constructor_field4_2, _, _, _, _, _) => {
                                            let shared_constructor_field2_3 = *shared_constructor_field2_3;
                                            let shared_constructor_field3_2 = *shared_constructor_field3_2;
                                            let shared_constructor_field4_2 = *shared_constructor_field4_2;
                                            if !((shared_constructor_field8 > (0f64))) {
                                                self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                            } else {
                                                {
                                                    let instruction_3 = {
                                                        let source_ml_source2_array_read_value31707 = shared_constructor_field12.clone();
                                                        {
                                                            let source_ml_source2_array_read_index31708 = (shared_constructor_field8 - 1f64);
                                                            { let ml_reference_array1945 = source_ml_source2_array_read_value31707.clone(); let ml_reference_position1945 = source_ml_source2_array_read_index31708; let ml_reference_projection1945 = self::ml_array::optional_at(&*ml_reference_array1945.borrow(), self::ml_array::number_index(ml_reference_position1945)); ml_reference_projection1945 }
                                                        }
                                                    };
                                                    if (match (instruction_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction_3.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::CollectionForEach(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                    } else {
                                                        if ((((((match instruction_3.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::CollectionForEach(source_ml_source2_data_property_value31712, _) => {
                                                                Some((source_ml_source2_data_property_value31712,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0) != destination_3) || (realm_3 != shared_constructor_field11)) || (((((match instruction_3.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::CollectionForEach(_, source_ml_source2_data_property_value31717) => {
                                                                Some((source_ml_source2_data_property_value31717.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml_string::Text,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field1_3)) {
                                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                        } else {
                                                            {
                                                                let error_3 = self::read_java_script_collection_for_each_state_error(heap.clone(), state_3.clone());
                                                                if (error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                                    error_3.clone()
                                                                } else {
                                                                    if ((!(self::test_java_script_semantic_same_value(shared_constructor_field2_3.clone(), ({
                                                                        let source_ml_source2_array_read_value31721 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index31722 = 0f64;
                                                                            { let ml_reference_array1946 = source_ml_source2_array_read_value31721.clone(); let ml_reference_position1946 = source_ml_source2_array_read_index31722; let ml_reference_projection1946 = self::ml_array::optional_at(&*ml_reference_array1946.borrow(), self::ml_array::number_index(ml_reference_position1946)); ml_reference_projection1946 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) || !(self::test_java_script_semantic_same_value(shared_constructor_field3_2.clone(), ({
                                                                        let source_ml_source2_array_read_value31725 = shared_constructor_field10.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index31726 = 0f64;
                                                                            { let ml_reference_array1947 = source_ml_source2_array_read_value31725.clone(); let ml_reference_position1947 = source_ml_source2_array_read_index31726; let ml_reference_projection1947 = self::ml_array::optional_at(&*ml_reference_array1947.borrow(), self::ml_array::number_index(ml_reference_position1947)); ml_reference_projection1947 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || !(self::test_java_script_semantic_same_value(shared_constructor_field4_2.clone(), ({
                                                                        let source_ml_source2_array_read_value31729 = shared_constructor_field10.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index31730 = 1f64;
                                                                            { let ml_reference_array1948 = source_ml_source2_array_read_value31729.clone(); let ml_reference_position1948 = source_ml_source2_array_read_index31730; let ml_reference_projection1948 = self::ml_array::optional_at(&*ml_reference_array1948.borrow(), self::ml_array::number_index(ml_reference_position1948)); ml_reference_projection1948 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) {
                                                                        self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 97, 110, 100, 32, 116, 104, 105, 115, 65, 114, 103, 117, 109, 101, 110, 116])
                                                                    } else {
                                                                        if (suspended && match (match state_3.clone() {
                                                                            self::JavaScriptCollectionForEachState::CollectionForEachState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value31731) => {
                                                                                ((source_ml_source2_data_property_value31731.clone().unwrap_or_else(|| (None::<self::JavaScriptCollectionForEachCall>,))).0).clone()
                                                                            }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                            self::ml_string::Text::from_units(vec![97, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108])
                                                                        } else {
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
                                }
                                self::JavaScriptCallPending::WaitArrayCallback(
                                    destination_4,
                                    realm_4,
                                    state_4,
                                ) => {
                                    let state_4 = *state_4;
                                    match state_4.clone() {
                                        self::JavaScriptArrayCallbackState::ArrayCallbackState(
                                            shared_constructor_field1_4,
                                            shared_constructor_field2_4,
                                            shared_constructor_field3_3,
                                            shared_constructor_field4_3,
                                            shared_constructor_field5_2,
                                            _,
                                            _,
                                            _,
                                            shared_constructor_field6_2,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let shared_constructor_field2_4 =
                                                *shared_constructor_field2_4;
                                            let shared_constructor_field3_3 =
                                                *shared_constructor_field3_3;
                                            let shared_constructor_field4_3 =
                                                *shared_constructor_field4_3;
                                            let shared_constructor_field5_2 =
                                                *shared_constructor_field5_2;
                                            if !(shared_constructor_field8 > (0f64)) {
                                                self::ml_string::Text::from_units(vec![
                                                    65, 114, 114, 97, 121, 32, 99, 97, 108, 108,
                                                    98, 97, 99, 107, 32, 99, 111, 110, 116, 105,
                                                    110, 117, 97, 116, 105, 111, 110, 32, 114, 101,
                                                    116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                                                    97, 99, 116, 117, 97, 108, 32, 115, 111, 117,
                                                    114, 99, 101, 32, 111, 112, 101, 114, 97, 116,
                                                    105, 111, 110, 44, 32, 100, 101, 115, 116, 105,
                                                    110, 97, 116, 105, 111, 110, 32, 97, 110, 100,
                                                    32, 114, 101, 97, 108, 109,
                                                ])
                                            } else {
                                                {
                                                    let instruction_4 = {
                                                        let source_ml_source2_array_read_value31736 =
                                                            shared_constructor_field12.clone();
                                                        {
                                                            let source_ml_source2_array_read_index31737 =
                                                                (shared_constructor_field8 - 1f64);
                                                            {
                                                                let ml_reference_array1949 = source_ml_source2_array_read_value31736.clone();
                                                                let ml_reference_position1949 = source_ml_source2_array_read_index31737;
                                                                let ml_reference_projection1949 = self::ml_array::optional_at(&*ml_reference_array1949.borrow(), self::ml_array::number_index(ml_reference_position1949));
                                                                ml_reference_projection1949
                                                            }
                                                        }
                                                    };
                                                    if (match (instruction_4.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction_4.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::ArrayCallback(_, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 44, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                    } else {
                                                        if ((((((match instruction_4.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ArrayCallback(source_ml_source2_data_property_value31742, _, _) => {
                                                                Some((source_ml_source2_data_property_value31742,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0) != destination_4) || (realm_4 != shared_constructor_field11)) || (shared_constructor_field1_4 != ((((match instruction_4.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ArrayCallback(_, source_ml_source2_data_property_value31748, _) => {
                                                                Some((source_ml_source2_data_property_value31748.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml_string::Text,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()))) {
                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 44, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                        } else {
                                                            {
                                                                let error_4 = self::read_java_script_array_callback_state_error(heap.clone(), state_4.clone());
                                                                if (error_4 != (self::ml_string::Text::from_units(vec![]))) {
                                                                    error_4.clone()
                                                                } else {
                                                                    if ((((!(self::test_java_script_semantic_same_value(shared_constructor_field2_4.clone(), ({
                                                                        let source_ml_source2_array_read_value31752 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index31753 = ((((match instruction_4.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallInstruction::ArrayCallback(_, _, source_ml_source2_data_property_value31751) => {
                                                                                    Some((((source_ml_source2_data_property_value31751.clone().unwrap_or_else(|| (None::<f64>,))).0).clone(),))
                                                                                }
                                                                                _ => {
                                                                                    None::<(Option<f64>,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone()).clone().unwrap_or_else(|| 0f64);
                                                                            { let ml_reference_array1950 = source_ml_source2_array_read_value31752.clone(); let ml_reference_position1950 = source_ml_source2_array_read_index31753; let ml_reference_projection1950 = self::ml_array::optional_at(&*ml_reference_array1950.borrow(), self::ml_array::number_index(ml_reference_position1950)); ml_reference_projection1950 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) || !(self::test_java_script_semantic_same_value(shared_constructor_field3_3.clone(), ({
                                                                        let source_ml_source2_array_read_value31756 = shared_constructor_field10.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index31757 = 0f64;
                                                                            { let ml_reference_array1951 = source_ml_source2_array_read_value31756.clone(); let ml_reference_position1951 = source_ml_source2_array_read_index31757; let ml_reference_projection1951 = self::ml_array::optional_at(&*ml_reference_array1951.borrow(), self::ml_array::number_index(ml_reference_position1951)); ml_reference_projection1951 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || !(self::test_java_script_semantic_same_value(shared_constructor_field4_3.clone(), ({
                                                                        let source_ml_source2_array_read_value31760 = shared_constructor_field10.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index31761 = 1f64;
                                                                            { let ml_reference_array1952 = source_ml_source2_array_read_value31760.clone(); let ml_reference_position1952 = source_ml_source2_array_read_index31761; let ml_reference_projection1952 = self::ml_array::optional_at(&*ml_reference_array1952.borrow(), self::ml_array::number_index(ml_reference_position1952)); ml_reference_projection1952 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || !(self::test_java_script_semantic_same_value(shared_constructor_field5_2.clone(), shared_constructor_field7.clone()))) || (shared_constructor_field6_2 != ((({ let ml_reference_array1953 = shared_constructor_field10.clone(); let ml_reference_projection1953 = ml_reference_array1953.borrow().len() as f64; ml_reference_projection1953 }) >= (2f64))))) {
                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 116, 104, 101, 105, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 99, 97, 108, 108, 98, 97, 99, 107, 44, 32, 116, 104, 105, 115, 65, 114, 103, 32, 111, 114, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108])
                                                                    } else {
                                                                        if (suspended && match (match state_4.clone() {
                                                                            self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value31770) => {
                                                                                ((source_ml_source2_data_property_value31770.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                                                                            }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                            self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 103, 101, 116, 116, 101, 114, 44, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 111, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 97, 110, 32, 114, 101, 116, 97, 105, 110, 32, 97, 32, 99, 104, 105, 108, 100, 32, 102, 114, 97, 109, 101])
                                                                        } else {
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
                                }
                                self::JavaScriptCallPending::WaitArrayAssignment(
                                    destination_5,
                                    realm_5,
                                    state_5,
                                ) => {
                                    let state_5 = *state_5;
                                    if !(shared_constructor_field8 > (0f64)) {
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 97, 115, 115, 105, 103, 110,
                                            109, 101, 110, 116, 32, 114, 101, 116, 97, 105, 110,
                                            115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                            32, 115, 111, 117, 114, 99, 101, 32, 83, 101, 116, 32,
                                            105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110,
                                            44, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105,
                                            111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109,
                                        ])
                                    } else {
                                        {
                                            let instruction_5 = {
                                                let source_ml_source2_array_read_value31775 =
                                                    shared_constructor_field12.clone();
                                                {
                                                    let source_ml_source2_array_read_index31776 =
                                                        (shared_constructor_field8 - 1f64);
                                                    {
                                                        let ml_reference_array1954 =
                                                            source_ml_source2_array_read_value31775
                                                                .clone();
                                                        let ml_reference_position1954 =
                                                            source_ml_source2_array_read_index31776;
                                                        let ml_reference_projection1954 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array1954.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position1954,
                                                                ),
                                                            );
                                                        ml_reference_projection1954
                                                    }
                                                }
                                            };
                                            if (match (instruction_5.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } || match instruction_5
                                                .clone()
                                                .expect("a proven present value")
                                            {
                                                self::JavaScriptCallInstruction::OrdinarySet(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => false,
                                                _ => true,
                                            }) {
                                                self::ml_string::Text::from_units(vec![
                                                    65, 114, 114, 97, 121, 32, 97, 115, 115, 105,
                                                    103, 110, 109, 101, 110, 116, 32, 114, 101,
                                                    116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                                                    97, 99, 116, 117, 97, 108, 32, 115, 111, 117,
                                                    114, 99, 101, 32, 83, 101, 116, 32, 105, 110,
                                                    115, 116, 114, 117, 99, 116, 105, 111, 110, 44,
                                                    32, 100, 101, 115, 116, 105, 110, 97, 116, 105,
                                                    111, 110, 32, 97, 110, 100, 32, 114, 101, 97,
                                                    108, 109,
                                                ])
                                            } else {
                                                if ((shared_constructor_field11 != realm_5) || ((((match instruction_5.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::OrdinarySet(source_ml_source2_data_property_value31785, _, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value31785,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0) != destination_5)) {
                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                } else {
                                                    {
                                                        let error_5 = self::read_java_script_array_assignment_state_error(heap.clone(), state_5.clone());
                                                        if (error_5 != (self::ml_string::Text::from_units(vec![]))) {
                                                            error_5.clone()
                                                        } else {
                                                            {
                                                                let receiver = ({
                                                                    let source_ml_source2_array_read_value31788 = shared_constructor_field9.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index31789 = ((match instruction_5.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, source_ml_source2_data_property_value31787) => {
                                                                                Some((source_ml_source2_data_property_value31787,))
                                                                            }
                                                                            _ => {
                                                                                None::<(f64,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0;
                                                                        { let ml_reference_array1955 = source_ml_source2_array_read_value31788.clone(); let ml_reference_position1955 = source_ml_source2_array_read_index31789; let ml_reference_projection1955 = self::ml_array::optional_at(&*ml_reference_array1955.borrow(), self::ml_array::number_index(ml_reference_position1955)); ml_reference_projection1955 }
                                                                    }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                {
                                                                    let definition = match state_5.clone() {
                                                                        self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(source_ml_source2_data_property_value31790, _) => {
                                                                            let source_ml_source2_data_property_value31790 = *source_ml_source2_data_property_value31790;
                                                                            source_ml_source2_data_property_value31790.clone()
                                                                        }
                                                                        self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(source_ml_source2_data_property_value31791, _, _) => {
                                                                            let source_ml_source2_data_property_value31791 = *source_ml_source2_data_property_value31791;
                                                                            source_ml_source2_data_property_value31791.clone()
                                                                        }
                                                                        self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(source_ml_source2_data_property_value31792, _, _) => {
                                                                            let source_ml_source2_data_property_value31792 = *source_ml_source2_data_property_value31792;
                                                                            source_ml_source2_data_property_value31792.clone()
                                                                        }
                                                                    };
                                                                    match definition.clone() {
                                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(shared_constructor_field1_5, shared_constructor_field2_5, shared_constructor_field3_4, _, _, _) => {
                                                                            let shared_constructor_field2_5 = *shared_constructor_field2_5;
                                                                            if ((((match receiver.clone() {
                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            } || (((((match receiver.clone() {
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value31797, _) => {
                                                                                    Some((source_ml_source2_data_property_value31797.clone(),))
                                                                                }
                                                                                _ => {
                                                                                    None::<(self::ml::Big,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field1_5)) || !(self::test_java_script_semantic_same_value(shared_constructor_field2_5.clone(), ({
                                                                                let source_ml_source2_array_read_value31802 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index31803 = ((match instruction_5.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallInstruction::OrdinarySet(_, _, source_ml_source2_data_property_value31801, _, _) => {
                                                                                            Some((source_ml_source2_data_property_value31801,))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(f64,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0;
                                                                                    { let ml_reference_array1956 = source_ml_source2_array_read_value31802.clone(); let ml_reference_position1956 = source_ml_source2_array_read_index31803; let ml_reference_projection1956 = self::ml_array::optional_at(&*ml_reference_array1956.borrow(), self::ml_array::number_index(ml_reference_position1956)); ml_reference_projection1956 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || !(self::test_java_script_semantic_same_value((({ let ml_reference_value1957 = shared_constructor_field3_4.borrow().clone(); ml_reference_value1957 }).0).8, ({
                                                                                let source_ml_source2_array_read_value31807 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index31808 = ((match instruction_5.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallInstruction::OrdinarySet(_, _, _, source_ml_source2_data_property_value31806, _) => {
                                                                                            Some((source_ml_source2_data_property_value31806,))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(f64,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0;
                                                                                    { let ml_reference_array1958 = source_ml_source2_array_read_value31807.clone(); let ml_reference_position1958 = source_ml_source2_array_read_index31808; let ml_reference_projection1958 = self::ml_array::optional_at(&*ml_reference_array1958.borrow(), self::ml_array::number_index(ml_reference_position1958)); ml_reference_projection1958 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || !(self::test_java_script_semantic_same_value(match state_5.clone() {
                                                                                self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(_, source_ml_source2_data_property_value31809) => {
                                                                                    let source_ml_source2_data_property_value31809 = *source_ml_source2_data_property_value31809;
                                                                                    source_ml_source2_data_property_value31809.clone()
                                                                                }
                                                                                self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(_, source_ml_source2_data_property_value31810, _) => {
                                                                                    let source_ml_source2_data_property_value31810 = *source_ml_source2_data_property_value31810;
                                                                                    source_ml_source2_data_property_value31810.clone()
                                                                                }
                                                                                self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(_, source_ml_source2_data_property_value31811, _) => {
                                                                                    let source_ml_source2_data_property_value31811 = *source_ml_source2_data_property_value31811;
                                                                                    source_ml_source2_data_property_value31811.clone()
                                                                                }
                                                                            }, shared_constructor_field7.clone()))) {
                                                                                self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 105, 116, 115, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 107, 101, 121, 44, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108])
                                                                            } else {
                                                                                {
                                                                                    let descriptor = shared_constructor_field3_4.clone();
                                                                                    if (((((!((({ let ml_reference_value1959 = descriptor.borrow().clone(); ml_reference_value1959 }).0).9) || (({ let ml_reference_value1960 = descriptor.borrow().clone(); ml_reference_value1960 }).0).11) || (({ let ml_reference_value1961 = descriptor.borrow().clone(); ml_reference_value1961 }).0).3) || (({ let ml_reference_value1962 = descriptor.borrow().clone(); ml_reference_value1962 }).0).1) || (({ let ml_reference_value1963 = descriptor.borrow().clone(); ml_reference_value1963 }).0).5) || (({ let ml_reference_value1964 = descriptor.borrow().clone(); ml_reference_value1964 }).0).7) {
                                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 45, 111, 110, 108, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114])
                                                                                    } else {
                                                                                        if (suspended && match state_5.clone() {
                                                                                            self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(_, _, _) => {
                                                                                                false
                                                                                            }
                                                                                            _ => {
                                                                                                true
                                                                                            }
                                                                                        }) {
                                                                                            self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 97, 110, 32, 104, 97, 118, 101, 32, 97, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108])
                                                                                        } else {
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
        }
    }
}
