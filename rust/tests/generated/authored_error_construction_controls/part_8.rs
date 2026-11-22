fn ml_main() {
    assert!(((self::test_java_script_error_constructor_mode(self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114]))) == (true)), "assertion 1");
    assert!(((self::test_java_script_error_constructor_mode(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) == (false)), "assertion 2");
    assert!(((self::test_java_script_error_intrinsic_mode(self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]))) == (true)), "assertion 3");
    assert!(((self::test_java_script_error_intrinsic_mode(self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114]))) == (false)), "assertion 4");
    assert!(((self::read_java_script_error_construction_kind_error(self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]), true, self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 5");
    assert!(((self::read_java_script_error_construction_kind_error(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), false, self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 6");
    assert!(((self::read_java_script_error_construction_kind_error(self::ml_string::Text::from_units(vec![67, 117, 115, 116, 111, 109]), false, self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 118, 97, 108, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 108, 97, 115, 115]))), "assertion 7");
    assert!(((self::read_java_script_error_construction_kind_error(self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]), true, self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) == (self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 114, 105, 118, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 98, 105, 110, 100, 115, 32, 116, 104, 101, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114]))), "assertion 8");
    assert!(((self::read_java_script_error_allocation_result_error({
        let source_ml_source2_constructor_field1127 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
            let source_ml_source2_constructor_field1117 = self::ml::Big::from_i128(1);
            {
                let source_ml_source2_constructor_field1118 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1119 = true;
                    {
                        let source_ml_source2_constructor_field1120 = false;
                        {
                            let source_ml_source2_constructor_field1121 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1117.clone(), Box::new(source_ml_source2_constructor_field1118.clone()), source_ml_source2_constructor_field1119, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1120, source_ml_source2_constructor_field1121.clone())
                        }
                    }
                }
            }
        }, {
            let source_ml_source2_constructor_field1122 = self::ml::Big::from_i128(2);
            {
                let source_ml_source2_constructor_field1123 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1124 = true;
                    {
                        let source_ml_source2_constructor_field1125 = true;
                        {
                            let source_ml_source2_constructor_field1126 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1122.clone(), Box::new(source_ml_source2_constructor_field1123.clone()), source_ml_source2_constructor_field1124, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1125, source_ml_source2_constructor_field1126.clone())
                        }
                    }
                }
            }
        }, self::JavaScriptOrdinaryObject::HostErrorObject(self::ml::Big::from_i128(7), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false)), true, false, std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new())), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)))]));
        {
            let source_ml_source2_constructor_field1128 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]));
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_constructor_field1127.clone(), source_ml_source2_constructor_field1128.clone(), None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,)>)
        }
    }, self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(7), false), self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false), self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(1), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), false), self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(2), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), true)])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]))))) == (self::ml_string::Text::from_units(vec![]))), "assertion 9");
    assert!(((self::read_java_script_error_allocation_result_error({
        let source_ml_source2_constructor_field1139 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
            let source_ml_source2_constructor_field1129 = self::ml::Big::from_i128(1);
            {
                let source_ml_source2_constructor_field1130 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1131 = true;
                    {
                        let source_ml_source2_constructor_field1132 = false;
                        {
                            let source_ml_source2_constructor_field1133 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1129.clone(), Box::new(source_ml_source2_constructor_field1130.clone()), source_ml_source2_constructor_field1131, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1132, source_ml_source2_constructor_field1133.clone())
                        }
                    }
                }
            }
        }, {
            let source_ml_source2_constructor_field1134 = self::ml::Big::from_i128(2);
            {
                let source_ml_source2_constructor_field1135 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1136 = true;
                    {
                        let source_ml_source2_constructor_field1137 = true;
                        {
                            let source_ml_source2_constructor_field1138 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1134.clone(), Box::new(source_ml_source2_constructor_field1135.clone()), source_ml_source2_constructor_field1136, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1137, source_ml_source2_constructor_field1138.clone())
                        }
                    }
                }
            }
        }, self::JavaScriptOrdinaryObject::HostErrorObject(self::ml::Big::from_i128(7), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false)), true, false, std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new())), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)))]));
        {
            let source_ml_source2_constructor_field1140 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]));
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_constructor_field1139.clone(), source_ml_source2_constructor_field1140.clone(), None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,)>)
        }
    }, self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false), self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false), self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(1), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), false), self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(2), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), true)])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]))))) == (self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 103, 108, 111, 98, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]))), "assertion 10");
    assert!(((self::read_java_script_error_prior_directory_error({
        let source_ml_source2_constructor_field1146 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
            let source_ml_source2_constructor_field1141 = self::ml::Big::from_i128(1);
            {
                let source_ml_source2_constructor_field1142 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1143 = true;
                    {
                        let source_ml_source2_constructor_field1144 = false;
                        {
                            let source_ml_source2_constructor_field1145 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1141.clone(), Box::new(source_ml_source2_constructor_field1142.clone()), source_ml_source2_constructor_field1143, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1144, source_ml_source2_constructor_field1145.clone())
                        }
                    }
                }
            }
        }, self::JavaScriptOrdinaryObject::HostErrorObject(self::ml::Big::from_i128(7), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false)), true, false, std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new())), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)))]));
        {
            let source_ml_source2_constructor_field1147 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]));
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_constructor_field1146.clone(), source_ml_source2_constructor_field1147.clone(), None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,)>)
        }
    }, self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(1), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), false), self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(2), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), true)])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]))))) == (self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))), "assertion 11");
    assert!(((self::read_java_script_error_prior_directory_error({
        let source_ml_source2_constructor_field1158 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
            let source_ml_source2_constructor_field1148 = self::ml::Big::from_i128(1);
            {
                let source_ml_source2_constructor_field1149 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1150 = true;
                    {
                        let source_ml_source2_constructor_field1151 = true;
                        {
                            let source_ml_source2_constructor_field1152 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1148.clone(), Box::new(source_ml_source2_constructor_field1149.clone()), source_ml_source2_constructor_field1150, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1151, source_ml_source2_constructor_field1152.clone())
                        }
                    }
                }
            }
        }, {
            let source_ml_source2_constructor_field1153 = self::ml::Big::from_i128(2);
            {
                let source_ml_source2_constructor_field1154 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1155 = true;
                    {
                        let source_ml_source2_constructor_field1156 = true;
                        {
                            let source_ml_source2_constructor_field1157 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1153.clone(), Box::new(source_ml_source2_constructor_field1154.clone()), source_ml_source2_constructor_field1155, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1156, source_ml_source2_constructor_field1157.clone())
                        }
                    }
                }
            }
        }, self::JavaScriptOrdinaryObject::HostErrorObject(self::ml::Big::from_i128(7), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false)), true, false, std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new())), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)))]));
        {
            let source_ml_source2_constructor_field1159 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]));
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_constructor_field1158.clone(), source_ml_source2_constructor_field1159.clone(), None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,)>)
        }
    }, self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(1), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), false), self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(2), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), true)])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]))))) == (self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))), "assertion 12");
    assert!(((self::read_java_script_error_prior_directory_error({
        let source_ml_source2_constructor_field1170 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
            let source_ml_source2_constructor_field1160 = self::ml::Big::from_i128(1);
            {
                let source_ml_source2_constructor_field1161 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1162 = true;
                    {
                        let source_ml_source2_constructor_field1163 = false;
                        {
                            let source_ml_source2_constructor_field1164 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1160.clone(), Box::new(source_ml_source2_constructor_field1161.clone()), source_ml_source2_constructor_field1162, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1163, source_ml_source2_constructor_field1164.clone())
                        }
                    }
                }
            }
        }, {
            let source_ml_source2_constructor_field1165 = self::ml::Big::from_i128(2);
            {
                let source_ml_source2_constructor_field1166 = self::JavaScriptSemanticValue::Null;
                {
                    let source_ml_source2_constructor_field1167 = true;
                    {
                        let source_ml_source2_constructor_field1168 = true;
                        {
                            let source_ml_source2_constructor_field1169 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_constructor_field1165.clone(), Box::new(source_ml_source2_constructor_field1166.clone()), source_ml_source2_constructor_field1167, None::<(Option<bool>,)>, None::<(Option<bool>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, source_ml_source2_constructor_field1168, source_ml_source2_constructor_field1169.clone())
                        }
                    }
                }
            }
        }, self::JavaScriptOrdinaryObject::HostErrorObject(self::ml::Big::from_i128(7), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false)), true, false, std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new())), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)), Box::new(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(2), true)))]));
        {
            let source_ml_source2_constructor_field1171 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()));
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_constructor_field1170.clone(), source_ml_source2_constructor_field1171.clone(), None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistryEntry>>>>,)>)
        }
    }, self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(1), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), false), self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(self::ml::Big::from_i128(2), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), true)])), std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml::Big::from_i128(18446744073709551615)]))))) == (self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))), "assertion 13");
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
fn all_literal_error_construction_controls() { main(); }
