// Generated from complete js/src/translation/javascript-enumeration-rules.js. Do not edit by hand.
use super::ml_array;
use super::read_array_callback_parameter_role;
use super::render_array_callback_argument;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_render_array_callback_call_loop1(
    operation: &str,
    mut parameter_count: f64,
    suffix: &str,
    mut clone_element: bool,
    mut arguments_text: String,
    mut position: f64,
    mut maximum: f64,
) -> String {
    loop {
        if (position < parameter_count) && (position < maximum) {
            {
                let arguments_text_3 = if position > 0f64 {
                    { format!("{arguments_text}, ") }
                } else {
                    arguments_text
                };
                {
                    let arguments_text_4 = format!(
                        "{arguments_text_3}{}",
                        self::render_array_callback_argument(
                            operation,
                            position,
                            suffix,
                            clone_element
                        )
                    );
                    {
                        let position_2 = position + 1f64;
                        {
                            (arguments_text, position) = (arguments_text_4, position_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { arguments_text };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_guarded_record_projection_positions_loop2(
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index >= 0f64 {
            {
                let result_2 = {
                    let source_ml_source2_concat_value3 = result;
                    {
                        let source_ml_source2_concat_value4 = vec![index];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value3,
                            ),
                            source_ml_source2_concat_value4,
                        )
                    }
                };
                {
                    let index_2 = index - 1f64;
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptOwnKeyCallbackPositionLoop3Result {
    MlSource1TemporaryReadJavaScriptOwnKeyCallbackPositionLoop3Done,
    MlSource1TemporaryReadJavaScriptOwnKeyCallbackPositionLoop3Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_own_key_callback_position_loop3(
    operation: &str,
    mut position: f64,
) -> self::MlSource1TemporaryReadJavaScriptOwnKeyCallbackPositionLoop3Result {
    loop {
        return if position < 4f64 {
            if (self::read_array_callback_parameter_role(operation, position)) == "element" {
                self::MlSource1TemporaryReadJavaScriptOwnKeyCallbackPositionLoop3Result::MlSource1TemporaryReadJavaScriptOwnKeyCallbackPositionLoop3Return(position)
            } else {
                {
                    let position_2 = position + 1f64;
                    {
                        position = position_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOwnKeyCallbackPositionLoop3Result::MlSource1TemporaryReadJavaScriptOwnKeyCallbackPositionLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptOwnKeyCallbackReceiverPositionLoop4Result {
    MlSource1TemporaryReadJavaScriptOwnKeyCallbackReceiverPositionLoop4Done,
    MlSource1TemporaryReadJavaScriptOwnKeyCallbackReceiverPositionLoop4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_own_key_callback_receiver_position_loop4(
    operation: &str,
    mut position: f64,
) -> self::MlSource1TemporaryReadJavaScriptOwnKeyCallbackReceiverPositionLoop4Result {
    loop {
        return if position < 4f64 {
            if (self::read_array_callback_parameter_role(operation, position)) == "receiver" {
                self::MlSource1TemporaryReadJavaScriptOwnKeyCallbackReceiverPositionLoop4Result::MlSource1TemporaryReadJavaScriptOwnKeyCallbackReceiverPositionLoop4Return(position)
            } else {
                {
                    let position_2 = position + 1f64;
                    {
                        position = position_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOwnKeyCallbackReceiverPositionLoop4Result::MlSource1TemporaryReadJavaScriptOwnKeyCallbackReceiverPositionLoop4Done
        };
    }
}
