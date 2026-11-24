// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result;
use super::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10;
use super::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Result;
use super::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Result;
use super::apply_java_script_property_descriptor_fields;
use super::create_java_script_property_descriptor;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_property_descriptor_kind;
use super::source_ml_source1_temporary_find_java_script_heap_directory_object_loop12;
use super::source_ml_source1_temporary_read_java_script_literal_property_index_loop11;
use super::source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop13;
use super::source_ml_source3_record_order;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_descriptor_writes(
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
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
            Vec<f64>,
        )>,
    >,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    {
        let writes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
        {
            let writes_3 = if (({
                let ml_reference_value82 = descriptor.borrow().clone();
                ml_reference_value82
            })
            .0)
                .1
            {
                {
                    let writes_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value75 =
                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                self::ml_string::Text::from_units(vec![
                                    99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101,
                                ]),
                            ]));
                        self::ml_array::append(
                            self::ml_array::append(Vec::<self::ml_string::Text>::new(), {
                                let ml_reference_value83 = writes.borrow().clone();
                                ml_reference_value83
                            }),
                            {
                                let ml_reference_value84 =
                                    source_ml_source2_concat_value75.borrow().clone();
                                ml_reference_value84
                            },
                        )
                    }));
                    writes_2.clone()
                }
            } else {
                writes.clone()
            };
            {
                let writes_5 = if (({
                    let ml_reference_value85 = descriptor.borrow().clone();
                    ml_reference_value85
                })
                .0)
                    .3
                {
                    {
                        let writes_4 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_concat_value77 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    self::ml_string::Text::from_units(vec![
                                        101, 110, 117, 109, 101, 114, 97, 98, 108, 101,
                                    ]),
                                ]));
                            self::ml_array::append(
                                self::ml_array::append(Vec::<self::ml_string::Text>::new(), {
                                    let ml_reference_value86 = writes_3.borrow().clone();
                                    ml_reference_value86
                                }),
                                {
                                    let ml_reference_value87 =
                                        source_ml_source2_concat_value77.borrow().clone();
                                    ml_reference_value87
                                },
                            )
                        }));
                        writes_4.clone()
                    }
                } else {
                    writes_3.clone()
                };
                {
                    let writes_7 = if (({
                        let ml_reference_value88 = descriptor.borrow().clone();
                        ml_reference_value88
                    })
                    .0)
                        .9
                    {
                        {
                            let writes_6 = std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_concat_value79 =
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            118, 97, 108, 117, 101,
                                        ]),
                                    ]));
                                self::ml_array::append(
                                    self::ml_array::append(Vec::<self::ml_string::Text>::new(), {
                                        let ml_reference_value89 = writes_5.borrow().clone();
                                        ml_reference_value89
                                    }),
                                    {
                                        let ml_reference_value90 =
                                            source_ml_source2_concat_value79.borrow().clone();
                                        ml_reference_value90
                                    },
                                )
                            }));
                            writes_6.clone()
                        }
                    } else {
                        writes_5.clone()
                    };
                    {
                        let writes_9 = if (({
                            let ml_reference_value91 = descriptor.borrow().clone();
                            ml_reference_value91
                        })
                        .0)
                            .11
                        {
                            {
                                let writes_8 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_concat_value81 =
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            self::ml_string::Text::from_units(vec![
                                                119, 114, 105, 116, 97, 98, 108, 101,
                                            ]),
                                        ]));
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            {
                                                let ml_reference_value92 =
                                                    writes_7.borrow().clone();
                                                ml_reference_value92
                                            },
                                        ),
                                        {
                                            let ml_reference_value93 =
                                                source_ml_source2_concat_value81.borrow().clone();
                                            ml_reference_value93
                                        },
                                    )
                                }));
                                writes_8.clone()
                            }
                        } else {
                            writes_7.clone()
                        };
                        {
                            let writes_11 = if (({
                                let ml_reference_value94 = descriptor.borrow().clone();
                                ml_reference_value94
                            })
                            .0)
                                .5
                            {
                                {
                                    let writes_10 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_concat_value83 =
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                self::ml_string::Text::from_units(vec![
                                                    103, 101, 116,
                                                ]),
                                            ]));
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
                                                {
                                                    let ml_reference_value95 =
                                                        writes_9.borrow().clone();
                                                    ml_reference_value95
                                                },
                                            ),
                                            {
                                                let ml_reference_value96 =
                                                    source_ml_source2_concat_value83
                                                        .borrow()
                                                        .clone();
                                                ml_reference_value96
                                            },
                                        )
                                    }));
                                    writes_10.clone()
                                }
                            } else {
                                writes_9.clone()
                            };
                            {
                                let writes_13 = if (({
                                    let ml_reference_value97 = descriptor.borrow().clone();
                                    ml_reference_value97
                                })
                                .0)
                                    .7
                                {
                                    {
                                        let writes_12 =
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source2_concat_value85 =
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        vec![self::ml_string::Text::from_units(
                                                            vec![115, 101, 116],
                                                        )],
                                                    ));
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<self::ml_string::Text>::new(),
                                                        {
                                                            let ml_reference_value98 =
                                                                writes_11.borrow().clone();
                                                            ml_reference_value98
                                                        },
                                                    ),
                                                    {
                                                        let ml_reference_value99 =
                                                            source_ml_source2_concat_value85
                                                                .borrow()
                                                                .clone();
                                                        ml_reference_value99
                                                    },
                                                )
                                            }));
                                        writes_12.clone()
                                    }
                                } else {
                                    writes_11.clone()
                                };
                                writes_13.clone()
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
pub fn create_java_script_property_transition(
    disposition: &self::ml_string::Text,
    error: &self::ml_string::Text,
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
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
            Vec<f64>,
        )>,
    >,
    writes: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (
            std::rc::Rc<
                std::cell::RefCell<(
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
                    Vec<f64>,
                )>,
            >,
            self::ml_string::Text,
            self::ml_string::Text,
            std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
        ),
        Vec<f64>,
    )>,
> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source3_record_field25 = descriptor.clone();
        {
            let source_ml_source3_record_field26 = disposition.clone();
            {
                let source_ml_source3_record_field27 = error.clone();
                {
                    let source_ml_source3_record_field28 = writes.clone();
                    ((source_ml_source3_record_field25.clone(), source_ml_source3_record_field26.clone(), source_ml_source3_record_field27.clone(), source_ml_source3_record_field28.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![100f64, 101f64, 115f64, 99f64, 114f64, 105f64, 112f64, 116f64, 111f64, 114f64], vec![100f64, 105f64, 115f64, 112f64, 111f64, 115f64, 105f64, 116f64, 105f64, 111f64, 110f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![119f64, 114f64, 105f64, 116f64, 101f64, 115f64]], vec![1f64, 2f64, 0f64, 3f64]))
                }
            }
        }
    }))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_property_definition(
    exists: bool,
    extensible: bool,
    current: std::rc::Rc<
        std::cell::RefCell<(
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
            Vec<f64>,
        )>,
    >,
    proposed: std::rc::Rc<
        std::cell::RefCell<(
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
            Vec<f64>,
        )>,
    >,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (
            std::rc::Rc<
                std::cell::RefCell<(
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
                    Vec<f64>,
                )>,
            >,
            self::ml_string::Text,
            self::ml_string::Text,
            std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
        ),
        Vec<f64>,
    )>,
