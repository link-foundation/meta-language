// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result;
use super::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10;
use super::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result;
use super::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_property_descriptor_kind;
use super::source_ml_temporary_find_java_script_heap_directory_object_loop12;
use super::source_ml_temporary_read_java_script_literal_property_index_loop11;
use super::source_ml_temporary_test_java_script_heap_directory_symbol_loop13;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_property_descriptor(
    kind: &self::ml_string::Text,
    configurable: bool,
    enumerable: bool,
) -> std::rc::Rc<
    std::cell::RefCell<(
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
    )>,
> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source1_field496 = (*kind)
            != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]));
        {
            let source_ml_source1_field497 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source1_field498 = (*kind)
                    != (self::ml_string::Text::from_units(vec![
                        97, 99, 99, 101, 115, 115, 111, 114,
                    ]));
                {
                    let source_ml_source1_field500 = (*kind)
                        == (self::ml_string::Text::from_units(vec![
                            97, 99, 99, 101, 115, 115, 111, 114,
                        ]));
                    {
                        let source_ml_source1_field501 = self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source1_field502 = (*kind)
                                == (self::ml_string::Text::from_units(vec![
                                    97, 99, 99, 101, 115, 115, 111, 114,
                                ]));
                            {
                                let source_ml_source1_field503 =
                                    self::JavaScriptSemanticValue::Undefined;
                                (
                                    configurable,
                                    true,
                                    enumerable,
                                    true,
                                    source_ml_source1_field501,
                                    source_ml_source1_field500,
                                    source_ml_source1_field503,
                                    source_ml_source1_field502,
                                    source_ml_source1_field497,
                                    source_ml_source1_field496,
                                    false,
                                    source_ml_source1_field498,
                                )
                            }
                        }
                    }
                }
            }
        }
    }))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn apply_java_script_property_descriptor_fields(
    base: std::rc::Rc<
        std::cell::RefCell<(
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
        )>,
    >,
    proposed: std::rc::Rc<
        std::cell::RefCell<(
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
        )>,
    >,
) -> std::rc::Rc<
    std::cell::RefCell<(
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
    )>,
> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source1_field505 = {
            let ml_product_condition_6 = { proposed.borrow().clone() };
            if ml_product_condition_6 {
                ({ proposed.borrow().clone() }).0
            } else {
                ({ base.borrow().clone() }).0
            }
        };
        {
            let source_ml_source1_field507 = {
                let ml_product_condition_5 = { proposed.borrow().clone() };
                if ml_product_condition_5 {
                    ({ proposed.borrow().clone() }).2
                } else {
                    ({ base.borrow().clone() }).2
                }
            };
            {
                let source_ml_source1_field508 = ({ base.borrow().clone() }).9;
                {
                    let source_ml_source1_field509 = {
                        let ml_product_condition_4 = { base.borrow().clone() };
                        if ml_product_condition_4 {
                            ({ proposed.borrow().clone() }).8
                        } else {
                            ({ base.borrow().clone() }).8
                        }
                    };
                    {
                        let source_ml_source1_field510 = ({ base.borrow().clone() }).11;
                        {
                            let source_ml_source1_field511 = {
                                let ml_product_condition_3 = { base.borrow().clone() };
                                if ml_product_condition_3 {
                                    ({ proposed.borrow().clone() }).10
                                } else {
                                    ({ base.borrow().clone() }).10
                                }
                            };
                            {
                                let source_ml_source1_field512 = ({ base.borrow().clone() }).5;
                                {
                                    let source_ml_source1_field513 = {
                                        let ml_product_condition_2 = { base.borrow().clone() };
                                        if ml_product_condition_2 {
                                            ({ proposed.borrow().clone() }).4
                                        } else {
                                            ({ base.borrow().clone() }).4
                                        }
                                    };
                                    {
                                        let source_ml_source1_field514 =
                                            ({ base.borrow().clone() }).7;
                                        {
                                            let source_ml_source1_field515 = {
                                                let ml_product_condition_1 =
                                                    { base.borrow().clone() };
                                                if ml_product_condition_1 {
                                                    ({ proposed.borrow().clone() }).6
                                                } else {
                                                    ({ base.borrow().clone() }).6
                                                }
                                            };
                                            (
                                                source_ml_source1_field505,
                                                true,
                                                source_ml_source1_field507,
                                                true,
                                                source_ml_source1_field513,
                                                source_ml_source1_field512,
                                                source_ml_source1_field515,
                                                source_ml_source1_field514,
                                                source_ml_source1_field509,
                                                source_ml_source1_field508,
                                                source_ml_source1_field511,
                                                source_ml_source1_field510,
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
    }))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_property_descriptor_writes(
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
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
        )>,
    >,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    {
        let writes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
        {
            let writes_3 = {
                let ml_product_condition_6 = { descriptor.borrow().clone() };
                if ml_product_condition_6 {
                    {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source1_concat_value517 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    self::ml_string::Text::from_units(vec![
                                        99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101,
                                    ]),
                                ]));
                            self::ml_array::append(
                                self::ml_array::append(Vec::<self::ml_string::Text>::new(), {
                                    writes.borrow().clone()
                                }),
                                { source_ml_source1_concat_value517.borrow().clone() },
                            )
                        }))
                    }
                } else {
                    writes.clone()
                }
            };
            {
                let writes_5 = {
                    let ml_product_condition_5 = { descriptor.borrow().clone() };
                    if ml_product_condition_5 {
                        {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source1_concat_value519 =
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            101, 110, 117, 109, 101, 114, 97, 98, 108, 101,
                                        ]),
                                    ]));
                                self::ml_array::append(
                                    self::ml_array::append(Vec::<self::ml_string::Text>::new(), {
                                        writes_3.borrow().clone()
                                    }),
                                    { source_ml_source1_concat_value519.borrow().clone() },
                                )
                            }))
                        }
                    } else {
                        writes_3.clone()
                    }
                };
                {
                    let writes_7 = {
                        let ml_product_condition_4 = { descriptor.borrow().clone() };
                        if ml_product_condition_4 {
                            {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source1_concat_value521 =
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            self::ml_string::Text::from_units(vec![
                                                118, 97, 108, 117, 101,
                                            ]),
                                        ]));
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            { writes_5.borrow().clone() },
                                        ),
                                        { source_ml_source1_concat_value521.borrow().clone() },
                                    )
                                }))
                            }
                        } else {
                            writes_5.clone()
                        }
                    };
                    {
                        let writes_9 = {
                            let ml_product_condition_3 = { descriptor.borrow().clone() };
                            if ml_product_condition_3 {
                                {
                                    std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_concat_value523 =
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                self::ml_string::Text::from_units(vec![
                                                    119, 114, 105, 116, 97, 98, 108, 101,
                                                ]),
                                            ]));
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
                                                { writes_7.borrow().clone() },
                                            ),
                                            { source_ml_source1_concat_value523.borrow().clone() },
                                        )
                                    }))
                                }
                            } else {
                                writes_7.clone()
                            }
                        };
                        {
                            let writes_11 = {
                                let ml_product_condition_2 = { descriptor.borrow().clone() };
                                if ml_product_condition_2 {
                                    {
                                        std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source1_concat_value525 =
                                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        103, 101, 116,
                                                    ]),
                                                ]));
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    { writes_9.borrow().clone() },
                                                ),
                                                {
                                                    source_ml_source1_concat_value525
                                                        .borrow()
                                                        .clone()
                                                },
                                            )
                                        }))
                                    }
                                } else {
                                    writes_9.clone()
                                }
                            };
                            {
                                {
                                    let ml_product_condition_1 = { descriptor.borrow().clone() };
                                    if ml_product_condition_1 {
                                        {
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source1_concat_value527 =
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        vec![self::ml_string::Text::from_units(
                                                            vec![115, 101, 116],
                                                        )],
                                                    ));
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<self::ml_string::Text>::new(),
                                                        { writes_11.borrow().clone() },
                                                    ),
                                                    {
                                                        source_ml_source1_concat_value527
                                                            .borrow()
                                                            .clone()
                                                    },
                                                )
                                            }))
                                        }
                                    } else {
                                        writes_11.clone()
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
pub fn create_java_script_property_transition(
    disposition: &self::ml_string::Text,
    error: &self::ml_string::Text,
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
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
        )>,
    >,
    writes: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
) -> std::rc::Rc<
    std::cell::RefCell<(
        std::rc::Rc<
            std::cell::RefCell<(
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
            )>,
        >,
        self::ml_string::Text,
        self::ml_string::Text,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
    )>,
> {
    std::rc::Rc::new(std::cell::RefCell::new(
        descriptor.clone(),
        disposition.clone(),
        error.clone(),
        writes.clone(),
    ))
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
    current: std::rc::Rc<
        std::cell::RefCell<(
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
        )>,
    >,
    proposed: std::rc::Rc<
        std::cell::RefCell<(
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
        )>,
    >,
) -> std::rc::Rc<
    std::cell::RefCell<(
        std::rc::Rc<
            std::cell::RefCell<(
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
            )>,
        >,
        self::ml_string::Text,
        self::ml_string::Text,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
    )>,
