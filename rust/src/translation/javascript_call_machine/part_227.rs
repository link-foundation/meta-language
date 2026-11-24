// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result;
use super::accept_java_script_whitespace_unit;
use super::find_java_script_heap_directory_object;
use super::find_java_script_heap_directory_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::source_ml_source3_record_order;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_indexed_property_presence_loop138(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut key: self::JavaScriptSemanticValue,
    mut current: self::ml::Big,
    mut depth: f64,
) -> self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                shared_constructor_field1,
                _,
                _,
                _,
                _,
                _,
            ) => {
                if (depth
                    < ({
                        let ml_reference_array2660 = shared_constructor_field1.clone();
                        let ml_reference_projection2660 =
                            ml_reference_array2660.borrow().len() as f64;
                        ml_reference_projection2660
                    }))
                {
                    {
                        let object = {
                            let source_ml_source2_array_read_value39892 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_array_read_index39893 =
                                    self::find_java_script_heap_directory_object(
                                        shared_constructor_field1.clone(),
                                        current.clone(),
                                    );
                                {
                                    let ml_reference_array2661 =
                                        source_ml_source2_array_read_value39892.clone();
                                    let ml_reference_position2661 =
                                        source_ml_source2_array_read_index39893;
                                    let ml_reference_projection2661 = self::ml_array::optional_at(
                                        &*ml_reference_array2661.borrow(),
                                        self::ml_array::number_index(ml_reference_position2661),
                                    );
                                    ml_reference_projection2661
                                }
                            }
                        };
                        if match (object.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source3_record_field506 = self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 111, 107, 117, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 79, 98, 106, 101, 99, 116]);
                                {
                                    let source_ml_source3_record_field507 = false;
                                    ((source_ml_source3_record_field506.clone(), source_ml_source3_record_field507), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                }
                            })))
                        } else {
                            match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(_, object_prototype, _, _, properties, _, _, _, _, _) => {
                                    let object_prototype = *object_prototype;
                                    if ((self::find_java_script_heap_directory_property(properties.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source3_record_field508 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source3_record_field509 = true;
                                                ((source_ml_source3_record_field508.clone(), source_ml_source3_record_field509), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                            }
                                        })))
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
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source3_record_field510 = self::ml_string::Text::from_units(vec![]);
                                                    {
                                                        let source_ml_source3_record_field511 = false;
                                                        ((source_ml_source3_record_field510.clone(), source_ml_source3_record_field511), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                                    }
                                                })))
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, object_prototype_2, _, _, properties_2, _, _, _) => {
                                    let object_prototype_2 = *object_prototype_2;
                                    if ((self::find_java_script_heap_directory_property(properties_2.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source3_record_field512 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source3_record_field513 = true;
                                                ((source_ml_source3_record_field512.clone(), source_ml_source3_record_field513), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                            }
                                        })))
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
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source3_record_field514 = self::ml_string::Text::from_units(vec![]);
                                                    {
                                                        let source_ml_source3_record_field515 = false;
                                                        ((source_ml_source3_record_field514.clone(), source_ml_source3_record_field515), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                                    }
                                                })))
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(_, object_prototype_3, _, _, properties_3) => {
                                    let object_prototype_3 = *object_prototype_3;
                                    if ((self::find_java_script_heap_directory_property(properties_3.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source3_record_field516 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source3_record_field517 = true;
                                                ((source_ml_source3_record_field516.clone(), source_ml_source3_record_field517), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                            }
                                        })))
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
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source3_record_field518 = self::ml_string::Text::from_units(vec![]);
                                                    {
                                                        let source_ml_source3_record_field519 = false;
                                                        ((source_ml_source3_record_field518.clone(), source_ml_source3_record_field519), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                                    }
                                                })))
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, object_prototype_4, _, _, properties_4, _, _, _, _, _, _) => {
                                    let object_prototype_4 = *object_prototype_4;
                                    if ((self::find_java_script_heap_directory_property(properties_4.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source3_record_field520 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source3_record_field521 = true;
                                                ((source_ml_source3_record_field520.clone(), source_ml_source3_record_field521), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                            }
                                        })))
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
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source3_record_field522 = self::ml_string::Text::from_units(vec![]);
                                                    {
                                                        let source_ml_source3_record_field523 = false;
                                                        ((source_ml_source3_record_field522.clone(), source_ml_source3_record_field523), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                                    }
                                                })))
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptOrdinaryObject::MapObject(_, object_prototype_5, _, _, properties_5, _) => {
                                    let object_prototype_5 = *object_prototype_5;
                                    if ((self::find_java_script_heap_directory_property(properties_5.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source3_record_field524 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source3_record_field525 = true;
                                                ((source_ml_source3_record_field524.clone(), source_ml_source3_record_field525), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                            }
                                        })))
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
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source3_record_field526 = self::ml_string::Text::from_units(vec![]);
                                                    {
                                                        let source_ml_source3_record_field527 = false;
                                                        ((source_ml_source3_record_field526.clone(), source_ml_source3_record_field527), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                                    }
                                                })))
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptOrdinaryObject::SetObject(_, object_prototype_6, _, _, properties_6, _) => {
                                    let object_prototype_6 = *object_prototype_6;
                                    if ((self::find_java_script_heap_directory_property(properties_6.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source3_record_field528 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source3_record_field529 = true;
                                                ((source_ml_source3_record_field528.clone(), source_ml_source3_record_field529), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                            }
                                        })))
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
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source3_record_field530 = self::ml_string::Text::from_units(vec![]);
                                                    {
                                                        let source_ml_source3_record_field531 = false;
                                                        ((source_ml_source3_record_field530.clone(), source_ml_source3_record_field531), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                                    }
                                                })))
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, object_prototype_7, _, _, _, _, _, _, properties_7) => {
                                    let object_prototype_7 = *object_prototype_7;
                                    if ((self::find_java_script_heap_directory_property(properties_7.clone(), key.clone())) >= (0f64)) {
                                        self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source3_record_field532 = self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source3_record_field533 = true;
                                                ((source_ml_source3_record_field532.clone(), source_ml_source3_record_field533), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                            }
                                        })))
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
                                                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source3_record_field534 = self::ml_string::Text::from_units(vec![]);
                                                    {
                                                        let source_ml_source3_record_field535 = false;
                                                        ((source_ml_source3_record_field534.clone(), source_ml_source3_record_field535), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                                    }
                                                })))
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source3_record_field536 = self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 72, 97, 115, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 97, 113, 117, 101, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108]);
                                        {
                                            let source_ml_source3_record_field537 = false;
                                            ((source_ml_source3_record_field536.clone(), source_ml_source3_record_field537), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![112f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![1f64, 0f64]))
                                        }
                                    })))
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Done
                }
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_decimal_power_loop146(
    mut exponent: f64,
    mut value: self::ml::Big,
    mut base: self::ml::Big,
) -> self::ml::Big {
    loop {
        return if (exponent > (0f64)) {
            {
                let value_3 = if ((exponent % 2f64) == (1f64)) {
                    {
                        let value_2 = value.mul(&base);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let exponent_2 = (exponent / 2f64).floor();
                    if (exponent_2 > (0f64)) {
                        {
                            let base_2 = base.mul(&base);
                            {
                                (exponent, value, base) =
                                    (exponent_2, value_3.clone(), base_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (exponent, value) = (exponent_2, value_3.clone());
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_exact_mantissa_loop147(
    mut value: self::ml::Big,
    mut result: f64,
    mut scale: f64,
) -> f64 {
    loop {
        return if (value > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(
                    &value,
                    &self::ml::Big::from_i128(2),
                    "trunc",
                    Some("Division by zero"),
                    true,
                )) == (self::ml::Big::from_i128(1)))
                {
                    {
                        let result_2 = (result + scale);
                        result_2
                    }
                } else {
                    result
                };
                {
                    let value_2 = self::ml::divide(
                        &value,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    );
                    {
                        let scale_2 = (scale * 2f64);
                        {
                            (value, result, scale) = (value_2.clone(), result_3, scale_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop149(
    mut exponent: f64,
    mut result: f64,
) -> f64 {
    loop {
        return if (exponent < (0f64)) {
            {
                let result_2 = (result / 2f64);
                {
                    let exponent_2 = (exponent + 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadStringNumberBinaryPowerLoop148Result {
    MlSource1TemporaryReadStringNumberBinaryPowerLoop148Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop148(
    mut exponent: f64,
    mut result: f64,
) -> self::MlSource1TemporaryReadStringNumberBinaryPowerLoop148Result {
    loop {
        return if (exponent > (0f64)) {
            {
                let result_2 = (result * 2f64);
                {
                    let exponent_2 = (exponent - 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadStringNumberBinaryPowerLoop148Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop148Done(exponent, result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop153Result {
    MlSource1TemporaryConvertStringNumberRationalLoop153Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop153(
    mut quotient: self::ml::Big,
    mut remainder: self::ml::Big,
    mut scale: self::ml::Big,
    mut weight: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop153Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
            {
                let scale_2 = self::ml::divide(
                    &scale,
                    &self::ml::Big::from_i128(2),
                    "trunc",
                    Some("Division by zero"),
                    false,
                );
                {
                    let weight_2 = self::ml::divide(
                        &weight,
                        &self::ml::Big::from_i128(2),
                        "trunc",
                        Some("Division by zero"),
                        false,
                    );
                    if (remainder >= scale_2) {
                        {
                            let remainder_2 = remainder.sub(&scale_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (quotient, remainder, scale, weight) = (
                                        quotient_2.clone(),
                                        remainder_2.clone(),
                                        scale_2.clone(),
                                        weight_2.clone(),
                                    );
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            (scale, weight) = (scale_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop153Result::MlSource1TemporaryConvertStringNumberRationalLoop153Done(quotient.clone(), remainder.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop152Result {
    MlSource1TemporaryConvertStringNumberRationalLoop152Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop152(
    mut remainder: self::ml::Big,
    mut scale: self::ml::Big,
    mut weight: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop152Result {
    loop {
        return if (scale <= remainder) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                    {
                        (scale, weight) = (scale_2.clone(), weight_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop152Result::MlSource1TemporaryConvertStringNumberRationalLoop152Done(scale.clone(), weight.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin151 {
    MlSource1TemporaryConvertStringNumberRationalJoin151Next(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop154(
    mut numerator: self::ml::Big,
    mut shift: f64,
    mut index: f64,
) -> self::ml::Big {
    loop {
        return if (index < shift) {
            {
                let numerator_2 = numerator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (numerator, index) = (numerator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            numerator.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop155(
    mut denominator: self::ml::Big,
    mut shift: f64,
    mut index: f64,
) -> self::ml::Big {
    loop {
        return if (index < (-shift)) {
            {
                let denominator_2 = denominator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (denominator, index) = (denominator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            denominator.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin150 {
    MlSource1TemporaryConvertStringNumberRationalJoin150Next(f64),
    MlSource1TemporaryConvertStringNumberRationalJoin150Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop156Result {
    MlSource1TemporaryConvertStringNumberRationalLoop156Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop156Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop156(
    mut numerator: self::ml::Big,
    mut exponent: f64,
    mut scale: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop156Result {
    loop {
        return if (numerator >= (scale.mul(&self::ml::Big::from_i128(2)))) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent + 1f64);
                    if (exponent_2 >= (1024f64)) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop156Result::MlSource1TemporaryConvertStringNumberRationalLoop156Return(f64::INFINITY)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop156Result::MlSource1TemporaryConvertStringNumberRationalLoop156Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop157Result {
    MlSource1TemporaryConvertStringNumberRationalLoop157Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop157Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop157(
    mut denominator: self::ml::Big,
    mut exponent: f64,
    mut scale: self::ml::Big,
) -> self::MlSource1TemporaryConvertStringNumberRationalLoop157Result {
    loop {
        return if (scale < denominator) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent - 1f64);
                    if (exponent_2 < (-1075f64)) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop157Result::MlSource1TemporaryConvertStringNumberRationalLoop157Return(0f64)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop157Result::MlSource1TemporaryConvertStringNumberRationalLoop157Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptStringNumberInfinityLoop158Result {
    MlSource1TemporaryAcceptStringNumberInfinityLoop158Done,
    MlSource1TemporaryAcceptStringNumberInfinityLoop158Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_string_number_infinity_loop158(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut expected: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryAcceptStringNumberInfinityLoop158Result {
    loop {
        return if (index
            < ({
                let ml_reference_array2662 = expected.clone();
                let ml_reference_projection2662 = ml_reference_array2662.borrow().len() as f64;
                ml_reference_projection2662
            })) {
            if ((({
                let source_ml_source2_array_read_index39987 = (start + index);
                {
                    let ml_reference_array2663 = units.clone();
                    let ml_reference_position2663 = source_ml_source2_array_read_index39987;
                    let ml_reference_projection2663 = self::ml_array::optional_at(
                        &*ml_reference_array2663.borrow(),
                        self::ml_array::number_index(ml_reference_position2663),
                    );
                    ml_reference_projection2663
                }
            })
            .clone()
            .unwrap_or_else(|| (-1f64)))
                != (({
                    let ml_reference_array2664 = expected.clone();
                    let ml_reference_position2664 = index;
                    let ml_reference_projection2664 = self::ml_array::optional_at(
                        &*ml_reference_array2664.borrow(),
                        self::ml_array::number_index(ml_reference_position2664),
                    );
                    ml_reference_projection2664
                })
                .clone()
                .unwrap_or_else(|| (-2f64))))
            {
                self::MlSource1TemporaryAcceptStringNumberInfinityLoop158Result::MlSource1TemporaryAcceptStringNumberInfinityLoop158Return(false)
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptStringNumberInfinityLoop158Result::MlSource1TemporaryAcceptStringNumberInfinityLoop158Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop160(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        return if ((start < end)
            && self::accept_java_script_whitespace_unit(
                ({
                    let ml_reference_array2665 = units.clone();
                    let ml_reference_position2665 = start;
                    let ml_reference_projection2665 = self::ml_array::optional_at(
                        &*ml_reference_array2665.borrow(),
                        self::ml_array::number_index(ml_reference_position2665),
                    );
                    ml_reference_projection2665
                })
                .clone()
                .unwrap_or_else(|| (-1f64)),
            )) {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop161(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut start: f64,
    mut end: f64,
) -> f64 {
    loop {
        return if ((end > start)
            && self::accept_java_script_whitespace_unit(
                ({
                    let source_ml_source2_array_read_index39993 = (end - 1f64);
                    {
                        let ml_reference_array2666 = units.clone();
                        let ml_reference_position2666 = source_ml_source2_array_read_index39993;
                        let ml_reference_projection2666 = self::ml_array::optional_at(
                            &*ml_reference_array2666.borrow(),
                            self::ml_array::number_index(ml_reference_position2666),
                        );
                        ml_reference_projection2666
                    }
                })
                .clone()
                .unwrap_or_else(|| (-1f64)),
            )) {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop167(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut integer_digits: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut mantissa_end: f64,
    mut mantissa: self::ml::Big,
    mut index: f64,
) -> self::ml::Big {
    loop {
        return if (index < mantissa_end) {
            {
                let unit = ({
                    let ml_reference_array2667 = units.clone();
                    let ml_reference_position2667 = index;
                    let ml_reference_projection2667 = self::ml_array::optional_at(
                        &*ml_reference_array2667.borrow(),
                        self::ml_array::number_index(ml_reference_position2667),
                    );
                    ml_reference_projection2667
                })
                .clone()
                .unwrap_or_else(|| (-1f64));
                if (unit != (46f64)) {
                    {
                        let mantissa_2 = (mantissa.mul(&self::ml::Big::from_i128(10))).add(
                            &({
                                let source_ml_source2_array_read_index39997 = (unit - 48f64);
                                {
                                    let ml_reference_array2668 = integer_digits.clone();
                                    let ml_reference_position2668 =
                                        source_ml_source2_array_read_index39997;
                                    let ml_reference_projection2668 = self::ml_array::optional_at(
                                        &*ml_reference_array2668.borrow(),
                                        self::ml_array::number_index(ml_reference_position2668),
                                    );
                                    ml_reference_projection2668
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                        );
                        {
                            let index_2 = (index + 1f64);
                            {
                                (mantissa, index) = (mantissa_2.clone(), index_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let index_3 = (index + 1f64);
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            mantissa.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop166Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop166Done(f64, f64),
}
