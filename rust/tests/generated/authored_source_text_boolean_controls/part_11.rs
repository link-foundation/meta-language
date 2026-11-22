pub fn compare_java_script_collection_keys(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> f64 {
    {
        let a = self::read_java_script_collection_key_rank(left.clone());
        {
            let b = self::read_java_script_collection_key_rank(right.clone());
            if (a != b) {
                if (a < b) {
                    (-1f64)
                } else {
                    1f64
                }
            } else {
                {
                    let source_ml_source1_s1431 = match left.clone() {
                        self::JavaScriptSemanticValue::Boolean(left_value) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Boolean(value) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(if (left_value == value) {
                                        0f64
                                    } else {
                                        if left_value {
                                            1f64
                                        } else {
                                            (-1f64)
                                        }
                                    })
                                }
                                _ => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Number(left_value_2) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Number(value_2) => {
                                    if self::test_java_script_collection_key(left.clone(), right.clone()) {
                                        self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(0f64)
                                    } else {
                                        if left_value_2.is_nan() {
                                            self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(1f64)
                                        } else {
                                            if value_2.is_nan() {
                                                self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return((-1f64))
                                            } else {
                                                self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(if (left_value_2 < value_2) {
                                                    (-1f64)
                                                } else {
                                                    1f64
                                                })
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(left_value_3) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Integer(value_3) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(if (left_value_3 == value_3) {
                                        0f64
                                    } else {
                                        if (left_value_3 < value_3) {
                                            (-1f64)
                                        } else {
                                            1f64
                                        }
                                    })
                                }
                                _ => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::SourceText(_, left_text) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::SourceText(_, text) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(if (left_text == text) {
                                        0f64
                                    } else {
                                        if <self::ml_string::Text as std::cmp::PartialOrd>::lt(&left_text, &text) {
                                            (-1f64)
                                        } else {
                                            1f64
                                        }
                                    })
                                }
                                _ => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Next
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(left_identity) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Symbol(identity) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(if (left_identity == identity) {
                                        0f64
                                    } else {
                                        if (left_identity < identity) {
                                            (-1f64)
                                        } else {
                                            1f64
                                        }
                                    })
                                }
                                _ => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Object(left_identity_2, _) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(if (left_identity_2 == identity_2) {
                                        0f64
                                    } else {
                                        if (left_identity_2 < identity_2) {
                                            (-1f64)
                                        } else {
                                            1f64
                                        }
                                    })
                                }
                                _ => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(0f64)
                                }
                            }
                        }
                        _ => {
                            self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Next
                        }
                    };
                    match source_ml_source1_s1431.clone() {
                        self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Next => {
                            if (a == (5f64)) {
                                {
                                    let first = self::read_java_script_semantic_text_length(left.clone());
                                    {
                                        let second = self::read_java_script_semantic_text_length(right.clone());
                                        {
                                            let length = self::ml_math::min(first, second);
                                            {
                                                let index = 0f64;
                                                {
                                                    let source_ml_source1_s1483 = self::source_ml_temporary_compare_java_script_collection_keys_loop29(left.clone(), right.clone(), length, index);
                                                    match source_ml_source1_s1483.clone() {
                                                        self::MlTemporaryCompareJavaScriptCollectionKeysLoop29Result::MlTemporaryCompareJavaScriptCollectionKeysLoop29Done => {
                                                            if (first == second) {
                                                                0f64
                                                            } else {
                                                                if (first < second) {
                                                                    (-1f64)
                                                                } else {
                                                                    1f64
                                                                }
                                                            }
                                                        }
                                                        self::MlTemporaryCompareJavaScriptCollectionKeysLoop29Result::MlTemporaryCompareJavaScriptCollectionKeysLoop29Return(source_ml_temporary_result29) => {
                                                            source_ml_temporary_result29
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                0f64
                            }
                        }
                        self::MlTemporaryCompareJavaScriptCollectionKeysJoin28::MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(source_ml_temporary_result29_2) => {
                            source_ml_temporary_result29_2
                        }
                    }
                }
            }
        }
    }
}

pub fn order_java_script_collection_keys(values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    {
        let source = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value171 = values.borrow().clone(); ml_reference_value171 }, 0f64, f64::INFINITY)));
        {
            let width = 1f64;
            {
                let source_2 = self::source_ml_temporary_order_java_script_collection_keys_loop30(source.clone(), width);
                source_2.clone()
            }
        }
    }
}

