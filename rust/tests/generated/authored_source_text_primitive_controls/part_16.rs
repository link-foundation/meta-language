pub fn plan_java_script_array_property_definition(object: self::JavaScriptOrdinaryObject, key: self::JavaScriptSemanticValue, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::JavaScriptArrayPropertyPlan {
    if self::test_java_script_array_length_key(key.clone()) {
        if !(({ let ml_reference_value204 = proposed.borrow().clone(); ml_reference_value204 }).9) {
            self::plan_java_script_array_ordinary_definition(object.clone(), key.clone(), proposed.clone())
        } else {
            {
                let value = ({ let ml_reference_value205 = proposed.borrow().clone(); ml_reference_value205 }).8;
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
                        let source_ml_source2_array_read_value1315 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value1289, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1289.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value1290, _, _, _) => {
                                source_ml_source2_data_property_value1290.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value1291) => {
                                source_ml_source2_data_property_value1291.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value1292, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1292.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value1293, _) => {
                                source_ml_source2_data_property_value1293.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value1294, _) => {
                                source_ml_source2_data_property_value1294.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value1295, _) => {
                                source_ml_source2_data_property_value1295.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value1296, _) => {
                                source_ml_source2_data_property_value1296.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value1297) => {
                                source_ml_source2_data_property_value1297.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value1298, _, _) => {
                                source_ml_source2_data_property_value1298.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value1299, _, _) => {
                                source_ml_source2_data_property_value1299.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value1300, _, _) => {
                                source_ml_source2_data_property_value1300.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value1301, _, _) => {
                                source_ml_source2_data_property_value1301.clone()
                            }
                        };
                        {
                            let source_ml_source2_array_read_index1316 = self::find_java_script_array_property(match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value1302, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1302.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value1303, _, _, _) => {
                                    source_ml_source2_data_property_value1303.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value1304) => {
                                    source_ml_source2_data_property_value1304.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value1305, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1305.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value1306, _) => {
                                    source_ml_source2_data_property_value1306.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value1307, _) => {
                                    source_ml_source2_data_property_value1307.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value1308, _) => {
                                    source_ml_source2_data_property_value1308.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value1309, _) => {
                                    source_ml_source2_data_property_value1309.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value1310) => {
                                    source_ml_source2_data_property_value1310.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value1311, _, _) => {
                                    source_ml_source2_data_property_value1311.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value1312, _, _) => {
                                    source_ml_source2_data_property_value1312.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value1313, _, _) => {
                                    source_ml_source2_data_property_value1313.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value1314, _, _) => {
                                    source_ml_source2_data_property_value1314.clone()
                                }
                            }, self::read_java_script_array_length_key());
                            { let ml_reference_array206 = source_ml_source2_array_read_value1315.clone(); let ml_reference_position206 = source_ml_source2_array_read_index1316; let ml_reference_projection206 = self::ml_array::optional_at(&*ml_reference_array206.borrow(), self::ml_array::number_index(ml_reference_position206)); ml_reference_projection206 }
                        }
                    };
                    if ((index >= length) && (match (length_property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(({ let ml_reference_value207 = (match length_property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value1317) => {
                            source_ml_source2_data_property_value1317.clone()
                        }
                    }).borrow().clone(); ml_reference_value207 }).10))) {
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
                                                let source_ml_source1_temporary_values75 = match shared_constructor_field1.clone() {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value1320, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value1320.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value1321, _, _, _) => {
                                                        source_ml_source2_data_property_value1321.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value1322) => {
                                                        source_ml_source2_data_property_value1322.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value1323, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value1323.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value1324, _) => {
                                                        source_ml_source2_data_property_value1324.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value1325, _) => {
                                                        source_ml_source2_data_property_value1325.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value1326, _) => {
                                                        source_ml_source2_data_property_value1326.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value1327, _) => {
                                                        source_ml_source2_data_property_value1327.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value1328) => {
                                                        source_ml_source2_data_property_value1328.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value1329, _, _) => {
                                                        source_ml_source2_data_property_value1329.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value1330, _, _) => {
                                                        source_ml_source2_data_property_value1330.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value1331, _, _) => {
                                                        source_ml_source2_data_property_value1331.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value1332, _, _) => {
                                                        source_ml_source2_data_property_value1332.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_temporary_index75 = self::ml::Big::from_i128(0);
                                                    {
                                                        let properties_2 = self::source_ml_source1_temporary_plan_java_script_array_property_definition_loop76(index, properties.clone(), source_ml_source1_temporary_values75.clone(), source_ml_source1_temporary_index75.clone());
                                                        {
                                                            let source_ml_source2_nominal_field1446 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field1448 = shared_constructor_field2;
                                                                {
                                                                    let source_ml_source2_nominal_field1450 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field1452 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field1454 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_override1456 = match shared_constructor_field1.clone() {
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field1334, source_ml_source2_nominal_field1335, source_ml_source2_nominal_field1336, source_ml_source2_nominal_field1337, _, source_ml_source2_nominal_field1339, source_ml_source2_nominal_field1340, source_ml_source2_nominal_field1341, source_ml_source2_nominal_field1342, source_ml_source2_nominal_field1343) => {
                                                                                        let source_ml_source2_nominal_field1335 = *source_ml_source2_nominal_field1335;
                                                                                        let source_ml_source2_nominal_field1342 = *source_ml_source2_nominal_field1342;
                                                                                        self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_nominal_field1334.clone(), Box::new(source_ml_source2_nominal_field1335.clone()), source_ml_source2_nominal_field1336, source_ml_source2_nominal_field1337, properties_2.clone(), source_ml_source2_nominal_field1339.clone(), source_ml_source2_nominal_field1340, source_ml_source2_nominal_field1341.clone(), Box::new(source_ml_source2_nominal_field1342.clone()), source_ml_source2_nominal_field1343.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field1345, source_ml_source2_nominal_field1346, source_ml_source2_nominal_field1347, source_ml_source2_nominal_field1348, _, source_ml_source2_nominal_field1350, source_ml_source2_nominal_field1351, source_ml_source2_nominal_field1352) => {
                                                                                        let source_ml_source2_nominal_field1346 = *source_ml_source2_nominal_field1346;
                                                                                        let source_ml_source2_nominal_field1350 = *source_ml_source2_nominal_field1350;
                                                                                        let source_ml_source2_nominal_field1351 = *source_ml_source2_nominal_field1351;
                                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_nominal_field1345.clone(), Box::new(source_ml_source2_nominal_field1346.clone()), source_ml_source2_nominal_field1347, source_ml_source2_nominal_field1348, properties_2.clone(), Box::new(source_ml_source2_nominal_field1350.clone()), Box::new(source_ml_source2_nominal_field1351.clone()), source_ml_source2_nominal_field1352.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field1354, source_ml_source2_nominal_field1355, source_ml_source2_nominal_field1356, source_ml_source2_nominal_field1357, _) => {
                                                                                        let source_ml_source2_nominal_field1355 = *source_ml_source2_nominal_field1355;
                                                                                        self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_nominal_field1354.clone(), Box::new(source_ml_source2_nominal_field1355.clone()), source_ml_source2_nominal_field1356, source_ml_source2_nominal_field1357, properties_2.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field1360, source_ml_source2_nominal_field1361, source_ml_source2_nominal_field1362, source_ml_source2_nominal_field1363, _, source_ml_source2_nominal_field1365, source_ml_source2_nominal_field1366, source_ml_source2_nominal_field1367, source_ml_source2_nominal_field1368, source_ml_source2_nominal_field1369, source_ml_source2_nominal_field1370) => {
                                                                                        let source_ml_source2_nominal_field1361 = *source_ml_source2_nominal_field1361;
                                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_nominal_field1360.clone(), Box::new(source_ml_source2_nominal_field1361.clone()), source_ml_source2_nominal_field1362, source_ml_source2_nominal_field1363, properties_2.clone(), source_ml_source2_nominal_field1365.clone(), source_ml_source2_nominal_field1366, source_ml_source2_nominal_field1367.clone(), source_ml_source2_nominal_field1368.clone(), source_ml_source2_nominal_field1369.clone(), source_ml_source2_nominal_field1370.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field1372, source_ml_source2_nominal_field1373, source_ml_source2_nominal_field1374, source_ml_source2_nominal_field1375, _, source_ml_source2_nominal_field1377) => {
                                                                                        let source_ml_source2_nominal_field1373 = *source_ml_source2_nominal_field1373;
                                                                                        self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_nominal_field1372.clone(), Box::new(source_ml_source2_nominal_field1373.clone()), source_ml_source2_nominal_field1374, source_ml_source2_nominal_field1375, properties_2.clone(), source_ml_source2_nominal_field1377.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field1379, source_ml_source2_nominal_field1380, source_ml_source2_nominal_field1381, source_ml_source2_nominal_field1382, _, source_ml_source2_nominal_field1384) => {
                                                                                        let source_ml_source2_nominal_field1380 = *source_ml_source2_nominal_field1380;
                                                                                        self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_nominal_field1379.clone(), Box::new(source_ml_source2_nominal_field1380.clone()), source_ml_source2_nominal_field1381, source_ml_source2_nominal_field1382, properties_2.clone(), source_ml_source2_nominal_field1384.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field1386, source_ml_source2_nominal_field1387, source_ml_source2_nominal_field1388, source_ml_source2_nominal_field1389, _, source_ml_source2_nominal_field1391) => {
                                                                                        let source_ml_source2_nominal_field1387 = *source_ml_source2_nominal_field1387;
                                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_nominal_field1386.clone(), Box::new(source_ml_source2_nominal_field1387.clone()), source_ml_source2_nominal_field1388, source_ml_source2_nominal_field1389, properties_2.clone(), source_ml_source2_nominal_field1391.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field1393, source_ml_source2_nominal_field1394, source_ml_source2_nominal_field1395, source_ml_source2_nominal_field1396, _, source_ml_source2_nominal_field1398) => {
                                                                                        let source_ml_source2_nominal_field1394 = *source_ml_source2_nominal_field1394;
                                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_nominal_field1393.clone(), Box::new(source_ml_source2_nominal_field1394.clone()), source_ml_source2_nominal_field1395, source_ml_source2_nominal_field1396, properties_2.clone(), source_ml_source2_nominal_field1398.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field1400, source_ml_source2_nominal_field1401, source_ml_source2_nominal_field1402, source_ml_source2_nominal_field1403, source_ml_source2_nominal_field1404, source_ml_source2_nominal_field1405, source_ml_source2_nominal_field1406, source_ml_source2_nominal_field1407, _) => {
                                                                                        let source_ml_source2_nominal_field1401 = *source_ml_source2_nominal_field1401;
                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_nominal_field1400.clone(), Box::new(source_ml_source2_nominal_field1401.clone()), source_ml_source2_nominal_field1402, source_ml_source2_nominal_field1403.clone(), source_ml_source2_nominal_field1404.clone(), source_ml_source2_nominal_field1405.clone(), source_ml_source2_nominal_field1406.clone(), source_ml_source2_nominal_field1407, properties_2.clone())
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field1410, source_ml_source2_nominal_field1411, source_ml_source2_nominal_field1412, source_ml_source2_nominal_field1413, _, source_ml_source2_nominal_field1415, source_ml_source2_nominal_field1416) => {
                                                                                        let source_ml_source2_nominal_field1411 = *source_ml_source2_nominal_field1411;
                                                                                        let source_ml_source2_nominal_field1415 = *source_ml_source2_nominal_field1415;
                                                                                        let source_ml_source2_nominal_field1416 = *source_ml_source2_nominal_field1416;
                                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_nominal_field1410.clone(), Box::new(source_ml_source2_nominal_field1411.clone()), source_ml_source2_nominal_field1412, source_ml_source2_nominal_field1413, properties_2.clone(), Box::new(source_ml_source2_nominal_field1415.clone()), Box::new(source_ml_source2_nominal_field1416.clone()))
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field1418, source_ml_source2_nominal_field1419, source_ml_source2_nominal_field1420, source_ml_source2_nominal_field1421, _, source_ml_source2_nominal_field1423, source_ml_source2_nominal_field1424) => {
                                                                                        let source_ml_source2_nominal_field1419 = *source_ml_source2_nominal_field1419;
                                                                                        let source_ml_source2_nominal_field1423 = *source_ml_source2_nominal_field1423;
                                                                                        let source_ml_source2_nominal_field1424 = *source_ml_source2_nominal_field1424;
                                                                                        self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_nominal_field1418.clone(), Box::new(source_ml_source2_nominal_field1419.clone()), source_ml_source2_nominal_field1420, source_ml_source2_nominal_field1421, properties_2.clone(), Box::new(source_ml_source2_nominal_field1423.clone()), Box::new(source_ml_source2_nominal_field1424.clone()))
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field1426, source_ml_source2_nominal_field1427, source_ml_source2_nominal_field1428, source_ml_source2_nominal_field1429, _, source_ml_source2_nominal_field1431, source_ml_source2_nominal_field1432) => {
                                                                                        let source_ml_source2_nominal_field1427 = *source_ml_source2_nominal_field1427;
                                                                                        let source_ml_source2_nominal_field1431 = *source_ml_source2_nominal_field1431;
                                                                                        let source_ml_source2_nominal_field1432 = *source_ml_source2_nominal_field1432;
                                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_nominal_field1426.clone(), Box::new(source_ml_source2_nominal_field1427.clone()), source_ml_source2_nominal_field1428, source_ml_source2_nominal_field1429, properties_2.clone(), Box::new(source_ml_source2_nominal_field1431.clone()), Box::new(source_ml_source2_nominal_field1432.clone()))
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field1434, source_ml_source2_nominal_field1435, source_ml_source2_nominal_field1436, source_ml_source2_nominal_field1437, _, source_ml_source2_nominal_field1439, source_ml_source2_nominal_field1440) => {
                                                                                        let source_ml_source2_nominal_field1435 = *source_ml_source2_nominal_field1435;
                                                                                        let source_ml_source2_nominal_field1439 = *source_ml_source2_nominal_field1439;
                                                                                        let source_ml_source2_nominal_field1440 = *source_ml_source2_nominal_field1440;
                                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_nominal_field1434.clone(), Box::new(source_ml_source2_nominal_field1435.clone()), source_ml_source2_nominal_field1436, source_ml_source2_nominal_field1437, properties_2.clone(), Box::new(source_ml_source2_nominal_field1439.clone()), Box::new(source_ml_source2_nominal_field1440.clone()))
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override1457 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                        let source_ml_source2_concat_value1443 = shared_constructor_field5.clone();
                                                                                        {
                                                                                            let source_ml_source2_concat_value1444 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]));
                                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value208 = source_ml_source2_concat_value1443.borrow().clone(); ml_reference_value208 }), { let ml_reference_value209 = source_ml_source2_concat_value1444.borrow().clone(); ml_reference_value209 })
                                                                                        }
                                                                                    }));
                                                                                    self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(Box::new(source_ml_source2_nominal_override1456.clone()), source_ml_source2_nominal_field1448, source_ml_source2_nominal_field1450.clone(), source_ml_source2_nominal_field1452.clone(), source_ml_source2_nominal_override1457.clone())
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

