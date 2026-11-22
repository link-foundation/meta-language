// Generated from complete js/src/translation/javascript-collection-iteration.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptCollectionIteration;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryAdvanceJavaScriptCollectionIteratorLoop136Result;
use super::MlTemporaryAllocateJavaScriptCollectionIteratorLoop134Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::move_java_script_collection_iterator_cursor;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_advance_java_script_collection_iterator_loop136;
use super::source_ml_temporary_allocate_java_script_collection_iterator_loop134;
use super::source_ml_temporary_allocate_java_script_collection_iterator_loop135;
use super::test_java_script_ordinary_symbol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_iterator_receiver_error(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
    collection_kind: &self::ml_string::Text,
) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, bool)>> {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_2 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                std::rc::Rc::new(std::cell::RefCell::new(error.clone(), false))
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
                        let ml_library_inequality_1 =
                            invalid != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            std::rc::Rc::new(std::cell::RefCell::new(invalid.clone(), false))
                        } else {
                            match receiver.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_field17436 =
                                            ((self::ml_string::Text::from_units(vec![
                                                105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114,
                                                101, 97, 116, 105, 111, 110, 32, 110, 101, 101,
                                                100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                                97, 108, 32,
                                            ]))
                                            .concat(collection_kind))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115,
                                                108, 111, 116, 32, 114, 101, 99, 101, 105, 118,
                                                101, 114,
                                            ]));
                                        (source_ml_source1_field17436, true)
                                    }))
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_field17438 =
                                            ((self::ml_string::Text::from_units(vec![
                                                105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114,
                                                101, 97, 116, 105, 111, 110, 32, 110, 101, 101,
                                                100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                                97, 108, 32,
                                            ]))
                                            .concat(collection_kind))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115,
                                                108, 111, 116, 32, 114, 101, 99, 101, 105, 118,
                                                101, 114,
                                            ]));
                                        (source_ml_source1_field17438, true)
                                    }))
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_field17440 =
                                            ((self::ml_string::Text::from_units(vec![
                                                105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114,
                                                101, 97, 116, 105, 111, 110, 32, 110, 101, 101,
                                                100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                                97, 108, 32,
                                            ]))
                                            .concat(collection_kind))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115,
                                                108, 111, 116, 32, 114, 101, 99, 101, 105, 118,
                                                101, 114,
                                            ]));
                                        (source_ml_source1_field17440, true)
                                    }))
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_field17442 =
                                            ((self::ml_string::Text::from_units(vec![
                                                105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114,
                                                101, 97, 116, 105, 111, 110, 32, 110, 101, 101,
                                                100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                                97, 108, 32,
                                            ]))
                                            .concat(collection_kind))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115,
                                                108, 111, 116, 32, 114, 101, 99, 101, 105, 118,
                                                101, 114,
                                            ]));
                                        (source_ml_source1_field17442, true)
                                    }))
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_field17444 =
                                            ((self::ml_string::Text::from_units(vec![
                                                105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114,
                                                101, 97, 116, 105, 111, 110, 32, 110, 101, 101,
                                                100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                                97, 108, 32,
                                            ]))
                                            .concat(collection_kind))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115,
                                                108, 111, 116, 32, 114, 101, 99, 101, 105, 118,
                                                101, 114,
                                            ]));
                                        (source_ml_source1_field17444, true)
                                    }))
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_field17446 =
                                            ((self::ml_string::Text::from_units(vec![
                                                105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114,
                                                101, 97, 116, 105, 111, 110, 32, 110, 101, 101,
                                                100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                                97, 108, 32,
                                            ]))
                                            .concat(collection_kind))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115,
                                                108, 111, 116, 32, 114, 101, 99, 101, 105, 118,
                                                101, 114,
                                            ]));
                                        (source_ml_source1_field17446, true)
                                    }))
                                }
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_field17448 =
                                            ((self::ml_string::Text::from_units(vec![
                                                105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114,
                                                101, 97, 116, 105, 111, 110, 32, 110, 101, 101,
                                                100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                                97, 108, 32,
                                            ]))
                                            .concat(collection_kind))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115,
                                                108, 111, 116, 32, 114, 101, 99, 101, 105, 118,
                                                101, 114,
                                            ]));
                                        (source_ml_source1_field17448, true)
                                    }))
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_field17450 =
                                            ((self::ml_string::Text::from_units(vec![
                                                105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114,
                                                101, 97, 116, 105, 111, 110, 32, 110, 101, 101,
                                                100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                                97, 108, 32,
                                            ]))
                                            .concat(collection_kind))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115,
                                                108, 111, 116, 32, 114, 101, 99, 101, 105, 118,
                                                101, 114,
                                            ]));
                                        (source_ml_source1_field17450, true)
                                    }))
                                }
                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                    let collection = {
                                        let source_ml_source1_array_read_value17454 = match heap
                                            .clone()
                                        {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                source_ml_source1_data_property_value17452,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source1_data_property_value17452.clone(),
                                        };
                                        {
                                            let source_ml_source1_array_read_index17455 = self::find_java_script_ordinary_object(match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17453, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value17453.clone()
                                                }
                                            }, identity.clone());
                                            {
                                                let ml_reference_array82 =
                                                    source_ml_source1_array_read_value17454;
                                                let ml_reference_position82 =
                                                    source_ml_source1_array_read_index17455;
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array82.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position82,
                                                    ),
                                                )
                                            }
                                        }
                                    };
                                    if match (collection.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } || !((((*collection_kind)
                                        == (self::ml_string::Text::from_units(vec![77, 97, 112])))
                                        && match collection.clone().expect("a proven present value")
                                        {
                                            self::JavaScriptOrdinaryObject::MapObject(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => true,
                                            _ => false,
                                        })
                                        || (((*collection_kind)
                                            == (self::ml_string::Text::from_units(vec![
                                                83, 101, 116,
                                            ])))
                                            && match collection
                                                .clone()
                                                .expect("a proven present value")
                                            {
                                                self::JavaScriptOrdinaryObject::SetObject(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => true,
                                                _ => false,
                                            }))
                                    {
                                        std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source1_field17646 =
                                                ((self::ml_string::Text::from_units(vec![
                                                    105, 116, 101, 114, 97, 116, 111, 114, 32, 99,
                                                    114, 101, 97, 116, 105, 111, 110, 32, 110, 101,
                                                    101, 100, 115, 32, 105, 116, 115, 32, 97, 99,
                                                    116, 117, 97, 108, 32,
                                                ]))
                                                .concat(collection_kind))
                                                .concat(&self::ml_string::Text::from_units(vec![
                                                    32, 105, 110, 116, 101, 114, 110, 97, 108, 45,
                                                    115, 108, 111, 116, 32, 114, 101, 99, 101, 105,
                                                    118, 101, 114,
                                                ]));
                                            (source_ml_source1_field17646, true)
                                        }))
                                    } else {
                                        std::rc::Rc::new(std::cell::RefCell::new(
                                            self::ml_string::Text::from_units(vec![]),
                                            false,
                                        ))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                std::rc::Rc::new(std::cell::RefCell::new(
                    self::ml_string::Text::from_units(vec![
                        105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114, 101, 97, 116, 105, 111,
                        110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                        97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 111,
                        108, 108, 101, 99, 116, 105, 111, 110, 32, 107, 105, 110, 100,
                    ]),
                    false,
                ))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_java_script_collection_iterator(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    collection_kind: &self::ml_string::Text,
    kind: &self::ml_string::Text,
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
            } else if !(self::ml_array::includes(
                &{
                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                        self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
                        self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]),
                        self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115]),
                    ])))
                    .borrow()
                    .clone()
                },
                kind,
                0f64,
                false,
                |a, b| a == b,
            )) || !(self::ml_array::includes(
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
                self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97,
                        116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97,
                        99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101,
                        100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 107, 105, 110,
                        100,
                    ]),
                )
            } else {
                {
                    let source_ml_temporary_values132 =
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            prototype.clone(),
                            receiver.clone(),
                        ]));
                    {
                        let source_ml_temporary_index132 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s17650 = self::source_ml_temporary_allocate_java_script_collection_iterator_loop134(heap.clone(), source_ml_temporary_values132, source_ml_temporary_index132);
                            match source_ml_source1_s17650 {
                                self::MlTemporaryAllocateJavaScriptCollectionIteratorLoop134Result::MlTemporaryAllocateJavaScriptCollectionIteratorLoop134Done => {
                                    match receiver.clone() {
                                        self::JavaScriptSemanticValue::Undefined => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptSemanticValue::Null => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptSemanticValue::Number(_) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptSemanticValue::Integer(_) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptSemanticValue::Text(_) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                        }
                                        self::JavaScriptSemanticValue::Object(receiver_identity, _) => {
                                            {
                                                let collection = {
                                                    let source_ml_source1_array_read_value17661 = match heap.clone() {
                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17659, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value17659.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_array_read_index17662 = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17660, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value17660.clone()
                                                            }
                                                        }, receiver_identity.clone());
                                                        { let ml_reference_array85 = source_ml_source1_array_read_value17661; let ml_reference_position85 = source_ml_source1_array_read_index17662; self::ml_array::optional_at(&*ml_reference_array85.borrow(), self::ml_array::number_index(ml_reference_position85)) }
                                                    }
                                                };
                                                if match (collection.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !((((*collection_kind) == (self::ml_string::Text::from_units(vec![77, 97, 112]))) && match collection.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                        true
                                                    }
                                                    _ => {
                                                        false
                                                    }
                                                }) || (((*collection_kind) == (self::ml_string::Text::from_units(vec![83, 101, 116]))) && match collection.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                        true
                                                    }
                                                    _ => {
                                                        false
                                                    }
                                                })) {
                                                    self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                } else if match prototype.clone() {
                                                        self::JavaScriptSemanticValue::Null => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    } && match prototype.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    } {
                                                        self::JavaScriptCollectionIteration::CollectionIterationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                    } else if (((identity <= (self::ml::Big::from_i128(0))) || (identity > (self::ml::Big::from_i128(18_446_744_073_709_551_615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17872, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value17872.clone()
                                                            }
                                                        }, identity.clone())) >= 0f64)) || self::test_java_script_ordinary_symbol(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value17873, _, _, _, _) => {
                                                                source_ml_source1_data_property_value17873.clone()
                                                            }
                                                        }, identity.clone()) {
                                                            self::JavaScriptCollectionIteration::CollectionIterationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 102, 114, 101, 115, 104, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                                                        } else {
let ml_product_condition_1 = { let ml_reference_array86 = match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17875, _, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value17875.clone()
                                                                }
                                                            }; ml_reference_array86.borrow().len() as f64 };
