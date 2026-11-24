// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Result;
use super::create_java_script_property_descriptor;
use super::find_java_script_array_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_length_key;
use super::read_java_script_array_length_value;
use super::source_ml_source1_temporary_read_java_script_array_object_error_loop15;
use super::test_java_script_array_length;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_object_error(
    object: self::JavaScriptOrdinaryObject,
) -> self::ml_string::Text {
    if match object.clone() {
        self::JavaScriptOrdinaryObject::ClosureObject(
            _,
            _,
            _,
            source_ml_source2_data_property_value164,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => source_ml_source2_data_property_value164,
        self::JavaScriptOrdinaryObject::BoundFunctionObject(
            _,
            _,
            _,
            source_ml_source2_data_property_value165,
            _,
            _,
            _,
            _,
        ) => source_ml_source2_data_property_value165,
        self::JavaScriptOrdinaryObject::ArrayObject(
            _,
            _,
            _,
            source_ml_source2_data_property_value166,
            _,
        ) => source_ml_source2_data_property_value166,
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
            _,
            _,
            _,
            source_ml_source2_data_property_value167,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => source_ml_source2_data_property_value167,
        self::JavaScriptOrdinaryObject::MapObject(
            _,
            _,
            _,
            source_ml_source2_data_property_value168,
            _,
            _,
        ) => source_ml_source2_data_property_value168,
        self::JavaScriptOrdinaryObject::SetObject(
            _,
            _,
            _,
            source_ml_source2_data_property_value169,
            _,
            _,
        ) => source_ml_source2_data_property_value169,
        self::JavaScriptOrdinaryObject::WeakMapObject(
            _,
            _,
            _,
            source_ml_source2_data_property_value170,
            _,
            _,
        ) => source_ml_source2_data_property_value170,
        self::JavaScriptOrdinaryObject::WeakSetObject(
            _,
            _,
            _,
            source_ml_source2_data_property_value171,
            _,
            _,
        ) => source_ml_source2_data_property_value171,
        self::JavaScriptOrdinaryObject::OrdinaryObject(
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            source_ml_source2_data_property_value172,
            _,
        ) => source_ml_source2_data_property_value172,
        self::JavaScriptOrdinaryObject::HostErrorObject(
            _,
            _,
            _,
            source_ml_source2_data_property_value173,
            _,
            _,
            _,
        ) => source_ml_source2_data_property_value173,
        self::JavaScriptOrdinaryObject::HostURLObject(
            _,
            _,
            _,
            source_ml_source2_data_property_value174,
            _,
            _,
            _,
        ) => source_ml_source2_data_property_value174,
        self::JavaScriptOrdinaryObject::HostArrayObject(
            _,
            _,
            _,
            source_ml_source2_data_property_value175,
            _,
            _,
            _,
        ) => source_ml_source2_data_property_value175,
        self::JavaScriptOrdinaryObject::HostByteBufferObject(
            _,
            _,
            _,
            source_ml_source2_data_property_value176,
            _,
            _,
            _,
        ) => source_ml_source2_data_property_value176,
    } {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 65, 114, 114, 97, 121, 32, 101, 120, 111, 116, 105, 99, 32, 111, 98, 106,
            101, 99, 116, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101,
        ])
    } else {
        {
            let property = {
                let source_ml_source2_array_read_value203 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value177,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value177.clone(),
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value178,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value178.clone(),
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value179,
                    ) => source_ml_source2_data_property_value179.clone(),
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value180,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value180.clone(),
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value181,
                        _,
                    ) => source_ml_source2_data_property_value181.clone(),
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value182,
                        _,
                    ) => source_ml_source2_data_property_value182.clone(),
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value183,
                        _,
                    ) => source_ml_source2_data_property_value183.clone(),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value184,
                        _,
                    ) => source_ml_source2_data_property_value184.clone(),
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value185,
                    ) => source_ml_source2_data_property_value185.clone(),
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value186,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value186.clone(),
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value187,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value187.clone(),
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value188,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value188.clone(),
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value189,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value189.clone(),
                };
                {
                    let source_ml_source2_array_read_index204 =
                        self::find_java_script_array_property(
                            match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value190,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value190.clone(),
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value191,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value191.clone(),
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value192,
                                ) => source_ml_source2_data_property_value192.clone(),
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value193,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value193.clone(),
                                self::JavaScriptOrdinaryObject::MapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value194,
                                    _,
                                ) => source_ml_source2_data_property_value194.clone(),
                                self::JavaScriptOrdinaryObject::SetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value195,
                                    _,
                                ) => source_ml_source2_data_property_value195.clone(),
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value196,
                                    _,
                                ) => source_ml_source2_data_property_value196.clone(),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value197,
                                    _,
                                ) => source_ml_source2_data_property_value197.clone(),
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value198,
                                ) => source_ml_source2_data_property_value198.clone(),
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value199,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value199.clone(),
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value200,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value200.clone(),
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value201,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value201.clone(),
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value202,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value202.clone(),
                            },
                            self::read_java_script_array_length_key(),
                        );
                    {
                        let ml_reference_array135 = source_ml_source2_array_read_value203.clone();
                        let ml_reference_position135 = source_ml_source2_array_read_index204;
                        let ml_reference_projection135 = self::ml_array::optional_at(
                            &*ml_reference_array135.borrow(),
                            self::ml_array::number_index(ml_reference_position135),
                        );
                        ml_reference_projection135
                    }
                }
            };
            if (((((match (property.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } || !((({
                let ml_reference_value136 =
                    (match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            _,
                            source_ml_source2_data_property_value205,
                        ) => source_ml_source2_data_property_value205.clone(),
                    })
                    .borrow()
                    .clone();
                ml_reference_value136
            })
            .0)
                .9))
                || (({
                    let ml_reference_value137 =
                        (match property.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                _,
                                source_ml_source2_data_property_value206,
                            ) => source_ml_source2_data_property_value206.clone(),
                        })
                        .borrow()
                        .clone();
                    ml_reference_value137
                })
                .0)
                    .5)
                || (({
                    let ml_reference_value138 =
                        (match property.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                _,
                                source_ml_source2_data_property_value207,
                            ) => source_ml_source2_data_property_value207.clone(),
                        })
                        .borrow()
                        .clone();
                    ml_reference_value138
                })
                .0)
                    .7)
                || (({
                    let ml_reference_value139 =
                        (match property.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                _,
                                source_ml_source2_data_property_value208,
                            ) => source_ml_source2_data_property_value208.clone(),
                        })
                        .borrow()
                        .clone();
                    ml_reference_value139
                })
                .0)
                    .2)
                || (({
                    let ml_reference_value140 =
                        (match property.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                _,
                                source_ml_source2_data_property_value209,
                            ) => source_ml_source2_data_property_value209.clone(),
                        })
                        .borrow()
                        .clone();
                    ml_reference_value140
                })
                .0)
                    .0)
            {
                self::ml_string::Text::from_units(vec![
                    97, 110, 32, 65, 114, 114, 97, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 101, 110, 117,
                    109, 101, 114, 97, 98, 108, 101, 32, 110, 111, 110, 99, 111, 110, 102, 105,
                    103, 117, 114, 97, 98, 108, 101, 32, 105, 110, 116, 101, 103, 114, 97, 108, 32,
                    108, 101, 110, 103, 116, 104, 32, 100, 97, 116, 97, 32, 112, 114, 111, 112,
                    101, 114, 116, 121,
                ])
            } else {
                {
                    let length = self::read_java_script_array_length_value(
                        (({
                            let ml_reference_value141 =
                                (match property.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        _,
                                        source_ml_source2_data_property_value210,
                                    ) => source_ml_source2_data_property_value210.clone(),
                                })
                                .borrow()
                                .clone();
                            ml_reference_value141
                        })
                        .0)
                            .8,
                    );
                    if !(self::test_java_script_array_length(length)) {
                        self::ml_string::Text::from_units(vec![
                            97, 110, 32, 65, 114, 114, 97, 121, 32, 114, 101, 116, 97, 105, 110,
                            115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111,
                            110, 101, 110, 117, 109, 101, 114, 97, 98, 108, 101, 32, 110, 111, 110,
                            99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101, 32, 105, 110,
                            116, 101, 103, 114, 97, 108, 32, 108, 101, 110, 103, 116, 104, 32, 100,
                            97, 116, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121,
                        ])
                    } else {
                        {
                            let source_ml_source1_temporary_values14 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value211,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value211.clone(),
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value212,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value212.clone(),
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value213,
                                ) => source_ml_source2_data_property_value213.clone(),
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value214,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value214.clone(),
                                self::JavaScriptOrdinaryObject::MapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value215,
                                    _,
                                ) => source_ml_source2_data_property_value215.clone(),
                                self::JavaScriptOrdinaryObject::SetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value216,
                                    _,
                                ) => source_ml_source2_data_property_value216.clone(),
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value217,
                                    _,
                                ) => source_ml_source2_data_property_value217.clone(),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value218,
                                    _,
                                ) => source_ml_source2_data_property_value218.clone(),
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value219,
                                ) => source_ml_source2_data_property_value219.clone(),
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value220,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value220.clone(),
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value221,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value221.clone(),
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value222,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value222.clone(),
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value223,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value223.clone(),
                            };
                            {
                                let source_ml_source1_temporary_index14 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s224 = self::source_ml_source1_temporary_read_java_script_array_object_error_loop15(length, source_ml_source1_temporary_values14.clone(), source_ml_source1_temporary_index14.clone());
                                    match source_ml_source2_s224.clone() {
                                        self::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Result::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Done => {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                        self::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Result::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop15Return(source_ml_source1_temporary_result15) => {
                                            source_ml_source1_temporary_result15.clone()
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
pub fn construct_java_script_array_object(
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
    length: f64,
) -> self::JavaScriptOrdinaryObject {
    {
        let descriptor = self::create_java_script_property_descriptor(
            &(self::ml_string::Text::from_units(vec![100, 97, 116, 97])),
            true,
            false,
        );
        {
            let source_ml_source1_temporary_construct_java_script_array_object_written16 = {
                let ml_field_owner142 = descriptor.clone();
                let ml_field_value142 = false;
                ml_field_owner142.borrow_mut().0.0 = ml_field_value142;
                ()
            };
            {
                let source_ml_source1_temporary_construct_java_script_array_object_written17 = {
                    let ml_field_owner143 = descriptor.clone();
                    let ml_field_value143 = true;
                    ml_field_owner143.borrow_mut().0.10 = ml_field_value143;
                    ()
                };
                {
                    let source_ml_source1_temporary_construct_java_script_array_object_written18 = {
                        let ml_field_owner144 = descriptor.clone();
                        let ml_field_value144 = self::JavaScriptSemanticValue::Number(length);
                        ml_field_owner144.borrow_mut().0.8 = ml_field_value144;
                        ()
                    };
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        identity.clone(),
                        Box::new(prototype.clone()),
                        true,
                        false,
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                Box::new(self::read_java_script_array_length_key()),
                                descriptor.clone(),
                            ),
                        ])),
                    )
                }
            }
        }
    }
}
