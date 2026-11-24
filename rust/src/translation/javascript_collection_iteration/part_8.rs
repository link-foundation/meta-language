// Generated from complete js/src/translation/javascript-collection-iteration.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::JavaScriptWeakCollectionEntry;
use super::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54;
use super::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55;
use super::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58;
use super::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::source_ml_temporary_read_java_script_weak_collection_object_error_loop60;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_iterator_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
) -> self::ml_string::Text {
    {
        let source_ml_source1_s4446 = match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
            }
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, callable, _, object_collection, cursor, kind, collection_kind, executing, iterated_text) => {
                if (((((((!(self::ml_array::includes(&{ (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])]))).borrow().clone() }, &collection_kind.clone(), 0f64, false, |a, b| a == b)) || callable) || !(self::ml_math::is_safe_integer(cursor))) || (cursor < 0f64)) || {
                    let source_ml_source1_same_value_operand4536 = -0f64;
                    (((cursor == source_ml_source1_same_value_operand4536) && ((cursor != 0f64) || ((1f64 / cursor) == (1f64 / source_ml_source1_same_value_operand4536)))) || ((cursor != cursor) && (source_ml_source1_same_value_operand4536 != source_ml_source1_same_value_operand4536)))
                }) || (cursor > (if collection_kind == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121])) {
                    9_007_199_254_740_991f64
                } else if collection_kind == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])) {
                        16_777_216f64
                    } else {
                        65_536f64
                    }))) || !(self::ml_array::includes(&{ (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![107, 101, 121, 115]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])]))).borrow().clone() }, &kind.clone(), 0f64, false, |a, b| a == b))) || ((collection_kind == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) && ((match ((executing.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))))) || ((collection_kind != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) && {
                    let source_ml_source1_comparison_operand4537 = ((executing.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                    {
                        let source_ml_source1_comparison_operand4538 = None::<()>;
                        !(match (source_ml_source1_comparison_operand4537, source_ml_source1_comparison_operand4538) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                    }
                }) {
                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 117, 114, 115, 111, 114, 32, 97, 110, 100, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 107, 105, 110, 100]))
                } else if collection_kind == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])) {
                        {
                            let text = ((iterated_text.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                            if ((object_collection != (self::ml::Big::from_i128(0))) || (kind != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])))) || match (text.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 108, 121, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 99, 117, 114, 115, 111, 114]))
                            } else {
                                {
                                    let error = self::read_java_script_ordinary_value_error(heap.clone(), text.clone().expect("a proven present value"));
                                    {
let ml_library_inequality_3 = error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_3 {
                                        self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(error.clone())
                                    } else {
                                        {
                                            let length = self::read_java_script_semantic_text_length(text.clone().expect("a proven present value"));
                                            {
                                                let source_ml_source1_s4548 = match text.clone().expect("a proven present value") {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        {
let ml_library_inequality_2 = cursor != 0f64 ;
if ml_library_inequality_2 {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 108, 101, 97, 115, 101, 115, 32, 105, 116, 115, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 117, 114, 115, 111, 114]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next
                                                        }
}
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Number(_) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next
                                                        }
                                                    }
                                                };
                                                match source_ml_source1_s4548 {
                                                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next => {
                                                        {
                                                            let source_ml_temporary_read_java_script_collection_iterator_object_error_written56 = { let ml_field_owner53 = budget.clone(); let ml_field_previous53 = ml_field_owner53.borrow().3; let ml_field_value53 = self::ml_math::max(0f64, length); ml_field_owner53.borrow_mut().3 = ml_field_previous53 + ml_field_value53; () };
                                                            if (({ budget.borrow().clone() }).3) > 67_108_864f64 {
                                                                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 101, 120, 116, 115, 32, 97, 110, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 118, 97, 108, 117, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                            } else {
                                                                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(source_ml_temporary_result56) => {
                                                        self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(source_ml_temporary_result56.clone())
                                                    }
                                                }
                                            }
                                        }
                                    }
}
                                }
                            }
                        }
                    } else if (match (((iterated_text.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {{
let ml_library_inequality_1 = object_collection != (self::ml::Big::from_i128(0)) ;
if ml_library_inequality_1 {
                                {
                                    let collection = {
                                        let source_ml_source1_array_read_value4560 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value4558, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value4558.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_array_read_index4561 = self::find_java_script_ordinary_object(match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value4559, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value4559.clone()
                                                }
                                            }, object_collection.clone());
                                            { let ml_reference_array55 = source_ml_source1_array_read_value4560; let ml_reference_position55 = source_ml_source1_array_read_index4561; self::ml_array::optional_at(&*ml_reference_array55.borrow(), self::ml_array::number_index(ml_reference_position55)) }
                                        }
                                    };
                                    if match (collection.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((collection_kind != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) && (!(((collection_kind == (self::ml_string::Text::from_units(vec![77, 97, 112]))) && match collection.clone().expect("a proven present value") {
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
                                    })) || (cursor > ({
                                        let source_ml_source1_nullish_field_receiver4758 = (match collection.clone().expect("a proven present value") {
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
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, source_ml_source1_data_property_value4756) => {
                                                Some(source_ml_source1_data_property_value4756.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, source_ml_source1_data_property_value4757) => {
                                                Some(source_ml_source1_data_property_value4757.clone())
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
                                        },);
                                        if match (source_ml_source1_nullish_field_receiver4758.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            {
                                                let source_ml_source1_nullish_field_present4759 = ((source_ml_source1_nullish_field_receiver4758.clone()).0).clone().expect("a proven present value");
                                                { let ml_reference_array56 = source_ml_source1_nullish_field_present4759; ml_reference_array56.borrow().len() as f64 }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                        }
                                    })))) {
                                        self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 97, 110, 100, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                    } else {
                                        self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
                                    }
                                }
                            } else {
                                if (cursor != 0f64) || {
                                    let source_ml_source1_comparison_operand4760 = ((executing.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                    match (source_ml_source1_comparison_operand4760, true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                } {
                                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 108, 101, 97, 115, 101, 115, 32, 105, 116, 115, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 44, 32, 99, 117, 114, 115, 111, 114, 32, 97, 110, 100, 32, 101, 120, 101, 99, 117, 116, 105, 110, 103, 32, 115, 116, 97, 116, 101]))
                                } else {
                                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
                                }
                            }
}} else {
                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text::from_units(vec![110, 111, 110, 45, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 102, 97, 109, 105, 108, 105, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 83, 116, 114, 105, 110, 103, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                        }
            }
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
            }
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
            }
            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next
            }
        };
        match source_ml_source1_s4446 {
            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(source_ml_temporary_result54) => {
                source_ml_temporary_result54.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_weak_collection_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
) -> self::ml_string::Text {
    {
        let source_ml_source1_s4972 = if match object.clone() {
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => true,
            _ => false,
        } || match object.clone() {
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => true,
            _ => false,
        } {
            {
                let entries = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                        None::<
                            std::rc::Rc<
                                std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
                            >,
                        >
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                        None::<
                            std::rc::Rc<
                                std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
                            >,
                        >
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                        None::<
                            std::rc::Rc<
                                std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
                            >,
                        >
                    }
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
                    ) => {
                        None::<
                            std::rc::Rc<
                                std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
                            >,
                        >
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                        None::<
                            std::rc::Rc<
                                std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
                            >,
                        >
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                        None::<
                            std::rc::Rc<
                                std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
                            >,
                        >
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5163,
                    ) => Some(source_ml_source1_data_property_value5163.clone()),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5164,
                    ) => Some(source_ml_source1_data_property_value5164.clone()),
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                        None::<
                            std::rc::Rc<
                                std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
                            >,
                        >
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                        None::<
                            std::rc::Rc<
                                std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
                            >,
                        >
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                        None::<
                            std::rc::Rc<
                                std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
                            >,
                        >
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                        None::<
                            std::rc::Rc<
                                std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
                            >,
                        >
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                        None::<
                            std::rc::Rc<
                                std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
                            >,
                        >
                    }
                };
                if (((match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5165,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5165,
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5166,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5166,
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5167,
                        _,
                    ) => source_ml_source1_data_property_value5167,
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5168,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5168,
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5169,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5169,
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5170,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5170,
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5171,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5171,
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5172,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5172,
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5173,
                        _,
                    ) => source_ml_source1_data_property_value5173,
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5174,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5174,
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5175,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5175,
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5176,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5176,
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5177,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5177,
                } || match (entries.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) || !({
                    let source_ml_source1_array_argument5178 =
                        entries.clone().expect("a proven present value");
                    true
                })) || (({
                    let ml_reference_array57 = entries.clone().expect("a proven present value");
                    ml_reference_array57.borrow().len() as f64
                }) > 65_536f64))
                    || !(match (
                        match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
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
                                None::<
                                    std::rc::Rc<
                                        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
                                    >,
                                >
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                None::<
                                    std::rc::Rc<
                                        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
                                    >,
                                >
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                None::<
                                    std::rc::Rc<
                                        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
                                    >,
                                >
                            }
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
                            ) => {
                                None::<
                                    std::rc::Rc<
                                        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
                                    >,
                                >
                            }
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value5180,
                            ) => Some(source_ml_source1_data_property_value5180.clone()),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value5181,
                            ) => Some(source_ml_source1_data_property_value5181.clone()),
                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                None::<
                                    std::rc::Rc<
                                        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
                                    >,
                                >
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                None::<
                                    std::rc::Rc<
                                        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
                                    >,
                                >
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
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
                                None::<
                                    std::rc::Rc<
                                        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
                                    >,
                                >
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                None::<
                                    std::rc::Rc<
                                        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
                                    >,
                                >
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                None::<
                                    std::rc::Rc<
                                        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
                                    >,
                                >
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                None::<
                                    std::rc::Rc<
                                        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
                                    >,
                                >
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                None::<
                                    std::rc::Rc<
                                        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
                                    >,
                                >
                            }
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    })
                {
                    self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 119, 101, 97, 107, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                } else {
                    {
                        let source_ml_temporary_read_java_script_weak_collection_object_error_written59 = {
                            let ml_field_owner59 = budget.clone();
                            let ml_field_previous59 = ml_field_owner59.borrow().1;
                            let ml_field_value59 = {
                                let ml_reference_array59 =
                                    entries.clone().expect("a proven present value");
                                ml_reference_array59.borrow().len() as f64
                            };
                            ml_field_owner59.borrow_mut().1 =
                                ml_field_previous59 + ml_field_value59;
                            ()
                        };
                        if (({ budget.borrow().clone() }).1) > 1_048_576f64 {
                            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 97, 110, 100, 32, 115, 116, 114, 111, 110, 103, 32, 115, 108, 111, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let previous_weak_key = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_temporary_values57 =
                                        entries.clone().expect("a proven present value");
                                    {
                                        let source_ml_temporary_index57 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source1_s5182 = self::source_ml_temporary_read_java_script_weak_collection_object_error_loop60(heap.clone(), object.clone(), budget.clone(), Some(previous_weak_key), source_ml_temporary_values57, source_ml_temporary_index57);
                                            match source_ml_source1_s5182 {
                                                self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Done => {
                                                    self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Next
                                                }
                                                self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(source_ml_temporary_result60) => {
                                                    self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Return(source_ml_temporary_result60.clone())
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
        } else {
            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Next
        };
        match source_ml_source1_s4972 {
            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Return(source_ml_temporary_result58) => {
                source_ml_temporary_result58.clone()
            }
        }
    }
}
