pub fn source_ml_source1_temporary_read_java_script_indexed_property_presence_loop106(mut heap: self::JavaScriptOrdinaryHeap, mut key: self::JavaScriptSemanticValue, mut current: self::ml::Big, mut depth: f64) -> self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
                if (depth < ({ let ml_reference_array587 = shared_constructor_field1.clone(); let ml_reference_projection587 = ml_reference_array587.borrow().len() as f64; ml_reference_projection587 })) {
                    {
                        let object = {
                            let source_ml_source2_array_read_value3581 = shared_constructor_field1.clone();
                            {
                                let source_ml_source2_array_read_index3582 = self::find_java_script_heap_directory_object(shared_constructor_field1.clone(), current.clone());
                                { let ml_reference_array588 = source_ml_source2_array_read_value3581.clone(); let ml_reference_position588 = source_ml_source2_array_read_index3582; let ml_reference_projection588 = self::ml_array::optional_at(&*ml_reference_array588.borrow(), self::ml_array::number_index(ml_reference_position588)); ml_reference_projection588 }
                            }
                        };
                        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 111, 107, 117, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 79, 98, 106, 101, 99, 116]), false))))
                        } else {
                            match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, object_prototype, _, _, properties, _, _, _, _, _) => {
                                    let object_prototype = *object_prototype;
                                    if ((self::find_java_script_heap_directory_property(properties.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), true))))
                                    } else {
                                        match object_prototype.clone() {
                                            self::JavaScriptSemanticValue::Object(current_2, _) => {
                                                {
                                                    let depth_2 = (depth + 1f64);
                                                    {
                                                        (current, depth) = (current_2.clone(), depth_2);
                                                        continue;
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), false))))
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, object_prototype_2, _, _, properties_2, _, _, _) => {
                                    let object_prototype_2 = *object_prototype_2;
                                    if ((self::find_java_script_heap_directory_property(properties_2.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), true))))
                                    } else {
                                        match object_prototype_2.clone() {
                                            self::JavaScriptSemanticValue::Object(current_3, _) => {
                                                {
                                                    let depth_3 = (depth + 1f64);
                                                    {
                                                        (current, depth) = (current_3.clone(), depth_3);
                                                        continue;
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), false))))
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, object_prototype_3, _, _, properties_3) => {
                                    let object_prototype_3 = *object_prototype_3;
                                    if ((self::find_java_script_heap_directory_property(properties_3.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), true))))
                                    } else {
                                        match object_prototype_3.clone() {
                                            self::JavaScriptSemanticValue::Object(current_4, _) => {
                                                {
                                                    let depth_4 = (depth + 1f64);
                                                    {
                                                        (current, depth) = (current_4.clone(), depth_4);
                                                        continue;
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), false))))
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, object_prototype_4, _, _, properties_4, _, _, _, _, _, _) => {
                                    let object_prototype_4 = *object_prototype_4;
                                    if ((self::find_java_script_heap_directory_property(properties_4.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), true))))
                                    } else {
                                        match object_prototype_4.clone() {
                                            self::JavaScriptSemanticValue::Object(current_5, _) => {
                                                {
                                                    let depth_5 = (depth + 1f64);
                                                    {
                                                        (current, depth) = (current_5.clone(), depth_5);
                                                        continue;
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), false))))
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, object_prototype_5, _, _, properties_5, _) => {
                                    let object_prototype_5 = *object_prototype_5;
                                    if ((self::find_java_script_heap_directory_property(properties_5.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), true))))
                                    } else {
                                        match object_prototype_5.clone() {
                                            self::JavaScriptSemanticValue::Object(current_6, _) => {
                                                {
                                                    let depth_6 = (depth + 1f64);
                                                    {
                                                        (current, depth) = (current_6.clone(), depth_6);
                                                        continue;
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), false))))
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, object_prototype_6, _, _, properties_6, _) => {
                                    let object_prototype_6 = *object_prototype_6;
                                    if ((self::find_java_script_heap_directory_property(properties_6.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), true))))
                                    } else {
                                        match object_prototype_6.clone() {
                                            self::JavaScriptSemanticValue::Object(current_7, _) => {
                                                {
                                                    let depth_7 = (depth + 1f64);
                                                    {
                                                        (current, depth) = (current_7.clone(), depth_7);
                                                        continue;
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), false))))
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, object_prototype_7, _, _, _, _, _, _, properties_7) => {
                                    let object_prototype_7 = *object_prototype_7;
                                    if ((self::find_java_script_heap_directory_property(properties_7.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), true))))
                                    } else {
                                        match object_prototype_7.clone() {
                                            self::JavaScriptSemanticValue::Object(current_8, _) => {
                                                {
                                                    let depth_8 = (depth + 1f64);
                                                    {
                                                        (current, depth) = (current_8.clone(), depth_8);
                                                        continue;
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![]), false))))
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 72, 97, 115, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 97, 113, 117, 101, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]), false))))
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadPrimitiveBitwiseDomainLoop108Result {
    MlSource1TemporaryReadPrimitiveBitwiseDomainLoop108Done(bool, bool),
}