pub fn dependency_function2(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integral = if (value < (0f64)) {
                (-(((-value)).floor()))
            } else {
                value.floor()
            };
            {
                let remainder = (integral % 4294967296f64);
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

pub fn read_utf16_text_integer(value: f64) -> f64 {
    if value.is_nan() {
        0f64
    } else {
        value.trunc()
    }
}

pub fn read_utf16_text_slice_position(value: f64, length: f64) -> f64 {
    {
        let integer = self::read_utf16_text_integer(value);
        if (integer < (0f64)) {
            self::ml_math::max((length + integer), 0f64)
        } else {
            self::ml_math::min(integer, length)
        }
    }
}

pub fn slice_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, start: f64, end: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let first = self::read_utf16_text_slice_position(start, { let ml_reference_array210 = text.clone(); let ml_reference_projection210 = ml_reference_array210.borrow().len() as f64; ml_reference_projection210 });
        {
            let last = self::read_utf16_text_slice_position(end, { let ml_reference_array211 = text.clone(); let ml_reference_projection211 = ml_reference_array211.borrow().len() as f64; ml_reference_projection211 });
            {
                let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                {
                    let result_2 = self::source_ml_source1_temporary_slice_utf16_text_loop77(text.clone(), last, result.clone(), first);
                    result_2.clone()
                }
            }
        }
    }
}

