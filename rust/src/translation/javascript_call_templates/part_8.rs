// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::JavaScriptWeakCollectionEntry;
use super::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65;
use super::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66;
use super::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69;
use super::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::source_ml_temporary_read_java_script_weak_collection_object_error_loop71;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_iterator_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
) -> self::ml_string::Text {
    {
        let source_ml_source1_s4726 = match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
            }
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, callable, _, object_collection, cursor, kind, collection_kind, executing, iterated_text) => {
                if (((((((!(self::ml_array::includes(&{ (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])]))).borrow().clone() }, &collection_kind.clone(), 0f64, false, |a, b| a == b)) || callable) || !(self::ml_math::is_safe_integer(cursor))) || (cursor < 0f64)) || {
                    let source_ml_source1_same_value_operand4816 = -0f64;
                    (((cursor == source_ml_source1_same_value_operand4816) && ((cursor != 0f64) || ((1f64 / cursor) == (1f64 / source_ml_source1_same_value_operand4816)))) || ((cursor != cursor) && (source_ml_source1_same_value_operand4816 != source_ml_source1_same_value_operand4816)))
                }) || (cursor > (if collection_kind == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121])) {
                    9_007_199_254_740_991f64
                } else if collection_kind == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])) {
                        16_777_216f64
                    } else {
                        65_536f64
                    }))) || !(self::ml_array::includes(&{ (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![107, 101, 121, 115]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])]))).borrow().clone() }, &kind.clone(), 0f64, false, |a, b| a == b))) || ((collection_kind == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) && ((match ((executing.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))))) || ((collection_kind != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) && {
                    let source_ml_source1_comparison_operand4817 = ((executing.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                    {
                        let source_ml_source1_comparison_operand4818 = None::<()>;
                        !(match (source_ml_source1_comparison_operand4817, source_ml_source1_comparison_operand4818) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                    }
                }) {
                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 117, 114, 115, 111, 114, 32, 97, 110, 100, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 107, 105, 110, 100]))
                } else if collection_kind == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])) {
                        {
                            let text = ((iterated_text.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone();
                            if ((object_collection != (self::ml::Big::from_i128(0))) || (kind != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])))) || match (text.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Return(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 108, 121, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 99, 117, 114, 115, 111, 114]))
                            } else {
                                {
                                    let error = self::read_java_script_ordinary_value_error(heap.clone(), text.clone().expect("a proven present value"));
                                    {
let ml_library_inequality_3 = error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_3 {
                                        self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Return(error.clone())
                                    } else {
                                        {
                                            let length = self::read_java_script_semantic_text_length(text.clone().expect("a proven present value"));
                                            {
                                                let source_ml_source1_s4828 = match text.clone().expect("a proven present value") {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        {
let ml_library_inequality_2 = cursor != 0f64 ;
if ml_library_inequality_2 {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Return(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 108, 101, 97, 115, 101, 115, 32, 105, 116, 115, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 117, 114, 115, 111, 114]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Next
                                                        }
}
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Number(_) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        if (length < 0f64) || (cursor > length) {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 99, 111, 100, 101, 45, 117, 110, 105, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Next
                                                        }
                                                    }
                                                };
                                                match source_ml_source1_s4828 {
                                                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Next => {
                                                        {
                                                            let source_ml_temporary_read_java_script_collection_iterator_object_error_written67 = { let ml_field_owner54 = budget.clone(); let ml_field_previous54 = ml_field_owner54.borrow().3; let ml_field_value54 = self::ml_math::max(0f64, length); ml_field_owner54.borrow_mut().3 = ml_field_previous54 + ml_field_value54; () };
                                                            if (({ budget.borrow().clone() }).3) > 67_108_864f64 {
                                                                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 101, 120, 116, 115, 32, 97, 110, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 118, 97, 108, 117, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                            } else {
                                                                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Return(source_ml_temporary_result67) => {
                                                        self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Return(source_ml_temporary_result67.clone())
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
                                        let source_ml_source1_array_read_value4840 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value4838, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value4838.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_array_read_index4841 = self::find_java_script_ordinary_object(match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value4839, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value4839.clone()
                                                }
                                            }, object_collection.clone());
                                            { let ml_reference_array56 = source_ml_source1_array_read_value4840; let ml_reference_position56 = source_ml_source1_array_read_index4841; self::ml_array::optional_at(&*ml_reference_array56.borrow(), self::ml_array::number_index(ml_reference_position56)) }
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
                                        let source_ml_source1_nullish_field_receiver5038 = (match collection.clone().expect("a proven present value") {
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
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, source_ml_source1_data_property_value5036) => {
                                                Some(source_ml_source1_data_property_value5036.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, source_ml_source1_data_property_value5037) => {
                                                Some(source_ml_source1_data_property_value5037.clone())
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
                                        if match (source_ml_source1_nullish_field_receiver5038.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            {
                                                let source_ml_source1_nullish_field_present5039 = ((source_ml_source1_nullish_field_receiver5038.clone()).0).clone().expect("a proven present value");
                                                { let ml_reference_array57 = source_ml_source1_nullish_field_present5039; ml_reference_array57.borrow().len() as f64 }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                        }
                                    })))) {
                                        self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Return(self::ml_string::Text::from_units(vec![97, 32, 108, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 97, 110, 100, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                    } else {
                                        self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
                                    }
                                }
                            } else {
                                if (cursor != 0f64) || {
                                    let source_ml_source1_comparison_operand5040 = ((executing.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                    match (source_ml_source1_comparison_operand5040, true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                } {
                                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Return(self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 108, 101, 97, 115, 101, 115, 32, 105, 116, 115, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 44, 32, 99, 117, 114, 115, 111, 114, 32, 97, 110, 100, 32, 101, 120, 101, 99, 117, 116, 105, 110, 103, 32, 115, 116, 97, 116, 101]))
                                } else {
                                    self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
                                }
                            }
}} else {
                            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Return(self::ml_string::Text::from_units(vec![110, 111, 110, 45, 83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 102, 97, 109, 105, 108, 105, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 83, 116, 114, 105, 110, 103, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                        }
            }
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
            }
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
            }
            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next
            }
        };
        match source_ml_source1_s4726 {
            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65::MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Return(source_ml_temporary_result65) => {
                source_ml_temporary_result65.clone()
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
        let source_ml_source1_s5252 = if match object.clone() {
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
                        source_ml_source1_data_property_value5443,
                    ) => Some(source_ml_source1_data_property_value5443.clone()),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5444,
                    ) => Some(source_ml_source1_data_property_value5444.clone()),
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
                        source_ml_source1_data_property_value5445,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5445,
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5446,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5446,
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5447,
                        _,
                    ) => source_ml_source1_data_property_value5447,
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5448,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5448,
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5449,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5449,
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5450,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5450,
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5451,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5451,
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5452,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5452,
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5453,
                        _,
                    ) => source_ml_source1_data_property_value5453,
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5454,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5454,
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5455,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5455,
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5456,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5456,
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5457,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5457,
                } || match (entries.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) || !({
                    let source_ml_source1_array_argument5458 =
                        entries.clone().expect("a proven present value");
                    true
                })) || (({
                    let ml_reference_array58 = entries.clone().expect("a proven present value");
                    ml_reference_array58.borrow().len() as f64
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
                                source_ml_source1_data_property_value5460,
                            ) => Some(source_ml_source1_data_property_value5460.clone()),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value5461,
                            ) => Some(source_ml_source1_data_property_value5461.clone()),
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
                    self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 119, 101, 97, 107, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                } else {
                    {
                        let source_ml_temporary_read_java_script_weak_collection_object_error_written70 = {
                            let ml_field_owner60 = budget.clone();
                            let ml_field_previous60 = ml_field_owner60.borrow().1;
                            let ml_field_value60 = {
                                let ml_reference_array60 =
                                    entries.clone().expect("a proven present value");
                                ml_reference_array60.borrow().len() as f64
                            };
                            ml_field_owner60.borrow_mut().1 =
                                ml_field_previous60 + ml_field_value60;
                            ()
                        };
                        if (({ budget.borrow().clone() }).1) > 1_048_576f64 {
                            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 97, 110, 100, 32, 115, 116, 114, 111, 110, 103, 32, 115, 108, 111, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let previous_weak_key = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_temporary_values68 =
                                        entries.clone().expect("a proven present value");
                                    {
                                        let source_ml_temporary_index68 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source1_s5462 = self::source_ml_temporary_read_java_script_weak_collection_object_error_loop71(heap.clone(), object.clone(), budget.clone(), Some(previous_weak_key), source_ml_temporary_values68, source_ml_temporary_index68);
                                            match source_ml_source1_s5462 {
                                                self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Done => {
                                                    self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69Next
                                                }
                                                self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Return(source_ml_temporary_result71) => {
                                                    self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69Return(source_ml_temporary_result71.clone())
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
            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69Next
        };
        match source_ml_source1_s5252 {
            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69::MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69Return(source_ml_temporary_result69) => {
                source_ml_temporary_result69.clone()
            }
        }
    }
}
