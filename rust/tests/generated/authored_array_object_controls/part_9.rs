pub fn source_ml_temporary_plan_java_script_array_property_definition_loop28(mut index: f64, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_values27: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index27: self::ml::Big) -> self::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Result {
    loop {
        return if (source_ml_temporary_index27 < ({ let ml_reference_array190 = source_ml_temporary_values27.clone(); let ml_reference_projection190 = self::ml::Big::from_u128(ml_reference_array190.borrow().len() as u128); ml_reference_projection190 })) {
            {
                let property = {
                    let source_ml_source1_array_read_value1518 = source_ml_temporary_values27.clone();
                    {
                        let source_ml_source1_array_read_index1519 = source_ml_temporary_index27.clone();
                        { let ml_reference_array191 = source_ml_source1_array_read_value1518.clone(); let ml_reference_position191 = source_ml_source1_array_read_index1519.clone(); let ml_reference_projection191 = self::ml_array::at(&*ml_reference_array191.borrow(), ml_reference_position191.to_index()); ml_reference_projection191 }
                    }
                };
                {
                    let source_ml_temporary_plan_java_script_array_property_definition_ignored29 = { let ml_method_owner192 = properties.clone(); let ml_method_argument192_0 = if self::test_java_script_array_length_key(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1520, _) => {
                            let source_ml_source1_data_property_value1520 = *source_ml_source1_data_property_value1520;
                            source_ml_source1_data_property_value1520.clone()
                        }
                    }) {
                        {
                            let source_ml_source1_nominal_spread1536 = property.clone();
                            {
                                let source_ml_source1_nominal_field1537 = match source_ml_source1_nominal_spread1536.clone() {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1538, _) => {
                                        let source_ml_source1_data_property_value1538 = *source_ml_source1_data_property_value1538;
                                        source_ml_source1_data_property_value1538.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field1539 = match source_ml_source1_nominal_spread1536.clone() {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1540) => {
                                            source_ml_source1_data_property_value1540.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_override1541 = std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source1_spread1522 = { let ml_reference_value193 = (match property.clone() {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1521) => {
                                                    source_ml_source1_data_property_value1521.clone()
                                                }
                                            }).borrow().clone(); ml_reference_value193 };
                                            {
                                                let source_ml_source1_spread_field1523 = (source_ml_source1_spread1522.clone()).0;
                                                {
                                                    let source_ml_source1_spread_field1524 = (source_ml_source1_spread1522.clone()).1;
                                                    {
                                                        let source_ml_source1_spread_field1525 = (source_ml_source1_spread1522.clone()).2;
                                                        {
                                                            let source_ml_source1_spread_field1526 = (source_ml_source1_spread1522.clone()).3;
                                                            {
                                                                let source_ml_source1_spread_field1527 = (source_ml_source1_spread1522.clone()).4;
                                                                {
                                                                    let source_ml_source1_spread_field1528 = (source_ml_source1_spread1522.clone()).5;
                                                                    {
                                                                        let source_ml_source1_spread_field1529 = (source_ml_source1_spread1522.clone()).6;
                                                                        {
                                                                            let source_ml_source1_spread_field1530 = (source_ml_source1_spread1522.clone()).7;
                                                                            {
                                                                                let source_ml_source1_spread_field1531 = (source_ml_source1_spread1522.clone()).8;
                                                                                {
                                                                                    let source_ml_source1_spread_field1532 = (source_ml_source1_spread1522.clone()).9;
                                                                                    {
                                                                                        let source_ml_source1_spread_field1533 = (source_ml_source1_spread1522.clone()).10;
                                                                                        {
                                                                                            let source_ml_source1_spread_field1534 = (source_ml_source1_spread1522.clone()).11;
                                                                                            {
                                                                                                let source_ml_source1_field1535 = self::JavaScriptSemanticValue::Number((index + 1f64));
                                                                                                (source_ml_source1_spread_field1523, source_ml_source1_spread_field1524, source_ml_source1_spread_field1525, source_ml_source1_spread_field1526, source_ml_source1_spread_field1527.clone(), source_ml_source1_spread_field1528, source_ml_source1_spread_field1529.clone(), source_ml_source1_spread_field1530, source_ml_source1_field1535.clone(), source_ml_source1_spread_field1532, source_ml_source1_spread_field1533, source_ml_source1_spread_field1534)
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
                                        }));
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source1_nominal_field1537.clone()), source_ml_source1_nominal_override1541.clone())
                                    }
                                }
                            }
                        }
                    } else {
                        property.clone()
                    }; let ml_inserted = vec![ml_method_argument192_0]; let mut ml_method_values192 = ml_method_owner192.borrow_mut(); let ml_length = ml_method_values192.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values192.extend(ml_inserted); ml_length as f64 };
                    {
                        let source_ml_temporary_index27_2 = source_ml_temporary_index27.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index27 = source_ml_temporary_index27_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Result::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Done
        };
    }
}

fn ml_main() {
    assert!(((self::test_java_script_array_length(0f64)) == (true)), "assertion 1");
    assert!(((self::test_java_script_array_length(4294967295f64)) == (true)), "assertion 2");
    assert!(((self::test_java_script_array_length(4294967296f64)) == (false)), "assertion 3");
    assert!(((self::test_java_script_array_length((-0f64))) == (false)), "assertion 4");
    assert!(((self::test_java_script_array_length(f64::NAN)) == (false)), "assertion 5");
    assert!(self::ml_number::same_value(self::read_java_script_array_index(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![48f64])))), 0f64), "assertion 6");
    assert!(self::ml_number::same_value(self::read_java_script_array_index(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![52, 50, 57, 52, 57, 54, 55, 50, 57, 52]))), 4294967294f64), "assertion 7");
    assert!(self::ml_number::same_value(self::read_java_script_array_index(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![52, 50, 57, 52, 57, 54, 55, 50, 57, 53]))), (-1f64)), "assertion 8");
    assert!(self::ml_number::same_value(self::read_java_script_array_index(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![48, 49]))), (-1f64)), "assertion 9");
    assert!(self::ml_number::same_value(self::convert_java_script_array_unsigned_length((-1f64)), 4294967295f64), "assertion 10");
    assert!(self::ml_number::same_value(self::convert_java_script_array_unsigned_length((-0f64)), 0f64), "assertion 11");
    assert!(self::ml_number::same_value(self::convert_java_script_array_unsigned_length(f64::INFINITY), 0f64), "assertion 12");
    assert!(self::ml_number::same_value(self::convert_java_script_array_unsigned_length(4294967297f64), 1f64), "assertion 13");
    assert!(((self::test_java_script_array_length_key(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))) == (true)), "assertion 14");
    assert!(((self::test_java_script_array_length_key(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![48f64]))))) == (false)), "assertion 15");
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
fn all_literal_array_object_controls() { main(); }
