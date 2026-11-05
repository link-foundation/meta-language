// Generated from complete js/src/translation/rust-saturating-integer-rules.js. Do not edit by hand.
use super::ml;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_rust_saturating_integer_bound_negative(
    bound: &self::ml_string::Text,
    signed: bool,
) -> bool {
    ((*bound) == (self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109])))
        && signed
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_rust_saturating_integer(
    method: &self::ml_string::Text,
    signed: bool,
) -> Vec<self::ml_string::Text> {
    if ((*method)
        == (self::ml_string::Text::from_units(vec![
            115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 110, 101, 103,
        ])))
        && signed
    {
        vec![
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![101, 113]),
            self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![110, 101, 103]),
            self::ml_string::Text::from_units(vec![105, 102]),
        ]
    } else if ((*method)
        == (self::ml_string::Text::from_units(vec![
            115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 97, 98, 115,
        ])))
        && signed
    {
        vec![
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![101, 113]),
            self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
            self::ml_string::Text::from_units(vec![108, 116]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![110, 101, 103]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![105, 102]),
            self::ml_string::Text::from_units(vec![105, 102]),
        ]
    } else if ((*method)
        == (self::ml_string::Text::from_units(vec![
            115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 97, 100, 100,
        ])))
        && !signed
    {
        vec![
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![115, 117, 98]),
            self::ml_string::Text::from_units(vec![103, 116]),
            self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![97, 100, 100]),
            self::ml_string::Text::from_units(vec![105, 102]),
        ]
    } else if ((*method)
        == (self::ml_string::Text::from_units(vec![
            115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 115, 117, 98,
        ])))
        && !signed
    {
        vec![
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![108, 116]),
            self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![115, 117, 98]),
            self::ml_string::Text::from_units(vec![105, 102]),
        ]
    } else if ((*method)
        == (self::ml_string::Text::from_units(vec![
            115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 97, 100, 100,
        ])))
        && signed
    {
        vec![
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
            self::ml_string::Text::from_units(vec![103, 116]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![115, 117, 98]),
            self::ml_string::Text::from_units(vec![103, 116]),
            self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![97, 100, 100]),
            self::ml_string::Text::from_units(vec![105, 102]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
            self::ml_string::Text::from_units(vec![108, 116]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![115, 117, 98]),
            self::ml_string::Text::from_units(vec![108, 116]),
            self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![97, 100, 100]),
            self::ml_string::Text::from_units(vec![105, 102]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![105, 102]),
            self::ml_string::Text::from_units(vec![105, 102]),
        ]
    } else if ((*method)
        == (self::ml_string::Text::from_units(vec![
            115, 97, 116, 117, 114, 97, 116, 105, 110, 103, 95, 115, 117, 98,
        ])))
        && signed
    {
        vec![
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
            self::ml_string::Text::from_units(vec![108, 116]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![97, 100, 100]),
            self::ml_string::Text::from_units(vec![103, 116]),
            self::ml_string::Text::from_units(vec![109, 97, 120, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![115, 117, 98]),
            self::ml_string::Text::from_units(vec![105, 102]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![122, 101, 114, 111]),
            self::ml_string::Text::from_units(vec![103, 116]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![97, 100, 100]),
            self::ml_string::Text::from_units(vec![108, 116]),
            self::ml_string::Text::from_units(vec![109, 105, 110, 105, 109, 117, 109]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116]),
            self::ml_string::Text::from_units(vec![115, 117, 98]),
            self::ml_string::Text::from_units(vec![105, 102]),
            self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 110, 100]),
            self::ml_string::Text::from_units(vec![105, 102]),
            self::ml_string::Text::from_units(vec![105, 102]),
        ]
    } else {
        Vec::<self::ml_string::Text>::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_wrap_fixed_integer_bits_loop1(
    mut bits: f64,
    mut modulus: self::ml::Big,
    mut bit: f64,
) -> self::ml::Big {
    loop {
        if bit < bits {
            {
                let modulus_2 = modulus.mul(&self::ml::Big::from_i128(2));
                {
                    let bit_2 = bit + 1f64;
                    {
                        (modulus, bit) = (modulus_2, bit_2);
                        continue;
                    }
                }
            }
        }
        return { modulus.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_integer_bound_predecessor_loop2(
    source: &self::ml_string::Text,
    mut end: f64,
) -> f64 {
    loop {
        if (end > 0f64)
            && (({
                let source_ml_source2_string_unit_operand26 = source.clone();
                {
                    let source_ml_source2_string_unit_operand27 = end - 1f64;
                    (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 {
                        {
                            let index = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if (index < 0f64) || (index >= (text.length())) {
                                f64::NAN
                            } else {
                                ({
                            let source_ml_source2_string_unit_operand33 = {
                                let source_ml_source2_string_range_operand28 = text.clone();
                                {
                                    let source_ml_source2_string_range_operand29 = index;
                                    {
                                        let source_ml_source2_string_range_operand30 = index + 1f64;
                                        {
                                            let source_ml_source2_string_range_position31 = source_ml_source2_string_range_operand29;
                                            {
                                                let source_ml_source2_string_range_position32 = source_ml_source2_string_range_operand30;
                                                source_ml_source2_string_range_operand28.slice(source_ml_source2_string_range_position31, source_ml_source2_string_range_position32)
                                            }
                                        }
                                    }
                                }
                            };
                            {
                                let source_ml_source2_string_unit_operand34 = 0f64;
                                source_ml_source2_string_unit_operand33.code_point_at(source_ml_source2_string_unit_operand34)
                            }
                        }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        }
                    }))(
                        source_ml_source2_string_unit_operand26,
                        source_ml_source2_string_unit_operand27,
                    )
                }
            }) == 48f64)
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
pub fn source_ml_source1_temporary_read_rust_integer_bound_predecessor_loop3(
    source: &self::ml_string::Text,
    mut result: self::ml_string::Text,
    mut index: f64,
) -> self::ml_string::Text {
    loop {
        if index < (source.length()) {
            {
                let result_2 = result.concat(&self::ml_string::Text::from_units(vec![57]));
                {
                    let index_2 = index + 1f64;
                    {
                        (result, index) = (result_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}
