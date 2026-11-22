// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::JavaScriptWeakCollectionEntry;
use super::find_java_script_heap_directory_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::source_ml_source3_record_order;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_fresh_collection_construction_receiver(
    heap: self::JavaScriptOrdinaryHeap,
    kind: &self::ml_string::Text,
    identity: self::ml::Big,
    receiver: self::JavaScriptSemanticValue,
    prototype: self::JavaScriptSemanticValue,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (self::ml_string::Text, Option<self::JavaScriptSemanticValue>),
        Vec<f64>,
    )>,
> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let object = {
                let source_ml_source2_array_read_value32459 = shared_constructor_field1.clone();
                {
                    let source_ml_source2_array_read_index32460 =
                        self::find_java_script_heap_directory_object(
                            shared_constructor_field1.clone(),
                            identity.clone(),
                        );
                    {
                        let ml_reference_array2005 =
                            source_ml_source2_array_read_value32459.clone();
                        let ml_reference_position2005 = source_ml_source2_array_read_index32460;
                        let ml_reference_projection2005 = self::ml_array::optional_at(
                            &*ml_reference_array2005.borrow(),
                            self::ml_array::number_index(ml_reference_position2005),
                        );
                        ml_reference_projection2005
                    }
                }
            };
            {
                let brand = if ((*kind) == (self::ml_string::Text::from_units(vec![77, 97, 112]))) {
                    self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                } else {
                    if ((*kind) == (self::ml_string::Text::from_units(vec![83, 101, 116]))) {
                        self::ml_string::Text::from_units(vec![
                            115, 101, 116, 79, 98, 106, 101, 99, 116,
                        ])
                    } else {
                        if ((*kind)
                            == (self::ml_string::Text::from_units(vec![
                                87, 101, 97, 107, 77, 97, 112,
                            ])))
                        {
                            self::ml_string::Text::from_units(vec![
                                119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116,
                            ])
                        } else {
                            self::ml_string::Text::from_units(vec![
                                119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116,
                            ])
                        }
                    }
                };
                {
                    let empty = if (!(match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && (match object.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => true,
                        _ => false,
                    } || match object
                        .clone()
                        .expect("a proven present value")
                    {
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => true,
                        _ => false,
                    })) {
                        (({
                            let ml_reference_array2006 = (((match object
                                .clone()
                                .expect("a proven present value")
                            {
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value32475,
                                ) => Some((source_ml_source2_data_property_value32475.clone(),)),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value32476,
                                ) => Some((source_ml_source2_data_property_value32476.clone(),)),
                                _ => {
                                    None::<(
                                        std::rc::Rc<
                                            std::cell::RefCell<
                                                Vec<self::JavaScriptWeakCollectionEntry>,
                                            >,
                                        >,
                                    )>
                                }
                            })
                            .clone()
                            .expect("a proven present value"))
                            .0)
                                .clone();
                            let ml_reference_projection2006 =
                                ml_reference_array2006.borrow().len() as f64;
                            ml_reference_projection2006
                        }) == (0f64))
                    } else {
                        if (!(match (object.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }) && (match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => true,
                            _ => false,
                        } || match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => true,
                            _ => false,
                        })) {
                            (({
                                let ml_reference_array2007 =
                                    (((match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::MapObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value32491,
                                        ) => Some((
                                            source_ml_source2_data_property_value32491.clone(),
                                        )),
                                        self::JavaScriptOrdinaryObject::SetObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value32492,
                                        ) => Some((
                                            source_ml_source2_data_property_value32492.clone(),
                                        )),
                                        _ => {
                                            None::<(
                                                std::rc::Rc<
                                                    std::cell::RefCell<
                                                        Vec<self::JavaScriptCollectionEntry>,
                                                    >,
                                                >,
                                            )>
                                        }
                                    })
                                    .clone()
                                    .expect("a proven present value"))
                                    .0)
                                        .clone();
                                let ml_reference_projection2007 =
                                    ml_reference_array2007.borrow().len() as f64;
                                ml_reference_projection2007
                            }) == (0f64))
                        } else {
                            false
                        }
                    };
                    if (((match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } || ((match object.clone().expect("a proven present value") {
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
                            98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98,
                            106, 101, 99, 116,
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
                            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97,
                            116, 111, 114, 79, 98, 106, 101, 99, 116,
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
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                            self::ml_string::Text::from_units(vec![
                                119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116,
                            ])
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                            self::ml_string::Text::from_units(vec![
                                119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116,
                            ])
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
                        ) => self::ml_string::Text::from_units(vec![
                            111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116,
                        ]),
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                            self::ml_string::Text::from_units(vec![
                                104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99,
                                116,
                            ])
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                            self::ml_string::Text::from_units(vec![
                                104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116,
                            ])
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                            self::ml_string::Text::from_units(vec![
                                104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99,
                                116,
                            ])
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => self::ml_string::Text::from_units(vec![
                            104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79,
                            98, 106, 101, 99, 116,
                        ]),
                    }) != brand))
                        || !empty)
                        || !(self::test_java_script_semantic_same_value(
                            match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    source_ml_source2_data_property_value32493,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value32493 =
                                        *source_ml_source2_data_property_value32493;
                                    source_ml_source2_data_property_value32493.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    source_ml_source2_data_property_value32494,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value32494 =
                                        *source_ml_source2_data_property_value32494;
                                    source_ml_source2_data_property_value32494.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    _,
                                    source_ml_source2_data_property_value32495,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value32495 =
                                        *source_ml_source2_data_property_value32495;
                                    source_ml_source2_data_property_value32495.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    source_ml_source2_data_property_value32496,
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
                                    let source_ml_source2_data_property_value32496 =
                                        *source_ml_source2_data_property_value32496;
                                    source_ml_source2_data_property_value32496.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(
                                    _,
                                    source_ml_source2_data_property_value32497,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value32497 =
                                        *source_ml_source2_data_property_value32497;
                                    source_ml_source2_data_property_value32497.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(
                                    _,
                                    source_ml_source2_data_property_value32498,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value32498 =
                                        *source_ml_source2_data_property_value32498;
                                    source_ml_source2_data_property_value32498.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    source_ml_source2_data_property_value32499,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value32499 =
                                        *source_ml_source2_data_property_value32499;
                                    source_ml_source2_data_property_value32499.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    source_ml_source2_data_property_value32500,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value32500 =
                                        *source_ml_source2_data_property_value32500;
                                    source_ml_source2_data_property_value32500.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    source_ml_source2_data_property_value32501,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value32501 =
                                        *source_ml_source2_data_property_value32501;
                                    source_ml_source2_data_property_value32501.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    source_ml_source2_data_property_value32502,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value32502 =
                                        *source_ml_source2_data_property_value32502;
                                    source_ml_source2_data_property_value32502.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    source_ml_source2_data_property_value32503,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value32503 =
                                        *source_ml_source2_data_property_value32503;
                                    source_ml_source2_data_property_value32503.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    source_ml_source2_data_property_value32504,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value32504 =
                                        *source_ml_source2_data_property_value32504;
                                    source_ml_source2_data_property_value32504.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    source_ml_source2_data_property_value32505,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value32505 =
                                        *source_ml_source2_data_property_value32505;
                                    source_ml_source2_data_property_value32505.clone()
                                }
                            },
                            prototype.clone(),
                        )))
                    {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_field32506 =
                                None::<self::JavaScriptSemanticValue>;
                            {
                                let source_ml_source3_record_field465 =
                                    self::ml_string::Text::from_units(vec![
                                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105,
                                        110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 114,
                                        101, 117, 115, 101, 32, 114, 101, 113, 117, 105, 114, 101,
                                        115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                        102, 114, 101, 115, 104, 32, 101, 109, 112, 116, 121, 32,
                                        105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111,
                                        116, 115, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99,
                                        116, 101, 100, 32, 110, 101, 119, 84, 97, 114, 103, 101,
                                        116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ]);
                                {
                                    let source_ml_source3_record_field466 =
                                        source_ml_source2_field32506.clone();
                                    ((source_ml_source3_record_field465.clone(), source_ml_source3_record_field466.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64]], vec![1f64, 0f64]))
                                }
                            }
                        }))
                    } else {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_field32508 = Some(receiver.clone());
                            {
                                let source_ml_source3_record_field467 =
                                    self::ml_string::Text::from_units(vec![]);
                                {
                                    let source_ml_source3_record_field468 =
                                        source_ml_source2_field32508.clone();
                                    ((source_ml_source3_record_field467.clone(), source_ml_source3_record_field468.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64]], vec![1f64, 0f64]))
                                }
                            }
                        }))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_text_protocol_operation_error(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
            116, 101, 120, 116,
        ])))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101,
            113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
            114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32,
            111, 112, 101, 114, 97, 116, 105, 111, 110,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_arity(operation: &self::ml_string::Text) -> f64 {
    if ((((*operation)
        == (self::ml_string::Text::from_units(vec![
            110, 101, 103, 97, 116, 101, 32, 105, 110, 116, 101, 103, 101, 114,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 105, 110, 116, 101, 103, 101,
                114, 32, 98, 105, 116, 115,
            ]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 110, 117, 109, 98, 101, 114,
                32, 98, 105, 116, 115,
            ]))))
    {
        1f64
    } else {
        if self::ml_array::includes(
            &{
                let ml_reference_value2008 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    self::ml_string::Text::from_units(vec![
                        99, 111, 110, 118, 101, 114, 116, 32, 105, 110, 116, 101, 103, 101, 114,
                        32, 114, 97, 100, 105, 120, 32, 116, 101, 120, 116,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        119, 114, 97, 112, 32, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116,
                        101, 103, 101, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        119, 114, 97, 112, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105,
                        110, 116, 101, 103, 101, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114,
                        115,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101,
                        114, 115,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101,
                        109, 97, 105, 110, 100, 101, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112,
                        111, 119, 101, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114, 32, 105,
                        110, 116, 101, 103, 101, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116, 32, 105, 110, 116,
                        101, 103, 101, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        99, 111, 110, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114,
                        32, 98, 105, 116, 115,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        100, 105, 115, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114,
                        32, 98, 105, 116, 115,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 105, 110,
                        116, 101, 103, 101, 114, 32, 98, 105, 116, 115,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98,
                        105, 116, 115, 32, 108, 101, 102, 116,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98,
                        105, 116, 115, 32, 114, 105, 103, 104, 116,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        99, 111, 110, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98,
                        105, 116, 115,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        100, 105, 115, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98,
                        105, 116, 115,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 110, 117,
                        109, 98, 101, 114, 32, 98, 105, 116, 115,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116,
                        115, 32, 108, 101, 102, 116,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116,
                        115, 32, 114, 105, 103, 104, 116,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116,
                        115, 32, 114, 105, 103, 104, 116, 32, 117, 110, 115, 105, 103, 110, 101,
                        100,
                    ]),
                ])))
                .borrow()
                .clone();
                ml_reference_value2008
            },
            operation,
            0f64,
            false,
            |a, b| a == b,
        ) {
            2f64
        } else {
            0f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_numeric_parsing_arity(operation: &self::ml_string::Text) -> f64 {
    if ((*operation)
        == (self::ml_string::Text::from_units(vec![
            112, 97, 114, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 116, 101, 120, 116,
            32, 112, 114, 101, 102, 105, 120,
        ])))
    {
        2f64
    } else {
        if ((*operation)
            == (self::ml_string::Text::from_units(vec![
                112, 97, 114, 115, 101, 32, 102, 108, 111, 97, 116, 32, 116, 101, 120, 116, 32,
                112, 114, 101, 102, 105, 120,
            ])))
        {
            1f64
        } else {
            (-1f64)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_method_primitive_arity(operation: &self::ml_string::Text) -> f64 {
    if self::ml_array::includes(
        &{
            let ml_reference_value2009 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![
                    116, 101, 115, 116, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100,
                    32, 83, 116, 114, 105, 110, 103,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 111, 112, 121, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100,
                    32, 83, 116, 114, 105, 110, 103,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 111, 110, 118, 101, 114, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114,
                    32, 99, 111, 100, 101, 32, 116, 101, 120, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 111, 110, 118, 101, 114, 116, 32, 99, 111, 100, 101, 32, 112, 111, 105,
                    110, 116, 32, 116, 101, 120, 116,
                ]),
            ])))
            .borrow()
            .clone();
            ml_reference_value2009
        },
        operation,
        0f64,
        false,
        |a, b| a == b,
    ) {
        1f64
    } else {
        if self::ml_array::includes(
            &{
                let ml_reference_value2010 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    self::ml_string::Text::from_units(vec![
                        109, 97, 112, 32, 108, 111, 119, 101, 114, 32, 83, 116, 114, 105, 110, 103,
                        32, 99, 97, 115, 101,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        109, 97, 112, 32, 117, 112, 112, 101, 114, 32, 83, 116, 114, 105, 110, 103,
                        32, 99, 97, 115, 101,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        114, 101, 113, 117, 105, 114, 101, 32, 83, 116, 114, 105, 110, 103, 32,
                        114, 101, 99, 101, 105, 118, 101, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        114, 101, 97, 100, 32, 83, 116, 114, 105, 110, 103, 32, 99, 111, 100, 101,
                        32, 117, 110, 105, 116,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32,
                        107, 101, 121,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        114, 101, 97, 100, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32,
                        107, 101, 121,
                    ]),
                ])))
                .borrow()
                .clone();
                ml_reference_value2010
            },
            operation,
            0f64,
            false,
            |a, b| a == b,
        ) {
            2f64
        } else {
            if self::ml_array::includes(
                &{
                    let ml_reference_value2011 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                        self::ml_string::Text::from_units(vec![
                            115, 117, 98, 115, 116, 114, 105, 110, 103, 32, 83, 116, 114, 105, 110,
                            103, 32, 117, 110, 105, 116, 115,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            115, 117, 98, 115, 116, 114, 32, 83, 116, 114, 105, 110, 103, 32, 117,
                            110, 105, 116, 115,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 112, 114, 101,
                            102, 105, 120, 32, 97, 116,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 115, 117, 102,
                            102, 105, 120, 32, 97, 116,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 105, 110, 99,
                            108, 117, 115, 105, 111, 110, 32, 97, 116,
                        ]),
                    ])))
                    .borrow()
                    .clone();
                    ml_reference_value2011
                },
                operation,
                0f64,
                false,
                |a, b| a == b,
            ) {
                3f64
            } else {
                (-1f64)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_unary_number_operation(operation: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &{
            let ml_reference_value2012 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![
                    116, 101, 115, 116, 32, 78, 97, 78, 32, 110, 117, 109, 98, 101, 114,
                ]),
                self::ml_string::Text::from_units(vec![
                    116, 101, 115, 116, 32, 102, 105, 110, 105, 116, 101, 32, 110, 117, 109, 98,
                    101, 114,
                ]),
                self::ml_string::Text::from_units(vec![
                    110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114,
                ]),
                self::ml_string::Text::from_units(vec![
                    116, 114, 117, 110, 99, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114,
                ]),
                self::ml_string::Text::from_units(vec![
                    114, 101, 97, 100, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 110, 117, 109,
                    98, 101, 114,
                ]),
                self::ml_string::Text::from_units(vec![
                    102, 108, 111, 111, 114, 32, 110, 117, 109, 98, 101, 114,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 101, 105, 108, 32, 110, 117, 109, 98, 101, 114,
                ]),
                self::ml_string::Text::from_units(vec![
                    114, 111, 117, 110, 100, 32, 110, 117, 109, 98, 101, 114,
                ]),
                self::ml_string::Text::from_units(vec![
                    114, 101, 97, 100, 32, 110, 117, 109, 98, 101, 114, 32, 115, 105, 103, 110,
                ]),
                self::ml_string::Text::from_units(vec![
                    114, 101, 97, 100, 32, 115, 113, 117, 97, 114, 101, 32, 114, 111, 111, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    116, 101, 115, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109,
                    98, 101, 114,
                ]),
                self::ml_string::Text::from_units(vec![
                    116, 101, 115, 116, 32, 115, 97, 102, 101, 32, 105, 110, 116, 101, 103, 101,
                    114, 32, 110, 117, 109, 98, 101, 114,
                ]),
            ])))
            .borrow()
            .clone();
            ml_reference_value2012
        },
        operation,
        0f64,
        false,
        |a, b| a == b,
    )
}
