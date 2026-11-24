pub fn source_ml_source1_temporary_power_java_script_number_loop131(mut exponent: f64, mut negative: bool, mut input: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut power: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut precision: self::ml::Big) -> f64 {
    loop {
        return {
            let scale = ({
                let source_ml_source2_array_read_value3872 = self::power_java_script_big_integer(self::ml::Big::from_i128(2), precision.clone());
                {
                    let source_ml_source2_array_read_index3873 = 0f64;
                    { let ml_reference_array597 = source_ml_source2_array_read_value3872.clone(); let ml_reference_position597 = source_ml_source2_array_read_index3873; let ml_reference_projection597 = self::ml_array::optional_at(&*ml_reference_array597.borrow(), self::ml_array::number_index(ml_reference_position597)); ml_reference_projection597 }
                }
            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
            {
                let value = (({
                    let source_ml_source2_array_read_index3875 = 0f64;
                    { let ml_reference_array598 = input.clone(); let ml_reference_position598 = source_ml_source2_array_read_index3875; let ml_reference_projection598 = self::ml_array::optional_at(&*ml_reference_array598.borrow(), self::ml_array::number_index(ml_reference_position598)); ml_reference_projection598 }
                }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).mul(&({
                    let source_ml_source2_array_read_value3876 = self::power_java_script_big_integer(self::ml::Big::from_i128(2), precision.sub(&self::ml::Big::from_i128(52)));
                    {
                        let source_ml_source2_array_read_index3877 = 0f64;
                        { let ml_reference_array599 = source_ml_source2_array_read_value3876.clone(); let ml_reference_position599 = source_ml_source2_array_read_index3877; let ml_reference_projection599 = self::ml_array::optional_at(&*ml_reference_array599.borrow(), self::ml_array::number_index(ml_reference_position599)); ml_reference_projection599 }
                    }
                }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)));
                {
                    let initial_exponent = (({
                        let source_ml_source2_array_read_index3879 = 1f64;
                        { let ml_reference_array600 = input.clone(); let ml_reference_position600 = source_ml_source2_array_read_index3879; let ml_reference_projection600 = self::ml_array::optional_at(&*ml_reference_array600.borrow(), self::ml_array::number_index(ml_reference_position600)); ml_reference_projection600 }
                    }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))).add(&self::ml::Big::from_i128(52));
                    {
                        let interval = std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone(), initial_exponent.clone(), value.clone(), initial_exponent.clone()]));
                        {
                            let interval_3 = if (exponent < (0f64)) {
                                {
                                    let interval_2 = self::invert_number_power_interval(interval.clone(), scale.clone());
                                    interval_2.clone()
                                }
                            } else {
                                interval.clone()
                            };
                            {
                                let shift = ({
                                    let source_ml_source2_array_read_index3881 = 1f64;
                                    { let ml_reference_array601 = power.clone(); let ml_reference_position601 = source_ml_source2_array_read_index3881; let ml_reference_projection601 = self::ml_array::optional_at(&*ml_reference_array601.borrow(), self::ml_array::number_index(ml_reference_position601)); ml_reference_projection601 }
                                }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                {
                                    let source_ml_source2_s3882 = self::source_ml_source1_temporary_power_java_script_number_loop132(scale.clone(), interval_3.clone(), shift.clone());
                                    match source_ml_source2_s3882.clone() {
                                        self::MlSource1TemporaryPowerJavaScriptNumberLoop132Result::MlSource1TemporaryPowerJavaScriptNumberLoop132Done(interval_4, shift_2) => {
                                            {
                                                let interval_5 = self::source_ml_source1_temporary_power_java_script_number_loop133(scale.clone(), interval_4.clone(), shift_2.clone());
                                                {
                                                    let result = std::rc::Rc::new(std::cell::RefCell::new(vec![scale.clone(), self::ml::Big::from_i128(0), scale.clone(), self::ml::Big::from_i128(0)]));
                                                    {
                                                        let remaining = ({
                                                            let source_ml_source2_array_read_index3884 = 0f64;
                                                            { let ml_reference_array602 = power.clone(); let ml_reference_position602 = source_ml_source2_array_read_index3884; let ml_reference_projection602 = self::ml_array::optional_at(&*ml_reference_array602.borrow(), self::ml_array::number_index(ml_reference_position602)); ml_reference_projection602 }
                                                        }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                        {
                                                            let result_2 = self::source_ml_source1_temporary_power_java_script_number_loop134(scale.clone(), interval_5.clone(), result.clone(), remaining.clone());
                                                            {
                                                                let lower = self::round_number_power_bound(({
                                                                    let source_ml_source2_array_read_index3886 = 0f64;
                                                                    { let ml_reference_array603 = result_2.clone(); let ml_reference_position603 = source_ml_source2_array_read_index3886; let ml_reference_projection603 = self::ml_array::optional_at(&*ml_reference_array603.borrow(), self::ml_array::number_index(ml_reference_position603)); ml_reference_projection603 }
                                                                }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), ({
                                                                    let source_ml_source2_array_read_index3888 = 1f64;
                                                                    { let ml_reference_array604 = result_2.clone(); let ml_reference_position604 = source_ml_source2_array_read_index3888; let ml_reference_projection604 = self::ml_array::optional_at(&*ml_reference_array604.borrow(), self::ml_array::number_index(ml_reference_position604)); ml_reference_projection604 }
                                                                }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), precision.clone());
                                                                {
                                                                    let upper = self::round_number_power_bound(({
                                                                        let source_ml_source2_array_read_index3890 = 2f64;
                                                                        { let ml_reference_array605 = result_2.clone(); let ml_reference_position605 = source_ml_source2_array_read_index3890; let ml_reference_projection605 = self::ml_array::optional_at(&*ml_reference_array605.borrow(), self::ml_array::number_index(ml_reference_position605)); ml_reference_projection605 }
                                                                    }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), ({
                                                                        let source_ml_source2_array_read_index3892 = 3f64;
                                                                        { let ml_reference_array606 = result_2.clone(); let ml_reference_position606 = source_ml_source2_array_read_index3892; let ml_reference_projection606 = self::ml_array::optional_at(&*ml_reference_array606.borrow(), self::ml_array::number_index(ml_reference_position606)); ml_reference_projection606 }
                                                                    }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)), precision.clone());
                                                                    if (lower == upper) {
                                                                        if negative {
                                                                            (-lower)
                                                                        } else {
                                                                            lower
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let precision_2 = precision.mul(&self::ml::Big::from_i128(2));
                                                                            {
                                                                                precision = precision_2.clone();
                                                                                continue;
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
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDependencyFunction3Loop135Result {
    MlSource1TemporaryDependencyFunction3Loop135Done(f64),
    MlSource1TemporaryDependencyFunction3Loop135Return(f64),
}

pub fn source_ml_source1_temporary_dependency_function3_loop135(mut rest: self::ml::Big, mut width: f64) -> self::MlSource1TemporaryDependencyFunction3Loop135Result {
    loop {
        return if (rest > (self::ml::Big::from_i128(0))) {
            if (width >= (4096f64)) {
                self::MlSource1TemporaryDependencyFunction3Loop135Result::MlSource1TemporaryDependencyFunction3Loop135Return((-1f64))
            } else {
                {
                    let rest_2 = self::ml::divide(&rest, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    {
                        let width_2 = (width + 1f64);
                        {
                            (rest, width) = (rest_2.clone(), width_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDependencyFunction3Loop135Result::MlSource1TemporaryDependencyFunction3Loop135Done(width)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137 {
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137Next(self::ml::Big),
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin137Return(Box<self::JavaScriptCallPrimitiveResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin136 {
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin136Next,
    MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin136Return(Box<self::JavaScriptCallPrimitiveResult>),
}

pub fn source_ml_source1_temporary_execute_java_script_call_integer_wrapping_loop138(mut bit_width: f64, mut modulus: self::ml::Big, mut bit: f64) -> self::ml::Big {
    loop {
        return if (bit < bit_width) {
            {
                let modulus_2 = modulus.mul(&self::ml::Big::from_i128(2));
                {
                    let bit_2 = (bit + 1f64);
                    {
                        (modulus, bit) = (modulus_2.clone(), bit_2);
                        continue;
                    }
                }
            }
        } else {
            modulus.clone()
        };
    }
}

pub fn source_ml_source1_temporary_execute_java_script_call_integer_radix_text_loop139(mut remaining: self::ml::Big, mut divisor: self::ml::Big, mut reversed: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return {
            let digit = self::ml_string_number::convert_java_script_big_integer_number({
                let source_ml_source2_integer_division3893 = self::ml_big_integer_conversion::divide_java_script_big_integer(remaining.clone(),divisor.clone());
                if (((source_ml_source2_integer_division3893.len() as f64)) == (2f64)) {
                    (self::ml_array::optional_at(&source_ml_source2_integer_division3893, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                } else {
                    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111])).0, message_present: true }).raise()
                }
            });
            {
                let reversed_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value607 = reversed.borrow().clone(); ml_reference_value607 }, vec![if (digit < (10f64)) {
                    (48f64 + digit)
                } else {
                    (87f64 + digit)
                }])));
                {
                    let remaining_2 = {
                        let source_ml_source2_integer_division3894 = self::ml_big_integer_conversion::divide_java_script_big_integer(remaining.clone(),divisor.clone());
                        if (((source_ml_source2_integer_division3894.len() as f64)) == (2f64)) {
                            (self::ml_array::optional_at(&source_ml_source2_integer_division3894, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111])).0, message_present: true }).raise()
                        }
                    };
                    if (remaining_2 > (self::ml::Big::from_i128(0))) {
                        {
                            (remaining, reversed) = (remaining_2.clone(), reversed_2.clone());
                            continue;
                        }
                    } else {
                        reversed_2.clone()
                    }
                }
            }
        };
    }
}

pub fn source_ml_source1_temporary_execute_java_script_call_integer_radix_text_loop140(mut reversed: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (index >= (0f64)) {
            {
                let units_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value608 = units.borrow().clone(); ml_reference_value608 }, vec![({ let ml_reference_array609 = reversed.clone(); let ml_reference_position609 = index; let ml_reference_projection609 = self::ml_array::optional_at(&*ml_reference_array609.borrow(), self::ml_array::number_index(ml_reference_position609)); ml_reference_projection609 }).clone().unwrap_or_else(|| 48f64)])));
                {
                    let index_2 = (index - 1f64);
                    {
                        (units, index) = (units_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            units.clone()
        };
    }
}

pub fn source_ml_source1_temporary_dependency_function5_loop142(mut text: self::ml_string::Text, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (index < (text.length())) {
            {
                let units_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value610 = units.borrow().clone(); ml_reference_value610 }, vec![{
                    let source_ml_source2_string_unit_operand3897 = text.clone();
                    {
                        let source_ml_source2_string_unit_operand3898 = index;
                        (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text_2.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand3904 = {
                                        let source_ml_source2_string_range_operand3899 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand3900 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand3901 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position3902 = source_ml_source2_string_range_operand3900;
                                                    {
                                                        let source_ml_source2_string_range_position3903 = source_ml_source2_string_range_operand3901;
                                                        source_ml_source2_string_range_operand3899.slice(source_ml_source2_string_range_position3902, source_ml_source2_string_range_position3903)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand3905 = 0f64;
                                        source_ml_source2_string_unit_operand3904.code_point_at(source_ml_source2_string_unit_operand3905)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand3897.clone(), source_ml_source2_string_unit_operand3898)
                    }
                }])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (units, index) = (units_2.clone(), index_2);
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
pub enum MlSource1TemporaryDependencyFunction5Join141 {
    MlSource1TemporaryDependencyFunction5Join141Next(self::ml_string::Text),
    MlSource1TemporaryDependencyFunction5Join141Return(Box<self::JavaScriptCallPrimitiveResult>),
}

pub fn source_ml_source1_temporary_read_java_script_call_integer_text_loop143(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut text: self::ml_string::Text, mut index: f64) -> self::ml_string::Text {
    loop {
        return if (index < ({ let ml_reference_array611 = units.clone(); let ml_reference_projection611 = ml_reference_array611.borrow().len() as f64; ml_reference_projection611 })) {
            {
                let text_2 = text.concat(&{
                    let source_ml_source2_character_argument3908 = ({ let ml_reference_array612 = units.clone(); let ml_reference_position612 = index; let ml_reference_projection612 = self::ml_array::optional_at(&*ml_reference_array612.borrow(), self::ml_array::number_index(ml_reference_position612)); ml_reference_projection612 }).clone().unwrap_or_else(|| 0f64);
                    self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source2_character_argument3908]).into_iter().map(|ml_unit| ml_unit as u16).collect())
                });
                {
                    let index_2 = (index + 1f64);
                    {
                        (text, index) = (text_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            text.clone()
        };
    }
}

pub fn source_ml_source1_temporary_construct_utf16_text_repetition_loop144(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut length: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (index < length) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value613 = result.borrow().clone(); ml_reference_value613 }, vec![({
                    let source_ml_source2_array_read_index3910 = (index % { let ml_reference_array614 = text.clone(); let ml_reference_projection614 = ml_reference_array614.borrow().len() as f64; ml_reference_projection614 });
                    { let ml_reference_array615 = text.clone(); let ml_reference_position615 = source_ml_source2_array_read_index3910; let ml_reference_projection615 = self::ml_array::optional_at(&*ml_reference_array615.borrow(), self::ml_array::number_index(ml_reference_position615)); ml_reference_projection615 }
                }).clone().unwrap_or_else(|| 0f64)])));
                {
                    let index_2 = (index + 1f64);
                    {
                        (result, index) = (result_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_construct_utf16_text_padding_loop145(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut filler: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut length: f64, mut start: bool, mut added: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (index < length) {
            {
                let original = if start {
                    (index >= added)
                } else {
                    (index < ({ let ml_reference_array616 = text.clone(); let ml_reference_projection616 = ml_reference_array616.borrow().len() as f64; ml_reference_projection616 }))
                };
                {
                    let position = if original {
                        if start {
                            (index - added)
                        } else {
                            index
                        }
                    } else {
                        if start {
                            index
                        } else {
                            (index - { let ml_reference_array617 = text.clone(); let ml_reference_projection617 = ml_reference_array617.borrow().len() as f64; ml_reference_projection617 })
                        }
                    };
                    {
                        let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value618 = result.borrow().clone(); ml_reference_value618 }, vec![(if original {
                            { let ml_reference_array619 = text.clone(); let ml_reference_position619 = position; let ml_reference_projection619 = self::ml_array::optional_at(&*ml_reference_array619.borrow(), self::ml_array::number_index(ml_reference_position619)); ml_reference_projection619 }
                        } else {
                            {
                                let source_ml_source2_array_read_index3914 = (position % { let ml_reference_array620 = filler.clone(); let ml_reference_projection620 = ml_reference_array620.borrow().len() as f64; ml_reference_projection620 });
                                { let ml_reference_array621 = filler.clone(); let ml_reference_position621 = source_ml_source2_array_read_index3914; let ml_reference_projection621 = self::ml_array::optional_at(&*ml_reference_array621.borrow(), self::ml_array::number_index(ml_reference_position621)); ml_reference_projection621 }
                            }
                        }).clone().unwrap_or_else(|| 0f64)])));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (result, index) = (result_2.clone(), index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop147Result {
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop147Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop147Return(Box<self::JavaScriptCallPrimitiveResult>),
}

pub fn source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop147(mut numbers: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values146: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut source_ml_source1_temporary_index146: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop147Result {
    loop {
        return if (source_ml_source1_temporary_index146 < ({ let ml_reference_array622 = source_ml_source1_temporary_values146.clone(); let ml_reference_projection622 = self::ml::Big::from_u128(ml_reference_array622.borrow().len() as u128); ml_reference_projection622 })) {
            {
                let value = { let ml_reference_array623 = source_ml_source1_temporary_values146.clone(); let ml_reference_position623 = source_ml_source1_temporary_index146.clone(); let ml_reference_projection623 = self::ml_array::at(&*ml_reference_array623.borrow(), ml_reference_position623.to_index()); ml_reference_projection623 };
                match value.clone() {
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        {
                            let numbers_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value624 = numbers.borrow().clone(); ml_reference_value624 }, vec![value_value])));
                            {
                                let source_ml_source1_temporary_index146_2 = source_ml_source1_temporary_index146.add(&self::ml::Big::from_i128(1));
                                {
                                    (numbers, source_ml_source1_temporary_index146) = (numbers_2.clone(), source_ml_source1_temporary_index146_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                    _ => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop147Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop147Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop147Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop147Done(numbers.clone())
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_call_text_replacement_work_error_loop148(mut length: f64, mut search: f64, mut plan: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut width: f64, mut index: f64) -> f64 {
    loop {
        return if (index < ({ let ml_reference_array625 = plan.clone(); let ml_reference_projection625 = ml_reference_array625.borrow().len() as f64; ml_reference_projection625 })) {
            {
                let kind = ({ let ml_reference_array626 = plan.clone(); let ml_reference_position626 = index; let ml_reference_projection626 = self::ml_array::optional_at(&*ml_reference_array626.borrow(), self::ml_array::number_index(ml_reference_position626)); ml_reference_projection626 }).clone().unwrap_or_else(|| 0f64);
                {
                    let width_2 = (width + if (kind == (0f64)) {
                        (({
                            let source_ml_source2_array_read_index3928 = (index + 2f64);
                            { let ml_reference_array627 = plan.clone(); let ml_reference_position627 = source_ml_source2_array_read_index3928; let ml_reference_projection627 = self::ml_array::optional_at(&*ml_reference_array627.borrow(), self::ml_array::number_index(ml_reference_position627)); ml_reference_projection627 }
                        }).clone().unwrap_or_else(|| 0f64) - ({
                            let source_ml_source2_array_read_index3930 = (index + 1f64);
                            { let ml_reference_array628 = plan.clone(); let ml_reference_position628 = source_ml_source2_array_read_index3930; let ml_reference_projection628 = self::ml_array::optional_at(&*ml_reference_array628.borrow(), self::ml_array::number_index(ml_reference_position628)); ml_reference_projection628 }
                        }).clone().unwrap_or_else(|| 0f64))
                    } else {
                        if (kind == (1f64)) {
                            search
                        } else {
                            length
                        }
                    });
                    {
                        let index_2 = (index + 3f64);
                        {
                            (width, index) = (width_2, index_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            width
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop150Result {
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop150Done,
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop150Return(Box<self::JavaScriptCallPrimitiveResult>),
}

pub fn source_ml_source1_temporary_execute_java_script_call_primitive_loop150(mut source_ml_source1_temporary_values149: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut source_ml_source1_temporary_index149: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop150Result {
    loop {
        return if (source_ml_source1_temporary_index149 < ({ let ml_reference_array629 = source_ml_source1_temporary_values149.clone(); let ml_reference_projection629 = self::ml::Big::from_u128(ml_reference_array629.borrow().len() as u128); ml_reference_projection629 })) {
            {
                let value = { let ml_reference_array630 = source_ml_source1_temporary_values149.clone(); let ml_reference_position630 = source_ml_source1_temporary_index149.clone(); let ml_reference_projection630 = self::ml_array::at(&*ml_reference_array630.borrow(), ml_reference_position630.to_index()); ml_reference_projection630 };
                {
                    let error = self::read_java_script_semantic_value_error(value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop150Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop150Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(error.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index149_2 = source_ml_source1_temporary_index149.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index149 = source_ml_source1_temporary_index149_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop150Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop150Done
        };
    }
}

pub fn source_ml_source1_temporary_execute_java_script_call_primitive_scalar_loop152(mut kind: self::ml_string::Text, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < (kind.length())) {
            {
                let units_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value631 = units.borrow().clone(); ml_reference_value631 }, vec![{
                    let source_ml_source2_string_unit_operand3933 = kind.clone();
                    {
                        let source_ml_source2_string_unit_operand3934 = at;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index < (0f64)) || (index >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand3940 = {
                                        let source_ml_source2_string_range_operand3935 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand3936 = index;
                                            {
                                                let source_ml_source2_string_range_operand3937 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position3938 = source_ml_source2_string_range_operand3936;
                                                    {
                                                        let source_ml_source2_string_range_position3939 = source_ml_source2_string_range_operand3937;
                                                        source_ml_source2_string_range_operand3935.slice(source_ml_source2_string_range_position3938, source_ml_source2_string_range_position3939)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand3941 = 0f64;
                                        source_ml_source2_string_unit_operand3940.code_point_at(source_ml_source2_string_unit_operand3941)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand3933.clone(), source_ml_source2_string_unit_operand3934)
                    }
                }])));
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

pub fn source_ml_source1_temporary_execute_java_script_call_primitive_scalar_loop153(mut materialized: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut source_ml_source1_temporary_values151: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut source_ml_source1_temporary_index151: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    loop {
        return if (source_ml_source1_temporary_index151 < ({ let ml_reference_array632 = source_ml_source1_temporary_values151.clone(); let ml_reference_projection632 = self::ml::Big::from_u128(ml_reference_array632.borrow().len() as u128); ml_reference_projection632 })) {
            {
                let value = { let ml_reference_array633 = source_ml_source1_temporary_values151.clone(); let ml_reference_position633 = source_ml_source1_temporary_index151.clone(); let ml_reference_projection633 = self::ml_array::at(&*ml_reference_array633.borrow(), ml_reference_position633.to_index()); ml_reference_projection633 };
                {
                    let operand = if match value.clone() {
                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                            true
                        }
                        _ => {
                            false
                        }
                    } {
                        self::JavaScriptSemanticValue::Text(self::construct_java_script_semantic_text_units(value.clone(), 1048576f64))
                    } else {
                        value.clone()
                    };
                    {
                        let materialized_2 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_concat_value3954 = std::rc::Rc::new(std::cell::RefCell::new(vec![operand.clone()]));
                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), { let ml_reference_value634 = materialized.borrow().clone(); ml_reference_value634 }), { let ml_reference_value635 = source_ml_source2_concat_value3954.borrow().clone(); ml_reference_value635 })
                        }));
                        {
                            let source_ml_source1_temporary_index151_2 = source_ml_source1_temporary_index151.add(&self::ml::Big::from_i128(1));
                            {
                                (materialized, source_ml_source1_temporary_index151) = (materialized_2.clone(), source_ml_source1_temporary_index151_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            materialized.clone()
        };
    }
}

fn ml_main() {
    assert!(((self::execute_java_script_call_primitive(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 121, 112, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![104, 101, 108, 108, 111]))])))) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![115f64, 116f64, 114f64, 105f64, 110f64, 103f64]))))))), "assertion 1");
    assert!(((self::execute_java_script_call_primitive(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110, 103, 116, 104]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units((vec![55296f64, 120f64]).into_iter().map(|ml_unit| ml_unit as u16).collect()))])))) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(2f64))))), "assertion 2");
    assert!(((self::execute_java_script_call_primitive(self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units((vec![55296f64, 120f64]).into_iter().map(|ml_unit| ml_unit as u16).collect()))])))) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units((vec![55296f64, 120f64]).into_iter().map(|ml_unit| ml_unit as u16).collect())))))), "assertion 3");
    assert!(((self::execute_java_script_call_primitive(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![55296f64]))), self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![120]))])))) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units((vec![55296f64, 120f64]).into_iter().map(|ml_unit| ml_unit as u16).collect())))))), "assertion 4");
    assert!(((self::execute_java_script_call_primitive(self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![45, 48]))])))) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number((-0f64)))))), "assertion 5");
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
fn all_literal_source_text_primitive_controls() { main(); }
