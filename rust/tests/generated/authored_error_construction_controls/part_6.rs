pub fn read_java_script_numeric_operation_failure(operation: self::ml_string::Text, left: self::ml_string::Text, right: self::ml_string::Text) -> self::ml_string::Text {
    {
        let first = ((left == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (left == (self::ml_string::Text::from_units(vec![110, 97, 116]))));
        {
            let second = ((right == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (right == (self::ml_string::Text::from_units(vec![110, 97, 116]))));
            if ((first && (right == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))) || ((left == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) && second)) {
                self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100])
            } else {
                if ((first && second) && (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116, 85, 110, 115, 105, 103, 110, 101, 100])))) {
                    self::ml_string::Text::from_units(vec![117, 110, 115, 105, 103, 110, 101, 100])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn read_java_script_numeric_operation_message(failure: self::ml_string::Text) -> self::ml_string::Text {
    if (failure == (self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100]))) {
        self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 109, 105, 120, 32, 66, 105, 103, 73, 110, 116, 32, 97, 110, 100, 32, 111, 116, 104, 101, 114, 32, 116, 121, 112, 101, 115, 44, 32, 117, 115, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 115])
    } else {
        if (failure == (self::ml_string::Text::from_units(vec![117, 110, 115, 105, 103, 110, 101, 100]))) {
            self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 115, 32, 104, 97, 118, 101, 32, 110, 111, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 114, 105, 103, 104, 116, 32, 115, 104, 105, 102, 116, 44, 32, 117, 115, 101, 32, 62, 62, 32, 105, 110, 115, 116, 101, 97, 100])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn test_java_script_error_constructor_mode(mode: self::ml_string::Text) -> bool {
    (mode == (self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114])))
}

pub fn test_java_script_error_intrinsic_mode(mode: self::ml_string::Text) -> bool {
    (mode == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99])))
}

pub fn read_java_script_error_construction_kind_error(error_class: self::ml_string::Text, bind_receiver: bool, mode: self::ml_string::Text) -> self::ml_string::Text {
    if ((self::read_java_script_source_exception_operation(error_class.clone())) == (self::ml_string::Text::from_units(vec![]))) {
        self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 118, 97, 108, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 108, 97, 115, 115])
    } else {
        if (bind_receiver && !(self::test_java_script_error_constructor_mode(mode.clone()))) {
            self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 114, 105, 118, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 98, 105, 110, 100, 115, 32, 116, 104, 101, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn read_java_script_error_allocation_directory(heap: self::JavaScriptOrdinaryHeap) -> self::JavaScriptErrorAllocationDirectory {
    self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values135 = { let ml_reference_value136 = (match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value597, _, _, _, _, _) => {
            source_ml_source2_data_property_value597.clone()
        }
    }).borrow().clone(); ml_reference_value136 }; let ml_callback135 = std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptErrorAllocationIdentity { self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(match object.clone() {
        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value598, _, _, _, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value598.clone()
        }
        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value599, _, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value599.clone()
        }
        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value600, _, _, _, _) => {
            source_ml_source2_data_property_value600.clone()
        }
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value601, _, _, _, _, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value601.clone()
        }
        self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value602, _, _, _, _, _) => {
            source_ml_source2_data_property_value602.clone()
        }
        self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value603, _, _, _, _, _) => {
            source_ml_source2_data_property_value603.clone()
        }
        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value604, _, _, _, _, _) => {
            source_ml_source2_data_property_value604.clone()
        }
        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value605, _, _, _, _, _) => {
            source_ml_source2_data_property_value605.clone()
        }
        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value606, _, _, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value606.clone()
        }
        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value607, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value607.clone()
        }
        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value608, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value608.clone()
        }
        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value609, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value609.clone()
        }
        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value610, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value610.clone()
        }
    }, match object.clone() {
        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
        }
        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
        }
    }, match object.clone() {
        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value611, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value611
        }
        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value612, _, _, _, _) => {
            source_ml_source2_data_property_value612
        }
        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value613, _) => {
            source_ml_source2_data_property_value613
        }
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value614, _, _, _, _, _, _, _) => {
            source_ml_source2_data_property_value614
        }
        self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value615, _, _) => {
            source_ml_source2_data_property_value615
        }
        self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value616, _, _) => {
            source_ml_source2_data_property_value616
        }
        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value617, _, _) => {
            source_ml_source2_data_property_value617
        }
        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value618, _, _) => {
            source_ml_source2_data_property_value618
        }
        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value619, _) => {
            source_ml_source2_data_property_value619
        }
        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value620, _, _, _) => {
            source_ml_source2_data_property_value620
        }
        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value621, _, _, _) => {
            source_ml_source2_data_property_value621
        }
        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value622, _, _, _) => {
            source_ml_source2_data_property_value622
        }
        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value623, _, _, _) => {
            source_ml_source2_data_property_value623
        }
    }) }); ml_values135.iter().cloned().enumerate().map(|(ml_index135, ml_value135)| ml_callback135(ml_value135)).collect::<Vec<_>>() })), std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(Vec::<self::ml::Big>::new(), { let ml_reference_value137 = (match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value624, _, _, _, _) => {
            source_ml_source2_data_property_value624.clone()
        }
    }).borrow().clone(); ml_reference_value137 }))))
}