pub fn find_java_script_ordinary_object(objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, identity: self::ml::Big) -> f64 {
    self::find_java_script_heap_directory_object(objects.clone(), identity.clone())
}

pub fn test_java_script_ordinary_symbol(symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, identity: self::ml::Big) -> bool {
    self::test_java_script_heap_directory_symbol(symbols.clone(), identity.clone())
}

pub fn read_java_script_ordinary_value_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    self::read_java_script_heap_directory_value_error(heap.clone(), value.clone())
}

pub fn read_java_script_ordinary_key_error(heap: self::JavaScriptOrdinaryHeap, key: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    self::read_java_script_heap_directory_key_error(heap.clone(), key.clone())
}

pub fn find_java_script_ordinary_property(properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, key: self::JavaScriptSemanticValue) -> f64 {
    self::find_java_script_heap_directory_property(properties.clone(), key.clone())
}

pub fn read_java_script_opaque_object_error(heap: self::JavaScriptOrdinaryHeap, object: self::JavaScriptOrdinaryObject) -> self::ml_string::Text {
    {
        let source_ml_source1_s2360 = if ((((((((match object.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                false
            }
            _ => {
                true
            }
        } && match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                false
            }
            _ => {
                true
            }
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                false
            }
            _ => {
                true
            }
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                false
            }
            _ => {
                true
            }
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                false
            }
            _ => {
                true
            }
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                false
            }
            _ => {
                true
            }
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                false
            }
            _ => {
                true
            }
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                false
            }
            _ => {
                true
            }
        }) && match object.clone() {
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                false
            }
            _ => {
                true
            }
        }) {
            if (((match object.clone() {
                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source1_data_property_value3216, _, _, _) => {
                    Some((source_ml_source1_data_property_value3216,))
                }
                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source1_data_property_value3217, _, _, _) => {
                    Some((source_ml_source1_data_property_value3217,))
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source1_data_property_value3218, _, _, _) => {
                    Some((source_ml_source1_data_property_value3218,))
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source1_data_property_value3219, _, _, _) => {
                    Some((source_ml_source1_data_property_value3219,))
                }
                _ => {
                    None::<(bool,)>
                }
            }).clone().expect("a proven present value")).0 || (({ let ml_reference_array172 = (((match object.clone() {
                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value3224, _, _) => {
                    Some((source_ml_source1_data_property_value3224.clone(),))
                }
                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value3225, _, _) => {
                    Some((source_ml_source1_data_property_value3225.clone(),))
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value3226, _, _) => {
                    Some((source_ml_source1_data_property_value3226.clone(),))
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value3227, _, _) => {
                    Some((source_ml_source1_data_property_value3227.clone(),))
                }
                _ => {
                    None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                }
            }).clone().expect("a proven present value")).0).clone(); let ml_reference_projection172 = ml_reference_array172.borrow().len() as f64; ml_reference_projection172 }) != (0f64))) {
                self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 111, 98, 106, 101, 99, 116, 115, 32, 107, 101, 101, 112, 32, 104, 111, 115, 116, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 117, 116, 115, 105, 100, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116, 115]))
            } else {
                {
                    let source_ml_temporary_values33 = std::rc::Rc::new(std::cell::RefCell::new(vec![Some((((match object.clone() {
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, source_ml_source1_data_property_value3228, _) => {
                            let source_ml_source1_data_property_value3228 = *source_ml_source1_data_property_value3228;
                            Some((source_ml_source1_data_property_value3228.clone(),))
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, source_ml_source1_data_property_value3229, _) => {
                            let source_ml_source1_data_property_value3229 = *source_ml_source1_data_property_value3229;
                            Some((source_ml_source1_data_property_value3229.clone(),))
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, source_ml_source1_data_property_value3230, _) => {
                            let source_ml_source1_data_property_value3230 = *source_ml_source1_data_property_value3230;
                            Some((source_ml_source1_data_property_value3230.clone(),))
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, source_ml_source1_data_property_value3231, _) => {
                            let source_ml_source1_data_property_value3231 = *source_ml_source1_data_property_value3231;
                            Some((source_ml_source1_data_property_value3231.clone(),))
                        }
                        _ => {
                            None::<(self::JavaScriptSemanticValue,)>
                        }
                    }).clone().expect("a proven present value")).0).clone()), Some((((match object.clone() {
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, source_ml_source1_data_property_value3232) => {
                            let source_ml_source1_data_property_value3232 = *source_ml_source1_data_property_value3232;
                            Some((source_ml_source1_data_property_value3232.clone(),))
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, source_ml_source1_data_property_value3233) => {
                            let source_ml_source1_data_property_value3233 = *source_ml_source1_data_property_value3233;
                            Some((source_ml_source1_data_property_value3233.clone(),))
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, source_ml_source1_data_property_value3234) => {
                            let source_ml_source1_data_property_value3234 = *source_ml_source1_data_property_value3234;
                            Some((source_ml_source1_data_property_value3234.clone(),))
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, source_ml_source1_data_property_value3235) => {
                            let source_ml_source1_data_property_value3235 = *source_ml_source1_data_property_value3235;
                            Some((source_ml_source1_data_property_value3235.clone(),))
                        }
                        _ => {
                            None::<(self::JavaScriptSemanticValue,)>
                        }
                    }).clone().expect("a proven present value")).0).clone())]));
                    {
                        let source_ml_temporary_index33 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s3236 = self::source_ml_temporary_read_java_script_opaque_object_error_loop35(heap.clone(), source_ml_temporary_values33.clone(), source_ml_temporary_index33.clone());
                            match source_ml_source1_s3236.clone() {
                                self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Done => {
                                    self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34Next
                                }
                                self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Return(source_ml_temporary_result35) => {
                                    self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34Return(source_ml_temporary_result35.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34Next
        };
        match source_ml_source1_s2360.clone() {
            self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34::MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34Return(source_ml_temporary_result34) => {
                source_ml_temporary_result34.clone()
            }
        }
    }
}

pub fn read_java_script_closure_object_error(heap: self::JavaScriptOrdinaryHeap, object: self::JavaScriptOrdinaryObject, budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            {
                let source_ml_source1_s3546 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, callable, _, object_owner, ordinal, captures, lexical_receiver, lexical_new_target) => {
                        let lexical_receiver = *lexical_receiver;
                        if (((((((!callable || (object_owner <= (self::ml::Big::from_i128(0)))) || (object_owner > (self::ml::Big::from_i128(18446744073709551615)))) || !(self::ml_math::is_safe_integer(ordinal))) || (ordinal < (0f64))) || (ordinal > (4095f64))) || {
                            let source_ml_source1_same_value_operand3637 = (-0f64);
                            (((ordinal == source_ml_source1_same_value_operand3637) && ((ordinal != (0f64)) || (((1f64 / ordinal)) == ((1f64 / source_ml_source1_same_value_operand3637))))) || ((ordinal != ordinal) && (source_ml_source1_same_value_operand3637 != source_ml_source1_same_value_operand3637)))
                        }) || (({ let ml_reference_array173 = captures.clone(); let ml_reference_projection173 = ml_reference_array173.borrow().len() as f64; ml_reference_projection173 }) > (4096f64))) {
                            self::MlTemporaryReadJavaScriptClosureObjectErrorJoin37::MlTemporaryReadJavaScriptClosureObjectErrorJoin37Return(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114, 44, 32, 108, 101, 120, 105, 99, 97, 108, 32, 115, 105, 116, 101, 44, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 98, 114, 97, 110, 100, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 97, 112, 116, 117, 114, 101, 115]))
                        } else {
                            {
                                let source_ml_temporary_read_java_script_closure_object_error_written38 = { let ml_field_owner175 = budget.clone(); let ml_field_previous175 = ml_field_owner175.borrow().0; let ml_field_value175 = { let ml_reference_array175 = captures.clone(); let ml_reference_projection175 = ml_reference_array175.borrow().len() as f64; ml_reference_projection175 }; ml_field_owner175.borrow_mut().0 = ml_field_previous175 + ml_field_value175; () };
                                if ((({ let ml_reference_value176 = budget.borrow().clone(); ml_reference_value176 }).0) > (1048576f64)) {
                                    self::MlTemporaryReadJavaScriptClosureObjectErrorJoin37::MlTemporaryReadJavaScriptClosureObjectErrorJoin37Return(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112, 32, 118, 97, 108, 117, 101, 32, 98, 117, 100, 103, 101, 116]))
                                } else {
                                    {
                                        let owner = {
                                            let source_ml_source1_array_read_value3640 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source1_array_read_index3641 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), object_owner.clone());
                                                { let ml_reference_array177 = source_ml_source1_array_read_value3640.clone(); let ml_reference_position177 = source_ml_source1_array_read_index3641; let ml_reference_projection177 = self::ml_array::optional_at(&*ml_reference_array177.borrow(), self::ml_array::number_index(ml_reference_position177)); ml_reference_projection177 }
                                            }
                                        };
                                        if (((match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match owner.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source1_data_property_value3642, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value3642
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source1_data_property_value3643, _, _, _, _) => {
                                                source_ml_source1_data_property_value3643
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source1_data_property_value3644, _) => {
                                                source_ml_source1_data_property_value3644
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source1_data_property_value3645, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value3645
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source1_data_property_value3646, _, _) => {
                                                source_ml_source1_data_property_value3646
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source1_data_property_value3647, _, _) => {
                                                source_ml_source1_data_property_value3647
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source1_data_property_value3648, _, _) => {
                                                source_ml_source1_data_property_value3648
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source1_data_property_value3649, _, _) => {
                                                source_ml_source1_data_property_value3649
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source1_data_property_value3650, _) => {
                                                source_ml_source1_data_property_value3650
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source1_data_property_value3651, _, _, _) => {
                                                source_ml_source1_data_property_value3651
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source1_data_property_value3652, _, _, _) => {
                                                source_ml_source1_data_property_value3652
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source1_data_property_value3653, _, _, _) => {
                                                source_ml_source1_data_property_value3653
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source1_data_property_value3654, _, _, _) => {
                                                source_ml_source1_data_property_value3654
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
                                            self::MlTemporaryReadJavaScriptClosureObjectErrorJoin37::MlTemporaryReadJavaScriptClosureObjectErrorJoin37Return(self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 99, 111, 100, 101, 32, 111, 119, 110, 101, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 116, 105, 99, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 44, 32, 110, 101, 118, 101, 114, 32, 97, 110, 111, 116, 104, 101, 114, 32, 114, 117, 110, 116, 105, 109, 101, 32, 99, 108, 111, 115, 117, 114, 101, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
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
                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value3854) => {
                                                        Some((source_ml_source1_data_property_value3854,))
                                                    }
                                                    _ => {
                                                        None::<(bool,)>
                                                    }
                                                }).clone().expect("a proven present value")).0)) || ((self::read_java_script_ordinary_value_error(heap.clone(), new_target.clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![]))))) {
                                                    self::MlTemporaryReadJavaScriptClosureObjectErrorJoin37::MlTemporaryReadJavaScriptClosureObjectErrorJoin37Return(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 116, 105, 111, 110, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                                                } else {
                                                    {
                                                        let source_ml_temporary_values36 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![lexical_receiver.clone()], { let ml_reference_value178 = captures.borrow().clone(); ml_reference_value178 })));
                                                        {
                                                            let source_ml_temporary_index36 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source1_s3855 = self::source_ml_temporary_read_java_script_closure_object_error_loop39(heap.clone(), source_ml_temporary_values36.clone(), source_ml_temporary_index36.clone());
                                                                match source_ml_source1_s3855.clone() {
                                                                    self::MlTemporaryReadJavaScriptClosureObjectErrorLoop39Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop39Done => {
                                                                        self::MlTemporaryReadJavaScriptClosureObjectErrorJoin37::MlTemporaryReadJavaScriptClosureObjectErrorJoin37Next
                                                                    }
                                                                    self::MlTemporaryReadJavaScriptClosureObjectErrorLoop39Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop39Return(source_ml_temporary_result39) => {
                                                                        self::MlTemporaryReadJavaScriptClosureObjectErrorJoin37::MlTemporaryReadJavaScriptClosureObjectErrorJoin37Return(source_ml_temporary_result39.clone())
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
                        self::MlTemporaryReadJavaScriptClosureObjectErrorJoin37::MlTemporaryReadJavaScriptClosureObjectErrorJoin37Next
                    }
                };
                match source_ml_source1_s3546.clone() {
                    self::MlTemporaryReadJavaScriptClosureObjectErrorJoin37::MlTemporaryReadJavaScriptClosureObjectErrorJoin37Next => {
                        self::ml_string::Text::from_units(vec![])
                    }
                    self::MlTemporaryReadJavaScriptClosureObjectErrorJoin37::MlTemporaryReadJavaScriptClosureObjectErrorJoin37Return(source_ml_temporary_result37) => {
                        source_ml_temporary_result37.clone()
                    }
                }
            }
        }
    }
}

