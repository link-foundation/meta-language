// Generated from complete js/src/translation/array-formal-parameter-rules.js. Do not edit by hand.
use super::MlSource1TemporaryFindJavaScriptFormalDefaultExpressionEndLoop11Result;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_formal_default_expression_end_loop11(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut start: f64,
    mut stack: Vec<self::ml_string::Text>,
    mut index: f64,
) -> self::MlSource1TemporaryFindJavaScriptFormalDefaultExpressionEndLoop11Result {
    loop {
        return if index < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                    self::MlSource1TemporaryFindJavaScriptFormalDefaultExpressionEndLoop11Result::MlSource1TemporaryFindJavaScriptFormalDefaultExpressionEndLoop11Return(-1f64)
                } else {
                    if kind != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let value = self::read_required_text_fact(values, index);
                            if ((stack.len() as f64) == 0f64)
                                && ((value == (self::ml_string::Text::from_units(vec![44])))
                                    || (value == (self::ml_string::Text::from_units(vec![93]))))
                            {
                                self::MlSource1TemporaryFindJavaScriptFormalDefaultExpressionEndLoop11Result::MlSource1TemporaryFindJavaScriptFormalDefaultExpressionEndLoop11Return(if (index).partial_cmp(&(start)) == Some(std::cmp::Ordering::Equal) {
                                    -1f64
                                } else {
                                    index
                                })
                            } else {
                                if ((value == (self::ml_string::Text::from_units(vec![40])))
                                    || (value == (self::ml_string::Text::from_units(vec![91]))))
                                    || (value == (self::ml_string::Text::from_units(vec![123])))
                                {
                                    {
                                        let stack_2 = {
                                            let source_ml_source2_concat_value101 = stack;
                                            {
                                                let source_ml_source2_concat_value102 =
                                                    vec![if value
                                                        == (self::ml_string::Text::from_units(
                                                            vec![40],
                                                        )) {
                                                        self::ml_string::Text::from_units(vec![41])
                                                    } else if value
                                                        == (self::ml_string::Text::from_units(
                                                            vec![91],
                                                        ))
                                                    {
                                                        self::ml_string::Text::from_units(vec![93])
                                                    } else {
                                                        self::ml_string::Text::from_units(vec![125])
                                                    }];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<self::ml_string::Text>::new(),
                                                        source_ml_source2_concat_value101,
                                                    ),
                                                    source_ml_source2_concat_value102,
                                                )
                                            }
                                        };
                                        {
                                            let index_3 = index + 1f64;
                                            {
                                                (stack, index) = (stack_2, index_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    if ((value == (self::ml_string::Text::from_units(vec![41])))
                                        || (value == (self::ml_string::Text::from_units(vec![93]))))
                                        || (value == (self::ml_string::Text::from_units(vec![125])))
                                    {
                                        if ((stack.len() as f64) == 0f64)
                                            || ((self::read_required_text_fact(
                                                &(stack),
                                                ((stack.len() as f64) - 1f64),
                                            )) != value)
                                        {
                                            self::MlSource1TemporaryFindJavaScriptFormalDefaultExpressionEndLoop11Result::MlSource1TemporaryFindJavaScriptFormalDefaultExpressionEndLoop11Return(-1f64)
                                        } else {
                                            {
                                                let stack_3 =
                                                    self::ml_array::slice(&stack, 0f64, -1f64);
                                                {
                                                    let index_4 = index + 1f64;
                                                    {
                                                        (stack, index) = (stack_3, index_4);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let index_5 = index + 1f64;
                                            {
                                                index = index_5;
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
            self::MlSource1TemporaryFindJavaScriptFormalDefaultExpressionEndLoop11Result::MlSource1TemporaryFindJavaScriptFormalDefaultExpressionEndLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Result {
    MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Done(f64, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_formal_default_binding_positions_loop13(
    plan: &[f64],
    mut cursor: f64,
    mut names: Vec<f64>,
    mut count: f64,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Result {
    loop {
        if index < count {
            {
                let names_2 = {
                    let source_ml_source2_concat_value103 = names;
                    {
                        let source_ml_source2_concat_value104 =
                            vec![self::read_required_number_fact(plan, (cursor + 2f64))];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value103,
                            ),
                            source_ml_source2_concat_value104,
                        )
                    }
                };
                {
                    let cursor_2 = cursor + 5f64;
                    {
                        let index_2 = index + 1f64;
                        {
                            (cursor, names, index) = (cursor_2, names_2, index_2);
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Result::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Done(cursor, names)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_formal_default_binding_positions_loop12(
    plan: &[f64],
    mut cursor: f64,
    mut names: Vec<f64>,
) -> Vec<f64> {
    loop {
        return if cursor < (plan.len() as f64) {
            {
                let form = self::read_required_number_fact(plan, cursor);
                {
                    let cursor_2 = cursor + 2f64;
                    if form == 0f64 {
                        {
                            let names_2 = {
                                let source_ml_source2_concat_value105 = names;
                                {
                                    let source_ml_source2_concat_value106 =
                                        vec![self::read_required_number_fact(plan, cursor_2)];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value105,
                                        ),
                                        source_ml_source2_concat_value106,
                                    )
                                }
                            };
                            {
                                let cursor_3 = cursor_2 + 1f64;
                                {
                                    (cursor, names) = (cursor_3, names_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if form == 2f64 {
                            {
                                let count =
                                    self::read_required_number_fact(plan, (cursor_2 + 2f64));
                                {
                                    let cursor_4 = cursor_2 + 3f64;
                                    {
                                        let index = 0f64;
                                        {
                                            let source_ml_source2_s107 = self::source_ml_source1_temporary_read_java_script_array_formal_default_binding_positions_loop13(plan, cursor_4, names, count, index);
                                            match source_ml_source2_s107 {
                                                self::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Result::MlSource1TemporaryReadJavaScriptArrayFormalDefaultBindingPositionsLoop13Done(cursor_5, names_3) => {
                                                    {
                                                        (cursor, names) = (cursor_5, names_3);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("RangeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 97, 114, 114, 97, 121, 32, 100, 101, 102, 97, 117,
                                    108, 116, 32, 98, 105, 110, 100, 105, 110, 103, 32, 99, 101,
                                    110, 115, 117, 115, 32, 110, 101, 101, 100, 115, 32, 97, 99,
                                    116, 117, 97, 108, 32, 102, 111, 114, 109, 97, 108, 32, 105,
                                    110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    }
                }
            }
        } else {
            names
        };
    }
}
