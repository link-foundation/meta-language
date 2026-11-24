// Generated from complete js/src/translation/formal-datatype-module-rules.js. Do not edit by hand.
use super::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Result;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::source_ml_temporary_plan_formal_datatype_module_prefixes_loop5;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_formal_datatype_module_prefixes_loop2(
    mut paths: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<String>>>>>>,
    mut groups: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut common: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Result {
    loop {
        return if index
            < ({
                let ml_reference_array16 = paths.clone();
                ml_reference_array16.borrow().len() as f64
            }) {
            {
                let group = {
                    let source_ml_source1_array_read_value32 = groups.clone();
                    {
                        let source_ml_source1_array_read_index33 = index;
                        {
                            let ml_reference_array17 = source_ml_source1_array_read_value32;
                            let ml_reference_position17 = source_ml_source1_array_read_index33;
                            self::ml_array::optional_at(
                                &*ml_reference_array17.borrow(),
                                self::ml_array::number_index(ml_reference_position17),
                            )
                        }
                    }
                };
                {
                    let path = {
                        let source_ml_source1_array_read_value34 = paths.clone();
                        {
                            let source_ml_source1_array_read_index35 = index;
                            {
                                let ml_reference_array18 = source_ml_source1_array_read_value34;
                                let ml_reference_position18 = source_ml_source1_array_read_index35;
                                self::ml_array::optional_at(
                                    &*ml_reference_array18.borrow(),
                                    self::ml_array::number_index(ml_reference_position18),
                                )
                            }
                        }
                    };
                    if {
                        let source_ml_source1_comparison_operand36 = group.clone();
                        {
                            let source_ml_source1_comparison_operand37 = None::<()>;
                            match (
                                source_ml_source1_comparison_operand36,
                                source_ml_source1_comparison_operand37,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }
                        }
                    } {
                        self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    } else if match (path.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    } else if (group.clone().expect("a proven present value")) > index {
                        self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    } else {
                        {
                            let representative = {
                                let source_ml_source1_array_read_value38 = paths.clone();
                                {
                                    let source_ml_source1_array_read_index39 =
                                        group.clone().expect("a proven present value");
                                    {
                                        let ml_reference_array19 =
                                            source_ml_source1_array_read_value38;
                                        let ml_reference_position19 =
                                            source_ml_source1_array_read_index39;
                                        self::ml_array::optional_at(
                                            &*ml_reference_array19.borrow(),
                                            self::ml_array::number_index(ml_reference_position19),
                                        )
                                    }
                                }
                            };
                            if match (representative.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                            } else {
                                {
                                    let depth = 0f64;
                                    {
                                        let depth_2 = self::source_ml_temporary_plan_formal_datatype_module_prefixes_loop5(Some(path.clone().expect("a proven present value")), Some(representative.clone().expect("a proven present value")), depth);
                                        {
                                            let source_ml_temporary_plan_formal_datatype_module_prefixes_ignored6 = {
                                                let ml_array_owner21 = common.clone();
                                                let ml_array_argument21_0: f64 =
                                                    group.clone().expect("a proven present value");
                                                let ml_array_argument21_1: f64 = 1f64;
                                                let ml_array_argument21_2: f64 = self::ml_math::min(({
                                                        let source_ml_source1_array_read_value40 = common.clone();
                                                        {
                                                            let source_ml_source1_array_read_index41 = group.clone().expect("a proven present value");
                                                            { let ml_reference_array21 = source_ml_source1_array_read_value40; let ml_reference_position21 = source_ml_source1_array_read_index41; self::ml_array::optional_at(&*ml_reference_array21.borrow(), self::ml_array::number_index(ml_reference_position21)) }
                                                        }
                                                    }).clone().unwrap_or_else(|| 0f64), depth_2);
                                                let ml_array_length =
                                                    ml_array_owner21.borrow().len();
                                                let ml_array_start = {
                                                    let ml_relative_value: f64 =
                                                        ml_array_argument21_0;
                                                    let ml_relative = if ml_relative_value.is_nan()
                                                    {
                                                        0f64
                                                    } else {
                                                        ml_relative_value.trunc()
                                                    };
                                                    let ml_bound = ml_array_length as f64;
                                                    (if ml_relative < 0f64 {
                                                        (ml_bound + ml_relative)
                                                            .clamp(0f64, ml_bound)
                                                    } else {
                                                        ml_relative.min(ml_bound)
                                                    })
                                                        as usize
                                                };
                                                let ml_array_count = {
                                                    let ml_delete_value: f64 =
                                                        ml_array_argument21_1;
                                                    ml_delete_value.trunc().max(0f64).min(
                                                        (ml_array_length - ml_array_start) as f64,
                                                    )
                                                        as usize
                                                };
                                                let ml_array_inserted = vec![ml_array_argument21_2];
                                                let _ = (ml_array_length - ml_array_count)
                                                    .checked_add(ml_array_inserted.len())
                                                    .filter(|length| *length <= 4_294_967_295usize)
                                                    .expect("Invalid array length");
                                                let ml_removed: Vec<_> = ml_array_owner21
                                                    .borrow_mut()
                                                    .splice(
                                                        ml_array_start
                                                            ..ml_array_start + ml_array_count,
                                                        ml_array_inserted,
                                                    )
                                                    .collect();
                                                std::rc::Rc::new(std::cell::RefCell::new(
                                                    ml_removed,
                                                ))
                                            };
                                            {
                                                let index_2 = index + 1f64;
                                                {
                                                    index = index_2;
                                                    continue;
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
        } else {
            self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Result {
    MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Done,
    MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Return(
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Result {
    MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Done(f64),
    MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Return(
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_formal_datatype_module_prefixes_loop7(
    mut total: f64,
    mut path: Option<std::rc::Rc<std::cell::RefCell<Vec<String>>>>,
    mut part: f64,
) -> self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Result {
    loop {
        return if part
            < ({
                let source_ml_source1_nullish_field_receiver44 = (path.clone(),);
                if match (source_ml_source1_nullish_field_receiver44.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    {
                        let source_ml_source1_nullish_field_present45 =
                            ((source_ml_source1_nullish_field_receiver44.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        {
                            let ml_reference_array22 = source_ml_source1_nullish_field_present45;
                            ml_reference_array22.borrow().len() as f64
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
                    let source_ml_source1_nullish_field_receiver46 = (path.clone(), part);
                    {
                        let source_ml_source1_computed_property_key47 =
                            (source_ml_source1_nullish_field_receiver46.clone()).1;
                        if match (source_ml_source1_nullish_field_receiver46.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present48 =
                                    ((source_ml_source1_nullish_field_receiver46.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let source_ml_source1_array_read_value49 =
                                        source_ml_source1_nullish_field_present48;
                                    {
                                        let source_ml_source1_array_read_index50 =
                                            source_ml_source1_computed_property_key47;
                                        {
                                            let ml_reference_array23 =
                                                source_ml_source1_array_read_value49;
                                            let ml_reference_position23 =
                                                source_ml_source1_array_read_index50;
                                            self::ml_array::optional_at(
                                                &*ml_reference_array23.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position23,
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
                                        (source_ml_source1_nullish_field_receiver46.clone()).1
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
                    let source_ml_source1_comparison_operand51 = value.clone();
                    {
                        let source_ml_source1_comparison_operand52 = None::<()>;
                        match (
                            source_ml_source1_comparison_operand51,
                            source_ml_source1_comparison_operand52,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }
                    }
                } || (({
                    let source_ml_source1_nullish_field_receiver55 = (value.clone(),);
                    if (match (source_ml_source1_nullish_field_receiver55.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present;
                            String::from("string")
                        }
                        None => String::from("undefined"),
                    }) == "string"
                    {
                        {
                            let source_ml_source1_nullish_field_present56 =
                                ((source_ml_source1_nullish_field_receiver55.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            (source_ml_source1_nullish_field_present56
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
                        let source_ml_source1_nullish_field_receiver59 = (value.clone(),);
                        if (match (source_ml_source1_nullish_field_receiver59.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present;
                                String::from("string")
                            }
                            None => String::from("undefined"),
                        }) == "string"
                        {
                            {
                                let source_ml_source1_nullish_field_present60 =
                                    ((source_ml_source1_nullish_field_receiver59.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                (source_ml_source1_nullish_field_present60
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
                    self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                } else {
                    {
                        let total_2 = total + {
                            let source_ml_source1_nullish_field_receiver61 = (value.clone(),);
                            if (match (source_ml_source1_nullish_field_receiver61.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present;
                                    String::from("string")
                                }
                                None => String::from("undefined"),
                            }) == "string"
                            {
                                {
                                    let source_ml_source1_nullish_field_present62 =
                                        ((source_ml_source1_nullish_field_receiver61.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    (source_ml_source1_nullish_field_present62
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
                        };
                        if total_2 > 8_388_608f64 {
                            self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                        } else {
                            {
                                let part_2 = part + 1f64;
                                {
                                    (total, part) = (total_2, part_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Done(total)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_formal_datatype_module_prefixes_loop1(
    mut paths: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<String>>>>>>,
    mut groups: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut common: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut total: f64,
    mut index: f64,
) -> self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Result {
    loop {
        return if index
            < ({
                let ml_reference_array24 = paths.clone();
                ml_reference_array24.borrow().len() as f64
            }) {
            {
                let path = {
                    let source_ml_source1_array_read_value63 = paths.clone();
                    {
                        let source_ml_source1_array_read_index64 = index;
                        {
                            let ml_reference_array25 = source_ml_source1_array_read_value63;
                            let ml_reference_position25 = source_ml_source1_array_read_index64;
                            self::ml_array::optional_at(
                                &*ml_reference_array25.borrow(),
                                self::ml_array::number_index(ml_reference_position25),
                            )
                        }
                    }
                };
                {
                    let group = {
                        let source_ml_source1_array_read_value65 = groups.clone();
                        {
                            let source_ml_source1_array_read_index66 = index;
                            {
                                let ml_reference_array26 = source_ml_source1_array_read_value65;
                                let ml_reference_position26 = source_ml_source1_array_read_index66;
                                self::ml_array::optional_at(
                                    &*ml_reference_array26.borrow(),
                                    self::ml_array::number_index(ml_reference_position26),
                                )
                            }
                        }
                    };
                    if match (path.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    } else if {
                        let source_ml_source1_comparison_operand67 = group.clone();
                        {
                            let source_ml_source1_comparison_operand68 = None::<()>;
                            match (
                                source_ml_source1_comparison_operand67,
                                source_ml_source1_comparison_operand68,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }
                        }
                    } {
                        self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    } else if ((((({
                        let ml_reference_array27 = path.clone().expect("a proven present value");
                        ml_reference_array27.borrow().len() as f64
                    }) > 256f64)
                        || !(self::ml_math::is_safe_integer(
                            group.clone().expect("a proven present value"),
                        )))
                        || ((group.clone().expect("a proven present value")) < 0f64))
                        || ((group.clone().expect("a proven present value"))
                            >= ({
                                let ml_reference_array28 = paths.clone();
                                ml_reference_array28.borrow().len() as f64
                            })))
                        || (((group.clone().expect("a proven present value")) == 0f64)
                            && ((1f64 / group.clone().expect("a proven present value")) < 0f64))
                    {
                        self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    } else if {
                        let source_ml_source1_comparison_operand71 = {
                            let source_ml_source1_array_read_value69 = groups.clone();
                            {
                                let source_ml_source1_array_read_index70 =
                                    group.clone().expect("a proven present value");
                                {
                                    let ml_reference_array29 = source_ml_source1_array_read_value69;
                                    let ml_reference_position29 =
                                        source_ml_source1_array_read_index70;
                                    self::ml_array::optional_at(
                                        &*ml_reference_array29.borrow(),
                                        self::ml_array::number_index(ml_reference_position29),
                                    )
                                }
                            }
                        };
                        {
                            let source_ml_source1_comparison_operand72 =
                                group.clone().expect("a proven present value");
                            !(match (
                                source_ml_source1_comparison_operand71,
                                source_ml_source1_comparison_operand72,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    } {
                        self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    } else {
                        {
                            let part = 0f64;
                            {
                                let source_ml_source1_s73 = self::source_ml_temporary_plan_formal_datatype_module_prefixes_loop7(total, Some(path.clone().expect("a proven present value")), part);
                                match source_ml_source1_s73 {
                                                self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Done(total_2) => {
                                                    {
                                                        let source_ml_temporary_plan_formal_datatype_module_prefixes_ignored8 = { let ml_method_owner30 = common.clone(); let ml_method_argument30_0 = { let ml_reference_array31 = path.clone().expect("a proven present value"); ml_reference_array31.borrow().len() as f64 }; let ml_inserted = vec![ml_method_argument30_0]; let mut ml_method_values30 = ml_method_owner30.borrow_mut(); let ml_length = ml_method_values30.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values30.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let index_2 = index + 1f64;
                                                            {
                                                                (total, index) = (total_2, index_2);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop7Return(source_ml_temporary_result8) => {
                                                    self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Return(source_ml_temporary_result8.clone())
                                                }
                                            }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Result::MlTemporaryPlanFormalDatatypeModulePrefixesLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result {
    MlTemporaryPlanFormalDatatypeModuleClosureLoop10Done,
    MlTemporaryPlanFormalDatatypeModuleClosureLoop10Return(
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanFormalDatatypeModuleClosureLoop11Result {
    MlTemporaryPlanFormalDatatypeModuleClosureLoop11Done(f64),
    MlTemporaryPlanFormalDatatypeModuleClosureLoop11Return(
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_formal_datatype_module_closure_loop11(
    mut boundaries: std::rc::Rc<std::cell::RefCell<Vec<(String, Option<f64>)>>>,
    mut selected: f64,
    mut key: String,
    mut path: Option<std::rc::Rc<std::cell::RefCell<Vec<String>>>>,
    mut part: f64,
) -> self::MlTemporaryPlanFormalDatatypeModuleClosureLoop11Result {
    loop {
        return if part
            < ({
                let source_ml_source1_nullish_field_receiver76 = (path.clone(),);
                if match (source_ml_source1_nullish_field_receiver76.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    {
                        let source_ml_source1_nullish_field_present77 =
                            ((source_ml_source1_nullish_field_receiver76.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        {
                            let ml_reference_array32 = source_ml_source1_nullish_field_present77;
                            ml_reference_array32.borrow().len() as f64
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
                    let source_ml_source1_nullish_field_receiver78 = (path.clone(), part);
                    {
                        let source_ml_source1_computed_property_key79 =
                            (source_ml_source1_nullish_field_receiver78.clone()).1;
                        if match (source_ml_source1_nullish_field_receiver78.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present80 =
                                    ((source_ml_source1_nullish_field_receiver78.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let source_ml_source1_array_read_value81 =
                                        source_ml_source1_nullish_field_present80;
                                    {
                                        let source_ml_source1_array_read_index82 =
                                            source_ml_source1_computed_property_key79;
                                        {
                                            let ml_reference_array33 =
                                                source_ml_source1_array_read_value81;
                                            let ml_reference_position33 =
                                                source_ml_source1_array_read_index82;
                                            self::ml_array::optional_at(
                                                &*ml_reference_array33.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position33,
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
                                        (source_ml_source1_nullish_field_receiver78.clone()).1
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
                if {
                    let source_ml_source1_comparison_operand83 = value.clone();
                    {
                        let source_ml_source1_comparison_operand84 = None::<()>;
                        match (
                            source_ml_source1_comparison_operand83,
                            source_ml_source1_comparison_operand84,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }
                    }
                } {
                    self::MlTemporaryPlanFormalDatatypeModuleClosureLoop11Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop11Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                } else {
                    {
                        let key_2 = format!(
                            "{key}{}:{}",
                            {
                                let source_ml_source1_string_argument85 =
                                    (value.clone().expect("a proven present value"))
                                        .encode_utf16()
                                        .count() as f64;
                                self::ml_number::js_number(source_ml_source1_string_argument85)
                            },
                            value.clone().expect("a proven present value")
                        );
                        {
                            let protected_depth = match {
                                let ml_method_owner34 = boundaries.clone();
                                let ml_method_argument34_0 = key_2.clone();
                                let ml_method_key34 = ml_method_argument34_0;
                                let ml_method_values34 = ml_method_owner34.borrow();
                                ml_method_values34
                                    .iter()
                                    .position(|(ml_key, _)| ml_key == &ml_method_key34)
                                    .map(|index| ml_method_values34[index].1.clone())
                            } {
                                Some(ml_nested) => match ml_nested {
                                    Some(ml_present) => Some(ml_present),
                                    None => None,
                                },
                                None => None,
                            };
                            if {
                                let source_ml_source1_comparison_operand86 =
                                    protected_depth.clone();
                                {
                                    let source_ml_source1_comparison_operand87 = None::<()>;
                                    !(match (
                                        source_ml_source1_comparison_operand86,
                                        source_ml_source1_comparison_operand87,
                                    ) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    })
                                }
                            } {
                                {
                                    let selected_2 = self::ml_math::min(
                                        selected,
                                        protected_depth.clone().expect("a proven present value"),
                                    );
                                    {
                                        let part_2 = part + 1f64;
                                        {
                                            (selected, key, part) =
                                                (selected_2, key_2.clone(), part_2);
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let part_3 = part + 1f64;
                                    {
                                        (key, part) = (key_2.clone(), part_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop11Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop11Done(selected)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_formal_datatype_module_closure_loop10(
    mut paths: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<String>>>>>>,
    mut depths: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut boundaries: std::rc::Rc<std::cell::RefCell<Vec<(String, Option<f64>)>>>,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result {
    loop {
        return if index
            < ({
                let ml_reference_array35 = paths.clone();
                ml_reference_array35.borrow().len() as f64
            }) {
            {
                let selected = ({
                    let source_ml_source1_array_read_value88 = depths.clone();
                    {
                        let source_ml_source1_array_read_index89 = index;
                        {
                            let ml_reference_array36 = source_ml_source1_array_read_value88;
                            let ml_reference_position36 = source_ml_source1_array_read_index89;
                            self::ml_array::optional_at(
                                &*ml_reference_array36.borrow(),
                                self::ml_array::number_index(ml_reference_position36),
                            )
                        }
                    }
                })
                .clone()
                .unwrap_or_else(|| -1f64);
                {
                    let key = String::new();
                    {
                        let path = {
                            let source_ml_source1_array_read_value90 = paths.clone();
                            {
                                let source_ml_source1_array_read_index91 = index;
                                {
                                    let ml_reference_array37 = source_ml_source1_array_read_value90;
                                    let ml_reference_position37 =
                                        source_ml_source1_array_read_index91;
                                    self::ml_array::optional_at(
                                        &*ml_reference_array37.borrow(),
                                        self::ml_array::number_index(ml_reference_position37),
                                    )
                                }
                            }
                        };
                        if match (path.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                        } else {
                            {
                                let part = 0f64;
                                {
                                    let source_ml_source1_s92 = self::source_ml_temporary_plan_formal_datatype_module_closure_loop11(boundaries.clone(), selected, key, Some(path.clone().expect("a proven present value")), part);
                                    match source_ml_source1_s92 {
                                        self::MlTemporaryPlanFormalDatatypeModuleClosureLoop11Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop11Done(selected_2) => {
                                            {
                                                let source_ml_temporary_plan_formal_datatype_module_closure_ignored12 = { let ml_method_owner38 = result.clone(); let ml_method_argument38_0 = selected_2; let ml_inserted = vec![ml_method_argument38_0]; let mut ml_method_values38 = ml_method_owner38.borrow_mut(); let ml_length = ml_method_values38.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values38.extend(ml_inserted); ml_length as f64 };
                                                {
                                                    let index_2 = index + 1f64;
                                                    {
                                                        index = index_2;
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryPlanFormalDatatypeModuleClosureLoop11Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop11Return(source_ml_temporary_result12) => {
                                            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Return(source_ml_temporary_result12.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanFormalDatatypeModuleClosureLoop9Result {
    MlTemporaryPlanFormalDatatypeModuleClosureLoop9Done,
    MlTemporaryPlanFormalDatatypeModuleClosureLoop9Return(
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanFormalDatatypeModuleClosureLoop13Result {
    MlTemporaryPlanFormalDatatypeModuleClosureLoop13Done(f64),
    MlTemporaryPlanFormalDatatypeModuleClosureLoop13Return(
        std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    ),
}
