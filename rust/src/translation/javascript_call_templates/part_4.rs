// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Result;
use super::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result;
use super::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Result;
use super::construct_java_script_semantic_text_units;
use super::find_java_script_property_descriptor_entry;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_find_java_script_heap_directory_object_loop23;
use super::source_ml_temporary_read_java_script_literal_property_index_loop22;
use super::source_ml_temporary_test_java_script_heap_directory_symbol_loop24;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_literal_property_index(
    key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> f64 {
    if (({
        let ml_reference_array25 = key.clone();
        ml_reference_array25.borrow().len() as f64
    }) == 0f64)
        || (({
            let ml_reference_array26 = key.clone();
            ml_reference_array26.borrow().len() as f64
        }) > 10f64)
    {
        -1f64
    } else {
        if (({
            let ml_reference_array27 = key.clone();
            ml_reference_array27.borrow().len() as f64
        }) > 1f64)
            && {
                let source_ml_source1_comparison_operand815 = {
                    let source_ml_source1_array_read_index814 = 0f64;
                    {
                        let ml_reference_array28 = key.clone();
                        let ml_reference_position28 = source_ml_source1_array_read_index814;
                        self::ml_array::optional_at(
                            &*ml_reference_array28.borrow(),
                            self::ml_array::number_index(ml_reference_position28),
                        )
                    }
                };
                {
                    let source_ml_source1_comparison_operand816 = 48f64;
                    match (
                        source_ml_source1_comparison_operand815,
                        source_ml_source1_comparison_operand816,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            }
        {
            -1f64
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s817 = self::source_ml_temporary_read_java_script_literal_property_index_loop22(key.clone(), value, at);
                        match source_ml_source1_s817 {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Done(value_2) => {
                                if value_2 <= 4_294_967_294f64 {
                                    value_2
                                } else {
                                    -1f64
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Return(source_ml_temporary_result22) => {
                                source_ml_temporary_result22
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
pub fn find_java_script_heap_directory_object(
    objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>,
    identity: self::ml::Big,
) -> f64 {
    {
        let low = 0f64;
        {
            let high = {
                let ml_reference_array29 = objects.clone();
                ml_reference_array29.borrow().len() as f64
            };
            {
                let source_ml_source1_s818 =
                    self::source_ml_temporary_find_java_script_heap_directory_object_loop23(
                        objects.clone(),
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s818 {
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Done => {
                        -1f64
                    }
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop23Return(source_ml_temporary_result23) => {
                        source_ml_temporary_result23
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_heap_directory_symbol(
    symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    identity: self::ml::Big,
) -> bool {
    {
        let low = 0f64;
        {
            let high = {
                let ml_reference_array30 = symbols.clone();
                ml_reference_array30.borrow().len() as f64
            };
            {
                let source_ml_source1_s819 =
                    self::source_ml_temporary_test_java_script_heap_directory_symbol_loop24(
                        symbols.clone(),
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s819 {
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Done => {
                        false
                    }
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop24Return(source_ml_temporary_result24) => {
                        source_ml_temporary_result24
                    }
                }
            }
        }
    }
}

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
                                    source_ml_source1_data_property_value877,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value877.clone(),
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
                        source_ml_source1_f826,
                        source_ml_source1_f827,
                    ) => {
                        let at = self::find_java_script_heap_directory_object(
                            match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    source_ml_source1_data_property_value885,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value885.clone(),
                            },
                            source_ml_source1_f826.clone(),
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
                                    let source_ml_source1_array_read_value887 = match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            source_ml_source1_data_property_value886,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value886.clone(),
                                    };
                                    {
                                        let ml_reference_array31 =
                                            source_ml_source1_array_read_value887;
                                        let ml_reference_position31 = at;
                                        self::ml_array::optional_at(
                                            &*ml_reference_array31.borrow(),
                                            self::ml_array::number_index(ml_reference_position31),
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
                                        source_ml_source1_data_property_value902,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value902,
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value903,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value903,
                                    self::JavaScriptOrdinaryObject::ArrayObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value904,
                                        _,
                                    ) => source_ml_source1_data_property_value904,
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value905,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value905,
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value906,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value906,
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value907,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value907,
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value908,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value908,
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value909,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value909,
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value910,
                                        _,
                                    ) => source_ml_source1_data_property_value910,
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value911,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value911,
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value912,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value912,
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value913,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value913,
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value914,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value914,
                                }) == source_ml_source1_f827)
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
            let source_ml_source1_same_value_operand980 = -0f64;
            ((((length).partial_cmp(&(source_ml_source1_same_value_operand980))
                == Some(std::cmp::Ordering::Equal))
                && ((length != 0f64)
                    || ((1f64 / length) == (1f64 / source_ml_source1_same_value_operand980))))
                || ((length != length)
                    && (source_ml_source1_same_value_operand980
                        != source_ml_source1_same_value_operand980)))
        })
}
