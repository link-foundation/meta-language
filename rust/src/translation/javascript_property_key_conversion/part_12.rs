// Generated from complete js/src/translation/javascript-property-key-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_literal_property_index;

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
                    source_ml_source1_data_property_value1025,
                    _,
                ) => source_ml_source1_data_property_value1025.as_slice(),
            })
            .len() as f64)
        {
            {
                let index = self::find_java_script_ordinary_object(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value1026,
                            _,
                        ) => source_ml_source1_data_property_value1026,
                    },
                    current.clone(),
                );
                {
                    let object = self::ml_array::optional_at(
                        (match &heap {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value1027,
                                _,
                            ) => source_ml_source1_data_property_value1027.as_slice(),
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
                                    let source_ml_source1_nullish_field_receiver1028 =
                                        (object.clone(),);
                                    if match (source_ml_source1_nullish_field_receiver1028.clone())
                                        .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source1_nullish_field_present1029 =
                                                ((source_ml_source1_nullish_field_receiver1028
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source1_nullish_field_present1029 {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source1_data_property_value1030,
                                                ) => source_ml_source1_data_property_value1030,
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
                                                    97, 100, 105, 110, 103, 32, 39, 112, 114, 111,
                                                    112, 101, 114, 116, 105, 101, 115, 39, 41,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    }
                                },
                                key.clone(),
                            );
                            {
                                let property = self::ml_array::optional_at(
                                    &{
                                        let source_ml_source1_nullish_field_receiver1031 =
                                            (object.clone(),);
                                        if match (source_ml_source1_nullish_field_receiver1031
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
                                                let source_ml_source1_nullish_field_present1032 = ((source_ml_source1_nullish_field_receiver1031.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source1_nullish_field_present1032 {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1033) => {
                                                    source_ml_source1_data_property_value1033
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
                                                        39, 112, 114, 111, 112, 101, 114, 116, 105,
                                                        101, 115, 39, 41,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
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
                                            let source_ml_source1_nullish_field_receiver1040 =
                                                (object.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver1040
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
                                                    let source_ml_source1_nullish_field_present1041 = ((source_ml_source1_nullish_field_receiver1040.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present1041 {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1042, _, _, _) => {
                                                            let source_ml_source1_data_property_value1042 = *source_ml_source1_data_property_value1042;
                                                            source_ml_source1_data_property_value1042.clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException {
                                                    class_name: String::from("TypeError"),
                                                    message_units:
                                                        (self::ml_string::Text::from_units(vec![
                                                            67, 97, 110, 110, 111, 116, 32, 114,
                                                            101, 97, 100, 32, 112, 114, 111, 112,
                                                            101, 114, 116, 105, 101, 115, 32, 111,
                                                            102, 32, 117, 110, 100, 101, 102, 105,
                                                            110, 101, 100, 32, 40, 114, 101, 97,
                                                            100, 105, 110, 103, 32, 39, 112, 114,
                                                            111, 116, 111, 116, 121, 112, 101, 39,
                                                            41,
                                                        ]))
                                                        .0,
                                                    message_present: true,
                                                })
                                                .raise()
                                            }
                                        };
                                        match prototype {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1043) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1044) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1045) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1046) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1047) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Object(current_2, source_ml_source1_f1048) => {
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
                                        let source_ml_source1_field1034 = true;
                                        {
                                            let source_ml_source1_field1035 = current.clone();
                                            {
                                                let source_ml_source1_field1039 = {
                                                    let source_ml_source1_nullish_field_receiver1036 = (property.clone(),);
                                                    if match (source_ml_source1_nullish_field_receiver1036.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        {
                                                            let source_ml_source1_nullish_field_present1037 = ((source_ml_source1_nullish_field_receiver1036.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_source1_nullish_field_present1037 {
                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1038) => {
                                                                    source_ml_source1_data_property_value1038.clone()
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                    }
                                                };
                                                (source_ml_source1_field1039, source_ml_source1_field1034, source_ml_source1_field1035)
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
                                source_ml_source1_data_property_value1049,
                            ) => source_ml_source1_data_property_value1049.as_slice(),
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
        (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool),
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
                    source_ml_source1_data_property_value1051,
                    _,
                ) => source_ml_source1_data_property_value1051.as_slice(),
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1052) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f1053) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1054) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f1055) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1056) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Object(next_identity, source_ml_source1_f1057) => {
                    if next_identity == identity {
                        self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return({
                            let source_ml_source1_field1058 = heap.clone();
                            {
                                let source_ml_source1_field1059 = false;
                                {
                                    let source_ml_source1_field1060 = self::ml_string::Text::from_units(vec![]);
                                    (source_ml_source1_field1060, source_ml_source1_field1058, source_ml_source1_field1059)
                                }
                            }
                        })
                    } else {
                        {
                            let at = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1061, _) => {
                                    source_ml_source1_data_property_value1061
                                }
                            }, next_identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1062, _) => source_ml_source1_data_property_value1062.as_slice() }), self::ml_array::number_index(at));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return({
                                        let source_ml_source1_field1063 = heap.clone();
                                        {
                                            let source_ml_source1_field1064 = false;
                                            {
                                                let source_ml_source1_field1065 = self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]);
                                                (source_ml_source1_field1065, source_ml_source1_field1063, source_ml_source1_field1064)
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let next_2 = {
                                            let source_ml_source1_nullish_field_receiver1066 = (parent.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver1066.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present1067 = ((source_ml_source1_nullish_field_receiver1066.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present1067 {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1068, _, _, _) => {
                                                            let source_ml_source1_data_property_value1068 = *source_ml_source1_data_property_value1068;
                                                            source_ml_source1_data_property_value1068.clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
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
                            source_ml_source1_data_property_value1069,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value1069 =
                                *source_ml_source1_data_property_value1069;
                            source_ml_source1_data_property_value1069.clone()
                        }
                    };
                    match key.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            let symbols_2 = {
                                let source_ml_source1_concat_value1076 = symbols;
                                {
                                    let source_ml_source1_concat_value1077 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1076,
                                        ),
                                        source_ml_source1_concat_value1077,
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
                                let source_ml_source1_concat_value1078 = symbols;
                                {
                                    let source_ml_source1_concat_value1079 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1078,
                                        ),
                                        source_ml_source1_concat_value1079,
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1070) => {
                            let symbols_4 = {
                                let source_ml_source1_concat_value1080 = symbols;
                                {
                                    let source_ml_source1_concat_value1081 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1080,
                                        ),
                                        source_ml_source1_concat_value1081,
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
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1071) => {
                            let symbols_5 = {
                                let source_ml_source1_concat_value1082 = symbols;
                                {
                                    let source_ml_source1_concat_value1083 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1082,
                                        ),
                                        source_ml_source1_concat_value1083,
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
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1072) => {
                            let symbols_6 = {
                                let source_ml_source1_concat_value1084 = symbols;
                                {
                                    let source_ml_source1_concat_value1085 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1084,
                                        ),
                                        source_ml_source1_concat_value1085,
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
                                        let source_ml_source1_concat_value1086 = text;
                                        {
                                            let source_ml_source1_concat_value1087 =
                                                vec![key.clone()];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::JavaScriptSemanticValue>::new(),
                                                    source_ml_source1_concat_value1086,
                                                ),
                                                source_ml_source1_concat_value1087,
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
                                                let source_ml_source1_concat_value1088 =
                                                    self::ml_array::slice(&numeric, 0f64, at_2);
                                                {
                                                    let source_ml_source1_concat_value1089 =
                                                        vec![key.clone()];
                                                    {
                                                        let source_ml_source1_concat_value1090 =
                                                            self::ml_array::slice(
                                                                &numeric,
                                                                at_2,
                                                                f64::INFINITY,
                                                            );
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1088), source_ml_source1_concat_value1089), source_ml_source1_concat_value1090)
                                                    }
                                                }
                                            };
                                            {
                                                let positions_2 = {
                                                    let source_ml_source1_concat_value1091 =
                                                        self::ml_array::slice(
                                                            &positions, 0f64, at_2,
                                                        );
                                                    {
                                                        let source_ml_source1_concat_value1092 =
                                                            vec![position];
                                                        {
                                                            let source_ml_source1_concat_value1093 =
                                                                self::ml_array::slice(
                                                                    &positions,
                                                                    at_2,
                                                                    f64::INFINITY,
                                                                );
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value1091), source_ml_source1_concat_value1092), source_ml_source1_concat_value1093)
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
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1073) => {
                            let symbols_7 = {
                                let source_ml_source1_concat_value1094 = symbols;
                                {
                                    let source_ml_source1_concat_value1095 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1094,
                                        ),
                                        source_ml_source1_concat_value1095,
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
                            source_ml_source1_f1074,
                            source_ml_source1_f1075,
                        ) => {
                            let symbols_8 = {
                                let source_ml_source1_concat_value1096 = symbols;
                                {
                                    let source_ml_source1_concat_value1097 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1096,
                                        ),
                                        source_ml_source1_concat_value1097,
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_insert_java_script_ordinary_object_loop26(
    mut identity: self::ml::Big,
    mut at: f64,
    mut source_ml_temporary_values25: Vec<self::JavaScriptOrdinaryObject>,
    mut source_ml_temporary_index25: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index25
            < (self::ml::Big::from_u128(source_ml_temporary_values25.len() as u128))
        {
            {
                let object = self::ml_array::at(
                    &source_ml_temporary_values25,
                    source_ml_temporary_index25.to_index(),
                );
                if (match object {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        source_ml_source1_data_property_value1099,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value1099.clone(),
                }) < identity
                {
                    {
                        let at_2 = at + 1f64;
                        {
                            let source_ml_temporary_index25_2 =
                                source_ml_temporary_index25.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index25) =
                                    (at_2, source_ml_temporary_index25_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_temporary_index25_3 =
                            source_ml_temporary_index25.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index25 = source_ml_temporary_index25_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { at };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_register_java_script_ordinary_symbol_loop28(
    mut identity: self::ml::Big,
    mut at: f64,
    mut source_ml_temporary_values27: Vec<self::ml::Big>,
    mut source_ml_temporary_index27: self::ml::Big,
) -> f64 {
    loop {
        if source_ml_temporary_index27
            < (self::ml::Big::from_u128(source_ml_temporary_values27.len() as u128))
        {
            {
                let symbol = self::ml_array::at(
                    &source_ml_temporary_values27,
                    source_ml_temporary_index27.to_index(),
                );
                if symbol < identity {
                    {
                        let at_2 = at + 1f64;
                        {
                            let source_ml_temporary_index27_2 =
                                source_ml_temporary_index27.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index27) =
                                    (at_2, source_ml_temporary_index27_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_temporary_index27_3 =
                            source_ml_temporary_index27.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index27 = source_ml_temporary_index27_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { at };
    }
}