pub fn substring_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, start: f64, end: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let first = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(start), 0f64), { let ml_reference_array212 = text.clone(); let ml_reference_projection212 = ml_reference_array212.borrow().len() as f64; ml_reference_projection212 });
        {
            let last = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(end), 0f64), { let ml_reference_array213 = text.clone(); let ml_reference_projection213 = ml_reference_array213.borrow().len() as f64; ml_reference_projection213 });
            self::slice_utf16_text(text.clone(), self::ml_math::min(first, last), self::ml_math::max(first, last))
        }
    }
}

pub fn character_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, position: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ({ let ml_reference_array214 = text.clone(); let ml_reference_projection214 = ml_reference_array214.borrow().len() as f64; ml_reference_projection214 }))) {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
        } else {
            std::rc::Rc::new(std::cell::RefCell::new(vec![({ let ml_reference_array215 = text.clone(); let ml_reference_position215 = at; let ml_reference_projection215 = self::ml_array::optional_at(&*ml_reference_array215.borrow(), self::ml_array::number_index(ml_reference_position215)); ml_reference_projection215 }).clone().unwrap_or_else(|| 0f64)]))
        }
    }
}

pub fn relative_character_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, position: f64) -> std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>> {
    {
        let integer = self::read_utf16_text_integer(position);
        {
            let at = if (integer < (0f64)) {
                ({ let ml_reference_array216 = text.clone(); let ml_reference_projection216 = ml_reference_array216.borrow().len() as f64; ml_reference_projection216 } + integer)
            } else {
                integer
            };
            if ((at < (0f64)) || (at >= ({ let ml_reference_array217 = text.clone(); let ml_reference_projection217 = ml_reference_array217.borrow().len() as f64; ml_reference_projection217 }))) {
                std::rc::Rc::new(std::cell::RefCell::new(Vec::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>::new()))
            } else {
                std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![({ let ml_reference_array218 = text.clone(); let ml_reference_position218 = at; let ml_reference_projection218 = self::ml_array::optional_at(&*ml_reference_array218.borrow(), self::ml_array::number_index(ml_reference_position218)); ml_reference_projection218 }).clone().unwrap_or_else(|| 0f64)]))]))
            }
        }
    }
}

