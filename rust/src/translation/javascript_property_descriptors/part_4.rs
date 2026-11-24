// Generated from complete js/src/translation/javascript-property-descriptors.js. Do not edit by hand.
use super::JavaScriptSemanticValue;
use super::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_property_descriptor_kind;
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
        let source_ml_source1_field302 = true;
        {
            let source_ml_source1_field303 = configurable;
            {
                let source_ml_source1_field304 = true;
                {
                    let source_ml_source1_field305 = enumerable;
                    {
                        let source_ml_source1_field306 = (*kind)
                            != (self::ml_string::Text::from_units(vec![
                                97, 99, 99, 101, 115, 115, 111, 114,
                            ]));
                        {
                            let source_ml_source1_field307 =
                                self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field308 = (*kind)
                                    != (self::ml_string::Text::from_units(vec![
                                        97, 99, 99, 101, 115, 115, 111, 114,
                                    ]));
                                {
                                    let source_ml_source1_field309 = false;
                                    {
                                        let source_ml_source1_field310 = (*kind)
                                            == (self::ml_string::Text::from_units(vec![
                                                97, 99, 99, 101, 115, 115, 111, 114,
                                            ]));
                                        {
                                            let source_ml_source1_field311 =
                                                self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field312 = (*kind)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        97, 99, 99, 101, 115, 115, 111, 114,
                                                    ]));
                                                {
                                                    let source_ml_source1_field313 =
                                                        self::JavaScriptSemanticValue::Undefined;
                                                    (
                                                        source_ml_source1_field303,
                                                        source_ml_source1_field302,
                                                        source_ml_source1_field305,
                                                        source_ml_source1_field304,
                                                        source_ml_source1_field311,
                                                        source_ml_source1_field310,
                                                        source_ml_source1_field313,
                                                        source_ml_source1_field312,
                                                        source_ml_source1_field307,
                                                        source_ml_source1_field306,
                                                        source_ml_source1_field309,
                                                        source_ml_source1_field308,
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
        let source_ml_source1_field314 = true;
        {
            let source_ml_source1_field315 = if (proposed.clone()).1 {
                (proposed.clone()).0
            } else {
                (base.clone()).0
            };
            {
                let source_ml_source1_field316 = true;
                {
                    let source_ml_source1_field317 = if (proposed.clone()).3 {
                        (proposed.clone()).2
                    } else {
                        (base.clone()).2
                    };
                    {
                        let source_ml_source1_field318 = (base.clone()).9;
                        {
                            let source_ml_source1_field319 =
                                if (base.clone()).9 && (proposed.clone()).9 {
                                    (proposed.clone()).8
                                } else {
                                    (base.clone()).8
                                };
                            {
                                let source_ml_source1_field320 = (base.clone()).11;
                                {
                                    let source_ml_source1_field321 =
                                        if (base.clone()).11 && (proposed.clone()).11 {
                                            (proposed.clone()).10
                                        } else {
                                            (base.clone()).10
                                        };
                                    {
                                        let source_ml_source1_field322 = (base.clone()).5;
                                        {
                                            let source_ml_source1_field323 =
                                                if (base.clone()).5 && (proposed.clone()).5 {
                                                    (proposed.clone()).4
                                                } else {
                                                    (base.clone()).4
                                                };
                                            {
                                                let source_ml_source1_field324 = (base.clone()).7;
                                                {
                                                    let source_ml_source1_field325 =
                                                        if (base.clone()).7 && (proposed.clone()).7
                                                        {
                                                            (proposed.clone()).6
                                                        } else {
                                                            (base.clone()).6
                                                        };
                                                    (
                                                        source_ml_source1_field315,
                                                        source_ml_source1_field314,
                                                        source_ml_source1_field317,
                                                        source_ml_source1_field316,
                                                        source_ml_source1_field323,
                                                        source_ml_source1_field322,
                                                        source_ml_source1_field325,
                                                        source_ml_source1_field324,
                                                        source_ml_source1_field319,
                                                        source_ml_source1_field318,
                                                        source_ml_source1_field321,
                                                        source_ml_source1_field320,
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
                        let source_ml_source1_concat_value326 = writes;
                        {
                            let source_ml_source1_concat_value327 =
                                vec![self::ml_string::Text::from_units(vec![
                                    99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101,
                                ])];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<self::ml_string::Text>::new(),
                                    source_ml_source1_concat_value326,
                                ),
                                source_ml_source1_concat_value327,
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
                            let source_ml_source1_concat_value328 = writes_3;
                            {
                                let source_ml_source1_concat_value329 =
                                    vec![self::ml_string::Text::from_units(vec![
                                        101, 110, 117, 109, 101, 114, 97, 98, 108, 101,
                                    ])];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source1_concat_value328,
                                    ),
                                    source_ml_source1_concat_value329,
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
                                let source_ml_source1_concat_value330 = writes_5;
                                {
                                    let source_ml_source1_concat_value331 =
                                        vec![self::ml_string::Text::from_units(vec![
                                            118, 97, 108, 117, 101,
                                        ])];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            source_ml_source1_concat_value330,
                                        ),
                                        source_ml_source1_concat_value331,
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
                                    let source_ml_source1_concat_value332 = writes_7;
                                    {
                                        let source_ml_source1_concat_value333 =
                                            vec![self::ml_string::Text::from_units(vec![
                                                119, 114, 105, 116, 97, 98, 108, 101,
                                            ])];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
                                                source_ml_source1_concat_value332,
                                            ),
                                            source_ml_source1_concat_value333,
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
                                        let source_ml_source1_concat_value334 = writes_9;
                                        {
                                            let source_ml_source1_concat_value335 =
                                                vec![self::ml_string::Text::from_units(vec![
                                                    103, 101, 116,
                                                ])];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    source_ml_source1_concat_value334,
                                                ),
                                                source_ml_source1_concat_value335,
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
                                            let source_ml_source1_concat_value336 = writes_11;
                                            {
                                                let source_ml_source1_concat_value337 =
                                                    vec![self::ml_string::Text::from_units(vec![
                                                        115, 101, 116,
                                                    ])];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<self::ml_string::Text>::new(),
                                                        source_ml_source1_concat_value336,
                                                    ),
                                                    source_ml_source1_concat_value337,
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
        let source_ml_source1_field338 = disposition.clone();
        {
            let source_ml_source1_field339 = error.clone();
            {
                let source_ml_source1_field340 = descriptor.clone();
                {
                    let source_ml_source1_field341 = writes;
                    (
                        source_ml_source1_field340,
                        source_ml_source1_field338,
                        source_ml_source1_field339,
                        source_ml_source1_field341,
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
                                    let source_ml_source1_s342 = if ((current.clone()).0) {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Next
                                    } else if (proposed.clone()).1 && (proposed.clone()).0 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                    } else if (proposed.clone()).3
                                        && (((proposed.clone()).2) != ((current.clone()).2))
                                    {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                    } else if (kind
                                        != (self::ml_string::Text::from_units(vec![
                                            103, 101, 110, 101, 114, 105, 99,
                                        ])))
                                        && (kind != original)
                                    {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
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
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                        } else if (proposed.clone()).7
                                            && !(self::test_java_script_semantic_same_value(
                                                (proposed.clone()).6,
                                                (current.clone()).6,
                                            ))
                                        {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                        } else {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Next
                                        }
                                    } else if ((current.clone()).10) {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Next
                                    } else if (proposed.clone()).11 && (proposed.clone()).10 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                    } else if (proposed.clone()).9 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value((proposed.clone()).8, (current.clone()).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }), &(self::ml_string::Text::from_units(vec![])), current.clone(), Vec::<self::ml_string::Text>::new()))
                                    } else {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Next
                                    };
                                    match source_ml_source1_s342 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Next => {
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
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6::MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Return(source_ml_temporary_result6) => {
                                            source_ml_temporary_result6.clone()
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(
    units: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if at < (units.len() as f64) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done,
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(bool),
}
