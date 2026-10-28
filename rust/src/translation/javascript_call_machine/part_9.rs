// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryAcceptStringNumberInfinityLoop54Result;
use super::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31;
use super::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop34Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop55Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop60Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop62Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop65Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop66Result;
use super::MlSource1TemporaryConvertStringNumberRationalJoin46;
use super::MlSource1TemporaryConvertStringNumberRationalJoin47;
use super::MlSource1TemporaryConvertStringNumberRationalLoop48Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop49Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop52Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop53Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop74Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop75Result;
use super::MlSource1TemporaryReadStringNumberBinaryPowerLoop44Result;
use super::apply_java_script_primitive_conversion_method;
use super::continue_java_script_primitive_conversion;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::source_ml_source1_temporary_accept_string_number_infinity_loop54;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop32;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop33;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop34;
use super::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop36;
use super::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop37;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop55;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop56;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop57;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop60;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop62;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop63;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop65;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop66;
use super::source_ml_source1_temporary_convert_string_number_rational_loop48;
use super::source_ml_source1_temporary_convert_string_number_rational_loop49;
use super::source_ml_source1_temporary_convert_string_number_rational_loop50;
use super::source_ml_source1_temporary_convert_string_number_rational_loop51;
use super::source_ml_source1_temporary_convert_string_number_rational_loop52;
use super::source_ml_source1_temporary_convert_string_number_rational_loop53;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop74;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop75;
use super::source_ml_source1_temporary_power_java_script_big_integer_loop76;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop44;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop45;
use super::source_ml_source1_temporary_read_string_number_decimal_power_loop42;
use super::source_ml_source1_temporary_read_string_number_exact_mantissa_loop43;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_primitive_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState, completion: self::JavaScriptPrimitiveConversionCompletion) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, shared_constructor_field1) => {
            {
                let error = self::read_java_script_primitive_conversion_state_error(heap.clone(), state.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
                } else {
                    {
                        let getter = (((shared_constructor_field1 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]))) || (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114])))) || (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]))));
                        {
                            let call = (((shared_constructor_field1 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]))) || (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108])))) || (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108]))));
                            if (!getter && !call) {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 109, 101, 116, 104, 111, 100]))
                            } else {
                                {
                                    let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                        self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value824) => {
                                            let source_ml_source2_data_property_value824 = *source_ml_source2_data_property_value824;
                                            source_ml_source2_data_property_value824.clone()
                                        }
                                        self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value825) => {
                                            let source_ml_source2_data_property_value825 = *source_ml_source2_data_property_value825;
                                            source_ml_source2_data_property_value825.clone()
                                        }
                                    });
                                    if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(value_error.clone())
                                    } else {
                                        match completion.clone() {
                                            self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                                let value = *value;
                                                if getter {
                                                    self::apply_java_script_primitive_conversion_method(state.clone(), value.clone())
                                                } else {
                                                    if match value.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    } {
                                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(value.clone()))
                                                    } else {
                                                        if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]))) {
                                                            self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                                        } else {
                                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]))) {
                                                                self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 117, 115, 116, 32, 114, 101, 116, 117, 114, 110, 32, 97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101])
                                                            } else {
                                                                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115])
                                                            })
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                                let value_2 = *value_2;
                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(Box::new(value_2.clone()))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_big_integer_bitwise_operation(operation: f64, left: self::ml::Big, right: self::ml::Big) -> self::ml::Big {
    if (operation == (6f64)) {
        (self::ml::Big::from_i128(-1)).sub(&left)
    } else {
        if ((operation == (3f64)) || (operation == (4f64))) {
            if (left == (self::ml::Big::from_i128(0))) {
                self::ml::Big::from_i128(0)
            } else {
                {
                    let count = right.clone();
                    {
                        let forward = (operation == (3f64));
                        {
                            let source_ml_source2_s838 = if (count < (self::ml::Big::from_i128(0))) {
                                {
                                    let count_2 = count.neg();
                                    {
                                        let forward_2 = !forward;
                                        self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31Next(count_2.clone(), forward_2)
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31Next(count.clone(), forward)
                            };
                            match source_ml_source2_s838.clone() {
                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31Next(count_3, forward_3) => {
                                    {
                                        let value = left.clone();
                                        {
                                            let value_4 = if forward_3 {
                                                {
                                                    let factor = self::ml::Big::from_i128(2);
                                                    {
                                                        let value_2 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop32(count_3.clone(), value.clone(), factor.clone());
                                                        value_2.clone()
                                                    }
                                                }
                                            } else {
                                                {
                                                    let value_3 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop33(count_3.clone(), value.clone());
                                                    value_3.clone()
                                                }
                                            };
                                            value_4.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if (((operation != (0f64)) && (operation != (1f64))) && (operation != (2f64))) {
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 110, 32, 117, 110, 107, 110, 111, 119, 110, 32, 66, 105, 103, 73, 110, 116, 32, 98, 105, 116, 119, 105, 115, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])).0, message_present: true }).raise()
            } else {
                {
                    let negative_left = (left < (self::ml::Big::from_i128(0)));
                    {
                        let negative_right = (right < (self::ml::Big::from_i128(0)));
                        {
                            let first = if negative_left {
                                (self::ml::Big::from_i128(-1)).sub(&left)
                            } else {
                                left.clone()
                            };
                            {
                                let second = if negative_right {
                                    (self::ml::Big::from_i128(-1)).sub(&right)
                                } else {
                                    right.clone()
                                };
                                {
                                    let weight = self::ml::Big::from_i128(1);
                                    {
                                        let result = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s839 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop34(operation, negative_left, negative_right, first.clone(), second.clone(), weight.clone(), result.clone());
                                            match source_ml_source2_s839.clone() {
                                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop34Result::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop34Done(weight_2, result_2) => {
                                                    {
                                                        let negative = if (operation == (0f64)) {
                                                            (negative_left && negative_right)
                                                        } else {
                                                            if (operation == (1f64)) {
                                                                (negative_left || negative_right)
                                                            } else {
                                                                (negative_left != negative_right)
                                                            }
                                                        };
                                                        if negative {
                                                            result_2.sub(&weight_2)
                                                        } else {
                                                            result_2.clone()
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function1(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integral = if (value < (0f64)) {
                (-(((-value)).floor()))
            } else {
                value.floor()
            };
            {
                let remainder = (integral % 4294967296f64);
                if (remainder < (0f64)) {
                    (remainder + 4294967296f64)
                } else {
                    if (remainder == (0f64)) {
                        0f64
                    } else {
                        remainder
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_java_script_signed_integer(value: f64) -> f64 {
    {
        let unsigned = self::dependency_function1(value);
        if (unsigned >= (2147483648f64)) {
            (unsigned - 4294967296f64)
        } else {
            unsigned
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_number_bitwise_operation(operation: f64, left: f64, right: f64) -> f64 {
    if (operation == (6f64)) {
        ((-1f64) - self::normalize_java_script_signed_integer(left))
    } else {
        if (((operation == (3f64)) || (operation == (4f64))) || (operation == (5f64))) {
            {
                let count = (self::dependency_function1(right) % 32f64);
                {
                    let factor = 1f64;
                    {
                        let position = 0f64;
                        {
                            let factor_2 = self::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop36(count, factor, position);
                            if (operation == (3f64)) {
                                self::normalize_java_script_signed_integer((self::dependency_function1(left) * factor_2))
                            } else {
                                if (operation == (4f64)) {
                                    ((self::normalize_java_script_signed_integer(left) / factor_2)).floor()
                                } else {
                                    ((self::dependency_function1(left) / factor_2)).floor()
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if !((((operation == (0f64)) || (operation == (1f64))) || (operation == (2f64)))) {
                f64::NAN
            } else {
                {
                    let first = self::dependency_function1(left);
                    {
                        let second = self::dependency_function1(right);
                        {
                            let result = 0f64;
                            {
                                let weight = 1f64;
                                {
                                    let position_2 = 0f64;
                                    {
                                        let result_2 = self::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop37(operation, first, second, result, weight, position_2);
                                        self::normalize_java_script_signed_integer(result_2)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_whitespace_unit(unit: f64) -> bool {
    ((((((((((((unit >= (9f64)) && (unit <= (13f64))) || (unit == (32f64))) || (unit == (160f64))) || (unit == (5760f64))) || ((unit >= (8192f64)) && (unit <= (8202f64)))) || (unit == (8232f64))) || (unit == (8233f64))) || (unit == (8239f64))) || (unit == (8287f64))) || (unit == (12288f64))) || (unit == (65279f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_numeric_digit_unit(unit: f64) -> f64 {
    if ((unit >= (48f64)) && (unit <= (57f64))) {
        (unit - 48f64)
    } else {
        if ((unit >= (65f64)) && (unit <= (70f64))) {
            (unit - 55f64)
        } else {
            if ((unit >= (97f64)) && (unit <= (102f64))) {
                (unit - 87f64)
            } else {
                (-1f64)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_string_number_digit(unit: f64) -> f64 {
    self::read_java_script_numeric_digit_unit(unit)
}

pub fn read_string_number_decimal_power(exponent: f64) -> self::ml::Big {
    {
        let value = self::ml::Big::from_i128(1);
        {
            let base = self::ml::Big::from_i128(10);
            {
                let value_2 = self::source_ml_source1_temporary_read_string_number_decimal_power_loop42(exponent, value.clone(), base.clone());
                value_2.clone()
            }
        }
    }
}

pub fn read_string_number_exact_mantissa(value: self::ml::Big) -> f64 {
    {
        let result = 0f64;
        {
            let scale = 1f64;
            {
                let result_2 = self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop43(value.clone(), result, scale);
                result_2
            }
        }
    }
}

pub fn read_string_number_binary_power(exponent: f64) -> f64 {
    {
        let result = 1f64;
        {
            let source_ml_source2_s842 = self::source_ml_source1_temporary_read_string_number_binary_power_loop44(exponent, result);
            match source_ml_source2_s842.clone() {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop44Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop44Done(exponent_2, result_2) => {
                    {
                        let result_3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop45(exponent_2, result_2);
                        result_3
                    }
                }
            }
        }
    }
}

pub fn convert_string_number_rational(numerator: self::ml::Big, denominator: self::ml::Big) -> f64 {
    if ((numerator < (self::ml::Big::from_i128(0))) || (denominator <= (self::ml::Big::from_i128(0)))) {
        f64::NAN
    } else {
        if (numerator == (self::ml::Big::from_i128(0))) {
            0f64
        } else {
            {
                let exponent = 0f64;
                {
                    let source_ml_source2_s845 = if (numerator >= denominator) {
                        {
                            let source_ml_source2_s846 = self::source_ml_source1_temporary_convert_string_number_rational_loop52(numerator.clone(), exponent, denominator.clone());
                            match source_ml_source2_s846.clone() {
                                self::MlSource1TemporaryConvertStringNumberRationalLoop52Result::MlSource1TemporaryConvertStringNumberRationalLoop52Done(exponent_2) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin46::MlSource1TemporaryConvertStringNumberRationalJoin46Next(exponent_2)
                                }
                                self::MlSource1TemporaryConvertStringNumberRationalLoop52Result::MlSource1TemporaryConvertStringNumberRationalLoop52Return(source_ml_source1_temporary_result52) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin46::MlSource1TemporaryConvertStringNumberRationalJoin46Return(source_ml_source1_temporary_result52)
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_source2_s847 = self::source_ml_source1_temporary_convert_string_number_rational_loop53(denominator.clone(), exponent, numerator.clone());
                            match source_ml_source2_s847.clone() {
                                self::MlSource1TemporaryConvertStringNumberRationalLoop53Result::MlSource1TemporaryConvertStringNumberRationalLoop53Done(exponent_3) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin46::MlSource1TemporaryConvertStringNumberRationalJoin46Next(exponent_3)
                                }
                                self::MlSource1TemporaryConvertStringNumberRationalLoop53Result::MlSource1TemporaryConvertStringNumberRationalLoop53Return(source_ml_source1_temporary_result53) => {
                                    self::MlSource1TemporaryConvertStringNumberRationalJoin46::MlSource1TemporaryConvertStringNumberRationalJoin46Return(source_ml_source1_temporary_result53)
                                }
                            }
                        }
                    };
                    match source_ml_source2_s845.clone() {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin46::MlSource1TemporaryConvertStringNumberRationalJoin46Next(exponent_4) => {
                            {
                                let shift = if (exponent_4 >= ((-1022f64))) {
                                    (52f64 - exponent_4)
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s848 = if (shift >= (0f64)) {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop50(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin47::MlSource1TemporaryConvertStringNumberRationalJoin47Next(numerator_2.clone(), denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop51(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin47::MlSource1TemporaryConvertStringNumberRationalJoin47Next(numerator.clone(), denominator_2.clone())
                                            }
                                        }
                                    };
                                    match source_ml_source2_s848.clone() {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin47::MlSource1TemporaryConvertStringNumberRationalJoin47Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = self::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale = denominator_3.clone();
                                                        {
                                                            let weight = self::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s849 = self::source_ml_source1_temporary_convert_string_number_rational_loop48(remainder.clone(), scale.clone(), weight.clone());
                                                                match source_ml_source2_s849.clone() {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop48Result::MlSource1TemporaryConvertStringNumberRationalLoop48Done(scale_2, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s850 = self::source_ml_source1_temporary_convert_string_number_rational_loop49(quotient.clone(), remainder.clone(), scale_2.clone(), weight_2.clone());
                                                                            match source_ml_source2_s850.clone() {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop49Result::MlSource1TemporaryConvertStringNumberRationalLoop49Done(quotient_2, remainder_2) => {
                                                                                    {
                                                                                        let twice = remainder_2.mul(&self::ml::Big::from_i128(2));
                                                                                        {
                                                                                            let quotient_4 = if ((twice > denominator_3) || ((twice == denominator_3) && ((self::ml::divide(&quotient_2, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))))) {
                                                                                                {
                                                                                                    let quotient_3 = quotient_2.add(&self::ml::Big::from_i128(1));
                                                                                                    quotient_3.clone()
                                                                                                }
                                                                                            } else {
                                                                                                quotient_2.clone()
                                                                                            };
                                                                                            (self::read_string_number_exact_mantissa(quotient_4.clone()) * self::read_string_number_binary_power((-shift)))
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
                                        }
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryConvertStringNumberRationalJoin46::MlSource1TemporaryConvertStringNumberRationalJoin46Return(source_ml_source1_temporary_result51) => {
                            source_ml_source1_temporary_result51
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_string_number_infinity(units: Vec<f64>, start: f64, end: f64) -> bool {
    if (((end - start)) != (8f64)) {
        false
    } else {
        {
            let expected = vec![73f64, 110f64, 102f64, 105f64, 110f64, 105f64, 116f64, 121f64];
            {
                let index = 0f64;
                {
                    let source_ml_source2_s851 = self::source_ml_source1_temporary_accept_string_number_infinity_loop54(units, start, expected, index);
                    match source_ml_source2_s851.clone() {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop54Result::MlSource1TemporaryAcceptStringNumberInfinityLoop54Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop54Result::MlSource1TemporaryAcceptStringNumberInfinityLoop54Return(source_ml_source1_temporary_result54) => {
                            source_ml_source1_temporary_result54
                        }
                    }
                }
            }
        }
    }
}

pub fn convert_java_script_string_number_units(units: Vec<f64>) -> f64 {
    {
        let index = 0f64;
        {
            let source_ml_source2_s852 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop55(units.clone(), index);
            match source_ml_source2_s852.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop55Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop55Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop56(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop57(units.clone(), start_2, end);
                                    if (start_2 == end_2) {
                                        0f64
                                    } else {
                                        {
                                            let negative = false;
                                            {
                                                let signed = false;
                                                {
                                                    let sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(start_2))).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let source_ml_source2_s853 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = (start_2 + 1f64);
                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s853.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin58Next(start_4, negative_3, signed_3) => {
                                                                if (start_4 == end_2) {
                                                                    f64::NAN
                                                                } else {
                                                                    if self::accept_string_number_infinity(units.clone(), start_4, end_2) {
                                                                        if negative_3 {
                                                                            f64::NEG_INFINITY
                                                                        } else {
                                                                            f64::INFINITY
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let integer_digits = vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(1), self::ml::Big::from_i128(2), self::ml::Big::from_i128(3), self::ml::Big::from_i128(4), self::ml::Big::from_i128(5), self::ml::Big::from_i128(6), self::ml::Big::from_i128(7), self::ml::Big::from_i128(8), self::ml::Big::from_i128(9), self::ml::Big::from_i128(10), self::ml::Big::from_i128(11), self::ml::Big::from_i128(12), self::ml::Big::from_i128(13), self::ml::Big::from_i128(14), self::ml::Big::from_i128(15)];
                                                                            {
                                                                                let source_ml_source2_s855 = if ((!signed_3 && (((self::ml_array::optional_at(&units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
                                                                                    {
                                                                                        let marker = (self::ml_array::optional_at(&units, self::ml_array::number_index((start_4 + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                                                                                        {
                                                                                            let radix = if ((marker == (120f64)) || (marker == (88f64))) {
                                                                                                16f64
                                                                                            } else {
                                                                                                if ((marker == (98f64)) || (marker == (66f64))) {
                                                                                                    2f64
                                                                                                } else {
                                                                                                    if ((marker == (111f64)) || (marker == (79f64))) {
                                                                                                        8f64
                                                                                                    } else {
                                                                                                        0f64
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            if (radix != (0f64)) {
                                                                                                {
                                                                                                    let value = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let overflow = false;
                                                                                                        {
                                                                                                            let limit = self::read_string_number_decimal_power(310f64);
                                                                                                            {
                                                                                                                let base = if (radix == (16f64)) {
                                                                                                                    self::ml::Big::from_i128(16)
                                                                                                                } else {
                                                                                                                    if (radix == (8f64)) {
                                                                                                                        self::ml::Big::from_i128(8)
                                                                                                                    } else {
                                                                                                                        self::ml::Big::from_i128(2)
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let start_5 = (start_4 + 2f64);
                                                                                                                    if (start_5 == end_2) {
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let source_ml_source2_s856 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop66(units.clone(), end_2, integer_digits.clone(), radix, value.clone(), overflow, limit.clone(), base.clone(), start_5);
                                                                                                                            match source_ml_source2_s856.clone() {
                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop66Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop66Done(value_2, overflow_2) => {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Return(if overflow_2 {
                                                                                                                                        f64::INFINITY
                                                                                                                                    } else {
                                                                                                                                        self::convert_string_number_rational(value_2.clone(), self::ml::Big::from_i128(1))
                                                                                                                                    })
                                                                                                                                }
                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop66Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop66Return(source_ml_source1_temporary_result66) => {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Return(source_ml_source1_temporary_result66)
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
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Next(start_4)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s855.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Next(start_6) => {
                                                                                        {
                                                                                            let at = start_6;
                                                                                            {
                                                                                                let fraction = 0f64;
                                                                                                {
                                                                                                    let digits = 0f64;
                                                                                                    {
                                                                                                        let significant = 0f64;
                                                                                                        {
                                                                                                            let dot = false;
                                                                                                            {
                                                                                                                let source_ml_source2_s857 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop60(units.clone(), end_2, at, fraction, digits, significant, dot);
                                                                                                                match source_ml_source2_s857.clone() {
                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop60Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop60Done(at_2, fraction_2, digits_2, significant_2) => {
                                                                                                                        if (digits_2 == (0f64)) {
                                                                                                                            f64::NAN
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let decimal_end = at_2;
                                                                                                                                {
                                                                                                                                    let exponent = 0f64;
                                                                                                                                    {
                                                                                                                                        let exponent_limit = ((units.len() as f64) + 10000f64);
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_s860 = if ((at_2 < end_2) && ((((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (101f64)) || (((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (69f64)))) {
                                                                                                                                                {
                                                                                                                                                    let at_3 = (at_2 + 1f64);
                                                                                                                                                    {
                                                                                                                                                        let exponent_negative = false;
                                                                                                                                                        {
                                                                                                                                                            let exponent_sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| (-1f64));
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_s861 = if ((exponent_sign == (43f64)) || (exponent_sign == (45f64))) {
                                                                                                                                                                    {
                                                                                                                                                                        let exponent_negative_2 = (exponent_sign == (45f64));
                                                                                                                                                                        {
                                                                                                                                                                            let at_4 = (at_3 + 1f64);
                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64Next(at_4, exponent_negative_2)
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64Next(at_3, exponent_negative)
                                                                                                                                                                };
                                                                                                                                                                match source_ml_source2_s861.clone() {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin64Next(at_5, exponent_negative_3) => {
                                                                                                                                                                        {
                                                                                                                                                                            let first = at_5;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_s862 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop65(units.clone(), end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                                match source_ml_source2_s862.clone() {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop65Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop65Done(at_6, exponent_2) => {
                                                                                                                                                                                        if (at_6 == first) {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61Return(f64::NAN)
                                                                                                                                                                                        } else {
                                                                                                                                                                                            if exponent_negative_3 {
                                                                                                                                                                                                {
                                                                                                                                                                                                    let exponent_3 = (-exponent_2);
                                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61Next(at_6, exponent_3)
                                                                                                                                                                                                }
                                                                                                                                                                                            } else {
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61Next(at_6, exponent_2)
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61Next(at_2, exponent)
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s860.clone() {
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61Next(at_7, exponent_4) => {
                                                                                                                                                    if (at_7 != end_2) {
                                                                                                                                                        f64::NAN
                                                                                                                                                    } else {
                                                                                                                                                        if (significant_2 == (0f64)) {
                                                                                                                                                            if negative_3 {
                                                                                                                                                                (-0f64)
                                                                                                                                                            } else {
                                                                                                                                                                0f64
                                                                                                                                                            }
                                                                                                                                                        } else {
                                                                                                                                                            {
                                                                                                                                                                let exponent_5 = (exponent_4 - fraction_2);
                                                                                                                                                                {
                                                                                                                                                                    let order = (significant_2 + exponent_5);
                                                                                                                                                                    if (order > (310f64)) {
                                                                                                                                                                        if negative_3 {
                                                                                                                                                                            f64::NEG_INFINITY
                                                                                                                                                                        } else {
                                                                                                                                                                            f64::INFINITY
                                                                                                                                                                        }
                                                                                                                                                                    } else {
                                                                                                                                                                        if (order < ((-325f64))) {
                                                                                                                                                                            if negative_3 {
                                                                                                                                                                                (-0f64)
                                                                                                                                                                            } else {
                                                                                                                                                                                0f64
                                                                                                                                                                            }
                                                                                                                                                                        } else {
                                                                                                                                                                            {
                                                                                                                                                                                let mantissa_end = decimal_end;
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_s863 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop62(units.clone(), start_6, exponent_5, mantissa_end);
                                                                                                                                                                                    match source_ml_source2_s863.clone() {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop62Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop62Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa = self::ml::Big::from_i128(0);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop63(units, integer_digits, mantissa_end_2, mantissa.clone(), start_6);
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let numerator = if (exponent_6 >= (0f64)) {
                                                                                                                                                                                                            mantissa_2.mul(&self::read_string_number_decimal_power(exponent_6))
                                                                                                                                                                                                        } else {
                                                                                                                                                                                                            mantissa_2.clone()
                                                                                                                                                                                                        };
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let denominator = if (exponent_6 < (0f64)) {
                                                                                                                                                                                                                self::read_string_number_decimal_power((-exponent_6))
                                                                                                                                                                                                            } else {
                                                                                                                                                                                                                self::ml::Big::from_i128(1)
                                                                                                                                                                                                            };
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let value_3 = self::convert_string_number_rational(numerator.clone(), denominator.clone());
                                                                                                                                                                                                                if negative_3 {
                                                                                                                                                                                                                    (-value_3)
                                                                                                                                                                                                                } else {
                                                                                                                                                                                                                    value_3
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
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin61Return(source_ml_source1_temporary_result63) => {
                                                                                                                                                    source_ml_source1_temporary_result63
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
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin59Return(source_ml_source1_temporary_result65) => {
                                                                                        source_ml_source1_temporary_result65
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
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop55Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop55Return(source_ml_source1_temporary_result66_2) => {
                    source_ml_source1_temporary_result66_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_big_integer_number(value: self::ml::Big) -> f64 {
    if (value < (self::ml::Big::from_i128(0))) {
        (-(self::convert_string_number_rational(value.neg(), self::ml::Big::from_i128(1))))
    } else {
        self::convert_string_number_rational(value.clone(), self::ml::Big::from_i128(1))
    }
}

pub fn divide_java_script_big_integer(left: self::ml::Big, right: self::ml::Big) -> Vec<self::ml::Big> {
    if (right == (self::ml::Big::from_i128(0))) {
        Vec::<self::ml::Big>::new()
    } else {
        {
            let remaining = if (left < (self::ml::Big::from_i128(0))) {
                left.neg()
            } else {
                left.clone()
            };
            {
                let divisor = if (right < (self::ml::Big::from_i128(0))) {
                    right.neg()
                } else {
                    right.clone()
                };
                {
                    let shifted = divisor.clone();
                    {
                        let weight = self::ml::Big::from_i128(1);
                        {
                            let quotient = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s868 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop74(remaining.clone(), shifted.clone(), weight.clone());
                                match source_ml_source2_s868.clone() {
                                    self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop74Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop74Done(shifted_2, weight_2) => {
                                        {
                                            let source_ml_source2_s869 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop75(remaining.clone(), shifted_2.clone(), weight_2.clone(), quotient.clone());
                                            match source_ml_source2_s869.clone() {
                                                self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop75Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop75Done(remaining_2, quotient_2) => {
                                                    vec![if (((left < (self::ml::Big::from_i128(0)))) != ((right < (self::ml::Big::from_i128(0))))) {
                                                        quotient_2.neg()
                                                    } else {
                                                        quotient_2.clone()
                                                    }, if (left < (self::ml::Big::from_i128(0))) {
                                                        remaining_2.neg()
                                                    } else {
                                                        remaining_2.clone()
                                                    }]
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
}

pub fn power_java_script_big_integer(base: self::ml::Big, exponent: self::ml::Big) -> Vec<self::ml::Big> {
    if (exponent < (self::ml::Big::from_i128(0))) {
        Vec::<self::ml::Big>::new()
    } else {
        {
            let result = self::ml::Big::from_i128(1);
            {
                let result_2 = self::source_ml_source1_temporary_power_java_script_big_integer_loop76(result.clone(), base.clone(), exponent.clone());
                vec![result_2.clone()]
            }
        }
    }
}
