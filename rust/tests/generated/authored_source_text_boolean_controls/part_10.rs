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
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value943, _, _, _, _, _) => {
                            source_ml_source1_data_property_value943.clone()
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value944, _, _, _) => {
                            source_ml_source1_data_property_value944.clone()
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value945) => {
                            source_ml_source1_data_property_value945.clone()
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value946, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value946.clone()
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value947, _) => {
                            source_ml_source1_data_property_value947.clone()
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value948, _) => {
                            source_ml_source1_data_property_value948.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value949, _) => {
                            source_ml_source1_data_property_value949.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value950, _) => {
                            source_ml_source1_data_property_value950.clone()
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value951) => {
                            source_ml_source1_data_property_value951.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value952, _, _) => {
                            source_ml_source1_data_property_value952.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value953, _, _) => {
                            source_ml_source1_data_property_value953.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value954, _, _) => {
                            source_ml_source1_data_property_value954.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value955, _, _) => {
                            source_ml_source1_data_property_value955.clone()
                        }
                    }, key.clone());
                    {
                        let old = {
                            let source_ml_source1_array_read_value969 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value956, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value956.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value957, _, _, _) => {
                                    source_ml_source1_data_property_value957.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value958) => {
                                    source_ml_source1_data_property_value958.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value959, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value959.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value960, _) => {
                                    source_ml_source1_data_property_value960.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value961, _) => {
                                    source_ml_source1_data_property_value961.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value962, _) => {
                                    source_ml_source1_data_property_value962.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value963, _) => {
                                    source_ml_source1_data_property_value963.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value964) => {
                                    source_ml_source1_data_property_value964.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value965, _, _) => {
                                    source_ml_source1_data_property_value965.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value966, _, _) => {
                                    source_ml_source1_data_property_value966.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value967, _, _) => {
                                    source_ml_source1_data_property_value967.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value968, _, _) => {
                                    source_ml_source1_data_property_value968.clone()
                                }
                            };
                            { let ml_reference_array153 = source_ml_source1_array_read_value969.clone(); let ml_reference_position153 = at; let ml_reference_projection153 = self::ml_array::optional_at(&*ml_reference_array153.borrow(), self::ml_array::number_index(ml_reference_position153)); ml_reference_projection153 }
                        };
                        if match (old.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 112, 114, 101, 115, 101, 110, 116]), self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            {
                                let length = self::read_java_script_array_length(object.clone());
                                {
                                    let descriptor = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_spread971 = { let ml_reference_value154 = proposed.borrow().clone(); ml_reference_value154 };
                                        {
                                            let source_ml_source1_spread_field972 = (source_ml_source1_spread971.clone()).0;
                                            {
                                                let source_ml_source1_spread_field973 = (source_ml_source1_spread971.clone()).1;
                                                {
                                                    let source_ml_source1_spread_field974 = (source_ml_source1_spread971.clone()).2;
                                                    {
                                                        let source_ml_source1_spread_field975 = (source_ml_source1_spread971.clone()).3;
                                                        {
                                                            let source_ml_source1_spread_field976 = (source_ml_source1_spread971.clone()).4;
                                                            {
                                                                let source_ml_source1_spread_field977 = (source_ml_source1_spread971.clone()).5;
                                                                {
                                                                    let source_ml_source1_spread_field978 = (source_ml_source1_spread971.clone()).6;
                                                                    {
                                                                        let source_ml_source1_spread_field979 = (source_ml_source1_spread971.clone()).7;
                                                                        {
                                                                            let source_ml_source1_spread_field980 = (source_ml_source1_spread971.clone()).8;
                                                                            {
                                                                                let source_ml_source1_spread_field981 = (source_ml_source1_spread971.clone()).9;
                                                                                {
                                                                                    let source_ml_source1_spread_field982 = (source_ml_source1_spread971.clone()).10;
                                                                                    {
                                                                                        let source_ml_source1_spread_field983 = (source_ml_source1_spread971.clone()).11;
                                                                                        {
                                                                                            let source_ml_source1_field985 = self::JavaScriptSemanticValue::Number(unsigned_length);
                                                                                            (source_ml_source1_spread_field972, source_ml_source1_spread_field973, source_ml_source1_spread_field974, source_ml_source1_spread_field975, source_ml_source1_spread_field976.clone(), source_ml_source1_spread_field977, source_ml_source1_spread_field978.clone(), source_ml_source1_spread_field979, source_ml_source1_field985.clone(), true, source_ml_source1_spread_field982, source_ml_source1_spread_field983)
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
                                        if !(({ let ml_reference_value155 = (match old.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value986) => {
                                                source_ml_source1_data_property_value986.clone()
                                            }
                                        }).borrow().clone(); ml_reference_value155 }).10) {
                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                                        } else {
                                            {
                                                let new_writable = (!(({ let ml_reference_value156 = descriptor.borrow().clone(); ml_reference_value156 }).11) || ({ let ml_reference_value157 = descriptor.borrow().clone(); ml_reference_value157 }).10);
                                                {
                                                    let temporary = if new_writable {
                                                        descriptor.clone()
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_spread987 = { let ml_reference_value158 = descriptor.borrow().clone(); ml_reference_value158 };
                                                            {
                                                                let source_ml_source1_spread_field988 = (source_ml_source1_spread987.clone()).0;
                                                                {
                                                                    let source_ml_source1_spread_field989 = (source_ml_source1_spread987.clone()).1;
                                                                    {
                                                                        let source_ml_source1_spread_field990 = (source_ml_source1_spread987.clone()).2;
                                                                        {
                                                                            let source_ml_source1_spread_field991 = (source_ml_source1_spread987.clone()).3;
                                                                            {
                                                                                let source_ml_source1_spread_field992 = (source_ml_source1_spread987.clone()).4;
                                                                                {
                                                                                    let source_ml_source1_spread_field993 = (source_ml_source1_spread987.clone()).5;
                                                                                    {
                                                                                        let source_ml_source1_spread_field994 = (source_ml_source1_spread987.clone()).6;
                                                                                        {
                                                                                            let source_ml_source1_spread_field995 = (source_ml_source1_spread987.clone()).7;
                                                                                            {
                                                                                                let source_ml_source1_spread_field996 = (source_ml_source1_spread987.clone()).8;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field997 = (source_ml_source1_spread987.clone()).9;
                                                                                                    {
                                                                                                        let source_ml_source1_spread_field998 = (source_ml_source1_spread987.clone()).10;
                                                                                                        {
                                                                                                            let source_ml_source1_spread_field999 = (source_ml_source1_spread987.clone()).11;
                                                                                                            (source_ml_source1_spread_field988, source_ml_source1_spread_field989, source_ml_source1_spread_field990, source_ml_source1_spread_field991, source_ml_source1_spread_field992.clone(), source_ml_source1_spread_field993, source_ml_source1_spread_field994.clone(), source_ml_source1_spread_field995, source_ml_source1_spread_field996.clone(), source_ml_source1_spread_field997, true, source_ml_source1_spread_field999)
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
                                                                            let source_ml_temporary_values20 = match object.clone() {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1002, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value1002.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1003, _, _, _) => {
                                                                                    source_ml_source1_data_property_value1003.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1004) => {
                                                                                    source_ml_source1_data_property_value1004.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1005, _, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value1005.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1006, _) => {
                                                                                    source_ml_source1_data_property_value1006.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1007, _) => {
                                                                                    source_ml_source1_data_property_value1007.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1008, _) => {
                                                                                    source_ml_source1_data_property_value1008.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1009, _) => {
                                                                                    source_ml_source1_data_property_value1009.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1010) => {
                                                                                    source_ml_source1_data_property_value1010.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1011, _, _) => {
                                                                                    source_ml_source1_data_property_value1011.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1012, _, _) => {
                                                                                    source_ml_source1_data_property_value1012.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1013, _, _) => {
                                                                                    source_ml_source1_data_property_value1013.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1014, _, _) => {
                                                                                    source_ml_source1_data_property_value1014.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_temporary_index20 = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let blocked_2 = self::source_ml_temporary_plan_java_script_array_length_definition_loop22(unsigned_length, blocked, source_ml_temporary_values20.clone(), source_ml_temporary_index20.clone());
                                                                                    {
                                                                                        let final_length = if (blocked_2 < (0f64)) {
                                                                                            unsigned_length
                                                                                        } else {
                                                                                            (blocked_2 + 1f64)
                                                                                        };
                                                                                        {
                                                                                            let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                                            {
                                                                                                let source_ml_temporary_values21 = match shared_constructor_field1.clone() {
                                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1016, _, _, _, _, _) => {
                                                                                                        source_ml_source1_data_property_value1016.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1017, _, _, _) => {
                                                                                                        source_ml_source1_data_property_value1017.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1018) => {
                                                                                                        source_ml_source1_data_property_value1018.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1019, _, _, _, _, _, _) => {
                                                                                                        source_ml_source1_data_property_value1019.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1020, _) => {
                                                                                                        source_ml_source1_data_property_value1020.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1021, _) => {
                                                                                                        source_ml_source1_data_property_value1021.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1022, _) => {
                                                                                                        source_ml_source1_data_property_value1022.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1023, _) => {
                                                                                                        source_ml_source1_data_property_value1023.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1024) => {
                                                                                                        source_ml_source1_data_property_value1024.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1025, _, _) => {
                                                                                                        source_ml_source1_data_property_value1025.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1026, _, _) => {
                                                                                                        source_ml_source1_data_property_value1026.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1027, _, _) => {
                                                                                                        source_ml_source1_data_property_value1027.clone()
                                                                                                    }
                                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1028, _, _) => {
                                                                                                        source_ml_source1_data_property_value1028.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_temporary_index21 = self::ml::Big::from_i128(0);
                                                                                                    {
                                                                                                        let properties_2 = self::source_ml_temporary_plan_java_script_array_length_definition_loop23(new_writable, final_length, properties.clone(), source_ml_temporary_values21.clone(), source_ml_temporary_index21.clone());
                                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(match shared_constructor_field1.clone() {
                                                                                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field1030, source_ml_source1_nominal_field1031, source_ml_source1_nominal_field1032, source_ml_source1_nominal_field1033, _, source_ml_source1_nominal_field1035, source_ml_source1_nominal_field1036, source_ml_source1_nominal_field1037, source_ml_source1_nominal_field1038, source_ml_source1_nominal_field1039) => {
                                                                                                                let source_ml_source1_nominal_field1031 = *source_ml_source1_nominal_field1031;
                                                                                                                let source_ml_source1_nominal_field1038 = *source_ml_source1_nominal_field1038;
                                                                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field1030.clone(), Box::new(source_ml_source1_nominal_field1031.clone()), source_ml_source1_nominal_field1032, source_ml_source1_nominal_field1033, properties_2.clone(), source_ml_source1_nominal_field1035.clone(), source_ml_source1_nominal_field1036, source_ml_source1_nominal_field1037.clone(), Box::new(source_ml_source1_nominal_field1038.clone()), source_ml_source1_nominal_field1039.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field1041, source_ml_source1_nominal_field1042, source_ml_source1_nominal_field1043, source_ml_source1_nominal_field1044, _, source_ml_source1_nominal_field1046, source_ml_source1_nominal_field1047, source_ml_source1_nominal_field1048) => {
                                                                                                                let source_ml_source1_nominal_field1042 = *source_ml_source1_nominal_field1042;
                                                                                                                let source_ml_source1_nominal_field1046 = *source_ml_source1_nominal_field1046;
                                                                                                                let source_ml_source1_nominal_field1047 = *source_ml_source1_nominal_field1047;
                                                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field1041.clone(), Box::new(source_ml_source1_nominal_field1042.clone()), source_ml_source1_nominal_field1043, source_ml_source1_nominal_field1044, properties_2.clone(), Box::new(source_ml_source1_nominal_field1046.clone()), Box::new(source_ml_source1_nominal_field1047.clone()), source_ml_source1_nominal_field1048.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field1050, source_ml_source1_nominal_field1051, source_ml_source1_nominal_field1052, source_ml_source1_nominal_field1053, _) => {
                                                                                                                let source_ml_source1_nominal_field1051 = *source_ml_source1_nominal_field1051;
                                                                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field1050.clone(), Box::new(source_ml_source1_nominal_field1051.clone()), source_ml_source1_nominal_field1052, source_ml_source1_nominal_field1053, properties_2.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field1056, source_ml_source1_nominal_field1057, source_ml_source1_nominal_field1058, source_ml_source1_nominal_field1059, _, source_ml_source1_nominal_field1061, source_ml_source1_nominal_field1062, source_ml_source1_nominal_field1063, source_ml_source1_nominal_field1064, source_ml_source1_nominal_field1065, source_ml_source1_nominal_field1066) => {
                                                                                                                let source_ml_source1_nominal_field1057 = *source_ml_source1_nominal_field1057;
                                                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field1056.clone(), Box::new(source_ml_source1_nominal_field1057.clone()), source_ml_source1_nominal_field1058, source_ml_source1_nominal_field1059, properties_2.clone(), source_ml_source1_nominal_field1061.clone(), source_ml_source1_nominal_field1062, source_ml_source1_nominal_field1063.clone(), source_ml_source1_nominal_field1064.clone(), source_ml_source1_nominal_field1065.clone(), source_ml_source1_nominal_field1066.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field1068, source_ml_source1_nominal_field1069, source_ml_source1_nominal_field1070, source_ml_source1_nominal_field1071, _, source_ml_source1_nominal_field1073) => {
                                                                                                                let source_ml_source1_nominal_field1069 = *source_ml_source1_nominal_field1069;
                                                                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field1068.clone(), Box::new(source_ml_source1_nominal_field1069.clone()), source_ml_source1_nominal_field1070, source_ml_source1_nominal_field1071, properties_2.clone(), source_ml_source1_nominal_field1073.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1075, source_ml_source1_nominal_field1076, source_ml_source1_nominal_field1077, source_ml_source1_nominal_field1078, _, source_ml_source1_nominal_field1080) => {
                                                                                                                let source_ml_source1_nominal_field1076 = *source_ml_source1_nominal_field1076;
                                                                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1075.clone(), Box::new(source_ml_source1_nominal_field1076.clone()), source_ml_source1_nominal_field1077, source_ml_source1_nominal_field1078, properties_2.clone(), source_ml_source1_nominal_field1080.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1082, source_ml_source1_nominal_field1083, source_ml_source1_nominal_field1084, source_ml_source1_nominal_field1085, _, source_ml_source1_nominal_field1087) => {
                                                                                                                let source_ml_source1_nominal_field1083 = *source_ml_source1_nominal_field1083;
                                                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1082.clone(), Box::new(source_ml_source1_nominal_field1083.clone()), source_ml_source1_nominal_field1084, source_ml_source1_nominal_field1085, properties_2.clone(), source_ml_source1_nominal_field1087.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1089, source_ml_source1_nominal_field1090, source_ml_source1_nominal_field1091, source_ml_source1_nominal_field1092, _, source_ml_source1_nominal_field1094) => {
                                                                                                                let source_ml_source1_nominal_field1090 = *source_ml_source1_nominal_field1090;
                                                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1089.clone(), Box::new(source_ml_source1_nominal_field1090.clone()), source_ml_source1_nominal_field1091, source_ml_source1_nominal_field1092, properties_2.clone(), source_ml_source1_nominal_field1094.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1096, source_ml_source1_nominal_field1097, source_ml_source1_nominal_field1098, source_ml_source1_nominal_field1099, source_ml_source1_nominal_field1100, source_ml_source1_nominal_field1101, source_ml_source1_nominal_field1102, source_ml_source1_nominal_field1103, _) => {
                                                                                                                let source_ml_source1_nominal_field1097 = *source_ml_source1_nominal_field1097;
                                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1096.clone(), Box::new(source_ml_source1_nominal_field1097.clone()), source_ml_source1_nominal_field1098, source_ml_source1_nominal_field1099.clone(), source_ml_source1_nominal_field1100.clone(), source_ml_source1_nominal_field1101.clone(), source_ml_source1_nominal_field1102.clone(), source_ml_source1_nominal_field1103, properties_2.clone())
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1106, source_ml_source1_nominal_field1107, source_ml_source1_nominal_field1108, source_ml_source1_nominal_field1109, _, source_ml_source1_nominal_field1111, source_ml_source1_nominal_field1112) => {
                                                                                                                let source_ml_source1_nominal_field1107 = *source_ml_source1_nominal_field1107;
                                                                                                                let source_ml_source1_nominal_field1111 = *source_ml_source1_nominal_field1111;
                                                                                                                let source_ml_source1_nominal_field1112 = *source_ml_source1_nominal_field1112;
                                                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1106.clone(), Box::new(source_ml_source1_nominal_field1107.clone()), source_ml_source1_nominal_field1108, source_ml_source1_nominal_field1109, properties_2.clone(), Box::new(source_ml_source1_nominal_field1111.clone()), Box::new(source_ml_source1_nominal_field1112.clone()))
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1114, source_ml_source1_nominal_field1115, source_ml_source1_nominal_field1116, source_ml_source1_nominal_field1117, _, source_ml_source1_nominal_field1119, source_ml_source1_nominal_field1120) => {
                                                                                                                let source_ml_source1_nominal_field1115 = *source_ml_source1_nominal_field1115;
                                                                                                                let source_ml_source1_nominal_field1119 = *source_ml_source1_nominal_field1119;
                                                                                                                let source_ml_source1_nominal_field1120 = *source_ml_source1_nominal_field1120;
                                                                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1114.clone(), Box::new(source_ml_source1_nominal_field1115.clone()), source_ml_source1_nominal_field1116, source_ml_source1_nominal_field1117, properties_2.clone(), Box::new(source_ml_source1_nominal_field1119.clone()), Box::new(source_ml_source1_nominal_field1120.clone()))
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1122, source_ml_source1_nominal_field1123, source_ml_source1_nominal_field1124, source_ml_source1_nominal_field1125, _, source_ml_source1_nominal_field1127, source_ml_source1_nominal_field1128) => {
                                                                                                                let source_ml_source1_nominal_field1123 = *source_ml_source1_nominal_field1123;
                                                                                                                let source_ml_source1_nominal_field1127 = *source_ml_source1_nominal_field1127;
                                                                                                                let source_ml_source1_nominal_field1128 = *source_ml_source1_nominal_field1128;
                                                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1122.clone(), Box::new(source_ml_source1_nominal_field1123.clone()), source_ml_source1_nominal_field1124, source_ml_source1_nominal_field1125, properties_2.clone(), Box::new(source_ml_source1_nominal_field1127.clone()), Box::new(source_ml_source1_nominal_field1128.clone()))
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1130, source_ml_source1_nominal_field1131, source_ml_source1_nominal_field1132, source_ml_source1_nominal_field1133, _, source_ml_source1_nominal_field1135, source_ml_source1_nominal_field1136) => {
                                                                                                                let source_ml_source1_nominal_field1131 = *source_ml_source1_nominal_field1131;
                                                                                                                let source_ml_source1_nominal_field1135 = *source_ml_source1_nominal_field1135;
                                                                                                                let source_ml_source1_nominal_field1136 = *source_ml_source1_nominal_field1136;
                                                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1130.clone(), Box::new(source_ml_source1_nominal_field1131.clone()), source_ml_source1_nominal_field1132, source_ml_source1_nominal_field1133, properties_2.clone(), Box::new(source_ml_source1_nominal_field1135.clone()), Box::new(source_ml_source1_nominal_field1136.clone()))
                                                                                                            }
                                                                                                        }), (blocked_2 < (0f64)), self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                            let source_ml_source1_concat_value1141 = shared_constructor_field3.clone();
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value1142 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 105, 110, 100, 105, 99, 101, 115]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                                                {
                                                                                                                    let source_ml_source1_concat_value1143 = if (!new_writable && !(self::ml_array::includes(&{ let ml_reference_value159 = shared_constructor_field3.borrow().clone(); ml_reference_value159 }, &self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]), 0f64, false, |a, b| a == b))) {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])]))
                                                                                                                    } else {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))
                                                                                                                    };
                                                                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value160 = source_ml_source1_concat_value1141.borrow().clone(); ml_reference_value160 }), { let ml_reference_value161 = source_ml_source1_concat_value1142.borrow().clone(); ml_reference_value161 }), { let ml_reference_value162 = source_ml_source1_concat_value1143.borrow().clone(); ml_reference_value162 })
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

pub fn plan_java_script_array_property_definition(object: self::JavaScriptOrdinaryObject, key: self::JavaScriptSemanticValue, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::JavaScriptArrayPropertyPlan {
    if self::test_java_script_array_length_key(key.clone()) {
        if !(({ let ml_reference_value163 = proposed.borrow().clone(); ml_reference_value163 }).9) {
            self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), proposed.clone())
        } else {
            {
                let value = ({ let ml_reference_value164 = proposed.borrow().clone(); ml_reference_value164 }).8;
                match value.clone() {
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        self::plan_java_script_array_length_definition(object.clone(), proposed.clone(), self::convert_java_script_array_unsigned_length(value_value), value_value)
                    }
                    _ => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                }
            }
        }
    } else {
        {
            let index = self::read_java_script_array_index(key.clone());
            {
                let length = self::read_java_script_array_length(object.clone());
                {
                    let length_property = {
                        let source_ml_source1_array_read_value1178 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1152, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1152.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1153, _, _, _) => {
                                source_ml_source1_data_property_value1153.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1154) => {
                                source_ml_source1_data_property_value1154.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1155, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1155.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1156, _) => {
                                source_ml_source1_data_property_value1156.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1157, _) => {
                                source_ml_source1_data_property_value1157.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1158, _) => {
                                source_ml_source1_data_property_value1158.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1159, _) => {
                                source_ml_source1_data_property_value1159.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1160) => {
                                source_ml_source1_data_property_value1160.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1161, _, _) => {
                                source_ml_source1_data_property_value1161.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1162, _, _) => {
                                source_ml_source1_data_property_value1162.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1163, _, _) => {
                                source_ml_source1_data_property_value1163.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1164, _, _) => {
                                source_ml_source1_data_property_value1164.clone()
                            }
                        };
                        {
                            let source_ml_source1_array_read_index1179 = self::find_java_script_array_property(match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1165, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1165.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1166, _, _, _) => {
                                    source_ml_source1_data_property_value1166.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1167) => {
                                    source_ml_source1_data_property_value1167.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1168, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1168.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1169, _) => {
                                    source_ml_source1_data_property_value1169.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1170, _) => {
                                    source_ml_source1_data_property_value1170.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1171, _) => {
                                    source_ml_source1_data_property_value1171.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1172, _) => {
                                    source_ml_source1_data_property_value1172.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1173) => {
                                    source_ml_source1_data_property_value1173.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1174, _, _) => {
                                    source_ml_source1_data_property_value1174.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1175, _, _) => {
                                    source_ml_source1_data_property_value1175.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1176, _, _) => {
                                    source_ml_source1_data_property_value1176.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1177, _, _) => {
                                    source_ml_source1_data_property_value1177.clone()
                                }
                            }, self::read_java_script_array_length_key());
                            { let ml_reference_array165 = source_ml_source1_array_read_value1178.clone(); let ml_reference_position165 = source_ml_source1_array_read_index1179; let ml_reference_projection165 = self::ml_array::optional_at(&*ml_reference_array165.borrow(), self::ml_array::number_index(ml_reference_position165)); ml_reference_projection165 }
                        }
                    };
                    if ((index >= length) && (match (length_property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(({ let ml_reference_value166 = (match length_property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1180) => {
                            source_ml_source1_data_property_value1180.clone()
                        }
                    }).borrow().clone(); ml_reference_value166 }).10))) {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    } else {
                        {
                            let plan = self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), proposed.clone());
                            match plan.clone() {
                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5) => {
                                    let shared_constructor_field1 = *shared_constructor_field1;
                                    if ((!shared_constructor_field2 || (index < length)) || (index < (0f64))) {
                                        plan.clone()
                                    } else {
                                        {
                                            let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                            {
                                                let source_ml_temporary_values24 = match shared_constructor_field1.clone() {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1183, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value1183.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1184, _, _, _) => {
                                                        source_ml_source1_data_property_value1184.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1185) => {
                                                        source_ml_source1_data_property_value1185.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1186, _, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value1186.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1187, _) => {
                                                        source_ml_source1_data_property_value1187.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1188, _) => {
                                                        source_ml_source1_data_property_value1188.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1189, _) => {
                                                        source_ml_source1_data_property_value1189.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1190, _) => {
                                                        source_ml_source1_data_property_value1190.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1191) => {
                                                        source_ml_source1_data_property_value1191.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1192, _, _) => {
                                                        source_ml_source1_data_property_value1192.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1193, _, _) => {
                                                        source_ml_source1_data_property_value1193.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1194, _, _) => {
                                                        source_ml_source1_data_property_value1194.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1195, _, _) => {
                                                        source_ml_source1_data_property_value1195.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_index24 = self::ml::Big::from_i128(0);
                                                    {
                                                        let properties_2 = self::source_ml_temporary_plan_java_script_array_property_definition_loop25(index, properties.clone(), source_ml_temporary_values24.clone(), source_ml_temporary_index24.clone());
                                                        {
                                                            let source_ml_source1_nominal_field1309 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source1_nominal_field1311 = shared_constructor_field2;
                                                                {
                                                                    let source_ml_source1_nominal_field1313 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source1_nominal_field1315 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source1_nominal_field1317 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source1_nominal_override1319 = match shared_constructor_field1.clone() {
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field1197, source_ml_source1_nominal_field1198, source_ml_source1_nominal_field1199, source_ml_source1_nominal_field1200, _, source_ml_source1_nominal_field1202, source_ml_source1_nominal_field1203, source_ml_source1_nominal_field1204, source_ml_source1_nominal_field1205, source_ml_source1_nominal_field1206) => {
                                                                                        let source_ml_source1_nominal_field1198 = *source_ml_source1_nominal_field1198;
                                                                                        let source_ml_source1_nominal_field1205 = *source_ml_source1_nominal_field1205;
                                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field1197.clone(), Box::new(source_ml_source1_nominal_field1198.clone()), source_ml_source1_nominal_field1199, source_ml_source1_nominal_field1200, properties_2.clone(), source_ml_source1_nominal_field1202.clone(), source_ml_source1_nominal_field1203, source_ml_source1_nominal_field1204.clone(), Box::new(source_ml_source1_nominal_field1205.clone()), source_ml_source1_nominal_field1206.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field1208, source_ml_source1_nominal_field1209, source_ml_source1_nominal_field1210, source_ml_source1_nominal_field1211, _, source_ml_source1_nominal_field1213, source_ml_source1_nominal_field1214, source_ml_source1_nominal_field1215) => {
                                                                                        let source_ml_source1_nominal_field1209 = *source_ml_source1_nominal_field1209;
                                                                                        let source_ml_source1_nominal_field1213 = *source_ml_source1_nominal_field1213;
                                                                                        let source_ml_source1_nominal_field1214 = *source_ml_source1_nominal_field1214;
                                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field1208.clone(), Box::new(source_ml_source1_nominal_field1209.clone()), source_ml_source1_nominal_field1210, source_ml_source1_nominal_field1211, properties_2.clone(), Box::new(source_ml_source1_nominal_field1213.clone()), Box::new(source_ml_source1_nominal_field1214.clone()), source_ml_source1_nominal_field1215.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field1217, source_ml_source1_nominal_field1218, source_ml_source1_nominal_field1219, source_ml_source1_nominal_field1220, _) => {
                                                                                        let source_ml_source1_nominal_field1218 = *source_ml_source1_nominal_field1218;
                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field1217.clone(), Box::new(source_ml_source1_nominal_field1218.clone()), source_ml_source1_nominal_field1219, source_ml_source1_nominal_field1220, properties_2.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field1223, source_ml_source1_nominal_field1224, source_ml_source1_nominal_field1225, source_ml_source1_nominal_field1226, _, source_ml_source1_nominal_field1228, source_ml_source1_nominal_field1229, source_ml_source1_nominal_field1230, source_ml_source1_nominal_field1231, source_ml_source1_nominal_field1232, source_ml_source1_nominal_field1233) => {
                                                                                        let source_ml_source1_nominal_field1224 = *source_ml_source1_nominal_field1224;
                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field1223.clone(), Box::new(source_ml_source1_nominal_field1224.clone()), source_ml_source1_nominal_field1225, source_ml_source1_nominal_field1226, properties_2.clone(), source_ml_source1_nominal_field1228.clone(), source_ml_source1_nominal_field1229, source_ml_source1_nominal_field1230.clone(), source_ml_source1_nominal_field1231.clone(), source_ml_source1_nominal_field1232.clone(), source_ml_source1_nominal_field1233.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field1235, source_ml_source1_nominal_field1236, source_ml_source1_nominal_field1237, source_ml_source1_nominal_field1238, _, source_ml_source1_nominal_field1240) => {
                                                                                        let source_ml_source1_nominal_field1236 = *source_ml_source1_nominal_field1236;
                                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field1235.clone(), Box::new(source_ml_source1_nominal_field1236.clone()), source_ml_source1_nominal_field1237, source_ml_source1_nominal_field1238, properties_2.clone(), source_ml_source1_nominal_field1240.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1242, source_ml_source1_nominal_field1243, source_ml_source1_nominal_field1244, source_ml_source1_nominal_field1245, _, source_ml_source1_nominal_field1247) => {
                                                                                        let source_ml_source1_nominal_field1243 = *source_ml_source1_nominal_field1243;
                                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1242.clone(), Box::new(source_ml_source1_nominal_field1243.clone()), source_ml_source1_nominal_field1244, source_ml_source1_nominal_field1245, properties_2.clone(), source_ml_source1_nominal_field1247.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1249, source_ml_source1_nominal_field1250, source_ml_source1_nominal_field1251, source_ml_source1_nominal_field1252, _, source_ml_source1_nominal_field1254) => {
                                                                                        let source_ml_source1_nominal_field1250 = *source_ml_source1_nominal_field1250;
                                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1249.clone(), Box::new(source_ml_source1_nominal_field1250.clone()), source_ml_source1_nominal_field1251, source_ml_source1_nominal_field1252, properties_2.clone(), source_ml_source1_nominal_field1254.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1256, source_ml_source1_nominal_field1257, source_ml_source1_nominal_field1258, source_ml_source1_nominal_field1259, _, source_ml_source1_nominal_field1261) => {
                                                                                        let source_ml_source1_nominal_field1257 = *source_ml_source1_nominal_field1257;
                                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1256.clone(), Box::new(source_ml_source1_nominal_field1257.clone()), source_ml_source1_nominal_field1258, source_ml_source1_nominal_field1259, properties_2.clone(), source_ml_source1_nominal_field1261.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1263, source_ml_source1_nominal_field1264, source_ml_source1_nominal_field1265, source_ml_source1_nominal_field1266, source_ml_source1_nominal_field1267, source_ml_source1_nominal_field1268, source_ml_source1_nominal_field1269, source_ml_source1_nominal_field1270, _) => {
                                                                                        let source_ml_source1_nominal_field1264 = *source_ml_source1_nominal_field1264;
                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1263.clone(), Box::new(source_ml_source1_nominal_field1264.clone()), source_ml_source1_nominal_field1265, source_ml_source1_nominal_field1266.clone(), source_ml_source1_nominal_field1267.clone(), source_ml_source1_nominal_field1268.clone(), source_ml_source1_nominal_field1269.clone(), source_ml_source1_nominal_field1270, properties_2.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1273, source_ml_source1_nominal_field1274, source_ml_source1_nominal_field1275, source_ml_source1_nominal_field1276, _, source_ml_source1_nominal_field1278, source_ml_source1_nominal_field1279) => {
                                                                                        let source_ml_source1_nominal_field1274 = *source_ml_source1_nominal_field1274;
                                                                                        let source_ml_source1_nominal_field1278 = *source_ml_source1_nominal_field1278;
                                                                                        let source_ml_source1_nominal_field1279 = *source_ml_source1_nominal_field1279;
                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1273.clone(), Box::new(source_ml_source1_nominal_field1274.clone()), source_ml_source1_nominal_field1275, source_ml_source1_nominal_field1276, properties_2.clone(), Box::new(source_ml_source1_nominal_field1278.clone()), Box::new(source_ml_source1_nominal_field1279.clone()))
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1281, source_ml_source1_nominal_field1282, source_ml_source1_nominal_field1283, source_ml_source1_nominal_field1284, _, source_ml_source1_nominal_field1286, source_ml_source1_nominal_field1287) => {
                                                                                        let source_ml_source1_nominal_field1282 = *source_ml_source1_nominal_field1282;
                                                                                        let source_ml_source1_nominal_field1286 = *source_ml_source1_nominal_field1286;
                                                                                        let source_ml_source1_nominal_field1287 = *source_ml_source1_nominal_field1287;
                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1281.clone(), Box::new(source_ml_source1_nominal_field1282.clone()), source_ml_source1_nominal_field1283, source_ml_source1_nominal_field1284, properties_2.clone(), Box::new(source_ml_source1_nominal_field1286.clone()), Box::new(source_ml_source1_nominal_field1287.clone()))
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1289, source_ml_source1_nominal_field1290, source_ml_source1_nominal_field1291, source_ml_source1_nominal_field1292, _, source_ml_source1_nominal_field1294, source_ml_source1_nominal_field1295) => {
                                                                                        let source_ml_source1_nominal_field1290 = *source_ml_source1_nominal_field1290;
                                                                                        let source_ml_source1_nominal_field1294 = *source_ml_source1_nominal_field1294;
                                                                                        let source_ml_source1_nominal_field1295 = *source_ml_source1_nominal_field1295;
                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1289.clone(), Box::new(source_ml_source1_nominal_field1290.clone()), source_ml_source1_nominal_field1291, source_ml_source1_nominal_field1292, properties_2.clone(), Box::new(source_ml_source1_nominal_field1294.clone()), Box::new(source_ml_source1_nominal_field1295.clone()))
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1297, source_ml_source1_nominal_field1298, source_ml_source1_nominal_field1299, source_ml_source1_nominal_field1300, _, source_ml_source1_nominal_field1302, source_ml_source1_nominal_field1303) => {
                                                                                        let source_ml_source1_nominal_field1298 = *source_ml_source1_nominal_field1298;
                                                                                        let source_ml_source1_nominal_field1302 = *source_ml_source1_nominal_field1302;
                                                                                        let source_ml_source1_nominal_field1303 = *source_ml_source1_nominal_field1303;
                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1297.clone(), Box::new(source_ml_source1_nominal_field1298.clone()), source_ml_source1_nominal_field1299, source_ml_source1_nominal_field1300, properties_2.clone(), Box::new(source_ml_source1_nominal_field1302.clone()), Box::new(source_ml_source1_nominal_field1303.clone()))
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_override1320 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                        let source_ml_source1_concat_value1306 = shared_constructor_field5.clone();
                                                                                        {
                                                                                            let source_ml_source1_concat_value1307 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value167 = source_ml_source1_concat_value1306.borrow().clone(); ml_reference_value167 }), { let ml_reference_value168 = source_ml_source1_concat_value1307.borrow().clone(); ml_reference_value168 })
                                                                                        }
                                                                                    }));
                                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(source_ml_source1_nominal_override1319.clone()), source_ml_source1_nominal_field1311, source_ml_source1_nominal_field1313.clone(), source_ml_source1_nominal_field1315.clone(), source_ml_source1_nominal_override1320.clone())
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

pub fn convert_java_script_array_unsigned_length(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integer = value.trunc();
            {
                let remainder = (integer % 4294967296f64);
                if (remainder < (0f64)) {
                    (remainder + 4294967296f64)
                } else {
                    if (remainder == (0f64)) {
                        0f64
                    } else {
                        remainder
                    }
                }
            }
        }
    }
}

pub fn test_java_script_collection_key(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    {
        let source_ml_source1_s1337 = match left.clone() {
            self::JavaScriptSemanticValue::Number(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Number(value) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin26::MlTemporaryTestJavaScriptCollectionKeyJoin26Return(((left_value == value) || (left_value.is_nan() && value.is_nan())))
                    }
                    _ => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin26::MlTemporaryTestJavaScriptCollectionKeyJoin26Next
                    }
                }
            }
            _ => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin26::MlTemporaryTestJavaScriptCollectionKeyJoin26Next
            }
        };
        match source_ml_source1_s1337.clone() {
            self::MlTemporaryTestJavaScriptCollectionKeyJoin26::MlTemporaryTestJavaScriptCollectionKeyJoin26Next => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::MlTemporaryTestJavaScriptCollectionKeyJoin26::MlTemporaryTestJavaScriptCollectionKeyJoin26Return(source_ml_temporary_result26) => {
                source_ml_temporary_result26
            }
        }
    }
}

pub fn normalize_java_script_collection_key(key: self::JavaScriptSemanticValue) -> self::JavaScriptSemanticValue {
    {
        let source_ml_source1_s1362 = match key.clone() {
            self::JavaScriptSemanticValue::Number(value) => {
                if (value == (0f64)) {
                    self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin27::MlTemporaryNormalizeJavaScriptCollectionKeyJoin27Return(Box::new(self::JavaScriptSemanticValue::Number(0f64)))
                } else {
                    self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin27::MlTemporaryNormalizeJavaScriptCollectionKeyJoin27Next
                }
            }
            _ => {
                self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin27::MlTemporaryNormalizeJavaScriptCollectionKeyJoin27Next
            }
        };
        match source_ml_source1_s1362.clone() {
            self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin27::MlTemporaryNormalizeJavaScriptCollectionKeyJoin27Next => {
                key.clone()
            }
            self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin27::MlTemporaryNormalizeJavaScriptCollectionKeyJoin27Return(source_ml_temporary_result27) => {
                let source_ml_temporary_result27 = *source_ml_temporary_result27;
                source_ml_temporary_result27.clone()
            }
        }
    }
}

pub fn read_java_script_collection_operation_kind(operation: self::ml_string::Text) -> self::ml_string::Text {
    if self::ml_array::includes(&{ let ml_reference_value169 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112, 46, 103, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 115, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 104, 97, 115]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 100, 101, 108, 101, 116, 101]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 99, 108, 101, 97, 114]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 115, 105, 122, 101])]))).borrow().clone(); ml_reference_value169 }, &operation.clone(), 0f64, false, |a, b| a == b) {
        self::ml_string::Text::from_units(vec![77, 97, 112])
    } else {
        if self::ml_array::includes(&{ let ml_reference_value170 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![83, 101, 116, 46, 97, 100, 100]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 104, 97, 115]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 100, 101, 108, 101, 116, 101]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 99, 108, 101, 97, 114]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 115, 105, 122, 101])]))).borrow().clone(); ml_reference_value170 }, &operation.clone(), 0f64, false, |a, b| a == b) {
            self::ml_string::Text::from_units(vec![83, 101, 116])
        } else {
            self::read_java_script_weak_collection_method_owner(operation.clone())
        }
    }
}

pub fn read_java_script_collection_key_rank(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            0f64
        }
        self::JavaScriptSemanticValue::Null => {
            1f64
        }
        self::JavaScriptSemanticValue::Boolean(_) => {
            2f64
        }
        self::JavaScriptSemanticValue::Number(_) => {
            3f64
        }
        self::JavaScriptSemanticValue::Integer(_) => {
            4f64
        }
        self::JavaScriptSemanticValue::Symbol(_) => {
            6f64
        }
        self::JavaScriptSemanticValue::Object(_, _) => {
            7f64
        }
        _ => {
            5f64
        }
    }
}
