pub fn read_java_script_ordinary_class_receiver_error(heap: self::JavaScriptOrdinaryHeap, callee: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    {
        let invalid_receiver = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 110, 100, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114]);
        match callee.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Null => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f679) => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f680) => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f681) => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f682) => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source1_f683, source_ml_source1_f684) => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f685) => {
                invalid_receiver.clone()
            }
            self::JavaScriptSemanticValue::Object(callee_identity, callee_callable) => {
                if !callee_callable {
                    invalid_receiver.clone()
                } else {
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Null => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f686) => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f687) => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f688) => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f689) => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f690, source_ml_source1_f691) => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f692) => {
                            invalid_receiver.clone()
                        }
                        self::JavaScriptSemanticValue::Object(identity, callable) => {
                            if callable {
                                invalid_receiver.clone()
                            } else {
                                {
                                    let constructor = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value693, _, _, _, _, _) => source_ml_source1_data_property_value693.as_slice() }), self::ml_array::number_index(self::find_java_script_heap_directory_object(match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value694, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value694
                                        }
                                    }, callee_identity.clone())));
                                    {
                                        let instance = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value695, _, _, _, _, _) => source_ml_source1_data_property_value695.as_slice() }), self::ml_array::number_index(self::find_java_script_heap_directory_object(match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value696, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value696
                                            }
                                        }, identity.clone())));
                                        {
                                            let invalid_kind = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 111, 112, 97, 113, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 107, 105, 110, 100, 115]);
                                            if match (constructor.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                invalid_kind.clone()
                                            } else {
                                                match constructor.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_f697, source_ml_source1_f698, source_ml_source1_f699, source_ml_source1_f700, source_ml_source1_f701, source_ml_source1_f702, source_ml_source1_f703, source_ml_source1_f704, source_ml_source1_f705, source_ml_source1_f706) => {
                                                        let source_ml_source1_f698 = *source_ml_source1_f698;
                                                        let source_ml_source1_f705 = *source_ml_source1_f705;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_f707, source_ml_source1_f708, source_ml_source1_f709, source_ml_source1_f710, source_ml_source1_f711, source_ml_source1_f712, source_ml_source1_f713, source_ml_source1_f714) => {
                                                        let source_ml_source1_f708 = *source_ml_source1_f708;
                                                        let source_ml_source1_f712 = *source_ml_source1_f712;
                                                        let source_ml_source1_f713 = *source_ml_source1_f713;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_f715, source_ml_source1_f716, source_ml_source1_f717, source_ml_source1_f718, source_ml_source1_f719) => {
                                                        let source_ml_source1_f716 = *source_ml_source1_f716;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_f720, source_ml_source1_f721, source_ml_source1_f722, source_ml_source1_f723, source_ml_source1_f724, source_ml_source1_f725, source_ml_source1_f726, source_ml_source1_f727, source_ml_source1_f728, source_ml_source1_f729, source_ml_source1_f730) => {
                                                        let source_ml_source1_f721 = *source_ml_source1_f721;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_f731, source_ml_source1_f732, source_ml_source1_f733, source_ml_source1_f734, source_ml_source1_f735, source_ml_source1_f736) => {
                                                        let source_ml_source1_f732 = *source_ml_source1_f732;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_f737, source_ml_source1_f738, source_ml_source1_f739, source_ml_source1_f740, source_ml_source1_f741, source_ml_source1_f742) => {
                                                        let source_ml_source1_f738 = *source_ml_source1_f738;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_f743, source_ml_source1_f744, source_ml_source1_f745, source_ml_source1_f746, source_ml_source1_f747, source_ml_source1_f748) => {
                                                        let source_ml_source1_f744 = *source_ml_source1_f744;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_f749, source_ml_source1_f750, source_ml_source1_f751, source_ml_source1_f752, source_ml_source1_f753, source_ml_source1_f754) => {
                                                        let source_ml_source1_f750 = *source_ml_source1_f750;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_f755, source_ml_source1_f756, source_ml_source1_f757, source_ml_source1_f758, source_ml_source1_f759, source_ml_source1_f760, source_ml_source1_f761, source_ml_source1_f762, properties) => {
                                                        let source_ml_source1_f756 = *source_ml_source1_f756;
                                                        if match (instance.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            invalid_kind.clone()
                                                        } else {
                                                            match instance.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_f791, source_ml_source1_f792, source_ml_source1_f793, source_ml_source1_f794, source_ml_source1_f795, source_ml_source1_f796, source_ml_source1_f797, source_ml_source1_f798, source_ml_source1_f799, source_ml_source1_f800) => {
                                                                    let source_ml_source1_f792 = *source_ml_source1_f792;
                                                                    let source_ml_source1_f799 = *source_ml_source1_f799;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_f801, source_ml_source1_f802, source_ml_source1_f803, source_ml_source1_f804, source_ml_source1_f805, source_ml_source1_f806, source_ml_source1_f807, source_ml_source1_f808) => {
                                                                    let source_ml_source1_f802 = *source_ml_source1_f802;
                                                                    let source_ml_source1_f806 = *source_ml_source1_f806;
                                                                    let source_ml_source1_f807 = *source_ml_source1_f807;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_f809, source_ml_source1_f810, source_ml_source1_f811, source_ml_source1_f812, source_ml_source1_f813) => {
                                                                    let source_ml_source1_f810 = *source_ml_source1_f810;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_f814, source_ml_source1_f815, source_ml_source1_f816, source_ml_source1_f817, source_ml_source1_f818, source_ml_source1_f819, source_ml_source1_f820, source_ml_source1_f821, source_ml_source1_f822, source_ml_source1_f823, source_ml_source1_f824) => {
                                                                    let source_ml_source1_f815 = *source_ml_source1_f815;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_f825, source_ml_source1_f826, source_ml_source1_f827, source_ml_source1_f828, source_ml_source1_f829, source_ml_source1_f830) => {
                                                                    let source_ml_source1_f826 = *source_ml_source1_f826;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_f831, source_ml_source1_f832, source_ml_source1_f833, source_ml_source1_f834, source_ml_source1_f835, source_ml_source1_f836) => {
                                                                    let source_ml_source1_f832 = *source_ml_source1_f832;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_f837, source_ml_source1_f838, source_ml_source1_f839, source_ml_source1_f840, source_ml_source1_f841, source_ml_source1_f842) => {
                                                                    let source_ml_source1_f838 = *source_ml_source1_f838;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_f843, source_ml_source1_f844, source_ml_source1_f845, source_ml_source1_f846, source_ml_source1_f847, source_ml_source1_f848) => {
                                                                    let source_ml_source1_f844 = *source_ml_source1_f844;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_f849, prototype, source_ml_source1_f850, source_ml_source1_f851, source_ml_source1_f852, source_ml_source1_f853, source_ml_source1_f854, source_ml_source1_f855, source_ml_source1_f856) => {
                                                                    let prototype = *prototype;
                                                                    {
                                                                        let field = self::ml_array::optional_at(&properties, self::ml_array::number_index(self::find_java_script_heap_directory_property(properties, self::JavaScriptSemanticValue::Text(vec![112f64, 114f64, 111f64, 116f64, 111f64, 116f64, 121f64, 112f64, 101f64]))));
                                                                        if (((((!(match (field.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (match field.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value885) => {
                                                                                source_ml_source1_data_property_value885.clone()
                                                                            }
                                                                        }).9) && !((match field.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value886) => {
                                                                                source_ml_source1_data_property_value886.clone()
                                                                            }
                                                                        }).5)) && !((match field.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value887) => {
                                                                                source_ml_source1_data_property_value887.clone()
                                                                            }
                                                                        }).7)) && {
                                                                            let source_ml_source1_s889 = (match field.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value899) => {
                                                                                    source_ml_source1_data_property_value899.clone()
                                                                                }
                                                                            }).8;
                                                                            match source_ml_source1_s889.clone() {
                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f890) => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f891) => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f892) => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f893) => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f894, source_ml_source1_f895) => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f896) => {
                                                                                    false
                                                                                }
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f897, source_ml_source1_f898) => {
                                                                                    true
                                                                                }
                                                                            }
                                                                        }) && self::test_java_script_semantic_same_value((match field.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value900) => {
                                                                                source_ml_source1_data_property_value900.clone()
                                                                            }
                                                                        }).8, prototype.clone())) {
                                                                            self::ml_string::Text::from_units(vec![])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 99, 108, 97, 115, 115, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 119, 110, 32, 100, 97, 116, 97, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                                        }
                                                                    }
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_f857, source_ml_source1_f858, source_ml_source1_f859, source_ml_source1_f860, source_ml_source1_f861, source_ml_source1_f862, source_ml_source1_f863) => {
                                                                    let source_ml_source1_f858 = *source_ml_source1_f858;
                                                                    let source_ml_source1_f862 = *source_ml_source1_f862;
                                                                    let source_ml_source1_f863 = *source_ml_source1_f863;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_f864, source_ml_source1_f865, source_ml_source1_f866, source_ml_source1_f867, source_ml_source1_f868, source_ml_source1_f869, source_ml_source1_f870) => {
                                                                    let source_ml_source1_f865 = *source_ml_source1_f865;
                                                                    let source_ml_source1_f869 = *source_ml_source1_f869;
                                                                    let source_ml_source1_f870 = *source_ml_source1_f870;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_f871, source_ml_source1_f872, source_ml_source1_f873, source_ml_source1_f874, source_ml_source1_f875, source_ml_source1_f876, source_ml_source1_f877) => {
                                                                    let source_ml_source1_f872 = *source_ml_source1_f872;
                                                                    let source_ml_source1_f876 = *source_ml_source1_f876;
                                                                    let source_ml_source1_f877 = *source_ml_source1_f877;
                                                                    invalid_kind.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_f878, source_ml_source1_f879, source_ml_source1_f880, source_ml_source1_f881, source_ml_source1_f882, source_ml_source1_f883, source_ml_source1_f884) => {
                                                                    let source_ml_source1_f879 = *source_ml_source1_f879;
                                                                    let source_ml_source1_f883 = *source_ml_source1_f883;
                                                                    let source_ml_source1_f884 = *source_ml_source1_f884;
                                                                    invalid_kind.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_f763, source_ml_source1_f764, source_ml_source1_f765, source_ml_source1_f766, source_ml_source1_f767, source_ml_source1_f768, source_ml_source1_f769) => {
                                                        let source_ml_source1_f764 = *source_ml_source1_f764;
                                                        let source_ml_source1_f768 = *source_ml_source1_f768;
                                                        let source_ml_source1_f769 = *source_ml_source1_f769;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_f770, source_ml_source1_f771, source_ml_source1_f772, source_ml_source1_f773, source_ml_source1_f774, source_ml_source1_f775, source_ml_source1_f776) => {
                                                        let source_ml_source1_f771 = *source_ml_source1_f771;
                                                        let source_ml_source1_f775 = *source_ml_source1_f775;
                                                        let source_ml_source1_f776 = *source_ml_source1_f776;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_f777, source_ml_source1_f778, source_ml_source1_f779, source_ml_source1_f780, source_ml_source1_f781, source_ml_source1_f782, source_ml_source1_f783) => {
                                                        let source_ml_source1_f778 = *source_ml_source1_f778;
                                                        let source_ml_source1_f782 = *source_ml_source1_f782;
                                                        let source_ml_source1_f783 = *source_ml_source1_f783;
                                                        invalid_kind.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_f784, source_ml_source1_f785, source_ml_source1_f786, source_ml_source1_f787, source_ml_source1_f788, source_ml_source1_f789, source_ml_source1_f790) => {
                                                        let source_ml_source1_f785 = *source_ml_source1_f785;
                                                        let source_ml_source1_f789 = *source_ml_source1_f789;
                                                        let source_ml_source1_f790 = *source_ml_source1_f790;
                                                        invalid_kind.clone()
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(mut units: Vec<f64>, mut at: f64) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done,
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(bool),
}

