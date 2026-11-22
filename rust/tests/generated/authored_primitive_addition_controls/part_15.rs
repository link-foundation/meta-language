pub fn plan_java_script_array_ordinary_definition(object: self::JavaScriptOrdinaryObject, key: self::JavaScriptSemanticValue, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::JavaScriptArrayPropertyPlan {
    {
        let at = self::find_java_script_array_property(match object.clone() {
            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value891, _, _, _, _, _) => {
                source_ml_source2_data_property_value891.clone()
            }
            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value892, _, _, _) => {
                source_ml_source2_data_property_value892.clone()
            }
            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value893) => {
                source_ml_source2_data_property_value893.clone()
            }
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value894, _, _, _, _, _, _) => {
                source_ml_source2_data_property_value894.clone()
            }
            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value895, _) => {
                source_ml_source2_data_property_value895.clone()
            }
            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value896, _) => {
                source_ml_source2_data_property_value896.clone()
            }
            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value897, _) => {
                source_ml_source2_data_property_value897.clone()
            }
            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value898, _) => {
                source_ml_source2_data_property_value898.clone()
            }
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value899) => {
                source_ml_source2_data_property_value899.clone()
            }
            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value900, _, _) => {
                source_ml_source2_data_property_value900.clone()
            }
            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value901, _, _) => {
                source_ml_source2_data_property_value901.clone()
            }
            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value902, _, _) => {
                source_ml_source2_data_property_value902.clone()
            }
            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value903, _, _) => {
                source_ml_source2_data_property_value903.clone()
            }
        }, key.clone());
        {
            let property = {
                let source_ml_source2_array_read_value917 = match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value904, _, _, _, _, _) => {
                        source_ml_source2_data_property_value904.clone()
                    }
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value905, _, _, _) => {
                        source_ml_source2_data_property_value905.clone()
                    }
                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value906) => {
                        source_ml_source2_data_property_value906.clone()
                    }
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value907, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value907.clone()
                    }
                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value908, _) => {
                        source_ml_source2_data_property_value908.clone()
                    }
                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value909, _) => {
                        source_ml_source2_data_property_value909.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value910, _) => {
                        source_ml_source2_data_property_value910.clone()
                    }
                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value911, _) => {
                        source_ml_source2_data_property_value911.clone()
                    }
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value912) => {
                        source_ml_source2_data_property_value912.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value913, _, _) => {
                        source_ml_source2_data_property_value913.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value914, _, _) => {
                        source_ml_source2_data_property_value914.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value915, _, _) => {
                        source_ml_source2_data_property_value915.clone()
                    }
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value916, _, _) => {
                        source_ml_source2_data_property_value916.clone()
                    }
                };
                { let ml_reference_array186 = source_ml_source2_array_read_value917.clone(); let ml_reference_position186 = at; let ml_reference_projection186 = self::ml_array::optional_at(&*ml_reference_array186.borrow(), self::ml_array::number_index(ml_reference_position186)); ml_reference_projection186 }
            };
            {
                let current = if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::create_java_script_property_descriptor(self::ml_string::Text::from_units(vec![100, 97, 116, 97]), false, false)
                } else {
                    match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value919) => {
                            source_ml_source2_data_property_value919.clone()
                        }
                    }
                };
                {
                    let plan = self::plan_java_script_property_definition(!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }), match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, source_ml_source2_data_property_value920, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value920
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, source_ml_source2_data_property_value921, _, _, _, _, _) => {
                            source_ml_source2_data_property_value921
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, source_ml_source2_data_property_value922, _, _) => {
                            source_ml_source2_data_property_value922
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, source_ml_source2_data_property_value923, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value923
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, source_ml_source2_data_property_value924, _, _, _) => {
                            source_ml_source2_data_property_value924
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, source_ml_source2_data_property_value925, _, _, _) => {
                            source_ml_source2_data_property_value925
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, source_ml_source2_data_property_value926, _, _, _) => {
                            source_ml_source2_data_property_value926
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, source_ml_source2_data_property_value927, _, _, _) => {
                            source_ml_source2_data_property_value927
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value928, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value928
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, source_ml_source2_data_property_value929, _, _, _, _) => {
                            source_ml_source2_data_property_value929
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, source_ml_source2_data_property_value930, _, _, _, _) => {
                            source_ml_source2_data_property_value930
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, source_ml_source2_data_property_value931, _, _, _, _) => {
                            source_ml_source2_data_property_value931
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, source_ml_source2_data_property_value932, _, _, _, _) => {
                            source_ml_source2_data_property_value932
                        }
                    }, current.clone(), proposed.clone());
                    if ((({ let ml_reference_value187 = plan.borrow().clone(); ml_reference_value187 }).1) == (self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]))) {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, ({ let ml_reference_value188 = plan.borrow().clone(); ml_reference_value188 }).2, self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    } else {
                        if ((({ let ml_reference_value189 = plan.borrow().clone(); ml_reference_value189 }).1) == (self::ml_string::Text::from_units(vec![107, 101, 101, 112]))) {
                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), true, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            if ((at < (0f64)) && (({ let ml_reference_array190 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value946, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value946.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value947, _, _, _) => {
                                    source_ml_source2_data_property_value947.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value948) => {
                                    source_ml_source2_data_property_value948.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value949, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value949.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value950, _) => {
                                    source_ml_source2_data_property_value950.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value951, _) => {
                                    source_ml_source2_data_property_value951.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value952, _) => {
                                    source_ml_source2_data_property_value952.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value953, _) => {
                                    source_ml_source2_data_property_value953.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value954) => {
                                    source_ml_source2_data_property_value954.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value955, _, _) => {
                                    source_ml_source2_data_property_value955.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value956, _, _) => {
                                    source_ml_source2_data_property_value956.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value957, _, _) => {
                                    source_ml_source2_data_property_value957.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value958, _, _) => {
                                    source_ml_source2_data_property_value958.clone()
                                }
                            }; let ml_reference_projection190 = ml_reference_array190.borrow().len() as f64; ml_reference_projection190 }) >= (4096f64))) {
                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]), self::ml_string::Text::from_units(vec![114, 101, 115, 111, 117, 114, 99, 101]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                            } else {
                                {
                                    let replacement = self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(key.clone()), ({ let ml_reference_value191 = plan.borrow().clone(); ml_reference_value191 }).0);
                                    {
                                        let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                        {
                                            let index = 0f64;
                                            {
                                                let source_ml_source1_temporary_values69 = match object.clone() {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value959, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value959.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value960, _, _, _) => {
                                                        source_ml_source2_data_property_value960.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value961) => {
                                                        source_ml_source2_data_property_value961.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value962, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value962.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value963, _) => {
                                                        source_ml_source2_data_property_value963.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value964, _) => {
                                                        source_ml_source2_data_property_value964.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value965, _) => {
                                                        source_ml_source2_data_property_value965.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value966, _) => {
                                                        source_ml_source2_data_property_value966.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value967) => {
                                                        source_ml_source2_data_property_value967.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value968, _, _) => {
                                                        source_ml_source2_data_property_value968.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value969, _, _) => {
                                                        source_ml_source2_data_property_value969.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value970, _, _) => {
                                                        source_ml_source2_data_property_value970.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value971, _, _) => {
                                                        source_ml_source2_data_property_value971.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_temporary_index69 = self::ml::Big::from_i128(0);
                                                    {
                                                        let properties_2 = self::source_ml_source1_temporary_plan_java_script_array_ordinary_definition_loop70(at, replacement.clone(), properties.clone(), index, source_ml_source1_temporary_values69.clone(), source_ml_source1_temporary_index69.clone());
                                                        {
                                                            let properties_4 = if (at < (0f64)) {
                                                                {
                                                                    let properties_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value192 = properties_2.borrow().clone(); ml_reference_value192 }, vec![replacement.clone()])));
                                                                    properties_3.clone()
                                                                }
                                                            } else {
                                                                properties_2.clone()
                                                            };
                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(match object.clone() {
                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field972, source_ml_source2_nominal_field973, source_ml_source2_nominal_field974, source_ml_source2_nominal_field975, _, source_ml_source2_nominal_field977, source_ml_source2_nominal_field978, source_ml_source2_nominal_field979, source_ml_source2_nominal_field980, source_ml_source2_nominal_field981) => {
                                                                    let source_ml_source2_nominal_field973 = *source_ml_source2_nominal_field973;
                                                                    let source_ml_source2_nominal_field980 = *source_ml_source2_nominal_field980;
                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field972.clone(), Box::new(source_ml_source2_nominal_field973.clone()), source_ml_source2_nominal_field974, source_ml_source2_nominal_field975, properties_4.clone(), source_ml_source2_nominal_field977.clone(), source_ml_source2_nominal_field978, source_ml_source2_nominal_field979.clone(), Box::new(source_ml_source2_nominal_field980.clone()), source_ml_source2_nominal_field981.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field983, source_ml_source2_nominal_field984, source_ml_source2_nominal_field985, source_ml_source2_nominal_field986, _, source_ml_source2_nominal_field988, source_ml_source2_nominal_field989, source_ml_source2_nominal_field990) => {
                                                                    let source_ml_source2_nominal_field984 = *source_ml_source2_nominal_field984;
                                                                    let source_ml_source2_nominal_field988 = *source_ml_source2_nominal_field988;
                                                                    let source_ml_source2_nominal_field989 = *source_ml_source2_nominal_field989;
                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field983.clone(), Box::new(source_ml_source2_nominal_field984.clone()), source_ml_source2_nominal_field985, source_ml_source2_nominal_field986, properties_4.clone(), Box::new(source_ml_source2_nominal_field988.clone()), Box::new(source_ml_source2_nominal_field989.clone()), source_ml_source2_nominal_field990.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field992, source_ml_source2_nominal_field993, source_ml_source2_nominal_field994, source_ml_source2_nominal_field995, _) => {
                                                                    let source_ml_source2_nominal_field993 = *source_ml_source2_nominal_field993;
                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field992.clone(), Box::new(source_ml_source2_nominal_field993.clone()), source_ml_source2_nominal_field994, source_ml_source2_nominal_field995, properties_4.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field998, source_ml_source2_nominal_field999, source_ml_source2_nominal_field1000, source_ml_source2_nominal_field1001, _, source_ml_source2_nominal_field1003, source_ml_source2_nominal_field1004, source_ml_source2_nominal_field1005, source_ml_source2_nominal_field1006, source_ml_source2_nominal_field1007, source_ml_source2_nominal_field1008) => {
                                                                    let source_ml_source2_nominal_field999 = *source_ml_source2_nominal_field999;
                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field998.clone(), Box::new(source_ml_source2_nominal_field999.clone()), source_ml_source2_nominal_field1000, source_ml_source2_nominal_field1001, properties_4.clone(), source_ml_source2_nominal_field1003.clone(), source_ml_source2_nominal_field1004, source_ml_source2_nominal_field1005.clone(), source_ml_source2_nominal_field1006.clone(), source_ml_source2_nominal_field1007.clone(), source_ml_source2_nominal_field1008.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field1010, source_ml_source2_nominal_field1011, source_ml_source2_nominal_field1012, source_ml_source2_nominal_field1013, _, source_ml_source2_nominal_field1015) => {
                                                                    let source_ml_source2_nominal_field1011 = *source_ml_source2_nominal_field1011;
                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field1010.clone(), Box::new(source_ml_source2_nominal_field1011.clone()), source_ml_source2_nominal_field1012, source_ml_source2_nominal_field1013, properties_4.clone(), source_ml_source2_nominal_field1015.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field1017, source_ml_source2_nominal_field1018, source_ml_source2_nominal_field1019, source_ml_source2_nominal_field1020, _, source_ml_source2_nominal_field1022) => {
                                                                    let source_ml_source2_nominal_field1018 = *source_ml_source2_nominal_field1018;
                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field1017.clone(), Box::new(source_ml_source2_nominal_field1018.clone()), source_ml_source2_nominal_field1019, source_ml_source2_nominal_field1020, properties_4.clone(), source_ml_source2_nominal_field1022.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field1024, source_ml_source2_nominal_field1025, source_ml_source2_nominal_field1026, source_ml_source2_nominal_field1027, _, source_ml_source2_nominal_field1029) => {
                                                                    let source_ml_source2_nominal_field1025 = *source_ml_source2_nominal_field1025;
                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field1024.clone(), Box::new(source_ml_source2_nominal_field1025.clone()), source_ml_source2_nominal_field1026, source_ml_source2_nominal_field1027, properties_4.clone(), source_ml_source2_nominal_field1029.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field1031, source_ml_source2_nominal_field1032, source_ml_source2_nominal_field1033, source_ml_source2_nominal_field1034, _, source_ml_source2_nominal_field1036) => {
                                                                    let source_ml_source2_nominal_field1032 = *source_ml_source2_nominal_field1032;
                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field1031.clone(), Box::new(source_ml_source2_nominal_field1032.clone()), source_ml_source2_nominal_field1033, source_ml_source2_nominal_field1034, properties_4.clone(), source_ml_source2_nominal_field1036.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field1038, source_ml_source2_nominal_field1039, source_ml_source2_nominal_field1040, source_ml_source2_nominal_field1041, source_ml_source2_nominal_field1042, source_ml_source2_nominal_field1043, source_ml_source2_nominal_field1044, source_ml_source2_nominal_field1045, _) => {
                                                                    let source_ml_source2_nominal_field1039 = *source_ml_source2_nominal_field1039;
                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field1038.clone(), Box::new(source_ml_source2_nominal_field1039.clone()), source_ml_source2_nominal_field1040, source_ml_source2_nominal_field1041.clone(), source_ml_source2_nominal_field1042.clone(), source_ml_source2_nominal_field1043.clone(), source_ml_source2_nominal_field1044.clone(), source_ml_source2_nominal_field1045, properties_4.clone())
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field1048, source_ml_source2_nominal_field1049, source_ml_source2_nominal_field1050, source_ml_source2_nominal_field1051, _, source_ml_source2_nominal_field1053, source_ml_source2_nominal_field1054) => {
                                                                    let source_ml_source2_nominal_field1049 = *source_ml_source2_nominal_field1049;
                                                                    let source_ml_source2_nominal_field1053 = *source_ml_source2_nominal_field1053;
                                                                    let source_ml_source2_nominal_field1054 = *source_ml_source2_nominal_field1054;
                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field1048.clone(), Box::new(source_ml_source2_nominal_field1049.clone()), source_ml_source2_nominal_field1050, source_ml_source2_nominal_field1051, properties_4.clone(), Box::new(source_ml_source2_nominal_field1053.clone()), Box::new(source_ml_source2_nominal_field1054.clone()))
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field1056, source_ml_source2_nominal_field1057, source_ml_source2_nominal_field1058, source_ml_source2_nominal_field1059, _, source_ml_source2_nominal_field1061, source_ml_source2_nominal_field1062) => {
                                                                    let source_ml_source2_nominal_field1057 = *source_ml_source2_nominal_field1057;
                                                                    let source_ml_source2_nominal_field1061 = *source_ml_source2_nominal_field1061;
                                                                    let source_ml_source2_nominal_field1062 = *source_ml_source2_nominal_field1062;
                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field1056.clone(), Box::new(source_ml_source2_nominal_field1057.clone()), source_ml_source2_nominal_field1058, source_ml_source2_nominal_field1059, properties_4.clone(), Box::new(source_ml_source2_nominal_field1061.clone()), Box::new(source_ml_source2_nominal_field1062.clone()))
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field1064, source_ml_source2_nominal_field1065, source_ml_source2_nominal_field1066, source_ml_source2_nominal_field1067, _, source_ml_source2_nominal_field1069, source_ml_source2_nominal_field1070) => {
                                                                    let source_ml_source2_nominal_field1065 = *source_ml_source2_nominal_field1065;
                                                                    let source_ml_source2_nominal_field1069 = *source_ml_source2_nominal_field1069;
                                                                    let source_ml_source2_nominal_field1070 = *source_ml_source2_nominal_field1070;
                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field1064.clone(), Box::new(source_ml_source2_nominal_field1065.clone()), source_ml_source2_nominal_field1066, source_ml_source2_nominal_field1067, properties_4.clone(), Box::new(source_ml_source2_nominal_field1069.clone()), Box::new(source_ml_source2_nominal_field1070.clone()))
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field1072, source_ml_source2_nominal_field1073, source_ml_source2_nominal_field1074, source_ml_source2_nominal_field1075, _, source_ml_source2_nominal_field1077, source_ml_source2_nominal_field1078) => {
                                                                    let source_ml_source2_nominal_field1073 = *source_ml_source2_nominal_field1073;
                                                                    let source_ml_source2_nominal_field1077 = *source_ml_source2_nominal_field1077;
                                                                    let source_ml_source2_nominal_field1078 = *source_ml_source2_nominal_field1078;
                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field1072.clone(), Box::new(source_ml_source2_nominal_field1073.clone()), source_ml_source2_nominal_field1074, source_ml_source2_nominal_field1075, properties_4.clone(), Box::new(source_ml_source2_nominal_field1077.clone()), Box::new(source_ml_source2_nominal_field1078.clone()))
                                                                }
                                                            }), true, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), ({ let ml_reference_value193 = plan.borrow().clone(); ml_reference_value193 }).3)
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

