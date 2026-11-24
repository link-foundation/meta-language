// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1357Result;
use super::accept_unicode_case_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_semantic_text_unit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_map_unicode_case_utf16_columns_loop1357(
    mut cased: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut ignorable: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut points: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut following: std::rc::Rc<std::cell::RefCell<Vec<bool>>>,
    mut next: bool,
    mut at: f64,
) -> self::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1357Result {
    loop {
        return if (at >= (0f64)) {
            {
                let point = ({
                    let ml_reference_array3146 = points.clone();
                    let ml_reference_position3146 = at;
                    let ml_reference_projection3146 = self::ml_array::optional_at(
                        &*ml_reference_array3146.borrow(),
                        self::ml_array::number_index(ml_reference_position3146),
                    );
                    ml_reference_projection3146
                })
                .clone()
                .unwrap_or_else(|| 0f64);
                {
                    let source_ml_source1_temporary_map_unicode_case_utf16_columns_ignored1363 = {
                        let ml_method_owner3147 = following.clone();
                        let ml_method_argument3147_0 = next;
                        let ml_inserted = vec![ml_method_argument3147_0];
                        let mut ml_method_values3147 = ml_method_owner3147.borrow_mut();
                        let ml_length = ml_method_values3147
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4294967295usize)
                            .expect("Invalid array length");
                        ml_method_values3147.extend(ml_inserted);
                        ml_length as f64
                    };
                    if !(self::accept_unicode_case_property(point, ignorable.clone())) {
                        {
                            let next_2 = self::accept_unicode_case_property(point, cased.clone());
                            {
                                let at_2 = (at - 1f64);
                                {
                                    (next, at) = (next_2, at_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let at_3 = (at - 1f64);
                            {
                                at = at_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1357Result::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1357Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1356Result {
    MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1356Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_map_unicode_case_utf16_columns_loop1356(
    mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut points: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1356Result {
    loop {
        return if (at
            < ({
                let ml_reference_array3148 = text.clone();
                let ml_reference_projection3148 = ml_reference_array3148.borrow().len() as f64;
                ml_reference_projection3148
            })) {
            {
                let first = ({
                    let ml_reference_array3149 = text.clone();
                    let ml_reference_position3149 = at;
                    let ml_reference_projection3149 = self::ml_array::optional_at(
                        &*ml_reference_array3149.borrow(),
                        self::ml_array::number_index(ml_reference_position3149),
                    );
                    ml_reference_projection3149
                })
                .clone()
                .unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(first)) || (first < (0f64)))
                    || (first > (65535f64)))
                {
                    panic!("{}", "Unicode casing requires actual UTF16 units")
                } else {
                    {
                        let second = ({
                            let source_ml_source2_array_read_index42093 = (at + 1f64);
                            {
                                let ml_reference_array3150 = text.clone();
                                let ml_reference_position3150 =
                                    source_ml_source2_array_read_index42093;
                                let ml_reference_projection3150 = self::ml_array::optional_at(
                                    &*ml_reference_array3150.borrow(),
                                    self::ml_array::number_index(ml_reference_position3150),
                                );
                                ml_reference_projection3150
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| (-1f64));
                        if (((((first >= (55296f64)) && (first <= (56319f64)))
                            && self::ml_math::is_safe_integer(second))
                            && (second >= (56320f64)))
                            && (second <= (57343f64)))
                        {
                            {
                                let source_ml_source1_temporary_map_unicode_case_utf16_columns_ignored1364 = {
                                    let ml_method_owner3151 = points.clone();
                                    let ml_method_argument3151_0 =
                                        (((65536f64 + ((first - 55296f64) * 1024f64)) + second)
                                            - 56320f64);
                                    let ml_inserted = vec![ml_method_argument3151_0];
                                    let mut ml_method_values3151 = ml_method_owner3151.borrow_mut();
                                    let ml_length = ml_method_values3151
                                        .len()
                                        .checked_add(ml_inserted.len())
                                        .filter(|length| *length <= 4294967295usize)
                                        .expect("Invalid array length");
                                    ml_method_values3151.extend(ml_inserted);
                                    ml_length as f64
                                };
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        let at_3 = (at_2 + 1f64);
                                        {
                                            at = at_3;
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source1_temporary_map_unicode_case_utf16_columns_ignored1365 = {
                                    let ml_method_owner3152 = points.clone();
                                    let ml_method_argument3152_0 = first;
                                    let ml_inserted = vec![ml_method_argument3152_0];
                                    let mut ml_method_values3152 = ml_method_owner3152.borrow_mut();
                                    let ml_length = ml_method_values3152
                                        .len()
                                        .checked_add(ml_inserted.len())
                                        .filter(|length| *length <= 4294967295usize)
                                        .expect("Invalid array length");
                                    ml_method_values3152.extend(ml_inserted);
                                    ml_length as f64
                                };
                                {
                                    let at_4 = (at + 1f64);
                                    {
                                        at = at_4;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1356Result::MlSource1TemporaryMapUnicodeCaseUtf16ColumnsLoop1356Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1366Result {
    MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1366Done(bool),
    MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1366Return(
        Box<self::JavaScriptCallPrimitiveResult>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveJoin1367 {
    MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveJoin1367Next,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1366(
    operation: &self::ml_string::Text,
    mut receiver: self::JavaScriptSemanticValue,
    mut length: f64,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut formed: bool,
    mut at: f64,
) -> self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1366Result {
    loop {
        return if (at < length) {
            {
                let unit = self::read_java_script_semantic_text_unit(receiver.clone(), at);
                {
                    let high = ((unit >= (55296f64)) && (unit <= (56319f64)));
                    {
                        let low = ((unit >= (56320f64)) && (unit <= (57343f64)));
                        {
                            let next = if ((at + 1f64) < length) {
                                self::read_java_script_semantic_text_unit(
                                    receiver.clone(),
                                    (at + 1f64),
                                )
                            } else {
                                (-1f64)
                            };
                            if ((high && (next >= (56320f64))) && (next <= (57343f64))) {
                                {
                                    let source_ml_source2_s42094 = if ((*operation)
                                        == (self::ml_string::Text::from_units(vec![
                                            99, 111, 112, 121, 32, 119, 101, 108, 108, 32, 102,
                                            111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110,
                                            103,
                                        ]))) {
                                        {
                                            let source_ml_source1_temporary_execute_java_script_string_method_primitive_ignored1368 = {
                                                let ml_method_owner3153 = units.clone();
                                                let ml_method_argument3153_0 = unit;
                                                let ml_inserted = vec![ml_method_argument3153_0];
                                                let mut ml_method_values3153 =
                                                    ml_method_owner3153.borrow_mut();
                                                let ml_length = ml_method_values3153
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4294967295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values3153.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            {
                                                let source_ml_source1_temporary_execute_java_script_string_method_primitive_ignored1369 = {
                                                    let ml_method_owner3154 = units.clone();
                                                    let ml_method_argument3154_0 = next;
                                                    let ml_inserted =
                                                        vec![ml_method_argument3154_0];
                                                    let mut ml_method_values3154 =
                                                        ml_method_owner3154.borrow_mut();
                                                    let ml_length = ml_method_values3154
                                                        .len()
                                                        .checked_add(ml_inserted.len())
                                                        .filter(|length| *length <= 4294967295usize)
                                                        .expect("Invalid array length");
                                                    ml_method_values3154.extend(ml_inserted);
                                                    ml_length as f64
                                                };
                                                self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveJoin1367::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveJoin1367Next
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveJoin1367::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveJoin1367Next
                                    };
                                    match source_ml_source2_s42094.clone() {
                                        self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveJoin1367::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveJoin1367Next => {
                                            {
                                                let at_2 = (at + 1f64);
                                                {
                                                    let at_3 = (at_2 + 1f64);
                                                    {
                                                        at = at_3;
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if (high || low) {
                                    if ((*operation)
                                        == (self::ml_string::Text::from_units(vec![
                                            116, 101, 115, 116, 32, 119, 101, 108, 108, 32, 102,
                                            111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110,
                                            103,
                                        ])))
                                    {
                                        self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1366Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1366Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(false)))))
                                    } else {
                                        {
                                            let source_ml_source1_temporary_execute_java_script_string_method_primitive_ignored1370 = {
                                                let ml_method_owner3155 = units.clone();
                                                let ml_method_argument3155_0 = 65533f64;
                                                let ml_inserted = vec![ml_method_argument3155_0];
                                                let mut ml_method_values3155 =
                                                    ml_method_owner3155.borrow_mut();
                                                let ml_length = ml_method_values3155
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4294967295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values3155.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            {
                                                let at_4 = (at + 1f64);
                                                {
                                                    (formed, at) = (false, at_4);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if ((*operation)
                                        == (self::ml_string::Text::from_units(vec![
                                            99, 111, 112, 121, 32, 119, 101, 108, 108, 32, 102,
                                            111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110,
                                            103,
                                        ])))
                                    {
                                        {
                                            let source_ml_source1_temporary_execute_java_script_string_method_primitive_ignored1371 = {
                                                let ml_method_owner3156 = units.clone();
                                                let ml_method_argument3156_0 = unit;
                                                let ml_inserted = vec![ml_method_argument3156_0];
                                                let mut ml_method_values3156 =
                                                    ml_method_owner3156.borrow_mut();
                                                let ml_length = ml_method_values3156
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4294967295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values3156.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            {
                                                let at_5 = (at + 1f64);
                                                {
                                                    at = at_5;
                                                    continue;
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let at_6 = (at + 1f64);
                                            {
                                                at = at_6;
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
            self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1366Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1366Done(formed)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1372Result {
    MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1372Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1372(
    mut receiver: self::JavaScriptSemanticValue,
    mut high: f64,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1372Result {
    loop {
        return if (at < high) {
            {
                let source_ml_source1_temporary_execute_java_script_string_method_primitive_ignored1373 = {
                    let ml_method_owner3157 = units.clone();
                    let ml_method_argument3157_0 =
                        self::read_java_script_semantic_text_unit(receiver.clone(), at);
                    let ml_inserted = vec![ml_method_argument3157_0];
                    let mut ml_method_values3157 = ml_method_owner3157.borrow_mut();
                    let ml_length = ml_method_values3157
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3157.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1372Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1372Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1374(
    mut receiver: self::JavaScriptSemanticValue,
    mut second: self::JavaScriptSemanticValue,
    mut needle_length: f64,
    mut start: f64,
    mut matches: bool,
    mut at: f64,
) -> bool {
    loop {
        return if (matches && (at < needle_length)) {
            if ((self::read_java_script_semantic_text_unit(receiver.clone(), (start + at)))
                != (self::read_java_script_semantic_text_unit(second.clone(), at)))
            {
                {
                    let at_2 = (at + 1f64);
                    {
                        (matches, at) = (false, at_2);
                        continue;
                    }
                }
            } else {
                {
                    let at_3 = (at + 1f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            matches
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1375Result {
    MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1375Done,
    MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1375Return(
        Box<self::JavaScriptCallPrimitiveResult>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1376(
    mut receiver: self::JavaScriptSemanticValue,
    mut second: self::JavaScriptSemanticValue,
    mut needle_length: f64,
    mut source_ml_source1_shadow_start_1: f64,
    mut matches: bool,
    mut at: f64,
) -> bool {
    loop {
        return if (matches && (at < needle_length)) {
            if ((self::read_java_script_semantic_text_unit(
                receiver.clone(),
                (source_ml_source1_shadow_start_1 + at),
            )) != (self::read_java_script_semantic_text_unit(second.clone(), at)))
            {
                {
                    let at_2 = (at + 1f64);
                    {
                        (matches, at) = (false, at_2);
                        continue;
                    }
                }
            } else {
                {
                    let at_3 = (at + 1f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            matches
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1375(
    mut receiver: self::JavaScriptSemanticValue,
    mut second: self::JavaScriptSemanticValue,
    mut length: f64,
    mut needle_length: f64,
    mut source_ml_source1_shadow_start_1: f64,
) -> self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1375Result {
    loop {
        return if (source_ml_source1_shadow_start_1 <= (length - needle_length)) {
            {
                let matches = true;
                {
                    let at = 0f64;
                    {
                        let matches_2 = self::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1376(receiver.clone(), second.clone(), needle_length, source_ml_source1_shadow_start_1, matches, at);
                        if matches_2 {
                            self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1375Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1375Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(true)))))
                        } else {
                            {
                                let source_ml_source1_shadow_start_1_2 =
                                    (source_ml_source1_shadow_start_1 + 1f64);
                                {
                                    source_ml_source1_shadow_start_1 =
                                        source_ml_source1_shadow_start_1_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1375Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1375Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop1380(
    mut count: self::ml::Big,
    mut value: self::ml::Big,
    mut factor: self::ml::Big,
) -> self::ml::Big {
    loop {
        return if (count > (self::ml::Big::from_i128(0))) {
            {
                let value_3 = if ((self::ml::divide(
                    &count,
                    &self::ml::Big::from_i128(2),
                    "trunc",
                    Some("Division by zero"),
                    true,
                )) == (self::ml::Big::from_i128(1)))
                {
                    {
                        let value_2 = value.mul(&factor);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let count_2 = self::ml::divide(
                        &count,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    );
                    if (count_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let factor_2 = factor.mul(&factor);
                            {
                                (count, value, factor) =
                                    (count_2.clone(), value_3.clone(), factor_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (count, value) = (count_2.clone(), value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop1381(
    mut count: self::ml::Big,
    mut value: self::ml::Big,
) -> self::ml::Big {
    loop {
        return if (((count > (self::ml::Big::from_i128(0)))
            && (value != (self::ml::Big::from_i128(0))))
            && (value != (self::ml::Big::from_i128(-1))))
        {
            {
                let value_2 = if (value < (self::ml::Big::from_i128(0))) {
                    self::ml::divide(
                        &value.sub(&self::ml::Big::from_i128(1)),
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    )
                } else {
                    self::ml::divide(
                        &value,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    )
                };
                {
                    let count_2 = count.sub(&self::ml::Big::from_i128(1));
                    {
                        (count, value) = (count_2.clone(), value_2.clone());
                        continue;
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin1379 {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin1379Next(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop1382Result {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop1382Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop1382(
    mut operation: f64,
    mut negative_left: bool,
    mut negative_right: bool,
    mut first: self::ml::Big,
    mut second: self::ml::Big,
    mut weight: self::ml::Big,
    mut result: self::ml::Big,
) -> self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop1382Result {
    loop {
        return if ((first > (self::ml::Big::from_i128(0)))
            || (second > (self::ml::Big::from_i128(0))))
        {
            {
                let a = if negative_left {
                    (self::ml::Big::from_i128(1)).sub(&self::ml::divide(
                        &first,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        true,
                    ))
                } else {
                    self::ml::divide(
                        &first,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        true,
                    )
                };
                {
                    let b = if negative_right {
                        (self::ml::Big::from_i128(1)).sub(&self::ml::divide(
                            &second,
                            &self::ml::Big::from_i128(2),
                            "trunc",
                            Some("Division by zero"),
                            true,
                        ))
                    } else {
                        self::ml::divide(
                            &second,
                            &self::ml::Big::from_i128(2),
                            "trunc",
                            Some("Division by zero"),
                            true,
                        )
                    };
                    {
                        let selected = if (operation == (0f64)) {
                            ((a == (self::ml::Big::from_i128(1)))
                                && (b == (self::ml::Big::from_i128(1))))
                        } else {
                            if (operation == (1f64)) {
                                ((a == (self::ml::Big::from_i128(1)))
                                    || (b == (self::ml::Big::from_i128(1))))
                            } else {
                                (a != b)
                            }
                        };
                        {
                            let result_3 = if selected {
                                {
                                    let result_2 = result.add(&weight);
                                    result_2.clone()
                                }
                            } else {
                                result.clone()
                            };
                            {
                                let first_2 = self::ml::divide(
                                    &first,
                                    &self::ml::Big::from_i128(2),
                                    "trunc",
                                    Some("Division by zero"),
                                    false,
                                );
                                {
                                    let second_2 = self::ml::divide(
                                        &second,
                                        &self::ml::Big::from_i128(2),
                                        "trunc",
                                        Some("Division by zero"),
                                        false,
                                    );
                                    {
                                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                                        {
                                            (first, second, weight, result) = (
                                                first_2.clone(),
                                                second_2.clone(),
                                                weight_2.clone(),
                                                result_3.clone(),
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
        } else {
            self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop1382Result::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop1382Done(weight.clone(), result.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385 {
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385Next(self::ml::Big),
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385Return(
        Box<self::JavaScriptCallPrimitiveResult>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384 {
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384Next,
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384Return(
        Box<self::JavaScriptCallPrimitiveResult>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_integer_wrapping_loop1386(
    mut bit_width: f64,
    mut modulus: self::ml::Big,
    mut bit: f64,
) -> self::ml::Big {
    loop {
        return if (bit < bit_width) {
            {
                let modulus_2 = modulus.mul(&self::ml::Big::from_i128(2));
                {
                    let bit_2 = (bit + 1f64);
                    {
                        (modulus, bit) = (modulus_2.clone(), bit_2);
                        continue;
                    }
                }
            }
        } else {
            modulus.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1389Result {
    MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1389Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_integer_radix_text_loop1389(
    mut reversed: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1389Result {
    loop {
        return if (index >= (0f64)) {
            {
                let source_ml_source1_temporary_execute_java_script_call_integer_radix_text_ignored1390 = {
                    let ml_method_owner3158 = units.clone();
                    let ml_method_argument3158_0 = ({
                        let ml_reference_array3159 = reversed.clone();
                        let ml_reference_position3159 = index;
                        let ml_reference_projection3159 = self::ml_array::optional_at(
                            &*ml_reference_array3159.borrow(),
                            self::ml_array::number_index(ml_reference_position3159),
                        );
                        ml_reference_projection3159
                    })
                    .clone()
                    .unwrap_or_else(|| 48f64);
                    let ml_inserted = vec![ml_method_argument3158_0];
                    let mut ml_method_values3158 = ml_method_owner3158.borrow_mut();
                    let ml_length = ml_method_values3158
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3158.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let index_2 = (index - 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1389Result::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1389Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextJoin1388 {
    MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextJoin1388Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1387Result {
    MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1387Done,
}
