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
                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value869, _, _, _, _, _) => {
                            source_ml_source1_data_property_value869.clone()
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value870, _, _, _) => {
                            source_ml_source1_data_property_value870.clone()
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value871) => {
                            source_ml_source1_data_property_value871.clone()
                        }
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value872, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value872.clone()
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value873, _) => {
                            source_ml_source1_data_property_value873.clone()
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value874, _) => {
                            source_ml_source1_data_property_value874.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value875, _) => {
                            source_ml_source1_data_property_value875.clone()
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value876, _) => {
                            source_ml_source1_data_property_value876.clone()
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value877) => {
                            source_ml_source1_data_property_value877.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value878, _, _) => {
                            source_ml_source1_data_property_value878.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value879, _, _) => {
                            source_ml_source1_data_property_value879.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value880, _, _) => {
                            source_ml_source1_data_property_value880.clone()
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value881, _, _) => {
                            source_ml_source1_data_property_value881.clone()
                        }
                    }, key.clone());
                    {
                        let old = {
                            let source_ml_source1_array_read_value895 = match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value882, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value882.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value883, _, _, _) => {
                                    source_ml_source1_data_property_value883.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value884) => {
                                    source_ml_source1_data_property_value884.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value885, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value885.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value886, _) => {
                                    source_ml_source1_data_property_value886.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value887, _) => {
                                    source_ml_source1_data_property_value887.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value888, _) => {
                                    source_ml_source1_data_property_value888.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value889, _) => {
                                    source_ml_source1_data_property_value889.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value890) => {
                                    source_ml_source1_data_property_value890.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value891, _, _) => {
                                    source_ml_source1_data_property_value891.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value892, _, _) => {
                                    source_ml_source1_data_property_value892.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value893, _, _) => {
                                    source_ml_source1_data_property_value893.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value894, _, _) => {
                                    source_ml_source1_data_property_value894.clone()
                                }
                            };
                            {
                                let source_ml_source1_array_read_index896 = at;
                                { let ml_reference_array149 = source_ml_source1_array_read_value895.clone(); let ml_reference_position149 = source_ml_source1_array_read_index896; let ml_reference_projection149 = self::ml_array::optional_at(&*ml_reference_array149.borrow(), self::ml_array::number_index(ml_reference_position149)); ml_reference_projection149 }
                            }
                        };
                        if match (old.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 112, 114, 101, 115, 101, 110, 116]), self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            {
                                let length = self::read_java_script_array_length(object.clone());
                                {
                                    let descriptor = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_spread897 = { let ml_reference_value150 = proposed.borrow().clone(); ml_reference_value150 };
                                        {
                                            let source_ml_source1_spread_field898 = (source_ml_source1_spread897.clone()).0;
                                            {
                                                let source_ml_source1_spread_field899 = (source_ml_source1_spread897.clone()).1;
                                                {
                                                    let source_ml_source1_spread_field900 = (source_ml_source1_spread897.clone()).2;
                                                    {
                                                        let source_ml_source1_spread_field901 = (source_ml_source1_spread897.clone()).3;
                                                        {
                                                            let source_ml_source1_spread_field902 = (source_ml_source1_spread897.clone()).4;
                                                            {
                                                                let source_ml_source1_spread_field903 = (source_ml_source1_spread897.clone()).5;
                                                                {
                                                                    let source_ml_source1_spread_field904 = (source_ml_source1_spread897.clone()).6;
                                                                    {
                                                                        let source_ml_source1_spread_field905 = (source_ml_source1_spread897.clone()).7;
                                                                        {
                                                                            let source_ml_source1_spread_field906 = (source_ml_source1_spread897.clone()).8;
                                                                            {
                                                                                let source_ml_source1_spread_field907 = (source_ml_source1_spread897.clone()).9;
                                                                                {
                                                                                    let source_ml_source1_spread_field908 = (source_ml_source1_spread897.clone()).10;
                                                                                    {
                                                                                        let source_ml_source1_spread_field909 = (source_ml_source1_spread897.clone()).11;
                                                                                        {
                                                                                            let source_ml_source1_field910 = true;
                                                                                            {
                                                                                                let source_ml_source1_field911 = self::JavaScriptSemanticValue::Number(unsigned_length);
                                                                                                (source_ml_source1_spread_field898, source_ml_source1_spread_field899, source_ml_source1_spread_field900, source_ml_source1_spread_field901, source_ml_source1_spread_field902.clone(), source_ml_source1_spread_field903, source_ml_source1_spread_field904.clone(), source_ml_source1_spread_field905, source_ml_source1_field911.clone(), source_ml_source1_field910, source_ml_source1_spread_field908, source_ml_source1_spread_field909)
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
                                    }));
                                    if (unsigned_length >= length) {
                                        self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), descriptor.clone())
                                    } else {
                                        if !(({ let ml_reference_value151 = (match old.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value912) => {
                                                source_ml_source1_data_property_value912.clone()
                                            }
                                        }).borrow().clone(); ml_reference_value151 }).10) {
                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                                        } else {
                                            {
                                                let new_writable = (!(({ let ml_reference_value152 = descriptor.borrow().clone(); ml_reference_value152 }).11) || ({ let ml_reference_value153 = descriptor.borrow().clone(); ml_reference_value153 }).10);
                                                {
                                                    let temporary = if new_writable {
                                                        descriptor.clone()
                                                    } else {
                                                        std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_spread913 = { let ml_reference_value154 = descriptor.borrow().clone(); ml_reference_value154 };
                                                            {
                                                                let source_ml_source1_spread_field914 = (source_ml_source1_spread913.clone()).0;
                                                                {
                                                                    let source_ml_source1_spread_field915 = (source_ml_source1_spread913.clone()).1;
                                                                    {
                                                                        let source_ml_source1_spread_field916 = (source_ml_source1_spread913.clone()).2;
                                                                        {
                                                                            let source_ml_source1_spread_field917 = (source_ml_source1_spread913.clone()).3;
                                                                            {
                                                                                let source_ml_source1_spread_field918 = (source_ml_source1_spread913.clone()).4;
                                                                                {
                                                                                    let source_ml_source1_spread_field919 = (source_ml_source1_spread913.clone()).5;
                                                                                    {
                                                                                        let source_ml_source1_spread_field920 = (source_ml_source1_spread913.clone()).6;
                                                                                        {
                                                                                            let source_ml_source1_spread_field921 = (source_ml_source1_spread913.clone()).7;
                                                                                            {
                                                                                                let source_ml_source1_spread_field922 = (source_ml_source1_spread913.clone()).8;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field923 = (source_ml_source1_spread913.clone()).9;
                                                                                                    {
                                                                                                        let source_ml_source1_spread_field924 = (source_ml_source1_spread913.clone()).10;
                                                                                                        {
                                                                                                            let source_ml_source1_spread_field925 = (source_ml_source1_spread913.clone()).11;
                                                                                                            {
                                                                                                                let source_ml_source1_field926 = true;
                                                                                                                (source_ml_source1_spread_field914, source_ml_source1_spread_field915, source_ml_source1_spread_field916, source_ml_source1_spread_field917, source_ml_source1_spread_field918.clone(), source_ml_source1_spread_field919, source_ml_source1_spread_field920.clone(), source_ml_source1_spread_field921, source_ml_source1_spread_field922.clone(), source_ml_source1_spread_field923, source_ml_source1_field926, source_ml_source1_spread_field925)
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
                                                        }))
                                                    };
                                                    {
                                                        let initial = self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), temporary.clone());
                                                        if !(match initial.clone() {
                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, source_ml_source1_data_property_value927, _, _, _) => {
                                                                source_ml_source1_data_property_value927
                                                            }
                                                        }) {
                                                            initial.clone()
                                                        } else {
                                                            {
                                                                let blocked = (-1f64);
                                                                {
                                                                    let source_ml_temporary_values22 = match object.clone() {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value928, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value928.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value929, _, _, _) => {
                                                                            source_ml_source1_data_property_value929.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value930) => {
                                                                            source_ml_source1_data_property_value930.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value931, _, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value931.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value932, _) => {
                                                                            source_ml_source1_data_property_value932.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value933, _) => {
                                                                            source_ml_source1_data_property_value933.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value934, _) => {
                                                                            source_ml_source1_data_property_value934.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value935, _) => {
                                                                            source_ml_source1_data_property_value935.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value936) => {
                                                                            source_ml_source1_data_property_value936.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value937, _, _) => {
                                                                            source_ml_source1_data_property_value937.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value938, _, _) => {
                                                                            source_ml_source1_data_property_value938.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value939, _, _) => {
                                                                            source_ml_source1_data_property_value939.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value940, _, _) => {
                                                                            source_ml_source1_data_property_value940.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_temporary_index22 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let blocked_2 = self::source_ml_temporary_plan_java_script_array_length_definition_loop24(unsigned_length, blocked, source_ml_temporary_values22.clone(), source_ml_temporary_index22.clone());
                                                                            {
                                                                                let final_length = if (blocked_2 < (0f64)) {
                                                                                    unsigned_length
                                                                                } else {
                                                                                    (blocked_2 + 1f64)
                                                                                };
                                                                                {
                                                                                    let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                                    {
                                                                                        let source_ml_temporary_values23 = match match initial.clone() {
                                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value941, _, _, _, _) => {
                                                                                                let source_ml_source1_data_property_value941 = *source_ml_source1_data_property_value941;
                                                                                                source_ml_source1_data_property_value941.clone()
                                                                                            }
                                                                                        } {
                                                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value942, _, _, _, _, _) => {
                                                                                                source_ml_source1_data_property_value942.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value943, _, _, _) => {
                                                                                                source_ml_source1_data_property_value943.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value944) => {
                                                                                                source_ml_source1_data_property_value944.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value945, _, _, _, _, _, _) => {
                                                                                                source_ml_source1_data_property_value945.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value946, _) => {
                                                                                                source_ml_source1_data_property_value946.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value947, _) => {
                                                                                                source_ml_source1_data_property_value947.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value948, _) => {
                                                                                                source_ml_source1_data_property_value948.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value949, _) => {
                                                                                                source_ml_source1_data_property_value949.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value950) => {
                                                                                                source_ml_source1_data_property_value950.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value951, _, _) => {
                                                                                                source_ml_source1_data_property_value951.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value952, _, _) => {
                                                                                                source_ml_source1_data_property_value952.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value953, _, _) => {
                                                                                                source_ml_source1_data_property_value953.clone()
                                                                                            }
                                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value954, _, _) => {
                                                                                                source_ml_source1_data_property_value954.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_temporary_index23 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source1_s955 = self::source_ml_temporary_plan_java_script_array_length_definition_loop25(new_writable, final_length, properties.clone(), source_ml_temporary_values23.clone(), source_ml_temporary_index23.clone());
                                                                                                match source_ml_source1_s955.clone() {
                                                                                                    self::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Result::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Done => {
                                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(match match initial.clone() {
                                                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value956, _, _, _, _) => {
                                                                                                                let source_ml_source1_data_property_value956 = *source_ml_source1_data_property_value956;
                                                                                                                source_ml_source1_data_property_value956.clone()
                                                                                                            }
                                                                                                        } {
                                                                                                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field957, source_ml_source1_nominal_field958, source_ml_source1_nominal_field959, source_ml_source1_nominal_field960, source_ml_source1_nominal_field961, source_ml_source1_nominal_field962, source_ml_source1_nominal_field963, source_ml_source1_nominal_field964, source_ml_source1_nominal_field965, source_ml_source1_nominal_field966) => {
                                                                                                                let source_ml_source1_nominal_field958 = *source_ml_source1_nominal_field958;
                                                                                                                let source_ml_source1_nominal_field965 = *source_ml_source1_nominal_field965;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override967 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field957.clone(), Box::new(source_ml_source1_nominal_field958.clone()), source_ml_source1_nominal_field959, source_ml_source1_nominal_field960, source_ml_source1_nominal_override967.clone(), source_ml_source1_nominal_field962.clone(), source_ml_source1_nominal_field963, source_ml_source1_nominal_field964.clone(), Box::new(source_ml_source1_nominal_field965.clone()), source_ml_source1_nominal_field966.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field968, source_ml_source1_nominal_field969, source_ml_source1_nominal_field970, source_ml_source1_nominal_field971, source_ml_source1_nominal_field972, source_ml_source1_nominal_field973, source_ml_source1_nominal_field974, source_ml_source1_nominal_field975) => {
                                                                                                                let source_ml_source1_nominal_field969 = *source_ml_source1_nominal_field969;
                                                                                                                let source_ml_source1_nominal_field973 = *source_ml_source1_nominal_field973;
                                                                                                                let source_ml_source1_nominal_field974 = *source_ml_source1_nominal_field974;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override976 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field968.clone(), Box::new(source_ml_source1_nominal_field969.clone()), source_ml_source1_nominal_field970, source_ml_source1_nominal_field971, source_ml_source1_nominal_override976.clone(), Box::new(source_ml_source1_nominal_field973.clone()), Box::new(source_ml_source1_nominal_field974.clone()), source_ml_source1_nominal_field975.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field977, source_ml_source1_nominal_field978, source_ml_source1_nominal_field979, source_ml_source1_nominal_field980, source_ml_source1_nominal_field981) => {
                                                                                                                let source_ml_source1_nominal_field978 = *source_ml_source1_nominal_field978;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override982 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field977.clone(), Box::new(source_ml_source1_nominal_field978.clone()), source_ml_source1_nominal_field979, source_ml_source1_nominal_field980, source_ml_source1_nominal_override982.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field983, source_ml_source1_nominal_field984, source_ml_source1_nominal_field985, source_ml_source1_nominal_field986, source_ml_source1_nominal_field987, source_ml_source1_nominal_field988, source_ml_source1_nominal_field989, source_ml_source1_nominal_field990, source_ml_source1_nominal_field991, source_ml_source1_nominal_field992, source_ml_source1_nominal_field993) => {
                                                                                                                let source_ml_source1_nominal_field984 = *source_ml_source1_nominal_field984;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override994 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field983.clone(), Box::new(source_ml_source1_nominal_field984.clone()), source_ml_source1_nominal_field985, source_ml_source1_nominal_field986, source_ml_source1_nominal_override994.clone(), source_ml_source1_nominal_field988.clone(), source_ml_source1_nominal_field989, source_ml_source1_nominal_field990.clone(), source_ml_source1_nominal_field991.clone(), source_ml_source1_nominal_field992.clone(), source_ml_source1_nominal_field993.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field995, source_ml_source1_nominal_field996, source_ml_source1_nominal_field997, source_ml_source1_nominal_field998, source_ml_source1_nominal_field999, source_ml_source1_nominal_field1000) => {
                                                                                                                let source_ml_source1_nominal_field996 = *source_ml_source1_nominal_field996;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1001 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field995.clone(), Box::new(source_ml_source1_nominal_field996.clone()), source_ml_source1_nominal_field997, source_ml_source1_nominal_field998, source_ml_source1_nominal_override1001.clone(), source_ml_source1_nominal_field1000.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1002, source_ml_source1_nominal_field1003, source_ml_source1_nominal_field1004, source_ml_source1_nominal_field1005, source_ml_source1_nominal_field1006, source_ml_source1_nominal_field1007) => {
                                                                                                                let source_ml_source1_nominal_field1003 = *source_ml_source1_nominal_field1003;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1008 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1002.clone(), Box::new(source_ml_source1_nominal_field1003.clone()), source_ml_source1_nominal_field1004, source_ml_source1_nominal_field1005, source_ml_source1_nominal_override1008.clone(), source_ml_source1_nominal_field1007.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1009, source_ml_source1_nominal_field1010, source_ml_source1_nominal_field1011, source_ml_source1_nominal_field1012, source_ml_source1_nominal_field1013, source_ml_source1_nominal_field1014) => {
                                                                                                                let source_ml_source1_nominal_field1010 = *source_ml_source1_nominal_field1010;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1015 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1009.clone(), Box::new(source_ml_source1_nominal_field1010.clone()), source_ml_source1_nominal_field1011, source_ml_source1_nominal_field1012, source_ml_source1_nominal_override1015.clone(), source_ml_source1_nominal_field1014.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1016, source_ml_source1_nominal_field1017, source_ml_source1_nominal_field1018, source_ml_source1_nominal_field1019, source_ml_source1_nominal_field1020, source_ml_source1_nominal_field1021) => {
                                                                                                                let source_ml_source1_nominal_field1017 = *source_ml_source1_nominal_field1017;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1022 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1016.clone(), Box::new(source_ml_source1_nominal_field1017.clone()), source_ml_source1_nominal_field1018, source_ml_source1_nominal_field1019, source_ml_source1_nominal_override1022.clone(), source_ml_source1_nominal_field1021.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1023, source_ml_source1_nominal_field1024, source_ml_source1_nominal_field1025, source_ml_source1_nominal_field1026, source_ml_source1_nominal_field1027, source_ml_source1_nominal_field1028, source_ml_source1_nominal_field1029, source_ml_source1_nominal_field1030, source_ml_source1_nominal_field1031) => {
                                                                                                                let source_ml_source1_nominal_field1024 = *source_ml_source1_nominal_field1024;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1032 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1023.clone(), Box::new(source_ml_source1_nominal_field1024.clone()), source_ml_source1_nominal_field1025, source_ml_source1_nominal_field1026.clone(), source_ml_source1_nominal_field1027.clone(), source_ml_source1_nominal_field1028.clone(), source_ml_source1_nominal_field1029.clone(), source_ml_source1_nominal_field1030, source_ml_source1_nominal_override1032.clone())
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1033, source_ml_source1_nominal_field1034, source_ml_source1_nominal_field1035, source_ml_source1_nominal_field1036, source_ml_source1_nominal_field1037, source_ml_source1_nominal_field1038, source_ml_source1_nominal_field1039) => {
                                                                                                                let source_ml_source1_nominal_field1034 = *source_ml_source1_nominal_field1034;
                                                                                                                let source_ml_source1_nominal_field1038 = *source_ml_source1_nominal_field1038;
                                                                                                                let source_ml_source1_nominal_field1039 = *source_ml_source1_nominal_field1039;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1040 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1033.clone(), Box::new(source_ml_source1_nominal_field1034.clone()), source_ml_source1_nominal_field1035, source_ml_source1_nominal_field1036, source_ml_source1_nominal_override1040.clone(), Box::new(source_ml_source1_nominal_field1038.clone()), Box::new(source_ml_source1_nominal_field1039.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1041, source_ml_source1_nominal_field1042, source_ml_source1_nominal_field1043, source_ml_source1_nominal_field1044, source_ml_source1_nominal_field1045, source_ml_source1_nominal_field1046, source_ml_source1_nominal_field1047) => {
                                                                                                                let source_ml_source1_nominal_field1042 = *source_ml_source1_nominal_field1042;
                                                                                                                let source_ml_source1_nominal_field1046 = *source_ml_source1_nominal_field1046;
                                                                                                                let source_ml_source1_nominal_field1047 = *source_ml_source1_nominal_field1047;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1048 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1041.clone(), Box::new(source_ml_source1_nominal_field1042.clone()), source_ml_source1_nominal_field1043, source_ml_source1_nominal_field1044, source_ml_source1_nominal_override1048.clone(), Box::new(source_ml_source1_nominal_field1046.clone()), Box::new(source_ml_source1_nominal_field1047.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1049, source_ml_source1_nominal_field1050, source_ml_source1_nominal_field1051, source_ml_source1_nominal_field1052, source_ml_source1_nominal_field1053, source_ml_source1_nominal_field1054, source_ml_source1_nominal_field1055) => {
                                                                                                                let source_ml_source1_nominal_field1050 = *source_ml_source1_nominal_field1050;
                                                                                                                let source_ml_source1_nominal_field1054 = *source_ml_source1_nominal_field1054;
                                                                                                                let source_ml_source1_nominal_field1055 = *source_ml_source1_nominal_field1055;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1056 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1049.clone(), Box::new(source_ml_source1_nominal_field1050.clone()), source_ml_source1_nominal_field1051, source_ml_source1_nominal_field1052, source_ml_source1_nominal_override1056.clone(), Box::new(source_ml_source1_nominal_field1054.clone()), Box::new(source_ml_source1_nominal_field1055.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1057, source_ml_source1_nominal_field1058, source_ml_source1_nominal_field1059, source_ml_source1_nominal_field1060, source_ml_source1_nominal_field1061, source_ml_source1_nominal_field1062, source_ml_source1_nominal_field1063) => {
                                                                                                                let source_ml_source1_nominal_field1058 = *source_ml_source1_nominal_field1058;
                                                                                                                let source_ml_source1_nominal_field1062 = *source_ml_source1_nominal_field1062;
                                                                                                                let source_ml_source1_nominal_field1063 = *source_ml_source1_nominal_field1063;
                                                                                                                {
                                                                                                                    let source_ml_source1_nominal_override1064 = properties.clone();
                                                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1057.clone(), Box::new(source_ml_source1_nominal_field1058.clone()), source_ml_source1_nominal_field1059, source_ml_source1_nominal_field1060, source_ml_source1_nominal_override1064.clone(), Box::new(source_ml_source1_nominal_field1062.clone()), Box::new(source_ml_source1_nominal_field1063.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                        }), (blocked_2 < (0f64)), self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                            let source_ml_source1_concat_value1068 = match initial.clone() {
                                                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value1065) => {
                                                                                                                    source_ml_source1_data_property_value1065.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value1069 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 105, 110, 100, 105, 99, 101, 115]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                                                {
                                                                                                                    let source_ml_source1_concat_value1070 = if (!new_writable && !(self::ml_array::includes(&{ let ml_reference_value155 = (match initial.clone() {
                                                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value1067) => {
                                                                                                                            source_ml_source1_data_property_value1067.clone()
                                                                                                                        }
                                                                                                                    }).borrow().clone(); ml_reference_value155 }, &self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]), 0f64, false, |a, b| a == b))) {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])]))
                                                                                                                    } else {
                                                                                                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))
                                                                                                                    };
                                                                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value156 = source_ml_source1_concat_value1068.borrow().clone(); ml_reference_value156 }), { let ml_reference_value157 = source_ml_source1_concat_value1069.borrow().clone(); ml_reference_value157 }), { let ml_reference_value158 = source_ml_source1_concat_value1070.borrow().clone(); ml_reference_value158 })
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
        if !(({ let ml_reference_value159 = proposed.borrow().clone(); ml_reference_value159 }).9) {
            self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), proposed.clone())
        } else {
            {
                let value = ({ let ml_reference_value160 = proposed.borrow().clone(); ml_reference_value160 }).8;
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1071) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        self::plan_java_script_array_length_definition(object.clone(), proposed.clone(), self::convert_java_script_array_unsigned_length(value_value), value_value)
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1072) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f1073) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f1074, source_ml_source1_f1075) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1076) => {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 119, 111, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1077, source_ml_source1_f1078) => {
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
                        let source_ml_source1_array_read_value1105 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1079, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1079.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1080, _, _, _) => {
                                source_ml_source1_data_property_value1080.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1081) => {
                                source_ml_source1_data_property_value1081.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1082, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1082.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1083, _) => {
                                source_ml_source1_data_property_value1083.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1084, _) => {
                                source_ml_source1_data_property_value1084.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1085, _) => {
                                source_ml_source1_data_property_value1085.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1086, _) => {
                                source_ml_source1_data_property_value1086.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1087) => {
                                source_ml_source1_data_property_value1087.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1088, _, _) => {
                                source_ml_source1_data_property_value1088.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1089, _, _) => {
                                source_ml_source1_data_property_value1089.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1090, _, _) => {
                                source_ml_source1_data_property_value1090.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1091, _, _) => {
                                source_ml_source1_data_property_value1091.clone()
                            }
                        };
                        {
                            let source_ml_source1_array_read_index1106 = self::find_java_script_array_property(match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1092, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1092.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1093, _, _, _) => {
                                    source_ml_source1_data_property_value1093.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1094) => {
                                    source_ml_source1_data_property_value1094.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1095, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1095.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1096, _) => {
                                    source_ml_source1_data_property_value1096.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1097, _) => {
                                    source_ml_source1_data_property_value1097.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1098, _) => {
                                    source_ml_source1_data_property_value1098.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1099, _) => {
                                    source_ml_source1_data_property_value1099.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1100) => {
                                    source_ml_source1_data_property_value1100.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1101, _, _) => {
                                    source_ml_source1_data_property_value1101.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1102, _, _) => {
                                    source_ml_source1_data_property_value1102.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1103, _, _) => {
                                    source_ml_source1_data_property_value1103.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1104, _, _) => {
                                    source_ml_source1_data_property_value1104.clone()
                                }
                            }, self::read_java_script_array_length_key());
                            { let ml_reference_array161 = source_ml_source1_array_read_value1105.clone(); let ml_reference_position161 = source_ml_source1_array_read_index1106; let ml_reference_projection161 = self::ml_array::optional_at(&*ml_reference_array161.borrow(), self::ml_array::number_index(ml_reference_position161)); ml_reference_projection161 }
                        }
                    };
                    if ((index >= length) && (match (length_property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(({ let ml_reference_value162 = (match length_property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1107) => {
                            source_ml_source1_data_property_value1107.clone()
                        }
                    }).borrow().clone(); ml_reference_value162 }).10))) {
                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(object.clone()), false, self::ml_string::Text::from_units(vec![]), self::ml_string::Text::from_units(vec![]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                    } else {
                        {
                            let plan = self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), proposed.clone());
                            if ((!(match plan.clone() {
                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, source_ml_source1_data_property_value1108, _, _, _) => {
                                    source_ml_source1_data_property_value1108
                                }
                            }) || (index < length)) || (index < (0f64))) {
                                plan.clone()
                            } else {
                                {
                                    let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                    {
                                        let source_ml_temporary_values27 = match match plan.clone() {
                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value1109, _, _, _, _) => {
                                                let source_ml_source1_data_property_value1109 = *source_ml_source1_data_property_value1109;
                                                source_ml_source1_data_property_value1109.clone()
                                            }
                                        } {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source1_data_property_value1110, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value1110.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source1_data_property_value1111, _, _, _) => {
                                                source_ml_source1_data_property_value1111.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source1_data_property_value1112) => {
                                                source_ml_source1_data_property_value1112.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source1_data_property_value1113, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value1113.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source1_data_property_value1114, _) => {
                                                source_ml_source1_data_property_value1114.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source1_data_property_value1115, _) => {
                                                source_ml_source1_data_property_value1115.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source1_data_property_value1116, _) => {
                                                source_ml_source1_data_property_value1116.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source1_data_property_value1117, _) => {
                                                source_ml_source1_data_property_value1117.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value1118) => {
                                                source_ml_source1_data_property_value1118.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source1_data_property_value1119, _, _) => {
                                                source_ml_source1_data_property_value1119.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source1_data_property_value1120, _, _) => {
                                                source_ml_source1_data_property_value1120.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source1_data_property_value1121, _, _) => {
                                                source_ml_source1_data_property_value1121.clone()
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source1_data_property_value1122, _, _) => {
                                                source_ml_source1_data_property_value1122.clone()
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index27 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_s1123 = self::source_ml_temporary_plan_java_script_array_property_definition_loop28(index, properties.clone(), source_ml_temporary_values27.clone(), source_ml_temporary_index27.clone());
                                                match source_ml_source1_s1123.clone() {
                                                    self::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Result::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Done => {
                                                        {
                                                            let source_ml_source1_nominal_spread1236 = plan.clone();
                                                            {
                                                                let source_ml_source1_nominal_field1237 = match source_ml_source1_nominal_spread1236.clone() {
                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value1238, _, _, _, _) => {
                                                                        let source_ml_source1_data_property_value1238 = *source_ml_source1_data_property_value1238;
                                                                        source_ml_source1_data_property_value1238.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1239 = match source_ml_source1_nominal_spread1236.clone() {
                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, source_ml_source1_data_property_value1240, _, _, _) => {
                                                                            source_ml_source1_data_property_value1240
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1241 = match source_ml_source1_nominal_spread1236.clone() {
                                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, source_ml_source1_data_property_value1242, _, _) => {
                                                                                source_ml_source1_data_property_value1242.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_field1243 = match source_ml_source1_nominal_spread1236.clone() {
                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, source_ml_source1_data_property_value1244, _) => {
                                                                                    source_ml_source1_data_property_value1244.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field1245 = match source_ml_source1_nominal_spread1236.clone() {
                                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value1246) => {
                                                                                        source_ml_source1_data_property_value1246.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_nominal_override1247 = match match plan.clone() {
                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(source_ml_source1_data_property_value1124, _, _, _, _) => {
                                                                                            let source_ml_source1_data_property_value1124 = *source_ml_source1_data_property_value1124;
                                                                                            source_ml_source1_data_property_value1124.clone()
                                                                                        }
                                                                                    } {
                                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field1125, source_ml_source1_nominal_field1126, source_ml_source1_nominal_field1127, source_ml_source1_nominal_field1128, source_ml_source1_nominal_field1129, source_ml_source1_nominal_field1130, source_ml_source1_nominal_field1131, source_ml_source1_nominal_field1132, source_ml_source1_nominal_field1133, source_ml_source1_nominal_field1134) => {
                                                                                            let source_ml_source1_nominal_field1126 = *source_ml_source1_nominal_field1126;
                                                                                            let source_ml_source1_nominal_field1133 = *source_ml_source1_nominal_field1133;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1135 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_nominal_field1125.clone(), Box::new(source_ml_source1_nominal_field1126.clone()), source_ml_source1_nominal_field1127, source_ml_source1_nominal_field1128, source_ml_source1_nominal_override1135.clone(), source_ml_source1_nominal_field1130.clone(), source_ml_source1_nominal_field1131, source_ml_source1_nominal_field1132.clone(), Box::new(source_ml_source1_nominal_field1133.clone()), source_ml_source1_nominal_field1134.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field1136, source_ml_source1_nominal_field1137, source_ml_source1_nominal_field1138, source_ml_source1_nominal_field1139, source_ml_source1_nominal_field1140, source_ml_source1_nominal_field1141, source_ml_source1_nominal_field1142, source_ml_source1_nominal_field1143) => {
                                                                                            let source_ml_source1_nominal_field1137 = *source_ml_source1_nominal_field1137;
                                                                                            let source_ml_source1_nominal_field1141 = *source_ml_source1_nominal_field1141;
                                                                                            let source_ml_source1_nominal_field1142 = *source_ml_source1_nominal_field1142;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1144 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_nominal_field1136.clone(), Box::new(source_ml_source1_nominal_field1137.clone()), source_ml_source1_nominal_field1138, source_ml_source1_nominal_field1139, source_ml_source1_nominal_override1144.clone(), Box::new(source_ml_source1_nominal_field1141.clone()), Box::new(source_ml_source1_nominal_field1142.clone()), source_ml_source1_nominal_field1143.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field1145, source_ml_source1_nominal_field1146, source_ml_source1_nominal_field1147, source_ml_source1_nominal_field1148, source_ml_source1_nominal_field1149) => {
                                                                                            let source_ml_source1_nominal_field1146 = *source_ml_source1_nominal_field1146;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1150 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_nominal_field1145.clone(), Box::new(source_ml_source1_nominal_field1146.clone()), source_ml_source1_nominal_field1147, source_ml_source1_nominal_field1148, source_ml_source1_nominal_override1150.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field1151, source_ml_source1_nominal_field1152, source_ml_source1_nominal_field1153, source_ml_source1_nominal_field1154, source_ml_source1_nominal_field1155, source_ml_source1_nominal_field1156, source_ml_source1_nominal_field1157, source_ml_source1_nominal_field1158, source_ml_source1_nominal_field1159, source_ml_source1_nominal_field1160, source_ml_source1_nominal_field1161) => {
                                                                                            let source_ml_source1_nominal_field1152 = *source_ml_source1_nominal_field1152;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1162 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_nominal_field1151.clone(), Box::new(source_ml_source1_nominal_field1152.clone()), source_ml_source1_nominal_field1153, source_ml_source1_nominal_field1154, source_ml_source1_nominal_override1162.clone(), source_ml_source1_nominal_field1156.clone(), source_ml_source1_nominal_field1157, source_ml_source1_nominal_field1158.clone(), source_ml_source1_nominal_field1159.clone(), source_ml_source1_nominal_field1160.clone(), source_ml_source1_nominal_field1161.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field1163, source_ml_source1_nominal_field1164, source_ml_source1_nominal_field1165, source_ml_source1_nominal_field1166, source_ml_source1_nominal_field1167, source_ml_source1_nominal_field1168) => {
                                                                                            let source_ml_source1_nominal_field1164 = *source_ml_source1_nominal_field1164;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1169 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_nominal_field1163.clone(), Box::new(source_ml_source1_nominal_field1164.clone()), source_ml_source1_nominal_field1165, source_ml_source1_nominal_field1166, source_ml_source1_nominal_override1169.clone(), source_ml_source1_nominal_field1168.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1170, source_ml_source1_nominal_field1171, source_ml_source1_nominal_field1172, source_ml_source1_nominal_field1173, source_ml_source1_nominal_field1174, source_ml_source1_nominal_field1175) => {
                                                                                            let source_ml_source1_nominal_field1171 = *source_ml_source1_nominal_field1171;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1176 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_nominal_field1170.clone(), Box::new(source_ml_source1_nominal_field1171.clone()), source_ml_source1_nominal_field1172, source_ml_source1_nominal_field1173, source_ml_source1_nominal_override1176.clone(), source_ml_source1_nominal_field1175.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1177, source_ml_source1_nominal_field1178, source_ml_source1_nominal_field1179, source_ml_source1_nominal_field1180, source_ml_source1_nominal_field1181, source_ml_source1_nominal_field1182) => {
                                                                                            let source_ml_source1_nominal_field1178 = *source_ml_source1_nominal_field1178;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1183 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_nominal_field1177.clone(), Box::new(source_ml_source1_nominal_field1178.clone()), source_ml_source1_nominal_field1179, source_ml_source1_nominal_field1180, source_ml_source1_nominal_override1183.clone(), source_ml_source1_nominal_field1182.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1184, source_ml_source1_nominal_field1185, source_ml_source1_nominal_field1186, source_ml_source1_nominal_field1187, source_ml_source1_nominal_field1188, source_ml_source1_nominal_field1189) => {
                                                                                            let source_ml_source1_nominal_field1185 = *source_ml_source1_nominal_field1185;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1190 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_nominal_field1184.clone(), Box::new(source_ml_source1_nominal_field1185.clone()), source_ml_source1_nominal_field1186, source_ml_source1_nominal_field1187, source_ml_source1_nominal_override1190.clone(), source_ml_source1_nominal_field1189.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1191, source_ml_source1_nominal_field1192, source_ml_source1_nominal_field1193, source_ml_source1_nominal_field1194, source_ml_source1_nominal_field1195, source_ml_source1_nominal_field1196, source_ml_source1_nominal_field1197, source_ml_source1_nominal_field1198, source_ml_source1_nominal_field1199) => {
                                                                                            let source_ml_source1_nominal_field1192 = *source_ml_source1_nominal_field1192;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1200 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_nominal_field1191.clone(), Box::new(source_ml_source1_nominal_field1192.clone()), source_ml_source1_nominal_field1193, source_ml_source1_nominal_field1194.clone(), source_ml_source1_nominal_field1195.clone(), source_ml_source1_nominal_field1196.clone(), source_ml_source1_nominal_field1197.clone(), source_ml_source1_nominal_field1198, source_ml_source1_nominal_override1200.clone())
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1201, source_ml_source1_nominal_field1202, source_ml_source1_nominal_field1203, source_ml_source1_nominal_field1204, source_ml_source1_nominal_field1205, source_ml_source1_nominal_field1206, source_ml_source1_nominal_field1207) => {
                                                                                            let source_ml_source1_nominal_field1202 = *source_ml_source1_nominal_field1202;
                                                                                            let source_ml_source1_nominal_field1206 = *source_ml_source1_nominal_field1206;
                                                                                            let source_ml_source1_nominal_field1207 = *source_ml_source1_nominal_field1207;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1208 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_nominal_field1201.clone(), Box::new(source_ml_source1_nominal_field1202.clone()), source_ml_source1_nominal_field1203, source_ml_source1_nominal_field1204, source_ml_source1_nominal_override1208.clone(), Box::new(source_ml_source1_nominal_field1206.clone()), Box::new(source_ml_source1_nominal_field1207.clone()))
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1209, source_ml_source1_nominal_field1210, source_ml_source1_nominal_field1211, source_ml_source1_nominal_field1212, source_ml_source1_nominal_field1213, source_ml_source1_nominal_field1214, source_ml_source1_nominal_field1215) => {
                                                                                            let source_ml_source1_nominal_field1210 = *source_ml_source1_nominal_field1210;
                                                                                            let source_ml_source1_nominal_field1214 = *source_ml_source1_nominal_field1214;
                                                                                            let source_ml_source1_nominal_field1215 = *source_ml_source1_nominal_field1215;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1216 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_nominal_field1209.clone(), Box::new(source_ml_source1_nominal_field1210.clone()), source_ml_source1_nominal_field1211, source_ml_source1_nominal_field1212, source_ml_source1_nominal_override1216.clone(), Box::new(source_ml_source1_nominal_field1214.clone()), Box::new(source_ml_source1_nominal_field1215.clone()))
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1217, source_ml_source1_nominal_field1218, source_ml_source1_nominal_field1219, source_ml_source1_nominal_field1220, source_ml_source1_nominal_field1221, source_ml_source1_nominal_field1222, source_ml_source1_nominal_field1223) => {
                                                                                            let source_ml_source1_nominal_field1218 = *source_ml_source1_nominal_field1218;
                                                                                            let source_ml_source1_nominal_field1222 = *source_ml_source1_nominal_field1222;
                                                                                            let source_ml_source1_nominal_field1223 = *source_ml_source1_nominal_field1223;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1224 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_nominal_field1217.clone(), Box::new(source_ml_source1_nominal_field1218.clone()), source_ml_source1_nominal_field1219, source_ml_source1_nominal_field1220, source_ml_source1_nominal_override1224.clone(), Box::new(source_ml_source1_nominal_field1222.clone()), Box::new(source_ml_source1_nominal_field1223.clone()))
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1225, source_ml_source1_nominal_field1226, source_ml_source1_nominal_field1227, source_ml_source1_nominal_field1228, source_ml_source1_nominal_field1229, source_ml_source1_nominal_field1230, source_ml_source1_nominal_field1231) => {
                                                                                            let source_ml_source1_nominal_field1226 = *source_ml_source1_nominal_field1226;
                                                                                            let source_ml_source1_nominal_field1230 = *source_ml_source1_nominal_field1230;
                                                                                            let source_ml_source1_nominal_field1231 = *source_ml_source1_nominal_field1231;
                                                                                            {
                                                                                                let source_ml_source1_nominal_override1232 = properties.clone();
                                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_nominal_field1225.clone(), Box::new(source_ml_source1_nominal_field1226.clone()), source_ml_source1_nominal_field1227, source_ml_source1_nominal_field1228, source_ml_source1_nominal_override1232.clone(), Box::new(source_ml_source1_nominal_field1230.clone()), Box::new(source_ml_source1_nominal_field1231.clone()))
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source1_nominal_override1248 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                            let source_ml_source1_concat_value1234 = match plan.clone() {
                                                                                                self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(_, _, _, _, source_ml_source1_data_property_value1233) => {
                                                                                                    source_ml_source1_data_property_value1233.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_concat_value1235 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value163 = source_ml_source1_concat_value1234.borrow().clone(); ml_reference_value163 }), { let ml_reference_value164 = source_ml_source1_concat_value1235.borrow().clone(); ml_reference_value164 })
                                                                                            }
                                                                                        }));
                                                                                        self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(source_ml_source1_nominal_override1247.clone()), source_ml_source1_nominal_field1239, source_ml_source1_nominal_field1241.clone(), source_ml_source1_nominal_field1243.clone(), source_ml_source1_nominal_override1248.clone())
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}
