// Generated from complete js/src/translation/rust-source-constant-evaluation-rules.js. Do not edit by hand.
use super::ml_array;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_constant_byte_length_loop33(
    text: &self::ml_string::Text,
    mut units: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (text.length()) {
            {
                let units_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![{
                            let source_ml_source1_string_unit_operand195 = text.clone();
                            {
                                let source_ml_source1_string_unit_operand196 = at;
                                (std::rc::Rc::new(
                                    move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                        {
                                            let index = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if (index < 0f64) || (index >= (text_2.length())) {
                                                f64::NAN
                                            } else {
                                                ({
                                    let source_ml_source1_string_unit_operand202 = {
                                        let source_ml_source1_string_range_operand197 = text_2.clone();
                                        {
                                            let source_ml_source1_string_range_operand198 = index;
                                            {
                                                let source_ml_source1_string_range_operand199 = index + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position200 = source_ml_source1_string_range_operand198;
                                                    {
                                                        let source_ml_source1_string_range_position201 = source_ml_source1_string_range_operand199;
                                                        source_ml_source1_string_range_operand197.slice(source_ml_source1_string_range_position200, source_ml_source1_string_range_position201)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand203 = 0f64;
                                        source_ml_source1_string_unit_operand202.code_point_at(source_ml_source1_string_unit_operand203)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        }
                                    },
                                ))(
                                    source_ml_source1_string_unit_operand195,
                                    source_ml_source1_string_unit_operand196,
                                )
                            }
                        }],
                        units,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (units, at) = (units_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { units };
    }
}
