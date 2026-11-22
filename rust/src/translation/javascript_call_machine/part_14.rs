// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptWeakCollectionEntry;
use super::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51;
use super::MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Result;
use super::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47;
use super::MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Result;
use super::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54;
use super::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55;
use super::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44;
use super::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Result;
use super::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58;
use super::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::source_ml_source1_temporary_read_java_script_bound_object_error_loop53;
use super::source_ml_source1_temporary_read_java_script_closure_object_error_loop49;
use super::source_ml_source1_temporary_read_java_script_opaque_object_error_loop45;
use super::source_ml_source1_temporary_read_java_script_weak_collection_object_error_loop60;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_opaque_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
) -> self::ml_string::Text {
    {
        let source_ml_source2_s897 = if ((((((((match object.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => false,
            _ => true,
        } && match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => false,
            _ => true,
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => false,
            _ => true,
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => false,
            _ => true,
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => false,
            _ => true,
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => false,
            _ => true,
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => false,
            _ => true,
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
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
                _,
            ) => false,
            _ => true,
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => false,
            _ => true,
        }) {
            if (((match object.clone() {
                self::JavaScriptOrdinaryObject::HostErrorObject(
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value965,
                    _,
                    _,
                    _,
                ) => Some((source_ml_source2_data_property_value965,)),
                self::JavaScriptOrdinaryObject::HostURLObject(
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value966,
                    _,
                    _,
                    _,
                ) => Some((source_ml_source2_data_property_value966,)),
                self::JavaScriptOrdinaryObject::HostArrayObject(
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value967,
                    _,
                    _,
                    _,
                ) => Some((source_ml_source2_data_property_value967,)),
                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value968,
                    _,
                    _,
                    _,
                ) => Some((source_ml_source2_data_property_value968,)),
                _ => None::<(bool,)>,
            })
            .clone()
            .expect("a proven present value"))
            .0 || (({
                let ml_reference_array172 = (((match object.clone() {
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value973,
                        _,
                        _,
                    ) => Some((source_ml_source2_data_property_value973.clone(),)),
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value974,
                        _,
                        _,
                    ) => Some((source_ml_source2_data_property_value974.clone(),)),
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value975,
                        _,
                        _,
                    ) => Some((source_ml_source2_data_property_value975.clone(),)),
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value976,
                        _,
                        _,
                    ) => Some((source_ml_source2_data_property_value976.clone(),)),
                    _ => {
                        None::<(
                            std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
                        )>
                    }
                })
                .clone()
                .expect("a proven present value"))
                .0)
                    .clone();
                let ml_reference_projection172 = ml_reference_array172.borrow().len() as f64;
                ml_reference_projection172
            }) != (0f64)))
            {
                self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 111, 98, 106, 101, 99, 116, 115, 32, 107, 101, 101, 112, 32, 104, 111, 115, 116, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 117, 116, 115, 105, 100, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116, 115]))
            } else {
                {
                    let source_ml_source1_temporary_values43 =
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            Some(
                                (((match object.clone() {
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value977,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value977 =
                                            *source_ml_source2_data_property_value977;
                                        Some((source_ml_source2_data_property_value977.clone(),))
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value978,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value978 =
                                            *source_ml_source2_data_property_value978;
                                        Some((source_ml_source2_data_property_value978.clone(),))
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value979,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value979 =
                                            *source_ml_source2_data_property_value979;
                                        Some((source_ml_source2_data_property_value979.clone(),))
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value980,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value980 =
                                            *source_ml_source2_data_property_value980;
                                        Some((source_ml_source2_data_property_value980.clone(),))
                                    }
                                    _ => None::<(self::JavaScriptSemanticValue,)>,
                                })
                                .clone()
                                .expect("a proven present value"))
                                .0)
                                    .clone(),
                            ),
                            Some(
                                (((match object.clone() {
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value981,
                                    ) => {
                                        let source_ml_source2_data_property_value981 =
                                            *source_ml_source2_data_property_value981;
                                        Some((source_ml_source2_data_property_value981.clone(),))
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value982,
                                    ) => {
                                        let source_ml_source2_data_property_value982 =
                                            *source_ml_source2_data_property_value982;
                                        Some((source_ml_source2_data_property_value982.clone(),))
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value983,
                                    ) => {
                                        let source_ml_source2_data_property_value983 =
                                            *source_ml_source2_data_property_value983;
                                        Some((source_ml_source2_data_property_value983.clone(),))
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value984,
                                    ) => {
                                        let source_ml_source2_data_property_value984 =
                                            *source_ml_source2_data_property_value984;
                                        Some((source_ml_source2_data_property_value984.clone(),))
                                    }
                                    _ => None::<(self::JavaScriptSemanticValue,)>,
                                })
                                .clone()
                                .expect("a proven present value"))
                                .0)
                                    .clone(),
                            ),
                        ]));
                    {
                        let source_ml_source1_temporary_index43 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s985 = self::source_ml_source1_temporary_read_java_script_opaque_object_error_loop45(heap.clone(), source_ml_source1_temporary_values43.clone(), source_ml_source1_temporary_index43.clone());
                            match source_ml_source2_s985.clone() {
                                self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Done => {
                                    self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44Next
                                }
                                self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(source_ml_source1_temporary_result45) => {
                                    self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44Return(source_ml_source1_temporary_result45.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44Next
        };
        match source_ml_source2_s897.clone() {
            self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44Return(source_ml_source1_temporary_result44) => {
                source_ml_source1_temporary_result44.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closure_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    budget: std::rc::Rc<std::cell::RefCell<((f64, f64, f64, f64, f64), Vec<f64>)>>,
) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let source_ml_source2_s1031 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, callable, _, object_owner, ordinal, captures, lexical_receiver, lexical_new_target) => {
                        let lexical_receiver = *lexical_receiver;
                        if (((((((!callable || (object_owner <= (self::ml::Big::from_i128(0)))) || (object_owner > (self::ml::Big::from_i128(18446744073709551615)))) || !(self::ml_math::is_safe_integer(ordinal))) || (ordinal < (0f64))) || (ordinal > (4095f64))) || {
                            let source_ml_source2_same_value_operand1037 = (-0f64);
                            (((ordinal == source_ml_source2_same_value_operand1037) && ((ordinal != (0f64)) || (((1f64 / ordinal)) == ((1f64 / source_ml_source2_same_value_operand1037))))) || ((ordinal != ordinal) && (source_ml_source2_same_value_operand1037 != source_ml_source2_same_value_operand1037)))
                        }) || (({ let ml_reference_array173 = captures.clone(); let ml_reference_projection173 = ml_reference_array173.borrow().len() as f64; ml_reference_projection173 }) > (4096f64))) {
                            self::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47Return(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114, 44, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 105, 116, 101, 44, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 98, 114, 97, 110, 100, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 97, 112, 116, 117, 114, 101, 115]))
                        } else {
                            {
                                let source_ml_source1_temporary_read_java_script_closure_object_error_written48 = { let ml_field_owner175 = budget.clone(); let ml_field_previous175 = ml_field_owner175.borrow().0.0; let ml_field_value175 = { let ml_reference_array175 = captures.clone(); let ml_reference_projection175 = ml_reference_array175.borrow().len() as f64; ml_reference_projection175 }; ml_field_owner175.borrow_mut().0.0 = ml_field_previous175 + ml_field_value175; () };
                                if (((({ let ml_reference_value176 = budget.borrow().clone(); ml_reference_value176 }).0).0) > (1048576f64)) {
                                    self::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47Return(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112, 32, 118, 97, 108, 117, 101, 32, 98, 117, 100, 103, 101, 116]))
                                } else {
                                    {
                                        let owner = {
                                            let source_ml_source2_array_read_value1040 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_array_read_index1041 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), object_owner.clone());
                                                { let ml_reference_array177 = source_ml_source2_array_read_value1040.clone(); let ml_reference_position177 = source_ml_source2_array_read_index1041; let ml_reference_projection177 = self::ml_array::optional_at(&*ml_reference_array177.borrow(), self::ml_array::number_index(ml_reference_position177)); ml_reference_projection177 }
                                            }
                                        };
                                        if (((match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match owner.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value1042, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value1042
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value1043, _, _, _, _) => {
                                                source_ml_source2_data_property_value1043
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value1044, _) => {
                                                source_ml_source2_data_property_value1044
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value1045, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value1045
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value1046, _, _) => {
                                                source_ml_source2_data_property_value1046
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value1047, _, _) => {
                                                source_ml_source2_data_property_value1047
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value1048, _, _) => {
                                                source_ml_source2_data_property_value1048
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value1049, _, _) => {
                                                source_ml_source2_data_property_value1049
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value1050, _) => {
                                                source_ml_source2_data_property_value1050
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value1051, _, _, _) => {
                                                source_ml_source2_data_property_value1051
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value1052, _, _, _) => {
                                                source_ml_source2_data_property_value1052
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value1053, _, _, _) => {
                                                source_ml_source2_data_property_value1053
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value1054, _, _, _) => {
                                                source_ml_source2_data_property_value1054
                                            }
                                        })) || match owner.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                true
                                            }
                                            _ => {
                                                false
                                            }
                                        }) || match owner.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                true
                                            }
                                            _ => {
                                                false
                                            }
                                        }) {
                                            self::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47Return(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 99, 111, 100, 101, 32, 111, 119, 110, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 116, 105, 99, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 44, 32, 110, 101, 118, 101, 114, 32, 97, 110, 111, 116, 104, 101, 114, 32, 114, 117, 110, 116, 105, 109, 101, 32, 99, 108, 111, 115, 117, 114, 101, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                                        } else {
                                            {
                                                let new_target = ((lexical_new_target.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                                                if (!(match (new_target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match new_target.clone().expect("a proven present value") {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                } || !(((match new_target.clone().expect("a proven present value") {
                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value1075) => {
                                                        Some((source_ml_source2_data_property_value1075,))
                                                    }
                                                    _ => {
                                                        None::<(bool,)>
                                                    }
                                                }).clone().expect("a proven present value")).0)) || ((self::read_java_script_ordinary_value_error(heap.clone(), new_target.clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![]))))) {
                                                    self::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47Return(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 116, 105, 111, 110, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                                                } else {
                                                    {
                                                        let source_ml_source1_temporary_values46 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![lexical_receiver.clone()], { let ml_reference_value178 = captures.borrow().clone(); ml_reference_value178 })));
                                                        {
                                                            let source_ml_source1_temporary_index46 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s1076 = self::source_ml_source1_temporary_read_java_script_closure_object_error_loop49(heap.clone(), source_ml_source1_temporary_values46.clone(), source_ml_source1_temporary_index46.clone());
                                                                match source_ml_source2_s1076.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Result::MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Done => {
                                                                        self::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47Next
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Result::MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Return(source_ml_source1_temporary_result49) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47Return(source_ml_source1_temporary_result49.clone())
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
                    _ => {
                        self::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47Next
                    }
                };
            match source_ml_source2_s1031.clone() {
                    self::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47Next => {
                        self::ml_string::Text::from_units(vec![])
                    }
                    self::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47::MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47Return(source_ml_source1_temporary_result47) => {
                        source_ml_source1_temporary_result47.clone()
                    }
                }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_bound_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    budget: std::rc::Rc<std::cell::RefCell<((f64, f64, f64, f64, f64), Vec<f64>)>>,
) -> self::ml_string::Text {
    {
        let source_ml_source2_s1087 = match object.clone() {
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, callable, _, bound_target, bound_this, bound_arguments) => {
                let bound_target = *bound_target;
                let bound_this = *bound_this;
                if (((!callable || match bound_target.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        false
                    }
                    _ => {
                        true
                    }
                }) || !(match match bound_target.clone() {
                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value1094) => {
                        Some(source_ml_source2_data_property_value1094)
                    }
                    _ => {
                        None::<bool>
                    }
                } { Some(ml_present) => ml_present, None => false })) || (({ let ml_reference_array179 = bound_arguments.clone(); let ml_reference_projection179 = ml_reference_array179.borrow().len() as f64; ml_reference_projection179 }) > (1048576f64))) {
                    self::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 116, 97, 114, 103, 101, 116, 115, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_read_java_script_bound_object_error_written52 = { let ml_field_owner181 = budget.clone(); let ml_field_previous181 = ml_field_owner181.borrow().0.0; let ml_field_value181 = ({ let ml_reference_array181 = bound_arguments.clone(); let ml_reference_projection181 = ml_reference_array181.borrow().len() as f64; ml_reference_projection181 } + 2f64); ml_field_owner181.borrow_mut().0.0 = ml_field_previous181 + ml_field_value181; () };
                        if (((({ let ml_reference_value182 = budget.borrow().clone(); ml_reference_value182 }).0).0) > (1048576f64)) {
                            self::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 115, 108, 111, 116, 115, 32, 97, 110, 100, 32, 99, 108, 111, 115, 117, 114, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 115, 104, 97, 114, 101, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 118, 97, 108, 117, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let source_ml_source1_temporary_values50 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![bound_target.clone(), bound_this.clone()], { let ml_reference_value183 = bound_arguments.borrow().clone(); ml_reference_value183 })));
                                {
                                    let source_ml_source1_temporary_index50 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s1097 = self::source_ml_source1_temporary_read_java_script_bound_object_error_loop53(heap.clone(), source_ml_source1_temporary_values50.clone(), source_ml_source1_temporary_index50.clone());
                                        match source_ml_source2_s1097.clone() {
                                            self::MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Result::MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Done => {
                                                self::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51Next
                                            }
                                            self::MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Result::MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Return(source_ml_source1_temporary_result53) => {
                                                self::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51Return(source_ml_source1_temporary_result53.clone())
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                self::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51Next
            }
        };
        match source_ml_source2_s1087.clone() {
            self::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51::MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51Return(source_ml_source1_temporary_result51) => {
                source_ml_source1_temporary_result51.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_iterator_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    budget: std::rc::Rc<std::cell::RefCell<((f64, f64, f64, f64, f64), Vec<f64>)>>,
) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let source_ml_source2_s1129 = match object.clone() {
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, callable, _, object_collection, cursor, kind, collection_kind, executing, iterated_text) => {
                        if ((((((((!(self::ml_array::includes(&{ let ml_reference_value184 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])]))).borrow().clone(); ml_reference_value184 }, &collection_kind.clone(), 0f64, false, |a, b| a == b)) || callable) || !(self::ml_math::is_safe_integer(cursor))) || (cursor < (0f64))) || {
                            let source_ml_source2_same_value_operand1135 = (-0f64);
                            (((cursor == source_ml_source2_same_value_operand1135) && ((cursor != (0f64)) || (((1f64 / cursor)) == ((1f64 / source_ml_source2_same_value_operand1135))))) || ((cursor != cursor) && (source_ml_source2_same_value_operand1135 != source_ml_source2_same_value_operand1135)))
                        }) || (cursor > (if (collection_kind == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) {
                            9007199254740991f64
                        } else {
                            if (collection_kind == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) {
                                16777216f64
                            } else {
                                65536f64
                            }
                        }))) || !(self::ml_array::includes(&{ let ml_reference_value185 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![107, 101, 121, 115]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])]))).borrow().clone(); ml_reference_value185 }, &kind.clone(), 0f64, false, |a, b| a == b))) || ((collection_kind == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) && ((match ((executing.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))))) || ((collection_kind != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) && {
                            let source_ml_source2_comparison_operand1136 = ((executing.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                            {
                                let source_ml_source2_comparison_operand1137 = None::<()>;
                                !(match (source_ml_source2_comparison_operand1136.clone(), source_ml_source2_comparison_operand1137.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                            }
                        })) {
                            self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 117, 114, 115, 111, 114, 32, 97, 110, 100, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 107, 105, 110, 100]))
                        } else {
                            if (collection_kind == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) {
                                {
                                    let text = ((iterated_text.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                                    if (((object_collection != (self::ml::Big::from_i128(0))) || (kind != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])))) || match (text.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 108, 121, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 99, 117, 114, 115, 111, 114]))
                                    } else {
                                        {
                                            let error = self::read_java_script_ordinary_value_error(heap.clone(), text.clone().expect("a proven present value"));
                                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(error.clone())
                                            } else {
                                                {
                                                    let length = self::read_java_script_semantic_text_length(text.clone().expect("a proven present value"));
                                                    {
                                                        let source_ml_source2_s1138 = match text.clone().expect("a proven present value") {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                if (cursor != (0f64)) {
                                                                    self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 108, 101, 97, 115, 101, 115, 32, 105, 116, 115, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 117, 114, 115, 111, 114]))
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next
                                                                }
                                                            }
                                                            _ => {
                                                                if ((length < (0f64)) || (cursor > length)) {
                                                                    self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next
                                                                }
                                                            }
                                                        };
                                                        match source_ml_source2_s1138.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next => {
                                                                {
                                                                    let source_ml_source1_temporary_read_java_script_collection_iterator_object_error_written56 = { let ml_field_owner186 = budget.clone(); let ml_field_previous186 = ml_field_owner186.borrow().0.3; let ml_field_value186 = self::ml_math::max(0f64, length); ml_field_owner186.borrow_mut().0.3 = ml_field_previous186 + ml_field_value186; () };
                                                                    if (((({ let ml_reference_value187 = budget.borrow().clone(); ml_reference_value187 }).0).3) > (67108864f64)) {
                                                                        self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 101, 120, 116, 115, 32, 97, 110, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 118, 97, 108, 117, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(source_ml_source1_temporary_result56) => {
                                                                self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(source_ml_source1_temporary_result56.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if !(match (((iterated_text.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text::from_units(vec![110, 111, 110, 45, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 102, 97, 109, 105, 108, 105, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 83, 116, 114, 105, 110, 103, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                                } else {
                                    if (object_collection != (self::ml::Big::from_i128(0))) {
                                        {
                                            let collection = {
                                                let source_ml_source2_array_read_value1141 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_array_read_index1142 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), object_collection.clone());
                                                    { let ml_reference_array188 = source_ml_source2_array_read_value1141.clone(); let ml_reference_position188 = source_ml_source2_array_read_index1142; let ml_reference_projection188 = self::ml_array::optional_at(&*ml_reference_array188.borrow(), self::ml_array::number_index(ml_reference_position188)); ml_reference_projection188 }
                                                }
                                            };
                                            if (match (collection.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((collection_kind != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) && (!((((collection_kind == (self::ml_string::Text::from_units(vec![77, 97, 112]))) && match collection.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            }) || ((collection_kind == (self::ml_string::Text::from_units(vec![83, 101, 116]))) && match collection.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            }))) || (cursor > ({ let ml_reference_array189 = (((match collection.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, source_ml_source2_data_property_value1157) => {
                                                    Some((source_ml_source2_data_property_value1157.clone(),))
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, source_ml_source2_data_property_value1158) => {
                                                    Some((source_ml_source2_data_property_value1158.clone(),))
                                                }
                                                _ => {
                                                    None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>,)>
                                                }
                                            }).clone().expect("a proven present value")).0).clone(); let ml_reference_projection189 = ml_reference_array189.borrow().len() as f64; ml_reference_projection189 }))))) {
                                                self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 97, 110, 100, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
                                            }
                                        }
                                    } else {
                                        if ((cursor != (0f64)) || {
                                            let source_ml_source2_comparison_operand1159 = ((executing.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                            match (source_ml_source2_comparison_operand1159.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        }) {
                                            self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 108, 101, 97, 115, 101, 115, 32, 105, 116, 115, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 44, 32, 99, 117, 114, 115, 111, 114, 32, 97, 110, 100, 32, 101, 120, 101, 99, 117, 116, 105, 110, 103, 32, 115, 116, 97, 116, 101]))
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
                    }
                };
            match source_ml_source2_s1129.clone() {
                    self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next => {
                        self::ml_string::Text::from_units(vec![])
                    }
                    self::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(source_ml_source1_temporary_result54) => {
                        source_ml_source1_temporary_result54.clone()
                    }
                }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_weak_collection_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    budget: std::rc::Rc<std::cell::RefCell<((f64, f64, f64, f64, f64), Vec<f64>)>>,
) -> self::ml_string::Text {
    {
        let source_ml_source2_s1183 = if (match object.clone() {
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => true,
            _ => false,
        } || match object.clone() {
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => true,
            _ => false,
        }) {
            {
                let entries = (((match object.clone() {
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1196,
                    ) => Some((source_ml_source2_data_property_value1196.clone(),)),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1197,
                    ) => Some((source_ml_source2_data_property_value1197.clone(),)),
                    _ => {
                        None::<(
                            std::rc::Rc<
                                std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
                            >,
                        )>
                    }
                })
                .clone()
                .expect("a proven present value"))
                .0)
                    .clone();
                if (((((match object.clone() {
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1198,
                        _,
                        _,
                    ) => Some((source_ml_source2_data_property_value1198,)),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value1199,
                        _,
                        _,
                    ) => Some((source_ml_source2_data_property_value1199,)),
                    _ => None::<(bool,)>,
                })
                .clone()
                .expect("a proven present value"))
                .0 || match (entries.clone(), None::<()>) {
                    (_ml_left, Some(_ml_right)) => false,
                    _ => false,
                }) || (({
                    let ml_reference_array190 = entries.clone();
                    let ml_reference_projection190 = ml_reference_array190.borrow().len() as f64;
                    ml_reference_projection190
                }) > (65536f64)))
                    || {
                        let source_ml_source2_comparison_operand1203 = None::<()>;
                        {
                            let source_ml_source2_comparison_operand1204 = None::<()>;
                            !(match (
                                source_ml_source2_comparison_operand1203.clone(),
                                source_ml_source2_comparison_operand1204.clone(),
                            ) {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            })
                        }
                    })
                {
                    self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 119, 101, 97, 107, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_read_java_script_weak_collection_object_error_written59 = {
                            let ml_field_owner192 = budget.clone();
                            let ml_field_previous192 = ml_field_owner192.borrow().0.1;
                            let ml_field_value192 = {
                                let ml_reference_array192 = entries.clone();
                                let ml_reference_projection192 =
                                    ml_reference_array192.borrow().len() as f64;
                                ml_reference_projection192
                            };
                            ml_field_owner192.borrow_mut().0.1 =
                                ml_field_previous192 + ml_field_value192;
                            ()
                        };
                        if (((({
                            let ml_reference_value193 = budget.borrow().clone();
                            ml_reference_value193
                        })
                        .0)
                            .1)
                            > (1048576f64))
                        {
                            self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 97, 110, 100, 32, 115, 116, 114, 111, 110, 103, 32, 115, 108, 111, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let previous_weak_key = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source1_temporary_index57 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s1205 = self::source_ml_source1_temporary_read_java_script_weak_collection_object_error_loop60(heap.clone(), object.clone(), budget.clone(), Some(previous_weak_key.clone()), entries.clone(), source_ml_source1_temporary_index57.clone());
                                        match source_ml_source2_s1205.clone() {
                                            self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Done => {
                                                self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Next
                                            }
                                            self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(source_ml_source1_temporary_result60) => {
                                                self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Return(source_ml_source1_temporary_result60.clone())
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Next
        };
        match source_ml_source2_s1183.clone() {
            self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Return(source_ml_source1_temporary_result58) => {
                source_ml_source1_temporary_result58.clone()
            }
        }
    }
}
