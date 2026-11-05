// Generated from complete js/src/translation/javascript-primitive-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::read_java_script_literal_property_index;

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
                        source_ml_source1_data_property_value989,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value989.clone(),
                }) <= previous)
                    || ((match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source1_data_property_value991,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value991.clone(),
                    }) > (self::ml::Big::from_i128(18_446_744_073_709_551_615)))
                {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(String::from("ordinary Objects need strictly ordered unique positive unsigned64 identities"))
                } else {
                    {
                        let previous_2 = match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value992,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value992.clone(),
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
                    source_ml_source1_data_property_value994,
                    _,
                ) => source_ml_source1_data_property_value994.as_slice(),
            })
            .len() as f64)
        {
            {
                let index = self::find_java_script_ordinary_object(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value995,
                            _,
                        ) => source_ml_source1_data_property_value995,
                    },
                    current.clone(),
                );
                {
                    let object = self::ml_array::optional_at(
                        (match &heap {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value996,
                                _,
                            ) => source_ml_source1_data_property_value996.as_slice(),
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
                                    let source_ml_source1_nullish_field_receiver997 =
                                        (object.clone(),);
                                    if match (source_ml_source1_nullish_field_receiver997.clone()).0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source1_nullish_field_present998 =
                                                ((source_ml_source1_nullish_field_receiver997
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source1_nullish_field_present998 {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source1_data_property_value999,
                                                ) => source_ml_source1_data_property_value999,
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
                                        let source_ml_source1_nullish_field_receiver1000 =
                                            (object.clone(),);
                                        if match (source_ml_source1_nullish_field_receiver1000
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
                                                let source_ml_source1_nullish_field_present1001 = ((source_ml_source1_nullish_field_receiver1000.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source1_nullish_field_present1001 {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1002) => {
                                                    source_ml_source1_data_property_value1002
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
                                            let source_ml_source1_nullish_field_receiver1009 =
                                                (object.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver1009
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
                                        match prototype {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1012) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1013) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1014) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1015) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1016) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Object(current_2, source_ml_source1_f1017) => {
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
                                        let source_ml_source1_field1003 = true;
                                        {
                                            let source_ml_source1_field1004 = current.clone();
                                            {
                                                let source_ml_source1_field1008 = {
                                                    let source_ml_source1_nullish_field_receiver1005 = (property.clone(),);
                                                    if match (source_ml_source1_nullish_field_receiver1005.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        {
                                                            let source_ml_source1_nullish_field_present1006 = ((source_ml_source1_nullish_field_receiver1005.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_source1_nullish_field_present1006 {
                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1007) => {
                                                                    source_ml_source1_data_property_value1007.clone()
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'descriptor')")).encode_utf16().collect(), message_present: true }).raise()
                                                    }
                                                };
                                                (source_ml_source1_field1008, source_ml_source1_field1003, source_ml_source1_field1004)
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
                                source_ml_source1_data_property_value1018,
                            ) => source_ml_source1_data_property_value1018.as_slice(),
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
                    source_ml_source1_data_property_value1020,
                    _,
                ) => source_ml_source1_data_property_value1020.as_slice(),
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1021) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f1022) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1023) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f1024) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1025) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Object(next_identity, source_ml_source1_f1026) => {
                    if next_identity == identity {
                        self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return({
                            let source_ml_source1_field1027 = heap.clone();
                            {
                                let source_ml_source1_field1028 = false;
                                {
                                    let source_ml_source1_field1029 = String::new();
                                    (source_ml_source1_field1029, source_ml_source1_field1027, source_ml_source1_field1028)
                                }
                            }
                        })
                    } else {
                        {
                            let at = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1030, _) => {
                                    source_ml_source1_data_property_value1030
                                }
                            }, next_identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1031, _) => source_ml_source1_data_property_value1031.as_slice() }), self::ml_array::number_index(at));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return({
                                        let source_ml_source1_field1032 = heap.clone();
                                        {
                                            let source_ml_source1_field1033 = false;
                                            {
                                                let source_ml_source1_field1034 = String::from("every ordinary prototype must remain in the global directory");
                                                (source_ml_source1_field1034, source_ml_source1_field1032, source_ml_source1_field1033)
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let next_2 = {
                                            let source_ml_source1_nullish_field_receiver1035 = (parent.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver1035.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present1036 = ((source_ml_source1_nullish_field_receiver1035.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present1036 {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1037, _, _, _) => {
                                                            let source_ml_source1_data_property_value1037 = *source_ml_source1_data_property_value1037;
                                                            source_ml_source1_data_property_value1037.clone()
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
                            source_ml_source1_data_property_value1038,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value1038 =
                                *source_ml_source1_data_property_value1038;
                            source_ml_source1_data_property_value1038.clone()
                        }
                    };
                    match key.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            let symbols_2 = {
                                let source_ml_source1_concat_value1045 = symbols;
                                {
                                    let source_ml_source1_concat_value1046 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1045,
                                        ),
                                        source_ml_source1_concat_value1046,
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
                                let source_ml_source1_concat_value1047 = symbols;
                                {
                                    let source_ml_source1_concat_value1048 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1047,
                                        ),
                                        source_ml_source1_concat_value1048,
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1039) => {
                            let symbols_4 = {
                                let source_ml_source1_concat_value1049 = symbols;
                                {
                                    let source_ml_source1_concat_value1050 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1049,
                                        ),
                                        source_ml_source1_concat_value1050,
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
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1040) => {
                            let symbols_5 = {
                                let source_ml_source1_concat_value1051 = symbols;
                                {
                                    let source_ml_source1_concat_value1052 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1051,
                                        ),
                                        source_ml_source1_concat_value1052,
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
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1041) => {
                            let symbols_6 = {
                                let source_ml_source1_concat_value1053 = symbols;
                                {
                                    let source_ml_source1_concat_value1054 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1053,
                                        ),
                                        source_ml_source1_concat_value1054,
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
                                        let source_ml_source1_concat_value1055 = text;
                                        {
                                            let source_ml_source1_concat_value1056 =
                                                vec![key.clone()];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::JavaScriptSemanticValue>::new(),
                                                    source_ml_source1_concat_value1055,
                                                ),
                                                source_ml_source1_concat_value1056,
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
                                                let source_ml_source1_concat_value1057 =
                                                    self::ml_array::slice(&numeric, 0f64, at_2);
                                                {
                                                    let source_ml_source1_concat_value1058 =
                                                        vec![key.clone()];
                                                    {
                                                        let source_ml_source1_concat_value1059 =
                                                            self::ml_array::slice(
                                                                &numeric,
                                                                at_2,
                                                                f64::INFINITY,
                                                            );
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1057), source_ml_source1_concat_value1058), source_ml_source1_concat_value1059)
                                                    }
                                                }
                                            };
                                            {
                                                let positions_2 = {
                                                    let source_ml_source1_concat_value1060 =
                                                        self::ml_array::slice(
                                                            &positions, 0f64, at_2,
                                                        );
                                                    {
                                                        let source_ml_source1_concat_value1061 =
                                                            vec![position];
                                                        {
                                                            let source_ml_source1_concat_value1062 =
                                                                self::ml_array::slice(
                                                                    &positions,
                                                                    at_2,
                                                                    f64::INFINITY,
                                                                );
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value1060), source_ml_source1_concat_value1061), source_ml_source1_concat_value1062)
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
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1042) => {
                            let symbols_7 = {
                                let source_ml_source1_concat_value1063 = symbols;
                                {
                                    let source_ml_source1_concat_value1064 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1063,
                                        ),
                                        source_ml_source1_concat_value1064,
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
                            source_ml_source1_f1043,
                            source_ml_source1_f1044,
                        ) => {
                            let symbols_8 = {
                                let source_ml_source1_concat_value1065 = symbols;
                                {
                                    let source_ml_source1_concat_value1066 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value1065,
                                        ),
                                        source_ml_source1_concat_value1066,
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
                        source_ml_source1_data_property_value1068,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value1068.clone(),
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
