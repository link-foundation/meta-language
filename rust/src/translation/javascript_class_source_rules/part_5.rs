// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptClosedClassDirectoryLoop15Result;
use super::find_java_script_class_group_end;
use super::find_java_script_closed_class_field_end;
use super::ml_array;
use super::ml_string;
use super::read_java_script_closed_class_parameter_positions;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop15(
    values: &[self::ml_string::Text],
    mut body_end: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop15Result {
    loop {
        return if at < body_end {
            if ((self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]))
            {
                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop15Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop15Return(vec![-6f64, at])
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop15Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop15Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Done(Vec<f64>),
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop16(
    values: &[self::ml_string::Text],
    fields: &[f64],
    name: &self::ml_string::Text,
    mut declaration: bool,
    mut field: f64,
) -> bool {
    loop {
        if field < (fields.len() as f64) {
            if (((self::ml_array::optional_at(fields, self::ml_array::number_index(field)))
                .unwrap_or_else(|| 0f64))
                < 0f64)
                && (((self::ml_array::optional_at(
                    values,
                    self::ml_array::number_index(
                        (-((self::ml_array::optional_at(
                            fields,
                            self::ml_array::number_index(field),
                        ))
                        .unwrap_or_else(|| 0f64)))
                            - 1f64,
                    ),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    == (*name))
            {
                {
                    let declaration_2 = true;
                    {
                        let field_2 = field + 3f64;
                        {
                            (declaration, field) = (declaration_2, field_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let field_3 = field + 3f64;
                    {
                        field = field_3;
                        continue;
                    }
                }
            }
        }
        return { declaration };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Done(f64),
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop19Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop19Done,
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop19Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop19(
    values: &[self::ml_string::Text],
    mut group: f64,
    mut inner: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop19Result {
    loop {
        return if inner < group {
            if ((((self::ml_array::optional_at(values, self::ml_array::number_index(inner)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))
                && (((self::ml_array::optional_at(
                    values,
                    self::ml_array::number_index(inner - 1f64),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    != (self::ml_string::Text::from_units(vec![46]))))
                && (((self::ml_array::optional_at(
                    values,
                    self::ml_array::number_index(inner - 1f64),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    != (self::ml_string::Text::from_units(vec![63, 46])))
            {
                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop19Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop19Return(vec![-5f64, inner])
            } else {
                {
                    let inner_2 = inner + 1f64;
                    {
                        inner = inner_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop19Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop19Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop18(
    values: &[self::ml_string::Text],
    mut body_end: f64,
    mut finish: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Result {
    loop {
        return if finish < body_end {
            {
                let value =
                    (self::ml_array::optional_at(values, self::ml_array::number_index(finish)))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if value == (self::ml_string::Text::from_units(vec![59])) {
                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Done(finish)
                } else if value == (self::ml_string::Text::from_units(vec![44])) {
                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Return(vec![-5f64, finish])
                } else if ((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))
                    && (((self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(finish - 1f64),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        != (self::ml_string::Text::from_units(vec![46]))))
                    && (((self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(finish - 1f64),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        != (self::ml_string::Text::from_units(vec![63, 46])))
                {
                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Return(vec![-5f64, finish])
                } else if ((value == (self::ml_string::Text::from_units(vec![40])))
                    || (value == (self::ml_string::Text::from_units(vec![91]))))
                    || (value == (self::ml_string::Text::from_units(vec![123])))
                {
                    {
                        let group = self::find_java_script_class_group_end(values, finish);
                        if (group < 0f64) || (group >= body_end) {
                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Return(vec![-5f64, finish])
                        } else {
                            {
                                let inner = finish + 1f64;
                                {
                                    let source_ml_source1_s133 = self::source_ml_temporary_read_java_script_closed_class_directory_loop19(values, group, inner);
                                    match source_ml_source1_s133 {
                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop19Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop19Done => {
                                                        {
                                                            let finish_2 = group;
                                                            {
                                                                let finish_3 = finish_2 + 1f64;
                                                                {
                                                                    finish = finish_3;
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop19Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop19Return(source_ml_temporary_result19) => {
                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Return(source_ml_temporary_result19)
                                                    }
                                                }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let finish_4 = finish + 1f64;
                        {
                            finish = finish_4;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Done(finish)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop17Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop17Done,
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop17Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop17(
    values: &[self::ml_string::Text],
    fields: &[f64],
    mut cursor: f64,
    name: &self::ml_string::Text,
    mut previous: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop17Result {
    loop {
        return if previous < (fields.len() as f64) {
            if (((self::ml_array::optional_at(fields, self::ml_array::number_index(previous)))
                .unwrap_or_else(|| -1f64))
                >= 0f64)
                && (((self::ml_array::optional_at(
                    values,
                    self::ml_array::number_index(
                        (self::ml_array::optional_at(
                            fields,
                            self::ml_array::number_index(previous),
                        ))
                        .unwrap_or_else(|| 0f64),
                    ),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                    == (*name))
            {
                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop17Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop17Return(vec![-4f64, (cursor + 2f64)])
            } else {
                {
                    let previous_2 = previous + 3f64;
                    {
                        previous = previous_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop17Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop17Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop13(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut fields: Vec<f64>,
    names: &[self::ml_string::Text],
    mut body_start: f64,
    mut body_end: f64,
    mut cursor: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Result {
    loop {
        return if (((((body_start >= 0f64) && (cursor < body_end))
            && (((self::ml_array::optional_at(values, self::ml_array::number_index(cursor)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))))
            && (((self::ml_array::optional_at(
                values,
                self::ml_array::number_index(cursor + 1f64),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![46]))))
            && (((self::ml_array::optional_at(
                kinds,
                self::ml_array::number_index(cursor + 2f64),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![
                    105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                ]))))
            && (((self::ml_array::optional_at(
                values,
                self::ml_array::number_index(cursor + 3f64),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![61])))
        {
            {
                let name = (self::ml_array::optional_at(
                    values,
                    self::ml_array::number_index(cursor + 2f64),
                ))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let declaration = false;
                    {
                        let field = 0f64;
                        {
                            let declaration_2 = self::source_ml_temporary_read_java_script_closed_class_directory_loop16(values, &(fields), &(name), declaration, field);
                            if (name == (self::ml_string::Text::from_units(vec![36])))
                                || (self::ml_array::includes(
                                    names,
                                    &(self::ml_string::Text::from_units(vec![
                                        105, 110, 115, 116, 97, 110, 99, 101, 58,
                                    ]))
                                    .concat(&name),
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                ) && !declaration_2)
                            {
                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Return(vec![-4f64, (cursor + 2f64)])
                            } else {
                                {
                                    let previous = 0f64;
                                    {
                                        let source_ml_source1_s134 = self::source_ml_temporary_read_java_script_closed_class_directory_loop17(values, &(fields), cursor, &(name), previous);
                                        match source_ml_source1_s134 {
                                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop17Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop17Done => {
                                                {
                                                    let start = cursor + 4f64;
                                                    {
                                                        let finish = start;
                                                        {
                                                            let source_ml_source1_s135 = self::source_ml_temporary_read_java_script_closed_class_directory_loop18(values, body_end, finish);
                                                            match source_ml_source1_s135 {
                                                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Done(finish_2) => {
                                                                    if (finish_2 == start) || (finish_2 >= body_end) {
                                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Return(vec![-5f64, start])
                                                                    } else {
                                                                        {
                                                                            let fields_2 = {
                                                                                let source_ml_source1_concat_value136 = fields;
                                                                                {
                                                                                    let source_ml_source1_concat_value137 = vec![(cursor + 2f64), start, finish_2];
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value136), source_ml_source1_concat_value137)
                                                                                }
                                                                            };
                                                                            {
                                                                                let cursor_2 = finish_2 + 1f64;
                                                                                {
                                                                                    (fields, cursor) = (fields_2, cursor_2);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop18Return(source_ml_temporary_result18) => {
                                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Return(source_ml_temporary_result18)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop17Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop17Return(source_ml_temporary_result19) => {
                                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Return(source_ml_temporary_result19)
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
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop13Done(fields)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result {
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Done(
        Vec<f64>,
        Vec<f64>,
        Vec<f64>,
        Vec<self::ml_string::Text>,
    ),
    MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassDirectoryJoin20 {
    MlTemporaryReadJavaScriptClosedClassDirectoryJoin20Next(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_java_script_closed_class_directory_loop12(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    mut end: f64,
    mut constructor: Vec<f64>,
    mut methods: Vec<f64>,
    mut fields: Vec<f64>,
    mut names: Vec<self::ml_string::Text>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result {
    loop {
        return if at < end {
            if ((self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (self::ml_string::Text::from_units(vec![59]))
            {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                {
                    let static_member = ((((self::ml_array::optional_at(
                        values,
                        self::ml_array::number_index(at),
                    ))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        == (self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99])))
                        && (((self::ml_array::optional_at(
                            kinds,
                            self::ml_array::number_index(at + 1f64),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                            == (self::ml_string::Text::from_units(vec![
                                105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                            ]))))
                        && (((self::ml_array::optional_at(
                            values,
                            self::ml_array::number_index(at + 2f64),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                            == (self::ml_string::Text::from_units(vec![40])));
                    {
                        let position = if static_member { at + 1f64 } else { at };
                        {
                            let name = (self::ml_array::optional_at(
                                values,
                                self::ml_array::number_index(position),
                            ))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                            {
                                let owner = (if static_member {
                                    self::ml_string::Text::from_units(vec![
                                        115, 116, 97, 116, 105, 99, 58,
                                    ])
                                } else {
                                    self::ml_string::Text::from_units(vec![
                                        105, 110, 115, 116, 97, 110, 99, 101, 58,
                                    ])
                                })
                                .concat(&name);
                                {
                                    let field = (((self::ml_array::optional_at(
                                        values,
                                        self::ml_array::number_index(position + 1f64),
                                    ))
                                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                                        == (self::ml_string::Text::from_units(vec![61])))
                                        || (((self::ml_array::optional_at(
                                            values,
                                            self::ml_array::number_index(position + 1f64),
                                        ))
                                        .unwrap_or_else(|| {
                                            self::ml_string::Text::from_units(vec![])
                                        })) == (self::ml_string::Text::from_units(vec![59])));
                                    if (!static_member && field)
                                        && (((self::ml_array::optional_at(
                                            kinds,
                                            self::ml_array::number_index(position),
                                        ))
                                        .unwrap_or_else(|| {
                                            self::ml_string::Text::from_units(vec![])
                                        })) == (self::ml_string::Text::from_units(vec![
                                            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                                        ])))
                                    {
                                        if self::ml_array::includes(
                                            &vec![
                                                self::ml_string::Text::from_units(vec![
                                                    99, 111, 110, 115, 116, 114, 117, 99, 116, 111,
                                                    114,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    95, 95, 112, 114, 111, 116, 111, 95, 95,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    112, 114, 111, 116, 111, 116, 121, 112, 101,
                                                ]),
                                                self::ml_string::Text::from_units(vec![36]),
                                            ],
                                            &name.clone(),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        ) || self::ml_array::includes(
                                            &names,
                                            &(self::ml_string::Text::from_units(vec![
                                                105, 110, 115, 116, 97, 110, 99, 101, 58,
                                            ]))
                                            .concat(&name),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        ) {
                                            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![-2f64, position])
                                        } else {
                                            {
                                                let initial = if ((self::ml_array::optional_at(
                                                    values,
                                                    self::ml_array::number_index(position + 1f64),
                                                ))
                                                .unwrap_or_else(|| {
                                                    self::ml_string::Text::from_units(vec![])
                                                }))
                                                    == (self::ml_string::Text::from_units(vec![61]))
                                                {
                                                    position + 2f64
                                                } else {
                                                    position + 1f64
                                                };
                                                {
                                                    let finish = self::find_java_script_closed_class_field_end(values, initial, end);
                                                    if (finish < 0f64)
                                                        || ((((self::ml_array::optional_at(
                                                            values,
                                                            self::ml_array::number_index(
                                                                position + 1f64,
                                                            ),
                                                        ))
                                                        .unwrap_or_else(|| {
                                                            self::ml_string::Text::from_units(
                                                                vec![],
                                                            )
                                                        }))
                                                            == (self::ml_string::Text::from_units(
                                                                vec![61],
                                                            )))
                                                            && (finish == initial))
                                                    {
                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![-10f64, initial])
                                                    } else {
                                                        {
                                                            let fields_2 = {
                                                                let source_ml_source1_concat_value138 =
                                                                    fields;
                                                                {
                                                                    let source_ml_source1_concat_value139 = vec![
                                                                        ((-position) - 1f64),
                                                                        initial,
                                                                        finish,
                                                                    ];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value138), source_ml_source1_concat_value139)
                                                                }
                                                            };
                                                            {
                                                                let names_2 = {
                                                                    let source_ml_source1_concat_value140 =
                                                                        names;
                                                                    {
                                                                        let source_ml_source1_concat_value141 = vec![(self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 58])).concat(&name)];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value140), source_ml_source1_concat_value141)
                                                                    }
                                                                };
                                                                {
                                                                    let at_3 = finish + 1f64;
                                                                    {
                                                                        (fields, names, at) = (
                                                                            fields_2, names_2, at_3,
                                                                        );
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else if ((((((((((((self::ml_array::optional_at(
                                        kinds,
                                        self::ml_array::number_index(position),
                                    ))
                                    .unwrap_or_else(|| {
                                        self::ml_string::Text::from_units(vec![])
                                    }))
                                        != (self::ml_string::Text::from_units(vec![
                                            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                                        ])))
                                        || (name
                                            == (self::ml_string::Text::from_units(vec![
                                                115, 116, 97, 116, 105, 99,
                                            ]))))
                                        || (name
                                            == (self::ml_string::Text::from_units(vec![
                                                103, 101, 116,
                                            ]))))
                                        || (name
                                            == (self::ml_string::Text::from_units(vec![
                                                115, 101, 116,
                                            ]))))
                                        || (name
                                            == (self::ml_string::Text::from_units(vec![
                                                97, 115, 121, 110, 99,
                                            ]))))
                                        || (name
                                            == (self::ml_string::Text::from_units(vec![
                                                95, 95, 112, 114, 111, 116, 111, 95, 95,
                                            ]))))
                                        || (name
                                            == (self::ml_string::Text::from_units(vec![
                                                112, 114, 111, 116, 111, 116, 121, 112, 101,
                                            ]))))
                                        || (name
                                            == (self::ml_string::Text::from_units(vec![36]))))
                                        || (static_member
                                            && (name
                                                == (self::ml_string::Text::from_units(vec![
                                                    99, 111, 110, 115, 116, 114, 117, 99, 116, 111,
                                                    114,
                                                ])))))
                                        || self::ml_array::includes(
                                            &names,
                                            &owner.clone(),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        ))
                                        || (((self::ml_array::optional_at(
                                            values,
                                            self::ml_array::number_index(position + 1f64),
                                        ))
                                        .unwrap_or_else(|| {
                                            self::ml_string::Text::from_units(vec![])
                                        })) != (self::ml_string::Text::from_units(vec![40])))
                                    {
                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![-2f64, at])
                                    } else {
                                        {
                                            let parameters = self::find_java_script_class_group_end(
                                                values,
                                                (position + 1f64),
                                            );
                                            if (parameters < 0f64)
                                                || (((self::ml_array::optional_at(
                                                    values,
                                                    self::ml_array::number_index(parameters + 1f64),
                                                ))
                                                .unwrap_or_else(|| {
                                                    self::ml_string::Text::from_units(vec![])
                                                })) != (self::ml_string::Text::from_units(
                                                    vec![123],
                                                )))
                                            {
                                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![-2f64, at])
                                            } else {
                                                {
                                                    let positions = self::read_java_script_closed_class_parameter_positions(values, kinds, (position + 1f64), parameters);
                                                    if ((self::ml_array::optional_at(
                                                        &positions,
                                                        self::ml_array::number_index(0f64),
                                                    ))
                                                    .unwrap_or_else(|| 0f64))
                                                        < 0f64
                                                    {
                                                        self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![-3f64, (self::ml_array::optional_at(&positions, self::ml_array::number_index(1f64))).unwrap_or_else(|| at)])
                                                    } else {
                                                        {
                                                            let body = self::find_java_script_class_group_end(values, (parameters + 1f64));
                                                            if (body < 0f64) || (body > end) {
                                                                self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Return(vec![-2f64, at])
                                                            } else {
                                                                {
                                                                    let names_3 = {
                                                                        let source_ml_source1_concat_value142 =
                                                                            names;
                                                                        {
                                                                            let source_ml_source1_concat_value143 =
                                                                                vec![owner.clone()];
                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source1_concat_value142), source_ml_source1_concat_value143)
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_s146 = if (name == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) && !static_member {
                                                                                {
                                                                                    let constructor_2 = vec![position, (position + 1f64), parameters, (parameters + 1f64), body];
                                                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin20::MlTemporaryReadJavaScriptClosedClassDirectoryJoin20Next(constructor_2, methods)
                                                                                }
                                                                            } else {
                                                                                {
                                                                                    let methods_2 = {
                                                                                        let source_ml_source1_concat_value147 = methods;
                                                                                        {
                                                                                            let source_ml_source1_concat_value148 = vec![if static_member {
                                                                                                (-position) - 1f64
                                                                                            } else {
                                                                                                position
                                                                                            }, (position + 1f64), parameters, (parameters + 1f64), body];
                                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value147), source_ml_source1_concat_value148)
                                                                                        }
                                                                                    };
                                                                                    self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin20::MlTemporaryReadJavaScriptClosedClassDirectoryJoin20Next(constructor, methods_2)
                                                                                }
                                                                            };
                                                                        match source_ml_source1_s146 {
                                                                                self::MlTemporaryReadJavaScriptClosedClassDirectoryJoin20::MlTemporaryReadJavaScriptClosedClassDirectoryJoin20Next(constructor_3, methods_3) => {
                                                                                    {
                                                                                        let at_4 = body + 1f64;
                                                                                        {
                                                                                            (constructor, methods, names, at) = (constructor_3, methods_3, names_3, at_4);
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
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlTemporaryReadJavaScriptClosedClassDirectoryLoop12Done(constructor, methods, fields, names)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryRenderJavaScriptClosedClassRangeLoop21Result {
    MlTemporaryRenderJavaScriptClosedClassRangeLoop21Done(self::ml_string::Text, f64),
}
