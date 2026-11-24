pub fn source_ml_source1_temporary_convert_string_number_rational_loop24(mut numerator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < shift) {
            {
                let numerator_2 = numerator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (numerator, index) = (numerator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            numerator.clone()
        };
    }
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop25(mut denominator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < ((-shift))) {
            {
                let denominator_2 = denominator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (denominator, index) = (denominator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            denominator.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin20 {
    MlSource1TemporaryConvertStringNumberRationalJoin20Next(f64),
    MlSource1TemporaryConvertStringNumberRationalJoin20Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop26Result {
    MlSource1TemporaryConvertStringNumberRationalLoop26Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop26Return(f64),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop26(mut numerator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop26Result {
    loop {
        return if (numerator >= (scale.mul(&self::ml::Big::from_i128(2)))) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent + 1f64);
                    if (exponent_2 >= (1024f64)) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop26Result::MlSource1TemporaryConvertStringNumberRationalLoop26Return(f64::INFINITY)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop26Result::MlSource1TemporaryConvertStringNumberRationalLoop26Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop27Result {
    MlSource1TemporaryConvertStringNumberRationalLoop27Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop27Return(f64),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop27(mut denominator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop27Result {
    loop {
        return if (scale < denominator) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent - 1f64);
                    if (exponent_2 < ((-1075f64))) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop27Result::MlSource1TemporaryConvertStringNumberRationalLoop27Return(0f64)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop27Result::MlSource1TemporaryConvertStringNumberRationalLoop27Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptStringNumberInfinityLoop28Result {
    MlSource1TemporaryAcceptStringNumberInfinityLoop28Done,
    MlSource1TemporaryAcceptStringNumberInfinityLoop28Return(bool),
}

pub fn source_ml_source1_temporary_accept_string_number_infinity_loop28(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut start: f64, mut expected: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::MlSource1TemporaryAcceptStringNumberInfinityLoop28Result {
    loop {
        return if (index < ({ let ml_reference_array432 = expected.clone(); let ml_reference_projection432 = ml_reference_array432.borrow().len() as f64; ml_reference_projection432 })) {
            if ((({
                let source_ml_source2_array_read_index3301 = (start + index);
                { let ml_reference_array433 = units.clone(); let ml_reference_position433 = source_ml_source2_array_read_index3301; let ml_reference_projection433 = self::ml_array::optional_at(&*ml_reference_array433.borrow(), self::ml_array::number_index(ml_reference_position433)); ml_reference_projection433 }
            }).clone().unwrap_or_else(|| (-1f64))) != (({ let ml_reference_array434 = expected.clone(); let ml_reference_position434 = index; let ml_reference_projection434 = self::ml_array::optional_at(&*ml_reference_array434.borrow(), self::ml_array::number_index(ml_reference_position434)); ml_reference_projection434 }).clone().unwrap_or_else(|| (-2f64)))) {
                self::MlSource1TemporaryAcceptStringNumberInfinityLoop28Result::MlSource1TemporaryAcceptStringNumberInfinityLoop28Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptStringNumberInfinityLoop28Result::MlSource1TemporaryAcceptStringNumberInfinityLoop28Done
        };
    }
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop30(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((start < end) && self::accept_java_script_whitespace_unit(({ let ml_reference_array435 = units.clone(); let ml_reference_position435 = start; let ml_reference_projection435 = self::ml_array::optional_at(&*ml_reference_array435.borrow(), self::ml_array::number_index(ml_reference_position435)); ml_reference_projection435 }).clone().unwrap_or_else(|| (-1f64)))) {
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

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop31(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((end > start) && self::accept_java_script_whitespace_unit(({
            let source_ml_source2_array_read_index3307 = (end - 1f64);
            { let ml_reference_array436 = units.clone(); let ml_reference_position436 = source_ml_source2_array_read_index3307; let ml_reference_projection436 = self::ml_array::optional_at(&*ml_reference_array436.borrow(), self::ml_array::number_index(ml_reference_position436)); ml_reference_projection436 }
        }).clone().unwrap_or_else(|| (-1f64)))) {
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

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop37(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut integer_digits: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut mantissa_end: f64, mut mantissa: self::ml::Big, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < mantissa_end) {
            {
                let unit = ({ let ml_reference_array437 = units.clone(); let ml_reference_position437 = index; let ml_reference_projection437 = self::ml_array::optional_at(&*ml_reference_array437.borrow(), self::ml_array::number_index(ml_reference_position437)); ml_reference_projection437 }).clone().unwrap_or_else(|| (-1f64));
                if (unit != (46f64)) {
                    {
                        let mantissa_2 = (mantissa.mul(&self::ml::Big::from_i128(10))).add(&({
                            let source_ml_source2_array_read_index3311 = (unit - 48f64);
                            { let ml_reference_array438 = integer_digits.clone(); let ml_reference_position438 = source_ml_source2_array_read_index3311; let ml_reference_projection438 = self::ml_array::optional_at(&*ml_reference_array438.borrow(), self::ml_array::number_index(ml_reference_position438)); ml_reference_projection438 }
                        }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (mantissa, index) = (mantissa_2.clone(), index_2);
                                continue;
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
        } else {
            mantissa.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Done(f64, f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop36(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut start: f64, mut exponent: f64, mut mantissa_end: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Result {
    loop {
        return if (mantissa_end > start) {
            {
                let unit = ({
                    let source_ml_source2_array_read_index3313 = (mantissa_end - 1f64);
                    { let ml_reference_array439 = units.clone(); let ml_reference_position439 = source_ml_source2_array_read_index3313; let ml_reference_projection439 = self::ml_array::optional_at(&*ml_reference_array439.borrow(), self::ml_array::number_index(ml_reference_position439)); ml_reference_projection439 }
                }).clone().unwrap_or_else(|| (-1f64));
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
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Done(exponent, mantissa_end)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Done(exponent, mantissa_end)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35Next(f64, f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Done(f64, f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop39(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut end: f64, mut at: f64, mut exponent: f64, mut exponent_limit: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Result {
    loop {
        return if (at < end) {
            {
                let digit = (({ let ml_reference_array440 = units.clone(); let ml_reference_position440 = at; let ml_reference_projection440 = self::ml_array::optional_at(&*ml_reference_array440.borrow(), self::ml_array::number_index(ml_reference_position440)); ml_reference_projection440 }).clone().unwrap_or_else(|| (-1f64)) - 48f64);
                if ((digit < (0f64)) || (digit > (9f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Done(at, exponent)
                } else {
                    {
                        let exponent_3 = if (exponent < exponent_limit) {
                            {
                                let exponent_2 = self::ml_math::min(exponent_limit, ((exponent * 10f64) + digit));
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Done(at, exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin38 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin38Next(f64, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Done(f64, f64, f64, f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop34(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut end: f64, mut at: f64, mut fraction: f64, mut digits: f64, mut significant: f64, mut dot: bool) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Result {
    loop {
        return if (at < end) {
            {
                let unit = ({ let ml_reference_array441 = units.clone(); let ml_reference_position441 = at; let ml_reference_projection441 = self::ml_array::optional_at(&*ml_reference_array441.borrow(), self::ml_array::number_index(ml_reference_position441)); ml_reference_projection441 }).clone().unwrap_or_else(|| (-1f64));
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
                                let significant_3 = if ((unit != (48f64)) || (significant > (0f64))) {
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
                                        (at, fraction, digits, significant) = (at_2, fraction_3, digits_2, significant_3);
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
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Done(at, fraction, digits, significant)
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop34Done(at, fraction, digits, significant)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Next(f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin33Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Done(self::ml::Big, bool),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Return(f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop40(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut end: f64, mut integer_digits: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut radix: f64, mut value: self::ml::Big, mut overflow: bool, mut limit: self::ml::Big, mut base: self::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_string_number_digit(({ let ml_reference_array442 = units.clone(); let ml_reference_position442 = index; let ml_reference_projection442 = self::ml_array::optional_at(&*ml_reference_array442.borrow(), self::ml_array::number_index(ml_reference_position442)); ml_reference_projection442 }).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Return(f64::NAN)
                } else {
                    if !overflow {
                        {
                            let value_2 = (value.mul(&base)).add(&({ let ml_reference_array443 = integer_digits.clone(); let ml_reference_position443 = digit; let ml_reference_projection443 = self::ml_array::optional_at(&*ml_reference_array443.borrow(), self::ml_array::number_index(ml_reference_position443)); ml_reference_projection443 }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)));
                            {
                                let overflow_2 = (value_2 > limit);
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        (value, overflow, index) = (value_2.clone(), overflow_2, index_2);
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop40Done(value.clone(), overflow)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin32 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin32Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done,
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(f64),
}

pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop29(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result {
    loop {
        return if (index < ({ let ml_reference_array444 = units.clone(); let ml_reference_projection444 = ml_reference_array444.borrow().len() as f64; ml_reference_projection444 })) {
            {
                let unit = ({ let ml_reference_array445 = units.clone(); let ml_reference_position445 = index; let ml_reference_projection445 = self::ml_array::optional_at(&*ml_reference_array445.borrow(), self::ml_array::number_index(ml_reference_position445)); ml_reference_projection445 }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Return(f64::NAN)
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
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop29Done
        };
    }
}

pub fn source_ml_source1_temporary_parse_java_script_integer_prefix_loop41(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut start: f64) -> f64 {
    loop {
        return if ((start < ({ let ml_reference_array446 = units.clone(); let ml_reference_projection446 = ml_reference_array446.borrow().len() as f64; ml_reference_projection446 })) && self::accept_java_script_whitespace_unit(({ let ml_reference_array447 = units.clone(); let ml_reference_position447 = start; let ml_reference_projection447 = self::ml_array::optional_at(&*ml_reference_array447.borrow(), self::ml_array::number_index(ml_reference_position447)); ml_reference_projection447 }).clone().unwrap_or_else(|| (-1f64)))) {
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryParseJavaScriptIntegerPrefixLoop44Result {
    MlSource1TemporaryParseJavaScriptIntegerPrefixLoop44Done(self::ml::Big, f64),
}

pub fn source_ml_source1_temporary_parse_java_script_integer_prefix_loop44(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut radix: f64, mut value: self::ml::Big, mut count: f64, mut base: self::ml::Big, mut at: f64) -> self::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop44Result {
    loop {
        return if (at < ({ let ml_reference_array448 = units.clone(); let ml_reference_projection448 = ml_reference_array448.borrow().len() as f64; ml_reference_projection448 })) {
            {
                let digit = self::read_java_script_radix_digit(({ let ml_reference_array449 = units.clone(); let ml_reference_position449 = at; let ml_reference_projection449 = self::ml_array::optional_at(&*ml_reference_array449.borrow(), self::ml_array::number_index(ml_reference_position449)); ml_reference_projection449 }).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop44Result::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop44Done(value.clone(), count)
                } else {
                    {
                        let value_2 = (value.mul(&base)).add(&{
                            let source_ml_source2_integer_result3329 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(digit);
                            if (((source_ml_source2_integer_result3329.len() as f64)) == (1f64)) {
                                (self::ml_array::optional_at(&source_ml_source2_integer_result3329, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(digit)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                            }
                        });
                        {
                            let count_2 = (count + 1f64);
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (value, count, at) = (value_2.clone(), count_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop44Result::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop44Done(value.clone(), count)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryParseJavaScriptIntegerPrefixJoin43 {
    MlSource1TemporaryParseJavaScriptIntegerPrefixJoin43Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryParseJavaScriptIntegerPrefixJoin42 {
    MlSource1TemporaryParseJavaScriptIntegerPrefixJoin42Next(f64, bool),
}

pub fn source_ml_source1_temporary_parse_java_script_float_prefix_loop45(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut start: f64) -> f64 {
    loop {
        return if ((start < ({ let ml_reference_array450 = units.clone(); let ml_reference_projection450 = ml_reference_array450.borrow().len() as f64; ml_reference_projection450 })) && self::accept_java_script_whitespace_unit(({ let ml_reference_array451 = units.clone(); let ml_reference_position451 = start; let ml_reference_projection451 = self::ml_array::optional_at(&*ml_reference_array451.borrow(), self::ml_array::number_index(ml_reference_position451)); ml_reference_projection451 }).clone().unwrap_or_else(|| (-1f64)))) {
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

pub fn source_ml_source1_temporary_parse_java_script_float_prefix_loop46(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64, mut infinity: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut matches: bool, mut index: f64) -> bool {
    loop {
        return if (index < ({ let ml_reference_array452 = infinity.clone(); let ml_reference_projection452 = ml_reference_array452.borrow().len() as f64; ml_reference_projection452 })) {
            if ((({
                let source_ml_source2_array_read_index3337 = (at + index);
                { let ml_reference_array453 = units.clone(); let ml_reference_position453 = source_ml_source2_array_read_index3337; let ml_reference_projection453 = self::ml_array::optional_at(&*ml_reference_array453.borrow(), self::ml_array::number_index(ml_reference_position453)); ml_reference_projection453 }
            }).clone().unwrap_or_else(|| (-1f64))) != (({ let ml_reference_array454 = infinity.clone(); let ml_reference_position454 = index; let ml_reference_projection454 = self::ml_array::optional_at(&*ml_reference_array454.borrow(), self::ml_array::number_index(ml_reference_position454)); ml_reference_projection454 }).clone().unwrap_or_else(|| (-2f64)))) {
                {
                    let index_2 = (index + 1f64);
                    {
                        (matches, index) = (false, index_2);
                        continue;
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
        } else {
            matches
        };
    }
}

pub fn source_ml_source1_temporary_parse_java_script_float_prefix_loop49(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> f64 {
    loop {
        return if (((({ let ml_reference_array455 = units.clone(); let ml_reference_position455 = at; let ml_reference_projection455 = self::ml_array::optional_at(&*ml_reference_array455.borrow(), self::ml_array::number_index(ml_reference_position455)); ml_reference_projection455 }).clone().unwrap_or_else(|| (-1f64))) >= (48f64)) && ((({ let ml_reference_array456 = units.clone(); let ml_reference_position456 = at; let ml_reference_projection456 = self::ml_array::optional_at(&*ml_reference_array456.borrow(), self::ml_array::number_index(ml_reference_position456)); ml_reference_projection456 }).clone().unwrap_or_else(|| (-1f64))) <= (57f64))) {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryParseJavaScriptFloatPrefixJoin48 {
    MlSource1TemporaryParseJavaScriptFloatPrefixJoin48Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryParseJavaScriptFloatPrefixLoop50Result {
    MlSource1TemporaryParseJavaScriptFloatPrefixLoop50Done(f64, f64),
}

pub fn source_ml_source1_temporary_parse_java_script_float_prefix_loop50(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64, mut digits: f64) -> self::MlSource1TemporaryParseJavaScriptFloatPrefixLoop50Result {
    loop {
        return if (((({ let ml_reference_array457 = units.clone(); let ml_reference_position457 = at; let ml_reference_projection457 = self::ml_array::optional_at(&*ml_reference_array457.borrow(), self::ml_array::number_index(ml_reference_position457)); ml_reference_projection457 }).clone().unwrap_or_else(|| (-1f64))) >= (48f64)) && ((({ let ml_reference_array458 = units.clone(); let ml_reference_position458 = at; let ml_reference_projection458 = self::ml_array::optional_at(&*ml_reference_array458.borrow(), self::ml_array::number_index(ml_reference_position458)); ml_reference_projection458 }).clone().unwrap_or_else(|| (-1f64))) <= (57f64))) {
            {
                let at_2 = (at + 1f64);
                {
                    let digits_2 = (digits + 1f64);
                    {
                        (at, digits) = (at_2, digits_2);
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryParseJavaScriptFloatPrefixLoop50Result::MlSource1TemporaryParseJavaScriptFloatPrefixLoop50Done(at, digits)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryParseJavaScriptFloatPrefixLoop47Result {
    MlSource1TemporaryParseJavaScriptFloatPrefixLoop47Done(f64, f64),
}

pub fn source_ml_source1_temporary_parse_java_script_float_prefix_loop47(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64, mut digits: f64) -> self::MlSource1TemporaryParseJavaScriptFloatPrefixLoop47Result {
    loop {
        return if (((({ let ml_reference_array459 = units.clone(); let ml_reference_position459 = at; let ml_reference_projection459 = self::ml_array::optional_at(&*ml_reference_array459.borrow(), self::ml_array::number_index(ml_reference_position459)); ml_reference_projection459 }).clone().unwrap_or_else(|| (-1f64))) >= (48f64)) && ((({ let ml_reference_array460 = units.clone(); let ml_reference_position460 = at; let ml_reference_projection460 = self::ml_array::optional_at(&*ml_reference_array460.borrow(), self::ml_array::number_index(ml_reference_position460)); ml_reference_projection460 }).clone().unwrap_or_else(|| (-1f64))) <= (57f64))) {
            {
                let at_2 = (at + 1f64);
                {
                    let digits_2 = (digits + 1f64);
                    {
                        (at, digits) = (at_2, digits_2);
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryParseJavaScriptFloatPrefixLoop47Result::MlSource1TemporaryParseJavaScriptFloatPrefixLoop47Done(at, digits)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptUnicodeCasePropertyLoop51Result {
    MlSource1TemporaryAcceptUnicodeCasePropertyLoop51Done,
    MlSource1TemporaryAcceptUnicodeCasePropertyLoop51Return(bool),
}

pub fn source_ml_source1_temporary_accept_unicode_case_property_loop51(mut point: f64, mut ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut first: f64, mut last: f64) -> self::MlSource1TemporaryAcceptUnicodeCasePropertyLoop51Result {
    loop {
        return if (first < last) {
            {
                let middle = (((first + last) / 2f64)).floor();
                {
                    let start = ({
                        let source_ml_source2_array_read_index3365 = (middle * 2f64);
                        { let ml_reference_array461 = ranges.clone(); let ml_reference_position461 = source_ml_source2_array_read_index3365; let ml_reference_projection461 = self::ml_array::optional_at(&*ml_reference_array461.borrow(), self::ml_array::number_index(ml_reference_position461)); ml_reference_projection461 }
                    }).clone().unwrap_or_else(|| (-1f64));
                    {
                        let end = ({
                            let source_ml_source2_array_read_index3367 = ((middle * 2f64) + 1f64);
                            { let ml_reference_array462 = ranges.clone(); let ml_reference_position462 = source_ml_source2_array_read_index3367; let ml_reference_projection462 = self::ml_array::optional_at(&*ml_reference_array462.borrow(), self::ml_array::number_index(ml_reference_position462)); ml_reference_projection462 }
                        }).clone().unwrap_or_else(|| (-1f64));
                        if (point < start) {
                            {
                                last = middle;
                                continue;
                            }
                        } else {
                            if (point > end) {
                                {
                                    let first_2 = (middle + 1f64);
                                    {
                                        first = first_2;
                                        continue;
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryAcceptUnicodeCasePropertyLoop51Result::MlSource1TemporaryAcceptUnicodeCasePropertyLoop51Return(true)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptUnicodeCasePropertyLoop51Result::MlSource1TemporaryAcceptUnicodeCasePropertyLoop51Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadUnicodeCaseMappingLoop53Result {
    MlSource1TemporaryReadUnicodeCaseMappingLoop53Done,
    MlSource1TemporaryReadUnicodeCaseMappingLoop53Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_read_unicode_case_mapping_loop53(mut point: f64, mut ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut first: f64, mut last: f64) -> self::MlSource1TemporaryReadUnicodeCaseMappingLoop53Result {
    loop {
        return if (first < last) {
            {
                let middle = (((first + last) / 2f64)).floor();
                {
                    let start = ({
                        let source_ml_source2_array_read_index3369 = (middle * 4f64);
                        { let ml_reference_array463 = ranges.clone(); let ml_reference_position463 = source_ml_source2_array_read_index3369; let ml_reference_projection463 = self::ml_array::optional_at(&*ml_reference_array463.borrow(), self::ml_array::number_index(ml_reference_position463)); ml_reference_projection463 }
                    }).clone().unwrap_or_else(|| (-1f64));
                    {
                        let end = ({
                            let source_ml_source2_array_read_index3371 = ((middle * 4f64) + 1f64);
                            { let ml_reference_array464 = ranges.clone(); let ml_reference_position464 = source_ml_source2_array_read_index3371; let ml_reference_projection464 = self::ml_array::optional_at(&*ml_reference_array464.borrow(), self::ml_array::number_index(ml_reference_position464)); ml_reference_projection464 }
                        }).clone().unwrap_or_else(|| (-1f64));
                        if (point < start) {
                            {
                                last = middle;
                                continue;
                            }
                        } else {
                            if (point > end) {
                                {
                                    let first_2 = (middle + 1f64);
                                    {
                                        first = first_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let stride = ({
                                        let source_ml_source2_array_read_index3373 = ((middle * 4f64) + 2f64);
                                        { let ml_reference_array465 = ranges.clone(); let ml_reference_position465 = source_ml_source2_array_read_index3373; let ml_reference_projection465 = self::ml_array::optional_at(&*ml_reference_array465.borrow(), self::ml_array::number_index(ml_reference_position465)); ml_reference_projection465 }
                                    }).clone().unwrap_or_else(|| 1f64);
                                    {
                                        let delta = ({
                                            let source_ml_source2_array_read_index3375 = ((middle * 4f64) + 3f64);
                                            { let ml_reference_array466 = ranges.clone(); let ml_reference_position466 = source_ml_source2_array_read_index3375; let ml_reference_projection466 = self::ml_array::optional_at(&*ml_reference_array466.borrow(), self::ml_array::number_index(ml_reference_position466)); ml_reference_projection466 }
                                        }).clone().unwrap_or_else(|| 0f64);
                                        self::MlSource1TemporaryReadUnicodeCaseMappingLoop53Result::MlSource1TemporaryReadUnicodeCaseMappingLoop53Return(std::rc::Rc::new(std::cell::RefCell::new(vec![if ((((point - start) % stride)) == (0f64)) {
                                            (point + delta)
                                        } else {
                                            point
                                        }])))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadUnicodeCaseMappingLoop53Result::MlSource1TemporaryReadUnicodeCaseMappingLoop53Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadUnicodeCaseMappingLoop52Result {
    MlSource1TemporaryReadUnicodeCaseMappingLoop52Done(f64, f64),
    MlSource1TemporaryReadUnicodeCaseMappingLoop52Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_read_unicode_case_mapping_loop54(mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut offset: f64, mut count: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < count) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value467 = result.borrow().clone(); ml_reference_value467 }, vec![({
                    let source_ml_source2_array_read_index3377 = (offset + at);
                    { let ml_reference_array468 = values.clone(); let ml_reference_position468 = source_ml_source2_array_read_index3377; let ml_reference_projection468 = self::ml_array::optional_at(&*ml_reference_array468.borrow(), self::ml_array::number_index(ml_reference_position468)); ml_reference_projection468 }
                }).clone().unwrap_or_else(|| 0f64)])));
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_read_unicode_case_mapping_loop52(mut point: f64, mut indexes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut first: f64, mut last: f64) -> self::MlSource1TemporaryReadUnicodeCaseMappingLoop52Result {
    loop {
        return if (first < last) {
            {
                let middle = (((first + last) / 2f64)).floor();
                {
                    let key = ({
                        let source_ml_source2_array_read_index3379 = (middle * 3f64);
                        { let ml_reference_array469 = indexes.clone(); let ml_reference_position469 = source_ml_source2_array_read_index3379; let ml_reference_projection469 = self::ml_array::optional_at(&*ml_reference_array469.borrow(), self::ml_array::number_index(ml_reference_position469)); ml_reference_projection469 }
                    }).clone().unwrap_or_else(|| (-1f64));
                    if (point < key) {
                        {
                            last = middle;
                            continue;
                        }
                    } else {
                        if (point > key) {
                            {
                                let first_2 = (middle + 1f64);
                                {
                                    first = first_2;
                                    continue;
                                }
                            }
                        } else {
                            {
                                let offset = ({
                                    let source_ml_source2_array_read_index3381 = ((middle * 3f64) + 1f64);
                                    { let ml_reference_array470 = indexes.clone(); let ml_reference_position470 = source_ml_source2_array_read_index3381; let ml_reference_projection470 = self::ml_array::optional_at(&*ml_reference_array470.borrow(), self::ml_array::number_index(ml_reference_position470)); ml_reference_projection470 }
                                }).clone().unwrap_or_else(|| 0f64);
                                {
                                    let count = ({
                                        let source_ml_source2_array_read_index3383 = ((middle * 3f64) + 2f64);
                                        { let ml_reference_array471 = indexes.clone(); let ml_reference_position471 = source_ml_source2_array_read_index3383; let ml_reference_projection471 = self::ml_array::optional_at(&*ml_reference_array471.borrow(), self::ml_array::number_index(ml_reference_position471)); ml_reference_projection471 }
                                    }).clone().unwrap_or_else(|| 0f64);
                                    {
                                        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                        {
                                            let at = 0f64;
                                            {
                                                let result_2 = self::source_ml_source1_temporary_read_unicode_case_mapping_loop54(values.clone(), offset, count, result.clone(), at);
                                                self::MlSource1TemporaryReadUnicodeCaseMappingLoop52Result::MlSource1TemporaryReadUnicodeCaseMappingLoop52Return(result_2.clone())
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
            self::MlSource1TemporaryReadUnicodeCaseMappingLoop52Result::MlSource1TemporaryReadUnicodeCaseMappingLoop52Done(first, last)
        };
    }
}

pub fn source_ml_source1_temporary_map_unicode_case_utf16_columns_loop55(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut points: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ({ let ml_reference_array472 = text.clone(); let ml_reference_projection472 = ml_reference_array472.borrow().len() as f64; ml_reference_projection472 })) {
            {
                let first = ({ let ml_reference_array473 = text.clone(); let ml_reference_position473 = at; let ml_reference_projection473 = self::ml_array::optional_at(&*ml_reference_array473.borrow(), self::ml_array::number_index(ml_reference_position473)); ml_reference_projection473 }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(first)) || (first < (0f64))) || (first > (65535f64))) {
                    panic!("{}", "Unicode casing requires actual UTF16 units")
                } else {
                    {
                        let second = ({
                            let source_ml_source2_array_read_index3387 = (at + 1f64);
                            { let ml_reference_array474 = text.clone(); let ml_reference_position474 = source_ml_source2_array_read_index3387; let ml_reference_projection474 = self::ml_array::optional_at(&*ml_reference_array474.borrow(), self::ml_array::number_index(ml_reference_position474)); ml_reference_projection474 }
                        }).clone().unwrap_or_else(|| (-1f64));
                        if (((((first >= (55296f64)) && (first <= (56319f64))) && self::ml_math::is_safe_integer(second)) && (second >= (56320f64))) && (second <= (57343f64))) {
                            {
                                let points_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value475 = points.borrow().clone(); ml_reference_value475 }, vec![(((65536f64 + ((first - 55296f64) * 1024f64)) + second) - 56320f64)])));
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        let at_3 = (at_2 + 1f64);
                                        {
                                            (points, at) = (points_2.clone(), at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let points_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value476 = points.borrow().clone(); ml_reference_value476 }, vec![first])));
                                {
                                    let at_4 = (at + 1f64);
                                    {
                                        (points, at) = (points_3.clone(), at_4);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            points.clone()
        };
    }
}
