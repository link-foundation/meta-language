pub fn source_ml_temporary_plan_formal_datatype_module_closure_loop8(mut paths: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<String>>>>>>, mut depths: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut boundaries: std::rc::Rc<std::cell::RefCell<Vec<(String, Option<f64>)>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::MlTemporaryPlanFormalDatatypeModuleClosureLoop8Result {
    loop {
        return if (index < ({ let ml_reference_array35 = paths.clone(); let ml_reference_projection35 = ml_reference_array35.borrow().len() as f64; ml_reference_projection35 })) {
            {
                let selected = ({
                    let source_ml_source1_array_read_value78 = depths.clone();
                    {
                        let source_ml_source1_array_read_index79 = index;
                        { let ml_reference_array36 = source_ml_source1_array_read_value78.clone(); let ml_reference_position36 = source_ml_source1_array_read_index79; let ml_reference_projection36 = self::ml_array::optional_at(&*ml_reference_array36.borrow(), self::ml_array::number_index(ml_reference_position36)); ml_reference_projection36 }
                    }
                }).clone().unwrap_or_else(|| (-1f64));
                {
                    let key = String::from("");
                    {
                        let path = {
                            let source_ml_source1_array_read_value80 = paths.clone();
                            {
                                let source_ml_source1_array_read_index81 = index;
                                { let ml_reference_array37 = source_ml_source1_array_read_value80.clone(); let ml_reference_position37 = source_ml_source1_array_read_index81; let ml_reference_projection37 = self::ml_array::optional_at(&*ml_reference_array37.borrow(), self::ml_array::number_index(ml_reference_position37)); ml_reference_projection37 }
                            }
                        };
                        if match (path.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop8Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop8Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                        } else {
                            {
                                let part = 0f64;
                                {
                                    let source_ml_source1_s82 = self::source_ml_temporary_plan_formal_datatype_module_closure_loop9(boundaries.clone(), selected, key.clone(), Some(path.clone().expect("a proven present value")), part);
                                    match source_ml_source1_s82.clone() {
                                        self::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Done(selected_2) => {
                                            {
                                                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value38 = result.borrow().clone(); ml_reference_value38 }, vec![selected_2])));
                                                {
                                                    let index_2 = (index + 1f64);
                                                    {
                                                        (result, index) = (result_2.clone(), index_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop9Return(source_ml_temporary_result9) => {
                                            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop8Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop8Return(source_ml_temporary_result9.clone())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop8Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop8Done(result.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanFormalDatatypeModuleClosureLoop7Result {
    MlTemporaryPlanFormalDatatypeModuleClosureLoop7Done,
    MlTemporaryPlanFormalDatatypeModuleClosureLoop7Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result {
    MlTemporaryPlanFormalDatatypeModuleClosureLoop10Done(f64),
    MlTemporaryPlanFormalDatatypeModuleClosureLoop10Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_temporary_plan_formal_datatype_module_closure_loop10(mut boundaries: std::rc::Rc<std::cell::RefCell<Vec<(String, Option<f64>)>>>, mut budget: f64, mut path: Option<std::rc::Rc<std::cell::RefCell<Vec<String>>>>, mut depth: Option<f64>, mut key: String, mut part: f64) -> self::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result {
    loop {
        return if (part < ({
            let source_ml_source1_nullish_field_receiver85 = (path.clone(),);
            if match (source_ml_source1_nullish_field_receiver85.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                {
                    let source_ml_source1_nullish_field_present86 = ((source_ml_source1_nullish_field_receiver85.clone()).0).clone().expect("a proven present value");
                    { let ml_reference_array39 = source_ml_source1_nullish_field_present86.clone(); let ml_reference_projection39 = ml_reference_array39.borrow().len() as f64; ml_reference_projection39 }
                }
            } else {
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'length')")).encode_utf16().collect(), message_present: true }).raise()
            }
        })) {
            {
                let value = {
                    let source_ml_source1_nullish_field_receiver87 = (path.clone(), part);
                    {
                        let source_ml_source1_computed_property_key88 = (source_ml_source1_nullish_field_receiver87.clone()).1;
                        if match (source_ml_source1_nullish_field_receiver87.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source1_nullish_field_present89 = ((source_ml_source1_nullish_field_receiver87.clone()).0).clone().expect("a proven present value");
                                {
                                    let source_ml_source1_array_read_value90 = source_ml_source1_nullish_field_present89.clone();
                                    {
                                        let source_ml_source1_array_read_index91 = source_ml_source1_computed_property_key88;
                                        { let ml_reference_array40 = source_ml_source1_array_read_value90.clone(); let ml_reference_position40 = source_ml_source1_array_read_index91; let ml_reference_projection40 = self::ml_array::optional_at(&*ml_reference_array40.borrow(), self::ml_array::number_index(ml_reference_position40)); ml_reference_projection40 }
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (format!("{}{}", (format!("{}{}", (String::from("Cannot read properties of undefined (reading '")), (self::ml_number::js_number((source_ml_source1_nullish_field_receiver87.clone()).1)))), (String::from("')")))).encode_utf16().collect(), message_present: true }).raise()
                        }
                    }
                };
                if (({
                    let source_ml_source1_comparison_operand92 = value.clone();
                    {
                        let source_ml_source1_comparison_operand93 = None::<()>;
                        match (source_ml_source1_comparison_operand92.clone(), source_ml_source1_comparison_operand93.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                    }
                } || ((((value.clone().expect("a proven present value")).encode_utf16().count() as f64)) == (0f64))) || ((((value.clone().expect("a proven present value")).encode_utf16().count() as f64)) > (4096f64))) {
                    self::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                } else {
                    {
                        let budget_2 = (budget + (((value.clone().expect("a proven present value")).encode_utf16().count() as f64) + 1f64));
                        if (budget_2 > (8388608f64)) {
                            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                        } else {
                            {
                                let key_2 = format!("{}{}", key, (format!("{}{}", (format!("{}{}", ({
                                    let source_ml_source1_string_argument94 = ((value.clone().expect("a proven present value")).encode_utf16().count() as f64);
                                    self::ml_number::js_number(source_ml_source1_string_argument94)
                                }), (String::from(":")))), (value.clone().expect("a proven present value")))));
                                if {
                                    let source_ml_source1_comparison_operand95 = part;
                                    {
                                        let source_ml_source1_comparison_operand96 = depth.clone();
                                        match (source_ml_source1_comparison_operand95, source_ml_source1_comparison_operand96.clone()) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false }
                                    }
                                } {
                                    {
                                        let previous = match { let ml_method_owner41 = boundaries.clone(); let ml_method_argument41_0 = key_2.clone(); let ml_method_key41 = ml_method_argument41_0; let ml_method_values41 = ml_method_owner41.borrow(); ml_method_values41.iter().position(|(ml_key, _)| ml_key == &ml_method_key41).map(|index| ml_method_values41[index].1.clone()) } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None };
                                        {
                                            let source_ml_temporary_plan_formal_datatype_module_closure_ignored11 = { let ml_method_owner42 = boundaries.clone(); let ml_method_argument42_0 = key_2.clone(); let ml_method_argument42_1 = if {
                                                let source_ml_source1_comparison_operand97 = previous.clone();
                                                {
                                                    let source_ml_source1_comparison_operand98 = None::<()>;
                                                    match (source_ml_source1_comparison_operand97.clone(), source_ml_source1_comparison_operand98.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                }
                                            } {
                                                depth.clone()
                                            } else {
                                                Some({
                                                    let source_ml_source1_mathematical_argument99 = previous.clone().expect("a proven present value");
                                                    {
                                                        let source_ml_source1_mathematical_argument100 = depth.clone();
                                                        self::ml_math::min(source_ml_source1_mathematical_argument99, {
                                                            let source_ml_source1_numeric_optional101 = source_ml_source1_mathematical_argument100.clone();
                                                            if match (source_ml_source1_numeric_optional101.clone(), None::<f64>) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false } {
                                                                f64::NAN
                                                            } else {
                                                                source_ml_source1_numeric_optional101.clone().expect("a proven present value")
                                                            }
                                                        })
                                                    }
                                                })
                                            }; let ml_method_key42 = ml_method_argument42_0; { let mut ml_method_values42 = ml_method_owner42.borrow_mut(); match ml_method_values42.iter().position(|(ml_key, _)| ml_key == &ml_method_key42) { Some(index) => { ml_method_values42[index].1 = ml_method_argument42_1; }, None => { ml_method_values42.push((ml_method_key42, ml_method_argument42_1)); } } } ml_method_owner42 };
                                            {
                                                let part_2 = (part + 1f64);
                                                {
                                                    (budget, key, part) = (budget_2, key_2.clone(), part_2);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let part_3 = (part + 1f64);
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
            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Done(budget)
        };
    }
}

pub fn source_ml_temporary_plan_formal_datatype_module_closure_loop7(mut paths: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<String>>>>>>, mut depths: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut boundaries: std::rc::Rc<std::cell::RefCell<Vec<(String, Option<f64>)>>>, mut budget: f64, mut index: f64) -> self::MlTemporaryPlanFormalDatatypeModuleClosureLoop7Result {
    loop {
        return if (index < ({ let ml_reference_array43 = paths.clone(); let ml_reference_projection43 = ml_reference_array43.borrow().len() as f64; ml_reference_projection43 })) {
            {
                let path = {
                    let source_ml_source1_array_read_value102 = paths.clone();
                    {
                        let source_ml_source1_array_read_index103 = index;
                        { let ml_reference_array44 = source_ml_source1_array_read_value102.clone(); let ml_reference_position44 = source_ml_source1_array_read_index103; let ml_reference_projection44 = self::ml_array::optional_at(&*ml_reference_array44.borrow(), self::ml_array::number_index(ml_reference_position44)); ml_reference_projection44 }
                    }
                };
                {
                    let depth = {
                        let source_ml_source1_array_read_value104 = depths.clone();
                        {
                            let source_ml_source1_array_read_index105 = index;
                            { let ml_reference_array45 = source_ml_source1_array_read_value104.clone(); let ml_reference_position45 = source_ml_source1_array_read_index105; let ml_reference_projection45 = self::ml_array::optional_at(&*ml_reference_array45.borrow(), self::ml_array::number_index(ml_reference_position45)); ml_reference_projection45 }
                        }
                    };
                    if match (path.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryPlanFormalDatatypeModuleClosureLoop7Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop7Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    } else {
                        if {
                            let source_ml_source1_comparison_operand106 = depth.clone();
                            {
                                let source_ml_source1_comparison_operand107 = None::<()>;
                                match (source_ml_source1_comparison_operand106.clone(), source_ml_source1_comparison_operand107.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                            }
                        } {
                            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop7Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop7Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                        } else {
                            if (((((({ let ml_reference_array46 = path.clone().expect("a proven present value"); let ml_reference_projection46 = ml_reference_array46.borrow().len() as f64; ml_reference_projection46 }) > (256f64)) || !(self::ml_math::is_safe_integer(depth.clone().expect("a proven present value")))) || ((depth.clone().expect("a proven present value")) < (0f64))) || ((depth.clone().expect("a proven present value")) > ({ let ml_reference_array47 = path.clone().expect("a proven present value"); let ml_reference_projection47 = ml_reference_array47.borrow().len() as f64; ml_reference_projection47 }))) || (((depth.clone().expect("a proven present value")) == (0f64)) && (((1f64 / depth.clone().expect("a proven present value"))) < (0f64)))) {
                                self::MlTemporaryPlanFormalDatatypeModuleClosureLoop7Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop7Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                            } else {
                                {
                                    let key = String::from("");
                                    {
                                        let part = 0f64;
                                        {
                                            let source_ml_source1_s108 = self::source_ml_temporary_plan_formal_datatype_module_closure_loop10(boundaries.clone(), budget, Some(path.clone().expect("a proven present value")), Some(depth.clone().expect("a proven present value")), key.clone(), part);
                                            match source_ml_source1_s108.clone() {
                                                self::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Done(budget_2) => {
                                                    {
                                                        let index_2 = (index + 1f64);
                                                        {
                                                            (budget, index) = (budget_2, index_2);
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop10Return(source_ml_temporary_result10) => {
                                                    self::MlTemporaryPlanFormalDatatypeModuleClosureLoop7Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop7Return(source_ml_temporary_result10.clone())
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
            self::MlTemporaryPlanFormalDatatypeModuleClosureLoop7Result::MlTemporaryPlanFormalDatatypeModuleClosureLoop7Done
        };
    }
}

fn ml_main() {
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(Vec::<std::rc::Rc<std::cell::RefCell<Vec<String>>>>::new())), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 1");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()))])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])))), "assertion 2");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))), "assertion 3");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("b")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 1f64])))), "assertion 4");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("b")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64])))), "assertion 5");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a"), String::from("left")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a"), String::from("right")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 1f64])))), "assertion 6");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a"), String::from("inner")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 1f64])))), "assertion 7");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a"), String::from("left")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a"), String::from("right")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a"), String::from("left"), String::from("inner")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, 0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 1f64, 1f64])))), "assertion 8");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("b"), String::from("left")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("b"), String::from("right")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 1f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 1f64, 1f64])))), "assertion 9");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 10");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 11");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-0f64)])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 12");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![0.5f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 13");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("b")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 1f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 14");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("b")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 15");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 16");
    assert!(((self::plan_formal_datatype_module_prefixes(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")]))])), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 17");
    assert!(((self::plan_formal_datatype_module_closure(std::rc::Rc::new(std::cell::RefCell::new(Vec::<std::rc::Rc<std::cell::RefCell<Vec<String>>>>::new())), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 18");
    assert!(((self::plan_formal_datatype_module_closure(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("x")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("y")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("x"), String::from("inner")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("z")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, 2f64, 1f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, 0f64, 1f64])))), "assertion 19");
    assert!(((self::plan_formal_datatype_module_closure(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("x"), String::from("inner")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("x")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("x"), String::from("inner"), String::from("more")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("x"), String::from("other")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 1f64, 3f64, 2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 1f64, 1f64, 2f64])))), "assertion 20");
    assert!(((self::plan_formal_datatype_module_closure(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("x")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("x"), String::from("inner")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("x"), String::from("other")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 1f64, 2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 0f64, 0f64])))), "assertion 21");
    assert!(((self::plan_formal_datatype_module_closure(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a:b"), String::from("c")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a"), String::from("b:c")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 2f64])))), "assertion 22");
    assert!(((self::plan_formal_datatype_module_closure(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("x")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 23");
    assert!(((self::plan_formal_datatype_module_closure(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("x")]))])), std::rc::Rc::new(std::cell::RefCell::new(vec![(-0f64)])))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 24");
    assert!(((self::plan_formal_datatype_module_closure(std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("x")]))])), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))) == (std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))), "assertion 25");
}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || { ml_main() })
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}

#[test]
fn all_literal_formal_datatype_module_controls() { main(); }
