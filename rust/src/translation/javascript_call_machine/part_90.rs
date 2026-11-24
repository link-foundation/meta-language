// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCallTemplate;
use super::JavaScriptCollectionCompletion;
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryCopyJavaScriptCollectionLoop427Result;
use super::MlSource1TemporaryDependencyFunction3Join716;
use super::MlSource1TemporaryDependencyFunction3Loop718Result;
use super::MlSource1TemporaryFindJavaScriptCallTemplateLoop923Result;
use super::MlSource1TemporaryReadJavaScriptCollectionConstructorKindLoop845Result;
use super::construct_java_script_semantic_text_units;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_value_error;
use super::replace_java_script_collection_entries;
use super::source_ml_source1_temporary_copy_java_script_collection_loop427;
use super::source_ml_source1_temporary_dependency_function3_loop718;
use super::source_ml_source1_temporary_find_java_script_call_template_loop923;
use super::source_ml_source1_temporary_read_java_script_collection_constructor_kind_loop845;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn copy_java_script_collection(
    heap: self::JavaScriptOrdinaryHeap,
    target: self::JavaScriptSemanticValue,
    input: self::JavaScriptSemanticValue,
    adder: self::JavaScriptSemanticValue,
    iterator_key: self::JavaScriptSemanticValue,
    iterator_method: self::JavaScriptSemanticValue,
    iterator_prototype: self::JavaScriptSemanticValue,
    next_method: self::JavaScriptSemanticValue,
) -> self::JavaScriptCollectionCompletion {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptCollectionCompletion::CollectionInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                {
                    let source_ml_source1_temporary_values426 =
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            target.clone(),
                            input.clone(),
                            adder.clone(),
                            iterator_key.clone(),
                            iterator_method.clone(),
                            iterator_prototype.clone(),
                            next_method.clone(),
                        ]));
                    {
                        let source_ml_source1_temporary_index426 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s17238 = self::source_ml_source1_temporary_copy_java_script_collection_loop427(heap.clone(), source_ml_source1_temporary_values426.clone(), source_ml_source1_temporary_index426.clone());
                            match source_ml_source2_s17238.clone() {
                                    self::MlSource1TemporaryCopyJavaScriptCollectionLoop427Result::MlSource1TemporaryCopyJavaScriptCollectionLoop427Done => {
                                        {
                                            let object = match target.clone() {
                                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                                    {
                                                        let source_ml_source2_array_read_value17242 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_array_read_index17243 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                            { let ml_reference_array910 = source_ml_source2_array_read_value17242.clone(); let ml_reference_position910 = source_ml_source2_array_read_index17243; let ml_reference_projection910 = self::ml_array::optional_at(&*ml_reference_array910.borrow(), self::ml_array::number_index(ml_reference_position910)); ml_reference_projection910 }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    None::<self::JavaScriptOrdinaryObject>
                                                }
                                            };
                                            if ((match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            } && match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            })) || (({ let ml_reference_array911 = (((match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, source_ml_source2_data_property_value17258) => {
                                                    Some((source_ml_source2_data_property_value17258.clone(),))
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, source_ml_source2_data_property_value17259) => {
                                                    Some((source_ml_source2_data_property_value17259.clone(),))
                                                }
                                                _ => {
                                                    None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>,)>
                                                }
                                            }).clone().expect("a proven present value")).0).clone(); let ml_reference_projection911 = ml_reference_array911.borrow().len() as f64; ml_reference_projection911 }) != (0f64))) {
                                                self::JavaScriptCollectionCompletion::CollectionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 111, 112, 121, 105, 110, 103, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 101, 109, 112, 116, 121, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110]))
                                            } else {
                                                if (match input.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        true
                                                    }
                                                    _ => {
                                                        false
                                                    }
                                                } || match input.clone() {
                                                    self::JavaScriptSemanticValue::Null => {
                                                        true
                                                    }
                                                    _ => {
                                                        false
                                                    }
                                                }) {
                                                    self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(heap.clone()), Box::new(target.clone()))
                                                } else {
                                                    {
                                                        let source = match input.clone() {
                                                            self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                                                {
                                                                    let source_ml_source2_array_read_value17263 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index17264 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity_2.clone());
                                                                        { let ml_reference_array912 = source_ml_source2_array_read_value17263.clone(); let ml_reference_position912 = source_ml_source2_array_read_index17264; let ml_reference_projection912 = self::ml_array::optional_at(&*ml_reference_array912.borrow(), self::ml_array::number_index(ml_reference_position912)); ml_reference_projection912 }
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                None::<self::JavaScriptOrdinaryObject>
                                                            }
                                                        };
                                                        if (((match (source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (match source.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        } && match source.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        })) || ((match source.clone().expect("a proven present value") {
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
                                                        }) != (match object.clone().expect("a proven present value") {
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
                                                        }))) || match iterator_key.clone() {
                                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) {
                                                            self::JavaScriptCollectionCompletion::CollectionNeedIterable(Box::new(heap.clone()), Box::new(target.clone()), Box::new(input.clone()))
                                                        } else {
                                                            match iterator_prototype.clone() {
                                                                self::JavaScriptSemanticValue::Object(identity_3, _) => {
                                                                    if (((((match adder.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    } || !(((match adder.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value17281) => {
                                                                            Some((source_ml_source2_data_property_value17281,))
                                                                        }
                                                                        _ => {
                                                                            None::<(bool,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0)) || match iterator_method.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }) || !(((match iterator_method.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value17284) => {
                                                                            Some((source_ml_source2_data_property_value17284,))
                                                                        }
                                                                        _ => {
                                                                            None::<(bool,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0)) || match next_method.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }) || !(((match next_method.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value17287) => {
                                                                            Some((source_ml_source2_data_property_value17287,))
                                                                        }
                                                                        _ => {
                                                                            None::<(bool,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0)) {
                                                                        self::JavaScriptCollectionCompletion::CollectionNeedIterable(Box::new(heap.clone()), Box::new(target.clone()), Box::new(input.clone()))
                                                                    } else {
                                                                        {
                                                                            let add_key = if match object.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                                    true
                                                                                }
                                                                                _ => {
                                                                                    false
                                                                                }
                                                                            } {
                                                                                self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![97f64, 100f64, 100f64])))
                                                                            } else {
                                                                                self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![115f64, 101f64, 116f64])))
                                                                            };
                                                                            {
                                                                                let next_key = self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![110f64, 101f64, 120f64, 116f64])));
                                                                                {
                                                                                    let actual_adder = self::plan_java_script_ordinary_get(heap.clone(), (((match object.clone().expect("a proven present value") {
                                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value17294, _, _, _, _, _) => {
                                                                                            Some((source_ml_source2_data_property_value17294.clone(),))
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value17295, _, _, _, _, _) => {
                                                                                            Some((source_ml_source2_data_property_value17295.clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(self::ml::Big,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone(), add_key.clone(), target.clone());
                                                                                    {
                                                                                        let actual_iterator = self::plan_java_script_ordinary_get(heap.clone(), (((match source.clone().expect("a proven present value") {
                                                                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value17296, _, _, _, _, _) => {
                                                                                                Some((source_ml_source2_data_property_value17296.clone(),))
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value17297, _, _, _, _, _) => {
                                                                                                Some((source_ml_source2_data_property_value17297.clone(),))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(self::ml::Big,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0).clone(), iterator_key.clone(), input.clone());
                                                                                        {
                                                                                            let actual_next = self::plan_java_script_ordinary_get(heap.clone(), identity_3.clone(), next_key.clone(), iterator_prototype.clone());
                                                                                            if ((((((((({ let ml_reference_value913 = actual_adder.borrow().clone(); ml_reference_value913 }).0).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) || (((({ let ml_reference_value914 = actual_iterator.borrow().clone(); ml_reference_value914 }).0).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))) || (((({ let ml_reference_value915 = actual_next.borrow().clone(); ml_reference_value915 }).0).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))) || !(self::test_java_script_semantic_same_value((({ let ml_reference_value916 = actual_adder.borrow().clone(); ml_reference_value916 }).0).5, adder.clone()))) || !(self::test_java_script_semantic_same_value((({ let ml_reference_value917 = actual_iterator.borrow().clone(); ml_reference_value917 }).0).5, iterator_method.clone()))) || !(self::test_java_script_semantic_same_value((({ let ml_reference_value918 = actual_next.borrow().clone(); ml_reference_value918 }).0).5, next_method.clone()))) {
                                                                                                self::JavaScriptCollectionCompletion::CollectionNeedIterable(Box::new(heap.clone()), Box::new(target.clone()), Box::new(input.clone()))
                                                                                            } else {
                                                                                                {
                                                                                                    let entries = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values919 = { let ml_reference_value920 = (std::rc::Rc::new(std::cell::RefCell::new({ let ml_values921 = { let ml_reference_value922 = ((((match source.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, source_ml_source2_data_property_value17298) => {
                                                                                                            Some((source_ml_source2_data_property_value17298.clone(),))
                                                                                                        }
                                                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, source_ml_source2_data_property_value17299) => {
                                                                                                            Some((source_ml_source2_data_property_value17299.clone(),))
                                                                                                        }
                                                                                                        _ => {
                                                                                                            None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>,)>
                                                                                                        }
                                                                                                    }).clone().expect("a proven present value")).0).clone()).borrow().clone(); ml_reference_value922 }; let ml_callback921 = std::rc::Rc::new(move |entry: self::JavaScriptCollectionEntry| -> bool { match entry.clone() {
                                                                                                        self::JavaScriptCollectionEntry::CollectionEntry(_, _, source_ml_source2_data_property_value17300) => {
                                                                                                            source_ml_source2_data_property_value17300
                                                                                                        }
                                                                                                    } }); ml_values921.iter().cloned().enumerate().filter_map(|(ml_index921, ml_value921)| if ml_callback921(ml_value921.clone()) { Some(ml_value921) } else { None }).collect::<Vec<_>>() }))).borrow().clone(); ml_reference_value920 }; let ml_callback919 = std::rc::Rc::new(move |entry: self::JavaScriptCollectionEntry| -> self::JavaScriptCollectionEntry { {
                                                                                                        let source_ml_source2_nominal_spread17301 = entry.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field17302 = match source_ml_source2_nominal_spread17301.clone() {
                                                                                                                self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value17303, _, _) => {
                                                                                                                    let source_ml_source2_data_property_value17303 = *source_ml_source2_data_property_value17303;
                                                                                                                    source_ml_source2_data_property_value17303.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field17304 = match source_ml_source2_nominal_spread17301.clone() {
                                                                                                                    self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source2_data_property_value17305, _) => {
                                                                                                                        let source_ml_source2_data_property_value17305 = *source_ml_source2_data_property_value17305;
                                                                                                                        source_ml_source2_data_property_value17305.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field17306 = match source_ml_source2_nominal_spread17301.clone() {
                                                                                                                        self::JavaScriptCollectionEntry::CollectionEntry(_, _, source_ml_source2_data_property_value17307) => {
                                                                                                                            source_ml_source2_data_property_value17307
                                                                                                                        }
                                                                                                                    };
                                                                                                                    self::JavaScriptCollectionEntry::CollectionEntry(Box::new(source_ml_source2_nominal_field17302.clone()), Box::new(source_ml_source2_nominal_field17304.clone()), source_ml_source2_nominal_field17306)
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    } }); ml_values919.iter().cloned().enumerate().map(|(ml_index919, ml_value919)| ml_callback919(ml_value919)).collect::<Vec<_>>() }));
                                                                                                    {
                                                                                                        let updated = self::replace_java_script_collection_entries(heap.clone(), object.clone().expect("a proven present value"), entries.clone());
                                                                                                        {
                                                                                                            let limit = self::read_java_script_ordinary_heap_error(updated.clone());
                                                                                                            if (limit != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                self::JavaScriptCollectionCompletion::CollectionResourceLimit(Box::new(heap.clone()), limit.clone())
                                                                                                            } else {
                                                                                                                self::JavaScriptCollectionCompletion::CollectionNormal(Box::new(updated.clone()), Box::new(target.clone()))
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
                                                                    self::JavaScriptCollectionCompletion::CollectionNeedIterable(Box::new(heap.clone()), Box::new(target.clone()), Box::new(input.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryCopyJavaScriptCollectionLoop427Result::MlSource1TemporaryCopyJavaScriptCollectionLoop427Return(source_ml_source1_temporary_result427) => {
                                        let source_ml_source1_temporary_result427 = *source_ml_source1_temporary_result427;
                                        source_ml_source1_temporary_result427.clone()
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
pub fn test_java_script_constructed_collection_kind(kind: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &{
            let ml_reference_value923 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![77, 97, 112]),
                self::ml_string::Text::from_units(vec![83, 101, 116]),
                self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]),
                self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116]),
            ])))
            .borrow()
            .clone();
            ml_reference_value923
        },
        kind,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_constructed_collection_allocation_kind(
    kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    if self::ml_array::includes(
        &{
            let ml_reference_value924 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![77, 97, 112]),
                self::ml_string::Text::from_units(vec![83, 101, 116]),
            ])))
            .borrow()
            .clone();
            ml_reference_value924
        },
        kind,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110])
    } else {
        if self::ml_array::includes(
            &{
                let ml_reference_value925 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]),
                    self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116]),
                ])))
                .borrow()
                .clone();
                ml_reference_value925
            },
            kind,
            0f64,
            false,
            |a, b| a == b,
        ) {
            self::ml_string::Text::from_units(vec![
                119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110,
            ])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function3(
    description: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let error = self::read_java_script_semantic_value_error(description.clone());
        if ((error != (self::ml_string::Text::from_units(vec![])))
            || ((match description.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                _ => true,
            } && match description.clone() {
                self::JavaScriptSemanticValue::Text(_) => false,
                _ => true,
            }) && match description.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => false,
                _ => true,
            }))
        {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(if !error.0.is_empty() {
                error.clone()
            } else {
                self::ml_string::Text::from_units(vec![
                    83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105,
                    118, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115,
                    32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 116, 105, 111, 110, 97, 108, 32,
                    116, 101, 120, 116, 32, 109, 101, 116, 97, 100, 97, 116, 97,
                ])
            })
        } else {
            {
                let length = if match description.clone() {
                    self::JavaScriptSemanticValue::Undefined => true,
                    _ => false,
                } {
                    0f64
                } else {
                    self::read_java_script_semantic_text_length(description.clone())
                };
                if (length > (1048568f64)) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(
                        self::ml_string::Text::from_units(vec![
                            83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116,
                            105, 118, 101, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100,
                            115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                            49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 111, 117, 116,
                            112, 117, 116, 32, 112, 114, 111, 102, 105, 108, 101,
                        ]),
                    )
                } else {
                    {
                        let units = std::rc::Rc::new(std::cell::RefCell::new(vec![
                            83f64, 121f64, 109f64, 98f64, 111f64, 108f64, 40f64,
                        ]));
                        {
                            let source_ml_source2_s17674 = match description.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlSource1TemporaryDependencyFunction3Join716::MlSource1TemporaryDependencyFunction3Join716Next
                                }
                                _ => {
                                    {
                                        let source_ml_source1_temporary_values715 = self::construct_java_script_semantic_text_units(description.clone(), 1048576f64);
                                        {
                                            let source_ml_source1_temporary_index715 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s17675 = self::source_ml_source1_temporary_dependency_function3_loop718(units.clone(), source_ml_source1_temporary_values715.clone(), source_ml_source1_temporary_index715.clone());
                                                match source_ml_source2_s17675.clone() {
                                                    self::MlSource1TemporaryDependencyFunction3Loop718Result::MlSource1TemporaryDependencyFunction3Loop718Done => {
                                                        self::MlSource1TemporaryDependencyFunction3Join716::MlSource1TemporaryDependencyFunction3Join716Next
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            };
                            match source_ml_source2_s17674.clone() {
                                self::MlSource1TemporaryDependencyFunction3Join716::MlSource1TemporaryDependencyFunction3Join716Next => {
                                    {
                                        let source_ml_source1_temporary_dependency_function3_ignored717 = { let ml_method_owner926 = units.clone(); let ml_method_argument926_0 = 41f64; let ml_inserted = vec![ml_method_argument926_0]; let mut ml_method_values926 = ml_method_owner926.borrow_mut(); let ml_length = ml_method_values926.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values926.extend(ml_inserted); ml_length as f64 };
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(units.clone())))
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
pub fn test_java_script_collection_iterator_intrinsic_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111,
            114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99,
        ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_collection_constructor_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117,
            99, 116, 111, 114,
        ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_collection_intrinsic_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 116, 114, 105, 110, 115,
            105, 99,
        ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_primitive_intrinsic_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode)
        == (self::ml_string::Text::from_units(vec![
            112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115,
            105, 99,
        ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_constructor_kind(
    fn_: self::JavaScriptCallFunction,
) -> self::ml_string::Text {
    match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(
            _,
            _,
            shared_constructor_field1,
            _,
            _,
            _,
            _,
            shared_constructor_field2,
            _,
            _,
        ) => {
            if !(self::test_java_script_collection_constructor_mode(&(shared_constructor_field1))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                {
                    let kind = self::ml_string::Text::from_units(vec![]);
                    {
                        let source_ml_source1_temporary_values844 =
                            shared_constructor_field2.clone();
                        {
                            let source_ml_source1_temporary_index844 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s17858 = self::source_ml_source1_temporary_read_java_script_collection_constructor_kind_loop845(kind.clone(), source_ml_source1_temporary_values844.clone(), source_ml_source1_temporary_index844.clone());
                                match source_ml_source2_s17858.clone() {
                                    self::MlSource1TemporaryReadJavaScriptCollectionConstructorKindLoop845Result::MlSource1TemporaryReadJavaScriptCollectionConstructorKindLoop845Done(kind_2) => {
                                        kind_2.clone()
                                    }
                                    self::MlSource1TemporaryReadJavaScriptCollectionConstructorKindLoop845Result::MlSource1TemporaryReadJavaScriptCollectionConstructorKindLoop845Return(source_ml_source1_temporary_result845) => {
                                        source_ml_source1_temporary_result845.clone()
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
pub fn find_java_script_call_template(
    program: self::JavaScriptCallProgram,
    owner: self::ml::Big,
    ordinal: f64,
) -> f64 {
    {
        let templates = (match program.clone() {
            self::JavaScriptCallProgram::CallProgram(
                _,
                _,
                _,
                source_ml_source2_data_property_value18311,
                _,
                _,
            ) => ((source_ml_source2_data_property_value18311
                .clone()
                .unwrap_or_else(|| {
                    (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,)
                }))
            .0)
                .clone(),
        })
        .clone()
        .unwrap_or_else(|| {
            std::rc::Rc::new(std::cell::RefCell::new(
                Vec::<self::JavaScriptCallTemplate>::new(),
            ))
        });
        {
            let low = 0f64;
            {
                let high = {
                    let ml_reference_array927 = templates.clone();
                    let ml_reference_projection927 = ml_reference_array927.borrow().len() as f64;
                    ml_reference_projection927
                };
                {
                    let source_ml_source2_s18312 =
                        self::source_ml_source1_temporary_find_java_script_call_template_loop923(
                            owner.clone(),
                            ordinal,
                            templates.clone(),
                            low,
                            high,
                        );
                    match source_ml_source2_s18312.clone() {
                        self::MlSource1TemporaryFindJavaScriptCallTemplateLoop923Result::MlSource1TemporaryFindJavaScriptCallTemplateLoop923Done => {
                            (-1f64)
                        }
                        self::MlSource1TemporaryFindJavaScriptCallTemplateLoop923Result::MlSource1TemporaryFindJavaScriptCallTemplateLoop923Return(source_ml_source1_temporary_result923) => {
                            source_ml_source1_temporary_result923
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn project_java_script_call_template_function(
    template: self::JavaScriptCallTemplate,
    identity: self::ml::Big,
    captures: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    receiver: self::JavaScriptSemanticValue,
    new_target: Option<self::JavaScriptSemanticValue>,
) -> self::JavaScriptCallFunction {
    match template.clone() {
        self::JavaScriptCallTemplate::CallTemplate(
            _,
            _,
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            shared_constructor_field4,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let source_ml_source1_temporary_constructor_field925 =
                shared_constructor_field1.clone();
            {
                let source_ml_source1_temporary_constructor_field926 = (match template.clone() {
                    self::JavaScriptCallTemplate::CallTemplate(
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
                        source_ml_source2_data_property_value18314,
                        _,
                    ) => ((source_ml_source2_data_property_value18314
                        .clone()
                        .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                    .0)
                        .clone(),
                })
                .clone()
                .unwrap_or_else(|| {
                    self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])
                });
                {
                    let source_ml_source1_temporary_constructor_field928 =
                        shared_constructor_field2;
                    {
                        let source_ml_source1_temporary_constructor_field929 =
                            shared_constructor_field3;
                        {
                            let source_ml_source1_temporary_constructor_field931 =
                                shared_constructor_field4.clone();
                            if (!(match (new_target.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }) && (((match template.clone() {
                                self::JavaScriptCallTemplate::CallTemplate(
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
                                    source_ml_source2_data_property_value18319,
                                    _,
                                ) => ((source_ml_source2_data_property_value18319
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .unwrap_or_else(|| {
                                self::ml_string::Text::from_units(vec![
                                    108, 101, 120, 105, 99, 97, 108,
                                ])
                            })) == (self::ml_string::Text::from_units(vec![
                                108, 101, 120, 105, 99, 97, 108,
                            ])))) {
                                {
                                    let source_ml_source2_constructor_field18328 =
                                        Some(new_target.clone().expect("a proven present value"));
                                    self::JavaScriptCallFunction::CallFunction(
                                        identity.clone(),
                                        source_ml_source1_temporary_constructor_field925.clone(),
                                        source_ml_source1_temporary_constructor_field926.clone(),
                                        Box::new(receiver.clone()),
                                        source_ml_source1_temporary_constructor_field928,
                                        source_ml_source1_temporary_constructor_field929,
                                        captures.clone(),
                                        source_ml_source1_temporary_constructor_field931.clone(),
                                        Some((source_ml_source2_constructor_field18328.clone(),)),
                                        None::<(Option<self::JavaScriptCallSourceSpan>,)>,
                                    )
                                }
                            } else {
                                self::JavaScriptCallFunction::CallFunction(
                                    identity.clone(),
                                    source_ml_source1_temporary_constructor_field925.clone(),
                                    source_ml_source1_temporary_constructor_field926.clone(),
                                    Box::new(receiver.clone()),
                                    source_ml_source1_temporary_constructor_field928,
                                    source_ml_source1_temporary_constructor_field929,
                                    captures.clone(),
                                    source_ml_source1_temporary_constructor_field931.clone(),
                                    None::<(Option<self::JavaScriptSemanticValue>,)>,
                                    None::<(Option<self::JavaScriptCallSourceSpan>,)>,
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}
