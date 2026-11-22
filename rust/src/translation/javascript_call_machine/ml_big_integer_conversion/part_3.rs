// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Result;
use super::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Result;
use super::MlSource1TemporaryRootNumberPowerBoundJoin44;
use super::convert_java_script_number_big_integer;
use super::convert_string_number_rational;
use super::divide_number_power_bound;
use super::ml_array;
use super::ml_math;
use super::normalize_number_power_bound;
use super::power_java_script_big_integer;
use super::root_number_power_integer;
use super::source_ml_source1_temporary_decompose_number_power_magnitude_loop45;
use super::source_ml_source1_temporary_decompose_number_power_magnitude_loop46;
use super::source_ml_source1_temporary_power_java_script_number_loop47;

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn root_number_power_bound(
    value: self::ml::Big,
    exponent: self::ml::Big,
    scale: self::ml::Big,
    upward: bool,
) -> Vec<self::ml::Big> {
    {
        let source_ml_source2_s33 = if ((self::ml::divide(
            &exponent,
            &self::ml::Big::from_i128(2),
            "trunc",
            Some("Division by zero"),
            true,
        )) != (self::ml::Big::from_i128(0)))
        {
            {
                let value_2 = value.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent.sub(&self::ml::Big::from_i128(1));
                    self::MlSource1TemporaryRootNumberPowerBoundJoin44::MlSource1TemporaryRootNumberPowerBoundJoin44Next(value_2.clone(), exponent_2.clone())
                }
            }
        } else {
            self::MlSource1TemporaryRootNumberPowerBoundJoin44::MlSource1TemporaryRootNumberPowerBoundJoin44Next(value.clone(), exponent.clone())
        };
        match source_ml_source2_s33.clone() {
            self::MlSource1TemporaryRootNumberPowerBoundJoin44::MlSource1TemporaryRootNumberPowerBoundJoin44Next(value_3, exponent_3) => {
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn root_number_power_interval(
    interval: Vec<self::ml::Big>,
    scale: self::ml::Big,
) -> Vec<self::ml::Big> {
    {
        let lower = self::root_number_power_bound(
            (self::ml_array::optional_at(&interval, self::ml_array::number_index(0f64)))
                .clone()
                .unwrap_or_else(|| self::ml::Big::from_i128(0)),
            (self::ml_array::optional_at(&interval, self::ml_array::number_index(1f64)))
                .clone()
                .unwrap_or_else(|| self::ml::Big::from_i128(0)),
            scale.clone(),
            false,
        );
        {
            let upper = self::root_number_power_bound(
                (self::ml_array::optional_at(&interval, self::ml_array::number_index(2f64)))
                    .clone()
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&interval, self::ml_array::number_index(3f64)))
                    .clone()
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                scale.clone(),
                true,
            );
            vec![
                (self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64)))
                    .clone()
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64)))
                    .clone()
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64)))
                    .clone()
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64)))
                    .clone()
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
            ]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn invert_number_power_interval(
    interval: Vec<self::ml::Big>,
    scale: self::ml::Big,
) -> Vec<self::ml::Big> {
    {
        let lower = self::normalize_number_power_bound(
            self::divide_number_power_bound(
                scale.mul(&scale),
                (self::ml_array::optional_at(&interval, self::ml_array::number_index(2f64)))
                    .clone()
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                false,
            ),
            ((self::ml_array::optional_at(&interval, self::ml_array::number_index(3f64)))
                .clone()
                .unwrap_or_else(|| self::ml::Big::from_i128(0)))
            .neg(),
            scale.clone(),
            false,
        );
        {
            let upper = self::normalize_number_power_bound(
                self::divide_number_power_bound(
                    scale.mul(&scale),
                    (self::ml_array::optional_at(&interval, self::ml_array::number_index(0f64)))
                        .clone()
                        .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                    true,
                ),
                ((self::ml_array::optional_at(&interval, self::ml_array::number_index(1f64)))
                    .clone()
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)))
                .neg(),
                scale.clone(),
                true,
            );
            vec![
                (self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64)))
                    .clone()
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64)))
                    .clone()
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64)))
                    .clone()
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64)))
                    .clone()
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
            ]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn decompose_number_power_magnitude(value: f64) -> Vec<self::ml::Big> {
    {
        let exponent = self::ml::Big::from_i128(0);
        {
            let source_ml_source2_s34 =
                self::source_ml_source1_temporary_decompose_number_power_magnitude_loop45(
                    value,
                    exponent.clone(),
                );
            match source_ml_source2_s34.clone() {
                self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Done(value_2, exponent_2) => {
                    {
                        let source_ml_source2_s35 = self::source_ml_source1_temporary_decompose_number_power_magnitude_loop46(value_2, exponent_2.clone());
                        match source_ml_source2_s35.clone() {
                            self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Done(value_3, exponent_3) => {
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn round_number_power_bound(
    value: self::ml::Big,
    exponent: self::ml::Big,
    precision: self::ml::Big,
) -> f64 {
    if ((value == (self::ml::Big::from_i128(0))) || (exponent < (self::ml::Big::from_i128(-1076))))
    {
        0f64
    } else {
        if (exponent > (self::ml::Big::from_i128(1024))) {
            f64::INFINITY
        } else {
            {
                let shift = exponent.sub(&precision);
                if (shift >= (self::ml::Big::from_i128(0))) {
                    self::convert_string_number_rational(
                        value.mul(
                            &(self::ml_array::optional_at(
                                &self::power_java_script_big_integer(
                                    self::ml::Big::from_i128(2),
                                    shift.clone(),
                                ),
                                self::ml_array::number_index(0f64),
                            ))
                            .clone()
                            .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                        ),
                        self::ml::Big::from_i128(1),
                    )
                } else {
                    self::convert_string_number_rational(
                        value.clone(),
                        (self::ml_array::optional_at(
                            &self::power_java_script_big_integer(
                                self::ml::Big::from_i128(2),
                                shift.neg(),
                            ),
                            self::ml_array::number_index(0f64),
                        ))
                        .clone()
                        .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
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
                            let odd = (self::ml_math::is_integer(exponent)
                                && ((exponent.abs() % 2f64) == (1f64)));
                            {
                                let negative = (((base < (0f64))
                                    || ((base == (0f64)) && ((1f64 / base) < (0f64))))
                                    && odd);
                                if (base == (0f64)) {
                                    {
                                        let value = if (exponent > (0f64)) {
                                            0f64
                                        } else {
                                            f64::INFINITY
                                        };
                                        if negative { (-value) } else { value }
                                    }
                                } else {
                                    if !(base.is_finite()) {
                                        {
                                            let value_2 = if (exponent > (0f64)) {
                                                f64::INFINITY
                                            } else {
                                                0f64
                                            };
                                            if negative { (-value_2) } else { value_2 }
                                        }
                                    } else {
                                        if ((base < (0f64))
                                            && !(self::ml_math::is_integer(exponent)))
                                        {
                                            f64::NAN
                                        } else {
                                            if (magnitude == (1f64)) {
                                                if negative { (-1f64) } else { 1f64 }
                                            } else {
                                                {
                                                    let input =
                                                        self::decompose_number_power_magnitude(
                                                            magnitude,
                                                        );
                                                    {
                                                        let power =
                                                            self::decompose_number_power_magnitude(
                                                                exponent.abs(),
                                                            );
                                                        {
                                                            let precision =
                                                                self::ml::Big::from_i128(192);
                                                            self::source_ml_source1_temporary_power_java_script_number_loop47(exponent, negative, input, power, precision.clone())
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_exponentiation_base(prefix: &str, parenthesized: bool) -> bool {
    (parenthesized
        || !(self::ml_array::includes(
            &vec![
                String::from("+"),
                String::from("-"),
                String::from("!"),
                String::from("~"),
                String::from("typeof"),
                String::from("void"),
                String::from("delete"),
                String::from("await"),
            ],
            &prefix.to_owned(),
            0f64,
            false,
            |a, b| a == b,
        )))
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_exponent_message() -> String {
    String::from("Exponent must be positive")
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_power(target: &str, base: &str, exponent: &str) -> String {
    if (target == "JavaScript") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (String::from("ml_big_integer_conversion.power(")),
                        base.to_owned()
                    )),
                    (String::from(","))
                )),
                exponent.to_owned()
            )),
            (String::from(")"))
        )
    } else {
        if (target == "Rust") {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from(
                                "crate::ml_big_integer_conversion::power_java_script_big_integer("
                            )),
                            base.to_owned()
                        )),
                        (String::from(","))
                    )),
                    exponent.to_owned()
                )),
                (String::from(")"))
            )
        } else {
            if ((target == "Lean") || (target == "Rocq")) {
                format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (String::from("(MLBigIntegerConversion.power ")),
                                base.to_owned()
                            )),
                            (String::from(" "))
                        )),
                        exponent.to_owned()
                    )),
                    (String::from(")"))
                )
            } else {
                String::from("")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_nonzero_java_script_integer_literal(value: &str) -> bool {
    (((value != "") && (value != "0")) && (value != "-0"))
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_big_integer_division(
    language: &str,
    operation: &str,
    kind: &str,
    literal: &str,
) -> bool {
    ((((language == "JavaScript") && (kind == "int"))
        && ((operation == "div") || (operation == "rem")))
        && !(self::accept_nonzero_java_script_integer_literal(literal)))
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_division_message() -> String {
    String::from("Division by zero")
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_division(target: &str, left: &str, right: &str) -> String {
    if (target == "JavaScript") {
        format!(
            "{}{}",
            (format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (String::from("ml_big_integer_conversion.divide(")),
                        left.to_owned()
                    )),
                    (String::from(","))
                )),
                right.to_owned()
            )),
            (String::from(")"))
        )
    } else {
        if (target == "Rust") {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (String::from(
                                "crate::ml_big_integer_conversion::divide_java_script_big_integer("
                            )),
                            left.to_owned()
                        )),
                        (String::from(","))
                    )),
                    right.to_owned()
                )),
                (String::from(")"))
            )
        } else {
            if ((target == "Lean") || (target == "Rocq")) {
                format!(
                    "{}{}",
                    (format!(
                        "{}{}",
                        (format!(
                            "{}{}",
                            (format!(
                                "{}{}",
                                (String::from("(MLBigIntegerConversion.divide ")),
                                left.to_owned()
                            )),
                            (String::from(" "))
                        )),
                        right.to_owned()
                    )),
                    (String::from(")"))
                )
            } else {
                String::from("")
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_conversion_form(kind: &str) -> String {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_conversion_error_class(form: &str) -> String {
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

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_conversion_message(form: &str, value: &str) -> String {
    {
        let pieces = self::read_java_script_big_integer_conversion_message_pieces(&(form));
        if ((pieces.len() as f64) == (2f64)) {
            format!(
                "{}{}",
                (format!(
                    "{}{}",
                    ((self::ml_array::optional_at(&pieces, self::ml_array::number_index(0f64)))
                        .clone()
                        .unwrap_or_else(|| String::from(""))),
                    value.to_owned()
                )),
                ((self::ml_array::optional_at(&pieces, self::ml_array::number_index(1f64)))
                    .clone()
                    .unwrap_or_else(|| String::from("")))
            )
        } else {
            String::from("")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_conversion_message_pieces(form: &str) -> Vec<String> {
    if (form == "number") {
        vec![
            String::from("The number "),
            String::from(" cannot be converted to a BigInt because it is not an integer"),
        ]
    } else {
        if (((form == "string") || (form == "undefined")) || (form == "null")) {
            vec![
                String::from("Cannot convert "),
                String::from(" to a BigInt"),
            ]
        } else {
            Vec::<String>::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_builtin_operation(name: &str) -> String {
    if (name == "BigInt") {
        String::from("convertJavaScriptBigInteger")
    } else {
        String::from("")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_result_operation(operation: &str) -> String {
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

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_java_script_big_integer_result(kind: &str, operation: &str) -> bool {
    ((kind == "math") && ((self::read_java_script_big_integer_result_operation(operation)) != ""))
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_intrinsic_operand_kinds(operation: &str) -> Vec<String> {
    {
        let form = self::read_java_script_big_integer_result_operation(operation);
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

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_intrinsic_result_kind(operation: &str) -> String {
    {
        let form = self::read_java_script_big_integer_result_operation(operation);
        if (((form == "number-power") || (form == "number-compare")) || (form == "string-compare"))
        {
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
