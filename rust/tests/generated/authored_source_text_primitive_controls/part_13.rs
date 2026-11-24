pub fn read_java_script_mathematical_conversion_form(language: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    if (language != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 65, 114, 103, 117, 109, 101, 110, 116, 115]))) {
            self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121])
        } else {
            if ((operation == (self::ml_string::Text::from_units(vec![109, 97, 120]))) || (operation == (self::ml_string::Text::from_units(vec![109, 105, 110])))) {
                self::ml_string::Text::from_units(vec![118, 97, 114, 105, 97, 100, 105, 99])
            } else {
                if (((((((operation == (self::ml_string::Text::from_units(vec![97, 98, 115]))) || (operation == (self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114])))) || (operation == (self::ml_string::Text::from_units(vec![99, 101, 105, 108])))) || (operation == (self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99])))) || (operation == (self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100])))) || (operation == (self::ml_string::Text::from_units(vec![115, 105, 103, 110])))) || (operation == (self::ml_string::Text::from_units(vec![115, 113, 114, 116])))) {
                    self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn render_java_script_number_scalar_conversion(format: self::ml_string::Text, operation: self::ml_string::Text, form: self::ml_string::Text, value: self::ml_string::Text, utf16: bool) -> self::ml_string::Text {
    if ((!(self::accept_linked_java_script_number_conversion(self::ml_string::Text::from_units(vec![109, 97, 116, 104]), operation.clone())) || ((self::read_java_script_number_conversion_form(form.clone())) == (self::ml_string::Text::from_units(vec![117, 110, 115, 117, 112, 112, 111, 114, 116, 101, 100])))) || (form == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (format == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) {
                ((self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                ((self::ml_string::Text::from_units(vec![40, 43, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
            }
        } else {
            if ((format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (format == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                if !(self::read_java_script_number_conversion_failure(operation.clone(), form.clone())).0.is_empty() {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    if (form == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                        value.clone()
                    } else {
                        if (form == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
                            if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                ((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32, 95, 32, 58, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 40, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 47, 32, 48, 41, 41]))
                            } else {
                                ((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32, 95, 32, 58, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 105, 110, 32, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 110, 97, 110, 41]))
                            }
                        } else {
                            if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
                                if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                    ((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 116, 104, 101, 110, 32, 40, 49, 32, 58, 32, 70, 108, 111, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 48, 32, 58, 32, 70, 108, 111, 97, 116, 41, 41]))
                                } else {
                                    ((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 116, 104, 101, 110, 32, 49, 37, 102, 108, 111, 97, 116, 32, 101, 108, 115, 101, 32, 48, 37, 102, 108, 111, 97, 116, 41]))
                                }
                            } else {
                                if ((form == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (form == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
                                    ((self::ml_string::Text::from_units(vec![40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 73, 110, 116, 101, 103, 101, 114, 32])).concat(&if (form == (self::ml_string::Text::from_units(vec![110, 97, 116]))) {
                                        if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                            ((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                        } else {
                                            ((self::ml_string::Text::from_units(vec![40, 90, 46, 111, 102, 95, 78, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                        }
                                    } else {
                                        value.clone()
                                    })).concat(&self::ml_string::Text::from_units(vec![41]))
                                } else {
                                    if (format == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                                        ((self::ml_string::Text::from_units(vec![40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 85, 110, 105, 116, 115, 32, 40, 83, 116, 114, 105, 110, 103, 46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32, 40, 117, 110, 105, 116, 115, 32, 58, 32, 65, 114, 114, 97, 121, 32, 70, 108, 111, 97, 116, 41, 32, 99, 32, 61, 62, 32, 105, 102, 32, 99, 46, 116, 111, 78, 97, 116, 32, 60, 32, 54, 53, 53, 51, 54, 32, 116, 104, 101, 110, 32, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 99, 46, 116, 111, 78, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 53, 50, 57, 54, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 47, 32, 49, 48, 50, 52, 41, 41, 41, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 54, 51, 50, 48, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 37, 32, 49, 48, 50, 52, 41, 41, 41, 32, 35, 91, 93, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                    } else {
                                        ((self::ml_string::Text::from_units(vec![40, 77, 76, 83, 116, 114, 105, 110, 103, 78, 117, 109, 98, 101, 114, 46, 99, 111, 110, 118, 101, 114, 116, 85, 116, 102, 56, 32, 40, 40, 102, 105, 120, 32, 109, 108, 95, 110, 117, 109, 101, 114, 105, 99, 95, 98, 121, 116, 101, 115, 32, 40, 116, 101, 120, 116, 32, 58, 32, 115, 116, 114, 105, 110, 103, 41, 32, 58, 32, 108, 105, 115, 116, 32, 102, 108, 111, 97, 116, 32, 58, 61, 32, 109, 97, 116, 99, 104, 32, 116, 101, 120, 116, 32, 119, 105, 116, 104, 32, 69, 109, 112, 116, 121, 83, 116, 114, 105, 110, 103, 32, 61, 62, 32, 110, 105, 108, 32, 124, 32, 83, 116, 114, 105, 110, 103, 32, 99, 32, 114, 101, 115, 116, 32, 61, 62, 32, 99, 111, 110, 115, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 111, 102, 95, 117, 105, 110, 116, 54, 51, 32, 40, 85, 105, 110, 116, 54, 51, 46, 111, 102, 95, 90, 32, 40, 90, 46, 111, 102, 95, 110, 97, 116, 32, 40, 65, 115, 99, 105, 105, 46, 110, 97, 116, 95, 111, 102, 95, 97, 115, 99, 105, 105, 32, 99, 41, 41, 41, 41, 32, 40, 109, 108, 95, 110, 117, 109, 101, 114, 105, 99, 95, 98, 121, 116, 101, 115, 32, 114, 101, 115, 116, 41, 32, 101, 110, 100, 41, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if (format != (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    if (form == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                        value.clone()
                    } else {
                        if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
                            ((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 49, 46, 48, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 48, 46, 48, 102, 54, 52, 32, 125, 41]))
                        } else {
                            if (form == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
                                ((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 95, 32, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 102, 54, 52, 58, 58, 78, 65, 78, 32, 125]))
                            } else {
                                if ((form == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (form == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
                                    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114]))) {
                                        ((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 95, 32, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 111, 117, 114, 99, 101, 95, 101, 120, 99, 101, 112, 116, 105, 111, 110, 58, 58, 83, 111, 117, 114, 99, 101, 69, 120, 99, 101, 112, 116, 105, 111, 110, 58, 58, 102, 114, 111, 109, 95, 116, 101, 120, 116, 40, 34, 84, 121, 112, 101, 69, 114, 114, 111, 114, 34, 44, 32, 34, 67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114, 34, 41, 46, 114, 97, 105, 115, 101, 40, 41, 32, 125]))
                                    } else {
                                        ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 110, 117, 109, 98, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                                    }
                                } else {
                                    {
                                        let units = if utf16 {
                                            ((self::ml_string::Text::from_units(vec![40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 46, 48, 46, 105, 116, 101, 114, 40, 41, 46, 99, 111, 112, 105, 101, 100, 40, 41]))
                                        } else {
                                            ((self::ml_string::Text::from_units(vec![40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 46, 101, 110, 99, 111, 100, 101, 95, 117, 116, 102, 49, 54, 40, 41]))
                                        };
                                        ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 115, 116, 114, 105, 110, 103, 95, 110, 117, 109, 98, 101, 114, 95, 117, 110, 105, 116, 115, 40, 38])).concat(&units)).concat(&self::ml_string::Text::from_units(vec![46, 109, 97, 112, 40, 102, 54, 52, 58, 58, 102, 114, 111, 109, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 102, 54, 52, 62, 62, 40, 41, 41]))
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

pub fn read_java_script_number_conversion_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![69, 67, 77, 65, 83, 99, 114, 105, 112, 116, 32, 78, 117, 109, 98, 101, 114, 32, 97, 110, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 112, 114, 101, 115, 101, 114, 118, 101, 32, 102, 117, 108, 108, 32, 85, 84, 70, 45, 49, 54, 32, 83, 116, 114, 105, 110, 103, 78, 117, 109, 101, 114, 105, 99, 76, 105, 116, 101, 114, 97, 108, 32, 115, 121, 110, 116, 97, 120, 44, 32, 101, 120, 97, 99, 116, 32, 98, 105, 110, 97, 114, 121, 54, 52, 32, 116, 105, 101, 115, 45, 116, 111, 45, 101, 118, 101, 110, 32, 114, 111, 117, 110, 100, 105, 110, 103, 44, 32, 115, 105, 103, 110, 101, 100, 32, 122, 101, 114, 111, 44, 32, 73, 110, 102, 105, 110, 105, 116, 121, 44, 32, 78, 97, 78, 44, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 97, 98, 115, 101, 110, 99, 101, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 101, 118, 97, 108, 117, 97, 116, 105, 111, 110, 32, 111, 114, 100, 101, 114, 59, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 111, 117, 110, 100, 115, 32, 111, 110, 99, 101, 32, 97, 110, 100, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 116, 104, 114, 111, 119, 115])
}

pub fn read_java_script_radix_digit(unit: f64) -> f64 {
    if ((unit >= (48f64)) && (unit <= (57f64))) {
        (unit - 48f64)
    } else {
        if ((unit >= (65f64)) && (unit <= (90f64))) {
            (unit - 55f64)
        } else {
            if ((unit >= (97f64)) && (unit <= (122f64))) {
                (unit - 87f64)
            } else {
                (-1f64)
            }
        }
    }
}

pub fn parse_java_script_integer_prefix(units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, radix_number: f64) -> f64 {
    {
        let start = 0f64;
        {
            let start_2 = self::source_ml_source1_temporary_parse_java_script_integer_prefix_loop41(units.clone(), start);
            {
                let negative = false;
                {
                    let sign = ({ let ml_reference_array31 = units.clone(); let ml_reference_position31 = start_2; let ml_reference_projection31 = self::ml_array::optional_at(&*ml_reference_array31.borrow(), self::ml_array::number_index(ml_reference_position31)); ml_reference_projection31 }).clone().unwrap_or_else(|| (-1f64));
                    {
                        let source_ml_source2_s537 = if ((sign == (43f64)) || (sign == (45f64))) {
                            {
                                let negative_2 = (sign == (45f64));
                                {
                                    let start_3 = (start_2 + 1f64);
                                    self::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin42::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin42Next(start_3, negative_2)
                                }
                            }
                        } else {
                            self::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin42::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin42Next(start_2, negative)
                        };
                        match source_ml_source2_s537.clone() {
                            self::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin42::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin42Next(start_4, negative_3) => {
                                {
                                    let radix = self::normalize_java_script_signed_integer(radix_number);
                                    {
                                        let strip = ((radix == (0f64)) || (radix == (16f64)));
                                        if ((radix != (0f64)) && ((radix < (2f64)) || (radix > (36f64)))) {
                                            f64::NAN
                                        } else {
                                            {
                                                let radix_3 = if (radix == (0f64)) {
                                                    {
                                                        let radix_2 = 10f64;
                                                        radix_2
                                                    }
                                                } else {
                                                    radix
                                                };
                                                {
                                                    let source_ml_source2_s550 = if ((strip && ((({ let ml_reference_array32 = units.clone(); let ml_reference_position32 = start_4; let ml_reference_projection32 = self::ml_array::optional_at(&*ml_reference_array32.borrow(), self::ml_array::number_index(ml_reference_position32)); ml_reference_projection32 }).clone().unwrap_or_else(|| (-1f64))) == (48f64))) && (((({
                                                        let source_ml_source2_array_read_index558 = (start_4 + 1f64);
                                                        { let ml_reference_array33 = units.clone(); let ml_reference_position33 = source_ml_source2_array_read_index558; let ml_reference_projection33 = self::ml_array::optional_at(&*ml_reference_array33.borrow(), self::ml_array::number_index(ml_reference_position33)); ml_reference_projection33 }
                                                    }).clone().unwrap_or_else(|| (-1f64))) == (120f64)) || ((({
                                                        let source_ml_source2_array_read_index562 = (start_4 + 1f64);
                                                        { let ml_reference_array34 = units.clone(); let ml_reference_position34 = source_ml_source2_array_read_index562; let ml_reference_projection34 = self::ml_array::optional_at(&*ml_reference_array34.borrow(), self::ml_array::number_index(ml_reference_position34)); ml_reference_projection34 }
                                                    }).clone().unwrap_or_else(|| (-1f64))) == (88f64)))) {
                                                        {
                                                            let radix_4 = 16f64;
                                                            {
                                                                let start_5 = (start_4 + 2f64);
                                                                self::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin43::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin43Next(start_5, radix_4)
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin43::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin43Next(start_4, radix_3)
                                                    };
                                                    match source_ml_source2_s550.clone() {
                                                        self::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin43::MlSource1TemporaryParseJavaScriptIntegerPrefixJoin43Next(start_6, radix_5) => {
                                                            {
                                                                let value = self::ml::Big::from_i128(0);
                                                                {
                                                                    let count = 0f64;
                                                                    {
                                                                        let base = {
                                                                            let source_ml_source2_integer_result564 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(radix_5);
                                                                            if (((source_ml_source2_integer_result564.len() as f64)) == (1f64)) {
                                                                                (self::ml_array::optional_at(&source_ml_source2_integer_result564, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(radix_5)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_s565 = self::source_ml_source1_temporary_parse_java_script_integer_prefix_loop44(units.clone(), radix_5, value.clone(), count, base.clone(), start_6);
                                                                            match source_ml_source2_s565.clone() {
                                                                                self::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop44Result::MlSource1TemporaryParseJavaScriptIntegerPrefixLoop44Done(value_2, count_2) => {
                                                                                    if (count_2 == (0f64)) {
                                                                                        f64::NAN
                                                                                    } else {
                                                                                        {
                                                                                            let result = self::convert_java_script_big_integer_number(value_2.clone());
                                                                                            if negative_3 {
                                                                                                (-result)
                                                                                            } else {
                                                                                                result
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
    }
}

pub fn parse_java_script_float_prefix(units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> f64 {
    {
        let start = 0f64;
        {
            let start_2 = self::source_ml_source1_temporary_parse_java_script_float_prefix_loop45(units.clone(), start);
            {
                let at = start_2;
                {
                    let sign = ({ let ml_reference_array35 = units.clone(); let ml_reference_position35 = at; let ml_reference_projection35 = self::ml_array::optional_at(&*ml_reference_array35.borrow(), self::ml_array::number_index(ml_reference_position35)); ml_reference_projection35 }).clone().unwrap_or_else(|| (-1f64));
                    {
                        let at_3 = if ((sign == (43f64)) || (sign == (45f64))) {
                            {
                                let at_2 = (at + 1f64);
                                at_2
                            }
                        } else {
                            at
                        };
                        {
                            let infinity = std::rc::Rc::new(std::cell::RefCell::new(vec![73f64, 110f64, 102f64, 105f64, 110f64, 105f64, 116f64, 121f64]));
                            {
                                let matches = true;
                                {
                                    let index = 0f64;
                                    {
                                        let matches_2 = self::source_ml_source1_temporary_parse_java_script_float_prefix_loop46(units.clone(), at_3, infinity.clone(), matches, index);
                                        if matches_2 {
                                            if (sign == (45f64)) {
                                                f64::NEG_INFINITY
                                            } else {
                                                f64::INFINITY
                                            }
                                        } else {
                                            {
                                                let digits = 0f64;
                                                {
                                                    let source_ml_source2_s568 = self::source_ml_source1_temporary_parse_java_script_float_prefix_loop47(units.clone(), at_3, digits);
                                                    match source_ml_source2_s568.clone() {
                                                        self::MlSource1TemporaryParseJavaScriptFloatPrefixLoop47Result::MlSource1TemporaryParseJavaScriptFloatPrefixLoop47Done(at_4, digits_2) => {
                                                            {
                                                                let source_ml_source2_s574 = if ((({ let ml_reference_array36 = units.clone(); let ml_reference_position36 = at_4; let ml_reference_projection36 = self::ml_array::optional_at(&*ml_reference_array36.borrow(), self::ml_array::number_index(ml_reference_position36)); ml_reference_projection36 }).clone().unwrap_or_else(|| (-1f64))) == (46f64)) {
                                                                    {
                                                                        let at_5 = (at_4 + 1f64);
                                                                        {
                                                                            let source_ml_source2_s579 = self::source_ml_source1_temporary_parse_java_script_float_prefix_loop50(units.clone(), at_5, digits_2);
                                                                            match source_ml_source2_s579.clone() {
                                                                                self::MlSource1TemporaryParseJavaScriptFloatPrefixLoop50Result::MlSource1TemporaryParseJavaScriptFloatPrefixLoop50Done(at_6, digits_3) => {
                                                                                    self::MlSource1TemporaryParseJavaScriptFloatPrefixJoin48::MlSource1TemporaryParseJavaScriptFloatPrefixJoin48Next(at_6, digits_3)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlSource1TemporaryParseJavaScriptFloatPrefixJoin48::MlSource1TemporaryParseJavaScriptFloatPrefixJoin48Next(at_4, digits_2)
                                                                };
                                                                match source_ml_source2_s574.clone() {
                                                                    self::MlSource1TemporaryParseJavaScriptFloatPrefixJoin48::MlSource1TemporaryParseJavaScriptFloatPrefixJoin48Next(at_7, digits_4) => {
                                                                        if (digits_4 == (0f64)) {
                                                                            f64::NAN
                                                                        } else {
                                                                            {
                                                                                let mantissa_end = at_7;
                                                                                {
                                                                                    let at_12 = if (((({ let ml_reference_array37 = units.clone(); let ml_reference_position37 = at_7; let ml_reference_projection37 = self::ml_array::optional_at(&*ml_reference_array37.borrow(), self::ml_array::number_index(ml_reference_position37)); ml_reference_projection37 }).clone().unwrap_or_else(|| (-1f64))) == (69f64)) || ((({ let ml_reference_array38 = units.clone(); let ml_reference_position38 = at_7; let ml_reference_projection38 = self::ml_array::optional_at(&*ml_reference_array38.borrow(), self::ml_array::number_index(ml_reference_position38)); ml_reference_projection38 }).clone().unwrap_or_else(|| (-1f64))) == (101f64))) {
                                                                                        {
                                                                                            let at_8 = (at_7 + 1f64);
                                                                                            {
                                                                                                let at_10 = if (((({ let ml_reference_array39 = units.clone(); let ml_reference_position39 = at_8; let ml_reference_projection39 = self::ml_array::optional_at(&*ml_reference_array39.borrow(), self::ml_array::number_index(ml_reference_position39)); ml_reference_projection39 }).clone().unwrap_or_else(|| (-1f64))) == (43f64)) || ((({ let ml_reference_array40 = units.clone(); let ml_reference_position40 = at_8; let ml_reference_projection40 = self::ml_array::optional_at(&*ml_reference_array40.borrow(), self::ml_array::number_index(ml_reference_position40)); ml_reference_projection40 }).clone().unwrap_or_else(|| (-1f64))) == (45f64))) {
                                                                                                    {
                                                                                                        let at_9 = (at_8 + 1f64);
                                                                                                        at_9
                                                                                                    }
                                                                                                } else {
                                                                                                    at_8
                                                                                                };
                                                                                                {
                                                                                                    let exponent_start = at_10;
                                                                                                    {
                                                                                                        let at_11 = self::source_ml_source1_temporary_parse_java_script_float_prefix_loop49(units.clone(), at_10);
                                                                                                        if (at_11 == exponent_start) {
                                                                                                            mantissa_end
                                                                                                        } else {
                                                                                                            at_11
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        at_7
                                                                                    };
                                                                                    self::convert_java_script_string_number_units(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value41 = units.borrow().clone(); ml_reference_value41 }, start_2, at_12))))
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

pub fn execute_java_script_numeric_parsing(operation: self::ml_string::Text, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    {
        let arity = self::read_java_script_numeric_parsing_arity(operation.clone());
        {
            let input = ({
                let source_ml_source2_array_read_index597 = 0f64;
                { let ml_reference_array42 = values.clone(); let ml_reference_position42 = source_ml_source2_array_read_index597; let ml_reference_projection42 = self::ml_array::optional_at(&*ml_reference_array42.borrow(), self::ml_array::number_index(ml_reference_position42)); ml_reference_projection42 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            {
                let radix = ({
                    let source_ml_source2_array_read_index599 = 1f64;
                    { let ml_reference_array43 = values.clone(); let ml_reference_position43 = source_ml_source2_array_read_index599; let ml_reference_projection43 = self::ml_array::optional_at(&*ml_reference_array43.borrow(), self::ml_array::number_index(ml_reference_position43)); ml_reference_projection43 }
                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                {
                    let length = self::read_java_script_semantic_text_length(input.clone());
                    if ((((arity < (0f64)) || (({ let ml_reference_array44 = values.clone(); let ml_reference_projection44 = ml_reference_array44.borrow().len() as f64; ml_reference_projection44 }) != arity)) || (length < (0f64))) || ((arity == (2f64)) && match radix.clone() {
                        self::JavaScriptSemanticValue::Number(_) => {
                            false
                        }
                        _ => {
                            true
                        }
                    })) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 101, 102, 105, 120, 32, 112, 97, 114, 115, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 114, 97, 100, 105, 120]))
                    } else {
                        if (length > (4096f64)) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 101, 102, 105, 120, 32, 112, 97, 114, 115, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                        } else {
                            {
                                let units = self::construct_java_script_semantic_text_units(input.clone(), 4096f64);
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (arity == (2f64)) {
                                    self::parse_java_script_integer_prefix(units.clone(), match radix.clone() {
                                        self::JavaScriptSemanticValue::Number(value) => {
                                            value
                                        }
                                        _ => {
                                            f64::NAN
                                        }
                                    })
                                } else {
                                    self::parse_java_script_float_prefix(units.clone())
                                })))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_unicode_case_lower_ranges() -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![65f64, 90f64, 1f64, 32f64, 192f64, 214f64, 1f64, 32f64, 216f64, 222f64, 1f64, 32f64, 256f64, 302f64, 2f64, 1f64, 306f64, 310f64, 2f64, 1f64, 313f64, 327f64, 2f64, 1f64, 330f64, 374f64, 2f64, 1f64, 376f64, 376f64, 1f64, (-121f64), 377f64, 381f64, 2f64, 1f64, 385f64, 385f64, 1f64, 210f64, 386f64, 388f64, 2f64, 1f64, 390f64, 390f64, 1f64, 206f64, 391f64, 391f64, 1f64, 1f64, 393f64, 394f64, 1f64, 205f64, 395f64, 395f64, 1f64, 1f64, 398f64, 398f64, 1f64, 79f64, 399f64, 399f64, 1f64, 202f64, 400f64, 400f64, 1f64, 203f64, 401f64, 401f64, 1f64, 1f64, 403f64, 403f64, 1f64, 205f64, 404f64, 404f64, 1f64, 207f64, 406f64, 406f64, 1f64, 211f64, 407f64, 407f64, 1f64, 209f64, 408f64, 408f64, 1f64, 1f64, 412f64, 412f64, 1f64, 211f64, 413f64, 413f64, 1f64, 213f64, 415f64, 415f64, 1f64, 214f64, 416f64, 420f64, 2f64, 1f64, 422f64, 422f64, 1f64, 218f64, 423f64, 423f64, 1f64, 1f64, 425f64, 425f64, 1f64, 218f64, 428f64, 428f64, 1f64, 1f64, 430f64, 430f64, 1f64, 218f64, 431f64, 431f64, 1f64, 1f64, 433f64, 434f64, 1f64, 217f64, 435f64, 437f64, 2f64, 1f64, 439f64, 439f64, 1f64, 219f64, 440f64, 440f64, 1f64, 1f64, 444f64, 444f64, 1f64, 1f64, 452f64, 452f64, 1f64, 2f64, 453f64, 453f64, 1f64, 1f64, 455f64, 455f64, 1f64, 2f64, 456f64, 456f64, 1f64, 1f64, 458f64, 458f64, 1f64, 2f64, 459f64, 475f64, 2f64, 1f64, 478f64, 494f64, 2f64, 1f64, 497f64, 497f64, 1f64, 2f64, 498f64, 500f64, 2f64, 1f64, 502f64, 502f64, 1f64, (-97f64), 503f64, 503f64, 1f64, (-56f64), 504f64, 542f64, 2f64, 1f64, 544f64, 544f64, 1f64, (-130f64), 546f64, 562f64, 2f64, 1f64, 570f64, 570f64, 1f64, 10795f64, 571f64, 571f64, 1f64, 1f64, 573f64, 573f64, 1f64, (-163f64), 574f64, 574f64, 1f64, 10792f64, 577f64, 577f64, 1f64, 1f64, 579f64, 579f64, 1f64, (-195f64), 580f64, 580f64, 1f64, 69f64, 581f64, 581f64, 1f64, 71f64, 582f64, 590f64, 2f64, 1f64, 880f64, 882f64, 2f64, 1f64, 886f64, 886f64, 1f64, 1f64, 895f64, 895f64, 1f64, 116f64, 902f64, 902f64, 1f64, 38f64, 904f64, 906f64, 1f64, 37f64, 908f64, 908f64, 1f64, 64f64, 910f64, 911f64, 1f64, 63f64, 913f64, 929f64, 1f64, 32f64, 931f64, 939f64, 1f64, 32f64, 975f64, 975f64, 1f64, 8f64, 984f64, 1006f64, 2f64, 1f64, 1012f64, 1012f64, 1f64, (-60f64), 1015f64, 1015f64, 1f64, 1f64, 1017f64, 1017f64, 1f64, (-7f64), 1018f64, 1018f64, 1f64, 1f64, 1021f64, 1023f64, 1f64, (-130f64), 1024f64, 1039f64, 1f64, 80f64, 1040f64, 1071f64, 1f64, 32f64, 1120f64, 1152f64, 2f64, 1f64, 1162f64, 1214f64, 2f64, 1f64, 1216f64, 1216f64, 1f64, 15f64, 1217f64, 1229f64, 2f64, 1f64, 1232f64, 1326f64, 2f64, 1f64, 1329f64, 1366f64, 1f64, 48f64, 4256f64, 4293f64, 1f64, 7264f64, 4295f64, 4295f64, 1f64, 7264f64, 4301f64, 4301f64, 1f64, 7264f64, 5024f64, 5103f64, 1f64, 38864f64, 5104f64, 5109f64, 1f64, 8f64, 7305f64, 7305f64, 1f64, 1f64, 7312f64, 7354f64, 1f64, (-3008f64), 7357f64, 7359f64, 1f64, (-3008f64), 7680f64, 7828f64, 2f64, 1f64, 7838f64, 7838f64, 1f64, (-7615f64), 7840f64, 7934f64, 2f64, 1f64, 7944f64, 7951f64, 1f64, (-8f64), 7960f64, 7965f64, 1f64, (-8f64), 7976f64, 7983f64, 1f64, (-8f64), 7992f64, 7999f64, 1f64, (-8f64), 8008f64, 8013f64, 1f64, (-8f64), 8025f64, 8031f64, 2f64, (-8f64), 8040f64, 8047f64, 1f64, (-8f64), 8072f64, 8079f64, 1f64, (-8f64), 8088f64, 8095f64, 1f64, (-8f64), 8104f64, 8111f64, 1f64, (-8f64), 8120f64, 8121f64, 1f64, (-8f64), 8122f64, 8123f64, 1f64, (-74f64), 8124f64, 8124f64, 1f64, (-9f64), 8136f64, 8139f64, 1f64, (-86f64), 8140f64, 8140f64, 1f64, (-9f64), 8152f64, 8153f64, 1f64, (-8f64), 8154f64, 8155f64, 1f64, (-100f64), 8168f64, 8169f64, 1f64, (-8f64), 8170f64, 8171f64, 1f64, (-112f64), 8172f64, 8172f64, 1f64, (-7f64), 8184f64, 8185f64, 1f64, (-128f64), 8186f64, 8187f64, 1f64, (-126f64), 8188f64, 8188f64, 1f64, (-9f64), 8486f64, 8486f64, 1f64, (-7517f64), 8490f64, 8490f64, 1f64, (-8383f64), 8491f64, 8491f64, 1f64, (-8262f64), 8498f64, 8498f64, 1f64, 28f64, 8544f64, 8559f64, 1f64, 16f64, 8579f64, 8579f64, 1f64, 1f64, 9398f64, 9423f64, 1f64, 26f64, 11264f64, 11311f64, 1f64, 48f64, 11360f64, 11360f64, 1f64, 1f64, 11362f64, 11362f64, 1f64, (-10743f64), 11363f64, 11363f64, 1f64, (-3814f64), 11364f64, 11364f64, 1f64, (-10727f64), 11367f64, 11371f64, 2f64, 1f64, 11373f64, 11373f64, 1f64, (-10780f64), 11374f64, 11374f64, 1f64, (-10749f64), 11375f64, 11375f64, 1f64, (-10783f64), 11376f64, 11376f64, 1f64, (-10782f64), 11378f64, 11378f64, 1f64, 1f64, 11381f64, 11381f64, 1f64, 1f64, 11390f64, 11391f64, 1f64, (-10815f64), 11392f64, 11490f64, 2f64, 1f64, 11499f64, 11501f64, 2f64, 1f64, 11506f64, 11506f64, 1f64, 1f64, 42560f64, 42604f64, 2f64, 1f64, 42624f64, 42650f64, 2f64, 1f64, 42786f64, 42798f64, 2f64, 1f64, 42802f64, 42862f64, 2f64, 1f64, 42873f64, 42875f64, 2f64, 1f64, 42877f64, 42877f64, 1f64, (-35332f64), 42878f64, 42886f64, 2f64, 1f64, 42891f64, 42891f64, 1f64, 1f64, 42893f64, 42893f64, 1f64, (-42280f64), 42896f64, 42898f64, 2f64, 1f64, 42902f64, 42920f64, 2f64, 1f64, 42922f64, 42922f64, 1f64, (-42308f64), 42923f64, 42923f64, 1f64, (-42319f64), 42924f64, 42924f64, 1f64, (-42315f64), 42925f64, 42925f64, 1f64, (-42305f64), 42926f64, 42926f64, 1f64, (-42308f64), 42928f64, 42928f64, 1f64, (-42258f64), 42929f64, 42929f64, 1f64, (-42282f64), 42930f64, 42930f64, 1f64, (-42261f64), 42931f64, 42931f64, 1f64, 928f64, 42932f64, 42946f64, 2f64, 1f64, 42948f64, 42948f64, 1f64, (-48f64), 42949f64, 42949f64, 1f64, (-42307f64), 42950f64, 42950f64, 1f64, (-35384f64), 42951f64, 42953f64, 2f64, 1f64, 42955f64, 42955f64, 1f64, (-42343f64), 42956f64, 42956f64, 1f64, 1f64, 42960f64, 42960f64, 1f64, 1f64, 42966f64, 42970f64, 2f64, 1f64, 42972f64, 42972f64, 1f64, (-42561f64), 42997f64, 42997f64, 1f64, 1f64, 65313f64, 65338f64, 1f64, 32f64, 66560f64, 66599f64, 1f64, 40f64, 66736f64, 66771f64, 1f64, 40f64, 66928f64, 66938f64, 1f64, 39f64, 66940f64, 66954f64, 1f64, 39f64, 66956f64, 66962f64, 1f64, 39f64, 66964f64, 66965f64, 1f64, 39f64, 68736f64, 68786f64, 1f64, 64f64, 68944f64, 68965f64, 1f64, 32f64, 71840f64, 71871f64, 1f64, 32f64, 93760f64, 93791f64, 1f64, 32f64, 125184f64, 125217f64, 1f64, 34f64]))
}

pub fn read_unicode_case_lower_indexes() -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![304f64, 0f64, 2f64]))
}

pub fn read_unicode_case_lower_values() -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![105f64, 775f64]))
}

pub fn read_unicode_case_upper_ranges() -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![97f64, 122f64, 1f64, (-32f64), 181f64, 181f64, 1f64, 743f64, 224f64, 246f64, 1f64, (-32f64), 248f64, 254f64, 1f64, (-32f64), 255f64, 255f64, 1f64, 121f64, 257f64, 303f64, 2f64, (-1f64), 305f64, 305f64, 1f64, (-232f64), 307f64, 311f64, 2f64, (-1f64), 314f64, 328f64, 2f64, (-1f64), 331f64, 375f64, 2f64, (-1f64), 378f64, 382f64, 2f64, (-1f64), 383f64, 383f64, 1f64, (-300f64), 384f64, 384f64, 1f64, 195f64, 387f64, 389f64, 2f64, (-1f64), 392f64, 392f64, 1f64, (-1f64), 396f64, 396f64, 1f64, (-1f64), 402f64, 402f64, 1f64, (-1f64), 405f64, 405f64, 1f64, 97f64, 409f64, 409f64, 1f64, (-1f64), 410f64, 410f64, 1f64, 163f64, 411f64, 411f64, 1f64, 42561f64, 414f64, 414f64, 1f64, 130f64, 417f64, 421f64, 2f64, (-1f64), 424f64, 424f64, 1f64, (-1f64), 429f64, 429f64, 1f64, (-1f64), 432f64, 432f64, 1f64, (-1f64), 436f64, 438f64, 2f64, (-1f64), 441f64, 441f64, 1f64, (-1f64), 445f64, 445f64, 1f64, (-1f64), 447f64, 447f64, 1f64, 56f64, 453f64, 453f64, 1f64, (-1f64), 454f64, 454f64, 1f64, (-2f64), 456f64, 456f64, 1f64, (-1f64), 457f64, 457f64, 1f64, (-2f64), 459f64, 459f64, 1f64, (-1f64), 460f64, 460f64, 1f64, (-2f64), 462f64, 476f64, 2f64, (-1f64), 477f64, 477f64, 1f64, (-79f64), 479f64, 495f64, 2f64, (-1f64), 498f64, 498f64, 1f64, (-1f64), 499f64, 499f64, 1f64, (-2f64), 501f64, 501f64, 1f64, (-1f64), 505f64, 543f64, 2f64, (-1f64), 547f64, 563f64, 2f64, (-1f64), 572f64, 572f64, 1f64, (-1f64), 575f64, 576f64, 1f64, 10815f64, 578f64, 578f64, 1f64, (-1f64), 583f64, 591f64, 2f64, (-1f64), 592f64, 592f64, 1f64, 10783f64, 593f64, 593f64, 1f64, 10780f64, 594f64, 594f64, 1f64, 10782f64, 595f64, 595f64, 1f64, (-210f64), 596f64, 596f64, 1f64, (-206f64), 598f64, 599f64, 1f64, (-205f64), 601f64, 601f64, 1f64, (-202f64), 603f64, 603f64, 1f64, (-203f64), 604f64, 604f64, 1f64, 42319f64, 608f64, 608f64, 1f64, (-205f64), 609f64, 609f64, 1f64, 42315f64, 611f64, 611f64, 1f64, (-207f64), 612f64, 612f64, 1f64, 42343f64, 613f64, 613f64, 1f64, 42280f64, 614f64, 614f64, 1f64, 42308f64, 616f64, 616f64, 1f64, (-209f64), 617f64, 617f64, 1f64, (-211f64), 618f64, 618f64, 1f64, 42308f64, 619f64, 619f64, 1f64, 10743f64, 620f64, 620f64, 1f64, 42305f64, 623f64, 623f64, 1f64, (-211f64), 625f64, 625f64, 1f64, 10749f64, 626f64, 626f64, 1f64, (-213f64), 629f64, 629f64, 1f64, (-214f64), 637f64, 637f64, 1f64, 10727f64, 640f64, 640f64, 1f64, (-218f64), 642f64, 642f64, 1f64, 42307f64, 643f64, 643f64, 1f64, (-218f64), 647f64, 647f64, 1f64, 42282f64, 648f64, 648f64, 1f64, (-218f64), 649f64, 649f64, 1f64, (-69f64), 650f64, 651f64, 1f64, (-217f64), 652f64, 652f64, 1f64, (-71f64), 658f64, 658f64, 1f64, (-219f64), 669f64, 669f64, 1f64, 42261f64, 670f64, 670f64, 1f64, 42258f64, 837f64, 837f64, 1f64, 84f64, 881f64, 883f64, 2f64, (-1f64), 887f64, 887f64, 1f64, (-1f64), 891f64, 893f64, 1f64, 130f64, 940f64, 940f64, 1f64, (-38f64), 941f64, 943f64, 1f64, (-37f64), 945f64, 961f64, 1f64, (-32f64), 962f64, 962f64, 1f64, (-31f64), 963f64, 971f64, 1f64, (-32f64), 972f64, 972f64, 1f64, (-64f64), 973f64, 974f64, 1f64, (-63f64), 976f64, 976f64, 1f64, (-62f64), 977f64, 977f64, 1f64, (-57f64), 981f64, 981f64, 1f64, (-47f64), 982f64, 982f64, 1f64, (-54f64), 983f64, 983f64, 1f64, (-8f64), 985f64, 1007f64, 2f64, (-1f64), 1008f64, 1008f64, 1f64, (-86f64), 1009f64, 1009f64, 1f64, (-80f64), 1010f64, 1010f64, 1f64, 7f64, 1011f64, 1011f64, 1f64, (-116f64), 1013f64, 1013f64, 1f64, (-96f64), 1016f64, 1016f64, 1f64, (-1f64), 1019f64, 1019f64, 1f64, (-1f64), 1072f64, 1103f64, 1f64, (-32f64), 1104f64, 1119f64, 1f64, (-80f64), 1121f64, 1153f64, 2f64, (-1f64), 1163f64, 1215f64, 2f64, (-1f64), 1218f64, 1230f64, 2f64, (-1f64), 1231f64, 1231f64, 1f64, (-15f64), 1233f64, 1327f64, 2f64, (-1f64), 1377f64, 1414f64, 1f64, (-48f64), 4304f64, 4346f64, 1f64, 3008f64, 4349f64, 4351f64, 1f64, 3008f64, 5112f64, 5117f64, 1f64, (-8f64), 7296f64, 7296f64, 1f64, (-6254f64), 7297f64, 7297f64, 1f64, (-6253f64), 7298f64, 7298f64, 1f64, (-6244f64), 7299f64, 7300f64, 1f64, (-6242f64), 7301f64, 7301f64, 1f64, (-6243f64), 7302f64, 7302f64, 1f64, (-6236f64), 7303f64, 7303f64, 1f64, (-6181f64), 7304f64, 7304f64, 1f64, 35266f64, 7306f64, 7306f64, 1f64, (-1f64), 7545f64, 7545f64, 1f64, 35332f64, 7549f64, 7549f64, 1f64, 3814f64, 7566f64, 7566f64, 1f64, 35384f64, 7681f64, 7829f64, 2f64, (-1f64), 7835f64, 7835f64, 1f64, (-59f64), 7841f64, 7935f64, 2f64, (-1f64), 7936f64, 7943f64, 1f64, 8f64, 7952f64, 7957f64, 1f64, 8f64, 7968f64, 7975f64, 1f64, 8f64, 7984f64, 7991f64, 1f64, 8f64, 8000f64, 8005f64, 1f64, 8f64, 8017f64, 8023f64, 2f64, 8f64, 8032f64, 8039f64, 1f64, 8f64, 8048f64, 8049f64, 1f64, 74f64, 8050f64, 8053f64, 1f64, 86f64, 8054f64, 8055f64, 1f64, 100f64, 8056f64, 8057f64, 1f64, 128f64, 8058f64, 8059f64, 1f64, 112f64, 8060f64, 8061f64, 1f64, 126f64, 8112f64, 8113f64, 1f64, 8f64, 8126f64, 8126f64, 1f64, (-7205f64), 8144f64, 8145f64, 1f64, 8f64, 8160f64, 8161f64, 1f64, 8f64, 8165f64, 8165f64, 1f64, 7f64, 8526f64, 8526f64, 1f64, (-28f64), 8560f64, 8575f64, 1f64, (-16f64), 8580f64, 8580f64, 1f64, (-1f64), 9424f64, 9449f64, 1f64, (-26f64), 11312f64, 11359f64, 1f64, (-48f64), 11361f64, 11361f64, 1f64, (-1f64), 11365f64, 11365f64, 1f64, (-10795f64), 11366f64, 11366f64, 1f64, (-10792f64), 11368f64, 11372f64, 2f64, (-1f64), 11379f64, 11379f64, 1f64, (-1f64), 11382f64, 11382f64, 1f64, (-1f64), 11393f64, 11491f64, 2f64, (-1f64), 11500f64, 11502f64, 2f64, (-1f64), 11507f64, 11507f64, 1f64, (-1f64), 11520f64, 11557f64, 1f64, (-7264f64), 11559f64, 11559f64, 1f64, (-7264f64), 11565f64, 11565f64, 1f64, (-7264f64), 42561f64, 42605f64, 2f64, (-1f64), 42625f64, 42651f64, 2f64, (-1f64), 42787f64, 42799f64, 2f64, (-1f64), 42803f64, 42863f64, 2f64, (-1f64), 42874f64, 42876f64, 2f64, (-1f64), 42879f64, 42887f64, 2f64, (-1f64), 42892f64, 42892f64, 1f64, (-1f64), 42897f64, 42899f64, 2f64, (-1f64), 42900f64, 42900f64, 1f64, 48f64, 42903f64, 42921f64, 2f64, (-1f64), 42933f64, 42947f64, 2f64, (-1f64), 42952f64, 42954f64, 2f64, (-1f64), 42957f64, 42957f64, 1f64, (-1f64), 42961f64, 42961f64, 1f64, (-1f64), 42967f64, 42971f64, 2f64, (-1f64), 42998f64, 42998f64, 1f64, (-1f64), 43859f64, 43859f64, 1f64, (-928f64), 43888f64, 43967f64, 1f64, (-38864f64), 65345f64, 65370f64, 1f64, (-32f64), 66600f64, 66639f64, 1f64, (-40f64), 66776f64, 66811f64, 1f64, (-40f64), 66967f64, 66977f64, 1f64, (-39f64), 66979f64, 66993f64, 1f64, (-39f64), 66995f64, 67001f64, 1f64, (-39f64), 67003f64, 67004f64, 1f64, (-39f64), 68800f64, 68850f64, 1f64, (-64f64), 68976f64, 68997f64, 1f64, (-32f64), 71872f64, 71903f64, 1f64, (-32f64), 93792f64, 93823f64, 1f64, (-32f64), 125218f64, 125251f64, 1f64, (-34f64)]))
}

pub fn read_unicode_case_upper_indexes() -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![223f64, 0f64, 2f64, 329f64, 2f64, 2f64, 496f64, 4f64, 2f64, 912f64, 6f64, 3f64, 944f64, 9f64, 3f64, 1415f64, 12f64, 2f64, 7830f64, 14f64, 2f64, 7831f64, 16f64, 2f64, 7832f64, 18f64, 2f64, 7833f64, 20f64, 2f64, 7834f64, 22f64, 2f64, 8016f64, 24f64, 2f64, 8018f64, 26f64, 3f64, 8020f64, 29f64, 3f64, 8022f64, 32f64, 3f64, 8064f64, 35f64, 2f64, 8065f64, 37f64, 2f64, 8066f64, 39f64, 2f64, 8067f64, 41f64, 2f64, 8068f64, 43f64, 2f64, 8069f64, 45f64, 2f64, 8070f64, 47f64, 2f64, 8071f64, 49f64, 2f64, 8072f64, 51f64, 2f64, 8073f64, 53f64, 2f64, 8074f64, 55f64, 2f64, 8075f64, 57f64, 2f64, 8076f64, 59f64, 2f64, 8077f64, 61f64, 2f64, 8078f64, 63f64, 2f64, 8079f64, 65f64, 2f64, 8080f64, 67f64, 2f64, 8081f64, 69f64, 2f64, 8082f64, 71f64, 2f64, 8083f64, 73f64, 2f64, 8084f64, 75f64, 2f64, 8085f64, 77f64, 2f64, 8086f64, 79f64, 2f64, 8087f64, 81f64, 2f64, 8088f64, 83f64, 2f64, 8089f64, 85f64, 2f64, 8090f64, 87f64, 2f64, 8091f64, 89f64, 2f64, 8092f64, 91f64, 2f64, 8093f64, 93f64, 2f64, 8094f64, 95f64, 2f64, 8095f64, 97f64, 2f64, 8096f64, 99f64, 2f64, 8097f64, 101f64, 2f64, 8098f64, 103f64, 2f64, 8099f64, 105f64, 2f64, 8100f64, 107f64, 2f64, 8101f64, 109f64, 2f64, 8102f64, 111f64, 2f64, 8103f64, 113f64, 2f64, 8104f64, 115f64, 2f64, 8105f64, 117f64, 2f64, 8106f64, 119f64, 2f64, 8107f64, 121f64, 2f64, 8108f64, 123f64, 2f64, 8109f64, 125f64, 2f64, 8110f64, 127f64, 2f64, 8111f64, 129f64, 2f64, 8114f64, 131f64, 2f64, 8115f64, 133f64, 2f64, 8116f64, 135f64, 2f64, 8118f64, 137f64, 2f64, 8119f64, 139f64, 3f64, 8124f64, 142f64, 2f64, 8130f64, 144f64, 2f64, 8131f64, 146f64, 2f64, 8132f64, 148f64, 2f64, 8134f64, 150f64, 2f64, 8135f64, 152f64, 3f64, 8140f64, 155f64, 2f64, 8146f64, 157f64, 3f64, 8147f64, 160f64, 3f64, 8150f64, 163f64, 2f64, 8151f64, 165f64, 3f64, 8162f64, 168f64, 3f64, 8163f64, 171f64, 3f64, 8164f64, 174f64, 2f64, 8166f64, 176f64, 2f64, 8167f64, 178f64, 3f64, 8178f64, 181f64, 2f64, 8179f64, 183f64, 2f64, 8180f64, 185f64, 2f64, 8182f64, 187f64, 2f64, 8183f64, 189f64, 3f64, 8188f64, 192f64, 2f64, 64256f64, 194f64, 2f64, 64257f64, 196f64, 2f64, 64258f64, 198f64, 2f64, 64259f64, 200f64, 3f64, 64260f64, 203f64, 3f64, 64261f64, 206f64, 2f64, 64262f64, 208f64, 2f64, 64275f64, 210f64, 2f64, 64276f64, 212f64, 2f64, 64277f64, 214f64, 2f64, 64278f64, 216f64, 2f64, 64279f64, 218f64, 2f64]))
}

pub fn read_unicode_case_upper_values() -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![83f64, 83f64, 700f64, 78f64, 74f64, 780f64, 921f64, 776f64, 769f64, 933f64, 776f64, 769f64, 1333f64, 1362f64, 72f64, 817f64, 84f64, 776f64, 87f64, 778f64, 89f64, 778f64, 65f64, 702f64, 933f64, 787f64, 933f64, 787f64, 768f64, 933f64, 787f64, 769f64, 933f64, 787f64, 834f64, 7944f64, 921f64, 7945f64, 921f64, 7946f64, 921f64, 7947f64, 921f64, 7948f64, 921f64, 7949f64, 921f64, 7950f64, 921f64, 7951f64, 921f64, 7944f64, 921f64, 7945f64, 921f64, 7946f64, 921f64, 7947f64, 921f64, 7948f64, 921f64, 7949f64, 921f64, 7950f64, 921f64, 7951f64, 921f64, 7976f64, 921f64, 7977f64, 921f64, 7978f64, 921f64, 7979f64, 921f64, 7980f64, 921f64, 7981f64, 921f64, 7982f64, 921f64, 7983f64, 921f64, 7976f64, 921f64, 7977f64, 921f64, 7978f64, 921f64, 7979f64, 921f64, 7980f64, 921f64, 7981f64, 921f64, 7982f64, 921f64, 7983f64, 921f64, 8040f64, 921f64, 8041f64, 921f64, 8042f64, 921f64, 8043f64, 921f64, 8044f64, 921f64, 8045f64, 921f64, 8046f64, 921f64, 8047f64, 921f64, 8040f64, 921f64, 8041f64, 921f64, 8042f64, 921f64, 8043f64, 921f64, 8044f64, 921f64, 8045f64, 921f64, 8046f64, 921f64, 8047f64, 921f64, 8122f64, 921f64, 913f64, 921f64, 902f64, 921f64, 913f64, 834f64, 913f64, 834f64, 921f64, 913f64, 921f64, 8138f64, 921f64, 919f64, 921f64, 905f64, 921f64, 919f64, 834f64, 919f64, 834f64, 921f64, 919f64, 921f64, 921f64, 776f64, 768f64, 921f64, 776f64, 769f64, 921f64, 834f64, 921f64, 776f64, 834f64, 933f64, 776f64, 768f64, 933f64, 776f64, 769f64, 929f64, 787f64, 933f64, 834f64, 933f64, 776f64, 834f64, 8186f64, 921f64, 937f64, 921f64, 911f64, 921f64, 937f64, 834f64, 937f64, 834f64, 921f64, 937f64, 921f64, 70f64, 70f64, 70f64, 73f64, 70f64, 76f64, 70f64, 70f64, 73f64, 70f64, 70f64, 76f64, 83f64, 84f64, 83f64, 84f64, 1348f64, 1350f64, 1348f64, 1333f64, 1348f64, 1339f64, 1358f64, 1350f64, 1348f64, 1341f64]))
}

pub fn read_unicode_case_cased_ranges() -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![65f64, 90f64, 97f64, 122f64, 170f64, 170f64, 181f64, 181f64, 186f64, 186f64, 192f64, 214f64, 216f64, 246f64, 248f64, 442f64, 444f64, 447f64, 452f64, 659f64, 661f64, 687f64, 688f64, 696f64, 704f64, 705f64, 736f64, 740f64, 837f64, 837f64, 880f64, 883f64, 886f64, 887f64, 890f64, 890f64, 891f64, 893f64, 895f64, 895f64, 902f64, 902f64, 904f64, 906f64, 908f64, 908f64, 910f64, 929f64, 931f64, 1013f64, 1015f64, 1153f64, 1162f64, 1327f64, 1329f64, 1366f64, 1376f64, 1416f64, 4256f64, 4293f64, 4295f64, 4295f64, 4301f64, 4301f64, 4304f64, 4346f64, 4348f64, 4348f64, 4349f64, 4351f64, 5024f64, 5109f64, 5112f64, 5117f64, 7296f64, 7306f64, 7312f64, 7354f64, 7357f64, 7359f64, 7424f64, 7467f64, 7468f64, 7530f64, 7531f64, 7543f64, 7544f64, 7544f64, 7545f64, 7578f64, 7579f64, 7615f64, 7680f64, 7957f64, 7960f64, 7965f64, 7968f64, 8005f64, 8008f64, 8013f64, 8016f64, 8023f64, 8025f64, 8025f64, 8027f64, 8027f64, 8029f64, 8029f64, 8031f64, 8061f64, 8064f64, 8116f64, 8118f64, 8124f64, 8126f64, 8126f64, 8130f64, 8132f64, 8134f64, 8140f64, 8144f64, 8147f64, 8150f64, 8155f64, 8160f64, 8172f64, 8178f64, 8180f64, 8182f64, 8188f64, 8305f64, 8305f64, 8319f64, 8319f64, 8336f64, 8348f64, 8450f64, 8450f64, 8455f64, 8455f64, 8458f64, 8467f64, 8469f64, 8469f64, 8473f64, 8477f64, 8484f64, 8484f64, 8486f64, 8486f64, 8488f64, 8488f64, 8490f64, 8493f64, 8495f64, 8500f64, 8505f64, 8505f64, 8508f64, 8511f64, 8517f64, 8521f64, 8526f64, 8526f64, 8544f64, 8575f64, 8579f64, 8580f64, 9398f64, 9449f64, 11264f64, 11387f64, 11388f64, 11389f64, 11390f64, 11492f64, 11499f64, 11502f64, 11506f64, 11507f64, 11520f64, 11557f64, 11559f64, 11559f64, 11565f64, 11565f64, 42560f64, 42605f64, 42624f64, 42651f64, 42652f64, 42653f64, 42786f64, 42863f64, 42864f64, 42864f64, 42865f64, 42887f64, 42891f64, 42894f64, 42896f64, 42957f64, 42960f64, 42961f64, 42963f64, 42963f64, 42965f64, 42972f64, 42994f64, 42996f64, 42997f64, 42998f64, 43000f64, 43001f64, 43002f64, 43002f64, 43824f64, 43866f64, 43868f64, 43871f64, 43872f64, 43880f64, 43881f64, 43881f64, 43888f64, 43967f64, 64256f64, 64262f64, 64275f64, 64279f64, 65313f64, 65338f64, 65345f64, 65370f64, 66560f64, 66639f64, 66736f64, 66771f64, 66776f64, 66811f64, 66928f64, 66938f64, 66940f64, 66954f64, 66956f64, 66962f64, 66964f64, 66965f64, 66967f64, 66977f64, 66979f64, 66993f64, 66995f64, 67001f64, 67003f64, 67004f64, 67456f64, 67456f64, 67459f64, 67461f64, 67463f64, 67504f64, 67506f64, 67514f64, 68736f64, 68786f64, 68800f64, 68850f64, 68944f64, 68965f64, 68976f64, 68997f64, 71840f64, 71903f64, 93760f64, 93823f64, 119808f64, 119892f64, 119894f64, 119964f64, 119966f64, 119967f64, 119970f64, 119970f64, 119973f64, 119974f64, 119977f64, 119980f64, 119982f64, 119993f64, 119995f64, 119995f64, 119997f64, 120003f64, 120005f64, 120069f64, 120071f64, 120074f64, 120077f64, 120084f64, 120086f64, 120092f64, 120094f64, 120121f64, 120123f64, 120126f64, 120128f64, 120132f64, 120134f64, 120134f64, 120138f64, 120144f64, 120146f64, 120485f64, 120488f64, 120512f64, 120514f64, 120538f64, 120540f64, 120570f64, 120572f64, 120596f64, 120598f64, 120628f64, 120630f64, 120654f64, 120656f64, 120686f64, 120688f64, 120712f64, 120714f64, 120744f64, 120746f64, 120770f64, 120772f64, 120779f64, 122624f64, 122633f64, 122635f64, 122654f64, 122661f64, 122666f64, 122928f64, 122989f64, 125184f64, 125251f64, 127280f64, 127305f64, 127312f64, 127337f64, 127344f64, 127369f64]))
}

pub fn read_unicode_case_ignorable_ranges() -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![39f64, 39f64, 46f64, 46f64, 58f64, 58f64, 94f64, 94f64, 96f64, 96f64, 168f64, 168f64, 173f64, 173f64, 175f64, 175f64, 180f64, 180f64, 183f64, 183f64, 184f64, 184f64, 688f64, 705f64, 706f64, 709f64, 710f64, 721f64, 722f64, 735f64, 736f64, 740f64, 741f64, 747f64, 748f64, 748f64, 749f64, 749f64, 750f64, 750f64, 751f64, 767f64, 768f64, 879f64, 884f64, 884f64, 885f64, 885f64, 890f64, 890f64, 900f64, 901f64, 903f64, 903f64, 1155f64, 1159f64, 1160f64, 1161f64, 1369f64, 1369f64, 1375f64, 1375f64, 1425f64, 1469f64, 1471f64, 1471f64, 1473f64, 1474f64, 1476f64, 1477f64, 1479f64, 1479f64, 1524f64, 1524f64, 1536f64, 1541f64, 1552f64, 1562f64, 1564f64, 1564f64, 1600f64, 1600f64, 1611f64, 1631f64, 1648f64, 1648f64, 1750f64, 1756f64, 1757f64, 1757f64, 1759f64, 1764f64, 1765f64, 1766f64, 1767f64, 1768f64, 1770f64, 1773f64, 1807f64, 1807f64, 1809f64, 1809f64, 1840f64, 1866f64, 1958f64, 1968f64, 2027f64, 2035f64, 2036f64, 2037f64, 2042f64, 2042f64, 2045f64, 2045f64, 2070f64, 2073f64, 2074f64, 2074f64, 2075f64, 2083f64, 2084f64, 2084f64, 2085f64, 2087f64, 2088f64, 2088f64, 2089f64, 2093f64, 2137f64, 2139f64, 2184f64, 2184f64, 2192f64, 2193f64, 2199f64, 2207f64, 2249f64, 2249f64, 2250f64, 2273f64, 2274f64, 2274f64, 2275f64, 2306f64, 2362f64, 2362f64, 2364f64, 2364f64, 2369f64, 2376f64, 2381f64, 2381f64, 2385f64, 2391f64, 2402f64, 2403f64, 2417f64, 2417f64, 2433f64, 2433f64, 2492f64, 2492f64, 2497f64, 2500f64, 2509f64, 2509f64, 2530f64, 2531f64, 2558f64, 2558f64, 2561f64, 2562f64, 2620f64, 2620f64, 2625f64, 2626f64, 2631f64, 2632f64, 2635f64, 2637f64, 2641f64, 2641f64, 2672f64, 2673f64, 2677f64, 2677f64, 2689f64, 2690f64, 2748f64, 2748f64, 2753f64, 2757f64, 2759f64, 2760f64, 2765f64, 2765f64, 2786f64, 2787f64, 2810f64, 2815f64, 2817f64, 2817f64, 2876f64, 2876f64, 2879f64, 2879f64, 2881f64, 2884f64, 2893f64, 2893f64, 2901f64, 2902f64, 2914f64, 2915f64, 2946f64, 2946f64, 3008f64, 3008f64, 3021f64, 3021f64, 3072f64, 3072f64, 3076f64, 3076f64, 3132f64, 3132f64, 3134f64, 3136f64, 3142f64, 3144f64, 3146f64, 3149f64, 3157f64, 3158f64, 3170f64, 3171f64, 3201f64, 3201f64, 3260f64, 3260f64, 3263f64, 3263f64, 3270f64, 3270f64, 3276f64, 3277f64, 3298f64, 3299f64, 3328f64, 3329f64, 3387f64, 3388f64, 3393f64, 3396f64, 3405f64, 3405f64, 3426f64, 3427f64, 3457f64, 3457f64, 3530f64, 3530f64, 3538f64, 3540f64, 3542f64, 3542f64, 3633f64, 3633f64, 3636f64, 3642f64, 3654f64, 3654f64, 3655f64, 3662f64, 3761f64, 3761f64, 3764f64, 3772f64, 3782f64, 3782f64, 3784f64, 3790f64, 3864f64, 3865f64, 3893f64, 3893f64, 3895f64, 3895f64, 3897f64, 3897f64, 3953f64, 3966f64, 3968f64, 3972f64, 3974f64, 3975f64, 3981f64, 3991f64, 3993f64, 4028f64, 4038f64, 4038f64, 4141f64, 4144f64, 4146f64, 4151f64, 4153f64, 4154f64, 4157f64, 4158f64, 4184f64, 4185f64, 4190f64, 4192f64, 4209f64, 4212f64, 4226f64, 4226f64, 4229f64, 4230f64, 4237f64, 4237f64, 4253f64, 4253f64, 4348f64, 4348f64, 4957f64, 4959f64, 5906f64, 5908f64, 5938f64, 5939f64, 5970f64, 5971f64, 6002f64, 6003f64, 6068f64, 6069f64, 6071f64, 6077f64, 6086f64, 6086f64, 6089f64, 6099f64, 6103f64, 6103f64, 6109f64, 6109f64, 6155f64, 6157f64, 6158f64, 6158f64, 6159f64, 6159f64, 6211f64, 6211f64, 6277f64, 6278f64, 6313f64, 6313f64, 6432f64, 6434f64, 6439f64, 6440f64, 6450f64, 6450f64, 6457f64, 6459f64, 6679f64, 6680f64, 6683f64, 6683f64, 6742f64, 6742f64, 6744f64, 6750f64, 6752f64, 6752f64, 6754f64, 6754f64, 6757f64, 6764f64, 6771f64, 6780f64, 6783f64, 6783f64, 6823f64, 6823f64, 6832f64, 6845f64, 6846f64, 6846f64, 6847f64, 6862f64, 6912f64, 6915f64, 6964f64, 6964f64, 6966f64, 6970f64, 6972f64, 6972f64, 6978f64, 6978f64, 7019f64, 7027f64, 7040f64, 7041f64, 7074f64, 7077f64, 7080f64, 7081f64, 7083f64, 7085f64, 7142f64, 7142f64, 7144f64, 7145f64, 7149f64, 7149f64, 7151f64, 7153f64, 7212f64, 7219f64, 7222f64, 7223f64, 7288f64, 7293f64, 7376f64, 7378f64, 7380f64, 7392f64, 7394f64, 7400f64, 7405f64, 7405f64, 7412f64, 7412f64, 7416f64, 7417f64, 7468f64, 7530f64, 7544f64, 7544f64, 7579f64, 7615f64, 7616f64, 7679f64, 8125f64, 8125f64, 8127f64, 8129f64, 8141f64, 8143f64, 8157f64, 8159f64, 8173f64, 8175f64, 8189f64, 8190f64, 8203f64, 8207f64, 8216f64, 8216f64, 8217f64, 8217f64, 8228f64, 8228f64, 8231f64, 8231f64, 8234f64, 8238f64, 8288f64, 8292f64, 8294f64, 8303f64, 8305f64, 8305f64, 8319f64, 8319f64, 8336f64, 8348f64, 8400f64, 8412f64, 8413f64, 8416f64, 8417f64, 8417f64, 8418f64, 8420f64, 8421f64, 8432f64, 11388f64, 11389f64, 11503f64, 11505f64, 11631f64, 11631f64, 11647f64, 11647f64, 11744f64, 11775f64, 11823f64, 11823f64, 12293f64, 12293f64, 12330f64, 12333f64, 12337f64, 12341f64, 12347f64, 12347f64, 12441f64, 12442f64, 12443f64, 12444f64, 12445f64, 12446f64, 12540f64, 12542f64, 40981f64, 40981f64, 42232f64, 42237f64, 42508f64, 42508f64, 42607f64, 42607f64, 42608f64, 42610f64, 42612f64, 42621f64, 42623f64, 42623f64, 42652f64, 42653f64, 42654f64, 42655f64, 42736f64, 42737f64, 42752f64, 42774f64, 42775f64, 42783f64, 42784f64, 42785f64, 42864f64, 42864f64, 42888f64, 42888f64, 42889f64, 42890f64, 42994f64, 42996f64, 43000f64, 43001f64, 43010f64, 43010f64, 43014f64, 43014f64, 43019f64, 43019f64, 43045f64, 43046f64, 43052f64, 43052f64, 43204f64, 43205f64, 43232f64, 43249f64, 43263f64, 43263f64, 43302f64, 43309f64, 43335f64, 43345f64, 43392f64, 43394f64, 43443f64, 43443f64, 43446f64, 43449f64, 43452f64, 43453f64, 43471f64, 43471f64, 43493f64, 43493f64, 43494f64, 43494f64, 43561f64, 43566f64, 43569f64, 43570f64, 43573f64, 43574f64, 43587f64, 43587f64, 43596f64, 43596f64, 43632f64, 43632f64, 43644f64, 43644f64, 43696f64, 43696f64, 43698f64, 43700f64, 43703f64, 43704f64, 43710f64, 43711f64, 43713f64, 43713f64, 43741f64, 43741f64, 43756f64, 43757f64, 43763f64, 43764f64, 43766f64, 43766f64, 43867f64, 43867f64, 43868f64, 43871f64, 43881f64, 43881f64, 43882f64, 43883f64, 44005f64, 44005f64, 44008f64, 44008f64, 44013f64, 44013f64, 64286f64, 64286f64, 64434f64, 64450f64, 65024f64, 65039f64, 65043f64, 65043f64, 65056f64, 65071f64, 65106f64, 65106f64, 65109f64, 65109f64, 65279f64, 65279f64, 65287f64, 65287f64, 65294f64, 65294f64, 65306f64, 65306f64, 65342f64, 65342f64, 65344f64, 65344f64, 65392f64, 65392f64, 65438f64, 65439f64, 65507f64, 65507f64, 65529f64, 65531f64, 66045f64, 66045f64, 66272f64, 66272f64, 66422f64, 66426f64, 67456f64, 67461f64, 67463f64, 67504f64, 67506f64, 67514f64, 68097f64, 68099f64, 68101f64, 68102f64, 68108f64, 68111f64, 68152f64, 68154f64, 68159f64, 68159f64, 68325f64, 68326f64, 68900f64, 68903f64, 68942f64, 68942f64, 68969f64, 68973f64, 68975f64, 68975f64, 69291f64, 69292f64, 69372f64, 69375f64, 69446f64, 69456f64, 69506f64, 69509f64, 69633f64, 69633f64, 69688f64, 69702f64, 69744f64, 69744f64, 69747f64, 69748f64, 69759f64, 69761f64, 69811f64, 69814f64, 69817f64, 69818f64, 69821f64, 69821f64, 69826f64, 69826f64, 69837f64, 69837f64, 69888f64, 69890f64, 69927f64, 69931f64, 69933f64, 69940f64, 70003f64, 70003f64, 70016f64, 70017f64, 70070f64, 70078f64, 70089f64, 70092f64, 70095f64, 70095f64, 70191f64, 70193f64, 70196f64, 70196f64, 70198f64, 70199f64, 70206f64, 70206f64, 70209f64, 70209f64, 70367f64, 70367f64, 70371f64, 70378f64, 70400f64, 70401f64, 70459f64, 70460f64, 70464f64, 70464f64, 70502f64, 70508f64, 70512f64, 70516f64, 70587f64, 70592f64, 70606f64, 70606f64, 70608f64, 70608f64, 70610f64, 70610f64, 70625f64, 70626f64, 70712f64, 70719f64, 70722f64, 70724f64, 70726f64, 70726f64, 70750f64, 70750f64, 70835f64, 70840f64, 70842f64, 70842f64, 70847f64, 70848f64, 70850f64, 70851f64, 71090f64, 71093f64, 71100f64, 71101f64, 71103f64, 71104f64, 71132f64, 71133f64, 71219f64, 71226f64, 71229f64, 71229f64, 71231f64, 71232f64, 71339f64, 71339f64, 71341f64, 71341f64, 71344f64, 71349f64, 71351f64, 71351f64, 71453f64, 71453f64, 71455f64, 71455f64, 71458f64, 71461f64, 71463f64, 71467f64, 71727f64, 71735f64, 71737f64, 71738f64, 71995f64, 71996f64, 71998f64, 71998f64, 72003f64, 72003f64, 72148f64, 72151f64, 72154f64, 72155f64, 72160f64, 72160f64, 72193f64, 72202f64, 72243f64, 72248f64, 72251f64, 72254f64, 72263f64, 72263f64, 72273f64, 72278f64, 72281f64, 72283f64, 72330f64, 72342f64, 72344f64, 72345f64, 72752f64, 72758f64, 72760f64, 72765f64, 72767f64, 72767f64, 72850f64, 72871f64, 72874f64, 72880f64, 72882f64, 72883f64, 72885f64, 72886f64, 73009f64, 73014f64, 73018f64, 73018f64, 73020f64, 73021f64, 73023f64, 73029f64, 73031f64, 73031f64, 73104f64, 73105f64, 73109f64, 73109f64, 73111f64, 73111f64, 73459f64, 73460f64, 73472f64, 73473f64, 73526f64, 73530f64, 73536f64, 73536f64, 73538f64, 73538f64, 73562f64, 73562f64, 78896f64, 78911f64, 78912f64, 78912f64, 78919f64, 78933f64, 90398f64, 90409f64, 90413f64, 90415f64, 92912f64, 92916f64, 92976f64, 92982f64, 92992f64, 92995f64, 93504f64, 93506f64, 93547f64, 93548f64, 94031f64, 94031f64, 94095f64, 94098f64, 94099f64, 94111f64, 94176f64, 94177f64, 94179f64, 94179f64, 94180f64, 94180f64, 110576f64, 110579f64, 110581f64, 110587f64, 110589f64, 110590f64, 113821f64, 113822f64, 113824f64, 113827f64, 118528f64, 118573f64, 118576f64, 118598f64, 119143f64, 119145f64, 119155f64, 119162f64, 119163f64, 119170f64, 119173f64, 119179f64, 119210f64, 119213f64, 119362f64, 119364f64, 121344f64, 121398f64, 121403f64, 121452f64, 121461f64, 121461f64, 121476f64, 121476f64, 121499f64, 121503f64, 121505f64, 121519f64, 122880f64, 122886f64, 122888f64, 122904f64, 122907f64, 122913f64, 122915f64, 122916f64, 122918f64, 122922f64, 122928f64, 122989f64, 123023f64, 123023f64, 123184f64, 123190f64, 123191f64, 123197f64, 123566f64, 123566f64, 123628f64, 123631f64, 124139f64, 124139f64, 124140f64, 124143f64, 124398f64, 124399f64, 125136f64, 125142f64, 125252f64, 125258f64, 125259f64, 125259f64, 127995f64, 127999f64, 917505f64, 917505f64, 917536f64, 917631f64, 917760f64, 917999f64]))
}

pub fn accept_unicode_case_property(point: f64, ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> bool {
    {
        let first = 0f64;
        {
            let last = (({ let ml_reference_array45 = ranges.clone(); let ml_reference_projection45 = ml_reference_array45.borrow().len() as f64; ml_reference_projection45 } / 2f64)).floor();
            {
                let source_ml_source2_s617 = self::source_ml_source1_temporary_accept_unicode_case_property_loop51(point, ranges.clone(), first, last);
                match source_ml_source2_s617.clone() {
                    self::MlSource1TemporaryAcceptUnicodeCasePropertyLoop51Result::MlSource1TemporaryAcceptUnicodeCasePropertyLoop51Done => {
                        false
                    }
                    self::MlSource1TemporaryAcceptUnicodeCasePropertyLoop51Result::MlSource1TemporaryAcceptUnicodeCasePropertyLoop51Return(source_ml_source1_temporary_result51) => {
                        source_ml_source1_temporary_result51
                    }
                }
            }
        }
    }
}

pub fn read_unicode_case_mapping(point: f64, ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, indexes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let first = 0f64;
        {
            let last = (({ let ml_reference_array46 = indexes.clone(); let ml_reference_projection46 = ml_reference_array46.borrow().len() as f64; ml_reference_projection46 } / 3f64)).floor();
            {
                let source_ml_source2_s618 = self::source_ml_source1_temporary_read_unicode_case_mapping_loop52(point, indexes.clone(), values.clone(), first, last);
                match source_ml_source2_s618.clone() {
                    self::MlSource1TemporaryReadUnicodeCaseMappingLoop52Result::MlSource1TemporaryReadUnicodeCaseMappingLoop52Done(first_2, last_2) => {
                        {
                            let first_3 = 0f64;
                            {
                                let last_3 = (({ let ml_reference_array47 = ranges.clone(); let ml_reference_projection47 = ml_reference_array47.borrow().len() as f64; ml_reference_projection47 } / 4f64)).floor();
                                {
                                    let source_ml_source2_s619 = self::source_ml_source1_temporary_read_unicode_case_mapping_loop53(point, ranges.clone(), first_3, last_3);
                                    match source_ml_source2_s619.clone() {
                                        self::MlSource1TemporaryReadUnicodeCaseMappingLoop53Result::MlSource1TemporaryReadUnicodeCaseMappingLoop53Done => {
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![point]))
                                        }
                                        self::MlSource1TemporaryReadUnicodeCaseMappingLoop53Result::MlSource1TemporaryReadUnicodeCaseMappingLoop53Return(source_ml_source1_temporary_result53) => {
                                            source_ml_source1_temporary_result53.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryReadUnicodeCaseMappingLoop52Result::MlSource1TemporaryReadUnicodeCaseMappingLoop52Return(source_ml_source1_temporary_result53_2) => {
                        source_ml_source1_temporary_result53_2.clone()
                    }
                }
            }
        }
    }
}

pub fn map_unicode_case_utf16_columns(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, upper: bool, ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, indexes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, cased: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, ignorable: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let points = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let at = 0f64;
            {
                let points_2 = self::source_ml_source1_temporary_map_unicode_case_utf16_columns_loop55(text.clone(), points.clone(), at);
                {
                    let following = std::rc::Rc::new(std::cell::RefCell::new(Vec::<bool>::new()));
                    {
                        let at_2 = ({ let ml_reference_array48 = points_2.clone(); let ml_reference_projection48 = ml_reference_array48.borrow().len() as f64; ml_reference_projection48 } - 1f64);
                        {
                            let following_2 = self::source_ml_source1_temporary_map_unicode_case_utf16_columns_loop56(cased.clone(), ignorable.clone(), points_2.clone(), following.clone(), false, at_2);
                            {
                                let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                {
                                    let at_3 = 0f64;
                                    {
                                        let result_2 = self::source_ml_source1_temporary_map_unicode_case_utf16_columns_loop57(upper, ranges.clone(), indexes.clone(), values.clone(), cased.clone(), ignorable.clone(), points_2.clone(), following_2.clone(), result.clone(), false, at_3);
                                        result_2.clone()
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

pub fn map_unicode_case_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, upper: bool) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let ranges = if upper {
            self::read_unicode_case_upper_ranges()
        } else {
            self::read_unicode_case_lower_ranges()
        };
        {
            let indexes = if upper {
                self::read_unicode_case_upper_indexes()
            } else {
                self::read_unicode_case_lower_indexes()
            };
            {
                let values = if upper {
                    self::read_unicode_case_upper_values()
                } else {
                    self::read_unicode_case_lower_values()
                };
                self::map_unicode_case_utf16_columns(text.clone(), upper, ranges.clone(), indexes.clone(), values.clone(), self::read_unicode_case_cased_ranges(), self::read_unicode_case_ignorable_ranges())
            }
        }
    }
}

pub fn find_java_script_property_descriptor_entry(properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, key: self::JavaScriptSemanticValue) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source2_s620 = self::source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop59(properties.clone(), key.clone(), at);
            match source_ml_source2_s620.clone() {
                self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop59Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop59Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop59Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop59Return(source_ml_source1_temporary_result59) => {
                    source_ml_source1_temporary_result59
                }
            }
        }
    }
}

pub fn read_java_script_property_descriptor_kind(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::ml_string::Text {
    {
        let data = (({ let ml_reference_value49 = descriptor.borrow().clone(); ml_reference_value49 }).9 || ({ let ml_reference_value50 = descriptor.borrow().clone(); ml_reference_value50 }).11);
        {
            let accessor = (({ let ml_reference_value51 = descriptor.borrow().clone(); ml_reference_value51 }).5 || ({ let ml_reference_value52 = descriptor.borrow().clone(); ml_reference_value52 }).7);
            if (data && accessor) {
                self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])
            } else {
                if data {
                    self::ml_string::Text::from_units(vec![100, 97, 116, 97])
                } else {
                    if accessor {
                        self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])
                    } else {
                        self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99])
                    }
                }
            }
        }
    }
}

pub fn test_java_script_property_descriptors_same_value(left: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, right: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> bool {
    !((((((((((((((({ let ml_reference_value53 = left.borrow().clone(); ml_reference_value53 }).1) != (({ let ml_reference_value54 = right.borrow().clone(); ml_reference_value54 }).1)) || (({ let ml_reference_value55 = left.borrow().clone(); ml_reference_value55 }).1 && ((({ let ml_reference_value56 = left.borrow().clone(); ml_reference_value56 }).0) != (({ let ml_reference_value57 = right.borrow().clone(); ml_reference_value57 }).0)))) || ((({ let ml_reference_value58 = left.borrow().clone(); ml_reference_value58 }).3) != (({ let ml_reference_value59 = right.borrow().clone(); ml_reference_value59 }).3))) || (({ let ml_reference_value60 = left.borrow().clone(); ml_reference_value60 }).3 && ((({ let ml_reference_value61 = left.borrow().clone(); ml_reference_value61 }).2) != (({ let ml_reference_value62 = right.borrow().clone(); ml_reference_value62 }).2)))) || ((({ let ml_reference_value63 = left.borrow().clone(); ml_reference_value63 }).11) != (({ let ml_reference_value64 = right.borrow().clone(); ml_reference_value64 }).11))) || (({ let ml_reference_value65 = left.borrow().clone(); ml_reference_value65 }).11 && ((({ let ml_reference_value66 = left.borrow().clone(); ml_reference_value66 }).10) != (({ let ml_reference_value67 = right.borrow().clone(); ml_reference_value67 }).10)))) || ((({ let ml_reference_value68 = left.borrow().clone(); ml_reference_value68 }).9) != (({ let ml_reference_value69 = right.borrow().clone(); ml_reference_value69 }).9))) || (({ let ml_reference_value70 = left.borrow().clone(); ml_reference_value70 }).9 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value71 = left.borrow().clone(); ml_reference_value71 }).8, ({ let ml_reference_value72 = right.borrow().clone(); ml_reference_value72 }).8)))) || ((({ let ml_reference_value73 = left.borrow().clone(); ml_reference_value73 }).5) != (({ let ml_reference_value74 = right.borrow().clone(); ml_reference_value74 }).5))) || (({ let ml_reference_value75 = left.borrow().clone(); ml_reference_value75 }).5 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value76 = left.borrow().clone(); ml_reference_value76 }).4, ({ let ml_reference_value77 = right.borrow().clone(); ml_reference_value77 }).4)))) || ((({ let ml_reference_value78 = left.borrow().clone(); ml_reference_value78 }).7) != (({ let ml_reference_value79 = right.borrow().clone(); ml_reference_value79 }).7))) || (({ let ml_reference_value80 = left.borrow().clone(); ml_reference_value80 }).7 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value81 = left.borrow().clone(); ml_reference_value81 }).6, ({ let ml_reference_value82 = right.borrow().clone(); ml_reference_value82 }).6)))))
}

pub fn read_java_script_property_descriptor_error(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::ml_string::Text {
    if ((self::read_java_script_property_descriptor_kind(descriptor.clone())) == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))) {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        if (({ let ml_reference_value83 = descriptor.borrow().clone(); ml_reference_value83 }).9 && ((self::read_java_script_semantic_value_error(({ let ml_reference_value84 = descriptor.borrow().clone(); ml_reference_value84 }).8)) != (self::ml_string::Text::from_units(vec![])))) {
            self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108, 117, 101])
        } else {
            if (({ let ml_reference_value85 = descriptor.borrow().clone(); ml_reference_value85 }).5 && (((self::read_java_script_semantic_value_error(({ let ml_reference_value86 = descriptor.borrow().clone(); ml_reference_value86 }).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor(({ let ml_reference_value87 = descriptor.borrow().clone(); ml_reference_value87 }).4)))) {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                if (({ let ml_reference_value88 = descriptor.borrow().clone(); ml_reference_value88 }).7 && (((self::read_java_script_semantic_value_error(({ let ml_reference_value89 = descriptor.borrow().clone(); ml_reference_value89 }).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor(({ let ml_reference_value90 = descriptor.borrow().clone(); ml_reference_value90 }).6)))) {
                    self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn test_complete_java_script_property_descriptor(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> bool {
    if ((!(({ let ml_reference_value91 = descriptor.borrow().clone(); ml_reference_value91 }).1) || !(({ let ml_reference_value92 = descriptor.borrow().clone(); ml_reference_value92 }).3)) || ((self::read_java_script_property_descriptor_error(descriptor.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if (kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97]))) {
                (({ let ml_reference_value93 = descriptor.borrow().clone(); ml_reference_value93 }).9 && ({ let ml_reference_value94 = descriptor.borrow().clone(); ml_reference_value94 }).11)
            } else {
                (((kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) && ({ let ml_reference_value95 = descriptor.borrow().clone(); ml_reference_value95 }).5) && ({ let ml_reference_value96 = descriptor.borrow().clone(); ml_reference_value96 }).7)
            }
        }
    }
}

pub fn create_java_script_property_descriptor(kind: self::ml_string::Text, configurable: bool, enumerable: bool) -> std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source2_field625 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
        {
            let source_ml_source2_field626 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source2_field627 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                {
                    let source_ml_source2_field629 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                    {
                        let source_ml_source2_field630 = self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source2_field631 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                            {
                                let source_ml_source2_field632 = self::JavaScriptSemanticValue::Undefined;
                                (configurable, true, enumerable, true, source_ml_source2_field630.clone(), source_ml_source2_field629, source_ml_source2_field632.clone(), source_ml_source2_field631, source_ml_source2_field626.clone(), source_ml_source2_field625, false, source_ml_source2_field627)
                            }
                        }
                    }
                }
            }
        }
    }))
}

pub fn apply_java_script_property_descriptor_fields(base: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source2_field634 = if ({ let ml_reference_value97 = proposed.borrow().clone(); ml_reference_value97 }).1 {
            ({ let ml_reference_value98 = proposed.borrow().clone(); ml_reference_value98 }).0
        } else {
            ({ let ml_reference_value99 = base.borrow().clone(); ml_reference_value99 }).0
        };
        {
            let source_ml_source2_field636 = if ({ let ml_reference_value100 = proposed.borrow().clone(); ml_reference_value100 }).3 {
                ({ let ml_reference_value101 = proposed.borrow().clone(); ml_reference_value101 }).2
            } else {
                ({ let ml_reference_value102 = base.borrow().clone(); ml_reference_value102 }).2
            };
            {
                let source_ml_source2_field637 = ({ let ml_reference_value103 = base.borrow().clone(); ml_reference_value103 }).9;
                {
                    let source_ml_source2_field638 = if (({ let ml_reference_value104 = base.borrow().clone(); ml_reference_value104 }).9 && ({ let ml_reference_value105 = proposed.borrow().clone(); ml_reference_value105 }).9) {
                        ({ let ml_reference_value106 = proposed.borrow().clone(); ml_reference_value106 }).8
                    } else {
                        ({ let ml_reference_value107 = base.borrow().clone(); ml_reference_value107 }).8
                    };
                    {
                        let source_ml_source2_field639 = ({ let ml_reference_value108 = base.borrow().clone(); ml_reference_value108 }).11;
                        {
                            let source_ml_source2_field640 = if (({ let ml_reference_value109 = base.borrow().clone(); ml_reference_value109 }).11 && ({ let ml_reference_value110 = proposed.borrow().clone(); ml_reference_value110 }).11) {
                                ({ let ml_reference_value111 = proposed.borrow().clone(); ml_reference_value111 }).10
                            } else {
                                ({ let ml_reference_value112 = base.borrow().clone(); ml_reference_value112 }).10
                            };
                            {
                                let source_ml_source2_field641 = ({ let ml_reference_value113 = base.borrow().clone(); ml_reference_value113 }).5;
                                {
                                    let source_ml_source2_field642 = if (({ let ml_reference_value114 = base.borrow().clone(); ml_reference_value114 }).5 && ({ let ml_reference_value115 = proposed.borrow().clone(); ml_reference_value115 }).5) {
                                        ({ let ml_reference_value116 = proposed.borrow().clone(); ml_reference_value116 }).4
                                    } else {
                                        ({ let ml_reference_value117 = base.borrow().clone(); ml_reference_value117 }).4
                                    };
                                    {
                                        let source_ml_source2_field643 = ({ let ml_reference_value118 = base.borrow().clone(); ml_reference_value118 }).7;
                                        {
                                            let source_ml_source2_field644 = if (({ let ml_reference_value119 = base.borrow().clone(); ml_reference_value119 }).7 && ({ let ml_reference_value120 = proposed.borrow().clone(); ml_reference_value120 }).7) {
                                                ({ let ml_reference_value121 = proposed.borrow().clone(); ml_reference_value121 }).6
                                            } else {
                                                ({ let ml_reference_value122 = base.borrow().clone(); ml_reference_value122 }).6
                                            };
                                            (source_ml_source2_field634, true, source_ml_source2_field636, true, source_ml_source2_field642.clone(), source_ml_source2_field641, source_ml_source2_field644.clone(), source_ml_source2_field643, source_ml_source2_field638.clone(), source_ml_source2_field637, source_ml_source2_field640, source_ml_source2_field639)
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

pub fn read_java_script_property_descriptor_writes(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    {
        let writes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
        {
            let writes_3 = if ({ let ml_reference_value123 = descriptor.borrow().clone(); ml_reference_value123 }).1 {
                {
                    let writes_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_concat_value646 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])]));
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value124 = writes.borrow().clone(); ml_reference_value124 }), { let ml_reference_value125 = source_ml_source2_concat_value646.borrow().clone(); ml_reference_value125 })
                    }));
                    writes_2.clone()
                }
            } else {
                writes.clone()
            };
            {
                let writes_5 = if ({ let ml_reference_value126 = descriptor.borrow().clone(); ml_reference_value126 }).3 {
                    {
                        let writes_4 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_concat_value648 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])]));
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value127 = writes_3.borrow().clone(); ml_reference_value127 }), { let ml_reference_value128 = source_ml_source2_concat_value648.borrow().clone(); ml_reference_value128 })
                        }));
                        writes_4.clone()
                    }
                } else {
                    writes_3.clone()
                };
                {
                    let writes_7 = if ({ let ml_reference_value129 = descriptor.borrow().clone(); ml_reference_value129 }).9 {
                        {
                            let writes_6 = std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source2_concat_value650 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])]));
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value130 = writes_5.borrow().clone(); ml_reference_value130 }), { let ml_reference_value131 = source_ml_source2_concat_value650.borrow().clone(); ml_reference_value131 })
                            }));
                            writes_6.clone()
                        }
                    } else {
                        writes_5.clone()
                    };
                    {
                        let writes_9 = if ({ let ml_reference_value132 = descriptor.borrow().clone(); ml_reference_value132 }).11 {
                            {
                                let writes_8 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_concat_value652 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])]));
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value133 = writes_7.borrow().clone(); ml_reference_value133 }), { let ml_reference_value134 = source_ml_source2_concat_value652.borrow().clone(); ml_reference_value134 })
                                }));
                                writes_8.clone()
                            }
                        } else {
                            writes_7.clone()
                        };
                        {
                            let writes_11 = if ({ let ml_reference_value135 = descriptor.borrow().clone(); ml_reference_value135 }).5 {
                                {
                                    let writes_10 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_concat_value654 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![103, 101, 116])]));
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value136 = writes_9.borrow().clone(); ml_reference_value136 }), { let ml_reference_value137 = source_ml_source2_concat_value654.borrow().clone(); ml_reference_value137 })
                                    }));
                                    writes_10.clone()
                                }
                            } else {
                                writes_9.clone()
                            };
                            {
                                let writes_13 = if ({ let ml_reference_value138 = descriptor.borrow().clone(); ml_reference_value138 }).7 {
                                    {
                                        let writes_12 = std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_concat_value656 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 101, 116])]));
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value139 = writes_11.borrow().clone(); ml_reference_value139 }), { let ml_reference_value140 = source_ml_source2_concat_value656.borrow().clone(); ml_reference_value140 })
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

pub fn create_java_script_property_transition(disposition: self::ml_string::Text, error: self::ml_string::Text, descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, writes: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>> {
    std::rc::Rc::new(std::cell::RefCell::new((descriptor.clone(), disposition.clone(), error.clone(), writes.clone())))
}
