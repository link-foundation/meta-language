// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptCollectionIteration;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Result;
use super::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_temporary_advance_java_script_collection_iterator_loop136;
use super::source_ml_temporary_insert_java_script_ordinary_object_loop129;
use super::test_java_script_ordinary_symbol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn insert_java_script_ordinary_object(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool)>> {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_2 = heap_error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                std::rc::Rc::new(std::cell::RefCell::new(
                    heap_error.clone(),
                    heap.clone(),
                    false,
                ))
            } else {
                let ml_product_condition_1 = {
                    let ml_reference_array200 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value14171,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value14171.clone(),
                    };
                    ml_reference_array200.borrow().len() as f64
                };
                if ml_product_condition_1 {
                    std::rc::Rc::new(std::cell::RefCell::new(
                        self::ml_string::Text::from_units(vec![
                            111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105,
                            100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116,
                            111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115,
                            32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117,
                            114, 99, 101, 32, 98, 117, 100, 103, 101, 116,
                        ]),
                        heap.clone(),
                        false,
                    ))
                } else if (((identity <= (self::ml::Big::from_i128(0)))
                    || (identity > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                    || ((self::find_java_script_ordinary_object(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value14176,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value14176.clone(),
                        },
                        identity.clone(),
                    )) >= 0f64))
                    || self::test_java_script_ordinary_symbol(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                _,
                                source_ml_source1_data_property_value14177,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value14177.clone(),
                        },
                        identity.clone(),
                    )
                {
                    std::rc::Rc::new(std::cell::RefCell::new(
                        self::ml_string::Text::from_units(vec![
                            97, 32, 102, 114, 101, 115, 104, 32, 111, 114, 100, 105, 110, 97, 114,
                            121, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114,
                            101, 115, 32, 105, 116, 115, 32, 100, 105, 115, 116, 105, 110, 99, 116,
                            32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108,
                            111, 99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100,
                            54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116,
                            105, 116, 121,
                        ]),
                        heap.clone(),
                        false,
                    ))
                } else if match prototype.clone() {
                    self::JavaScriptSemanticValue::Null => false,
                    _ => true,
                } && match prototype.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => false,
                    _ => true,
                } {
                    std::rc::Rc::new(std::cell::RefCell::new(
                        self::ml_string::Text::from_units(vec![
                            97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111,
                            116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32,
                            110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97,
                            114, 101, 100, 32, 79, 98, 106, 101, 99, 116,
                        ]),
                        heap.clone(),
                        false,
                    ))
                } else {
                    {
                        let error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            prototype.clone(),
                        );
                        {
                            let ml_library_inequality_1 =
                                error != (self::ml_string::Text::from_units(vec![]));
                            if ml_library_inequality_1 {
                                std::rc::Rc::new(std::cell::RefCell::new(
                                    error.clone(),
                                    heap.clone(),
                                    false,
                                ))
                            } else {
                                {
                                    let source_ml_source1_s15075 = match prototype.clone() {
                                        self::JavaScriptSemanticValue::Undefined => {
                                            self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Next
                                        }
                                        self::JavaScriptSemanticValue::Null => {
                                            self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Next
                                        }
                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                            self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Next
                                        }
                                        self::JavaScriptSemanticValue::Number(_) => {
                                            self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Next
                                        }
                                        self::JavaScriptSemanticValue::Integer(_) => {
                                            self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Next
                                        }
                                        self::JavaScriptSemanticValue::Text(_) => {
                                            self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Next
                                        }
                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                            self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Next
                                        }
                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                            self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Next
                                        }
                                        self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                            {
                                                let parent = {
                                                    let source_ml_source1_array_read_value15086 = match heap.clone() {
                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value15084, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value15084.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_array_read_index15087 = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value15085, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value15085.clone()
                                                            }
                                                        }, identity_2.clone());
                                                        { let ml_reference_array201 = source_ml_source1_array_read_value15086; let ml_reference_position201 = source_ml_source1_array_read_index15087; self::ml_array::optional_at(&*ml_reference_array201.borrow(), self::ml_array::number_index(ml_reference_position201)) }
                                                    }
                                                };
                                                if ((((((((!(match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match parent.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }) && match parent.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }) && match parent.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }) && match parent.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }) && match parent.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }) && match parent.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }) && match parent.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }) && match parent.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }) && match parent.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                } {
                                                    self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 119, 105, 116, 104, 32, 97, 110, 32, 111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 104, 111, 115, 116, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), false)))
                                                } else {
                                                    self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Next
                                                }
                                            }
                                        }
                                    };
                                    match source_ml_source1_s15075 {
                                        self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Next => {
                                            {
                                                let at = 0f64;
                                                {
                                                    let source_ml_temporary_values127 = match heap.clone() {
                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value15946, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value15946.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_temporary_index127 = self::ml::Big::from_i128(0);
                                                        {
                                                            let at_2 = self::source_ml_temporary_insert_java_script_ordinary_object_loop129(identity.clone(), at, source_ml_temporary_values127, source_ml_temporary_index127);
                                                            {
                                                                let object = {
                                                                    let source_ml_source1_constructor_field15951 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(identity.clone(), Box::new(prototype.clone()), true, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, false, source_ml_source1_constructor_field15951)
                                                                };
                                                                {
                                                                    let objects = std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_concat_value15956 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ (match heap.clone() {
                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value15953, _, _, _, _, _) => {
                                                                                source_ml_source1_data_property_value15953.clone()
                                                                            }
                                                                        }).borrow().clone() }, 0f64, at_2)));
                                                                        {
                                                                            let source_ml_source1_concat_value15957 = std::rc::Rc::new(std::cell::RefCell::new(vec![object]));
                                                                            {
                                                                                let source_ml_source1_concat_value15958 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ (match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value15955, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value15955.clone()
                                                                                    }
                                                                                }).borrow().clone() }, at_2, f64::INFINITY)));
                                                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), { source_ml_source1_concat_value15956.borrow().clone() }), { source_ml_source1_concat_value15957.borrow().clone() }), { source_ml_source1_concat_value15958.borrow().clone() })
                                                                            }
                                                                        }
                                                                    }));
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field15973 = {
                                                                            let source_ml_source1_nominal_field15960 = match heap.clone() {
                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value15961, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value15961.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field15962 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value15963, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value15963.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_field15964 = match heap.clone() {
                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source1_data_property_value15965, _, _, _) => {
                                                                                            source_ml_source1_data_property_value15965.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field15966 = match heap.clone() {
                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value15967, _, _) => {
                                                                                                source_ml_source1_data_property_value15967.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field15968 = match heap.clone() {
                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source1_data_property_value15969, _) => {
                                                                                                    source_ml_source1_data_property_value15969.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field15970 = match heap.clone() {
                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value15971) => {
                                                                                                        source_ml_source1_data_property_value15971.clone()
                                                                                                    }
                                                                                                };
                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects, source_ml_source1_nominal_field15962, source_ml_source1_nominal_field15964, source_ml_source1_nominal_field15966, source_ml_source1_nominal_field15968, source_ml_source1_nominal_field15970)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        };
                                                                        (self::ml_string::Text::from_units(vec![]), source_ml_source1_field15973, true)
                                                                    }))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128::MlTemporaryInsertJavaScriptOrdinaryObjectJoin128Return(source_ml_temporary_result129) => {
                                            source_ml_temporary_result129.clone()
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
pub fn move_java_script_collection_iterator_cursor(
    heap: self::JavaScriptOrdinaryHeap,
    iterator: Option<self::JavaScriptOrdinaryObject>,
    collection: self::ml::Big,
    cursor: f64,
) -> self::JavaScriptOrdinaryHeap {
    if match (iterator.clone(), None::<()>) {
        (Some(_ml_left), Some(_ml_right)) => false,
        (None, None) => true,
        _ => false,
    } {
        heap.clone()
    } else {
        match iterator.clone().expect("a proven present value") {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                heap.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                heap.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => heap.clone(),
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                identity,
                prototype,
                extensible,
                callable,
                properties,
                _,
                _,
                kind,
                collection_kind,
                _,
                _,
            ) => {
                let prototype = *prototype;
                self::replace_java_script_ordinary_object(
                    heap.clone(),
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        identity.clone(),
                        Box::new(prototype.clone()),
                        extensible,
                        callable,
                        properties.clone(),
                        collection.clone(),
                        cursor,
                        kind.clone(),
                        collection_kind.clone(),
                        None::<(Option<bool>,)>,
                        None::<(Option<self::JavaScriptSemanticValue>,)>,
                    ),
                )
            }
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => heap.clone(),
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => heap.clone(),
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => heap.clone(),
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => heap.clone(),
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                heap.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => heap.clone(),
            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => heap.clone(),
            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => heap.clone(),
            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                heap.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn advance_java_script_collection_iterator(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
) -> self::JavaScriptCollectionIteration {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_2 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                    {
                        let ml_library_inequality_1 =
                            invalid != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                                Box::new(heap.clone()),
                                invalid.clone(),
                            )
                        } else {
                            match receiver.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Number(_) => {
                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                        self::JavaScriptSemanticValue::Object(identity, _) => {
                            {
                                let iterator = {
                                    let source_ml_source1_array_read_value16619 = match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value16617, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value16617.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_array_read_index16620 = self::find_java_script_ordinary_object(match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value16618, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value16618.clone()
                                            }
                                        }, identity.clone());
                                        { let ml_reference_array207 = source_ml_source1_array_read_value16619; let ml_reference_position207 = source_ml_source1_array_read_index16620; self::ml_array::optional_at(&*ml_reference_array207.borrow(), self::ml_array::number_index(ml_reference_position207)) }
                                    }
                                };
                                if match (iterator.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                } else {
                                    match iterator.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, iterator_collection, iterator_cursor, iterator_kind, collection_kind, _, _) => {
                                            if (self::ml_array::includes(&{ (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116])]))).borrow().clone() }, &collection_kind.clone(), 0f64, false, |a, b| a == b)) {if iterator_collection == (self::ml::Big::from_i128(0)) {
                                                    self::JavaScriptCollectionIteration::CollectionIteration(Box::new(heap.clone()), true, Box::new(self::JavaScriptSemanticValue::Undefined), Box::new(self::JavaScriptSemanticValue::Undefined), false)
                                                } else {
                                                    {
                                                        let collection = {
                                                            let source_ml_source1_array_read_value16714 = match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value16712, _, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value16712.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_array_read_index16715 = self::find_java_script_ordinary_object(match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value16713, _, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value16713.clone()
                                                                    }
                                                                }, iterator_collection.clone());
                                                                { let ml_reference_array209 = source_ml_source1_array_read_value16714; let ml_reference_position209 = source_ml_source1_array_read_index16715; self::ml_array::optional_at(&*ml_reference_array209.borrow(), self::ml_array::number_index(ml_reference_position209)) }
                                                            }
                                                        };
                                                        if match (collection.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (match collection.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        } && match collection.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) {
                                                            self::JavaScriptCollectionIteration::CollectionIterationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112]))
                                                        } else {
                                                            {
                                                                let entries = match collection.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, source_ml_source1_data_property_value16906) => {
                                                                        Some(source_ml_source1_data_property_value16906.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, source_ml_source1_data_property_value16907) => {
                                                                        Some(source_ml_source1_data_property_value16907.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_s16908 = self::source_ml_temporary_advance_java_script_collection_iterator_loop136(heap.clone(), Some(iterator.clone().expect("a proven present value")), iterator_collection.clone(), &(iterator_kind), entries, iterator_cursor);
                                                                    match source_ml_source1_s16908 {
                                                                        self::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Result::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Done => {
                                                                            self::JavaScriptCollectionIteration::CollectionIteration(Box::new(self::move_java_script_collection_iterator_cursor(heap.clone(), Some(iterator.clone().expect("a proven present value")), self::ml::Big::from_i128(0), 0f64)), true, Box::new(self::JavaScriptSemanticValue::Undefined), Box::new(self::JavaScriptSemanticValue::Undefined), false)
                                                                        }
                                                                        self::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Result::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Return(source_ml_temporary_result136) => {
                                                                            let source_ml_temporary_result136 = *source_ml_temporary_result136;
                                                                            source_ml_temporary_result136.clone()
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }} else {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                            }
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
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
pub fn advance_java_script_branded_collection_iterator(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
    collection_kind: &self::ml_string::Text,
) -> self::JavaScriptCollectionIteration {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_3 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_3 {
                self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else if (self::ml_array::includes(
                &{
                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                        self::ml_string::Text::from_units(vec![77, 97, 112]),
                        self::ml_string::Text::from_units(vec![83, 101, 116]),
                    ])))
                    .borrow()
                    .clone()
                },
                collection_kind,
                0f64,
                false,
                |a, b| a == b,
            )) {
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                    {
                        let ml_library_inequality_2 =
                            invalid != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_2 {
                            self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                                Box::new(heap.clone()),
                                invalid.clone(),
                            )
                        } else {
                            match receiver.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Text(_) => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                            }
                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                {
                                    let iterator = {
                                        let source_ml_source1_array_read_value16919 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value16917, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value16917.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_array_read_index16920 = self::find_java_script_ordinary_object(match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value16918, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value16918.clone()
                                                }
                                            }, identity.clone());
                                            { let ml_reference_array211 = source_ml_source1_array_read_value16919; let ml_reference_position211 = source_ml_source1_array_read_index16920; self::ml_array::optional_at(&*ml_reference_array211.borrow(), self::ml_array::number_index(ml_reference_position211)) }
                                        }
                                    };
                                    if match (iterator.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                    } else {
                                        match iterator.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, iterator_collection_kind, _, _) => {
                                                {
let ml_library_inequality_1 = iterator_collection_kind != (*collection_kind) ;
if ml_library_inequality_1 {
                                                    self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                                } else {
                                                    self::advance_java_script_collection_iterator(heap.clone(), receiver.clone())
                                                }
}
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
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
                self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110,
                        101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                        114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114,
                        97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100,
                    ]),
                )
            }
        }
    }
}
