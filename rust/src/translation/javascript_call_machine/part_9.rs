// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::construct_java_script_semantic_text_units;
use super::find_java_script_heap_directory_object;
use super::find_java_script_property_descriptor_entry;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_literal_property_index;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_value_error;
use super::test_java_script_heap_directory_symbol;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_heap_directory_value_error(
    heap: self::JavaScriptOrdinaryHeap,
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
            shared_constructor_field1,
            shared_constructor_field2,
            _,
            _,
            _,
            _,
        ) => {
            let error = self::read_java_script_semantic_value_error(value.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                error.clone()
            } else {
                match value.clone() {
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        if self::test_java_script_heap_directory_symbol(
                            shared_constructor_field2.clone(),
                            identity.clone(),
                        ) {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            self::ml_string::Text::from_units(vec![
                                97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98,
                                111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103,
                                32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101,
                                116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110,
                                116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121,
                            ])
                        }
                    }
                    _ => match value.clone() {
                        self::JavaScriptSemanticValue::Object(identity_2, callable) => {
                            let at = self::find_java_script_heap_directory_object(
                                shared_constructor_field1.clone(),
                                identity_2.clone(),
                            );
                            if (at < (0f64)) {
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106,
                                    101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111,
                                    110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109,
                                    112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32,
                                    105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101,
                                    99, 116, 111, 114, 121,
                                ])
                            } else {
                                {
                                    let object = {
                                        let source_ml_source2_array_read_value101 =
                                            shared_constructor_field1.clone();
                                        {
                                            let ml_reference_array132 =
                                                source_ml_source2_array_read_value101.clone();
                                            let ml_reference_position132 = at;
                                            let ml_reference_projection132 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array132.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position132,
                                                    ),
                                                );
                                            ml_reference_projection132
                                        }
                                    };
                                    if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value116, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value116
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value117, _, _, _, _) => {
                                                        source_ml_source2_data_property_value117
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value118, _) => {
                                                        source_ml_source2_data_property_value118
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value119, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value119
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value120, _, _) => {
                                                        source_ml_source2_data_property_value120
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value121, _, _) => {
                                                        source_ml_source2_data_property_value121
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value122, _, _) => {
                                                        source_ml_source2_data_property_value122
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value123, _, _) => {
                                                        source_ml_source2_data_property_value123
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value124, _) => {
                                                        source_ml_source2_data_property_value124
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value125, _, _, _) => {
                                                        source_ml_source2_data_property_value125
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value126, _, _, _) => {
                                                        source_ml_source2_data_property_value126
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value127, _, _, _) => {
                                                        source_ml_source2_data_property_value127
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value128, _, _, _) => {
                                                        source_ml_source2_data_property_value128
                                                    }
                                                }) == callable)) {
                                                    self::ml_string::Text::from_units(vec![])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121, 32, 109, 117, 115, 116, 32, 97, 103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                                }
                                }
                            }
                        }
                        _ => self::ml_string::Text::from_units(vec![]),
                    },
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_heap_directory_key_error(
    heap: self::JavaScriptOrdinaryHeap,
    key: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    if ((match key.clone() {
        self::JavaScriptSemanticValue::Text(_) => false,
        _ => true,
    } && match key.clone() {
        self::JavaScriptSemanticValue::SourceText(_, _) => false,
        _ => true,
    }) && match key.clone() {
        self::JavaScriptSemanticValue::Symbol(_) => false,
        _ => true,
    }) {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114,
            116, 121, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 101, 120,
            116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 104, 111, 117,
            116, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 99, 111, 101, 114, 99, 105, 111,
            110,
        ])
    } else {
        self::read_java_script_heap_directory_value_error(heap.clone(), key.clone())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_heap_directory_property(
    properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    self::find_java_script_property_descriptor_entry(properties.clone(), key.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_length_key() -> self::JavaScriptSemanticValue {
    self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![
        108f64, 101f64, 110f64, 103f64, 116f64, 104f64,
    ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_length_key(key: self::JavaScriptSemanticValue) -> bool {
    self::test_java_script_semantic_same_value(
        key.clone(),
        self::read_java_script_array_length_key(),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_index(key: self::JavaScriptSemanticValue) -> f64 {
    {
        let length = self::read_java_script_semantic_text_length(key.clone());
        if ((length < (0f64)) || (length > (10f64))) {
            (-1f64)
        } else {
            self::read_java_script_literal_property_index(
                self::construct_java_script_semantic_text_units(key.clone(), 10f64),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_array_property(
    properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    self::find_java_script_heap_directory_property(properties.clone(), key.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_length(object: self::JavaScriptOrdinaryObject) -> f64 {
    {
        let property = {
            let source_ml_source2_array_read_value159 = match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value133,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value133.clone(),
                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value134,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value134.clone(),
                self::JavaScriptOrdinaryObject::ArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value135,
                ) => source_ml_source2_data_property_value135.clone(),
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value136,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value136.clone(),
                self::JavaScriptOrdinaryObject::MapObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value137,
                    _,
                ) => source_ml_source2_data_property_value137.clone(),
                self::JavaScriptOrdinaryObject::SetObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value138,
                    _,
                ) => source_ml_source2_data_property_value138.clone(),
                self::JavaScriptOrdinaryObject::WeakMapObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value139,
                    _,
                ) => source_ml_source2_data_property_value139.clone(),
                self::JavaScriptOrdinaryObject::WeakSetObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value140,
                    _,
                ) => source_ml_source2_data_property_value140.clone(),
                self::JavaScriptOrdinaryObject::OrdinaryObject(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value141,
                ) => source_ml_source2_data_property_value141.clone(),
                self::JavaScriptOrdinaryObject::HostErrorObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value142,
                    _,
                    _,
                ) => source_ml_source2_data_property_value142.clone(),
                self::JavaScriptOrdinaryObject::HostURLObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value143,
                    _,
                    _,
                ) => source_ml_source2_data_property_value143.clone(),
                self::JavaScriptOrdinaryObject::HostArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value144,
                    _,
                    _,
                ) => source_ml_source2_data_property_value144.clone(),
                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value145,
                    _,
                    _,
                ) => source_ml_source2_data_property_value145.clone(),
            };
            {
                let source_ml_source2_array_read_index160 = self::find_java_script_array_property(
                    match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value146,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value146.clone(),
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value147,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value147.clone(),
                        self::JavaScriptOrdinaryObject::ArrayObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value148,
                        ) => source_ml_source2_data_property_value148.clone(),
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value149,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value149.clone(),
                        self::JavaScriptOrdinaryObject::MapObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value150,
                            _,
                        ) => source_ml_source2_data_property_value150.clone(),
                        self::JavaScriptOrdinaryObject::SetObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value151,
                            _,
                        ) => source_ml_source2_data_property_value151.clone(),
                        self::JavaScriptOrdinaryObject::WeakMapObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value152,
                            _,
                        ) => source_ml_source2_data_property_value152.clone(),
                        self::JavaScriptOrdinaryObject::WeakSetObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value153,
                            _,
                        ) => source_ml_source2_data_property_value153.clone(),
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value154,
                        ) => source_ml_source2_data_property_value154.clone(),
                        self::JavaScriptOrdinaryObject::HostErrorObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value155,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value155.clone(),
                        self::JavaScriptOrdinaryObject::HostURLObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value156,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value156.clone(),
                        self::JavaScriptOrdinaryObject::HostArrayObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value157,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value157.clone(),
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value158,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value158.clone(),
                    },
                    self::read_java_script_array_length_key(),
                );
                {
                    let ml_reference_array133 = source_ml_source2_array_read_value159.clone();
                    let ml_reference_position133 = source_ml_source2_array_read_index160;
                    let ml_reference_projection133 = self::ml_array::optional_at(
                        &*ml_reference_array133.borrow(),
                        self::ml_array::number_index(ml_reference_position133),
                    );
                    ml_reference_projection133
                }
            }
        };
        if match (property.clone(), None::<()>) {
            (Some(_ml_left), Some(_ml_right)) => false,
            (None, None) => true,
            _ => false,
        } {
            (-1f64)
        } else {
            self::read_java_script_array_length_value(
                (({
                    let ml_reference_value134 =
                        (match property.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                _,
                                source_ml_source2_data_property_value161,
                            ) => source_ml_source2_data_property_value161.clone(),
                        })
                        .borrow()
                        .clone();
                    ml_reference_value134
                })
                .0)
                    .8,
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_length_value(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Number(value_value) => value_value,
        _ => (-1f64),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_length(length: f64) -> bool {
    (((self::ml_math::is_integer(length) && (length >= (0f64))) && (length <= (4294967295f64)))
        && !({
            let source_ml_source2_same_value_operand163 = (-0f64);
            (((length == source_ml_source2_same_value_operand163)
                && ((length != (0f64))
                    || ((1f64 / length) == (1f64 / source_ml_source2_same_value_operand163))))
                || ((length != length)
                    && (source_ml_source2_same_value_operand163
                        != source_ml_source2_same_value_operand163)))
        }))
}
