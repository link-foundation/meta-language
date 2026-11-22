// Generated from complete js/src/translation/javascript-collection-iteration.js. Do not edit by hand.
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_heap_directory_value_error(
    heap: self::JavaScriptOrdinaryHeap,
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    {
        let error = self::read_java_script_semantic_value_error(value.clone());
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                error.clone()
            } else {
                match value.clone() {
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        if self::test_java_script_heap_directory_symbol(
                            match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    _,
                                    source_ml_source1_data_property_value597,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value597.clone(),
                            },
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
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f546,
                        source_ml_source1_f547,
                    ) => {
                        let at = self::find_java_script_heap_directory_object(
                            match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    source_ml_source1_data_property_value605,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value605.clone(),
                            },
                            source_ml_source1_f546.clone(),
                        );
                        if at < 0f64 {
                            self::ml_string::Text::from_units(vec![
                                97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101,
                                99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103,
                                32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101,
                                116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110,
                                116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121,
                            ])
                        } else {
                            {
                                let object = {
                                    let source_ml_source1_array_read_value607 = match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            source_ml_source1_data_property_value606,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value606.clone(),
                                    };
                                    {
                                        let ml_reference_array30 =
                                            source_ml_source1_array_read_value607;
                                        let ml_reference_position30 = at;
                                        self::ml_array::optional_at(
                                            &*ml_reference_array30.borrow(),
                                            self::ml_array::number_index(ml_reference_position30),
                                        )
                                    }
                                };
                                if !(match (object.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) && ((match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value622,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value622,
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value623,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value623,
                                    self::JavaScriptOrdinaryObject::ArrayObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value624,
                                        _,
                                    ) => source_ml_source1_data_property_value624,
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value625,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value625,
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value626,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value626,
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value627,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value627,
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value628,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value628,
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value629,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value629,
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value630,
                                        _,
                                    ) => source_ml_source1_data_property_value630,
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value631,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value631,
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value632,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value632,
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value633,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value633,
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value634,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value634,
                                }) == source_ml_source1_f547)
                                {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    self::ml_string::Text::from_units(vec![
                                        79, 98, 106, 101, 99, 116, 32, 99, 97, 108, 108, 97, 98,
                                        105, 108, 105, 116, 121, 32, 109, 117, 115, 116, 32, 97,
                                        103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 105, 116,
                                        115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32,
                                        103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116,
                                        105, 116, 121,
                                    ])
                                }
                            }
                        }
                    }
                    _ => self::ml_string::Text::from_units(vec![]),
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
    if (match key.clone() {
        self::JavaScriptSemanticValue::Text(_) => false,
        _ => true,
    } && match key.clone() {
        self::JavaScriptSemanticValue::SourceText(_, _) => false,
        _ => true,
    }) && match key.clone() {
        self::JavaScriptSemanticValue::Symbol(_) => false,
        _ => true,
    } {
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
pub fn read_java_script_array_index(key: self::JavaScriptSemanticValue) -> f64 {
    {
        let length = self::read_java_script_semantic_text_length(key.clone());
        if (length < 0f64) || (length > 10f64) {
            -1f64
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
pub fn read_java_script_array_length_value(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Number(value_value) => value_value,
        _ => -1f64,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_length(length: f64) -> bool {
    ((self::ml_math::is_integer(length) && (length >= 0f64)) && (length <= 4_294_967_295f64))
        && !({
            let source_ml_source1_same_value_operand700 = -0f64;
            ((((length).partial_cmp(&(source_ml_source1_same_value_operand700))
                == Some(std::cmp::Ordering::Equal))
                && ((length != 0f64)
                    || ((1f64 / length) == (1f64 / source_ml_source1_same_value_operand700))))
                || ((length != length)
                    && (source_ml_source1_same_value_operand700
                        != source_ml_source1_same_value_operand700)))
        })
}
