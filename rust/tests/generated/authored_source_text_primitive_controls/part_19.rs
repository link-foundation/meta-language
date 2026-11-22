pub fn read_java_script_big_integer_intrinsic_result_kind(operation: self::ml_string::Text) -> self::ml_string::Text {
    {
        let form = self::read_java_script_big_integer_result_operation(operation.clone());
        if (((form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 112, 111, 119, 101, 114]))) || (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 99, 111, 109, 112, 97, 114, 101])))) || (form == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 45, 99, 111, 109, 112, 97, 114, 101])))) {
            self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])
        } else {
            if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) {
                self::ml_string::Text::from_units(vec![105, 110, 116])
            } else {
                if (form == (self::ml_string::Text::from_units(vec![]))) {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 45, 105, 110, 116, 101, 103, 101, 114])
                }
            }
        }
    }
}

pub fn render_java_script_big_integer_comparison(target: self::ml_string::Text, form: self::ml_string::Text, left: self::ml_string::Text, right: self::ml_string::Text, utf16: bool) -> self::ml_string::Text {
    if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 45, 99, 111, 109, 112, 97, 114, 101]))) {
        if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            ((((self::ml_string::Text::from_units(vec![109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 109, 112, 97, 114, 101, 78, 117, 109, 98, 101, 114, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                ((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 99, 111, 109, 112, 97, 114, 101, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 110, 117, 109, 98, 101, 114, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                if ((target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                    ((((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 109, 112, 97, 114, 101, 78, 117, 109, 98, 101, 114, 32])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    } else {
        if (form != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103, 45, 99, 111, 109, 112, 97, 114, 101]))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
                ((((self::ml_string::Text::from_units(vec![40, 40, 109, 108, 95, 116, 101, 120, 116, 44, 109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 41, 61, 62, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 109, 112, 97, 114, 101, 85, 110, 105, 116, 115, 40, 65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 40, 123, 108, 101, 110, 103, 116, 104, 58, 109, 108, 95, 116, 101, 120, 116, 46, 108, 101, 110, 103, 116, 104, 125, 44, 40, 95, 44, 105, 110, 100, 101, 120, 41, 61, 62, 109, 108, 95, 116, 101, 120, 116, 46, 99, 104, 97, 114, 67, 111, 100, 101, 65, 116, 40, 105, 110, 100, 101, 120, 41, 41, 44, 109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 41, 41, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                    ((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 99, 111, 109, 112, 97, 114, 101, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 115, 116, 114, 105, 110, 103, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 117, 110, 105, 116, 115, 40, 38])).concat(&if utf16 {
                        ((self::ml_string::Text::from_units(vec![40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 46, 48, 46, 105, 116, 101, 114, 40, 41, 46, 99, 111, 112, 105, 101, 100, 40, 41]))
                    } else {
                        ((self::ml_string::Text::from_units(vec![40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 46, 101, 110, 99, 111, 100, 101, 95, 117, 116, 102, 49, 54, 40, 41]))
                    })).concat(&self::ml_string::Text::from_units(vec![46, 109, 97, 112, 40, 102, 54, 52, 58, 58, 102, 114, 111, 109, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 102, 54, 52, 62, 62, 40, 41, 44]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
                } else {
                    if (target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                        ((((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 109, 112, 97, 114, 101, 85, 110, 105, 116, 115, 32, 40, 83, 116, 114, 105, 110, 103, 46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32, 40, 117, 110, 105, 116, 115, 32, 58, 32, 65, 114, 114, 97, 121, 32, 70, 108, 111, 97, 116, 41, 32, 99, 32, 61, 62, 32, 105, 102, 32, 99, 46, 116, 111, 78, 97, 116, 32, 60, 32, 54, 53, 53, 51, 54, 32, 116, 104, 101, 110, 32, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 99, 46, 116, 111, 78, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 53, 50, 57, 54, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 47, 32, 49, 48, 50, 52, 41, 41, 41, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 54, 51, 50, 48, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 37, 32, 49, 48, 50, 52, 41, 41, 41, 32, 35, 91, 93, 32])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 32]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
                    } else {
                        if (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113]))) {
                            ((((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 109, 112, 97, 114, 101, 85, 116, 102, 56, 32, 40, 40, 102, 105, 120, 32, 109, 108, 95, 99, 111, 109, 112, 97, 114, 101, 95, 98, 121, 116, 101, 115, 32, 40, 116, 101, 120, 116, 32, 58, 32, 115, 116, 114, 105, 110, 103, 41, 32, 58, 32, 108, 105, 115, 116, 32, 102, 108, 111, 97, 116, 32, 58, 61, 32, 109, 97, 116, 99, 104, 32, 116, 101, 120, 116, 32, 119, 105, 116, 104, 32, 69, 109, 112, 116, 121, 83, 116, 114, 105, 110, 103, 32, 61, 62, 32, 110, 105, 108, 32, 124, 32, 83, 116, 114, 105, 110, 103, 32, 99, 32, 114, 101, 115, 116, 32, 61, 62, 32, 99, 111, 110, 115, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 111, 102, 95, 117, 105, 110, 116, 54, 51, 32, 40, 85, 105, 110, 116, 54, 51, 46, 111, 102, 95, 90, 32, 40, 90, 46, 111, 102, 95, 110, 97, 116, 32, 40, 65, 115, 99, 105, 105, 46, 110, 97, 116, 95, 111, 102, 95, 97, 115, 99, 105, 105, 32, 99, 41, 41, 41, 41, 32, 40, 109, 108, 95, 99, 111, 109, 112, 97, 114, 101, 95, 98, 121, 116, 101, 115, 32, 114, 101, 115, 116, 41, 32, 101, 110, 100, 41, 32])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 32]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41]))
                        } else {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

pub fn render_java_script_number_power(target: self::ml_string::Text, base: self::ml_string::Text, exponent: self::ml_string::Text) -> self::ml_string::Text {
    if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
        ((((self::ml_string::Text::from_units(vec![109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 112, 111, 119, 101, 114, 78, 117, 109, 98, 101, 114, 40])).concat(&base)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&exponent)).concat(&self::ml_string::Text::from_units(vec![41]))
    } else {
        if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
            ((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 112, 111, 119, 101, 114, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 110, 117, 109, 98, 101, 114, 40])).concat(&base)).concat(&self::ml_string::Text::from_units(vec![44]))).concat(&exponent)).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if ((target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                ((((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 112, 111, 119, 101, 114, 78, 117, 109, 98, 101, 114, 32])).concat(&base)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&exponent)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn render_java_script_big_integer_result_expression(target: self::ml_string::Text, form: self::ml_string::Text, value: self::ml_string::Text, utf16: bool) -> self::ml_string::Text {
    if (form == (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) {
        if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            ((self::ml_string::Text::from_units(vec![109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 66, 111, 111, 108, 101, 97, 110, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 98, 111, 111, 108, 101, 97, 110, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
            } else {
                if ((target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) || (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113])))) {
                    ((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 66, 111, 111, 108, 101, 97, 110, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    } else {
        if ((form != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) && (form != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
                if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                    ((self::ml_string::Text::from_units(vec![109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 78, 117, 109, 98, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                } else {
                    ((self::ml_string::Text::from_units(vec![40, 40, 109, 108, 95, 116, 101, 120, 116, 41, 61, 62, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 85, 110, 105, 116, 115, 40, 65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 40, 123, 108, 101, 110, 103, 116, 104, 58, 109, 108, 95, 116, 101, 120, 116, 46, 108, 101, 110, 103, 116, 104, 125, 44, 32, 40, 95, 44, 105, 110, 100, 101, 120, 41, 61, 62, 109, 108, 95, 116, 101, 120, 116, 46, 99, 104, 97, 114, 67, 111, 100, 101, 65, 116, 40, 105, 110, 100, 101, 120, 41, 41, 41, 41, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                }
            } else {
                if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                    if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                        ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 110, 117, 109, 98, 101, 114, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                    } else {
                        {
                            let units = if utf16 {
                                ((self::ml_string::Text::from_units(vec![40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 46, 48, 46, 105, 116, 101, 114, 40, 41, 46, 99, 111, 112, 105, 101, 100, 40, 41]))
                            } else {
                                ((self::ml_string::Text::from_units(vec![40])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 46, 101, 110, 99, 111, 100, 101, 95, 117, 116, 102, 49, 54, 40, 41]))
                            };
                            ((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 58, 58, 99, 111, 110, 118, 101, 114, 116, 95, 106, 97, 118, 97, 95, 115, 99, 114, 105, 112, 116, 95, 115, 116, 114, 105, 110, 103, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 117, 110, 105, 116, 115, 40, 38])).concat(&units)).concat(&self::ml_string::Text::from_units(vec![46, 109, 97, 112, 40, 102, 54, 52, 58, 58, 102, 114, 111, 109, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 102, 54, 52, 62, 62, 40, 41, 41]))
                        }
                    }
                } else {
                    if (target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                        if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                            ((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 78, 117, 109, 98, 101, 114, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                        } else {
                            ((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 85, 110, 105, 116, 115, 32, 40, 83, 116, 114, 105, 110, 103, 46, 102, 111, 108, 100, 108, 32, 40, 102, 117, 110, 32, 40, 117, 110, 105, 116, 115, 32, 58, 32, 65, 114, 114, 97, 121, 32, 70, 108, 111, 97, 116, 41, 32, 99, 32, 61, 62, 32, 105, 102, 32, 99, 46, 116, 111, 78, 97, 116, 32, 60, 32, 54, 53, 53, 51, 54, 32, 116, 104, 101, 110, 32, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 99, 46, 116, 111, 78, 97, 116, 41, 32, 101, 108, 115, 101, 32, 40, 117, 110, 105, 116, 115, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 53, 50, 57, 54, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 47, 32, 49, 48, 50, 52, 41, 41, 41, 46, 112, 117, 115, 104, 32, 40, 70, 108, 111, 97, 116, 46, 111, 102, 78, 97, 116, 32, 40, 53, 54, 51, 50, 48, 32, 43, 32, 40, 99, 46, 116, 111, 78, 97, 116, 32, 45, 32, 54, 53, 53, 51, 54, 41, 32, 37, 32, 49, 48, 50, 52, 41, 41, 41, 32, 35, 91, 93, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                        }
                    } else {
                        if (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113]))) {
                            if (form == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
                                ((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 78, 117, 109, 98, 101, 114, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41]))
                            } else {
                                ((self::ml_string::Text::from_units(vec![40, 77, 76, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 67, 111, 110, 118, 101, 114, 115, 105, 111, 110, 46, 99, 111, 110, 118, 101, 114, 116, 85, 116, 102, 56, 32, 40, 40, 102, 105, 120, 32, 109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 95, 98, 121, 116, 101, 115, 32, 40, 116, 101, 120, 116, 32, 58, 32, 115, 116, 114, 105, 110, 103, 41, 32, 58, 32, 108, 105, 115, 116, 32, 102, 108, 111, 97, 116, 32, 58, 61, 32, 109, 97, 116, 99, 104, 32, 116, 101, 120, 116, 32, 119, 105, 116, 104, 32, 69, 109, 112, 116, 121, 83, 116, 114, 105, 110, 103, 32, 61, 62, 32, 110, 105, 108, 32, 124, 32, 83, 116, 114, 105, 110, 103, 32, 99, 32, 114, 101, 115, 116, 32, 61, 62, 32, 99, 111, 110, 115, 32, 40, 80, 114, 105, 109, 70, 108, 111, 97, 116, 46, 111, 102, 95, 117, 105, 110, 116, 54, 51, 32, 40, 85, 105, 110, 116, 54, 51, 46, 111, 102, 95, 90, 32, 40, 90, 46, 111, 102, 95, 110, 97, 116, 32, 40, 65, 115, 99, 105, 105, 46, 110, 97, 116, 95, 111, 102, 95, 97, 115, 99, 105, 105, 32, 99, 41, 41, 41, 41, 32, 40, 109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 95, 98, 121, 116, 101, 115, 32, 114, 101, 115, 116, 41, 32, 101, 110, 100, 41, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                            }
                        } else {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

pub fn dependency_function3(value: self::ml::Big) -> f64 {
    {
        let rest = if (value < (self::ml::Big::from_i128(0))) {
            value.neg()
        } else {
            value.clone()
        };
        {
            let width = 0f64;
            {
                let source_ml_source2_s1749 = self::source_ml_source1_temporary_dependency_function3_loop135(rest.clone(), width);
                match source_ml_source2_s1749.clone() {
                    self::MlSource1TemporaryDependencyFunction3Loop135Result::MlSource1TemporaryDependencyFunction3Loop135Done(width_2) => {
                        width_2
                    }
                    self::MlSource1TemporaryDependencyFunction3Loop135Result::MlSource1TemporaryDependencyFunction3Loop135Return(source_ml_source1_temporary_result135) => {
                        source_ml_source1_temporary_result135
                    }
                }
            }
        }
    }
}

pub fn dependency_function4(value: self::ml::Big) -> f64 {
    self::dependency_function3(value.clone())
}

pub fn read_java_script_call_integer_bit_code(operation: self::ml_string::Text) -> f64 {
    if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
        0f64
    } else {
        if ((operation == (self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) || (operation == (self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
            1f64
        } else {
            if ((operation == (self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) || (operation == (self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
                2f64
            } else {
                if ((operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]))) || (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116])))) {
                    3f64
                } else {
                    if ((operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116]))) || (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116])))) {
                        4f64
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116, 32, 117, 110, 115, 105, 103, 110, 101, 100]))) {
                            5f64
                        } else {
                            if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])))) {
                                6f64
                            } else {
                                (-1f64)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn execute_java_script_call_number_bits(operation: self::ml_string::Text, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    {
        let left = ({
            let source_ml_source2_array_read_index1751 = 0f64;
            { let ml_reference_array296 = values.clone(); let ml_reference_position296 = source_ml_source2_array_read_index1751; let ml_reference_projection296 = self::ml_array::optional_at(&*ml_reference_array296.borrow(), self::ml_array::number_index(ml_reference_position296)); ml_reference_projection296 }
        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let right = ({
                let source_ml_source2_array_read_index1753 = 1f64;
                { let ml_reference_array297 = values.clone(); let ml_reference_position297 = source_ml_source2_array_read_index1753; let ml_reference_projection297 = self::ml_array::optional_at(&*ml_reference_array297.borrow(), self::ml_array::number_index(ml_reference_position297)); ml_reference_projection297 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Number(0f64));
            match left.clone() {
                self::JavaScriptSemanticValue::Number(left_value) => {
                    match right.clone() {
                        self::JavaScriptSemanticValue::Number(value) => {
                            {
                                let code = self::read_java_script_call_integer_bit_code(operation.clone());
                                if (code < (0f64)) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                } else {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::apply_java_script_number_bitwise_operation(code, left_value, value))))
                                }
                            }
                        }
                        _ => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                    }
                }
                _ => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                }
            }
        }
    }
}

pub fn execute_java_script_call_integer_primitive(operation: self::ml_string::Text, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 97, 100, 105, 120, 32, 116, 101, 120, 116]))) {
        self::execute_java_script_call_integer_radix_text(values.clone())
    } else {
        if ((operation == (self::ml_string::Text::from_units(vec![119, 114, 97, 112, 32, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![119, 114, 97, 112, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114])))) {
            self::execute_java_script_call_integer_wrapping(operation.clone(), values.clone())
        } else {
            if operation.contains(&self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115])) {
                self::execute_java_script_call_number_bits(operation.clone(), values.clone())
            } else {
                if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 116, 101, 103, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116, 32, 105, 110, 116, 101, 103, 101, 114])))) {
                    self::execute_java_script_call_mixed_integer_comparison(operation.clone(), values.clone())
                } else {
                    {
                        let first = ({
                            let source_ml_source2_array_read_index1771 = 0f64;
                            { let ml_reference_array298 = values.clone(); let ml_reference_position298 = source_ml_source2_array_read_index1771; let ml_reference_projection298 = self::ml_array::optional_at(&*ml_reference_array298.borrow(), self::ml_array::number_index(ml_reference_position298)); ml_reference_projection298 }
                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        {
                            let second = ({
                                let source_ml_source2_array_read_index1773 = 1f64;
                                { let ml_reference_array299 = values.clone(); let ml_reference_position299 = source_ml_source2_array_read_index1773; let ml_reference_projection299 = self::ml_array::optional_at(&*ml_reference_array299.borrow(), self::ml_array::number_index(ml_reference_position299)); ml_reference_projection299 }
                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(0)));
                            match first.clone() {
                                self::JavaScriptSemanticValue::Integer(first_value) => {
                                    match second.clone() {
                                        self::JavaScriptSemanticValue::Integer(value) => {
                                            if (((operation == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) || (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114])))) && (value == (self::ml::Big::from_i128(0)))) {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111]))
                                            } else {
                                                if ((operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) && (value < (self::ml::Big::from_i128(0)))) {
                                                    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![69, 120, 112, 111, 110, 101, 110, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101]))
                                                } else {
                                                    {
                                                        let left_width = self::dependency_function4(first_value.clone());
                                                        {
                                                            let right_width = self::dependency_function4(value.clone());
                                                            if ((left_width < (0f64)) || (right_width < (0f64))) {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 105, 110, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                            } else {
                                                                if (operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (first_value < value) {
                                                                        (-1f64)
                                                                    } else {
                                                                        if (first_value > value) {
                                                                            1f64
                                                                        } else {
                                                                            0f64
                                                                        }
                                                                    })))
                                                                } else {
                                                                    if ((operation == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) && (((left_width + right_width)) > (4096f64))) {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                    } else {
                                                                        if (((((operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) && (first_value != (self::ml::Big::from_i128(0)))) && (first_value != (self::ml::Big::from_i128(1)))) && (first_value != (self::ml::Big::from_i128(-1)))) && ((value.mul(&{
                                                                            let source_ml_source2_integer_result1793 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(left_width);
                                                                            if (((source_ml_source2_integer_result1793.len() as f64)) == (1f64)) {
                                                                                (self::ml_array::optional_at(&source_ml_source2_integer_result1793, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(left_width)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                                            }
                                                                        })) > (self::ml::Big::from_i128(4096)))) {
                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 101, 120, 112, 111, 110, 101, 110, 116, 105, 97, 116, 105, 111, 110, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                        } else {
                                                                            {
                                                                                let source_ml_source2_s1798 = if ((operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]))) || (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116])))) {
                                                                                    {
                                                                                        let growing = if (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]))) {
                                                                                            (value >= (self::ml::Big::from_i128(0)))
                                                                                        } else {
                                                                                            (value < (self::ml::Big::from_i128(0)))
                                                                                        };
                                                                                        {
                                                                                            let count = if (value < (self::ml::Big::from_i128(0))) {
                                                                                                value.neg()
                                                                                            } else {
                                                                                                value.clone()
                                                                                            };
                                                                                            if (((first_value != (self::ml::Big::from_i128(0))) && growing) && (count > ({
                                                                                                let source_ml_source2_integer_operand1801 = (4096f64 - left_width);
                                                                                                {
                                                                                                    let source_ml_source2_integer_result1802 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(source_ml_source2_integer_operand1801);
                                                                                                    if (((source_ml_source2_integer_result1802.len() as f64)) == (1f64)) {
                                                                                                        (self::ml_array::optional_at(&source_ml_source2_integer_result1802, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                                                    } else {
                                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_integer_operand1801)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                                                                    }
                                                                                                }
                                                                                            }))) {
                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin136::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin136Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 115, 104, 105, 102, 116, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))))
                                                                                            } else {
                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin136::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin136Next
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin136::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin136Next
                                                                                };
                                                                                match source_ml_source2_s1798.clone() {
                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin136::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin136Next => {
                                                                                        {
                                                                                            let code = self::read_java_script_call_integer_bit_code(operation.clone());
                                                                                            {
                                                                                                let result = self::ml::Big::from_i128(0);
                                                                                                {
                                                                                                    let source_ml_source2_s1807 = if (operation == (self::ml_string::Text::from_units(vec![97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                        {
                                                                                                            let result_2 = first_value.add(&value);
                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137Next(result_2.clone())
                                                                                                        }
                                                                                                    } else {
                                                                                                        if (operation == (self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                            {
                                                                                                                let result_3 = first_value.sub(&value);
                                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137Next(result_3.clone())
                                                                                                            }
                                                                                                        } else {
                                                                                                            if (operation == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                                {
                                                                                                                    let result_4 = first_value.mul(&value);
                                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137Next(result_4.clone())
                                                                                                                }
                                                                                                            } else {
                                                                                                                if ((operation == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) || (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114])))) {
                                                                                                                    {
                                                                                                                        let result_5 = ({
                                                                                                                            let source_ml_source2_array_read_value1808 = self::divide_java_script_big_integer(first_value.clone(), value.clone());
                                                                                                                            {
                                                                                                                                let source_ml_source2_array_read_index1809 = if (operation == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                                                    0f64
                                                                                                                                } else {
                                                                                                                                    1f64
                                                                                                                                };
                                                                                                                                { let ml_reference_array300 = source_ml_source2_array_read_value1808.clone(); let ml_reference_position300 = source_ml_source2_array_read_index1809; let ml_reference_projection300 = self::ml_array::optional_at(&*ml_reference_array300.borrow(), self::ml_array::number_index(ml_reference_position300)); ml_reference_projection300 }
                                                                                                                            }
                                                                                                                        }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137Next(result_5.clone())
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    if (operation == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) {
                                                                                                                        {
                                                                                                                            let result_6 = ({
                                                                                                                                let source_ml_source2_array_read_value1810 = self::power_java_script_big_integer(first_value.clone(), value.clone());
                                                                                                                                {
                                                                                                                                    let source_ml_source2_array_read_index1811 = 0f64;
                                                                                                                                    { let ml_reference_array301 = source_ml_source2_array_read_value1810.clone(); let ml_reference_position301 = source_ml_source2_array_read_index1811; let ml_reference_projection301 = self::ml_array::optional_at(&*ml_reference_array301.borrow(), self::ml_array::number_index(ml_reference_position301)); ml_reference_projection301 }
                                                                                                                                }
                                                                                                                            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137Next(result_6.clone())
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        if (operation == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 105, 110, 116, 101, 103, 101, 114]))) {
                                                                                                                            {
                                                                                                                                let result_7 = first_value.neg();
                                                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137Next(result_7.clone())
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            if (code >= (0f64)) {
                                                                                                                                {
                                                                                                                                    let result_8 = self::apply_big_integer_bitwise_operation(code, first_value.clone(), value.clone());
                                                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137Next(result_8.clone())
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))))
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    };
                                                                                                    match source_ml_source2_s1807.clone() {
                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137Next(result_9) => {
                                                                                                            if ((self::dependency_function4(result_9.clone())) < (0f64)) {
                                                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                                                            } else {
                                                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(result_9.clone())))
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137Return(source_ml_source1_temporary_result137) => {
                                                                                                            let source_ml_source1_temporary_result137 = *source_ml_source1_temporary_result137;
                                                                                                            source_ml_source1_temporary_result137.clone()
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin136::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin136Return(source_ml_source1_temporary_result137_2) => {
                                                                                        let source_ml_source1_temporary_result137_2 = *source_ml_source1_temporary_result137_2;
                                                                                        source_ml_source1_temporary_result137_2.clone()
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
                                        _ => {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                        }
                                    }
                                }
                                _ => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn execute_java_script_call_mixed_integer_comparison(operation: self::ml_string::Text, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    {
        let first = ({
            let source_ml_source2_array_read_index1813 = 0f64;
            { let ml_reference_array302 = values.clone(); let ml_reference_position302 = source_ml_source2_array_read_index1813; let ml_reference_projection302 = self::ml_array::optional_at(&*ml_reference_array302.borrow(), self::ml_array::number_index(ml_reference_position302)); ml_reference_projection302 }
        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let second = ({
                let source_ml_source2_array_read_index1815 = 1f64;
                { let ml_reference_array303 = values.clone(); let ml_reference_position303 = source_ml_source2_array_read_index1815; let ml_reference_projection303 = self::ml_array::optional_at(&*ml_reference_array303.borrow(), self::ml_array::number_index(ml_reference_position303)); ml_reference_projection303 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            match second.clone() {
                self::JavaScriptSemanticValue::Integer(second_value) => {
                    if ((self::dependency_function4(second_value.clone())) < (0f64)) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 116, 101, 103, 101, 114]))) {
                            match first.clone() {
                                self::JavaScriptSemanticValue::Number(value) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::compare_java_script_number_big_integer(value, second_value.clone()))))
                                }
                                _ => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                }
                            }
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116, 32, 105, 110, 116, 101, 103, 101, 114]))) {
                                match first.clone() {
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f1835) => {
                                        if (({ let ml_reference_array304 = source_ml_source2_f1835.clone(); let ml_reference_projection304 = ml_reference_array304.borrow().len() as f64; ml_reference_projection304 }) > (4096f64)) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                        } else {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::compare_java_script_string_big_integer_units(source_ml_source2_f1835.clone(), second_value.clone()))))
                                        }
                                    }
                                    self::JavaScriptSemanticValue::SourceText(_, text) => {
                                        if ((text.length()) > (4096f64)) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                        } else {
                                            self::execute_java_script_call_mixed_integer_comparison(operation.clone(), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::Text(self::construct_java_script_semantic_text_units(first.clone(), 4096f64)), second.clone()])))
                                        }
                                    }
                                    _ => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 116, 101, 120, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                                    }
                                }
                            } else {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                            }
                        }
                    }
                }
                _ => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
            }
        }
    }
}

pub fn execute_java_script_call_integer_wrapping(operation: self::ml_string::Text, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    {
        let width = ({
            let source_ml_source2_array_read_index1905 = 0f64;
            { let ml_reference_array305 = values.clone(); let ml_reference_position305 = source_ml_source2_array_read_index1905; let ml_reference_projection305 = self::ml_array::optional_at(&*ml_reference_array305.borrow(), self::ml_array::number_index(ml_reference_position305)); ml_reference_projection305 }
        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let input = ({
                let source_ml_source2_array_read_index1907 = 1f64;
                { let ml_reference_array306 = values.clone(); let ml_reference_position306 = source_ml_source2_array_read_index1907; let ml_reference_projection306 = self::ml_array::optional_at(&*ml_reference_array306.borrow(), self::ml_array::number_index(ml_reference_position306)); ml_reference_projection306 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            if !(self::ml_array::includes(&{ let ml_reference_value307 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 97, 112, 32, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 97, 112, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114])]))).borrow().clone(); ml_reference_value307 }, &operation.clone(), 0f64, false, |a, b| a == b)) {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 119, 114, 97, 112, 112, 105, 110, 103, 32, 99, 111, 110, 115, 117, 109, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 73, 110, 100, 101, 120, 32, 97, 110, 100, 32, 84, 111, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
            } else {
                match width.clone() {
                    self::JavaScriptSemanticValue::Number(bit_width) => {
                        if (!(self::ml_math::is_safe_integer(bit_width)) || (bit_width < (0f64))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 119, 114, 97, 112, 112, 105, 110, 103, 32, 99, 111, 110, 115, 117, 109, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 73, 110, 100, 101, 120, 32, 97, 110, 100, 32, 84, 111, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        } else {
                            match input.clone() {
                                self::JavaScriptSemanticValue::Integer(integer) => {
                                    if (bit_width == (0f64)) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(0))))
                                    } else {
                                        if ((bit_width > (4096f64)) || ((self::dependency_function4(integer.clone())) < (0f64))) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 119, 114, 97, 112, 112, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                        } else {
                                            {
                                                let modulus = self::ml::Big::from_i128(1);
                                                {
                                                    let bit = 0f64;
                                                    {
                                                        let modulus_2 = self::source_ml_source1_temporary_execute_java_script_call_integer_wrapping_loop138(bit_width, modulus.clone(), bit);
                                                        {
                                                            let result = {
                                                                let source_ml_source2_integer_division1924 = self::ml_big_integer_conversion::divide_java_script_big_integer(integer.clone(),modulus_2.clone());
                                                                if (((source_ml_source2_integer_division1924.len() as f64)) == (2f64)) {
                                                                    (self::ml_array::optional_at(&source_ml_source2_integer_division1924, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                } else {
                                                                    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111])).0, message_present: true }).raise()
                                                                }
                                                            };
                                                            {
                                                                let result_3 = if (result < (self::ml::Big::from_i128(0))) {
                                                                    {
                                                                        let result_2 = result.add(&modulus_2);
                                                                        result_2.clone()
                                                                    }
                                                                } else {
                                                                    result.clone()
                                                                };
                                                                {
                                                                    let result_5 = if ((operation == (self::ml_string::Text::from_units(vec![119, 114, 97, 112, 32, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114]))) && (result_3 >= (self::ml::divide(&modulus_2, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)))) {
                                                                        {
                                                                            let result_4 = result_3.sub(&modulus_2);
                                                                            result_4.clone()
                                                                        }
                                                                    } else {
                                                                        result_3.clone()
                                                                    };
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(result_5.clone())))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 119, 114, 97, 112, 112, 105, 110, 103, 32, 99, 111, 110, 115, 117, 109, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 73, 110, 100, 101, 120, 32, 97, 110, 100, 32, 84, 111, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                            }
                        }
                    }
                    _ => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 119, 114, 97, 112, 112, 105, 110, 103, 32, 99, 111, 110, 115, 117, 109, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 73, 110, 100, 101, 120, 32, 97, 110, 100, 32, 84, 111, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                    }
                }
            }
        }
    }
}

pub fn execute_java_script_call_integer_radix_text(values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    {
        let input = ({
            let source_ml_source2_array_read_index1926 = 0f64;
            { let ml_reference_array308 = values.clone(); let ml_reference_position308 = source_ml_source2_array_read_index1926; let ml_reference_projection308 = self::ml_array::optional_at(&*ml_reference_array308.borrow(), self::ml_array::number_index(ml_reference_position308)); ml_reference_projection308 }
        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let radix = ({
                let source_ml_source2_array_read_index1928 = 1f64;
                { let ml_reference_array309 = values.clone(); let ml_reference_position309 = source_ml_source2_array_read_index1928; let ml_reference_projection309 = self::ml_array::optional_at(&*ml_reference_array309.borrow(), self::ml_array::number_index(ml_reference_position309)); ml_reference_projection309 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            match input.clone() {
                self::JavaScriptSemanticValue::Integer(value) => {
                    match radix.clone() {
                        self::JavaScriptSemanticValue::Number(radix_value) => {
                            {
                                let base = if (radix_value.is_nan() || (radix_value == (0f64))) {
                                    0f64
                                } else {
                                    radix_value.trunc()
                                };
                                if ((base < (2f64)) || (base > (36f64))) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103, 40, 41, 32, 114, 97, 100, 105, 120, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 98, 101, 116, 119, 101, 101, 110, 32, 50, 32, 97, 110, 100, 32, 51, 54]))
                                } else {
                                    if ((self::dependency_function4(value.clone())) < (0f64)) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 114, 97, 100, 105, 120, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                    } else {
                                        {
                                            let remaining = if (value < (self::ml::Big::from_i128(0))) {
                                                value.neg()
                                            } else {
                                                value.clone()
                                            };
                                            {
                                                let divisor = {
                                                    let source_ml_source2_integer_result1946 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(base);
                                                    if (((source_ml_source2_integer_result1946.len() as f64)) == (1f64)) {
                                                        (self::ml_array::optional_at(&source_ml_source2_integer_result1946, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                    } else {
                                                        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(base)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                    }
                                                };
                                                {
                                                    let reversed = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                    {
                                                        let reversed_2 = self::source_ml_source1_temporary_execute_java_script_call_integer_radix_text_loop139(remaining.clone(), divisor.clone(), reversed.clone());
                                                        {
                                                            let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                            {
                                                                let units_3 = if (value < (self::ml::Big::from_i128(0))) {
                                                                    {
                                                                        let units_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value310 = units.borrow().clone(); ml_reference_value310 }, vec![45f64])));
                                                                        units_2.clone()
                                                                    }
                                                                } else {
                                                                    units.clone()
                                                                };
                                                                {
                                                                    let index = ({ let ml_reference_array311 = reversed_2.clone(); let ml_reference_projection311 = ml_reference_array311.borrow().len() as f64; ml_reference_projection311 } - 1f64);
                                                                    {
                                                                        let units_4 = self::source_ml_source1_temporary_execute_java_script_call_integer_radix_text_loop140(reversed_2.clone(), units_3.clone(), index);
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(units_4.clone())))
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
                        _ => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 114, 97, 100, 105, 120, 32, 102, 111, 114, 109, 97, 116, 116, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66, 105, 103, 73, 110, 116, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 114, 97, 100, 105, 120]))
                        }
                    }
                }
                _ => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 114, 97, 100, 105, 120, 32, 102, 111, 114, 109, 97, 116, 116, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66, 105, 103, 73, 110, 116, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 114, 97, 100, 105, 120]))
                }
            }
        }
    }
}

pub fn dependency_function5(value: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    {
        let text = self::ml_string::Text::from_units(vec![]);
        {
            let source_ml_source2_s1955 = match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::MlSource1TemporaryDependencyFunction5Join141::MlSource1TemporaryDependencyFunction5Join141Next(self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))
                }
                self::JavaScriptSemanticValue::Null => {
                    self::MlSource1TemporaryDependencyFunction5Join141::MlSource1TemporaryDependencyFunction5Join141Next(self::ml_string::Text::from_units(vec![110, 117, 108, 108]))
                }
                self::JavaScriptSemanticValue::Boolean(value_value) => {
                    {
                        let text_2 = if value_value {
                            self::ml_string::Text::from_units(vec![116, 114, 117, 101])
                        } else {
                            self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
                        };
                        self::MlSource1TemporaryDependencyFunction5Join141::MlSource1TemporaryDependencyFunction5Join141Next(text_2.clone())
                    }
                }
                self::JavaScriptSemanticValue::Number(value_value_2) => {
                    {
                        let text_3 = self::ml_string::Text::from_text(&self::ml_number::js_number(value_value_2));
                        self::MlSource1TemporaryDependencyFunction5Join141::MlSource1TemporaryDependencyFunction5Join141Next(text_3.clone())
                    }
                }
                self::JavaScriptSemanticValue::Integer(value_value_3) => {
                    if ((self::dependency_function3(value_value_3.clone())) < (0f64)) {
                        self::MlSource1TemporaryDependencyFunction5Join141::MlSource1TemporaryDependencyFunction5Join141Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))))
                    } else {
                        {
                            let text_4 = self::ml_string::Text::from_text(&value_value_3.to_string());
                            self::MlSource1TemporaryDependencyFunction5Join141::MlSource1TemporaryDependencyFunction5Join141Next(text_4.clone())
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    self::MlSource1TemporaryDependencyFunction5Join141::MlSource1TemporaryDependencyFunction5Join141Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 115, 116, 114, 105, 110, 103]))))
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    self::MlSource1TemporaryDependencyFunction5Join141::MlSource1TemporaryDependencyFunction5Join141Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))))
                }
                _ => {
                    self::MlSource1TemporaryDependencyFunction5Join141::MlSource1TemporaryDependencyFunction5Join141Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))))
                }
            };
            match source_ml_source2_s1955.clone() {
                self::MlSource1TemporaryDependencyFunction5Join141::MlSource1TemporaryDependencyFunction5Join141Next(text_5) => {
                    {
                        let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let index = 0f64;
                            {
                                let units_2 = self::source_ml_source1_temporary_dependency_function5_loop142(text_5.clone(), units.clone(), index);
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(units_2.clone())))
                            }
                        }
                    }
                }
                self::MlSource1TemporaryDependencyFunction5Join141::MlSource1TemporaryDependencyFunction5Join141Return(source_ml_source1_temporary_result142) => {
                    let source_ml_source1_temporary_result142 = *source_ml_source1_temporary_result142;
                    source_ml_source1_temporary_result142.clone()
                }
            }
        }
    }
}