pub fn read_java_script_bound_object_error(heap: self::JavaScriptOrdinaryHeap, object: self::JavaScriptOrdinaryObject, budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>) -> self::ml_string::Text {
    {
        let source_ml_source1_s3960 = match object.clone() {
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
                    self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value4061) => {
                        Some(source_ml_source1_data_property_value4061)
                    }
                    _ => {
                        None::<bool>
                    }
                } { Some(ml_present) => ml_present, None => false })) || (({ let ml_reference_array179 = bound_arguments.clone(); let ml_reference_projection179 = ml_reference_array179.borrow().len() as f64; ml_reference_projection179 }) > (1048576f64))) {
                    self::MlTemporaryReadJavaScriptBoundObjectErrorJoin41::MlTemporaryReadJavaScriptBoundObjectErrorJoin41Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 116, 97, 114, 103, 101, 116, 115, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116, 115]))
                } else {
                    {
                        let source_ml_temporary_read_java_script_bound_object_error_written42 = { let ml_field_owner181 = budget.clone(); let ml_field_previous181 = ml_field_owner181.borrow().0; let ml_field_value181 = ({ let ml_reference_array181 = bound_arguments.clone(); let ml_reference_projection181 = ml_reference_array181.borrow().len() as f64; ml_reference_projection181 } + 2f64); ml_field_owner181.borrow_mut().0 = ml_field_previous181 + ml_field_value181; () };
                        if ((({ let ml_reference_value182 = budget.borrow().clone(); ml_reference_value182 }).0) > (1048576f64)) {
                            self::MlTemporaryReadJavaScriptBoundObjectErrorJoin41::MlTemporaryReadJavaScriptBoundObjectErrorJoin41Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 115, 108, 111, 116, 115, 32, 97, 110, 100, 32, 99, 108, 111, 115, 117, 114, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 115, 104, 97, 114, 101, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 118, 97, 108, 117, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let source_ml_temporary_values40 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![bound_target.clone(), bound_this.clone()], { let ml_reference_value183 = bound_arguments.borrow().clone(); ml_reference_value183 })));
                                {
                                    let source_ml_temporary_index40 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s4064 = self::source_ml_temporary_read_java_script_bound_object_error_loop43(heap.clone(), source_ml_temporary_values40.clone(), source_ml_temporary_index40.clone());
                                        match source_ml_source1_s4064.clone() {
                                            self::MlTemporaryReadJavaScriptBoundObjectErrorLoop43Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop43Done => {
                                                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin41::MlTemporaryReadJavaScriptBoundObjectErrorJoin41Next
                                            }
                                            self::MlTemporaryReadJavaScriptBoundObjectErrorLoop43Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop43Return(source_ml_temporary_result43) => {
                                                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin41::MlTemporaryReadJavaScriptBoundObjectErrorJoin41Return(source_ml_temporary_result43.clone())
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
                self::MlTemporaryReadJavaScriptBoundObjectErrorJoin41::MlTemporaryReadJavaScriptBoundObjectErrorJoin41Next
            }
        };
        match source_ml_source1_s3960.clone() {
            self::MlTemporaryReadJavaScriptBoundObjectErrorJoin41::MlTemporaryReadJavaScriptBoundObjectErrorJoin41Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptBoundObjectErrorJoin41::MlTemporaryReadJavaScriptBoundObjectErrorJoin41Return(source_ml_temporary_result41) => {
                source_ml_temporary_result41.clone()
            }
        }
    }
}