pub fn plan_java_script_array_length_definition(object: self::JavaScriptOrdinaryObject, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, unsigned_length: f64, number_length: f64) -> self::JavaScriptArrayPropertyPlan {
    if !(self::test_java_script_array_length(unsigned_length)) {
        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 115, 105, 103, 110, 101, 100, 51, 50, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
    } else {
        if (unsigned_length != number_length) {
            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
        } else {
            {
                let key = self::read_java_script_array_length_key();
                {
                    let at = self::find_java_script_array_property(match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value1080, _, _, _, _, _) => {
                            source_ml_source2_data_property_value1080.clone()
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value1081, _, _, _) => {
                            source_ml_source2_data_property_value1081.clone()
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value1082) => {
                            source_ml_source2_data_property_value1082.clone()
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value1083, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value1083.clone()
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value1084, _) => {
                            source_ml_source2_data_property_value1084.clone()
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value1085, _) => {
                            source_ml_source2_data_property_value1085.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value1086, _) => {
                            source_ml_source2_data_property_value1086.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value1087, _) => {
                            source_ml_source2_data_property_value1087.clone()
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value1088) => {
                            source_ml_source2_data_property_value1088.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value1089, _, _) => {
                            source_ml_source2_data_property_value1089.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value1090, _, _) => {
                            source_ml_source2_data_property_value1090.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value1091, _, _) => {
                            source_ml_source2_data_property_value1091.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value1092, _, _) => {
                            source_ml_source2_data_property_value1092.clone()
                        }
                    }, key.clone());
                    {
                        let old = {
                            let source_ml_source2_array_read_value1106 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value1093, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1093.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value1094, _, _, _) => {
                                    source_ml_source2_data_property_value1094.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value1095) => {
                                    source_ml_source2_data_property_value1095.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value1096, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1096.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value1097, _) => {
                                    source_ml_source2_data_property_value1097.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value1098, _) => {
                                    source_ml_source2_data_property_value1098.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value1099, _) => {
                                    source_ml_source2_data_property_value1099.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value1100, _) => {
                                    source_ml_source2_data_property_value1100.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value1101) => {
                                    source_ml_source2_data_property_value1101.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value1102, _, _) => {
                                    source_ml_source2_data_property_value1102.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value1103, _, _) => {
                                    source_ml_source2_data_property_value1103.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value1104, _, _) => {
                                    source_ml_source2_data_property_value1104.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value1105, _, _) => {
                                    source_ml_source2_data_property_value1105.clone()
                                }
                            };
                            { let ml_reference_array194 = source_ml_source2_array_read_value1106.clone(); let ml_reference_position194 = at; let ml_reference_projection194 = self::ml_array::optional_at(&*ml_reference_array194.borrow(), self::ml_array::number_index(ml_reference_position194)); ml_reference_projection194 }
                        };
                        if match (old.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 112, 114, 101, 115, 101, 110, 116]), self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            {
                                let length = self::read_java_script_array_length(object.clone());
                                {
                                    let descriptor = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source2_spread1108 = { let ml_reference_value195 = proposed.borrow().clone(); ml_reference_value195 };
                                        {
                                            let source_ml_source2_spread_field1109 = (source_ml_source2_spread1108.clone()).0;
                                            {
                                                let source_ml_source2_spread_field1110 = (source_ml_source2_spread1108.clone()).1;
                                                {
                                                    let source_ml_source2_spread_field1111 = (source_ml_source2_spread1108.clone()).2;
                                                    {
                                                        let source_ml_source2_spread_field1112 = (source_ml_source2_spread1108.clone()).3;
                                                        {
                                                            let source_ml_source2_spread_field1113 = (source_ml_source2_spread1108.clone()).4;
                                                            {
                                                                let source_ml_source2_spread_field1114 = (source_ml_source2_spread1108.clone()).5;
                                                                {
                                                                    let source_ml_source2_spread_field1115 = (source_ml_source2_spread1108.clone()).6;
                                                                    {
                                                                        let source_ml_source2_spread_field1116 = (source_ml_source2_spread1108.clone()).7;
                                                                        {
                                                                            let source_ml_source2_spread_field1117 = (source_ml_source2_spread1108.clone()).8;
                                                                            {
                                                                                let source_ml_source2_spread_field1118 = (source_ml_source2_spread1108.clone()).9;
                                                                                {
                                                                                    let source_ml_source2_spread_field1119 = (source_ml_source2_spread1108.clone()).10;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1120 = (source_ml_source2_spread1108.clone()).11;
                                                                                        {
                                                                                            let source_ml_source2_field1122 = self::JavaScriptSemanticValue::Number(unsigned_length);
                                                                                            (source_ml_source2_spread_field1109, source_ml_source2_spread_field1110, source_ml_source2_spread_field1111, source_ml_source2_spread_field1112, source_ml_source2_spread_field1113.clone(), source_ml_source2_spread_field1114, source_ml_source2_spread_field1115.clone(), source_ml_source2_spread_field1116, source_ml_source2_field1122.clone(), true, source_ml_source2_spread_field1119, source_ml_source2_spread_field1120)
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
                                    if (unsigned_length >= length) {
                                        self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), descriptor.clone())
                                    } else {
                                        if !(({ let ml_reference_value196 = (match old.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value1123) => {
                                                source_ml_source2_data_property_value1123.clone()
                                            }
                                        }).borrow().clone(); ml_reference_value196 }).10) {
                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                                        } else {
                                            {
                                                let new_writable = (!(({ let ml_reference_value197 = descriptor.borrow().clone(); ml_reference_value197 }).11) || ({ let ml_reference_value198 = descriptor.borrow().clone(); ml_reference_value198 }).10);
                                                {
                                                    let temporary = if new_writable {
                                                        descriptor.clone()
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source2_spread1124 = { let ml_reference_value199 = descriptor.borrow().clone(); ml_reference_value199 };
                                                            {
                                                                let source_ml_source2_spread_field1125 = (source_ml_source2_spread1124.clone()).0;
                                                                {
                                                                    let source_ml_source2_spread_field1126 = (source_ml_source2_spread1124.clone()).1;
                                                                    {
                                                                        let source_ml_source2_spread_field1127 = (source_ml_source2_spread1124.clone()).2;
                                                                        {
                                                                            let source_ml_source2_spread_field1128 = (source_ml_source2_spread1124.clone()).3;
                                                                            {
                                                                                let source_ml_source2_spread_field1129 = (source_ml_source2_spread1124.clone()).4;
                                                                                {
                                                                                    let source_ml_source2_spread_field1130 = (source_ml_source2_spread1124.clone()).5;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1131 = (source_ml_source2_spread1124.clone()).6;
                                                                                        {
                                                                                            let source_ml_source2_spread_field1132 = (source_ml_source2_spread1124.clone()).7;
                                                                                            {
                                                                                                let source_ml_source2_spread_field1133 = (source_ml_source2_spread1124.clone()).8;
                                                                                                {
                                                                                                    let source_ml_source2_spread_field1134 = (source_ml_source2_spread1124.clone()).9;
                                                                                                    {
                                                                                                        let source_ml_source2_spread_field1135 = (source_ml_source2_spread1124.clone()).10;
                                                                                                        {
                                                                                                            let source_ml_source2_spread_field1136 = (source_ml_source2_spread1124.clone()).11;
                                                                                                            (source_ml_source2_spread_field1125, source_ml_source2_spread_field1126, source_ml_source2_spread_field1127, source_ml_source2_spread_field1128, source_ml_source2_spread_field1129.clone(), source_ml_source2_spread_field1130, source_ml_source2_spread_field1131.clone(), source_ml_source2_spread_field1132, source_ml_source2_spread_field1133.clone(), source_ml_source2_spread_field1134, true, source_ml_source2_spread_field1136)
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
                                                        }))
                                                    };
                                                    {
                                                        let initial = self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), temporary.clone());
                                                        match initial.clone() {
                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(shared_constructor_field1, shared_constructor_field2, _, _, shared_constructor_field3) => {
                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                if !shared_constructor_field2 {
                                                                    initial.clone()
                                                                } else {
                                                                    {
                                                                        let blocked = (-1f64);
                                                                        {
                                                                            let source_ml_source1_temporary_values71 = match object.clone() {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value1139, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1139.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value1140, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1140.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value1141) => {
                                                                                    source_ml_source2_data_property_value1141.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value1142, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value1142.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value1143, _) => {
                                                                                    source_ml_source2_data_property_value1143.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value1144, _) => {
                                                                                    source_ml_source2_data_property_value1144.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value1145, _) => {
                                                                                    source_ml_source2_data_property_value1145.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value1146, _) => {
                                                                                    source_ml_source2_data_property_value1146.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value1147) => {
                                                                                    source_ml_source2_data_property_value1147.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value1148, _, _) => {
                                                                                    source_ml_source2_data_property_value1148.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value1149, _, _) => {
                                                                                    source_ml_source2_data_property_value1149.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value1150, _, _) => {
                                                                                    source_ml_source2_data_property_value1150.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value1151, _, _) => {
                                                                                    source_ml_source2_data_property_value1151.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_temporary_index71 = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let blocked_2 = self::source_ml_source1_temporary_plan_java_script_array_length_definition_loop73(unsigned_length, blocked, source_ml_source1_temporary_values71.clone(), source_ml_source1_temporary_index71.clone());
                                                                                    {
                                                                                        let final_length = if (blocked_2 < (0f64)) {
                                                                                            unsigned_length
                                                                                        } else {
                                                                                            (blocked_2 + 1f64)
                                                                                        };
                                                                                        {
                                                                                            let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                                            {
                                                                                                let source_ml_source1_temporary_values72 = match shared_constructor_field1.clone() {
                                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value1153, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value1153.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value1154, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value1154.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value1155) => {
                                                                                                        source_ml_source2_data_property_value1155.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value1156, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value1156.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value1157, _) => {
                                                                                                        source_ml_source2_data_property_value1157.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value1158, _) => {
                                                                                                        source_ml_source2_data_property_value1158.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value1159, _) => {
                                                                                                        source_ml_source2_data_property_value1159.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value1160, _) => {
                                                                                                        source_ml_source2_data_property_value1160.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value1161) => {
                                                                                                        source_ml_source2_data_property_value1161.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value1162, _, _) => {
                                                                                                        source_ml_source2_data_property_value1162.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value1163, _, _) => {
                                                                                                        source_ml_source2_data_property_value1163.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value1164, _, _) => {
                                                                                                        source_ml_source2_data_property_value1164.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value1165, _, _) => {
                                                                                                        source_ml_source2_data_property_value1165.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source1_temporary_index72 = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let properties_2 = self::source_ml_source1_temporary_plan_java_script_array_length_definition_loop74(new_writable, final_length, properties.clone(), source_ml_source1_temporary_values72.clone(), source_ml_source1_temporary_index72.clone());
                                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(match shared_constructor_field1.clone() {
                                                                                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field1167, source_ml_source2_nominal_field1168, source_ml_source2_nominal_field1169, source_ml_source2_nominal_field1170, _, source_ml_source2_nominal_field1172, source_ml_source2_nominal_field1173, source_ml_source2_nominal_field1174, source_ml_source2_nominal_field1175, source_ml_source2_nominal_field1176) => {
                                                                                                                let source_ml_source2_nominal_field1168 = *source_ml_source2_nominal_field1168;
                                                                                                                let source_ml_source2_nominal_field1175 = *source_ml_source2_nominal_field1175;
                                                                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field1167.clone(), Box::new(source_ml_source2_nominal_field1168.clone()), source_ml_source2_nominal_field1169, source_ml_source2_nominal_field1170, properties_2.clone(), source_ml_source2_nominal_field1172.clone(), source_ml_source2_nominal_field1173, source_ml_source2_nominal_field1174.clone(), Box::new(source_ml_source2_nominal_field1175.clone()), source_ml_source2_nominal_field1176.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field1178, source_ml_source2_nominal_field1179, source_ml_source2_nominal_field1180, source_ml_source2_nominal_field1181, _, source_ml_source2_nominal_field1183, source_ml_source2_nominal_field1184, source_ml_source2_nominal_field1185) => {
                                                                                                                let source_ml_source2_nominal_field1179 = *source_ml_source2_nominal_field1179;
                                                                                                                let source_ml_source2_nominal_field1183 = *source_ml_source2_nominal_field1183;
                                                                                                                let source_ml_source2_nominal_field1184 = *source_ml_source2_nominal_field1184;
                                                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field1178.clone(), Box::new(source_ml_source2_nominal_field1179.clone()), source_ml_source2_nominal_field1180, source_ml_source2_nominal_field1181, properties_2.clone(), Box::new(source_ml_source2_nominal_field1183.clone()), Box::new(source_ml_source2_nominal_field1184.clone()), source_ml_source2_nominal_field1185.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field1187, source_ml_source2_nominal_field1188, source_ml_source2_nominal_field1189, source_ml_source2_nominal_field1190, _) => {
                                                                                                                let source_ml_source2_nominal_field1188 = *source_ml_source2_nominal_field1188;
                                                                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field1187.clone(), Box::new(source_ml_source2_nominal_field1188.clone()), source_ml_source2_nominal_field1189, source_ml_source2_nominal_field1190, properties_2.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field1193, source_ml_source2_nominal_field1194, source_ml_source2_nominal_field1195, source_ml_source2_nominal_field1196, _, source_ml_source2_nominal_field1198, source_ml_source2_nominal_field1199, source_ml_source2_nominal_field1200, source_ml_source2_nominal_field1201, source_ml_source2_nominal_field1202, source_ml_source2_nominal_field1203) => {
                                                                                                                let source_ml_source2_nominal_field1194 = *source_ml_source2_nominal_field1194;
                                                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field1193.clone(), Box::new(source_ml_source2_nominal_field1194.clone()), source_ml_source2_nominal_field1195, source_ml_source2_nominal_field1196, properties_2.clone(), source_ml_source2_nominal_field1198.clone(), source_ml_source2_nominal_field1199, source_ml_source2_nominal_field1200.clone(), source_ml_source2_nominal_field1201.clone(), source_ml_source2_nominal_field1202.clone(), source_ml_source2_nominal_field1203.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field1205, source_ml_source2_nominal_field1206, source_ml_source2_nominal_field1207, source_ml_source2_nominal_field1208, _, source_ml_source2_nominal_field1210) => {
                                                                                                                let source_ml_source2_nominal_field1206 = *source_ml_source2_nominal_field1206;
                                                                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field1205.clone(), Box::new(source_ml_source2_nominal_field1206.clone()), source_ml_source2_nominal_field1207, source_ml_source2_nominal_field1208, properties_2.clone(), source_ml_source2_nominal_field1210.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field1212, source_ml_source2_nominal_field1213, source_ml_source2_nominal_field1214, source_ml_source2_nominal_field1215, _, source_ml_source2_nominal_field1217) => {
                                                                                                                let source_ml_source2_nominal_field1213 = *source_ml_source2_nominal_field1213;
                                                                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field1212.clone(), Box::new(source_ml_source2_nominal_field1213.clone()), source_ml_source2_nominal_field1214, source_ml_source2_nominal_field1215, properties_2.clone(), source_ml_source2_nominal_field1217.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field1219, source_ml_source2_nominal_field1220, source_ml_source2_nominal_field1221, source_ml_source2_nominal_field1222, _, source_ml_source2_nominal_field1224) => {
                                                                                                                let source_ml_source2_nominal_field1220 = *source_ml_source2_nominal_field1220;
                                                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field1219.clone(), Box::new(source_ml_source2_nominal_field1220.clone()), source_ml_source2_nominal_field1221, source_ml_source2_nominal_field1222, properties_2.clone(), source_ml_source2_nominal_field1224.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field1226, source_ml_source2_nominal_field1227, source_ml_source2_nominal_field1228, source_ml_source2_nominal_field1229, _, source_ml_source2_nominal_field1231) => {
                                                                                                                let source_ml_source2_nominal_field1227 = *source_ml_source2_nominal_field1227;
                                                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field1226.clone(), Box::new(source_ml_source2_nominal_field1227.clone()), source_ml_source2_nominal_field1228, source_ml_source2_nominal_field1229, properties_2.clone(), source_ml_source2_nominal_field1231.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field1233, source_ml_source2_nominal_field1234, source_ml_source2_nominal_field1235, source_ml_source2_nominal_field1236, source_ml_source2_nominal_field1237, source_ml_source2_nominal_field1238, source_ml_source2_nominal_field1239, source_ml_source2_nominal_field1240, _) => {
                                                                                                                let source_ml_source2_nominal_field1234 = *source_ml_source2_nominal_field1234;
                                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field1233.clone(), Box::new(source_ml_source2_nominal_field1234.clone()), source_ml_source2_nominal_field1235, source_ml_source2_nominal_field1236.clone(), source_ml_source2_nominal_field1237.clone(), source_ml_source2_nominal_field1238.clone(), source_ml_source2_nominal_field1239.clone(), source_ml_source2_nominal_field1240, properties_2.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field1243, source_ml_source2_nominal_field1244, source_ml_source2_nominal_field1245, source_ml_source2_nominal_field1246, _, source_ml_source2_nominal_field1248, source_ml_source2_nominal_field1249) => {
                                                                                                                let source_ml_source2_nominal_field1244 = *source_ml_source2_nominal_field1244;
                                                                                                                let source_ml_source2_nominal_field1248 = *source_ml_source2_nominal_field1248;
                                                                                                                let source_ml_source2_nominal_field1249 = *source_ml_source2_nominal_field1249;
                                                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field1243.clone(), Box::new(source_ml_source2_nominal_field1244.clone()), source_ml_source2_nominal_field1245, source_ml_source2_nominal_field1246, properties_2.clone(), Box::new(source_ml_source2_nominal_field1248.clone()), Box::new(source_ml_source2_nominal_field1249.clone()))
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field1251, source_ml_source2_nominal_field1252, source_ml_source2_nominal_field1253, source_ml_source2_nominal_field1254, _, source_ml_source2_nominal_field1256, source_ml_source2_nominal_field1257) => {
                                                                                                                let source_ml_source2_nominal_field1252 = *source_ml_source2_nominal_field1252;
                                                                                                                let source_ml_source2_nominal_field1256 = *source_ml_source2_nominal_field1256;
                                                                                                                let source_ml_source2_nominal_field1257 = *source_ml_source2_nominal_field1257;
                                                                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field1251.clone(), Box::new(source_ml_source2_nominal_field1252.clone()), source_ml_source2_nominal_field1253, source_ml_source2_nominal_field1254, properties_2.clone(), Box::new(source_ml_source2_nominal_field1256.clone()), Box::new(source_ml_source2_nominal_field1257.clone()))
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field1259, source_ml_source2_nominal_field1260, source_ml_source2_nominal_field1261, source_ml_source2_nominal_field1262, _, source_ml_source2_nominal_field1264, source_ml_source2_nominal_field1265) => {
                                                                                                                let source_ml_source2_nominal_field1260 = *source_ml_source2_nominal_field1260;
                                                                                                                let source_ml_source2_nominal_field1264 = *source_ml_source2_nominal_field1264;
                                                                                                                let source_ml_source2_nominal_field1265 = *source_ml_source2_nominal_field1265;
                                                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field1259.clone(), Box::new(source_ml_source2_nominal_field1260.clone()), source_ml_source2_nominal_field1261, source_ml_source2_nominal_field1262, properties_2.clone(), Box::new(source_ml_source2_nominal_field1264.clone()), Box::new(source_ml_source2_nominal_field1265.clone()))
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field1267, source_ml_source2_nominal_field1268, source_ml_source2_nominal_field1269, source_ml_source2_nominal_field1270, _, source_ml_source2_nominal_field1272, source_ml_source2_nominal_field1273) => {
                                                                                                                let source_ml_source2_nominal_field1268 = *source_ml_source2_nominal_field1268;
                                                                                                                let source_ml_source2_nominal_field1272 = *source_ml_source2_nominal_field1272;
                                                                                                                let source_ml_source2_nominal_field1273 = *source_ml_source2_nominal_field1273;
                                                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field1267.clone(), Box::new(source_ml_source2_nominal_field1268.clone()), source_ml_source2_nominal_field1269, source_ml_source2_nominal_field1270, properties_2.clone(), Box::new(source_ml_source2_nominal_field1272.clone()), Box::new(source_ml_source2_nominal_field1273.clone()))
                                                                                                            }
                                                                                                        }), (blocked_2 < (0f64)), self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                            let source_ml_source2_concat_value1278 = shared_constructor_field3.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_concat_value1279 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 105, 110, 100, 105, 99, 101, 115]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                                                {
                                                                                                                    let source_ml_source2_concat_value1280 = if (!new_writable && !(self::ml_array::includes(&{ let ml_reference_value200 = shared_constructor_field3.borrow().clone(); ml_reference_value200 }, &self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]), 0f64, false, |a, b| a == b))) {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])]))
                                                                                                                    } else {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))
                                                                                                                    };
                                                                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value201 = source_ml_source2_concat_value1278.borrow().clone(); ml_reference_value201 }), { let ml_reference_value202 = source_ml_source2_concat_value1279.borrow().clone(); ml_reference_value202 }), { let ml_reference_value203 = source_ml_source2_concat_value1280.borrow().clone(); ml_reference_value203 })
                                                                                                                }
                                                                                                            }
                                                                                                        })))
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
    }
}
