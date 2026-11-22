pub fn commit_java_script_ordinary_own_property(heap: self::JavaScriptOrdinaryHeap, object: self::JavaScriptOrdinaryObject, key: self::JavaScriptSemanticValue, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptOrdinaryHeap, Option<(Option<bool>,)>, bool, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>> {
    {
        let at = self::find_java_script_ordinary_property(match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value7744, _, _, _, _, _) => {
                source_ml_source1_data_property_value7744.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value7745, _, _, _) => {
                source_ml_source1_data_property_value7745.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value7746) => {
                source_ml_source1_data_property_value7746.clone()
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value7747, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value7747.clone()
            }
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value7748, _) => {
                source_ml_source1_data_property_value7748.clone()
            }
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value7749, _) => {
                source_ml_source1_data_property_value7749.clone()
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value7750, _) => {
                source_ml_source1_data_property_value7750.clone()
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value7751, _) => {
                source_ml_source1_data_property_value7751.clone()
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value7752) => {
                source_ml_source1_data_property_value7752.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value7753, _, _) => {
                source_ml_source1_data_property_value7753.clone()
            }
            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value7754, _, _) => {
                source_ml_source1_data_property_value7754.clone()
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value7755, _, _) => {
                source_ml_source1_data_property_value7755.clone()
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value7756, _, _) => {
                source_ml_source1_data_property_value7756.clone()
            }
        }, key.clone());
        {
            let property = {
                let source_ml_source1_array_read_value7770 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value7757, _, _, _, _, _) => {
                        source_ml_source1_data_property_value7757.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value7758, _, _, _) => {
                        source_ml_source1_data_property_value7758.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value7759) => {
                        source_ml_source1_data_property_value7759.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value7760, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value7760.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value7761, _) => {
                        source_ml_source1_data_property_value7761.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value7762, _) => {
                        source_ml_source1_data_property_value7762.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value7763, _) => {
                        source_ml_source1_data_property_value7763.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value7764, _) => {
                        source_ml_source1_data_property_value7764.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value7765) => {
                        source_ml_source1_data_property_value7765.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value7766, _, _) => {
                        source_ml_source1_data_property_value7766.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value7767, _, _) => {
                        source_ml_source1_data_property_value7767.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value7768, _, _) => {
                        source_ml_source1_data_property_value7768.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value7769, _, _) => {
                        source_ml_source1_data_property_value7769.clone()
                    }
                };
                { let ml_reference_array231 = source_ml_source1_array_read_value7770.clone(); let ml_reference_position231 = at; let ml_reference_projection231 = self::ml_array::optional_at(&*ml_reference_array231.borrow(), self::ml_array::number_index(ml_reference_position231)); ml_reference_projection231 }
            };
            {
                let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::create_java_script_property_descriptor(self::ml_string::Text::from_units(vec![100, 97, 116, 97]), false, false)
                } else {
                    match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value7772) => {
                            source_ml_source1_data_property_value7772.clone()
                        }
                    }
                };
                {
                    let transition = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, source_ml_source1_data_property_value7773, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value7773
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, source_ml_source1_data_property_value7774, _, _, _, _, _) => {
                            source_ml_source1_data_property_value7774
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, source_ml_source1_data_property_value7775, _, _) => {
                            source_ml_source1_data_property_value7775
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, source_ml_source1_data_property_value7776, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value7776
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, source_ml_source1_data_property_value7777, _, _, _) => {
                            source_ml_source1_data_property_value7777
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, source_ml_source1_data_property_value7778, _, _, _) => {
                            source_ml_source1_data_property_value7778
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, source_ml_source1_data_property_value7779, _, _, _) => {
                            source_ml_source1_data_property_value7779
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, source_ml_source1_data_property_value7780, _, _, _) => {
                            source_ml_source1_data_property_value7780
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value7781, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value7781
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, source_ml_source1_data_property_value7782, _, _, _, _) => {
                            source_ml_source1_data_property_value7782
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, source_ml_source1_data_property_value7783, _, _, _, _) => {
                            source_ml_source1_data_property_value7783
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, source_ml_source1_data_property_value7784, _, _, _, _) => {
                            source_ml_source1_data_property_value7784
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, source_ml_source1_data_property_value7785, _, _, _, _) => {
                            source_ml_source1_data_property_value7785
                        }
                    }, current.clone(), proposed.clone());
                    if ((({ let ml_reference_value232 = transition.borrow().clone(); ml_reference_value232 }).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source1_field7788 = ({ let ml_reference_value233 = transition.borrow().clone(); ml_reference_value233 }).2;
                            {
                                let source_ml_source1_field7789 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                (source_ml_source1_field7788.clone(), heap.clone(), None::<(Option<bool>,)>, false, source_ml_source1_field7789.clone())
                            }
                        }))
                    } else {
                        if ((({ let ml_reference_value234 = transition.borrow().clone(); ml_reference_value234 }).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source1_field7793 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                (self::ml_string::Text::from_units(vec![]), heap.clone(), None::<(Option<bool>,)>, true, source_ml_source1_field7793.clone())
                            }))
                        } else {
                            if ((at < (0f64)) && (({ let ml_reference_array235 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value7807, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value7807.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value7808, _, _, _) => {
                                    source_ml_source1_data_property_value7808.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value7809) => {
                                    source_ml_source1_data_property_value7809.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value7810, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value7810.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value7811, _) => {
                                    source_ml_source1_data_property_value7811.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value7812, _) => {
                                    source_ml_source1_data_property_value7812.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value7813, _) => {
                                    source_ml_source1_data_property_value7813.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value7814, _) => {
                                    source_ml_source1_data_property_value7814.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value7815) => {
                                    source_ml_source1_data_property_value7815.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value7816, _, _) => {
                                    source_ml_source1_data_property_value7816.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value7817, _, _) => {
                                    source_ml_source1_data_property_value7817.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value7818, _, _) => {
                                    source_ml_source1_data_property_value7818.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value7819, _, _) => {
                                    source_ml_source1_data_property_value7819.clone()
                                }
                            }; let ml_reference_projection235 = ml_reference_array235.borrow().len() as f64; ml_reference_projection235 }) >= (4096f64))) {
                                std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source1_field7823 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                    {
                                        let source_ml_source1_field7824 = Some((Some(true),));
                                        (self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]), heap.clone(), source_ml_source1_field7824.clone(), false, source_ml_source1_field7823.clone())
                                    }
                                }))
                            } else {
                                {
                                    let count = 0f64;
                                    {
                                        let source_ml_temporary_values106 = match heap.clone() {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value7825, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value7825.clone()
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index106 = self::ml::Big::from_i128(0);
                                            {
                                                let count_2 = self::source_ml_temporary_commit_java_script_ordinary_own_property_loop107(count, source_ml_temporary_values106.clone(), source_ml_temporary_index106.clone());
                                                if ((at < (0f64)) && (count_2 >= (16384f64))) {
                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                        let source_ml_source1_field7829 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
                                                        {
                                                            let source_ml_source1_field7830 = Some((Some(true),));
                                                            (self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]), heap.clone(), source_ml_source1_field7830.clone(), false, source_ml_source1_field7829.clone())
                                                        }
                                                    }))
                                                } else {
                                                    {
                                                        let updated = self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), ({ let ml_reference_value236 = transition.borrow().clone(); ml_reference_value236 }).0);
                                                        {
                                                            let properties = if (at < (0f64)) {
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_concat_value7844 = match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value7831, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value7831.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value7832, _, _, _) => {
                                                                            source_ml_source1_data_property_value7832.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value7833) => {
                                                                            source_ml_source1_data_property_value7833.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value7834, _, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value7834.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value7835, _) => {
                                                                            source_ml_source1_data_property_value7835.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value7836, _) => {
                                                                            source_ml_source1_data_property_value7836.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value7837, _) => {
                                                                            source_ml_source1_data_property_value7837.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value7838, _) => {
                                                                            source_ml_source1_data_property_value7838.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value7839) => {
                                                                            source_ml_source1_data_property_value7839.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value7840, _, _) => {
                                                                            source_ml_source1_data_property_value7840.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value7841, _, _) => {
                                                                            source_ml_source1_data_property_value7841.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value7842, _, _) => {
                                                                            source_ml_source1_data_property_value7842.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value7843, _, _) => {
                                                                            source_ml_source1_data_property_value7843.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_concat_value7845 = std::rc::Rc::new(std::cell::RefCell::new(vec![updated.clone()]));
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryProperty>::new(), { let ml_reference_value237 = source_ml_source1_concat_value7844.borrow().clone(); ml_reference_value237 }), { let ml_reference_value238 = source_ml_source1_concat_value7845.borrow().clone(); ml_reference_value238 })
                                                                    }
                                                                }))
                                                            } else {
                                                                std::rc::Rc::new(std::cell::RefCell::new({ let ml_values239 = { let ml_reference_value240 = (match object.clone() {
                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value7846, _, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value7846.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value7847, _, _, _) => {
                                                                        source_ml_source1_data_property_value7847.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value7848) => {
                                                                        source_ml_source1_data_property_value7848.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value7849, _, _, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value7849.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value7850, _) => {
                                                                        source_ml_source1_data_property_value7850.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value7851, _) => {
                                                                        source_ml_source1_data_property_value7851.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value7852, _) => {
                                                                        source_ml_source1_data_property_value7852.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value7853, _) => {
                                                                        source_ml_source1_data_property_value7853.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value7854) => {
                                                                        source_ml_source1_data_property_value7854.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value7855, _, _) => {
                                                                        source_ml_source1_data_property_value7855.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value7856, _, _) => {
                                                                        source_ml_source1_data_property_value7856.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value7857, _, _) => {
                                                                        source_ml_source1_data_property_value7857.clone()
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value7858, _, _) => {
                                                                        source_ml_source1_data_property_value7858.clone()
                                                                    }
                                                                }).borrow().clone(); ml_reference_value240 }; let ml_callback239 = { let at = at; let updated = updated.clone(); std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> self::JavaScriptOrdinaryProperty { if (position == at) {
                                                                    updated.clone()
                                                                } else {
                                                                    entry.clone()
                                                                } }) }; ml_values239.iter().cloned().enumerate().map(|(ml_index239, ml_value239)| ml_callback239(ml_value239, (ml_index239 as f64))).collect::<Vec<_>>() }))
                                                            };
                                                            {
                                                                let replacement = match object.clone() {
                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field7859, source_ml_source1_nominal_field7860, source_ml_source1_nominal_field7861, source_ml_source1_nominal_field7862, _, source_ml_source1_nominal_field7864, source_ml_source1_nominal_field7865, source_ml_source1_nominal_field7866, source_ml_source1_nominal_field7867, source_ml_source1_nominal_field7868) => {
                                                                        let source_ml_source1_nominal_field7860 = *source_ml_source1_nominal_field7860;
                                                                        let source_ml_source1_nominal_field7867 = *source_ml_source1_nominal_field7867;
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field7859.clone(), Box::new(source_ml_source1_nominal_field7860.clone()), source_ml_source1_nominal_field7861, source_ml_source1_nominal_field7862, properties.clone(), source_ml_source1_nominal_field7864.clone(), source_ml_source1_nominal_field7865, source_ml_source1_nominal_field7866.clone(), Box::new(source_ml_source1_nominal_field7867.clone()), source_ml_source1_nominal_field7868.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field7870, source_ml_source1_nominal_field7871, source_ml_source1_nominal_field7872, source_ml_source1_nominal_field7873, _, source_ml_source1_nominal_field7875, source_ml_source1_nominal_field7876, source_ml_source1_nominal_field7877) => {
                                                                        let source_ml_source1_nominal_field7871 = *source_ml_source1_nominal_field7871;
                                                                        let source_ml_source1_nominal_field7875 = *source_ml_source1_nominal_field7875;
                                                                        let source_ml_source1_nominal_field7876 = *source_ml_source1_nominal_field7876;
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field7870.clone(), Box::new(source_ml_source1_nominal_field7871.clone()), source_ml_source1_nominal_field7872, source_ml_source1_nominal_field7873, properties.clone(), Box::new(source_ml_source1_nominal_field7875.clone()), Box::new(source_ml_source1_nominal_field7876.clone()), source_ml_source1_nominal_field7877.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field7879, source_ml_source1_nominal_field7880, source_ml_source1_nominal_field7881, source_ml_source1_nominal_field7882, _) => {
                                                                        let source_ml_source1_nominal_field7880 = *source_ml_source1_nominal_field7880;
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field7879.clone(), Box::new(source_ml_source1_nominal_field7880.clone()), source_ml_source1_nominal_field7881, source_ml_source1_nominal_field7882, properties.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field7885, source_ml_source1_nominal_field7886, source_ml_source1_nominal_field7887, source_ml_source1_nominal_field7888, _, source_ml_source1_nominal_field7890, source_ml_source1_nominal_field7891, source_ml_source1_nominal_field7892, source_ml_source1_nominal_field7893, source_ml_source1_nominal_field7894, source_ml_source1_nominal_field7895) => {
                                                                        let source_ml_source1_nominal_field7886 = *source_ml_source1_nominal_field7886;
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field7885.clone(), Box::new(source_ml_source1_nominal_field7886.clone()), source_ml_source1_nominal_field7887, source_ml_source1_nominal_field7888, properties.clone(), source_ml_source1_nominal_field7890.clone(), source_ml_source1_nominal_field7891, source_ml_source1_nominal_field7892.clone(), source_ml_source1_nominal_field7893.clone(), source_ml_source1_nominal_field7894.clone(), source_ml_source1_nominal_field7895.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field7897, source_ml_source1_nominal_field7898, source_ml_source1_nominal_field7899, source_ml_source1_nominal_field7900, _, source_ml_source1_nominal_field7902) => {
                                                                        let source_ml_source1_nominal_field7898 = *source_ml_source1_nominal_field7898;
                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field7897.clone(), Box::new(source_ml_source1_nominal_field7898.clone()), source_ml_source1_nominal_field7899, source_ml_source1_nominal_field7900, properties.clone(), source_ml_source1_nominal_field7902.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field7904, source_ml_source1_nominal_field7905, source_ml_source1_nominal_field7906, source_ml_source1_nominal_field7907, _, source_ml_source1_nominal_field7909) => {
                                                                        let source_ml_source1_nominal_field7905 = *source_ml_source1_nominal_field7905;
                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field7904.clone(), Box::new(source_ml_source1_nominal_field7905.clone()), source_ml_source1_nominal_field7906, source_ml_source1_nominal_field7907, properties.clone(), source_ml_source1_nominal_field7909.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field7911, source_ml_source1_nominal_field7912, source_ml_source1_nominal_field7913, source_ml_source1_nominal_field7914, _, source_ml_source1_nominal_field7916) => {
                                                                        let source_ml_source1_nominal_field7912 = *source_ml_source1_nominal_field7912;
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field7911.clone(), Box::new(source_ml_source1_nominal_field7912.clone()), source_ml_source1_nominal_field7913, source_ml_source1_nominal_field7914, properties.clone(), source_ml_source1_nominal_field7916.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field7918, source_ml_source1_nominal_field7919, source_ml_source1_nominal_field7920, source_ml_source1_nominal_field7921, _, source_ml_source1_nominal_field7923) => {
                                                                        let source_ml_source1_nominal_field7919 = *source_ml_source1_nominal_field7919;
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field7918.clone(), Box::new(source_ml_source1_nominal_field7919.clone()), source_ml_source1_nominal_field7920, source_ml_source1_nominal_field7921, properties.clone(), source_ml_source1_nominal_field7923.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field7925, source_ml_source1_nominal_field7926, source_ml_source1_nominal_field7927, source_ml_source1_nominal_field7928, source_ml_source1_nominal_field7929, source_ml_source1_nominal_field7930, source_ml_source1_nominal_field7931, source_ml_source1_nominal_field7932, _) => {
                                                                        let source_ml_source1_nominal_field7926 = *source_ml_source1_nominal_field7926;
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field7925.clone(), Box::new(source_ml_source1_nominal_field7926.clone()), source_ml_source1_nominal_field7927, source_ml_source1_nominal_field7928.clone(), source_ml_source1_nominal_field7929.clone(), source_ml_source1_nominal_field7930.clone(), source_ml_source1_nominal_field7931.clone(), source_ml_source1_nominal_field7932, properties.clone())
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field7935, source_ml_source1_nominal_field7936, source_ml_source1_nominal_field7937, source_ml_source1_nominal_field7938, _, source_ml_source1_nominal_field7940, source_ml_source1_nominal_field7941) => {
                                                                        let source_ml_source1_nominal_field7936 = *source_ml_source1_nominal_field7936;
                                                                        let source_ml_source1_nominal_field7940 = *source_ml_source1_nominal_field7940;
                                                                        let source_ml_source1_nominal_field7941 = *source_ml_source1_nominal_field7941;
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field7935.clone(), Box::new(source_ml_source1_nominal_field7936.clone()), source_ml_source1_nominal_field7937, source_ml_source1_nominal_field7938, properties.clone(), Box::new(source_ml_source1_nominal_field7940.clone()), Box::new(source_ml_source1_nominal_field7941.clone()))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field7943, source_ml_source1_nominal_field7944, source_ml_source1_nominal_field7945, source_ml_source1_nominal_field7946, _, source_ml_source1_nominal_field7948, source_ml_source1_nominal_field7949) => {
                                                                        let source_ml_source1_nominal_field7944 = *source_ml_source1_nominal_field7944;
                                                                        let source_ml_source1_nominal_field7948 = *source_ml_source1_nominal_field7948;
                                                                        let source_ml_source1_nominal_field7949 = *source_ml_source1_nominal_field7949;
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field7943.clone(), Box::new(source_ml_source1_nominal_field7944.clone()), source_ml_source1_nominal_field7945, source_ml_source1_nominal_field7946, properties.clone(), Box::new(source_ml_source1_nominal_field7948.clone()), Box::new(source_ml_source1_nominal_field7949.clone()))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field7951, source_ml_source1_nominal_field7952, source_ml_source1_nominal_field7953, source_ml_source1_nominal_field7954, _, source_ml_source1_nominal_field7956, source_ml_source1_nominal_field7957) => {
                                                                        let source_ml_source1_nominal_field7952 = *source_ml_source1_nominal_field7952;
                                                                        let source_ml_source1_nominal_field7956 = *source_ml_source1_nominal_field7956;
                                                                        let source_ml_source1_nominal_field7957 = *source_ml_source1_nominal_field7957;
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field7951.clone(), Box::new(source_ml_source1_nominal_field7952.clone()), source_ml_source1_nominal_field7953, source_ml_source1_nominal_field7954, properties.clone(), Box::new(source_ml_source1_nominal_field7956.clone()), Box::new(source_ml_source1_nominal_field7957.clone()))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field7959, source_ml_source1_nominal_field7960, source_ml_source1_nominal_field7961, source_ml_source1_nominal_field7962, _, source_ml_source1_nominal_field7964, source_ml_source1_nominal_field7965) => {
                                                                        let source_ml_source1_nominal_field7960 = *source_ml_source1_nominal_field7960;
                                                                        let source_ml_source1_nominal_field7964 = *source_ml_source1_nominal_field7964;
                                                                        let source_ml_source1_nominal_field7965 = *source_ml_source1_nominal_field7965;
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field7959.clone(), Box::new(source_ml_source1_nominal_field7960.clone()), source_ml_source1_nominal_field7961, source_ml_source1_nominal_field7962, properties.clone(), Box::new(source_ml_source1_nominal_field7964.clone()), Box::new(source_ml_source1_nominal_field7965.clone()))
                                                                    }
                                                                };
                                                                std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source1_field7967 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                    {
                                                                        let source_ml_source1_field7970 = ({ let ml_reference_value241 = transition.borrow().clone(); ml_reference_value241 }).3;
                                                                        (self::ml_string::Text::from_units(vec![]), source_ml_source1_field7967.clone(), None::<(Option<bool>,)>, true, source_ml_source1_field7970.clone())
                                                                    }
                                                                }))
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

