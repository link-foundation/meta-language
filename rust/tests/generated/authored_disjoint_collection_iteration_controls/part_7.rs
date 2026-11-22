pub fn read_reference_callback_operand_fields(expression: self::ml_string::Text, ordinary: self::ml_string::Text) -> self::ml_string::Text {
    if (expression == (self::ml_string::Text::from_units(vec![108, 97, 109, 98, 100, 97]))) {
        self::ml_string::Text::from_units(vec![98, 111, 100, 121])
    } else {
        if (expression == (self::ml_string::Text::from_units(vec![108, 101, 116]))) {
            self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 32, 98, 111, 100, 121])
        } else {
            if (expression == (self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104]))) {
                self::ml_string::Text::from_units(vec![115, 99, 114, 117, 116, 105, 110, 101, 101, 32, 99, 97, 115, 101, 115])
            } else {
                if (expression == (self::ml_string::Text::from_units(vec![111, 117, 116, 67, 111, 110, 115]))) {
                    self::ml_string::Text::from_units(vec![104, 101, 97, 100, 32, 116, 97, 105, 108])
                } else {
                    ordinary.clone()
                }
            }
        }
    }
}

pub fn read_reference_method_required_arguments(kind: self::ml_string::Text, operation: self::ml_string::Text) -> f64 {
    if ((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && (operation == (self::ml_string::Text::from_units(vec![115, 101, 116])))) {
        2f64
    } else {
        if ((((operation == (self::ml_string::Text::from_units(vec![97, 100, 100]))) || (operation == (self::ml_string::Text::from_units(vec![103, 101, 116])))) || (operation == (self::ml_string::Text::from_units(vec![104, 97, 115])))) || (operation == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101])))) {
            1f64
        } else {
            0f64
        }
    }
}

pub fn read_reference_method_argument_role(kind: self::ml_string::Text, operation: self::ml_string::Text, position: f64) -> self::ml_string::Text {
    if ((kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) && ((operation == (self::ml_string::Text::from_units(vec![112, 117, 115, 104]))) || (operation == (self::ml_string::Text::from_units(vec![117, 110, 115, 104, 105, 102, 116]))))) {
        self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
    } else {
        if ((((kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) || (kind == (self::ml_string::Text::from_units(vec![109, 97, 112])))) && (position == (0f64))) && (operation != (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114])))) {
            self::ml_string::Text::from_units(vec![107, 101, 121])
        } else {
            if (((kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) && (operation == (self::ml_string::Text::from_units(vec![115, 101, 116])))) && (position == (1f64))) {
                self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn accept_reference_method_key_kind(kind: self::ml_string::Text) -> bool {
    ((((((kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) || (kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || (kind == (self::ml_string::Text::from_units(vec![105, 110, 116])))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116])))) || (kind == (self::ml_string::Text::from_units(vec![102, 105, 120, 101, 100]))))
}

pub fn read_reference_method_identity(kind: self::ml_string::Text, operation: self::ml_string::Text) -> self::ml_string::Text {
    {
        let source_ml_source2_s9 = if (kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
            if (operation == (self::ml_string::Text::from_units(vec![112, 117, 115, 104]))) {
                self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![97, 112, 112, 101, 110, 100, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115]))
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![112, 111, 112]))) {
                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 45, 102, 105, 110, 97, 108, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116]))
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116]))) {
                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 45, 105, 110, 105, 116, 105, 97, 108, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116]))
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![117, 110, 115, 104, 105, 102, 116]))) {
                            self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![112, 114, 101, 112, 101, 110, 100, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115]))
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![114, 101, 118, 101, 114, 115, 101]))) {
                                self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(self::ml_string::Text::from_units(vec![114, 101, 118, 101, 114, 115, 101, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115]))
                            } else {
                                self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Next
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Next
        };
        match source_ml_source2_s9.clone() {
            self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Next => {
                {
                    let source_ml_source2_s10 = if (kind == (self::ml_string::Text::from_units(vec![115, 101, 116]))) {
                        if (operation == (self::ml_string::Text::from_units(vec![97, 100, 100]))) {
                            self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text::from_units(vec![97, 100, 100, 45, 115, 101, 116, 45, 109, 101, 109, 98, 101, 114]))
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101]))) {
                                self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 45, 115, 101, 116, 45, 109, 101, 109, 98, 101, 114]))
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114]))) {
                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114, 45, 115, 101, 116, 45, 109, 101, 109, 98, 101, 114, 115]))
                                } else {
                                    if (operation == (self::ml_string::Text::from_units(vec![104, 97, 115]))) {
                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 45, 115, 101, 116, 45, 109, 101, 109, 98, 101, 114, 115, 104, 105, 112]))
                                    } else {
                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Next
                                    }
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Next
                    };
                    match source_ml_source2_s10.clone() {
                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Next => {
                            {
                                let source_ml_source2_s11 = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                    if (operation == (self::ml_string::Text::from_units(vec![115, 101, 116]))) {
                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![115, 101, 116, 45, 109, 97, 112, 45, 101, 110, 116, 114, 121]))
                                    } else {
                                        if (operation == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101]))) {
                                            self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 45, 109, 97, 112, 45, 101, 110, 116, 114, 121]))
                                        } else {
                                            if (operation == (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114]))) {
                                                self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114, 45, 109, 97, 112, 45, 101, 110, 116, 114, 105, 101, 115]))
                                            } else {
                                                if (operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))) {
                                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 45, 109, 97, 112, 45, 101, 110, 116, 114, 121]))
                                                } else {
                                                    if (operation == (self::ml_string::Text::from_units(vec![104, 97, 115]))) {
                                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 45, 109, 97, 112, 45, 109, 101, 109, 98, 101, 114, 115, 104, 105, 112]))
                                                    } else {
                                                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Next
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Next
                                };
                                match source_ml_source2_s11.clone() {
                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Next => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlSource1TemporaryReadReferenceMethodIdentityJoin5::MlSource1TemporaryReadReferenceMethodIdentityJoin5Return(source_ml_source1_temporary_result5) => {
                                        source_ml_source1_temporary_result5.clone()
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryReadReferenceMethodIdentityJoin4::MlSource1TemporaryReadReferenceMethodIdentityJoin4Return(source_ml_source1_temporary_result5_2) => {
                            source_ml_source1_temporary_result5_2.clone()
                        }
                    }
                }
            }
            self::MlSource1TemporaryReadReferenceMethodIdentityJoin3::MlSource1TemporaryReadReferenceMethodIdentityJoin3Return(source_ml_source1_temporary_result5_3) => {
                source_ml_source1_temporary_result5_3.clone()
            }
        }
    }
}

