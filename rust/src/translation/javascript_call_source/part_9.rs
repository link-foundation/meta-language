// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop57Result;
use super::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop58Result;
use super::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop63Result;
use super::MlSource1TemporaryRootNumberPowerBoundJoin56;
use super::apply_java_script_number_bitwise_operation;
use super::convert_java_script_number_big_integer;
use super::convert_string_number_rational;
use super::divide_java_script_big_integer;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::normalize_number_power_bound;
use super::power_java_script_big_integer;
use super::source_ml_source1_temporary_decompose_number_power_magnitude_loop57;
use super::source_ml_source1_temporary_decompose_number_power_magnitude_loop58;
use super::source_ml_source1_temporary_power_java_script_number_loop59;
use super::source_ml_source1_temporary_read_java_script_call_integer_width_loop63;
use super::source_ml_source1_temporary_root_number_power_integer_loop54;
use super::source_ml_source1_temporary_root_number_power_integer_loop55;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn divide_number_power_bound(numerator: self::ml::Big, denominator: self::ml::Big, upward: bool) -> self::ml::Big {
    {
        let result = self::divide_java_script_big_integer(numerator.clone(), denominator.clone());
        {
            let quotient = (self::ml_array::optional_at(&result, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
            {
                let remainder = (self::ml_array::optional_at(&result, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                quotient.add(&if (upward && (remainder != (self::ml::Big::from_i128(0)))) {
                    self::ml::Big::from_i128(1)
                } else {
                    self::ml::Big::from_i128(0)
                })
            }
        }
    }
}

pub fn root_number_power_integer(value: self::ml::Big) -> self::ml::Big {
    if (value <= (self::ml::Big::from_i128(0))) {
        self::ml::Big::from_i128(0)
    } else {
        {
            let root = self::ml::Big::from_i128(1);
            {
                let remaining = value.clone();
                {
                    let root_2 = self::source_ml_source1_temporary_root_number_power_integer_loop54(root.clone(), remaining.clone());
                    {
                        let root_3 = root_2.mul(&self::ml::Big::from_i128(2));
                        self::source_ml_source1_temporary_root_number_power_integer_loop55(value.clone(), root_3.clone())
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn multiply_number_power_interval(left: Vec<self::ml::Big>, right: Vec<self::ml::Big>, scale: self::ml::Big) -> Vec<self::ml::Big> {
    {
        let lower = self::normalize_number_power_bound(self::divide_number_power_bound(((self::ml_array::optional_at(&left, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).mul(&(self::ml_array::optional_at(&right, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), scale.clone(), false), ((self::ml_array::optional_at(&left, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).add(&(self::ml_array::optional_at(&right, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), scale.clone(), false);
        {
            let upper = self::normalize_number_power_bound(self::divide_number_power_bound(((self::ml_array::optional_at(&left, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).mul(&(self::ml_array::optional_at(&right, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), scale.clone(), true), ((self::ml_array::optional_at(&left, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).add(&(self::ml_array::optional_at(&right, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), scale.clone(), true);
            vec![(self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))]
        }
    }
}

pub fn root_number_power_bound(value: self::ml::Big, exponent: self::ml::Big, scale: self::ml::Big, upward: bool) -> Vec<self::ml::Big> {
    {
        let source_ml_source2_s170 = if ((self::ml::divide(&exponent, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) != (self::ml::Big::from_i128(0))) {
            {
                let value_2 = value.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent.sub(&self::ml::Big::from_i128(1));
                    self::MlSource1TemporaryRootNumberPowerBoundJoin56::MlSource1TemporaryRootNumberPowerBoundJoin56Next(value_2.clone(), exponent_2.clone())
                }
            }
        } else {
            self::MlSource1TemporaryRootNumberPowerBoundJoin56::MlSource1TemporaryRootNumberPowerBoundJoin56Next(value.clone(), exponent.clone())
        };
        match source_ml_source2_s170.clone() {
            self::MlSource1TemporaryRootNumberPowerBoundJoin56::MlSource1TemporaryRootNumberPowerBoundJoin56Next(value_3, exponent_3) => {
                {
                    let square = value_3.mul(&scale);
                    {
                        let root = self::root_number_power_integer(square.clone());
                        {
                            let root_3 = if (upward && ((root.mul(&root)) < square)) {
                                {
                                    let root_2 = root.add(&self::ml::Big::from_i128(1));
                                    root_2.clone()
                                }
                            } else {
                                root.clone()
                            };
                            self::normalize_number_power_bound(root_3.clone(), self::ml::divide(&exponent_3, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false), scale.clone(), upward)
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn root_number_power_interval(interval: Vec<self::ml::Big>, scale: self::ml::Big) -> Vec<self::ml::Big> {
    {
        let lower = self::root_number_power_bound((self::ml_array::optional_at(&interval, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&interval, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), scale.clone(), false);
        {
            let upper = self::root_number_power_bound((self::ml_array::optional_at(&interval, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&interval, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), scale.clone(), true);
            vec![(self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn invert_number_power_interval(interval: Vec<self::ml::Big>, scale: self::ml::Big) -> Vec<self::ml::Big> {
    {
        let lower = self::normalize_number_power_bound(self::divide_number_power_bound(scale.mul(&scale), (self::ml_array::optional_at(&interval, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), false), ((self::ml_array::optional_at(&interval, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).neg(), scale.clone(), false);
        {
            let upper = self::normalize_number_power_bound(self::divide_number_power_bound(scale.mul(&scale), (self::ml_array::optional_at(&interval, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), true), ((self::ml_array::optional_at(&interval, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).neg(), scale.clone(), true);
            vec![(self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))]
        }
    }
}

pub fn decompose_number_power_magnitude(value: f64) -> Vec<self::ml::Big> {
    {
        let exponent = self::ml::Big::from_i128(0);
        {
            let source_ml_source2_s171 = self::source_ml_source1_temporary_decompose_number_power_magnitude_loop57(value, exponent.clone());
            match source_ml_source2_s171.clone() {
                self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop57Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop57Done(value_2, exponent_2) => {
                    {
                        let source_ml_source2_s172 = self::source_ml_source1_temporary_decompose_number_power_magnitude_loop58(value_2, exponent_2.clone());
                        match source_ml_source2_s172.clone() {
                            self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop58Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop58Done(value_3, exponent_3) => {
                                vec![(self::ml_array::optional_at(&self::convert_java_script_number_big_integer((value_3 * 4503599627370496f64)), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), exponent_3.sub(&self::ml::Big::from_i128(52))]
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
pub fn round_number_power_bound(value: self::ml::Big, exponent: self::ml::Big, precision: self::ml::Big) -> f64 {
    if ((value == (self::ml::Big::from_i128(0))) || (exponent < (self::ml::Big::from_i128(-1076)))) {
        0f64
    } else {
        if (exponent > (self::ml::Big::from_i128(1024))) {
            f64::INFINITY
        } else {
            {
                let shift = exponent.sub(&precision);
                if (shift >= (self::ml::Big::from_i128(0))) {
                    self::convert_string_number_rational(value.mul(&(self::ml_array::optional_at(&self::power_java_script_big_integer(self::ml::Big::from_i128(2), shift.clone()), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))), self::ml::Big::from_i128(1))
                } else {
                    self::convert_string_number_rational(value.clone(), (self::ml_array::optional_at(&self::power_java_script_big_integer(self::ml::Big::from_i128(2), shift.neg()), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)))
                }
            }
        }
    }
}

pub fn power_java_script_number(base: f64, exponent: f64) -> f64 {
    if exponent.is_nan() {
        f64::NAN
    } else {
        if (exponent == (0f64)) {
            1f64
        } else {
            if base.is_nan() {
                f64::NAN
            } else {
                {
                    let magnitude = base.abs();
                    if !(exponent.is_finite()) {
                        if (magnitude == (1f64)) {
                            f64::NAN
                        } else {
                            if (magnitude > (1f64)) {
                                if (exponent > (0f64)) {
                                    f64::INFINITY
                                } else {
                                    0f64
                                }
                            } else {
                                if (exponent > (0f64)) {
                                    0f64
                                } else {
                                    f64::INFINITY
                                }
                            }
                        }
                    } else {
                        {
                            let odd = (self::ml_math::is_integer(exponent) && (((exponent.abs() % 2f64)) == (1f64)));
                            {
                                let negative = (((base < (0f64)) || ((base == (0f64)) && (((1f64 / base)) < (0f64)))) && odd);
                                if (base == (0f64)) {
                                    {
                                        let value = if (exponent > (0f64)) {
                                            0f64
                                        } else {
                                            f64::INFINITY
                                        };
                                        if negative {
                                            (-value)
                                        } else {
                                            value
                                        }
                                    }
                                } else {
                                    if !(base.is_finite()) {
                                        {
                                            let value_2 = if (exponent > (0f64)) {
                                                f64::INFINITY
                                            } else {
                                                0f64
                                            };
                                            if negative {
                                                (-value_2)
                                            } else {
                                                value_2
                                            }
                                        }
                                    } else {
                                        if ((base < (0f64)) && !(self::ml_math::is_integer(exponent))) {
                                            f64::NAN
                                        } else {
                                            if (magnitude == (1f64)) {
                                                if negative {
                                                    (-1f64)
                                                } else {
                                                    1f64
                                                }
                                            } else {
                                                {
                                                    let input = self::decompose_number_power_magnitude(magnitude);
                                                    {
                                                        let power = self::decompose_number_power_magnitude(exponent.abs());
                                                        {
                                                            let precision = self::ml::Big::from_i128(192);
                                                            self::source_ml_source1_temporary_power_java_script_number_loop59(exponent, negative, input, power, precision.clone())
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

pub fn accept_java_script_exponentiation_base(prefix: String, parenthesized: bool) -> bool {
    (parenthesized || !(self::ml_array::includes(&vec![String::from("+"), String::from("-"), String::from("!"), String::from("~"), String::from("typeof"), String::from("void"), String::from("delete"), String::from("await")], &prefix.clone(), 0f64, false, |a, b| a == b)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_exponent_message() -> String {
    String::from("Exponent must be positive")
}

pub fn render_java_script_big_integer_power(target: String, base: String, exponent: String) -> String {
    if (target == "JavaScript") {
        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("ml_big_integer_conversion.power(")), base)), (String::from(",")))), exponent)), (String::from(")")))
    } else {
        if (target == "Rust") {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_conversion::power_java_script_big_integer(")), base)), (String::from(",")))), exponent)), (String::from(")")))
        } else {
            if ((target == "Lean") || (target == "Rocq")) {
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.power ")), base)), (String::from(" ")))), exponent)), (String::from(")")))
            } else {
                String::from("")
            }
        }
    }
}

pub fn accept_nonzero_java_script_integer_literal(value: String) -> bool {
    (((value != "") && (value != "0")) && (value != "-0"))
}

pub fn require_java_script_big_integer_division(language: String, operation: String, kind: String, literal: String) -> bool {
    ((((language == "JavaScript") && (kind == "int")) && ((operation == "div") || (operation == "rem"))) && !(self::accept_nonzero_java_script_integer_literal(literal.clone())))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_division_message() -> String {
    String::from("Division by zero")
}

pub fn render_java_script_big_integer_division(target: String, left: String, right: String) -> String {
    if (target == "JavaScript") {
        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("ml_big_integer_conversion.divide(")), left)), (String::from(",")))), right)), (String::from(")")))
    } else {
        if (target == "Rust") {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_conversion::divide_java_script_big_integer(")), left)), (String::from(",")))), right)), (String::from(")")))
        } else {
            if ((target == "Lean") || (target == "Rocq")) {
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.divide ")), left)), (String::from(" ")))), right)), (String::from(")")))
            } else {
                String::from("")
            }
        }
    }
}

pub fn read_java_script_big_integer_conversion_form(kind: String) -> String {
    if ((kind == "int") || (kind == "nat")) {
        String::from("identity")
    } else {
        if (kind == "float") {
            String::from("number")
        } else {
            if (kind == "string") {
                String::from("string")
            } else {
                if (kind == "bool") {
                    String::from("boolean")
                } else {
                    if (kind == "unit") {
                        String::from("undefined")
                    } else {
                        if (kind == "option") {
                            String::from("optional")
                        } else {
                            String::from("unsupported")
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_big_integer_conversion_error_class(form: String) -> String {
    if (form == "string") {
        String::from("SyntaxError")
    } else {
        if (form == "number") {
            String::from("RangeError")
        } else {
            if ((form == "undefined") || (form == "null")) {
                String::from("TypeError")
            } else {
                String::from("")
            }
        }
    }
}

pub fn render_java_script_big_integer_conversion_message(form: String, value: String) -> String {
    {
        let pieces = self::read_java_script_big_integer_conversion_message_pieces(form.clone());
        if (((pieces.len() as f64)) == (2f64)) {
            format!("{}{}", (format!("{}{}", ((self::ml_array::optional_at(&pieces, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| String::from(""))), value)), ((self::ml_array::optional_at(&pieces, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| String::from(""))))
        } else {
            String::from("")
        }
    }
}

pub fn read_java_script_big_integer_conversion_message_pieces(form: String) -> Vec<String> {
    if (form == "number") {
        vec![String::from("The number "), String::from(" cannot be converted to a BigInt because it is not an integer")]
    } else {
        if (((form == "string") || (form == "undefined")) || (form == "null")) {
            vec![String::from("Cannot convert "), String::from(" to a BigInt")]
        } else {
            Vec::<String>::new()
        }
    }
}

pub fn read_java_script_big_integer_builtin_operation(name: String) -> String {
    if (name == "BigInt") {
        String::from("convertJavaScriptBigInteger")
    } else {
        String::from("")
    }
}

pub fn read_java_script_big_integer_result_operation(operation: String) -> String {
    if (operation == "compareJavaScriptNumberBigIntegerValue") {
        String::from("number-compare")
    } else {
        if (operation == "compareJavaScriptStringBigIntegerValue") {
            String::from("string-compare")
        } else {
            if (operation == "powerJavaScriptNumberValue") {
                String::from("number-power")
            } else {
                if (operation == "powerJavaScriptBigIntegerResult") {
                    String::from("power")
                } else {
                    if (operation == "divideJavaScriptBigIntegerResult") {
                        String::from("division")
                    } else {
                        if (operation == "parseJavaScriptStringBigInteger") {
                            String::from("string")
                        } else {
                            if (operation == "convertJavaScriptBooleanBigIntegerValue") {
                                String::from("boolean")
                            } else {
                                if (operation == "convertJavaScriptNumberBigIntegerResult") {
                                    String::from("number")
                                } else {
                                    String::from("")
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_linked_java_script_big_integer_result(kind: String, operation: String) -> bool {
    ((kind == "math") && ((self::read_java_script_big_integer_result_operation(operation.clone())) != ""))
}

pub fn read_java_script_big_integer_intrinsic_operand_kinds(operation: String) -> Vec<String> {
    {
        let form = self::read_java_script_big_integer_result_operation(operation.clone());
        if ((form == "division") || (form == "power")) {
            vec![String::from("int"), String::from("int")]
        } else {
            if (form == "number-power") {
                vec![String::from("float"), String::from("float")]
            } else {
                if (form == "number-compare") {
                    vec![String::from("float"), String::from("int")]
                } else {
                    if (form == "string-compare") {
                        vec![String::from("string"), String::from("int")]
                    } else {
                        if (form == "string") {
                            vec![String::from("string")]
                        } else {
                            if (form == "number") {
                                vec![String::from("float")]
                            } else {
                                if (form == "boolean") {
                                    vec![String::from("bool")]
                                } else {
                                    Vec::<String>::new()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_big_integer_intrinsic_result_kind(operation: String) -> String {
    {
        let form = self::read_java_script_big_integer_result_operation(operation.clone());
        if (((form == "number-power") || (form == "number-compare")) || (form == "string-compare")) {
            String::from("float")
        } else {
            if (form == "boolean") {
                String::from("int")
            } else {
                if (form == "") {
                    String::from("")
                } else {
                    String::from("array-integer")
                }
            }
        }
    }
}

pub fn render_java_script_big_integer_comparison(target: String, form: String, left: String, right: String, utf16: bool) -> String {
    if (form == "number-compare") {
        if (target == "JavaScript") {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("ml_big_integer_conversion.compareNumber(")), left)), (String::from(",")))), right)), (String::from(")")))
        } else {
            if (target == "Rust") {
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_conversion::compare_java_script_number_big_integer(")), left)), (String::from(",")))), right)), (String::from(")")))
            } else {
                if ((target == "Lean") || (target == "Rocq")) {
                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.compareNumber ")), left)), (String::from(" ")))), right)), (String::from(")")))
                } else {
                    String::from("")
                }
            }
        }
    } else {
        if (form != "string-compare") {
            String::from("")
        } else {
            if (target == "JavaScript") {
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("((ml_text,ml_integer)=>ml_big_integer_conversion.compareUnits(Array.from({length:ml_text.length},(_,index)=>ml_text.charCodeAt(index)),ml_integer))(")), left)), (String::from(",")))), right)), (String::from(")")))
            } else {
                if (target == "Rust") {
                    format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_conversion::compare_java_script_string_big_integer_units(&")), (if utf16 {
                        format!("{}{}", (format!("{}{}", (String::from("(")), left)), (String::from(").0.iter().copied()")))
                    } else {
                        format!("{}{}", (format!("{}{}", (String::from("(")), left)), (String::from(").encode_utf16()")))
                    }))), (String::from(".map(f64::from).collect::<Vec<f64>>(),")))), right)), (String::from(")")))
                } else {
                    if (target == "Lean") {
                        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.compareUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] ")), left)), (String::from(") ")))), right)), (String::from(")")))
                    } else {
                        if (target == "Rocq") {
                            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.compareUtf8 ((fix ml_compare_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_compare_bytes rest) end) ")), left)), (String::from(") ")))), right)), (String::from(")")))
                        } else {
                            String::from("")
                        }
                    }
                }
            }
        }
    }
}

pub fn render_java_script_number_power(target: String, base: String, exponent: String) -> String {
    if (target == "JavaScript") {
        format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("ml_big_integer_conversion.powerNumber(")), base)), (String::from(",")))), exponent)), (String::from(")")))
    } else {
        if (target == "Rust") {
            format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_conversion::power_java_script_number(")), base)), (String::from(",")))), exponent)), (String::from(")")))
        } else {
            if ((target == "Lean") || (target == "Rocq")) {
                format!("{}{}", (format!("{}{}", (format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.powerNumber ")), base)), (String::from(" ")))), exponent)), (String::from(")")))
            } else {
                String::from("")
            }
        }
    }
}

pub fn render_java_script_big_integer_result_expression(target: String, form: String, value: String, utf16: bool) -> String {
    if (form == "boolean") {
        if (target == "JavaScript") {
            format!("{}{}", (format!("{}{}", (String::from("ml_big_integer_conversion.convertBoolean(")), value)), (String::from(")")))
        } else {
            if (target == "Rust") {
                format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_conversion::convert_java_script_boolean_big_integer(")), value)), (String::from(")")))
            } else {
                if ((target == "Lean") || (target == "Rocq")) {
                    format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.convertBoolean ")), value)), (String::from(")")))
                } else {
                    String::from("")
                }
            }
        }
    } else {
        if ((form != "string") && (form != "number")) {
            String::from("")
        } else {
            if (target == "JavaScript") {
                if (form == "number") {
                    format!("{}{}", (format!("{}{}", (String::from("ml_big_integer_conversion.convertNumber(")), value)), (String::from(")")))
                } else {
                    format!("{}{}", (format!("{}{}", (String::from("((ml_text)=>ml_big_integer_conversion.convertUnits(Array.from({length:ml_text.length}, (_,index)=>ml_text.charCodeAt(index))))(")), value)), (String::from(")")))
                }
            } else {
                if (target == "Rust") {
                    if (form == "number") {
                        format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_conversion::convert_java_script_number_big_integer(")), value)), (String::from(")")))
                    } else {
                        {
                            let units = if utf16 {
                                format!("{}{}", (format!("{}{}", (String::from("(")), value)), (String::from(").0.iter().copied()")))
                            } else {
                                format!("{}{}", (format!("{}{}", (String::from("(")), value)), (String::from(").encode_utf16()")))
                            };
                            format!("{}{}", (format!("{}{}", (String::from("crate::ml_big_integer_conversion::convert_java_script_string_big_integer_units(&")), units)), (String::from(".map(f64::from).collect::<Vec<f64>>())")))
                        }
                    }
                } else {
                    if (target == "Lean") {
                        if (form == "number") {
                            format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.convertNumber ")), value)), (String::from(")")))
                        } else {
                            format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.convertUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] ")), value)), (String::from("))")))
                        }
                    } else {
                        if (target == "Rocq") {
                            if (form == "number") {
                                format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.convertNumber ")), value)), (String::from(")")))
                            } else {
                                format!("{}{}", (format!("{}{}", (String::from("(MLBigIntegerConversion.convertUtf8 ((fix ml_integer_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_integer_bytes rest) end) ")), value)), (String::from("))")))
                            }
                        } else {
                            String::from("")
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_arity(operation: &str) -> f64 {
    if (((operation == "negate integer") || (operation == "complement integer bits")) || (operation == "complement number bits")) {
        1f64
    } else {
        if self::ml_array::includes(&vec![String::from("add integers"), String::from("subtract integers"), String::from("multiply integers"), String::from("divide integers"), String::from("read integer remainder"), String::from("raise integer power"), String::from("compare integers"), String::from("conjoin integer bits"), String::from("disjoin integer bits"), String::from("exclude equal integer bits"), String::from("shift integer bits left"), String::from("shift integer bits right"), String::from("conjoin number bits"), String::from("disjoin number bits"), String::from("exclude equal number bits"), String::from("shift number bits left"), String::from("shift number bits right"), String::from("shift number bits right unsigned")], &operation.to_owned(), 0f64, false, |a, b| a == b) {
            2f64
        } else {
            0f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_width(value: self::ml::Big) -> f64 {
    {
        let rest = if (value < (self::ml::Big::from_i128(0))) {
            value.neg()
        } else {
            value.clone()
        };
        {
            let width = 0f64;
            {
                let source_ml_source2_s173 = self::source_ml_source1_temporary_read_java_script_call_integer_width_loop63(rest.clone(), width);
                match source_ml_source2_s173.clone() {
                    self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop63Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop63Done(width_2) => {
                        width_2
                    }
                    self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop63Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop63Return(source_ml_source1_temporary_result63) => {
                        source_ml_source1_temporary_result63
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_bit_code(operation: &str) -> f64 {
    if ((operation == "conjoin integer bits") || (operation == "conjoin number bits")) {
        0f64
    } else {
        if ((operation == "disjoin integer bits") || (operation == "disjoin number bits")) {
            1f64
        } else {
            if ((operation == "exclude equal integer bits") || (operation == "exclude equal number bits")) {
                2f64
            } else {
                if ((operation == "shift integer bits left") || (operation == "shift number bits left")) {
                    3f64
                } else {
                    if ((operation == "shift integer bits right") || (operation == "shift number bits right")) {
                        4f64
                    } else {
                        if (operation == "shift number bits right unsigned") {
                            5f64
                        } else {
                            if ((operation == "complement integer bits") || (operation == "complement number bits")) {
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
pub fn execute_java_script_call_number_bits(operation: &str, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let left = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let right = (self::ml_array::optional_at(&values, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Number(0f64));
            match left.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                }
                self::JavaScriptSemanticValue::Null => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f174) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                }
                self::JavaScriptSemanticValue::Number(left_value) => {
                    match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f180) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Number(value) => {
                            {
                                let code = self::read_java_script_call_integer_bit_code(operation);
                                if (code < (0f64)) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require their actual registered operation"))
                                } else {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::apply_java_script_number_bitwise_operation(code, left_value, value))))
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f181) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f182) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f183) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f184, source_ml_source2_f185) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f175) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f176) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f177) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f178, source_ml_source2_f179) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("Number bit operations require actual converted Number operands"))
                }
            }
        }
    }
}
