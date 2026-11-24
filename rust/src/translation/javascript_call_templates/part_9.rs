// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82;
use super::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83;
use super::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84;
use super::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Result;
use super::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result;
use super::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74;
use super::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Result;
use super::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop77Result;
use super::ml;
use super::ml_string;
use super::order_java_script_collection_keys;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_read_java_script_ordinary_object_descriptor_error_loop86;
use super::source_ml_temporary_read_java_script_ordinary_prototype_chain_error_loop87;
use super::source_ml_temporary_read_java_script_strong_collection_object_error_loop76;
use super::source_ml_temporary_read_java_script_strong_collection_object_error_loop77;
use super::test_complete_java_script_property_descriptor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_strong_collection_object_error(
    heap: self::JavaScriptOrdinaryHeap,
    object: self::JavaScriptOrdinaryObject,
    budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
) -> self::ml_string::Text {
    {
        let source_ml_source1_s5670 = if match object.clone() {
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
                        source_ml_source1_data_property_value5861,
                    ) => Some(source_ml_source1_data_property_value5861.clone()),
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5862,
                    ) => Some(source_ml_source1_data_property_value5862.clone()),
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
                        source_ml_source1_data_property_value5863,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5863,
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5864,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5864,
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5865,
                        _,
                    ) => source_ml_source1_data_property_value5865,
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5866,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5866,
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5867,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5867,
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5868,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5868,
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5869,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5869,
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5870,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5870,
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5871,
                        _,
                    ) => source_ml_source1_data_property_value5871,
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5872,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5872,
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5873,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5873,
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5874,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5874,
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value5875,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value5875,
                } || match (entries.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) || (({
                    let ml_reference_array62 = entries.clone().expect("a proven present value");
                    ml_reference_array62.borrow().len() as f64
                }) > 65_536f64)
                {
                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 111, 98, 106, 101, 99, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                } else {
                    {
                        let source_ml_temporary_read_java_script_strong_collection_object_error_written75 = {
                            let ml_field_owner64 = budget.clone();
                            let ml_field_previous64 = ml_field_owner64.borrow().1;
                            let ml_field_value64 = {
                                let ml_reference_array64 =
                                    entries.clone().expect("a proven present value");
                                ml_reference_array64.borrow().len() as f64
                            };
                            ml_field_owner64.borrow_mut().1 =
                                ml_field_previous64 + ml_field_value64;
                            ()
                        };
                        if (({ budget.borrow().clone() }).1) > 1_048_576f64 {
                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 110, 116, 114, 121, 32, 104, 105, 115, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::JavaScriptSemanticValue,
                                >::new(
                                )));
                                {
                                    let source_ml_temporary_values73 =
                                        entries.clone().expect("a proven present value");
                                    {
                                        let source_ml_temporary_index73 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source1_s5876 = self::source_ml_temporary_read_java_script_strong_collection_object_error_loop76(heap.clone(), object.clone(), budget.clone(), keys.clone(), source_ml_temporary_values73, source_ml_temporary_index73);
                                            match source_ml_source1_s5876 {
                                                self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Done => {
                                                    {
                                                        let ordered_keys = self::order_java_script_collection_keys(keys.clone());
                                                        {
                                                            let at = 1f64;
                                                            {
                                                                let source_ml_source1_s5877 = self::source_ml_temporary_read_java_script_strong_collection_object_error_loop77(ordered_keys, at);
                                                                match source_ml_source1_s5877 {
                                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop77Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop77Done => {
                                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74Next
                                                                    }
                                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop77Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop77Return(source_ml_temporary_result77) => {
                                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74Return(source_ml_temporary_result77.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Return(source_ml_temporary_result77_2) => {
                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74Return(source_ml_temporary_result77_2.clone())
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
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74Next
        };
        match source_ml_source1_s5670 {
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74Next => {
                self::ml_string::Text::from_units(vec![])
            }
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74Return(source_ml_temporary_result74) => {
                source_ml_temporary_result74.clone()
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
            let source_ml_source1_s5878 = {
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
                                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82Return(error.clone())
                            } else {
                                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82Next
                            }
                        }
                    }
                } else {
                    self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82Next
                }
            };
            match source_ml_source1_s5878 {
                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82Next => {
                    {
                        let source_ml_source1_s5879 = {
let ml_product_condition_2 = { descriptor.borrow().clone() };
if ml_product_condition_2 {
                            {
                                let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), ({ descriptor.borrow().clone() }).4);
                                {
let ml_library_inequality_2 = error_2 != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_2 {
                                    self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83Return(error_2.clone())
                                } else {
                                    self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83Next
                                }
}
                            }
                        } else {
                            self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83Next
                        }
};
                        match source_ml_source1_s5879 {
                            self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83Next => {
                                {
                                    let source_ml_source1_s5880 = {
let ml_product_condition_1 = { descriptor.borrow().clone() };
if ml_product_condition_1 {
                                        {
                                            let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), ({ descriptor.borrow().clone() }).6);
                                            {
let ml_library_inequality_1 = error_3 != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84Return(error_3.clone())
                                            } else {
                                                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84Next
                                            }
}
                                        }
                                    } else {
                                        self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84Next
                                    }
};
                                    match source_ml_source1_s5880 {
                                        self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84Next => {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                        self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84Return(source_ml_temporary_result84) => {
                                            source_ml_temporary_result84.clone()
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83Return(source_ml_temporary_result84_2) => {
                                source_ml_temporary_result84_2.clone()
                            }
                        }
                    }
                }
                self::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82::MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82Return(source_ml_temporary_result84_3) => {
                    source_ml_temporary_result84_3.clone()
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
                        let source_ml_temporary_read_java_script_ordinary_object_descriptor_error_written85 = {
                            let ml_field_owner73 = budget.clone();
                            let ml_field_previous73 = ml_field_owner73.borrow().4;
                            let ml_field_value73 = {
                                let ml_reference_array73 = properties.clone();
                                ml_reference_array73.borrow().len() as f64
                            };
                            ml_field_owner73.borrow_mut().4 =
                                ml_field_previous73 + ml_field_value73;
                            ()
                        };
                        if ((({ budget.borrow().clone() }).4) > 16_384f64)
                            || (({
                                let ml_reference_array75 = properties.clone();
                                ml_reference_array75.borrow().len() as f64
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
                                    let source_ml_source1_s5899 = self::source_ml_temporary_read_java_script_ordinary_object_descriptor_error_loop86(heap.clone(), properties.clone(), at);
                                    match source_ml_source1_s5899 {
                                    self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Done => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Return(source_ml_temporary_result86) => {
                                        source_ml_temporary_result86.clone()
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
                source_ml_source1_data_property_value5900,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5900 =
                    *source_ml_source1_data_property_value5900;
                source_ml_source1_data_property_value5900.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                _,
                source_ml_source1_data_property_value5901,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5901 =
                    *source_ml_source1_data_property_value5901;
                source_ml_source1_data_property_value5901.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(
                _,
                source_ml_source1_data_property_value5902,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5902 =
                    *source_ml_source1_data_property_value5902;
                source_ml_source1_data_property_value5902.clone()
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                _,
                source_ml_source1_data_property_value5903,
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
                let source_ml_source1_data_property_value5903 =
                    *source_ml_source1_data_property_value5903;
                source_ml_source1_data_property_value5903.clone()
            }
            self::JavaScriptOrdinaryObject::MapObject(
                _,
                source_ml_source1_data_property_value5904,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5904 =
                    *source_ml_source1_data_property_value5904;
                source_ml_source1_data_property_value5904.clone()
            }
            self::JavaScriptOrdinaryObject::SetObject(
                _,
                source_ml_source1_data_property_value5905,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5905 =
                    *source_ml_source1_data_property_value5905;
                source_ml_source1_data_property_value5905.clone()
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(
                _,
                source_ml_source1_data_property_value5906,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5906 =
                    *source_ml_source1_data_property_value5906;
                source_ml_source1_data_property_value5906.clone()
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(
                _,
                source_ml_source1_data_property_value5907,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5907 =
                    *source_ml_source1_data_property_value5907;
                source_ml_source1_data_property_value5907.clone()
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(
                _,
                source_ml_source1_data_property_value5908,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5908 =
                    *source_ml_source1_data_property_value5908;
                source_ml_source1_data_property_value5908.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(
                _,
                source_ml_source1_data_property_value5909,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5909 =
                    *source_ml_source1_data_property_value5909;
                source_ml_source1_data_property_value5909.clone()
            }
            self::JavaScriptOrdinaryObject::HostURLObject(
                _,
                source_ml_source1_data_property_value5910,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5910 =
                    *source_ml_source1_data_property_value5910;
                source_ml_source1_data_property_value5910.clone()
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(
                _,
                source_ml_source1_data_property_value5911,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5911 =
                    *source_ml_source1_data_property_value5911;
                source_ml_source1_data_property_value5911.clone()
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                _,
                source_ml_source1_data_property_value5912,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_data_property_value5912 =
                    *source_ml_source1_data_property_value5912;
                source_ml_source1_data_property_value5912.clone()
            }
        };
        {
            let depth = 0f64;
            {
                let source_ml_source1_s5913 = self::source_ml_temporary_read_java_script_ordinary_prototype_chain_error_loop87(heap.clone(), object.clone(), prototype, depth);
                match source_ml_source1_s5913 {
                    self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Done => {
                        self::ml_string::Text::from_units(vec![])
                    }
                    self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Return(source_ml_temporary_result87) => {
                        source_ml_temporary_result87.clone()
                    }
                }
            }
        }
    }
}