pub fn read_reference_method_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![83, 104, 97, 114, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 115, 32, 101, 118, 97, 108, 117, 97, 116, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 101, 118, 101, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 111, 110, 99, 101, 32, 105, 110, 32, 115, 111, 117, 114, 99, 101, 32, 111, 114, 100, 101, 114, 59, 32, 101, 97, 99, 104, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 109, 117, 116, 97, 116, 105, 111, 110, 32, 99, 104, 97, 110, 103, 101, 115, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 44, 32, 112, 114, 101, 115, 101, 114, 118, 105, 110, 103, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 111, 114, 100, 101, 114, 44, 32, 83, 97, 109, 101, 86, 97, 108, 117, 101, 90, 101, 114, 111, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 107, 101, 121, 115, 32, 97, 110, 100, 32, 116, 104, 101, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 32, 114, 101, 115, 117, 108, 116, 46])
}

pub fn render_reference_method_expression(target: self::ml_string::Text, kind: self::ml_string::Text, operation: self::ml_string::Text, receiver: self::ml_string::Text, arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, float_key: bool, suffix: self::ml_string::Text) -> self::ml_string::Text {
    if ((self::read_reference_method_result(kind.clone(), operation.clone())) == (self::ml_string::Text::from_units(vec![]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (target == (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
            ((((((self::ml_string::Text::from_units(vec![40])).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![41, 46]))).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&self::ml_string::join(&{ let ml_reference_value7 = arguments_.borrow().clone(); ml_reference_value7 }, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![41]))
        } else {
            if (target != (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                {
                    let owner = (self::ml_string::Text::from_units(vec![109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 111, 119, 110, 101, 114])).concat(&suffix);
                    {
                        let values = (self::ml_string::Text::from_units(vec![109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 118, 97, 108, 117, 101, 115])).concat(&suffix);
                        {
                            let prefix = ((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32])).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                            {
                                let index = 0f64;
                                {
                                    let prefix_2 = self::source_ml_source1_temporary_render_reference_method_expression_loop6(arguments_.clone(), suffix.clone(), prefix.clone(), index);
                                    {
                                        let first = ((self::ml_string::Text::from_units(vec![109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![95, 48]));
                                        {
                                            let second = ((self::ml_string::Text::from_units(vec![109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 97, 114, 103, 117, 109, 101, 110, 116])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![95, 49]));
                                            if (kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
                                                if ((operation == (self::ml_string::Text::from_units(vec![112, 117, 115, 104]))) || (operation == (self::ml_string::Text::from_units(vec![117, 110, 115, 104, 105, 102, 116])))) {
                                                    {
                                                        let names = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                        {
                                                            let index_2 = 0f64;
                                                            {
                                                                let names_2 = self::source_ml_source1_temporary_render_reference_method_expression_loop7(arguments_.clone(), suffix.clone(), names.clone(), index_2);
                                                                {
                                                                    let changes = if (operation == (self::ml_string::Text::from_units(vec![112, 117, 115, 104]))) {
                                                                        values.concat(&self::ml_string::Text::from_units(vec![46, 101, 120, 116, 101, 110, 100, 40, 109, 108, 95, 105, 110, 115, 101, 114, 116, 101, 100, 41, 59]))
                                                                    } else {
                                                                        values.concat(&self::ml_string::Text::from_units(vec![46, 115, 112, 108, 105, 99, 101, 40, 48, 46, 46, 48, 44, 32, 109, 108, 95, 105, 110, 115, 101, 114, 116, 101, 100, 41, 59]))
                                                                    };
                                                                    (((((((((((prefix_2.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 105, 110, 115, 101, 114, 116, 101, 100, 32, 61, 32, 118, 101, 99, 33, 91]))).concat(&self::ml_string::join(&{ let ml_reference_value8 = names_2.borrow().clone(); ml_reference_value8 }, &self::ml_string::Text::from_units(vec![44, 32])))).concat(&self::ml_string::Text::from_units(vec![93, 59, 32, 108, 101, 116, 32, 109, 117, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 108, 101, 110, 103, 116, 104, 32, 61, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![46, 108, 101, 110, 40, 41, 46, 99, 104, 101, 99, 107, 101, 100, 95, 97, 100, 100, 40, 109, 108, 95, 105, 110, 115, 101, 114, 116, 101, 100, 46, 108, 101, 110, 40, 41, 41, 46, 102, 105, 108, 116, 101, 114, 40, 124, 108, 101, 110, 103, 116, 104, 124, 32, 42, 108, 101, 110, 103, 116, 104, 32, 60, 61, 32, 52, 50, 57, 52, 57, 54, 55, 50, 57, 53, 117, 115, 105, 122, 101, 41, 46, 101, 120, 112, 101, 99, 116, 40, 34, 73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 34, 41, 59, 32]))).concat(&changes)).concat(&self::ml_string::Text::from_units(vec![32, 109, 108, 95, 108, 101, 110, 103, 116, 104, 32, 97, 115, 32, 102, 54, 52, 32, 125]))
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if (operation == (self::ml_string::Text::from_units(vec![112, 111, 112]))) {
                                                        ((prefix_2.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 114, 101, 115, 117, 108, 116, 32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 46, 112, 111, 112, 40, 41, 59, 32, 109, 108, 95, 114, 101, 115, 117, 108, 116, 32, 125]))
                                                    } else {
                                                        if (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116]))) {
                                                            ((((((((prefix_2.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 117, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32, 105, 102, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 101, 109, 112, 116, 121, 40, 41, 32, 123, 32, 78, 111, 110, 101, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 83, 111, 109, 101, 40]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![46, 114, 101, 109, 111, 118, 101, 40, 48, 41, 41, 32, 125, 32, 125]))
                                                        } else {
                                                            (((prefix_2.concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 46, 114, 101, 118, 101, 114, 115, 101, 40, 41, 59, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                                        }
                                                    }
                                                }
                                            } else {
                                                if (operation == (self::ml_string::Text::from_units(vec![99, 108, 101, 97, 114]))) {
                                                    (prefix_2.concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 46, 99, 108, 101, 97, 114, 40, 41, 59, 32, 40, 41, 32, 125]))
                                                } else {
                                                    {
                                                        let key = (self::ml_string::Text::from_units(vec![109, 108, 95, 109, 101, 116, 104, 111, 100, 95, 107, 101, 121])).concat(&suffix);
                                                        {
                                                            let prefix_3 = prefix_2.concat(&((((self::ml_string::Text::from_units(vec![108, 101, 116, 32])).concat(&key)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&if float_key {
                                                                ((((self::ml_string::Text::from_units(vec![105, 102, 32])).concat(&first)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32, 48, 46, 48, 32, 123, 32, 48, 46, 48, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32]))).concat(&first)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                                            } else {
                                                                first.clone()
                                                            })).concat(&self::ml_string::Text::from_units(vec![59, 32])));
                                                            {
                                                                let pattern = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                                                    self::ml_string::Text::from_units(vec![40, 109, 108, 95, 107, 101, 121, 44, 32, 95, 41])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![109, 108, 95, 107, 101, 121])
                                                                };
                                                                {
                                                                    let equality = if float_key {
                                                                        ((((self::ml_string::Text::from_units(vec![40, 109, 108, 95, 107, 101, 121, 32, 61, 61, 32, 38])).concat(&key)).concat(&self::ml_string::Text::from_units(vec![32, 124, 124, 32, 40, 109, 108, 95, 107, 101, 121, 46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 38, 38, 32]))).concat(&key)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 110, 97, 110, 40, 41, 41, 41]))
                                                                    } else {
                                                                        (self::ml_string::Text::from_units(vec![109, 108, 95, 107, 101, 121, 32, 61, 61, 32, 38])).concat(&key)
                                                                    };
                                                                    {
                                                                        let position = ((((values.concat(&self::ml_string::Text::from_units(vec![46, 105, 116, 101, 114, 40, 41, 46, 112, 111, 115, 105, 116, 105, 111, 110, 40, 124]))).concat(&pattern)).concat(&self::ml_string::Text::from_units(vec![124, 32]))).concat(&equality)).concat(&self::ml_string::Text::from_units(vec![41]));
                                                                        if (operation == (self::ml_string::Text::from_units(vec![104, 97, 115]))) {
                                                                            ((((((prefix_3.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 40, 41, 59, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 115, 111, 109, 101, 40, 41, 32, 125]))
                                                                        } else {
                                                                            if (operation == (self::ml_string::Text::from_units(vec![103, 101, 116]))) {
                                                                                ((((((((prefix_3.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 40, 41, 59, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![46, 109, 97, 112, 40, 124, 105, 110, 100, 101, 120, 124, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![91, 105, 110, 100, 101, 120, 93, 46, 49, 46, 99, 108, 111, 110, 101, 40, 41, 41, 32, 125]))
                                                                            } else {
                                                                                if (operation == (self::ml_string::Text::from_units(vec![100, 101, 108, 101, 116, 101]))) {
                                                                                    ((((((((prefix_3.concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 117, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32, 109, 97, 116, 99, 104, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40, 105, 110, 100, 101, 120, 41, 32, 61, 62, 32, 123, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![46, 114, 101, 109, 111, 118, 101, 40, 105, 110, 100, 101, 120, 41, 59, 32, 116, 114, 117, 101, 32, 125, 44, 32, 78, 111, 110, 101, 32, 61, 62, 32, 102, 97, 108, 115, 101, 32, 125, 32, 125]))
                                                                                } else {
                                                                                    {
                                                                                        let insertion = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                                                                            ((((values.concat(&self::ml_string::Text::from_units(vec![46, 112, 117, 115, 104, 40, 40]))).concat(&key)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&second)).concat(&self::ml_string::Text::from_units(vec![41, 41, 59]))
                                                                                        } else {
                                                                                            ((values.concat(&self::ml_string::Text::from_units(vec![46, 112, 117, 115, 104, 40]))).concat(&key)).concat(&self::ml_string::Text::from_units(vec![41, 59]))
                                                                                        };
                                                                                        {
                                                                                            let replacement = if (kind == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                                                                                ((values.concat(&self::ml_string::Text::from_units(vec![91, 105, 110, 100, 101, 120, 93, 46, 49, 32, 61, 32]))).concat(&second)).concat(&self::ml_string::Text::from_units(vec![59]))
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![108, 101, 116, 32, 95, 32, 61, 32, 105, 110, 100, 101, 120, 59])
                                                                                            };
                                                                                            ((((((((((((prefix_3.concat(&self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 117, 116, 32]))).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32, 109, 97, 116, 99, 104, 32]))).concat(&position)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40, 105, 110, 100, 101, 120, 41, 32, 61, 62, 32, 123, 32]))).concat(&replacement)).concat(&self::ml_string::Text::from_units(vec![32, 125, 44, 32, 78, 111, 110, 101, 32, 61, 62, 32, 123, 32]))).concat(&insertion)).concat(&self::ml_string::Text::from_units(vec![32, 125, 32, 125, 32, 125, 32]))).concat(&owner)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
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

pub fn read_array_callback_result_form(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
        self::ml_string::Text::from_units(vec![109, 97, 112, 112, 101, 100, 32, 97, 114, 114, 97, 121])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 116, 101, 114]))) {
            self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100]))) {
                self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108, 32, 101, 108, 101, 109, 101, 110, 116])
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 73, 110, 100, 101, 120]))) {
                    self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])
                } else {
                    if ((operation == (self::ml_string::Text::from_units(vec![115, 111, 109, 101]))) || (operation == (self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121])))) {
                        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112]))) {
                            self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107, 32, 97, 114, 114, 97, 121])
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![102, 111, 114, 69, 97, 99, 104]))) {
                                self::ml_string::Text::from_units(vec![117, 110, 105, 116])
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
                                    self::ml_string::Text::from_units(vec![97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])
                                } else {
                                    self::ml_string::Text::from_units(vec![])
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_array_callback_expected_result_form(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (((((operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 116, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100])))) || (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 73, 110, 100, 101, 120])))) || (operation == (self::ml_string::Text::from_units(vec![115, 111, 109, 101])))) || (operation == (self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121])))) {
        self::ml_string::Text::from_units(vec![98, 111, 111, 108])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
            self::ml_string::Text::from_units(vec![97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112]))) {
                self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_array_callback_maximum_arguments(operation: self::ml_string::Text) -> f64 {
    if (operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
        2f64
    } else {
        if ((self::read_array_callback_result_form(operation.clone())) == (self::ml_string::Text::from_units(vec![]))) {
            (-1f64)
        } else {
            1f64
        }
    }
}

pub fn read_array_callback_signature_error(operation: self::ml_string::Text, kind: self::ml_string::Text, declared_count: f64, provided_count: f64, parameters_match: bool, result_matches: bool, expected_result: self::ml_string::Text) -> self::ml_string::Text {
    if (kind != (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]))) {
        ((self::ml_string::Text::from_units(vec![46])).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
    } else {
        if ((declared_count > provided_count) || !parameters_match) {
            ((self::ml_string::Text::from_units(vec![46])).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 100, 105, 102, 102, 101, 114]))
        } else {
            if ((expected_result != (self::ml_string::Text::from_units(vec![]))) && !result_matches) {
                (((self::ml_string::Text::from_units(vec![46])).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 114, 101, 115, 117, 108, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32]))).concat(&expected_result)
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_array_callback_parameter_role(operation: self::ml_string::Text, position: f64) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
        if (position == (0f64)) {
            self::ml_string::Text::from_units(vec![97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])
        } else {
            if (position == (1f64)) {
                self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
            } else {
                if (position == (2f64)) {
                    self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
                } else {
                    if (position == (3f64)) {
                        self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        }
    } else {
        if (position == (0f64)) {
            self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
        } else {
            if (position == (1f64)) {
                self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
            } else {
                if (position == (2f64)) {
                    self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn render_array_callback_argument(operation: self::ml_string::Text, position: f64, suffix: self::ml_string::Text, clone_element: bool) -> self::ml_string::Text {
    {
        let role = self::read_array_callback_parameter_role(operation.clone(), position);
        if (role == (self::ml_string::Text::from_units(vec![97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114]))) {
            (self::ml_string::Text::from_units(vec![109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114])).concat(&suffix)
        } else {
            if (role == (self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116]))) {
                ((self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])).concat(&suffix)).concat(&if clone_element {
                    self::ml_string::Text::from_units(vec![46, 99, 108, 111, 110, 101, 40, 41])
                } else {
                    self::ml_string::Text::from_units(vec![])
                })
            } else {
                if (role == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120]))) {
                    ((self::ml_string::Text::from_units(vec![40, 109, 108, 95, 105, 110, 100, 101, 120])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![32, 97, 115, 32, 102, 54, 52, 41]))
                } else {
                    if (role == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114]))) {
                        ((self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101, 115])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![46, 99, 108, 111, 110, 101, 40, 41]))
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        }
    }
}

pub fn render_array_callback_call(operation: self::ml_string::Text, parameter_count: f64, suffix: self::ml_string::Text, clone_element: bool) -> self::ml_string::Text {
    {
        let arguments_text = self::ml_string::Text::from_units(vec![]);
        {
            let position = 0f64;
            {
                let maximum = if (operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
                    4f64
                } else {
                    3f64
                };
                {
                    let arguments_text_2 = self::source_ml_source1_temporary_render_array_callback_call_loop8(operation.clone(), parameter_count, suffix.clone(), clone_element, arguments_text.clone(), position, maximum);
                    ((((self::ml_string::Text::from_units(vec![109, 108, 95, 99, 97, 108, 108, 98, 97, 99, 107])).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&arguments_text_2)).concat(&self::ml_string::Text::from_units(vec![41]))
                }
            }
        }
    }
}

pub fn render_array_callback_expression(operation: self::ml_string::Text, object: self::ml_string::Text, callback: self::ml_string::Text, parameter_count: f64, suffix: self::ml_string::Text, initial: self::ml_string::Text, has_initial: bool) -> self::ml_string::Text {
    {
        let values = (self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101, 115])).concat(&suffix);
        {
            let index = (self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 100, 101, 120])).concat(&suffix);
            {
                let value = (self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])).concat(&suffix);
                {
                    let call = self::render_array_callback_call(operation.clone(), parameter_count, suffix.clone(), false);
                    {
                        let head = ((((((((self::ml_string::Text::from_units(vec![108, 101, 116, 32])).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&object)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 99, 97, 108, 108, 98, 97, 99, 107]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&callback)).concat(&self::ml_string::Text::from_units(vec![59]));
                        {
                            let iterator = values.concat(&self::ml_string::Text::from_units(vec![46, 105, 116, 101, 114, 40, 41, 46, 99, 108, 111, 110, 101, 100, 40, 41, 46, 101, 110, 117, 109, 101, 114, 97, 116, 101, 40, 41]));
                            if ((operation == (self::ml_string::Text::from_units(vec![115, 111, 109, 101]))) || (operation == (self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121])))) {
                                ((((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46]))).concat(&if (operation == (self::ml_string::Text::from_units(vec![115, 111, 109, 101]))) {
                                    self::ml_string::Text::from_units(vec![97, 110, 121])
                                } else {
                                    self::ml_string::Text::from_units(vec![97, 108, 108])
                                })).concat(&self::ml_string::Text::from_units(vec![40, 124, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125]))
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 73, 110, 100, 101, 120]))) {
                                    ((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46, 112, 111, 115, 105, 116, 105, 111, 110, 40, 124, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![41, 46, 109, 97, 112, 95, 111, 114, 40, 45, 49, 102, 54, 52, 44, 32, 124, 109, 108, 95, 112, 111, 115, 105, 116, 105, 111, 110, 124, 32, 109, 108, 95, 112, 111, 115, 105, 116, 105, 111, 110, 32, 97, 115, 32, 102, 54, 52, 41, 32, 125]))
                                } else {
                                    if (operation == (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112]))) {
                                        ((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46, 102, 108, 97, 116, 95, 109, 97, 112, 40, 124, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 95, 62, 62, 40, 41, 32, 125]))
                                    } else {
                                        if ((operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 116, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100])))) {
                                            {
                                                let predicate = self::render_array_callback_call(operation.clone(), parameter_count, suffix.clone(), true);
                                                if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 116, 101, 114]))) {
                                                    ((((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46, 102, 105, 108, 116, 101, 114, 95, 109, 97, 112, 40, 124, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32, 105, 102, 32]))).concat(&predicate)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 78, 111, 110, 101, 32, 125, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 95, 62, 62, 40, 41, 32, 125]))
                                                } else {
                                                    ((((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46, 102, 105, 110, 100, 95, 109, 97, 112, 40, 124, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32, 105, 102, 32]))).concat(&predicate)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 78, 111, 110, 101, 32, 125, 41, 32, 125]))
                                                }
                                            }
                                        } else {
                                            if (operation == (self::ml_string::Text::from_units(vec![102, 111, 114, 69, 97, 99, 104]))) {
                                                ((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32, 102, 111, 114, 32, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 32, 105, 110, 32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![59, 32, 125, 32, 40, 41, 32, 125]))
                                            } else {
                                                if (operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) {
                                                    if !has_initial {
                                                        ((((((((((((((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32, 108, 101, 116, 32, 109, 117, 116, 32, 109, 108, 95, 105, 116, 101, 114, 97, 116, 111, 114]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 105, 110, 105, 116, 105, 97, 108]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 109, 108, 95, 105, 116, 101, 114, 97, 116, 111, 114]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![46, 110, 101, 120, 116, 40, 41, 46, 109, 97, 112, 40, 124, 40, 95, 44, 32, 109, 108, 95, 102, 105, 114, 115, 116, 41, 124, 32, 109, 108, 95, 102, 105, 114, 115, 116, 41, 46, 101, 120, 112, 101, 99, 116, 40, 34, 82, 101, 100, 117, 99, 101, 32, 111, 102, 32, 101, 109, 112, 116, 121, 32, 97, 114, 114, 97, 121, 32, 119, 105, 116, 104, 32, 110, 111, 32, 105, 110, 105, 116, 105, 97, 108, 32, 118, 97, 108, 117, 101, 34, 41, 59, 32, 109, 108, 95, 105, 116, 101, 114, 97, 116, 111, 114]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![46, 102, 111, 108, 100, 40, 109, 108, 95, 105, 110, 105, 116, 105, 97, 108]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![44, 32, 124, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![44, 32, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125]))
                                                    } else {
                                                        ((((((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46, 102, 111, 108, 100, 40]))).concat(&initial)).concat(&self::ml_string::Text::from_units(vec![44, 32, 124, 109, 108, 95, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![44, 32, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125]))
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
                    }
                }
            }
        }
    }
}

pub fn read_array_callback_argument_count_error(operation: self::ml_string::Text, count: f64) -> self::ml_string::Text {
    {
        let maximum = self::read_array_callback_maximum_arguments(operation.clone());
        if ((maximum < (0f64)) || ((count >= (1f64)) && (count <= maximum))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let expected = if (maximum == (1f64)) {
                    self::ml_string::Text::from_units(vec![49])
                } else {
                    self::ml_string::Text::from_units(vec![49, 32, 116, 111, 32, 50])
                };
                ((((self::ml_string::Text::from_units(vec![46])).concat(&operation)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 101, 120, 112, 101, 99, 116, 115, 32]))).concat(&expected)).concat(&self::ml_string::Text::from_units(vec![32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
            }
        }
    }
}

pub fn read_array_callback_checked_result_context(operation: self::ml_string::Text) -> self::ml_string::Text {
    {
        let expected = self::read_array_callback_expected_result_form(operation.clone());
        if (expected == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            expected.clone()
        }
    }
}

pub fn read_array_callback_result_error(operation: self::ml_string::Text, result_kind: self::ml_string::Text) -> self::ml_string::Text {
    if ((operation == (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112]))) && (result_kind != (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) {
        self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 109, 117, 115, 116, 32, 114, 101, 116, 117, 114, 110, 32, 97, 110, 32, 97, 114, 114, 97, 121])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

pub fn read_array_callback_checked_argument_position(operation: self::ml_string::Text, has_initial: bool, position: f64) -> f64 {
    if ((operation == (self::ml_string::Text::from_units(vec![114, 101, 100, 117, 99, 101]))) && has_initial) {
        if (position == (0f64)) {
            1f64
        } else {
            if (position == (1f64)) {
                0f64
            } else {
                (-1f64)
            }
        }
    } else {
        if (position == (0f64)) {
            0f64
        } else {
            (-1f64)
        }
    }
}

pub fn render_array_callback_reference_expression(operation: self::ml_string::Text, object: self::ml_string::Text, callback: self::ml_string::Text, parameter_count: f64, suffix: self::ml_string::Text, reference_result: bool) -> self::ml_string::Text {
    if ((operation != (self::ml_string::Text::from_units(vec![102, 108, 97, 116, 77, 97, 112]))) || !reference_result) {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let values = (self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101, 115])).concat(&suffix);
            {
                let index = (self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 100, 101, 120])).concat(&suffix);
                {
                    let value = (self::ml_string::Text::from_units(vec![109, 108, 95, 118, 97, 108, 117, 101])).concat(&suffix);
                    {
                        let result = (self::ml_string::Text::from_units(vec![109, 108, 95, 99, 97, 108, 108, 98, 97, 99, 107, 95, 118, 97, 108, 117, 101, 115])).concat(&suffix);
                        {
                            let call = self::render_array_callback_call(operation.clone(), parameter_count, suffix.clone(), false);
                            {
                                let head = ((((((((self::ml_string::Text::from_units(vec![108, 101, 116, 32])).concat(&values)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&object)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 99, 97, 108, 108, 98, 97, 99, 107]))).concat(&suffix)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&callback)).concat(&self::ml_string::Text::from_units(vec![59]));
                                {
                                    let iterator = values.concat(&self::ml_string::Text::from_units(vec![46, 105, 116, 101, 114, 40, 41, 46, 99, 108, 111, 110, 101, 100, 40, 41, 46, 101, 110, 117, 109, 101, 114, 97, 116, 101, 40, 41]));
                                    ((((((((((((((self::ml_string::Text::from_units(vec![123, 32])).concat(&head)).concat(&self::ml_string::Text::from_units(vec![32, 115, 116, 100, 58, 58, 114, 99, 58, 58, 82, 99, 58, 58, 110, 101, 119, 40, 115, 116, 100, 58, 58, 99, 101, 108, 108, 58, 58, 82, 101, 102, 67, 101, 108, 108, 58, 58, 110, 101, 119, 40]))).concat(&iterator)).concat(&self::ml_string::Text::from_units(vec![46, 102, 108, 97, 116, 95, 109, 97, 112, 40, 124, 40]))).concat(&index)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 124, 32, 123, 32, 108, 101, 116, 32]))).concat(&result)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32]))).concat(&call)).concat(&self::ml_string::Text::from_units(vec![46, 98, 111, 114, 114, 111, 119, 40, 41, 46, 99, 108, 111, 110, 101, 40, 41, 59, 32]))).concat(&result)).concat(&self::ml_string::Text::from_units(vec![32, 125, 41, 46, 99, 111, 108, 108, 101, 99, 116, 58, 58, 60, 86, 101, 99, 60, 95, 62, 62, 40, 41, 41, 41, 32, 125]))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_array_state_method_result(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101]))) {
        self::ml_string::Text::from_units(vec![114, 101, 109, 111, 118, 101, 100, 32, 97, 114, 114, 97, 121])
    } else {
        if ((operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110])))) {
            self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn accept_linked_array_state_method(expression: self::ml_string::Text, operation: self::ml_string::Text, kind: self::ml_string::Text, reference: bool) -> bool {
    ((((expression == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) && (kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))) && reference) && ((self::read_array_state_method_result(operation.clone())) != (self::ml_string::Text::from_units(vec![]))))
}

pub fn read_array_state_method_required_arguments(operation: self::ml_string::Text) -> f64 {
    if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) {
        1f64
    } else {
        0f64
    }
}

pub fn read_array_state_method_argument_role(operation: self::ml_string::Text, position: f64) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101]))) {
        if (position < (2f64)) {
            self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 105, 110, 100, 101, 120])
        } else {
            self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
        }
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) {
            if (position == (0f64)) {
                self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])
            } else {
                if (position == (1f64)) {
                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 105, 110, 100, 101, 120])
                } else {
                    if (position == (2f64)) {
                        self::ml_string::Text::from_units(vec![101, 110, 100, 32, 105, 110, 100, 101, 120])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110]))) {
                if (position < (2f64)) {
                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 105, 110, 100, 101, 120])
                } else {
                    if (position == (2f64)) {
                        self::ml_string::Text::from_units(vec![101, 110, 100, 32, 105, 110, 100, 101, 120])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_array_state_method_argument_profile(kind: self::ml_string::Text, element: self::ml_string::Text, absence: self::ml_string::Text) -> self::ml_string::Text {
    if (((kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || (kind == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))) || (kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))) {
        kind.clone()
    } else {
        if (((kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && (((element == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || (element == (self::ml_string::Text::from_units(vec![98, 111, 111, 108])))) || (element == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))) && (((absence == (self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))) || (absence == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))) || (absence == (self::ml_string::Text::from_units(vec![98, 111, 116, 104]))))) {
            (((self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 32])).concat(&absence)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&element)
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn read_array_state_method_suffix(initial: f64, receiver: self::ml_string::Text, arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> f64 {
    {
        let text = (receiver.concat(&self::ml_string::Text::from_units(vec![32]))).concat(&self::ml_string::join(&{ let ml_reference_value9 = arguments_.borrow().clone(); ml_reference_value9 }, &self::ml_string::Text::from_units(vec![32])));
        {
            let suffix = initial;
            {
                let suffix_2 = self::source_ml_source1_temporary_read_array_state_method_suffix_loop9(text.clone(), suffix);
                suffix_2
            }
        }
    }
}

pub fn render_array_state_numeric_value(profile: self::ml_string::Text, value: self::ml_string::Text, undefined_value: self::ml_string::Text) -> self::ml_string::Text {
    if (profile == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
        value.clone()
    } else {
        if (profile == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
            ((self::ml_string::Text::from_units(vec![105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 49, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 48, 102, 54, 52, 32, 125]))
        } else {
            if (profile == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) {
                undefined_value.clone()
            } else {
                {
                    let parts = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source2_string_unit_operand12 = profile.clone();
                        {
                            let source_ml_source2_string_unit_operand13 = self::ml_string::Text::from_units(vec![32]);
                            {
                                let source_ml_source2_string_split_limit14 = 4294967295f64;
                                {
                                    let source_ml_source2_string_split_separator15 = source_ml_source2_string_unit_operand13.clone();
                                    (|ml_text:&self::ml_string::Text,ml_separator:Option<&self::ml_string::Text>,ml_limit:f64|{self::ml_utf16_text::split_utf16_text(&(ml_text.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()), &(ml_separator.map(|ml_value|ml_value.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()).unwrap_or_default()), ml_limit, ml_separator.is_none()).into_iter().map(|ml_units|self::ml_string::Text::from_units(ml_units.into_iter().map(|ml_unit|ml_unit as u16).collect())).collect::<Vec<_>>()})(&(source_ml_source2_string_unit_operand12), Some(&source_ml_source2_string_split_separator15), source_ml_source2_string_split_limit14)
                                }
                            }
                        }
                    }));
                    {
                        let absence = ({
                            let source_ml_source2_array_read_value16 = parts.clone();
                            {
                                let source_ml_source2_array_read_index17 = 1f64;
                                { let ml_reference_array10 = source_ml_source2_array_read_value16.clone(); let ml_reference_position10 = source_ml_source2_array_read_index17; let ml_reference_projection10 = self::ml_array::optional_at(&*ml_reference_array10.borrow(), self::ml_array::number_index(ml_reference_position10)); ml_reference_projection10 }
                            }
                        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        {
                            let element = ({
                                let source_ml_source2_array_read_value18 = parts.clone();
                                {
                                    let source_ml_source2_array_read_index19 = 2f64;
                                    { let ml_reference_array11 = source_ml_source2_array_read_value18.clone(); let ml_reference_position11 = source_ml_source2_array_read_index19; let ml_reference_projection11 = self::ml_array::optional_at(&*ml_reference_array11.borrow(), self::ml_array::number_index(ml_reference_position11)); ml_reference_projection11 }
                                }
                            }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                            {
                                let present = if (element == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
                                    self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101, 110, 116])
                                } else {
                                    if (element == (self::ml_string::Text::from_units(vec![98, 111, 111, 108]))) {
                                        self::ml_string::Text::from_units(vec![105, 102, 32, 109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101, 110, 116, 32, 123, 32, 49, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 48, 102, 54, 52, 32, 125])
                                    } else {
                                        self::ml_string::Text::from_units(vec![102, 54, 52, 58, 58, 78, 65, 78])
                                    }
                                };
                                if (absence == (self::ml_string::Text::from_units(vec![98, 111, 116, 104]))) {
                                    ((((((self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 99, 114, 97, 116, 101, 58, 58, 77, 108, 79, 112, 116, 105, 111, 110, 97, 108, 58, 58, 86, 97, 108, 117, 101, 40, 109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101, 110, 116, 41, 32, 61, 62, 32]))).concat(&present)).concat(&self::ml_string::Text::from_units(vec![44, 32, 99, 114, 97, 116, 101, 58, 58, 77, 108, 79, 112, 116, 105, 111, 110, 97, 108, 58, 58, 78, 117, 108, 108, 32, 61, 62, 32, 48, 102, 54, 52, 44, 32, 99, 114, 97, 116, 101, 58, 58, 77, 108, 79, 112, 116, 105, 111, 110, 97, 108, 58, 58, 85, 110, 100, 101, 102, 105, 110, 101, 100, 32, 61, 62, 32]))).concat(&undefined_value)).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                } else {
                                    ((((((self::ml_string::Text::from_units(vec![109, 97, 116, 99, 104, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 123, 32, 83, 111, 109, 101, 40, 109, 108, 95, 105, 110, 100, 101, 120, 95, 112, 114, 101, 115, 101, 110, 116, 41, 32, 61, 62, 32]))).concat(&present)).concat(&self::ml_string::Text::from_units(vec![44, 32, 78, 111, 110, 101, 32, 61, 62, 32]))).concat(&if (absence == (self::ml_string::Text::from_units(vec![110, 117, 108, 108]))) {
                                        self::ml_string::Text::from_units(vec![48, 102, 54, 52])
                                    } else {
                                        undefined_value.clone()
                                    })).concat(&self::ml_string::Text::from_units(vec![32, 125]))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn render_array_state_relative_index(value: self::ml_string::Text, length: self::ml_string::Text) -> self::ml_string::Text {
    (((((self::ml_string::Text::from_units(vec![123, 32, 108, 101, 116, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 95, 118, 97, 108, 117, 101, 58, 32, 102, 54, 52, 32, 61, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 32, 61, 32, 105, 102, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 95, 118, 97, 108, 117, 101, 46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 123, 32, 48, 102, 54, 52, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 95, 118, 97, 108, 117, 101, 46, 116, 114, 117, 110, 99, 40, 41, 32, 125, 59, 32]))).concat(&self::ml_string::Text::from_units(vec![108, 101, 116, 32, 109, 108, 95, 98, 111, 117, 110, 100, 32, 61, 32]))).concat(&length)).concat(&self::ml_string::Text::from_units(vec![32, 97, 115, 32, 102, 54, 52, 59, 32, 40, 105, 102, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 32, 60, 32, 48, 102, 54, 52, 32, 123, 32, 40, 109, 108, 95, 98, 111, 117, 110, 100, 32, 43, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 41, 46, 99, 108, 97, 109, 112, 40, 48, 102, 54, 52, 44, 32, 109, 108, 95, 98, 111, 117, 110, 100, 41, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32, 109, 108, 95, 114, 101, 108, 97, 116, 105, 118, 101, 46, 109, 105, 110, 40, 109, 108, 95, 98, 111, 117, 110, 100, 41, 32, 125, 41, 32, 97, 115, 32, 117, 115, 105, 122, 101, 32, 125]))
}

pub fn read_array_state_method_identity(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![115, 112, 108, 105, 99, 101]))) {
        self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![102, 105, 108, 108]))) {
            self::ml_string::Text::from_units(vec![102, 105, 108, 108, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115])
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 87, 105, 116, 104, 105, 110]))) {
                self::ml_string::Text::from_units(vec![99, 111, 112, 121, 45, 97, 114, 114, 97, 121, 45, 101, 108, 101, 109, 101, 110, 116, 115, 45, 119, 105, 116, 104, 105, 110])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

pub fn read_array_state_method_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![68, 101, 110, 115, 101, 32, 115, 104, 97, 114, 101, 100, 32, 97, 114, 114, 97, 121, 32, 114, 97, 110, 103, 101, 32, 109, 117, 116, 97, 116, 105, 111, 110, 115, 32, 101, 118, 97, 108, 117, 97, 116, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 101, 118, 101, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 111, 110, 99, 101, 32, 105, 110, 32, 111, 114, 100, 101, 114, 44, 32, 100, 105, 115, 116, 105, 110, 103, 117, 105, 115, 104, 32, 109, 105, 115, 115, 105, 110, 103, 32, 97, 110, 100, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 44, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 32, 78, 117, 109, 98, 101, 114, 32, 105, 110, 100, 105, 99, 101, 115, 32, 119, 105, 116, 104, 111, 117, 116, 32, 110, 97, 114, 114, 111, 119, 105, 110, 103, 32, 98, 101, 102, 111, 114, 101, 32, 98, 111, 117, 110, 100, 115, 44, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 97, 110, 100, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 101, 108, 101, 109, 101, 110, 116, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 115, 44, 32, 97, 110, 100, 32, 99, 111, 112, 121, 32, 111, 118, 101, 114, 108, 97, 112, 112, 105, 110, 103, 32, 114, 97, 110, 103, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 114, 101, 113, 117, 105, 114, 101, 100, 32, 100, 105, 114, 101, 99, 116, 105, 111, 110, 59, 32, 115, 112, 108, 105, 99, 101, 32, 114, 101, 116, 117, 114, 110, 115, 32, 97, 32, 102, 114, 101, 115, 104, 32, 114, 101, 109, 111, 118, 101, 100, 32, 97, 114, 114, 97, 121, 46])
}