pub fn code_point_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, position: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ({ let ml_reference_array219 = text.clone(); let ml_reference_projection219 = ml_reference_array219.borrow().len() as f64; ml_reference_projection219 }))) {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
        } else {
            {
                let first = ({ let ml_reference_array220 = text.clone(); let ml_reference_position220 = at; let ml_reference_projection220 = self::ml_array::optional_at(&*ml_reference_array220.borrow(), self::ml_array::number_index(ml_reference_position220)); ml_reference_projection220 }).clone().unwrap_or_else(|| 0f64);
                {
                    let second = ({
                        let source_ml_source2_array_read_index1465 = (at + 1f64);
                        { let ml_reference_array221 = text.clone(); let ml_reference_position221 = source_ml_source2_array_read_index1465; let ml_reference_projection221 = self::ml_array::optional_at(&*ml_reference_array221.borrow(), self::ml_array::number_index(ml_reference_position221)); ml_reference_projection221 }
                    }).clone().unwrap_or_else(|| (-1f64));
                    std::rc::Rc::new(std::cell::RefCell::new(vec![if ((((first >= (55296f64)) && (first <= (56319f64))) && (second >= (56320f64))) && (second <= (57343f64))) {
                        (((65536f64 + ((first - 55296f64) * 1024f64)) + second) - 56320f64)
                    } else {
                        first
                    }]))
                }
            }
        }
    }
}