pub fn source_ml_temporary_test_java_script_semantic_text_equal_loop2(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut at: f64) -> self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    loop {
        return if (at < length) {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        (-1f64)
                    }
                    self::JavaScriptSemanticValue::Null => {
                        (-1f64)
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f901) => {
                        {
                            let source_ml_source1_f925 = source_ml_source1_f901;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f902) => {
                        {
                            let source_ml_source1_f934 = source_ml_source1_f902;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f903) => {
                        {
                            let source_ml_source1_f943 = source_ml_source1_f903.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Text(units) => {
                        (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f904, source_ml_source1_f905) => {
                        {
                            let source_ml_source1_f953 = source_ml_source1_f904.clone();
                            {
                                let text = source_ml_source1_f905.clone();
                                {
                                    let source_ml_source1_string_unit_operand957 = text.clone();
                                    {
                                        let source_ml_source1_string_unit_operand958 = at;
                                        (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 { {
                                            let index = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if ((index < (0f64)) || (index >= (text_2.length()))) {
                                                f64::NAN
                                            } else {
                                                ({
                                                    let source_ml_source1_string_unit_operand964 = {
                                                        let source_ml_source1_string_range_operand959 = text_2.clone();
                                                        {
                                                            let source_ml_source1_string_range_operand960 = index;
                                                            {
                                                                let source_ml_source1_string_range_operand961 = (index + 1f64);
                                                                {
                                                                    let source_ml_source1_string_range_position962 = source_ml_source1_string_range_operand960;
                                                                    {
                                                                        let source_ml_source1_string_range_position963 = source_ml_source1_string_range_operand961;
                                                                        source_ml_source1_string_range_operand959.slice(source_ml_source1_string_range_position962, source_ml_source1_string_range_position963)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_string_unit_operand965 = 0f64;
                                                        source_ml_source1_string_unit_operand964.code_point_at(source_ml_source1_string_unit_operand965)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source1_string_unit_operand957.clone(), source_ml_source1_string_unit_operand958)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f906) => {
                        {
                            let source_ml_source1_f971 = source_ml_source1_f906.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f907, source_ml_source1_f908) => {
                        {
                            let source_ml_source1_f980 = source_ml_source1_f907.clone();
                            {
                                let source_ml_source1_f981 = source_ml_source1_f908;
                                (-1f64)
                            }
                        }
                    }
                };
                {
                    let b = match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            (-1f64)
                        }
                        self::JavaScriptSemanticValue::Null => {
                            (-1f64)
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f982) => {
                            {
                                let source_ml_source1_f1006 = source_ml_source1_f982;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f983) => {
                            {
                                let source_ml_source1_f1015 = source_ml_source1_f983;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f984) => {
                            {
                                let source_ml_source1_f1024 = source_ml_source1_f984.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Text(units_2) => {
                            (self::ml_array::optional_at(&units_2, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f985, source_ml_source1_f986) => {
                            {
                                let source_ml_source1_f1034 = source_ml_source1_f985.clone();
                                {
                                    let text_2 = source_ml_source1_f986.clone();
                                    {
                                        let source_ml_source1_string_unit_operand1038 = text_2.clone();
                                        {
                                            let source_ml_source1_string_unit_operand1039 = at;
                                            (std::rc::Rc::new(move |text_3: self::ml_string::Text, position: f64| -> f64 { {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if ((index < (0f64)) || (index >= (text_3.length()))) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                        let source_ml_source1_string_unit_operand1045 = {
                                                            let source_ml_source1_string_range_operand1040 = text_3.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand1041 = index;
                                                                {
                                                                    let source_ml_source1_string_range_operand1042 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source1_string_range_position1043 = source_ml_source1_string_range_operand1041;
                                                                        {
                                                                            let source_ml_source1_string_range_position1044 = source_ml_source1_string_range_operand1042;
                                                                            source_ml_source1_string_range_operand1040.slice(source_ml_source1_string_range_position1043, source_ml_source1_string_range_position1044)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_string_unit_operand1046 = 0f64;
                                                            source_ml_source1_string_unit_operand1045.code_point_at(source_ml_source1_string_unit_operand1046)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source1_string_unit_operand1038.clone(), source_ml_source1_string_unit_operand1039)
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f987) => {
                            {
                                let source_ml_source1_f1052 = source_ml_source1_f987.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f988, source_ml_source1_f989) => {
                            {
                                let source_ml_source1_f1061 = source_ml_source1_f988.clone();
                                {
                                    let source_ml_source1_f1062 = source_ml_source1_f989;
                                    (-1f64)
                                }
                            }
                        }
                    };
                    if (a != b) {
                        self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(false)
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
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done
        };
    }
}

pub fn source_ml_temporary_read_java_script_semantic_text_string_loop4(mut text: self::ml_string::Text, mut source_ml_temporary_values3: Vec<f64>, mut source_ml_temporary_index3: self::ml::Big) -> self::ml_string::Text {
    loop {
        return if (source_ml_temporary_index3 < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))) {
            {
                let unit = self::ml_array::at(&source_ml_temporary_values3, source_ml_temporary_index3.to_index());
                {
                    let text_2 = text.concat(&{
                        let source_ml_source1_character_argument1063 = unit;
                        self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source1_character_argument1063]).into_iter().map(|ml_unit| ml_unit as u16).collect())
                    });
                    {
                        let source_ml_temporary_index3_2 = source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            (text, source_ml_temporary_index3) = (text_2.clone(), source_ml_temporary_index3_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            text.clone()
        };
    }
}

pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop5(mut value: self::JavaScriptSemanticValue, mut length: f64, mut units: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < length) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_java_script_semantic_text_unit(value.clone(), at)], units); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (units, at) = (units_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            units
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done,
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(f64),
}

pub fn source_ml_temporary_find_java_script_property_descriptor_entry_loop6(mut properties: Vec<self::JavaScriptOrdinaryProperty>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    loop {
        return if (at < ((properties.len() as f64))) {
            {
                let property = self::ml_array::optional_at(&properties, self::ml_array::number_index(at));
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value(match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1064, _) => {
                        let source_ml_source1_data_property_value1064 = *source_ml_source1_data_property_value1064;
                        source_ml_source1_data_property_value1064.clone()
                    }
                }, key.clone())) {
                    self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(at)
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
            self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin7 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), self::ml_string::Text, self::ml_string::Text, Vec<self::ml_string::Text>)),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result {
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Done,
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(f64),
}

pub fn source_ml_temporary_find_java_script_heap_directory_object_loop8(mut objects: Vec<self::JavaScriptOrdinaryObject>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value1078, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1078.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value1079, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1079.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value1080, _, _, _, _) => {
                                source_ml_source1_data_property_value1080.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value1081, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1081.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value1082, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1082.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value1083, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1083.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value1084, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1084.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value1085, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1085.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1086, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1086.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value1087, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1087.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value1088, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1088.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value1089, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1089.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value1090, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1090.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value1104, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1104.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value1105, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1105.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value1106, _, _, _, _) => {
                                    source_ml_source1_data_property_value1106.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value1107, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1107.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value1108, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1108.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value1109, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1109.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value1110, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1110.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value1111, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1111.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1112, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1112.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value1113, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1113.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value1114, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1114.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value1115, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1115.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value1116, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1116.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result {
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Done,
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(bool),
}

pub fn source_ml_temporary_test_java_script_heap_directory_symbol_loop9(mut symbols: Vec<self::ml::Big>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = self::ml_array::optional_at(&symbols, self::ml_array::number_index(middle));
                    if {
                        let source_ml_source1_comparison_operand1117 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand1118 = None::<()>;
                            match (source_ml_source1_comparison_operand1117.clone(), source_ml_source1_comparison_operand1118.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                        }
                    } {
                        self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(true)
                        } else {
                            if ((symbol.clone().expect("a proven present value")) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop9Done
        };
    }
}

fn ml_main() {
    assert!(((self::test_java_script_ordinary_class_constructor_mode(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]))) == (true)), "assertion 1");
    assert!(((self::test_java_script_ordinary_class_constructor_mode(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) == (false)), "assertion 2");
    assert!(((self::test_java_script_ordinary_class_constructor_mode(self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114]))) == (false)), "assertion 3");
    assert!(((self::test_java_script_ordinary_class_receiver_result(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))) == (true)), "assertion 4");
    assert!(((self::test_java_script_ordinary_class_receiver_result(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) == (true)), "assertion 5");
    assert!(((self::test_java_script_ordinary_class_receiver_result(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) == (false)), "assertion 6");
    assert!(((self::test_java_script_ordinary_class_receiver_result(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]), self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) == (false)), "assertion 7");
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
fn all_literal_ordinary_class_controls() { main(); }
