// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryDependencyFunction3Loop1Result;
use super::ml_array;
use super::ml_string;
use super::source_ml_source1_temporary_dependency_function3_loop1;

pub mod ml_source_exception {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct SourceException {
        pub class_name: String,
        pub message_units: Vec<u16>,
        pub message_present: bool,
    }
    impl SourceException {
        #[must_use]
        pub fn raise(self) -> ! {
            std::panic::panic_any((self.class_name, self.message_units, self.message_present))
        }
        #[must_use]
        pub fn from_text(class_name: &str, message: &str) -> Self {
            Self {
                class_name: class_name.to_owned(),
                message_units: message.encode_utf16().collect(),
                message_present: true,
            }
        }
    }
    impl std::fmt::Display for SourceException {
        fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str(&String::from_utf16_lossy(&self.message_units))
        }
    }
    impl std::error::Error for SourceException {}
}

#[path = "ml_utf16_text.rs"]
pub mod ml_utf16_text;

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
pub fn dependency_function1(values: &[f64], index: f64) -> f64 {
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
pub fn dependency_function2(values: &[self::ml_string::Text], index: f64) -> self::ml_string::Text {
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
pub fn dependency_function3(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    field: &self::ml_string::Text,
) -> f64 {
    if node < 0f64 {
        -1f64
    } else {
        {
            let start = self::dependency_function1(starts, node);
            {
                let count = self::dependency_function1(counts, node);
                {
                    let at = start;
                    {
                        let source_ml_source2_s1 =
                            self::source_ml_source1_temporary_dependency_function3_loop1(
                                labels, targets, field, start, count, at,
                            );
                        match source_ml_source2_s1 {
                            self::MlSource1TemporaryDependencyFunction3Loop1Result::MlSource1TemporaryDependencyFunction3Loop1Done => {
                                -1f64
                            }
                            self::MlSource1TemporaryDependencyFunction3Loop1Result::MlSource1TemporaryDependencyFunction3Loop1Return(source_ml_source1_temporary_result1) => {
                                source_ml_source1_temporary_result1
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
pub fn dependency_function4(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    node: f64,
    index: f64,
) -> f64 {
    if ((node < 0f64) || (index < 0f64)) || (index >= (self::dependency_function1(counts, node))) {
        -1f64
    } else {
        self::dependency_function1(targets, (self::dependency_function1(starts, node) + index))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_kind(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
) -> self::ml_string::Text {
    {
        let kind = self::dependency_function3(
            starts,
            counts,
            labels,
            targets,
            node,
            &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
        );
        if kind < 0f64 {
            self::ml_string::Text::from_units(vec![])
        } else {
            self::dependency_function2(texts, kind)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_collection_construction_inference(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    kind: &self::ml_string::Text,
    expected: f64,
) -> Vec<f64> {
    {
        let context = (self::read_collection_construction_kind(
            texts, starts, counts, labels, targets, expected,
        )) == (*kind);
        if (*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
            vec![
                1f64,
                if context { 1f64 } else { 0f64 },
                if context {
                    self::dependency_function3(
                        starts,
                        counts,
                        labels,
                        targets,
                        expected,
                        &(self::ml_string::Text::from_units(vec![
                            101, 108, 101, 109, 101, 110, 116,
                        ])),
                    )
                } else {
                    -1f64
                },
                -1f64,
            ]
        } else if (*kind) == (self::ml_string::Text::from_units(vec![109, 97, 112])) {
            vec![
                2f64,
                if context { 1f64 } else { 0f64 },
                if context {
                    self::dependency_function3(
                        starts,
                        counts,
                        labels,
                        targets,
                        expected,
                        &(self::ml_string::Text::from_units(vec![107, 101, 121])),
                    )
                } else {
                    -1f64
                },
                if context {
                    self::dependency_function3(
                        starts,
                        counts,
                        labels,
                        targets,
                        expected,
                        &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                    )
                } else {
                    -1f64
                },
            ]
        } else {
            vec![0f64, 0f64, -1f64, -1f64]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_collection_construction_checked(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    kind: &self::ml_string::Text,
    items: f64,
) -> Vec<f64> {
    {
        let array = (self::read_collection_construction_kind(
            texts, starts, counts, labels, targets, items,
        )) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]));
        {
            let element = self::dependency_function3(
                starts,
                counts,
                labels,
                targets,
                items,
                &(self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])),
            );
            if (*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
                if array {
                    vec![0f64, element, -1f64]
                } else {
                    vec![1f64, -1f64, -1f64]
                }
            } else {
                {
                    let tuple =
                        (self::read_collection_construction_kind(
                            texts, starts, counts, labels, targets, element,
                        )) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]));
                    {
                        let elements = self::dependency_function3(
                            starts,
                            counts,
                            labels,
                            targets,
                            element,
                            &(self::ml_string::Text::from_units(vec![
                                101, 108, 101, 109, 101, 110, 116, 115,
                            ])),
                        );
                        if ((!array || !tuple) || (elements < 0f64))
                            || ((self::dependency_function1(counts, elements)) != 2f64)
                        {
                            vec![2f64, -1f64, -1f64]
                        } else {
                            vec![
                                0f64,
                                self::dependency_function4(starts, counts, targets, elements, 0f64),
                                self::dependency_function4(starts, counts, targets, elements, 1f64),
                            ]
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_key_admission(
    kind: &self::ml_string::Text,
    primitive: bool,
) -> f64 {
    if primitive {
        0f64
    } else if (*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
        3f64
    } else {
        4f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_diagnostic(
    code: f64,
    part: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*part) == (self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101])))
        && (code == 1f64)
    {
        self::ml_string::Text::from_units(vec![
            83, 101, 116, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 110, 101,
            101, 100, 115, 32, 97, 110, 32, 105, 116, 101, 114, 97, 98, 108, 101, 32, 97, 114, 114,
            97, 121,
        ])
    } else if ((*part)
        == (self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101])))
        && (code == 2f64)
    {
        self::ml_string::Text::from_units(vec![
            77, 97, 112, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 110, 101,
            101, 100, 115, 32, 97, 110, 32, 105, 116, 101, 114, 97, 98, 108, 101, 32, 111, 102, 32,
            107, 101, 121, 47, 118, 97, 108, 117, 101, 32, 112, 97, 105, 114, 115,
        ])
    } else if ((*part)
        == (self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101])))
        && (code == 3f64)
    {
        self::ml_string::Text::from_units(vec![
            115, 101, 116, 32, 107, 101, 121, 115, 32, 110, 101, 101, 100, 32, 101, 120, 112, 108,
            105, 99, 105, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 100, 101,
            110, 116, 105, 116, 105, 101, 115, 32, 102, 111, 114, 32, 115, 116, 114, 117, 99, 116,
            117, 114, 101, 100, 32, 118, 97, 108, 117, 101, 115,
        ])
    } else if ((*part)
        == (self::ml_string::Text::from_units(vec![109, 101, 115, 115, 97, 103, 101])))
        && (code == 4f64)
    {
        self::ml_string::Text::from_units(vec![
            109, 97, 112, 32, 107, 101, 121, 115, 32, 110, 101, 101, 100, 32, 101, 120, 112, 108,
            105, 99, 105, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 105, 100, 101,
            110, 116, 105, 116, 105, 101, 115, 32, 102, 111, 114, 32, 115, 116, 114, 117, 99, 116,
            117, 114, 101, 100, 32, 118, 97, 108, 117, 101, 115,
        ])
    } else if ((*part)
        == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])))
        && (code == 3f64)
    {
        self::ml_string::Text::from_units(vec![
            83, 101, 116, 32, 111, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116,
            121,
        ])
    } else if ((*part)
        == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])))
        && (code == 4f64)
    {
        self::ml_string::Text::from_units(vec![
            77, 97, 112, 32, 111, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_collection_construction_binding(
    target: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    role: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let set = (*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116]));
        if (*target) == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])) {
            if (*role)
                == (self::ml_string::Text::from_units(vec![112, 114, 101, 118, 105, 111, 117, 115]))
            {
                if set {
                    self::ml_string::Text::from_units(vec![
                        109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![
                        40, 102, 115, 116, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117,
                        115, 41,
                    ])
                }
            } else if set {
                self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])
            } else {
                self::ml_string::Text::from_units(vec![
                    40, 102, 115, 116, 32, 109, 108, 95, 118, 97, 108, 117, 101, 41,
                ])
            }
        } else if (*role)
            == (self::ml_string::Text::from_units(vec![112, 114, 101, 118, 105, 111, 117, 115]))
        {
            if set {
                self::ml_string::Text::from_units(vec![
                    109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115,
                ])
            } else {
                self::ml_string::Text::from_units(vec![
                    109, 108, 95, 112, 114, 101, 118, 105, 111, 117, 115, 46, 49,
                ])
            }
        } else if set {
            self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])
        } else {
            self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101, 46, 49])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_collection_construction_type(
    target: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    first: &self::ml_string::Text,
    second: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*target) == (self::ml_string::Text::from_units(vec![76, 101, 97, 110])) {
        if (*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
            ((self::ml_string::Text::from_units(vec![40, 65, 114, 114, 97, 121, 32])).concat(first))
                .concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            ((((self::ml_string::Text::from_units(vec![40, 65, 114, 114, 97, 121, 32, 40]))
                .concat(first))
            .concat(&self::ml_string::Text::from_units(vec![32, 215, 32])))
            .concat(second))
            .concat(&self::ml_string::Text::from_units(vec![41, 41]))
        }
    } else if (*target) == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])) {
        if (*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116])) {
            ((self::ml_string::Text::from_units(vec![40, 108, 105, 115, 116, 32])).concat(first))
                .concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            ((((self::ml_string::Text::from_units(vec![40, 108, 105, 115, 116, 32, 40]))
                .concat(first))
            .concat(&self::ml_string::Text::from_units(vec![32, 42, 32])))
            .concat(second))
            .concat(&self::ml_string::Text::from_units(vec![
                41, 41, 37, 116, 121, 112, 101,
            ]))
        }
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_collection_construction(
    target: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    key_kind: &self::ml_string::Text,
    values: &self::ml_string::Text,
    collection_type: &self::ml_string::Text,
    element_type: &self::ml_string::Text,
    comparison: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let set = (*kind) == (self::ml_string::Text::from_units(vec![115, 101, 116]));
        if (*target)
            == (self::ml_string::Text::from_units(vec![
                74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
            ]))
        {
            ((((self::ml_string::Text::from_units(vec![110, 101, 119, 32])).concat(&if set {
                self::ml_string::Text::from_units(vec![83, 101, 116])
            } else {
                self::ml_string::Text::from_units(vec![77, 97, 112])
            }))
            .concat(&self::ml_string::Text::from_units(vec![40])))
            .concat(values))
            .concat(&self::ml_string::Text::from_units(vec![41]))
        } else if (*target) == (self::ml_string::Text::from_units(vec![82, 117, 115, 116])) {
            {
                let comparator = if (*key_kind)
                    == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))
                {
                    self::ml_string::Text::from_units(vec![
                        124, 97, 58, 32, 38, 102, 54, 52, 44, 32, 98, 58, 32, 38, 102, 54, 52, 124,
                        32, 97, 32, 61, 61, 32, 98, 32, 124, 124, 32, 40, 97, 46, 105, 115, 95,
                        110, 97, 110, 40, 41, 32, 38, 38, 32, 98, 46, 105, 115, 95, 110, 97, 110,
                        40, 41, 41,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![
                        124, 97, 44, 32, 98, 124, 32, 97, 32, 61, 61, 32, 98,
                    ])
                };
                {
                    let values_3 = if (*key_kind)
                        == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))
                    {
                        {
                            ((((self::ml_string::Text::from_units(vec![40])).concat(values))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    41, 46, 105, 110, 116, 111, 95, 105, 116, 101, 114, 40, 41, 46,
                                    109, 97, 112, 40, 124, 109, 108, 95, 118, 97, 108, 117, 101,
                                    124, 32,
                                ])))
                            .concat(&if set {
                                self::ml_string::Text::from_units(vec![
                                    105, 102, 32, 109, 108, 95, 118, 97, 108, 117, 101, 32, 61, 61,
                                    32, 48, 102, 54, 52, 32, 123, 32, 48, 102, 54, 52, 32, 125, 32,
                                    101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 118, 97, 108,
                                    117, 101, 32, 125,
                                ])
                            } else {
                                self::ml_string::Text::from_units(vec![
                                    40, 105, 102, 32, 109, 108, 95, 118, 97, 108, 117, 101, 46, 48,
                                    32, 61, 61, 32, 48, 102, 54, 52, 32, 123, 32, 48, 102, 54, 52,
                                    32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 109, 108, 95,
                                    118, 97, 108, 117, 101, 46, 48, 32, 125, 44, 32, 109, 108, 95,
                                    118, 97, 108, 117, 101, 46, 49, 41,
                                ])
                            }))
                            .concat(
                                &self::ml_string::Text::from_units(vec![
                                    41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101,
                                    99, 60, 95, 62, 62, 40, 41,
                                ]),
                            )
                        }
                    } else {
                        values.clone()
                    };
                    ((((((self::ml_string::Text::from_units(vec![
                        99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 97, 114, 114, 97, 121, 58, 58,
                    ]))
                    .concat(kind))
                    .concat(&self::ml_string::Text::from_units(vec![40])))
                    .concat(&values_3))
                    .concat(&self::ml_string::Text::from_units(vec![44, 32])))
                    .concat(&comparator))
                    .concat(&self::ml_string::Text::from_units(vec![41]))
                }
            }
        } else {
            {
                let value = self::read_collection_construction_binding(
                    target,
                    kind,
                    &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                );
                {
                    let previous = self::read_collection_construction_binding(
                        target,
                        kind,
                        &(self::ml_string::Text::from_units(vec![
                            112, 114, 101, 118, 105, 111, 117, 115,
                        ])),
                    );
                    if (*target) == (self::ml_string::Text::from_units(vec![76, 101, 97, 110])) {
                        {
                            let equal = if (*key_kind)
                                == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))
                            {
                                ((((((((self::ml_string::Text::from_units(vec![40]))
                                    .concat(&previous))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    32, 61, 61, 32,
                                ])))
                                .concat(&value))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    32, 124, 124, 32, 40,
                                ])))
                                .concat(&previous))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    46, 105, 115, 78, 97, 78, 32, 38, 38, 32,
                                ])))
                                .concat(&value))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    46, 105, 115, 78, 97, 78, 41, 41,
                                ]))
                            } else {
                                ((((self::ml_string::Text::from_units(vec![40]))
                                    .concat(&previous))
                                .concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32])))
                                .concat(&value))
                                .concat(&self::ml_string::Text::from_units(vec![41]))
                            };
                            {
                                let normalized = if (*key_kind)
                                    == (self::ml_string::Text::from_units(vec![
                                        102, 108, 111, 97, 116,
                                    ])) {
                                    if set {
                                        self::ml_string::Text::from_units(vec![
                                            40, 105, 102, 32, 109, 108, 95, 111, 114, 105, 103,
                                            105, 110, 97, 108, 32, 61, 61, 32, 40, 48, 32, 58, 32,
                                            70, 108, 111, 97, 116, 41, 32, 116, 104, 101, 110, 32,
                                            40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 101,
                                            108, 115, 101, 32, 109, 108, 95, 111, 114, 105, 103,
                                            105, 110, 97, 108, 41,
                                        ])
                                    } else {
                                        self::ml_string::Text::from_units(vec![
                                            40, 40, 105, 102, 32, 109, 108, 95, 111, 114, 105, 103,
                                            105, 110, 97, 108, 46, 49, 32, 61, 61, 32, 40, 48, 32,
                                            58, 32, 70, 108, 111, 97, 116, 41, 32, 116, 104, 101,
                                            110, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41,
                                            32, 101, 108, 115, 101, 32, 109, 108, 95, 111, 114,
                                            105, 103, 105, 110, 97, 108, 46, 49, 41, 44, 32, 109,
                                            108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 46, 50,
                                            41,
                                        ])
                                    }
                                } else {
                                    self::ml_string::Text::from_units(vec![
                                        109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108,
                                    ])
                                };
                                {
                                    let existing = if set {
                                        self::ml_string::Text::from_units(vec![
                                            109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116,
                                            111, 114,
                                        ])
                                    } else {
                                        ((self::ml_string::Text::from_units(vec![
                                            40, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97,
                                            116, 111, 114, 46, 109, 97, 112, 32, 40, 102, 117, 110,
                                            32, 109, 108, 95, 112, 114, 101, 118, 105, 111, 117,
                                            115, 32, 61, 62, 32, 105, 102, 32,
                                        ]))
                                        .concat(&equal))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 116, 104, 101, 110, 32, 40, 109, 108, 95, 112, 114,
                                            101, 118, 105, 111, 117, 115, 46, 49, 44, 32, 109, 108,
                                            95, 118, 97, 108, 117, 101, 46, 50, 41, 32, 101, 108,
                                            115, 101, 32, 109, 108, 95, 112, 114, 101, 118, 105,
                                            111, 117, 115, 41, 41,
                                        ]))
                                    };
                                    ((((((((((self::ml_string::Text::from_units(vec![40]))
                                        .concat(values))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32,
                                        109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111,
                                        114, 32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97,
                                        108, 32, 61, 62, 32, 108, 101, 116, 32, 109, 108, 95, 118,
                                        97, 108, 117, 101, 32, 58, 61, 32,
                                    ])))
                                    .concat(&normalized))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        59, 32, 105, 102, 32, 109, 108, 95, 97, 99, 99, 117, 109,
                                        117, 108, 97, 116, 111, 114, 46, 97, 110, 121, 32, 40, 102,
                                        117, 110, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111,
                                        117, 115, 32, 61, 62, 32,
                                    ])))
                                    .concat(&equal))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        41, 32, 116, 104, 101, 110, 32,
                                    ])))
                                    .concat(&existing))
                                    .concat(&self::ml_string::Text::from_units(vec![
                                        32, 101, 108, 115, 101, 32, 109, 108, 95, 97, 99, 99, 117,
                                        109, 117, 108, 97, 116, 111, 114, 46, 112, 117, 115, 104,
                                        32, 109, 108, 95, 118, 97, 108, 117, 101, 41, 32, 40, 35,
                                        91, 93, 32, 58, 32,
                                    ])))
                                    .concat(collection_type))
                                    .concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                }
                            }
                        }
                    } else {
                        {
                            let equal_2 = comparison.clone();
                            {
                                let equal_4 = if (*key_kind)
                                    == (self::ml_string::Text::from_units(vec![
                                        102, 108, 111, 97, 116,
                                    ])) {
                                    {
                                        ((((((self::ml_string::Text::from_units(vec![
                                            40, 111, 114, 98, 32,
                                        ]))
                                        .concat(&equal_2))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 40, 97, 110, 100, 98, 32, 40, 80, 114, 105, 109,
                                            70, 108, 111, 97, 116, 46, 105, 115, 95, 110, 97, 110,
                                            32,
                                        ])))
                                        .concat(&previous))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            41, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116,
                                            46, 105, 115, 95, 110, 97, 110, 32,
                                        ])))
                                        .concat(&value))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            41, 41, 41,
                                        ]))
                                    }
                                } else {
                                    equal_2.clone()
                                };
                                {
                                    let normalized_2 = if (*key_kind)
                                        == (self::ml_string::Text::from_units(vec![
                                            102, 108, 111, 97, 116,
                                        ])) {
                                        if set {
                                            self::ml_string::Text::from_units(vec![
                                                40, 105, 102, 32, 80, 114, 105, 109, 70, 108, 111,
                                                97, 116, 46, 101, 113, 98, 32, 109, 108, 95, 111,
                                                114, 105, 103, 105, 110, 97, 108, 32, 48, 37, 102,
                                                108, 111, 97, 116, 32, 116, 104, 101, 110, 32, 48,
                                                37, 102, 108, 111, 97, 116, 32, 101, 108, 115, 101,
                                                32, 109, 108, 95, 111, 114, 105, 103, 105, 110, 97,
                                                108, 41,
                                            ])
                                        } else {
                                            self::ml_string::Text::from_units(vec![
                                                40, 40, 105, 102, 32, 80, 114, 105, 109, 70, 108,
                                                111, 97, 116, 46, 101, 113, 98, 32, 40, 102, 115,
                                                116, 32, 109, 108, 95, 111, 114, 105, 103, 105,
                                                110, 97, 108, 41, 32, 48, 37, 102, 108, 111, 97,
                                                116, 32, 116, 104, 101, 110, 32, 48, 37, 102, 108,
                                                111, 97, 116, 32, 101, 108, 115, 101, 32, 102, 115,
                                                116, 32, 109, 108, 95, 111, 114, 105, 103, 105,
                                                110, 97, 108, 41, 44, 32, 115, 110, 100, 32, 109,
                                                108, 95, 111, 114, 105, 103, 105, 110, 97, 108, 41,
                                            ])
                                        }
                                    } else {
                                        self::ml_string::Text::from_units(vec![
                                            109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108,
                                        ])
                                    };
                                    {
                                        let existing_2 = if set {
                                            self::ml_string::Text::from_units(vec![
                                                109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97,
                                                116, 111, 114,
                                            ])
                                        } else {
                                            ((self::ml_string::Text::from_units(vec![
                                                40, 76, 105, 115, 116, 46, 109, 97, 112, 32, 40,
                                                102, 117, 110, 32, 109, 108, 95, 112, 114, 101,
                                                118, 105, 111, 117, 115, 32, 61, 62, 32, 105, 102,
                                                32,
                                            ]))
                                            .concat(&equal_4))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                32, 116, 104, 101, 110, 32, 40, 102, 115, 116, 32,
                                                109, 108, 95, 112, 114, 101, 118, 105, 111, 117,
                                                115, 44, 32, 115, 110, 100, 32, 109, 108, 95, 118,
                                                97, 108, 117, 101, 41, 32, 101, 108, 115, 101, 32,
                                                109, 108, 95, 112, 114, 101, 118, 105, 111, 117,
                                                115, 41, 32, 109, 108, 95, 97, 99, 99, 117, 109,
                                                117, 108, 97, 116, 111, 114, 41,
                                            ]))
                                        };
                                        ((((((((((self::ml_string::Text::from_units(vec![
                                            40, 76, 105, 115, 116, 46, 102, 111, 108, 100, 95, 108,
                                            101, 102, 116, 32, 40, 102, 117, 110, 32, 109, 108, 95,
                                            97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 32,
                                            109, 108, 95, 111, 114, 105, 103, 105, 110, 97, 108,
                                            32, 61, 62, 32, 108, 101, 116, 32, 109, 108, 95, 118,
                                            97, 108, 117, 101, 32, 58, 61, 32,
                                        ]))
                                        .concat(&normalized_2))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 105, 110, 32, 105, 102, 32, 76, 105, 115, 116, 46,
                                            101, 120, 105, 115, 116, 115, 98, 32, 40, 102, 117,
                                            110, 32, 109, 108, 95, 112, 114, 101, 118, 105, 111,
                                            117, 115, 32, 61, 62, 32,
                                        ])))
                                        .concat(&equal_4))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            41, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108,
                                            97, 116, 111, 114, 32, 116, 104, 101, 110, 32,
                                        ])))
                                        .concat(&existing_2))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 101, 108, 115, 101, 32, 76, 105, 115, 116, 46, 97,
                                            112, 112, 32, 109, 108, 95, 97, 99, 99, 117, 109, 117,
                                            108, 97, 116, 111, 114, 32, 40, 109, 108, 95, 118, 97,
                                            108, 117, 101, 32, 58, 58, 32, 110, 105, 108, 41, 41,
                                            32,
                                        ])))
                                        .concat(values))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 40, 64, 110, 105, 108, 32,
                                        ])))
                                        .concat(element_type))
                                        .concat(&self::ml_string::Text::from_units(vec![41, 41]))
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
