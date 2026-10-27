// Generated from complete js/src/translation/javascript-property-descriptors.js. Do not edit by hand.
use super::JavaScriptSemanticValue;
use super::ml_array;
use super::ml_math;
use super::read_java_script_property_descriptor_kind;
use super::read_java_script_semantic_value_error;
use super::test_java_script_semantic_accessor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_property_descriptor_error(
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
) -> String {
    if (self::read_java_script_property_descriptor_kind(descriptor.clone())) == "invalid" {
        String::from("TypeError")
    } else if (descriptor.clone()).9
        && (!(self::read_java_script_semantic_value_error((descriptor.clone()).8)).is_empty())
    {
        String::from("InvalidSemanticValue")
    } else if ((descriptor.clone()).5
        && ((!(self::read_java_script_semantic_value_error((descriptor.clone()).4)).is_empty())
            || !(self::test_java_script_semantic_accessor((descriptor.clone()).4))))
        || ((descriptor.clone()).7
            && ((!(self::read_java_script_semantic_value_error((descriptor.clone()).6))
                .is_empty())
                || !(self::test_java_script_semantic_accessor((descriptor.clone()).6))))
    {
        String::from("TypeError")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn test_complete_java_script_property_descriptor(
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
) -> bool {
    if (!((descriptor.clone()).1) || !((descriptor.clone()).3))
        || ((self::read_java_script_property_descriptor_error(descriptor.clone())) != "")
    {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if kind == "data" {
                (descriptor.clone()).9 && (descriptor.clone()).11
            } else {
                ((kind == "accessor") && (descriptor.clone()).5) && (descriptor.clone()).7
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_property_descriptor(
    kind: &str,
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
                        let source_ml_source1_field111 = kind != "accessor";
                        {
                            let source_ml_source1_field112 =
                                self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field113 = kind != "accessor";
                                {
                                    let source_ml_source1_field114 = false;
                                    {
                                        let source_ml_source1_field115 = kind == "accessor";
                                        {
                                            let source_ml_source1_field116 =
                                                self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field117 = kind == "accessor";
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
) -> Vec<String> {
    {
        let writes = Vec::<String>::new();
        {
            let writes_3 = if (descriptor.clone()).1 {
                {
                    {
                        let source_ml_source1_concat_value131 = writes;
                        {
                            let source_ml_source1_concat_value132 =
                                vec![String::from("configurable")];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<String>::new(),
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
                                    vec![String::from("enumerable")];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<String>::new(),
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
                                        vec![String::from("value")];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<String>::new(),
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
                                            vec![String::from("writable")];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<String>::new(),
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
                                                vec![String::from("get")];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<String>::new(),
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
                                                    vec![String::from("set")];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<String>::new(),
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
    disposition: &str,
    error: &str,
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
    writes: Vec<String>,
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
    String,
    String,
    Vec<String>,
) {
    {
        let source_ml_source1_field143 = disposition.to_owned();
        {
            let source_ml_source1_field144 = error.to_owned();
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
    String,
    String,
    Vec<String>,
) {
    {
        let error = self::read_java_script_property_descriptor_error(proposed.clone());
        {
            let ml_library_inequality_1 = error != "";
            if ml_library_inequality_1 {
                self::create_java_script_property_transition(
                    "reject",
                    &(error),
                    current.clone(),
                    Vec::<String>::new(),
                )
            } else if exists
                && !(self::test_complete_java_script_property_descriptor(current.clone()))
            {
                self::create_java_script_property_transition(
                    "reject",
                    "InvalidCurrentDescriptor",
                    current.clone(),
                    Vec::<String>::new(),
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
                                "keep",
                                "",
                                current.clone(),
                                Vec::<String>::new(),
                            )
                        } else {
                            {
                                let original = self::read_java_script_property_descriptor_kind(
                                    current.clone(),
                                );
                                {
                                    let source_ml_source1_s147 = if ((current.clone()).0) {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next
                                    } else if (((proposed.clone()).1 && (proposed.clone()).0)
                                        || ((proposed.clone()).3
                                            && (((proposed.clone()).2) != ((current.clone()).2))))
                                        || ((kind != "generic") && (kind != original))
                                    {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition("reject", "", current.clone(), Vec::<String>::new()))
                                    } else if original == "accessor" {
                                        if ((proposed.clone()).5
                                            && !(self::test_java_script_semantic_same_value(
                                                (proposed.clone()).4,
                                                (current.clone()).4,
                                            )))
                                            || ((proposed.clone()).7
                                                && !(self::test_java_script_semantic_same_value(
                                                    (proposed.clone()).6,
                                                    (current.clone()).6,
                                                )))
                                        {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition("reject", "", current.clone(), Vec::<String>::new()))
                                        } else {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next
                                        }
                                    } else if ((current.clone()).10) {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next
                                    } else if (proposed.clone()).11 && (proposed.clone()).10 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition("reject", "", current.clone(), Vec::<String>::new()))
                                    } else if (proposed.clone()).9 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value((proposed.clone()).8, (current.clone()).8) {
                                                                        String::from("keep")
                                                                    } else {
                                                                        String::from("reject")
                                                                    }), "", current.clone(), Vec::<String>::new()))
                                    } else {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next
                                    };
                                    match source_ml_source1_s147 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3::MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next => {
                                            {
                                                let base_2 = if (kind != "generic") && (kind != original) {
                                                    self::create_java_script_property_descriptor(&(kind), (current.clone()).0, (current.clone()).2)
                                                } else {
                                                    current.clone()
                                                };
                                                {
                                                    let descriptor_2 = self::apply_java_script_property_descriptor_fields(base_2, proposed.clone());
                                                    {
                                                        let conversion = (kind != "generic") && (kind != original);
                                                        self::create_java_script_property_transition(&(if conversion {
                                                            String::from("convert")
                                                        } else {
                                                            String::from("update")
                                                        }), "", descriptor_2.clone(), self::read_java_script_property_descriptor_writes(if conversion {
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
                                    "create",
                                    "",
                                    descriptor.clone(),
                                    self::read_java_script_property_descriptor_writes(
                                        descriptor.clone(),
                                    ),
                                )
                            }
                        }
                    } else {
                        self::create_java_script_property_transition(
                            "reject",
                            "",
                            current.clone(),
                            Vec::<String>::new(),
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
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String),
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
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String::from("semantic text requires complete ordered UTF16 code units"))
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
pub enum MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_same_value_loop2(
    left_units: &[f64],
    right_units: &[f64],
    mut at: f64,
) -> self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    loop {
        return if at < (left_units.len() as f64) {
            {
                let ml_library_inequality_1 =
                    ((self::ml_array::optional_at(left_units, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| -1f64))
                        != ((self::ml_array::optional_at(
                            right_units,
                            self::ml_array::number_index(at),
                        ))
                        .unwrap_or_else(|| -1f64));
                if ml_library_inequality_1 {
                    self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(false)
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
            self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin3 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(
        (
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
            String,
            String,
            Vec<String>,
        ),
    ),
}
