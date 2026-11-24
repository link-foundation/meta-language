// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop166Result;
use super::accept_java_script_whitespace_unit;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::ml_utf16_text;
use super::read_java_script_numeric_digit_unit;
use super::read_string_number_digit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop166(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut exponent: f64,
    mut mantissa_end: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop166Result {
    loop {
        return if (mantissa_end > start) {
            {
                let unit = ({
                    let source_ml_source2_array_read_index39999 = (mantissa_end - 1f64);
                    {
                        let ml_reference_array2669 = units.clone();
                        let ml_reference_position2669 = source_ml_source2_array_read_index39999;
                        let ml_reference_projection2669 = self::ml_array::optional_at(
                            &*ml_reference_array2669.borrow(),
                            self::ml_array::number_index(ml_reference_position2669),
                        );
                        ml_reference_projection2669
                    }
                })
                .clone()
                .unwrap_or_else(|| (-1f64));
                if (unit == (46f64)) {
                    {
                        let mantissa_end_2 = (mantissa_end - 1f64);
                        {
                            mantissa_end = mantissa_end_2;
                            continue;
                        }
                    }
                } else {
                    if (unit != (48f64)) {
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop166Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop166Done(exponent, mantissa_end)
                    } else {
                        {
                            let mantissa_end_3 = (mantissa_end - 1f64);
                            {
                                let exponent_2 = (exponent + 1f64);
                                {
                                    (exponent, mantissa_end) = (exponent_2, mantissa_end_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop166Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop166Done(exponent, mantissa_end)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165Next(f64, f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin165Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop169(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut end: f64,
    mut at: f64,
    mut exponent: f64,
    mut exponent_limit: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Result {
    loop {
        return if (at < end) {
            {
                let digit = (({
                    let ml_reference_array2670 = units.clone();
                    let ml_reference_position2670 = at;
                    let ml_reference_projection2670 = self::ml_array::optional_at(
                        &*ml_reference_array2670.borrow(),
                        self::ml_array::number_index(ml_reference_position2670),
                    );
                    ml_reference_projection2670
                })
                .clone()
                .unwrap_or_else(|| (-1f64))
                    - 48f64);
                if ((digit < (0f64)) || (digit > (9f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Done(at, exponent)
                } else {
                    {
                        let exponent_3 = if (exponent < exponent_limit) {
                            {
                                let exponent_2 = self::ml_math::min(
                                    exponent_limit,
                                    ((exponent * 10f64) + digit),
                                );
                                exponent_2
                            }
                        } else {
                            exponent
                        };
                        {
                            let at_2 = (at + 1f64);
                            {
                                (at, exponent) = (at_2, exponent_3);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Done(at, exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin168 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin168Next(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop164Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop164Done(f64, f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop164(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut end: f64,
    mut at: f64,
    mut fraction: f64,
    mut digits: f64,
    mut significant: f64,
    mut dot: bool,
) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop164Result {
    loop {
        return if (at < end) {
            {
                let unit = ({
                    let ml_reference_array2671 = units.clone();
                    let ml_reference_position2671 = at;
                    let ml_reference_projection2671 = self::ml_array::optional_at(
                        &*ml_reference_array2671.borrow(),
                        self::ml_array::number_index(ml_reference_position2671),
                    );
                    ml_reference_projection2671
                })
                .clone()
                .unwrap_or_else(|| (-1f64));
                if ((unit >= (48f64)) && (unit <= (57f64))) {
                    {
                        let digits_2 = (digits + 1f64);
                        {
                            let fraction_3 = if dot {
                                {
                                    let fraction_2 = (fraction + 1f64);
                                    fraction_2
                                }
                            } else {
                                fraction
                            };
                            {
                                let significant_3 = if ((unit != (48f64)) || (significant > (0f64)))
                                {
                                    {
                                        let significant_2 = (significant + 1f64);
                                        significant_2
                                    }
                                } else {
                                    significant
                                };
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        (at, fraction, digits, significant) =
                                            (at_2, fraction_3, digits_2, significant_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if ((unit == (46f64)) && !dot) {
                        {
                            let at_3 = (at + 1f64);
                            {
                                (at, dot) = (at_3, true);
                                continue;
                            }
                        }
                    } else {
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop164Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop164Done(at, fraction, digits, significant)
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop164Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop164Done(at, fraction, digits, significant)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163Next(f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin163Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop170Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop170Done(self::ml::Big, bool),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop170Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop170(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut end: f64,
    mut integer_digits: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut radix: f64,
    mut value: self::ml::Big,
    mut overflow: bool,
    mut limit: self::ml::Big,
    mut base: self::ml::Big,
    mut index: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop170Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_string_number_digit(
                    ({
                        let ml_reference_array2672 = units.clone();
                        let ml_reference_position2672 = index;
                        let ml_reference_projection2672 = self::ml_array::optional_at(
                            &*ml_reference_array2672.borrow(),
                            self::ml_array::number_index(ml_reference_position2672),
                        );
                        ml_reference_projection2672
                    })
                    .clone()
                    .unwrap_or_else(|| (-1f64)),
                );
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop170Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop170Return(f64::NAN)
                } else {
                    if !overflow {
                        {
                            let value_2 = (value.mul(&base)).add(
                                &({
                                    let ml_reference_array2673 = integer_digits.clone();
                                    let ml_reference_position2673 = digit;
                                    let ml_reference_projection2673 = self::ml_array::optional_at(
                                        &*ml_reference_array2673.borrow(),
                                        self::ml_array::number_index(ml_reference_position2673),
                                    );
                                    ml_reference_projection2673
                                })
                                .clone()
                                .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                            );
                            {
                                let overflow_2 = (value_2 > limit);
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        (value, overflow, index) =
                                            (value_2.clone(), overflow_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let index_3 = (index + 1f64);
                            {
                                index = index_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop170Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop170Done(value.clone(), overflow)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop159Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop159Done,
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop159Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop159(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop159Result {
    loop {
        return if (index
            < ({
                let ml_reference_array2674 = units.clone();
                let ml_reference_projection2674 = ml_reference_array2674.borrow().len() as f64;
                ml_reference_projection2674
            })) {
            {
                let unit = ({
                    let ml_reference_array2675 = units.clone();
                    let ml_reference_position2675 = index;
                    let ml_reference_projection2675 = self::ml_array::optional_at(
                        &*ml_reference_array2675.borrow(),
                        self::ml_array::number_index(ml_reference_position2675),
                    );
                    ml_reference_projection2675
                })
                .clone()
                .unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64)))
                    || (unit > (65535f64)))
                {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop159Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop159Return(f64::NAN)
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop159Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop159Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop172(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        return if ((start < end)
            && self::accept_java_script_whitespace_unit(
                ({
                    let ml_reference_array2676 = units.clone();
                    let ml_reference_position2676 = start;
                    let ml_reference_projection2676 = self::ml_array::optional_at(
                        &*ml_reference_array2676.borrow(),
                        self::ml_array::number_index(ml_reference_position2676),
                    );
                    ml_reference_projection2676
                })
                .clone()
                .unwrap_or_else(|| (-1f64)),
            )) {
            {
                let start_2 = (start + 1f64);
                {
                    start = start_2;
                    continue;
                }
            }
        } else {
            start
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop173(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        return if ((end > start)
            && self::accept_java_script_whitespace_unit(
                ({
                    let source_ml_source2_array_read_index40013 = (end - 1f64);
                    {
                        let ml_reference_array2677 = units.clone();
                        let ml_reference_position2677 = source_ml_source2_array_read_index40013;
                        let ml_reference_projection2677 = self::ml_array::optional_at(
                            &*ml_reference_array2677.borrow(),
                            self::ml_array::number_index(ml_reference_position2677),
                        );
                        ml_reference_projection2677
                    }
                })
                .clone()
                .unwrap_or_else(|| (-1f64)),
            )) {
            {
                let end_2 = (end - 1f64);
                {
                    end = end_2;
                    continue;
                }
            }
        } else {
            end
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop176Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop176Done(self::ml::Big),
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop176Return(
        std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop176(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut end: f64,
    mut radix: f64,
    mut base: self::ml::Big,
    mut digits: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut value: self::ml::Big,
    mut index: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop176Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_java_script_numeric_digit_unit(
                    ({
                        let ml_reference_array2678 = units.clone();
                        let ml_reference_position2678 = index;
                        let ml_reference_projection2678 = self::ml_array::optional_at(
                            &*ml_reference_array2678.borrow(),
                            self::ml_array::number_index(ml_reference_position2678),
                        );
                        ml_reference_projection2678
                    })
                    .clone()
                    .unwrap_or_else(|| (-1f64)),
                );
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop176Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop176Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new())))
                } else {
                    {
                        let value_2 = (value.mul(&base)).add(
                            &({
                                let ml_reference_array2679 = digits.clone();
                                let ml_reference_position2679 = digit;
                                let ml_reference_projection2679 = self::ml_array::optional_at(
                                    &*ml_reference_array2679.borrow(),
                                    self::ml_array::number_index(ml_reference_position2679),
                                );
                                ml_reference_projection2679
                            })
                            .clone()
                            .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                        );
                        {
                            let index_2 = (index + 1f64);
                            {
                                (value, index) = (value_2.clone(), index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop176Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop176Done(value.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin175 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin175Next(f64, f64, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin174Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop171Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop171Done,
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop171Return(
        std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop171(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop171Result {
    loop {
        return if (index
            < ({
                let ml_reference_array2680 = units.clone();
                let ml_reference_projection2680 = ml_reference_array2680.borrow().len() as f64;
                ml_reference_projection2680
            })) {
            {
                let unit = ({
                    let ml_reference_array2681 = units.clone();
                    let ml_reference_position2681 = index;
                    let ml_reference_projection2681 = self::ml_array::optional_at(
                        &*ml_reference_array2681.borrow(),
                        self::ml_array::number_index(ml_reference_position2681),
                    );
                    ml_reference_projection2681
                })
                .clone()
                .unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64)))
                    || (unit > (65535f64)))
                {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop171Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop171Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new())))
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop171Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop171Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_number_big_integer_loop177(
    mut value: f64,
    mut integer: self::ml::Big,
    mut weight: self::ml::Big,
) -> self::ml::Big {
    loop {
        return if (value >= (1f64)) {
            {
                let integer_3 = if ((value % 2f64) == (1f64)) {
                    {
                        let integer_2 = integer.add(&weight);
                        integer_2.clone()
                    }
                } else {
                    integer.clone()
                };
                {
                    let value_2 = (value / 2f64).floor();
                    {
                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                        {
                            (value, integer, weight) =
                                (value_2, integer_3.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            integer.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop179Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop179Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop179(
    mut remaining: self::ml::Big,
    mut shifted: self::ml::Big,
    mut weight: self::ml::Big,
    mut quotient: self::ml::Big,
) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop179Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
            {
                let shifted_2 = self::ml::divide(
                    &shifted,
                    &self::ml::Big::from_i128(2),
                    "trunc",
                    Some("Division by zero"),
                    false,
                );
                {
                    let weight_2 = self::ml::divide(
                        &weight,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    );
                    if (shifted_2 <= remaining) {
                        {
                            let remaining_2 = remaining.sub(&shifted_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (remaining, shifted, weight, quotient) = (
                                        remaining_2.clone(),
                                        shifted_2.clone(),
                                        weight_2.clone(),
                                        quotient_2.clone(),
                                    );
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
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop179Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop179Done(remaining.clone(), quotient.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop178Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop178Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop178(
    mut remaining: self::ml::Big,
    mut shifted: self::ml::Big,
    mut weight: self::ml::Big,
) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop178Result {
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
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop178Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop178Done(shifted.clone(), weight.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_power_java_script_big_integer_loop180(
    mut result: self::ml::Big,
    mut weight: self::ml::Big,
    mut remaining: self::ml::Big,
) -> self::ml::Big {
    loop {
        return if (remaining > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(
                    &remaining,
                    &self::ml::Big::from_i128(2),
                    "trunc",
                    Some("Division by zero"),
                    true,
                )) == (self::ml::Big::from_i128(1)))
                {
                    {
                        let result_2 = result.mul(&weight);
                        result_2.clone()
                    }
                } else {
                    result.clone()
                };
                {
                    let remaining_2 = self::ml::divide(
                        &remaining,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    );
                    if (remaining_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let weight_2 = weight.mul(&weight);
                            {
                                (result, weight, remaining) =
                                    (result_3.clone(), weight_2.clone(), remaining_2.clone());
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDependencyFunction1Loop192Result {
    MlSource1TemporaryDependencyFunction1Loop192Done(f64),
    MlSource1TemporaryDependencyFunction1Loop192Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_dependency_function1_loop192(
    mut rest: self::ml::Big,
    mut width: f64,
) -> self::MlSource1TemporaryDependencyFunction1Loop192Result {
    loop {
        return if (rest > (self::ml::Big::from_i128(0))) {
            if (width >= (4096f64)) {
                self::MlSource1TemporaryDependencyFunction1Loop192Result::MlSource1TemporaryDependencyFunction1Loop192Return((-1f64))
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
                        let width_2 = (width + 1f64);
                        {
                            (rest, width) = (rest_2.clone(), width_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDependencyFunction1Loop192Result::MlSource1TemporaryDependencyFunction1Loop192Done(width)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_integer_text_loop193(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut text: self::ml_string::Text,
    mut index: f64,
) -> self::ml_string::Text {
    loop {
        return if (index
            < ({
                let ml_reference_array2682 = units.clone();
                let ml_reference_projection2682 = ml_reference_array2682.borrow().len() as f64;
                ml_reference_projection2682
            })) {
            {
                let text_2 = text.concat(&{
                    let source_ml_source2_character_argument40043 = ({
                        let ml_reference_array2683 = units.clone();
                        let ml_reference_position2683 = index;
                        let ml_reference_projection2683 = self::ml_array::optional_at(
                            &*ml_reference_array2683.borrow(),
                            self::ml_array::number_index(ml_reference_position2683),
                        );
                        ml_reference_projection2683
                    })
                    .clone()
                    .unwrap_or_else(|| 0f64);
                    self::ml_string::Text::from_units(
                        self::ml_utf16_text::character_units_utf16_text(&vec![
                            source_ml_source2_character_argument40043,
                        ])
                        .into_iter()
                        .map(|ml_unit| ml_unit as u16)
                        .collect(),
                    )
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
pub enum MlSource1TemporaryDependencyFunction2Loop195Result {
    MlSource1TemporaryDependencyFunction2Loop195Done,
}
