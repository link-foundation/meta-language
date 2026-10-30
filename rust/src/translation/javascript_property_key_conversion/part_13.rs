// Generated from complete js/src/translation/javascript-property-key-conversion.js. Do not edit by hand.
use super::ml_array;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_finish_java_script_property_key_conversion_loop29(
    spelling: &self::ml_string::Text,
    mut units: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (spelling.length()) {
            {
                let units_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![{
                            let source_ml_source1_string_unit_operand1100 = spelling.clone();
                            {
                                let source_ml_source1_string_unit_operand1101 = index;
                                (std::rc::Rc::new(
                                    move |text: self::ml_string::Text, position: f64| -> f64 {
                                        {
                                            let index_2 = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if (index_2 < 0f64) || (index_2 >= (text.length())) {
                                                f64::NAN
                                            } else {
                                                ({
                                    let source_ml_source1_string_unit_operand1107 = {
                                        let source_ml_source1_string_range_operand1102 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand1103 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand1104 = index_2 + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position1105 = source_ml_source1_string_range_operand1103;
                                                    {
                                                        let source_ml_source1_string_range_position1106 = source_ml_source1_string_range_operand1104;
                                                        source_ml_source1_string_range_operand1102.slice(source_ml_source1_string_range_position1105, source_ml_source1_string_range_position1106)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand1108 = 0f64;
                                        source_ml_source1_string_unit_operand1107.code_point_at(source_ml_source1_string_unit_operand1108)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        }
                                    },
                                ))(
                                    source_ml_source1_string_unit_operand1100,
                                    source_ml_source1_string_unit_operand1101,
                                )
                            }
                        }],
                        units,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (units, index) = (units_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { units };
    }
}
