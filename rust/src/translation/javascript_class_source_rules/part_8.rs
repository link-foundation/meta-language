// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41;
use super::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result;
use super::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Result;
use super::accept_java_script_closed_class_collection_size;
use super::accept_java_script_closed_class_function_export;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_java_script_closed_class_methods;
use super::read_java_script_closed_class_static_target;
use super::render_java_script_closed_class_parameter_arguments;
use super::render_java_script_closed_class_range;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop42(
    mut at: f64,
    source_ml_temporary_values38: &[Vec<f64>],
    mut source_ml_temporary_index38: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Result {
    loop {
        return if source_ml_temporary_index38
            < (self::ml::Big::from_u128(source_ml_temporary_values38.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values38,
                    source_ml_temporary_index38.to_index(),
                );
                if (at
                    > ((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64)))
                        .unwrap_or_else(|| 0f64)))
                    && (at
                        < ((self::ml_array::optional_at(&row, self::ml_array::number_index(1f64)))
                            .unwrap_or_else(|| 0f64)))
                {
                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Return(at)
                } else {
                    {
                        let source_ml_temporary_index38_2 =
                            source_ml_temporary_index38.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index38 = source_ml_temporary_index38_2;
                            continue;
                        }
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
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop40(
    values: &[self::ml_string::Text],
    kinds: &[self::ml_string::Text],
    directory: &[Vec<f64>],
    static_calls: &[self::ml_string::Text],
    names: &[self::ml_string::Text],
    methods: &[self::ml_string::Text],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result {
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
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(at)
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
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(at)
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
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(at)
                            } else {
                                {
                                    let source_ml_source1_s213 = if value
                                        == (self::ml_string::Text::from_units(vec![
                                            102, 117, 110, 99, 116, 105, 111, 110,
                                        ])) {
                                        {
                                            let source_ml_temporary_values38 = directory.to_vec();
                                            {
                                                let source_ml_temporary_index38 =
                                                    self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s214 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop42(at, &(source_ml_temporary_values38), source_ml_temporary_index38);
                                                    match source_ml_source1_s214 {
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Done => {
                                                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41Next
                                                                }
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop42Return(source_ml_temporary_result42) => {
                                                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41Return(source_ml_temporary_result42)
                                                                }
                                                            }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41Next
                                    };
                                    match source_ml_source1_s213 {
                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41Next => {
                                                    if ((((kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) && self::ml_array::includes(names, &value.clone(), 0f64, false, |a, b| a == b)) && (previous != (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])))) && ((previous != (self::ml_string::Text::from_units(vec![110, 101, 119]))) || (next != (self::ml_string::Text::from_units(vec![40]))))) && ((self::read_java_script_closed_class_static_target(values, names, static_calls, at)) == (self::ml_string::Text::from_units(vec![]))) {
                                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(at)
                                                    } else if ((((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (previous != (self::ml_string::Text::from_units(vec![46])))) && (previous != (self::ml_string::Text::from_units(vec![63, 46])))) && (next != (self::ml_string::Text::from_units(vec![46])))) && (next != (self::ml_string::Text::from_units(vec![91]))) {
                                                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(at)
                                                        } else if ((previous == (self::ml_string::Text::from_units(vec![46]))) || (previous == (self::ml_string::Text::from_units(vec![63, 46])))) && ((value == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) || ((self::ml_array::includes(methods, &value.clone(), 0f64, false, |a, b| a == b) && (next != (self::ml_string::Text::from_units(vec![40])))) && !(self::accept_java_script_closed_class_collection_size(values, directory, at)))) {
                                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(at)
                                                            } else if ((previous == (self::ml_string::Text::from_units(vec![91]))) && (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) && self::ml_array::includes(methods, &{
                                                                    let source_ml_source1_string_range_operand215 = value.clone();
                                                                    {
                                                                        let source_ml_source1_string_range_operand216 = 1f64;
                                                                        {
                                                                            let source_ml_source1_string_range_position217 = source_ml_source1_string_range_operand216;
                                                                            {
                                                                                let source_ml_source1_string_range_position218 = f64::INFINITY;
                                                                                source_ml_source1_string_range_operand215.slice(source_ml_source1_string_range_position217, source_ml_source1_string_range_position218)
                                                                            }
                                                                        }
                                                                    }
                                                                }, 0f64, false, |a, b| a == b) {
                                                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(at)
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
                                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41::MlTemporaryReadJavaScriptClosedClassObservationErrorJoin41Return(source_ml_temporary_result41) => {
                                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Return(source_ml_temporary_result41)
                                                }
                                            }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop40Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop39Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop39Done(
        Vec<self::ml_string::Text>,
        Vec<self::ml_string::Text>,
    ),
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop39Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop43Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop43Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop43Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptClosedClassObservationErrorLoop44Result {
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop44Done,
    MlTemporaryReadJavaScriptClosedClassObservationErrorLoop44Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop44(
    values: &[self::ml_string::Text],
    row: &[f64],
    mut at: f64,
    mut inner: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop44Result {
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
                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop44Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop44Return(inner)
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
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop44Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop44Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop43(
    values: &[self::ml_string::Text],
    row: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop43Result {
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
                        let source_ml_source1_s219 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop44(values, row, at, inner);
                        match source_ml_source1_s219 {
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop44Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop44Done => {
                                {
                                    let at_2 = at + 5f64;
                                    {
                                        at = at_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop44Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop44Return(source_ml_temporary_result44) => {
                                self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop43Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop43Return(source_ml_temporary_result44)
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
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop43Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop43Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closed_class_observation_error_loop39(
    values: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    mut methods: Vec<self::ml_string::Text>,
    source_ml_temporary_values37: &[Vec<f64>],
    mut source_ml_temporary_index37: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop39Result {
    loop {
        return if source_ml_temporary_index37
            < (self::ml::Big::from_u128(source_ml_temporary_values37.len() as u128))
        {
            {
                let row = self::ml_array::at(
                    source_ml_temporary_values37,
                    source_ml_temporary_index37.to_index(),
                );
                {
                    let names_2 = {
                        let source_ml_source1_concat_value220 = names;
                        {
                            let source_ml_source1_concat_value221 = vec![
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
                                    source_ml_source1_concat_value220,
                                ),
                                source_ml_source1_concat_value221,
                            )
                        }
                    };
                    {
                        let methods_2 = {
                            let source_ml_source1_concat_value222 = methods;
                            {
                                let source_ml_source1_concat_value223 =
                                    self::read_java_script_closed_class_methods(values, &(row));
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value222,
                                    ),
                                    source_ml_source1_concat_value223,
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
                                let source_ml_source1_s224 = self::source_ml_temporary_read_java_script_closed_class_observation_error_loop43(values, &(row), at);
                                match source_ml_source1_s224 {
                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop43Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop43Done => {
                                        {
                                            let source_ml_temporary_index37_2 = source_ml_temporary_index37.add(&self::ml::Big::from_i128(1));
                                            {
                                                (names, methods, source_ml_temporary_index37) = (names_2, methods_2, source_ml_temporary_index37_2);
                                                continue;
                                            }
                                        }
                                    }
                                    self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop43Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop43Return(source_ml_temporary_result43) => {
                                        self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop39Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop39Return(source_ml_temporary_result43)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop39Result::MlTemporaryReadJavaScriptClosedClassObservationErrorLoop39Done(names, methods)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderJavaScriptClosedClassLoop46Result {
    MlTemporaryRenderJavaScriptClosedClassLoop46Done(
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
pub fn source_ml_temporary_render_java_script_closed_class_loop46(
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
) -> self::MlTemporaryRenderJavaScriptClosedClassLoop46Result {
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
                                            let source_ml_source1_string_argument225 = ordinal;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument225,
                                                ),
                                            )
                                        }))
                                        .concat(&self::ml_string::Text::from_units(vec![95])))
                                        .concat(&{
                                            let source_ml_source1_string_argument226 = at;
                                            self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(
                                                    source_ml_source1_string_argument226,
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
                                                    let source_ml_source1_string_range_operand227 =
                                                        source.clone();
                                                    {
                                                        let source_ml_source1_string_range_operand228 =
                                                            (self::ml_array::optional_at(
                                                                ends,
                                                                self::ml_array::number_index(
                                                                    declaration - 1f64,
                                                                ),
                                                            ))
                                                            .unwrap_or_else(|| 0f64);
                                                        {
                                                            let source_ml_source1_string_range_operand229 =
                                                                (self::ml_array::optional_at(
                                                                    starts,
                                                                    self::ml_array::number_index(
                                                                        declaration,
                                                                    ),
                                                                ))
                                                                .unwrap_or_else(|| 0f64);
                                                            {
                                                                let source_ml_source1_string_range_position230 = source_ml_source1_string_range_operand228;
                                                                {
                                                                    let source_ml_source1_string_range_position231 = source_ml_source1_string_range_operand229;
                                                                    source_ml_source1_string_range_operand227.slice(source_ml_source1_string_range_position230, source_ml_source1_string_range_position231)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                if member < 0f64 {
                                                    {
                                                        let stubs_2 = stubs.concat(&((((((((((((self::ml_string::Text::from_units(vec![10])).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(prefix)).concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 115, 116, 97, 116, 105, 99, 95]))).concat(&{
                                                            let source_ml_source1_string_argument232 = ordinal;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument232))
                                                        })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                            let source_ml_source1_string_argument233 = at;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument233))
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
                                                                let source_ml_source1_string_argument234 = ordinal;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument234))
                                                            })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                                let source_ml_source1_string_argument235 = at;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument235))
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
                                                                let source_ml_source1_string_argument236 = ordinal;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument236))
                                                            })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                                                let source_ml_source1_string_argument237 = at;
                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument237))
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
            self::MlTemporaryRenderJavaScriptClosedClassLoop46Result::MlTemporaryRenderJavaScriptClosedClassLoop46Done(stubs.clone(), fields.clone(), assignments.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderJavaScriptClosedClassLoop45Result {
    MlTemporaryRenderJavaScriptClosedClassLoop45Done(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderJavaScriptClosedClassJoin48 {
    MlTemporaryRenderJavaScriptClosedClassJoin48Next(self::ml_string::Text, self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRenderJavaScriptClosedClassJoin47 {
    MlTemporaryRenderJavaScriptClosedClassJoin47Next(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
}
