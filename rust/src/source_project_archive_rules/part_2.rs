// Generated from complete js/src/source-project-archive-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::ml_utf16_text;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_number_fact() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 78,
            117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118,
            97, 108, 117, 101,
        ]))
        .0,
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_text_fact() -> self::ml_string::Text {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 116,
            101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 118, 97, 108,
            117, 101,
        ]))
        .0,
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_required_boolean_fact() -> bool {
    (self::ml_source_exception::SourceException {
        class_name: String::from("RangeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            109, 105, 115, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 66,
            111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32,
            118, 97, 108, 117, 101,
        ]))
        .0,
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_number_fact(values: &[f64], index: f64) -> f64 {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                78, 117, 109, 98, 101, 114, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110,
                32, 112, 111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_number_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_text_fact(
    values: &[self::ml_string::Text],
    index: f64,
) -> self::ml_string::Text {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                116, 101, 120, 116, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109, 110, 32, 112,
                111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_text_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_required_boolean_fact(values: &[bool], index: f64) -> bool {
    if ((index < 0f64) || ((index % 1f64) != 0f64)) || (index >= (values.len() as f64)) {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32,
                66, 111, 111, 108, 101, 97, 110, 32, 102, 97, 99, 116, 32, 99, 111, 108, 117, 109,
                110, 32, 112, 111, 115, 105, 116, 105, 111, 110,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
            .unwrap_or_else(self::refuse_required_boolean_fact)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_source_project_archive_vocabulary(
    role: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*role) == (self::ml_string::Text::from_units(vec![109, 111, 100, 101, 108])) {
        self::ml_string::Text::from_units(vec![
            99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114,
            111, 106, 101, 99, 116, 32, 99, 111, 110, 99, 114, 101, 116, 101, 32, 103, 114, 97,
            112, 104, 115, 32, 49,
        ])
    } else if (*role)
        == (self::ml_string::Text::from_units(vec![100, 105, 114, 101, 99, 116, 111, 114, 121]))
    {
        self::ml_string::Text::from_units(vec![
            109, 101, 116, 97, 45, 108, 97, 110, 103, 117, 97, 103, 101, 47, 115, 111, 117, 114,
            99, 101, 45, 112, 114, 111, 106, 101, 99, 116, 47, 100, 105, 114, 101, 99, 116, 111,
            114, 121,
        ])
    } else if (*role)
        == (self::ml_string::Text::from_units(vec![
            115, 111, 117, 114, 99, 101, 80, 114, 101, 102, 105, 120,
        ]))
    {
        self::ml_string::Text::from_units(vec![
            109, 101, 116, 97, 45, 108, 97, 110, 103, 117, 97, 103, 101, 47, 115, 111, 117, 114,
            99, 101, 45, 112, 114, 111, 106, 101, 99, 116, 47, 115, 111, 117, 114, 99, 101, 47,
        ])
    } else if (*role) == (self::ml_string::Text::from_units(vec![114, 111, 111, 116])) {
        self::ml_string::Text::from_units(vec![
            99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32,
            115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 99, 114, 101, 116, 101, 32, 114, 111,
            111, 116, 115,
        ])
    } else if (*role)
        == (self::ml_string::Text::from_units(vec![108, 97, 110, 103, 117, 97, 103, 101]))
    {
        self::ml_string::Text::from_units(vec![
            109, 101, 116, 97, 45, 108, 97, 110, 103, 117, 97, 103, 101, 32, 115, 111, 117, 114,
            99, 101, 32, 112, 114, 111, 106, 101, 99, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_project_path(path: &self::ml_string::Text) -> bool {
    if (((((*path) == (self::ml_string::Text::from_units(vec![])))
        || path.starts_with(&self::ml_string::Text::from_units(vec![47])))
        || path.contains(&self::ml_string::Text::from_units(vec![92])))
        || path.contains(&self::ml_string::Text::from_units(vec![0])))
        || path.contains(&self::ml_string::Text::from_units(vec![58]))
    {
        false
    } else {
        {
            let source_ml_temporary_values1 = {
                let source_ml_source1_string_unit_operand1 = path.clone();
                {
                    let source_ml_source1_string_unit_operand2 =
                        self::ml_string::Text::from_units(vec![47]);
                    {
                        let source_ml_source1_string_split_limit3 = 4_294_967_295f64;
                        {
                            let source_ml_source1_string_split_separator4 =
                                source_ml_source1_string_unit_operand2;
                            (|ml_text: &self::ml_string::Text,
                              ml_separator: Option<&self::ml_string::Text>,
                              ml_limit: f64| {
                                self::ml_utf16_text::split_utf16_text(
                                    &(ml_text
                                        .0
                                        .iter()
                                        .map(|ml_unit| f64::from(*ml_unit))
                                        .collect::<Vec<f64>>()),
                                    &(ml_separator
                                        .map(|ml_value| {
                                            ml_value
                                                .0
                                                .iter()
                                                .map(|ml_unit| f64::from(*ml_unit))
                                                .collect::<Vec<f64>>()
                                        })
                                        .unwrap_or_default()),
                                    ml_limit,
                                    ml_separator.is_none(),
                                )
                                .into_iter()
                                .map(|ml_units| {
                                    self::ml_string::Text::from_units(
                                        ml_units
                                            .into_iter()
                                            .map(|ml_unit| ml_unit as u16)
                                            .collect(),
                                    )
                                })
                                .collect::<Vec<_>>()
                            })(
                                &(source_ml_source1_string_unit_operand1),
                                Some(&source_ml_source1_string_split_separator4),
                                source_ml_source1_string_split_limit3,
                            )
                        }
                    }
                }
            };
            {
                let source_ml_temporary_index1 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source1_s5 =
                        self::source_ml_temporary_accept_source_project_path_loop2(
                            &(source_ml_temporary_values1),
                            source_ml_temporary_index1,
                        );
                    match source_ml_source1_s5 {
                        self::MlTemporaryAcceptSourceProjectPathLoop2Result::MlTemporaryAcceptSourceProjectPathLoop2Done => {
                            true
                        }
                        self::MlTemporaryAcceptSourceProjectPathLoop2Result::MlTemporaryAcceptSourceProjectPathLoop2Return(source_ml_temporary_result2) => {
                            source_ml_temporary_result2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_source_project_syntax_projection(projection: &self::ml_string::Text) -> bool {
    ((*projection)
        == (self::ml_string::Text::from_units(vec![
            67, 111, 110, 99, 114, 101, 116, 101, 83, 121, 110, 116, 97, 120,
        ])))
        || ((*projection)
            == (self::ml_string::Text::from_units(vec![
                65, 98, 115, 116, 114, 97, 99, 116, 83, 121, 110, 116, 97, 120,
            ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_source_project_syntax_root_positions(
    kinds: &[self::ml_string::Text],
    identifiers: &[f64],
    firsts: &[f64],
    counts: &[f64],
    targets: &[f64],
) -> Vec<f64> {
    if (((kinds.len() as f64) != (identifiers.len() as f64))
        || ((kinds.len() as f64) != (firsts.len() as f64)))
        || ((kinds.len() as f64) != (counts.len() as f64))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                117, 110, 97, 108, 105, 103, 110, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32,
                112, 114, 111, 106, 101, 99, 116, 32, 103, 114, 97, 112, 104, 32, 99, 111, 108,
                117, 109, 110, 115,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let nested = Vec::<f64>::new();
            {
                let position = 0f64;
                {
                    let nested_2 =
                        self::source_ml_temporary_select_source_project_syntax_root_positions_loop3(
                            kinds, firsts, counts, targets, nested, position,
                        );
                    {
                        let children = self::ml_array::set(
                            (nested_2)
                                .into_iter()
                                .map(|ml_value| if ml_value == 0f64 { 0f64 } else { ml_value })
                                .collect::<Vec<_>>(),
                            |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                        );
                        {
                            let roots = Vec::<f64>::new();
                            {
                                let position_2 = 0f64;
                                {
                                    self::source_ml_temporary_select_source_project_syntax_root_positions_loop5(kinds, identifiers, &(children), roots, position_2)
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
pub fn accept_source_project_digest(value: &self::ml_string::Text) -> bool {
    {
        let ml_library_inequality_1 = (value.length()) != 64f64;
        if ml_library_inequality_1 {
            false
        } else {
            {
                let index = 0f64;
                {
                    let source_ml_source1_s6 =
                        self::source_ml_temporary_accept_source_project_digest_loop6(value, index);
                    match source_ml_source1_s6 {
                    self::MlTemporaryAcceptSourceProjectDigestLoop6Result::MlTemporaryAcceptSourceProjectDigestLoop6Done => {
                        true
                    }
                    self::MlTemporaryAcceptSourceProjectDigestLoop6Result::MlTemporaryAcceptSourceProjectDigestLoop6Return(source_ml_temporary_result6) => {
                        source_ml_temporary_result6
                    }
                }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_source_project_directory_error(
    model: &self::ml_string::Text,
    version: f64,
    projection: &self::ml_string::Text,
    paths: &[self::ml_string::Text],
    languages: &[self::ml_string::Text],
    identifiers: &[self::ml_string::Text],
    bytes: &[f64],
    digests: &[self::ml_string::Text],
) -> self::ml_string::Text {
    if (((*model)
        != (self::read_source_project_archive_vocabulary(
            &(self::ml_string::Text::from_units(vec![109, 111, 100, 101, 108])),
        )))
        || (version != 1f64))
        || !(self::accept_source_project_syntax_projection(projection))
    {
        self::ml_string::Text::from_units(vec![
            117, 110, 107, 110, 111, 119, 110, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111,
            106, 101, 99, 116, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 109, 111, 100,
            101, 108,
        ])
    } else if (((((paths.len() as f64) == 0f64)
        || ((languages.len() as f64) != (paths.len() as f64)))
        || ((identifiers.len() as f64) != (paths.len() as f64)))
        || ((bytes.len() as f64) != (paths.len() as f64)))
        || ((digests.len() as f64) != (paths.len() as f64))
    {
        self::ml_string::Text::from_units(vec![
            117, 110, 97, 108, 105, 103, 110, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 112,
            114, 111, 106, 101, 99, 116, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121,
        ])
    } else {
        {
            let names = self::ml_array::set(paths.to_vec(), |a, b| a == b);
            {
                let ml_library_inequality_1 = (names.len() as f64) != (paths.len() as f64);
                if ml_library_inequality_1 {
                    self::ml_string::Text::from_units(vec![
                        100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 115, 111, 117, 114, 99, 101,
                        32, 112, 114, 111, 106, 101, 99, 116, 32, 112, 97, 116, 104,
                    ])
                } else {
                    {
                        let index = 0f64;
                        {
                            let source_ml_source1_s7 =
                                self::source_ml_temporary_read_source_project_directory_error_loop7(
                                    paths,
                                    languages,
                                    identifiers,
                                    bytes,
                                    digests,
                                    index,
                                );
                            match source_ml_source1_s7 {
                                self::MlTemporaryReadSourceProjectDirectoryErrorLoop7Result::MlTemporaryReadSourceProjectDirectoryErrorLoop7Done => {
                                    self::ml_string::Text::from_units(vec![])
                                }
                                self::MlTemporaryReadSourceProjectDirectoryErrorLoop7Result::MlTemporaryReadSourceProjectDirectoryErrorLoop7Return(source_ml_temporary_result7) => {
                                    source_ml_temporary_result7.clone()
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
pub fn read_source_project_extension_error(
    existing: &[self::ml_string::Text],
    added: &[self::ml_string::Text],
) -> self::ml_string::Text {
    if (added.len() as f64) == 0f64 {
        self::ml_string::Text::from_units(vec![
            101, 120, 112, 101, 99, 116, 101, 100, 32, 97, 32, 98, 111, 117, 110, 100, 101, 100,
            32, 110, 111, 110, 101, 109, 112, 116, 121, 32, 115, 111, 117, 114, 99, 101, 32, 105,
            110, 118, 101, 110, 116, 111, 114, 121,
        ])
    } else {
        {
            let prior = self::ml_array::set(existing.to_vec(), |a, b| a == b);
            {
                let incoming = self::ml_array::set(added.to_vec(), |a, b| a == b);
                {
                    let ml_library_inequality_1 = (incoming.len() as f64) != (added.len() as f64);
                    if ml_library_inequality_1 {
                        self::ml_string::Text::from_units(vec![
                            100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 115, 111, 117, 114, 99,
                            101, 32, 112, 114, 111, 106, 101, 99, 116, 32, 112, 97, 116, 104,
                        ])
                    } else {
                        {
                            let source_ml_temporary_values8 = added.to_vec();
                            {
                                let source_ml_temporary_index8 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source1_s8 = self::source_ml_temporary_read_source_project_extension_error_loop9(&(prior), &(source_ml_temporary_values8), source_ml_temporary_index8);
                                    match source_ml_source1_s8 {
                                    self::MlTemporaryReadSourceProjectExtensionErrorLoop9Result::MlTemporaryReadSourceProjectExtensionErrorLoop9Done => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlTemporaryReadSourceProjectExtensionErrorLoop9Result::MlTemporaryReadSourceProjectExtensionErrorLoop9Return(source_ml_temporary_result9) => {
                                        source_ml_temporary_result9.clone()
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptSourceProjectPathLoop2Result {
    MlTemporaryAcceptSourceProjectPathLoop2Done,
    MlTemporaryAcceptSourceProjectPathLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_source_project_path_loop2(
    source_ml_temporary_values1: &[self::ml_string::Text],
    mut source_ml_temporary_index1: self::ml::Big,
) -> self::MlTemporaryAcceptSourceProjectPathLoop2Result {
    loop {
        return if source_ml_temporary_index1
            < (self::ml::Big::from_u128(source_ml_temporary_values1.len() as u128))
        {
            {
                let part = self::ml_array::at(
                    source_ml_temporary_values1,
                    source_ml_temporary_index1.to_index(),
                );
                if ((part == (self::ml_string::Text::from_units(vec![])))
                    || (part == (self::ml_string::Text::from_units(vec![46]))))
                    || (part == (self::ml_string::Text::from_units(vec![46, 46])))
                {
                    self::MlTemporaryAcceptSourceProjectPathLoop2Result::MlTemporaryAcceptSourceProjectPathLoop2Return(false)
                } else {
                    {
                        let source_ml_temporary_index1_2 =
                            source_ml_temporary_index1.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index1 = source_ml_temporary_index1_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptSourceProjectPathLoop2Result::MlTemporaryAcceptSourceProjectPathLoop2Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_select_source_project_syntax_root_positions_loop4(
    targets: &[f64],
    mut nested: Vec<f64>,
    mut first: f64,
    mut count: f64,
    mut child: f64,
) -> Vec<f64> {
    loop {
        if child < count {
            {
                let nested_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::read_required_number_fact(targets, (first + child))],
                        nested,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let child_2 = child + 1f64;
                    {
                        (nested, child) = (nested_2, child_2);
                        continue;
                    }
                }
            }
        }
        return { nested };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_select_source_project_syntax_root_positions_loop3(
    kinds: &[self::ml_string::Text],
    firsts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut nested: Vec<f64>,
    mut position: f64,
) -> Vec<f64> {
    loop {
        return if position < (kinds.len() as f64) {
            if (self::read_required_text_fact(kinds, position))
                != (self::ml_string::Text::from_units(vec![115, 121, 110, 116, 97, 120]))
            {
                {
                    let position_2 = position + 1f64;
                    {
                        position = position_2;
                        continue;
                    }
                }
            }
            {
                {
                    let first = self::read_required_number_fact(firsts, position);
                    {
                        let count = self::read_required_number_fact(counts, position);
                        if (((!(self::ml_math::is_safe_integer(first))
                            || !(self::ml_math::is_safe_integer(count)))
                            || (first < 0f64))
                            || (count < 0f64))
                            || ((first + count) > (targets.len() as f64))
                        {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    105, 110, 118, 97, 108, 105, 100, 32, 115, 111, 117, 114, 99,
                                    101, 32, 112, 114, 111, 106, 101, 99, 116, 32, 114, 101, 102,
                                    101, 114, 101, 110, 99, 101, 32, 105, 110, 116, 101, 114, 118,
                                    97, 108,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        } else {
                            {
                                let child = 0f64;
                                {
                                    let nested_2 = self::source_ml_temporary_select_source_project_syntax_root_positions_loop4(targets, nested, first, count, child);
                                    {
                                        let position_3 = position + 1f64;
                                        {
                                            (nested, position) = (nested_2, position_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            nested
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_select_source_project_syntax_root_positions_loop5(
    kinds: &[self::ml_string::Text],
    identifiers: &[f64],
    children: &[f64],
    mut roots: Vec<f64>,
    mut position: f64,
) -> Vec<f64> {
    loop {
        if position < (kinds.len() as f64) {
            if ((self::read_required_text_fact(kinds, position))
                == (self::ml_string::Text::from_units(vec![115, 121, 110, 116, 97, 120])))
                && !({
                    let ml_collection = children.to_vec();
                    let ml_search = self::read_required_number_fact(identifiers, position);
                    ml_collection.iter().any(|ml_key| {
                        (ml_key == &ml_search || (ml_key.is_nan() && ml_search.is_nan()))
                    })
                })
            {
                {
                    let roots_2 = {
                        let (ml_append_right, ml_append_left) = (vec![position], roots);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let position_2 = position + 1f64;
                        {
                            (roots, position) = (roots_2, position_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let position_3 = position + 1f64;
                    {
                        position = position_3;
                        continue;
                    }
                }
            }
        }
        return { roots };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptSourceProjectDigestLoop6Result {
    MlTemporaryAcceptSourceProjectDigestLoop6Done,
    MlTemporaryAcceptSourceProjectDigestLoop6Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_source_project_digest_loop6(
    value: &self::ml_string::Text,
    mut index: f64,
) -> self::MlTemporaryAcceptSourceProjectDigestLoop6Result {
    loop {
        return if index < (value.length()) {
            if ((self::ml_string::Text::from_units(vec![
                48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 97, 98, 99, 100, 101, 102,
            ]))
            .contains(&{
                let source_ml_source1_string_range_operand9 = value.clone();
                {
                    let source_ml_source1_string_range_operand10 = index;
                    {
                        let source_ml_source1_string_range_operand11 = index + 1f64;
                        {
                            let source_ml_source1_string_range_position12 =
                                source_ml_source1_string_range_operand10;
                            {
                                let source_ml_source1_string_range_position13 =
                                    source_ml_source1_string_range_operand11;
                                source_ml_source1_string_range_operand9.slice(
                                    source_ml_source1_string_range_position12,
                                    source_ml_source1_string_range_position13,
                                )
                            }
                        }
                    }
                }
            })) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlTemporaryAcceptSourceProjectDigestLoop6Result::MlTemporaryAcceptSourceProjectDigestLoop6Return(false)
            }
        } else {
            self::MlTemporaryAcceptSourceProjectDigestLoop6Result::MlTemporaryAcceptSourceProjectDigestLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadSourceProjectDirectoryErrorLoop7Result {
    MlTemporaryReadSourceProjectDirectoryErrorLoop7Done,
    MlTemporaryReadSourceProjectDirectoryErrorLoop7Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_source_project_directory_error_loop7(
    paths: &[self::ml_string::Text],
    languages: &[self::ml_string::Text],
    identifiers: &[self::ml_string::Text],
    bytes: &[f64],
    digests: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlTemporaryReadSourceProjectDirectoryErrorLoop7Result {
    loop {
        return if index < (paths.len() as f64) {
            {
                let size = self::read_required_number_fact(bytes, index);
                {
                    let path = self::read_required_text_fact(paths, index);
                    if ((((!(self::accept_source_project_path(&(path)))
                        || ((self::read_required_text_fact(languages, index))
                            == (self::ml_string::Text::from_units(vec![]))))
                        || ((self::read_required_text_fact(identifiers, index))
                            != ((self::read_source_project_archive_vocabulary(
                                &(self::ml_string::Text::from_units(vec![
                                    115, 111, 117, 114, 99, 101, 80, 114, 101, 102, 105, 120,
                                ])),
                            ))
                            .concat(&path))))
                        || !(self::ml_math::is_safe_integer(size)))
                        || (size < 0f64))
                        || !(self::accept_source_project_digest(
                            &(self::read_required_text_fact(digests, index)),
                        ))
                    {
                        self::MlTemporaryReadSourceProjectDirectoryErrorLoop7Result::MlTemporaryReadSourceProjectDirectoryErrorLoop7Return(self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 106, 101, 99, 116, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 110, 116, 114, 121]))
                    } else {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadSourceProjectDirectoryErrorLoop7Result::MlTemporaryReadSourceProjectDirectoryErrorLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadSourceProjectExtensionErrorLoop9Result {
    MlTemporaryReadSourceProjectExtensionErrorLoop9Done,
    MlTemporaryReadSourceProjectExtensionErrorLoop9Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_source_project_extension_error_loop9(
    prior: &[self::ml_string::Text],
    source_ml_temporary_values8: &[self::ml_string::Text],
    mut source_ml_temporary_index8: self::ml::Big,
) -> self::MlTemporaryReadSourceProjectExtensionErrorLoop9Result {
    loop {
        return if source_ml_temporary_index8
            < (self::ml::Big::from_u128(source_ml_temporary_values8.len() as u128))
        {
            {
                let path = self::ml_array::at(
                    source_ml_temporary_values8,
                    source_ml_temporary_index8.to_index(),
                );
                if (self::accept_source_project_path(&(path))) {
                    if {
                        let ml_collection = prior.to_vec();
                        let ml_search = path.clone();
                        ml_collection.iter().any(|ml_key| ml_key == &ml_search)
                    } {
                        self::MlTemporaryReadSourceProjectExtensionErrorLoop9Result::MlTemporaryReadSourceProjectExtensionErrorLoop9Return(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 106, 101, 99, 116, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 110, 32, 101, 120, 105, 115, 116, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101]))
                    } else {
                        {
                            let source_ml_temporary_index8_2 =
                                source_ml_temporary_index8.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index8 = source_ml_temporary_index8_2;
                                continue;
                            }
                        }
                    }
                } else {
                    self::MlTemporaryReadSourceProjectExtensionErrorLoop9Result::MlTemporaryReadSourceProjectExtensionErrorLoop9Return(self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 112, 117, 116]))
                }
            }
        } else {
            self::MlTemporaryReadSourceProjectExtensionErrorLoop9Result::MlTemporaryReadSourceProjectExtensionErrorLoop9Done
        };
    }
}
