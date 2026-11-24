// Generated from complete js/src/translation/javascript-property-key-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_ordinary_symbol_loop6(
    mut symbols: Vec<self::ml::Big>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let symbol =
                        self::ml_array::optional_at(&symbols, self::ml_array::number_index(middle));
                    if {
                        let source_ml_source1_comparison_operand946 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand947 = None::<()>;
                            match (
                                source_ml_source1_comparison_operand946,
                                source_ml_source1_comparison_operand947,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }
                        }
                    } {
                        self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(false)
                    } else if {
                        let source_ml_source1_comparison_operand948 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand949 = identity.clone();
                            match (
                                source_ml_source1_comparison_operand948,
                                source_ml_source1_comparison_operand949,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            }
                        }
                    } {
                        self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(true)
                    } else if {
                        let source_ml_source1_comparison_operand950 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand951 = identity.clone();
                            if match (
                                source_ml_source1_comparison_operand950.clone(),
                                None::<self::ml::Big>,
                            ) {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                {
                                    let source_ml_source1_comparison_order952 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source1_comparison_operand951.clone());
                                    (source_ml_source1_comparison_order952 == -1f64)
                                }
                            } else {
                                (source_ml_source1_comparison_operand950
                                    .clone()
                                    .expect("a proven present value"))
                                    < source_ml_source1_comparison_operand951
                            }
                        }
                    } {
                        {
                            let low_2 = middle + 1f64;
                            {
                                low = low_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            let high_2 = middle;
                            {
                                high = high_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result {
    MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Done,
    MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_ordinary_property_loop7(
    mut properties: Vec<self::JavaScriptOrdinaryProperty>,
    mut key: self::JavaScriptSemanticValue,
    mut at: f64,
) -> self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result {
    loop {
        return if at < (properties.len() as f64) {
            {
                let property =
                    self::ml_array::optional_at(&properties, self::ml_array::number_index(at));
                if !(match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && self::test_java_script_semantic_same_value(
                    {
                        let source_ml_source1_nullish_field_receiver953 = (property.clone(),);
                        if match (source_ml_source1_nullish_field_receiver953.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present954 =
                                    ((source_ml_source1_nullish_field_receiver953.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_nullish_field_present954 {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        source_ml_source1_data_property_value955,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value955 =
                                            *source_ml_source1_data_property_value955;
                                        source_ml_source1_data_property_value955.clone()
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    },
                    key.clone(),
                ) {
                    self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Return(at)
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop15(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut prototype: self::JavaScriptSemanticValue,
    mut depth: f64,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result {
    loop {
        return match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Null => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f956) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f957) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f958) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f959) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f960) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f961) => {
                {
                    let depth_2 = depth + 1f64;
                    if (identity == (match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value963, _, _, _, _) => {
                            source_ml_source1_data_property_value963.clone()
                        }
                    })) || (depth_2 > ((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value965, _) => source_ml_source1_data_property_value965.as_slice() }).len() as f64)) {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104, 97, 105, 110, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 121, 99, 108, 105, 99]))
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value966, _) => {
                                    source_ml_source1_data_property_value966
                                }
                            }, identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value967, _) => source_ml_source1_data_property_value967.as_slice() }), self::ml_array::number_index(index));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                } else {
                                    {
                                        let prototype_2 = {
                                            let source_ml_source1_nullish_field_receiver968 = (parent.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver968.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present969 = ((source_ml_source1_nullish_field_receiver968.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present969 {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value970, _, _, _) => {
                                                            let source_ml_source1_data_property_value970 = *source_ml_source1_data_property_value970;
                                                            source_ml_source1_data_property_value970.clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                                            }
                                        };
                                        {
                                            (prototype, depth) = (prototype_2, depth_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop14(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result {
    loop {
        return if at
            < ((match &object {
                self::JavaScriptOrdinaryObject::OrdinaryObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value972,
                ) => source_ml_source1_data_property_value972.as_slice(),
            })
            .len() as f64)
        {
            {
                let property = self::ml_array::optional_at(
                    (match &object {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value973,
                        ) => source_ml_source1_data_property_value973.as_slice(),
                    }),
                    self::ml_array::number_index(at),
                );
                if match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_key_error(heap.clone(), {
                            let source_ml_source1_nullish_field_receiver974 = (property.clone(),);
                            if match (source_ml_source1_nullish_field_receiver974.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source1_nullish_field_present975 =
                                        ((source_ml_source1_nullish_field_receiver974.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source1_nullish_field_present975 {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                            source_ml_source1_data_property_value976,
                                            _,
                                        ) => {
                                            let source_ml_source1_data_property_value976 =
                                                *source_ml_source1_data_property_value976;
                                            source_ml_source1_data_property_value976.clone()
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121,
                                        39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        });
                        {
                            let ml_library_inequality_4 =
                                key_error != (self::ml_string::Text::from_units(vec![]));
                            if ml_library_inequality_4 {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(key_error.clone())
                            } else if (self::find_java_script_ordinary_property(
                                match object.clone() {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value981,
                                    ) => source_ml_source1_data_property_value981,
                                },
                                {
                                    let source_ml_source1_nullish_field_receiver982 =
                                        (property.clone(),);
                                    if match (source_ml_source1_nullish_field_receiver982.clone()).0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source1_nullish_field_present983 =
                                                ((source_ml_source1_nullish_field_receiver982
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source1_nullish_field_present983 {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value984, _) => {
                                                let source_ml_source1_data_property_value984 = *source_ml_source1_data_property_value984;
                                                source_ml_source1_data_property_value984.clone()
                                            }
                                        }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97,
                                                    100, 32, 112, 114, 111, 112, 101, 114, 116,
                                                    105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                                                    101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                                    97, 100, 105, 110, 103, 32, 39, 107, 101, 121,
                                                    39, 41,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    }
                                },
                            )) != at
                            {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 114, 101, 111, 114, 100, 101, 114, 105, 110, 103, 32, 116, 104, 101, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                            } else {
                                {
                                    let descriptor = {
                                        let source_ml_source1_nullish_field_receiver985 =
                                            (property.clone(),);
                                        if match (source_ml_source1_nullish_field_receiver985
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
                                                let source_ml_source1_nullish_field_present986 =
                                                    ((source_ml_source1_nullish_field_receiver985
                                                        .clone())
                                                    .0)
                                                        .clone()
                                                        .expect("a proven present value");
                                                match source_ml_source1_nullish_field_present986 {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value987) => {
                                                        source_ml_source1_data_property_value987.clone()
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
                                                        39, 100, 101, 115, 99, 114, 105, 112, 116,
                                                        111, 114, 39, 41,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
                                        }
                                    };
                                    if (self::test_complete_java_script_property_descriptor(
                                        descriptor.clone(),
                                    )) {
                                        {
                                            let source_ml_source1_s988 = if (descriptor.clone()).9 {
                                                {
                                                    let error =
                                                        self::read_java_script_ordinary_value_error(
                                                            heap.clone(),
                                                            (descriptor.clone()).8,
                                                        );
                                                    {
                                                        let ml_library_inequality_3 = error
                                                            != (self::ml_string::Text::from_units(
                                                                vec![],
                                                            ));
                                                        if ml_library_inequality_3 {
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(error.clone())
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next
                                            };
                                            match source_ml_source1_s988 {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next => {
                                                    {
                                                        let source_ml_source1_s989 = if (descriptor.clone()).5 {
                                                            {
                                                                let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4);
                                                                {
let ml_library_inequality_2 = error_2 != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_2 {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(error_2.clone())
                                                                } else {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next
                                                                }
}
                                                            }
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next
                                                        };
                                                        match source_ml_source1_s989 {
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next => {
                                                                if (descriptor.clone()).7 {
                                                                    {
                                                                        let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6);
                                                                        {
let ml_library_inequality_1 = error_3 != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(error_3.clone())
                                                                        } else {
                                                                            {
                                                                                let at_2 = at + 1f64;
                                                                                {
                                                                                    at = at_2;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
}
                                                                    }
                                                                } else {
                                                                    {
                                                                        let at_3 = at + 1f64;
                                                                        {
                                                                            at = at_3;
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(source_ml_temporary_result17) => {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_temporary_result17.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(source_ml_temporary_result17_2) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_temporary_result17_2.clone())
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 105, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115]))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop13(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut property_count: f64,
    mut source_ml_temporary_values10: Vec<self::JavaScriptOrdinaryObject>,
    mut source_ml_temporary_index10: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result {
    loop {
        return if source_ml_temporary_index10
            < (self::ml::Big::from_u128(source_ml_temporary_values10.len() as u128))
        {
            {
                let object = self::ml_array::at(
                    &source_ml_temporary_values10,
                    source_ml_temporary_index10.to_index(),
                );
                if {
                    let source_ml_source1_s991 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            source_ml_source1_data_property_value999,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value999 =
                                *source_ml_source1_data_property_value999;
                            source_ml_source1_data_property_value999.clone()
                        }
                    };
                    match source_ml_source1_s991 {
                        self::JavaScriptSemanticValue::Undefined => true,
                        self::JavaScriptSemanticValue::Null => false,
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f992) => true,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f993) => true,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f994) => true,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f995) => true,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f996) => true,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f997,
                            source_ml_source1_f998,
                        ) => true,
                    }
                } && {
                    let source_ml_source1_s1001 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            source_ml_source1_data_property_value1009,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value1009 =
                                *source_ml_source1_data_property_value1009;
                            source_ml_source1_data_property_value1009.clone()
                        }
                    };
                    match source_ml_source1_s1001 {
                        self::JavaScriptSemanticValue::Undefined => true,
                        self::JavaScriptSemanticValue::Null => true,
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1002) => true,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1003) => true,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1004) => true,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1005) => true,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1006) => true,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f1007,
                            source_ml_source1_f1008,
                        ) => false,
                    }
                } {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]))
                } else {
                    {
                        let prototype_error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            match object.clone() {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    source_ml_source1_data_property_value1010,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value1010 =
                                        *source_ml_source1_data_property_value1010;
                                    source_ml_source1_data_property_value1010.clone()
                                }
                            },
                        );
                        {
                            let ml_library_inequality_1 =
                                prototype_error != (self::ml_string::Text::from_units(vec![]));
                            if ml_library_inequality_1 {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(prototype_error.clone())
                            } else {
                                {
                                    let property_count_2 = property_count
                                        + ((match &object {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source1_data_property_value1011,
                                            ) => {
                                                source_ml_source1_data_property_value1011.as_slice()
                                            }
                                        })
                                        .len() as f64);
                                    if (property_count_2 > 16_384f64)
                                        || (((match &object {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source1_data_property_value1013,
                                            ) => {
                                                source_ml_source1_data_property_value1013.as_slice()
                                            }
                                        })
                                        .len() as f64)
                                            > 4096f64)
                                    {
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                    } else {
                                        {
                                            let at = 0f64;
                                            {
                                                let source_ml_source1_s1014 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop14(heap.clone(), object.clone(), at);
                                                match source_ml_source1_s1014 {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done => {
                                                    {
                                                        let prototype = match object.clone() {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1015, _, _, _) => {
                                                                let source_ml_source1_data_property_value1015 = *source_ml_source1_data_property_value1015;
                                                                source_ml_source1_data_property_value1015.clone()
                                                            }
                                                        };
                                                        {
                                                            let depth = 0f64;
                                                            {
                                                                let source_ml_source1_s1016 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop15(heap.clone(), object.clone(), prototype, depth);
                                                                match source_ml_source1_s1016 {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done => {
                                                                        {
                                                                            let source_ml_temporary_index10_2 = source_ml_temporary_index10.add(&self::ml::Big::from_i128(1));
                                                                            {
                                                                                (property_count, source_ml_temporary_index10) = (property_count_2, source_ml_temporary_index10_2);
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(source_ml_temporary_result15) => {
                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(source_ml_temporary_result15.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_temporary_result15_2) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(source_ml_temporary_result15_2.clone())
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
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop12(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut previous: self::ml::Big,
    mut source_ml_temporary_values9: Vec<self::ml::Big>,
    mut source_ml_temporary_index9: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result {
    loop {
        return if source_ml_temporary_index9
            < (self::ml::Big::from_u128(source_ml_temporary_values9.len() as u128))
        {
            {
                let symbol = self::ml_array::at(
                    &source_ml_temporary_values9,
                    source_ml_temporary_index9.to_index(),
                );
                if ((symbol <= previous)
                    || (symbol > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                    || ((self::find_java_script_ordinary_object(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value1018,
                                _,
                            ) => source_ml_source1_data_property_value1018,
                        },
                        symbol.clone(),
                    )) >= 0f64)
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = symbol.clone();
                        {
                            let source_ml_temporary_index9_2 =
                                source_ml_temporary_index9.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_temporary_index9) =
                                    (previous_2, source_ml_temporary_index9_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Done(self::ml::Big),
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop11(
    mut previous: self::ml::Big,
    mut source_ml_temporary_values8: Vec<self::JavaScriptOrdinaryObject>,
    mut source_ml_temporary_index8: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result {
    loop {
        return if source_ml_temporary_index8
            < (self::ml::Big::from_u128(source_ml_temporary_values8.len() as u128))
        {
            {
                let object = self::ml_array::at(
                    &source_ml_temporary_values8,
                    source_ml_temporary_index8.to_index(),
                );
                if ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        source_ml_source1_data_property_value1020,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value1020.clone(),
                }) <= previous)
                    || ((match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source1_data_property_value1022,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value1022.clone(),
                    }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615)))
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value1023,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value1023.clone(),
                        };
                        {
                            let source_ml_temporary_index8_2 =
                                source_ml_temporary_index8.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_temporary_index8) =
                                    (previous_2, source_ml_temporary_index8_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Done(previous.clone())
        };
    }
}