pub fn delete_java_script_ordinary_property(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool)>> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                {
                    let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                    {
                        let error = if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                            heap_error.clone()
                        } else {
                            key_error.clone()
                        };
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            std::rc::Rc::new(std::cell::RefCell::new((error.clone(), heap.clone(), false)))
                        } else {
                            {
                                let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                {
                                    let object = {
                                        let source_ml_source1_array_read_value7976 = shared_constructor_field1.clone();
                                        { let ml_reference_array242 = source_ml_source1_array_read_value7976.clone(); let ml_reference_position242 = index; let ml_reference_projection242 = self::ml_array::optional_at(&*ml_reference_array242.borrow(), self::ml_array::number_index(ml_reference_position242)); ml_reference_projection242 }
                                    };
                                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![68, 101, 108, 101, 116, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]), heap.clone(), false)))
                                    } else {
                                        if ((((((((match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        } && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) && match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) {
                                            std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 100, 101, 108, 101, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), false)))
                                        } else {
                                            {
                                                let at = self::find_java_script_ordinary_property((((match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value8839, _, _, _, _, _) => {
                                                        Some((source_ml_source1_data_property_value8839.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value8840, _, _, _) => {
                                                        Some((source_ml_source1_data_property_value8840.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value8841) => {
                                                        Some((source_ml_source1_data_property_value8841.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value8842, _, _, _, _, _, _) => {
                                                        Some((source_ml_source1_data_property_value8842.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value8843, _) => {
                                                        Some((source_ml_source1_data_property_value8843.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value8844, _) => {
                                                        Some((source_ml_source1_data_property_value8844.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value8845, _) => {
                                                        Some((source_ml_source1_data_property_value8845.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value8846, _) => {
                                                        Some((source_ml_source1_data_property_value8846.clone(),))
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value8847) => {
                                                        Some((source_ml_source1_data_property_value8847.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone(), key.clone());
                                                {
                                                    let property = {
                                                        let source_ml_source1_array_read_value8857 = (((match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value8848, _, _, _, _, _) => {
                                                                Some((source_ml_source1_data_property_value8848.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value8849, _, _, _) => {
                                                                Some((source_ml_source1_data_property_value8849.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value8850) => {
                                                                Some((source_ml_source1_data_property_value8850.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value8851, _, _, _, _, _, _) => {
                                                                Some((source_ml_source1_data_property_value8851.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value8852, _) => {
                                                                Some((source_ml_source1_data_property_value8852.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value8853, _) => {
                                                                Some((source_ml_source1_data_property_value8853.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value8854, _) => {
                                                                Some((source_ml_source1_data_property_value8854.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value8855, _) => {
                                                                Some((source_ml_source1_data_property_value8855.clone(),))
                                                            }
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value8856) => {
                                                                Some((source_ml_source1_data_property_value8856.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone();
                                                        { let ml_reference_array243 = source_ml_source1_array_read_value8857.clone(); let ml_reference_position243 = at; let ml_reference_projection243 = self::ml_array::optional_at(&*ml_reference_array243.borrow(), self::ml_array::number_index(ml_reference_position243)); ml_reference_projection243 }
                                                    };
                                                    if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), heap.clone(), true)))
                                                    } else {
                                                        if !(({ let ml_reference_value244 = (match property.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value8862) => {
                                                                source_ml_source1_data_property_value8862.clone()
                                                            }
                                                        }).borrow().clone(); ml_reference_value244 }).0) {
                                                            std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), heap.clone(), false)))
                                                        } else {
                                                            {
                                                                let properties = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values245 = { let ml_reference_value246 = ((((match object.clone().expect("a proven present value") {
                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value8866, _, _, _, _, _) => {
                                                                        Some((source_ml_source1_data_property_value8866.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value8867, _, _, _) => {
                                                                        Some((source_ml_source1_data_property_value8867.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value8868) => {
                                                                        Some((source_ml_source1_data_property_value8868.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value8869, _, _, _, _, _, _) => {
                                                                        Some((source_ml_source1_data_property_value8869.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value8870, _) => {
                                                                        Some((source_ml_source1_data_property_value8870.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value8871, _) => {
                                                                        Some((source_ml_source1_data_property_value8871.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value8872, _) => {
                                                                        Some((source_ml_source1_data_property_value8872.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value8873, _) => {
                                                                        Some((source_ml_source1_data_property_value8873.clone(),))
                                                                    }
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value8874) => {
                                                                        Some((source_ml_source1_data_property_value8874.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone()).borrow().clone(); ml_reference_value246 }; let ml_callback245 = { let at = at; std::rc::Rc::new(move |entry: self::JavaScriptOrdinaryProperty, position: f64| -> bool { (position != at) }) }; ml_values245.iter().cloned().enumerate().filter_map(|(ml_index245, ml_value245)| if ml_callback245(ml_value245.clone(), (ml_index245 as f64)) { Some(ml_value245) } else { None }).collect::<Vec<_>>() }));
                                                                {
                                                                    let replacement = match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field8875, source_ml_source1_nominal_field8876, source_ml_source1_nominal_field8877, source_ml_source1_nominal_field8878, _, source_ml_source1_nominal_field8880, source_ml_source1_nominal_field8881, source_ml_source1_nominal_field8882, source_ml_source1_nominal_field8883, source_ml_source1_nominal_field8884) => {
                                                                            let source_ml_source1_nominal_field8876 = *source_ml_source1_nominal_field8876;
                                                                            let source_ml_source1_nominal_field8883 = *source_ml_source1_nominal_field8883;
                                                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field8875.clone(), Box::new(source_ml_source1_nominal_field8876.clone()), source_ml_source1_nominal_field8877, source_ml_source1_nominal_field8878, properties.clone(), source_ml_source1_nominal_field8880.clone(), source_ml_source1_nominal_field8881, source_ml_source1_nominal_field8882.clone(), Box::new(source_ml_source1_nominal_field8883.clone()), source_ml_source1_nominal_field8884.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field8886, source_ml_source1_nominal_field8887, source_ml_source1_nominal_field8888, source_ml_source1_nominal_field8889, _, source_ml_source1_nominal_field8891, source_ml_source1_nominal_field8892, source_ml_source1_nominal_field8893) => {
                                                                            let source_ml_source1_nominal_field8887 = *source_ml_source1_nominal_field8887;
                                                                            let source_ml_source1_nominal_field8891 = *source_ml_source1_nominal_field8891;
                                                                            let source_ml_source1_nominal_field8892 = *source_ml_source1_nominal_field8892;
                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field8886.clone(), Box::new(source_ml_source1_nominal_field8887.clone()), source_ml_source1_nominal_field8888, source_ml_source1_nominal_field8889, properties.clone(), Box::new(source_ml_source1_nominal_field8891.clone()), Box::new(source_ml_source1_nominal_field8892.clone()), source_ml_source1_nominal_field8893.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field8895, source_ml_source1_nominal_field8896, source_ml_source1_nominal_field8897, source_ml_source1_nominal_field8898, _) => {
                                                                            let source_ml_source1_nominal_field8896 = *source_ml_source1_nominal_field8896;
                                                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field8895.clone(), Box::new(source_ml_source1_nominal_field8896.clone()), source_ml_source1_nominal_field8897, source_ml_source1_nominal_field8898, properties.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field8901, source_ml_source1_nominal_field8902, source_ml_source1_nominal_field8903, source_ml_source1_nominal_field8904, _, source_ml_source1_nominal_field8906, source_ml_source1_nominal_field8907, source_ml_source1_nominal_field8908, source_ml_source1_nominal_field8909, source_ml_source1_nominal_field8910, source_ml_source1_nominal_field8911) => {
                                                                            let source_ml_source1_nominal_field8902 = *source_ml_source1_nominal_field8902;
                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field8901.clone(), Box::new(source_ml_source1_nominal_field8902.clone()), source_ml_source1_nominal_field8903, source_ml_source1_nominal_field8904, properties.clone(), source_ml_source1_nominal_field8906.clone(), source_ml_source1_nominal_field8907, source_ml_source1_nominal_field8908.clone(), source_ml_source1_nominal_field8909.clone(), source_ml_source1_nominal_field8910.clone(), source_ml_source1_nominal_field8911.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field8913, source_ml_source1_nominal_field8914, source_ml_source1_nominal_field8915, source_ml_source1_nominal_field8916, _, source_ml_source1_nominal_field8918) => {
                                                                            let source_ml_source1_nominal_field8914 = *source_ml_source1_nominal_field8914;
                                                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field8913.clone(), Box::new(source_ml_source1_nominal_field8914.clone()), source_ml_source1_nominal_field8915, source_ml_source1_nominal_field8916, properties.clone(), source_ml_source1_nominal_field8918.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field8920, source_ml_source1_nominal_field8921, source_ml_source1_nominal_field8922, source_ml_source1_nominal_field8923, _, source_ml_source1_nominal_field8925) => {
                                                                            let source_ml_source1_nominal_field8921 = *source_ml_source1_nominal_field8921;
                                                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field8920.clone(), Box::new(source_ml_source1_nominal_field8921.clone()), source_ml_source1_nominal_field8922, source_ml_source1_nominal_field8923, properties.clone(), source_ml_source1_nominal_field8925.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field8927, source_ml_source1_nominal_field8928, source_ml_source1_nominal_field8929, source_ml_source1_nominal_field8930, _, source_ml_source1_nominal_field8932) => {
                                                                            let source_ml_source1_nominal_field8928 = *source_ml_source1_nominal_field8928;
                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field8927.clone(), Box::new(source_ml_source1_nominal_field8928.clone()), source_ml_source1_nominal_field8929, source_ml_source1_nominal_field8930, properties.clone(), source_ml_source1_nominal_field8932.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field8934, source_ml_source1_nominal_field8935, source_ml_source1_nominal_field8936, source_ml_source1_nominal_field8937, _, source_ml_source1_nominal_field8939) => {
                                                                            let source_ml_source1_nominal_field8935 = *source_ml_source1_nominal_field8935;
                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field8934.clone(), Box::new(source_ml_source1_nominal_field8935.clone()), source_ml_source1_nominal_field8936, source_ml_source1_nominal_field8937, properties.clone(), source_ml_source1_nominal_field8939.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field8941, source_ml_source1_nominal_field8942, source_ml_source1_nominal_field8943, source_ml_source1_nominal_field8944, source_ml_source1_nominal_field8945, source_ml_source1_nominal_field8946, source_ml_source1_nominal_field8947, source_ml_source1_nominal_field8948, _) => {
                                                                            let source_ml_source1_nominal_field8942 = *source_ml_source1_nominal_field8942;
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field8941.clone(), Box::new(source_ml_source1_nominal_field8942.clone()), source_ml_source1_nominal_field8943, source_ml_source1_nominal_field8944.clone(), source_ml_source1_nominal_field8945.clone(), source_ml_source1_nominal_field8946.clone(), source_ml_source1_nominal_field8947.clone(), source_ml_source1_nominal_field8948, properties.clone())
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field8951, source_ml_source1_nominal_field8952, source_ml_source1_nominal_field8953, source_ml_source1_nominal_field8954, _, source_ml_source1_nominal_field8956, source_ml_source1_nominal_field8957) => {
                                                                            let source_ml_source1_nominal_field8952 = *source_ml_source1_nominal_field8952;
                                                                            let source_ml_source1_nominal_field8956 = *source_ml_source1_nominal_field8956;
                                                                            let source_ml_source1_nominal_field8957 = *source_ml_source1_nominal_field8957;
                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field8951.clone(), Box::new(source_ml_source1_nominal_field8952.clone()), source_ml_source1_nominal_field8953, source_ml_source1_nominal_field8954, properties.clone(), Box::new(source_ml_source1_nominal_field8956.clone()), Box::new(source_ml_source1_nominal_field8957.clone()))
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field8959, source_ml_source1_nominal_field8960, source_ml_source1_nominal_field8961, source_ml_source1_nominal_field8962, _, source_ml_source1_nominal_field8964, source_ml_source1_nominal_field8965) => {
                                                                            let source_ml_source1_nominal_field8960 = *source_ml_source1_nominal_field8960;
                                                                            let source_ml_source1_nominal_field8964 = *source_ml_source1_nominal_field8964;
                                                                            let source_ml_source1_nominal_field8965 = *source_ml_source1_nominal_field8965;
                                                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field8959.clone(), Box::new(source_ml_source1_nominal_field8960.clone()), source_ml_source1_nominal_field8961, source_ml_source1_nominal_field8962, properties.clone(), Box::new(source_ml_source1_nominal_field8964.clone()), Box::new(source_ml_source1_nominal_field8965.clone()))
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field8967, source_ml_source1_nominal_field8968, source_ml_source1_nominal_field8969, source_ml_source1_nominal_field8970, _, source_ml_source1_nominal_field8972, source_ml_source1_nominal_field8973) => {
                                                                            let source_ml_source1_nominal_field8968 = *source_ml_source1_nominal_field8968;
                                                                            let source_ml_source1_nominal_field8972 = *source_ml_source1_nominal_field8972;
                                                                            let source_ml_source1_nominal_field8973 = *source_ml_source1_nominal_field8973;
                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field8967.clone(), Box::new(source_ml_source1_nominal_field8968.clone()), source_ml_source1_nominal_field8969, source_ml_source1_nominal_field8970, properties.clone(), Box::new(source_ml_source1_nominal_field8972.clone()), Box::new(source_ml_source1_nominal_field8973.clone()))
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field8975, source_ml_source1_nominal_field8976, source_ml_source1_nominal_field8977, source_ml_source1_nominal_field8978, _, source_ml_source1_nominal_field8980, source_ml_source1_nominal_field8981) => {
                                                                            let source_ml_source1_nominal_field8976 = *source_ml_source1_nominal_field8976;
                                                                            let source_ml_source1_nominal_field8980 = *source_ml_source1_nominal_field8980;
                                                                            let source_ml_source1_nominal_field8981 = *source_ml_source1_nominal_field8981;
                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field8975.clone(), Box::new(source_ml_source1_nominal_field8976.clone()), source_ml_source1_nominal_field8977, source_ml_source1_nominal_field8978, properties.clone(), Box::new(source_ml_source1_nominal_field8980.clone()), Box::new(source_ml_source1_nominal_field8981.clone()))
                                                                        }
                                                                    };
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source1_field8983 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                                                        (self::ml_string::Text::from_units(vec![]), source_ml_source1_field8983.clone(), true)
                                                                    }))
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

pub fn prevent_java_script_ordinary_extensions(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool)>> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            {
                let error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    std::rc::Rc::new(std::cell::RefCell::new((error.clone(), heap.clone(), false)))
                } else {
                    {
                        let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                        {
                            let object = {
                                let source_ml_source1_array_read_value8991 = shared_constructor_field1.clone();
                                { let ml_reference_array247 = source_ml_source1_array_read_value8991.clone(); let ml_reference_position247 = index; let ml_reference_projection247 = self::ml_array::optional_at(&*ml_reference_array247.borrow(), self::ml_array::number_index(ml_reference_position247)); ml_reference_projection247 }
                            };
                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![80, 114, 101, 118, 101, 110, 116, 69, 120, 116, 101, 110, 115, 105, 111, 110, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]), heap.clone(), false)))
                            } else {
                                if ((((((((match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                } && match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                }) && match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                }) && match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                }) && match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                }) && match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                }) && match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                }) && match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                }) && match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                }) {
                                    std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 101, 120, 116, 101, 110, 115, 105, 98, 105, 108, 105, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), heap.clone(), false)))
                                } else {
                                    {
                                        let replacement = match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field9854, source_ml_source1_nominal_field9855, _, source_ml_source1_nominal_field9857, source_ml_source1_nominal_field9858, source_ml_source1_nominal_field9859, source_ml_source1_nominal_field9860, source_ml_source1_nominal_field9861, source_ml_source1_nominal_field9862, source_ml_source1_nominal_field9863) => {
                                                let source_ml_source1_nominal_field9855 = *source_ml_source1_nominal_field9855;
                                                let source_ml_source1_nominal_field9862 = *source_ml_source1_nominal_field9862;
                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field9854.clone(), Box::new(source_ml_source1_nominal_field9855.clone()), false, source_ml_source1_nominal_field9857, source_ml_source1_nominal_field9858.clone(), source_ml_source1_nominal_field9859.clone(), source_ml_source1_nominal_field9860, source_ml_source1_nominal_field9861.clone(), Box::new(source_ml_source1_nominal_field9862.clone()), source_ml_source1_nominal_field9863.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field9865, source_ml_source1_nominal_field9866, _, source_ml_source1_nominal_field9868, source_ml_source1_nominal_field9869, source_ml_source1_nominal_field9870, source_ml_source1_nominal_field9871, source_ml_source1_nominal_field9872) => {
                                                let source_ml_source1_nominal_field9866 = *source_ml_source1_nominal_field9866;
                                                let source_ml_source1_nominal_field9870 = *source_ml_source1_nominal_field9870;
                                                let source_ml_source1_nominal_field9871 = *source_ml_source1_nominal_field9871;
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field9865.clone(), Box::new(source_ml_source1_nominal_field9866.clone()), false, source_ml_source1_nominal_field9868, source_ml_source1_nominal_field9869.clone(), Box::new(source_ml_source1_nominal_field9870.clone()), Box::new(source_ml_source1_nominal_field9871.clone()), source_ml_source1_nominal_field9872.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field9874, source_ml_source1_nominal_field9875, _, source_ml_source1_nominal_field9877, source_ml_source1_nominal_field9878) => {
                                                let source_ml_source1_nominal_field9875 = *source_ml_source1_nominal_field9875;
                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field9874.clone(), Box::new(source_ml_source1_nominal_field9875.clone()), false, source_ml_source1_nominal_field9877, source_ml_source1_nominal_field9878.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field9880, source_ml_source1_nominal_field9881, _, source_ml_source1_nominal_field9883, source_ml_source1_nominal_field9884, source_ml_source1_nominal_field9885, source_ml_source1_nominal_field9886, source_ml_source1_nominal_field9887, source_ml_source1_nominal_field9888, source_ml_source1_nominal_field9889, source_ml_source1_nominal_field9890) => {
                                                let source_ml_source1_nominal_field9881 = *source_ml_source1_nominal_field9881;
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field9880.clone(), Box::new(source_ml_source1_nominal_field9881.clone()), false, source_ml_source1_nominal_field9883, source_ml_source1_nominal_field9884.clone(), source_ml_source1_nominal_field9885.clone(), source_ml_source1_nominal_field9886, source_ml_source1_nominal_field9887.clone(), source_ml_source1_nominal_field9888.clone(), source_ml_source1_nominal_field9889.clone(), source_ml_source1_nominal_field9890.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field9892, source_ml_source1_nominal_field9893, _, source_ml_source1_nominal_field9895, source_ml_source1_nominal_field9896, source_ml_source1_nominal_field9897) => {
                                                let source_ml_source1_nominal_field9893 = *source_ml_source1_nominal_field9893;
                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field9892.clone(), Box::new(source_ml_source1_nominal_field9893.clone()), false, source_ml_source1_nominal_field9895, source_ml_source1_nominal_field9896.clone(), source_ml_source1_nominal_field9897.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field9899, source_ml_source1_nominal_field9900, _, source_ml_source1_nominal_field9902, source_ml_source1_nominal_field9903, source_ml_source1_nominal_field9904) => {
                                                let source_ml_source1_nominal_field9900 = *source_ml_source1_nominal_field9900;
                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field9899.clone(), Box::new(source_ml_source1_nominal_field9900.clone()), false, source_ml_source1_nominal_field9902, source_ml_source1_nominal_field9903.clone(), source_ml_source1_nominal_field9904.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field9906, source_ml_source1_nominal_field9907, _, source_ml_source1_nominal_field9909, source_ml_source1_nominal_field9910, source_ml_source1_nominal_field9911) => {
                                                let source_ml_source1_nominal_field9907 = *source_ml_source1_nominal_field9907;
                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field9906.clone(), Box::new(source_ml_source1_nominal_field9907.clone()), false, source_ml_source1_nominal_field9909, source_ml_source1_nominal_field9910.clone(), source_ml_source1_nominal_field9911.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field9913, source_ml_source1_nominal_field9914, _, source_ml_source1_nominal_field9916, source_ml_source1_nominal_field9917, source_ml_source1_nominal_field9918) => {
                                                let source_ml_source1_nominal_field9914 = *source_ml_source1_nominal_field9914;
                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field9913.clone(), Box::new(source_ml_source1_nominal_field9914.clone()), false, source_ml_source1_nominal_field9916, source_ml_source1_nominal_field9917.clone(), source_ml_source1_nominal_field9918.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field9920, source_ml_source1_nominal_field9921, _, source_ml_source1_nominal_field9923, source_ml_source1_nominal_field9924, source_ml_source1_nominal_field9925, source_ml_source1_nominal_field9926, source_ml_source1_nominal_field9927, source_ml_source1_nominal_field9928) => {
                                                let source_ml_source1_nominal_field9921 = *source_ml_source1_nominal_field9921;
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field9920.clone(), Box::new(source_ml_source1_nominal_field9921.clone()), false, source_ml_source1_nominal_field9923.clone(), source_ml_source1_nominal_field9924.clone(), source_ml_source1_nominal_field9925.clone(), source_ml_source1_nominal_field9926.clone(), source_ml_source1_nominal_field9927, source_ml_source1_nominal_field9928.clone())
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field9930, source_ml_source1_nominal_field9931, _, source_ml_source1_nominal_field9933, source_ml_source1_nominal_field9934, source_ml_source1_nominal_field9935, source_ml_source1_nominal_field9936) => {
                                                let source_ml_source1_nominal_field9931 = *source_ml_source1_nominal_field9931;
                                                let source_ml_source1_nominal_field9935 = *source_ml_source1_nominal_field9935;
                                                let source_ml_source1_nominal_field9936 = *source_ml_source1_nominal_field9936;
                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field9930.clone(), Box::new(source_ml_source1_nominal_field9931.clone()), false, source_ml_source1_nominal_field9933, source_ml_source1_nominal_field9934.clone(), Box::new(source_ml_source1_nominal_field9935.clone()), Box::new(source_ml_source1_nominal_field9936.clone()))
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field9938, source_ml_source1_nominal_field9939, _, source_ml_source1_nominal_field9941, source_ml_source1_nominal_field9942, source_ml_source1_nominal_field9943, source_ml_source1_nominal_field9944) => {
                                                let source_ml_source1_nominal_field9939 = *source_ml_source1_nominal_field9939;
                                                let source_ml_source1_nominal_field9943 = *source_ml_source1_nominal_field9943;
                                                let source_ml_source1_nominal_field9944 = *source_ml_source1_nominal_field9944;
                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field9938.clone(), Box::new(source_ml_source1_nominal_field9939.clone()), false, source_ml_source1_nominal_field9941, source_ml_source1_nominal_field9942.clone(), Box::new(source_ml_source1_nominal_field9943.clone()), Box::new(source_ml_source1_nominal_field9944.clone()))
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field9946, source_ml_source1_nominal_field9947, _, source_ml_source1_nominal_field9949, source_ml_source1_nominal_field9950, source_ml_source1_nominal_field9951, source_ml_source1_nominal_field9952) => {
                                                let source_ml_source1_nominal_field9947 = *source_ml_source1_nominal_field9947;
                                                let source_ml_source1_nominal_field9951 = *source_ml_source1_nominal_field9951;
                                                let source_ml_source1_nominal_field9952 = *source_ml_source1_nominal_field9952;
                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field9946.clone(), Box::new(source_ml_source1_nominal_field9947.clone()), false, source_ml_source1_nominal_field9949, source_ml_source1_nominal_field9950.clone(), Box::new(source_ml_source1_nominal_field9951.clone()), Box::new(source_ml_source1_nominal_field9952.clone()))
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field9954, source_ml_source1_nominal_field9955, _, source_ml_source1_nominal_field9957, source_ml_source1_nominal_field9958, source_ml_source1_nominal_field9959, source_ml_source1_nominal_field9960) => {
                                                let source_ml_source1_nominal_field9955 = *source_ml_source1_nominal_field9955;
                                                let source_ml_source1_nominal_field9959 = *source_ml_source1_nominal_field9959;
                                                let source_ml_source1_nominal_field9960 = *source_ml_source1_nominal_field9960;
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field9954.clone(), Box::new(source_ml_source1_nominal_field9955.clone()), false, source_ml_source1_nominal_field9957, source_ml_source1_nominal_field9958.clone(), Box::new(source_ml_source1_nominal_field9959.clone()), Box::new(source_ml_source1_nominal_field9960.clone()))
                                            }
                                        };
                                        std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source1_field9962 = self::replace_java_script_ordinary_object(heap.clone(), replacement.clone());
                                            (self::ml_string::Text::from_units(vec![]), source_ml_source1_field9962.clone(), true)
                                        }))
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
