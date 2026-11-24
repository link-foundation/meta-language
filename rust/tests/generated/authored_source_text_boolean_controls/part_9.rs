pub fn read_java_script_array_index(key: self::JavaScriptSemanticValue) -> f64 {
    {
        let length = self::read_java_script_semantic_text_length(key.clone());
        if ((length < (0f64)) || (length > (10f64))) {
            (-1f64)
        } else {
            self::read_java_script_literal_property_index(self::construct_java_script_semantic_text_units(key.clone(), 10f64))
        }
    }
}

pub fn find_java_script_array_property(properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, key: self::JavaScriptSemanticValue) -> f64 {
    self::find_java_script_heap_directory_property(properties.clone(), key.clone())
}

pub fn read_java_script_array_length(object: self::JavaScriptOrdinaryObject) -> f64 {
    {
        let property = {
            let source_ml_source1_array_read_value680 = match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value654, _, _, _, _, _) => {
                    source_ml_source1_data_property_value654.clone()
                }
                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value655, _, _, _) => {
                    source_ml_source1_data_property_value655.clone()
                }
                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value656) => {
                    source_ml_source1_data_property_value656.clone()
                }
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value657, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value657.clone()
                }
                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value658, _) => {
                    source_ml_source1_data_property_value658.clone()
                }
                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value659, _) => {
                    source_ml_source1_data_property_value659.clone()
                }
                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value660, _) => {
                    source_ml_source1_data_property_value660.clone()
                }
                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value661, _) => {
                    source_ml_source1_data_property_value661.clone()
                }
                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value662) => {
                    source_ml_source1_data_property_value662.clone()
                }
                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value663, _, _) => {
                    source_ml_source1_data_property_value663.clone()
                }
                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value664, _, _) => {
                    source_ml_source1_data_property_value664.clone()
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value665, _, _) => {
                    source_ml_source1_data_property_value665.clone()
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value666, _, _) => {
                    source_ml_source1_data_property_value666.clone()
                }
            };
            {
                let source_ml_source1_array_read_index681 = self::find_java_script_array_property(match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value667, _, _, _, _, _) => {
                        source_ml_source1_data_property_value667.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value668, _, _, _) => {
                        source_ml_source1_data_property_value668.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value669) => {
                        source_ml_source1_data_property_value669.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value670, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value670.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value671, _) => {
                        source_ml_source1_data_property_value671.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value672, _) => {
                        source_ml_source1_data_property_value672.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value673, _) => {
                        source_ml_source1_data_property_value673.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value674, _) => {
                        source_ml_source1_data_property_value674.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value675) => {
                        source_ml_source1_data_property_value675.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value676, _, _) => {
                        source_ml_source1_data_property_value676.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value677, _, _) => {
                        source_ml_source1_data_property_value677.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value678, _, _) => {
                        source_ml_source1_data_property_value678.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value679, _, _) => {
                        source_ml_source1_data_property_value679.clone()
                    }
                }, self::read_java_script_array_length_key());
                { let ml_reference_array133 = source_ml_source1_array_read_value680.clone(); let ml_reference_position133 = source_ml_source1_array_read_index681; let ml_reference_projection133 = self::ml_array::optional_at(&*ml_reference_array133.borrow(), self::ml_array::number_index(ml_reference_position133)); ml_reference_projection133 }
            }
        };
        if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
            (-1f64)
        } else {
            self::read_java_script_array_length_value(({ let ml_reference_value134 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value682) => {
                    source_ml_source1_data_property_value682.clone()
                }
            }).borrow().clone(); ml_reference_value134 }).8)
        }
    }
}

pub fn read_java_script_array_length_value(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Number(value_value) => {
            value_value
        }
        _ => {
            (-1f64)
        }
    }
}

pub fn test_java_script_array_length(length: f64) -> bool {
    (((self::ml_math::is_integer(length) && (length >= (0f64))) && (length <= (4294967295f64))) && !({
        let source_ml_source1_same_value_operand692 = (-0f64);
        (((length == source_ml_source1_same_value_operand692) && ((length != (0f64)) || (((1f64 / length)) == ((1f64 / source_ml_source1_same_value_operand692))))) || ((length != length) && (source_ml_source1_same_value_operand692 != source_ml_source1_same_value_operand692)))
    }))
}

