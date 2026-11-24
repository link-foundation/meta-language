// Generated from complete js/src/translation/formal-datatype-module-rules.js. Do not edit by hand.
use super::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Result;
use super::MlTemporaryPlanFormalDatatypeModuleClosureLoop13Result;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_formal_datatype_module_closure_loop13(
    mut boundaries: std::rc::Rc<std::cell::RefCell<Vec<(String, Option<f64>)>>>,
    mut budget: f64,
    mut path: Option<std::rc::Rc<std::cell::RefCell<Vec<String>>>>,
    mut depth: Option<f64>,
    mut key: String,
    mut part: f64,
) -> self::MlTemporaryPlanFormalDatatypeModuleClosureLoop13Result {
    loop {
        return if part
            < ({
                let source_ml_source1_nullish_field_receiver95 = (path.clone(),);
                if match (source_ml_source1_nullish_field_receiver95.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    {
                        let source_ml_source1_nullish_field_present96 =
                            ((source_ml_source1_nullish_field_receiver95.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        {
                            let ml_reference_array39 = source_ml_source1_nullish_field_present96;
                            ml_reference_array39.borrow().len() as f64
                        }
                    }
                } else {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: (String::from(
                            "Cannot read properties of undefined (reading 'length')",
                        ))
                        .encode_utf16()
                        .collect(),
                        message_present: true,
                    })
                    .raise()
                }
            }) {
            {
                let value = {
                    let source_ml_source1_nullish_field_receiver97 = (path.clone(), part);
                    {
                        let source_ml_source1_computed_property_key98 =
                            (source_ml_source1_nullish_field_receiver97.clone()).1;
                        if match (source_ml_source1_nullish_field_receiver97.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present99 =
                                    ((source_ml_source1_nullish_field_receiver97.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let source_ml_source1_array_read_value100 =
                                        source_ml_source1_nullish_field_present99;
                                    {
                                        let source_ml_source1_array_read_index101 =
                                            source_ml_source1_computed_property_key98;
                                        {
                                            let ml_reference_array40 =
                                                source_ml_source1_array_read_value100;
                                            let ml_reference_position40 =
                                                source_ml_source1_array_read_index101;
                                            self::ml_array::optional_at(
                                                &*ml_reference_array40.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position40,
                                                ),
                                            )
                                        }
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (format!(
                                    "Cannot read properties of undefined (reading '{}')",
                                    self::ml_number::js_number(
                                        (source_ml_source1_nullish_field_receiver97.clone()).1
                                    )
                                ))
                                .encode_utf16()
                                .collect(),
                                message_present: true,
                            })
                            .raise()
                        }
                    }
                };
                if ({
                    let source_ml_source1_comparison_operand102 = value.clone();
                    {
                        let source_ml_source1_comparison_operand103 = None::<()>;
                        match (
                            source_ml_source1_comparison_operand102,
                            source_ml_source1_comparison_operand103,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }
                    }
                } || (({
                    let source_ml_source1_nullish_field_receiver106 = (value.clone(),);
                    if (match (source_ml_source1_nullish_field_receiver106.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present;
                            String::from("string")
                        }
                        None => String::from("undefined"),
                    }) == "string"
                    {
                        {
                            let source_ml_source1_nullish_field_present107 =
                                ((source_ml_source1_nullish_field_receiver106.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            (source_ml_source1_nullish_field_present107
                                .encode_utf16()
                                .count() as f64)
                        }
                    } else {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units: (String::from(
                                "Cannot read properties of undefined (reading 'length')",
                            ))
                            .encode_utf16()
                            .collect(),
                            message_present: true,
                        })
                        .raise()
                    }
                }) == 0f64))
                    || (({
                        let source_ml_source1_nullish_field_receiver110 = (value.clone(),);
                        if (match (source_ml_source1_nullish_field_receiver110.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present;
                                String::from("string")
                            }
                            None => String::from("undefined"),
                        }) == "string"
                        {
                            {
                                let source_ml_source1_nullish_field_present111 =
                                    ((source_ml_source1_nullish_field_receiver110.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                (source_ml_source1_nullish_field_present111
                                    .encode_utf16()
                                    .count() as f64)
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (String::from(
                                    "Cannot read properties of undefined (reading 'length')",
                                ))
                                .encode_utf16()
                                .collect(),
                                message_present: true,
                            })
                            .raise()
                        }
                    }) > 4096f64)
                {
                    self::MlTemporaryPlanFormalDatatypeModuleClosureLoop13Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop13Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                } else {
                    {
                        let budget_2 = budget
                            + ({
                                let source_ml_source1_nullish_field_receiver112 = (value.clone(),);
                                if (match (source_ml_source1_nullish_field_receiver112.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present;
                                        String::from("string")
                                    }
                                    None => String::from("undefined"),
                                }) == "string"
                                {
                                    {
                                        let source_ml_source1_nullish_field_present113 =
                                            ((source_ml_source1_nullish_field_receiver112.clone())
                                                .0)
                                                .clone()
                                                .expect("a proven present value");
                                        (source_ml_source1_nullish_field_present113
                                            .encode_utf16()
                                            .count()
                                            as f64)
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'length')")).encode_utf16().collect(), message_present: true }).raise()
                                }
                            } + 1f64);
                        if budget_2 > 8_388_608f64 {
                            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop13Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop13Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                        } else {
                            {
                                let key_2 = format!(
                                    "{key}{}:{}",
                                    {
                                        let source_ml_source1_string_argument116 = {
                                            let source_ml_source1_nullish_field_receiver114 =
                                                (value.clone(),);
                                            if (match (source_ml_source1_nullish_field_receiver114
                                                .clone())
                                            .0
                                            {
                                                Some(ml_present) => {
                                                    let _ = ml_present;
                                                    String::from("string")
                                                }
                                                None => String::from("undefined"),
                                            }) == "string"
                                            {
                                                {
                                                    let source_ml_source1_nullish_field_present115 = ((source_ml_source1_nullish_field_receiver114.clone()).0).clone().expect("a proven present value");
                                                    (source_ml_source1_nullish_field_present115
                                                        .encode_utf16()
                                                        .count()
                                                        as f64)
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'length')")).encode_utf16().collect(), message_present: true }).raise()
                                            }
                                        };
                                        self::ml_number::js_number(
                                            source_ml_source1_string_argument116,
                                        )
                                    },
                                    match value.clone() {
                                        Some(ml_present) => ml_present.clone(),
                                        None => String::from("undefined"),
                                    }
                                );
                                if {
                                    let source_ml_source1_comparison_operand117 = part;
                                    {
                                        let source_ml_source1_comparison_operand118 = depth.clone();
                                        match (
                                            source_ml_source1_comparison_operand117,
                                            source_ml_source1_comparison_operand118,
                                        ) {
                                            (ml_left, Some(ml_right)) => ml_left == ml_right,
                                            _ => false,
                                        }
                                    }
                                } {
                                    {
                                        let previous = match {
                                            let ml_method_owner41 = boundaries.clone();
                                            let ml_method_argument41_0 = key_2.clone();
                                            let ml_method_key41 = ml_method_argument41_0;
                                            let ml_method_values41 = ml_method_owner41.borrow();
                                            ml_method_values41
                                                .iter()
                                                .position(|(ml_key, _)| ml_key == &ml_method_key41)
                                                .map(|index| ml_method_values41[index].1.clone())
                                        } {
                                            Some(ml_nested) => match ml_nested {
                                                Some(ml_present) => Some(ml_present),
                                                None => None,
                                            },
                                            None => None,
                                        };
                                        {
                                            let source_ml_temporary_plan_formal_datatype_module_closure_ignored14 = {
                                                let ml_method_owner42 = boundaries.clone();
                                                let ml_method_argument42_0 = key_2.clone();
                                                let ml_method_argument42_1 = if {
                                                    let source_ml_source1_comparison_operand119 =
                                                        previous.clone();
                                                    {
                                                        let source_ml_source1_comparison_operand120 =
                                                            None::<()>;
                                                        match (
                                                            source_ml_source1_comparison_operand119,
                                                            source_ml_source1_comparison_operand120,
                                                        ) {
                                                            (Some(_ml_left), Some(_ml_right)) => {
                                                                false
                                                            }
                                                            (None, None) => true,
                                                            _ => false,
                                                        }
                                                    }
                                                } {
                                                    depth.clone()
                                                } else {
                                                    Some({
                                                        let source_ml_source1_mathematical_argument121 =
                                                            previous
                                                                .clone()
                                                                .expect("a proven present value");
                                                        {
                                                            let source_ml_source1_mathematical_argument122 =
                                                                depth.clone();
                                                            self::ml_math::min(source_ml_source1_mathematical_argument121, {
                                                            let source_ml_source1_numeric_optional123 = source_ml_source1_mathematical_argument122;
                                                            if match (source_ml_source1_numeric_optional123.clone(), None::<f64>) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false } {
                                                                f64::NAN
                                                            } else {
                                                                source_ml_source1_numeric_optional123.clone().expect("a proven present value")
                                                            }
                                                        })
                                                        }
                                                    })
                                                };
                                                let ml_method_key42 = ml_method_argument42_0;
                                                {
                                                    let mut ml_method_values42 =
                                                        ml_method_owner42.borrow_mut();
                                                    match ml_method_values42.iter().position(
                                                        |(ml_key, _)| ml_key == &ml_method_key42,
                                                    ) {
                                                        Some(index) => {
                                                            ml_method_values42[index].1 =
                                                                ml_method_argument42_1;
                                                        }
                                                        None => {
                                                            ml_method_values42.push(
                                                                ml_method_key42,
                                                                ml_method_argument42_1,
                                                            );
                                                        }
                                                    }
                                                }
                                                ml_method_owner42
                                            };
                                            {
                                                let part_2 = part + 1f64;
                                                {
                                                    (budget, key, part) =
                                                        (budget_2, key_2.clone(), part_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let part_3 = part + 1f64;
                                        {
                                            (budget, key, part) = (budget_2, key_2.clone(), part_3);
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
            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop13Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop13Done(budget)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_formal_datatype_module_closure_loop9(
    mut paths: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<String>>>>>>,
    mut depths: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut boundaries: std::rc::Rc<std::cell::RefCell<Vec<(String, Option<f64>)>>>,
    mut budget: f64,
    mut index: f64,
) -> self::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Result {
    loop {
        return if index
            < ({
                let ml_reference_array43 = paths.clone();
                ml_reference_array43.borrow().len() as f64
            }) {
            {
                let path = {
                    let source_ml_source1_array_read_value124 = paths.clone();
                    {
                        let source_ml_source1_array_read_index125 = index;
                        {
                            let ml_reference_array44 = source_ml_source1_array_read_value124;
                            let ml_reference_position44 = source_ml_source1_array_read_index125;
                            self::ml_array::optional_at(
                                &*ml_reference_array44.borrow(),
                                self::ml_array::number_index(ml_reference_position44),
                            )
                        }
                    }
                };
                {
                    let depth = {
                        let source_ml_source1_array_read_value126 = depths.clone();
                        {
                            let source_ml_source1_array_read_index127 = index;
                            {
                                let ml_reference_array45 = source_ml_source1_array_read_value126;
                                let ml_reference_position45 = source_ml_source1_array_read_index127;
                                self::ml_array::optional_at(
                                    &*ml_reference_array45.borrow(),
                                    self::ml_array::number_index(ml_reference_position45),
                                )
                            }
                        }
                    };
                    if match (path.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    } else if {
                        let source_ml_source1_comparison_operand128 = depth.clone();
                        {
                            let source_ml_source1_comparison_operand129 = None::<()>;
                            match (
                                source_ml_source1_comparison_operand128,
                                source_ml_source1_comparison_operand129,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }
                        }
                    } {
                        self::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    } else if ((((({
                        let ml_reference_array46 = path.clone().expect("a proven present value");
                        ml_reference_array46.borrow().len() as f64
                    }) > 256f64)
                        || !(self::ml_math::is_safe_integer(
                            depth.clone().expect("a proven present value"),
                        )))
                        || ((depth.clone().expect("a proven present value")) < 0f64))
                        || ((depth.clone().expect("a proven present value"))
                            > ({
                                let ml_reference_array47 =
                                    path.clone().expect("a proven present value");
                                ml_reference_array47.borrow().len() as f64
                            })))
                        || (((depth.clone().expect("a proven present value")) == 0f64)
                            && ((1f64 / depth.clone().expect("a proven present value")) < 0f64))
                    {
                        self::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    } else {
                        {
                            let key = String::new();
                            {
                                let part = 0f64;
                                {
                                    let source_ml_source1_s130 = self::source_ml_temporary_plan_formal_datatype_module_closure_loop13(boundaries.clone(), budget, Some(path.clone().expect("a proven present value")), Some(depth.clone().expect("a proven present value")), key, part);
                                    match source_ml_source1_s130 {
                                                self::MlTemporaryPlanFormalDatatypeModuleClosureLoop13Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop13Done(budget_2) => {
                                                    {
                                                        let index_2 = index + 1f64;
                                                        {
                                                            (budget, index) = (budget_2, index_2);
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryPlanFormalDatatypeModuleClosureLoop13Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop13Return(source_ml_temporary_result13) => {
                                                    self::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Return(source_ml_temporary_result13.clone())
                                                }
                                            }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Done
        };
    }
}
