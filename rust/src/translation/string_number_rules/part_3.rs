// Generated from complete js/src/translation/string-number-rules.js. Do not edit by hand.
use super::accept_java_script_whitespace_unit;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_string_number_digit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop15(units: &[f64], mut start: f64, mut end: f64) -> f64 {
    loop {
        if (start < end) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(units, self::ml_array::number_index(start))).unwrap_or_else(|| -1f64)) {
            {
                let start_2 = start + 1f64;
                {
                    start = start_2;
                    continue;
                }
            }
        }
return {
            start
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop16(units: &[f64], mut start: f64, mut end: f64) -> f64 {
    loop {
        if (end > start) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(units, self::ml_array::number_index(end - 1f64))).unwrap_or_else(|| -1f64)) {
            {
                let end_2 = end - 1f64;
                {
                    end = end_2;
                    continue;
                }
            }
        }
return {
            end
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop22(units: &[f64], mut integer_digits: Vec<self::ml::Big>, mut mantissa_end: f64, mut mantissa: self::ml::Big, mut index: f64) -> self::ml::Big {
    loop {
        if index < mantissa_end {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(index))).unwrap_or_else(|| -1f64);
                if unit != 46f64 {
                    {
                        let mantissa_2 = (mantissa.mul(&self::ml::Big::from_i128(10))).add(&(self::ml_array::optional_at(&integer_digits, self::ml_array::number_index(unit - 48f64))).unwrap_or_else(|| self::ml::Big::from_i128(0)));
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
return {
            mantissa.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop21(units: &[f64], mut start: f64, mut exponent: f64, mut mantissa_end: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result {
    loop {
        return if mantissa_end > start {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(mantissa_end - 1f64))).unwrap_or_else(|| -1f64);
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
let ml_library_inequality_1 = unit != 48f64 ;
if ml_library_inequality_1 {
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done(exponent, mantissa_end)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop21Done(exponent, mantissa_end)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Next(f64, f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin20Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop24(units: &[f64], mut end: f64, mut at: f64, mut exponent: f64, mut exponent_limit: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Result {
    loop {
        return if at < end {
            {
                let digit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).unwrap_or_else(|| -1f64) - 48f64;
                if (digit < 0f64) || (digit > 9f64) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Done(at, exponent)
                } else {
                    {
                        let exponent_3 = if exponent < exponent_limit {
                            {
                                self::ml_math::min(exponent_limit, ((exponent * 10f64) + digit))
                            }
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop24Done(at, exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin23Next(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Done(f64, f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop19(units: &[f64], mut end: f64, mut at: f64, mut fraction: f64, mut digits: f64, mut significant: f64, mut dot: bool) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Result {
    loop {
        return if at < end {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).unwrap_or_else(|| -1f64);
                if (48f64..=57f64).contains(&unit) {
                    {
                        let digits_2 = digits + 1f64;
                        {
                            let fraction_3 = if dot {
                                {
                                    fraction + 1f64
                                }
                            } else {
                                fraction
                            };
                            {
                                let significant_3 = if (unit != 48f64) || (significant > 0f64) {
                                    {
                                        significant + 1f64
                                    }
                                } else {
                                    significant
                                };
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        (at, fraction, digits, significant) = (at_2, fraction_3, digits_2, significant_3);
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
                            let dot_2 = true;
                            {
                                let at_3 = at + 1f64;
                                {
                                    (at, dot) = (at_3, dot_2);
                                    continue;
                                }
                            }
                        }
                    }
{
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Done(at, fraction, digits, significant)
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop19Done(at, fraction, digits, significant)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Next(f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin18Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(self::ml::Big, bool),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(clippy::too_many_arguments, reason = "checked source arguments and captured continuation state retain their original order")]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop25(units: &[f64], mut end: f64, mut integer_digits: Vec<self::ml::Big>, mut radix: f64, mut value: self::ml::Big, mut overflow: bool, mut limit: self::ml::Big, mut base: self::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result {
    loop {
        return if index < end {
            {
                let digit = self::read_string_number_digit((self::ml_array::optional_at(units, self::ml_array::number_index(index))).unwrap_or_else(|| -1f64));
                if (digit < 0f64) || (digit >= radix) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Return(f64::NAN)
                } else {
                    if !overflow {
                        {
                            let value_2 = (value.mul(&base)).add(&(self::ml_array::optional_at(&integer_digits, self::ml_array::number_index(digit))).unwrap_or_else(|| self::ml::Big::from_i128(0)));
                            {
                                let overflow_2 = value_2 > limit;
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        (value, overflow, index) = (value_2.clone(), overflow_2, index_2);
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop25Done(value.clone(), overflow)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin17Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Done,
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop14(units: &[f64], mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Result {
    loop {
        return if index < (units.len() as f64) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(index))).unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_safe_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop14Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result {
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Done(Vec<f64>),
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Result {
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Done(f64, f64),
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop29(bytes: &[f64], mut index: f64, mut scalar: f64, mut remaining: f64) -> self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Result {
    loop {
        return if remaining > 0f64 {
            {
                let next = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).unwrap_or_else(|| -1f64);
                if (next < 128f64) || (next > 191f64) {
                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Return(f64::NAN)
                } else {
                    {
                        let scalar_2 = ((scalar * 64f64) + next) - 128f64;
                        {
                            let index_2 = index + 1f64;
                            {
                                let remaining_2 = remaining - 1f64;
                                {
                                    (index, scalar, remaining) = (index_2, scalar_2, remaining_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Done(index, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28 {
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Next(f64, f64, f64),
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop27(bytes: &[f64], mut units: Vec<f64>, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result {
    loop {
        return if index < (bytes.len() as f64) {
            {
                let scalar = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).unwrap_or_else(|| -1f64);
                {
                    let index_2 = index + 1f64;
                    {
                        let remaining = 0f64;
                        {
                            let minimum = 0f64;
                            {
                                let source_ml_source2_s25 = if (194f64..=223f64).contains(&scalar) {
                                    {
                                        let scalar_2 = scalar - 192f64;
                                        {
                                            let remaining_2 = 1f64;
                                            {
                                                let minimum_2 = 128f64;
                                                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Next(scalar_2, remaining_2, minimum_2)
                                            }
                                        }
                                    }
                                } else if (224f64..=239f64).contains(&scalar) {
                                        {
                                            let scalar_3 = scalar - 224f64;
                                            {
                                                let remaining_3 = 2f64;
                                                {
                                                    let minimum_3 = 2048f64;
                                                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Next(scalar_3, remaining_3, minimum_3)
                                                }
                                            }
                                        }
                                    } else if (240f64..=244f64).contains(&scalar) {
                                            {
                                                let scalar_4 = scalar - 240f64;
                                                {
                                                    let remaining_4 = 3f64;
                                                    {
                                                        let minimum_4 = 65_536f64;
                                                        self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Next(scalar_4, remaining_4, minimum_4)
                                                    }
                                                }
                                            }
                                        } else if scalar >= 128f64 {
                                                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Return(f64::NAN)
                                            } else {
                                                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Next(scalar, remaining, minimum)
                                            };
                                match source_ml_source2_s25 {
                                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source2_s26 = self::source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop29(bytes, index_2, scalar_5, remaining_5);
                                            match source_ml_source2_s26 {
                                                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Done(index_3, scalar_6) => {
                                                    if ((scalar_6 < minimum_5) || (scalar_6 > 1_114_111f64)) || ((55_296f64..=57_343f64).contains(&scalar_6)) {
                                                        self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Return(f64::NAN)
                                                    } else {
                                                        if scalar_6 <= 65_535f64 {
                                                            {
                                                                let units_2 = { let (ml_append_right, ml_append_left) = (vec![scalar_6], units); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    (units, index) = (units_2, index_3);
                                                                    continue;
                                                                }
                                                            }
                                                        }
{
                                                            {
                                                                let units_3 = { let (ml_append_right, ml_append_left) = (vec![(55_296f64 + ((scalar_6 - 65_536f64) / 1024f64).floor())], units); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let units_4 = { let (ml_append_right, ml_append_left) = (vec![(56_320f64 + ((scalar_6 - 65_536f64) % 1024f64))], units_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        (units, index) = (units_4, index_3);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop29Return(source_ml_source1_temporary_result29) => {
                                                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Return(source_ml_source1_temporary_result29)
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsJoin28Return(source_ml_source1_temporary_result29_2) => {
                                        self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Return(source_ml_source1_temporary_result29_2)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop27Done(units)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Result {
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Done,
    MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_utf8_string_number_units_loop26(bytes: &[f64], mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Result {
    loop {
        return if index < (bytes.len() as f64) {
            {
                let unit = (self::ml_array::optional_at(bytes, self::ml_array::number_index(index))).unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_safe_integer(unit)) || (unit < 0f64)) || (unit > 255f64) {
                    self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Result::MlSource1TemporaryConvertJavaScriptUtf8StringNumberUnitsLoop26Done
        };
    }
}
