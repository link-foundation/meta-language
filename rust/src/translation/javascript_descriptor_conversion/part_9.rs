// Generated from complete js/src/translation/javascript-descriptor-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_source_exception;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_ordinary_object_loop5(
    mut objects: Vec<self::JavaScriptOrdinaryObject>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let object =
                        self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(-1f64)
                    } else {
                        let ml_product_condition_2 = {
                            let source_ml_source1_nullish_field_receiver978 = (object.clone(),);
                            if match (source_ml_source1_nullish_field_receiver978.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source1_nullish_field_present979 =
                                        ((source_ml_source1_nullish_field_receiver978.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source1_nullish_field_present979 {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                                            source_ml_source1_data_property_value980,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value980.clone(),
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (String::from(
                                        "Cannot read properties of undefined (reading 'identity')",
                                    ))
                                    .encode_utf16()
                                    .collect(),
                                    message_present: true,
                                })
                                .raise()
                            }
                        };
                        if ml_product_condition_2 {
                            self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(middle)
                        } else {
                            {
                                let ml_product_condition_1 = {
                                    let source_ml_source1_nullish_field_receiver984 =
                                        (object.clone(),);
                                    if match (source_ml_source1_nullish_field_receiver984.clone()).0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source1_nullish_field_present985 =
                                                ((source_ml_source1_nullish_field_receiver984
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source1_nullish_field_present985 {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                                    source_ml_source1_data_property_value986,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => {
                                                    source_ml_source1_data_property_value986.clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'identity')")).encode_utf16().collect(), message_present: true }).raise()
                                    }
                                };
                                if ml_product_condition_1 {
                                    {
                                        let low_2 = middle + 1f64;
                                        {
                                            low = low_2;
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
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
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result {
    MlTemporaryTestJavaScriptOrdinarySymbolLoop6Done,
    MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(bool),
}

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
                        let source_ml_source1_comparison_operand987 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand988 = None::<()>;
                            match (
                                source_ml_source1_comparison_operand987,
                                source_ml_source1_comparison_operand988,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }
                        }
                    } {
                        self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(false)
                    } else if {
                        let source_ml_source1_comparison_operand989 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand990 = identity.clone();
                            match (
                                source_ml_source1_comparison_operand989,
                                source_ml_source1_comparison_operand990,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            }
                        }
                    } {
                        self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(true)
                    } else if {
                        let source_ml_source1_comparison_operand991 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand992 = identity.clone();
                            if match (
                                source_ml_source1_comparison_operand991.clone(),
                                None::<self::ml::Big>,
                            ) {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                {
                                    let source_ml_source1_comparison_order993 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source1_comparison_operand992.clone());
                                    (source_ml_source1_comparison_order993 == -1f64)
                                }
                            } else {
                                (source_ml_source1_comparison_operand991
                                    .clone()
                                    .expect("a proven present value"))
                                    < source_ml_source1_comparison_operand992
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
                        let source_ml_source1_nullish_field_receiver994 = (property.clone(),);
                        if match (source_ml_source1_nullish_field_receiver994.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present995 =
                                    ((source_ml_source1_nullish_field_receiver994.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_nullish_field_present995 {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        source_ml_source1_data_property_value996,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value996 =
                                            *source_ml_source1_data_property_value996;
                                        source_ml_source1_data_property_value996.clone()
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (String::from(
                                    "Cannot read properties of undefined (reading 'key')",
                                ))
                                .encode_utf16()
                                .collect(),
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
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(String),
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f997) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f998) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f999) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f1000) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1001) => {
                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done
            }
            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f1002) => {
                {
                    let depth_2 = depth + 1f64;
                    if (identity == (match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1004, _, _, _, _) => {
                            source_ml_source1_data_property_value1004.clone()
                        }
                    })) || (depth_2 > ((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1006, _) => source_ml_source1_data_property_value1006.as_slice() }).len() as f64)) {
                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(String::from("ordinary prototype chains must be acyclic"))
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1007, _) => {
                                    source_ml_source1_data_property_value1007
                                }
                            }, identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1008, _) => source_ml_source1_data_property_value1008.as_slice() }), self::ml_array::number_index(index));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(String::from("every ordinary prototype must remain in the global directory"))
                                } else {
                                    {
                                        let prototype_2 = {
                                            let source_ml_source1_nullish_field_receiver1009 = (parent.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver1009.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present1010 = ((source_ml_source1_nullish_field_receiver1009.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present1010 {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1011, _, _, _) => {
                                                            let source_ml_source1_data_property_value1011 = *source_ml_source1_data_property_value1011;
                                                            source_ml_source1_data_property_value1011.clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'prototype')")).encode_utf16().collect(), message_present: true }).raise()
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
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(String),
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
                    source_ml_source1_data_property_value1013,
                ) => source_ml_source1_data_property_value1013.as_slice(),
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
                            source_ml_source1_data_property_value1014,
                        ) => source_ml_source1_data_property_value1014.as_slice(),
                    }),
                    self::ml_array::number_index(at),
                );
                if match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(String::from("ordinary property storage must be dense"))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_key_error(heap.clone(), {
                            let source_ml_source1_nullish_field_receiver1015 = (property.clone(),);
                            if match (source_ml_source1_nullish_field_receiver1015.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source1_nullish_field_present1016 =
                                        ((source_ml_source1_nullish_field_receiver1015.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source1_nullish_field_present1016 {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                            source_ml_source1_data_property_value1017,
                                            _,
                                        ) => {
                                            let source_ml_source1_data_property_value1017 =
                                                *source_ml_source1_data_property_value1017;
                                            source_ml_source1_data_property_value1017.clone()
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (String::from(
                                        "Cannot read properties of undefined (reading 'key')",
                                    ))
                                    .encode_utf16()
                                    .collect(),
                                    message_present: true,
                                })
                                .raise()
                            }
                        });
                        if !(key_error).is_empty() {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(key_error.clone())
                        } else if (self::find_java_script_ordinary_property(
                            match object.clone() {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value1022,
                                ) => source_ml_source1_data_property_value1022,
                            },
                            {
                                let source_ml_source1_nullish_field_receiver1023 =
                                    (property.clone(),);
                                if match (source_ml_source1_nullish_field_receiver1023.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source1_nullish_field_present1024 =
                                            ((source_ml_source1_nullish_field_receiver1023
                                                .clone())
                                            .0)
                                                .clone()
                                                .expect("a proven present value");
                                        match source_ml_source1_nullish_field_present1024 {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                                source_ml_source1_data_property_value1025,
                                                _,
                                            ) => {
                                                let source_ml_source1_data_property_value1025 =
                                                    *source_ml_source1_data_property_value1025;
                                                source_ml_source1_data_property_value1025.clone()
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (String::from(
                                            "Cannot read properties of undefined (reading 'key')",
                                        ))
                                        .encode_utf16()
                                        .collect(),
                                        message_present: true,
                                    })
                                    .raise()
                                }
                            },
                        )) != at
                        {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(String::from("ordinary property keys must be unique without reordering their creation history"))
                        } else {
                            {
                                let descriptor = {
                                    let source_ml_source1_nullish_field_receiver1026 =
                                        (property.clone(),);
                                    if match (source_ml_source1_nullish_field_receiver1026.clone())
                                        .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source1_nullish_field_present1027 =
                                                ((source_ml_source1_nullish_field_receiver1026
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source1_nullish_field_present1027 {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1028) => {
                                                        source_ml_source1_data_property_value1028.clone()
                                                    }
                                                }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'descriptor')")).encode_utf16().collect(), message_present: true }).raise()
                                    }
                                };
                                if (self::test_complete_java_script_property_descriptor(
                                    descriptor.clone(),
                                )) {
                                    {
                                        let source_ml_source1_s1029 = if (descriptor.clone()).9 {
                                            {
                                                let error =
                                                    self::read_java_script_ordinary_value_error(
                                                        heap.clone(),
                                                        (descriptor.clone()).8,
                                                    );
                                                if !(error).is_empty() {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(error.clone())
                                                } else {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next
                                                }
                                            }
                                        } else {
                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next
                                        };
                                        match source_ml_source1_s1029 {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next => {
                                                    {
                                                        let source_ml_source1_s1030 = if (descriptor.clone()).5 {
                                                            {
                                                                let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4);
                                                                if !(error_2).is_empty() {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(error_2.clone())
                                                                } else {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next
                                                        };
                                                        match source_ml_source1_s1030 {
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next => {
                                                                if (descriptor.clone()).7 {
                                                                    {
                                                                        let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6);
                                                                        if !(error_3).is_empty() {
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
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(String::from("ordinary storage requires complete valid property descriptors"))
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
                    let source_ml_source1_s1032 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            source_ml_source1_data_property_value1040,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value1040 =
                                *source_ml_source1_data_property_value1040;
                            source_ml_source1_data_property_value1040.clone()
                        }
                    };
                    match source_ml_source1_s1032 {
                        self::JavaScriptSemanticValue::Undefined => true,
                        self::JavaScriptSemanticValue::Null => false,
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1033) => true,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1034) => true,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1035) => true,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1036) => true,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1037) => true,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f1038,
                            source_ml_source1_f1039,
                        ) => true,
                    }
                } && {
                    let source_ml_source1_s1042 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            source_ml_source1_data_property_value1050,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value1050 =
                                *source_ml_source1_data_property_value1050;
                            source_ml_source1_data_property_value1050.clone()
                        }
                    };
                    match source_ml_source1_s1042 {
                        self::JavaScriptSemanticValue::Undefined => true,
                        self::JavaScriptSemanticValue::Null => true,
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1043) => true,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1044) => true,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1045) => true,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1046) => true,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1047) => true,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f1048,
                            source_ml_source1_f1049,
                        ) => false,
                    }
                } {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(String::from("an ordinary prototype must be null or a declared Object"))
                } else {
                    {
                        let prototype_error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            match object.clone() {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    source_ml_source1_data_property_value1051,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value1051 =
                                        *source_ml_source1_data_property_value1051;
                                    source_ml_source1_data_property_value1051.clone()
                                }
                            },
                        );
                        if !(prototype_error).is_empty() {
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
                                            source_ml_source1_data_property_value1052,
                                        ) => source_ml_source1_data_property_value1052.as_slice(),
                                    })
                                    .len() as f64);
                                if (property_count_2 > 16_384f64)
                                    || (((match &object {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value1054,
                                        ) => source_ml_source1_data_property_value1054.as_slice(),
                                    })
                                    .len() as f64)
                                        > 4096f64)
                                {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(String::from("ordinary properties exceed their explicit ordered storage budget"))
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source1_s1055 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop14(heap.clone(), object.clone(), at);
                                            match source_ml_source1_s1055 {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done => {
                                                    {
                                                        let prototype = match object.clone() {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1056, _, _, _) => {
                                                                let source_ml_source1_data_property_value1056 = *source_ml_source1_data_property_value1056;
                                                                source_ml_source1_data_property_value1056.clone()
                                                            }
                                                        };
                                                        {
                                                            let depth = 0f64;
                                                            {
                                                                let source_ml_source1_s1057 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop15(heap.clone(), object.clone(), prototype, depth);
                                                                match source_ml_source1_s1057 {
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
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(String),
}