pub fn execute_java_script_call_number_conversion(operation: self::ml_string::Text, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    if (((operation != (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) && (operation != (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) && (operation != (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99])))) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(f64::NAN)))
            }
            self::JavaScriptSemanticValue::Null => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(0f64)))
            }
            self::JavaScriptSemanticValue::Boolean(value_value) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if value_value {
                    1f64
                } else {
                    0f64
                })))
            }
            self::JavaScriptSemanticValue::Number(_) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
            }
            self::JavaScriptSemanticValue::Integer(value_value_2) => {
                if (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114]))
                    } else {
                        if ((self::dependency_function3(value_value_2.clone())) < (0f64)) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![101, 120, 112, 108, 105, 99, 105, 116, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::convert_java_script_big_integer_number(value_value_2.clone()))))
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(units) => {
                if (({ let ml_reference_array312 = units.clone(); let ml_reference_projection312 = ml_reference_array312.borrow().len() as f64; ml_reference_projection312 }) > (4096f64)) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                } else {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::convert_java_script_string_number_units(units.clone()))))
                }
            }
            self::JavaScriptSemanticValue::SourceText(_, text) => {
                if ((text.length()) > (4096f64)) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                } else {
                    self::execute_java_script_call_number_conversion(operation.clone(), self::JavaScriptSemanticValue::Text(self::construct_java_script_semantic_text_units(value.clone(), 4096f64)))
                }
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114]))
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
            }
        }
    }
}

