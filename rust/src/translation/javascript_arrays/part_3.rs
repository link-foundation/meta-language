// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result;
use super::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_construct_java_script_semantic_text_units_loop7;
use super::source_ml_temporary_find_java_script_property_descriptor_entry_loop9;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_semantic_text_units(
    value: self::JavaScriptSemanticValue,
    limit: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let length = self::read_java_script_semantic_text_length(value.clone());
        if (length < 0f64) || (length > limit) {
            (self::ml_source_exception::SourceException {
                class_name: String::from("RangeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 99, 111, 110,
                    118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115,
                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97,
                    116, 105, 111, 110, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s410 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units.clone(), at);
                            match source_ml_source1_s410 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Null => {
                    let units_2 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_2 = 0f64;
                        {
                            let source_ml_source1_s411 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_2.clone(), at_2);
                            match source_ml_source1_s411 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_2.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Boolean(_) => {
                    let units_3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_3 = 0f64;
                        {
                            let source_ml_source1_s412 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_3.clone(), at_3);
                            match source_ml_source1_s412 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_3.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Number(_) => {
                    let units_4 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_4 = 0f64;
                        {
                            let source_ml_source1_s413 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_4.clone(), at_4);
                            match source_ml_source1_s413 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_4.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(_) => {
                    let units_5 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_5 = 0f64;
                        {
                            let source_ml_source1_s414 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_5.clone(), at_5);
                            match source_ml_source1_s414 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_5.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(units_6) => units_6.clone(),
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    let units_7 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_6 = 0f64;
                        {
                            let source_ml_source1_s415 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_7.clone(), at_6);
                            match source_ml_source1_s415 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_7.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    let units_8 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_7 = 0f64;
                        {
                            let source_ml_source1_s416 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_8.clone(), at_7);
                            match source_ml_source1_s416 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_8.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    let units_9 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_8 = 0f64;
                        {
                            let source_ml_source1_s417 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_9.clone(), at_8);
                            match source_ml_source1_s417 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_9.clone()
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
pub fn test_java_script_semantic_accessor(value: self::JavaScriptSemanticValue) -> bool {
    {
        let ml_library_inequality_1 = (self::read_java_script_semantic_value_error(value.clone()))
            != (self::ml_string::Text::from_units(vec![]));
        if ml_library_inequality_1 {
            false
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => true,
                self::JavaScriptSemanticValue::Object(_, source_ml_source1_f426) => {
                    source_ml_source1_f426
                }
                _ => false,
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_property_descriptor_entry(
    properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source1_s491 =
                self::source_ml_temporary_find_java_script_property_descriptor_entry_loop9(
                    properties.clone(),
                    key.clone(),
                    at,
                );
            match source_ml_source1_s491 {
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Done => {
                    -1f64
                }
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Return(source_ml_temporary_result9) => {
                    source_ml_temporary_result9
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
pub fn read_java_script_property_descriptor_kind(
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
) -> self::ml_string::Text {
    {
        let data = ({ descriptor.borrow().clone() }).9 || ({ descriptor.borrow().clone() }).11;
        {
            let accessor =
                ({ descriptor.borrow().clone() }).5 || ({ descriptor.borrow().clone() }).7;
            if data && accessor {
                self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])
            } else if data {
                self::ml_string::Text::from_units(vec![100, 97, 116, 97])
            } else if accessor {
                self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])
            } else {
                self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99])
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
pub fn read_java_script_property_descriptor_error(
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
) -> self::ml_string::Text {
    if (self::read_java_script_property_descriptor_kind(descriptor.clone()))
        == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))
    {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        let ml_product_condition_3 = { descriptor.borrow().clone() };
        if ml_product_condition_3 {
            self::ml_string::Text::from_units(vec![
                73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108,
                117, 101,
            ])
        } else {
            let ml_product_condition_2 = { descriptor.borrow().clone() };
            if ml_product_condition_2 {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                let ml_product_condition_1 = { descriptor.borrow().clone() };
                if ml_product_condition_1 {
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![])
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
