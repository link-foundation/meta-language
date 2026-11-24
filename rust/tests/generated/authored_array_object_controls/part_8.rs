pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ({ let ml_reference_array165 = units.clone(); let ml_reference_projection165 = ml_reference_array165.borrow().len() as f64; ml_reference_projection165 })) {
            {
                let unit = ({
                    let source_ml_source1_array_read_value1249 = units.clone();
                    {
                        let source_ml_source1_array_read_index1250 = at;
                        { let ml_reference_array166 = source_ml_source1_array_read_value1249.clone(); let ml_reference_position166 = source_ml_source1_array_read_index1250; let ml_reference_projection166 = self::ml_array::optional_at(&*ml_reference_array166.borrow(), self::ml_array::number_index(ml_reference_position166)); ml_reference_projection166 }
                    }
                }).clone().unwrap_or_else(|| (-1f64));
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1251) => {
                        {
                            let source_ml_source1_f1275 = source_ml_source1_f1251;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f1252) => {
                        {
                            let source_ml_source1_f1284 = source_ml_source1_f1252;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1253) => {
                        {
                            let source_ml_source1_f1293 = source_ml_source1_f1253.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Text(units) => {
                        ({
                            let source_ml_source1_array_read_value1299 = units.clone();
                            {
                                let source_ml_source1_array_read_index1300 = at;
                                { let ml_reference_array167 = source_ml_source1_array_read_value1299.clone(); let ml_reference_position167 = source_ml_source1_array_read_index1300; let ml_reference_projection167 = self::ml_array::optional_at(&*ml_reference_array167.borrow(), self::ml_array::number_index(ml_reference_position167)); ml_reference_projection167 }
                            }
                        }).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f1254, source_ml_source1_f1255) => {
                        {
                            let source_ml_source1_f1305 = source_ml_source1_f1254.clone();
                            {
                                let text = source_ml_source1_f1255.clone();
                                {
                                    let source_ml_source1_string_unit_operand1309 = text.clone();
                                    {
                                        let source_ml_source1_string_unit_operand1310 = at;
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
                                                    let source_ml_source1_string_unit_operand1316 = {
                                                        let source_ml_source1_string_range_operand1311 = text_2.clone();
                                                        {
                                                            let source_ml_source1_string_range_operand1312 = index;
                                                            {
                                                                let source_ml_source1_string_range_operand1313 = (index + 1f64);
                                                                {
                                                                    let source_ml_source1_string_range_position1314 = source_ml_source1_string_range_operand1312;
                                                                    {
                                                                        let source_ml_source1_string_range_position1315 = source_ml_source1_string_range_operand1313;
                                                                        source_ml_source1_string_range_operand1311.slice(source_ml_source1_string_range_position1314, source_ml_source1_string_range_position1315)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_string_unit_operand1317 = 0f64;
                                                        source_ml_source1_string_unit_operand1316.code_point_at(source_ml_source1_string_unit_operand1317)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source1_string_unit_operand1309.clone(), source_ml_source1_string_unit_operand1310)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1256) => {
                        {
                            let source_ml_source1_f1323 = source_ml_source1_f1256.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1257, source_ml_source1_f1258) => {
                        {
                            let source_ml_source1_f1332 = source_ml_source1_f1257.clone();
                            {
                                let source_ml_source1_f1333 = source_ml_source1_f1258;
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1334) => {
                            {
                                let source_ml_source1_f1358 = source_ml_source1_f1334;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1335) => {
                            {
                                let source_ml_source1_f1367 = source_ml_source1_f1335;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1336) => {
                            {
                                let source_ml_source1_f1376 = source_ml_source1_f1336.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Text(units_2) => {
                            ({
                                let source_ml_source1_array_read_value1382 = units_2.clone();
                                {
                                    let source_ml_source1_array_read_index1383 = at;
                                    { let ml_reference_array168 = source_ml_source1_array_read_value1382.clone(); let ml_reference_position168 = source_ml_source1_array_read_index1383; let ml_reference_projection168 = self::ml_array::optional_at(&*ml_reference_array168.borrow(), self::ml_array::number_index(ml_reference_position168)); ml_reference_projection168 }
                                }
                            }).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f1337, source_ml_source1_f1338) => {
                            {
                                let source_ml_source1_f1388 = source_ml_source1_f1337.clone();
                                {
                                    let text_2 = source_ml_source1_f1338.clone();
                                    {
                                        let source_ml_source1_string_unit_operand1392 = text_2.clone();
                                        {
                                            let source_ml_source1_string_unit_operand1393 = at;
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
                                                        let source_ml_source1_string_unit_operand1399 = {
                                                            let source_ml_source1_string_range_operand1394 = text_3.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand1395 = index;
                                                                {
                                                                    let source_ml_source1_string_range_operand1396 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source1_string_range_position1397 = source_ml_source1_string_range_operand1395;
                                                                        {
                                                                            let source_ml_source1_string_range_position1398 = source_ml_source1_string_range_operand1396;
                                                                            source_ml_source1_string_range_operand1394.slice(source_ml_source1_string_range_position1397, source_ml_source1_string_range_position1398)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_string_unit_operand1400 = 0f64;
                                                            source_ml_source1_string_unit_operand1399.code_point_at(source_ml_source1_string_unit_operand1400)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source1_string_unit_operand1392.clone(), source_ml_source1_string_unit_operand1393)
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1339) => {
                            {
                                let source_ml_source1_f1406 = source_ml_source1_f1339.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1340, source_ml_source1_f1341) => {
                            {
                                let source_ml_source1_f1415 = source_ml_source1_f1340.clone();
                                {
                                    let source_ml_source1_f1416 = source_ml_source1_f1341;
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

pub fn source_ml_temporary_read_java_script_semantic_text_string_loop4(mut text: self::ml_string::Text, mut source_ml_temporary_values3: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_temporary_index3: self::ml::Big) -> self::ml_string::Text {
    loop {
        return if (source_ml_temporary_index3 < ({ let ml_reference_array169 = source_ml_temporary_values3.clone(); let ml_reference_projection169 = self::ml::Big::from_u128(ml_reference_array169.borrow().len() as u128); ml_reference_projection169 })) {
            {
                let unit = {
                    let source_ml_source1_array_read_value1417 = source_ml_temporary_values3.clone();
                    {
                        let source_ml_source1_array_read_index1418 = source_ml_temporary_index3.clone();
                        { let ml_reference_array170 = source_ml_source1_array_read_value1417.clone(); let ml_reference_position170 = source_ml_source1_array_read_index1418.clone(); let ml_reference_projection170 = self::ml_array::at(&*ml_reference_array170.borrow(), ml_reference_position170.to_index()); ml_reference_projection170 }
                    }
                };
                {
                    let text_2 = text.concat(&{
                        let source_ml_source1_character_argument1419 = unit;
                        self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source1_character_argument1419]).into_iter().map(|ml_unit| ml_unit as u16).collect())
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result {
    MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done,
}

pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop5(mut value: self::JavaScriptSemanticValue, mut length: f64, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result {
    loop {
        return if (at < length) {
            {
                let source_ml_temporary_construct_java_script_semantic_text_units_ignored6 = { let ml_method_owner171 = units.clone(); let ml_method_argument171_0 = self::read_java_script_semantic_text_unit(value.clone(), at); let ml_inserted = vec![ml_method_argument171_0]; let mut ml_method_values171 = ml_method_owner171.borrow_mut(); let ml_length = ml_method_values171.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values171.extend(ml_inserted); ml_length as f64 };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Result {
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Done,
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Return(f64),
}

pub fn source_ml_temporary_find_java_script_property_descriptor_entry_loop7(mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Result {
    loop {
        return if (at < ({ let ml_reference_array172 = properties.clone(); let ml_reference_projection172 = ml_reference_array172.borrow().len() as f64; ml_reference_projection172 })) {
            {
                let property = {
                    let source_ml_source1_array_read_value1420 = properties.clone();
                    {
                        let source_ml_source1_array_read_index1421 = at;
                        { let ml_reference_array173 = source_ml_source1_array_read_value1420.clone(); let ml_reference_position173 = source_ml_source1_array_read_index1421; let ml_reference_projection173 = self::ml_array::optional_at(&*ml_reference_array173.borrow(), self::ml_array::number_index(ml_reference_position173)); ml_reference_projection173 }
                    }
                };
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value(match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1422, _) => {
                        let source_ml_source1_data_property_value1422 = *source_ml_source1_data_property_value1422;
                        source_ml_source1_data_property_value1422.clone()
                    }
                }, key.clone())) {
                    self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Return(at)
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
            self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin8 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin8Return(std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Return(f64),
}

pub fn source_ml_temporary_read_java_script_literal_property_index_loop9(mut key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut value: f64, mut at: f64) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Result {
    loop {
        return if (at < ({ let ml_reference_array174 = key.clone(); let ml_reference_projection174 = ml_reference_array174.borrow().len() as f64; ml_reference_projection174 })) {
            {
                let digit = (match {
                    let source_ml_source1_array_read_value1423 = key.clone();
                    {
                        let source_ml_source1_array_read_index1424 = at;
                        { let ml_reference_array175 = source_ml_source1_array_read_value1423.clone(); let ml_reference_position175 = source_ml_source1_array_read_index1424; let ml_reference_projection175 = self::ml_array::optional_at(&*ml_reference_array175.borrow(), self::ml_array::number_index(ml_reference_position175)); ml_reference_projection175 }
                    }
                } { Some(ml_present) => ml_present, None => f64::NAN } - 48f64);
                if ((!(self::ml_math::is_integer(digit)) || (digit < (0f64))) || (digit > (9f64))) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Return((-1f64))
                } else {
                    {
                        let value_2 = ((value * 10f64) + digit);
                        {
                            let at_2 = (at + 1f64);
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop9Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result {
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Done,
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Return(f64),
}

pub fn source_ml_temporary_find_java_script_heap_directory_object_loop10(mut objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = {
                        let source_ml_source1_array_read_value1425 = objects.clone();
                        {
                            let source_ml_source1_array_read_index1426 = middle;
                            { let ml_reference_array176 = source_ml_source1_array_read_value1425.clone(); let ml_reference_position176 = source_ml_source1_array_read_index1426; let ml_reference_projection176 = self::ml_array::optional_at(&*ml_reference_array176.borrow(), self::ml_array::number_index(ml_reference_position176)); ml_reference_projection176 }
                        }
                    };
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value1440, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1440.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value1441, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1441.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value1442, _, _, _, _) => {
                                source_ml_source1_data_property_value1442.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value1443, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1443.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value1444, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1444.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value1445, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1445.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value1446, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1446.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value1447, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1447.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1448, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1448.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value1449, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1449.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value1450, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1450.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value1451, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1451.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value1452, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value1452.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value1466, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1466.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value1467, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1467.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value1468, _, _, _, _) => {
                                    source_ml_source1_data_property_value1468.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value1469, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1469.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value1470, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1470.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value1471, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1471.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value1472, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1472.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value1473, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1473.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1474, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1474.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value1475, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1475.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value1476, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1476.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value1477, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1477.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value1478, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value1478.clone()
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
            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result {
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Done,
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Return(bool),
}

pub fn source_ml_temporary_test_java_script_heap_directory_symbol_loop11(mut symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = {
                        let source_ml_source1_array_read_value1479 = symbols.clone();
                        {
                            let source_ml_source1_array_read_index1480 = middle;
                            { let ml_reference_array177 = source_ml_source1_array_read_value1479.clone(); let ml_reference_position177 = source_ml_source1_array_read_index1480; let ml_reference_projection177 = self::ml_array::optional_at(&*ml_reference_array177.borrow(), self::ml_array::number_index(ml_reference_position177)); ml_reference_projection177 }
                        }
                    };
                    if {
                        let source_ml_source1_comparison_operand1481 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand1482 = None::<()>;
                            match (source_ml_source1_comparison_operand1481.clone(), source_ml_source1_comparison_operand1482.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                        }
                    } {
                        self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Return(true)
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
            self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptArrayObjectErrorLoop13Result {
    MlTemporaryReadJavaScriptArrayObjectErrorLoop13Done,
    MlTemporaryReadJavaScriptArrayObjectErrorLoop13Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_array_object_error_loop13(mut length: f64, mut source_ml_temporary_values12: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index12: self::ml::Big) -> self::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Result {
    loop {
        return if (source_ml_temporary_index12 < ({ let ml_reference_array178 = source_ml_temporary_values12.clone(); let ml_reference_projection178 = self::ml::Big::from_u128(ml_reference_array178.borrow().len() as u128); ml_reference_projection178 })) {
            {
                let entry = {
                    let source_ml_source1_array_read_value1483 = source_ml_temporary_values12.clone();
                    {
                        let source_ml_source1_array_read_index1484 = source_ml_temporary_index12.clone();
                        { let ml_reference_array179 = source_ml_source1_array_read_value1483.clone(); let ml_reference_position179 = source_ml_source1_array_read_index1484.clone(); let ml_reference_projection179 = self::ml_array::at(&*ml_reference_array179.borrow(), ml_reference_position179.to_index()); ml_reference_projection179 }
                    }
                };
                {
                    let index = self::read_java_script_array_index(match entry.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1485, _) => {
                            let source_ml_source1_data_property_value1485 = *source_ml_source1_data_property_value1485;
                            source_ml_source1_data_property_value1485.clone()
                        }
                    });
                    if (index >= length) {
                        self::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Return(self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 98, 101, 108, 111, 119, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 110, 103, 116, 104]))
                    } else {
                        {
                            let source_ml_temporary_index12_2 = source_ml_temporary_index12.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index12 = source_ml_temporary_index12_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19 {
    MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin19Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Result {
    MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Done,
}

pub fn source_ml_temporary_plan_java_script_array_ordinary_definition_loop18(mut at: f64, mut replacement: self::JavaScriptOrdinaryProperty, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut index: f64, mut source_ml_temporary_values17: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index17: self::ml::Big) -> self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Result {
    loop {
        return if (source_ml_temporary_index17 < ({ let ml_reference_array180 = source_ml_temporary_values17.clone(); let ml_reference_projection180 = self::ml::Big::from_u128(ml_reference_array180.borrow().len() as u128); ml_reference_projection180 })) {
            {
                let entry = {
                    let source_ml_source1_array_read_value1486 = source_ml_temporary_values17.clone();
                    {
                        let source_ml_source1_array_read_index1487 = source_ml_temporary_index17.clone();
                        { let ml_reference_array181 = source_ml_source1_array_read_value1486.clone(); let ml_reference_position181 = source_ml_source1_array_read_index1487.clone(); let ml_reference_projection181 = self::ml_array::at(&*ml_reference_array181.borrow(), ml_reference_position181.to_index()); ml_reference_projection181 }
                    }
                };
                {
                    let source_ml_temporary_plan_java_script_array_ordinary_definition_ignored21 = { let ml_method_owner182 = properties.clone(); let ml_method_argument182_0 = if (index == at) {
                        replacement.clone()
                    } else {
                        entry.clone()
                    }; let ml_inserted = vec![ml_method_argument182_0]; let mut ml_method_values182 = ml_method_owner182.borrow_mut(); let ml_length = ml_method_values182.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values182.extend(ml_inserted); ml_length as f64 };
                    {
                        let index_2 = (index + 1f64);
                        {
                            let source_ml_temporary_index17_2 = source_ml_temporary_index17.add(&self::ml::Big::from_i128(1));
                            {
                                (index, source_ml_temporary_index17) = (index_2, source_ml_temporary_index17_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Result::MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop18Done
        };
    }
}

pub fn source_ml_temporary_plan_java_script_array_length_definition_loop24(mut unsigned_length: f64, mut blocked: f64, mut source_ml_temporary_values22: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index22: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index22 < ({ let ml_reference_array183 = source_ml_temporary_values22.clone(); let ml_reference_projection183 = self::ml::Big::from_u128(ml_reference_array183.borrow().len() as u128); ml_reference_projection183 })) {
            {
                let property = {
                    let source_ml_source1_array_read_value1488 = source_ml_temporary_values22.clone();
                    {
                        let source_ml_source1_array_read_index1489 = source_ml_temporary_index22.clone();
                        { let ml_reference_array184 = source_ml_source1_array_read_value1488.clone(); let ml_reference_position184 = source_ml_source1_array_read_index1489.clone(); let ml_reference_projection184 = self::ml_array::at(&*ml_reference_array184.borrow(), ml_reference_position184.to_index()); ml_reference_projection184 }
                    }
                };
                {
                    let index = self::read_java_script_array_index(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1490, _) => {
                            let source_ml_source1_data_property_value1490 = *source_ml_source1_data_property_value1490;
                            source_ml_source1_data_property_value1490.clone()
                        }
                    });
                    if (((index >= unsigned_length) && !(({ let ml_reference_value185 = (match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1491) => {
                            source_ml_source1_data_property_value1491.clone()
                        }
                    }).borrow().clone(); ml_reference_value185 }).0)) && (index > blocked)) {
                        {
                            let blocked_2 = index;
                            {
                                let source_ml_temporary_index22_2 = source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                {
                                    (blocked, source_ml_temporary_index22) = (blocked_2, source_ml_temporary_index22_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_temporary_index22_3 = source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index22 = source_ml_temporary_index22_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            blocked
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Result {
    MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Done,
}

pub fn source_ml_temporary_plan_java_script_array_length_definition_loop25(mut new_writable: bool, mut final_length: f64, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_values23: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index23: self::ml::Big) -> self::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Result {
    loop {
        return if (source_ml_temporary_index23 < ({ let ml_reference_array186 = source_ml_temporary_values23.clone(); let ml_reference_projection186 = self::ml::Big::from_u128(ml_reference_array186.borrow().len() as u128); ml_reference_projection186 })) {
            {
                let property = {
                    let source_ml_source1_array_read_value1492 = source_ml_temporary_values23.clone();
                    {
                        let source_ml_source1_array_read_index1493 = source_ml_temporary_index23.clone();
                        { let ml_reference_array187 = source_ml_source1_array_read_value1492.clone(); let ml_reference_position187 = source_ml_source1_array_read_index1493.clone(); let ml_reference_projection187 = self::ml_array::at(&*ml_reference_array187.borrow(), ml_reference_position187.to_index()); ml_reference_projection187 }
                    }
                };
                {
                    let index = self::read_java_script_array_index(match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1494, _) => {
                            let source_ml_source1_data_property_value1494 = *source_ml_source1_data_property_value1494;
                            source_ml_source1_data_property_value1494.clone()
                        }
                    });
                    if ((index < (0f64)) || (index < final_length)) {
                        {
                            let source_ml_temporary_plan_java_script_array_length_definition_ignored26 = { let ml_method_owner188 = properties.clone(); let ml_method_argument188_0 = if self::test_java_script_array_length_key(match property.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1495, _) => {
                                    let source_ml_source1_data_property_value1495 = *source_ml_source1_data_property_value1495;
                                    source_ml_source1_data_property_value1495.clone()
                                }
                            }) {
                                {
                                    let source_ml_source1_nominal_spread1512 = property.clone();
                                    {
                                        let source_ml_source1_nominal_field1513 = match source_ml_source1_nominal_spread1512.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1514, _) => {
                                                let source_ml_source1_data_property_value1514 = *source_ml_source1_data_property_value1514;
                                                source_ml_source1_data_property_value1514.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_nominal_field1515 = match source_ml_source1_nominal_spread1512.clone() {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1516) => {
                                                    source_ml_source1_data_property_value1516.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_nominal_override1517 = std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source1_spread1497 = { let ml_reference_value189 = (match property.clone() {
                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1496) => {
                                                            source_ml_source1_data_property_value1496.clone()
                                                        }
                                                    }).borrow().clone(); ml_reference_value189 };
                                                    {
                                                        let source_ml_source1_spread_field1498 = (source_ml_source1_spread1497.clone()).0;
                                                        {
                                                            let source_ml_source1_spread_field1499 = (source_ml_source1_spread1497.clone()).1;
                                                            {
                                                                let source_ml_source1_spread_field1500 = (source_ml_source1_spread1497.clone()).2;
                                                                {
                                                                    let source_ml_source1_spread_field1501 = (source_ml_source1_spread1497.clone()).3;
                                                                    {
                                                                        let source_ml_source1_spread_field1502 = (source_ml_source1_spread1497.clone()).4;
                                                                        {
                                                                            let source_ml_source1_spread_field1503 = (source_ml_source1_spread1497.clone()).5;
                                                                            {
                                                                                let source_ml_source1_spread_field1504 = (source_ml_source1_spread1497.clone()).6;
                                                                                {
                                                                                    let source_ml_source1_spread_field1505 = (source_ml_source1_spread1497.clone()).7;
                                                                                    {
                                                                                        let source_ml_source1_spread_field1506 = (source_ml_source1_spread1497.clone()).8;
                                                                                        {
                                                                                            let source_ml_source1_spread_field1507 = (source_ml_source1_spread1497.clone()).9;
                                                                                            {
                                                                                                let source_ml_source1_spread_field1508 = (source_ml_source1_spread1497.clone()).10;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field1509 = (source_ml_source1_spread1497.clone()).11;
                                                                                                    {
                                                                                                        let source_ml_source1_field1510 = self::JavaScriptSemanticValue::Number(final_length);
                                                                                                        {
                                                                                                            let source_ml_source1_field1511 = new_writable;
                                                                                                            (source_ml_source1_spread_field1498, source_ml_source1_spread_field1499, source_ml_source1_spread_field1500, source_ml_source1_spread_field1501, source_ml_source1_spread_field1502.clone(), source_ml_source1_spread_field1503, source_ml_source1_spread_field1504.clone(), source_ml_source1_spread_field1505, source_ml_source1_field1510.clone(), source_ml_source1_spread_field1507, source_ml_source1_field1511, source_ml_source1_spread_field1509)
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
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source1_nominal_field1513.clone()), source_ml_source1_nominal_override1517.clone())
                                            }
                                        }
                                    }
                                }
                            } else {
                                property.clone()
                            }; let ml_inserted = vec![ml_method_argument188_0]; let mut ml_method_values188 = ml_method_owner188.borrow_mut(); let ml_length = ml_method_values188.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values188.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_temporary_index23_2 = source_ml_temporary_index23.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_temporary_index23 = source_ml_temporary_index23_2.clone();
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_temporary_index23_3 = source_ml_temporary_index23.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index23 = source_ml_temporary_index23_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Result::MlTemporaryPlanJavaScriptArrayLengthDefinitionLoop25Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Result {
    MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop28Done,
}
