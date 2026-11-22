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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f397) => {
                        {
                            let source_ml_source1_f421 = source_ml_source1_f397;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f398) => {
                        {
                            let source_ml_source1_f430 = source_ml_source1_f398;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f399) => {
                        {
                            let source_ml_source1_f439 = source_ml_source1_f399.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Text(units) => {
                        (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f400, source_ml_source1_f401) => {
                        {
                            let source_ml_source1_f449 = source_ml_source1_f400.clone();
                            {
                                let text = source_ml_source1_f401.clone();
                                {
                                    let source_ml_source1_string_unit_operand453 = text.clone();
                                    {
                                        let source_ml_source1_string_unit_operand454 = at;
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
                                                    let source_ml_source1_string_unit_operand460 = {
                                                        let source_ml_source1_string_range_operand455 = text_2.clone();
                                                        {
                                                            let source_ml_source1_string_range_operand456 = index;
                                                            {
                                                                let source_ml_source1_string_range_operand457 = (index + 1f64);
                                                                {
                                                                    let source_ml_source1_string_range_position458 = source_ml_source1_string_range_operand456;
                                                                    {
                                                                        let source_ml_source1_string_range_position459 = source_ml_source1_string_range_operand457;
                                                                        source_ml_source1_string_range_operand455.slice(source_ml_source1_string_range_position458, source_ml_source1_string_range_position459)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_string_unit_operand461 = 0f64;
                                                        source_ml_source1_string_unit_operand460.code_point_at(source_ml_source1_string_unit_operand461)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source1_string_unit_operand453.clone(), source_ml_source1_string_unit_operand454)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f402) => {
                        {
                            let source_ml_source1_f467 = source_ml_source1_f402.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f403, source_ml_source1_f404) => {
                        {
                            let source_ml_source1_f476 = source_ml_source1_f403.clone();
                            {
                                let source_ml_source1_f477 = source_ml_source1_f404;
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f478) => {
                            {
                                let source_ml_source1_f502 = source_ml_source1_f478;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f479) => {
                            {
                                let source_ml_source1_f511 = source_ml_source1_f479;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f480) => {
                            {
                                let source_ml_source1_f520 = source_ml_source1_f480.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Text(units_2) => {
                            (self::ml_array::optional_at(&units_2, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f481, source_ml_source1_f482) => {
                            {
                                let source_ml_source1_f530 = source_ml_source1_f481.clone();
                                {
                                    let text_2 = source_ml_source1_f482.clone();
                                    {
                                        let source_ml_source1_string_unit_operand534 = text_2.clone();
                                        {
                                            let source_ml_source1_string_unit_operand535 = at;
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
                                                        let source_ml_source1_string_unit_operand541 = {
                                                            let source_ml_source1_string_range_operand536 = text_3.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand537 = index;
                                                                {
                                                                    let source_ml_source1_string_range_operand538 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source1_string_range_position539 = source_ml_source1_string_range_operand537;
                                                                        {
                                                                            let source_ml_source1_string_range_position540 = source_ml_source1_string_range_operand538;
                                                                            source_ml_source1_string_range_operand536.slice(source_ml_source1_string_range_position539, source_ml_source1_string_range_position540)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_string_unit_operand542 = 0f64;
                                                            source_ml_source1_string_unit_operand541.code_point_at(source_ml_source1_string_unit_operand542)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source1_string_unit_operand534.clone(), source_ml_source1_string_unit_operand535)
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f483) => {
                            {
                                let source_ml_source1_f548 = source_ml_source1_f483.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f484, source_ml_source1_f485) => {
                            {
                                let source_ml_source1_f557 = source_ml_source1_f484.clone();
                                {
                                    let source_ml_source1_f558 = source_ml_source1_f485;
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

pub fn source_ml_temporary_read_java_script_semantic_text_string_loop4(mut text: self::ml_string::Text, mut source_ml_temporary_values3: Vec<f64>, mut source_ml_temporary_index3: self::ml::Big) -> self::ml_string::Text {
    loop {
        return if (source_ml_temporary_index3 < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))) {
            {
                let unit = self::ml_array::at(&source_ml_temporary_values3, source_ml_temporary_index3.to_index());
                {
                    let text_2 = text.concat(&{
                        let source_ml_source1_character_argument559 = unit;
                        self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source1_character_argument559]).into_iter().map(|ml_unit| ml_unit as u16).collect())
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

pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop5(mut value: self::JavaScriptSemanticValue, mut length: f64, mut units: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < length) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_java_script_semantic_text_unit(value.clone(), at)], units); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (units, at) = (units_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            units
        };
    }
}

fn ml_main() {
    assert!(((self::read_java_script_semantic_value_error(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units((vec![55296f64, 120f64, 57343f64]).into_iter().map(|ml_unit| ml_unit as u16).collect())))) == (self::ml_string::Text::from_units(vec![]))), "assertion 1");
    assert!(((self::test_java_script_semantic_same_value(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units((vec![55296f64, 120f64, 57343f64]).into_iter().map(|ml_unit| ml_unit as u16).collect())), self::JavaScriptSemanticValue::Text(vec![55296f64, 120f64, 57343f64]))) == (true)), "assertion 2");
    assert!(((self::test_java_script_semantic_same_value(self::JavaScriptSemanticValue::Text(vec![55296f64, 120f64, 57343f64]), self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units((vec![55296f64, 120f64, 57343f64]).into_iter().map(|ml_unit| ml_unit as u16).collect())))) == (true)), "assertion 3");
    assert!(((self::test_java_script_semantic_same_value(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![97])), self::JavaScriptSemanticValue::Text(vec![98f64]))) == (false)), "assertion 4");
    assert!(((self::test_java_script_semantic_same_value(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![])), self::JavaScriptSemanticValue::Text(Vec::<f64>::new()))) == (true)), "assertion 5");
    assert!(self::ml_number::same_value(self::read_java_script_semantic_text_length(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![55357, 56489]))), 2f64), "assertion 6");
    assert!(self::ml_number::same_value(self::read_java_script_semantic_text_unit(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![55357, 56489])), 1f64), 56489f64), "assertion 7");
    assert!(self::ml_number::same_value(self::read_java_script_semantic_text_unit(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![55357, 56489])), 2f64), (-1f64)), "assertion 8");
    assert!(((self::read_java_script_semantic_value_error(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![117, 110, 113, 117, 97, 108, 105, 102, 105, 101, 100]), self::ml_string::Text::from_units(vec![97])))) == (self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101]))), "assertion 9");
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
fn all_literal_source_text_semantic_controls() { main(); }