> {
    {
        let error = self::read_java_script_property_descriptor_error(proposed.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::create_java_script_property_transition(
                &(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])),
                &(error),
                current.clone(),
                std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())),
            )
        } else {
            if (exists && !(self::test_complete_java_script_property_descriptor(current.clone()))) {
                self::create_java_script_property_transition(
                    &(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])),
                    &(self::ml_string::Text::from_units(vec![
                        73, 110, 118, 97, 108, 105, 100, 67, 117, 114, 114, 101, 110, 116, 68, 101,
                        115, 99, 114, 105, 112, 116, 111, 114,
                    ])),
                    current.clone(),
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())),
                )
            } else {
                {
                    let kind = self::read_java_script_property_descriptor_kind(proposed.clone());
                    if !exists {
                        if !extensible {
                            self::create_java_script_property_transition(
                                &(self::ml_string::Text::from_units(vec![
                                    114, 101, 106, 101, 99, 116,
                                ])),
                                &(self::ml_string::Text::from_units(vec![])),
                                current.clone(),
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::ml_string::Text,
                                >::new(
                                ))),
                            )
                        } else {
                            {
                                let base = self::create_java_script_property_descriptor(
                                    &(kind),
                                    false,
                                    false,
                                );
                                {
                                    let descriptor =
                                        self::apply_java_script_property_descriptor_fields(
                                            base.clone(),
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
                        }
                    } else {
                        if (((((!((({
                            let ml_reference_value100 = proposed.borrow().clone();
                            ml_reference_value100
                        })
                        .0)
                            .1)
                            && !((({
                                let ml_reference_value101 = proposed.borrow().clone();
                                ml_reference_value101
                            })
                            .0)
                                .3))
                            && !((({
                                let ml_reference_value102 = proposed.borrow().clone();
                                ml_reference_value102
                            })
                            .0)
                                .9))
                            && !((({
                                let ml_reference_value103 = proposed.borrow().clone();
                                ml_reference_value103
                            })
                            .0)
                                .11))
                            && !((({
                                let ml_reference_value104 = proposed.borrow().clone();
                                ml_reference_value104
                            })
                            .0)
                                .5))
                            && !((({
                                let ml_reference_value105 = proposed.borrow().clone();
                                ml_reference_value105
                            })
                            .0)
                                .7))
                        {
                            self::create_java_script_property_transition(
                                &(self::ml_string::Text::from_units(vec![107, 101, 101, 112])),
                                &(self::ml_string::Text::from_units(vec![])),
                                current.clone(),
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::ml_string::Text,
                                >::new(
                                ))),
                            )
                        } else {
                            {
                                let original = self::read_java_script_property_descriptor_kind(
                                    current.clone(),
                                );
                                {
                                    let source_ml_source2_s90 = if !((({
                                        let ml_reference_value106 = current.borrow().clone();
                                        ml_reference_value106
                                    })
                                    .0)
                                        .0)
                                    {
                                        if ((({
                                            let ml_reference_value107 = proposed.borrow().clone();
                                            ml_reference_value107
                                        })
                                        .0)
                                            .1
                                            && (({
                                                let ml_reference_value108 =
                                                    proposed.borrow().clone();
                                                ml_reference_value108
                                            })
                                            .0)
                                                .0)
                                        {
                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                        } else {
                                            if ((({
                                                let ml_reference_value109 =
                                                    proposed.borrow().clone();
                                                ml_reference_value109
                                            })
                                            .0)
                                                .3
                                                && (((({
                                                    let ml_reference_value110 =
                                                        proposed.borrow().clone();
                                                    ml_reference_value110
                                                })
                                                .0)
                                                    .2)
                                                    != ((({
                                                        let ml_reference_value111 =
                                                            current.borrow().clone();
                                                        ml_reference_value111
                                                    })
                                                    .0)
                                                        .2)))
                                            {
                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                            } else {
                                                if ((kind
                                                    != (self::ml_string::Text::from_units(vec![
                                                        103, 101, 110, 101, 114, 105, 99,
                                                    ])))
                                                    && (kind != original))
                                                {
                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                } else {
                                                    if (original
                                                        == (self::ml_string::Text::from_units(
                                                            vec![
                                                                97, 99, 99, 101, 115, 115, 111, 114,
                                                            ],
                                                        )))
                                                    {
                                                        if ((({ let ml_reference_value112 = proposed.borrow().clone(); ml_reference_value112 }).0).5 && !(self::test_java_script_semantic_same_value((({ let ml_reference_value113 = proposed.borrow().clone(); ml_reference_value113 }).0).4, (({ let ml_reference_value114 = current.borrow().clone(); ml_reference_value114 }).0).4))) {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                        } else {
                                                            if ((({ let ml_reference_value115 = proposed.borrow().clone(); ml_reference_value115 }).0).7 && !(self::test_java_script_semantic_same_value((({ let ml_reference_value116 = proposed.borrow().clone(); ml_reference_value116 }).0).6, (({ let ml_reference_value117 = current.borrow().clone(); ml_reference_value117 }).0).6))) {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Next
                                                            }
                                                        }
                                                    } else {
                                                        if !((({
                                                            let ml_reference_value118 =
                                                                current.borrow().clone();
                                                            ml_reference_value118
                                                        })
                                                        .0)
                                                            .10)
                                                        {
                                                            if ((({
                                                                let ml_reference_value119 =
                                                                    proposed.borrow().clone();
                                                                ml_reference_value119
                                                            })
                                                            .0)
                                                                .11
                                                                && (({
                                                                    let ml_reference_value120 =
                                                                        proposed.borrow().clone();
                                                                    ml_reference_value120
                                                                })
                                                                .0)
                                                                    .10)
                                                            {
                                                                self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                if (({
                                                                    let ml_reference_value121 =
                                                                        proposed.borrow().clone();
                                                                    ml_reference_value121
                                                                })
                                                                .0)
                                                                    .9
                                                                {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value((({ let ml_reference_value122 = proposed.borrow().clone(); ml_reference_value122 }).0).8, (({ let ml_reference_value123 = current.borrow().clone(); ml_reference_value123 }).0).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                                } else {
                                                                    self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Next
                                    };
                                    match source_ml_source2_s90.clone() {
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Next => {
                                            {
                                                let base_2 = if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::create_java_script_property_descriptor(&(kind), (({ let ml_reference_value124 = current.borrow().clone(); ml_reference_value124 }).0).0, (({ let ml_reference_value125 = current.borrow().clone(); ml_reference_value125 }).0).2)
                                                } else {
                                                    current.clone()
                                                };
                                                {
                                                    let descriptor_2 = self::apply_java_script_property_descriptor_fields(base_2.clone(), proposed.clone());
                                                    {
                                                        let conversion = ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original));
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
                                        self::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10::MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Return(source_ml_source1_temporary_result10) => {
                                            source_ml_source1_temporary_result10.clone()
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
pub fn read_java_script_literal_property_index(
    key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> f64 {
    if ((({
        let ml_reference_array126 = key.clone();
        let ml_reference_projection126 = ml_reference_array126.borrow().len() as f64;
        ml_reference_projection126
    }) == (0f64))
        || (({
            let ml_reference_array127 = key.clone();
            let ml_reference_projection127 = ml_reference_array127.borrow().len() as f64;
            ml_reference_projection127
        }) > (10f64)))
    {
        (-1f64)
    } else {
        if ((({
            let ml_reference_array128 = key.clone();
            let ml_reference_projection128 = ml_reference_array128.borrow().len() as f64;
            ml_reference_projection128
        }) > (1f64))
            && {
                let source_ml_source2_comparison_operand93 = {
                    let source_ml_source2_array_read_index92 = 0f64;
                    {
                        let ml_reference_array129 = key.clone();
                        let ml_reference_position129 = source_ml_source2_array_read_index92;
                        let ml_reference_projection129 = self::ml_array::optional_at(
                            &*ml_reference_array129.borrow(),
                            self::ml_array::number_index(ml_reference_position129),
                        );
                        ml_reference_projection129
                    }
                };
                {
                    let source_ml_source2_comparison_operand94 = 48f64;
                    match (
                        source_ml_source2_comparison_operand93.clone(),
                        source_ml_source2_comparison_operand94,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            })
        {
            (-1f64)
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s95 = self::source_ml_source1_temporary_read_java_script_literal_property_index_loop11(key.clone(), value, at);
                        match source_ml_source2_s95.clone() {
                            self::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(value_2) => {
                                if (value_2 <= (4294967294f64)) {
                                    value_2
                                } else {
                                    (-1f64)
                                }
                            }
                            self::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Return(source_ml_source1_temporary_result11) => {
                                source_ml_source1_temporary_result11
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
pub fn find_java_script_heap_directory_object(
    objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>,
    identity: self::ml::Big,
) -> f64 {
    {
        let low = 0f64;
        {
            let high = {
                let ml_reference_array130 = objects.clone();
                let ml_reference_projection130 = ml_reference_array130.borrow().len() as f64;
                ml_reference_projection130
            };
            {
                let source_ml_source2_s96 =
                    self::source_ml_source1_temporary_find_java_script_heap_directory_object_loop12(
                        objects.clone(),
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source2_s96.clone() {
                    self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(source_ml_source1_temporary_result12) => {
                        source_ml_source1_temporary_result12
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_heap_directory_symbol(
    symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    identity: self::ml::Big,
) -> bool {
    {
        let low = 0f64;
        {
            let high = {
                let ml_reference_array131 = symbols.clone();
                let ml_reference_projection131 = ml_reference_array131.borrow().len() as f64;
                ml_reference_projection131
            };
            {
                let source_ml_source2_s97 =
                    self::source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop13(
                        symbols.clone(),
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source2_s97.clone() {
                    self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Done => {
                        false
                    }
                    self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop13Return(source_ml_source1_temporary_result13) => {
                        source_ml_source1_temporary_result13
                    }
                }
            }
        }
    }
}
