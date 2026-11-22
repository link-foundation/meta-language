// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallArgumentList;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallIteratorRegion;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487;
use super::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488;
use super::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1485Result;
use super::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1486Result;
use super::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1489Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_argument_lists_error;
use super::read_java_script_call_frame_control_error;
use super::read_java_script_call_pending_error;
use super::read_java_script_iterator_regions_error;
use super::source_ml_source1_temporary_read_java_script_call_frame_error_loop1485;
use super::source_ml_source1_temporary_read_java_script_call_frame_error_loop1486;
use super::source_ml_source1_temporary_read_java_script_call_frame_error_loop1489;
use super::test_java_script_call_coordinate;
use super::test_java_script_ordinary_class_constructor_mode;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_pending_destination(
    pending: self::JavaScriptCallPending,
    registers: f64,
) -> bool {
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
        self::JavaScriptCallPending::WaitHostSet(destination_4) => {
            self::test_java_script_call_coordinate(destination_4, registers)
        }
        self::JavaScriptCallPending::WaitArgumentRest(destination_5, _, _) => {
            self::test_java_script_call_coordinate(destination_5, registers)
        }
        self::JavaScriptCallPending::WaitIteratorProtocol(destination_6, _, _, _, _, _) => {
            self::test_java_script_call_coordinate(destination_6, registers)
        }
        self::JavaScriptCallPending::WaitArraySpread(destination_7, _, _) => {
            self::test_java_script_call_coordinate(destination_7, registers)
        }
        self::JavaScriptCallPending::WaitArrayIteratorNext(destination_8, _, _) => {
            self::test_java_script_call_coordinate(destination_8, registers)
        }
        self::JavaScriptCallPending::WaitCollectionIteratorResult(destination_9, _, _) => {
            self::test_java_script_call_coordinate(destination_9, registers)
        }
        self::JavaScriptCallPending::WaitArrayConcatenation(destination_10, _, _, _) => {
            self::test_java_script_call_coordinate(destination_10, registers)
        }
        self::JavaScriptCallPending::WaitArraySlice(destination_11, _, _, _) => {
            self::test_java_script_call_coordinate(destination_11, registers)
        }
        self::JavaScriptCallPending::WaitArrayTransform(destination_12, _, _, _) => {
            self::test_java_script_call_coordinate(destination_12, registers)
        }
        self::JavaScriptCallPending::WaitArraySearch(destination_13, _, _) => {
            self::test_java_script_call_coordinate(destination_13, registers)
        }
        self::JavaScriptCallPending::WaitCollectionForEach(destination_14, _, _) => {
            self::test_java_script_call_coordinate(destination_14, registers)
        }
        self::JavaScriptCallPending::WaitArrayCallback(destination_15, _, _) => {
            self::test_java_script_call_coordinate(destination_15, registers)
        }
        self::JavaScriptCallPending::WaitObjectSpread(destination_16, _, _) => {
            self::test_java_script_call_coordinate(destination_16, registers)
        }
        self::JavaScriptCallPending::WaitObjectEnumeration(destination_17, _, _) => {
            self::test_java_script_call_coordinate(destination_17, registers)
        }
        self::JavaScriptCallPending::WaitPropertyDefinitions(destination_18, _, _) => {
            self::test_java_script_call_coordinate(destination_18, registers)
        }
        self::JavaScriptCallPending::WaitPropertyReflection(destination_19, _, _) => {
            self::test_java_script_call_coordinate(destination_19, registers)
        }
        self::JavaScriptCallPending::WaitInstanceOf(destination_20, _, _) => {
            self::test_java_script_call_coordinate(destination_20, registers)
        }
        self::JavaScriptCallPending::WaitObjectIntegrity(destination_21, _, _) => {
            self::test_java_script_call_coordinate(destination_21, registers)
        }
        self::JavaScriptCallPending::WaitObjectPrototype(destination_22, _, _) => {
            self::test_java_script_call_coordinate(destination_22, registers)
        }
        self::JavaScriptCallPending::WaitArrayAssignment(destination_23, _, _) => {
            self::test_java_script_call_coordinate(destination_23, registers)
        }
        self::JavaScriptCallPending::WaitKey(destination_24, _, _) => {
            self::test_java_script_call_coordinate(destination_24, registers)
        }
        self::JavaScriptCallPending::WaitDefinition(destination_25, _, _) => {
            self::test_java_script_call_coordinate(destination_25, registers)
        }
        self::JavaScriptCallPending::WaitNumber(destination_26, _, _) => {
            self::test_java_script_call_coordinate(destination_26, registers)
        }
        self::JavaScriptCallPending::WaitText(destination_27, _, _) => {
            self::test_java_script_call_coordinate(destination_27, registers)
        }
        self::JavaScriptCallPending::WaitArrayCreation(destination_28, _, _, _, _, _) => {
            self::test_java_script_call_coordinate(destination_28, registers)
        }
        self::JavaScriptCallPending::WaitConstruction(destination_29, _, _, _) => {
            self::test_java_script_call_coordinate(destination_29, registers)
        }
        self::JavaScriptCallPending::WaitFunctionBind(destination_30, _, _) => {
            self::test_java_script_call_coordinate(destination_30, registers)
        }
        self::JavaScriptCallPending::WaitFunctionText(destination_31, _, _, _) => {
            self::test_java_script_call_coordinate(destination_31, registers)
        }
        self::JavaScriptCallPending::WaitConstructorCheck(destination_32, _, _) => {
            self::test_java_script_call_coordinate(destination_32, registers)
        }
        self::JavaScriptCallPending::WaitObjectFromEntries(destination_33, _, _, _) => {
            self::test_java_script_call_coordinate(destination_33, registers)
        }
        self::JavaScriptCallPending::WaitPrimitiveWrapper(destination_34, _) => {
            self::test_java_script_call_coordinate(destination_34, registers)
        }
        self::JavaScriptCallPending::WaitSymbolAllocation(destination_35, _, _) => {
            self::test_java_script_call_coordinate(destination_35, registers)
        }
        self::JavaScriptCallPending::WaitSymbolDescription(destination_36, _) => {
            self::test_java_script_call_coordinate(destination_36, registers)
        }
        self::JavaScriptCallPending::WaitFunctionNameSymbol(destination_37, _, _, _, _) => {
            self::test_java_script_call_coordinate(destination_37, registers)
        }
        self::JavaScriptCallPending::WaitErrorValue(destination_38, _, _, _, _, _, _, _, _, _) => {
            self::test_java_script_call_coordinate(destination_38, registers)
        }
        self::JavaScriptCallPending::WaitErrorIntrinsic(destination_39) => {
            self::test_java_script_call_coordinate(destination_39, registers)
        }
        self::JavaScriptCallPending::WaitHostCapability(destination_40, _) => {
            self::test_java_script_call_coordinate(destination_40, registers)
        }
        _ => true,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_frame_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    frame: self::JavaScriptCallFrame,
    fn_: self::JavaScriptCallFunction,
    suspended: bool,
) -> self::ml_string::Text {
    match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(
            _,
            _,
            shared_constructor_field7,
            _,
            _,
            shared_constructor_field8,
            _,
            _,
            _,
            _,
        ) => match frame.clone() {
            self::JavaScriptCallFrame::CallFrame(
                shared_constructor_field2,
                _,
                shared_constructor_field3,
                shared_constructor_field4,
                shared_constructor_field5,
                shared_constructor_field6,
                _,
                _,
                _,
                _,
            ) => {
                let shared_constructor_field6 = *shared_constructor_field6;
                match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        shared_constructor_field1,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source1_temporary_values1482 =
                            shared_constructor_field3.clone();
                        {
                            let source_ml_source1_temporary_index1482 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s35198 = self::source_ml_source1_temporary_read_java_script_call_frame_error_loop1485(heap.clone(), source_ml_source1_temporary_values1482.clone(), source_ml_source1_temporary_index1482.clone());
                                match source_ml_source2_s35198.clone() {
                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1485Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1485Done => {
                                                {
                                                    let source_ml_source1_temporary_values1483 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source1_temporary_index1483 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s35200 = self::source_ml_source1_temporary_read_java_script_call_frame_error_loop1486(heap.clone(), source_ml_source1_temporary_values1483.clone(), source_ml_source1_temporary_index1483.clone());
                                                            match source_ml_source2_s35200.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1486Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1486Done => {
                                                                    {
                                                                        let closure = {
                                                                            let source_ml_source2_array_read_value35204 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index35205 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), shared_constructor_field2.clone());
                                                                                { let ml_reference_array2266 = source_ml_source2_array_read_value35204.clone(); let ml_reference_position2266 = source_ml_source2_array_read_index35205; let ml_reference_projection2266 = self::ml_array::optional_at(&*ml_reference_array2266.borrow(), self::ml_array::number_index(ml_reference_position2266)); ml_reference_projection2266 }
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_s35223 = if (!(match (closure.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match closure.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                                    true
                                                                                }
                                                                                _ => {
                                                                                    false
                                                                                }
                                                                            }) {
                                                                                if ((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108]))) && !(self::test_java_script_semantic_same_value(({
                                                                                    let source_ml_source2_array_read_value35237 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_array_read_index35238 = 0f64;
                                                                                        { let ml_reference_array2267 = source_ml_source2_array_read_value35237.clone(); let ml_reference_position2267 = source_ml_source2_array_read_index35238; let ml_reference_projection2267 = self::ml_array::optional_at(&*ml_reference_array2267.borrow(), self::ml_array::number_index(ml_reference_position2267)); ml_reference_projection2267 }
                                                                                    }
                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (((match closure.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value35239, _) => {
                                                                                        let source_ml_source2_data_property_value35239 = *source_ml_source2_data_property_value35239;
                                                                                        Some((source_ml_source2_data_property_value35239.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(self::JavaScriptSemanticValue,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone()))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 114, 114, 111, 119, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                                                } else {
                                                                                    {
                                                                                        let index = 0f64;
                                                                                        {
                                                                                            let source_ml_source2_s35240 = self::source_ml_source1_temporary_read_java_script_call_frame_error_loop1489(frame.clone(), fn_.clone(), Some(closure.clone().expect("a proven present value")), index);
                                                                                            match source_ml_source2_s35240.clone() {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1489Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1489Done => {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487Next
                                                                                                }
                                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1489Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1489Return(source_ml_source1_temporary_result1489) => {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487Return(source_ml_source1_temporary_result1489.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487Next
                                                                            };
                                                                            match source_ml_source2_s35223.clone() {
                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487Next => {
                                                                                    {
                                                                                        let source_ml_source2_s35256 = if self::test_java_script_ordinary_class_constructor_mode(&(shared_constructor_field7)) {
                                                                                            {
                                                                                                let receiver = ({
                                                                                                    let source_ml_source2_array_read_value35259 = shared_constructor_field3.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_array_read_index35260 = 0f64;
                                                                                                        { let ml_reference_array2268 = source_ml_source2_array_read_value35259.clone(); let ml_reference_position2268 = source_ml_source2_array_read_index35260; let ml_reference_projection2268 = self::ml_array::optional_at(&*ml_reference_array2268.borrow(), self::ml_array::number_index(ml_reference_position2268)); ml_reference_projection2268 }
                                                                                                    }
                                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                                                {
                                                                                                    let object = match receiver.clone() {
                                                                                                        self::JavaScriptSemanticValue::Object(identity, _) => {
                                                                                                            {
                                                                                                                let source_ml_source2_array_read_value35264 = shared_constructor_field1.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_array_read_index35265 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                                                                                    { let ml_reference_array2269 = source_ml_source2_array_read_value35264.clone(); let ml_reference_position2269 = source_ml_source2_array_read_index35265; let ml_reference_projection2269 = self::ml_array::optional_at(&*ml_reference_array2269.borrow(), self::ml_array::number_index(ml_reference_position2269)); ml_reference_projection2269 }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        _ => {
                                                                                                            None::<self::JavaScriptOrdinaryObject>
                                                                                                        }
                                                                                                    };
                                                                                                    if ((match receiver.clone() {
                                                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                            false
                                                                                                        }
                                                                                                        _ => {
                                                                                                            true
                                                                                                        }
                                                                                                    } || ((match receiver.clone() {
                                                                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value35268) => {
                                                                                                            Some((source_ml_source2_data_property_value35268,))
                                                                                                        }
                                                                                                        _ => {
                                                                                                            None::<(bool,)>
                                                                                                        }
                                                                                                    }).clone().expect("a proven present value")).0) || {
                                                                                                        let source_ml_source2_comparison_operand35270 = {
                                                                                                            let source_ml_source2_optional_chain_receiver35269 = (object.clone(),);
                                                                                                            if match (source_ml_source2_optional_chain_receiver35269.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                Some({
                                                                                                                    let source_ml_source1_temporary_optional_chain1484 = ((source_ml_source2_optional_chain_receiver35269.clone()).0).clone().expect("a proven present value");
                                                                                                                    match source_ml_source1_temporary_optional_chain1484.clone() {
                                                                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                                                                            self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                                                                                                        }
                                                                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                                                                            self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                                                                                                        }
                                                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                                                                        }
                                                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                                                                        }
                                                                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                                                                            self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                                                                        }
                                                                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                                                                            self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                                                                        }
                                                                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                                                                            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                                                                        }
                                                                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                                                                            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                                                                        }
                                                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                                                                                                        }
                                                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                                                                        }
                                                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                                                                                                        }
                                                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                                                                        }
                                                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                                                                                                        }
                                                                                                                    }
                                                                                                                })
                                                                                                            } else {
                                                                                                                None::<self::ml_string::Text>
                                                                                                            }
                                                                                                        };
                                                                                                        !(match (source_ml_source2_comparison_operand35270.clone(), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                                    }) {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 105, 118, 101, 32, 98, 97, 115, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                                                                    } else {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488Next
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488Next
                                                                                        };
                                                                                        match source_ml_source2_s35256.clone() {
                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488Next => {
                                                                                                {
                                                                                                    let iterator_error = self::read_java_script_iterator_regions_error(heap.clone(), (match frame.clone() {
                                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, source_ml_source2_data_property_value35272, _, _, _) => {
                                                                                                            ((source_ml_source2_data_property_value35272.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,))).0).clone()
                                                                                                        }
                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallIteratorRegion>::new()))));
                                                                                                    if (iterator_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                        iterator_error.clone()
                                                                                                    } else {
                                                                                                        {
                                                                                                            let argument_list_error = self::read_java_script_call_argument_lists_error(heap.clone(), (match frame.clone() {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, source_ml_source2_data_property_value35273, _, _) => {
                                                                                                                    ((source_ml_source2_data_property_value35273.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>>,))).0).clone()
                                                                                                                }
                                                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallArgumentList>::new()))), { let ml_reference_array2270 = shared_constructor_field5.clone(); let ml_reference_projection2270 = ml_reference_array2270.borrow().len() as f64; ml_reference_projection2270 });
                                                                                                            if (argument_list_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                argument_list_error.clone()
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let control_error = self::read_java_script_call_frame_control_error(program.clone(), heap.clone(), frame.clone(), fn_.clone());
                                                                                                                    if (control_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                        control_error.clone()
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let pending = shared_constructor_field6.clone();
                                                                                                                            if !(self::test_java_script_call_pending_destination(pending.clone(), shared_constructor_field8)) {
                                                                                                                                self::ml_string::Text::from_units(vec![112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114])
                                                                                                                            } else {
                                                                                                                                {
                                                                                                                                    let pending_error = self::read_java_script_call_pending_error(program.clone(), heap.clone(), frame.clone(), fn_.clone(), suspended, pending.clone());
                                                                                                                                    if (pending_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                        pending_error.clone()
                                                                                                                                    } else {
                                                                                                                                        if (suspended && ((((((((((match pending.clone() {
                                                                                                                                            self::JavaScriptCallPending::WaitConstructorCheck(_, _, _) => {
                                                                                                                                                true
                                                                                                                                            }
                                                                                                                                            _ => {
                                                                                                                                                false
                                                                                                                                            }
                                                                                                                                        } || match pending.clone() {
                                                                                                                                            self::JavaScriptCallPending::WaitPrimitiveWrapper(_, _) => {
                                                                                                                                                true
                                                                                                                                            }
                                                                                                                                            _ => {
                                                                                                                                                false
                                                                                                                                            }
                                                                                                                                        }) || match pending.clone() {
                                                                                                                                            self::JavaScriptCallPending::WaitSymbolAllocation(_, _, _) => {
                                                                                                                                                true
                                                                                                                                            }
                                                                                                                                            _ => {
                                                                                                                                                false
                                                                                                                                            }
                                                                                                                                        }) || match pending.clone() {
                                                                                                                                            self::JavaScriptCallPending::WaitSymbolDescription(_, _) => {
                                                                                                                                                true
                                                                                                                                            }
                                                                                                                                            _ => {
                                                                                                                                                false
                                                                                                                                            }
                                                                                                                                        }) || match pending.clone() {
                                                                                                                                            self::JavaScriptCallPending::WaitFunctionNameSymbol(_, _, _, _, _) => {
                                                                                                                                                true
                                                                                                                                            }
                                                                                                                                            _ => {
                                                                                                                                                false
                                                                                                                                            }
                                                                                                                                        }) || match pending.clone() {
                                                                                                                                            self::JavaScriptCallPending::CallIdle => {
                                                                                                                                                true
                                                                                                                                            }
                                                                                                                                            _ => {
                                                                                                                                                false
                                                                                                                                            }
                                                                                                                                        }) || match pending.clone() {
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
                                                                                                                                        }) || match pending.clone() {
                                                                                                                                            self::JavaScriptCallPending::WaitErrorValue(_, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                true
                                                                                                                                            }
                                                                                                                                            _ => {
                                                                                                                                                false
                                                                                                                                            }
                                                                                                                                        }) || match pending.clone() {
                                                                                                                                            self::JavaScriptCallPending::WaitErrorIntrinsic(_) => {
                                                                                                                                                true
                                                                                                                                            }
                                                                                                                                            _ => {
                                                                                                                                                false
                                                                                                                                            }
                                                                                                                                        }) || match pending.clone() {
                                                                                                                                            self::JavaScriptCallPending::WaitHostCapability(_, _) => {
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
                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1488Return(source_ml_source1_temporary_result1488) => {
                                                                                                source_ml_source1_temporary_result1488.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487::MlSource1TemporaryReadJavaScriptCallFrameErrorJoin1487Return(source_ml_source1_temporary_result1488_2) => {
                                                                                    source_ml_source1_temporary_result1488_2.clone()
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1486Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1486Return(source_ml_source1_temporary_result1489_2) => {
                                                                    source_ml_source1_temporary_result1489_2.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1485Result::MlSource1TemporaryReadJavaScriptCallFrameErrorLoop1485Return(source_ml_source1_temporary_result1489_3) => {
                                                source_ml_source1_temporary_result1489_3.clone()
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