pub fn points_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>::new()));
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_points_utf16_text_loop78(text.clone(), result.clone(), at);
                result_2.clone()
            }
        }
    }
}

pub fn split_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, separator: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, limit: f64, missing: bool) -> std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>> {
    {
        let count = self::dependency_function2(limit);
        {
            let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>::new()));
            if (count == (0f64)) {
                result.clone()
            } else {
                if missing {
                    std::rc::Rc::new(std::cell::RefCell::new(vec![self::slice_utf16_text(text.clone(), 0f64, { let ml_reference_array222 = text.clone(); let ml_reference_projection222 = ml_reference_array222.borrow().len() as f64; ml_reference_projection222 })]))
                } else {
                    if (({ let ml_reference_array223 = separator.clone(); let ml_reference_projection223 = ml_reference_array223.borrow().len() as f64; ml_reference_projection223 }) == (0f64)) {
                        {
                            let at = 0f64;
                            {
                                let result_2 = self::source_ml_source1_temporary_split_utf16_text_loop79(text.clone(), count, result.clone(), at);
                                result_2.clone()
                            }
                        }
                    } else {
                        {
                            let start = 0f64;
                            {
                                let source_ml_source2_s1466 = self::source_ml_source1_temporary_split_utf16_text_loop80(text.clone(), separator.clone(), count, result.clone(), start);
                                match source_ml_source2_s1466.clone() {
                                    self::MlSource1TemporarySplitUtf16TextLoop80Result::MlSource1TemporarySplitUtf16TextLoop80Done(result_3, start_2) => {
                                        {
                                            let result_4 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value224 = result_3.borrow().clone(); ml_reference_value224 }, vec![self::slice_utf16_text(text.clone(), start_2, { let ml_reference_array225 = text.clone(); let ml_reference_projection225 = ml_reference_array225.borrow().len() as f64; ml_reference_projection225 })])));
                                            result_4.clone()
                                        }
                                    }
                                    self::MlSource1TemporarySplitUtf16TextLoop80Result::MlSource1TemporarySplitUtf16TextLoop80Return(source_ml_source1_temporary_result80) => {
                                        source_ml_source1_temporary_result80.clone()
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

pub fn match_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, start: f64) -> bool {
    if ((start < (0f64)) || (((start + { let ml_reference_array226 = search.clone(); let ml_reference_projection226 = ml_reference_array226.borrow().len() as f64; ml_reference_projection226 })) > ({ let ml_reference_array227 = text.clone(); let ml_reference_projection227 = ml_reference_array227.borrow().len() as f64; ml_reference_projection227 }))) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source2_s1467 = self::source_ml_source1_temporary_match_utf16_text_loop81(text.clone(), search.clone(), start, at);
                match source_ml_source2_s1467.clone() {
                    self::MlSource1TemporaryMatchUtf16TextLoop81Result::MlSource1TemporaryMatchUtf16TextLoop81Done => {
                        true
                    }
                    self::MlSource1TemporaryMatchUtf16TextLoop81Result::MlSource1TemporaryMatchUtf16TextLoop81Return(source_ml_source1_temporary_result81) => {
                        source_ml_source1_temporary_result81
                    }
                }
            }
        }
    }
}

