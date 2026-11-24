pub fn source_ml_source1_temporary_replace_utf16_text_loop59(mut text: Vec<f64>, mut result: Vec<f64>, mut found: f64, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < found) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_source1_temporary_replace_utf16_text_loop61(mut result: Vec<f64>, mut last: f64, mut source: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(&source, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_source1_temporary_replace_utf16_text_loop60(mut text: Vec<f64>, mut search: Vec<f64>, mut replacement: Vec<f64>, mut result: Vec<f64>, mut plan: Vec<f64>, mut found: f64, mut instruction: f64) -> Vec<f64> {
    loop {
        return if (instruction < ((plan.len() as f64))) {
            {
                let kind = (self::ml_array::optional_at(&plan, self::ml_array::number_index(instruction))).clone().unwrap_or_else(|| 0f64);
                {
                    let first = if (kind == (0f64)) {
                        (self::ml_array::optional_at(&plan, self::ml_array::number_index((instruction + 1f64)))).clone().unwrap_or_else(|| 0f64)
                    } else {
                        if (kind == (2f64)) {
                            0f64
                        } else {
                            if (kind == (1f64)) {
                                found
                            } else {
                                (found + (search.len() as f64))
                            }
                        }
                    };
                    {
                        let last = if (kind == (0f64)) {
                            (self::ml_array::optional_at(&plan, self::ml_array::number_index((instruction + 2f64)))).clone().unwrap_or_else(|| 0f64)
                        } else {
                            if (kind == (2f64)) {
                                found
                            } else {
                                if (kind == (1f64)) {
                                    (found + (search.len() as f64))
                                } else {
                                    (text.len() as f64)
                                }
                            }
                        };
                        {
                            let source = if (kind == (0f64)) {
                                replacement.clone()
                            } else {
                                text.clone()
                            };
                            {
                                let at = first;
                                {
                                    let result_2 = self::source_ml_source1_temporary_replace_utf16_text_loop61(result, last, source, at);
                                    {
                                        let instruction_2 = (instruction + 3f64);
                                        {
                                            (result, instruction) = (result_2, instruction_2);
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
            result
        };
    }
}

pub fn source_ml_source1_temporary_replace_utf16_text_loop57(mut text: Vec<f64>, mut search: Vec<f64>, mut replacement: Vec<f64>, mut all: bool, mut result: Vec<f64>, mut plan: Vec<f64>, mut start: f64, mut from: f64, mut searching: bool) -> self::MlSource1TemporaryReplaceUtf16TextLoop57Result {
    loop {
        return if searching {
            {
                let found = self::index_utf16_text(text.clone(), search.clone(), from);
                if (found < (0f64)) {
                    {
                        let searching_2 = false;
                        {
                            searching = searching_2;
                            continue;
                        }
                    }
                } else {
                    {
                        let at = start;
                        {
                            let result_2 = self::source_ml_source1_temporary_replace_utf16_text_loop59(text.clone(), result, found, at);
                            {
                                let instruction = 0f64;
                                {
                                    let result_3 = self::source_ml_source1_temporary_replace_utf16_text_loop60(text.clone(), search.clone(), replacement.clone(), result_2, plan.clone(), found, instruction);
                                    {
                                        let start_2 = (found + (search.len() as f64));
                                        {
                                            let searching_3 = (all && !(((((search.len() as f64)) == (0f64)) && (found == ((text.len() as f64))))));
                                            {
                                                let from_2 = (start_2 + if (((search.len() as f64)) == (0f64)) {
                                                    1f64
                                                } else {
                                                    0f64
                                                });
                                                {
                                                    (result, start, from, searching) = (result_3, start_2, from_2, searching_3);
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
        } else {
            self::MlSource1TemporaryReplaceUtf16TextLoop57Result::MlSource1TemporaryReplaceUtf16TextLoop57Done(result, start)
        };
    }
}

pub fn source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(mut receiver_text: self::JavaScriptSemanticValue, mut search_text: self::JavaScriptSemanticValue, mut search_length: f64, mut start: f64, mut matches: bool, mut at: f64) -> bool {
    loop {
        return if (matches && (at < search_length)) {
            if ((self::read_java_script_semantic_text_unit(receiver_text.clone(), (start + at))) != (self::read_java_script_semantic_text_unit(search_text.clone(), at))) {
                {
                    let matches_2 = false;
                    {
                        let at_2 = (at + 1f64);
                        {
                            (matches, at) = (matches_2, at_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at + 1f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            matches
        };
    }
}

fn ml_main() {
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Text(vec![97f64, 98f64, 99f64]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Text(vec![98f64]), self::JavaScriptSemanticValue::Number(1f64)])) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(true))))), "assertion 1");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![97, 98, 99])), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Text(vec![98f64]), self::JavaScriptSemanticValue::Number(2f64)])) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(false))))), "assertion 2");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Text(vec![97f64, 98f64, 99f64]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Text(vec![97f64]), self::JavaScriptSemanticValue::Number(f64::NEG_INFINITY), self::JavaScriptSemanticValue::Number(19f64)])) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(true))))), "assertion 3");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Text(vec![97f64, 98f64, 99f64]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Text(Vec::<f64>::new()), self::JavaScriptSemanticValue::Number(f64::INFINITY)])) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(true))))), "assertion 4");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Text(vec![97f64, 98f64, 99f64]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Text(vec![97f64]), self::JavaScriptSemanticValue::Number(f64::NAN)])) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(true))))), "assertion 5");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Text(vec![97f64, 98f64, 99f64]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Text(vec![97f64]), self::JavaScriptSemanticValue::Number((-0f64))])) == (self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(true))))), "assertion 6");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Undefined, self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), Vec::<self::JavaScriptSemanticValue>::new())) == (self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104, 32, 99, 97, 108, 108, 101, 100, 32, 111, 110, 32, 110, 117, 108, 108, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100])))), "assertion 7");
    assert!(((self::execute_java_script_primitive_method(self::JavaScriptSemanticValue::Text(vec![97f64]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]), vec![self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(1), false)])) == (self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 73, 115, 82, 101, 103, 69, 120, 112, 32, 97, 110, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108, 115])))), "assertion 8");
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
fn all_literal_primitive_method_controls() { main(); }