pub fn read_java_script_error_prior_directory_error(heap: self::JavaScriptOrdinaryHeap, before: self::JavaScriptErrorAllocationDirectory) -> self::ml_string::Text {
    if ((({ let ml_reference_array138 = match before.clone() {
        self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(source_ml_source2_data_property_value626, _) => {
            source_ml_source2_data_property_value626.clone()
        }
    }; let ml_reference_projection138 = ml_reference_array138.borrow().len() as f64; ml_reference_projection138 }) > (4096f64)) || (({ let ml_reference_array139 = match before.clone() {
        self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(_, source_ml_source2_data_property_value628) => {
            source_ml_source2_data_property_value628.clone()
        }
    }; let ml_reference_projection139 = ml_reference_array139.borrow().len() as f64; ml_reference_projection139 }) > (16384f64))) {
        self::ml_string::Text::from_units(vec![116, 104, 101, 32, 112, 114, 105, 111, 114, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116])
    } else {
        {
            let previous = self::ml::Big::from_i128(0);
            {
                let source_ml_source1_temporary_values10 = match before.clone() {
                    self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(source_ml_source2_data_property_value629, _) => {
                        source_ml_source2_data_property_value629.clone()
                    }
                };
                {
                    let source_ml_source1_temporary_index10 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s630 = self::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop12(heap.clone(), previous.clone(), source_ml_source1_temporary_values10.clone(), source_ml_source1_temporary_index10.clone());
                        match source_ml_source2_s630.clone() {
                            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Done(previous_2) => {
                                {
                                    let previous_3 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_temporary_values11 = match before.clone() {
                                            self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(_, source_ml_source2_data_property_value631) => {
                                                source_ml_source2_data_property_value631.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_index11 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s632 = self::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop13(heap.clone(), before.clone(), previous_3.clone(), source_ml_source1_temporary_values11.clone(), source_ml_source1_temporary_index11.clone());
                                                match source_ml_source2_s632.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Done => {
                                                        self::ml_string::Text::from_units(vec![])
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(source_ml_source1_temporary_result13) => {
                                                        source_ml_source1_temporary_result13.clone()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(source_ml_source1_temporary_result13_2) => {
                                source_ml_source1_temporary_result13_2.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_error_construction_operands_error(heap: self::JavaScriptOrdinaryHeap, constructor: self::JavaScriptSemanticValue, new_target: self::JavaScriptSemanticValue, prototype: self::JavaScriptSemanticValue, message: self::JavaScriptSemanticValue, present: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    {
        let source_ml_source1_temporary_values14 = std::rc::Rc::new(std::cell::RefCell::new(vec![constructor.clone(), new_target.clone(), prototype.clone(), message.clone(), present.clone()]));
        {
            let source_ml_source1_temporary_index14 = self::ml::Big::from_i128(0);
            {
                let source_ml_source2_s633 = self::source_ml_source1_temporary_read_java_script_error_construction_operands_error_loop15(heap.clone(), source_ml_source1_temporary_values14.clone(), source_ml_source1_temporary_index14.clone());
                match source_ml_source2_s633.clone() {
                    self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Done => {
                        {
                            let invalid_constructor = self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]);
                            if (match constructor.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f634) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source2_f635) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f636) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source2_f637) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f638, source_ml_source2_f639) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f640) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source2_f641, source_ml_source2_f642) => {
                                    false
                                }
                            } || !(((match constructor.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    None::<(bool,)>
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    None::<(bool,)>
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    None::<(bool,)>
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    None::<(bool,)>
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    None::<(bool,)>
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    None::<(bool,)>
                                }
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    None::<(bool,)>
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    None::<(bool,)>
                                }
                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value643) => {
                                    Some((source_ml_source2_data_property_value643,))
                                }
                            }).clone().expect("a proven present value")).0)) {
                                invalid_constructor.clone()
                            } else {
                                match new_target.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f644) => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Number(source_ml_source2_f645) => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f646) => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f647) => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f648, source_ml_source2_f649) => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f650) => {
                                        invalid_constructor.clone()
                                    }
                                    self::JavaScriptSemanticValue::Object(identity, callable) => {
                                        if !callable {
                                            invalid_constructor.clone()
                                        } else {
                                            if (match prototype.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f651) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source2_f652) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f653) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source2_f654) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f655, source_ml_source2_f656) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f657) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source2_f658, source_ml_source2_f659) => {
                                                    false
                                                }
                                            } || ((match prototype.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    None::<(bool,)>
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    None::<(bool,)>
                                                }
                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                    None::<(bool,)>
                                                }
                                                self::JavaScriptSemanticValue::Number(_) => {
                                                    None::<(bool,)>
                                                }
                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                    None::<(bool,)>
                                                }
                                                self::JavaScriptSemanticValue::Text(_) => {
                                                    None::<(bool,)>
                                                }
                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                    None::<(bool,)>
                                                }
                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                    None::<(bool,)>
                                                }
                                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value660) => {
                                                    Some((source_ml_source2_data_property_value660,))
                                                }
                                            }).clone().expect("a proven present value")).0) {
                                                self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 69, 114, 114, 111, 114, 32, 115, 117, 98, 99, 108, 97, 115, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 79, 98, 106, 101, 99, 116])
                                            } else {
                                                if ((match message.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f661) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Number(source_ml_source2_f662) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f663) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f664) => {
                                                        false
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f665, source_ml_source2_f666) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f667) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f668, source_ml_source2_f669) => {
                                                        true
                                                    }
                                                } && match message.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f670) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Number(source_ml_source2_f671) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f672) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f673) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f674, source_ml_source2_f675) => {
                                                        false
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f676) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f677, source_ml_source2_f678) => {
                                                        true
                                                    }
                                                }) || match present.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f679) => {
                                                        false
                                                    }
                                                    self::JavaScriptSemanticValue::Number(source_ml_source2_f680) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f681) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f682) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f683, source_ml_source2_f684) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f685) => {
                                                        true
                                                    }
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f686, source_ml_source2_f687) => {
                                                        true
                                                    }
                                                }) {
                                                    self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 85, 84, 70, 49, 54, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 105, 110, 100, 101, 112, 101, 110, 100, 101, 110, 116, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101])
                                                } else {
                                                    {
                                                        let object = {
                                                            let source_ml_source2_array_read_value690 = match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value688, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value688.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_array_read_index691 = self::find_java_script_heap_directory_object(match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value689, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value689.clone()
                                                                    }
                                                                }, identity.clone());
                                                                { let ml_reference_array140 = source_ml_source2_array_read_value690.clone(); let ml_reference_position140 = source_ml_source2_array_read_index691; let ml_reference_projection140 = self::ml_array::optional_at(&*ml_reference_array140.borrow(), self::ml_array::number_index(ml_reference_position140)); ml_reference_projection140 }
                                                            }
                                                        };
                                                        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::ml_string::Text::from_units(vec![110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 104, 101, 97, 112])
                                                        } else {
                                                            {
                                                                let property = {
                                                                    let source_ml_source2_array_read_value718 = match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value692, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value692.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value693, _, _, _) => {
                                                                            source_ml_source2_data_property_value693.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value694) => {
                                                                            source_ml_source2_data_property_value694.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value695, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value695.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value696, _) => {
                                                                            source_ml_source2_data_property_value696.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value697, _) => {
                                                                            source_ml_source2_data_property_value697.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value698, _) => {
                                                                            source_ml_source2_data_property_value698.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value699, _) => {
                                                                            source_ml_source2_data_property_value699.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value700) => {
                                                                            source_ml_source2_data_property_value700.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value701, _, _) => {
                                                                            source_ml_source2_data_property_value701.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value702, _, _) => {
                                                                            source_ml_source2_data_property_value702.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value703, _, _) => {
                                                                            source_ml_source2_data_property_value703.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value704, _, _) => {
                                                                            source_ml_source2_data_property_value704.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_array_read_index719 = self::find_java_script_heap_directory_property(match object.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value705, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value705.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value706, _, _, _) => {
                                                                                source_ml_source2_data_property_value706.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value707) => {
                                                                                source_ml_source2_data_property_value707.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value708, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value708.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value709, _) => {
                                                                                source_ml_source2_data_property_value709.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value710, _) => {
                                                                                source_ml_source2_data_property_value710.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value711, _) => {
                                                                                source_ml_source2_data_property_value711.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value712, _) => {
                                                                                source_ml_source2_data_property_value712.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value713) => {
                                                                                source_ml_source2_data_property_value713.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value714, _, _) => {
                                                                                source_ml_source2_data_property_value714.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value715, _, _) => {
                                                                                source_ml_source2_data_property_value715.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value716, _, _) => {
                                                                                source_ml_source2_data_property_value716.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value717, _, _) => {
                                                                                source_ml_source2_data_property_value717.clone()
                                                                            }
                                                                        }, self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![112f64, 114f64, 111f64, 116f64, 111f64, 116f64, 121f64, 112f64, 101f64]))));
                                                                        { let ml_reference_array141 = source_ml_source2_array_read_value718.clone(); let ml_reference_position141 = source_ml_source2_array_read_index719; let ml_reference_projection141 = self::ml_array::optional_at(&*ml_reference_array141.borrow(), self::ml_array::number_index(ml_reference_position141)); ml_reference_projection141 }
                                                                    }
                                                                };
                                                                if ((match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(({ let ml_reference_value142 = (match property.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value720) => {
                                                                        source_ml_source2_data_property_value720.clone()
                                                                    }
                                                                }).borrow().clone(); ml_reference_value142 }).9)) || !(self::test_java_script_semantic_same_value(({ let ml_reference_value143 = (match property.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value721) => {
                                                                        source_ml_source2_data_property_value721.clone()
                                                                    }
                                                                }).borrow().clone(); ml_reference_value143 }).8, prototype.clone()))) {
                                                                    self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 112, 114, 111, 102, 105, 108, 101, 32, 110, 101, 101, 100, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 111, 119, 110, 32, 100, 97, 116, 97, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 97, 110, 32, 105, 110, 118, 101, 110, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 111, 114, 32, 103, 101, 116, 116, 101, 114])
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
                    self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Return(source_ml_source1_temporary_result15) => {
                        source_ml_source1_temporary_result15.clone()
                    }
                }
            }
        }
    }
}

