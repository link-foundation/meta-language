// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptObjectIntegrityState;
use super::JavaScriptObjectIntegrityTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_index;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_property_descriptor_kind;
use super::read_java_script_semantic_text_length;
use super::set_java_script_ordinary_prototype;
use super::test_java_script_array_length_key;
use super::test_java_script_ordinary_integrity_kind;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn observe_java_script_object_properties(
    heap: self::JavaScriptOrdinaryHeap,
    operation: self::ml_string::Text,
    input: self::JavaScriptSemanticValue,
    other: self::JavaScriptSemanticValue,
) -> self::JavaScriptObjectIntegrityTransition {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            if ((operation
                == (self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 80, 114, 111, 116, 111, 116, 121,
                    112, 101, 79, 102,
                ])))
                || (operation
                    == (self::ml_string::Text::from_units(vec![
                        79, 98, 106, 101, 99, 116, 46, 115, 101, 116, 80, 114, 111, 116, 111, 116,
                        121, 112, 101, 79, 102,
                    ]))))
            {
                if (match input.clone() {
                    self::JavaScriptSemanticValue::Null => true,
                    _ => false,
                } || match input.clone() {
                    self::JavaScriptSemanticValue::Undefined => true,
                    _ => false,
                }) {
                    self::JavaScriptObjectIntegrityTransition::ObjectIntegrityError(
                        Box::new(heap.clone()),
                        self::ml_string::Text::from_units(vec![
                            84, 121, 112, 101, 69, 114, 114, 111, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32,
                            117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 111, 114, 32, 110,
                            117, 108, 108, 32, 116, 111, 32, 111, 98, 106, 101, 99, 116,
                        ]),
                    )
                } else {
                    if (((operation
                        == (self::ml_string::Text::from_units(vec![
                            79, 98, 106, 101, 99, 116, 46, 115, 101, 116, 80, 114, 111, 116, 111,
                            116, 121, 112, 101, 79, 102,
                        ])))
                        && match other.clone() {
                            self::JavaScriptSemanticValue::Null => false,
                            _ => true,
                        })
                        && match other.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        })
                    {
                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityError(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                84, 121, 112, 101, 69, 114, 114, 111, 114,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121,
                                112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79,
                                98, 106, 101, 99, 116, 32, 111, 114, 32, 110, 117, 108, 108,
                            ]),
                        )
                    } else {
                        match input.clone() {
                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                let object = {
                                    let source_ml_source2_array_read_value26782 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_array_read_index26783 =
                                            self::find_java_script_ordinary_object(
                                                shared_constructor_field1.clone(),
                                                identity.clone(),
                                            );
                                        {
                                            let ml_reference_array1448 =
                                                source_ml_source2_array_read_value26782.clone();
                                            let ml_reference_position1448 =
                                                source_ml_source2_array_read_index26783;
                                            let ml_reference_projection1448 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array1448.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position1448,
                                                    ),
                                                );
                                            ml_reference_projection1448
                                        }
                                    }
                                };
                                if (match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_ordinary_integrity_kind(&(match object.clone().expect("a proven present value") {
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
                                    })))) {
                                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
                                    } else {
                                        if (operation == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102]))) {
                                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(Box::new(heap.clone()), Box::new(match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value26784, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value26784 = *source_ml_source2_data_property_value26784;
                                                    source_ml_source2_data_property_value26784.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value26785, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value26785 = *source_ml_source2_data_property_value26785;
                                                    source_ml_source2_data_property_value26785.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value26786, _, _, _) => {
                                                    let source_ml_source2_data_property_value26786 = *source_ml_source2_data_property_value26786;
                                                    source_ml_source2_data_property_value26786.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value26787, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value26787 = *source_ml_source2_data_property_value26787;
                                                    source_ml_source2_data_property_value26787.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value26788, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value26788 = *source_ml_source2_data_property_value26788;
                                                    source_ml_source2_data_property_value26788.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value26789, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value26789 = *source_ml_source2_data_property_value26789;
                                                    source_ml_source2_data_property_value26789.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value26790, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value26790 = *source_ml_source2_data_property_value26790;
                                                    source_ml_source2_data_property_value26790.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value26791, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value26791 = *source_ml_source2_data_property_value26791;
                                                    source_ml_source2_data_property_value26791.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value26792, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value26792 = *source_ml_source2_data_property_value26792;
                                                    source_ml_source2_data_property_value26792.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value26793, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value26793 = *source_ml_source2_data_property_value26793;
                                                    source_ml_source2_data_property_value26793.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source2_data_property_value26794, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value26794 = *source_ml_source2_data_property_value26794;
                                                    source_ml_source2_data_property_value26794.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source2_data_property_value26795, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value26795 = *source_ml_source2_data_property_value26795;
                                                    source_ml_source2_data_property_value26795.clone()
                                                }
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source2_data_property_value26796, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value26796 = *source_ml_source2_data_property_value26796;
                                                    source_ml_source2_data_property_value26796.clone()
                                                }
                                            }))
                                        } else {
                                            {
                                                let changed = self::set_java_script_ordinary_prototype(heap.clone(), identity.clone(), other.clone());
                                                if (((({ let ml_reference_value1449 = changed.borrow().clone(); ml_reference_value1449 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new((({ let ml_reference_value1450 = changed.borrow().clone(); ml_reference_value1450 }).0).1), (({ let ml_reference_value1451 = changed.borrow().clone(); ml_reference_value1451 }).0).0)
                                                } else {
                                                    if (({ let ml_reference_value1452 = changed.borrow().clone(); ml_reference_value1452 }).0).2 {
                                                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(Box::new((({ let ml_reference_value1453 = changed.borrow().clone(); ml_reference_value1453 }).0).1), Box::new(input.clone()))
                                                    } else {
                                                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityError(Box::new((({ let ml_reference_value1454 = changed.borrow().clone(); ml_reference_value1454 }).0).1), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 115, 101, 116, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                    }
                                                }
                                            }
                                        }
                                    }
                            }
                            _ => {
                                if (operation
                                    == (self::ml_string::Text::from_units(vec![
                                        79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 80, 114, 111,
                                        116, 111, 116, 121, 112, 101, 79, 102,
                                    ])))
                                {
                                    self::JavaScriptObjectIntegrityTransition::ObjectPrototypeMetadata(Box::new(heap.clone()), Box::new(input.clone()))
                                } else {
                                    self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(Box::new(heap.clone()), Box::new(input.clone()))
                                }
                            }
                        }
                    }
                }
            } else {
                if (((operation
                    == (self::ml_string::Text::from_units(vec![
                        79, 98, 106, 101, 99, 116, 46, 104, 97, 115, 79, 119, 110,
                    ])))
                    || (operation
                        == (self::ml_string::Text::from_units(vec![
                            79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112,
                            101, 46, 104, 97, 115, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116,
                            121,
                        ]))))
                    || (operation
                        == (self::ml_string::Text::from_units(vec![
                            79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112,
                            101, 46, 112, 114, 111, 112, 101, 114, 116, 121, 73, 115, 69, 110, 117,
                            109, 101, 114, 97, 98, 108, 101,
                        ]))))
                {
                    if (match input.clone() {
                        self::JavaScriptSemanticValue::Null => true,
                        _ => false,
                    } || match input.clone() {
                        self::JavaScriptSemanticValue::Undefined => true,
                        _ => false,
                    }) {
                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityError(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                84, 121, 112, 101, 69, 114, 114, 111, 114,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116,
                                32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 111, 114, 32,
                                110, 117, 108, 108, 32, 116, 111, 32, 111, 98, 106, 101, 99, 116,
                            ]),
                        )
                    } else {
                        {
                            let key_error = self::read_java_script_ordinary_key_error(
                                heap.clone(),
                                other.clone(),
                            );
                            if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(
                                    Box::new(heap.clone()),
                                    key_error.clone(),
                                )
                            } else {
                                {
                                    let present = false;
                                    {
                                        let source_ml_source2_s26834 = match input.clone() {
                                            self::JavaScriptSemanticValue::Object(
                                                identity_2,
                                                _,
                                            ) => {
                                                let object_2 = {
                                                    let source_ml_source2_array_read_value26838 =
                                                        shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_array_read_index26839 =
                                                            self::find_java_script_ordinary_object(
                                                                shared_constructor_field1.clone(),
                                                                identity_2.clone(),
                                                            );
                                                        {
                                                            let ml_reference_array1455 = source_ml_source2_array_read_value26838.clone();
                                                            let ml_reference_position1455 = source_ml_source2_array_read_index26839;
                                                            let ml_reference_projection1455 =
                                                                self::ml_array::optional_at(
                                                                    &*ml_reference_array1455
                                                                        .borrow(),
                                                                    self::ml_array::number_index(
                                                                        ml_reference_position1455,
                                                                    ),
                                                                );
                                                            ml_reference_projection1455
                                                        }
                                                    }
                                                };
                                                if (match (object_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_ordinary_integrity_kind(&(match object_2.clone().expect("a proven present value") {
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
                                                    })))) {
                                                        self::MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199::MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199Return(Box::new(self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 111, 119, 110, 45, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 112, 114, 111, 116, 111, 99, 111, 108]))))
                                                    } else {
                                                        {
                                                            let property = {
                                                                let source_ml_source2_array_read_value26866 = match object_2.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value26840, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value26840.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value26841, _, _, _) => {
                                                                        source_ml_source2_data_property_value26841.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value26842) => {
                                                                        source_ml_source2_data_property_value26842.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value26843, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value26843.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value26844, _) => {
                                                                        source_ml_source2_data_property_value26844.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value26845, _) => {
                                                                        source_ml_source2_data_property_value26845.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value26846, _) => {
                                                                        source_ml_source2_data_property_value26846.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value26847, _) => {
                                                                        source_ml_source2_data_property_value26847.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value26848) => {
                                                                        source_ml_source2_data_property_value26848.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value26849, _, _) => {
                                                                        source_ml_source2_data_property_value26849.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value26850, _, _) => {
                                                                        source_ml_source2_data_property_value26850.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value26851, _, _) => {
                                                                        source_ml_source2_data_property_value26851.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value26852, _, _) => {
                                                                        source_ml_source2_data_property_value26852.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_array_read_index26867 = self::find_java_script_ordinary_property(match object_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value26853, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value26853.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value26854, _, _, _) => {
                                                                            source_ml_source2_data_property_value26854.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value26855) => {
                                                                            source_ml_source2_data_property_value26855.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value26856, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value26856.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value26857, _) => {
                                                                            source_ml_source2_data_property_value26857.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value26858, _) => {
                                                                            source_ml_source2_data_property_value26858.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value26859, _) => {
                                                                            source_ml_source2_data_property_value26859.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value26860, _) => {
                                                                            source_ml_source2_data_property_value26860.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value26861) => {
                                                                            source_ml_source2_data_property_value26861.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value26862, _, _) => {
                                                                            source_ml_source2_data_property_value26862.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value26863, _, _) => {
                                                                            source_ml_source2_data_property_value26863.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value26864, _, _) => {
                                                                            source_ml_source2_data_property_value26864.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value26865, _, _) => {
                                                                            source_ml_source2_data_property_value26865.clone()
                                                                        }
                                                                    }, other.clone());
                                                                    { let ml_reference_array1456 = source_ml_source2_array_read_value26866.clone(); let ml_reference_position1456 = source_ml_source2_array_read_index26867; let ml_reference_projection1456 = self::ml_array::optional_at(&*ml_reference_array1456.borrow(), self::ml_array::number_index(ml_reference_position1456)); ml_reference_projection1456 }
                                                                }
                                                            };
                                                            {
                                                                let present_2 = (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((operation != (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 112, 114, 111, 112, 101, 114, 116, 121, 73, 115, 69, 110, 117, 109, 101, 114, 97, 98, 108, 101]))) || (({ let ml_reference_value1457 = (match property.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value26868) => {
                                                                        source_ml_source2_data_property_value26868.clone()
                                                                    }
                                                                }).borrow().clone(); ml_reference_value1457 }).0).2));
                                                                self::MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199::MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199Next(present_2)
                                                            }
                                                        }
                                                    }
                                            }
                                            _ => {
                                                if (match input.clone() {
                                                    self::JavaScriptSemanticValue::Text(_) => true,
                                                    _ => false,
                                                } || match input.clone() {
                                                    self::JavaScriptSemanticValue::SourceText(
                                                        _,
                                                        _,
                                                    ) => true,
                                                    _ => false,
                                                }) {
                                                    {
                                                        let index =
                                                            self::read_java_script_array_index(
                                                                other.clone(),
                                                            );
                                                        {
                                                            let present_3 = (((operation != (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 112, 114, 111, 112, 101, 114, 116, 121, 73, 115, 69, 110, 117, 109, 101, 114, 97, 98, 108, 101]))) && self::test_java_script_array_length_key(other.clone())) || ((index >= (0f64)) && (index < (self::read_java_script_semantic_text_length(input.clone())))));
                                                            self::MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199::MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199Next(present_3)
                                                        }
                                                    }
                                                } else {
                                                    self::MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199::MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199Next(present)
                                                }
                                            }
                                        };
                                        match source_ml_source2_s26834.clone() {
                                            self::MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199::MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199Next(present_4) => {
                                                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(Box::new(heap.clone()), Box::new(self::JavaScriptSemanticValue::Boolean(present_4)))
                                            }
                                            self::MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199::MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199Return(source_ml_source1_temporary_result1199) => {
                                                let source_ml_source1_temporary_result1199 = *source_ml_source1_temporary_result1199;
                                                source_ml_source1_temporary_result1199.clone()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    match input.clone() {
                        self::JavaScriptSemanticValue::Object(identity_3, _) => {
                            let object_3 = {
                                let source_ml_source2_array_read_value26875 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_array_read_index26876 =
                                        self::find_java_script_ordinary_object(
                                            shared_constructor_field1.clone(),
                                            identity_3.clone(),
                                        );
                                    {
                                        let ml_reference_array1458 =
                                            source_ml_source2_array_read_value26875.clone();
                                        let ml_reference_position1458 =
                                            source_ml_source2_array_read_index26876;
                                        let ml_reference_projection1458 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array1458.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position1458,
                                                ),
                                            );
                                        ml_reference_projection1458
                                    }
                                }
                            };
                            if (match (object_3.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } || !(self::test_java_script_ordinary_integrity_kind(
                                &(match object_3.clone().expect("a proven present value") {
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
                                    ) => self::ml_string::Text::from_units(vec![
                                        99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111,
                                        110, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![
                                            97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116,
                                        ])
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
                                    ) => self::ml_string::Text::from_units(vec![
                                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116,
                                        101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![
                                            109, 97, 112, 79, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![
                                            115, 101, 116, 79, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116,
                                    ]),
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
                                    ) => self::ml_string::Text::from_units(vec![
                                        111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101,
                                        99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106,
                                        101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106,
                                        101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102,
                                        101, 114, 79, 98, 106, 101, 99, 116,
                                    ]),
                                }),
                            ))) {
                                self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        111, 112, 97, 113, 117, 101, 32, 105, 110, 116, 101, 103,
                                        114, 105, 116, 121, 32, 99, 104, 97, 110, 103, 101, 115,
                                        32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101,
                                        105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110,
                                        45, 107, 101, 121, 32, 100, 101, 115, 99, 114, 105, 112,
                                        116, 111, 114, 32, 97, 110, 100, 32, 101, 120, 116, 101,
                                        110, 115, 105, 98, 105, 108, 105, 116, 121, 32, 112, 114,
                                        111, 116, 111, 99, 111, 108, 115,
                                    ]),
                                )
                            } else {
                                if (operation
                                    == (self::ml_string::Text::from_units(vec![
                                        79, 98, 106, 101, 99, 116, 46, 105, 115, 69, 120, 116, 101,
                                        110, 115, 105, 98, 108, 101,
                                    ])))
                                {
                                    self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(Box::new(heap.clone()), Box::new(self::JavaScriptSemanticValue::Boolean(match object_3.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, source_ml_source2_data_property_value26877, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value26877
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, source_ml_source2_data_property_value26878, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value26878
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, source_ml_source2_data_property_value26879, _, _) => {
                                                source_ml_source2_data_property_value26879
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, source_ml_source2_data_property_value26880, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value26880
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, source_ml_source2_data_property_value26881, _, _, _) => {
                                                source_ml_source2_data_property_value26881
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, source_ml_source2_data_property_value26882, _, _, _) => {
                                                source_ml_source2_data_property_value26882
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, source_ml_source2_data_property_value26883, _, _, _) => {
                                                source_ml_source2_data_property_value26883
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, source_ml_source2_data_property_value26884, _, _, _) => {
                                                source_ml_source2_data_property_value26884
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value26885, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value26885
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, source_ml_source2_data_property_value26886, _, _, _, _) => {
                                                source_ml_source2_data_property_value26886
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, source_ml_source2_data_property_value26887, _, _, _, _) => {
                                                source_ml_source2_data_property_value26887
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, source_ml_source2_data_property_value26888, _, _, _, _) => {
                                                source_ml_source2_data_property_value26888
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, source_ml_source2_data_property_value26889, _, _, _, _) => {
                                                source_ml_source2_data_property_value26889
                                            }
                                        })))
                                } else {
                                    if ((operation
                                        == (self::ml_string::Text::from_units(vec![
                                            79, 98, 106, 101, 99, 116, 46, 105, 115, 83, 101, 97,
                                            108, 101, 100,
                                        ])))
                                        || (operation
                                            == (self::ml_string::Text::from_units(vec![
                                                79, 98, 106, 101, 99, 116, 46, 105, 115, 70, 114,
                                                111, 122, 101, 110,
                                            ]))))
                                    {
                                        {
                                            let fixed = (!(match object_3.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, source_ml_source2_data_property_value26890, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value26890
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, source_ml_source2_data_property_value26891, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value26891
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, source_ml_source2_data_property_value26892, _, _) => {
                                                        source_ml_source2_data_property_value26892
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, source_ml_source2_data_property_value26893, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value26893
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, source_ml_source2_data_property_value26894, _, _, _) => {
                                                        source_ml_source2_data_property_value26894
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, source_ml_source2_data_property_value26895, _, _, _) => {
                                                        source_ml_source2_data_property_value26895
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, source_ml_source2_data_property_value26896, _, _, _) => {
                                                        source_ml_source2_data_property_value26896
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, source_ml_source2_data_property_value26897, _, _, _) => {
                                                        source_ml_source2_data_property_value26897
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value26898, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value26898
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, source_ml_source2_data_property_value26899, _, _, _, _) => {
                                                        source_ml_source2_data_property_value26899
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, source_ml_source2_data_property_value26900, _, _, _, _) => {
                                                        source_ml_source2_data_property_value26900
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, source_ml_source2_data_property_value26901, _, _, _, _) => {
                                                        source_ml_source2_data_property_value26901
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, source_ml_source2_data_property_value26902, _, _, _, _) => {
                                                        source_ml_source2_data_property_value26902
                                                    }
                                                }) && { let ml_values1459 = { let ml_reference_value1460 = (match object_3.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value26903, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value26903.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value26904, _, _, _) => {
                                                        source_ml_source2_data_property_value26904.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value26905) => {
                                                        source_ml_source2_data_property_value26905.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value26906, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value26906.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value26907, _) => {
                                                        source_ml_source2_data_property_value26907.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value26908, _) => {
                                                        source_ml_source2_data_property_value26908.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value26909, _) => {
                                                        source_ml_source2_data_property_value26909.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value26910, _) => {
                                                        source_ml_source2_data_property_value26910.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value26911) => {
                                                        source_ml_source2_data_property_value26911.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value26912, _, _) => {
                                                        source_ml_source2_data_property_value26912.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value26913, _, _) => {
                                                        source_ml_source2_data_property_value26913.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value26914, _, _) => {
                                                        source_ml_source2_data_property_value26914.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value26915, _, _) => {
                                                        source_ml_source2_data_property_value26915.clone()
                                                    }
                                                }).borrow().clone(); ml_reference_value1460 }; let ml_callback1459 = { let operation = operation.clone(); std::rc::Rc::new(move |property_2: self::JavaScriptOrdinaryProperty| -> bool { (!((({ let ml_reference_value1461 = (match property_2.clone() {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value26916) => {
                                                        source_ml_source2_data_property_value26916.clone()
                                                    }
                                                }).borrow().clone(); ml_reference_value1461 }).0).0) && (((operation == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 105, 115, 83, 101, 97, 108, 101, 100]))) || ((self::read_java_script_property_descriptor_kind(match property_2.clone() {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value26918) => {
                                                        source_ml_source2_data_property_value26918.clone()
                                                    }
                                                })) != (self::ml_string::Text::from_units(vec![100, 97, 116, 97])))) || !((({ let ml_reference_value1462 = (match property_2.clone() {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value26919) => {
                                                        source_ml_source2_data_property_value26919.clone()
                                                    }
                                                }).borrow().clone(); ml_reference_value1462 }).0).10))) }) }; ml_values1459.iter().cloned().enumerate().all(|(ml_index1459, ml_value1459)| ml_callback1459(ml_value1459)) });
                                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(Box::new(heap.clone()), Box::new(self::JavaScriptSemanticValue::Boolean(fixed)))
                                        }
                                    } else {
                                        {
                                            let state = self::JavaScriptObjectIntegrityState::ObjectIntegrity(operation.clone(), Box::new(input.clone()), self::ml_string::Text::from_units(vec![112, 114, 101, 118, 101, 110, 116]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new())), 0f64);
                                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityContinue(Box::new(heap.clone()), Box::new(state.clone()))
                                        }
                                    }
                                }
                            }
                        }
                        _ => self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(
                            Box::new(heap.clone()),
                            Box::new(
                                if (operation
                                    == (self::ml_string::Text::from_units(vec![
                                        79, 98, 106, 101, 99, 116, 46, 105, 115, 69, 120, 116, 101,
                                        110, 115, 105, 98, 108, 101,
                                    ])))
                                {
                                    self::JavaScriptSemanticValue::Boolean(false)
                                } else {
                                    if ((operation
                                        == (self::ml_string::Text::from_units(vec![
                                            79, 98, 106, 101, 99, 116, 46, 105, 115, 83, 101, 97,
                                            108, 101, 100,
                                        ])))
                                        || (operation
                                            == (self::ml_string::Text::from_units(vec![
                                                79, 98, 106, 101, 99, 116, 46, 105, 115, 70, 114,
                                                111, 122, 101, 110,
                                            ]))))
                                    {
                                        self::JavaScriptSemanticValue::Boolean(true)
                                    } else {
                                        input.clone()
                                    }
                                },
                            ),
                        ),
                    }
                }
            }
        }
    }
}
