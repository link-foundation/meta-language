pub fn read_java_script_array_object_error(object: self::JavaScriptOrdinaryObject) -> self::ml_string::Text {
    if match object.clone() {
        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source1_data_property_value617, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value617
        }
        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source1_data_property_value618, _, _, _, _) => {
            source_ml_source1_data_property_value618
        }
        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source1_data_property_value619, _) => {
            source_ml_source1_data_property_value619
        }
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source1_data_property_value620, _, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value620
        }
        self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source1_data_property_value621, _, _) => {
            source_ml_source1_data_property_value621
        }
        self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source1_data_property_value622, _, _) => {
            source_ml_source1_data_property_value622
        }
        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source1_data_property_value623, _, _) => {
            source_ml_source1_data_property_value623
        }
        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source1_data_property_value624, _, _) => {
            source_ml_source1_data_property_value624
        }
        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source1_data_property_value625, _) => {
            source_ml_source1_data_property_value625
        }
        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source1_data_property_value626, _, _, _) => {
            source_ml_source1_data_property_value626
        }
        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source1_data_property_value627, _, _, _) => {
            source_ml_source1_data_property_value627
        }
        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source1_data_property_value628, _, _, _) => {
            source_ml_source1_data_property_value628
        }
        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source1_data_property_value629, _, _, _) => {
            source_ml_source1_data_property_value629
        }
    } {
        self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 101, 120, 111, 116, 105, 99, 32, 111, 98, 106, 101, 99, 116, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])
    } else {
        {
            let property = {
                let source_ml_source1_array_read_value656 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value630, _, _, _, _, _) => {
                        source_ml_source1_data_property_value630.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value631, _, _, _) => {
                        source_ml_source1_data_property_value631.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value632) => {
                        source_ml_source1_data_property_value632.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value633, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value633.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value634, _) => {
                        source_ml_source1_data_property_value634.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value635, _) => {
                        source_ml_source1_data_property_value635.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value636, _) => {
                        source_ml_source1_data_property_value636.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value637, _) => {
                        source_ml_source1_data_property_value637.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value638) => {
                        source_ml_source1_data_property_value638.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value639, _, _) => {
                        source_ml_source1_data_property_value639.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value640, _, _) => {
                        source_ml_source1_data_property_value640.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value641, _, _) => {
                        source_ml_source1_data_property_value641.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value642, _, _) => {
                        source_ml_source1_data_property_value642.clone()
                    }
                };
                {
                    let source_ml_source1_array_read_index657 = self::find_java_script_array_property(match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value643, _, _, _, _, _) => {
                            source_ml_source1_data_property_value643.clone()
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value644, _, _, _) => {
                            source_ml_source1_data_property_value644.clone()
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value645) => {
                            source_ml_source1_data_property_value645.clone()
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value646, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value646.clone()
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value647, _) => {
                            source_ml_source1_data_property_value647.clone()
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value648, _) => {
                            source_ml_source1_data_property_value648.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value649, _) => {
                            source_ml_source1_data_property_value649.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value650, _) => {
                            source_ml_source1_data_property_value650.clone()
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value651) => {
                            source_ml_source1_data_property_value651.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value652, _, _) => {
                            source_ml_source1_data_property_value652.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value653, _, _) => {
                            source_ml_source1_data_property_value653.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value654, _, _) => {
                            source_ml_source1_data_property_value654.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value655, _, _) => {
                            source_ml_source1_data_property_value655.clone()
                        }
                    }, self::read_java_script_array_length_key());
                    { let ml_reference_array131 = source_ml_source1_array_read_value656.clone(); let ml_reference_position131 = source_ml_source1_array_read_index657; let ml_reference_projection131 = self::ml_array::optional_at(&*ml_reference_array131.borrow(), self::ml_array::number_index(ml_reference_position131)); ml_reference_projection131 }
                }
            };
            {
                let invalid = self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 101, 110, 117, 109, 101, 114, 97, 98, 108, 101, 32, 110, 111, 110, 99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101, 32, 105, 110, 116, 101, 103, 114, 97, 108, 32, 108, 101, 110, 103, 116, 104, 32, 100, 97, 116, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121]);
                if (((((match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(({ let ml_reference_value132 = (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value658) => {
                        source_ml_source1_data_property_value658.clone()
                    }
                }).borrow().clone(); ml_reference_value132 }).9)) || ({ let ml_reference_value133 = (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value659) => {
                        source_ml_source1_data_property_value659.clone()
                    }
                }).borrow().clone(); ml_reference_value133 }).5) || ({ let ml_reference_value134 = (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value660) => {
                        source_ml_source1_data_property_value660.clone()
                    }
                }).borrow().clone(); ml_reference_value134 }).7) || ({ let ml_reference_value135 = (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value661) => {
                        source_ml_source1_data_property_value661.clone()
                    }
                }).borrow().clone(); ml_reference_value135 }).2) || ({ let ml_reference_value136 = (match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value662) => {
                        source_ml_source1_data_property_value662.clone()
                    }
                }).borrow().clone(); ml_reference_value136 }).0) {
                    invalid.clone()
                } else {
                    {
                        let length = self::read_java_script_array_length_value(({ let ml_reference_value137 = (match property.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value663) => {
                                source_ml_source1_data_property_value663.clone()
                            }
                        }).borrow().clone(); ml_reference_value137 }).8);
                        if !(self::test_java_script_array_length(length)) {
                            invalid.clone()
                        } else {
                            {
                                let source_ml_temporary_values12 = match object.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value664, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value664.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value665, _, _, _) => {
                                        source_ml_source1_data_property_value665.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value666) => {
                                        source_ml_source1_data_property_value666.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value667, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value667.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value668, _) => {
                                        source_ml_source1_data_property_value668.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value669, _) => {
                                        source_ml_source1_data_property_value669.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value670, _) => {
                                        source_ml_source1_data_property_value670.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value671, _) => {
                                        source_ml_source1_data_property_value671.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value672) => {
                                        source_ml_source1_data_property_value672.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value673, _, _) => {
                                        source_ml_source1_data_property_value673.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value674, _, _) => {
                                        source_ml_source1_data_property_value674.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value675, _, _) => {
                                        source_ml_source1_data_property_value675.clone()
                                    }
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value676, _, _) => {
                                        source_ml_source1_data_property_value676.clone()
                                    }
                                };
                                {
                                    let source_ml_temporary_index12 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s677 = self::source_ml_temporary_read_java_script_array_object_error_loop13(length, source_ml_temporary_values12.clone(), source_ml_temporary_index12.clone());
                                        match source_ml_source1_s677.clone() {
                                            self::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Done => {
                                                self::ml_string::Text::from_units(vec![])
                                            }
                                            self::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Return(source_ml_temporary_result13) => {
                                                source_ml_temporary_result13.clone()
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

pub fn construct_java_script_array_object(identity: self::ml::Big, prototype: self::JavaScriptSemanticValue, length: f64) -> self::JavaScriptOrdinaryObject {
    {
        let descriptor = self::create_java_script_property_descriptor(self::ml_string::Text::from_units(vec![100, 97, 116, 97]), true, false);
        {
            let source_ml_temporary_construct_java_script_array_object_written14 = { let ml_field_owner138 = descriptor.clone(); let ml_field_value138 = false; ml_field_owner138.borrow_mut().0 = ml_field_value138; () };
            {
                let source_ml_temporary_construct_java_script_array_object_written15 = { let ml_field_owner139 = descriptor.clone(); let ml_field_value139 = true; ml_field_owner139.borrow_mut().10 = ml_field_value139; () };
                {
                    let source_ml_temporary_construct_java_script_array_object_written16 = { let ml_field_owner140 = descriptor.clone(); let ml_field_value140 = self::JavaScriptSemanticValue::Number(length); ml_field_owner140.borrow_mut().8 = ml_field_value140; () };
                    self::JavaScriptOrdinaryObject::ArrayObject(identity.clone(), Box::new(prototype.clone()), true, false, std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(self::read_java_script_array_length_key()), descriptor.clone())])))
                }
            }
        }
    }
}