if ml_product_condition_1 {
                                                                self::JavaScriptCollectionIteration::CollectionIterationResourceLimit(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 111, 98, 106, 101, 99, 116, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                            } else {
                                                                {
                                                                    let iterator = {
                                                                        let source_ml_source1_constructor_field17880 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                        {
                                                                            let source_ml_source1_constructor_field17894 = match collection.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value17881, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17881.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value17882, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17882.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value17883, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17883.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value17884, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17884.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value17885, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17885.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value17886, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17886.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value17887, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17887.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value17888, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17888.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value17889, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17889.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value17890, _, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17890.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value17891, _, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17891.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value17892, _, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17892.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value17893, _, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17893.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_constructor_field17895 = 0f64;
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(identity.clone(), Box::new(prototype.clone()), true, false, source_ml_source1_constructor_field17880, source_ml_source1_constructor_field17894, source_ml_source1_constructor_field17895, kind.clone(), collection_kind.clone(), None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>)
                                                                            }
                                                                        }
                                                                    };
                                                                    {
                                                                        let at = 0f64;
                                                                        {
                                                                            let at_2 = self::source_ml_temporary_allocate_java_script_collection_iterator_loop135(heap.clone(), identity.clone(), at);
                                                                            {
                                                                                let updated = {
                                                                                    let source_ml_source1_nominal_field17906 = match heap.clone() {
                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17907, _, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value17907.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field17908 = match heap.clone() {
                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value17909, _, _, _, _) => {
                                                                                                source_ml_source1_data_property_value17909.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field17910 = match heap.clone() {
                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source1_data_property_value17911, _, _, _) => {
                                                                                                    source_ml_source1_data_property_value17911.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field17912 = match heap.clone() {
                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value17913, _, _) => {
                                                                                                        source_ml_source1_data_property_value17913.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field17914 = match heap.clone() {
                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source1_data_property_value17915, _) => {
                                                                                                            source_ml_source1_data_property_value17915.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_field17916 = match heap.clone() {
                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value17917) => {
                                                                                                                source_ml_source1_data_property_value17917.clone()
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source1_nominal_override17918 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                let source_ml_source1_concat_value17902 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ (match heap.clone() {
                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17899, _, _, _, _, _) => {
                                                                                                                        source_ml_source1_data_property_value17899.clone()
                                                                                                                    }
                                                                                                                }).borrow().clone() }, 0f64, at_2)));
                                                                                                                {
                                                                                                                    let source_ml_source1_concat_value17903 = std::rc::Rc::new(std::cell::RefCell::new(vec![iterator]));
                                                                                                                    {
                                                                                                                        let source_ml_source1_concat_value17904 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ (match heap.clone() {
                                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17901, _, _, _, _, _) => {
                                                                                                                                source_ml_source1_data_property_value17901.clone()
                                                                                                                            }
                                                                                                                        }).borrow().clone() }, at_2, f64::INFINITY)));
                                                                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), { source_ml_source1_concat_value17902.borrow().clone() }), { source_ml_source1_concat_value17903.borrow().clone() }), { source_ml_source1_concat_value17904.borrow().clone() })
                                                                                                                    }
                                                                                                                }
                                                                                                            }));
                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_nominal_override17918, source_ml_source1_nominal_field17908, source_ml_source1_nominal_field17910, source_ml_source1_nominal_field17912, source_ml_source1_nominal_field17914, source_ml_source1_nominal_field17916)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let invalid_prototype = self::read_java_script_ordinary_heap_error(updated.clone());
                                                                                    {
let ml_library_inequality_1 = invalid_prototype != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                                                        self::JavaScriptCollectionIteration::CollectionIterationInvalid(Box::new(heap.clone()), invalid_prototype.clone())
                                                                                    } else {
                                                                                        self::JavaScriptCollectionIteration::CollectionIteratorAllocated(Box::new(updated.clone()), Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), false)))
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
                                self::MlTemporaryAllocateJavaScriptCollectionIteratorLoop134Result::MlTemporaryAllocateJavaScriptCollectionIteratorLoop134Return(source_ml_temporary_result135) => {
                                    let source_ml_temporary_result135 = *source_ml_temporary_result135;
                                    source_ml_temporary_result135.clone()
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
                                    let source_ml_source1_array_read_value17929 = match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17927, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value17927.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_array_read_index17930 = self::find_java_script_ordinary_object(match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17928, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value17928.clone()
                                            }
                                        }, identity.clone());
                                        { let ml_reference_array92 = source_ml_source1_array_read_value17929; let ml_reference_position92 = source_ml_source1_array_read_index17930; self::ml_array::optional_at(&*ml_reference_array92.borrow(), self::ml_array::number_index(ml_reference_position92)) }
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
                                                            let source_ml_source1_array_read_value18024 = match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value18022, _, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value18022.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_array_read_index18025 = self::find_java_script_ordinary_object(match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value18023, _, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value18023.clone()
                                                                    }
                                                                }, iterator_collection.clone());
                                                                { let ml_reference_array94 = source_ml_source1_array_read_value18024; let ml_reference_position94 = source_ml_source1_array_read_index18025; self::ml_array::optional_at(&*ml_reference_array94.borrow(), self::ml_array::number_index(ml_reference_position94)) }
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
                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, source_ml_source1_data_property_value18216) => {
                                                                        Some(source_ml_source1_data_property_value18216.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, source_ml_source1_data_property_value18217) => {
                                                                        Some(source_ml_source1_data_property_value18217.clone())
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
                                                                    let source_ml_source1_s18218 = self::source_ml_temporary_advance_java_script_collection_iterator_loop136(heap.clone(), Some(iterator.clone().expect("a proven present value")), iterator_collection.clone(), &(iterator_kind), entries, iterator_cursor);
                                                                    match source_ml_source1_s18218 {
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
