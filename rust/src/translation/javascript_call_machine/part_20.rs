// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::ml_utf16_text;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop38(mut remaining: self::ml::Big, mut shifted: self::ml::Big, mut weight: self::ml::Big, mut quotient: self::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
            {
                let shifted_2 = self::ml::divide(&shifted, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = self::ml::divide(&weight, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (shifted_2 <= remaining) {
                        {
                            let remaining_2 = remaining.sub(&shifted_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (remaining, shifted, weight, quotient) = (remaining_2.clone(), shifted_2.clone(), weight_2.clone(), quotient_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            (shifted, weight) = (shifted_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Done(remaining.clone(), quotient.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop37(mut remaining: self::ml::Big, mut shifted: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Result {
    loop {
        return if (shifted <= remaining) {
            {
                let shifted_2 = shifted.mul(&self::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                    {
                        (shifted, weight) = (shifted_2.clone(), weight_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Done(shifted.clone(), weight.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_big_integer_loop39(mut result: self::ml::Big, mut weight: self::ml::Big, mut remaining: self::ml::Big) -> self::ml::Big {
    loop {
        return if (remaining > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(&remaining, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let result_2 = result.mul(&weight);
                        result_2.clone()
                    }
                } else {
                    result.clone()
                };
                {
                    let remaining_2 = self::ml::divide(&remaining, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remaining_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let weight_2 = weight.mul(&weight);
                            {
                                (result, weight, remaining) = (result_3.clone(), weight_2.clone(), remaining_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (result, remaining) = (result_3.clone(), remaining_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop54(mut count: self::ml::Big, mut value: self::ml::Big, mut factor: self::ml::Big) -> self::ml::Big {
    loop {
        return if (count > (self::ml::Big::from_i128(0))) {
            {
                let value_3 = if ((self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let value_2 = value.mul(&factor);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let count_2 = self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (count_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let factor_2 = factor.mul(&factor);
                            {
                                (count, value, factor) = (count_2.clone(), value_3.clone(), factor_2.clone());
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
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop55(mut count: self::ml::Big, mut value: self::ml::Big) -> self::ml::Big {
    loop {
        return if (((count > (self::ml::Big::from_i128(0))) && (value != (self::ml::Big::from_i128(0)))) && (value != (self::ml::Big::from_i128(-1)))) {
            {
                let value_2 = if (value < (self::ml::Big::from_i128(0))) {
                    self::ml::divide(&value.sub(&self::ml::Big::from_i128(1)), &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                } else {
                    self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
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
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin53 {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin53Next(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop56Result {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop56Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop56(mut operation: f64, mut negative_left: bool, mut negative_right: bool, mut first: self::ml::Big, mut second: self::ml::Big, mut weight: self::ml::Big, mut result: self::ml::Big) -> self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop56Result {
    loop {
        return if ((first > (self::ml::Big::from_i128(0))) || (second > (self::ml::Big::from_i128(0)))) {
            {
                let a = if negative_left {
                    (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                } else {
                    self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                };
                {
                    let b = if negative_right {
                        (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                    } else {
                        self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                    };
                    {
                        let selected = if (operation == (0f64)) {
                            ((a == (self::ml::Big::from_i128(1))) && (b == (self::ml::Big::from_i128(1))))
                        } else {
                            if (operation == (1f64)) {
                                ((a == (self::ml::Big::from_i128(1))) || (b == (self::ml::Big::from_i128(1))))
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
                                let first_2 = self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                {
                                    let second_2 = self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                    {
                                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                                        {
                                            (first, second, weight, result) = (first_2.clone(), second_2.clone(), weight_2.clone(), result_3.clone());
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
            self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop56Result::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop56Done(weight.clone(), result.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop58(mut count: f64, mut factor: f64, mut position: f64) -> f64 {
    loop {
        return if (position < count) {
            {
                let factor_2 = (factor * 2f64);
                {
                    let position_2 = (position + 1f64);
                    {
                        (factor, position) = (factor_2, position_2);
                        continue;
                    }
                }
            }
        } else {
            factor
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop59(mut operation: f64, mut first: f64, mut second: f64, mut result: f64, mut weight: f64, mut position: f64) -> f64 {
    loop {
        return if (position < (32f64)) {
            {
                let first_bit = (first % 2f64);
                {
                    let second_bit = (second % 2f64);
                    {
                        let result_3 = if (((((operation == (0f64)) && (first_bit == (1f64))) && (second_bit == (1f64))) || ((operation == (1f64)) && ((first_bit == (1f64)) || (second_bit == (1f64))))) || ((operation == (2f64)) && (first_bit != second_bit))) {
                            {
                                let result_2 = (result + weight);
                                result_2
                            }
                        } else {
                            result
                        };
                        {
                            let first_2 = ((first - first_bit) / 2f64);
                            {
                                let second_2 = ((second - second_bit) / 2f64);
                                {
                                    let weight_2 = (weight * 2f64);
                                    {
                                        let position_2 = (position + 1f64);
                                        {
                                            (first, second, result, weight, position) = (first_2, second_2, result_3, weight_2, position_2);
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
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop60Result {
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop60Done(f64),
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop60Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_integer_width_loop60(mut rest: self::ml::Big, mut width: f64) -> self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop60Result {
    loop {
        return if (rest > (self::ml::Big::from_i128(0))) {
            if (width >= (4096f64)) {
                self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop60Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop60Return((-1f64))
            } else {
                {
                    let rest_2 = self::ml::divide(&rest, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    {
                        let width_2 = (width + 1f64);
                        {
                            (rest, width) = (rest_2.clone(), width_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop60Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop60Done(width)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin62 {
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin62Next(self::ml::Big),
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin62Return(Box<self::JavaScriptCallPrimitiveResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin61 {
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin61Next,
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin61Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_integer_text_loop63(units: &[f64], mut text: self::ml_string::Text, mut index: f64) -> self::ml_string::Text {
    loop {
        return if (index < ((units.len() as f64))) {
            {
                let text_2 = text.concat(&{
                    let source_ml_source2_character_argument2562 = (self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| 0f64);
                    self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source2_character_argument2562]).into_iter().map(|ml_unit| ml_unit as u16).collect())
                });
                {
                    let index_2 = (index + 1f64);
                    {
                        (text, index) = (text_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            text.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop64Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop64Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop64Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop64(units: &[f64], mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop64Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop64Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop64Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop64Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop64Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptSemanticSameValueLoop65Result {
    MlSource1TemporaryTestJavaScriptSemanticSameValueLoop65Done,
    MlSource1TemporaryTestJavaScriptSemanticSameValueLoop65Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_semantic_same_value_loop65(left_units: &[f64], right_units: &[f64], mut at: f64) -> self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop65Result {
    loop {
        return if (at < ((left_units.len() as f64))) {
            if (((self::ml_array::optional_at(left_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(right_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)))) {
                self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop65Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop65Return(false)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop65Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop65Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66 {
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Next,
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin66Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>)),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Result {
    MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Done,
    MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_ordinary_object_loop68(mut objects: Vec<self::JavaScriptOrdinaryObject>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2564, _, _, _, _) => {
                                source_ml_source2_data_property_value2564.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2566, _, _, _, _) => {
                                    source_ml_source2_data_property_value2566.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    high = middle;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Result::MlSource1TemporaryFindJavaScriptOrdinaryObjectLoop68Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Result {
    MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Done,
    MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_ordinary_symbol_loop69(mut symbols: Vec<self::ml::Big>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = self::ml_array::optional_at(&symbols, self::ml_array::number_index(middle));
                    if {
                        let source_ml_source2_comparison_operand2568 = None::<()>;
                        match (symbol.clone(), source_ml_source2_comparison_operand2568.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                    } {
                        self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Return(true)
                        } else {
                            if ((symbol.clone().expect("a proven present value")) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    high = middle;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Result::MlSource1TemporaryTestJavaScriptOrdinarySymbolLoop69Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop70Result {
    MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop70Done,
    MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop70Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_ordinary_property_loop70(mut properties: Vec<self::JavaScriptOrdinaryProperty>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop70Result {
    loop {
        return if (at < ((properties.len() as f64))) {
            {
                let property = self::ml_array::optional_at(&properties, self::ml_array::number_index(at));
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value({
                    let source_ml_source2_nullish_field_receiver2569 = (property.clone(),);
                    if match (source_ml_source2_nullish_field_receiver2569.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present2570 = ((source_ml_source2_nullish_field_receiver2569.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present2570.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2571, _) => {
                                    let source_ml_source2_data_property_value2571 = *source_ml_source2_data_property_value2571;
                                    source_ml_source2_data_property_value2571.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                    }
                }, key.clone())) {
                    self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop70Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop70Return(at)
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop70Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop70Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop78(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut prototype: self::JavaScriptSemanticValue, mut depth: f64) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _) => {
                match prototype.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Done
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Done
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Done
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Done
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Done
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Done
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Done
                    }
                    self::JavaScriptSemanticValue::Object(identity, _) => {
                        {
                            let depth_2 = (depth + 1f64);
                            if ((identity == (match object.clone() {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2579, _, _, _, _) => {
                                    source_ml_source2_data_property_value2579.clone()
                                }
                            })) || (depth_2 > ((shared_constructor_field1.len() as f64)))) {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104, 97, 105, 110, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 121, 99, 108, 105, 99]))
                            } else {
                                {
                                    let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                    {
                                        let parent = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(index));
                                        if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                        } else {
                                            {
                                                let prototype_2 = match parent.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2584, _, _, _) => {
                                                        let source_ml_source2_data_property_value2584 = *source_ml_source2_data_property_value2584;
                                                        source_ml_source2_data_property_value2584.clone()
                                                    }
                                                };
                                                {
                                                    (prototype, depth) = (prototype_2.clone(), depth_2);
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
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin80 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin80Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin80Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin79 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin79Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin79Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop77(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Result {
    loop {
        return match object.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, shared_constructor_field1) => {
                if (at < ((shared_constructor_field1.len() as f64))) {
                    {
                        let property = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(at));
                        if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_key_error(heap.clone(), match property.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2588, _) => {
                                        let source_ml_source2_data_property_value2588 = *source_ml_source2_data_property_value2588;
                                        source_ml_source2_data_property_value2588.clone()
                                    }
                                });
                                if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Return(key_error.clone())
                                } else {
                                    if ((self::find_java_script_ordinary_property(shared_constructor_field1, match property.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2592, _) => {
                                            let source_ml_source2_data_property_value2592 = *source_ml_source2_data_property_value2592;
                                            source_ml_source2_data_property_value2592.clone()
                                        }
                                    })) != at) {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 114, 101, 111, 114, 100, 101, 114, 105, 110, 103, 32, 116, 104, 101, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                                    } else {
                                        {
                                            let descriptor = match property.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2593) => {
                                                    source_ml_source2_data_property_value2593.clone()
                                                }
                                            };
                                            if !(self::test_complete_java_script_property_descriptor(descriptor.clone())) {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 105, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115]))
                                            } else {
                                                {
                                                    let source_ml_source2_s2594 = if (descriptor.clone()).9 {
                                                        {
                                                            let error = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).8);
                                                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin79::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin79Return(error.clone())
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin79::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin79Next
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin79::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin79Next
                                                    };
                                                    match source_ml_source2_s2594.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin79::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin79Next => {
                                                            {
                                                                let source_ml_source2_s2595 = if (descriptor.clone()).5 {
                                                                    {
                                                                        let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4);
                                                                        if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin80::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin80Return(error_2.clone())
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin80::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin80Next
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin80::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin80Next
                                                                };
                                                                match source_ml_source2_s2595.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin80::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin80Next => {
                                                                        if (descriptor.clone()).7 {
                                                                            {
                                                                                let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6);
                                                                                if (error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Return(error_3.clone())
                                                                                } else {
                                                                                    {
                                                                                        let at_2 = (at + 1f64);
                                                                                        {
                                                                                            at = at_2;
                                                                                            continue;
                                                                                        }
                                                                                    }
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
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin80::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin80Return(source_ml_source1_temporary_result80) => {
                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Return(source_ml_source1_temporary_result80.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin79::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin79Return(source_ml_source1_temporary_result80_2) => {
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Return(source_ml_source1_temporary_result80_2.clone())
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
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Done
                }
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop76(mut heap: self::JavaScriptOrdinaryHeap, mut property_count: f64, mut source_ml_source1_temporary_values73: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index73: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Result {
    loop {
        return if (source_ml_source1_temporary_index73 < (self::ml::Big::from_u128(source_ml_source1_temporary_values73.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_source1_temporary_values73, source_ml_source1_temporary_index73.to_index());
                match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, shared_constructor_field1, _, _, shared_constructor_field2) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        if ({
                            let source_ml_source2_s2597 = shared_constructor_field1.clone();
                            match source_ml_source2_s2597.clone() {
                                self::JavaScriptSemanticValue::Null => {
                                    false
                                }
                                _ => {
                                    true
                                }
                            }
                        } && {
                            let source_ml_source2_s2607 = shared_constructor_field1.clone();
                            match source_ml_source2_s2607.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    false
                                }
                                _ => {
                                    true
                                }
                            }
                        }) {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]))
                        } else {
                            {
                                let prototype_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field1.clone());
                                if (prototype_error != (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Return(prototype_error.clone())
                                } else {
                                    {
                                        let property_count_2 = (property_count + (shared_constructor_field2.len() as f64));
                                        if ((property_count_2 > (16384f64)) || (((shared_constructor_field2.len() as f64)) > (4096f64))) {
                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                        } else {
                                            {
                                                let at = 0f64;
                                                {
                                                    let source_ml_source2_s2620 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop77(heap.clone(), object.clone(), at);
                                                    match source_ml_source2_s2620.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Done => {
                                                            {
                                                                let prototype = shared_constructor_field1.clone();
                                                                {
                                                                    let depth = 0f64;
                                                                    {
                                                                        let source_ml_source2_s2622 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop78(heap.clone(), object.clone(), prototype.clone(), depth);
                                                                        match source_ml_source2_s2622.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Done => {
                                                                                {
                                                                                    let source_ml_source1_temporary_index73_2 = source_ml_source1_temporary_index73.add(&self::ml::Big::from_i128(1));
                                                                                    {
                                                                                        (property_count, source_ml_source1_temporary_index73) = (property_count_2, source_ml_source1_temporary_index73_2.clone());
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop78Return(source_ml_source1_temporary_result78) => {
                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Return(source_ml_source1_temporary_result78.clone())
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop77Return(source_ml_source1_temporary_result78_2) => {
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Return(source_ml_source1_temporary_result78_2.clone())
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
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop76Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Return(self::ml_string::Text),
}