pub fn index_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, position: f64) -> f64 {
    {
        let start = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(position), 0f64), { let ml_reference_array228 = text.clone(); let ml_reference_projection228 = ml_reference_array228.borrow().len() as f64; ml_reference_projection228 });
        {
            let source_ml_source2_s1468 = self::source_ml_source1_temporary_index_utf16_text_loop82(text.clone(), search.clone(), start);
            match source_ml_source2_s1468.clone() {
                self::MlSource1TemporaryIndexUtf16TextLoop82Result::MlSource1TemporaryIndexUtf16TextLoop82Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryIndexUtf16TextLoop82Result::MlSource1TemporaryIndexUtf16TextLoop82Return(source_ml_source1_temporary_result82) => {
                    source_ml_source1_temporary_result82
                }
            }
        }
    }
}

pub fn last_index_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, position: f64) -> f64 {
    {
        let integer = if position.is_nan() {
            f64::INFINITY
        } else {
            self::read_utf16_text_integer(position)
        };
        {
            let start = self::ml_math::min(self::ml_math::max(integer, 0f64), ({ let ml_reference_array229 = text.clone(); let ml_reference_projection229 = ml_reference_array229.borrow().len() as f64; ml_reference_projection229 } - { let ml_reference_array230 = search.clone(); let ml_reference_projection230 = ml_reference_array230.borrow().len() as f64; ml_reference_projection230 }));
            {
                let source_ml_source2_s1469 = self::source_ml_source1_temporary_last_index_utf16_text_loop83(text.clone(), search.clone(), start);
                match source_ml_source2_s1469.clone() {
                    self::MlSource1TemporaryLastIndexUtf16TextLoop83Result::MlSource1TemporaryLastIndexUtf16TextLoop83Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryLastIndexUtf16TextLoop83Result::MlSource1TemporaryLastIndexUtf16TextLoop83Return(source_ml_source1_temporary_result83) => {
                        source_ml_source1_temporary_result83
                    }
                }
            }
        }
    }
}

