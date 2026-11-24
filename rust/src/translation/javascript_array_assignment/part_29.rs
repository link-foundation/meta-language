// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::accept_java_script_whitespace_unit;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_string_number_digit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_exact_mantissa_loop146(
    mut value: self::ml::Big,
    mut result: f64,
    mut scale: f64,
) -> f64 {
    loop {
        if value > (self::ml::Big::from_i128(0)) {
            {
                let result_3 = if (self::ml::divide(
                    &value,
                    &self::ml::Big::from_i128(2),
                    "trunc",
                    Some("Division by zero"),
                    true,
                )) == (self::ml::Big::from_i128(1))
                {
                    { result + scale }
                } else {
                    result
                };
                {
                    let value_2 = self::ml::divide(
                        &value,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    );
                    {
                        let scale_2 = scale * 2f64;
                        {
                            (value, result, scale) = (value_2, result_3, scale_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop148(
    mut exponent: f64,
    mut result: f64,
) -> f64 {
    loop {
        if exponent < 0f64 {
            {
                let result_2 = result / 2f64;
                {
                    let exponent_2 = exponent + 1f64;
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadStringNumberBinaryPowerLoop147Result {
    MlSource1TemporaryReadStringNumberBinaryPowerLoop147Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop147(
    mut exponent: f64,
    mut result: f64,
) -> self::MlSource1TemporaryReadStringNumberBinaryPowerLoop147Result {
    loop {
        if exponent > 0f64 {
            {
                let result_2 = result * 2f64;
                {
                    let exponent_2 = exponent - 1f64;
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryReadStringNumberBinaryPowerLoop147Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop147Done(exponent, result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop152Result {
    MlSource1TemporaryConvertStringNumberRationalLoop152Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop152(
    mut quotient: self::ml::Big,
    mut remainder: self::ml::Big,
    mut scale: self::ml::Big,
    mut weight: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop152Result {
    loop {
        if weight > (self::ml::Big::from_i128(1)) {
            {
                let scale_2 = self::ml::divide(
                    &scale,
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
                    if remainder >= scale_2 {
                        {
                            let remainder_2 = remainder.sub(&scale_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (quotient, remainder, scale, weight) = (
                                        quotient_2,
                                        remainder_2,
                                        scale_2.clone(),
                                        weight_2.clone(),
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            (scale, weight) = (scale_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryConvertStringNumberRationalLoop152Result::MlSource1TemporaryConvertStringNumberRationalLoop152Done(quotient.clone(), remainder.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop151Result {
    MlSource1TemporaryConvertStringNumberRationalLoop151Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop151(
    mut remainder: self::ml::Big,
    mut scale: self::ml::Big,
    mut weight: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop151Result {
    loop {
        if scale <= remainder {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                    {
                        (scale, weight) = (scale_2, weight_2);
                        continue;
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryConvertStringNumberRationalLoop151Result::MlSource1TemporaryConvertStringNumberRationalLoop151Done(scale.clone(), weight.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin150 {
    MlSource1TemporaryConvertStringNumberRationalJoin150Next(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop153(
    mut numerator: self::ml::Big,
    mut shift: f64,
    mut index: f64,
) -> self::ml::Big {
    loop {
        if index < shift {
            {
                let numerator_2 = numerator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = index + 1f64;
                    {
                        (numerator, index) = (numerator_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { numerator.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop154(
    mut denominator: self::ml::Big,
    mut shift: f64,
    mut index: f64,
) -> self::ml::Big {
    loop {
        if index < (-shift) {
            {
                let denominator_2 = denominator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = index + 1f64;
                    {
                        (denominator, index) = (denominator_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { denominator.clone() };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin149 {
    MlSource1TemporaryConvertStringNumberRationalJoin149Next(f64),
    MlSource1TemporaryConvertStringNumberRationalJoin149Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop155Result {
    MlSource1TemporaryConvertStringNumberRationalLoop155Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop155Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop155(
    mut numerator: self::ml::Big,
    mut exponent: f64,
    mut scale: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop155Result {
    loop {
        return if numerator >= (scale.mul(&self::ml::Big::from_i128(2))) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent + 1f64;
                    if exponent_2 >= 1024f64 {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop155Result::MlSource1TemporaryConvertStringNumberRationalLoop155Return(f64::INFINITY)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop155Result::MlSource1TemporaryConvertStringNumberRationalLoop155Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop156Result {
    MlSource1TemporaryConvertStringNumberRationalLoop156Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop156Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop156(
    mut denominator: self::ml::Big,
    mut exponent: f64,
    mut scale: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop156Result {
    loop {
        return if scale < denominator {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent - 1f64;
                    if exponent_2 < -1075f64 {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop156Result::MlSource1TemporaryConvertStringNumberRationalLoop156Return(0f64)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop156Result::MlSource1TemporaryConvertStringNumberRationalLoop156Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptStringNumberInfinityLoop157Result {
    MlSource1TemporaryAcceptStringNumberInfinityLoop157Done,
    MlSource1TemporaryAcceptStringNumberInfinityLoop157Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_string_number_infinity_loop157(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut expected: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryAcceptStringNumberInfinityLoop157Result {
    loop {
        return if index
            < ({
                let ml_reference_array335 = expected.clone();
                ml_reference_array335.borrow().len() as f64
            }) {
            if (({
                let source_ml_source2_array_read_index20437 = start + index;
                {
                    let ml_reference_array336 = units.clone();
                    let ml_reference_position336 = source_ml_source2_array_read_index20437;
                    self::ml_array::optional_at(
                        &*ml_reference_array336.borrow(),
                        self::ml_array::number_index(ml_reference_position336),
                    )
                }
            })
            .clone()
            .unwrap_or_else(|| -1f64))
                != (({
                    let ml_reference_array337 = expected.clone();
                    let ml_reference_position337 = index;
                    self::ml_array::optional_at(
                        &*ml_reference_array337.borrow(),
                        self::ml_array::number_index(ml_reference_position337),
                    )
                })
                .clone()
                .unwrap_or_else(|| -2f64))
            {
                self::MlSource1TemporaryAcceptStringNumberInfinityLoop157Result::MlSource1TemporaryAcceptStringNumberInfinityLoop157Return(false)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptStringNumberInfinityLoop157Result::MlSource1TemporaryAcceptStringNumberInfinityLoop157Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop159(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        if (start < end)
            && self::accept_java_script_whitespace_unit(
                ({
                    let ml_reference_array338 = units.clone();
                    let ml_reference_position338 = start;
                    self::ml_array::optional_at(
                        &*ml_reference_array338.borrow(),
                        self::ml_array::number_index(ml_reference_position338),
                    )
                })
                .clone()
                .unwrap_or_else(|| -1f64),
            )
        {
            {
                let start_2 = start + 1f64;
                {
                    start = start_2;
                    continue;
                }
            }
        }
        return { start };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop160(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        if (end > start)
            && self::accept_java_script_whitespace_unit(
                ({
                    let source_ml_source2_array_read_index20443 = end - 1f64;
                    {
                        let ml_reference_array339 = units.clone();
                        let ml_reference_position339 = source_ml_source2_array_read_index20443;
                        self::ml_array::optional_at(
                            &*ml_reference_array339.borrow(),
                            self::ml_array::number_index(ml_reference_position339),
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop166(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut integer_digits: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut mantissa_end: f64,
    mut mantissa: self::ml::Big,
    mut index: f64,
) -> self::ml::Big {
    loop {
        if index < mantissa_end {
            {
                let unit = ({
                    let ml_reference_array340 = units.clone();
                    let ml_reference_position340 = index;
                    self::ml_array::optional_at(
                        &*ml_reference_array340.borrow(),
                        self::ml_array::number_index(ml_reference_position340),
                    )
                })
                .clone()
                .unwrap_or_else(|| -1f64);
                if unit != 46f64 {
                    {
                        let mantissa_2 = (mantissa.mul(&self::ml::Big::from_i128(10))).add(
                            &({
                                let source_ml_source2_array_read_index20447 = unit - 48f64;
                                {
                                    let ml_reference_array341 = integer_digits.clone();
                                    let ml_reference_position341 =
                                        source_ml_source2_array_read_index20447;
                                    self::ml_array::optional_at(
                                        &*ml_reference_array341.borrow(),
                                        self::ml_array::number_index(ml_reference_position341),
                                    )
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                        );
                        {
                            let index_2 = index + 1f64;
                            {
                                (mantissa, index) = (mantissa_2, index_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let index_3 = index + 1f64;
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { mantissa.clone() };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop165Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop165Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop165(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut exponent: f64,
    mut mantissa_end: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop165Result {
    loop {
        return if mantissa_end > start {
            {
                let unit = ({
                    let source_ml_source2_array_read_index20449 = mantissa_end - 1f64;
                    {
                        let ml_reference_array342 = units.clone();
                        let ml_reference_position342 = source_ml_source2_array_read_index20449;
                        self::ml_array::optional_at(
                            &*ml_reference_array342.borrow(),
                            self::ml_array::number_index(ml_reference_position342),
                        )
                    }
                })
                .clone()
                .unwrap_or_else(|| -1f64);
                if unit == 46f64 {
                    {
                        let mantissa_end_2 = mantissa_end - 1f64;
                        {
                            mantissa_end = mantissa_end_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let ml_library_inequality_1 = unit != 48f64;
                        if ml_library_inequality_1 {
                            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop165Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop165Done(exponent, mantissa_end)
                        } else {
                            {
                                let mantissa_end_3 = mantissa_end - 1f64;
                                {
                                    let exponent_2 = exponent + 1f64;
                                    {
                                        (exponent, mantissa_end) = (exponent_2, mantissa_end_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop165Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop165Done(exponent, mantissa_end)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164Next(f64, f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin164Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop168Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop168Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop168(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut end: f64,
    mut at: f64,
    mut exponent: f64,
    mut exponent_limit: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop168Result {
    loop {
        return if at < end {
            {
                let digit = ({
                    let ml_reference_array343 = units.clone();
                    let ml_reference_position343 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array343.borrow(),
                        self::ml_array::number_index(ml_reference_position343),
                    )
                })
                .clone()
                .unwrap_or_else(|| -1f64)
                    - 48f64;
                if (digit < 0f64) || (digit > 9f64) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop168Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop168Done(at, exponent)
                } else {
                    {
                        let exponent_3 = if exponent < exponent_limit {
                            { self::ml_math::min(exponent_limit, ((exponent * 10f64) + digit)) }
                        } else {
                            exponent
                        };
                        {
                            let at_2 = at + 1f64;
                            {
                                (at, exponent) = (at_2, exponent_3);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop168Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop168Done(at, exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin167 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin167Next(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop163Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop163Done(f64, f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop163(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut end: f64,
    mut at: f64,
    mut fraction: f64,
    mut digits: f64,
    mut significant: f64,
    mut dot: bool,
) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop163Result {
    loop {
        return if at < end {
            {
                let unit = ({
                    let ml_reference_array344 = units.clone();
                    let ml_reference_position344 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array344.borrow(),
                        self::ml_array::number_index(ml_reference_position344),
                    )
                })
                .clone()
                .unwrap_or_else(|| -1f64);
                if (48f64..=57f64).contains(&unit) {
                    {
                        let digits_2 = digits + 1f64;
                        {
                            let fraction_3 = if dot { { fraction + 1f64 } } else { fraction };
                            {
                                let significant_3 = if (unit != 48f64) || (significant > 0f64) {
                                    { significant + 1f64 }
                                } else {
                                    significant
                                };
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (at, fraction, digits, significant) =
                                            (at_2, fraction_3, digits_2, significant_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
                {
                    if (unit == 46f64) && !dot {
                        {
                            let at_3 = at + 1f64;
                            {
                                (at, dot) = (at_3, true);
                                continue;
                            }
                        }
                    }
                    {
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop163Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop163Done(at, fraction, digits, significant)
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop163Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop163Done(at, fraction, digits, significant)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162Next(f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin162Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Done(self::ml::Big, bool),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop169(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut end: f64,
    mut integer_digits: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut radix: f64,
    mut value: self::ml::Big,
    mut overflow: bool,
    mut limit: self::ml::Big,
    mut base: self::ml::Big,
    mut index: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Result {
    loop {
        return if index < end {
            {
                let digit = self::read_string_number_digit(
                    ({
                        let ml_reference_array345 = units.clone();
                        let ml_reference_position345 = index;
                        self::ml_array::optional_at(
                            &*ml_reference_array345.borrow(),
                            self::ml_array::number_index(ml_reference_position345),
                        )
                    })
                    .clone()
                    .unwrap_or_else(|| -1f64),
                );
                if (digit < 0f64) || (digit >= radix) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Return(f64::NAN)
                } else {
                    if !overflow {
                        {
                            let value_2 = (value.mul(&base)).add(
                                &({
                                    let ml_reference_array346 = integer_digits.clone();
                                    let ml_reference_position346 = digit;
                                    self::ml_array::optional_at(
                                        &*ml_reference_array346.borrow(),
                                        self::ml_array::number_index(ml_reference_position346),
                                    )
                                })
                                .clone()
                                .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                            );
                            {
                                let overflow_2 = value_2 > limit;
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        (value, overflow, index) =
                                            (value_2.clone(), overflow_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    {
                        {
                            let index_3 = index + 1f64;
                            {
                                index = index_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop169Done(value.clone(), overflow)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin161 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin161Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop158Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop158Done,
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop158Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop158(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop158Result {
    loop {
        return if index
            < ({
                let ml_reference_array347 = units.clone();
                ml_reference_array347.borrow().len() as f64
            }) {
            {
                let unit = ({
                    let ml_reference_array348 = units.clone();
                    let ml_reference_position348 = index;
                    self::ml_array::optional_at(
                        &*ml_reference_array348.borrow(),
                        self::ml_array::number_index(ml_reference_position348),
                    )
                })
                .clone()
                .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_safe_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64)
                {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop158Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop158Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop158Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop158Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop171(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        if (start < end)
            && self::accept_java_script_whitespace_unit(
                ({
                    let ml_reference_array349 = units.clone();
                    let ml_reference_position349 = start;
                    self::ml_array::optional_at(
                        &*ml_reference_array349.borrow(),
                        self::ml_array::number_index(ml_reference_position349),
                    )
                })
                .clone()
                .unwrap_or_else(|| -1f64),
            )
        {
            {
                let start_2 = start + 1f64;
                {
                    start = start_2;
                    continue;
                }
            }
        }
        return { start };
    }
}
