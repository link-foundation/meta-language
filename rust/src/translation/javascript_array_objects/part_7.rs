// Generated from complete js/src/translation/javascript-array-objects.js. Do not edit by hand.
use super::JavaScriptArrayObject;
use super::JavaScriptArrayPropertyPlan;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop26Result;
use super::find_java_script_array_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::plan_java_script_array_length_definition;
use super::plan_java_script_array_ordinary_definition;
use super::read_java_script_array_index;
use super::read_java_script_array_length;
use super::read_java_script_array_length_key;
use super::source_ml_temporary_plan_java_script_array_property_definition_loop26;
use super::test_java_script_array_length_key;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_java_script_array_property_definition(
    object: self::JavaScriptArrayObject,
    key: self::JavaScriptSemanticValue,
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
) -> self::JavaScriptArrayPropertyPlan {
    if self::test_java_script_array_length_key(key.clone()) {
        if (({ proposed.borrow().clone() }).9) {
            {
                let value = ({ proposed.borrow().clone() }).8;
                match value {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f480) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        self::plan_java_script_array_length_definition(
                            object.clone(),
                            proposed.clone(),
                            self::convert_java_script_array_unsigned_length(value_value),
                            value_value,
                        )
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f481) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f482) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                    self::JavaScriptSemanticValue::SourceText(
                        source_ml_source1_f483,
                        source_ml_source1_f484,
                    ) => self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                        Box::new(object.clone()),
                        false,
                        self::ml_string::Text::from_units(vec![
                            97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101,
                            113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                            97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32,
                            99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116,
                            105, 110, 117, 97, 116, 105, 111, 110, 115,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                        ]),
                        std::rc::Rc::new(std::cell::RefCell::new(
                            Vec::<self::ml_string::Text>::new(),
                        )),
                    ),
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f485) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f486,
                        source_ml_source1_f487,
                    ) => self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                        Box::new(object.clone()),
                        false,
                        self::ml_string::Text::from_units(vec![
                            97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101,
                            113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                            97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32,
                            99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116,
                            105, 110, 117, 97, 116, 105, 111, 110, 115,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                        ]),
                        std::rc::Rc::new(std::cell::RefCell::new(
                            Vec::<self::ml_string::Text>::new(),
                        )),
                    ),
                }
            }
        } else {
            self::plan_java_script_array_ordinary_definition(
                object.clone(),
                key.clone(),
                proposed.clone(),
            )
        }
    } else {
        {
            let index = self::read_java_script_array_index(key.clone());
            {
                let length = self::read_java_script_array_length(object.clone());
                {
                    let length_property = {
                        let source_ml_source1_array_read_value490 = match object.clone() {
                            self::JavaScriptArrayObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value488,
                            ) => source_ml_source1_data_property_value488.clone(),
                        };
                        {
                            let source_ml_source1_array_read_index491 =
                                self::find_java_script_array_property(
                                    match object.clone() {
                                        self::JavaScriptArrayObject::ArrayObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source1_data_property_value489,
                                        ) => source_ml_source1_data_property_value489.clone(),
                                    },
                                    self::read_java_script_array_length_key(),
                                );
                            {
                                let ml_reference_array158 = source_ml_source1_array_read_value490;
                                let ml_reference_position158 =
                                    source_ml_source1_array_read_index491;
                                self::ml_array::optional_at(
                                    &*ml_reference_array158.borrow(),
                                    self::ml_array::number_index(ml_reference_position158),
                                )
                            }
                        }
                    };
                    if (index >= length)
                        && (match (length_property.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } || !(({
                            ({
                                let source_ml_source1_nullish_field_receiver492 =
                                    (length_property.clone(),);
                                if match (source_ml_source1_nullish_field_receiver492.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source1_nullish_field_present493 =
                                            ((source_ml_source1_nullish_field_receiver492.clone())
                                                .0)
                                                .clone()
                                                .expect("a proven present value");
                                        match source_ml_source1_nullish_field_present493 {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                                _,
                                                source_ml_source1_data_property_value494,
                                            ) => source_ml_source1_data_property_value494.clone(),
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (self::ml_string::Text::from_units(vec![
                                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32,
                                            112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32,
                                            111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101,
                                            100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39,
                                            100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39,
                                            41,
                                        ]))
                                        .0,
                                        message_present: true,
                                    })
                                    .raise()
                                }
                            })
                            .borrow()
                            .clone()
                        })
                        .10))
                    {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                            Box::new(object.clone()),
                            false,
                            self::ml_string::Text::from_units(vec![]),
                            self::ml_string::Text::from_units(vec![]),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    } else {
                        {
                            let plan = self::plan_java_script_array_ordinary_definition(
                                object.clone(),
                                key.clone(),
                                proposed.clone(),
                            );
                            if (!(match plan.clone() {
                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(
                                    _,
                                    source_ml_source1_data_property_value495,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value495,
                            }) || (index < length))
                                || (index < 0f64)
                            {
                                plan.clone()
                            } else {
                                {
                                    let properties =
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::JavaScriptOrdinaryProperty,
                                        >::new(
                                        )));
                                    {
                                        let source_ml_temporary_values25 = match match plan.clone() {
                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value496, _, _, _, _) => {
                                                let source_ml_source1_data_property_value496 = *source_ml_source1_data_property_value496;
                                                source_ml_source1_data_property_value496.clone()
                                            }
                                        } {
                                            self::JavaScriptArrayObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value497) => {
                                                source_ml_source1_data_property_value497.clone()
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index25 =
                                                self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_s498 = self::source_ml_temporary_plan_java_script_array_property_definition_loop26(index, properties.clone(), source_ml_temporary_values25, source_ml_temporary_index25);
                                                match source_ml_source1_s498 {
                                                    self::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop26Result::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop26Done => {
                                                        {
                                                            let source_ml_source1_nominal_spread515 = plan.clone();
                                                            {
                                                                let source_ml_source1_nominal_field516 = match source_ml_source1_nominal_spread515.clone() {
                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value517, _, _, _, _) => {
                                                                        let source_ml_source1_data_property_value517 = *source_ml_source1_data_property_value517;
                                                                        source_ml_source1_data_property_value517.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field518 = match source_ml_source1_nominal_spread515.clone() {
                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, source_ml_source1_data_property_value519, _, _, _) => {
                                                                            source_ml_source1_data_property_value519
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field520 = match source_ml_source1_nominal_spread515.clone() {
                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, source_ml_source1_data_property_value521, _, _) => {
                                                                                source_ml_source1_data_property_value521.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_field522 = match source_ml_source1_nominal_spread515.clone() {
                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, source_ml_source1_data_property_value523, _) => {
                                                                                    source_ml_source1_data_property_value523.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field524 = match source_ml_source1_nominal_spread515.clone() {
                                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value525) => {
                                                                                        source_ml_source1_data_property_value525.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_override526 = {
                                                                                        let source_ml_source1_nominal_spread500 = match plan.clone() {
                                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value499, _, _, _, _) => {
                                                                                                let source_ml_source1_data_property_value499 = *source_ml_source1_data_property_value499;
                                                                                                source_ml_source1_data_property_value499.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field501 = match source_ml_source1_nominal_spread500.clone() {
                                                                                                self::JavaScriptArrayObject::ArrayObject(source_ml_source1_data_property_value502, _, _, _, _) => {
                                                                                                    source_ml_source1_data_property_value502.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field503 = match source_ml_source1_nominal_spread500.clone() {
                                                                                                    self::JavaScriptArrayObject::ArrayObject(_, source_ml_source1_data_property_value504, _, _, _) => {
                                                                                                        let source_ml_source1_data_property_value504 = *source_ml_source1_data_property_value504;
                                                                                                        source_ml_source1_data_property_value504.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field505 = match source_ml_source1_nominal_spread500.clone() {
                                                                                                        self::JavaScriptArrayObject::ArrayObject(_, _, source_ml_source1_data_property_value506, _, _) => {
                                                                                                            source_ml_source1_data_property_value506
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_field507 = match source_ml_source1_nominal_spread500.clone() {
                                                                                                            self::JavaScriptArrayObject::ArrayObject(_, _, _, source_ml_source1_data_property_value508, _) => {
                                                                                                                source_ml_source1_data_property_value508
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source1_nominal_field509 = match source_ml_source1_nominal_spread500.clone() {
                                                                                                                self::JavaScriptArrayObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value510) => {
                                                                                                                    source_ml_source1_data_property_value510.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_nominal_override511 = properties.clone();
                                                                                                                self::JavaScriptArrayObject::ArrayObject(source_ml_source1_nominal_field501, Box::new(source_ml_source1_nominal_field503), source_ml_source1_nominal_field505, source_ml_source1_nominal_field507, source_ml_source1_nominal_override511)
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_override527 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                            let source_ml_source1_concat_value513 = match plan.clone() {
                                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value512) => {
                                                                                                    source_ml_source1_data_property_value512.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_concat_value514 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { source_ml_source1_concat_value513.borrow().clone() }), { source_ml_source1_concat_value514.borrow().clone() })
                                                                                            }
                                                                                        }));
                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(source_ml_source1_nominal_override526), source_ml_source1_nominal_field518, source_ml_source1_nominal_field520, source_ml_source1_nominal_field522, source_ml_source1_nominal_override527)
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_array_unsigned_length(value: f64) -> f64 {
    if !(value.is_finite()) || (value == 0f64) {
        0f64
    } else {
        {
            let integer = value.trunc();
            {
                let remainder = integer % 4_294_967_296f64;
                if remainder < 0f64 {
                    remainder + 4_294_967_296f64
                } else if remainder == 0f64 {
                    0f64
                } else {
                    remainder
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
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if at
            < ({
                let ml_reference_array162 = units.clone();
                ml_reference_array162.borrow().len() as f64
            }) {
            {
                let unit = ({
                    let source_ml_source1_array_read_value528 = units.clone();
                    {
                        let source_ml_source1_array_read_index529 = at;
                        {
                            let ml_reference_array163 = source_ml_source1_array_read_value528;
                            let ml_reference_position163 = source_ml_source1_array_read_index529;
                            self::ml_array::optional_at(
                                &*ml_reference_array163.borrow(),
                                self::ml_array::number_index(ml_reference_position163),
                            )
                        }
                    }
                })
                .clone()
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