pub fn read_java_script_array_object_error(object: self::JavaScriptOrdinaryObject) -> self::ml_string::Text {
    if match object.clone() {
        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source1_data_property_value693, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value693
        }
        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source1_data_property_value694, _, _, _, _) => {
            source_ml_source1_data_property_value694
        }
        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source1_data_property_value695, _) => {
            source_ml_source1_data_property_value695
        }
        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source1_data_property_value696, _, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value696
        }
        self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source1_data_property_value697, _, _) => {
            source_ml_source1_data_property_value697
        }
        self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source1_data_property_value698, _, _) => {
            source_ml_source1_data_property_value698
        }
        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source1_data_property_value699, _, _) => {
            source_ml_source1_data_property_value699
        }
        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source1_data_property_value700, _, _) => {
            source_ml_source1_data_property_value700
        }
        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source1_data_property_value701, _) => {
            source_ml_source1_data_property_value701
        }
        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source1_data_property_value702, _, _, _) => {
            source_ml_source1_data_property_value702
        }
        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source1_data_property_value703, _, _, _) => {
            source_ml_source1_data_property_value703
        }
        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source1_data_property_value704, _, _, _) => {
            source_ml_source1_data_property_value704
        }
        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source1_data_property_value705, _, _, _) => {
            source_ml_source1_data_property_value705
        }
    } {
        self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 101, 120, 111, 116, 105, 99, 32, 111, 98, 106, 101, 99, 116, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101])
    } else {
        {
            let property = {
                let source_ml_source1_array_read_value732 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value706, _, _, _, _, _) => {
                        source_ml_source1_data_property_value706.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value707, _, _, _) => {
                        source_ml_source1_data_property_value707.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value708) => {
                        source_ml_source1_data_property_value708.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value709, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value709.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value710, _) => {
                        source_ml_source1_data_property_value710.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value711, _) => {
                        source_ml_source1_data_property_value711.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value712, _) => {
                        source_ml_source1_data_property_value712.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value713, _) => {
                        source_ml_source1_data_property_value713.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value714) => {
                        source_ml_source1_data_property_value714.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value715, _, _) => {
                        source_ml_source1_data_property_value715.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value716, _, _) => {
                        source_ml_source1_data_property_value716.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value717, _, _) => {
                        source_ml_source1_data_property_value717.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value718, _, _) => {
                        source_ml_source1_data_property_value718.clone()
                    }
                };
                {
                    let source_ml_source1_array_read_index733 = self::find_java_script_array_property(match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value719, _, _, _, _, _) => {
                            source_ml_source1_data_property_value719.clone()
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value720, _, _, _) => {
                            source_ml_source1_data_property_value720.clone()
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value721) => {
                            source_ml_source1_data_property_value721.clone()
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value722, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value722.clone()
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value723, _) => {
                            source_ml_source1_data_property_value723.clone()
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value724, _) => {
                            source_ml_source1_data_property_value724.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value725, _) => {
                            source_ml_source1_data_property_value725.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value726, _) => {
                            source_ml_source1_data_property_value726.clone()
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value727) => {
                            source_ml_source1_data_property_value727.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value728, _, _) => {
                            source_ml_source1_data_property_value728.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value729, _, _) => {
                            source_ml_source1_data_property_value729.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value730, _, _) => {
                            source_ml_source1_data_property_value730.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value731, _, _) => {
                            source_ml_source1_data_property_value731.clone()
                        }
                    }, self::read_java_script_array_length_key());
                    { let ml_reference_array135 = source_ml_source1_array_read_value732.clone(); let ml_reference_position135 = source_ml_source1_array_read_index733; let ml_reference_projection135 = self::ml_array::optional_at(&*ml_reference_array135.borrow(), self::ml_array::number_index(ml_reference_position135)); ml_reference_projection135 }
                }
            };
            if (((((match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(({ let ml_reference_value136 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value734) => {
                    source_ml_source1_data_property_value734.clone()
                }
            }).borrow().clone(); ml_reference_value136 }).9)) || ({ let ml_reference_value137 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value735) => {
                    source_ml_source1_data_property_value735.clone()
                }
            }).borrow().clone(); ml_reference_value137 }).5) || ({ let ml_reference_value138 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value736) => {
                    source_ml_source1_data_property_value736.clone()
                }
            }).borrow().clone(); ml_reference_value138 }).7) || ({ let ml_reference_value139 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value737) => {
                    source_ml_source1_data_property_value737.clone()
                }
            }).borrow().clone(); ml_reference_value139 }).2) || ({ let ml_reference_value140 = (match property.clone().expect("a proven present value") {
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value738) => {
                    source_ml_source1_data_property_value738.clone()
                }
            }).borrow().clone(); ml_reference_value140 }).0) {
                self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 101, 110, 117, 109, 101, 114, 97, 98, 108, 101, 32, 110, 111, 110, 99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101, 32, 105, 110, 116, 101, 103, 114, 97, 108, 32, 108, 101, 110, 103, 116, 104, 32, 100, 97, 116, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121])
            } else {
                {
                    let length = self::read_java_script_array_length_value(({ let ml_reference_value141 = (match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value739) => {
                            source_ml_source1_data_property_value739.clone()
                        }
                    }).borrow().clone(); ml_reference_value141 }).8);
                    if !(self::test_java_script_array_length(length)) {
                        self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 101, 110, 117, 109, 101, 114, 97, 98, 108, 101, 32, 110, 111, 110, 99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101, 32, 105, 110, 116, 101, 103, 114, 97, 108, 32, 108, 101, 110, 103, 116, 104, 32, 100, 97, 116, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121])
                    } else {
                        {
                            let source_ml_temporary_values13 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value740, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value740.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value741, _, _, _) => {
                                    source_ml_source1_data_property_value741.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value742) => {
                                    source_ml_source1_data_property_value742.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value743, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value743.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value744, _) => {
                                    source_ml_source1_data_property_value744.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value745, _) => {
                                    source_ml_source1_data_property_value745.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value746, _) => {
                                    source_ml_source1_data_property_value746.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value747, _) => {
                                    source_ml_source1_data_property_value747.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value748) => {
                                    source_ml_source1_data_property_value748.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value749, _, _) => {
                                    source_ml_source1_data_property_value749.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value750, _, _) => {
                                    source_ml_source1_data_property_value750.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value751, _, _) => {
                                    source_ml_source1_data_property_value751.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value752, _, _) => {
                                    source_ml_source1_data_property_value752.clone()
                                }
                            };
                            {
                                let source_ml_temporary_index13 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source1_s753 = self::source_ml_temporary_read_java_script_array_object_error_loop14(length, source_ml_temporary_values13.clone(), source_ml_temporary_index13.clone());
                                    match source_ml_source1_s753.clone() {
                                        self::MlTemporaryReadJavaScriptArrayObjectErrorLoop14Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop14Done => {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                        self::MlTemporaryReadJavaScriptArrayObjectErrorLoop14Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop14Return(source_ml_temporary_result14) => {
                                            source_ml_temporary_result14.clone()
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
            let source_ml_temporary_construct_java_script_array_object_written15 = { let ml_field_owner142 = descriptor.clone(); let ml_field_value142 = false; ml_field_owner142.borrow_mut().0 = ml_field_value142; () };
            {
                let source_ml_temporary_construct_java_script_array_object_written16 = { let ml_field_owner143 = descriptor.clone(); let ml_field_value143 = true; ml_field_owner143.borrow_mut().10 = ml_field_value143; () };
                {
                    let source_ml_temporary_construct_java_script_array_object_written17 = { let ml_field_owner144 = descriptor.clone(); let ml_field_value144 = self::JavaScriptSemanticValue::Number(length); ml_field_owner144.borrow_mut().8 = ml_field_value144; () };
                    self::JavaScriptOrdinaryObject::ArrayObject(identity.clone(), Box::new(prototype.clone()), true, false, std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(self::read_java_script_array_length_key()), descriptor.clone())])))
                }
            }
        }
    }
}

