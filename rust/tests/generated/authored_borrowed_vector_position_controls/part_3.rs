pub fn source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop5(mut identities: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut direct_copies: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut count: f64, mut at: f64) -> self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result {
    loop {
        return if (at < count) {
            if (((({
                let source_ml_source2_array_read_value77 = identities.clone();
                {
                    let source_ml_source2_array_read_index78 = at;
                    { let ml_reference_array63 = source_ml_source2_array_read_value77.clone(); let ml_reference_position63 = source_ml_source2_array_read_index78; let ml_reference_projection63 = self::ml_array::optional_at(&*ml_reference_array63.borrow(), self::ml_array::number_index(ml_reference_position63)); ml_reference_projection63 }
                }
            }).clone().unwrap_or_else(|| String::from(""))) == "") || {
                let source_ml_source2_comparison_operand81 = {
                    let source_ml_source2_array_read_value79 = direct_copies.clone();
                    {
                        let source_ml_source2_array_read_index80 = at;
                        { let ml_reference_array64 = source_ml_source2_array_read_value79.clone(); let ml_reference_position64 = source_ml_source2_array_read_index80; let ml_reference_projection64 = self::ml_array::optional_at(&*ml_reference_array64.borrow(), self::ml_array::number_index(ml_reference_position64)); ml_reference_projection64 }
                    }
                };
                {
                    let source_ml_source2_comparison_operand82 = true;
                    !(match (source_ml_source2_comparison_operand81.clone(), source_ml_source2_comparison_operand82) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            }) {
                self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(false)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result {
    MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Done,
    MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Return(bool),
}

pub fn source_ml_source1_temporary_accept_rust_borrowed_vector_completion_profiles_loop6(mut profiles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut index: f64) -> self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result {
    loop {
        return if (index < ({ let ml_reference_array65 = profiles.clone(); let ml_reference_projection65 = ml_reference_array65.borrow().len() as f64; ml_reference_projection65 })) {
            if ((index != (1f64)) && {
                let source_ml_source2_comparison_operand85 = {
                    let source_ml_source2_array_read_value83 = profiles.clone();
                    {
                        let source_ml_source2_array_read_index84 = index;
                        { let ml_reference_array66 = source_ml_source2_array_read_value83.clone(); let ml_reference_position66 = source_ml_source2_array_read_index84; let ml_reference_projection66 = self::ml_array::optional_at(&*ml_reference_array66.borrow(), self::ml_array::number_index(ml_reference_position66)); ml_reference_projection66 }
                    }
                };
                {
                    let source_ml_source2_comparison_operand86 = String::from("");
                    !(match (source_ml_source2_comparison_operand85.clone(), source_ml_source2_comparison_operand86.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            }) {
                self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Result::MlSource1TemporaryAcceptRustBorrowedVectorCompletionProfilesLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result {
    MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Done,
    MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Return(bool),
}

pub fn source_ml_source1_temporary_accept_rust_borrowed_vector_inherent_profiles_loop7(mut profiles: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut index: f64) -> self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result {
    loop {
        return if (index < ({ let ml_reference_array67 = profiles.clone(); let ml_reference_projection67 = ml_reference_array67.borrow().len() as f64; ml_reference_projection67 })) {
            if ((index != (2f64)) && {
                let source_ml_source2_comparison_operand89 = {
                    let source_ml_source2_array_read_value87 = profiles.clone();
                    {
                        let source_ml_source2_array_read_index88 = index;
                        { let ml_reference_array68 = source_ml_source2_array_read_value87.clone(); let ml_reference_position68 = source_ml_source2_array_read_index88; let ml_reference_projection68 = self::ml_array::optional_at(&*ml_reference_array68.borrow(), self::ml_array::number_index(ml_reference_position68)); ml_reference_projection68 }
                    }
                };
                {
                    let source_ml_source2_comparison_operand90 = String::from("");
                    !(match (source_ml_source2_comparison_operand89.clone(), source_ml_source2_comparison_operand90.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }
            }) {
                self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Done
        };
    }
}

pub fn source_ml_source1_temporary_plan_rust_borrowed_vector_result_parameters_loop8(mut lifetime: String, mut references: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut lifetimes: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (index < ({ let ml_reference_array69 = references.clone(); let ml_reference_projection69 = ml_reference_array69.borrow().len() as f64; ml_reference_projection69 })) {
            if match {
                let source_ml_source2_logical97 = {
                    let source_ml_source2_array_read_value91 = references.clone();
                    {
                        let source_ml_source2_array_read_index92 = index;
                        { let ml_reference_array70 = source_ml_source2_array_read_value91.clone(); let ml_reference_position70 = source_ml_source2_array_read_index92; let ml_reference_projection70 = self::ml_array::optional_at(&*ml_reference_array70.borrow(), self::ml_array::number_index(ml_reference_position70)); ml_reference_projection70 }
                    }
                };
                if match source_ml_source2_logical97.clone() { Some(ml_present) => ml_present, None => false } {
                    Some(if ((lifetime != "") && (lifetime != "_")) {
                        {
                            let source_ml_source2_comparison_operand95 = {
                                let source_ml_source2_array_read_value93 = lifetimes.clone();
                                {
                                    let source_ml_source2_array_read_index94 = index;
                                    { let ml_reference_array71 = source_ml_source2_array_read_value93.clone(); let ml_reference_position71 = source_ml_source2_array_read_index94; let ml_reference_projection71 = self::ml_array::optional_at(&*ml_reference_array71.borrow(), self::ml_array::number_index(ml_reference_position71)); ml_reference_projection71 }
                                }
                            };
                            {
                                let source_ml_source2_comparison_operand96 = lifetime.clone();
                                match (source_ml_source2_comparison_operand95.clone(), source_ml_source2_comparison_operand96.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                            }
                        }
                    } else {
                        true
                    })
                } else {
                    source_ml_source2_logical97.clone()
                }
            } { Some(ml_present) => ml_present, None => false } {
                {
                    let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value72 = result.borrow().clone(); ml_reference_value72 }, vec![index])));
                    {
                        let index_2 = (index + 1f64);
                        {
                            (result, index) = (result_2.clone(), index_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let index_3 = (index + 1f64);
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result {
    MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Done,
    MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_read_rust_borrowed_expression_position_reservation_loop10(mut source_ml_source1_temporary_values9: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index9: self::ml::Big) -> self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result {
    loop {
        return if (source_ml_source1_temporary_index9 < ({ let ml_reference_array73 = source_ml_source1_temporary_values9.clone(); let ml_reference_projection73 = self::ml::Big::from_u128(ml_reference_array73.borrow().len() as u128); ml_reference_projection73 })) {
            {
                let value = {
                    let source_ml_source2_array_read_value98 = source_ml_source1_temporary_values9.clone();
                    {
                        let source_ml_source2_array_read_index99 = source_ml_source1_temporary_index9.clone();
                        { let ml_reference_array74 = source_ml_source2_array_read_value98.clone(); let ml_reference_position74 = source_ml_source2_array_read_index99.clone(); let ml_reference_projection74 = self::ml_array::at(&*ml_reference_array74.borrow(), ml_reference_position74.to_index()); ml_reference_projection74 }
                    }
                };
                if (!(self::ml_math::is_safe_integer(value)) || (value < (0f64))) {
                    self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                } else {
                    {
                        let source_ml_source1_temporary_index9_2 = source_ml_source1_temporary_index9.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index9 = source_ml_source1_temporary_index9_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result {
    MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Done(f64, f64),
    MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop14(mut target_boxes: f64, mut target_references: f64, mut source_ml_source1_temporary_values12: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut source_ml_source1_temporary_index12: self::ml::Big) -> self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result {
    loop {
        return if (source_ml_source1_temporary_index12 < ({ let ml_reference_array75 = source_ml_source1_temporary_values12.clone(); let ml_reference_projection75 = self::ml::Big::from_u128(ml_reference_array75.borrow().len() as u128); ml_reference_projection75 })) {
            {
                let layer = {
                    let source_ml_source2_array_read_value100 = source_ml_source1_temporary_values12.clone();
                    {
                        let source_ml_source2_array_read_index101 = source_ml_source1_temporary_index12.clone();
                        { let ml_reference_array76 = source_ml_source2_array_read_value100.clone(); let ml_reference_position76 = source_ml_source2_array_read_index101.clone(); let ml_reference_projection76 = self::ml_array::at(&*ml_reference_array76.borrow(), ml_reference_position76.to_index()); ml_reference_projection76 }
                    }
                };
                if (layer == "box") {
                    {
                        let target_boxes_2 = (target_boxes + 1f64);
                        {
                            let source_ml_source1_temporary_index12_2 = source_ml_source1_temporary_index12.add(&self::ml::Big::from_i128(1));
                            {
                                (target_boxes, source_ml_source1_temporary_index12) = (target_boxes_2, source_ml_source1_temporary_index12_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    if (layer == "reference") {
                        {
                            let target_references_2 = (target_references + 1f64);
                            {
                                let source_ml_source1_temporary_index12_3 = source_ml_source1_temporary_index12.add(&self::ml::Big::from_i128(1));
                                {
                                    (target_references, source_ml_source1_temporary_index12) = (target_references_2, source_ml_source1_temporary_index12_3.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Done(target_boxes, target_references)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result {
    MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Done(f64, f64),
    MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop13(mut actual_boxes: f64, mut actual_references: f64, mut source_ml_source1_temporary_values11: std::rc::Rc<std::cell::RefCell<Vec<String>>>, mut source_ml_source1_temporary_index11: self::ml::Big) -> self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result {
    loop {
        return if (source_ml_source1_temporary_index11 < ({ let ml_reference_array77 = source_ml_source1_temporary_values11.clone(); let ml_reference_projection77 = self::ml::Big::from_u128(ml_reference_array77.borrow().len() as u128); ml_reference_projection77 })) {
            {
                let layer = {
                    let source_ml_source2_array_read_value102 = source_ml_source1_temporary_values11.clone();
                    {
                        let source_ml_source2_array_read_index103 = source_ml_source1_temporary_index11.clone();
                        { let ml_reference_array78 = source_ml_source2_array_read_value102.clone(); let ml_reference_position78 = source_ml_source2_array_read_index103.clone(); let ml_reference_projection78 = self::ml_array::at(&*ml_reference_array78.borrow(), ml_reference_position78.to_index()); ml_reference_projection78 }
                    }
                };
                if (layer == "box") {
                    {
                        let actual_boxes_2 = (actual_boxes + 1f64);
                        {
                            let source_ml_source1_temporary_index11_2 = source_ml_source1_temporary_index11.add(&self::ml::Big::from_i128(1));
                            {
                                (actual_boxes, source_ml_source1_temporary_index11) = (actual_boxes_2, source_ml_source1_temporary_index11_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    if (layer == "reference") {
                        {
                            let actual_references_2 = (actual_references + 1f64);
                            {
                                let source_ml_source1_temporary_index11_3 = source_ml_source1_temporary_index11.add(&self::ml::Big::from_i128(1));
                                {
                                    (actual_references, source_ml_source1_temporary_index11) = (actual_references_2, source_ml_source1_temporary_index11_3.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Done(actual_boxes, actual_references)
        };
    }
}

fn ml_main() {
    assert!(((self::accept_rust_borrowed_vector_signature_model(String::from("Rust source function signature 3"))) == (true)), "assertion 1");
    assert!(((self::accept_rust_borrowed_vector_signature_model(String::from("Rust source function signature 2"))) == (false)), "assertion 2");
    assert!(((self::read_rust_borrowed_vector_position_error(String::from("Rust source function signature 1"), true)) == "a retained array element position requires the borrowed vector source signature profile"), "assertion 3");
    assert!(((self::read_rust_borrowed_vector_position_error(String::from("Rust source function signature 2"), true)) == "a retained array element position requires the borrowed vector source signature profile"), "assertion 4");
    assert!(((self::read_rust_borrowed_vector_position_error(String::from("Rust source function signature 3"), true)) == ""), "assertion 5");
    assert!(((self::read_rust_borrowed_vector_position_error(String::from("Rust source function signature 2"), false)) == ""), "assertion 6");
    assert!(((self::retain_rust_borrowed_vector_element_position(std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), false)) == (true)), "assertion 7");
    assert!(((self::retain_rust_borrowed_vector_element_position(std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())), true)) == (true)), "assertion 8");
    assert!(((self::retain_rust_borrowed_vector_element_position(std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new())), false)) == (false)), "assertion 9");
    assert!(((self::read_rust_borrowed_vector_element_position_error(String::from("array"), String::from("library"), String::from("std::vec::Vec"), 1f64, true, true)) == ""), "assertion 10");
    assert!(((self::read_rust_borrowed_vector_element_position_error(String::from("array"), String::from("declaration"), String::from("std::vec::Vec"), 1f64, true, true)) == "a retained array element position must match its complete canonical vector element type and owner"), "assertion 11");
    assert!(((self::read_rust_borrowed_vector_element_position_error(String::from("array"), String::from("library"), String::from("std::vec::Vec"), 1f64, true, false)) == "a retained array element position must match its complete canonical vector element type and owner"), "assertion 12");
    assert!(((self::accept_rust_borrowed_vector_element(true, 90f64, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), true, false)) == (true)), "assertion 13");
    assert!(((self::accept_rust_borrowed_vector_element(true, 99f64, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("")])), true, false)) == (true)), "assertion 14");
    assert!(((self::accept_rust_borrowed_vector_element(true, 100f64, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), true, false)) == (false)), "assertion 15");
    assert!(((self::accept_rust_borrowed_vector_element(true, 90f64, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), true, true)) == (false)), "assertion 16");
    assert!(((self::accept_rust_borrowed_vector_element(true, 90f64, std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("reference"), String::from("box")])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a")])), true, false)) == (false)), "assertion 17");
    assert!(((self::read_rust_borrowed_vector_position_budget_error(100000f64, 128f64)) == ""), "assertion 18");
    assert!(((self::read_rust_borrowed_vector_position_budget_error(100001f64, 128f64)) == "retained borrowed vector element positions exceed their finite source profile"), "assertion 19");
    assert!(((self::read_rust_borrowed_vector_position_budget_error(f64::NAN, 0f64)) == "retained borrowed vector element positions exceed their finite source profile"), "assertion 20");
    assert!(((self::read_rust_borrowed_vector_position_budget_error(1f64, 1.5f64)) == "retained borrowed vector element positions exceed their finite source profile"), "assertion 21");
    assert!(((self::accept_rust_borrowed_vector_completion_profiles(String::from("Rust source function signature 3"), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("Rust source function signature 3"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")])))) == (true)), "assertion 22");
    assert!(((self::accept_rust_borrowed_vector_completion_profiles(String::from("Rust source function signature 3"), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("Rust source function signature 3"), String::from("other"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")])))) == (false)), "assertion 23");
    assert!(((self::accept_rust_borrowed_vector_source(String::from("Rust source function signature 3"), true, 90f64)) == (true)), "assertion 24");
    assert!(((self::accept_rust_borrowed_vector_source(String::from("Rust source function signature 3"), true, 99f64)) == (true)), "assertion 25");
    assert!(((self::accept_rust_borrowed_vector_source(String::from("Rust source function signature 3"), true, 100f64)) == (false)), "assertion 26");
    assert!(((self::accept_rust_borrowed_vector_owned_interpretation(false)) == (true)), "assertion 27");
    assert!(((self::accept_rust_borrowed_vector_owned_interpretation(true)) == (false)), "assertion 28");
    assert!(((self::plan_rust_borrowed_vector_result_parameters(String::from("a"), std::rc::Rc::new(std::cell::RefCell::new(vec![true, false, true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("a"), String::from(""), String::from("a")])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 2f64])))), "assertion 29");
    assert!(((self::plan_rust_borrowed_vector_result_parameters(String::from(""), std::rc::Rc::new(std::cell::RefCell::new(vec![true, false])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("")])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])))), "assertion 30");
    assert!(((self::plan_rust_borrowed_vector_result_parameters(String::from(""), std::rc::Rc::new(std::cell::RefCell::new(vec![true, true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from(""), String::from("")])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])))), "assertion 31");
    assert!(((self::plan_rust_borrowed_vector_result_parameters(String::from("static"), std::rc::Rc::new(std::cell::RefCell::new(vec![true, true])), std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("static"), String::from("a")])))) == (std::rc::Rc::new(std::cell::RefCell::new(vec![0f64])))), "assertion 32");
    assert!(((self::read_rust_borrowed_vector_argument_error(true, true, true, true, String::from("a"), String::from("static"))) == "borrowed vector arguments must retain their actual reference element owner and region contract"), "assertion 33");
    assert!(((self::read_rust_borrowed_vector_argument_error(true, false, true, true, String::from("a"), String::from(""))) == "borrowed vector arguments must retain their actual reference element owner and region contract"), "assertion 34");
    assert!(((self::read_rust_borrowed_vector_argument_error(true, true, true, true, String::from("a"), String::from("b"))) == ""), "assertion 35");
    assert!(((self::accept_rust_borrowed_vector_literal_context(true, true, false, 0f64)) == (true)), "assertion 36");
    assert!(((self::accept_rust_borrowed_vector_literal_context(true, true, true, 2f64)) == (true)), "assertion 37");
    assert!(((self::accept_rust_borrowed_vector_literal_context(true, true, false, 1f64)) == (false)), "assertion 38");
    assert!(((self::accept_rust_borrowed_vector_literal_context(true, false, false, 0f64)) == (false)), "assertion 39");
    assert!(((self::accept_rust_borrowed_vector_literal_context(false, true, false, 0f64)) == (false)), "assertion 40");
    assert!(((self::accept_rust_borrowed_vector_literal_context(true, true, true, 65537f64)) == (false)), "assertion 41");
    assert!(((self::read_rust_borrowed_vector_empty_constructor(true, String::from("Vec"), String::from("new"), 0f64)) == "construct"), "assertion 42");
    assert!(((self::read_rust_borrowed_vector_empty_constructor(true, String::from("std::vec::Vec"), String::from("new"), 0f64)) == "construct"), "assertion 43");
    assert!(((self::read_rust_borrowed_vector_empty_constructor(true, String::from("std::vec::Vec"), String::from("new"), 1f64)) == "arity"), "assertion 44");
    assert!(((self::read_rust_borrowed_vector_empty_constructor(false, String::from("Vec"), String::from("new"), 0f64)) == ""), "assertion 45");
    assert!(((self::read_rust_borrowed_vector_empty_constructor(true, String::from("my::Vec"), String::from("new"), 0f64)) == ""), "assertion 46");
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
fn all_literal_borrowed_vector_position_controls() { main(); }
