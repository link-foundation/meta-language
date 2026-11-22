// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin39;
use super::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result;
use super::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result;
use super::accept_java_script_closed_class_function_export;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_java_script_closed_class_methods;
use super::read_java_script_closed_class_static_target;
use super::render_java_script_closed_class_parameter_arguments;
use super::render_java_script_closed_class_range;
use super::source_ml_temporary_read_java_script_closed_class_observation_error_loop40;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop38(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    directory: &[Vec<f64>],
    static_calls: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    methods: &[self::ml_string::Text],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result {
    loop {
        return if at < (values.len() as f64) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let kind =
                        (self::ml_array::optional_at(kinds, self::ml_array::number_index(at)))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    {
                        let previous = (self::ml_array::optional_at(
                            values,
                            self::ml_array::number_index(at - 1f64),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        {
                            let next = (self::ml_array::optional_at(
                                values,
                                self::ml_array::number_index(at + 1f64),
                            ))
                            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                            if (kind
                                == (self::ml_string::Text::from_units(vec![
                                    105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                                ])))
                                && self::ml_array::includes(
                                    &vec![
                                        self::ml_string::Text::from_units(vec![
                                            79, 98, 106, 101, 99, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            82, 101, 102, 108, 101, 99, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![74, 83, 79, 78]),
                                        self::ml_string::Text::from_units(vec![
                                            83, 121, 109, 98, 111, 108,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            80, 114, 111, 120, 121,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            70, 117, 110, 99, 116, 105, 111, 110,
                                        ]),
                                        self::ml_string::Text::from_units(vec![101, 118, 97, 108]),
                                        self::ml_string::Text::from_units(vec![
                                            97, 114, 103, 117, 109, 101, 110, 116, 115,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            115, 117, 112, 101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            100, 101, 101, 112, 69, 113, 117, 97, 108,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            110, 111, 116, 68, 101, 101, 112, 69, 113, 117, 97, 108,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            100, 101, 101, 112, 83, 116, 114, 105, 99, 116, 69,
                                            113, 117, 97, 108,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            110, 111, 116, 68, 101, 101, 112, 83, 116, 114, 105,
                                            99, 116, 69, 113, 117, 97, 108,
                                        ]),
                                    ],
                                    &value.clone(),
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                )
                            {
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                            } else if ((kind
                                == (self::ml_string::Text::from_units(vec![
                                    105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                                ])))
                                && (value
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 120, 112, 111, 114, 116,
                                    ]))))
                                && !(self::accept_java_script_closed_class_function_export(
                                    values, kinds, at,
                                ))
                            {
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                            } else if ((value
                                == (self::ml_string::Text::from_units(vec![46, 46, 46])))
                                || (value
                                    == (self::ml_string::Text::from_units(vec![
                                        95, 95, 112, 114, 111, 116, 111, 95, 95,
                                    ]))))
                                || (value
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ])))
                            {
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                            } else {
                                {
                                    let source_ml_source1_s212 = if value
                                        == (self::ml_string::Text::from_units(vec![
                                            102, 117, 110, 99, 116, 105, 111, 110,
                                        ])) {
                                        {
                                            let source_ml_temporary_values36 = directory.to_vec();
                                            {
                                                let source_ml_temporary_index36 =
                                                    self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s213 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop40(at, &(source_ml_temporary_values36), source_ml_temporary_index36);
                                                    match source_ml_source1_s213 {
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Done => {
                                                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin39Next
                                                                }
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(source_ml_temporary_result40) => {
                                                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin39Return(source_ml_temporary_result40)
                                                                }
                                                            }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin39Next
                                    };
                                    match source_ml_source1_s212 {
                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin39Next => {
                                                    if ((((kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && self::ml_array::includes(names, &value.clone(), 0f64, false, |a, b| a == b)) && (previous != (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))) && ((previous != (self::ml_string::Text::from_units(vec![110, 101, 119]))) || (next != (self::ml_string::Text::from_units(vec![40]))))) && ((self::read_java_script_closed_class_static_target(values, names, static_calls, at)) == (self::ml_string::Text::from_units(vec![]))) {
                                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                                    } else if ((((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (previous != (self::ml_string::Text::from_units(vec![46])))) && (previous != (self::ml_string::Text::from_units(vec![63, 46])))) && (next != (self::ml_string::Text::from_units(vec![46])))) && (next != (self::ml_string::Text::from_units(vec![91]))) {
                                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                                        } else if ((previous == (self::ml_string::Text::from_units(vec![46]))) || (previous == (self::ml_string::Text::from_units(vec![63, 46])))) && ((self::ml_array::includes(methods, &value.clone(), 0f64, false, |a, b| a == b) && (next != (self::ml_string::Text::from_units(vec![40])))) || (value == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])))) {
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                                            } else if ((previous == (self::ml_string::Text::from_units(vec![91]))) && (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) && self::ml_array::includes(methods, &{
                                                                    let source_ml_source1_string_range_operand214 = value.clone();
                                                                    {
                                                                        let source_ml_source1_string_range_operand215 = 1f64;
                                                                        {
                                                                            let source_ml_source1_string_range_position216 = source_ml_source1_string_range_operand215;
                                                                            {
                                                                                let source_ml_source1_string_range_position217 = f64::INFINITY;
                                                                                source_ml_source1_string_range_operand214.slice(source_ml_source1_string_range_position216, source_ml_source1_string_range_position217)
                                                                            }
                                                                        }
                                                                    }
                                                                }, 0f64, false, |a, b| a == b) {
                                                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(at)
                                                                } else {
                                                                    {
                                                                        let at_2 = at + 1f64;
                                                                        {
                                                                            at = at_2;
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                }
                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin39::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin39Return(source_ml_temporary_result39) => {
                                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(source_ml_temporary_result39)
                                                }
                                            }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop38Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop37Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop37Done(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
    ),
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop37Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop41Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop41Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop41Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop42(
    values: &[self::ml_string::Text],
    row: &[f64],
    mut at: f64,
    mut inner: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Result {
    loop {
        return if inner
            < ((self::ml_array::optional_at(row, self::ml_array::number_index(at + 4f64)))
                .unwrap_or_else(|| 0f64))
        {
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
                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Return(inner)
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
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop41(
    values: &[self::ml_string::Text],
    row: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop41Result {
    loop {
        return if at < (row.len() as f64) {
            if ((self::ml_array::optional_at(row, self::ml_array::number_index(at)))
                .unwrap_or_else(|| 0f64))
                < 0f64
            {
                {
                    let inner =
                        (self::ml_array::optional_at(row, self::ml_array::number_index(at + 3f64)))
                            .unwrap_or_else(|| 0f64)
                            + 1f64;
                    {
                        let source_ml_source1_s218 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop42(values, row, at, inner);
                        match source_ml_source1_s218 {
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Done => {
                                {
                                    let at_2 = at + 5f64;
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Return(source_ml_temporary_result42) => {
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop41Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop41Return(source_ml_temporary_result42)
                            }
                        }
                    }
                }
            } else {
                {
                    let at_3 = at + 5f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop41Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop41Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop37(
    values: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    mut methods: Vec<self::ml_string::Text>,
    source_ml_temporary_values35: &[Vec<f64>],
    mut source_ml_temporary_index35: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop37Result {
    loop {
        return if source_ml_temporary_index35
            < (self::ml::Big::from_u128(source_ml_temporary_values35.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values35,
                    source_ml_temporary_index35.to_index(),
                );
                {
                    let names_2 = {
                        let source_ml_source1_concat_value219 = names;
                        {
                            let source_ml_source1_concat_value220 = vec![
                                (self::ml_array::optional_at(
                                    values,
                                    self::ml_array::number_index(
                                        (self::ml_array::optional_at(
                                            &row,
                                            self::ml_array::number_index(0f64),
                                        ))
                                        .unwrap_or_else(|| 0f64)
                                            + 1f64,
                                    ),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<self::ml_string::Text>::new(),
                                    source_ml_source1_concat_value219,
                                ),
                                source_ml_source1_concat_value220,
                            )
                        }
                    };
                    {
                        let methods_2 = {
                            let source_ml_source1_concat_value221 = methods;
                            {
                                let source_ml_source1_concat_value222 =
                                    self::read_java_script_closed_class_methods(values, &(row));
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value221,
                                    ),
                                    source_ml_source1_concat_value222,
                                )
                            }
                        };
                        {
                            let at = 9f64
                                + (3f64
                                    * (self::ml_array::optional_at(
                                        &row,
                                        self::ml_array::number_index(7f64),
                                    ))
                                    .unwrap_or_else(|| 0f64));
                            {
                                let source_ml_source1_s223 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop41(values, &(row), at);
                                match source_ml_source1_s223 {
                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop41Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop41Done => {
                                        {
                                            let source_ml_temporary_index35_2 = source_ml_temporary_index35.add(&self::ml::Big::from_i128(1));
                                            {
                                                (names, methods, source_ml_temporary_index35) = (names_2, methods_2, source_ml_temporary_index35_2);
                                                continue;
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop41Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop41Return(source_ml_temporary_result41) => {
                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop37Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop37Return(source_ml_temporary_result41)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop37Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop37Done(names, methods)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderJavaScriptClosedClassLoop44Result {
    MlTemporaryRenderJavaScriptClosedClassLoop44Done(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_render_java_script_closed_class_loop44(
    source: &self::ml_string::Text,
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    row: &[f64],
    prefix: &self::ml_string::Text,
    mut ordinal: f64,
    class_names: &[self::ml_string::Text],
    static_calls: &[self::ml_string::Text],
    mut prototype: bool,
    receiver: &self::ml_string::Text,
    mut stubs: self::ml_string::Text,
    mut fields: self::ml_string::Text,
    mut assignments: self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryRenderJavaScriptClosedClassLoop44Result {
    loop {
        if at < (row.len() as f64) {
            {
                let member = (self::ml_array::optional_at(row, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| 0f64);
                {
                    let position = if member < 0f64 {
                        (-member) - 1f64
                    } else {
                        member
                    };
                    {
                        let method = (self::ml_array::optional_at(
                            values,
                            self::ml_array::number_index(position),
                        ))
                        .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        {
                            let args = self::render_java_script_closed_class_range(
                                source,
                                values,
                                starts,
                                ends,
                                ((self::ml_array::optional_at(
                                    row,
                                    self::ml_array::number_index(at + 1f64),
                                ))
                                .unwrap_or_else(|| 0f64)
                                    + 1f64),
                                (self::ml_array::optional_at(
                                    row,
                                    self::ml_array::number_index(at + 2f64),
                                ))
                                .unwrap_or_else(|| 0f64),
                                &(self::ml_string::Text::from_units(vec![])),
                                class_names,
                                static_calls,
                            );
                            {
                                let arguments_ =
                                    self::render_java_script_closed_class_parameter_arguments(
                                        source,
                                        values,
                                        starts,
                                        ends,
                                        (self::ml_array::optional_at(
                                            row,
                                            self::ml_array::number_index(at + 1f64),
                                        ))
                                        .unwrap_or_else(|| 0f64),
                                        (self::ml_array::optional_at(
                                            row,
                                            self::ml_array::number_index(at + 2f64),
                                        ))
                                        .unwrap_or_else(|| 0f64),
                                    );
                                {
                                    let body = self::render_java_script_closed_class_range(
                                        source,
                                        values,
                                        starts,
                                        ends,
                                        ((self::ml_array::optional_at(
                                            row,
                                            self::ml_array::number_index(at + 3f64),
                                        ))
                                        .unwrap_or_else(|| 0f64)
                                            + 1f64),
                                        (self::ml_array::optional_at(
                                            row,
                                            self::ml_array::number_index(at + 4f64),
                                        ))
                                        .unwrap_or_else(|| 0f64),
                                        &(if member < 0f64 {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            receiver.clone()
                                        }),
                                        class_names,
                                        static_calls,
                                    );
                                    {
                                        let stub = (((prefix.concat(
                                            &self::ml_string::Text::from_units(vec![
                                                99, 108, 97, 115, 115, 95, 117, 110, 105, 110, 105,
                                                116, 105, 97, 108, 105, 122, 101, 100, 95,
                                            ]),
                                        ))
                                        .concat(&{
                                            let source_ml_source1_string_argument224 = ordinal;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument224,
                                                ),
                                            )
                                        }))
                                        .concat(&self::ml_string::Text::from_units(vec![95])))
                                        .concat(&{
                                            let source_ml_source1_string_argument225 = at;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument225,
                                                ),
                                            )
                                        });
                                        {
                                            let declaration = if member < 0f64 {
                                                position - 1f64
                                            } else {
                                                position
                                            };
                                            {
                                                let documentation = {
                                                    let source_ml_source1_string_range_operand226 =
                                                        source.clone();
                                                    {
                                                        let source_ml_source1_string_range_operand227 =
                                                            (self::ml_array::optional_at(
                                                                ends,
                                                                self::ml_array::number_index(
                                                                    declaration - 1f64,
                                                                ),
                                                            ))
                                                            .unwrap_or_else(|| 0f64);
                                                        {
                                                            let source_ml_source1_string_range_operand228 =
                                                                (self::ml_array::optional_at(
                                                                    starts,
                                                                    self::ml_array::number_index(
                                                                        declaration,
                                                                    ),
                                                                ))
                                                                .unwrap_or_else(|| 0f64);
                                                            {
                                                                let source_ml_source1_string_range_position229 = source_ml_source1_string_range_operand227;
                                                                {
                                                                    let source_ml_source1_string_range_position230 = source_ml_source1_string_range_operand228;
                                                                    source_ml_source1_string_range_operand226.slice(source_ml_source1_string_range_position229, source_ml_source1_string_range_position230)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                if member < 0f64 {
                                                    {
                                                        let stubs_2 = stubs.concat(&((((((((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(prefix)).concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 115, 116, 97, 116, 105, 99, 95]))).concat(&{
                                                            let source_ml_source1_string_argument231 = ordinal;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument231))
                                                        })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                            let source_ml_source1_string_argument232 = at;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument232))
                                                        })).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&args)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32]))).concat(&body)).concat(&self::ml_string::Text::from_units(vec![32, 125, 10])));
                                                        {
                                                            let at_2 = at + 5f64;
                                                            {
                                                                (stubs, at) = (stubs_2, at_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                                {
                                                    if prototype {
                                                        {
                                                            let stubs_3 = stubs.concat(&(((((((((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(prefix)).concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95]))).concat(&{
                                                                let source_ml_source1_string_argument233 = ordinal;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument233))
                                                            })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                                let source_ml_source1_string_argument234 = at;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument234))
                                                            })).concat(&self::ml_string::Text::from_units(vec![40]))).concat(receiver)).concat(&if args == (self::ml_string::Text::from_units(vec![])) {
                                                                self::ml_string::Text::from_units(vec![])
                                                            } else {
                                                                (self::ml_string::Text::from_units(vec![44, 32])).concat(&args)
                                                            })).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32]))).concat(&body)).concat(&self::ml_string::Text::from_units(vec![32, 125, 10])));
                                                            {
                                                                let at_3 = at + 5f64;
                                                                {
                                                                    (stubs, at) = (stubs_3, at_3);
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    {
                                                        {
                                                            let environment = (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 95]))).concat(&{
                                                                let source_ml_source1_string_argument235 = ordinal;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument235))
                                                            })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                                let source_ml_source1_string_argument236 = at;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument236))
                                                            });
                                                            {
                                                                let stubs_4 = stubs.concat(&((((((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&stub)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&args)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32, 114, 101, 116, 117, 114, 110, 32]))).concat(&stub)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&arguments_)).concat(&self::ml_string::Text::from_units(vec![41, 59, 32, 125, 10])));
                                                                {
                                                                    let stubs_5 = stubs_4.concat(&((((((((self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32])).concat(&environment)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(receiver)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 32, 114, 101, 116, 117, 114, 110, 32, 40]))).concat(&args)).concat(&self::ml_string::Text::from_units(vec![41, 32, 61, 62, 32, 123, 32]))).concat(&body)).concat(&self::ml_string::Text::from_units(vec![32, 125, 59, 32, 125, 10])));
                                                                    {
                                                                        let fields_2 = fields.concat(&(((self::ml_string::Text::from_units(vec![44, 32])).concat(&method)).concat(&self::ml_string::Text::from_units(vec![58, 32]))).concat(&stub));
                                                                        {
                                                                            let assignments_2 = assignments.concat(&((((((((self::ml_string::Text::from_units(vec![10])).concat(receiver)).concat(&self::ml_string::Text::from_units(vec![46]))).concat(&method)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&environment)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(receiver)).concat(&self::ml_string::Text::from_units(vec![41, 59, 10])));
                                                                            {
                                                                                let at_4 =
                                                                                    at + 5f64;
                                                                                {
                                                                                    (stubs, fields, assignments, at) = (stubs_5, fields_2, assignments_2, at_4);
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
        return {
            self::MlTemporaryRenderJavaScriptClosedClassLoop44Result::MlTemporaryRenderJavaScriptClosedClassLoop44Done(stubs.clone(), fields.clone(), assignments.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderJavaScriptClosedClassLoop43Result {
    MlTemporaryRenderJavaScriptClosedClassLoop43Done(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderJavaScriptClosedClassJoin46 {
    MlTemporaryRenderJavaScriptClosedClassJoin46Next(self::ml_string::Text, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderJavaScriptClosedClassJoin45 {
    MlTemporaryRenderJavaScriptClosedClassJoin45Next(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
}
