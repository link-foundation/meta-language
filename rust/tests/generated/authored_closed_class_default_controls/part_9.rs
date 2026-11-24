pub fn source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop51(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut method: f64) -> self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result {
    loop {
        return if (method < ((row.len() as f64))) {
            if (((self::ml_array::optional_at(&row, self::ml_array::number_index(method))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) {
                {
                    let at = ((self::ml_array::optional_at(&row, self::ml_array::number_index((method + 3f64)))).clone().unwrap_or_else(|| 0f64) + 1f64);
                    {
                        let source_ml_source2_s269 = self::source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop52(values.clone(), names.clone(), row.clone(), method, at);
                        match source_ml_source2_s269.clone() {
                            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Done => {
                                {
                                    let method_2 = (method + 5f64);
                                    {
                                        method = method_2;
                                        continue;
                                    }
                                }
                            }
                            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop52Return(source_ml_source1_temporary_result52) => {
                                self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Return(source_ml_source1_temporary_result52)
                            }
                        }
                    }
                }
            } else {
                {
                    let method_3 = (method + 5f64);
                    {
                        method = method_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Done
        };
    }
}

pub fn source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop50(mut values: Vec<self::ml_string::Text>, mut names: Vec<self::ml_string::Text>, mut source_ml_source1_temporary_values48: Vec<Vec<f64>>, mut source_ml_source1_temporary_index48: self::ml::Big) -> self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result {
    loop {
        return if (source_ml_source1_temporary_index48 < (self::ml::Big::from_u128(source_ml_source1_temporary_values48.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values48, source_ml_source1_temporary_index48.to_index());
                {
                    let method = (9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
                    {
                        let source_ml_source2_s270 = self::source_ml_source1_temporary_require_java_script_closed_class_prototype_methods_loop51(values.clone(), names.clone(), row, method);
                        match source_ml_source2_s270.clone() {
                            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Done => {
                                {
                                    let source_ml_source1_temporary_index48_2 = source_ml_source1_temporary_index48.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index48 = source_ml_source1_temporary_index48_2.clone();
                                        continue;
                                    }
                                }
                            }
                            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop51Return(source_ml_source1_temporary_result51) => {
                                self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Return(source_ml_source1_temporary_result51)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Result::MlSource1TemporaryRequireJavaScriptClosedClassPrototypeMethodsLoop50Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Result {
    MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Done(self::ml_string::Text),
    MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_method_target_loop53(mut methods: Vec<self::ml_string::Text>, mut receiver_fields: Vec<self::ml_string::Text>, mut method: self::ml_string::Text, mut target: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Result {
    loop {
        return if (at < ((methods.len() as f64))) {
            if (self::ml_array::includes(&receiver_fields, &(self::ml_array::optional_at(&methods, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b) && (((self::ml_array::optional_at(&methods, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == method)) {
                if (target != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Result::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Return(self::ml_string::Text::from_units(vec![]))
                } else {
                    {
                        let target_2 = (self::ml_array::optional_at(&methods, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                        {
                            let at_2 = (at + 3f64);
                            {
                                (target, at) = (target_2.clone(), at_2);
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let at_3 = (at + 3f64);
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Result::MlSource1TemporaryReadJavaScriptClosedClassMethodTargetLoop53Done(target.clone())
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop55(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut syntax: Vec<self::ml_string::Text>, mut at: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let syntax_2 = {
                    let source_ml_source2_concat_value271 = syntax;
                    {
                        let source_ml_source2_concat_value272 = vec![if ((((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) || (((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))) {
                            (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))
                        } else {
                            (self::ml_string::Text::from_units(vec![35])).concat(&(self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                        }];
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value271), source_ml_source2_concat_value272)
                    }
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        (syntax, at) = (syntax_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            syntax
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop61(mut values: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut previous: f64, mut first: f64) -> f64 {
    loop {
        return if ((first < ((values.len() as f64))) && (((self::ml_array::optional_at(&starts, self::ml_array::number_index(first))).clone().unwrap_or_else(|| 0f64)) < previous)) {
            {
                let first_2 = (first + 1f64);
                {
                    first = first_2;
                    continue;
                }
            }
        } else {
            first
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Result {
    MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Done(self::ml_string::Text, Vec<f64>, f64, Vec<self::ml_string::Text>, Vec<self::ml_string::Text>, Vec<self::ml_string::Text>, Vec<Vec<self::ml_string::Text>>),
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop62(mut values: Vec<self::ml_string::Text>, mut prefix: self::ml_string::Text, mut ordinal: f64, mut prototype_methods: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut brand: self::ml_string::Text, mut method: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (method < ((row.len() as f64))) {
            if (((self::ml_array::optional_at(&row, self::ml_array::number_index(method))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) {
                {
                    let prototype_methods_2 = {
                        let source_ml_source2_concat_value275 = prototype_methods;
                        {
                            let source_ml_source2_concat_value276 = vec![brand.clone(), (self::ml_array::optional_at(&values, self::ml_array::number_index((self::ml_array::optional_at(&row, self::ml_array::number_index(method))).clone().unwrap_or_else(|| 0f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), (((prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95]))).concat(&{
                                let source_ml_source2_string_argument273 = ordinal;
                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument273))
                            })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                let source_ml_source2_string_argument274 = method;
                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument274))
                            })];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value275), source_ml_source2_concat_value276)
                        }
                    };
                    {
                        let method_2 = (method + 5f64);
                        {
                            (prototype_methods, method) = (prototype_methods_2, method_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let method_3 = (method + 5f64);
                    {
                        method = method_3;
                        continue;
                    }
                }
            }
        } else {
            prototype_methods
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop63(mut values: Vec<self::ml_string::Text>, mut public_fields: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut brand: self::ml_string::Text, mut field: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (field < ((9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64))))) {
            {
                let position = (self::ml_array::optional_at(&row, self::ml_array::number_index(field))).clone().unwrap_or_else(|| 0f64);
                {
                    let public_fields_2 = {
                        let source_ml_source2_concat_value277 = public_fields;
                        {
                            let source_ml_source2_concat_value278 = vec![brand.clone(), (self::ml_array::optional_at(&values, self::ml_array::number_index(if (position < (0f64)) {
                                ((-position) - 1f64)
                            } else {
                                position
                            }))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value277), source_ml_source2_concat_value278)
                        }
                    };
                    {
                        let field_2 = (field + 3f64);
                        {
                            (public_fields, field) = (public_fields_2, field_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            public_fields
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop64(mut values: Vec<self::ml_string::Text>, mut row: Vec<f64>, mut own_fields: Vec<self::ml_string::Text>, mut field: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (field < ((9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64))))) {
            {
                let position = (self::ml_array::optional_at(&row, self::ml_array::number_index(field))).clone().unwrap_or_else(|| 0f64);
                {
                    let name = (self::ml_array::optional_at(&values, self::ml_array::number_index(if (position < (0f64)) {
                        ((-position) - 1f64)
                    } else {
                        position
                    }))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if !(self::ml_array::includes(&own_fields, &name.clone(), 0f64, false, |a, b| a == b)) {
                        {
                            let own_fields_2 = {
                                let source_ml_source2_concat_value279 = own_fields;
                                {
                                    let source_ml_source2_concat_value280 = vec![name.clone()];
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value279), source_ml_source2_concat_value280)
                                }
                            };
                            {
                                let field_2 = (field + 3f64);
                                {
                                    (own_fields, field) = (own_fields_2, field_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let field_3 = (field + 3f64);
                            {
                                field = field_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            own_fields
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop65(mut values: Vec<self::ml_string::Text>, mut prefix: self::ml_string::Text, mut ordinal: f64, mut prototype: bool, mut row: Vec<f64>, mut own_methods: Vec<self::ml_string::Text>, mut method: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (method < ((row.len() as f64))) {
            if (((self::ml_array::optional_at(&row, self::ml_array::number_index(method))).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) {
                {
                    let own_methods_2 = {
                        let source_ml_source2_concat_value283 = own_methods;
                        {
                            let source_ml_source2_concat_value284 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index((self::ml_array::optional_at(&row, self::ml_array::number_index(method))).clone().unwrap_or_else(|| 0f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), (((prefix.concat(&if prototype {
                                self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95])
                            } else {
                                self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 95])
                            })).concat(&{
                                let source_ml_source2_string_argument281 = ordinal;
                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument281))
                            })).concat(&self::ml_string::Text::from_units(vec![95]))).concat(&{
                                let source_ml_source2_string_argument282 = method;
                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument282))
                            })];
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value283), source_ml_source2_concat_value284)
                        }
                    };
                    {
                        let method_2 = (method + 5f64);
                        {
                            (own_methods, method) = (own_methods_2, method_2);
                            continue;
                        }
                    }
                }
            } else {
                {
                    let method_3 = (method + 5f64);
                    {
                        method = method_3;
                        continue;
                    }
                }
            }
        } else {
            own_methods
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop60(mut source: self::ml_string::Text, mut values: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut prefix: self::ml_string::Text, mut names: Vec<self::ml_string::Text>, mut static_calls: Vec<self::ml_string::Text>, mut text: self::ml_string::Text, mut intervals: Vec<f64>, mut previous: f64, mut ordinal: f64, mut brands: Vec<self::ml_string::Text>, mut public_fields: Vec<self::ml_string::Text>, mut prototype_methods: Vec<self::ml_string::Text>, mut owner_rows: Vec<Vec<self::ml_string::Text>>, mut prototype: bool, mut source_ml_source1_temporary_values54: Vec<Vec<f64>>, mut source_ml_source1_temporary_index54: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Result {
    loop {
        return if (source_ml_source1_temporary_index54 < (self::ml::Big::from_u128(source_ml_source1_temporary_values54.len() as u128))) {
            {
                let row = self::ml_array::at(&source_ml_source1_temporary_values54, source_ml_source1_temporary_index54.to_index());
                {
                    let first = (self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| 0f64);
                    {
                        let last = (self::ml_array::optional_at(&row, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| 0f64);
                        {
                            let before = {
                                let source_ml_source2_string_range_operand285 = source.clone();
                                {
                                    let source_ml_source2_string_range_operand286 = previous;
                                    {
                                        let source_ml_source2_string_range_operand287 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(first))).clone().unwrap_or_else(|| previous);
                                        {
                                            let source_ml_source2_string_range_position288 = source_ml_source2_string_range_operand286;
                                            {
                                                let source_ml_source2_string_range_position289 = source_ml_source2_string_range_operand287;
                                                source_ml_source2_string_range_operand285.slice(source_ml_source2_string_range_position288, source_ml_source2_string_range_position289)
                                            }
                                        }
                                    }
                                }
                            };
                            {
                                let intervals_2 = {
                                    let source_ml_source2_concat_value290 = intervals;
                                    {
                                        let source_ml_source2_concat_value291 = vec![text.length(), (text.length() + before.length()), previous, (self::ml_array::optional_at(&starts, self::ml_array::number_index(first))).clone().unwrap_or_else(|| previous)];
                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value290), source_ml_source2_concat_value291)
                                    }
                                };
                                {
                                    let text_2 = text.concat(&before);
                                    {
                                        let replacement = self::render_java_script_closed_class(source.clone(), values.clone(), starts.clone(), ends.clone(), row.clone(), prefix.clone(), ordinal, names.clone(), static_calls.clone(), prototype);
                                        {
                                            let intervals_3 = {
                                                let source_ml_source2_concat_value292 = intervals_2;
                                                {
                                                    let source_ml_source2_concat_value293 = vec![text_2.length(), (text_2.length() + replacement.length()), (self::ml_array::optional_at(&starts, self::ml_array::number_index(first))).clone().unwrap_or_else(|| 0f64), (self::ml_array::optional_at(&ends, self::ml_array::number_index(last))).clone().unwrap_or_else(|| 0f64)];
                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value292), source_ml_source2_concat_value293)
                                                }
                                            };
                                            {
                                                let text_3 = text_2.concat(&replacement);
                                                {
                                                    let previous_2 = (self::ml_array::optional_at(&ends, self::ml_array::number_index(last))).clone().unwrap_or_else(|| previous);
                                                    {
                                                        let brand = (prefix.concat(&self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95]))).concat(&{
                                                            let source_ml_source2_string_argument294 = ordinal;
                                                            self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument294))
                                                        });
                                                        {
                                                            let brands_2 = {
                                                                let source_ml_source2_concat_value295 = brands;
                                                                {
                                                                    let source_ml_source2_concat_value296 = vec![brand.clone()];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value295), source_ml_source2_concat_value296)
                                                                }
                                                            };
                                                            {
                                                                let prototype_methods_3 = if prototype {
                                                                    {
                                                                        let method = (9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
                                                                        {
                                                                            let prototype_methods_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop62(values.clone(), prefix.clone(), ordinal, prototype_methods, row.clone(), brand.clone(), method);
                                                                            prototype_methods_2
                                                                        }
                                                                    }
                                                                } else {
                                                                    prototype_methods
                                                                };
                                                                {
                                                                    let field = 9f64;
                                                                    {
                                                                        let public_fields_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop63(values.clone(), public_fields, row.clone(), brand.clone(), field);
                                                                        {
                                                                            let own_fields = Vec::<self::ml_string::Text>::new();
                                                                            {
                                                                                let own_methods = Vec::<self::ml_string::Text>::new();
                                                                                {
                                                                                    let field_2 = 9f64;
                                                                                    {
                                                                                        let own_fields_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop64(values.clone(), row.clone(), own_fields, field_2);
                                                                                        {
                                                                                            let method_2 = (9f64 + (3f64 * (self::ml_array::optional_at(&row, self::ml_array::number_index(7f64))).clone().unwrap_or_else(|| 0f64)));
                                                                                            {
                                                                                                let own_methods_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop65(values.clone(), prefix.clone(), ordinal, prototype, row, own_methods, method_2);
                                                                                                {
                                                                                                    let owner_rows_2 = {
                                                                                                        let source_ml_source2_concat_value302 = owner_rows;
                                                                                                        {
                                                                                                            let source_ml_source2_concat_value303 = vec![{
                                                                                                                let source_ml_source2_concat_value299 = vec![(self::ml_array::optional_at(&values, self::ml_array::number_index((first + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), brand.clone(), if prototype {
                                                                                                                    self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 104, 101, 108, 112, 101, 114, 115])
                                                                                                                } else {
                                                                                                                    self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115])
                                                                                                                }, {
                                                                                                                    let source_ml_source2_string_argument297 = (own_fields_2.len() as f64);
                                                                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument297))
                                                                                                                }, {
                                                                                                                    let source_ml_source2_string_argument298 = ((own_methods_2.len() as f64) / 2f64);
                                                                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument298))
                                                                                                                }];
                                                                                                                {
                                                                                                                    let source_ml_source2_concat_value300 = own_fields_2;
                                                                                                                    {
                                                                                                                        let source_ml_source2_concat_value301 = own_methods_2;
                                                                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value299), source_ml_source2_concat_value300), source_ml_source2_concat_value301)
                                                                                                                    }
                                                                                                                }
                                                                                                            }];
                                                                                                            self::ml_array::append(self::ml_array::append(Vec::<Vec<self::ml_string::Text>>::new(), source_ml_source2_concat_value302), source_ml_source2_concat_value303)
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let ordinal_2 = (ordinal + 1f64);
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index54_2 = source_ml_source1_temporary_index54.add(&self::ml::Big::from_i128(1));
                                                                                                            {
                                                                                                                (text, intervals, previous, ordinal, brands, public_fields, prototype_methods, owner_rows, source_ml_source1_temporary_index54) = (text_3.clone(), intervals_3, previous_2, ordinal_2, brands_2, public_fields_2, prototype_methods_3, owner_rows_2, source_ml_source1_temporary_index54_2.clone());
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
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Done(text.clone(), intervals, previous, brands, public_fields, prototype_methods, owner_rows)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59 {
    MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58 {
    MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57 {
    MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Result {
    MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Done(f64, f64),
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop66(mut source: self::ml_string::Text, mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut names: Vec<self::ml_string::Text>, mut error: f64, mut error_at: f64, mut at: f64) -> self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Result {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let previous = (self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if ((((previous != (self::ml_string::Text::from_units(vec![46]))) && (previous != (self::ml_string::Text::from_units(vec![63, 46])))) && ((((value == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) || (value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115])))) || ((value == (self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99]))) && (((self::ml_array::optional_at(&kinds, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))))) || ((value == (self::ml_string::Text::from_units(vec![110, 101, 119]))) && self::ml_array::includes(&names, &(self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)))) && (({
                        let source_ml_source2_string_range_operand309 = source.clone();
                        {
                            let source_ml_source2_string_range_operand310 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                            {
                                let source_ml_source2_string_range_operand311 = (self::ml_array::optional_at(&ends, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                                {
                                    let source_ml_source2_string_range_position312 = source_ml_source2_string_range_operand310;
                                    {
                                        let source_ml_source2_string_range_position313 = source_ml_source2_string_range_operand311;
                                        source_ml_source2_string_range_operand309.slice(source_ml_source2_string_range_position312, source_ml_source2_string_range_position313)
                                    }
                                }
                            }
                        }
                    }) != value)) {
                        {
                            let error_2 = (-9f64);
                            {
                                let error_at_2 = at;
                                self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Done(error_2, error_at_2)
                            }
                        }
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
            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Done(error, error_at)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result {
    MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(Vec<Vec<f64>>, Vec<self::ml_string::Text>, f64, f64),
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop67(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut end: f64) -> f64 {
    loop {
        return if ((end < ((values.len() as f64))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index(end))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![59])))) {
            if (((self::ml_array::optional_at(&kinds, self::ml_array::number_index(end))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                end
            } else {
                {
                    let end_2 = (end + 1f64);
                    {
                        end = end_2;
                        continue;
                    }
                }
            }
        } else {
            end
        };
    }
}

pub fn source_ml_source1_temporary_lower_java_script_closed_class_source_loop56(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut directory: Vec<Vec<f64>>, mut names: Vec<self::ml_string::Text>, mut depth: f64, mut declarations_finished: bool, mut error: f64, mut error_at: f64, mut at: f64) -> self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result {
    loop {
        return if ((at < ((values.len() as f64))) && (error == (0f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if (((((value == (self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115]))) && (((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![63, 46])))) && ((((self::ml_array::optional_at(&kinds, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) || (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![123]))))) {
                    if (((directory.len() as f64)) >= (64f64)) {
                        {
                            let error_2 = (-8f64);
                            {
                                let error_at_2 = at;
                                self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_2, error_at_2)
                            }
                        }
                    } else {
                        if ((depth != (0f64)) || declarations_finished) {
                            {
                                let error_3 = (-7f64);
                                {
                                    let error_at_3 = at;
                                    self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_3, error_at_3)
                                }
                            }
                        } else {
                            {
                                let row = self::read_java_script_closed_class_directory(values.clone(), kinds.clone(), at);
                                if (((self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| (-1f64))) < (0f64)) {
                                    {
                                        let error_4 = (self::ml_array::optional_at(&row, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| (-1f64));
                                        {
                                            let error_at_4 = (self::ml_array::optional_at(&row, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| at);
                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_4, error_at_4)
                                        }
                                    }
                                } else {
                                    {
                                        let name = (self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                                        if self::ml_array::includes(&names, &name.clone(), 0f64, false, |a, b| a == b) {
                                            {
                                                let error_5 = (-7f64);
                                                {
                                                    let error_at_5 = (at + 1f64);
                                                    self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_5, error_at_5)
                                                }
                                            }
                                        } else {
                                            {
                                                let names_2 = {
                                                    let source_ml_source2_concat_value314 = names;
                                                    {
                                                        let source_ml_source2_concat_value315 = vec![name.clone()];
                                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value314), source_ml_source2_concat_value315)
                                                    }
                                                };
                                                {
                                                    let directory_2 = {
                                                        let source_ml_source2_concat_value316 = directory;
                                                        {
                                                            let source_ml_source2_concat_value317 = vec![row.clone()];
                                                            self::ml_array::append(self::ml_array::append(Vec::<Vec<f64>>::new(), source_ml_source2_concat_value316), source_ml_source2_concat_value317)
                                                        }
                                                    };
                                                    {
                                                        let at_2 = (self::ml_array::optional_at(&row, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| at);
                                                        {
                                                            let at_3 = (at_2 + 1f64);
                                                            {
                                                                (directory, names, at) = (directory_2, names_2, at_3);
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
                } else {
                    if ((depth == (0f64)) && (value == (self::ml_string::Text::from_units(vec![105, 109, 112, 111, 114, 116])))) {
                        {
                            let end = at;
                            {
                                let end_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop67(values.clone(), kinds.clone(), end);
                                if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![35, 110, 111, 100, 101, 58, 97, 115, 115, 101, 114, 116]), self::ml_string::Text::from_units(vec![35, 110, 111, 100, 101, 58, 97, 115, 115, 101, 114, 116, 47, 115, 116, 114, 105, 99, 116]), self::ml_string::Text::from_units(vec![35, 97, 115, 115, 101, 114, 116]), self::ml_string::Text::from_units(vec![35, 97, 115, 115, 101, 114, 116, 47, 115, 116, 114, 105, 99, 116])], &(self::ml_array::optional_at(&values, self::ml_array::number_index(end_2))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) {
                                    {
                                        let error_6 = (-7f64);
                                        {
                                            let error_at_6 = at;
                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error_6, error_at_6)
                                        }
                                    }
                                } else {
                                    {
                                        let at_4 = end_2;
                                        {
                                            let at_5 = (at_4 + 1f64);
                                            {
                                                at = at_5;
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let declarations_finished_3 = if (((depth == (0f64)) && (value != (self::ml_string::Text::from_units(vec![59])))) && (value != (self::ml_string::Text::from_units(vec![35, 117, 115, 101, 32, 115, 116, 114, 105, 99, 116])))) {
                                {
                                    let declarations_finished_2 = true;
                                    declarations_finished_2
                                }
                            } else {
                                declarations_finished
                            };
                            {
                                let depth_3 = if (((value == (self::ml_string::Text::from_units(vec![40]))) || (value == (self::ml_string::Text::from_units(vec![91])))) || (value == (self::ml_string::Text::from_units(vec![123])))) {
                                    {
                                        let depth_2 = (depth + 1f64);
                                        depth_2
                                    }
                                } else {
                                    depth
                                };
                                if (((value == (self::ml_string::Text::from_units(vec![41]))) || (value == (self::ml_string::Text::from_units(vec![93])))) || (value == (self::ml_string::Text::from_units(vec![125])))) {
                                    {
                                        let depth_4 = (depth_3 - 1f64);
                                        {
                                            let at_6 = (at + 1f64);
                                            {
                                                (depth, declarations_finished, at) = (depth_4, declarations_finished_3, at_6);
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at_7 = (at + 1f64);
                                        {
                                            (depth, declarations_finished, at) = (depth_3, declarations_finished_3, at_7);
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
            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory, names, error, error_at)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result {
    MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Done,
    MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Return(bool),
}

pub fn source_ml_source1_temporary_accept_java_script_closed_class_own_field_loop68(mut receiver_fields: Vec<self::ml_string::Text>, mut public_fields: Vec<self::ml_string::Text>, mut field: self::ml_string::Text, mut at: f64) -> self::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result {
    loop {
        return if (at < ((public_fields.len() as f64))) {
            if (self::ml_array::includes(&receiver_fields, &(self::ml_array::optional_at(&public_fields, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b) && (((self::ml_array::optional_at(&public_fields, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == field)) {
                self::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Return(true)
            } else {
                {
                    let at_2 = (at + 2f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Result {
    MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Done(f64, f64),
}

pub fn source_ml_source1_temporary_project_java_script_class_source_interval_loop69(mut intervals: Vec<f64>, mut first: f64, mut last: f64, mut low: f64, mut high: f64, mut at: f64) -> self::MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Result {
    loop {
        return if (at < ((intervals.len() as f64))) {
            {
                let start = (self::ml_array::optional_at(&intervals, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let end = (self::ml_array::optional_at(&intervals, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| 0f64);
                    if (((end < first) || (start > last)) || (start == end)) {
                        {
                            let at_2 = (at + 4f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            let original_start = (self::ml_array::optional_at(&intervals, self::ml_array::number_index((at + 2f64)))).clone().unwrap_or_else(|| 0f64);
                            {
                                let original_end = (self::ml_array::optional_at(&intervals, self::ml_array::number_index((at + 3f64)))).clone().unwrap_or_else(|| 0f64);
                                {
                                    let low_2 = if (low < (0f64)) {
                                        original_start
                                    } else {
                                        self::ml_math::min(low, original_start)
                                    };
                                    {
                                        let high_2 = self::ml_math::max(high, original_end);
                                        {
                                            let at_3 = (at + 4f64);
                                            {
                                                (low, high, at) = (low_2, high_2, at_3);
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
        } else {
            self::MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Result::MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Done(low, high)
        };
    }
}

fn ml_main() {
    assert!(((self::read_java_script_closed_class_parameter_positions(vec![self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![61]), self::ml_string::Text::from_units(vec![110, 101, 119]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![41]), self::ml_string::Text::from_units(vec![41])], vec![self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])], 0f64, 7f64)) == (vec![(-1f64), 3f64])), "assertion 1");
    assert!(((self::read_java_script_closed_class_collection_parameter_positions(vec![self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![61]), self::ml_string::Text::from_units(vec![110, 101, 119]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![41]), self::ml_string::Text::from_units(vec![41])], vec![self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])], 0f64, 7f64)) == (vec![1f64])), "assertion 2");
    assert!(((self::read_java_script_closed_class_collection_parameter_positions(vec![self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![61]), self::ml_string::Text::from_units(vec![110, 101, 119]), self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![41]), self::ml_string::Text::from_units(vec![41])], vec![self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])], 0f64, 7f64)) == (vec![1f64])), "assertion 3");
    assert!(((self::read_java_script_closed_class_collection_parameter_positions(vec![self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![61]), self::ml_string::Text::from_units(vec![110, 101, 119]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![40]), self::ml_string::Text::from_units(vec![105, 116, 101, 109, 115]), self::ml_string::Text::from_units(vec![41]), self::ml_string::Text::from_units(vec![41])], vec![self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]), self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])], 0f64, 8f64)) == (vec![(-1f64), 3f64])), "assertion 4");
    assert!(((self::accept_java_script_closed_class_own_field(vec![self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121, 48]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116])], vec![self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121, 48]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116])], self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116]))) == (true)), "assertion 5");
    assert!(((self::accept_java_script_closed_class_own_field(vec![self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121, 49]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116])], vec![self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121, 48]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116])], self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116]))) == (false)), "assertion 6");
    assert!(((self::accept_java_script_closed_class_own_field(vec![self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104])], vec![self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121, 48]), self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116])], self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]))) == (false)), "assertion 7");
    assert!(((self::read_java_script_closed_class_own_field_use_error(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]), self::ml_string::Text::from_units(vec![]), true, false, true, false, true)) == (self::ml_string::Text::from_units(vec![]))), "assertion 8");
    assert!(((self::read_java_script_closed_class_use_error(self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]), self::ml_string::Text::from_units(vec![]), true, false, true, false)) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 105, 110, 115, 116, 97, 110, 99, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 114, 101, 99, 116, 32, 112, 117, 98, 108, 105, 99, 32, 102, 105, 101, 108, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 97, 110, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 45, 114, 101, 116, 97, 105, 110, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 32, 99, 97, 108, 108, 115, 59, 32, 112, 114, 105, 118, 97, 116, 101, 32, 115, 108, 111, 116, 115, 44, 32, 100, 101, 116, 97, 99, 104, 101, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 115, 32, 97, 110, 100, 32, 105, 110, 100, 105, 114, 101, 99, 116, 32, 114, 101, 102, 108, 101, 99, 116, 105, 118, 101, 32, 108, 111, 111, 107, 117, 112, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 32, 102, 117, 108, 108, 32, 111, 98, 106, 101, 99, 116, 32, 109, 111, 100, 101, 108]))), "assertion 9");
    assert!(((self::read_java_script_closed_class_method_target(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 48]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 49]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 49])], vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]))) == (self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 48]))), "assertion 10");
    assert!(((self::read_java_script_closed_class_method_target(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 48]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 49]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 49])], vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 49]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]))) == (self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 49]))), "assertion 11");
    assert!(((self::read_java_script_closed_class_method_target(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 48])], vec![self::ml_string::Text::from_units(vec![100, 105, 102, 102, 101, 114, 101, 110, 116]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 12");
    assert!(((self::read_java_script_closed_class_method_target(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 48])], vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], self::ml_string::Text::from_units(vec![100, 101, 116, 97, 99, 104, 101, 100]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 13");
    assert!(((self::read_java_script_closed_class_method_target(vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 48]), self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![104, 101, 108, 112, 101, 114, 49])], vec![self::ml_string::Text::from_units(vec![98, 114, 97, 110, 100, 48])], self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 14");
    assert!(((self::accept_java_script_closed_class_method_definition(self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 48, 95, 57]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114, 95, 48]), vec![self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], true)) == (true)), "assertion 15");
    assert!(((self::accept_java_script_closed_class_method_definition(self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 49, 95, 57]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114, 95, 48]), vec![self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], true)) == (false)), "assertion 16");
    assert!(((self::accept_java_script_closed_class_method_definition(self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 48, 95, 57]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114, 95, 48]), vec![self::ml_string::Text::from_units(vec![117, 115, 101, 100])], true)) == (false)), "assertion 17");
    assert!(((self::accept_java_script_closed_class_method_definition(self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 109, 101, 116, 104, 111, 100, 95, 48, 95, 57]), self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 114, 101, 99, 101, 105, 118, 101, 114, 95, 48]), vec![self::ml_string::Text::from_units(vec![109, 108, 95, 99, 108, 97, 115, 115, 95, 105, 100, 101, 110, 116, 105, 116, 121, 95, 48]), self::ml_string::Text::from_units(vec![117, 115, 101, 100])], false)) == (false)), "assertion 18");
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
fn all_literal_closed_class_default_controls() { main(); }