pub fn plan_java_script_array_ordinary_definition(object: self::JavaScriptOrdinaryObject, key: self::JavaScriptSemanticValue, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::JavaScriptArrayPropertyPlan {
    {
        let at = self::find_java_script_array_property(match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value678, _, _, _, _, _) => {
                source_ml_source1_data_property_value678.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value679, _, _, _) => {
                source_ml_source1_data_property_value679.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value680) => {
                source_ml_source1_data_property_value680.clone()
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value681, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value681.clone()
            }
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value682, _) => {
                source_ml_source1_data_property_value682.clone()
            }
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value683, _) => {
                source_ml_source1_data_property_value683.clone()
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value684, _) => {
                source_ml_source1_data_property_value684.clone()
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value685, _) => {
                source_ml_source1_data_property_value685.clone()
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value686) => {
                source_ml_source1_data_property_value686.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value687, _, _) => {
                source_ml_source1_data_property_value687.clone()
            }
            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value688, _, _) => {
                source_ml_source1_data_property_value688.clone()
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value689, _, _) => {
                source_ml_source1_data_property_value689.clone()
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value690, _, _) => {
                source_ml_source1_data_property_value690.clone()
            }
        }, key.clone());
        {
            let property = {
                let source_ml_source1_array_read_value704 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value691, _, _, _, _, _) => {
                        source_ml_source1_data_property_value691.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value692, _, _, _) => {
                        source_ml_source1_data_property_value692.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value693) => {
                        source_ml_source1_data_property_value693.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value694, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value694.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value695, _) => {
                        source_ml_source1_data_property_value695.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value696, _) => {
                        source_ml_source1_data_property_value696.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value697, _) => {
                        source_ml_source1_data_property_value697.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value698, _) => {
                        source_ml_source1_data_property_value698.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value699) => {
                        source_ml_source1_data_property_value699.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value700, _, _) => {
                        source_ml_source1_data_property_value700.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value701, _, _) => {
                        source_ml_source1_data_property_value701.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value702, _, _) => {
                        source_ml_source1_data_property_value702.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value703, _, _) => {
                        source_ml_source1_data_property_value703.clone()
                    }
                };
                {
                    let source_ml_source1_array_read_index705 = at;
                    { let ml_reference_array141 = source_ml_source1_array_read_value704.clone(); let ml_reference_position141 = source_ml_source1_array_read_index705; let ml_reference_projection141 = self::ml_array::optional_at(&*ml_reference_array141.borrow(), self::ml_array::number_index(ml_reference_position141)); ml_reference_projection141 }
                }
            };
            {
                let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::create_java_script_property_descriptor(self::ml_string::Text::from_units(vec![100, 97, 116, 97]), false, false)
                } else {
                    match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value706) => {
                            source_ml_source1_data_property_value706.clone()
                        }
                    }
                };
                {
                    let plan = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, source_ml_source1_data_property_value707, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value707
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, source_ml_source1_data_property_value708, _, _, _, _, _) => {
                            source_ml_source1_data_property_value708
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, source_ml_source1_data_property_value709, _, _) => {
                            source_ml_source1_data_property_value709
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, source_ml_source1_data_property_value710, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value710
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, source_ml_source1_data_property_value711, _, _, _) => {
                            source_ml_source1_data_property_value711
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, source_ml_source1_data_property_value712, _, _, _) => {
                            source_ml_source1_data_property_value712
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, source_ml_source1_data_property_value713, _, _, _) => {
                            source_ml_source1_data_property_value713
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, source_ml_source1_data_property_value714, _, _, _) => {
                            source_ml_source1_data_property_value714
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value715, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value715
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, source_ml_source1_data_property_value716, _, _, _, _) => {
                            source_ml_source1_data_property_value716
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, source_ml_source1_data_property_value717, _, _, _, _) => {
                            source_ml_source1_data_property_value717
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, source_ml_source1_data_property_value718, _, _, _, _) => {
                            source_ml_source1_data_property_value718
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, source_ml_source1_data_property_value719, _, _, _, _) => {
                            source_ml_source1_data_property_value719
                        }
                    }, current.clone(), proposed.clone());
                    if ((({ let ml_reference_value142 = plan.borrow().clone(); ml_reference_value142 }).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, ({ let ml_reference_value143 = plan.borrow().clone(); ml_reference_value143 }).2, self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    } else {
                        if ((({ let ml_reference_value144 = plan.borrow().clone(); ml_reference_value144 }).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), true, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            if ((at < (0f64)) && (({ let ml_reference_array145 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value733, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value733.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value734, _, _, _) => {
                                    source_ml_source1_data_property_value734.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value735) => {
                                    source_ml_source1_data_property_value735.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value736, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value736.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value737, _) => {
                                    source_ml_source1_data_property_value737.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value738, _) => {
                                    source_ml_source1_data_property_value738.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value739, _) => {
                                    source_ml_source1_data_property_value739.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value740, _) => {
                                    source_ml_source1_data_property_value740.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value741) => {
                                    source_ml_source1_data_property_value741.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value742, _, _) => {
                                    source_ml_source1_data_property_value742.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value743, _, _) => {
                                    source_ml_source1_data_property_value743.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value744, _, _) => {
                                    source_ml_source1_data_property_value744.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value745, _, _) => {
                                    source_ml_source1_data_property_value745.clone()
                                }
                            }; let ml_reference_projection145 = ml_reference_array145.borrow().len() as f64; ml_reference_projection145 }) >= (4096f64))) {
                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]), self::ml_string::Text::from_units(vec![114, 101, 115, 111, 117, 114, 99, 101]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                            } else {
                                {
                                    let replacement = self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), ({ let ml_reference_value146 = plan.borrow().clone(); ml_reference_value146 }).0);
                                    {
                                        let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                        {
                                            let index = 0f64;
                                            {
                                                let source_ml_temporary_values17 = match object.clone() {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value746, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value746.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value747, _, _, _) => {
                                                        source_ml_source1_data_property_value747.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value748) => {
                                                        source_ml_source1_data_property_value748.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value749, _, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value749.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value750, _) => {
                                                        source_ml_source1_data_property_value750.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value751, _) => {
                                                        source_ml_source1_data_property_value751.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value752, _) => {
                                                        source_ml_source1_data_property_value752.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value753, _) => {
                                                        source_ml_source1_data_property_value753.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value754) => {
                                                        source_ml_source1_data_property_value754.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value755, _, _) => {
                                                        source_ml_source1_data_property_value755.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value756, _, _) => {
                                                        source_ml_source1_data_property_value756.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value757, _, _) => {
                                                        source_ml_source1_data_property_value757.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value758, _, _) => {
                                                        source_ml_source1_data_property_value758.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_index17 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s759 = self::source_ml_temporary_plan_java_script_array_ordinary_definition_loop18(at, replacement.clone(), properties.clone(), index, source_ml_temporary_values17.clone(), source_ml_temporary_index17.clone());
                                                        match source_ml_source1_s759.clone() {
                                                            self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Result::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Done => {
                                                                {
                                                                    let source_ml_source1_s760 = if (at < (0f64)) {
                                                                        {
                                                                            let source_ml_temporary_plan_java_script_array_ordinary_definition_ignored20 = { let ml_method_owner147 = properties.clone(); let ml_method_argument147_0 = replacement.clone(); let ml_inserted = vec![ml_method_argument147_0]; let mut ml_method_values147 = ml_method_owner147.borrow_mut(); let ml_length = ml_method_values147.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values147.extend(ml_inserted); ml_length as f64 };
                                                                            self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19Next
                                                                        }
                                                                    } else {
                                                                        self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19Next
                                                                    };
                                                                    match source_ml_source1_s760.clone() {
                                                                        self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19Next => {
                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(match object.clone() {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field761, source_ml_source1_nominal_field762, source_ml_source1_nominal_field763, source_ml_source1_nominal_field764, source_ml_source1_nominal_field765, source_ml_source1_nominal_field766, source_ml_source1_nominal_field767, source_ml_source1_nominal_field768, source_ml_source1_nominal_field769, source_ml_source1_nominal_field770) => {
                                                                                    let source_ml_source1_nominal_field762 = *source_ml_source1_nominal_field762;
                                                                                    let source_ml_source1_nominal_field769 = *source_ml_source1_nominal_field769;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override771 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field761.clone(), Box::new(source_ml_source1_nominal_field762.clone()), source_ml_source1_nominal_field763, source_ml_source1_nominal_field764, source_ml_source1_nominal_override771.clone(), source_ml_source1_nominal_field766.clone(), source_ml_source1_nominal_field767, source_ml_source1_nominal_field768.clone(), Box::new(source_ml_source1_nominal_field769.clone()), source_ml_source1_nominal_field770.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field772, source_ml_source1_nominal_field773, source_ml_source1_nominal_field774, source_ml_source1_nominal_field775, source_ml_source1_nominal_field776, source_ml_source1_nominal_field777, source_ml_source1_nominal_field778, source_ml_source1_nominal_field779) => {
                                                                                    let source_ml_source1_nominal_field773 = *source_ml_source1_nominal_field773;
                                                                                    let source_ml_source1_nominal_field777 = *source_ml_source1_nominal_field777;
                                                                                    let source_ml_source1_nominal_field778 = *source_ml_source1_nominal_field778;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override780 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field772.clone(), Box::new(source_ml_source1_nominal_field773.clone()), source_ml_source1_nominal_field774, source_ml_source1_nominal_field775, source_ml_source1_nominal_override780.clone(), Box::new(source_ml_source1_nominal_field777.clone()), Box::new(source_ml_source1_nominal_field778.clone()), source_ml_source1_nominal_field779.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field781, source_ml_source1_nominal_field782, source_ml_source1_nominal_field783, source_ml_source1_nominal_field784, source_ml_source1_nominal_field785) => {
                                                                                    let source_ml_source1_nominal_field782 = *source_ml_source1_nominal_field782;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override786 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field781.clone(), Box::new(source_ml_source1_nominal_field782.clone()), source_ml_source1_nominal_field783, source_ml_source1_nominal_field784, source_ml_source1_nominal_override786.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field787, source_ml_source1_nominal_field788, source_ml_source1_nominal_field789, source_ml_source1_nominal_field790, source_ml_source1_nominal_field791, source_ml_source1_nominal_field792, source_ml_source1_nominal_field793, source_ml_source1_nominal_field794, source_ml_source1_nominal_field795, source_ml_source1_nominal_field796, source_ml_source1_nominal_field797) => {
                                                                                    let source_ml_source1_nominal_field788 = *source_ml_source1_nominal_field788;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override798 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field787.clone(), Box::new(source_ml_source1_nominal_field788.clone()), source_ml_source1_nominal_field789, source_ml_source1_nominal_field790, source_ml_source1_nominal_override798.clone(), source_ml_source1_nominal_field792.clone(), source_ml_source1_nominal_field793, source_ml_source1_nominal_field794.clone(), source_ml_source1_nominal_field795.clone(), source_ml_source1_nominal_field796.clone(), source_ml_source1_nominal_field797.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field799, source_ml_source1_nominal_field800, source_ml_source1_nominal_field801, source_ml_source1_nominal_field802, source_ml_source1_nominal_field803, source_ml_source1_nominal_field804) => {
                                                                                    let source_ml_source1_nominal_field800 = *source_ml_source1_nominal_field800;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override805 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field799.clone(), Box::new(source_ml_source1_nominal_field800.clone()), source_ml_source1_nominal_field801, source_ml_source1_nominal_field802, source_ml_source1_nominal_override805.clone(), source_ml_source1_nominal_field804.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field806, source_ml_source1_nominal_field807, source_ml_source1_nominal_field808, source_ml_source1_nominal_field809, source_ml_source1_nominal_field810, source_ml_source1_nominal_field811) => {
                                                                                    let source_ml_source1_nominal_field807 = *source_ml_source1_nominal_field807;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override812 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field806.clone(), Box::new(source_ml_source1_nominal_field807.clone()), source_ml_source1_nominal_field808, source_ml_source1_nominal_field809, source_ml_source1_nominal_override812.clone(), source_ml_source1_nominal_field811.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field813, source_ml_source1_nominal_field814, source_ml_source1_nominal_field815, source_ml_source1_nominal_field816, source_ml_source1_nominal_field817, source_ml_source1_nominal_field818) => {
                                                                                    let source_ml_source1_nominal_field814 = *source_ml_source1_nominal_field814;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override819 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field813.clone(), Box::new(source_ml_source1_nominal_field814.clone()), source_ml_source1_nominal_field815, source_ml_source1_nominal_field816, source_ml_source1_nominal_override819.clone(), source_ml_source1_nominal_field818.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field820, source_ml_source1_nominal_field821, source_ml_source1_nominal_field822, source_ml_source1_nominal_field823, source_ml_source1_nominal_field824, source_ml_source1_nominal_field825) => {
                                                                                    let source_ml_source1_nominal_field821 = *source_ml_source1_nominal_field821;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override826 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field820.clone(), Box::new(source_ml_source1_nominal_field821.clone()), source_ml_source1_nominal_field822, source_ml_source1_nominal_field823, source_ml_source1_nominal_override826.clone(), source_ml_source1_nominal_field825.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field827, source_ml_source1_nominal_field828, source_ml_source1_nominal_field829, source_ml_source1_nominal_field830, source_ml_source1_nominal_field831, source_ml_source1_nominal_field832, source_ml_source1_nominal_field833, source_ml_source1_nominal_field834, source_ml_source1_nominal_field835) => {
                                                                                    let source_ml_source1_nominal_field828 = *source_ml_source1_nominal_field828;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override836 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field827.clone(), Box::new(source_ml_source1_nominal_field828.clone()), source_ml_source1_nominal_field829, source_ml_source1_nominal_field830.clone(), source_ml_source1_nominal_field831.clone(), source_ml_source1_nominal_field832.clone(), source_ml_source1_nominal_field833.clone(), source_ml_source1_nominal_field834, source_ml_source1_nominal_override836.clone())
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field837, source_ml_source1_nominal_field838, source_ml_source1_nominal_field839, source_ml_source1_nominal_field840, source_ml_source1_nominal_field841, source_ml_source1_nominal_field842, source_ml_source1_nominal_field843) => {
                                                                                    let source_ml_source1_nominal_field838 = *source_ml_source1_nominal_field838;
                                                                                    let source_ml_source1_nominal_field842 = *source_ml_source1_nominal_field842;
                                                                                    let source_ml_source1_nominal_field843 = *source_ml_source1_nominal_field843;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override844 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field837.clone(), Box::new(source_ml_source1_nominal_field838.clone()), source_ml_source1_nominal_field839, source_ml_source1_nominal_field840, source_ml_source1_nominal_override844.clone(), Box::new(source_ml_source1_nominal_field842.clone()), Box::new(source_ml_source1_nominal_field843.clone()))
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field845, source_ml_source1_nominal_field846, source_ml_source1_nominal_field847, source_ml_source1_nominal_field848, source_ml_source1_nominal_field849, source_ml_source1_nominal_field850, source_ml_source1_nominal_field851) => {
                                                                                    let source_ml_source1_nominal_field846 = *source_ml_source1_nominal_field846;
                                                                                    let source_ml_source1_nominal_field850 = *source_ml_source1_nominal_field850;
                                                                                    let source_ml_source1_nominal_field851 = *source_ml_source1_nominal_field851;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override852 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field845.clone(), Box::new(source_ml_source1_nominal_field846.clone()), source_ml_source1_nominal_field847, source_ml_source1_nominal_field848, source_ml_source1_nominal_override852.clone(), Box::new(source_ml_source1_nominal_field850.clone()), Box::new(source_ml_source1_nominal_field851.clone()))
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field853, source_ml_source1_nominal_field854, source_ml_source1_nominal_field855, source_ml_source1_nominal_field856, source_ml_source1_nominal_field857, source_ml_source1_nominal_field858, source_ml_source1_nominal_field859) => {
                                                                                    let source_ml_source1_nominal_field854 = *source_ml_source1_nominal_field854;
                                                                                    let source_ml_source1_nominal_field858 = *source_ml_source1_nominal_field858;
                                                                                    let source_ml_source1_nominal_field859 = *source_ml_source1_nominal_field859;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override860 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field853.clone(), Box::new(source_ml_source1_nominal_field854.clone()), source_ml_source1_nominal_field855, source_ml_source1_nominal_field856, source_ml_source1_nominal_override860.clone(), Box::new(source_ml_source1_nominal_field858.clone()), Box::new(source_ml_source1_nominal_field859.clone()))
                                                                                    }
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field861, source_ml_source1_nominal_field862, source_ml_source1_nominal_field863, source_ml_source1_nominal_field864, source_ml_source1_nominal_field865, source_ml_source1_nominal_field866, source_ml_source1_nominal_field867) => {
                                                                                    let source_ml_source1_nominal_field862 = *source_ml_source1_nominal_field862;
                                                                                    let source_ml_source1_nominal_field866 = *source_ml_source1_nominal_field866;
                                                                                    let source_ml_source1_nominal_field867 = *source_ml_source1_nominal_field867;
                                                                                    {
                                                                                        let source_ml_source1_nominal_override868 = properties.clone();
                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field861.clone(), Box::new(source_ml_source1_nominal_field862.clone()), source_ml_source1_nominal_field863, source_ml_source1_nominal_field864, source_ml_source1_nominal_override868.clone(), Box::new(source_ml_source1_nominal_field866.clone()), Box::new(source_ml_source1_nominal_field867.clone()))
                                                                                    }
                                                                                }
                                                                            }), true, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), ({ let ml_reference_value148 = plan.borrow().clone(); ml_reference_value148 }).3)
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
