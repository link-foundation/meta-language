// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptCollectionIteration;
use super::JavaScriptCollectionIteratorResultCompletion;
use super::JavaScriptCollectionIteratorResultState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryAdvanceJavaScriptCollectionIteratorLoop1070Result;
use super::MlSource1TemporaryAllocateJavaScriptCollectionIteratorLoop1068Result;
use super::MlSource1TemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop1072Result;
use super::MlSource1TemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop1074Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::move_java_script_collection_iterator_cursor;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_advance_java_script_collection_iterator_loop1070;
use super::source_ml_source1_temporary_allocate_java_script_collection_iterator_loop1068;
use super::source_ml_source1_temporary_allocate_java_script_collection_iterator_loop1069;
use super::source_ml_source1_temporary_read_java_script_collection_iterator_result_prototypes_error_loop1072;
use super::source_ml_source1_temporary_read_java_script_collection_iterator_result_state_error_loop1074;
use super::source_ml_source1_temporary_step_java_script_collection_iterator_result_loop1076;
use super::test_java_script_ordinary_symbol;

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
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
        ) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                if (!(self::ml_array::includes(
                    &{
                        let ml_reference_value1194 =
                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
                                self::ml_string::Text::from_units(vec![
                                    118, 97, 108, 117, 101, 115,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    101, 110, 116, 114, 105, 101, 115,
                                ]),
                            ])))
                            .borrow()
                            .clone();
                        ml_reference_value1194
                    },
                    kind,
                    0f64,
                    false,
                    |a, b| a == b,
                )) || !(self::ml_array::includes(
                    &{
                        let ml_reference_value1195 =
                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                self::ml_string::Text::from_units(vec![77, 97, 112]),
                                self::ml_string::Text::from_units(vec![83, 101, 116]),
                            ])))
                            .borrow()
                            .clone();
                        ml_reference_value1195
                    },
                    collection_kind,
                    0f64,
                    false,
                    |a, b| a == b,
                ))) {
                    self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                        Box::new(heap.clone()),
                        self::ml_string::Text::from_units(vec![
                            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114,
                            97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115,
                            32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101,
                            114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 107,
                            105, 110, 100,
                        ]),
                    )
                } else {
                    {
                        let source_ml_source1_temporary_values1066 =
                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                prototype.clone(),
                                receiver.clone(),
                            ]));
                        {
                            let source_ml_source1_temporary_index1066 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s23516 = self::source_ml_source1_temporary_allocate_java_script_collection_iterator_loop1068(heap.clone(), source_ml_source1_temporary_values1066.clone(), source_ml_source1_temporary_index1066.clone());
                                match source_ml_source2_s23516.clone() {
                                        self::MlSource1TemporaryAllocateJavaScriptCollectionIteratorLoop1068Result::MlSource1TemporaryAllocateJavaScriptCollectionIteratorLoop1068Done => {
                                            match receiver.clone() {
                                                self::JavaScriptSemanticValue::Object(receiver_identity, _) => {
                                                    {
                                                        let collection = {
                                                            let source_ml_source2_array_read_value23520 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_array_read_index23521 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), receiver_identity.clone());
                                                                { let ml_reference_array1196 = source_ml_source2_array_read_value23520.clone(); let ml_reference_position1196 = source_ml_source2_array_read_index23521; let ml_reference_projection1196 = self::ml_array::optional_at(&*ml_reference_array1196.borrow(), self::ml_array::number_index(ml_reference_position1196)); ml_reference_projection1196 }
                                                            }
                                                        };
                                                        if (match (collection.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(((((*collection_kind) == (self::ml_string::Text::from_units(vec![77, 97, 112]))) && match collection.clone().expect("a proven present value") {
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
                                                        })))) {
                                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                        } else {
                                                            if (match prototype.clone() {
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
                                                            }) {
                                                                self::JavaScriptCollectionIteration::CollectionIterationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                            } else {
                                                                if ((((identity <= (self::ml::Big::from_i128(0))) || (identity > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone())) >= (0f64))) || self::test_java_script_ordinary_symbol(shared_constructor_field2.clone(), identity.clone())) {
                                                                    self::JavaScriptCollectionIteration::CollectionIterationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 102, 114, 101, 115, 104, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                                                                } else {
                                                                    if (({ let ml_reference_array1197 = shared_constructor_field1.clone(); let ml_reference_projection1197 = ml_reference_array1197.borrow().len() as f64; ml_reference_projection1197 }) >= (4096f64)) {
                                                                        self::JavaScriptCollectionIteration::CollectionIterationResourceLimit(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 111, 98, 106, 101, 99, 116, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        {
                                                                            let iterator = {
                                                                                let source_ml_source2_constructor_field23545 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                                {
                                                                                    let source_ml_source2_constructor_field23548 = (((match collection.clone().expect("a proven present value") {
                                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value23546, _, _, _, _, _) => {
                                                                                            Some((source_ml_source2_data_property_value23546.clone(),))
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value23547, _, _, _, _, _) => {
                                                                                            Some((source_ml_source2_data_property_value23547.clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(self::ml::Big,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone();
                                                                                    {
                                                                                        let source_ml_source2_constructor_field23549 = 0f64;
                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(identity.clone(), Box::new(prototype.clone()), true, false, source_ml_source2_constructor_field23545.clone(), source_ml_source2_constructor_field23548.clone(), source_ml_source2_constructor_field23549, kind.clone(), collection_kind.clone(), None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>)
                                                                                    }
                                                                                }
                                                                            };
                                                                            {
                                                                                let at = 0f64;
                                                                                {
                                                                                    let at_2 = self::source_ml_source1_temporary_allocate_java_script_collection_iterator_loop1069(heap.clone(), identity.clone(), at);
                                                                                    {
                                                                                        let updated = {
                                                                                            let source_ml_source2_nominal_field23560 = shared_constructor_field1.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field23562 = shared_constructor_field2.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field23564 = shared_constructor_field3.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field23566 = shared_constructor_field4.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field23568 = shared_constructor_field5.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field23570 = shared_constructor_field6.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override23572 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                        let source_ml_source2_concat_value23556 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1198 = shared_constructor_field1.borrow().clone(); ml_reference_value1198 }, 0f64, at_2)));
                                                                                                                        {
                                                                                                                            let source_ml_source2_concat_value23557 = std::rc::Rc::new(std::cell::RefCell::new(vec![iterator.clone()]));
                                                                                                                            {
                                                                                                                                let source_ml_source2_concat_value23558 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1199 = shared_constructor_field1.borrow().clone(); ml_reference_value1199 }, at_2, f64::INFINITY)));
                                                                                                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), { let ml_reference_value1200 = source_ml_source2_concat_value23556.borrow().clone(); ml_reference_value1200 }), { let ml_reference_value1201 = source_ml_source2_concat_value23557.borrow().clone(); ml_reference_value1201 }), { let ml_reference_value1202 = source_ml_source2_concat_value23558.borrow().clone(); ml_reference_value1202 })
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }));
                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_nominal_override23572.clone(), source_ml_source2_nominal_field23562.clone(), source_ml_source2_nominal_field23564.clone(), source_ml_source2_nominal_field23566.clone(), source_ml_source2_nominal_field23568.clone(), source_ml_source2_nominal_field23570.clone())
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let invalid_prototype = self::read_java_script_ordinary_heap_error(updated.clone());
                                                                                            if (invalid_prototype != (self::ml_string::Text::from_units(vec![]))) {
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
                                                _ => {
                                                    self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryAllocateJavaScriptCollectionIteratorLoop1068Result::MlSource1TemporaryAllocateJavaScriptCollectionIteratorLoop1068Return(source_ml_source1_temporary_result1069) => {
                                            let source_ml_source1_temporary_result1069 = *source_ml_source1_temporary_result1069;
                                            source_ml_source1_temporary_result1069.clone()
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
pub fn advance_java_script_collection_iterator(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
) -> self::JavaScriptCollectionIteration {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                            Box::new(heap.clone()),
                            invalid.clone(),
                        )
                    } else {
                        match receiver.clone() {
                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                let iterator = {
                                    let source_ml_source2_array_read_value23576 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_array_read_index23577 =
                                            self::find_java_script_ordinary_object(
                                                shared_constructor_field1.clone(),
                                                identity.clone(),
                                            );
                                        {
                                            let ml_reference_array1203 =
                                                source_ml_source2_array_read_value23576.clone();
                                            let ml_reference_position1203 =
                                                source_ml_source2_array_read_index23577;
                                            let ml_reference_projection1203 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array1203.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position1203,
                                                    ),
                                                );
                                            ml_reference_projection1203
                                        }
                                    }
                                };
                                if match (iterator.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                } else {
                                    match iterator.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, iterator_collection, iterator_cursor, iterator_kind, collection_kind, _, _) => {
                                                    if !(self::ml_array::includes(&{ let ml_reference_value1204 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116])]))).borrow().clone(); ml_reference_value1204 }, &collection_kind.clone(), 0f64, false, |a, b| a == b)) {
                                                        self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                    } else {
                                                        if (iterator_collection == (self::ml::Big::from_i128(0))) {
                                                            self::JavaScriptCollectionIteration::CollectionIteration(Box::new(heap.clone()), true, Box::new(self::JavaScriptSemanticValue::Undefined), Box::new(self::JavaScriptSemanticValue::Undefined), false)
                                                        } else {
                                                            {
                                                                let collection = {
                                                                    let source_ml_source2_array_read_value23587 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index23588 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), iterator_collection.clone());
                                                                        { let ml_reference_array1205 = source_ml_source2_array_read_value23587.clone(); let ml_reference_position1205 = source_ml_source2_array_read_index23588; let ml_reference_projection1205 = self::ml_array::optional_at(&*ml_reference_array1205.borrow(), self::ml_array::number_index(ml_reference_position1205)); ml_reference_projection1205 }
                                                                    }
                                                                };
                                                                if (match (collection.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (match collection.clone().expect("a proven present value") {
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
                                                                })) {
                                                                    self::JavaScriptCollectionIteration::CollectionIterationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112]))
                                                                } else {
                                                                    {
                                                                        let entries = (((match collection.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, source_ml_source2_data_property_value23601) => {
                                                                                Some((source_ml_source2_data_property_value23601.clone(),))
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, source_ml_source2_data_property_value23602) => {
                                                                                Some((source_ml_source2_data_property_value23602.clone(),))
                                                                            }
                                                                            _ => {
                                                                                None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0).clone();
                                                                        {
                                                                            let source_ml_source2_s23603 = self::source_ml_source1_temporary_advance_java_script_collection_iterator_loop1070(heap.clone(), Some(iterator.clone().expect("a proven present value")), iterator_collection.clone(), &(iterator_kind), Some(entries.clone()), iterator_cursor);
                                                                            match source_ml_source2_s23603.clone() {
                                                                                self::MlSource1TemporaryAdvanceJavaScriptCollectionIteratorLoop1070Result::MlSource1TemporaryAdvanceJavaScriptCollectionIteratorLoop1070Done => {
                                                                                    self::JavaScriptCollectionIteration::CollectionIteration(Box::new(self::move_java_script_collection_iterator_cursor(heap.clone(), Some(iterator.clone().expect("a proven present value")), self::ml::Big::from_i128(0), 0f64)), true, Box::new(self::JavaScriptSemanticValue::Undefined), Box::new(self::JavaScriptSemanticValue::Undefined), false)
                                                                                }
                                                                                self::MlSource1TemporaryAdvanceJavaScriptCollectionIteratorLoop1070Result::MlSource1TemporaryAdvanceJavaScriptCollectionIteratorLoop1070Return(source_ml_source1_temporary_result1070) => {
                                                                                    let source_ml_source1_temporary_result1070 = *source_ml_source1_temporary_result1070;
                                                                                    source_ml_source1_temporary_result1070.clone()
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
                                                    self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                }
                                            }
                                }
                            }
                            _ => self::JavaScriptCollectionIteration::CollectionIterationTypeError(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116,
                                    101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110,
                                    101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                    97, 108, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32,
                                    105, 110, 116, 101, 114, 110, 97, 108, 45, 115, 108, 111, 116,
                                    32, 114, 101, 99, 101, 105, 118, 101, 114,
                                ]),
                            ),
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
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                if !(self::ml_array::includes(
                    &{
                        let ml_reference_value1206 =
                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                self::ml_string::Text::from_units(vec![77, 97, 112]),
                                self::ml_string::Text::from_units(vec![83, 101, 116]),
                            ])))
                            .borrow()
                            .clone();
                        ml_reference_value1206
                    },
                    collection_kind,
                    0f64,
                    false,
                    |a, b| a == b,
                )) {
                    self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                        Box::new(heap.clone()),
                        self::ml_string::Text::from_units(vec![
                            105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 110,
                            101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                            32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112,
                            101, 114, 97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100,
                        ]),
                    )
                } else {
                    {
                        let invalid = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            receiver.clone(),
                        );
                        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                            self::JavaScriptCollectionIteration::CollectionIterationInvalid(
                                Box::new(heap.clone()),
                                invalid.clone(),
                            )
                        } else {
                            match receiver.clone() {
                                    self::JavaScriptSemanticValue::Object(identity, _) => {
                                        {
                                            let iterator = {
                                                let source_ml_source2_array_read_value23607 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_array_read_index23608 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                    { let ml_reference_array1207 = source_ml_source2_array_read_value23607.clone(); let ml_reference_position1207 = source_ml_source2_array_read_index23608; let ml_reference_projection1207 = self::ml_array::optional_at(&*ml_reference_array1207.borrow(), self::ml_array::number_index(ml_reference_position1207)); ml_reference_projection1207 }
                                                }
                                            };
                                            if match (iterator.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                            } else {
                                                match iterator.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, iterator_collection_kind, _, _) => {
                                                        if (iterator_collection_kind != (*collection_kind)) {
                                                            self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                                        } else {
                                                            self::advance_java_script_collection_iterator(heap.clone(), receiver.clone())
                                                        }
                                                    }
                                                    _ => {
                                                        self::JavaScriptCollectionIteration::CollectionIterationTypeError(Box::new(heap.clone()), ((self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32])).concat(collection_kind)).concat(&self::ml_string::Text::from_units(vec![32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 98, 114, 97, 110, 100])))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_iterator_result_prototypes_error(
    heap: self::JavaScriptOrdinaryHeap,
    array_prototype: self::JavaScriptSemanticValue,
    object_prototype: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                error.clone()
            } else {
                {
                    let source_ml_source1_temporary_values1071 =
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            array_prototype.clone(),
                            object_prototype.clone(),
                        ]));
                    {
                        let source_ml_source1_temporary_index1071 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s23619 = self::source_ml_source1_temporary_read_java_script_collection_iterator_result_prototypes_error_loop1072(heap.clone(), source_ml_source1_temporary_values1071.clone(), source_ml_source1_temporary_index1071.clone());
                            match source_ml_source2_s23619.clone() {
                                    self::MlSource1TemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop1072Result::MlSource1TemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop1072Done => {
                                        match array_prototype.clone() {
                                            self::JavaScriptSemanticValue::Object(array_prototype_identity, array_prototype_callable) => {
                                                match object_prototype.clone() {
                                                    self::JavaScriptSemanticValue::Object(identity, callable) => {
                                                        if (array_prototype_callable || callable) {
                                                            self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115])
                                                        } else {
                                                            {
                                                                let array = {
                                                                    let source_ml_source2_array_read_value23622 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index23623 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), array_prototype_identity.clone());
                                                                        { let ml_reference_array1208 = source_ml_source2_array_read_value23622.clone(); let ml_reference_position1208 = source_ml_source2_array_read_index23623; let ml_reference_projection1208 = self::ml_array::optional_at(&*ml_reference_array1208.borrow(), self::ml_array::number_index(ml_reference_position1208)); ml_reference_projection1208 }
                                                                    }
                                                                };
                                                                {
                                                                    let ordinary = {
                                                                        let source_ml_source2_array_read_value23626 = shared_constructor_field1.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index23627 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                                            { let ml_reference_array1209 = source_ml_source2_array_read_value23626.clone(); let ml_reference_position1209 = source_ml_source2_array_read_index23627; let ml_reference_projection1209 = self::ml_array::optional_at(&*ml_reference_array1209.borrow(), self::ml_array::number_index(ml_reference_position1209)); ml_reference_projection1209 }
                                                                        }
                                                                    };
                                                                    if (((match (array.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match array.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }) || match (ordinary.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match ordinary.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }) {
                                                                        self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 97, 110, 100, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 32, 107, 105, 110, 100, 115])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![])
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115])
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115])
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop1072Result::MlSource1TemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop1072Return(source_ml_source1_temporary_result1072) => {
                                        source_ml_source1_temporary_result1072.clone()
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
pub fn read_java_script_collection_iterator_result_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptCollectionIteratorResultState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let error = self::read_java_script_collection_iterator_result_prototypes_error(
                        heap.clone(),
                        shared_constructor_field5.clone(),
                        shared_constructor_field6.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        error.clone()
                    } else {
                        if ((((match state.clone() {
                                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, _, _, _) => {
                                    false
                                }
                            } || !(self::ml_array::includes(&{ let ml_reference_value1210 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121]), self::ml_string::Text::from_units(vec![114, 101, 115, 117, 108, 116])]))).borrow().clone(); ml_reference_value1210 }, &shared_constructor_field2.clone(), 0f64, false, |a, b| a == b))) || (({
                                let source_ml_source2_type_query_operand23656 = shared_constructor_field9;
                                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                            }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) || ((shared_constructor_field2 == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121]))) && shared_constructor_field9)) || !(self::ml_array::includes(&{ let ml_reference_value1211 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])]))).borrow().clone(); ml_reference_value1211 }, &shared_constructor_field4.clone(), 0f64, false, |a, b| a == b))) {
                                self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 112, 104, 97, 115, 101, 44, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110])
                            } else {
                                {
                                    let source_ml_source1_temporary_values1073 = std::rc::Rc::new(std::cell::RefCell::new(vec![shared_constructor_field3.clone(), shared_constructor_field7.clone(), shared_constructor_field8.clone()]));
                                    {
                                        let source_ml_source1_temporary_index1073 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s23664 = self::source_ml_source1_temporary_read_java_script_collection_iterator_result_state_error_loop1074(heap.clone(), source_ml_source1_temporary_values1073.clone(), source_ml_source1_temporary_index1073.clone());
                                            match source_ml_source2_s23664.clone() {
                                                self::MlSource1TemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop1074Result::MlSource1TemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop1074Done => {
                                                    {
                                                        let receiver = shared_constructor_field3.clone();
                                                        match receiver.clone() {
                                                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                                                {
                                                                    let iterator = {
                                                                        let source_ml_source2_array_read_value23669 = shared_constructor_field1.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index23670 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                                            { let ml_reference_array1212 = source_ml_source2_array_read_value23669.clone(); let ml_reference_position1212 = source_ml_source2_array_read_index23670; let ml_reference_projection1212 = self::ml_array::optional_at(&*ml_reference_array1212.borrow(), self::ml_array::number_index(ml_reference_position1212)); ml_reference_projection1212 }
                                                                        }
                                                                    };
                                                                    if match (iterator.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                        self::ml_string::Text::from_units(vec![97, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100])
                                                                    } else {
                                                                        match iterator.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, collection_kind, _, _) => {
                                                                                if (collection_kind != shared_constructor_field4) {
                                                                                    self::ml_string::Text::from_units(vec![97, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100])
                                                                                } else {
                                                                                    if ((shared_constructor_field4 == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) && (shared_constructor_field2 == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121])))) {
                                                                                        self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 115, 32, 121, 105, 101, 108, 100, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101, 115, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 101, 110, 116, 114, 121, 32, 112, 97, 105, 114, 115])
                                                                                    } else {
                                                                                        if (shared_constructor_field9 && ({
                                                                                            let source_ml_source2_s23689 = shared_constructor_field7.clone();
                                                                                            match source_ml_source2_s23689.clone() {
                                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                                    false
                                                                                                }
                                                                                                _ => {
                                                                                                    true
                                                                                                }
                                                                                            }
                                                                                        } || {
                                                                                            let source_ml_source2_s23692 = shared_constructor_field8.clone();
                                                                                            match source_ml_source2_s23692.clone() {
                                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                                    false
                                                                                                }
                                                                                                _ => {
                                                                                                    true
                                                                                                }
                                                                                            }
                                                                                        })) {
                                                                                            self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 121, 105, 101, 108, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 118, 97, 108, 117, 101])
                                                                                        } else {
                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            _ => {
                                                                                self::ml_string::Text::from_units(vec![97, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100])
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                self::ml_string::Text::from_units(vec![97, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100])
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop1074Result::MlSource1TemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop1074Return(source_ml_source1_temporary_result1074) => {
                                                    source_ml_source1_temporary_result1074.clone()
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
pub fn begin_java_script_collection_iterator_result(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
    collection_kind: &self::ml_string::Text,
    array_prototype: self::JavaScriptSemanticValue,
    object_prototype: self::JavaScriptSemanticValue,
) -> self::JavaScriptCollectionIteratorResultCompletion {
    {
        let error = self::read_java_script_collection_iterator_result_prototypes_error(
            heap.clone(),
            array_prototype.clone(),
            object_prototype.clone(),
        );
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            {
                let next = self::advance_java_script_branded_collection_iterator(
                    heap.clone(),
                    receiver.clone(),
                    collection_kind,
                );
                match next.clone() {
                    self::JavaScriptCollectionIteration::CollectionIterationTypeError(_, reason) => {
                        self::JavaScriptCollectionIteratorResultCompletion::IteratorResultTypeError(Box::new(heap.clone()), reason.clone())
                    }
                    _ => {
                        match next.clone() {
                            self::JavaScriptCollectionIteration::CollectionIteration(heap_2, done, key, value, entry) => {
                                let heap_2 = *heap_2;
                                let key = *key;
                                let value = *value;
                                {
                                    let state = self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(if entry {
                                        self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121])
                                    } else {
                                        self::ml_string::Text::from_units(vec![114, 101, 115, 117, 108, 116])
                                    }, Box::new(receiver.clone()), collection_kind.clone(), Box::new(array_prototype.clone()), Box::new(object_prototype.clone()), Box::new(key.clone()), Box::new(value.clone()), done);
                                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultContinue(Box::new(heap_2.clone()), Box::new(state.clone()))
                                }
                            }
                            _ => {
                                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 99, 117, 114, 115, 111, 114, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110]))
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
pub fn begin_java_script_array_iterator_result(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
    done: bool,
    key: self::JavaScriptSemanticValue,
    value: self::JavaScriptSemanticValue,
    entry: bool,
    array_prototype: self::JavaScriptSemanticValue,
    object_prototype: self::JavaScriptSemanticValue,
) -> self::JavaScriptCollectionIteratorResultCompletion {
    {
        let state = self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(
            if entry {
                self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121])
            } else {
                self::ml_string::Text::from_units(vec![114, 101, 115, 117, 108, 116])
            },
            Box::new(receiver.clone()),
            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]),
            Box::new(array_prototype.clone()),
            Box::new(object_prototype.clone()),
            Box::new(key.clone()),
            Box::new(value.clone()),
            done,
        );
        {
            let error = self::read_java_script_collection_iterator_result_state_error(
                heap.clone(),
                state.clone(),
            );
            if (error == (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultContinue(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                )
            } else {
                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_collection_iterator_result(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptCollectionIteratorResultState,
) -> self::JavaScriptCollectionIteratorResultCompletion {
    match state.clone() {
        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(
            shared_constructor_field2,
            _,
            _,
            shared_constructor_field3,
            shared_constructor_field4,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let error = self::read_java_script_collection_iterator_result_state_error(
                        heap.clone(),
                        state.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(
                            Box::new(heap.clone()),
                            error.clone(),
                        )
                    } else {
                        {
                            let properties = 0f64;
                            {
                                let source_ml_source1_temporary_values1075 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source1_temporary_index1075 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let properties_2 = self::source_ml_source1_temporary_step_java_script_collection_iterator_result_loop1076(properties, source_ml_source1_temporary_values1075.clone(), source_ml_source1_temporary_index1075.clone());
                                        if ((({
                                            let ml_reference_array1213 =
                                                shared_constructor_field1.clone();
                                            let ml_reference_projection1213 =
                                                ml_reference_array1213.borrow().len() as f64;
                                            ml_reference_projection1213
                                        }) >= (4096f64))
                                            || ((properties_2
                                                + if (shared_constructor_field2
                                                    == (self::ml_string::Text::from_units(vec![
                                                        101, 110, 116, 114, 121,
                                                    ])))
                                                {
                                                    3f64
                                                } else {
                                                    2f64
                                                })
                                                > (16384f64)))
                                        {
                                            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 111, 98, 106, 101, 99, 116, 47, 112, 114, 111, 112, 101, 114, 116, 121, 32, 98, 117, 100, 103, 101, 116]))
                                        } else {
                                            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultAllocate(Box::new(heap.clone()), Box::new(state.clone()), Box::new(if (shared_constructor_field2 == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121]))) {
                                                    shared_constructor_field3.clone()
                                                } else {
                                                    shared_constructor_field4.clone()
                                                }), if (shared_constructor_field2 == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121]))) {
                                                    self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121])
                                                })
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
