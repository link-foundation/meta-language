// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptObjectCopyCall;
use super::JavaScriptObjectEnumerationState;
use super::JavaScriptObjectFromEntriesCall;
use super::JavaScriptObjectFromEntriesErrorRequest;
use super::JavaScriptObjectFromEntriesState;
use super::JavaScriptObjectIntegrityState;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptCallPendingObjectObservationErrorLoop1323Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_object_enumeration_state_error;
use super::read_java_script_object_from_entries_state_error;
use super::read_java_script_object_integrity_state_error;
use super::read_java_script_object_spread_state_error;
use super::source_ml_source1_temporary_read_java_script_call_pending_object_observation_error_loop1323;
use super::test_java_script_pending_call_constructor_failure;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_pending_object_observation_error(
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
            shared_constructor_field10,
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field11,
            _,
            _,
        ) => {
            match frame.clone() {
                self::JavaScriptCallFrame::CallFrame(
                    _,
                    shared_constructor_field7,
                    shared_constructor_field8,
                    shared_constructor_field9,
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
                            shared_constructor_field5,
                            shared_constructor_field6,
                            _,
                            _,
                            _,
                        ) => {
                            let shared_constructor_field6 = *shared_constructor_field6;
                            match pending.clone() {
                                self::JavaScriptCallPending::WaitObjectSpread(
                                    destination,
                                    realm,
                                    state,
                                ) => {
                                    let state = *state;
                                    match state.clone() {
                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(
                                            shared_constructor_field1,
                                            shared_constructor_field2,
                                            shared_constructor_field3,
                                            _,
                                            _,
                                            shared_constructor_field4,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let shared_constructor_field1 =
                                                *shared_constructor_field1;
                                            let shared_constructor_field2 =
                                                *shared_constructor_field2;
                                            if !(shared_constructor_field7 > (0f64)) {
                                                self::ml_string::Text::from_units(vec![
                                                    111, 98, 106, 101, 99, 116, 32, 115, 112, 114,
                                                    101, 97, 100, 32, 114, 101, 116, 97, 105, 110,
                                                    115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                                    97, 108, 32, 115, 111, 117, 114, 99, 101, 32,
                                                    105, 110, 115, 116, 114, 117, 99, 116, 105,
                                                    111, 110, 44, 32, 100, 101, 115, 116, 105, 110,
                                                    97, 116, 105, 111, 110, 32, 97, 110, 100, 32,
                                                    114, 101, 97, 108, 109,
                                                ])
                                            } else {
                                                {
                                                    let instruction = {
                                                        let source_ml_source2_array_read_value30856 =
                                                            shared_constructor_field11.clone();
                                                        {
                                                            let source_ml_source2_array_read_index30857 =
                                                                (shared_constructor_field7 - 1f64);
                                                            {
                                                                let ml_reference_array1835 = source_ml_source2_array_read_value30856.clone();
                                                                let ml_reference_position1835 = source_ml_source2_array_read_index30857;
                                                                let ml_reference_projection1835 = self::ml_array::optional_at(&*ml_reference_array1835.borrow(), self::ml_array::number_index(ml_reference_position1835));
                                                                ml_reference_projection1835
                                                            }
                                                        }
                                                    };
                                                    if (match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::ObjectSpread(_, _, _, _, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                    } else {
                                                        if (((((((match instruction.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ObjectSpread(_, _, _, source_ml_source2_data_property_value30865, _, _) => {
                                                                Some((source_ml_source2_data_property_value30865.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml::Big,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()) != realm) || ((((match instruction.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ObjectSpread(source_ml_source2_data_property_value30867, _, _, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value30867,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0) != destination)) || !(self::ml_array::includes(&{ let ml_reference_value1836 = shared_constructor_field5.borrow().clone(); ml_reference_value1836 }, &realm.clone(), 0f64, false, |a, b| a == b))) {
                                                            self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                        } else {
                                                            {
                                                                let exotic_key = match state.clone() {
                                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value30870, _, _) => {
                                                                        ((source_ml_source2_data_property_value30870.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                    }
                                                                };
                                                                if ({
                                                                    let source_ml_source2_comparison_operand30873 = match state.clone() {
                                                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value30871, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value30871.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_comparison_operand30874 = (((match instruction.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallInstruction::ObjectSpread(_, _, _, _, _, source_ml_source2_data_property_value30872) => {
                                                                                Some((((source_ml_source2_data_property_value30872.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone(),))
                                                                            }
                                                                            _ => {
                                                                                None::<(Option<self::ml_string::Text>,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0).clone();
                                                                        !(match (source_ml_source2_comparison_operand30873.clone(), source_ml_source2_comparison_operand30874.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                                                                    }
                                                                } || ({
                                                                    let source_ml_source2_comparison_operand30876 = (((match instruction.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallInstruction::ObjectSpread(_, _, _, _, _, source_ml_source2_data_property_value30875) => {
                                                                            Some((((source_ml_source2_data_property_value30875.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(Option<self::ml_string::Text>,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone();
                                                                    match (source_ml_source2_comparison_operand30876.clone(), self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                } && (match (exotic_key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(exotic_key.clone().expect("a proven present value"), shared_constructor_field6.clone()))))) {
                                                                    self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 99, 111, 112, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 111, 114, 32, 83, 101, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 83, 121, 109, 98, 111, 108])
                                                                } else {
                                                                    {
                                                                        let error = self::read_java_script_object_spread_state_error(heap.clone(), state.clone());
                                                                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                            error.clone()
                                                                        } else {
                                                                            if (!(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), ({
                                                                                let source_ml_source2_array_read_value30882 = shared_constructor_field8.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index30883 = ((match instruction.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallInstruction::ObjectSpread(_, source_ml_source2_data_property_value30881, _, _, _, _) => {
                                                                                            Some((source_ml_source2_data_property_value30881,))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(f64,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0;
                                                                                    { let ml_reference_array1837 = source_ml_source2_array_read_value30882.clone(); let ml_reference_position1837 = source_ml_source2_array_read_index30883; let ml_reference_projection1837 = self::ml_array::optional_at(&*ml_reference_array1837.borrow(), self::ml_array::number_index(ml_reference_position1837)); ml_reference_projection1837 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), ({
                                                                                let source_ml_source2_array_read_value30887 = shared_constructor_field8.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index30888 = ((match instruction.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallInstruction::ObjectSpread(_, _, source_ml_source2_data_property_value30886, _, _, _) => {
                                                                                            Some((source_ml_source2_data_property_value30886,))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(f64,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0;
                                                                                    { let ml_reference_array1838 = source_ml_source2_array_read_value30887.clone(); let ml_reference_position1838 = source_ml_source2_array_read_index30888; let ml_reference_projection1838 = self::ml_array::optional_at(&*ml_reference_array1838.borrow(), self::ml_array::number_index(ml_reference_position1838)); ml_reference_projection1838 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) {
                                                                                self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 32, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 112, 117, 116])
                                                                            } else {
                                                                                {
                                                                                    let excluded = ((((match instruction.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallInstruction::ObjectSpread(_, _, _, _, source_ml_source2_data_property_value30889, _) => {
                                                                                            Some((((source_ml_source2_data_property_value30889.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone()).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                                                                    {
                                                                                        let excluded_values = shared_constructor_field3.clone();
                                                                                        if (({ let ml_reference_array1839 = excluded.clone(); let ml_reference_projection1839 = ml_reference_array1839.borrow().len() as f64; ml_reference_projection1839 }) != ({ let ml_reference_array1840 = excluded_values.clone(); let ml_reference_projection1840 = ml_reference_array1840.borrow().len() as f64; ml_reference_projection1840 })) {
                                                                                            self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 99, 111, 112, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 99, 108, 117, 100, 101, 100, 45, 107, 101, 121, 32, 111, 112, 101, 114, 97, 110, 100])
                                                                                        } else {
                                                                                            {
                                                                                                let at = 0f64;
                                                                                                {
                                                                                                    let source_ml_source2_s30891 = self::source_ml_source1_temporary_read_java_script_call_pending_object_observation_error_loop1323(frame.clone(), excluded.clone(), excluded_values.clone(), at);
                                                                                                    match source_ml_source2_s30891.clone() {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingObjectObservationErrorLoop1323Result::MlSource1TemporaryReadJavaScriptCallPendingObjectObservationErrorLoop1323Done => {
                                                                                                            if ((suspended && (shared_constructor_field4 != (self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114])))) && match (match state.clone() {
                                                                                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value30894) => {
                                                                                                                    ((source_ml_source2_data_property_value30894.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectCopyCall>,))).0).clone()
                                                                                                                }
                                                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                                self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 99, 111, 112, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 111, 110, 108, 121, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114, 115, 32, 115, 101, 116, 116, 101, 114, 115, 32, 97, 110, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115])
                                                                                                            } else {
                                                                                                                self::ml_string::Text::from_units(vec![])
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingObjectObservationErrorLoop1323Result::MlSource1TemporaryReadJavaScriptCallPendingObjectObservationErrorLoop1323Return(source_ml_source1_temporary_result1323) => {
                                                                                                            source_ml_source1_temporary_result1323.clone()
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
                                self::JavaScriptCallPending::WaitObjectEnumeration(
                                    destination_2,
                                    realm_2,
                                    state_2,
                                ) => {
                                    let state_2 = *state_2;
                                    match state_2.clone() {
                                        self::JavaScriptObjectEnumerationState::ObjectEnumeration(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, _, _, _, shared_constructor_field4_2, _, _, _, _, _, _) => {
                                            let shared_constructor_field2_2 = *shared_constructor_field2_2;
                                            let shared_constructor_field3_2 = *shared_constructor_field3_2;
                                            if !((shared_constructor_field7 > (0f64))) {
                                                self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 105, 110, 112, 117, 116, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                            } else {
                                                {
                                                    let instruction_2 = {
                                                        let source_ml_source2_array_read_value30899 = shared_constructor_field11.clone();
                                                        {
                                                            let source_ml_source2_array_read_index30900 = (shared_constructor_field7 - 1f64);
                                                            { let ml_reference_array1841 = source_ml_source2_array_read_value30899.clone(); let ml_reference_position1841 = source_ml_source2_array_read_index30900; let ml_reference_projection1841 = self::ml_array::optional_at(&*ml_reference_array1841.borrow(), self::ml_array::number_index(ml_reference_position1841)); ml_reference_projection1841 }
                                                        }
                                                    };
                                                    if (match (instruction_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction_2.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::ObjectEnumeration(_, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 105, 110, 112, 117, 116, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                    } else {
                                                        if ((((((((match instruction_2.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ObjectEnumeration(source_ml_source2_data_property_value30905, _, _) => {
                                                                Some((source_ml_source2_data_property_value30905,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0) != destination_2) || (((((match instruction_2.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ObjectEnumeration(_, source_ml_source2_data_property_value30908, _) => {
                                                                Some((source_ml_source2_data_property_value30908.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml_string::Text,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field1_2)) || (realm_2 != shared_constructor_field10)) || !(self::test_java_script_semantic_same_value(({
                                                            let source_ml_source2_array_read_value30913 = shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_array_read_index30914 = 0f64;
                                                                { let ml_reference_array1842 = source_ml_source2_array_read_value30913.clone(); let ml_reference_position1842 = source_ml_source2_array_read_index30914; let ml_reference_projection1842 = self::ml_array::optional_at(&*ml_reference_array1842.borrow(), self::ml_array::number_index(ml_reference_position1842)); ml_reference_projection1842 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2_2.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                            let source_ml_source2_array_read_value30918 = shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_array_read_index30919 = ((match instruction_2.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallInstruction::ObjectEnumeration(_, _, source_ml_source2_data_property_value30917) => {
                                                                        Some((source_ml_source2_data_property_value30917,))
                                                                    }
                                                                    _ => {
                                                                        None::<(f64,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0;
                                                                { let ml_reference_array1843 = source_ml_source2_array_read_value30918.clone(); let ml_reference_position1843 = source_ml_source2_array_read_index30919; let ml_reference_projection1843 = self::ml_array::optional_at(&*ml_reference_array1843.borrow(), self::ml_array::number_index(ml_reference_position1843)); ml_reference_projection1843 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field3_2.clone()))) {
                                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 105, 110, 112, 117, 116, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                        } else {
                                                            {
                                                                let invalid = self::read_java_script_object_enumeration_state_error(heap.clone(), state_2.clone());
                                                                if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                    invalid.clone()
                                                                } else {
                                                                    if (suspended && (shared_constructor_field4_2 != (self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114])))) {
                                                                        self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 103, 101, 116, 116, 101, 114, 32, 112, 104, 97, 115, 101])
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
                                self::JavaScriptCallPending::WaitObjectIntegrity(
                                    destination_3,
                                    realm_3,
                                    state_3,
                                ) => {
                                    let state_3 = *state_3;
                                    match state_3.clone() {
                                        self::JavaScriptObjectIntegrityState::ObjectIntegrity(
                                            shared_constructor_field1_3,
                                            shared_constructor_field2_3,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let shared_constructor_field2_3 =
                                                *shared_constructor_field2_3;
                                            if !(shared_constructor_field7 > (0f64)) {
                                                self::ml_string::Text::from_units(vec![
                                                    105, 110, 116, 101, 103, 114, 105, 116, 121,
                                                    32, 99, 104, 97, 110, 103, 101, 115, 32, 114,
                                                    101, 116, 97, 105, 110, 32, 97, 99, 116, 117,
                                                    97, 108, 32, 115, 111, 117, 114, 99, 101, 32,
                                                    111, 112, 101, 114, 97, 116, 105, 111, 110, 32,
                                                    105, 110, 112, 117, 116, 32, 97, 110, 100, 32,
                                                    110, 111, 32, 105, 110, 118, 101, 110, 116,
                                                    101, 100, 32, 99, 104, 105, 108, 100, 32, 99,
                                                    97, 108, 108,
                                                ])
                                            } else {
                                                {
                                                    let instruction_3 = {
                                                        let source_ml_source2_array_read_value30927 =
                                                            shared_constructor_field11.clone();
                                                        {
                                                            let source_ml_source2_array_read_index30928 =
                                                                (shared_constructor_field7 - 1f64);
                                                            {
                                                                let ml_reference_array1844 = source_ml_source2_array_read_value30927.clone();
                                                                let ml_reference_position1844 = source_ml_source2_array_read_index30928;
                                                                let ml_reference_projection1844 = self::ml_array::optional_at(&*ml_reference_array1844.borrow(), self::ml_array::number_index(ml_reference_position1844));
                                                                ml_reference_projection1844
                                                            }
                                                        }
                                                    };
                                                    if (match (instruction_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction_3.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::ObjectObservation(_, _, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 114, 105, 116, 121, 32, 99, 104, 97, 110, 103, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 105, 110, 112, 117, 116, 32, 97, 110, 100, 32, 110, 111, 32, 105, 110, 118, 101, 110, 116, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108])
                                                    } else {
                                                        if ((((((((match instruction_3.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ObjectObservation(source_ml_source2_data_property_value30934, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value30934,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0) != destination_3) || (((((match instruction_3.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ObjectObservation(_, source_ml_source2_data_property_value30937, _, _) => {
                                                                Some((source_ml_source2_data_property_value30937.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml_string::Text,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field1_3)) || (realm_3 != shared_constructor_field10)) || !(self::test_java_script_semantic_same_value(({
                                                            let source_ml_source2_array_read_value30943 = shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_array_read_index30944 = ((match instruction_3.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallInstruction::ObjectObservation(_, _, source_ml_source2_data_property_value30942, _) => {
                                                                        Some((source_ml_source2_data_property_value30942,))
                                                                    }
                                                                    _ => {
                                                                        None::<(f64,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0;
                                                                { let ml_reference_array1845 = source_ml_source2_array_read_value30943.clone(); let ml_reference_position1845 = source_ml_source2_array_read_index30944; let ml_reference_projection1845 = self::ml_array::optional_at(&*ml_reference_array1845.borrow(), self::ml_array::number_index(ml_reference_position1845)); ml_reference_projection1845 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2_3.clone()))) || suspended) {
                                                            self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 114, 105, 116, 121, 32, 99, 104, 97, 110, 103, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 105, 110, 112, 117, 116, 32, 97, 110, 100, 32, 110, 111, 32, 105, 110, 118, 101, 110, 116, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108])
                                                        } else {
                                                            {
                                                                let invalid_2 = self::read_java_script_object_integrity_state_error(heap.clone(), state_3.clone());
                                                                if (invalid_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                    invalid_2.clone()
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
                                self::JavaScriptCallPending::WaitObjectFromEntries(
                                    destination_4,
                                    realm_4,
                                    state_4,
                                    error_2,
                                ) => {
                                    let state_4 = *state_4;
                                    match state_4.clone() {
                                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(shared_constructor_field1_4, shared_constructor_field2_4, shared_constructor_field3_3, shared_constructor_field4_3, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            let shared_constructor_field1_4 = *shared_constructor_field1_4;
                                            let shared_constructor_field2_4 = *shared_constructor_field2_4;
                                            let shared_constructor_field3_3 = *shared_constructor_field3_3;
                                            let shared_constructor_field4_3 = *shared_constructor_field4_3;
                                            if !((shared_constructor_field7 > (0f64))) {
                                                self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 112, 117, 116, 32, 114, 101, 97, 108, 109, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 97, 110, 100, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108, 115])
                                            } else {
                                                {
                                                    let instruction_4 = {
                                                        let source_ml_source2_array_read_value30950 = shared_constructor_field11.clone();
                                                        {
                                                            let source_ml_source2_array_read_index30951 = (shared_constructor_field7 - 1f64);
                                                            { let ml_reference_array1846 = source_ml_source2_array_read_value30950.clone(); let ml_reference_position1846 = source_ml_source2_array_read_index30951; let ml_reference_projection1846 = self::ml_array::optional_at(&*ml_reference_array1846.borrow(), self::ml_array::number_index(ml_reference_position1846)); ml_reference_projection1846 }
                                                        }
                                                    };
                                                    if (match (instruction_4.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction_4.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::ObjectFromEntries(_, _, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 112, 117, 116, 32, 114, 101, 97, 108, 109, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 97, 110, 100, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108, 115])
                                                    } else {
                                                        if (((((((((match instruction_4.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ObjectFromEntries(source_ml_source2_data_property_value30957, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value30957,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0) != destination_4) || (realm_4 != shared_constructor_field10)) || !(self::test_java_script_semantic_same_value(({
                                                            let source_ml_source2_array_read_value30962 = shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_array_read_index30963 = ((match instruction_4.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallInstruction::ObjectFromEntries(_, source_ml_source2_data_property_value30961, _, _) => {
                                                                        Some((source_ml_source2_data_property_value30961,))
                                                                    }
                                                                    _ => {
                                                                        None::<(f64,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0;
                                                                { let ml_reference_array1847 = source_ml_source2_array_read_value30962.clone(); let ml_reference_position1847 = source_ml_source2_array_read_index30963; let ml_reference_projection1847 = self::ml_array::optional_at(&*ml_reference_array1847.borrow(), self::ml_array::number_index(ml_reference_position1847)); ml_reference_projection1847 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field1_4.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                            let source_ml_source2_array_read_value30967 = shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_array_read_index30968 = ((match instruction_4.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallInstruction::ObjectFromEntries(_, _, source_ml_source2_data_property_value30966, _) => {
                                                                        Some((source_ml_source2_data_property_value30966,))
                                                                    }
                                                                    _ => {
                                                                        None::<(f64,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0;
                                                                { let ml_reference_array1848 = source_ml_source2_array_read_value30967.clone(); let ml_reference_position1848 = source_ml_source2_array_read_index30968; let ml_reference_projection1848 = self::ml_array::optional_at(&*ml_reference_array1848.borrow(), self::ml_array::number_index(ml_reference_position1848)); ml_reference_projection1848 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2_4.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                            let source_ml_source2_array_read_value30972 = shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_array_read_index30973 = ((match instruction_4.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallInstruction::ObjectFromEntries(_, _, _, source_ml_source2_data_property_value30971) => {
                                                                        Some((source_ml_source2_data_property_value30971,))
                                                                    }
                                                                    _ => {
                                                                        None::<(f64,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0;
                                                                { let ml_reference_array1849 = source_ml_source2_array_read_value30972.clone(); let ml_reference_position1849 = source_ml_source2_array_read_index30973; let ml_reference_projection1849 = self::ml_array::optional_at(&*ml_reference_array1849.borrow(), self::ml_array::number_index(ml_reference_position1849)); ml_reference_projection1849 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field3_3.clone()))) || !(self::test_java_script_semantic_same_value(shared_constructor_field6.clone(), shared_constructor_field4_3.clone()))) {
                                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 112, 117, 116, 32, 114, 101, 97, 108, 109, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 97, 110, 100, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108, 115])
                                                        } else {
                                                            {
                                                                let invalid_3 = self::read_java_script_object_from_entries_state_error(heap.clone(), state_4.clone());
                                                                if (invalid_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                                    invalid_3.clone()
                                                                } else {
                                                                    if (suspended && (match (match state_4.clone() {
                                                                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value30977, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value30977.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesCall>,))).0).clone()
                                                                        }
                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match (((error_2.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) {
                                                                        self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 97, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 101, 115, 116])
                                                                    } else {
                                                                        {
                                                                            let setup = ((error_2.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone();
                                                                            if (!(match (setup.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(self::test_java_script_pending_call_constructor_failure(program.clone(), heap.clone(), ({
                                                                                let source_ml_source2_optional_chain_receiver30979 = (match state_4.clone() {
                                                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value30978, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value30978.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesCall>,))).0).clone()
                                                                                    }
                                                                                },);
                                                                                if match (source_ml_source2_optional_chain_receiver30979.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    Some({
                                                                                        let source_ml_source1_temporary_optional_chain1322 = ((source_ml_source2_optional_chain_receiver30979.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source1_temporary_optional_chain1322.clone() {
                                                                                            self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(source_ml_source2_data_property_value30980, _, _) => {
                                                                                                let source_ml_source2_data_property_value30980 = *source_ml_source2_data_property_value30980;
                                                                                                source_ml_source2_data_property_value30980.clone()
                                                                                            }
                                                                                        }
                                                                                    })
                                                                                } else {
                                                                                    None::<self::JavaScriptSemanticValue>
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), setup.clone().expect("a proven present value")))) {
                                                                                self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 45, 99, 97, 108, 108, 32, 102, 97, 105, 108, 117, 114, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 101, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 107, 105, 110, 100, 32, 69, 114, 114, 111, 114, 32, 114, 101, 97, 108, 109, 32, 97, 110, 100, 32, 114, 101, 97, 115, 111, 110])
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
                                _ => self::ml_string::Text::from_units(vec![]),
                            }
                        }
                    }
                }
            }
        }
    }
}
