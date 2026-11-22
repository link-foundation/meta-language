// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::accept_java_script_whitespace_unit;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::ml_utf16_text;
use super::read_java_script_numeric_digit_unit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop172(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        if (end > start)
            && self::accept_java_script_whitespace_unit(
                ({
                    let source_ml_source2_array_read_index21773 = end - 1f64;
                    {
                        let ml_reference_array350 = units.clone();
                        let ml_reference_position350 = source_ml_source2_array_read_index21773;
                        self::ml_array::optional_at(
                            &*ml_reference_array350.borrow(),
                            self::ml_array::number_index(ml_reference_position350),
                        )
                    }
                })
                .clone()
                .unwrap_or_else(|| -1f64),
            )
        {
            {
                let end_2 = end - 1f64;
                {
                    end = end_2;
                    continue;
                }
            }
        }
        return { end };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop175Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop175Done(self::ml::Big),
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop175Return(
        std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop175(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut end: f64,
    mut radix: f64,
    mut base: self::ml::Big,
    mut digits: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut value: self::ml::Big,
    mut index: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop175Result {
    loop {
        return if index < end {
            {
                let digit = self::read_java_script_numeric_digit_unit(
                    ({
                        let ml_reference_array351 = units.clone();
                        let ml_reference_position351 = index;
                        self::ml_array::optional_at(
                            &*ml_reference_array351.borrow(),
                            self::ml_array::number_index(ml_reference_position351),
                        )
                    })
                    .clone()
                    .unwrap_or_else(|| -1f64),
                );
                if (digit < 0f64) || (digit >= radix) {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop175Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop175Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new())))
                } else {
                    {
                        let value_2 = (value.mul(&base)).add(
                            &({
                                let ml_reference_array352 = digits.clone();
                                let ml_reference_position352 = digit;
                                self::ml_array::optional_at(
                                    &*ml_reference_array352.borrow(),
                                    self::ml_array::number_index(ml_reference_position352),
                                )
                            })
                            .clone()
                            .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                        );
                        {
                            let index_2 = index + 1f64;
                            {
                                (value, index) = (value_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop175Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop175Done(value.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174Next(f64, f64, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin173 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin173Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop170Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop170Done,
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop170Return(
        std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop170(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop170Result {
    loop {
        return if index
            < ({
                let ml_reference_array353 = units.clone();
                ml_reference_array353.borrow().len() as f64
            }) {
            {
                let unit = ({
                    let ml_reference_array354 = units.clone();
                    let ml_reference_position354 = index;
                    self::ml_array::optional_at(
                        &*ml_reference_array354.borrow(),
                        self::ml_array::number_index(ml_reference_position354),
                    )
                })
                .clone()
                .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_safe_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64)
                {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop170Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop170Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new())))
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop170Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop170Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_number_big_integer_loop176(
    mut value: f64,
    mut integer: self::ml::Big,
    mut weight: self::ml::Big,
) -> self::ml::Big {
    loop {
        if value >= 1f64 {
            {
                let integer_3 = if (value % 2f64) == 1f64 {
                    { integer.add(&weight) }
                } else {
                    integer.clone()
                };
                {
                    let value_2 = (value / 2f64).floor();
                    {
                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                        {
                            (value, integer, weight) = (value_2, integer_3, weight_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { integer.clone() };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop191Result {
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop191Done(f64),
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop191Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_integer_width_loop191(
    mut rest: self::ml::Big,
    mut width: f64,
) -> self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop191Result {
    loop {
        return if rest > (self::ml::Big::from_i128(0)) {
            if width >= 4096f64 {
                self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop191Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop191Return(-1f64)
            } else {
                {
                    let rest_2 = self::ml::divide(
                        &rest,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    );
                    {
                        let width_2 = width + 1f64;
                        {
                            (rest, width) = (rest_2, width_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop191Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop191Done(width)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_integer_text_loop192(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut text: self::ml_string::Text,
    mut index: f64,
) -> self::ml_string::Text {
    loop {
        if index
            < ({
                let ml_reference_array355 = units.clone();
                ml_reference_array355.borrow().len() as f64
            })
        {
            {
                let text_2 = text.concat(&{
                    let source_ml_source2_character_argument21803 = ({
                        let ml_reference_array356 = units.clone();
                        let ml_reference_position356 = index;
                        self::ml_array::optional_at(
                            &*ml_reference_array356.borrow(),
                            self::ml_array::number_index(ml_reference_position356),
                        )
                    })
                    .clone()
                    .unwrap_or_else(|| 0f64);
                    self::ml_string::Text::from_units(
                        self::ml_utf16_text::character_units_utf16_text(&vec![
                            source_ml_source2_character_argument21803,
                        ])
                        .into_iter()
                        .map(|ml_unit| ml_unit as u16)
                        .collect(),
                    )
                });
                {
                    let index_2 = index + 1f64;
                    {
                        (text, index) = (text_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { text.clone() };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallTextConversionLoop194Result {
    MlSource1TemporaryExecuteJavaScriptCallTextConversionLoop194Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_text_conversion_loop194(
    text: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryExecuteJavaScriptCallTextConversionLoop194Result {
    loop {
        if index < (text.length()) {
            {
                let source_ml_source1_temporary_execute_java_script_call_text_conversion_ignored195 = {
                    let ml_method_owner357 = units.clone();
                    let ml_method_argument357_0 = {
                        let source_ml_source2_string_unit_operand21804 = text.clone();
                        {
                            let source_ml_source2_string_unit_operand21805 = index;
                            (std::rc::Rc::new(
                                move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                    {
                                        let index_2 = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if (index_2 < 0f64) || (index_2 >= (text_2.length())) {
                                            f64::NAN
                                        } else {
                                            ({
                                    let source_ml_source2_string_unit_operand21811 = {
                                        let source_ml_source2_string_range_operand21806 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand21807 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand21808 = index_2 + 1f64;
                                                {
                                                    let source_ml_source2_string_range_position21809 = source_ml_source2_string_range_operand21807;
                                                    {
                                                        let source_ml_source2_string_range_position21810 = source_ml_source2_string_range_operand21808;
                                                        source_ml_source2_string_range_operand21806.slice(source_ml_source2_string_range_position21809, source_ml_source2_string_range_position21810)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand21812 = 0f64;
                                        source_ml_source2_string_unit_operand21811.code_point_at(source_ml_source2_string_unit_operand21812)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand21804,
                                source_ml_source2_string_unit_operand21805,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument357_0];
                    let mut ml_method_values357 = ml_method_owner357.borrow_mut();
                    let ml_length = ml_method_values357
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values357.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryExecuteJavaScriptCallTextConversionLoop194Result::MlSource1TemporaryExecuteJavaScriptCallTextConversionLoop194Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193 {
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Next(self::ml_string::Text),
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Return(
        Box<self::JavaScriptCallPrimitiveResult>,
    ),
}
