pub fn read_java_script_closed_class_directory(values: Vec<self::ml_string::Text>, kinds: Vec<self::ml_string::Text>, first: f64) -> Vec<f64> {
    if (((((self::ml_array::optional_at(&values, self::ml_array::number_index(first))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) || (((self::ml_array::optional_at(&kinds, self::ml_array::number_index((first + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((first + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![123])))) {
        vec![(-1f64), first]
    } else {
        {
            let end = self::find_java_script_class_group_end(values.clone(), (first + 2f64));
            if (end < (0f64)) {
                vec![(-1f64), (first + 2f64)]
            } else {
                {
                    let constructor = vec![(-1f64), (-1f64), (-1f64), (-1f64), (-1f64)];
                    {
                        let methods = Vec::<f64>::new();
                        {
                            let fields = Vec::<f64>::new();
                            {
                                let names = Vec::<self::ml_string::Text>::new();
                                {
                                    let at = (first + 3f64);
                                    {
                                        let source_ml_source2_s15 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop12(values.clone(), kinds.clone(), end, constructor, methods, fields, names, at);
                                        match source_ml_source2_s15.clone() {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Done(constructor_2, methods_2, fields_2, names_2) => {
                                                {
                                                    let body_start = (self::ml_array::optional_at(&constructor_2, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| (-1f64));
                                                    {
                                                        let body_end = (self::ml_array::optional_at(&constructor_2, self::ml_array::number_index(4f64))).clone().unwrap_or_else(|| (-1f64));
                                                        {
                                                            let initialization = if (body_start < (0f64)) {
                                                                (-1f64)
                                                            } else {
                                                                self::find_java_script_closed_class_initialization_start(values.clone(), (body_start + 1f64), body_end)
                                                            };
                                                            {
                                                                let cursor = initialization;
                                                                {
                                                                    let source_ml_source2_s16 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop13(values.clone(), kinds, fields_2, names_2, body_start, body_end, cursor);
                                                                    match source_ml_source2_s16.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Done(fields_3) => {
                                                                            {
                                                                                let source_ml_source2_s18 = if (body_start >= (0f64)) {
                                                                                    {
                                                                                        let at_2 = (body_start + 1f64);
                                                                                        {
                                                                                            let source_ml_source2_s19 = self::source_ml_source1_temporary_read_java_script_closed_class_directory_loop15(values, body_end, at_2);
                                                                                            match source_ml_source2_s19.clone() {
                                                                                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Done => {
                                                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Next
                                                                                                }
                                                                                                self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop15Return(source_ml_source1_temporary_result15) => {
                                                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Return(source_ml_source1_temporary_result15)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Next
                                                                                };
                                                                                match source_ml_source2_s18.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Next => {
                                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(self::ml_array::append(vec![first, end], constructor_2), vec![((fields_3.len() as f64) / 3f64), initialization]), fields_3), methods_2)
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14::MlSource1TemporaryReadJavaScriptClosedClassDirectoryJoin14Return(source_ml_source1_temporary_result14) => {
                                                                                        source_ml_source1_temporary_result14
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop13Return(source_ml_source1_temporary_result15_2) => {
                                                                            source_ml_source1_temporary_result15_2
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Result::MlSource1TemporaryReadJavaScriptClosedClassDirectoryLoop12Return(source_ml_source1_temporary_result19) => {
                                                source_ml_source1_temporary_result19
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

pub fn read_java_script_closed_class_refusal(code: f64) -> self::ml_string::Text {
    if (code == ((-1f64))) {
        self::ml_string::Text::from_units(vec![97, 32, 99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 110, 101, 101, 100, 115, 32, 97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 110, 97, 109, 101, 100, 32, 98, 97, 115, 101, 45, 99, 108, 97, 115, 115, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 97, 108, 97, 110, 99, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 103, 114, 111, 117, 112, 115])
    } else {
        if (code == ((-2f64))) {
            self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114, 115, 44, 32, 112, 114, 105, 118, 97, 116, 101, 32, 110, 97, 109, 101, 115, 44, 32, 105, 110, 104, 101, 114, 105, 116, 97, 110, 99, 101, 44, 32, 115, 116, 97, 116, 105, 99, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 109, 101, 109, 98, 101, 114, 32, 111, 119, 110, 101, 114, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 102, 117, 108, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 109, 111, 100, 101, 108])
        } else {
            if (code == ((-3f64))) {
                self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 99, 108, 97, 115, 115, 32, 112, 114, 111, 102, 105, 108, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 102, 105, 120, 101, 100, 32, 105, 100, 101, 110, 116, 105, 102, 105, 101, 114, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 119, 105, 116, 104, 32, 108, 105, 116, 101, 114, 97, 108, 32, 100, 101, 102, 97, 117, 108, 116, 115, 59, 32, 112, 97, 116, 116, 101, 114, 110, 115, 32, 97, 110, 100, 32, 100, 101, 112, 101, 110, 100, 101, 110, 116, 32, 111, 114, 32, 105, 110, 115, 116, 97, 110, 99, 101, 45, 114, 101, 97, 100, 105, 110, 103, 32, 100, 101, 102, 97, 117, 108, 116, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116])
            } else {
                if (code == ((-11f64))) {
                    self::ml_string::Text::from_units(vec![97, 32, 99, 108, 111, 115, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 100, 101, 102, 97, 117, 108, 116, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 108, 111, 99, 97, 108, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 119, 105, 116, 104, 32, 116, 104, 101, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 77, 97, 112, 32, 111, 114, 32, 83, 101, 116, 32, 111, 119, 110, 101, 114])
                } else {
                    if (code == ((-4f64))) {
                        self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 110, 100, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 101, 116, 104, 111, 100])
                    } else {
                        if (code == ((-5f64))) {
                            self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 115, 32, 110, 101, 101, 100, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 105, 99, 111, 108, 111, 110, 45, 116, 101, 114, 109, 105, 110, 97, 116, 101, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 112, 114, 101, 102, 105, 120, 32, 119, 105, 116, 104, 111, 117, 116, 32, 105, 110, 115, 116, 97, 110, 99, 101, 32, 101, 115, 99, 97, 112, 101, 32, 111, 114, 32, 114, 101, 97, 100, 115, 32, 98, 101, 102, 111, 114, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110])
                        } else {
                            if (code == ((-6f64))) {
                                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 116, 117, 114, 110, 32, 111, 118, 101, 114, 114, 105, 100, 101, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115])
                            } else {
                                if (code == ((-7f64))) {
                                    self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 118, 97, 108, 117, 101, 115, 44, 32, 114, 101, 102, 108, 101, 99, 116, 105, 118, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 44, 32, 100, 101, 116, 97, 99, 104, 101, 100, 32, 109, 101, 116, 104, 111, 100, 115, 44, 32, 109, 101, 116, 104, 111, 100, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 97, 110, 100, 32, 111, 98, 115, 101, 114, 118, 97, 98, 108, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 32, 102, 117, 108, 108, 32, 111, 98, 106, 101, 99, 116, 32, 109, 111, 100, 101, 108])
                                } else {
                                    if (code == ((-8f64))) {
                                        self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 97, 117, 116, 104, 111, 114, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 111, 117, 114, 99, 101, 44, 32, 116, 111, 107, 101, 110, 44, 32, 110, 101, 115, 116, 105, 110, 103, 32, 111, 114, 32, 99, 108, 97, 115, 115, 45, 99, 111, 117, 110, 116, 32, 98, 117, 100, 103, 101, 116])
                                    } else {
                                        if (code == ((-10f64))) {
                                            self::ml_string::Text::from_units(vec![112, 117, 98, 108, 105, 99, 32, 102, 105, 101, 108, 100, 115, 32, 110, 101, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 105, 99, 111, 108, 111, 110, 45, 116, 101, 114, 109, 105, 110, 97, 116, 101, 100, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 115, 32, 119, 105, 116, 104, 32, 112, 114, 111, 118, 101, 110, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 115, 32, 97, 110, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 105, 110, 103])
                                        } else {
                                            if (code == ((-9f64))) {
                                                self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 109, 101, 109, 98, 101, 114, 32, 109, 111, 100, 105, 102, 105, 101, 114, 32, 107, 101, 121, 119, 111, 114, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 101, 115, 99, 97, 112, 101, 100, 32, 69, 67, 77, 65, 83, 99, 114, 105, 112, 116, 32, 115, 111, 117, 114, 99, 101, 32, 115, 112, 101, 108, 108, 105, 110, 103])
                                            } else {
                                                self::ml_string::Text::from_units(vec![])
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

pub fn render_java_script_closed_class_range(source: self::ml_string::Text, values: Vec<self::ml_string::Text>, starts: Vec<f64>, ends: Vec<f64>, first: f64, last: f64, receiver: self::ml_string::Text, class_names: Vec<self::ml_string::Text>, static_calls: Vec<self::ml_string::Text>) -> self::ml_string::Text {
    if (first >= last) {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let result = self::ml_string::Text::from_units(vec![]);
            {
                let previous = (self::ml_array::optional_at(&starts, self::ml_array::number_index(first))).clone().unwrap_or_else(|| 0f64);
                {
                    let at = first;
                    {
                        let source_ml_source2_s20 = self::source_ml_source1_temporary_render_java_script_closed_class_range_loop21(source.clone(), values, starts, ends.clone(), last, receiver.clone(), class_names, static_calls, result.clone(), previous, at);
                        match source_ml_source2_s20.clone() {
                            self::MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Result::MlSource1TemporaryRenderJavaScriptClosedClassRangeLoop21Done(result_2, previous_2) => {
                                result_2.concat(&{
                                    let source_ml_source2_string_range_operand21 = source.clone();
                                    {
                                        let source_ml_source2_string_range_operand22 = previous_2;
                                        {
                                            let source_ml_source2_string_range_operand23 = (self::ml_array::optional_at(&ends, self::ml_array::number_index((last - 1f64)))).clone().unwrap_or_else(|| previous_2);
                                            {
                                                let source_ml_source2_string_range_position24 = source_ml_source2_string_range_operand22;
                                                {
                                                    let source_ml_source2_string_range_position25 = source_ml_source2_string_range_operand23;
                                                    source_ml_source2_string_range_operand21.slice(source_ml_source2_string_range_position24, source_ml_source2_string_range_position25)
                                                }
                                            }
                                        }
                                    }
                                })
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_methods(values: Vec<self::ml_string::Text>, directory: Vec<f64>) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let first = (9f64 + (3f64 * (self::ml_array::optional_at(&directory, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
            {
                let at = first;
                {
                    let result_2 = self::source_ml_source1_temporary_read_java_script_closed_class_methods_loop22(values, directory, result, at);
                    result_2
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_static_calls(values: Vec<self::ml_string::Text>, directory: Vec<Vec<f64>>, prefix: self::ml_string::Text) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let ordinal = 0f64;
            {
                let source_ml_source1_temporary_values23 = directory;
                {
                    let source_ml_source1_temporary_index23 = self::ml::Big::from_i128(0);
                    {
                        let result_2 = self::source_ml_source1_temporary_read_java_script_closed_class_static_calls_loop24(values, prefix.clone(), result, ordinal, source_ml_source1_temporary_values23, source_ml_source1_temporary_index23.clone());
                        result_2
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_static_target(values: Vec<self::ml_string::Text>, names: Vec<self::ml_string::Text>, calls: Vec<self::ml_string::Text>, at: f64) -> self::ml_string::Text {
    {
        let previous = (self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
        if (((((!(self::ml_array::includes(&names, &(self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) || (previous == (self::ml_string::Text::from_units(vec![110, 101, 119])))) || (previous == (self::ml_string::Text::from_units(vec![46])))) || (previous == (self::ml_string::Text::from_units(vec![63, 46])))) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![46])))) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 3f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![40])))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let key = (((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))).concat(&self::ml_string::Text::from_units(vec![46]))).concat(&(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])));
                {
                    let index = 0f64;
                    {
                        let source_ml_source2_s26 = self::source_ml_source1_temporary_read_java_script_closed_class_static_target_loop26(calls, key.clone(), index);
                        match source_ml_source2_s26.clone() {
                            self::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Result::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Result::MlSource1TemporaryReadJavaScriptClosedClassStaticTargetLoop26Return(source_ml_source1_temporary_result26) => {
                                source_ml_source1_temporary_result26.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_collection_default_class_owner_error(values: Vec<self::ml_string::Text>, directory: Vec<Vec<f64>>) -> f64 {
    {
        let names = Vec::<self::ml_string::Text>::new();
        {
            let source_ml_source1_temporary_values27 = directory.clone();
            {
                let source_ml_source1_temporary_index27 = self::ml::Big::from_i128(0);
                {
                    let names_2 = self::source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop29(values.clone(), names, source_ml_source1_temporary_values27, source_ml_source1_temporary_index27.clone());
                    {
                        let source_ml_source1_temporary_values28 = directory;
                        {
                            let source_ml_source1_temporary_index28 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s27 = self::source_ml_source1_temporary_read_java_script_closed_class_collection_default_class_owner_error_loop30(values, names_2, source_ml_source1_temporary_values28, source_ml_source1_temporary_index28.clone());
                                match source_ml_source2_s27.clone() {
                                    self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Done => {
                                        (-1f64)
                                    }
                                    self::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClosedClassCollectionDefaultClassOwnerErrorLoop30Return(source_ml_source1_temporary_result30) => {
                                        source_ml_source1_temporary_result30
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

pub fn accept_java_script_closed_class_function_export(values: Vec<self::ml_string::Text>, kinds: Vec<self::ml_string::Text>, at: f64) -> bool {
    {
        let previous = (self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
        ((((((((self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![101, 120, 112, 111, 114, 116]))) && (previous != (self::ml_string::Text::from_units(vec![46])))) && (previous != (self::ml_string::Text::from_units(vec![63, 46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])))) && (((self::ml_array::optional_at(&kinds, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 3f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![40]))))
    }
}

pub fn read_java_script_closed_class_function_exports(values: Vec<self::ml_string::Text>, kinds: Vec<self::ml_string::Text>) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_read_java_script_closed_class_function_exports_loop34(values, kinds, result, at);
                result_2
            }
        }
    }
}

pub fn read_java_script_closed_class_observation_error(values: Vec<self::ml_string::Text>, kinds: Vec<self::ml_string::Text>, directory: Vec<Vec<f64>>, static_calls: Vec<self::ml_string::Text>) -> f64 {
    {
        let names = Vec::<self::ml_string::Text>::new();
        {
            let methods = Vec::<self::ml_string::Text>::new();
            {
                let source_ml_source1_temporary_values35 = directory.clone();
                {
                    let source_ml_source1_temporary_index35 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s28 = self::source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop37(values.clone(), names, methods, source_ml_source1_temporary_values35, source_ml_source1_temporary_index35.clone());
                        match source_ml_source2_s28.clone() {
                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Done(names_2, methods_2) => {
                                {
                                    let at = 0f64;
                                    {
                                        let source_ml_source2_s29 = self::source_ml_source1_temporary_read_java_script_closed_class_observation_error_loop38(values, kinds, directory, static_calls, names_2, methods_2, at);
                                        match source_ml_source2_s29.clone() {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Done => {
                                                (-1f64)
                                            }
                                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop38Return(source_ml_source1_temporary_result38) => {
                                                source_ml_source1_temporary_result38
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Result::MlSource1TemporaryReadJavaScriptClosedClassObservationErrorLoop37Return(source_ml_source1_temporary_result40) => {
                                source_ml_source1_temporary_result40
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn render_java_script_closed_class(source: self::ml_string::Text, values: Vec<self::ml_string::Text>, starts: Vec<f64>, ends: Vec<f64>, row: Vec<f64>, prefix: self::ml_string::Text, ordinal: f64, class_names: Vec<self::ml_string::Text>, static_calls: Vec<self::ml_string::Text>, prototype: bool) -> self::ml_string::Text {
    {
        let name = (self::ml_array::optional_at(&values, self::ml_array::number_index(((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64) + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
        {
            let receiver = (prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114, 95]))).concat(&{
                let source_ml_source2_string_argument30 = ordinal;
                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument30))
            });
            {
                let brand = (prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95]))).concat(&{
                    let source_ml_source2_string_argument31 = ordinal;
                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument31))
                });
                {
                    let brand_value = (prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 98, 114, 97, 110, 100, 95, 118, 97, 108, 117, 101, 95]))).concat(&{
                        let source_ml_source2_string_argument32 = ordinal;
                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument32))
                    });
                    {
                        let field_count = (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64);
                        {
                            let methods = (9f64 + (field_count * 3f64));
                            {
                                let parameter_first = ((self::ml_array::optional_at(&row, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| (-1f64)) + 1f64);
                                {
                                    let parameter_last = (self::ml_array::optional_at(&row, self::ml_array::number_index(4f64))).clone().unwrap_or_else(|| (-1f64));
                                    {
                                        let parameters = if (parameter_last < (0f64)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::render_java_script_closed_class_range(source.clone(), values.clone(), starts.clone(), ends.clone(), parameter_first, parameter_last, self::ml_string::Text::from_units(vec![]), class_names.clone(), static_calls.clone())
                                        };
                                        {
                                            let stubs = ((self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32])).concat(&brand_value)).concat(&self::ml_string::Text::from_units(vec![40, 41, 32, 123, 32, 114, 101, 116, 117, 114, 110, 59, 32, 125, 10]));
                                            {
                                                let initializations = self::ml_string::Text::from_units(vec![]);
                                                {
                                                    let constructor_fields = self::ml_string::Text::from_units(vec![]);
                                                    {
                                                        let field_names = Vec::<self::ml_string::Text>::new();
                                                        {
                                                            let fields = ((brand.concat(&self::ml_string::Text::from_units(vec![58, 32]))).concat(&brand_value)).concat(&self::ml_string::Text::from_units(vec![40, 41]));
                                                            {
                                                                let at = 9f64;
                                                                {
                                                                    let source_ml_source2_s33 = self::source_ml_source1_temporary_render_java_script_closed_class_loop43(source.clone(), values.clone(), starts.clone(), ends.clone(), row.clone(), prefix.clone(), ordinal, class_names.clone(), static_calls.clone(), methods, stubs.clone(), initializations.clone(), constructor_fields.clone(), field_names, fields.clone(), at);
                                                                    match source_ml_source2_s33.clone() {
                                                                        self::MlSource1TemporaryRenderJavaScriptClosedClassLoop43Result::MlSource1TemporaryRenderJavaScriptClosedClassLoop43Done(stubs_2, initializations_2, constructor_fields_2, fields_2) => {
                                                                            {
                                                                                let assignments = self::ml_string::Text::from_units(vec![]);
                                                                                {
                                                                                    let at_2 = methods;
                                                                                    {
                                                                                        let source_ml_source2_s34 = self::source_ml_source1_temporary_render_java_script_closed_class_loop44(source.clone(), values.clone(), starts.clone(), ends.clone(), row.clone(), prefix.clone(), ordinal, class_names.clone(), static_calls.clone(), prototype, receiver.clone(), stubs_2.clone(), fields_2.clone(), assignments.clone(), at_2);
                                                                                        match source_ml_source2_s34.clone() {
                                                                                            self::MlSource1TemporaryRenderJavaScriptClosedClassLoop44Result::MlSource1TemporaryRenderJavaScriptClosedClassLoop44Done(stubs_3, fields_3, assignments_2) => {
                                                                                                {
                                                                                                    let constructor_body = (self::ml_array::optional_at(&row, self::ml_array::number_index(5f64))).clone().unwrap_or_else(|| (-1f64));
                                                                                                    {
                                                                                                        let constructor_end = (self::ml_array::optional_at(&row, self::ml_array::number_index(6f64))).clone().unwrap_or_else(|| (-1f64));
                                                                                                        {
                                                                                                            let initialization = (self::ml_array::optional_at(&row, self::ml_array::number_index(8f64))).clone().unwrap_or_else(|| (-1f64));
                                                                                                            {
                                                                                                                let prologue = if (constructor_body < (0f64)) {
                                                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                                                } else {
                                                                                                                    self::render_java_script_closed_class_range(source.clone(), values.clone(), starts.clone(), ends.clone(), (constructor_body + 1f64), initialization, self::ml_string::Text::from_units(vec![]), class_names.clone(), static_calls.clone())
                                                                                                                };
                                                                                                                {
                                                                                                                    let remainder = if (constructor_body < (0f64)) {
                                                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                                                    } else {
                                                                                                                        self::render_java_script_closed_class_range(source.clone(), values, starts.clone(), ends.clone(), self::read_java_script_closed_class_prefix_end(row.clone()), constructor_end, receiver.clone(), class_names, static_calls)
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let constructor_name = (self::ml_array::optional_at(&row, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| (-1f64));
                                                                                                                        {
                                                                                                                            let documentation = if (constructor_name < (0f64)) {
                                                                                                                                self::ml_string::Text::from_units(vec![])
                                                                                                                            } else {
                                                                                                                                {
                                                                                                                                    let source_ml_source2_string_range_operand35 = source.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_string_range_operand36 = (self::ml_array::optional_at(&ends, self::ml_array::number_index((constructor_name - 1f64)))).clone().unwrap_or_else(|| 0f64);
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_string_range_operand37 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(constructor_name))).clone().unwrap_or_else(|| 0f64);
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_string_range_position38 = source_ml_source2_string_range_operand36;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_string_range_position39 = source_ml_source2_string_range_operand37;
                                                                                                                                                    source_ml_source2_string_range_operand35.slice(source_ml_source2_string_range_position38, source_ml_source2_string_range_position39)
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            };
                                                                                                                            ((((((((((((((((((((stubs_3.concat(&self::ml_string::Text::from_units(vec![10]))).concat(&documentation)).concat(&self::ml_string::Text::from_units(vec![10, 102, 117, 110, 99, 116, 105, 111, 110, 32]))).concat(&name)).concat(&self::ml_string::Text::from_units(vec![40]))).concat(&parameters)).concat(&self::ml_string::Text::from_units(vec![41, 32, 123, 10]))).concat(&initializations_2)).concat(&prologue)).concat(&self::ml_string::Text::from_units(vec![10]))).concat(&constructor_fields_2)).concat(&self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 123, 32]))).concat(&fields_3)).concat(&self::ml_string::Text::from_units(vec![32, 125, 59, 10]))).concat(&assignments_2)).concat(&remainder)).concat(&self::ml_string::Text::from_units(vec![10, 114, 101, 116, 117, 114, 110, 32]))).concat(&receiver)).concat(&self::ml_string::Text::from_units(vec![59, 10, 125, 10]))
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
                }
            }
        }
    }
}

pub fn require_java_script_closed_class_prototype_methods(values: Vec<self::ml_string::Text>, directory: Vec<Vec<f64>>) -> bool {
    {
        let names = Vec::<self::ml_string::Text>::new();
        {
            let source_ml_source1_temporary_values47 = directory.clone();
            {
                let source_ml_source1_temporary_index47 = self::ml::Big::from_i128(0);
                {
                    let names_2 = self::source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop49(values.clone(), names, source_ml_source1_temporary_values47, source_ml_source1_temporary_index47.clone());
                    {
                        let source_ml_source1_temporary_values48 = directory;
                        {
                            let source_ml_source1_temporary_index48 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s40 = self::source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop50(values, names_2, source_ml_source1_temporary_values48, source_ml_source1_temporary_index48.clone());
                                match source_ml_source2_s40.clone() {
                                    self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Done => {
                                        false
                                    }
                                    self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Return(source_ml_source1_temporary_result50) => {
                                        source_ml_source1_temporary_result50
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

pub fn read_java_script_closed_class_method_target(methods: Vec<self::ml_string::Text>, receiver_fields: Vec<self::ml_string::Text>, method: self::ml_string::Text) -> self::ml_string::Text {
    if ((((methods.len() as f64) % 3f64)) != (0f64)) {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let target = self::ml_string::Text::from_units(vec![]);
            {
                let at = 0f64;
                {
                    let source_ml_source2_s41 = self::source_ml_source1_temporary_read_java_script_closed_class_method_target_loop53(methods, receiver_fields, method.clone(), target.clone(), at);
                    match source_ml_source2_s41.clone() {
                        self::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Result::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Done(target_2) => {
                            target_2.clone()
                        }
                        self::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Result::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Return(source_ml_source1_temporary_result53) => {
                            source_ml_source1_temporary_result53.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn accept_java_script_closed_class_method_definition(brand: self::ml_string::Text, method: self::ml_string::Text, helper: self::ml_string::Text, parameter: self::ml_string::Text, receiver_fields: Vec<self::ml_string::Text>, constructor: bool) -> bool {
    {
        let marker = {
            let source_ml_source2_string_search_operand42 = brand.clone();
            {
                let source_ml_source2_string_search_operand43 = self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95]);
                {
                    let source_ml_source2_string_search_text44 = source_ml_source2_string_search_operand43.clone();
                    {
                        let source_ml_source2_string_search_position45 = f64::NAN;
                        source_ml_source2_string_search_operand42.last_index_of(&source_ml_source2_string_search_text44, source_ml_source2_string_search_position45)
                    }
                }
            }
        };
        if (marker < (0f64)) {
            false
        } else {
            {
                let prefix = {
                    let source_ml_source2_string_range_operand46 = brand.clone();
                    {
                        let source_ml_source2_string_range_operand47 = 0f64;
                        {
                            let source_ml_source2_string_range_operand48 = marker;
                            {
                                let source_ml_source2_string_range_position49 = source_ml_source2_string_range_operand47;
                                {
                                    let source_ml_source2_string_range_position50 = source_ml_source2_string_range_operand48;
                                    source_ml_source2_string_range_operand46.slice(source_ml_source2_string_range_position49, source_ml_source2_string_range_position50)
                                }
                            }
                        }
                    }
                };
                {
                    let ordinal = {
                        let source_ml_source2_string_range_operand51 = brand.clone();
                        {
                            let source_ml_source2_string_range_operand52 = (marker + 15f64);
                            {
                                let source_ml_source2_string_range_position53 = source_ml_source2_string_range_operand52;
                                {
                                    let source_ml_source2_string_range_position54 = f64::INFINITY;
                                    source_ml_source2_string_range_operand51.slice(source_ml_source2_string_range_position53, source_ml_source2_string_range_position54)
                                }
                            }
                        }
                    };
                    ((((((ordinal != (self::ml_string::Text::from_units(vec![]))) && (method != (self::ml_string::Text::from_units(vec![])))) && helper.starts_with(&((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95]))).concat(&ordinal)).concat(&self::ml_string::Text::from_units(vec![95])))) && (parameter == ((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114, 95]))).concat(&ordinal)))) && self::ml_array::includes(&receiver_fields, &brand.clone(), 0f64, false, |a, b| a == b)) && constructor)
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_owner_row(row: Vec<self::ml_string::Text>) -> Vec<f64> {
    if (((row.len() as f64)) < (5f64)) {
        vec![(-1f64)]
    } else {
        {
            let fields = self::ml_string_number::convert_java_script_string_number_units(&((self::ml_array::optional_at(&row, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))).0.iter().copied().map(f64::from).collect::<Vec<f64>>());
            {
                let methods = self::ml_string_number::convert_java_script_string_number_units(&((self::ml_array::optional_at(&row, self::ml_array::number_index(4f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))).0.iter().copied().map(f64::from).collect::<Vec<f64>>());
                if ((((((((!(self::ml_math::is_safe_integer(fields)) || (fields < (0f64))) || (fields > (8192f64))) || (({
                    let source_ml_source2_string_argument56 = fields;
                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument56))
                }) != ((self::ml_array::optional_at(&row, self::ml_array::number_index(3f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))))) || !(self::ml_math::is_safe_integer(methods))) || (methods < (0f64))) || (methods > (8192f64))) || (({
                    let source_ml_source2_string_argument58 = methods;
                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument58))
                }) != ((self::ml_array::optional_at(&row, self::ml_array::number_index(4f64))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))))) || (((row.len() as f64)) != (((5f64 + fields) + (2f64 * methods))))) {
                    vec![(-1f64)]
                } else {
                    vec![0f64, 5f64, fields, (5f64 + fields), methods]
                }
            }
        }
    }
}