pub fn compare_utf16_text(left: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, right: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> f64 {
    {
        let count = self::ml_math::min({ let ml_reference_array231 = left.clone(); let ml_reference_projection231 = ml_reference_array231.borrow().len() as f64; ml_reference_projection231 }, { let ml_reference_array232 = right.clone(); let ml_reference_projection232 = ml_reference_array232.borrow().len() as f64; ml_reference_projection232 });
        {
            let at = 0f64;
            {
                let source_ml_source2_s1470 = self::source_ml_source1_temporary_compare_utf16_text_loop84(left.clone(), right.clone(), count, at);
                match source_ml_source2_s1470.clone() {
                    self::MlSource1TemporaryCompareUtf16TextLoop84Result::MlSource1TemporaryCompareUtf16TextLoop84Done => {
                        if (({ let ml_reference_array233 = left.clone(); let ml_reference_projection233 = ml_reference_array233.borrow().len() as f64; ml_reference_projection233 }) == ({ let ml_reference_array234 = right.clone(); let ml_reference_projection234 = ml_reference_array234.borrow().len() as f64; ml_reference_projection234 })) {
                            0f64
                        } else {
                            if (({ let ml_reference_array235 = left.clone(); let ml_reference_projection235 = ml_reference_array235.borrow().len() as f64; ml_reference_projection235 }) < ({ let ml_reference_array236 = right.clone(); let ml_reference_projection236 = ml_reference_array236.borrow().len() as f64; ml_reference_projection236 })) {
                                (-1f64)
                            } else {
                                1f64
                            }
                        }
                    }
                    self::MlSource1TemporaryCompareUtf16TextLoop84Result::MlSource1TemporaryCompareUtf16TextLoop84Return(source_ml_source1_temporary_result84) => {
                        source_ml_source1_temporary_result84
                    }
                }
            }
        }
    }
}

pub fn starts_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> bool {
    self::match_utf16_text(text.clone(), search.clone(), 0f64)
}

pub fn ends_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> bool {
    self::match_utf16_text(text.clone(), search.clone(), ({ let ml_reference_array237 = text.clone(); let ml_reference_projection237 = ml_reference_array237.borrow().len() as f64; ml_reference_projection237 } - { let ml_reference_array238 = search.clone(); let ml_reference_projection238 = ml_reference_array238.borrow().len() as f64; ml_reference_projection238 }))
}

pub fn includes_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> bool {
    ((self::index_utf16_text(text.clone(), search.clone(), 0f64)) >= (0f64))
}

pub fn join_utf16_text(values: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>>, separator: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_join_utf16_text_loop85(values.clone(), separator.clone(), result.clone(), at);
                result_2.clone()
            }
        }
    }
}

