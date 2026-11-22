// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptObjectIntegrityState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptObjectIntegrityStateErrorLoop1201Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_read_java_script_object_integrity_state_error_loop1201;
use super::test_java_script_ordinary_integrity_kind;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_integrity_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectIntegrityState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptObjectIntegrityState::ObjectIntegrity(
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let error = {
                        let source_ml_source2_logical26921 =
                            self::read_java_script_ordinary_heap_error(heap.clone());
                        if !source_ml_source2_logical26921.0.is_empty() {
                            source_ml_source2_logical26921.clone()
                        } else {
                            self::read_java_script_ordinary_value_error(
                                heap.clone(),
                                shared_constructor_field3.clone(),
                            )
                        }
                    };
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        error.clone()
                    } else {
                        {
                            let object = if {
                                let source_ml_source2_s26923 = shared_constructor_field3.clone();
                                match source_ml_source2_s26923.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => true,
                                    _ => false,
                                }
                            } {
                                {
                                    let source_ml_source2_array_read_value26931 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_array_read_index26932 =
                                            self::find_java_script_ordinary_object(
                                                shared_constructor_field1.clone(),
                                                (((match shared_constructor_field3.clone() {
                                                    self::JavaScriptSemanticValue::Object(
                                                        source_ml_source2_data_property_value26930,
                                                        _,
                                                    ) => Some((
                                                        source_ml_source2_data_property_value26930
                                                            .clone(),
                                                    )),
                                                    _ => None::<(self::ml::Big,)>,
                                                })
                                                .clone()
                                                .expect("a proven present value"))
                                                .0)
                                                    .clone(),
                                            );
                                        {
                                            let ml_reference_array1463 =
                                                source_ml_source2_array_read_value26931.clone();
                                            let ml_reference_position1463 =
                                                source_ml_source2_array_read_index26932;
                                            let ml_reference_projection1463 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array1463.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position1463,
                                                    ),
                                                );
                                            ml_reference_projection1463
                                        }
                                    }
                                }
                            } else {
                                None::<self::JavaScriptOrdinaryObject>
                            };
                            if ((((((((!(self::ml_array::includes(
                                &{
                                    let ml_reference_value1464 =
                                        (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            self::ml_string::Text::from_units(vec![
                                                79, 98, 106, 101, 99, 116, 46, 112, 114, 101, 118,
                                                101, 110, 116, 69, 120, 116, 101, 110, 115, 105,
                                                111, 110, 115,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                79, 98, 106, 101, 99, 116, 46, 115, 101, 97, 108,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                79, 98, 106, 101, 99, 116, 46, 102, 114, 101, 101,
                                                122, 101,
                                            ]),
                                        ])))
                                        .borrow()
                                        .clone();
                                    ml_reference_value1464
                                },
                                &shared_constructor_field2.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            )) || match (object.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }) || !(self::test_java_script_ordinary_integrity_kind(
                                &(match object.clone().expect("a proven present value") {
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
                            ))) || !(self::ml_array::includes(
                                &{
                                    let ml_reference_value1465 =
                                        (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            self::ml_string::Text::from_units(vec![
                                                112, 114, 101, 118, 101, 110, 116,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                107, 101, 121, 115,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                100, 101, 102, 105, 110, 101,
                                            ]),
                                        ])))
                                        .borrow()
                                        .clone();
                                    ml_reference_value1465
                                },
                                &shared_constructor_field4.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            ))) || !(self::ml_math::is_safe_integer(
                                shared_constructor_field6,
                            ))) || {
                                let source_ml_source2_same_value_operand26937 =
                                    shared_constructor_field6;
                                {
                                    let source_ml_source2_same_value_operand26938 = (-0f64);
                                    (((source_ml_source2_same_value_operand26937
                                        == source_ml_source2_same_value_operand26938)
                                        && ((source_ml_source2_same_value_operand26937 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand26937)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand26938))))
                                        || ((source_ml_source2_same_value_operand26937
                                            != source_ml_source2_same_value_operand26937)
                                            && (source_ml_source2_same_value_operand26938
                                                != source_ml_source2_same_value_operand26938)))
                                }
                            }) || (shared_constructor_field6 < (0f64)))
                                || (shared_constructor_field6
                                    > ({
                                        let ml_reference_array1466 =
                                            shared_constructor_field5.clone();
                                        let ml_reference_projection1466 =
                                            ml_reference_array1466.borrow().len() as f64;
                                        ml_reference_projection1466
                                    })))
                                || (({
                                    let ml_reference_array1467 = shared_constructor_field5.clone();
                                    let ml_reference_projection1467 =
                                        ml_reference_array1467.borrow().len() as f64;
                                    ml_reference_projection1467
                                }) > (4096f64)))
                            {
                                self::ml_string::Text::from_units(vec![
                                    105, 110, 116, 101, 103, 114, 105, 116, 121, 32, 99, 104, 97,
                                    110, 103, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116,
                                    104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111,
                                    114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99,
                                    116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 112,
                                    104, 97, 115, 101, 32, 97, 110, 100, 32, 98, 111, 117, 110,
                                    100, 101, 100, 32, 107, 101, 121, 32, 112, 111, 115, 105, 116,
                                    105, 111, 110,
                                ])
                            } else {
                                if (((shared_constructor_field4 != (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]))) && ((({ let ml_reference_array1468 = shared_constructor_field5.clone(); let ml_reference_projection1468 = ml_reference_array1468.borrow().len() as f64; ml_reference_projection1468 }) != (0f64)) || (shared_constructor_field6 != (0f64)))) || ((shared_constructor_field4 != (self::ml_string::Text::from_units(vec![112, 114, 101, 118, 101, 110, 116]))) && match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, source_ml_source2_data_property_value26955, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value26955
                                        }
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, source_ml_source2_data_property_value26956, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value26956
                                        }
                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, source_ml_source2_data_property_value26957, _, _) => {
                                            source_ml_source2_data_property_value26957
                                        }
                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, source_ml_source2_data_property_value26958, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value26958
                                        }
                                        self::JavaScriptOrdinaryObject::MapObject(_, _, source_ml_source2_data_property_value26959, _, _, _) => {
                                            source_ml_source2_data_property_value26959
                                        }
                                        self::JavaScriptOrdinaryObject::SetObject(_, _, source_ml_source2_data_property_value26960, _, _, _) => {
                                            source_ml_source2_data_property_value26960
                                        }
                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, source_ml_source2_data_property_value26961, _, _, _) => {
                                            source_ml_source2_data_property_value26961
                                        }
                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, source_ml_source2_data_property_value26962, _, _, _) => {
                                            source_ml_source2_data_property_value26962
                                        }
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value26963, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value26963
                                        }
                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, source_ml_source2_data_property_value26964, _, _, _, _) => {
                                            source_ml_source2_data_property_value26964
                                        }
                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, source_ml_source2_data_property_value26965, _, _, _, _) => {
                                            source_ml_source2_data_property_value26965
                                        }
                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, source_ml_source2_data_property_value26966, _, _, _, _) => {
                                            source_ml_source2_data_property_value26966
                                        }
                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, source_ml_source2_data_property_value26967, _, _, _, _) => {
                                            source_ml_source2_data_property_value26967
                                        }
                                    })) {
                                        self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 114, 105, 116, 121, 32, 111, 119, 110, 45, 107, 101, 121, 32, 112, 114, 111, 99, 101, 115, 115, 105, 110, 103, 32, 102, 111, 108, 108, 111, 119, 115, 32, 97, 99, 116, 117, 97, 108, 32, 80, 114, 101, 118, 101, 110, 116, 69, 120, 116, 101, 110, 115, 105, 111, 110, 115])
                                    } else {
                                        {
                                            let source_ml_source1_temporary_values1200 = shared_constructor_field5.clone();
                                            {
                                                let source_ml_source1_temporary_index1200 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s26969 = self::source_ml_source1_temporary_read_java_script_object_integrity_state_error_loop1201(heap.clone(), source_ml_source1_temporary_values1200.clone(), source_ml_source1_temporary_index1200.clone());
                                                    match source_ml_source2_s26969.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptObjectIntegrityStateErrorLoop1201Result::MlSource1TemporaryReadJavaScriptObjectIntegrityStateErrorLoop1201Done => {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptObjectIntegrityStateErrorLoop1201Result::MlSource1TemporaryReadJavaScriptObjectIntegrityStateErrorLoop1201Return(source_ml_source1_temporary_result1201) => {
                                                            source_ml_source1_temporary_result1201.clone()
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
}