pub fn read_java_script_collection_iterator_object_error(heap: self::JavaScriptOrdinaryHeap, object: self::JavaScriptOrdinaryObject, budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            {
                let source_ml_source1_s4376 = match object.clone() {
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, callable, _, object_collection, cursor, kind, collection_kind, executing, iterated_text) => {
                        if ((((((((!(self::ml_array::includes(&{ let ml_reference_value184 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])]))).borrow().clone(); ml_reference_value184 }, &collection_kind.clone(), 0f64, false, |a, b| a == b)) || callable) || !(self::ml_math::is_safe_integer(cursor))) || (cursor < (0f64))) || {
                            let source_ml_source1_same_value_operand4466 = (-0f64);
                            (((cursor == source_ml_source1_same_value_operand4466) && ((cursor != (0f64)) || (((1f64 / cursor)) == ((1f64 / source_ml_source1_same_value_operand4466))))) || ((cursor != cursor) && (source_ml_source1_same_value_operand4466 != source_ml_source1_same_value_operand4466)))
                        }) || (cursor > (if (collection_kind == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) {
                            9007199254740991f64
                        } else {
                            if (collection_kind == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) {
                                16777216f64
                            } else {
                                65536f64
                            }
                        }))) || !(self::ml_array::includes(&{ let ml_reference_value185 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![107, 101, 121, 115]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])]))).borrow().clone(); ml_reference_value185 }, &kind.clone(), 0f64, false, |a, b| a == b))) || ((collection_kind == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) && ((match ((executing.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))))) || ((collection_kind != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) && {
                            let source_ml_source1_comparison_operand4467 = ((executing.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                            {
                                let source_ml_source1_comparison_operand4468 = None::<()>;
                                !(match (source_ml_source1_comparison_operand4467.clone(), source_ml_source1_comparison_operand4468.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                            }
                        })) {
                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 117, 114, 115, 111, 114, 32, 97, 110, 100, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 107, 105, 110, 100]))
                        } else {
                            if (collection_kind == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) {
                                {
                                    let text = ((iterated_text.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                                    if (((object_collection != (self::ml::Big::from_i128(0))) || (kind != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])))) || match (text.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Return(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 108, 121, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 99, 117, 114, 115, 111, 114]))
                                    } else {
                                        {
                                            let error = self::read_java_script_ordinary_value_error(heap.clone(), text.clone().expect("a proven present value"));
                                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Return(error.clone())
                                            } else {
                                                {
                                                    let length = self::read_java_script_semantic_text_length(text.clone().expect("a proven present value"));
                                                    {
                                                        let source_ml_source1_s4478 = match text.clone().expect("a proven present value") {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                if (cursor != (0f64)) {
                                                                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45Return(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 108, 101, 97, 115, 101, 115, 32, 105, 116, 115, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 117, 114, 115, 111, 114]))
                                                                } else {
                                                                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45Next
                                                                }
                                                            }
                                                            _ => {
                                                                if ((length < (0f64)) || (cursor > length)) {
                                                                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                                } else {
                                                                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45Next
                                                                }
                                                            }
                                                        };
                                                        match source_ml_source1_s4478.clone() {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45Next => {
                                                                {
                                                                    let source_ml_temporary_read_java_script_collection_iterator_object_error_written46 = { let ml_field_owner186 = budget.clone(); let ml_field_previous186 = ml_field_owner186.borrow().3; let ml_field_value186 = self::ml_math::max(0f64, length); ml_field_owner186.borrow_mut().3 = ml_field_previous186 + ml_field_value186; () };
                                                                    if ((({ let ml_reference_value187 = budget.borrow().clone(); ml_reference_value187 }).3) > (67108864f64)) {
                                                                        self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 101, 120, 116, 115, 32, 97, 110, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 118, 97, 108, 117, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Next
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45Return(source_ml_temporary_result46) => {
                                                                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Return(source_ml_temporary_result46.clone())
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
                                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Return(self::ml_string::Text::from_units(vec![110, 111, 110, 45, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 102, 97, 109, 105, 108, 105, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 83, 116, 114, 105, 110, 103, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                                } else {
                                    if (object_collection != (self::ml::Big::from_i128(0))) {
                                        {
                                            let collection = {
                                                let source_ml_source1_array_read_value4490 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source1_array_read_index4491 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), object_collection.clone());
                                                    { let ml_reference_array188 = source_ml_source1_array_read_value4490.clone(); let ml_reference_position188 = source_ml_source1_array_read_index4491; let ml_reference_projection188 = self::ml_array::optional_at(&*ml_reference_array188.borrow(), self::ml_array::number_index(ml_reference_position188)); ml_reference_projection188 }
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
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, source_ml_source1_data_property_value4684) => {
                                                    Some((source_ml_source1_data_property_value4684.clone(),))
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, source_ml_source1_data_property_value4685) => {
                                                    Some((source_ml_source1_data_property_value4685.clone(),))
                                                }
                                                _ => {
                                                    None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>,)>
                                                }
                                            }).clone().expect("a proven present value")).0).clone(); let ml_reference_projection189 = ml_reference_array189.borrow().len() as f64; ml_reference_projection189 }))))) {
                                                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 97, 110, 100, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                            } else {
                                                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Next
                                            }
                                        }
                                    } else {
                                        if ((cursor != (0f64)) || {
                                            let source_ml_source1_comparison_operand4686 = ((executing.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                            match (source_ml_source1_comparison_operand4686.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                        }) {
                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Return(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 108, 101, 97, 115, 101, 115, 32, 105, 116, 115, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 44, 32, 99, 117, 114, 115, 111, 114, 32, 97, 110, 100, 32, 101, 120, 101, 99, 117, 116, 105, 110, 103, 32, 115, 116, 97, 116, 101]))
                                        } else {
                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Next
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Next
                    }
                };
                match source_ml_source1_s4376.clone() {
                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Next => {
                        self::ml_string::Text::from_units(vec![])
                    }
                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Return(source_ml_temporary_result44) => {
                        source_ml_temporary_result44.clone()
                    }
                }
            }
        }
    }
}

