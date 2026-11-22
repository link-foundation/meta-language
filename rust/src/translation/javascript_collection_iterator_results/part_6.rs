// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptArrayObjectErrorLoop15Result;
use super::create_java_script_property_descriptor;
use super::find_java_script_array_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_length_key;
use super::read_java_script_array_length_value;
use super::source_ml_temporary_read_java_script_array_object_error_loop15;
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
            source_ml_source1_data_property_value701,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value701,
        self::JavaScriptOrdinaryObject::BoundFunctionObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value702,
            _,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value702,
        self::JavaScriptOrdinaryObject::ArrayObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value703,
            _,
        ) => source_ml_source1_data_property_value703,
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value704,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value704,
        self::JavaScriptOrdinaryObject::MapObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value705,
            _,
            _,
        ) => source_ml_source1_data_property_value705,
        self::JavaScriptOrdinaryObject::SetObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value706,
            _,
            _,
        ) => source_ml_source1_data_property_value706,
        self::JavaScriptOrdinaryObject::WeakMapObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value707,
            _,
            _,
        ) => source_ml_source1_data_property_value707,
        self::JavaScriptOrdinaryObject::WeakSetObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value708,
            _,
            _,
        ) => source_ml_source1_data_property_value708,
        self::JavaScriptOrdinaryObject::OrdinaryObject(
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            source_ml_source1_data_property_value709,
            _,
        ) => source_ml_source1_data_property_value709,
        self::JavaScriptOrdinaryObject::HostErrorObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value710,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value710,
        self::JavaScriptOrdinaryObject::HostURLObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value711,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value711,
        self::JavaScriptOrdinaryObject::HostArrayObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value712,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value712,
        self::JavaScriptOrdinaryObject::HostByteBufferObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value713,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value713,
    } {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 65, 114, 114, 97, 121, 32, 101, 120, 111, 116, 105, 99, 32, 111, 98, 106,
            101, 99, 116, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101,
        ])
    } else {
        {
            let property = {
                let source_ml_source1_array_read_value740 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value714,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value714.clone(),
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value715,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value715.clone(),
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value716,
                    ) => source_ml_source1_data_property_value716.clone(),
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value717,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value717.clone(),
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value718,
                        _,
                    ) => source_ml_source1_data_property_value718.clone(),
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value719,
                        _,
                    ) => source_ml_source1_data_property_value719.clone(),
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value720,
                        _,
                    ) => source_ml_source1_data_property_value720.clone(),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value721,
                        _,
                    ) => source_ml_source1_data_property_value721.clone(),
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value722,
                    ) => source_ml_source1_data_property_value722.clone(),
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value723,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value723.clone(),
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value724,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value724.clone(),
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value725,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value725.clone(),
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value726,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value726.clone(),
                };
                {
                    let source_ml_source1_array_read_index741 =
                        self::find_java_script_array_property(
                            match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value727,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value727.clone(),
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value728,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value728.clone(),
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value729,
                                ) => source_ml_source1_data_property_value729.clone(),
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value730,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value730.clone(),
                                self::JavaScriptOrdinaryObject::MapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value731,
                                    _,
                                ) => source_ml_source1_data_property_value731.clone(),
                                self::JavaScriptOrdinaryObject::SetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value732,
                                    _,
                                ) => source_ml_source1_data_property_value732.clone(),
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value733,
                                    _,
                                ) => source_ml_source1_data_property_value733.clone(),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value734,
                                    _,
                                ) => source_ml_source1_data_property_value734.clone(),
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value735,
                                ) => source_ml_source1_data_property_value735.clone(),
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value736,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value736.clone(),
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value737,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value737.clone(),
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value738,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value738.clone(),
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value739,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value739.clone(),
                            },
                            self::read_java_script_array_length_key(),
                        );
                    {
                        let ml_reference_array103 = source_ml_source1_array_read_value740;
                        let ml_reference_position103 = source_ml_source1_array_read_index741;
                        self::ml_array::optional_at(
                            &*ml_reference_array103.borrow(),
                            self::ml_array::number_index(ml_reference_position103),
                        )
                    }
                }
            };
            if ((((match (property.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } || !(({
                (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                        _,
                        source_ml_source1_data_property_value742,
                    ) => source_ml_source1_data_property_value742.clone(),
                })
                .borrow()
                .clone()
            })
            .9)) || ({
                (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                        _,
                        source_ml_source1_data_property_value743,
                    ) => source_ml_source1_data_property_value743.clone(),
                })
                .borrow()
                .clone()
            })
            .5) || ({
                (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                        _,
                        source_ml_source1_data_property_value744,
                    ) => source_ml_source1_data_property_value744.clone(),
                })
                .borrow()
                .clone()
            })
            .7) || ({
                (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                        _,
                        source_ml_source1_data_property_value745,
                    ) => source_ml_source1_data_property_value745.clone(),
                })
                .borrow()
                .clone()
            })
            .2) || ({
                (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                        _,
                        source_ml_source1_data_property_value746,
                    ) => source_ml_source1_data_property_value746.clone(),
                })
                .borrow()
                .clone()
            })
            .0
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
                        ({
                            (match property.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    _,
                                    source_ml_source1_data_property_value747,
                                ) => source_ml_source1_data_property_value747.clone(),
                            })
                            .borrow()
                            .clone()
                        })
                        .8,
                    );
                    if (self::test_java_script_array_length(length)) {
                        {
                            let source_ml_temporary_values14 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value748,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value748.clone(),
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value749,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value749.clone(),
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value750,
                                ) => source_ml_source1_data_property_value750.clone(),
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value751,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value751.clone(),
                                self::JavaScriptOrdinaryObject::MapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value752,
                                    _,
                                ) => source_ml_source1_data_property_value752.clone(),
                                self::JavaScriptOrdinaryObject::SetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value753,
                                    _,
                                ) => source_ml_source1_data_property_value753.clone(),
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value754,
                                    _,
                                ) => source_ml_source1_data_property_value754.clone(),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value755,
                                    _,
                                ) => source_ml_source1_data_property_value755.clone(),
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value756,
                                ) => source_ml_source1_data_property_value756.clone(),
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value757,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value757.clone(),
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value758,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value758.clone(),
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value759,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value759.clone(),
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value760,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value760.clone(),
                            };
                            {
                                let source_ml_temporary_index14 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source1_s761 = self::source_ml_temporary_read_java_script_array_object_error_loop15(length, source_ml_temporary_values14, source_ml_temporary_index14);
                                    match source_ml_source1_s761 {
                                        self::MlTemporaryReadJavaScriptArrayObjectErrorLoop15Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop15Done => {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                        self::MlTemporaryReadJavaScriptArrayObjectErrorLoop15Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop15Return(source_ml_temporary_result15) => {
                                            source_ml_temporary_result15.clone()
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        self::ml_string::Text::from_units(vec![
                            97, 110, 32, 65, 114, 114, 97, 121, 32, 114, 101, 116, 97, 105, 110,
                            115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111,
                            110, 101, 110, 117, 109, 101, 114, 97, 98, 108, 101, 32, 110, 111, 110,
                            99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101, 32, 105, 110,
                            116, 101, 103, 114, 97, 108, 32, 108, 101, 110, 103, 116, 104, 32, 100,
                            97, 116, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121,
                        ])
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
            let source_ml_temporary_construct_java_script_array_object_written16 = {
                let ml_field_owner110 = descriptor.clone();
                let ml_field_value110 = false;
                ml_field_owner110.borrow_mut().0 = ml_field_value110;
                ()
            };
            {
                let source_ml_temporary_construct_java_script_array_object_written17 = {
                    let ml_field_owner111 = descriptor.clone();
                    let ml_field_value111 = true;
                    ml_field_owner111.borrow_mut().10 = ml_field_value111;
                    ()
                };
                {
                    let source_ml_temporary_construct_java_script_array_object_written18 = {
                        let ml_field_owner112 = descriptor.clone();
                        let ml_field_value112 = self::JavaScriptSemanticValue::Number(length);
                        ml_field_owner112.borrow_mut().8 = ml_field_value112;
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
