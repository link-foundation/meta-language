// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71;
use super::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72;
use super::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73;
use super::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result;
use super::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result;
use super::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63;
use super::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result;
use super::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result;
use super::ml;
use super::ml_string;
use super::order_java_script_collection_keys;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_read_java_script_ordinary_object_descriptor_error_loop75;
use super::source_ml_temporary_read_java_script_ordinary_prototype_chain_error_loop76;
use super::source_ml_temporary_read_java_script_strong_collection_object_error_loop65;
use super::source_ml_temporary_read_java_script_strong_collection_object_error_loop66;
use super::test_complete_java_script_property_descriptor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_strong_collection_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
) -> self::ml_string::Text {
    {
        let source_ml_source1_s5390 = if match object.clone() {
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => true,
            _ => false,
        } || match object.clone() {
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => true,
            _ => false,
        } {
            {
                let entries = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
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
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>>
                    }
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5581,
                    ) => Some(source_ml_source1_data_property_value5581.clone()),
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5582,
                    ) => Some(source_ml_source1_data_property_value5582.clone()),
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
                if (match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5583,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5583,
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5584,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5584,
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5585,
                        _,
                    ) => source_ml_source1_data_property_value5585,
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5586,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5586,
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5587,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5587,
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5588,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5588,
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5589,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5589,
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5590,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5590,
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5591,
                        _,
                    ) => source_ml_source1_data_property_value5591,
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5592,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5592,
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5593,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5593,
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5594,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5594,
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5595,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5595,
                } || match (entries.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) || (({
                    let ml_reference_array160 = entries.clone().expect("a proven present value");
                    ml_reference_array160.borrow().len() as f64
                }) > 65_536f64)
                {
                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                } else {
                    {
                        let source_ml_temporary_read_java_script_strong_collection_object_error_written64 = {
                            let ml_field_owner162 = budget.clone();
                            let ml_field_previous162 = ml_field_owner162.borrow().1;
                            let ml_field_value162 = {
                                let ml_reference_array162 =
                                    entries.clone().expect("a proven present value");
                                ml_reference_array162.borrow().len() as f64
                            };
                            ml_field_owner162.borrow_mut().1 =
                                ml_field_previous162 + ml_field_value162;
                            ()
                        };
                        if (({ budget.borrow().clone() }).1) > 1_048_576f64 {
                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 110, 116, 114, 121, 32, 104, 105, 115, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::JavaScriptSemanticValue,
                                >::new(
                                )));
                                {
                                    let source_ml_temporary_values62 =
                                        entries.clone().expect("a proven present value");
                                    {
                                        let source_ml_temporary_index62 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source1_s5596 = self::source_ml_temporary_read_java_script_strong_collection_object_error_loop65(heap.clone(), object.clone(), budget.clone(), keys.clone(), source_ml_temporary_values62, source_ml_temporary_index62);
                                            match source_ml_source1_s5596 {
                                                self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Done => {
                                                    {
                                                        let ordered_keys = self::order_java_script_collection_keys(keys.clone());
                                                        {
                                                            let at = 1f64;
                                                            {
                                                                let source_ml_source1_s5597 = self::source_ml_temporary_read_java_script_strong_collection_object_error_loop66(ordered_keys, at);
                                                                match source_ml_source1_s5597 {
                                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Done => {
                                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Next
                                                                    }
                                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Return(source_ml_temporary_result66) => {
                                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Return(source_ml_temporary_result66.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(source_ml_temporary_result66_2) => {
                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Return(source_ml_temporary_result66_2.clone())
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
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Next
        };
        match source_ml_source1_s5390 {
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Return(source_ml_temporary_result63) => {
                source_ml_temporary_result63.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_ordinary_descriptor_value_error(
    heap: self::JavaScriptOrdinaryHeap,
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
            bool,
            bool,
            bool,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            bool,
            bool,
        )>,
    >,
) -> self::ml_string::Text {
    if (self::test_complete_java_script_property_descriptor(descriptor.clone())) {
        {
            let source_ml_source1_s5598 = {
                let ml_product_condition_3 = { descriptor.borrow().clone() };
                if ml_product_condition_3 {
                    {
                        let error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            ({ descriptor.borrow().clone() }).8,
                        );
                        {
                            let ml_library_inequality_3 =
                                error != (self::ml_string::Text::from_units(vec![]));
                            if ml_library_inequality_3 {
                                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Return(error.clone())
                            } else {
                                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Next
                            }
                        }
                    }
                } else {
                    self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Next
                }
            };
            match source_ml_source1_s5598 {
                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Next => {
                    {
                        let source_ml_source1_s5599 = {
let ml_product_condition_2 = { descriptor.borrow().clone() };
if ml_product_condition_2 {
                            {
                                let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), ({ descriptor.borrow().clone() }).4);
                                {
let ml_library_inequality_2 = error_2 != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_2 {
                                    self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Return(error_2.clone())
                                } else {
                                    self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Next
                                }
}
                            }
                        } else {
                            self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Next
                        }
};
                        match source_ml_source1_s5599 {
                            self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Next => {
                                {
                                    let source_ml_source1_s5600 = {
let ml_product_condition_1 = { descriptor.borrow().clone() };
if ml_product_condition_1 {
                                        {
                                            let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), ({ descriptor.borrow().clone() }).6);
                                            {
let ml_library_inequality_1 = error_3 != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Return(error_3.clone())
                                            } else {
                                                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Next
                                            }
}
                                        }
                                    } else {
                                        self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Next
                                    }
};
                                    match source_ml_source1_s5600 {
                                        self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Next => {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                        self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Return(source_ml_temporary_result73) => {
                                            source_ml_temporary_result73.clone()
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Return(source_ml_temporary_result73_2) => {
                                source_ml_temporary_result73_2.clone()
                            }
                        }
                    }
                }
                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Return(source_ml_temporary_result73_3) => {
                    source_ml_temporary_result73_3.clone()
                }
            }
        }
    } else {
        self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 114,
            101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118,
            97, 108, 105, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99,
            114, 105, 112, 116, 111, 114, 115,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_ordinary_object_descriptor_error(
    heap: self::JavaScriptOrdinaryHeap,
    prototype: self::JavaScriptSemanticValue,
    properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
) -> self::ml_string::Text {
    if match prototype.clone() {
        self::JavaScriptSemanticValue::Null => false,
        _ => true,
    } && match prototype.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => false,
        _ => true,
    } {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116,
            121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111,
            114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116,
        ])
    } else {
        {
            let prototype_error =
                self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
            {
                let ml_library_inequality_1 =
                    prototype_error != (self::ml_string::Text::from_units(vec![]));
                if ml_library_inequality_1 {
                    prototype_error.clone()
                } else {
                    {
                        let source_ml_temporary_read_java_script_ordinary_object_descriptor_error_written74 = {
                            let ml_field_owner171 = budget.clone();
                            let ml_field_previous171 = ml_field_owner171.borrow().4;
                            let ml_field_value171 = {
                                let ml_reference_array171 = properties.clone();
                                ml_reference_array171.borrow().len() as f64
                            };
                            ml_field_owner171.borrow_mut().4 =
                                ml_field_previous171 + ml_field_value171;
                            ()
                        };
                        if ((({ budget.borrow().clone() }).4) > 16_384f64)
                            || (({
                                let ml_reference_array173 = properties.clone();
                                ml_reference_array173.borrow().len() as f64
                            }) > 4096f64)
                        {
                            self::ml_string::Text::from_units(vec![
                                111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101,
                                114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116,
                                104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                                111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103,
                                101, 32, 98, 117, 100, 103, 101, 116,
                            ])
                        } else {
                            {
                                let at = 0f64;
                                {
                                    let source_ml_source1_s5619 = self::source_ml_temporary_read_java_script_ordinary_object_descriptor_error_loop75(heap.clone(), properties.clone(), at);
                                    match source_ml_source1_s5619 {
                                    self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Done => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(source_ml_temporary_result75) => {
                                        source_ml_temporary_result75.clone()
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
pub fn read_java_script_ordinary_prototype_chain_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
) -> self::ml_string::Text {
    {
        let prototype = match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(
                _,
                source_ml_source1_data_property_value5620,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5620 =
                    *source_ml_source1_data_property_value5620;
                source_ml_source1_data_property_value5620.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                _,
                source_ml_source1_data_property_value5621,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5621 =
                    *source_ml_source1_data_property_value5621;
                source_ml_source1_data_property_value5621.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(
                _,
                source_ml_source1_data_property_value5622,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5622 =
                    *source_ml_source1_data_property_value5622;
                source_ml_source1_data_property_value5622.clone()
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                _,
                source_ml_source1_data_property_value5623,
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
                let source_ml_source1_data_property_value5623 =
                    *source_ml_source1_data_property_value5623;
                source_ml_source1_data_property_value5623.clone()
            }
            self::JavaScriptOrdinaryObject::MapObject(
                _,
                source_ml_source1_data_property_value5624,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5624 =
                    *source_ml_source1_data_property_value5624;
                source_ml_source1_data_property_value5624.clone()
            }
            self::JavaScriptOrdinaryObject::SetObject(
                _,
                source_ml_source1_data_property_value5625,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5625 =
                    *source_ml_source1_data_property_value5625;
                source_ml_source1_data_property_value5625.clone()
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(
                _,
                source_ml_source1_data_property_value5626,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5626 =
                    *source_ml_source1_data_property_value5626;
                source_ml_source1_data_property_value5626.clone()
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(
                _,
                source_ml_source1_data_property_value5627,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5627 =
                    *source_ml_source1_data_property_value5627;
                source_ml_source1_data_property_value5627.clone()
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(
                _,
                source_ml_source1_data_property_value5628,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5628 =
                    *source_ml_source1_data_property_value5628;
                source_ml_source1_data_property_value5628.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(
                _,
                source_ml_source1_data_property_value5629,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5629 =
                    *source_ml_source1_data_property_value5629;
                source_ml_source1_data_property_value5629.clone()
            }
            self::JavaScriptOrdinaryObject::HostURLObject(
                _,
                source_ml_source1_data_property_value5630,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5630 =
                    *source_ml_source1_data_property_value5630;
                source_ml_source1_data_property_value5630.clone()
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(
                _,
                source_ml_source1_data_property_value5631,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5631 =
                    *source_ml_source1_data_property_value5631;
                source_ml_source1_data_property_value5631.clone()
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                _,
                source_ml_source1_data_property_value5632,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5632 =
                    *source_ml_source1_data_property_value5632;
                source_ml_source1_data_property_value5632.clone()
            }
        };
        {
            let depth = 0f64;
            {
                let source_ml_source1_s5633 = self::source_ml_temporary_read_java_script_ordinary_prototype_chain_error_loop76(heap.clone(), object.clone(), prototype, depth);
                match source_ml_source1_s5633 {
                    self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done => {
                        self::ml_string::Text::from_units(vec![])
                    }
                    self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Return(source_ml_temporary_result76) => {
                        source_ml_temporary_result76.clone()
                    }
                }
            }
        }
    }
}
