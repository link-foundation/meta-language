// Generated from complete js/src/translation/javascript-property-key-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result;
use super::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3;
use super::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result;
use super::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_property_descriptor_kind;
use super::source_ml_temporary_find_java_script_ordinary_object_loop5;
use super::source_ml_temporary_read_java_script_literal_property_index_loop4;
use super::source_ml_temporary_test_java_script_ordinary_symbol_loop6;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_property_descriptor(
    kind: &self::ml_string::Text,
    configurable: bool,
    enumerable: bool,
) -> (
    bool,
    bool,
    bool,
    bool,
    self::JavaScriptSemanticValue,
    bool,
    self::JavaScriptSemanticValue,
    bool,
    self::JavaScriptSemanticValue,
    bool,
    bool,
    bool,
) {
    {
        let source_ml_source1_field107 = true;
        {
            let source_ml_source1_field108 = configurable;
            {
                let source_ml_source1_field109 = true;
                {
                    let source_ml_source1_field110 = enumerable;
                    {
                        let source_ml_source1_field111 = (*kind)
                            != (self::ml_string::Text::from_units(vec![
                                97, 99, 99, 101, 115, 115, 111, 114,
                            ]));
                        {
                            let source_ml_source1_field112 =
                                self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field113 = (*kind)
                                    != (self::ml_string::Text::from_units(vec![
                                        97, 99, 99, 101, 115, 115, 111, 114,
                                    ]));
                                {
                                    let source_ml_source1_field114 = false;
                                    {
                                        let source_ml_source1_field115 = (*kind)
                                            == (self::ml_string::Text::from_units(vec![
                                                97, 99, 99, 101, 115, 115, 111, 114,
                                            ]));
                                        {
                                            let source_ml_source1_field116 =
                                                self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field117 = (*kind)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        97, 99, 99, 101, 115, 115, 111, 114,
                                                    ]));
                                                {
                                                    let source_ml_source1_field118 =
                                                        self::JavaScriptSemanticValue::Undefined;
                                                    (
                                                        source_ml_source1_field108,
                                                        source_ml_source1_field107,
                                                        source_ml_source1_field110,
                                                        source_ml_source1_field109,
                                                        source_ml_source1_field116,
                                                        source_ml_source1_field115,
                                                        source_ml_source1_field118,
                                                        source_ml_source1_field117,
                                                        source_ml_source1_field112,
                                                        source_ml_source1_field111,
                                                        source_ml_source1_field114,
                                                        source_ml_source1_field113,
                                                    )
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn apply_java_script_property_descriptor_fields(
    base: (
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    ),
    proposed: (
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    ),
) -> (
    bool,
    bool,
    bool,
    bool,
    self::JavaScriptSemanticValue,
    bool,
    self::JavaScriptSemanticValue,
    bool,
    self::JavaScriptSemanticValue,
    bool,
    bool,
    bool,
) {
    {
        let source_ml_source1_field119 = true;
        {
            let source_ml_source1_field120 = if (proposed.clone()).1 {
                (proposed.clone()).0
            } else {
                (base.clone()).0
            };
            {
                let source_ml_source1_field121 = true;
                {
                    let source_ml_source1_field122 = if (proposed.clone()).3 {
                        (proposed.clone()).2
                    } else {
                        (base.clone()).2
                    };
                    {
                        let source_ml_source1_field123 = (base.clone()).9;
                        {
                            let source_ml_source1_field124 =
                                if (base.clone()).9 && (proposed.clone()).9 {
                                    (proposed.clone()).8
                                } else {
                                    (base.clone()).8
                                };
                            {
                                let source_ml_source1_field125 = (base.clone()).11;
                                {
                                    let source_ml_source1_field126 =
                                        if (base.clone()).11 && (proposed.clone()).11 {
                                            (proposed.clone()).10
                                        } else {
                                            (base.clone()).10
                                        };
                                    {
                                        let source_ml_source1_field127 = (base.clone()).5;
                                        {
                                            let source_ml_source1_field128 =
                                                if (base.clone()).5 && (proposed.clone()).5 {
                                                    (proposed.clone()).4
                                                } else {
                                                    (base.clone()).4
                                                };
                                            {
                                                let source_ml_source1_field129 = (base.clone()).7;
                                                {
                                                    let source_ml_source1_field130 =
                                                        if (base.clone()).7 && (proposed.clone()).7
                                                        {
                                                            (proposed.clone()).6
                                                        } else {
                                                            (base.clone()).6
                                                        };
                                                    (
                                                        source_ml_source1_field120,
                                                        source_ml_source1_field119,
                                                        source_ml_source1_field122,
                                                        source_ml_source1_field121,
                                                        source_ml_source1_field128,
                                                        source_ml_source1_field127,
                                                        source_ml_source1_field130,
                                                        source_ml_source1_field129,
                                                        source_ml_source1_field124,
                                                        source_ml_source1_field123,
                                                        source_ml_source1_field126,
                                                        source_ml_source1_field125,
                                                    )
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_property_descriptor_writes(
    descriptor: (
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    ),
) -> Vec<self::ml_string::Text> {
    {
        let writes = Vec::<self::ml_string::Text>::new();
        {
            let writes_3 = if (descriptor.clone()).1 {
                {
                    {
                        let source_ml_source1_concat_value131 = writes;
                        {
                            let source_ml_source1_concat_value132 =
                                vec![self::ml_string::Text::from_units(vec![
                                    99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101,
                                ])];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<self::ml_string::Text>::new(),
                                    source_ml_source1_concat_value131,
                                ),
                                source_ml_source1_concat_value132,
                            )
                        }
                    }
                }
            } else {
                writes
            };
            {
                let writes_5 = if (descriptor.clone()).3 {
                    {
                        {
                            let source_ml_source1_concat_value133 = writes_3;
                            {
                                let source_ml_source1_concat_value134 =
                                    vec![self::ml_string::Text::from_units(vec![
                                        101, 110, 117, 109, 101, 114, 97, 98, 108, 101,
                                    ])];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value133,
                                    ),
                                    source_ml_source1_concat_value134,
                                )
                            }
                        }
                    }
                } else {
                    writes_3
                };
                {
                    let writes_7 = if (descriptor.clone()).9 {
                        {
                            {
                                let source_ml_source1_concat_value135 = writes_5;
                                {
                                    let source_ml_source1_concat_value136 =
                                        vec![self::ml_string::Text::from_units(vec![
                                            118, 97, 108, 117, 101,
                                        ])];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            source_ml_source1_concat_value135,
                                        ),
                                        source_ml_source1_concat_value136,
                                    )
                                }
                            }
                        }
                    } else {
                        writes_5
                    };
                    {
                        let writes_9 = if (descriptor.clone()).11 {
                            {
                                {
                                    let source_ml_source1_concat_value137 = writes_7;
                                    {
                                        let source_ml_source1_concat_value138 =
                                            vec![self::ml_string::Text::from_units(vec![
                                                119, 114, 105, 116, 97, 98, 108, 101,
                                            ])];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
                                                source_ml_source1_concat_value137,
                                            ),
                                            source_ml_source1_concat_value138,
                                        )
                                    }
                                }
                            }
                        } else {
                            writes_7
                        };
                        {
                            let writes_11 = if (descriptor.clone()).5 {
                                {
                                    {
                                        let source_ml_source1_concat_value139 = writes_9;
                                        {
                                            let source_ml_source1_concat_value140 =
                                                vec![self::ml_string::Text::from_units(vec![
                                                    103, 101, 116,
                                                ])];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    source_ml_source1_concat_value139,
                                                ),
                                                source_ml_source1_concat_value140,
                                            )
                                        }
                                    }
                                }
                            } else {
                                writes_9
                            };
                            {
                                if (descriptor.clone()).7 {
                                    {
                                        {
                                            let source_ml_source1_concat_value141 = writes_11;
                                            {
                                                let source_ml_source1_concat_value142 =
                                                    vec![self::ml_string::Text::from_units(vec![
                                                        115, 101, 116,
                                                    ])];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<self::ml_string::Text>::new(),
                                                        source_ml_source1_concat_value141,
                                                    ),
                                                    source_ml_source1_concat_value142,
                                                )
                                            }
                                        }
                                    }
                                } else {
                                    writes_11
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn create_java_script_property_transition(
    disposition: &self::ml_string::Text,
    error: &self::ml_string::Text,
    descriptor: (
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    ),
    writes: Vec<self::ml_string::Text>,
) -> (
    (
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    ),
    self::ml_string::Text,
    self::ml_string::Text,
    Vec<self::ml_string::Text>,
) {
    {
        let source_ml_source1_field143 = disposition.clone();
        {
            let source_ml_source1_field144 = error.clone();
            {
                let source_ml_source1_field145 = descriptor.clone();
                {
                    let source_ml_source1_field146 = writes;
                    (
                        source_ml_source1_field145,
                        source_ml_source1_field143,
                        source_ml_source1_field144,
                        source_ml_source1_field146,
                    )
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
pub fn plan_java_script_property_definition(
    exists: bool,
    extensible: bool,
    current: (
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    ),
    proposed: (
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    ),
) -> (
    (
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    ),
    self::ml_string::Text,
    self::ml_string::Text,
    Vec<self::ml_string::Text>,
) {
    {
        let error = self::read_java_script_property_descriptor_error(proposed.clone());
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                self::create_java_script_property_transition(
                    &(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])),
                    &(error),
                    current.clone(),
                    Vec::<self::ml_string::Text>::new(),
                )
            } else if exists
                && !(self::test_complete_java_script_property_descriptor(current.clone()))
            {
                self::create_java_script_property_transition(
                    &(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])),
                    &(self::ml_string::Text::from_units(vec![
                        73, 110, 118, 97, 108, 105, 100, 67, 117, 114, 114, 101, 110, 116, 68, 101,
                        115, 99, 114, 105, 112, 116, 111, 114,
                    ])),
                    current.clone(),
                    Vec::<self::ml_string::Text>::new(),
                )
            } else {
                {
                    let kind = self::read_java_script_property_descriptor_kind(proposed.clone());
                    if exists {
                        if ((((!((proposed.clone()).1) && !((proposed.clone()).3))
                            && !((proposed.clone()).9))
                            && !((proposed.clone()).11))
                            && !((proposed.clone()).5))
                            && !((proposed.clone()).7)
                        {
                            self::create_java_script_property_transition(
                                &(self::ml_string::Text::from_units(vec![107, 101, 101, 112])),
                                &(self::ml_string::Text::from_units(vec![])),
                                current.clone(),
                                Vec::<self::ml_string::Text>::new(),
                            )
                        } else {
                            {
                                let original = self::read_java_script_property_descriptor_kind(
                                    current.clone(),
                                );
                                {
                                    let source_ml_source1_s147 = if ((current.clone()).0) {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next
                                    } else if (proposed.clone()).1 && (proposed.clone()).0 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                    } else if (proposed.clone()).3
                                        && (((proposed.clone()).2) != ((current.clone()).2))
                                    {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                    } else if (kind
                                        != (self::ml_string::Text::from_units(vec![
                                            103, 101, 110, 101, 114, 105, 99,
                                        ])))
                                        && (kind != original)
                                    {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                    } else if original
                                        == (self::ml_string::Text::from_units(vec![
                                            97, 99, 99, 101, 115, 115, 111, 114,
                                        ]))
                                    {
                                        if (proposed.clone()).5
                                            && !(self::test_java_script_semantic_same_value(
                                                (proposed.clone()).4,
                                                (current.clone()).4,
                                            ))
                                        {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                        } else if (proposed.clone()).7
                                            && !(self::test_java_script_semantic_same_value(
                                                (proposed.clone()).6,
                                                (current.clone()).6,
                                            ))
                                        {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                        } else {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next
                                        }
                                    } else if ((current.clone()).10) {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next
                                    } else if (proposed.clone()).11 && (proposed.clone()).10 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                    } else if (proposed.clone()).9 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value((proposed.clone()).8, (current.clone()).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                    } else {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next
                                    };
                                    match source_ml_source1_s147 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next => {
                                            {
                                                let base_2 = if (kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original) {
                                                    self::create_java_script_property_descriptor(&(kind), (current.clone()).0, (current.clone()).2)
                                                } else {
                                                    current.clone()
                                                };
                                                {
                                                    let descriptor_2 = self::apply_java_script_property_descriptor_fields(base_2, proposed.clone());
                                                    {
                                                        let conversion = (kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original);
                                                        self::create_java_script_property_transition(&(if conversion {
                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![117, 112, 100, 97, 116, 101])
                                                        }), &(self::ml_string::Text::from_units(vec![])), descriptor_2.clone(), self::read_java_script_property_descriptor_writes(if conversion {
                                                            descriptor_2.clone()
                                                        } else {
                                                            proposed.clone()
                                                        }))
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(source_ml_temporary_result3) => {
                                            source_ml_temporary_result3.clone()
                                        }
                                    }
                                }
                            }
                        }
                    } else if extensible {
                        {
                            let base =
                                self::create_java_script_property_descriptor(&(kind), false, false);
                            {
                                let descriptor = self::apply_java_script_property_descriptor_fields(
                                    base,
                                    proposed.clone(),
                                );
                                self::create_java_script_property_transition(
                                    &(self::ml_string::Text::from_units(vec![
                                        99, 114, 101, 97, 116, 101,
                                    ])),
                                    &(self::ml_string::Text::from_units(vec![])),
                                    descriptor.clone(),
                                    self::read_java_script_property_descriptor_writes(
                                        descriptor.clone(),
                                    ),
                                )
                            }
                        }
                    } else {
                        self::create_java_script_property_transition(
                            &(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])),
                            &(self::ml_string::Text::from_units(vec![])),
                            current.clone(),
                            Vec::<self::ml_string::Text>::new(),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_literal_property_index(key: &[f64]) -> f64 {
    if ((key.len() as f64) == 0f64) || ((key.len() as f64) > 10f64) {
        -1f64
    } else {
        if ((key.len() as f64) > 1f64) && {
            let source_ml_source1_comparison_operand148 =
                self::ml_array::optional_at(key, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand149 = 48f64;
                match (
                    source_ml_source1_comparison_operand148,
                    source_ml_source1_comparison_operand149,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } {
            -1f64
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s150 =
                            self::source_ml_temporary_read_java_script_literal_property_index_loop4(
                                key, value, at,
                            );
                        match source_ml_source1_s150 {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Done(value_2) => {
                                if value_2 <= 4_294_967_294f64 {
                                    value_2
                                } else {
                                    -1f64
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Return(source_ml_temporary_result4) => {
                                source_ml_temporary_result4
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
pub fn read_java_script_literal_property_form(
    kind: &self::ml_string::Text,
    index: f64,
) -> self::ml_string::Text {
    if ((((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
        && (index >= 0f64)
    {
        self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
    } else {
        self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_index_number_form(
    kind: &self::ml_string::Text,
    element: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![105, 110, 116])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![110, 97, 116])))
    {
        self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))
        || (((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
            && ((*element) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))
    {
        self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
        && ((((*element) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))
            || ((*element) == (self::ml_string::Text::from_units(vec![105, 110, 116]))))
            || ((*element) == (self::ml_string::Text::from_units(vec![110, 97, 116]))))
    {
        self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
    } else {
        self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_ordinary_object(
    objects: Vec<self::JavaScriptOrdinaryObject>,
    identity: self::ml::Big,
) -> f64 {
    {
        let low = 0f64;
        {
            let high = objects.len() as f64;
            {
                let source_ml_source1_s151 =
                    self::source_ml_temporary_find_java_script_ordinary_object_loop5(
                        objects,
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s151 {
                    self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Done => {
                        -1f64
                    }
                    self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(source_ml_temporary_result5) => {
                        source_ml_temporary_result5
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_ordinary_symbol(
    symbols: Vec<self::ml::Big>,
    identity: self::ml::Big,
) -> bool {
    {
        let low = 0f64;
        {
            let high = symbols.len() as f64;
            {
                let source_ml_source1_s152 =
                    self::source_ml_temporary_test_java_script_ordinary_symbol_loop6(
                        symbols,
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s152 {
                    self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Done => {
                        false
                    }
                    self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(source_ml_temporary_result6) => {
                        source_ml_temporary_result6
                    }
                }
            }
        }
    }
}
