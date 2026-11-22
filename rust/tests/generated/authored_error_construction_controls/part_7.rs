pub fn source_ml_source1_temporary_test_java_script_semantic_text_equal_loop2(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut at: f64) -> self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f836) => {
                        {
                            let source_ml_source2_f860 = source_ml_source2_f836;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f837) => {
                        {
                            let source_ml_source2_f869 = source_ml_source2_f837;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f838) => {
                        {
                            let source_ml_source2_f878 = source_ml_source2_f838.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Text(units) => {
                        ({
                            let source_ml_source2_array_read_value884 = units.clone();
                            {
                                let source_ml_source2_array_read_index885 = at;
                                { let ml_reference_array150 = source_ml_source2_array_read_value884.clone(); let ml_reference_position150 = source_ml_source2_array_read_index885; let ml_reference_projection150 = self::ml_array::optional_at(&*ml_reference_array150.borrow(), self::ml_array::number_index(ml_reference_position150)); ml_reference_projection150 }
                            }
                        }).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f839, source_ml_source2_f840) => {
                        {
                            let source_ml_source2_f890 = source_ml_source2_f839.clone();
                            {
                                let text = source_ml_source2_f840.clone();
                                {
                                    let source_ml_source2_string_unit_operand894 = text.clone();
                                    {
                                        let source_ml_source2_string_unit_operand895 = at;
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
                                                    let source_ml_source2_string_unit_operand901 = {
                                                        let source_ml_source2_string_range_operand896 = text_2.clone();
                                                        {
                                                            let source_ml_source2_string_range_operand897 = index;
                                                            {
                                                                let source_ml_source2_string_range_operand898 = (index + 1f64);
                                                                {
                                                                    let source_ml_source2_string_range_position899 = source_ml_source2_string_range_operand897;
                                                                    {
                                                                        let source_ml_source2_string_range_position900 = source_ml_source2_string_range_operand898;
                                                                        source_ml_source2_string_range_operand896.slice(source_ml_source2_string_range_position899, source_ml_source2_string_range_position900)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_string_unit_operand902 = 0f64;
                                                        source_ml_source2_string_unit_operand901.code_point_at(source_ml_source2_string_unit_operand902)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source2_string_unit_operand894.clone(), source_ml_source2_string_unit_operand895)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f841) => {
                        {
                            let source_ml_source2_f908 = source_ml_source2_f841.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f842, source_ml_source2_f843) => {
                        {
                            let source_ml_source2_f917 = source_ml_source2_f842.clone();
                            {
                                let source_ml_source2_f918 = source_ml_source2_f843;
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f919) => {
                            {
                                let source_ml_source2_f943 = source_ml_source2_f919;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f920) => {
                            {
                                let source_ml_source2_f952 = source_ml_source2_f920;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f921) => {
                            {
                                let source_ml_source2_f961 = source_ml_source2_f921.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Text(units_2) => {
                            ({
                                let source_ml_source2_array_read_value967 = units_2.clone();
                                {
                                    let source_ml_source2_array_read_index968 = at;
                                    { let ml_reference_array151 = source_ml_source2_array_read_value967.clone(); let ml_reference_position151 = source_ml_source2_array_read_index968; let ml_reference_projection151 = self::ml_array::optional_at(&*ml_reference_array151.borrow(), self::ml_array::number_index(ml_reference_position151)); ml_reference_projection151 }
                                }
                            }).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f922, source_ml_source2_f923) => {
                            {
                                let source_ml_source2_f973 = source_ml_source2_f922.clone();
                                {
                                    let text_2 = source_ml_source2_f923.clone();
                                    {
                                        let source_ml_source2_string_unit_operand977 = text_2.clone();
                                        {
                                            let source_ml_source2_string_unit_operand978 = at;
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
                                                        let source_ml_source2_string_unit_operand984 = {
                                                            let source_ml_source2_string_range_operand979 = text_3.clone();
                                                            {
                                                                let source_ml_source2_string_range_operand980 = index;
                                                                {
                                                                    let source_ml_source2_string_range_operand981 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source2_string_range_position982 = source_ml_source2_string_range_operand980;
                                                                        {
                                                                            let source_ml_source2_string_range_position983 = source_ml_source2_string_range_operand981;
                                                                            source_ml_source2_string_range_operand979.slice(source_ml_source2_string_range_position982, source_ml_source2_string_range_position983)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_string_unit_operand985 = 0f64;
                                                            source_ml_source2_string_unit_operand984.code_point_at(source_ml_source2_string_unit_operand985)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source2_string_unit_operand977.clone(), source_ml_source2_string_unit_operand978)
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f924) => {
                            {
                                let source_ml_source2_f991 = source_ml_source2_f924.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f925, source_ml_source2_f926) => {
                            {
                                let source_ml_source2_f1000 = source_ml_source2_f925.clone();
                                {
                                    let source_ml_source2_f1001 = source_ml_source2_f926;
                                    (-1f64)
                                }
                            }
                        }
                    };
                    if (a != b) {
                        self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Return(false)
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
            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_semantic_text_string_loop4(mut text: self::ml_string::Text, mut source_ml_source1_temporary_values3: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index3: self::ml::Big) -> self::ml_string::Text {
    loop {
        return if (source_ml_source1_temporary_index3 < ({ let ml_reference_array152 = source_ml_source1_temporary_values3.clone(); let ml_reference_projection152 = self::ml::Big::from_u128(ml_reference_array152.borrow().len() as u128); ml_reference_projection152 })) {
            {
                let unit = {
                    let source_ml_source2_array_read_value1002 = source_ml_source1_temporary_values3.clone();
                    {
                        let source_ml_source2_array_read_index1003 = source_ml_source1_temporary_index3.clone();
                        { let ml_reference_array153 = source_ml_source2_array_read_value1002.clone(); let ml_reference_position153 = source_ml_source2_array_read_index1003.clone(); let ml_reference_projection153 = self::ml_array::at(&*ml_reference_array153.borrow(), ml_reference_position153.to_index()); ml_reference_projection153 }
                    }
                };
                {
                    let text_2 = text.concat(&{
                        let source_ml_source2_character_argument1004 = unit;
                        self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source2_character_argument1004]).into_iter().map(|ml_unit| ml_unit as u16).collect())
                    });
                    {
                        let source_ml_source1_temporary_index3_2 = source_ml_source1_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            (text, source_ml_source1_temporary_index3) = (text_2.clone(), source_ml_source1_temporary_index3_2.clone());
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

pub fn source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(mut value: self::JavaScriptSemanticValue, mut length: f64, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < length) {
            {
                let units_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value154 = units.borrow().clone(); ml_reference_value154 }, vec![self::read_java_script_semantic_text_unit(value.clone(), at)])));
                {
                    let at_2 = (at + 1f64);
                    {
                        (units, at) = (units_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            units.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done,
    MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(f64),
}

pub fn source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop6(mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    loop {
        return if (at < ({ let ml_reference_array155 = properties.clone(); let ml_reference_projection155 = ml_reference_array155.borrow().len() as f64; ml_reference_projection155 })) {
            {
                let property = {
                    let source_ml_source2_array_read_value1005 = properties.clone();
                    {
                        let source_ml_source2_array_read_index1006 = at;
                        { let ml_reference_array156 = source_ml_source2_array_read_value1005.clone(); let ml_reference_position156 = source_ml_source2_array_read_index1006; let ml_reference_projection156 = self::ml_array::optional_at(&*ml_reference_array156.borrow(), self::ml_array::number_index(ml_reference_position156)); ml_reference_projection156 }
                    }
                };
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value(match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value1007, _) => {
                        let source_ml_source2_data_property_value1007 = *source_ml_source2_data_property_value1007;
                        source_ml_source2_data_property_value1007.clone()
                    }
                }, key.clone())) {
                    self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(at)
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
            self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7 {
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Next,
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin7Return(std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result {
    MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Done,
    MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(f64),
}

pub fn source_ml_source1_temporary_find_java_script_heap_directory_object_loop8(mut objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = {
                        let source_ml_source2_array_read_value1008 = objects.clone();
                        {
                            let source_ml_source2_array_read_index1009 = middle;
                            { let ml_reference_array157 = source_ml_source2_array_read_value1008.clone(); let ml_reference_position157 = source_ml_source2_array_read_index1009; let ml_reference_projection157 = self::ml_array::optional_at(&*ml_reference_array157.borrow(), self::ml_array::number_index(ml_reference_position157)); ml_reference_projection157 }
                        }
                    };
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value1023, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1023.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value1024, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1024.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value1025, _, _, _, _) => {
                                source_ml_source2_data_property_value1025.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value1026, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1026.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value1027, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1027.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value1028, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1028.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value1029, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1029.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value1030, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1030.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value1031, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1031.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value1032, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1032.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value1033, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1033.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value1034, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1034.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value1035, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value1035.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value1049, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1049.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value1050, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1050.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value1051, _, _, _, _) => {
                                    source_ml_source2_data_property_value1051.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value1052, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1052.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value1053, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1053.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value1054, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1054.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value1055, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1055.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value1056, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1056.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value1057, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1057.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value1058, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1058.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value1059, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1059.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value1060, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1060.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value1061, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1061.clone()
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
            self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result {
    MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Done,
    MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(bool),
}

pub fn source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop9(mut symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = {
                        let source_ml_source2_array_read_value1062 = symbols.clone();
                        {
                            let source_ml_source2_array_read_index1063 = middle;
                            { let ml_reference_array158 = source_ml_source2_array_read_value1062.clone(); let ml_reference_position158 = source_ml_source2_array_read_index1063; let ml_reference_projection158 = self::ml_array::optional_at(&*ml_reference_array158.borrow(), self::ml_array::number_index(ml_reference_position158)); ml_reference_projection158 }
                        }
                    };
                    if {
                        let source_ml_source2_comparison_operand1064 = symbol.clone();
                        {
                            let source_ml_source2_comparison_operand1065 = None::<()>;
                            match (source_ml_source2_comparison_operand1064.clone(), source_ml_source2_comparison_operand1065.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                        }
                    } {
                        self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(true)
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
            self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result {
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Done,
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop13(mut heap: self::JavaScriptOrdinaryHeap, mut before: self::JavaScriptErrorAllocationDirectory, mut previous: self::ml::Big, mut source_ml_source1_temporary_values11: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut source_ml_source1_temporary_index11: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result {
    loop {
        return if (source_ml_source1_temporary_index11 < ({ let ml_reference_array159 = source_ml_source1_temporary_values11.clone(); let ml_reference_projection159 = self::ml::Big::from_u128(ml_reference_array159.borrow().len() as u128); ml_reference_projection159 })) {
            {
                let symbol = {
                    let source_ml_source2_array_read_value1066 = source_ml_source1_temporary_values11.clone();
                    {
                        let source_ml_source2_array_read_index1067 = source_ml_source1_temporary_index11.clone();
                        { let ml_reference_array160 = source_ml_source2_array_read_value1066.clone(); let ml_reference_position160 = source_ml_source2_array_read_index1067.clone(); let ml_reference_projection160 = self::ml_array::at(&*ml_reference_array160.borrow(), ml_reference_position160.to_index()); ml_reference_projection160 }
                    }
                };
                if (((symbol <= previous) || (symbol > (self::ml::Big::from_i128(18446744073709551615)))) || { let ml_values161 = { let ml_reference_value162 = (match before.clone() {
                    self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(source_ml_source2_data_property_value1068, _) => {
                        source_ml_source2_data_property_value1068.clone()
                    }
                }).borrow().clone(); ml_reference_value162 }; let ml_callback161 = { let symbol = symbol.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorAllocationIdentity| -> bool { ((match row.clone() {
                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value1070, _, _) => {
                        source_ml_source2_data_property_value1070.clone()
                    }
                }) == symbol) }) }; ml_values161.iter().cloned().enumerate().any(|(ml_index161, ml_value161)| ml_callback161(ml_value161)) }) {
                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(self::ml_string::Text::from_units(vec![112, 114, 105, 111, 114, 32, 83, 121, 109, 98, 111, 108, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = symbol.clone();
                        if !(self::ml_array::includes(&{ let ml_reference_value163 = (match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value1072, _, _, _, _) => {
                                source_ml_source2_data_property_value1072.clone()
                            }
                        }).borrow().clone(); ml_reference_value163 }, &symbol.clone(), 0f64, false, |a, b| a == b)) {
                            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                        } else {
                            {
                                let source_ml_source1_temporary_index11_2 = source_ml_source1_temporary_index11.add(&self::ml::Big::from_i128(1));
                                {
                                    (previous, source_ml_source1_temporary_index11) = (previous_2.clone(), source_ml_source1_temporary_index11_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result {
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop12(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_source1_temporary_values10: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorAllocationIdentity>>>, mut source_ml_source1_temporary_index10: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result {
    loop {
        return if (source_ml_source1_temporary_index10 < ({ let ml_reference_array164 = source_ml_source1_temporary_values10.clone(); let ml_reference_projection164 = self::ml::Big::from_u128(ml_reference_array164.borrow().len() as u128); ml_reference_projection164 })) {
            {
                let row = {
                    let source_ml_source2_array_read_value1073 = source_ml_source1_temporary_values10.clone();
                    {
                        let source_ml_source2_array_read_index1074 = source_ml_source1_temporary_index10.clone();
                        { let ml_reference_array165 = source_ml_source2_array_read_value1073.clone(); let ml_reference_position165 = source_ml_source2_array_read_index1074.clone(); let ml_reference_projection165 = self::ml_array::at(&*ml_reference_array165.borrow(), ml_reference_position165.to_index()); ml_reference_projection165 }
                    }
                };
                if (((match row.clone() {
                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value1076, _, _) => {
                        source_ml_source2_data_property_value1076.clone()
                    }
                }) <= previous) || ((match row.clone() {
                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value1078, _, _) => {
                        source_ml_source2_data_property_value1078.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(self::ml_string::Text::from_units(vec![112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 115, 116, 114, 105, 99, 116, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 111, 114, 100, 101, 114]))
                } else {
                    {
                        let previous_2 = match row.clone() {
                            self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value1079, _, _) => {
                                source_ml_source2_data_property_value1079.clone()
                            }
                        };
                        {
                            let object = {
                                let source_ml_source2_array_read_value1083 = match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value1080, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value1080.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_array_read_index1084 = self::find_java_script_heap_directory_object(match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value1081, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value1081.clone()
                                        }
                                    }, match row.clone() {
                                        self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value1082, _, _) => {
                                            source_ml_source2_data_property_value1082.clone()
                                        }
                                    });
                                    { let ml_reference_array166 = source_ml_source2_array_read_value1083.clone(); let ml_reference_position166 = source_ml_source2_array_read_index1084; let ml_reference_projection166 = self::ml_array::optional_at(&*ml_reference_array166.borrow(), self::ml_array::number_index(ml_reference_position166)); ml_reference_projection166 }
                                }
                            };
                            if ((match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                }
                            }) != (match row.clone() {
                                self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(_, source_ml_source2_data_property_value1086, _) => {
                                    source_ml_source2_data_property_value1086.clone()
                                }
                            }))) || ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value1101, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1101
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value1102, _, _, _, _) => {
                                    source_ml_source2_data_property_value1102
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value1103, _) => {
                                    source_ml_source2_data_property_value1103
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value1104, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1104
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value1105, _, _) => {
                                    source_ml_source2_data_property_value1105
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value1106, _, _) => {
                                    source_ml_source2_data_property_value1106
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value1107, _, _) => {
                                    source_ml_source2_data_property_value1107
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value1108, _, _) => {
                                    source_ml_source2_data_property_value1108
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value1109, _) => {
                                    source_ml_source2_data_property_value1109
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value1110, _, _, _) => {
                                    source_ml_source2_data_property_value1110
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value1111, _, _, _) => {
                                    source_ml_source2_data_property_value1111
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value1112, _, _, _) => {
                                    source_ml_source2_data_property_value1112
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value1113, _, _, _) => {
                                    source_ml_source2_data_property_value1113
                                }
                            }) != (match row.clone() {
                                self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(_, _, source_ml_source2_data_property_value1114) => {
                                    source_ml_source2_data_property_value1114
                                }
                            }))) {
                                self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                            } else {
                                {
                                    let source_ml_source1_temporary_index10_2 = source_ml_source1_temporary_index10.add(&self::ml::Big::from_i128(1));
                                    {
                                        (previous, source_ml_source1_temporary_index10) = (previous_2.clone(), source_ml_source1_temporary_index10_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result {
    MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Done,
    MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_error_construction_operands_error_loop15(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values14: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut source_ml_source1_temporary_index14: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result {
    loop {
        return if (source_ml_source1_temporary_index14 < ({ let ml_reference_array167 = source_ml_source1_temporary_values14.clone(); let ml_reference_projection167 = self::ml::Big::from_u128(ml_reference_array167.borrow().len() as u128); ml_reference_projection167 })) {
            {
                let value = {
                    let source_ml_source2_array_read_value1115 = source_ml_source1_temporary_values14.clone();
                    {
                        let source_ml_source2_array_read_index1116 = source_ml_source1_temporary_index14.clone();
                        { let ml_reference_array168 = source_ml_source2_array_read_value1115.clone(); let ml_reference_position168 = source_ml_source2_array_read_index1116.clone(); let ml_reference_projection168 = self::ml_array::at(&*ml_reference_array168.borrow(), ml_reference_position168.to_index()); ml_reference_projection168 }
                    }
                };
                {
                    let error = self::read_java_script_heap_directory_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index14_2 = source_ml_source1_temporary_index14.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index14 = source_ml_source1_temporary_index14_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Done
        };
    }
}