pub fn plan_java_script_array_ordinary_definition(object: self::JavaScriptOrdinaryObject, key: self::JavaScriptSemanticValue, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::JavaScriptArrayPropertyPlan {
    {
        let at = self::find_java_script_array_property(match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value754, _, _, _, _, _) => {
                source_ml_source1_data_property_value754.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value755, _, _, _) => {
                source_ml_source1_data_property_value755.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value756) => {
                source_ml_source1_data_property_value756.clone()
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value757, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value757.clone()
            }
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value758, _) => {
                source_ml_source1_data_property_value758.clone()
            }
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value759, _) => {
                source_ml_source1_data_property_value759.clone()
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value760, _) => {
                source_ml_source1_data_property_value760.clone()
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value761, _) => {
                source_ml_source1_data_property_value761.clone()
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value762) => {
                source_ml_source1_data_property_value762.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value763, _, _) => {
                source_ml_source1_data_property_value763.clone()
            }
            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value764, _, _) => {
                source_ml_source1_data_property_value764.clone()
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value765, _, _) => {
                source_ml_source1_data_property_value765.clone()
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value766, _, _) => {
                source_ml_source1_data_property_value766.clone()
            }
        }, key.clone());
        {
            let property = {
                let source_ml_source1_array_read_value780 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value767, _, _, _, _, _) => {
                        source_ml_source1_data_property_value767.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value768, _, _, _) => {
                        source_ml_source1_data_property_value768.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value769) => {
                        source_ml_source1_data_property_value769.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value770, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value770.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value771, _) => {
                        source_ml_source1_data_property_value771.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value772, _) => {
                        source_ml_source1_data_property_value772.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value773, _) => {
                        source_ml_source1_data_property_value773.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value774, _) => {
                        source_ml_source1_data_property_value774.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value775) => {
                        source_ml_source1_data_property_value775.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value776, _, _) => {
                        source_ml_source1_data_property_value776.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value777, _, _) => {
                        source_ml_source1_data_property_value777.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value778, _, _) => {
                        source_ml_source1_data_property_value778.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value779, _, _) => {
                        source_ml_source1_data_property_value779.clone()
                    }
                };
                { let ml_reference_array145 = source_ml_source1_array_read_value780.clone(); let ml_reference_position145 = at; let ml_reference_projection145 = self::ml_array::optional_at(&*ml_reference_array145.borrow(), self::ml_array::number_index(ml_reference_position145)); ml_reference_projection145 }
            };
            {
                let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::create_java_script_property_descriptor(self::ml_string::Text::from_units(vec![100, 97, 116, 97]), false, false)
                } else {
                    match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value782) => {
                            source_ml_source1_data_property_value782.clone()
                        }
                    }
                };
                {
                    let plan = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, source_ml_source1_data_property_value783, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value783
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, source_ml_source1_data_property_value784, _, _, _, _, _) => {
                            source_ml_source1_data_property_value784
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, source_ml_source1_data_property_value785, _, _) => {
                            source_ml_source1_data_property_value785
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, source_ml_source1_data_property_value786, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value786
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, source_ml_source1_data_property_value787, _, _, _) => {
                            source_ml_source1_data_property_value787
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, source_ml_source1_data_property_value788, _, _, _) => {
                            source_ml_source1_data_property_value788
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, source_ml_source1_data_property_value789, _, _, _) => {
                            source_ml_source1_data_property_value789
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, source_ml_source1_data_property_value790, _, _, _) => {
                            source_ml_source1_data_property_value790
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value791, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value791
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, source_ml_source1_data_property_value792, _, _, _, _) => {
                            source_ml_source1_data_property_value792
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, source_ml_source1_data_property_value793, _, _, _, _) => {
                            source_ml_source1_data_property_value793
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, source_ml_source1_data_property_value794, _, _, _, _) => {
                            source_ml_source1_data_property_value794
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, source_ml_source1_data_property_value795, _, _, _, _) => {
                            source_ml_source1_data_property_value795
                        }
                    }, current.clone(), proposed.clone());
                    if ((({ let ml_reference_value146 = plan.borrow().clone(); ml_reference_value146 }).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, ({ let ml_reference_value147 = plan.borrow().clone(); ml_reference_value147 }).2, self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    } else {
                        if ((({ let ml_reference_value148 = plan.borrow().clone(); ml_reference_value148 }).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), true, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            if ((at < (0f64)) && (({ let ml_reference_array149 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value809, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value809.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value810, _, _, _) => {
                                    source_ml_source1_data_property_value810.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value811) => {
                                    source_ml_source1_data_property_value811.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value812, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value812.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value813, _) => {
                                    source_ml_source1_data_property_value813.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value814, _) => {
                                    source_ml_source1_data_property_value814.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value815, _) => {
                                    source_ml_source1_data_property_value815.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value816, _) => {
                                    source_ml_source1_data_property_value816.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value817) => {
                                    source_ml_source1_data_property_value817.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value818, _, _) => {
                                    source_ml_source1_data_property_value818.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value819, _, _) => {
                                    source_ml_source1_data_property_value819.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value820, _, _) => {
                                    source_ml_source1_data_property_value820.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value821, _, _) => {
                                    source_ml_source1_data_property_value821.clone()
                                }
                            }; let ml_reference_projection149 = ml_reference_array149.borrow().len() as f64; ml_reference_projection149 }) >= (4096f64))) {
                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]), self::ml_string::Text::from_units(vec![114, 101, 115, 111, 117, 114, 99, 101]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                            } else {
                                {
                                    let replacement = self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), ({ let ml_reference_value150 = plan.borrow().clone(); ml_reference_value150 }).0);
                                    {
                                        let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                        {
                                            let index = 0f64;
                                            {
                                                let source_ml_temporary_values18 = match object.clone() {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value822, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value822.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value823, _, _, _) => {
                                                        source_ml_source1_data_property_value823.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value824) => {
                                                        source_ml_source1_data_property_value824.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value825, _, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value825.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value826, _) => {
                                                        source_ml_source1_data_property_value826.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value827, _) => {
                                                        source_ml_source1_data_property_value827.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value828, _) => {
                                                        source_ml_source1_data_property_value828.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value829, _) => {
                                                        source_ml_source1_data_property_value829.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value830) => {
                                                        source_ml_source1_data_property_value830.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value831, _, _) => {
                                                        source_ml_source1_data_property_value831.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value832, _, _) => {
                                                        source_ml_source1_data_property_value832.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value833, _, _) => {
                                                        source_ml_source1_data_property_value833.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value834, _, _) => {
                                                        source_ml_source1_data_property_value834.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_index18 = self::ml::Big::from_i128(0);
                                                    {
                                                        let properties_2 = self::source_ml_temporary_plan_java_script_array_ordinary_definition_loop19(at, replacement.clone(), properties.clone(), index, source_ml_temporary_values18.clone(), source_ml_temporary_index18.clone());
                                                        {
                                                            let properties_4 = if (at < (0f64)) {
                                                                {
                                                                    let properties_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value151 = properties_2.borrow().clone(); ml_reference_value151 }, vec![replacement.clone()])));
                                                                    properties_3.clone()
                                                                }
                                                            } else {
                                                                properties_2.clone()
                                                            };
                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(match object.clone() {
                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field835, source_ml_source1_nominal_field836, source_ml_source1_nominal_field837, source_ml_source1_nominal_field838, _, source_ml_source1_nominal_field840, source_ml_source1_nominal_field841, source_ml_source1_nominal_field842, source_ml_source1_nominal_field843, source_ml_source1_nominal_field844) => {
                                                                    let source_ml_source1_nominal_field836 = *source_ml_source1_nominal_field836;
                                                                    let source_ml_source1_nominal_field843 = *source_ml_source1_nominal_field843;
                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field835.clone(), Box::new(source_ml_source1_nominal_field836.clone()), source_ml_source1_nominal_field837, source_ml_source1_nominal_field838, properties_4.clone(), source_ml_source1_nominal_field840.clone(), source_ml_source1_nominal_field841, source_ml_source1_nominal_field842.clone(), Box::new(source_ml_source1_nominal_field843.clone()), source_ml_source1_nominal_field844.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field846, source_ml_source1_nominal_field847, source_ml_source1_nominal_field848, source_ml_source1_nominal_field849, _, source_ml_source1_nominal_field851, source_ml_source1_nominal_field852, source_ml_source1_nominal_field853) => {
                                                                    let source_ml_source1_nominal_field847 = *source_ml_source1_nominal_field847;
                                                                    let source_ml_source1_nominal_field851 = *source_ml_source1_nominal_field851;
                                                                    let source_ml_source1_nominal_field852 = *source_ml_source1_nominal_field852;
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field846.clone(), Box::new(source_ml_source1_nominal_field847.clone()), source_ml_source1_nominal_field848, source_ml_source1_nominal_field849, properties_4.clone(), Box::new(source_ml_source1_nominal_field851.clone()), Box::new(source_ml_source1_nominal_field852.clone()), source_ml_source1_nominal_field853.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field855, source_ml_source1_nominal_field856, source_ml_source1_nominal_field857, source_ml_source1_nominal_field858, _) => {
                                                                    let source_ml_source1_nominal_field856 = *source_ml_source1_nominal_field856;
                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field855.clone(), Box::new(source_ml_source1_nominal_field856.clone()), source_ml_source1_nominal_field857, source_ml_source1_nominal_field858, properties_4.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field861, source_ml_source1_nominal_field862, source_ml_source1_nominal_field863, source_ml_source1_nominal_field864, _, source_ml_source1_nominal_field866, source_ml_source1_nominal_field867, source_ml_source1_nominal_field868, source_ml_source1_nominal_field869, source_ml_source1_nominal_field870, source_ml_source1_nominal_field871) => {
                                                                    let source_ml_source1_nominal_field862 = *source_ml_source1_nominal_field862;
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field861.clone(), Box::new(source_ml_source1_nominal_field862.clone()), source_ml_source1_nominal_field863, source_ml_source1_nominal_field864, properties_4.clone(), source_ml_source1_nominal_field866.clone(), source_ml_source1_nominal_field867, source_ml_source1_nominal_field868.clone(), source_ml_source1_nominal_field869.clone(), source_ml_source1_nominal_field870.clone(), source_ml_source1_nominal_field871.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field873, source_ml_source1_nominal_field874, source_ml_source1_nominal_field875, source_ml_source1_nominal_field876, _, source_ml_source1_nominal_field878) => {
                                                                    let source_ml_source1_nominal_field874 = *source_ml_source1_nominal_field874;
                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field873.clone(), Box::new(source_ml_source1_nominal_field874.clone()), source_ml_source1_nominal_field875, source_ml_source1_nominal_field876, properties_4.clone(), source_ml_source1_nominal_field878.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field880, source_ml_source1_nominal_field881, source_ml_source1_nominal_field882, source_ml_source1_nominal_field883, _, source_ml_source1_nominal_field885) => {
                                                                    let source_ml_source1_nominal_field881 = *source_ml_source1_nominal_field881;
                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field880.clone(), Box::new(source_ml_source1_nominal_field881.clone()), source_ml_source1_nominal_field882, source_ml_source1_nominal_field883, properties_4.clone(), source_ml_source1_nominal_field885.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field887, source_ml_source1_nominal_field888, source_ml_source1_nominal_field889, source_ml_source1_nominal_field890, _, source_ml_source1_nominal_field892) => {
                                                                    let source_ml_source1_nominal_field888 = *source_ml_source1_nominal_field888;
                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field887.clone(), Box::new(source_ml_source1_nominal_field888.clone()), source_ml_source1_nominal_field889, source_ml_source1_nominal_field890, properties_4.clone(), source_ml_source1_nominal_field892.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field894, source_ml_source1_nominal_field895, source_ml_source1_nominal_field896, source_ml_source1_nominal_field897, _, source_ml_source1_nominal_field899) => {
                                                                    let source_ml_source1_nominal_field895 = *source_ml_source1_nominal_field895;
                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field894.clone(), Box::new(source_ml_source1_nominal_field895.clone()), source_ml_source1_nominal_field896, source_ml_source1_nominal_field897, properties_4.clone(), source_ml_source1_nominal_field899.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field901, source_ml_source1_nominal_field902, source_ml_source1_nominal_field903, source_ml_source1_nominal_field904, source_ml_source1_nominal_field905, source_ml_source1_nominal_field906, source_ml_source1_nominal_field907, source_ml_source1_nominal_field908, _) => {
                                                                    let source_ml_source1_nominal_field902 = *source_ml_source1_nominal_field902;
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field901.clone(), Box::new(source_ml_source1_nominal_field902.clone()), source_ml_source1_nominal_field903, source_ml_source1_nominal_field904.clone(), source_ml_source1_nominal_field905.clone(), source_ml_source1_nominal_field906.clone(), source_ml_source1_nominal_field907.clone(), source_ml_source1_nominal_field908, properties_4.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field911, source_ml_source1_nominal_field912, source_ml_source1_nominal_field913, source_ml_source1_nominal_field914, _, source_ml_source1_nominal_field916, source_ml_source1_nominal_field917) => {
                                                                    let source_ml_source1_nominal_field912 = *source_ml_source1_nominal_field912;
                                                                    let source_ml_source1_nominal_field916 = *source_ml_source1_nominal_field916;
                                                                    let source_ml_source1_nominal_field917 = *source_ml_source1_nominal_field917;
                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field911.clone(), Box::new(source_ml_source1_nominal_field912.clone()), source_ml_source1_nominal_field913, source_ml_source1_nominal_field914, properties_4.clone(), Box::new(source_ml_source1_nominal_field916.clone()), Box::new(source_ml_source1_nominal_field917.clone()))
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field919, source_ml_source1_nominal_field920, source_ml_source1_nominal_field921, source_ml_source1_nominal_field922, _, source_ml_source1_nominal_field924, source_ml_source1_nominal_field925) => {
                                                                    let source_ml_source1_nominal_field920 = *source_ml_source1_nominal_field920;
                                                                    let source_ml_source1_nominal_field924 = *source_ml_source1_nominal_field924;
                                                                    let source_ml_source1_nominal_field925 = *source_ml_source1_nominal_field925;
                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field919.clone(), Box::new(source_ml_source1_nominal_field920.clone()), source_ml_source1_nominal_field921, source_ml_source1_nominal_field922, properties_4.clone(), Box::new(source_ml_source1_nominal_field924.clone()), Box::new(source_ml_source1_nominal_field925.clone()))
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field927, source_ml_source1_nominal_field928, source_ml_source1_nominal_field929, source_ml_source1_nominal_field930, _, source_ml_source1_nominal_field932, source_ml_source1_nominal_field933) => {
                                                                    let source_ml_source1_nominal_field928 = *source_ml_source1_nominal_field928;
                                                                    let source_ml_source1_nominal_field932 = *source_ml_source1_nominal_field932;
                                                                    let source_ml_source1_nominal_field933 = *source_ml_source1_nominal_field933;
                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field927.clone(), Box::new(source_ml_source1_nominal_field928.clone()), source_ml_source1_nominal_field929, source_ml_source1_nominal_field930, properties_4.clone(), Box::new(source_ml_source1_nominal_field932.clone()), Box::new(source_ml_source1_nominal_field933.clone()))
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field935, source_ml_source1_nominal_field936, source_ml_source1_nominal_field937, source_ml_source1_nominal_field938, _, source_ml_source1_nominal_field940, source_ml_source1_nominal_field941) => {
                                                                    let source_ml_source1_nominal_field936 = *source_ml_source1_nominal_field936;
                                                                    let source_ml_source1_nominal_field940 = *source_ml_source1_nominal_field940;
                                                                    let source_ml_source1_nominal_field941 = *source_ml_source1_nominal_field941;
                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field935.clone(), Box::new(source_ml_source1_nominal_field936.clone()), source_ml_source1_nominal_field937, source_ml_source1_nominal_field938, properties_4.clone(), Box::new(source_ml_source1_nominal_field940.clone()), Box::new(source_ml_source1_nominal_field941.clone()))
                                                                }
                                                            }), true, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), ({ let ml_reference_value152 = plan.borrow().clone(); ml_reference_value152 }).3)
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
