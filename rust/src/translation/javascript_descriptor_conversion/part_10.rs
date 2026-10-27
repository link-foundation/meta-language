// Generated from complete js/src/translation/javascript-descriptor-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::read_java_script_literal_property_index;

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
                                source_ml_source1_data_property_value1059,
                                _,
                            ) => source_ml_source1_data_property_value1059,
                        },
                        symbol.clone(),
                    )) >= 0f64)
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(String::from("Symbols and Objects need distinct strictly ordered positive unsigned64 identities"))
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
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(String),
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
                        source_ml_source1_data_property_value1061,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value1061.clone(),
                }) <= previous)
                    || ((match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source1_data_property_value1063,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value1063.clone(),
                    }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615)))
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(String::from("ordinary Objects need strictly ordered unique positive unsigned64 identities"))
                } else {
                    {
                        let previous_2 = match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value1064,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value1064.clone(),
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result {
    MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done,
    MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Return(
        (
            (
                bool,
                bool,
                bool,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                bool,
                bool,
            ),
            bool,
            self::ml::Big,
        ),
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_locate_java_script_ordinary_property_loop18(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut key: self::JavaScriptSemanticValue,
    mut current: self::ml::Big,
    mut depth: f64,
) -> self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result {
    loop {
        return if depth
            < ((match &heap {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    source_ml_source1_data_property_value1066,
                    _,
                ) => source_ml_source1_data_property_value1066.as_slice(),
            })
            .len() as f64)
        {
            {
                let index = self::find_java_script_ordinary_object(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value1067,
                            _,
                        ) => source_ml_source1_data_property_value1067,
                    },
                    current.clone(),
                );
                {
                    let object = self::ml_array::optional_at(
                        (match &heap {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value1068,
                                _,
                            ) => source_ml_source1_data_property_value1068.as_slice(),
                        }),
                        self::ml_array::number_index(index),
                    );
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                    } else {
                        {
                            let at = self::find_java_script_ordinary_property(
                                {
                                    let source_ml_source1_nullish_field_receiver1069 =
                                        (object.clone(),);
                                    if match (source_ml_source1_nullish_field_receiver1069.clone())
                                        .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source1_nullish_field_present1070 =
                                                ((source_ml_source1_nullish_field_receiver1069
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source1_nullish_field_present1070 {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source1_data_property_value1071,
                                                ) => source_ml_source1_data_property_value1071,
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'properties')")).encode_utf16().collect(), message_present: true }).raise()
                                    }
                                },
                                key.clone(),
                            );
                            {
                                let property = self::ml_array::optional_at(
                                    &{
                                        let source_ml_source1_nullish_field_receiver1072 =
                                            (object.clone(),);
                                        if match (source_ml_source1_nullish_field_receiver1072
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
                                                let source_ml_source1_nullish_field_present1073 = ((source_ml_source1_nullish_field_receiver1072.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source1_nullish_field_present1073 {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1074) => {
                                                    source_ml_source1_data_property_value1074
                                                }
                                            }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'properties')")).encode_utf16().collect(), message_present: true }).raise()
                                        }
                                    },
                                    self::ml_array::number_index(at),
                                );
                                if (match (property.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    {
                                        let prototype = {
                                            let source_ml_source1_nullish_field_receiver1081 =
                                                (object.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver1081
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
                                                    let source_ml_source1_nullish_field_present1082 = ((source_ml_source1_nullish_field_receiver1081.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present1082 {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1083, _, _, _) => {
                                                            let source_ml_source1_data_property_value1083 = *source_ml_source1_data_property_value1083;
                                                            source_ml_source1_data_property_value1083.clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'prototype')")).encode_utf16().collect(), message_present: true }).raise()
                                            }
                                        };
                                        match prototype {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1084) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1085) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1086) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1087) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1088) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Object(current_2, source_ml_source1_f1089) => {
                                                {
                                                    let depth_2 = depth + 1f64;
                                                    {
                                                        (current, depth) = (current_2.clone(), depth_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Return({
                                        let source_ml_source1_field1075 = true;
                                        {
                                            let source_ml_source1_field1076 = current.clone();
                                            {
                                                let source_ml_source1_field1080 = {
                                                    let source_ml_source1_nullish_field_receiver1077 = (property.clone(),);
                                                    if match (source_ml_source1_nullish_field_receiver1077.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        {
                                                            let source_ml_source1_nullish_field_present1078 = ((source_ml_source1_nullish_field_receiver1077.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_source1_nullish_field_present1078 {
                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1079) => {
                                                                    source_ml_source1_data_property_value1079.clone()
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'descriptor')")).encode_utf16().collect(), message_present: true }).raise()
                                                    }
                                                };
                                                (source_ml_source1_field1080, source_ml_source1_field1075, source_ml_source1_field1076)
                                            }
                                        }
                                    })
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_define_java_script_ordinary_property_loop20(
    mut count: f64,
    mut source_ml_temporary_values19: Vec<self::JavaScriptOrdinaryObject>,
    mut source_ml_temporary_index19: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index19
            < (self::ml::Big::from_u128(source_ml_temporary_values19.len() as u128))
        {
            {
                let entry = self::ml_array::at(
                    &source_ml_temporary_values19,
                    source_ml_temporary_index19.to_index(),
                );
                {
                    let count_2 = count
                        + ((match &entry {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value1090,
                            ) => source_ml_source1_data_property_value1090.as_slice(),
                        })
                        .len() as f64);
                    {
                        let source_ml_temporary_index19_2 =
                            source_ml_temporary_index19.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_temporary_index19) =
                                (count_2, source_ml_temporary_index19_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { count };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result {
    MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done,
    MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return(
        (String, self::JavaScriptOrdinaryHeap, bool),
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_set_java_script_ordinary_prototype_loop21(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut identity: self::ml::Big,
    mut next: self::JavaScriptSemanticValue,
    mut depth: f64,
) -> self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result {
    loop {
        return if depth
            <= ((match &heap {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    source_ml_source1_data_property_value1092,
                    _,
                ) => source_ml_source1_data_property_value1092.as_slice(),
            })
            .len() as f64)
        {
            match next.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Null => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1093) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f1094) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1095) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f1096) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1097) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Object(next_identity, source_ml_source1_f1098) => {
                    if next_identity == identity {
                        self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return({
                            let source_ml_source1_field1099 = heap.clone();
                            {
                                let source_ml_source1_field1100 = false;
                                {
                                    let source_ml_source1_field1101 = String::new();
                                    (source_ml_source1_field1101, source_ml_source1_field1099, source_ml_source1_field1100)
                                }
                            }
                        })
                    } else {
                        {
                            let at = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1102, _) => {
                                    source_ml_source1_data_property_value1102
                                }
                            }, next_identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1103, _) => source_ml_source1_data_property_value1103.as_slice() }), self::ml_array::number_index(at));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return({
                                        let source_ml_source1_field1104 = heap.clone();
                                        {
                                            let source_ml_source1_field1105 = false;
                                            {
                                                let source_ml_source1_field1106 = String::from("every ordinary prototype must remain in the global directory");
                                                (source_ml_source1_field1106, source_ml_source1_field1104, source_ml_source1_field1105)
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let next_2 = {
                                            let source_ml_source1_nullish_field_receiver1107 = (parent.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver1107.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present1108 = ((source_ml_source1_nullish_field_receiver1107.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present1108 {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1109, _, _, _) => {
                                                            let source_ml_source1_data_property_value1109 = *source_ml_source1_data_property_value1109;
                                                            source_ml_source1_data_property_value1109.clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'prototype')")).encode_utf16().collect(), message_present: true }).raise()
                                            }
                                        };
                                        {
                                            let depth_2 = depth + 1f64;
                                            {
                                                (next, depth) = (next_2, depth_2);
                                                continue;
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
            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result {
    MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Done(
        Vec<self::JavaScriptSemanticValue>,
        Vec<self::JavaScriptSemanticValue>,
        Vec<self::JavaScriptSemanticValue>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_own_keys_loop24(
    positions: &[f64],
    mut position: f64,
    mut at: f64,
) -> f64 {
    loop {
        if (at < (positions.len() as f64))
            && (((self::ml_array::optional_at(positions, self::ml_array::number_index(at)))
                .unwrap_or_else(|| -1f64))
                < position)
        {
            {
                let at_2 = at + 1f64;
                {
                    at = at_2;
                    continue;
                }
            }
        }
        return { at };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_own_keys_loop23(
    mut numeric: Vec<self::JavaScriptSemanticValue>,
    mut positions: Vec<f64>,
    mut text: Vec<self::JavaScriptSemanticValue>,
    mut symbols: Vec<self::JavaScriptSemanticValue>,
    mut source_ml_temporary_values22: Vec<self::JavaScriptOrdinaryProperty>,
    mut source_ml_temporary_index22: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result {
    loop {
        return if source_ml_temporary_index22
            < (self::ml::Big::from_u128(source_ml_temporary_values22.len() as u128))
        {
            {
                let property = self::ml_array::at(
                    &source_ml_temporary_values22,
                    source_ml_temporary_index22.to_index(),
                );
                {
                    let key = match property {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value1110,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value1110 =
                                *source_ml_source1_data_property_value1110;
                            source_ml_source1_data_property_value1110.clone()
                        }
                    };
                    match key.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            let symbols_2 = {
                                let source_ml_source1_concat_value1117 = symbols;
                                {
                                    let source_ml_source1_concat_value1118 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1117,
                                        ),
                                        source_ml_source1_concat_value1118,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_2 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_2, source_ml_temporary_index22_2);
                                    continue;
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Null => {
                            let symbols_3 = {
                                let source_ml_source1_concat_value1119 = symbols;
                                {
                                    let source_ml_source1_concat_value1120 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1119,
                                        ),
                                        source_ml_source1_concat_value1120,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_3 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_3, source_ml_temporary_index22_3);
                                    continue;
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1111) => {
                            let symbols_4 = {
                                let source_ml_source1_concat_value1121 = symbols;
                                {
                                    let source_ml_source1_concat_value1122 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1121,
                                        ),
                                        source_ml_source1_concat_value1122,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_4 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_4, source_ml_temporary_index22_4);
                                    continue;
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1112) => {
                            let symbols_5 = {
                                let source_ml_source1_concat_value1123 = symbols;
                                {
                                    let source_ml_source1_concat_value1124 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1123,
                                        ),
                                        source_ml_source1_concat_value1124,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_5 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_5, source_ml_temporary_index22_5);
                                    continue;
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1113) => {
                            let symbols_6 = {
                                let source_ml_source1_concat_value1125 = symbols;
                                {
                                    let source_ml_source1_concat_value1126 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1125,
                                        ),
                                        source_ml_source1_concat_value1126,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_6 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_6, source_ml_temporary_index22_6);
                                    continue;
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Text(units) => {
                            let position = self::read_java_script_literal_property_index(&(units));
                            if position < 0f64 {
                                {
                                    let text_2 = {
                                        let source_ml_source1_concat_value1127 = text;
                                        {
                                            let source_ml_source1_concat_value1128 =
                                                vec![key.clone()];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::JavaScriptSemanticValue>::new(),
                                                    source_ml_source1_concat_value1127,
                                                ),
                                                source_ml_source1_concat_value1128,
                                            )
                                        }
                                    };
                                    {
                                        let source_ml_temporary_index22_7 =
                                            source_ml_temporary_index22
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            (text, source_ml_temporary_index22) =
                                                (text_2, source_ml_temporary_index22_7);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                {
                                    let at = 0f64;
                                    {
                                        let at_2 = self::source_ml_temporary_read_java_script_ordinary_own_keys_loop24(&(positions), position, at);
                                        {
                                            let numeric_2 = {
                                                let source_ml_source1_concat_value1129 =
                                                    self::ml_array::slice(&numeric, 0f64, at_2);
                                                {
                                                    let source_ml_source1_concat_value1130 =
                                                        vec![key.clone()];
                                                    {
                                                        let source_ml_source1_concat_value1131 =
                                                            self::ml_array::slice(
                                                                &numeric,
                                                                at_2,
                                                                f64::INFINITY,
                                                            );
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1129), source_ml_source1_concat_value1130), source_ml_source1_concat_value1131)
                                                    }
                                                }
                                            };
                                            {
                                                let positions_2 = {
                                                    let source_ml_source1_concat_value1132 =
                                                        self::ml_array::slice(
                                                            &positions, 0f64, at_2,
                                                        );
                                                    {
                                                        let source_ml_source1_concat_value1133 =
                                                            vec![position];
                                                        {
                                                            let source_ml_source1_concat_value1134 =
                                                                self::ml_array::slice(
                                                                    &positions,
                                                                    at_2,
                                                                    f64::INFINITY,
                                                                );
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value1132), source_ml_source1_concat_value1133), source_ml_source1_concat_value1134)
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_index22_8 =
                                                        source_ml_temporary_index22
                                                            .add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (
                                                            numeric,
                                                            positions,
                                                            source_ml_temporary_index22,
                                                        ) = (
                                                            numeric_2,
                                                            positions_2,
                                                            source_ml_temporary_index22_8,
                                                        );
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1114) => {
                            let symbols_7 = {
                                let source_ml_source1_concat_value1135 = symbols;
                                {
                                    let source_ml_source1_concat_value1136 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1135,
                                        ),
                                        source_ml_source1_concat_value1136,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_9 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_7, source_ml_temporary_index22_9);
                                    continue;
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f1115,
                            source_ml_source1_f1116,
                        ) => {
                            let symbols_8 = {
                                let source_ml_source1_concat_value1137 = symbols;
                                {
                                    let source_ml_source1_concat_value1138 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1137,
                                        ),
                                        source_ml_source1_concat_value1138,
                                    )
                                }
                            };
                            {
                                let source_ml_temporary_index22_10 =
                                    source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (symbols, source_ml_temporary_index22) =
                                        (symbols_8, source_ml_temporary_index22_10);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Done(numeric, text, symbols)
        };
    }
}
