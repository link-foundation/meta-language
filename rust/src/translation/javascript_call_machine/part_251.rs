// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallArgumentList;
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptSemanticValue;
use super::accept_java_script_whitespace_unit;
use super::count_java_script_call_argument_list_values;
use super::index_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_radix_digit;
use super::read_java_script_semantic_text_unit;
use super::test_java_script_semantic_same_value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReplaceUtf16TextLoop1305Result {
    MlSource1TemporaryReplaceUtf16TextLoop1305Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReplaceUtf16TextLoop1306Result {
    MlSource1TemporaryReplaceUtf16TextLoop1306Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_replace_utf16_text_loop1306(
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut last: f64,
    mut source: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryReplaceUtf16TextLoop1306Result {
    loop {
        return if (at < last) {
            {
                let source_ml_source1_temporary_replace_utf16_text_ignored1307 = {
                    let ml_method_owner3085 = result.clone();
                    let ml_method_argument3085_0 = ({
                        let ml_reference_array3086 = source.clone();
                        let ml_reference_position3086 = at;
                        let ml_reference_projection3086 = self::ml_array::optional_at(
                            &*ml_reference_array3086.borrow(),
                            self::ml_array::number_index(ml_reference_position3086),
                        );
                        ml_reference_projection3086
                    })
                    .clone()
                    .unwrap_or_else(|| 0f64);
                    let ml_inserted = vec![ml_method_argument3085_0];
                    let mut ml_method_values3085 = ml_method_owner3085.borrow_mut();
                    let ml_length = ml_method_values3085
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3085.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReplaceUtf16TextLoop1306Result::MlSource1TemporaryReplaceUtf16TextLoop1306Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_replace_utf16_text_loop1305(
    mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut plan: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut found: f64,
    mut instruction: f64,
) -> self::MlSource1TemporaryReplaceUtf16TextLoop1305Result {
    loop {
        return if (instruction
            < ({
                let ml_reference_array3087 = plan.clone();
                let ml_reference_projection3087 = ml_reference_array3087.borrow().len() as f64;
                ml_reference_projection3087
            })) {
            {
                let kind = ({
                    let ml_reference_array3088 = plan.clone();
                    let ml_reference_position3088 = instruction;
                    let ml_reference_projection3088 = self::ml_array::optional_at(
                        &*ml_reference_array3088.borrow(),
                        self::ml_array::number_index(ml_reference_position3088),
                    );
                    ml_reference_projection3088
                })
                .clone()
                .unwrap_or_else(|| 0f64);
                {
                    let first = if (kind == (0f64)) {
                        ({
                            let source_ml_source2_array_read_index41990 = (instruction + 1f64);
                            {
                                let ml_reference_array3089 = plan.clone();
                                let ml_reference_position3089 =
                                    source_ml_source2_array_read_index41990;
                                let ml_reference_projection3089 = self::ml_array::optional_at(
                                    &*ml_reference_array3089.borrow(),
                                    self::ml_array::number_index(ml_reference_position3089),
                                );
                                ml_reference_projection3089
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| 0f64)
                    } else {
                        if (kind == (2f64)) {
                            0f64
                        } else {
                            if (kind == (1f64)) {
                                found
                            } else {
                                (found + {
                                    let ml_reference_array3090 = search.clone();
                                    let ml_reference_projection3090 =
                                        ml_reference_array3090.borrow().len() as f64;
                                    ml_reference_projection3090
                                })
                            }
                        }
                    };
                    {
                        let last = if (kind == (0f64)) {
                            ({
                                let source_ml_source2_array_read_index41992 = (instruction + 2f64);
                                {
                                    let ml_reference_array3091 = plan.clone();
                                    let ml_reference_position3091 =
                                        source_ml_source2_array_read_index41992;
                                    let ml_reference_projection3091 = self::ml_array::optional_at(
                                        &*ml_reference_array3091.borrow(),
                                        self::ml_array::number_index(ml_reference_position3091),
                                    );
                                    ml_reference_projection3091
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| 0f64)
                        } else {
                            if (kind == (2f64)) {
                                found
                            } else {
                                if (kind == (1f64)) {
                                    (found + {
                                        let ml_reference_array3092 = search.clone();
                                        let ml_reference_projection3092 =
                                            ml_reference_array3092.borrow().len() as f64;
                                        ml_reference_projection3092
                                    })
                                } else {
                                    {
                                        let ml_reference_array3093 = text.clone();
                                        let ml_reference_projection3093 =
                                            ml_reference_array3093.borrow().len() as f64;
                                        ml_reference_projection3093
                                    }
                                }
                            }
                        };
                        {
                            let source = if (kind == (0f64)) {
                                replacement.clone()
                            } else {
                                text.clone()
                            };
                            {
                                let source_ml_source2_s41993 =
                                    self::source_ml_source1_temporary_replace_utf16_text_loop1306(
                                        result.clone(),
                                        last,
                                        source.clone(),
                                        first,
                                    );
                                match source_ml_source2_s41993.clone() {
                                    self::MlSource1TemporaryReplaceUtf16TextLoop1306Result::MlSource1TemporaryReplaceUtf16TextLoop1306Done => {
                                        {
                                            let instruction_2 = (instruction + 3f64);
                                            {
                                                instruction = instruction_2;
                                                continue;
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
            self::MlSource1TemporaryReplaceUtf16TextLoop1305Result::MlSource1TemporaryReplaceUtf16TextLoop1305Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReplaceUtf16TextLoop1304Result {
    MlSource1TemporaryReplaceUtf16TextLoop1304Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_replace_utf16_text_loop1304(
    mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut found: f64,
    mut at: f64,
) -> self::MlSource1TemporaryReplaceUtf16TextLoop1304Result {
    loop {
        return if (at < found) {
            {
                let source_ml_source1_temporary_replace_utf16_text_ignored1308 = {
                    let ml_method_owner3094 = result.clone();
                    let ml_method_argument3094_0 = ({
                        let ml_reference_array3095 = text.clone();
                        let ml_reference_position3095 = at;
                        let ml_reference_projection3095 = self::ml_array::optional_at(
                            &*ml_reference_array3095.borrow(),
                            self::ml_array::number_index(ml_reference_position3095),
                        );
                        ml_reference_projection3095
                    })
                    .clone()
                    .unwrap_or_else(|| 0f64);
                    let ml_inserted = vec![ml_method_argument3094_0];
                    let mut ml_method_values3094 = ml_method_owner3094.borrow_mut();
                    let ml_length = ml_method_values3094
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3094.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReplaceUtf16TextLoop1304Result::MlSource1TemporaryReplaceUtf16TextLoop1304Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_replace_utf16_text_loop1303(
    mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut all: bool,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut plan: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut from: f64,
    mut searching: bool,
) -> f64 {
    loop {
        return if searching {
            {
                let found = self::index_utf16_text(text.clone(), search.clone(), from);
                if (found < (0f64)) {
                    {
                        searching = false;
                        continue;
                    }
                } else {
                    {
                        let at = start;
                        {
                            let source_ml_source2_s41996 =
                                self::source_ml_source1_temporary_replace_utf16_text_loop1304(
                                    text.clone(),
                                    result.clone(),
                                    found,
                                    at,
                                );
                            match source_ml_source2_s41996.clone() {
                                self::MlSource1TemporaryReplaceUtf16TextLoop1304Result::MlSource1TemporaryReplaceUtf16TextLoop1304Done => {
                                    {
                                        let instruction = 0f64;
                                        {
                                            let source_ml_source2_s41997 = self::source_ml_source1_temporary_replace_utf16_text_loop1305(text.clone(), search.clone(), replacement.clone(), result.clone(), plan.clone(), found, instruction);
                                            match source_ml_source2_s41997.clone() {
                                                self::MlSource1TemporaryReplaceUtf16TextLoop1305Result::MlSource1TemporaryReplaceUtf16TextLoop1305Done => {
                                                    {
                                                        let start_2 = (found + { let ml_reference_array3096 = search.clone(); let ml_reference_projection3096 = ml_reference_array3096.borrow().len() as f64; ml_reference_projection3096 });
                                                        {
                                                            let searching_2 = (all && !(((({ let ml_reference_array3097 = search.clone(); let ml_reference_projection3097 = ml_reference_array3097.borrow().len() as f64; ml_reference_projection3097 }) == (0f64)) && (found == ({ let ml_reference_array3098 = text.clone(); let ml_reference_projection3098 = ml_reference_array3098.borrow().len() as f64; ml_reference_projection3098 })))));
                                                            {
                                                                let from_2 = (start_2 + if (({ let ml_reference_array3099 = search.clone(); let ml_reference_projection3099 = ml_reference_array3099.borrow().len() as f64; ml_reference_projection3099 }) == (0f64)) {
                                                                    1f64
                                                                } else {
                                                                    0f64
                                                                });
                                                                {
                                                                    (start, from, searching) = (start_2, from_2, searching_2);
                                                                    continue;
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
        } else {
            start
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReplaceUtf16TextLoop1309Result {
    MlSource1TemporaryReplaceUtf16TextLoop1309Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_replace_utf16_text_loop1309(
    mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryReplaceUtf16TextLoop1309Result {
    loop {
        return if (at
            < ({
                let ml_reference_array3100 = text.clone();
                let ml_reference_projection3100 = ml_reference_array3100.borrow().len() as f64;
                ml_reference_projection3100
            })) {
            {
                let source_ml_source1_temporary_replace_utf16_text_ignored1310 = {
                    let ml_method_owner3101 = result.clone();
                    let ml_method_argument3101_0 = ({
                        let ml_reference_array3102 = text.clone();
                        let ml_reference_position3102 = at;
                        let ml_reference_projection3102 = self::ml_array::optional_at(
                            &*ml_reference_array3102.borrow(),
                            self::ml_array::number_index(ml_reference_position3102),
                        );
                        ml_reference_projection3102
                    })
                    .clone()
                    .unwrap_or_else(|| 0f64);
                    let ml_inserted = vec![ml_method_argument3101_0];
                    let mut ml_method_values3101 = ml_method_owner3101.borrow_mut();
                    let ml_length = ml_method_values3101
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3101.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReplaceUtf16TextLoop1309Result::MlSource1TemporaryReplaceUtf16TextLoop1309Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop1311(
    mut receiver_text: self::JavaScriptSemanticValue,
    mut search_text: self::JavaScriptSemanticValue,
    mut search_length: f64,
    mut start: f64,
    mut matches: bool,
    mut at: f64,
) -> bool {
    loop {
        return if (matches && (at < search_length)) {
            if ((self::read_java_script_semantic_text_unit(receiver_text.clone(), (start + at)))
                != (self::read_java_script_semantic_text_unit(search_text.clone(), at)))
            {
                {
                    let at_2 = (at + 1f64);
                    {
                        (matches, at) = (false, at_2);
                        continue;
                    }
                }
            } else {
                {
                    let at_3 = (at + 1f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            matches
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_call_argument_lists_update_loop1312(
    mut machine: self::JavaScriptCallMachine,
    mut updated_lists: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>,
    mut count: f64,
    mut position: f64,
) -> f64 {
    loop {
        return match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                if (position
                    < ({
                        let ml_reference_array3103 = shared_constructor_field1.clone();
                        let ml_reference_projection3103 =
                            ml_reference_array3103.borrow().len() as f64;
                        ml_reference_projection3103
                    }))
                {
                    {
                        let active = {
                            let source_ml_source2_array_read_value42003 =
                                shared_constructor_field1.clone();
                            {
                                let ml_reference_array3104 =
                                    source_ml_source2_array_read_value42003.clone();
                                let ml_reference_position3104 = position;
                                let ml_reference_projection3104 = self::ml_array::optional_at(
                                    &*ml_reference_array3104.borrow(),
                                    self::ml_array::number_index(ml_reference_position3104),
                                );
                                ml_reference_projection3104
                            }
                        };
                        {
                            let count_2 = (count + {
                                let ml_reference_array3105 = {
                                    let source_ml_source2_nullish_field_receiver42005 =
                                        (active.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver42005.clone())
                                        .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source2_nullish_field_present42006 =
                                                ((source_ml_source2_nullish_field_receiver42005
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source2_nullish_field_present42006
                                                .clone()
                                            {
                                                self::JavaScriptCallFrame::CallFrame(
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source2_data_property_value42007,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => source_ml_source2_data_property_value42007
                                                    .clone(),
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97,
                                                    100, 32, 112, 114, 111, 112, 101, 114, 116,
                                                    105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                                                    101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                                    97, 100, 105, 110, 103, 32, 39, 97, 114, 103,
                                                    117, 109, 101, 110, 116, 115, 39, 41,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    }
                                };
                                let ml_reference_projection3105 =
                                    ml_reference_array3105.borrow().len() as f64;
                                ml_reference_projection3105
                            });
                            {
                                let active_lists = {
                                    let source_ml_source2_nullish_field_receiver42008 =
                                        (active.clone(),);
                                    if match (source_ml_source2_nullish_field_receiver42008.clone())
                                        .0
                                    {
                                        Some(ml_present) => {
                                            let _ = ml_present.clone();
                                            true
                                        }
                                        None => false,
                                    } {
                                        {
                                            let source_ml_source2_nullish_field_present42009 =
                                                ((source_ml_source2_nullish_field_receiver42008
                                                    .clone())
                                                .0)
                                                    .clone()
                                                    .expect("a proven present value");
                                            match source_ml_source2_nullish_field_present42009.clone() {
                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, source_ml_source2_data_property_value42010, _, _) => {
                                                    ((source_ml_source2_data_property_value42010.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>>,))).0).clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97,
                                                    100, 32, 112, 114, 111, 112, 101, 114, 116,
                                                    105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                                                    101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                                    97, 100, 105, 110, 103, 32, 39, 97, 114, 103,
                                                    117, 109, 101, 110, 116, 76, 105, 115, 116,
                                                    115, 39, 41,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    }
                                };
                                if (position
                                    == ({
                                        let ml_reference_array3106 =
                                            shared_constructor_field1.clone();
                                        let ml_reference_projection3106 =
                                            ml_reference_array3106.borrow().len() as f64;
                                        ml_reference_projection3106
                                    } - 1f64))
                                {
                                    {
                                        let count_3 = (count_2
                                            + self::count_java_script_call_argument_list_values(
                                                updated_lists.clone(),
                                            ));
                                        {
                                            let position_2 = (position + 1f64);
                                            {
                                                (count, position) = (count_3, position_2);
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    if !(match (active_lists.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        {
                                            let count_4 = (count_2 + self::count_java_script_call_argument_list_values(active_lists.clone().expect("a proven present value")));
                                            {
                                                let position_3 = (position + 1f64);
                                                {
                                                    (count, position) = (count_4, position_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let position_4 = (position + 1f64);
                                            {
                                                (count, position) = (count_2, position_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    count
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallObjectInstructionJoin1317 {
    MlSource1TemporaryExecuteJavaScriptCallObjectInstructionJoin1317Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallPendingObjectObservationErrorLoop1323Result {
    MlSource1TemporaryReadJavaScriptCallPendingObjectObservationErrorLoop1323Done,
    MlSource1TemporaryReadJavaScriptCallPendingObjectObservationErrorLoop1323Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_pending_object_observation_error_loop1323(
    mut frame: self::JavaScriptCallFrame,
    mut excluded: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut excluded_values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptCallPendingObjectObservationErrorLoop1323Result {
    loop {
        return if (at
            < ({
                let ml_reference_array3107 = excluded.clone();
                let ml_reference_projection3107 = ml_reference_array3107.borrow().len() as f64;
                ml_reference_projection3107
            })) {
            if !(self::test_java_script_semantic_same_value(
                ({
                    let ml_reference_array3108 = excluded_values.clone();
                    let ml_reference_position3108 = at;
                    let ml_reference_projection3108 = self::ml_array::optional_at(
                        &*ml_reference_array3108.borrow(),
                        self::ml_array::number_index(ml_reference_position3108),
                    );
                    ml_reference_projection3108
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                ({
                    let source_ml_source2_array_read_value42018 = match frame.clone() {
                        self::JavaScriptCallFrame::CallFrame(
                            _,
                            _,
                            source_ml_source2_data_property_value42015,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value42015.clone(),
                    };
                    {
                        let source_ml_source2_array_read_index42019 = ({
                            let ml_reference_array3109 = excluded.clone();
                            let ml_reference_position3109 = at;
                            let ml_reference_projection3109 = self::ml_array::optional_at(
                                &*ml_reference_array3109.borrow(),
                                self::ml_array::number_index(ml_reference_position3109),
                            );
                            ml_reference_projection3109
                        })
                        .clone()
                        .unwrap_or_else(|| (-1f64));
                        {
                            let ml_reference_array3110 =
                                source_ml_source2_array_read_value42018.clone();
                            let ml_reference_position3110 = source_ml_source2_array_read_index42019;
                            let ml_reference_projection3110 = self::ml_array::optional_at(
                                &*ml_reference_array3110.borrow(),
                                self::ml_array::number_index(ml_reference_position3110),
                            );
                            ml_reference_projection3110
                        }
                    }
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
            )) {
                self::MlSource1TemporaryReadJavaScriptCallPendingObjectObservationErrorLoop1323Result::MlSource1TemporaryReadJavaScriptCallPendingObjectObservationErrorLoop1323Return(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 99, 111, 112, 121, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 97, 110, 32, 101, 120, 99, 108, 117, 100, 101, 100, 32, 107, 101, 121]))
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallPendingObjectObservationErrorLoop1323Result::MlSource1TemporaryReadJavaScriptCallPendingObjectObservationErrorLoop1323Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327 {
    MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Next,
    MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328 {
    MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328Next,
    MlSource1TemporaryReadJavaScriptCallPendingCallableConstructionErrorJoin1328Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329 {
    MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329Next,
    MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330 {
    MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330Next,
    MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331 {
    MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331Next,
    MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333 {
    MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333Next,
    MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335 {
    MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335Next,
    MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1335Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334 {
    MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Next,
    MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1334Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336 {
    MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336Next,
    MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1336Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337 {
    MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337Next,
    MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1337Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338 {
    MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338Next,
    MlSource1TemporaryReadJavaScriptCallPendingConversionErrorJoin1338Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_parse_java_script_integer_prefix_loop1341(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
) -> f64 {
    loop {
        return if ((start
            < ({
                let ml_reference_array3111 = units.clone();
                let ml_reference_projection3111 = ml_reference_array3111.borrow().len() as f64;
                ml_reference_projection3111
            }))
            && self::accept_java_script_whitespace_unit(
                ({
                    let ml_reference_array3112 = units.clone();
                    let ml_reference_position3112 = start;
                    let ml_reference_projection3112 = self::ml_array::optional_at(
                        &*ml_reference_array3112.borrow(),
                        self::ml_array::number_index(ml_reference_position3112),
                    );
                    ml_reference_projection3112
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryParseJavaScriptIntegerPrefixLoop1344Result {
    MlSource1TemporaryParseJavaScriptIntegerPrefixLoop1344Done(self::ml::Big, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_parse_java_script_integer_prefix_loop1344(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut radix: f64,
    mut value: self::ml::Big,
    mut count: f64,
    mut base: self::ml::Big,
    mut at: f64,
) -> self::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop1344Result {
    loop {
        return if (at
            < ({
                let ml_reference_array3113 = units.clone();
                let ml_reference_projection3113 = ml_reference_array3113.borrow().len() as f64;
                ml_reference_projection3113
            })) {
            {
                let digit = self::read_java_script_radix_digit(
                    ({
                        let ml_reference_array3114 = units.clone();
                        let ml_reference_position3114 = at;
                        let ml_reference_projection3114 = self::ml_array::optional_at(
                            &*ml_reference_array3114.borrow(),
                            self::ml_array::number_index(ml_reference_position3114),
                        );
                        ml_reference_projection3114
                    })
                    .clone()
                    .unwrap_or_else(|| (-1f64)),
                );
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop1344Result::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop1344Done(value.clone(), count)
                } else {
                    {
                        let value_2 = (value.mul(&base)).add(&{
                            let source_ml_source2_integer_result42025 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(digit);
                            if (((source_ml_source2_integer_result42025.len() as f64)) == (1f64)) {
                                (self::ml_array::optional_at(&source_ml_source2_integer_result42025, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
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
            self::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop1344Result::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop1344Done(value.clone(), count)
        };
    }
}