pub fn source_ml_source1_temporary_read_primitive_bitwise_domain_loop108(mut number: bool, mut integer: bool, mut source_ml_source1_temporary_values107: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_index107: self::ml::Big) -> self::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop108Result {
    loop {
        return if (source_ml_source1_temporary_index107 < ({ let ml_reference_array589 = source_ml_source1_temporary_values107.clone(); let ml_reference_projection589 = self::ml::Big::from_u128(ml_reference_array589.borrow().len() as u128); ml_reference_projection589 })) {
            {
                let kind = { let ml_reference_array590 = source_ml_source1_temporary_values107.clone(); let ml_reference_position590 = source_ml_source1_temporary_index107.clone(); let ml_reference_projection590 = self::ml_array::at(&*ml_reference_array590.borrow(), ml_reference_position590.to_index()); ml_reference_projection590 };
                {
                    let number_2 = (number && (kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))));
                    {
                        let integer_2 = (integer && ((kind == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116])))));
                        {
                            let source_ml_source1_temporary_index107_2 = source_ml_source1_temporary_index107.add(&self::ml::Big::from_i128(1));
                            {
                                (number, integer, source_ml_source1_temporary_index107) = (number_2, integer_2, source_ml_source1_temporary_index107_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop108Result::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop108Done(number, integer)
        };
    }
}

pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop110(mut count: self::ml::Big, mut value: self::ml::Big, mut factor: self::ml::Big) -> self::ml::Big {
    loop {
        return if (count > (self::ml::Big::from_i128(0))) {
            {
                let value_3 = if ((self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let value_2 = value.mul(&factor);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let count_2 = self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (count_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let factor_2 = factor.mul(&factor);
                            {
                                (count, value, factor) = (count_2.clone(), value_3.clone(), factor_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (count, value) = (count_2.clone(), value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop111(mut count: self::ml::Big, mut value: self::ml::Big) -> self::ml::Big {
    loop {
        return if (((count > (self::ml::Big::from_i128(0))) && (value != (self::ml::Big::from_i128(0)))) && (value != (self::ml::Big::from_i128(-1)))) {
            {
                let value_2 = if (value < (self::ml::Big::from_i128(0))) {
                    self::ml::divide(&value.sub(&self::ml::Big::from_i128(1)), &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                } else {
                    self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                };
                {
                    let count_2 = count.sub(&self::ml::Big::from_i128(1));
                    {
                        (count, value) = (count_2.clone(), value_2.clone());
                        continue;
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin109 {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin109Next(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop112Result {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop112Done(self::ml::Big, self::ml::Big),
}

pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop112(mut operation: f64, mut negative_left: bool, mut negative_right: bool, mut first: self::ml::Big, mut second: self::ml::Big, mut weight: self::ml::Big, mut result: self::ml::Big) -> self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop112Result {
    loop {
        return if ((first > (self::ml::Big::from_i128(0))) || (second > (self::ml::Big::from_i128(0)))) {
            {
                let a = if negative_left {
                    (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                } else {
                    self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                };
                {
                    let b = if negative_right {
                        (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                    } else {
                        self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                    };
                    {
                        let selected = if (operation == (0f64)) {
                            ((a == (self::ml::Big::from_i128(1))) && (b == (self::ml::Big::from_i128(1))))
                        } else {
                            if (operation == (1f64)) {
                                ((a == (self::ml::Big::from_i128(1))) || (b == (self::ml::Big::from_i128(1))))
                            } else {
                                (a != b)
                            }
                        };
                        {
                            let result_3 = if selected {
                                {
                                    let result_2 = result.add(&weight);
                                    result_2.clone()
                                }
                            } else {
                                result.clone()
                            };
                            {
                                let first_2 = self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                {
                                    let second_2 = self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                    {
                                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                                        {
                                            (first, second, weight, result) = (first_2.clone(), second_2.clone(), weight_2.clone(), result_3.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop112Result::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop112Done(weight.clone(), result.clone())
        };
    }
}

pub fn source_ml_source1_temporary_render_big_integer_bitwise_expression_loop113(mut text: self::ml_string::Text, mut suffix: f64) -> f64 {
    loop {
        return if (text.contains(&(self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 95, 108, 101, 102, 116])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix)))) || text.contains(&(self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 95, 114, 105, 103, 104, 116])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix))))) {
            {
                let suffix_2 = (suffix + 1f64);
                {
                    suffix = suffix_2;
                    continue;
                }
            }
        } else {
            suffix
        };
    }
}

pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop115(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((start < end) && self::accept_java_script_whitespace_unit(({ let ml_reference_array591 = units.clone(); let ml_reference_position591 = start; let ml_reference_projection591 = self::ml_array::optional_at(&*ml_reference_array591.borrow(), self::ml_array::number_index(ml_reference_position591)); ml_reference_projection591 }).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let start_2 = (start + 1f64);
                {
                    start = start_2;
                    continue;
                }
            }
        } else {
            start
        };
    }
}

pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop116(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((end > start) && self::accept_java_script_whitespace_unit(({
            let source_ml_source2_array_read_index3865 = (end - 1f64);
            { let ml_reference_array592 = units.clone(); let ml_reference_position592 = source_ml_source2_array_read_index3865; let ml_reference_projection592 = self::ml_array::optional_at(&*ml_reference_array592.borrow(), self::ml_array::number_index(ml_reference_position592)); ml_reference_projection592 }
        }).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let end_2 = (end - 1f64);
                {
                    end = end_2;
                    continue;
                }
            }
        } else {
            end
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop119Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop119Done(self::ml::Big),
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop119Return(std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>),
}

pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop119(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut end: f64, mut radix: f64, mut base: self::ml::Big, mut digits: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut value: self::ml::Big, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop119Result {
    loop {
        return if (index < end) {
            {
                let digit = self::read_java_script_numeric_digit_unit(({ let ml_reference_array593 = units.clone(); let ml_reference_position593 = index; let ml_reference_projection593 = self::ml_array::optional_at(&*ml_reference_array593.borrow(), self::ml_array::number_index(ml_reference_position593)); ml_reference_projection593 }).clone().unwrap_or_else(|| (-1f64)));
                if ((digit < (0f64)) || (digit >= radix)) {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop119Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop119Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new())))
                } else {
                    {
                        let value_2 = (value.mul(&base)).add(&({ let ml_reference_array594 = digits.clone(); let ml_reference_position594 = digit; let ml_reference_projection594 = self::ml_array::optional_at(&*ml_reference_array594.borrow(), self::ml_array::number_index(ml_reference_position594)); ml_reference_projection594 }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (value, index) = (value_2.clone(), index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop119Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop119Done(value.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin118 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin118Next(f64, f64, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin117 {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsJoin117Next(f64, bool, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop114Result {
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop114Done,
    MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop114Return(std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>),
}

pub fn source_ml_source1_temporary_convert_java_script_string_big_integer_units_loop114(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop114Result {
    loop {
        return if (index < ({ let ml_reference_array595 = units.clone(); let ml_reference_projection595 = ml_reference_array595.borrow().len() as f64; ml_reference_projection595 })) {
            {
                let unit = ({ let ml_reference_array596 = units.clone(); let ml_reference_position596 = index; let ml_reference_projection596 = self::ml_array::optional_at(&*ml_reference_array596.borrow(), self::ml_array::number_index(ml_reference_position596)); ml_reference_projection596 }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop114Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop114Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new())))
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop114Result::MlSource1TemporaryConvertJavaScriptStringBigIntegerUnitsLoop114Done
        };
    }
}

pub fn source_ml_source1_temporary_convert_java_script_number_big_integer_loop120(mut value: f64, mut integer: self::ml::Big, mut weight: self::ml::Big) -> self::ml::Big {
    loop {
        return if (value >= (1f64)) {
            {
                let integer_3 = if (((value % 2f64)) == (1f64)) {
                    {
                        let integer_2 = integer.add(&weight);
                        integer_2.clone()
                    }
                } else {
                    integer.clone()
                };
                {
                    let value_2 = ((value / 2f64)).floor();
                    {
                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                        {
                            (value, integer, weight) = (value_2, integer_3.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            integer.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop122Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop122Done(self::ml::Big, self::ml::Big),
}

pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop122(mut remaining: self::ml::Big, mut shifted: self::ml::Big, mut weight: self::ml::Big, mut quotient: self::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop122Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
            {
                let shifted_2 = self::ml::divide(&shifted, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = self::ml::divide(&weight, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (shifted_2 <= remaining) {
                        {
                            let remaining_2 = remaining.sub(&shifted_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (remaining, shifted, weight, quotient) = (remaining_2.clone(), shifted_2.clone(), weight_2.clone(), quotient_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            (shifted, weight) = (shifted_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop122Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop122Done(remaining.clone(), quotient.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDivideJavaScriptBigIntegerLoop121Result {
    MlSource1TemporaryDivideJavaScriptBigIntegerLoop121Done(self::ml::Big, self::ml::Big),
}

pub fn source_ml_source1_temporary_divide_java_script_big_integer_loop121(mut remaining: self::ml::Big, mut shifted: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop121Result {
    loop {
        return if (shifted <= remaining) {
            {
                let shifted_2 = shifted.mul(&self::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                    {
                        (shifted, weight) = (shifted_2.clone(), weight_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDivideJavaScriptBigIntegerLoop121Result::MlSource1TemporaryDivideJavaScriptBigIntegerLoop121Done(shifted.clone(), weight.clone())
        };
    }
}

pub fn source_ml_source1_temporary_power_java_script_big_integer_loop123(mut result: self::ml::Big, mut weight: self::ml::Big, mut remaining: self::ml::Big) -> self::ml::Big {
    loop {
        return if (remaining > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(&remaining, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let result_2 = result.mul(&weight);
                        result_2.clone()
                    }
                } else {
                    result.clone()
                };
                {
                    let remaining_2 = self::ml::divide(&remaining, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remaining_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let weight_2 = weight.mul(&weight);
                            {
                                (result, weight, remaining) = (result_3.clone(), weight_2.clone(), remaining_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (result, remaining) = (result_3.clone(), remaining_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryNormalizeNumberPowerBoundLoop125Result {
    MlSource1TemporaryNormalizeNumberPowerBoundLoop125Done(self::ml::Big, self::ml::Big),
}

pub fn source_ml_source1_temporary_normalize_number_power_bound_loop125(mut value: self::ml::Big, mut exponent: self::ml::Big, mut scale: self::ml::Big) -> self::MlSource1TemporaryNormalizeNumberPowerBoundLoop125Result {
    loop {
        return if (value < scale) {
            {
                let value_2 = value.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = exponent.sub(&self::ml::Big::from_i128(1));
                    {
                        (value, exponent) = (value_2.clone(), exponent_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryNormalizeNumberPowerBoundLoop125Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop125Done(value.clone(), exponent.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryNormalizeNumberPowerBoundLoop124Result {
    MlSource1TemporaryNormalizeNumberPowerBoundLoop124Done(self::ml::Big, self::ml::Big),
}

pub fn source_ml_source1_temporary_normalize_number_power_bound_loop124(mut value: self::ml::Big, mut exponent: self::ml::Big, mut scale: self::ml::Big, mut upward: bool) -> self::MlSource1TemporaryNormalizeNumberPowerBoundLoop124Result {
    loop {
        return if (value >= (scale.mul(&self::ml::Big::from_i128(2)))) {
            {
                let value_2 = self::ml::divide(&value.add(&if upward {
                    self::ml::Big::from_i128(1)
                } else {
                    self::ml::Big::from_i128(0)
                }), &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let exponent_2 = exponent.add(&self::ml::Big::from_i128(1));
                    {
                        (value, exponent) = (value_2.clone(), exponent_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryNormalizeNumberPowerBoundLoop124Result::MlSource1TemporaryNormalizeNumberPowerBoundLoop124Done(value.clone(), exponent.clone())
        };
    }
}

pub fn source_ml_source1_temporary_root_number_power_integer_loop126(mut root: self::ml::Big, mut remaining: self::ml::Big) -> self::ml::Big {
    loop {
        return if (remaining > (self::ml::Big::from_i128(1))) {
            {
                let remaining_2 = self::ml::divide(&remaining, &self::ml::Big::from_i128(4), "trunc", Some("Division by zero"), false);
                {
                    let root_2 = root.mul(&self::ml::Big::from_i128(2));
                    {
                        (root, remaining) = (root_2.clone(), remaining_2.clone());
                        continue;
                    }
                }
            }
        } else {
            root.clone()
        };
    }
}

pub fn source_ml_source1_temporary_root_number_power_integer_loop127(mut value: self::ml::Big, mut root: self::ml::Big) -> self::ml::Big {
    loop {
        return {
            let next = self::ml::divide(&root.add(&self::divide_number_power_bound(value.clone(), root.clone(), false)), &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
            if (next >= root) {
                root.clone()
            } else {
                {
                    root = next.clone();
                    continue;
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryRootNumberPowerBoundJoin128 {
    MlSource1TemporaryRootNumberPowerBoundJoin128Next(self::ml::Big, self::ml::Big),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop130Result {
    MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop130Done(f64, self::ml::Big),
}

pub fn source_ml_source1_temporary_decompose_number_power_magnitude_loop130(mut value: f64, mut exponent: self::ml::Big) -> self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop130Result {
    loop {
        return if (value < (1f64)) {
            {
                let value_2 = (value * 2f64);
                {
                    let exponent_2 = exponent.sub(&self::ml::Big::from_i128(1));
                    {
                        (value, exponent) = (value_2, exponent_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop130Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop130Done(value, exponent.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop129Result {
    MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop129Done(f64, self::ml::Big),
}

pub fn source_ml_source1_temporary_decompose_number_power_magnitude_loop129(mut value: f64, mut exponent: self::ml::Big) -> self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop129Result {
    loop {
        return if (value >= (2f64)) {
            {
                let value_2 = (value / 2f64);
                {
                    let exponent_2 = exponent.add(&self::ml::Big::from_i128(1));
                    {
                        (value, exponent) = (value_2, exponent_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop129Result::MlSource1TemporaryDecomposeNumberPowerMagnitudeLoop129Done(value, exponent.clone())
        };
    }
}

pub fn source_ml_source1_temporary_power_java_script_number_loop133(mut scale: self::ml::Big, mut interval: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut shift: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    loop {
        return if (shift < (self::ml::Big::from_i128(0))) {
            {
                let interval_2 = self::root_number_power_interval(interval.clone(), scale.clone());
                {
                    let shift_2 = shift.add(&self::ml::Big::from_i128(1));
                    {
                        (interval, shift) = (interval_2.clone(), shift_2.clone());
                        continue;
                    }
                }
            }
        } else {
            interval.clone()
        };
    }
}

pub fn source_ml_source1_temporary_power_java_script_number_loop134(mut scale: self::ml::Big, mut interval: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut remaining: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>> {
    loop {
        return if (remaining > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(&remaining, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let result_2 = self::multiply_number_power_interval(result.clone(), interval.clone(), scale.clone());
                        result_2.clone()
                    }
                } else {
                    result.clone()
                };
                {
                    let remaining_2 = self::ml::divide(&remaining, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remaining_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let interval_2 = self::multiply_number_power_interval(interval.clone(), interval.clone(), scale.clone());
                            {
                                (interval, result, remaining) = (interval_2.clone(), result_3.clone(), remaining_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (result, remaining) = (result_3.clone(), remaining_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryPowerJavaScriptNumberLoop132Result {
    MlSource1TemporaryPowerJavaScriptNumberLoop132Done(std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, self::ml::Big),
}

pub fn source_ml_source1_temporary_power_java_script_number_loop132(mut scale: self::ml::Big, mut interval: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut shift: self::ml::Big) -> self::MlSource1TemporaryPowerJavaScriptNumberLoop132Result {
    loop {
        return if (shift > (self::ml::Big::from_i128(0))) {
            {
                let interval_2 = self::multiply_number_power_interval(interval.clone(), interval.clone(), scale.clone());
                {
                    let shift_2 = shift.sub(&self::ml::Big::from_i128(1));
                    {
                        (interval, shift) = (interval_2.clone(), shift_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPowerJavaScriptNumberLoop132Result::MlSource1TemporaryPowerJavaScriptNumberLoop132Done(interval.clone(), shift.clone())
        };
    }
}
