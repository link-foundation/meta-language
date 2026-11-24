// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryParseJavaScriptFloatPrefixJoin1348;
use super::MlSource1TemporaryParseJavaScriptFloatPrefixLoop1347Result;
use super::MlSource1TemporaryParseJavaScriptFloatPrefixLoop1350Result;
use super::construct_java_script_semantic_text_units;
use super::convert_java_script_string_number_units;
use super::ml_array;
use super::ml_string;
use super::parse_java_script_integer_prefix;
use super::read_java_script_numeric_parsing_arity;
use super::read_java_script_semantic_text_length;
use super::source_ml_source1_temporary_parse_java_script_float_prefix_loop1345;
use super::source_ml_source1_temporary_parse_java_script_float_prefix_loop1346;
use super::source_ml_source1_temporary_parse_java_script_float_prefix_loop1347;
use super::source_ml_source1_temporary_parse_java_script_float_prefix_loop1349;
use super::source_ml_source1_temporary_parse_java_script_float_prefix_loop1350;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn parse_java_script_float_prefix(units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> f64 {
    {
        let start = 0f64;
        {
            let start_2 = self::source_ml_source1_temporary_parse_java_script_float_prefix_loop1345(
                units.clone(),
                start,
            );
            {
                let at = start_2;
                {
                    let sign = ({
                        let ml_reference_array2020 = units.clone();
                        let ml_reference_position2020 = at;
                        let ml_reference_projection2020 = self::ml_array::optional_at(
                            &*ml_reference_array2020.borrow(),
                            self::ml_array::number_index(ml_reference_position2020),
                        );
                        ml_reference_projection2020
                    })
                    .clone()
                    .unwrap_or_else(|| (-1f64));
                    {
                        let at_3 = if ((sign == (43f64)) || (sign == (45f64))) {
                            {
                                let at_2 = (at + 1f64);
                                at_2
                            }
                        } else {
                            at
                        };
                        {
                            let infinity = std::rc::Rc::new(std::cell::RefCell::new(vec![
                                73f64, 110f64, 102f64, 105f64, 110f64, 105f64, 116f64, 121f64,
                            ]));
                            {
                                let matches = true;
                                {
                                    let index = 0f64;
                                    {
                                        let matches_2 = self::source_ml_source1_temporary_parse_java_script_float_prefix_loop1346(units.clone(), at_3, infinity.clone(), matches, index);
                                        if matches_2 {
                                            if (sign == (45f64)) {
                                                f64::NEG_INFINITY
                                            } else {
                                                f64::INFINITY
                                            }
                                        } else {
                                            {
                                                let digits = 0f64;
                                                {
                                                    let source_ml_source2_s32543 = self::source_ml_source1_temporary_parse_java_script_float_prefix_loop1347(units.clone(), at_3, digits);
                                                    match source_ml_source2_s32543.clone() {
                                                        self::MlSource1TemporaryParseJavaScriptFloatPrefixLoop1347Result::MlSource1TemporaryParseJavaScriptFloatPrefixLoop1347Done(at_4, digits_2) => {
                                                            {
                                                                let source_ml_source2_s32549 = if ((({ let ml_reference_array2021 = units.clone(); let ml_reference_position2021 = at_4; let ml_reference_projection2021 = self::ml_array::optional_at(&*ml_reference_array2021.borrow(), self::ml_array::number_index(ml_reference_position2021)); ml_reference_projection2021 }).clone().unwrap_or_else(|| (-1f64))) == (46f64)) {
                                                                    {
                                                                        let at_5 = (at_4 + 1f64);
                                                                        {
                                                                            let source_ml_source2_s32554 = self::source_ml_source1_temporary_parse_java_script_float_prefix_loop1350(units.clone(), at_5, digits_2);
                                                                            match source_ml_source2_s32554.clone() {
                                                                                self::MlSource1TemporaryParseJavaScriptFloatPrefixLoop1350Result::MlSource1TemporaryParseJavaScriptFloatPrefixLoop1350Done(at_6, digits_3) => {
                                                                                    self::MlSource1TemporaryParseJavaScriptFloatPrefixJoin1348::MlSource1TemporaryParseJavaScriptFloatPrefixJoin1348Next(at_6, digits_3)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlSource1TemporaryParseJavaScriptFloatPrefixJoin1348::MlSource1TemporaryParseJavaScriptFloatPrefixJoin1348Next(at_4, digits_2)
                                                                };
                                                                match source_ml_source2_s32549.clone() {
                                                                    self::MlSource1TemporaryParseJavaScriptFloatPrefixJoin1348::MlSource1TemporaryParseJavaScriptFloatPrefixJoin1348Next(at_7, digits_4) => {
                                                                        if (digits_4 == (0f64)) {
                                                                            f64::NAN
                                                                        } else {
                                                                            {
                                                                                let mantissa_end = at_7;
                                                                                {
                                                                                    let at_12 = if (((({ let ml_reference_array2022 = units.clone(); let ml_reference_position2022 = at_7; let ml_reference_projection2022 = self::ml_array::optional_at(&*ml_reference_array2022.borrow(), self::ml_array::number_index(ml_reference_position2022)); ml_reference_projection2022 }).clone().unwrap_or_else(|| (-1f64))) == (69f64)) || ((({ let ml_reference_array2023 = units.clone(); let ml_reference_position2023 = at_7; let ml_reference_projection2023 = self::ml_array::optional_at(&*ml_reference_array2023.borrow(), self::ml_array::number_index(ml_reference_position2023)); ml_reference_projection2023 }).clone().unwrap_or_else(|| (-1f64))) == (101f64))) {
                                                                                        {
                                                                                            let at_8 = (at_7 + 1f64);
                                                                                            {
                                                                                                let at_10 = if (((({ let ml_reference_array2024 = units.clone(); let ml_reference_position2024 = at_8; let ml_reference_projection2024 = self::ml_array::optional_at(&*ml_reference_array2024.borrow(), self::ml_array::number_index(ml_reference_position2024)); ml_reference_projection2024 }).clone().unwrap_or_else(|| (-1f64))) == (43f64)) || ((({ let ml_reference_array2025 = units.clone(); let ml_reference_position2025 = at_8; let ml_reference_projection2025 = self::ml_array::optional_at(&*ml_reference_array2025.borrow(), self::ml_array::number_index(ml_reference_position2025)); ml_reference_projection2025 }).clone().unwrap_or_else(|| (-1f64))) == (45f64))) {
                                                                                                    {
                                                                                                        let at_9 = (at_8 + 1f64);
                                                                                                        at_9
                                                                                                    }
                                                                                                } else {
                                                                                                    at_8
                                                                                                };
                                                                                                {
                                                                                                    let exponent_start = at_10;
                                                                                                    {
                                                                                                        let at_11 = self::source_ml_source1_temporary_parse_java_script_float_prefix_loop1349(units.clone(), at_10);
                                                                                                        if (at_11 == exponent_start) {
                                                                                                            mantissa_end
                                                                                                        } else {
                                                                                                            at_11
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        at_7
                                                                                    };
                                                                                    self::convert_java_script_string_number_units(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value2026 = units.borrow().clone(); ml_reference_value2026 }, start_2, at_12))))
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_numeric_parsing(
    operation: &self::ml_string::Text,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let arity = self::read_java_script_numeric_parsing_arity(operation);
        {
            let input = ({
                let source_ml_source2_array_read_index32572 = 0f64;
                {
                    let ml_reference_array2027 = values.clone();
                    let ml_reference_position2027 = source_ml_source2_array_read_index32572;
                    let ml_reference_projection2027 = self::ml_array::optional_at(
                        &*ml_reference_array2027.borrow(),
                        self::ml_array::number_index(ml_reference_position2027),
                    );
                    ml_reference_projection2027
                }
            })
            .clone()
            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            {
                let radix = ({
                    let source_ml_source2_array_read_index32574 = 1f64;
                    {
                        let ml_reference_array2028 = values.clone();
                        let ml_reference_position2028 = source_ml_source2_array_read_index32574;
                        let ml_reference_projection2028 = self::ml_array::optional_at(
                            &*ml_reference_array2028.borrow(),
                            self::ml_array::number_index(ml_reference_position2028),
                        );
                        ml_reference_projection2028
                    }
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                {
                    let length = self::read_java_script_semantic_text_length(input.clone());
                    if ((((arity < (0f64))
                        || (({
                            let ml_reference_array2029 = values.clone();
                            let ml_reference_projection2029 =
                                ml_reference_array2029.borrow().len() as f64;
                            ml_reference_projection2029
                        }) != arity))
                        || (length < (0f64)))
                        || ((arity == (2f64))
                            && match radix.clone() {
                                self::JavaScriptSemanticValue::Number(_) => false,
                                _ => true,
                            }))
                    {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                            self::ml_string::Text::from_units(vec![
                                110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 101, 102, 105, 120,
                                32, 112, 97, 114, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105,
                                114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49,
                                54, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 110, 117, 109,
                                101, 114, 105, 99, 32, 114, 97, 100, 105, 120,
                            ]),
                        )
                    } else {
                        if (length > (4096f64)) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(
                                self::ml_string::Text::from_units(vec![
                                    110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 101, 102, 105,
                                    120, 32, 112, 97, 114, 115, 105, 110, 103, 32, 101, 120, 99,
                                    101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101,
                                    99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116,
                                    32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101,
                                ]),
                            )
                        } else {
                            {
                                let units = self::construct_java_script_semantic_text_units(
                                    input.clone(),
                                    4096f64,
                                );
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                    self::JavaScriptSemanticValue::Number(if (arity == (2f64)) {
                                        self::parse_java_script_integer_prefix(
                                            units.clone(),
                                            match radix.clone() {
                                                self::JavaScriptSemanticValue::Number(value) => {
                                                    value
                                                }
                                                _ => f64::NAN,
                                            },
                                        )
                                    } else {
                                        self::parse_java_script_float_prefix(units.clone())
                                    }),
                                ))
                            }
                        }
                    }
                }
            }
        }
    }
}
