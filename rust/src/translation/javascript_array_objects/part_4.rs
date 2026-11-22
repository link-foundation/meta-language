// Generated from complete js/src/translation/javascript-array-objects.js. Do not edit by hand.
use super::JavaScriptSemanticValue;
use super::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7;
use super::ml_array;
use super::ml_string;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_property_descriptor_kind;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn test_complete_java_script_property_descriptor(
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
) -> bool {
    if (!(({ descriptor.borrow().clone() }).1) || !(({ descriptor.borrow().clone() }).3))
        || ((self::read_java_script_property_descriptor_error(descriptor.clone()))
            != (self::ml_string::Text::from_units(vec![])))
    {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])) {
                ({ descriptor.borrow().clone() }).9 && ({ descriptor.borrow().clone() }).11
            } else {
                ((kind
                    == (self::ml_string::Text::from_units(vec![
                        97, 99, 99, 101, 115, 115, 111, 114,
                    ])))
                    && ({ descriptor.borrow().clone() }).5)
                    && ({ descriptor.borrow().clone() }).7
            }
        }
    }
}

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
        let source_ml_source1_field312 = true;
        {
            let source_ml_source1_field313 = configurable;
            {
                let source_ml_source1_field314 = true;
                {
                    let source_ml_source1_field315 = enumerable;
                    {
                        let source_ml_source1_field316 = (*kind)
                            != (self::ml_string::Text::from_units(vec![
                                97, 99, 99, 101, 115, 115, 111, 114,
                            ]));
                        {
                            let source_ml_source1_field317 =
                                self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field318 = (*kind)
                                    != (self::ml_string::Text::from_units(vec![
                                        97, 99, 99, 101, 115, 115, 111, 114,
                                    ]));
                                {
                                    let source_ml_source1_field319 = false;
                                    {
                                        let source_ml_source1_field320 = (*kind)
                                            == (self::ml_string::Text::from_units(vec![
                                                97, 99, 99, 101, 115, 115, 111, 114,
                                            ]));
                                        {
                                            let source_ml_source1_field321 =
                                                self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field322 = (*kind)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        97, 99, 99, 101, 115, 115, 111, 114,
                                                    ]));
                                                {
                                                    let source_ml_source1_field323 =
                                                        self::JavaScriptSemanticValue::Undefined;
                                                    (
                                                        source_ml_source1_field313,
                                                        source_ml_source1_field312,
                                                        source_ml_source1_field315,
                                                        source_ml_source1_field314,
                                                        source_ml_source1_field321,
                                                        source_ml_source1_field320,
                                                        source_ml_source1_field323,
                                                        source_ml_source1_field322,
                                                        source_ml_source1_field317,
                                                        source_ml_source1_field316,
                                                        source_ml_source1_field319,
                                                        source_ml_source1_field318,
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
        let source_ml_source1_field324 = true;
        {
            let source_ml_source1_field325 = {
                let ml_product_condition_6 = { proposed.borrow().clone() };
                if ml_product_condition_6 {
                    ({ proposed.borrow().clone() }).0
                } else {
                    ({ base.borrow().clone() }).0
                }
            };
            {
                let source_ml_source1_field326 = true;
                {
                    let source_ml_source1_field327 = {
                        let ml_product_condition_5 = { proposed.borrow().clone() };
                        if ml_product_condition_5 {
                            ({ proposed.borrow().clone() }).2
                        } else {
                            ({ base.borrow().clone() }).2
                        }
                    };
                    {
                        let source_ml_source1_field328 = ({ base.borrow().clone() }).9;
                        {
                            let source_ml_source1_field329 = {
                                let ml_product_condition_4 = { base.borrow().clone() };
                                if ml_product_condition_4 {
                                    ({ proposed.borrow().clone() }).8
                                } else {
                                    ({ base.borrow().clone() }).8
                                }
                            };
                            {
                                let source_ml_source1_field330 = ({ base.borrow().clone() }).11;
                                {
                                    let source_ml_source1_field331 = {
                                        let ml_product_condition_3 = { base.borrow().clone() };
                                        if ml_product_condition_3 {
                                            ({ proposed.borrow().clone() }).10
                                        } else {
                                            ({ base.borrow().clone() }).10
                                        }
                                    };
                                    {
                                        let source_ml_source1_field332 =
                                            ({ base.borrow().clone() }).5;
                                        {
                                            let source_ml_source1_field333 = {
                                                let ml_product_condition_2 =
                                                    { base.borrow().clone() };
                                                if ml_product_condition_2 {
                                                    ({ proposed.borrow().clone() }).4
                                                } else {
                                                    ({ base.borrow().clone() }).4
                                                }
                                            };
                                            {
                                                let source_ml_source1_field334 =
                                                    ({ base.borrow().clone() }).7;
                                                {
                                                    let source_ml_source1_field335 = {
                                                        let ml_product_condition_1 =
                                                            { base.borrow().clone() };
                                                        if ml_product_condition_1 {
                                                            ({ proposed.borrow().clone() }).6
                                                        } else {
                                                            ({ base.borrow().clone() }).6
                                                        }
                                                    };
                                                    (
                                                        source_ml_source1_field325,
                                                        source_ml_source1_field324,
                                                        source_ml_source1_field327,
                                                        source_ml_source1_field326,
                                                        source_ml_source1_field333,
                                                        source_ml_source1_field332,
                                                        source_ml_source1_field335,
                                                        source_ml_source1_field334,
                                                        source_ml_source1_field329,
                                                        source_ml_source1_field328,
                                                        source_ml_source1_field331,
                                                        source_ml_source1_field330,
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
                            let source_ml_source1_concat_value336 = writes.clone();
                            {
                                let source_ml_source1_concat_value337 =
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101,
                                        ]),
                                    ]));
                                self::ml_array::append(
                                    self::ml_array::append(Vec::<self::ml_string::Text>::new(), {
                                        source_ml_source1_concat_value336.borrow().clone()
                                    }),
                                    { source_ml_source1_concat_value337.borrow().clone() },
                                )
                            }
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
                                let source_ml_source1_concat_value338 = writes_3.clone();
                                {
                                    let source_ml_source1_concat_value339 =
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            self::ml_string::Text::from_units(vec![
                                                101, 110, 117, 109, 101, 114, 97, 98, 108, 101,
                                            ]),
                                        ]));
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<self::ml_string::Text>::new(),
                                            { source_ml_source1_concat_value338.borrow().clone() },
                                        ),
                                        { source_ml_source1_concat_value339.borrow().clone() },
                                    )
                                }
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
                                    let source_ml_source1_concat_value340 = writes_5.clone();
                                    {
                                        let source_ml_source1_concat_value341 =
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                self::ml_string::Text::from_units(vec![
                                                    118, 97, 108, 117, 101,
                                                ]),
                                            ]));
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
                                                {
                                                    source_ml_source1_concat_value340
                                                        .borrow()
                                                        .clone()
                                                },
                                            ),
                                            { source_ml_source1_concat_value341.borrow().clone() },
                                        )
                                    }
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
                                        let source_ml_source1_concat_value342 = writes_7.clone();
                                        {
                                            let source_ml_source1_concat_value343 =
                                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        119, 114, 105, 116, 97, 98, 108, 101,
                                                    ]),
                                                ]));
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    {
                                                        source_ml_source1_concat_value342
                                                            .borrow()
                                                            .clone()
                                                    },
                                                ),
                                                {
                                                    source_ml_source1_concat_value343
                                                        .borrow()
                                                        .clone()
                                                },
                                            )
                                        }
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
                                            let source_ml_source1_concat_value344 =
                                                writes_9.clone();
                                            {
                                                let source_ml_source1_concat_value345 =
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        vec![self::ml_string::Text::from_units(
                                                            vec![103, 101, 116],
                                                        )],
                                                    ));
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<self::ml_string::Text>::new(),
                                                        {
                                                            source_ml_source1_concat_value344
                                                                .borrow()
                                                                .clone()
                                                        },
                                                    ),
                                                    {
                                                        source_ml_source1_concat_value345
                                                            .borrow()
                                                            .clone()
                                                    },
                                                )
                                            }
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
                                                let source_ml_source1_concat_value346 =
                                                    writes_11.clone();
                                                {
                                                    let source_ml_source1_concat_value347 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            vec![
                                                                self::ml_string::Text::from_units(
                                                                    vec![115, 101, 116],
                                                                ),
                                                            ],
                                                        ));
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<self::ml_string::Text>::new(),
                                                            {
                                                                source_ml_source1_concat_value346
                                                                    .borrow()
                                                                    .clone()
                                                            },
                                                        ),
                                                        {
                                                            source_ml_source1_concat_value347
                                                                .borrow()
                                                                .clone()
                                                        },
                                                    )
                                                }
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
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source1_field348 = disposition.clone();
        {
            let source_ml_source1_field349 = error.clone();
            {
                let source_ml_source1_field350 = descriptor.clone();
                {
                    let source_ml_source1_field351 = writes.clone();
                    (
                        source_ml_source1_field350,
                        source_ml_source1_field348,
                        source_ml_source1_field349,
                        source_ml_source1_field351,
                    )
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
                                    let source_ml_source1_s352 = if (({ current.borrow().clone() })
                                        .0)
                                    {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                    } else {
                                        let ml_product_condition_6 = { proposed.borrow().clone() };
                                        if ml_product_condition_6 {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                        } else {
                                            let ml_product_condition_5 =
                                                { proposed.borrow().clone() };
                                            if ml_product_condition_5 {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                            } else if (kind
                                                != (self::ml_string::Text::from_units(vec![
                                                    103, 101, 110, 101, 114, 105, 99,
                                                ])))
                                                && (kind != original)
                                            {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                            } else if original
                                                == (self::ml_string::Text::from_units(vec![
                                                    97, 99, 99, 101, 115, 115, 111, 114,
                                                ]))
                                            {
                                                {
                                                    let ml_product_condition_4 =
                                                        { proposed.borrow().clone() };
                                                    if ml_product_condition_4 {
                                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                    } else {
                                                        let ml_product_condition_3 =
                                                            { proposed.borrow().clone() };
                                                        if ml_product_condition_3 {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                        } else {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                        }
                                                    }
                                                }
                                            } else if (({ current.borrow().clone() }).10) {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                            } else {
                                                let ml_product_condition_2 =
                                                    { proposed.borrow().clone() };
                                                if ml_product_condition_2 {
                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                } else {
                                                    let ml_product_condition_1 =
                                                        { proposed.borrow().clone() };
                                                    if ml_product_condition_1 {
                                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value(({ proposed.borrow().clone() }).8, ({ current.borrow().clone() }).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                    } else {
                                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    match source_ml_source1_s352 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next => {
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
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(source_ml_temporary_result7) => {
                                            source_ml_temporary_result7.clone()
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