pub fn read_java_script_weak_collection_object_error(heap: self::JavaScriptOrdinaryHeap, object: self::JavaScriptOrdinaryObject, budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>) -> self::ml_string::Text {
    {
        let source_ml_source1_s4888 = if (match object.clone() {
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                true
            }
            _ => {
                false
            }
        } || match object.clone() {
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                true
            }
            _ => {
                false
            }
        }) {
            {
                let entries = (((match object.clone() {
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, source_ml_source1_data_property_value5079) => {
                        Some((source_ml_source1_data_property_value5079.clone(),))
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, source_ml_source1_data_property_value5080) => {
                        Some((source_ml_source1_data_property_value5080.clone(),))
                    }
                    _ => {
                        None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>>,)>
                    }
                }).clone().expect("a proven present value")).0).clone();
                if (((((match object.clone() {
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source1_data_property_value5081, _, _) => {
                        Some((source_ml_source1_data_property_value5081,))
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source1_data_property_value5082, _, _) => {
                        Some((source_ml_source1_data_property_value5082,))
                    }
                    _ => {
                        None::<(bool,)>
                    }
                }).clone().expect("a proven present value")).0 || match (entries.clone(), None::<()>) { (_ml_left, Some(_ml_right)) => false, _ => false }) || (({ let ml_reference_array190 = entries.clone(); let ml_reference_projection190 = ml_reference_array190.borrow().len() as f64; ml_reference_projection190 }) > (65536f64))) || {
                    let source_ml_source1_comparison_operand5086 = None::<()>;
                    {
                        let source_ml_source1_comparison_operand5087 = None::<()>;
                        !(match (source_ml_source1_comparison_operand5086.clone(), source_ml_source1_comparison_operand5087.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                    }
                }) {
                    self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 119, 101, 97, 107, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                } else {
                    {
                        let source_ml_temporary_read_java_script_weak_collection_object_error_written49 = { let ml_field_owner192 = budget.clone(); let ml_field_previous192 = ml_field_owner192.borrow().1; let ml_field_value192 = { let ml_reference_array192 = entries.clone(); let ml_reference_projection192 = ml_reference_array192.borrow().len() as f64; ml_reference_projection192 }; ml_field_owner192.borrow_mut().1 = ml_field_previous192 + ml_field_value192; () };
                        if ((({ let ml_reference_value193 = budget.borrow().clone(); ml_reference_value193 }).1) > (1048576f64)) {
                            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 97, 110, 100, 32, 115, 116, 114, 111, 110, 103, 32, 115, 108, 111, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let previous_weak_key = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_temporary_index47 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s5088 = self::source_ml_temporary_read_java_script_weak_collection_object_error_loop50(heap.clone(), object.clone(), budget.clone(), Some(previous_weak_key.clone()), entries.clone(), source_ml_temporary_index47.clone());
                                        match source_ml_source1_s5088.clone() {
                                            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Done => {
                                                self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48Next
                                            }
                                            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Return(source_ml_temporary_result50) => {
                                                self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48Return(source_ml_temporary_result50.clone())
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
            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48Next
        };
        match source_ml_source1_s4888.clone() {
            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48Return(source_ml_temporary_result48) => {
                source_ml_temporary_result48.clone()
            }
        }
    }
}
