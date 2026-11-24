// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptObjectIntegrityTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::find_java_script_ordinary_object;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::observe_java_script_object_properties;
use super::select_java_script_object_observation_primitive;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn observe_java_script_boxed_primitive(
    heap: self::JavaScriptOrdinaryHeap,
    operation: &self::ml_string::Text,
    input: self::JavaScriptSemanticValue,
    other: self::JavaScriptSemanticValue,
) -> self::JavaScriptObjectIntegrityTransition {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            if self::ml_array::includes(
                &{
                    let ml_reference_value1445 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                        self::ml_string::Text::from_units(vec![
                            83, 121, 109, 98, 111, 108, 46, 118, 97, 108, 117, 101, 79, 102,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            78, 117, 109, 98, 101, 114, 46, 118, 97, 108, 117, 101, 79, 102,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            66, 111, 111, 108, 101, 97, 110, 46, 118, 97, 108, 117, 101, 79, 102,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            66, 111, 111, 108, 101, 97, 110, 46, 116, 111, 83, 116, 114, 105, 110,
                            103,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            66, 105, 103, 73, 110, 116, 46, 118, 97, 108, 117, 101, 79, 102,
                        ]),
                    ])))
                    .borrow()
                    .clone();
                    ml_reference_value1445
                },
                operation,
                0f64,
                false,
                |a, b| a == b,
            ) {
                {
                    let kind = if ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            83, 121, 109, 98, 111, 108, 46, 118, 97, 108, 117, 101, 79, 102,
                        ]))) {
                        self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
                    } else {
                        if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                78, 117, 109, 98, 101, 114, 46, 118, 97, 108, 117, 101, 79, 102,
                            ])))
                        {
                            self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                        } else {
                            if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    66, 105, 103, 73, 110, 116, 46, 118, 97, 108, 117, 101, 79, 102,
                                ])))
                            {
                                self::ml_string::Text::from_units(vec![
                                    105, 110, 116, 101, 103, 101, 114,
                                ])
                            } else {
                                self::ml_string::Text::from_units(vec![
                                    98, 111, 111, 108, 101, 97, 110,
                                ])
                            }
                        }
                    };
                    {
                        let object = match input.clone() {
                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                let source_ml_source2_array_read_value26749 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_array_read_index26750 =
                                        self::find_java_script_ordinary_object(
                                            shared_constructor_field1.clone(),
                                            identity.clone(),
                                        );
                                    {
                                        let ml_reference_array1446 =
                                            source_ml_source2_array_read_value26749.clone();
                                        let ml_reference_position1446 =
                                            source_ml_source2_array_read_index26750;
                                        let ml_reference_projection1446 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array1446.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position1446,
                                                ),
                                            );
                                        ml_reference_projection1446
                                    }
                                }
                            }
                            _ => None::<self::JavaScriptOrdinaryObject>,
                        };
                        {
                            let primitive =
                                if (!(match (object.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) && match object.clone().expect("a proven present value") {
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
                                    ) => true,
                                    _ => false,
                                }) {
                                    (((match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value26760,
                                            _,
                                            _,
                                        ) => Some((((source_ml_source2_data_property_value26760
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptSemanticValue>,)
                                            }))
                                        .0)
                                            .clone(),)),
                                        _ => None::<(Option<self::JavaScriptSemanticValue>,)>,
                                    })
                                    .clone()
                                    .expect("a proven present value"))
                                    .0)
                                        .clone()
                                } else {
                                    None::<self::JavaScriptSemanticValue>
                                };
                            {
                                let value = self::select_java_script_object_observation_primitive(
                                    input.clone(),
                                    primitive.clone(),
                                    &(kind),
                                );
                                if match (value.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    self::JavaScriptObjectIntegrityTransition::ObjectIntegrityError(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            84, 121, 112, 101, 69, 114, 114, 111, 114,
                                        ]),
                                        operation.concat(&self::ml_string::Text::from_units(vec![
                                            32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114,
                                            105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108,
                                            117, 101, 32, 111, 114, 32, 105, 110, 116, 101, 114,
                                            110, 97, 108, 32, 100, 97, 116, 97, 32, 115, 108, 111,
                                            116,
                                        ])),
                                    )
                                } else {
                                    self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(Box::new(heap.clone()), Box::new(if ((*operation) == (self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110, 46, 116, 111, 83, 116, 114, 105, 110, 103]))) {
                                        self::JavaScriptSemanticValue::Text(if (match value.clone().expect("a proven present value") {
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                true
                                            }
                                            _ => {
                                                false
                                            }
                                        } && ((match value.clone().expect("a proven present value") {
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_data_property_value26762) => {
                                                Some((source_ml_source2_data_property_value26762,))
                                            }
                                            _ => {
                                                None::<(bool,)>
                                            }
                                        }).clone().expect("a proven present value")).0) {
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![116f64, 114f64, 117f64, 101f64]))
                                        } else {
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![102f64, 97f64, 108f64, 115f64, 101f64]))
                                        })
                                    } else {
                                        value.clone().expect("a proven present value")
                                    }))
                                }
                            }
                        }
                    }
                }
            } else {
                if (((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        83, 116, 114, 105, 110, 103, 46, 116, 111, 83, 116, 114, 105, 110, 103,
                    ])))
                    || ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            83, 116, 114, 105, 110, 103, 46, 118, 97, 108, 117, 101, 79, 102,
                        ]))))
                {
                    if (match input.clone() {
                        self::JavaScriptSemanticValue::Text(_) => true,
                        _ => false,
                    } || match input.clone() {
                        self::JavaScriptSemanticValue::SourceText(_, _) => true,
                        _ => false,
                    }) {
                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(
                            Box::new(heap.clone()),
                            Box::new(input.clone()),
                        )
                    } else {
                        {
                            let object_2 = match input.clone() {
                                self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                    let source_ml_source2_array_read_value26769 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_array_read_index26770 =
                                            self::find_java_script_ordinary_object(
                                                shared_constructor_field1.clone(),
                                                identity_2.clone(),
                                            );
                                        {
                                            let ml_reference_array1447 =
                                                source_ml_source2_array_read_value26769.clone();
                                            let ml_reference_position1447 =
                                                source_ml_source2_array_read_index26770;
                                            let ml_reference_projection1447 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array1447.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position1447,
                                                    ),
                                                );
                                            ml_reference_projection1447
                                        }
                                    }
                                }
                                _ => None::<self::JavaScriptOrdinaryObject>,
                            };
                            {
                                let data = if {
                                    let source_ml_source2_comparison_operand26772 = {
                                        let source_ml_source2_optional_chain_receiver26771 =
                                            (object_2.clone(),);
                                        if match (source_ml_source2_optional_chain_receiver26771
                                            .clone())
                                        .0
                                        {
                                            Some(ml_present) => {
                                                let _ = ml_present.clone();
                                                true
                                            }
                                            None => false,
                                        } {
                                            Some({
                                                let source_ml_source1_temporary_optional_chain1198 = ((source_ml_source2_optional_chain_receiver26771.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source1_temporary_optional_chain1198.clone() {
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
                                                }
                                            })
                                        } else {
                                            None::<self::ml_string::Text>
                                        }
                                    };
                                    match (
                                        source_ml_source2_comparison_operand26772.clone(),
                                        self::ml_string::Text::from_units(vec![
                                            111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106,
                                            101, 99, 116,
                                        ]),
                                    ) {
                                        (Some(ml_left), ml_right) => ml_left == ml_right,
                                        _ => false,
                                    }
                                } {
                                    {
                                        let source_ml_source2_nullish_field_receiver26774 =
                                            (object_2.clone(),);
                                        if match (source_ml_source2_nullish_field_receiver26774
                                            .clone())
                                        .0
                                        {
                                            Some(ml_present) => {
                                                let _ = ml_present.clone();
                                                true
                                            }
                                            None => false,
                                        } {
                                            {
                                                let source_ml_source2_nullish_field_present26775 = ((source_ml_source2_nullish_field_receiver26774.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source2_nullish_field_present26775.clone() {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, source_ml_source2_data_property_value26776, _, _, _) => {
                                                        ((source_ml_source2_data_property_value26776.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                    }
                                                    _ => {
                                                        None::<self::JavaScriptSemanticValue>
                                                    }
                                                }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException {
                                                class_name: String::from("TypeError"),
                                                message_units: (self::ml_string::Text::from_units(
                                                    vec![
                                                        67, 97, 110, 110, 111, 116, 32, 114, 101,
                                                        97, 100, 32, 112, 114, 111, 112, 101, 114,
                                                        116, 105, 101, 115, 32, 111, 102, 32, 117,
                                                        110, 100, 101, 102, 105, 110, 101, 100, 32,
                                                        40, 114, 101, 97, 100, 105, 110, 103, 32,
                                                        39, 115, 116, 114, 105, 110, 103, 68, 97,
                                                        116, 97, 39, 41,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
                                        }
                                    }
                                } else {
                                    None::<self::JavaScriptSemanticValue>
                                };
                                if !(match (data.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(Box::new(heap.clone()), Box::new(data.clone().expect("a proven present value")))
                                } else {
                                    self::JavaScriptObjectIntegrityTransition::ObjectIntegrityError(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            84, 121, 112, 101, 69, 114, 114, 111, 114,
                                        ]),
                                        ((self::ml_string::Text::from_units(vec![
                                            83, 116, 114, 105, 110, 103, 46, 112, 114, 111, 116,
                                            111, 116, 121, 112, 101, 46,
                                        ]))
                                        .concat(
                                            &if ((*operation)
                                                == (self::ml_string::Text::from_units(vec![
                                                    83, 116, 114, 105, 110, 103, 46, 116, 111, 83,
                                                    116, 114, 105, 110, 103,
                                                ])))
                                            {
                                                self::ml_string::Text::from_units(vec![
                                                    116, 111, 83, 116, 114, 105, 110, 103,
                                                ])
                                            } else {
                                                self::ml_string::Text::from_units(vec![
                                                    118, 97, 108, 117, 101, 79, 102,
                                                ])
                                            },
                                        ))
                                        .concat(
                                            &self::ml_string::Text::from_units(vec![
                                                32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                                                116, 104, 97, 116, 32, 39, 116, 104, 105, 115, 39,
                                                32, 98, 101, 32, 97, 32, 83, 116, 114, 105, 110,
                                                103,
                                            ]),
                                        ),
                                    )
                                }
                            }
                        }
                    }
                } else {
                    if ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            79, 98, 106, 101, 99, 116, 46, 105, 115,
                        ])))
                    {
                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(
                            Box::new(heap.clone()),
                            Box::new(self::JavaScriptSemanticValue::Boolean(
                                self::test_java_script_semantic_same_value(
                                    input.clone(),
                                    other.clone(),
                                ),
                            )),
                        )
                    } else {
                        self::observe_java_script_object_properties(
                            heap.clone(),
                            operation.clone(),
                            input.clone(),
                            other.clone(),
                        )
                    }
                }
            }
        }
    }
}