pub fn trim_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, start: bool, end: bool) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let first = 0f64;
        {
            let last = { let ml_reference_array239 = text.clone(); let ml_reference_projection239 = ml_reference_array239.borrow().len() as f64; ml_reference_projection239 };
            {
                let first_3 = if start {
                    {
                        let first_2 = self::source_ml_source1_temporary_trim_utf16_text_loop88(text.clone(), first, last);
                        first_2
                    }
                } else {
                    first
                };
                {
                    let last_3 = if end {
                        {
                            let last_2 = self::source_ml_source1_temporary_trim_utf16_text_loop89(text.clone(), first_3, last);
                            last_2
                        }
                    } else {
                        last
                    };
                    self::slice_utf16_text(text.clone(), first_3, last_3)
                }
            }
        }
    }
}

pub fn character_units_utf16_text(values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_character_units_utf16_text_loop90(values.clone(), result.clone(), at);
                result_2.clone()
            }
        }
    }
}

pub fn code_point_units_utf16_text(values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)]));
        {
            let at = 0f64;
            {
                let source_ml_source2_s1471 = self::source_ml_source1_temporary_code_point_units_utf16_text_loop91(values.clone(), result.clone(), at);
                match source_ml_source2_s1471.clone() {
                    self::MlSource1TemporaryCodePointUnitsUtf16TextLoop91Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop91Done(result_2) => {
                        result_2.clone()
                    }
                    self::MlSource1TemporaryCodePointUnitsUtf16TextLoop91Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop91Return(source_ml_source1_temporary_result91) => {
                        source_ml_source1_temporary_result91.clone()
                    }
                }
            }
        }
    }
}

pub fn read_utf8_text_units(bytes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    self::read_java_script_utf8_string_units(bytes.clone())
}

pub fn display_points_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_display_points_utf16_text_loop92(text.clone(), result.clone(), at);
                result_2.clone()
            }
        }
    }
}

pub fn display_utf8_from_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let bytes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let points = self::display_points_utf16_text(text.clone());
            {
                let at = 0f64;
                {
                    let bytes_2 = self::source_ml_source1_temporary_display_utf8_from_utf16_text_loop94(bytes.clone(), points.clone(), at);
                    bytes_2.clone()
                }
            }
        }
    }
}

pub fn plan_utf16_text_substitution(replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let start = 0f64;
            {
                let at = 0f64;
                {
                    let source_ml_source2_s1472 = self::source_ml_source1_temporary_plan_utf16_text_substitution_loop95(replacement.clone(), result.clone(), start, at);
                    match source_ml_source2_s1472.clone() {
                        self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop95Result::MlSource1TemporaryPlanUtf16TextSubstitutionLoop95Done(result_2, start_2) => {
                            {
                                let result_4 = if (start_2 < ({ let ml_reference_array240 = replacement.clone(); let ml_reference_projection240 = ml_reference_array240.borrow().len() as f64; ml_reference_projection240 })) {
                                    {
                                        let result_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value241 = result_2.borrow().clone(); ml_reference_value241 }, vec![0f64, start_2, { let ml_reference_array242 = replacement.clone(); let ml_reference_projection242 = ml_reference_array242.borrow().len() as f64; ml_reference_projection242 }])));
                                        result_3.clone()
                                    }
                                } else {
                                    result_2.clone()
                                };
                                result_4.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn replace_utf16_text(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, all: bool) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let plan = self::plan_utf16_text_substitution(replacement.clone());
            {
                let start = 0f64;
                {
                    let from = 0f64;
                    {
                        let source_ml_source2_s1473 = self::source_ml_source1_temporary_replace_utf16_text_loop96(text.clone(), search.clone(), replacement.clone(), all, result.clone(), plan.clone(), start, from, true);
                        match source_ml_source2_s1473.clone() {
                            self::MlSource1TemporaryReplaceUtf16TextLoop96Result::MlSource1TemporaryReplaceUtf16TextLoop96Done(result_2, start_2) => {
                                {
                                    let result_3 = self::source_ml_source1_temporary_replace_utf16_text_loop97(text.clone(), result_2.clone(), start_2);
                                    result_3.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
