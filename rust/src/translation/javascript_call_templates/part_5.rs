// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptArrayObjectErrorLoop26Result;
use super::MlTemporaryTestJavaScriptCollectionKeyJoin43;
use super::find_java_script_array_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_length_key;
use super::read_java_script_array_length_value;
use super::source_ml_temporary_read_java_script_array_object_error_loop26;
use super::test_java_script_array_length;
use super::test_java_script_semantic_same_value;

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
            source_ml_source1_data_property_value981,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value981,
        self::JavaScriptOrdinaryObject::BoundFunctionObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value982,
            _,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value982,
        self::JavaScriptOrdinaryObject::ArrayObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value983,
            _,
        ) => source_ml_source1_data_property_value983,
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value984,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value984,
        self::JavaScriptOrdinaryObject::MapObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value985,
            _,
            _,
        ) => source_ml_source1_data_property_value985,
        self::JavaScriptOrdinaryObject::SetObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value986,
            _,
            _,
        ) => source_ml_source1_data_property_value986,
        self::JavaScriptOrdinaryObject::WeakMapObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value987,
            _,
            _,
        ) => source_ml_source1_data_property_value987,
        self::JavaScriptOrdinaryObject::WeakSetObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value988,
            _,
            _,
        ) => source_ml_source1_data_property_value988,
        self::JavaScriptOrdinaryObject::OrdinaryObject(
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            source_ml_source1_data_property_value989,
            _,
        ) => source_ml_source1_data_property_value989,
        self::JavaScriptOrdinaryObject::HostErrorObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value990,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value990,
        self::JavaScriptOrdinaryObject::HostURLObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value991,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value991,
        self::JavaScriptOrdinaryObject::HostArrayObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value992,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value992,
        self::JavaScriptOrdinaryObject::HostByteBufferObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value993,
            _,
            _,
            _,
        ) => source_ml_source1_data_property_value993,
    } {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 65, 114, 114, 97, 121, 32, 101, 120, 111, 116, 105, 99, 32, 111, 98, 106,
            101, 99, 116, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101,
        ])
    } else {
        {
            let property = {
                let source_ml_source1_array_read_value1020 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value994,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value994.clone(),
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value995,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value995.clone(),
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value996,
                    ) => source_ml_source1_data_property_value996.clone(),
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value997,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value997.clone(),
                    self::JavaScriptOrdinaryObject::MapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value998,
                        _,
                    ) => source_ml_source1_data_property_value998.clone(),
                    self::JavaScriptOrdinaryObject::SetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value999,
                        _,
                    ) => source_ml_source1_data_property_value999.clone(),
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value1000,
                        _,
                    ) => source_ml_source1_data_property_value1000.clone(),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value1001,
                        _,
                    ) => source_ml_source1_data_property_value1001.clone(),
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value1002,
                    ) => source_ml_source1_data_property_value1002.clone(),
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value1003,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value1003.clone(),
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value1004,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value1004.clone(),
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value1005,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value1005.clone(),
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value1006,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value1006.clone(),
                };
                {
                    let source_ml_source1_array_read_index1021 =
                        self::find_java_script_array_property(
                            match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1007,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1007.clone(),
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1008,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1008.clone(),
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1009,
                                ) => source_ml_source1_data_property_value1009.clone(),
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1010,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1010.clone(),
                                self::JavaScriptOrdinaryObject::MapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1011,
                                    _,
                                ) => source_ml_source1_data_property_value1011.clone(),
                                self::JavaScriptOrdinaryObject::SetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1012,
                                    _,
                                ) => source_ml_source1_data_property_value1012.clone(),
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1013,
                                    _,
                                ) => source_ml_source1_data_property_value1013.clone(),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1014,
                                    _,
                                ) => source_ml_source1_data_property_value1014.clone(),
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1015,
                                ) => source_ml_source1_data_property_value1015.clone(),
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1016,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1016.clone(),
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1017,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1017.clone(),
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1018,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1018.clone(),
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1019,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1019.clone(),
                            },
                            self::read_java_script_array_length_key(),
                        );
                    {
                        let ml_reference_array32 = source_ml_source1_array_read_value1020;
                        let ml_reference_position32 = source_ml_source1_array_read_index1021;
                        self::ml_array::optional_at(
                            &*ml_reference_array32.borrow(),
                            self::ml_array::number_index(ml_reference_position32),
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
                        source_ml_source1_data_property_value1022,
                    ) => source_ml_source1_data_property_value1022.clone(),
                })
                .borrow()
                .clone()
            })
            .9)) || ({
                (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                        _,
                        source_ml_source1_data_property_value1023,
                    ) => source_ml_source1_data_property_value1023.clone(),
                })
                .borrow()
                .clone()
            })
            .5) || ({
                (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                        _,
                        source_ml_source1_data_property_value1024,
                    ) => source_ml_source1_data_property_value1024.clone(),
                })
                .borrow()
                .clone()
            })
            .7) || ({
                (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                        _,
                        source_ml_source1_data_property_value1025,
                    ) => source_ml_source1_data_property_value1025.clone(),
                })
                .borrow()
                .clone()
            })
            .2) || ({
                (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                        _,
                        source_ml_source1_data_property_value1026,
                    ) => source_ml_source1_data_property_value1026.clone(),
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
                                    source_ml_source1_data_property_value1027,
                                ) => source_ml_source1_data_property_value1027.clone(),
                            })
                            .borrow()
                            .clone()
                        })
                        .8,
                    );
                    if (self::test_java_script_array_length(length)) {
                        {
                            let source_ml_temporary_values25 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1028,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1028.clone(),
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1029,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1029.clone(),
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1030,
                                ) => source_ml_source1_data_property_value1030.clone(),
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1031,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1031.clone(),
                                self::JavaScriptOrdinaryObject::MapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1032,
                                    _,
                                ) => source_ml_source1_data_property_value1032.clone(),
                                self::JavaScriptOrdinaryObject::SetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1033,
                                    _,
                                ) => source_ml_source1_data_property_value1033.clone(),
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1034,
                                    _,
                                ) => source_ml_source1_data_property_value1034.clone(),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1035,
                                    _,
                                ) => source_ml_source1_data_property_value1035.clone(),
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1036,
                                ) => source_ml_source1_data_property_value1036.clone(),
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1037,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1037.clone(),
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1038,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1038.clone(),
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1039,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1039.clone(),
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1040,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value1040.clone(),
                            };
                            {
                                let source_ml_temporary_index25 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source1_s1041 = self::source_ml_temporary_read_java_script_array_object_error_loop26(length, source_ml_temporary_values25, source_ml_temporary_index25);
                                    match source_ml_source1_s1041 {
                                        self::MlTemporaryReadJavaScriptArrayObjectErrorLoop26Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop26Done => {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                        self::MlTemporaryReadJavaScriptArrayObjectErrorLoop26Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop26Return(source_ml_temporary_result26) => {
                                            source_ml_temporary_result26.clone()
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
pub fn test_java_script_collection_key(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> bool {
    {
        let source_ml_source1_s1629 = match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
            }
            self::JavaScriptSemanticValue::Null => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
            }
            self::JavaScriptSemanticValue::Boolean(_) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
            }
            self::JavaScriptSemanticValue::Number(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
                    }
                    self::JavaScriptSemanticValue::Number(value) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Return((left_value == value) || (left_value.is_nan() && value.is_nan()))
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
                    }
                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next
            }
        };
        match source_ml_source1_s1629 {
            self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Next => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::MlTemporaryTestJavaScriptCollectionKeyJoin43::MlTemporaryTestJavaScriptCollectionKeyJoin43Return(source_ml_temporary_result43) => {
                source_ml_temporary_result43
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_key_rank(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => 0f64,
        self::JavaScriptSemanticValue::Null => 1f64,
        self::JavaScriptSemanticValue::Boolean(_) => 2f64,
        self::JavaScriptSemanticValue::Number(_) => 3f64,
        self::JavaScriptSemanticValue::Integer(_) => 4f64,
        self::JavaScriptSemanticValue::Symbol(_) => 6f64,
        self::JavaScriptSemanticValue::Object(_, _) => 7f64,
        _ => 5f64,
    }
}