> {
    {
        let error = self::read_java_script_property_descriptor_error(proposed.clone());
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                self::create_java_script_property_transition(
                    &(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])),
                    &(error),
                    current.clone(),
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())),
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
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())),
                )
            } else {
                {
                    let kind = self::read_java_script_property_descriptor_kind(proposed.clone());
                    if exists {
                        if ((((!(({ proposed.borrow().clone() }).1)
                            && !(({ proposed.borrow().clone() }).3))
                            && !(({ proposed.borrow().clone() }).9))
                            && !(({ proposed.borrow().clone() }).11))
                            && !(({ proposed.borrow().clone() }).5))
                            && !(({ proposed.borrow().clone() }).7)
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
                                    let source_ml_source1_s532 = if (({ current.borrow().clone() })
                                        .0)
                                    {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Next
                                    } else {
                                        let ml_product_condition_6 = { proposed.borrow().clone() };
                                        if ml_product_condition_6 {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                        } else {
                                            let ml_product_condition_5 =
                                                { proposed.borrow().clone() };
                                            if ml_product_condition_5 {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                            } else if (kind
                                                != (self::ml_string::Text::from_units(vec![
                                                    103, 101, 110, 101, 114, 105, 99,
                                                ])))
                                                && (kind != original)
                                            {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                            } else if original
                                                == (self::ml_string::Text::from_units(vec![
                                                    97, 99, 99, 101, 115, 115, 111, 114,
                                                ]))
                                            {
                                                {
                                                    let ml_product_condition_4 =
                                                        { proposed.borrow().clone() };
                                                    if ml_product_condition_4 {
                                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                    } else {
                                                        let ml_product_condition_3 =
                                                            { proposed.borrow().clone() };
                                                        if ml_product_condition_3 {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                        } else {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Next
                                                        }
                                                    }
                                                }
                                            } else if (({ current.borrow().clone() }).10) {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Next
                                            } else {
                                                let ml_product_condition_2 =
                                                    { proposed.borrow().clone() };
                                                if ml_product_condition_2 {
                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                } else {
                                                    let ml_product_condition_1 =
                                                        { proposed.borrow().clone() };
                                                    if ml_product_condition_1 {
                                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value(({ proposed.borrow().clone() }).8, ({ current.borrow().clone() }).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                    } else {
                                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Next
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    match source_ml_source1_s532 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Next => {
                                            {
                                                let base_2 = if (kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original) {
                                                    self::create_java_script_property_descriptor(&(kind), ({ current.borrow().clone() }).0, ({ current.borrow().clone() }).2)
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
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(source_ml_temporary_result10) => {
                                            source_ml_temporary_result10.clone()
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
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
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
    if (({
        let ml_reference_array94 = key.clone();
        ml_reference_array94.borrow().len() as f64
    }) == 0f64)
        || (({
            let ml_reference_array95 = key.clone();
            ml_reference_array95.borrow().len() as f64
        }) > 10f64)
    {
        -1f64
    } else {
        if (({
            let ml_reference_array96 = key.clone();
            ml_reference_array96.borrow().len() as f64
        }) > 1f64)
            && {
                let source_ml_source1_comparison_operand535 = {
                    let source_ml_source1_array_read_index534 = 0f64;
                    {
                        let ml_reference_array97 = key.clone();
                        let ml_reference_position97 = source_ml_source1_array_read_index534;
                        self::ml_array::optional_at(
                            &*ml_reference_array97.borrow(),
                            self::ml_array::number_index(ml_reference_position97),
                        )
                    }
                };
                {
                    let source_ml_source1_comparison_operand536 = 48f64;
                    match (
                        source_ml_source1_comparison_operand535,
                        source_ml_source1_comparison_operand536,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            }
        {
            -1f64
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s537 = self::source_ml_temporary_read_java_script_literal_property_index_loop11(key.clone(), value, at);
                        match source_ml_source1_s537 {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(value_2) => {
                                if value_2 <= 4_294_967_294f64 {
                                    value_2
                                } else {
                                    -1f64
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Return(source_ml_temporary_result11) => {
                                source_ml_temporary_result11
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
                let ml_reference_array98 = objects.clone();
                ml_reference_array98.borrow().len() as f64
            };
            {
                let source_ml_source1_s538 =
                    self::source_ml_temporary_find_java_script_heap_directory_object_loop12(
                        objects.clone(),
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s538 {
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Done => {
                        -1f64
                    }
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(source_ml_temporary_result12) => {
                        source_ml_temporary_result12
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
                let ml_reference_array99 = symbols.clone();
                ml_reference_array99.borrow().len() as f64
            };
            {
                let source_ml_source1_s539 =
                    self::source_ml_temporary_test_java_script_heap_directory_symbol_loop13(
                        symbols.clone(),
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s539 {
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Done => {
                        false
                    }
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Return(source_ml_temporary_result13) => {
                        source_ml_temporary_result13
                    }
                }
            }
        }
    }
}
