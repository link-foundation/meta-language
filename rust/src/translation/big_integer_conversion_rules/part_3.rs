// Generated from complete js/src/translation/big-integer-conversion-rules.js. Do not edit by hand.
use super::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Result;
use super::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Result;
use super::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result;
use super::MlSource1TemporaryNormalizeNumberPowerBoundLoop40Result;
use super::MlSource1TemporaryNormalizeNumberPowerBoundLoop41Result;
use super::MlSource1TemporaryRootNumberPowerBoundJoin44;
use super::convert_java_script_number_big_integer;
use super::convert_string_number_rational;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::source_ml_source1_temporary_decompose_number_power_magnitude_loop45;
use super::source_ml_source1_temporary_decompose_number_power_magnitude_loop46;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop37;
use super::source_ml_source1_temporary_divide_java_script_big_integer_loop38;
use super::source_ml_source1_temporary_normalize_number_power_bound_loop40;
use super::source_ml_source1_temporary_normalize_number_power_bound_loop41;
use super::source_ml_source1_temporary_power_java_script_big_integer_loop39;
use super::source_ml_source1_temporary_power_java_script_number_loop47;
use super::source_ml_source1_temporary_root_number_power_integer_loop42;
use super::source_ml_source1_temporary_root_number_power_integer_loop43;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn divide_java_script_big_integer(
    left: self::ml::Big,
    right: self::ml::Big,
) -> Vec<self::ml::Big> {
    if right == (self::ml::Big::from_i128(0)) {
        Vec::<self::ml::Big>::new()
    } else {
        {
            let remaining = if left < (self::ml::Big::from_i128(0)) {
                left.neg()
            } else {
                left.clone()
            };
            {
                let divisor = if right < (self::ml::Big::from_i128(0)) {
                    right.neg()
                } else {
                    right.clone()
                };
                {
                    let shifted = divisor;
                    {
                        let weight = self::ml::Big::from_i128(1);
                        {
                            let quotient = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s29 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop37(remaining.clone(), shifted, weight);
                                match source_ml_source2_s29 {
                                    self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop37Done(shifted_2, weight_2) => {
                                        {
                                            let source_ml_source2_s30 = self::source_ml_source1_temporary_divide_java_script_big_integer_loop38(remaining.clone(), shifted_2.clone(), weight_2.clone(), quotient);
                                            match source_ml_source2_s30 {
                                                self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop38Done(remaining_2, quotient_2) => {
                                                    vec![if (left < (self::ml::Big::from_i128(0))) != (right < (self::ml::Big::from_i128(0))) {
                                                        quotient_2.neg()
                                                    } else {
                                                        quotient_2.clone()
                                                    }, if left < (self::ml::Big::from_i128(0)) {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn power_java_script_big_integer(
    base: self::ml::Big,
    exponent: self::ml::Big,
) -> Vec<self::ml::Big> {
    if exponent < (self::ml::Big::from_i128(0)) {
        Vec::<self::ml::Big>::new()
    } else {
        {
            let result = self::ml::Big::from_i128(1);
            {
                let weight = base.clone();
                {
                    let remaining = exponent.clone();
                    {
                        let result_2 =
                            self::source_ml_source1_temporary_power_java_script_big_integer_loop39(
                                result, weight, remaining,
                            );
                        vec![result_2]
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_number_power_bound(
    value: self::ml::Big,
    exponent: self::ml::Big,
    scale: self::ml::Big,
    upward: bool,
) -> Vec<self::ml::Big> {
    if value <= (self::ml::Big::from_i128(0)) {
        vec![self::ml::Big::from_i128(0), self::ml::Big::from_i128(0)]
    } else {
        {
            let source_ml_source2_s31 =
                self::source_ml_source1_temporary_normalize_number_power_bound_loop40(
                    value.clone(),
                    exponent.clone(),
                    scale.clone(),
                    upward,
                );
            match source_ml_source2_s31 {
                self::MlSource1TemporaryNormalizeNumberPowerBoundLoop40Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop40Done(value_2, exponent_2) => {
                    {
                        let source_ml_source2_s32 = self::source_ml_source1_temporary_normalize_number_power_bound_loop41(value_2.clone(), exponent_2.clone(), scale.clone());
                        match source_ml_source2_s32 {
                            self::MlSource1TemporaryNormalizeNumberPowerBoundLoop41Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop41Done(value_3, exponent_3) => {
                                vec![value_3.clone(), exponent_3.clone()]
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
pub fn divide_number_power_bound(
    numerator: self::ml::Big,
    denominator: self::ml::Big,
    upward: bool,
) -> self::ml::Big {
    {
        let result = self::divide_java_script_big_integer(numerator.clone(), denominator.clone());
        {
            let quotient =
                (self::ml_array::optional_at(&result, self::ml_array::number_index(0f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0));
            {
                let remainder =
                    (self::ml_array::optional_at(&result, self::ml_array::number_index(1f64)))
                        .unwrap_or_else(|| self::ml::Big::from_i128(0));
                quotient.add(&if upward && (remainder != (self::ml::Big::from_i128(0))) {
                    self::ml::Big::from_i128(1)
                } else {
                    self::ml::Big::from_i128(0)
                })
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn root_number_power_integer(value: self::ml::Big) -> self::ml::Big {
    if value <= (self::ml::Big::from_i128(0)) {
        self::ml::Big::from_i128(0)
    } else {
        {
            let root = self::ml::Big::from_i128(1);
            {
                let remaining = value.clone();
                {
                    let root_2 = self::source_ml_source1_temporary_root_number_power_integer_loop42(
                        root, remaining,
                    );
                    {
                        let root_3 = root_2.mul(&self::ml::Big::from_i128(2));
                        self::source_ml_source1_temporary_root_number_power_integer_loop43(
                            value.clone(),
                            root_3,
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn multiply_number_power_interval(
    left: Vec<self::ml::Big>,
    right: Vec<self::ml::Big>,
    scale: self::ml::Big,
) -> Vec<self::ml::Big> {
    {
        let lower = self::normalize_number_power_bound(
            self::divide_number_power_bound(
                ((self::ml_array::optional_at(&left, self::ml_array::number_index(0f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)))
                .mul(
                    &(self::ml_array::optional_at(&right, self::ml_array::number_index(0f64)))
                        .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                ),
                scale.clone(),
                false,
            ),
            ((self::ml_array::optional_at(&left, self::ml_array::number_index(1f64)))
                .unwrap_or_else(|| self::ml::Big::from_i128(0)))
            .add(
                &(self::ml_array::optional_at(&right, self::ml_array::number_index(1f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
            ),
            scale.clone(),
            false,
        );
        {
            let upper = self::normalize_number_power_bound(
                self::divide_number_power_bound(
                    ((self::ml_array::optional_at(&left, self::ml_array::number_index(2f64)))
                        .unwrap_or_else(|| self::ml::Big::from_i128(0)))
                    .mul(
                        &(self::ml_array::optional_at(&right, self::ml_array::number_index(2f64)))
                            .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                    ),
                    scale.clone(),
                    true,
                ),
                ((self::ml_array::optional_at(&left, self::ml_array::number_index(3f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)))
                .add(
                    &(self::ml_array::optional_at(&right, self::ml_array::number_index(3f64)))
                        .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                ),
                scale.clone(),
                true,
            );
            vec![
                (self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
            ]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn root_number_power_bound(
    value: self::ml::Big,
    exponent: self::ml::Big,
    scale: self::ml::Big,
    upward: bool,
) -> Vec<self::ml::Big> {
    {
        let source_ml_source2_s33 = {
            let ml_library_inequality_1 = (self::ml::divide(
                &exponent,
                &self::ml::Big::from_i128(2),
                "trunc",
                Some("Division by zero"),
                true,
            )) != (self::ml::Big::from_i128(0));
            if ml_library_inequality_1 {
                {
                    let value_2 = value.mul(&self::ml::Big::from_i128(2));
                    {
                        let exponent_2 = exponent.sub(&self::ml::Big::from_i128(1));
                        self::MlSource1TemporaryRootNumberPowerBoundJoin44::MlSource1TemporaryRootNumberPowerBoundJoin44Next(value_2, exponent_2)
                    }
                }
            } else {
                self::MlSource1TemporaryRootNumberPowerBoundJoin44::MlSource1TemporaryRootNumberPowerBoundJoin44Next(value.clone(), exponent.clone())
            }
        };
        match source_ml_source2_s33 {
            self::MlSource1TemporaryRootNumberPowerBoundJoin44::MlSource1TemporaryRootNumberPowerBoundJoin44Next(value_3, exponent_3) => {
                {
                    let square = value_3.mul(&scale);
                    {
                        let root = self::root_number_power_integer(square.clone());
                        {
                            let root_3 = if upward && ((root.mul(&root)) < square) {
                                {
                                    root.add(&self::ml::Big::from_i128(1))
                                }
                            } else {
                                root.clone()
                            };
                            self::normalize_number_power_bound(root_3, self::ml::divide(&exponent_3, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false), scale.clone(), upward)
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn root_number_power_interval(
    interval: Vec<self::ml::Big>,
    scale: self::ml::Big,
) -> Vec<self::ml::Big> {
    {
        let lower = self::root_number_power_bound(
            (self::ml_array::optional_at(&interval, self::ml_array::number_index(0f64)))
                .unwrap_or_else(|| self::ml::Big::from_i128(0)),
            (self::ml_array::optional_at(&interval, self::ml_array::number_index(1f64)))
                .unwrap_or_else(|| self::ml::Big::from_i128(0)),
            scale.clone(),
            false,
        );
        {
            let upper = self::root_number_power_bound(
                (self::ml_array::optional_at(&interval, self::ml_array::number_index(2f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&interval, self::ml_array::number_index(3f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                scale.clone(),
                true,
            );
            vec![
                (self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
            ]
        }
    }
}

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
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                false,
            ),
            ((self::ml_array::optional_at(&interval, self::ml_array::number_index(3f64)))
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
                        .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                    true,
                ),
                ((self::ml_array::optional_at(&interval, self::ml_array::number_index(1f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)))
                .neg(),
                scale.clone(),
                true,
            );
            vec![
                (self::ml_array::optional_at(&lower, self::ml_array::number_index(0f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&lower, self::ml_array::number_index(1f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&upper, self::ml_array::number_index(0f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                (self::ml_array::optional_at(&upper, self::ml_array::number_index(1f64)))
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
            ]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn decompose_number_power_magnitude(value: f64) -> Vec<self::ml::Big> {
    {
        let exponent = self::ml::Big::from_i128(0);
        {
            let source_ml_source2_s34 =
                self::source_ml_source1_temporary_decompose_number_power_magnitude_loop45(
                    value, exponent,
                );
            match source_ml_source2_s34 {
                self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop45Done(value_2, exponent_2) => {
                    {
                        let source_ml_source2_s35 = self::source_ml_source1_temporary_decompose_number_power_magnitude_loop46(value_2, exponent_2.clone());
                        match source_ml_source2_s35 {
                            self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop46Done(value_3, exponent_3) => {
                                vec![(self::ml_array::optional_at(&self::convert_java_script_number_big_integer(value_3 * 4_503_599_627_370_496f64), self::ml_array::number_index(0f64))).unwrap_or_else(|| self::ml::Big::from_i128(0)), exponent_3.sub(&self::ml::Big::from_i128(52))]
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
pub fn round_number_power_bound(
    value: self::ml::Big,
    exponent: self::ml::Big,
    precision: self::ml::Big,
) -> f64 {
    if (value == (self::ml::Big::from_i128(0))) || (exponent < (self::ml::Big::from_i128(-1076))) {
        0f64
    } else if exponent > (self::ml::Big::from_i128(1024)) {
        f64::INFINITY
    } else {
        {
            let shift = exponent.sub(&precision);
            if shift >= (self::ml::Big::from_i128(0)) {
                self::convert_string_number_rational(
                    value.mul(
                        &(self::ml_array::optional_at(
                            &self::power_java_script_big_integer(
                                self::ml::Big::from_i128(2),
                                shift.clone(),
                            ),
                            self::ml_array::number_index(0f64),
                        ))
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
                    .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn power_java_script_number(base: f64, exponent: f64) -> f64 {
    if exponent.is_nan() {
        f64::NAN
    } else if exponent == 0f64 {
        1f64
    } else if base.is_nan() {
        f64::NAN
    } else {
        {
            let magnitude = base.abs();
            if (exponent.is_finite()) {
                {
                    let odd =
                        self::ml_math::is_integer(exponent) && ((exponent.abs() % 2f64) == 1f64);
                    {
                        let negative =
                            ((base < 0f64) || ((base == 0f64) && ((1f64 / base) < 0f64))) && odd;
                        if base == 0f64 {
                            {
                                let value = if exponent > 0f64 { 0f64 } else { f64::INFINITY };
                                if negative { -value } else { value }
                            }
                        } else if (base.is_finite()) {
                            if (base < 0f64) && !(self::ml_math::is_integer(exponent)) {
                                f64::NAN
                            } else if magnitude == 1f64 {
                                if negative { -1f64 } else { 1f64 }
                            } else {
                                {
                                    let input = self::decompose_number_power_magnitude(magnitude);
                                    {
                                        let power =
                                            self::decompose_number_power_magnitude(exponent.abs());
                                        {
                                            let precision = self::ml::Big::from_i128(192);
                                            self::source_ml_source1_temporary_power_java_script_number_loop47(exponent, negative, input, power, precision)
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let value_2 = if exponent > 0f64 { f64::INFINITY } else { 0f64 };
                                if negative { -value_2 } else { value_2 }
                            }
                        }
                    }
                }
            } else if magnitude == 1f64 {
                f64::NAN
            } else if magnitude > 1f64 {
                if exponent > 0f64 { f64::INFINITY } else { 0f64 }
            } else if exponent > 0f64 {
                0f64
            } else {
                f64::INFINITY
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_exponentiation_base(prefix: &str, parenthesized: bool) -> bool {
    parenthesized
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
        ))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_exponent_message() -> String {
    String::from("Exponent must be positive")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_power(target: &str, base: &str, exponent: &str) -> String {
    if target == "JavaScript" {
        format!(
            "ml_big_integer_conversion.power({},{})",
            base.to_owned(),
            exponent.to_owned()
        )
    } else if target == "Rust" {
        format!(
            "crate::ml_big_integer_conversion::power_java_script_big_integer({},{})",
            base.to_owned(),
            exponent.to_owned()
        )
    } else if (target == "Lean") || (target == "Rocq") {
        format!(
            "(MLBigIntegerConversion.power {} {})",
            base.to_owned(),
            exponent.to_owned()
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_nonzero_java_script_integer_literal(value: &str) -> bool {
    ((!(value).is_empty()) && (value != "0")) && (value != "-0")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_big_integer_division(
    language: &str,
    operation: &str,
    kind: &str,
    literal: &str,
) -> bool {
    (((language == "JavaScript") && (kind == "int"))
        && ((operation == "div") || (operation == "rem")))
        && !(self::accept_nonzero_java_script_integer_literal(literal))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_division_message() -> String {
    String::from("Division by zero")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_division(target: &str, left: &str, right: &str) -> String {
    if target == "JavaScript" {
        format!(
            "ml_big_integer_conversion.divide({},{})",
            left.to_owned(),
            right.to_owned()
        )
    } else if target == "Rust" {
        format!(
            "crate::ml_big_integer_conversion::divide_java_script_big_integer({},{})",
            left.to_owned(),
            right.to_owned()
        )
    } else if (target == "Lean") || (target == "Rocq") {
        format!(
            "(MLBigIntegerConversion.divide {} {})",
            left.to_owned(),
            right.to_owned()
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_conversion_form(kind: &str) -> String {
    if (kind == "int") || (kind == "nat") {
        String::from("identity")
    } else if kind == "float" {
        String::from("number")
    } else if kind == "string" {
        String::from("string")
    } else if kind == "bool" {
        String::from("boolean")
    } else if kind == "unit" {
        String::from("undefined")
    } else if kind == "option" {
        String::from("optional")
    } else {
        String::from("unsupported")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_conversion_error_class(form: &str) -> String {
    if form == "string" {
        String::from("SyntaxError")
    } else if form == "number" {
        String::from("RangeError")
    } else if (form == "undefined") || (form == "null") {
        String::from("TypeError")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_conversion_message(form: &str, value: &str) -> String {
    {
        let pieces = self::read_java_script_big_integer_conversion_message_pieces(form);
        if (pieces.len() as f64) == 2f64 {
            format!(
                "{}{}{}",
                (self::ml_array::optional_at(&pieces, self::ml_array::number_index(0f64)))
                    .unwrap_or_else(|| String::new()),
                value.to_owned(),
                (self::ml_array::optional_at(&pieces, self::ml_array::number_index(1f64)))
                    .unwrap_or_else(|| String::new())
            )
        } else {
            String::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_conversion_message_pieces(form: &str) -> Vec<String> {
    if form == "number" {
        vec![
            String::from("The number "),
            String::from(" cannot be converted to a BigInt because it is not an integer"),
        ]
    } else if ((form == "string") || (form == "undefined")) || (form == "null") {
        vec![
            String::from("Cannot convert "),
            String::from(" to a BigInt"),
        ]
    } else {
        Vec::<String>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_builtin_operation(name: &str) -> String {
    if name == "BigInt" {
        String::from("convertJavaScriptBigInteger")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_result_operation(operation: &str) -> String {
    if operation == "compareJavaScriptNumberBigIntegerValue" {
        String::from("number-compare")
    } else if operation == "compareJavaScriptStringBigIntegerValue" {
        String::from("string-compare")
    } else if operation == "powerJavaScriptNumberValue" {
        String::from("number-power")
    } else if operation == "powerJavaScriptBigIntegerResult" {
        String::from("power")
    } else if operation == "divideJavaScriptBigIntegerResult" {
        String::from("division")
    } else if operation == "parseJavaScriptStringBigInteger" {
        String::from("string")
    } else if operation == "convertJavaScriptBooleanBigIntegerValue" {
        String::from("boolean")
    } else if operation == "convertJavaScriptNumberBigIntegerResult" {
        String::from("number")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_java_script_big_integer_result(kind: &str, operation: &str) -> bool {
    (kind == "math")
        && (!(self::read_java_script_big_integer_result_operation(operation)).is_empty())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_intrinsic_operand_kinds(operation: &str) -> Vec<String> {
    {
        let form = self::read_java_script_big_integer_result_operation(operation);
        if (form == "division") || (form == "power") {
            vec![String::from("int"), String::from("int")]
        } else if form == "number-power" {
            vec![String::from("float"), String::from("float")]
        } else if form == "number-compare" {
            vec![String::from("float"), String::from("int")]
        } else if form == "string-compare" {
            vec![String::from("string"), String::from("int")]
        } else if form == "string" {
            vec![String::from("string")]
        } else if form == "number" {
            vec![String::from("float")]
        } else if form == "boolean" {
            vec![String::from("bool")]
        } else {
            Vec::<String>::new()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_big_integer_intrinsic_result_kind(operation: &str) -> String {
    {
        let form = self::read_java_script_big_integer_result_operation(operation);
        if ((form == "number-power") || (form == "number-compare")) || (form == "string-compare") {
            String::from("float")
        } else if form == "boolean" {
            String::from("int")
        } else if (form).is_empty() {
            String::new()
        } else {
            String::from("array-integer")
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_big_integer_comparison(
    target: &str,
    form: &str,
    left: &str,
    right: &str,
    utf16: bool,
) -> String {
    if form == "number-compare" {
        if target == "JavaScript" {
            format!(
                "ml_big_integer_conversion.compareNumber({},{})",
                left.to_owned(),
                right.to_owned()
            )
        } else if target == "Rust" {
            format!(
                "crate::ml_big_integer_conversion::compare_java_script_number_big_integer({},{})",
                left.to_owned(),
                right.to_owned()
            )
        } else if (target == "Lean") || (target == "Rocq") {
            format!(
                "(MLBigIntegerConversion.compareNumber {} {})",
                left.to_owned(),
                right.to_owned()
            )
        } else {
            String::new()
        }
    } else if form == "string-compare" {
        if target == "JavaScript" {
            format!(
                "((ml_text,ml_integer)=>ml_big_integer_conversion.compareUnits(Array.from({{length:ml_text.length}},(_,index)=>ml_text.charCodeAt(index)),ml_integer))({},{})",
                left.to_owned(),
                right.to_owned()
            )
        } else if target == "Rust" {
            format!(
                "crate::ml_big_integer_conversion::compare_java_script_string_big_integer_units(&{}.map(f64::from).collect::<Vec<f64>>(),{})",
                if utf16 {
                    format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(")), left.to_owned())),
                        (String::from(").0.iter().copied()"))
                    )
                } else {
                    format!(
                        "{}{}",
                        (format!("{}{}", (String::from("(")), left.to_owned())),
                        (String::from(").encode_utf16()"))
                    )
                },
                right.to_owned()
            )
        } else if target == "Lean" {
            format!(
                "(MLBigIntegerConversion.compareUnits (String.foldl (fun (units : Array Float) c => if c.toNat < 65536 then units.push (Float.ofNat c.toNat) else (units.push (Float.ofNat (55296 + (c.toNat - 65536) / 1024))).push (Float.ofNat (56320 + (c.toNat - 65536) % 1024))) #[] {}) {})",
                left.to_owned(),
                right.to_owned()
            )
        } else if target == "Rocq" {
            format!(
                "(MLBigIntegerConversion.compareUtf8 ((fix ml_compare_bytes (text : string) : list float := match text with EmptyString => nil | String c rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii c)))) (ml_compare_bytes rest) end) {}) {})",
                left.to_owned(),
                right.to_owned()
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_java_script_number_power(target: &str, base: &str, exponent: &str) -> String {
    if target == "JavaScript" {
        format!(
            "ml_big_integer_conversion.powerNumber({},{})",
            base.to_owned(),
            exponent.to_owned()
        )
    } else if target == "Rust" {
        format!(
            "crate::ml_big_integer_conversion::power_java_script_number({},{})",
            base.to_owned(),
            exponent.to_owned()
        )
    } else if (target == "Lean") || (target == "Rocq") {
        format!(
            "(MLBigIntegerConversion.powerNumber {} {})",
            base.to_owned(),
            exponent.to_owned()
        )
    } else {
        String::new()
    }
}
