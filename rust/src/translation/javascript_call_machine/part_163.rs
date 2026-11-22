// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptFunctionBindCall;
use super::JavaScriptFunctionBindState;
use super::JavaScriptFunctionTextResult;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveWrapperState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_construction_state_error;
use super::read_java_script_function_bind_state_error;
use super::read_java_script_function_text;
use super::read_java_script_primitive_wrapper_state_error;
use super::test_java_script_primitive_intrinsic_mode;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_pending_callable_construction_error(
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
            shared_constructor_field7,
            shared_constructor_field8,
            _,
            _,
            _,
            _,
            shared_constructor_field9,
            _,
            _,
        ) => {
            match frame.clone() {
                self::JavaScriptCallFrame::CallFrame(
                    _,
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
                    match pending.clone() {
                        self::JavaScriptCallPending::WaitConstruction(
                            pending_destination,
                            realm,
                            state,
                            held_arguments,
                        ) => {
                            let state = *state;
                            match state.clone() {
                                self::JavaScriptConstructionState::ConstructionState(
                                    shared_constructor_field1,
                                    shared_constructor_field2,
                                    shared_constructor_field3,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let shared_constructor_field1 = *shared_constructor_field1;
                                    let shared_constructor_field2 = *shared_constructor_field2;
                                    {
                                        let invalid_instruction =
                                            self::ml_string::Text::from_units(vec![
                                                99, 111, 110, 115, 116, 114, 117, 99, 116, 105,
                                                111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                                                101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97,
                                                108, 32, 104, 101, 108, 100, 32, 111, 114, 100,
                                                101, 114, 101, 100, 32, 97, 114, 103, 117, 109,
                                                101, 110, 116,
                                            ]);
                                        if !(shared_constructor_field4 > (0f64)) {
                                            invalid_instruction.clone()
                                        } else {
                                            {
                                                let instruction = {
                                                    let source_ml_source2_array_read_value31123 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_array_read_index31124 =
                                                            (shared_constructor_field4 - 1f64);
                                                        {
                                                            let ml_reference_array1866 = source_ml_source2_array_read_value31123.clone();
                                                            let ml_reference_position1866 = source_ml_source2_array_read_index31124;
                                                            let ml_reference_projection1866 =
                                                                self::ml_array::optional_at(
                                                                    &*ml_reference_array1866
                                                                        .borrow(),
                                                                    self::ml_array::number_index(
                                                                        ml_reference_position1866,
                                                                    ),
                                                                );
                                                            ml_reference_projection1866
                                                        }
                                                    }
                                                };
                                                {
                                                    let directory = match program.clone() {
                                                        self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value31125, _) => {
                                                            ((source_ml_source2_data_property_value31125.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                        }
                                                    };
                                                    if ((match (directory.clone(), None::<()>) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    } || match (
                                                        instruction.clone(),
                                                        None::<()>,
                                                    ) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    }) || (realm != shared_constructor_field7))
                                                    {
                                                        invalid_instruction.clone()
                                                    } else {
                                                        {
                                                            let source_ml_source2_s31164 = match instruction.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::ArgumentListCall(destination, _, callee, receiver, mode) => {
                                                                    if ((((mode != (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]))) || (destination != pending_destination)) || !(self::test_java_script_semantic_same_value(({
                                                                        let source_ml_source2_array_read_value31167 = shared_constructor_field5.clone();
                                                                        { let ml_reference_array1867 = source_ml_source2_array_read_value31167.clone(); let ml_reference_position1867 = callee; let ml_reference_projection1867 = self::ml_array::optional_at(&*ml_reference_array1867.borrow(), self::ml_array::number_index(ml_reference_position1867)); ml_reference_projection1867 }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field1.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                                        let source_ml_source2_array_read_value31171 = shared_constructor_field5.clone();
                                                                        { let ml_reference_array1868 = source_ml_source2_array_read_value31171.clone(); let ml_reference_position1868 = receiver; let ml_reference_projection1868 = self::ml_array::optional_at(&*ml_reference_array1868.borrow(), self::ml_array::number_index(ml_reference_position1868)); ml_reference_projection1868 }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2.clone()))) {
                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328Return(invalid_instruction.clone())
                                                                    } else {
                                                                        if ((match (((held_arguments.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (({
                                                                            let source_ml_source2_nullish_field_receiver31177 = (((held_arguments.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,))).0).clone(),);
                                                                            if match (source_ml_source2_nullish_field_receiver31177.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source2_nullish_field_present31178 = ((source_ml_source2_nullish_field_receiver31177.clone()).0).clone().expect("a proven present value");
                                                                                    { let ml_reference_array1869 = source_ml_source2_nullish_field_present31178.clone(); let ml_reference_projection1869 = ml_reference_array1869.borrow().len() as f64; ml_reference_projection1869 }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }) != ({ let ml_reference_array1870 = shared_constructor_field3.clone(); let ml_reference_projection1870 = ml_reference_array1870.borrow().len() as f64; ml_reference_projection1870 }))) || {
                                                                            let source_ml_source2_nullish_field_receiver31180 = (((held_arguments.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,))).0).clone(),);
                                                                            if match (source_ml_source2_nullish_field_receiver31180.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source2_nullish_field_present31181 = ((source_ml_source2_nullish_field_receiver31180.clone()).0).clone().expect("a proven present value");
                                                                                    { let ml_values1871 = { let ml_reference_value1872 = source_ml_source2_nullish_field_present31181.borrow().clone(); ml_reference_value1872 }; let ml_callback1871 = { let state = state.clone(); std::rc::Rc::new(move |value: self::JavaScriptSemanticValue, index: f64| -> bool { !(self::test_java_script_semantic_same_value(value.clone(), ({
                                                                                        let source_ml_source2_array_read_value31183 = match state.clone() {
                                                                                            self::JavaScriptConstructionState::ConstructionState(_, _, source_ml_source2_data_property_value31182, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value31182.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_array_read_index31184 = index;
                                                                                            { let ml_reference_array1873 = source_ml_source2_array_read_value31183.clone(); let ml_reference_position1873 = source_ml_source2_array_read_index31184; let ml_reference_projection1873 = self::ml_array::optional_at(&*ml_reference_array1873.borrow(), self::ml_array::number_index(ml_reference_position1873)); ml_reference_projection1873 }
                                                                                        }
                                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) }) }; ml_values1871.iter().cloned().enumerate().any(|(ml_index1871, ml_value1871)| ml_callback1871(ml_value1871, (ml_index1871 as f64))) }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 115, 111, 109, 101, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }) {
                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328Return(self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 100, 105, 99, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 110, 115, 117, 109, 101, 100, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 32, 112, 114, 111, 111, 102]))
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328Next
                                                                        }
                                                                    }
                                                                }
                                                                self::JavaScriptCallInstruction::Construct(destination_2, callee_2, new_target, arguments) => {
                                                                    if (((destination_2 != pending_destination) || !(self::test_java_script_semantic_same_value(({
                                                                        let source_ml_source2_array_read_value31186 = shared_constructor_field5.clone();
                                                                        { let ml_reference_array1874 = source_ml_source2_array_read_value31186.clone(); let ml_reference_position1874 = callee_2; let ml_reference_projection1874 = self::ml_array::optional_at(&*ml_reference_array1874.borrow(), self::ml_array::number_index(ml_reference_position1874)); ml_reference_projection1874 }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field1.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                                        let source_ml_source2_array_read_value31190 = shared_constructor_field5.clone();
                                                                        { let ml_reference_array1875 = source_ml_source2_array_read_value31190.clone(); let ml_reference_position1875 = new_target; let ml_reference_projection1875 = self::ml_array::optional_at(&*ml_reference_array1875.borrow(), self::ml_array::number_index(ml_reference_position1875)); ml_reference_projection1875 }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2.clone()))) {
                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328Return(invalid_instruction.clone())
                                                                    } else {
                                                                        if ((({ let ml_reference_array1876 = arguments.clone(); let ml_reference_projection1876 = ml_reference_array1876.borrow().len() as f64; ml_reference_projection1876 }) != ({ let ml_reference_array1877 = shared_constructor_field3.clone(); let ml_reference_projection1877 = ml_reference_array1877.borrow().len() as f64; ml_reference_projection1877 })) || { let ml_values1878 = { let ml_reference_value1879 = arguments.borrow().clone(); ml_reference_value1879 }; let ml_callback1878 = { let frame = frame.clone(); let state = state.clone(); std::rc::Rc::new(move |coordinate: f64, index: f64| -> bool { !(self::test_java_script_semantic_same_value(({
                                                                            let source_ml_source2_array_read_value31196 = match frame.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value31195, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value31195.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_array_read_index31197 = coordinate;
                                                                                { let ml_reference_array1880 = source_ml_source2_array_read_value31196.clone(); let ml_reference_position1880 = source_ml_source2_array_read_index31197; let ml_reference_projection1880 = self::ml_array::optional_at(&*ml_reference_array1880.borrow(), self::ml_array::number_index(ml_reference_position1880)); ml_reference_projection1880 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                                            let source_ml_source2_array_read_value31199 = match state.clone() {
                                                                                self::JavaScriptConstructionState::ConstructionState(_, _, source_ml_source2_data_property_value31198, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value31198.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_array_read_index31200 = index;
                                                                                { let ml_reference_array1881 = source_ml_source2_array_read_value31199.clone(); let ml_reference_position1881 = source_ml_source2_array_read_index31200; let ml_reference_projection1881 = self::ml_array::optional_at(&*ml_reference_array1881.borrow(), self::ml_array::number_index(ml_reference_position1881)); ml_reference_projection1881 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) }) }; ml_values1878.iter().cloned().enumerate().any(|(ml_index1878, ml_value1878)| ml_callback1878(ml_value1878, (ml_index1878 as f64))) }) {
                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328Return(invalid_instruction.clone())
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328Next
                                                                        }
                                                                    }
                                                                }
                                                                _ => {
                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328Return(invalid_instruction.clone())
                                                                }
                                                            };
                                                            match source_ml_source2_s31164.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328Next => {
                                                                    {
                                                                        let invalid = self::read_java_script_construction_state_error(program.clone(), heap.clone(), directory.clone().expect("a proven present value"), state.clone());
                                                                        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                            invalid.clone()
                                                                        } else {
                                                                            if (suspended && match (match state.clone() {
                                                                                self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value31201, _) => {
                                                                                    ((source_ml_source2_data_property_value31201.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                self::ml_string::Text::from_units(vec![97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 111, 114, 32, 98, 111, 100, 121, 32, 99, 97, 108, 108])
                                                                            } else {
                                                                                self::ml_string::Text::from_units(vec![])
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328::MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328Return(source_ml_source1_temporary_result1328) => {
                                                                    source_ml_source1_temporary_result1328.clone()
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
                        self::JavaScriptCallPending::WaitFunctionBind(
                            destination_3,
                            realm_2,
                            state_2,
                        ) => {
                            let state_2 = *state_2;
                            match state_2.clone() {
                                self::JavaScriptFunctionBindState::FunctionBindState(
                                    shared_constructor_field1_2,
                                    shared_constructor_field2_2,
                                    shared_constructor_field3_2,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let shared_constructor_field1_2 = *shared_constructor_field1_2;
                                    let shared_constructor_field2_2 = *shared_constructor_field2_2;
                                    {
                                        let invalid_instruction_2 =
                                            self::ml_string::Text::from_units(vec![
                                                98, 105, 110, 100, 105, 110, 103, 32, 114, 101,
                                                116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                                                99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105,
                                                110, 115, 105, 99, 32, 116, 97, 114, 103, 101, 116,
                                                32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97,
                                                110, 100, 32, 99, 111, 109, 112, 108, 101, 116,
                                                101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32,
                                                112, 114, 101, 102, 105, 120,
                                            ]);
                                        if !(shared_constructor_field4 > (0f64)) {
                                            invalid_instruction_2.clone()
                                        } else {
                                            {
                                                let instruction_2 = {
                                                    let source_ml_source2_array_read_value31206 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_array_read_index31207 =
                                                            (shared_constructor_field4 - 1f64);
                                                        {
                                                            let ml_reference_array1882 = source_ml_source2_array_read_value31206.clone();
                                                            let ml_reference_position1882 = source_ml_source2_array_read_index31207;
                                                            let ml_reference_projection1882 =
                                                                self::ml_array::optional_at(
                                                                    &*ml_reference_array1882
                                                                        .borrow(),
                                                                    self::ml_array::number_index(
                                                                        ml_reference_position1882,
                                                                    ),
                                                                );
                                                            ml_reference_projection1882
                                                        }
                                                    }
                                                };
                                                if (match (instruction_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction_2.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::FunctionBind(_) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }) {
                                                    invalid_instruction_2.clone()
                                                } else {
                                                    if ((((((!(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field8))) || ((((match instruction_2.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::FunctionBind(source_ml_source2_data_property_value31211) => {
                                                            Some((source_ml_source2_data_property_value31211,))
                                                        }
                                                        _ => {
                                                            None::<(f64,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0) != destination_3)) || (realm_2 != shared_constructor_field7)) || !(self::test_java_script_semantic_same_value(shared_constructor_field1_2.clone(), ({
                                                        let source_ml_source2_array_read_value31216 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_array_read_index31217 = 0f64;
                                                            { let ml_reference_array1883 = source_ml_source2_array_read_value31216.clone(); let ml_reference_position1883 = source_ml_source2_array_read_index31217; let ml_reference_projection1883 = self::ml_array::optional_at(&*ml_reference_array1883.borrow(), self::ml_array::number_index(ml_reference_position1883)); ml_reference_projection1883 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || !(self::test_java_script_semantic_same_value(shared_constructor_field2_2.clone(), ({
                                                        let source_ml_source2_array_read_value31220 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_array_read_index31221 = 0f64;
                                                            { let ml_reference_array1884 = source_ml_source2_array_read_value31220.clone(); let ml_reference_position1884 = source_ml_source2_array_read_index31221; let ml_reference_projection1884 = self::ml_array::optional_at(&*ml_reference_array1884.borrow(), self::ml_array::number_index(ml_reference_position1884)); ml_reference_projection1884 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || (({ let ml_reference_array1885 = shared_constructor_field3_2.clone(); let ml_reference_projection1885 = ml_reference_array1885.borrow().len() as f64; ml_reference_projection1885 }) != (self::ml_math::max(0f64, ({ let ml_reference_array1886 = shared_constructor_field6.clone(); let ml_reference_projection1886 = ml_reference_array1886.borrow().len() as f64; ml_reference_projection1886 } - 1f64))))) || { let ml_values1887 = { let ml_reference_value1888 = shared_constructor_field3_2.borrow().clone(); ml_reference_value1888 }; let ml_callback1887 = { let frame = frame.clone(); std::rc::Rc::new(move |value: self::JavaScriptSemanticValue, index: f64| -> bool { !(self::test_java_script_semantic_same_value(value.clone(), ({
                                                        let source_ml_source2_array_read_value31228 = match frame.clone() {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value31227, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value31227.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_array_read_index31229 = (index + 1f64);
                                                            { let ml_reference_array1889 = source_ml_source2_array_read_value31228.clone(); let ml_reference_position1889 = source_ml_source2_array_read_index31229; let ml_reference_projection1889 = self::ml_array::optional_at(&*ml_reference_array1889.borrow(), self::ml_array::number_index(ml_reference_position1889)); ml_reference_projection1889 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) }) }; ml_values1887.iter().cloned().enumerate().any(|(ml_index1887, ml_value1887)| ml_callback1887(ml_value1887, (ml_index1887 as f64))) }) {
                                                        invalid_instruction_2.clone()
                                                    } else {
                                                        {
                                                            let error = self::read_java_script_function_bind_state_error(heap.clone(), state_2.clone());
                                                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                error.clone()
                                                            } else {
                                                                if (suspended && match (match state_2.clone() {
                                                                    self::JavaScriptFunctionBindState::FunctionBindState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value31230) => {
                                                                        ((source_ml_source2_data_property_value31230.clone().unwrap_or_else(|| (None::<self::JavaScriptFunctionBindCall>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                    self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 99, 104, 105, 108, 100, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 103, 101, 116, 116, 101, 114])
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
                        self::JavaScriptCallPending::WaitFunctionText(
                            destination_4,
                            realm_3,
                            input,
                            kind,
                        ) => {
                            let input = *input;
                            if !(shared_constructor_field4 > (0f64)) {
                                self::ml_string::Text::from_units(vec![
                                    102, 117, 110, 99, 116, 105, 111, 110, 32, 116, 101, 120, 116,
                                    32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                                    97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110,
                                    115, 105, 99, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32,
                                    114, 101, 97, 108, 109, 32, 115, 111, 117, 114, 99, 101, 32,
                                    99, 97, 116, 101, 103, 111, 114, 121, 32, 97, 110, 100, 32,
                                    101, 120, 116, 101, 114, 110, 97, 108, 32, 109, 101, 116, 97,
                                    100, 97, 116, 97, 32, 114, 101, 113, 117, 101, 115, 116,
                                ])
                            } else {
                                {
                                    let instruction_3 = {
                                        let source_ml_source2_array_read_value31235 =
                                            shared_constructor_field9.clone();
                                        {
                                            let source_ml_source2_array_read_index31236 =
                                                (shared_constructor_field4 - 1f64);
                                            {
                                                let ml_reference_array1890 =
                                                    source_ml_source2_array_read_value31235.clone();
                                                let ml_reference_position1890 =
                                                    source_ml_source2_array_read_index31236;
                                                let ml_reference_projection1890 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1890.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1890,
                                                        ),
                                                    );
                                                ml_reference_projection1890
                                            }
                                        }
                                    };
                                    {
                                        let request = self::read_java_script_function_text(
                                            program.clone(),
                                            heap.clone(),
                                            input.clone(),
                                        );
                                        if (match (instruction_3.clone(), None::<()>) {
                                            (Some(_ml_left), Some(_ml_right)) => false,
                                            (None, None) => true,
                                            _ => false,
                                        } || match instruction_3
                                            .clone()
                                            .expect("a proven present value")
                                        {
                                            self::JavaScriptCallInstruction::FunctionToString(
                                                _,
                                            ) => false,
                                            _ => true,
                                        }) {
                                            self::ml_string::Text::from_units(vec![
                                                102, 117, 110, 99, 116, 105, 111, 110, 32, 116,
                                                101, 120, 116, 32, 114, 101, 116, 97, 105, 110,
                                                115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                                108, 32, 105, 110, 116, 114, 105, 110, 115, 105,
                                                99, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32,
                                                114, 101, 97, 108, 109, 32, 115, 111, 117, 114, 99,
                                                101, 32, 99, 97, 116, 101, 103, 111, 114, 121, 32,
                                                97, 110, 100, 32, 101, 120, 116, 101, 114, 110, 97,
                                                108, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32,
                                                114, 101, 113, 117, 101, 115, 116,
                                            ])
                                        } else {
                                            if ((((((((((match instruction_3.clone().expect("a proven present value") {
                                                self::JavaScriptCallInstruction::FunctionToString(source_ml_source2_data_property_value31239) => {
                                                    Some((source_ml_source2_data_property_value31239,))
                                                }
                                                _ => {
                                                    None::<(f64,)>
                                                }
                                            }).clone().expect("a proven present value")).0) != destination_4) || (realm_3 != shared_constructor_field7)) || !(self::test_java_script_primitive_intrinsic_mode(&(shared_constructor_field8)))) || !(self::test_java_script_semantic_same_value(input.clone(), ({
                                                let source_ml_source2_array_read_value31244 = shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_array_read_index31245 = 0f64;
                                                    { let ml_reference_array1891 = source_ml_source2_array_read_value31244.clone(); let ml_reference_position1891 = source_ml_source2_array_read_index31245; let ml_reference_projection1891 = self::ml_array::optional_at(&*ml_reference_array1891.borrow(), self::ml_array::number_index(ml_reference_position1891)); ml_reference_projection1891 }
                                                }
                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || match request.clone() {
                                                self::JavaScriptFunctionTextResult::FunctionTextNeedAuthority(_) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) || (((((match request.clone() {
                                                self::JavaScriptFunctionTextResult::FunctionTextNeedAuthority(source_ml_source2_data_property_value31248) => {
                                                    Some((source_ml_source2_data_property_value31248.clone(),))
                                                }
                                                _ => {
                                                    None::<(self::ml_string::Text,)>
                                                }
                                            }).clone().expect("a proven present value")).0).clone()) != kind)) || suspended) {
                                                self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 116, 101, 120, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 114, 101, 97, 108, 109, 32, 115, 111, 117, 114, 99, 101, 32, 99, 97, 116, 101, 103, 111, 114, 121, 32, 97, 110, 100, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 113, 117, 101, 115, 116])
                                            } else {
                                                self::ml_string::Text::from_units(vec![])
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallPending::WaitPrimitiveWrapper(
                            destination_5,
                            state_3,
                        ) => {
                            let state_3 = *state_3;
                            match state_3.clone() {
                                self::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(
                                    shared_constructor_field1_3,
                                    shared_constructor_field2_3,
                                ) => {
                                    let shared_constructor_field1_3 = *shared_constructor_field1_3;
                                    let shared_constructor_field2_3 = *shared_constructor_field2_3;
                                    if !(shared_constructor_field4 > (0f64)) {
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 119,
                                            114, 97, 112, 112, 101, 114, 115, 32, 114, 101, 116,
                                            97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99,
                                            116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116,
                                            101, 100, 32, 105, 110, 112, 117, 116, 32, 112, 114,
                                            111, 116, 111, 116, 121, 112, 101, 32, 97, 110, 100,
                                            32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32,
                                            105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110,
                                        ])
                                    } else {
                                        {
                                            let instruction_4 = {
                                                let source_ml_source2_array_read_value31253 =
                                                    shared_constructor_field9.clone();
                                                {
                                                    let source_ml_source2_array_read_index31254 =
                                                        (shared_constructor_field4 - 1f64);
                                                    {
                                                        let ml_reference_array1892 =
                                                            source_ml_source2_array_read_value31253
                                                                .clone();
                                                        let ml_reference_position1892 =
                                                            source_ml_source2_array_read_index31254;
                                                        let ml_reference_projection1892 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array1892.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position1892,
                                                                ),
                                                            );
                                                        ml_reference_projection1892
                                                    }
                                                }
                                            };
                                            if (match (instruction_4.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction_4.clone().expect("a proven present value") {
                                                self::JavaScriptCallInstruction::PrimitiveWrapper(_, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) {
                                                self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 119, 114, 97, 112, 112, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 112, 117, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 97, 110, 100, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110])
                                            } else {
                                                if (((((((match instruction_4.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::PrimitiveWrapper(source_ml_source2_data_property_value31259, _, _) => {
                                                        Some((source_ml_source2_data_property_value31259,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0) != destination_5) || !(self::test_java_script_semantic_same_value(({
                                                    let source_ml_source2_array_read_value31262 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_array_read_index31263 = ((match instruction_4.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::PrimitiveWrapper(_, source_ml_source2_data_property_value31261, _) => {
                                                                Some((source_ml_source2_data_property_value31261,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0;
                                                        { let ml_reference_array1893 = source_ml_source2_array_read_value31262.clone(); let ml_reference_position1893 = source_ml_source2_array_read_index31263; let ml_reference_projection1893 = self::ml_array::optional_at(&*ml_reference_array1893.borrow(), self::ml_array::number_index(ml_reference_position1893)); ml_reference_projection1893 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field1_3.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                    let source_ml_source2_array_read_value31267 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_array_read_index31268 = ((match instruction_4.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::PrimitiveWrapper(_, _, source_ml_source2_data_property_value31266) => {
                                                                Some((source_ml_source2_data_property_value31266,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0;
                                                        { let ml_reference_array1894 = source_ml_source2_array_read_value31267.clone(); let ml_reference_position1894 = source_ml_source2_array_read_index31268; let ml_reference_projection1894 = self::ml_array::optional_at(&*ml_reference_array1894.borrow(), self::ml_array::number_index(ml_reference_position1894)); ml_reference_projection1894 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2_3.clone()))) || suspended) {
                                                    self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 119, 114, 97, 112, 112, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 112, 117, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 97, 110, 100, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110])
                                                } else {
                                                    {
                                                        let invalid_2 = self::read_java_script_primitive_wrapper_state_error(heap.clone(), state_3.clone());
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
                        _ => self::ml_string::Text::from_units(vec![]),
                    }
                }
            }
        }
    }
}