pub fn read_java_script_error_allocation_result_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue, prototype: self::JavaScriptSemanticValue, before: self::JavaScriptErrorAllocationDirectory) -> self::ml_string::Text {
    {
        let prior = self::read_java_script_error_prior_directory_error(heap.clone(), before.clone());
        if (prior != (self::ml_string::Text::from_units(vec![]))) {
            prior.clone()
        } else {
            {
                let error = self::read_java_script_heap_directory_value_error(heap.clone(), value.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    {
                        let invalid_result = self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 103, 108, 111, 98, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                        match value.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Null => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f722) => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source2_f723) => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source2_f724) => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source2_f725) => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f726, source_ml_source2_f727) => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f728) => {
                                invalid_result.clone()
                            }
                            self::JavaScriptSemanticValue::Object(identity, callable) => {
                                if ((callable || { let ml_values144 = { let ml_reference_value145 = (match before.clone() {
                                    self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(source_ml_source2_data_property_value729, _) => {
                                        source_ml_source2_data_property_value729.clone()
                                    }
                                }).borrow().clone(); ml_reference_value145 }; let ml_callback144 = { let identity = identity.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorAllocationIdentity| -> bool { ((match row.clone() {
                                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value731, _, _) => {
                                        source_ml_source2_data_property_value731.clone()
                                    }
                                }) == identity) }) }; ml_values144.iter().cloned().enumerate().any(|(ml_index144, ml_value144)| ml_callback144(ml_value144)) }) || self::ml_array::includes(&{ let ml_reference_value146 = (match before.clone() {
                                    self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(_, source_ml_source2_data_property_value733) => {
                                        source_ml_source2_data_property_value733.clone()
                                    }
                                }).borrow().clone(); ml_reference_value146 }, &identity.clone(), 0f64, false, |a, b| a == b)) {
                                    invalid_result.clone()
                                } else {
                                    {
                                        let object = {
                                            let source_ml_source2_array_read_value736 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value734, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value734.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_array_read_index737 = self::find_java_script_heap_directory_object(match heap.clone() {
                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value735, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value735.clone()
                                                    }
                                                }, identity.clone());
                                                { let ml_reference_array147 = source_ml_source2_array_read_value736.clone(); let ml_reference_position147 = source_ml_source2_array_read_index737; let ml_reference_projection147 = self::ml_array::optional_at(&*ml_reference_array147.borrow(), self::ml_array::number_index(ml_reference_position147)); ml_reference_projection147 }
                                            }
                                        };
                                        {
                                            let invalid_result_prototype = self::ml_string::Text::from_units(vec![116, 104, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 112, 97, 113, 117, 101, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 114, 105, 118, 101, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]);
                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                invalid_result_prototype.clone()
                                            } else {
                                                if (match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_f738, source_ml_source2_f739, source_ml_source2_f740, source_ml_source2_f741, source_ml_source2_f742, source_ml_source2_f743, source_ml_source2_f744, source_ml_source2_f745, source_ml_source2_f746, source_ml_source2_f747) => {
                                                        let source_ml_source2_f739 = *source_ml_source2_f739;
                                                        let source_ml_source2_f746 = *source_ml_source2_f746;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_f748, source_ml_source2_f749, source_ml_source2_f750, source_ml_source2_f751, source_ml_source2_f752, source_ml_source2_f753, source_ml_source2_f754, source_ml_source2_f755) => {
                                                        let source_ml_source2_f749 = *source_ml_source2_f749;
                                                        let source_ml_source2_f753 = *source_ml_source2_f753;
                                                        let source_ml_source2_f754 = *source_ml_source2_f754;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_f756, source_ml_source2_f757, source_ml_source2_f758, source_ml_source2_f759, source_ml_source2_f760) => {
                                                        let source_ml_source2_f757 = *source_ml_source2_f757;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_f761, source_ml_source2_f762, source_ml_source2_f763, source_ml_source2_f764, source_ml_source2_f765, source_ml_source2_f766, source_ml_source2_f767, source_ml_source2_f768, source_ml_source2_f769, source_ml_source2_f770, source_ml_source2_f771) => {
                                                        let source_ml_source2_f762 = *source_ml_source2_f762;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_f772, source_ml_source2_f773, source_ml_source2_f774, source_ml_source2_f775, source_ml_source2_f776, source_ml_source2_f777) => {
                                                        let source_ml_source2_f773 = *source_ml_source2_f773;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_f778, source_ml_source2_f779, source_ml_source2_f780, source_ml_source2_f781, source_ml_source2_f782, source_ml_source2_f783) => {
                                                        let source_ml_source2_f779 = *source_ml_source2_f779;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_f784, source_ml_source2_f785, source_ml_source2_f786, source_ml_source2_f787, source_ml_source2_f788, source_ml_source2_f789) => {
                                                        let source_ml_source2_f785 = *source_ml_source2_f785;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_f790, source_ml_source2_f791, source_ml_source2_f792, source_ml_source2_f793, source_ml_source2_f794, source_ml_source2_f795) => {
                                                        let source_ml_source2_f791 = *source_ml_source2_f791;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_f796, source_ml_source2_f797, source_ml_source2_f798, source_ml_source2_f799, source_ml_source2_f800, source_ml_source2_f801, source_ml_source2_f802, source_ml_source2_f803, source_ml_source2_f804) => {
                                                        let source_ml_source2_f797 = *source_ml_source2_f797;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_f805, source_ml_source2_f806, source_ml_source2_f807, source_ml_source2_f808, source_ml_source2_f809, source_ml_source2_f810, source_ml_source2_f811) => {
                                                        let source_ml_source2_f806 = *source_ml_source2_f806;
                                                        let source_ml_source2_f810 = *source_ml_source2_f810;
                                                        let source_ml_source2_f811 = *source_ml_source2_f811;
                                                        true
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_f812, source_ml_source2_f813, source_ml_source2_f814, source_ml_source2_f815, source_ml_source2_f816, source_ml_source2_f817, source_ml_source2_f818) => {
                                                        let source_ml_source2_f813 = *source_ml_source2_f813;
                                                        let source_ml_source2_f817 = *source_ml_source2_f817;
                                                        let source_ml_source2_f818 = *source_ml_source2_f818;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_f819, source_ml_source2_f820, source_ml_source2_f821, source_ml_source2_f822, source_ml_source2_f823, source_ml_source2_f824, source_ml_source2_f825) => {
                                                        let source_ml_source2_f820 = *source_ml_source2_f820;
                                                        let source_ml_source2_f824 = *source_ml_source2_f824;
                                                        let source_ml_source2_f825 = *source_ml_source2_f825;
                                                        false
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_f826, source_ml_source2_f827, source_ml_source2_f828, source_ml_source2_f829, source_ml_source2_f830, source_ml_source2_f831, source_ml_source2_f832) => {
                                                        let source_ml_source2_f827 = *source_ml_source2_f827;
                                                        let source_ml_source2_f831 = *source_ml_source2_f831;
                                                        let source_ml_source2_f832 = *source_ml_source2_f832;
                                                        false
                                                    }
                                                } && self::test_java_script_semantic_same_value((((match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value833, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value833 = *source_ml_source2_data_property_value833;
                                                        Some((source_ml_source2_data_property_value833.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                        None::<(self::JavaScriptSemanticValue,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone(), prototype.clone())) {
                                                    self::ml_string::Text::from_units(vec![])
                                                } else {
                                                    invalid_result_prototype.clone()
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ({ let ml_reference_array148 = units.clone(); let ml_reference_projection148 = ml_reference_array148.borrow().len() as f64; ml_reference_projection148 })) {
            {
                let unit = ({
                    let source_ml_source2_array_read_value834 = units.clone();
                    {
                        let source_ml_source2_array_read_index835 = at;
                        { let ml_reference_array149 = source_ml_source2_array_read_value834.clone(); let ml_reference_position149 = source_ml_source2_array_read_index835; let ml_reference_projection149 = self::ml_array::optional_at(&*ml_reference_array149.borrow(), self::ml_array::number_index(ml_reference_position149)); ml_reference_projection149 }
                    }
                }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Done,
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Return(bool),
}
