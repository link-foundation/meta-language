// Generated from complete js/src/translation/javascript-ordinary-heap.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::read_java_script_literal_property_index;

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
            < ((match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    source_ml_source1_data_property_value873,
                    _,
                ) => source_ml_source1_data_property_value873,
            })
            .len() as f64)
        {
            {
                let index = self::find_java_script_ordinary_object(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value874,
                            _,
                        ) => source_ml_source1_data_property_value874,
                    },
                    current.clone(),
                );
                {
                    let object = self::ml_array::optional_at(
                        &match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value875,
                                _,
                            ) => source_ml_source1_data_property_value875,
                        },
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
                                    let source_ml_source1_nullish_field_receiver876 =
                                        (object.clone(),);
                                    if match (source_ml_source1_nullish_field_receiver876.clone()).0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source1_nullish_field_present877 =
                                                ((source_ml_source1_nullish_field_receiver876
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source1_nullish_field_present877 {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source1_data_property_value878,
                                                ) => source_ml_source1_data_property_value878,
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
                                        let source_ml_source1_nullish_field_receiver879 =
                                            (object.clone(),);
                                        if match (source_ml_source1_nullish_field_receiver879
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
                                                let source_ml_source1_nullish_field_present880 =
                                                    ((source_ml_source1_nullish_field_receiver879
                                                        .clone())
                                                    .0)
                                                        .clone()
                                                        .expect("a proven present value");
                                                match source_ml_source1_nullish_field_present880 {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value881) => {
                                                    source_ml_source1_data_property_value881
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
                                            let source_ml_source1_nullish_field_receiver888 =
                                                (object.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver888
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
                                                    let source_ml_source1_nullish_field_present889 = ((source_ml_source1_nullish_field_receiver888.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present889 {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value890, _, _, _) => {
                                                            let source_ml_source1_data_property_value890 = *source_ml_source1_data_property_value890;
                                                            source_ml_source1_data_property_value890.clone()
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
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f891) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source1_f892) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f893) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source1_f894) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f895) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Object(current_2, source_ml_source1_f896) => {
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
                                        let source_ml_source1_field882 = true;
                                        {
                                            let source_ml_source1_field883 = current.clone();
                                            {
                                                let source_ml_source1_field887 = {
                                                    let source_ml_source1_nullish_field_receiver884 = (property.clone(),);
                                                    if match (source_ml_source1_nullish_field_receiver884.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        {
                                                            let source_ml_source1_nullish_field_present885 = ((source_ml_source1_nullish_field_receiver884.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_source1_nullish_field_present885 {
                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value886) => {
                                                                    source_ml_source1_data_property_value886.clone()
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'descriptor')")).encode_utf16().collect(), message_present: true }).raise()
                                                    }
                                                };
                                                (source_ml_source1_field887, source_ml_source1_field882, source_ml_source1_field883)
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
                        + ((match entry {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value897,
                            ) => source_ml_source1_data_property_value897,
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
            <= ((match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    source_ml_source1_data_property_value899,
                    _,
                ) => source_ml_source1_data_property_value899,
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f900) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f901) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f902) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f903) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f904) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Object(next_identity, source_ml_source1_f905) => {
                    if next_identity == identity {
                        self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return({
                            let source_ml_source1_field906 = heap.clone();
                            {
                                let source_ml_source1_field907 = false;
                                {
                                    let source_ml_source1_field908 = String::new();
                                    (source_ml_source1_field908, source_ml_source1_field906, source_ml_source1_field907)
                                }
                            }
                        })
                    } else {
                        {
                            let at = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value909, _) => {
                                    source_ml_source1_data_property_value909
                                }
                            }, next_identity.clone());
                            {
                                let parent = self::ml_array::optional_at(&match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value910, _) => {
                                        source_ml_source1_data_property_value910
                                    }
                                }, self::ml_array::number_index(at));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return({
                                        let source_ml_source1_field911 = heap.clone();
                                        {
                                            let source_ml_source1_field912 = false;
                                            {
                                                let source_ml_source1_field913 = String::from("every ordinary prototype must remain in the global directory");
                                                (source_ml_source1_field913, source_ml_source1_field911, source_ml_source1_field912)
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let next_2 = {
                                            let source_ml_source1_nullish_field_receiver914 = (parent.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver914.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present915 = ((source_ml_source1_nullish_field_receiver914.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present915 {
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value916, _, _, _) => {
                                                            let source_ml_source1_data_property_value916 = *source_ml_source1_data_property_value916;
                                                            source_ml_source1_data_property_value916.clone()
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
                            source_ml_source1_data_property_value917,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value917 =
                                *source_ml_source1_data_property_value917;
                            source_ml_source1_data_property_value917.clone()
                        }
                    };
                    match key.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            let symbols_2 = {
                                let source_ml_source1_concat_value924 = symbols;
                                {
                                    let source_ml_source1_concat_value925 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value924,
                                        ),
                                        source_ml_source1_concat_value925,
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
                                let source_ml_source1_concat_value926 = symbols;
                                {
                                    let source_ml_source1_concat_value927 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value926,
                                        ),
                                        source_ml_source1_concat_value927,
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f918) => {
                            let symbols_4 = {
                                let source_ml_source1_concat_value928 = symbols;
                                {
                                    let source_ml_source1_concat_value929 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value928,
                                        ),
                                        source_ml_source1_concat_value929,
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
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f919) => {
                            let symbols_5 = {
                                let source_ml_source1_concat_value930 = symbols;
                                {
                                    let source_ml_source1_concat_value931 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value930,
                                        ),
                                        source_ml_source1_concat_value931,
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
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f920) => {
                            let symbols_6 = {
                                let source_ml_source1_concat_value932 = symbols;
                                {
                                    let source_ml_source1_concat_value933 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value932,
                                        ),
                                        source_ml_source1_concat_value933,
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
                                        let source_ml_source1_concat_value934 = text;
                                        {
                                            let source_ml_source1_concat_value935 =
                                                vec![key.clone()];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::JavaScriptSemanticValue>::new(),
                                                    source_ml_source1_concat_value934,
                                                ),
                                                source_ml_source1_concat_value935,
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
                                                let source_ml_source1_concat_value936 =
                                                    self::ml_array::slice(&numeric, 0f64, at_2);
                                                {
                                                    let source_ml_source1_concat_value937 =
                                                        vec![key.clone()];
                                                    {
                                                        let source_ml_source1_concat_value938 =
                                                            self::ml_array::slice(
                                                                &numeric,
                                                                at_2,
                                                                f64::INFINITY,
                                                            );
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value936), source_ml_source1_concat_value937), source_ml_source1_concat_value938)
                                                    }
                                                }
                                            };
                                            {
                                                let positions_2 = {
                                                    let source_ml_source1_concat_value939 =
                                                        self::ml_array::slice(
                                                            &positions, 0f64, at_2,
                                                        );
                                                    {
                                                        let source_ml_source1_concat_value940 =
                                                            vec![position];
                                                        {
                                                            let source_ml_source1_concat_value941 =
                                                                self::ml_array::slice(
                                                                    &positions,
                                                                    at_2,
                                                                    f64::INFINITY,
                                                                );
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value939), source_ml_source1_concat_value940), source_ml_source1_concat_value941)
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
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f921) => {
                            let symbols_7 = {
                                let source_ml_source1_concat_value942 = symbols;
                                {
                                    let source_ml_source1_concat_value943 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value942,
                                        ),
                                        source_ml_source1_concat_value943,
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
                            source_ml_source1_f922,
                            source_ml_source1_f923,
                        ) => {
                            let symbols_8 = {
                                let source_ml_source1_concat_value944 = symbols;
                                {
                                    let source_ml_source1_concat_value945 = vec![key.clone()];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::JavaScriptSemanticValue>::new(),
                                            source_ml_source1_concat_value944,
                                        ),
                                        source_ml_source1_concat_value945,
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
                        source_ml_source1_data_property_value947,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value947.clone(),
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