pub fn read_java_script_integer_protocol_operation_error(operation: self::ml_string::Text) -> self::ml_string::Text {
    if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 101, 103, 101, 114])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
    }
}

pub fn refuse_java_script_call_integer_conversion(form: self::ml_string::Text, value: self::ml_string::Text) -> self::JavaScriptCallPrimitiveResult {
    {
        let pieces = self::read_java_script_big_integer_conversion_message_pieces(form.clone());
        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::read_java_script_big_integer_conversion_error_class(form.clone()), ((({
            let source_ml_source2_array_read_index1970 = 0f64;
            { let ml_reference_array313 = pieces.clone(); let ml_reference_position313 = source_ml_source2_array_read_index1970; let ml_reference_projection313 = self::ml_array::optional_at(&*ml_reference_array313.borrow(), self::ml_array::number_index(ml_reference_position313)); ml_reference_projection313 }
        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))).concat(&value)).concat(&({
            let source_ml_source2_array_read_index1972 = 1f64;
            { let ml_reference_array314 = pieces.clone(); let ml_reference_position314 = source_ml_source2_array_read_index1972; let ml_reference_projection314 = self::ml_array::optional_at(&*ml_reference_array314.borrow(), self::ml_array::number_index(ml_reference_position314)); ml_reference_projection314 }
        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))))
    }
}

pub fn read_java_script_call_integer_text(units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::ml_string::Text {
    {
        let text = self::ml_string::Text::from_units(vec![]);
        {
            let index = 0f64;
            {
                let text_2 = self::source_ml_source1_temporary_read_java_script_call_integer_text_loop143(units.clone(), text.clone(), index);
                text_2.clone()
            }
        }
    }
}
