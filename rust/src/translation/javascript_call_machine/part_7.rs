// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptStringNumberInfinityLoop28Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin32;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin38;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Result;
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result;
use super::MlSource1TemporaryConvertStringNumberRationalJoin20;
use super::MlSource1TemporaryConvertStringNumberRationalJoin21;
use super::MlSource1TemporaryConvertStringNumberRationalLoop22Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop23Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop26Result;
use super::MlSource1TemporaryConvertStringNumberRationalLoop27Result;
use super::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop12Result;
use super::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result;
use super::MlSource1TemporaryReadStringNumberBinaryPowerLoop18Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::source_ml_source1_temporary_accept_string_number_infinity_loop28;
use super::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop10;
use super::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop11;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop29;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop30;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop31;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop34;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop36;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop37;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop39;
use super::source_ml_source1_temporary_convert_java_script_string_number_units_loop40;
use super::source_ml_source1_temporary_convert_string_number_rational_loop22;
use super::source_ml_source1_temporary_convert_string_number_rational_loop23;
use super::source_ml_source1_temporary_convert_string_number_rational_loop24;
use super::source_ml_source1_temporary_convert_string_number_rational_loop25;
use super::source_ml_source1_temporary_convert_string_number_rational_loop26;
use super::source_ml_source1_temporary_convert_string_number_rational_loop27;
use super::source_ml_source1_temporary_read_java_script_utf8_string_units_loop12;
use super::source_ml_source1_temporary_read_java_script_utf8_string_units_loop13;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop18;
use super::source_ml_source1_temporary_read_string_number_binary_power_loop19;
use super::source_ml_source1_temporary_read_string_number_decimal_power_loop16;
use super::source_ml_source1_temporary_read_string_number_exact_mantissa_loop17;

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
pub fn read_java_script_number_bitwise_operation_code(operation: &self::ml_string::Text) -> f64 {
    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
        0f64
    } else {
        if ((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
            1f64
        } else {
            if ((*operation) == (self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 69, 113, 117, 97, 108, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
                2f64
            } else {
                if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 76, 101, 102, 116]))) {
                    3f64
                } else {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116]))) {
                        4f64
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116, 85, 110, 115, 105, 103, 110, 101, 100]))) {
                            5f64
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
                                6f64
                            } else {
                                (-1f64)
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
pub fn read_java_script_number_bitwise_operation(symbol: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*symbol) == (self::ml_string::Text::from_units(vec![38]))) {
        self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115])
    } else {
        if ((*symbol) == (self::ml_string::Text::from_units(vec![124]))) {
            self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115])
        } else {
            if ((*symbol) == (self::ml_string::Text::from_units(vec![94]))) {
                self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 69, 113, 117, 97, 108, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115])
            } else {
                if ((*symbol) == (self::ml_string::Text::from_units(vec![60, 60]))) {
                    self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 76, 101, 102, 116])
                } else {
                    if ((*symbol) == (self::ml_string::Text::from_units(vec![62, 62]))) {
                        self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116])
                    } else {
                        if ((*symbol) == (self::ml_string::Text::from_units(vec![62, 62, 62]))) {
                            self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116, 85, 110, 115, 105, 103, 110, 101, 100])
                        } else {
                            if ((*symbol) == (self::ml_string::Text::from_units(vec![126]))) {
                                self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115])
                            } else {
                                self::ml_string::Text::from_units(vec![])
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
pub fn read_java_script_number_bitwise_assignment(symbol: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*symbol) == (self::ml_string::Text::from_units(vec![38, 61]))) {
        self::read_java_script_number_bitwise_operation(&(self::ml_string::Text::from_units(vec![38])))
    } else {
        if ((*symbol) == (self::ml_string::Text::from_units(vec![124, 61]))) {
            self::read_java_script_number_bitwise_operation(&(self::ml_string::Text::from_units(vec![124])))
        } else {
            if ((*symbol) == (self::ml_string::Text::from_units(vec![94, 61]))) {
                self::read_java_script_number_bitwise_operation(&(self::ml_string::Text::from_units(vec![94])))
            } else {
                if ((*symbol) == (self::ml_string::Text::from_units(vec![60, 60, 61]))) {
                    self::read_java_script_number_bitwise_operation(&(self::ml_string::Text::from_units(vec![60, 60])))
                } else {
                    if ((*symbol) == (self::ml_string::Text::from_units(vec![62, 62, 61]))) {
                        self::read_java_script_number_bitwise_operation(&(self::ml_string::Text::from_units(vec![62, 62])))
                    } else {
                        if ((*symbol) == (self::ml_string::Text::from_units(vec![62, 62, 62, 61]))) {
                            self::read_java_script_number_bitwise_operation(&(self::ml_string::Text::from_units(vec![62, 62, 62])))
                        } else {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_bitwise_symbol(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
        self::ml_string::Text::from_units(vec![38])
    } else {
        if ((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
            self::ml_string::Text::from_units(vec![124])
        } else {
            if ((*operation) == (self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 69, 113, 117, 97, 108, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
                self::ml_string::Text::from_units(vec![94])
            } else {
                if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 76, 101, 102, 116]))) {
                    self::ml_string::Text::from_units(vec![60, 60])
                } else {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116]))) {
                        self::ml_string::Text::from_units(vec![62, 62])
                    } else {
                        if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116, 85, 110, 115, 105, 103, 110, 101, 100]))) {
                            self::ml_string::Text::from_units(vec![62, 62, 62])
                        } else {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
                                self::ml_string::Text::from_units(vec![126])
                            } else {
                                self::ml_string::Text::from_units(vec![])
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
pub fn read_java_script_number_bitwise_arity(operation: &self::ml_string::Text) -> f64 {
    {
        let code = self::read_java_script_number_bitwise_operation_code(operation);
        if (code == (6f64)) {
            1f64
        } else {
            if (code >= (0f64)) {
                2f64
            } else {
                0f64
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_number_bitwise_operation(kind: &self::ml_string::Text, operation: &self::ml_string::Text) -> bool {
    (((*kind) == (self::ml_string::Text::from_units(vec![109, 97, 116, 104]))) && ((self::read_java_script_number_bitwise_arity(operation)) > (0f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_number_bitwise_operand_count(operation: &self::ml_string::Text, count: f64) -> bool {
    {
        let arity = self::read_java_script_number_bitwise_arity(operation);
        ((arity > (0f64)) && (count == arity))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_number_bitwise_precedence(symbol: &self::ml_string::Text, level: f64) -> bool {
    ((((level == (0f64)) && ((*symbol) == (self::ml_string::Text::from_units(vec![124])))) || ((level == (1f64)) && ((*symbol) == (self::ml_string::Text::from_units(vec![94]))))) || ((level == (2f64)) && ((*symbol) == (self::ml_string::Text::from_units(vec![38])))))
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
                            let factor_2 = self::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop10(count, factor, position);
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
                                        let result_2 = self::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop11(operation, first, second, result, weight, position_2);
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
pub fn conjoin_java_script_number_bits(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(0f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn disjoin_java_script_number_bits(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(1f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn exclude_equal_java_script_number_bits(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(2f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn shift_java_script_number_bits_left(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(3f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn shift_java_script_number_bits_right(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(4f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn shift_java_script_number_bits_right_unsigned(left: f64, right: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(5f64, left, right)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn complement_java_script_number_bits(left: f64) -> f64 {
    self::apply_java_script_number_bitwise_operation(6f64, left, 0f64)
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

pub fn read_java_script_utf8_string_units(bytes: Vec<f64>) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source2_s110 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop12(bytes.clone(), index);
            match source_ml_source2_s110.clone() {
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop12Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop12Done => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source2_s111 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop13(bytes, units, index_2);
                                match source_ml_source2_s111.clone() {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Done(units_2) => {
                                        units_2
                                    }
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Return(source_ml_source1_temporary_result13) => {
                                        source_ml_source1_temporary_result13
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop12Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop12Return(source_ml_source1_temporary_result15) => {
                    source_ml_source1_temporary_result15
                }
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
                let value_2 = self::source_ml_source1_temporary_read_string_number_decimal_power_loop16(exponent, value.clone(), base.clone());
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
                let result_2 = self::source_ml_source1_temporary_read_string_number_exact_mantissa_loop17(value.clone(), result, scale);
                result_2
            }
        }
    }
}

pub fn read_string_number_binary_power(exponent: f64) -> f64 {
    {
        let result = 1f64;
        {
            let source_ml_source2_s112 = self::source_ml_source1_temporary_read_string_number_binary_power_loop18(exponent, result);
            match source_ml_source2_s112.clone() {
                self::MlSource1TemporaryReadStringNumberBinaryPowerLoop18Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop18Done(exponent_2, result_2) => {
                    {
                        let result_3 = self::source_ml_source1_temporary_read_string_number_binary_power_loop19(exponent_2, result_2);
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
                    let source_ml_source2_s115 = if (numerator >= denominator) {
                        {
                            let scale = denominator.clone();
                            {
                                let source_ml_source2_s116 = self::source_ml_source1_temporary_convert_string_number_rational_loop26(numerator.clone(), exponent, scale.clone());
                                match source_ml_source2_s116.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop26Result::MlSource1TemporaryConvertStringNumberRationalLoop26Done(exponent_2) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin20::MlSource1TemporaryConvertStringNumberRationalJoin20Next(exponent_2)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop26Result::MlSource1TemporaryConvertStringNumberRationalLoop26Return(source_ml_source1_temporary_result26) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin20::MlSource1TemporaryConvertStringNumberRationalJoin20Return(source_ml_source1_temporary_result26)
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let scale_2 = numerator.clone();
                            {
                                let source_ml_source2_s117 = self::source_ml_source1_temporary_convert_string_number_rational_loop27(denominator.clone(), exponent, scale_2.clone());
                                match source_ml_source2_s117.clone() {
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop27Result::MlSource1TemporaryConvertStringNumberRationalLoop27Done(exponent_3) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin20::MlSource1TemporaryConvertStringNumberRationalJoin20Next(exponent_3)
                                    }
                                    self::MlSource1TemporaryConvertStringNumberRationalLoop27Result::MlSource1TemporaryConvertStringNumberRationalLoop27Return(source_ml_source1_temporary_result27) => {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin20::MlSource1TemporaryConvertStringNumberRationalJoin20Return(source_ml_source1_temporary_result27)
                                    }
                                }
                            }
                        }
                    };
                    match source_ml_source2_s115.clone() {
                        self::MlSource1TemporaryConvertStringNumberRationalJoin20::MlSource1TemporaryConvertStringNumberRationalJoin20Next(exponent_4) => {
                            {
                                let shift = if (exponent_4 >= ((-1022f64))) {
                                    (52f64 - exponent_4)
                                } else {
                                    1074f64
                                };
                                {
                                    let source_ml_source2_s118 = if (shift >= (0f64)) {
                                        {
                                            let index = 0f64;
                                            {
                                                let numerator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop24(numerator.clone(), shift, index);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin21::MlSource1TemporaryConvertStringNumberRationalJoin21Next(numerator_2.clone(), denominator.clone())
                                            }
                                        }
                                    } else {
                                        {
                                            let index_2 = 0f64;
                                            {
                                                let denominator_2 = self::source_ml_source1_temporary_convert_string_number_rational_loop25(denominator.clone(), shift, index_2);
                                                self::MlSource1TemporaryConvertStringNumberRationalJoin21::MlSource1TemporaryConvertStringNumberRationalJoin21Next(numerator.clone(), denominator_2.clone())
                                            }
                                        }
                                    };
                                    match source_ml_source2_s118.clone() {
                                        self::MlSource1TemporaryConvertStringNumberRationalJoin21::MlSource1TemporaryConvertStringNumberRationalJoin21Next(numerator_3, denominator_3) => {
                                            {
                                                let quotient = self::ml::Big::from_i128(0);
                                                {
                                                    let remainder = numerator_3.clone();
                                                    {
                                                        let scale_3 = denominator_3.clone();
                                                        {
                                                            let weight = self::ml::Big::from_i128(1);
                                                            {
                                                                let source_ml_source2_s119 = self::source_ml_source1_temporary_convert_string_number_rational_loop22(remainder.clone(), scale_3.clone(), weight.clone());
                                                                match source_ml_source2_s119.clone() {
                                                                    self::MlSource1TemporaryConvertStringNumberRationalLoop22Result::MlSource1TemporaryConvertStringNumberRationalLoop22Done(scale_4, weight_2) => {
                                                                        {
                                                                            let source_ml_source2_s120 = self::source_ml_source1_temporary_convert_string_number_rational_loop23(quotient.clone(), remainder.clone(), scale_4.clone(), weight_2.clone());
                                                                            match source_ml_source2_s120.clone() {
                                                                                self::MlSource1TemporaryConvertStringNumberRationalLoop23Result::MlSource1TemporaryConvertStringNumberRationalLoop23Done(quotient_2, remainder_2) => {
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
                        self::MlSource1TemporaryConvertStringNumberRationalJoin20::MlSource1TemporaryConvertStringNumberRationalJoin20Return(source_ml_source1_temporary_result25) => {
                            source_ml_source1_temporary_result25
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
                    let source_ml_source2_s121 = self::source_ml_source1_temporary_accept_string_number_infinity_loop28(units, start, expected, index);
                    match source_ml_source2_s121.clone() {
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop28Result::MlSource1TemporaryAcceptStringNumberInfinityLoop28Done => {
                            true
                        }
                        self::MlSource1TemporaryAcceptStringNumberInfinityLoop28Result::MlSource1TemporaryAcceptStringNumberInfinityLoop28Return(source_ml_source1_temporary_result28) => {
                            source_ml_source1_temporary_result28
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
            let source_ml_source2_s122 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop29(units.clone(), index);
            match source_ml_source2_s122.clone() {
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done => {
                    {
                        let start = 0f64;
                        {
                            let end = (units.len() as f64);
                            {
                                let start_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop30(units.clone(), start, end);
                                {
                                    let end_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop31(units.clone(), start_2, end);
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
                                                        let source_ml_source2_s123 = if ((sign == (43f64)) || (sign == (45f64))) {
                                                            {
                                                                let negative_2 = (sign == (45f64));
                                                                {
                                                                    let signed_2 = true;
                                                                    {
                                                                        let start_3 = (start_2 + 1f64);
                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin32::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin32Next(start_3, negative_2, signed_2)
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin32::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin32Next(start_2, negative, signed)
                                                        };
                                                        match source_ml_source2_s123.clone() {
                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin32::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin32Next(start_4, negative_3, signed_3) => {
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
                                                                                let source_ml_source2_s125 = if ((!signed_3 && (((self::ml_array::optional_at(&units, self::ml_array::number_index(start_4))).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((end_2 - start_4)) >= (2f64))) {
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
                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Return(f64::NAN)
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let index_2 = start_5;
                                                                                                                            {
                                                                                                                                let source_ml_source2_s126 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop40(units.clone(), end_2, integer_digits.clone(), radix, value.clone(), overflow, limit.clone(), base.clone(), index_2);
                                                                                                                                match source_ml_source2_s126.clone() {
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Done(value_2, overflow_2) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Return(if overflow_2 {
                                                                                                                                            f64::INFINITY
                                                                                                                                        } else {
                                                                                                                                            self::convert_string_number_rational(value_2.clone(), self::ml::Big::from_i128(1))
                                                                                                                                        })
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Return(source_ml_source1_temporary_result40) => {
                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Return(source_ml_source1_temporary_result40)
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
                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Next(start_4)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Next(start_4)
                                                                                };
                                                                                match source_ml_source2_s125.clone() {
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Next(start_6) => {
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
                                                                                                                let source_ml_source2_s127 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop34(units.clone(), end_2, at, fraction, digits, significant, dot);
                                                                                                                match source_ml_source2_s127.clone() {
                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Done(at_2, fraction_2, digits_2, significant_2) => {
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
                                                                                                                                            let source_ml_source2_s130 = if ((at_2 < end_2) && ((((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (101f64)) || (((self::ml_array::optional_at(&units, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| (-1f64))) == (69f64)))) {
                                                                                                                                                {
                                                                                                                                                    let at_3 = (at_2 + 1f64);
                                                                                                                                                    {
                                                                                                                                                        let exponent_negative = false;
                                                                                                                                                        {
                                                                                                                                                            let exponent_sign = (self::ml_array::optional_at(&units, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| (-1f64));
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_s131 = if ((exponent_sign == (43f64)) || (exponent_sign == (45f64))) {
                                                                                                                                                                    {
                                                                                                                                                                        let exponent_negative_2 = (exponent_sign == (45f64));
                                                                                                                                                                        {
                                                                                                                                                                            let at_4 = (at_3 + 1f64);
                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin38::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin38Next(at_4, exponent_negative_2)
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                } else {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin38::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin38Next(at_3, exponent_negative)
                                                                                                                                                                };
                                                                                                                                                                match source_ml_source2_s131.clone() {
                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin38::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin38Next(at_5, exponent_negative_3) => {
                                                                                                                                                                        {
                                                                                                                                                                            let first = at_5;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_s132 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop39(units.clone(), end_2, at_5, exponent, exponent_limit);
                                                                                                                                                                                match source_ml_source2_s132.clone() {
                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Done(at_6, exponent_2) => {
                                                                                                                                                                                        if (at_6 == first) {
                                                                                                                                                                                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35Return(f64::NAN)
                                                                                                                                                                                        } else {
                                                                                                                                                                                            if exponent_negative_3 {
                                                                                                                                                                                                {
                                                                                                                                                                                                    let exponent_3 = (-exponent_2);
                                                                                                                                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35Next(at_6, exponent_3)
                                                                                                                                                                                                }
                                                                                                                                                                                            } else {
                                                                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35Next(at_6, exponent_2)
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
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35Next(at_2, exponent)
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s130.clone() {
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35Next(at_7, exponent_4) => {
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
                                                                                                                                                                                    let source_ml_source2_s133 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop36(units.clone(), start_6, exponent_5, mantissa_end);
                                                                                                                                                                                    match source_ml_source2_s133.clone() {
                                                                                                                                                                                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Done(exponent_6, mantissa_end_2) => {
                                                                                                                                                                                            {
                                                                                                                                                                                                let mantissa = self::ml::Big::from_i128(0);
                                                                                                                                                                                                {
                                                                                                                                                                                                    let index_3 = start_6;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let mantissa_2 = self::source_ml_source1_temporary_convert_java_script_string_number_units_loop37(units, integer_digits, mantissa_end_2, mantissa.clone(), index_3);
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
                                                                                                                                                }
                                                                                                                                                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35Return(source_ml_source1_temporary_result37) => {
                                                                                                                                                    source_ml_source1_temporary_result37
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
                                                                                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33::MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Return(source_ml_source1_temporary_result39) => {
                                                                                        source_ml_source1_temporary_result39
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
                self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(source_ml_source1_temporary_result40_2) => {
                    source_ml_source1_temporary_result40_2
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
