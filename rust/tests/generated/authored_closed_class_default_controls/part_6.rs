pub fn lower_java_script_closed_class_source(source: self::ml_string::Text, values: Vec<self::ml_string::Text>, kinds: Vec<self::ml_string::Text>, starts: Vec<f64>, ends: Vec<f64>, prefix: self::ml_string::Text) -> (f64, Vec<self::ml_string::Text>, bool, self::ml_string::Text, Vec<f64>, Vec<Vec<self::ml_string::Text>>, Vec<self::ml_string::Text>, Vec<self::ml_string::Text>, self::ml_string::Text) {
    if (((!(self::accept_java_script_closed_class_source_size(source.length(), (values.len() as f64))) || (((kinds.len() as f64)) != ((values.len() as f64)))) || (((starts.len() as f64)) != ((values.len() as f64)))) || (((ends.len() as f64)) != ((values.len() as f64)))) {
        {
            let source_ml_source2_field59 = source.clone();
            {
                let source_ml_source2_field60 = Vec::<f64>::new();
                {
                    let source_ml_source2_field61 = Vec::<self::ml_string::Text>::new();
                    {
                        let source_ml_source2_field62 = Vec::<self::ml_string::Text>::new();
                        {
                            let source_ml_source2_field63 = Vec::<self::ml_string::Text>::new();
                            {
                                let source_ml_source2_field64 = Vec::<Vec<self::ml_string::Text>>::new();
                                {
                                    let source_ml_source2_field65 = self::read_java_script_closed_class_refusal((-8f64));
                                    {
                                        let source_ml_source2_field66 = 0f64;
                                        {
                                            let source_ml_source2_field67 = false;
                                            (source_ml_source2_field66, source_ml_source2_field61, source_ml_source2_field67, source_ml_source2_field65.clone(), source_ml_source2_field60, source_ml_source2_field64, source_ml_source2_field63, source_ml_source2_field62, source_ml_source2_field59.clone())
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
        {
            let syntax = Vec::<self::ml_string::Text>::new();
            {
                let at = 0f64;
                {
                    let syntax_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop55(values, kinds.clone(), syntax, at);
                    {
                        let values_2 = syntax_2;
                        {
                            let directory = Vec::<Vec<f64>>::new();
                            {
                                let names = Vec::<self::ml_string::Text>::new();
                                {
                                    let depth = 0f64;
                                    {
                                        let declarations_finished = false;
                                        {
                                            let error = 0f64;
                                            {
                                                let error_at = 0f64;
                                                {
                                                    let error_3 = if ((((((source.length()) > (131072f64)) || (((values_2.len() as f64)) > (8192f64))) || (((kinds.len() as f64)) != ((values_2.len() as f64)))) || (((starts.len() as f64)) != ((values_2.len() as f64)))) || (((ends.len() as f64)) != ((values_2.len() as f64)))) {
                                                        {
                                                            let error_2 = (-8f64);
                                                            error_2
                                                        }
                                                    } else {
                                                        error
                                                    };
                                                    {
                                                        let at_2 = 0f64;
                                                        {
                                                            let source_ml_source2_s68 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop56(values_2.clone(), kinds.clone(), directory, names, depth, declarations_finished, error_3, error_at, at_2);
                                                            match source_ml_source2_s68.clone() {
                                                                self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop56Done(directory_2, names_2, error_4, error_at_2) => {
                                                                    {
                                                                        let source_ml_source2_s70 = if ((error_4 == (0f64)) && (((directory_2.len() as f64)) > (0f64))) {
                                                                            {
                                                                                let at_3 = 0f64;
                                                                                {
                                                                                    let source_ml_source2_s71 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop66(source.clone(), values_2.clone(), kinds.clone(), starts.clone(), ends.clone(), names_2.clone(), error_4, error_at_2, at_3);
                                                                                    match source_ml_source2_s71.clone() {
                                                                                        self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop66Done(error_5, error_at_3) => {
                                                                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57Next(error_5, error_at_3)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57Next(error_4, error_at_2)
                                                                        };
                                                                        match source_ml_source2_s70.clone() {
                                                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin57Next(error_6, error_at_4) => {
                                                                                {
                                                                                    let static_calls = self::read_java_script_closed_class_static_calls(values_2.clone(), directory_2.clone(), prefix.clone());
                                                                                    {
                                                                                        let source_ml_source2_s72 = if ((error_6 == (0f64)) && (((directory_2.len() as f64)) > (0f64))) {
                                                                                            {
                                                                                                let owner = self::read_java_script_closed_class_collection_default_class_owner_error(values_2.clone(), directory_2.clone());
                                                                                                if (owner >= (0f64)) {
                                                                                                    {
                                                                                                        let error_7 = (-11f64);
                                                                                                        {
                                                                                                            let error_at_5 = owner;
                                                                                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58Next(error_7, error_at_5)
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58Next(error_6, error_at_4)
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58Next(error_6, error_at_4)
                                                                                        };
                                                                                        match source_ml_source2_s72.clone() {
                                                                                            self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin58Next(error_8, error_at_6) => {
                                                                                                {
                                                                                                    let source_ml_source2_s73 = if ((error_8 == (0f64)) && (((directory_2.len() as f64)) > (0f64))) {
                                                                                                        {
                                                                                                            let observation = self::read_java_script_closed_class_observation_error(values_2.clone(), kinds, directory_2.clone(), static_calls.clone());
                                                                                                            if (observation >= (0f64)) {
                                                                                                                {
                                                                                                                    let error_9 = (-7f64);
                                                                                                                    {
                                                                                                                        let error_at_7 = observation;
                                                                                                                        self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59Next(error_9, error_at_7)
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59Next(error_8, error_at_6)
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59Next(error_8, error_at_6)
                                                                                                    };
                                                                                                    match source_ml_source2_s73.clone() {
                                                                                                        self::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59::MlSource1TemporaryLowerJavaScriptClosedClassSourceJoin59Next(error_10, error_at_8) => {
                                                                                                            if (error_10 != (0f64)) {
                                                                                                                {
                                                                                                                    let source_ml_source2_field74 = source.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_field75 = Vec::<f64>::new();
                                                                                                                        {
                                                                                                                            let source_ml_source2_field76 = Vec::<self::ml_string::Text>::new();
                                                                                                                            {
                                                                                                                                let source_ml_source2_field77 = Vec::<self::ml_string::Text>::new();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_field78 = Vec::<self::ml_string::Text>::new();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_field79 = Vec::<Vec<self::ml_string::Text>>::new();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_field80 = self::read_java_script_closed_class_refusal(error_10);
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_field81 = error_at_8;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_field82 = false;
                                                                                                                                                    (source_ml_source2_field81, source_ml_source2_field76, source_ml_source2_field82, source_ml_source2_field80.clone(), source_ml_source2_field75, source_ml_source2_field79, source_ml_source2_field78, source_ml_source2_field77, source_ml_source2_field74.clone())
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
                                                                                                                if (((directory_2.len() as f64)) == (0f64)) {
                                                                                                                    {
                                                                                                                        let source_ml_source2_field83 = source.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_field84 = Vec::<f64>::new();
                                                                                                                            {
                                                                                                                                let source_ml_source2_field85 = Vec::<self::ml_string::Text>::new();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_field86 = Vec::<self::ml_string::Text>::new();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_field87 = Vec::<self::ml_string::Text>::new();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_field88 = Vec::<Vec<self::ml_string::Text>>::new();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_field89 = self::ml_string::Text::from_units(vec![]);
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_field90 = 0f64;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_field91 = false;
                                                                                                                                                        (source_ml_source2_field90, source_ml_source2_field85, source_ml_source2_field91, source_ml_source2_field89.clone(), source_ml_source2_field84, source_ml_source2_field88, source_ml_source2_field87, source_ml_source2_field86, source_ml_source2_field83.clone())
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
                                                                                                                    {
                                                                                                                        let text = self::ml_string::Text::from_units(vec![]);
                                                                                                                        {
                                                                                                                            let intervals = Vec::<f64>::new();
                                                                                                                            {
                                                                                                                                let previous = 0f64;
                                                                                                                                {
                                                                                                                                    let ordinal = 0f64;
                                                                                                                                    {
                                                                                                                                        let brands = Vec::<self::ml_string::Text>::new();
                                                                                                                                        {
                                                                                                                                            let public_fields = Vec::<self::ml_string::Text>::new();
                                                                                                                                            {
                                                                                                                                                let prototype_methods = Vec::<self::ml_string::Text>::new();
                                                                                                                                                {
                                                                                                                                                    let owner_rows = Vec::<Vec<self::ml_string::Text>>::new();
                                                                                                                                                    {
                                                                                                                                                        let prototype = self::require_java_script_closed_class_prototype_methods(values_2.clone(), directory_2.clone());
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source1_temporary_values54 = directory_2;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source1_temporary_index54 = self::ml::Big::from_i128(0);
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_s92 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop60(source.clone(), values_2.clone(), starts.clone(), ends.clone(), prefix.clone(), names_2.clone(), static_calls.clone(), text.clone(), intervals, previous, ordinal, brands, public_fields, prototype_methods, owner_rows, prototype, source_ml_source1_temporary_values54, source_ml_source1_temporary_index54.clone());
                                                                                                                                                                    match source_ml_source2_s92.clone() {
                                                                                                                                                                        self::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Result::MlSource1TemporaryLowerJavaScriptClosedClassSourceLoop60Done(text_2, intervals_2, previous_2, brands_2, public_fields_2, prototype_methods_2, owner_rows_2) => {
                                                                                                                                                                            {
                                                                                                                                                                                let first = 0f64;
                                                                                                                                                                                {
                                                                                                                                                                                    let first_2 = self::source_ml_source1_temporary_lower_java_script_closed_class_source_loop61(values_2.clone(), starts.clone(), previous_2, first);
                                                                                                                                                                                    {
                                                                                                                                                                                        let rest = if (first_2 >= ((values_2.len() as f64))) {
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_source2_string_range_operand93 = source.clone();
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source2_string_range_operand94 = previous_2;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source2_string_range_position95 = source_ml_source2_string_range_operand94;
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source2_string_range_position96 = f64::INFINITY;
                                                                                                                                                                                                            source_ml_source2_string_range_operand93.slice(source_ml_source2_string_range_position95, source_ml_source2_string_range_position96)
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        } else {
                                                                                                                                                                                            (({
                                                                                                                                                                                                let source_ml_source2_string_range_operand97 = source.clone();
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source2_string_range_operand98 = previous_2;
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source2_string_range_operand99 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(first_2))).clone().unwrap_or_else(|| previous_2);
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source2_string_range_position100 = source_ml_source2_string_range_operand98;
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let source_ml_source2_string_range_position101 = source_ml_source2_string_range_operand99;
                                                                                                                                                                                                                source_ml_source2_string_range_operand97.slice(source_ml_source2_string_range_position100, source_ml_source2_string_range_position101)
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }).concat(&self::render_java_script_closed_class_range(source.clone(), values_2.clone(), starts, ends.clone(), first_2, (values_2.len() as f64), self::ml_string::Text::from_units(vec![]), names_2, static_calls))).concat(&{
                                                                                                                                                                                                let source_ml_source2_string_range_operand102 = source.clone();
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source2_string_range_operand103 = (self::ml_array::optional_at(&ends, self::ml_array::number_index(((values_2.len() as f64) - 1f64)))).clone().unwrap_or_else(|| source.length());
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source2_string_range_position104 = source_ml_source2_string_range_operand103;
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source2_string_range_position105 = f64::INFINITY;
                                                                                                                                                                                                            source_ml_source2_string_range_operand102.slice(source_ml_source2_string_range_position104, source_ml_source2_string_range_position105)
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            })
                                                                                                                                                                                        };
                                                                                                                                                                                        {
                                                                                                                                                                                            let intervals_3 = {
                                                                                                                                                                                                let source_ml_source2_concat_value106 = intervals_2;
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_source2_concat_value107 = vec![text_2.length(), (text_2.length() + rest.length()), previous_2, source.length()];
                                                                                                                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value106), source_ml_source2_concat_value107)
                                                                                                                                                                                                }
                                                                                                                                                                                            };
                                                                                                                                                                                            {
                                                                                                                                                                                                let text_3 = text_2.concat(&rest);
                                                                                                                                                                                                if ((text_3.length()) > (1048576f64)) {
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source2_field108 = source.clone();
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source2_field109 = Vec::<f64>::new();
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let source_ml_source2_field110 = Vec::<self::ml_string::Text>::new();
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let source_ml_source2_field111 = Vec::<self::ml_string::Text>::new();
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let source_ml_source2_field112 = Vec::<self::ml_string::Text>::new();
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            let source_ml_source2_field113 = Vec::<Vec<self::ml_string::Text>>::new();
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let source_ml_source2_field114 = self::read_java_script_closed_class_refusal((-8f64));
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    let source_ml_source2_field115 = 0f64;
                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                        let source_ml_source2_field116 = false;
                                                                                                                                                                                                                                        (source_ml_source2_field115, source_ml_source2_field110, source_ml_source2_field116, source_ml_source2_field114.clone(), source_ml_source2_field109, source_ml_source2_field113, source_ml_source2_field112, source_ml_source2_field111, source_ml_source2_field108.clone())
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
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let source_ml_source2_field117 = text_3.clone();
                                                                                                                                                                                                        {
                                                                                                                                                                                                            let source_ml_source2_field118 = intervals_3;
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let source_ml_source2_field119 = brands_2;
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let source_ml_source2_field120 = public_fields_2;
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        let source_ml_source2_field121 = prototype_methods_2;
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            let source_ml_source2_field122 = owner_rows_2;
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let source_ml_source2_field123 = self::ml_string::Text::from_units(vec![]);
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    let source_ml_source2_field124 = 0f64;
                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                        let source_ml_source2_field125 = true;
                                                                                                                                                                                                                                        (source_ml_source2_field124, source_ml_source2_field119, source_ml_source2_field125, source_ml_source2_field123.clone(), source_ml_source2_field118, source_ml_source2_field122, source_ml_source2_field121, source_ml_source2_field120, source_ml_source2_field117.clone())
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

pub fn accept_java_script_closed_class_source_size(units: f64, tokens: f64) -> bool {
    (((((self::ml_math::is_safe_integer(units) && self::ml_math::is_safe_integer(tokens)) && (units >= (0f64))) && (units <= (131072f64))) && (tokens >= (0f64))) && (tokens <= (8192f64)))
}

pub fn read_java_script_closed_class_context_error(shared: bool, externals: f64, constants: f64, bindings: f64, known: bool, module: bool) -> self::ml_string::Text {
    if (((((shared && (externals == (0f64))) && (constants == (0f64))) && (bindings == (0f64))) && known) && !module) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![116, 104, 101, 32, 99, 108, 111, 115, 101, 100, 32, 112, 114, 111, 102, 105, 108, 101, 32, 110, 101, 101, 100, 115, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 102, 101, 114, 101, 110, 99, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115, 32, 97, 110, 100, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 108, 111, 99, 97, 108, 32, 109, 111, 100, 117, 108, 101, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 117, 110, 112, 114, 111, 118, 101, 100, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 99, 111, 110, 115, 117, 109, 101, 114, 115])
    }
}

pub fn read_java_script_closed_class_use_error(kind: self::ml_string::Text, operation: self::ml_string::Text, instance: bool, private_field: bool, callable: bool, direct: bool) -> self::ml_string::Text {
    if !instance {
        self::ml_string::Text::from_units(vec![])
    } else {
        if (((kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]))) && !private_field) && (!callable || direct)) {
            self::ml_string::Text::from_units(vec![])
        } else {
            if ((kind == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))) && operation.starts_with(&self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100]))) {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101, 100, 32, 99, 108, 97, 115, 115, 32, 105, 110, 115, 116, 97, 110, 99, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 114, 101, 99, 116, 32, 112, 117, 98, 108, 105, 99, 32, 102, 105, 101, 108, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 97, 110, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 45, 114, 101, 116, 97, 105, 110, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 32, 99, 97, 108, 108, 115, 59, 32, 112, 114, 105, 118, 97, 116, 101, 32, 115, 108, 111, 116, 115, 44, 32, 100, 101, 116, 97, 99, 104, 101, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 115, 32, 97, 110, 100, 32, 105, 110, 100, 105, 114, 101, 99, 116, 32, 114, 101, 102, 108, 101, 99, 116, 105, 118, 101, 32, 108, 111, 111, 107, 117, 112, 115, 32, 110, 101, 101, 100, 32, 116, 104, 101, 32, 102, 117, 108, 108, 32, 111, 98, 106, 101, 99, 116, 32, 109, 111, 100, 101, 108])
            }
        }
    }
}

pub fn accept_java_script_closed_class_own_field(receiver_fields: Vec<self::ml_string::Text>, public_fields: Vec<self::ml_string::Text>, field: self::ml_string::Text) -> bool {
    if ((((public_fields.len() as f64) % 2f64)) != (0f64)) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source2_s126 = self::source_ml_source1_temporary_accept_java_script_closed_class_own_field_loop68(receiver_fields, public_fields, field.clone(), at);
                match source_ml_source2_s126.clone() {
                    self::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Done => {
                        false
                    }
                    self::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result::MlSource1TemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Return(source_ml_source1_temporary_result68) => {
                        source_ml_source1_temporary_result68
                    }
                }
            }
        }
    }
}

pub fn read_java_script_closed_class_own_field_use_error(kind: self::ml_string::Text, operation: self::ml_string::Text, instance: bool, private_field: bool, callable: bool, direct: bool, own_field: bool) -> self::ml_string::Text {
    if (((instance && (kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100])))) && !private_field) && own_field) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::read_java_script_closed_class_use_error(kind.clone(), operation.clone(), instance, private_field, callable, direct)
    }
}

pub fn project_java_script_class_source_interval(intervals: Vec<f64>, first: f64, last: f64) -> Vec<f64> {
    {
        let low = (-1f64);
        {
            let high = (-1f64);
            {
                let at = 0f64;
                {
                    let source_ml_source2_s127 = self::source_ml_source1_temporary_project_java_script_class_source_interval_loop69(intervals, first, last, low, high, at);
                    match source_ml_source2_s127.clone() {
                        self::MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Result::MlSource1TemporaryProjectJavaScriptClassSourceIntervalLoop69Done(low_2, high_2) => {
                            if (low_2 < (0f64)) {
                                Vec::<f64>::new()
                            } else {
                                vec![low_2, high_2]
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result {
    MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Done,
    MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return(f64),
}

pub fn source_ml_source1_temporary_find_java_script_class_group_end_loop1(mut values: Vec<self::ml_string::Text>, mut open: f64, mut expected: Vec<self::ml_string::Text>, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result {
    loop {
        return if ((at < ((values.len() as f64))) && (((at - open)) <= (8192f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let closing = if (value == (self::ml_string::Text::from_units(vec![40]))) {
                        self::ml_string::Text::from_units(vec![41])
                    } else {
                        if (value == (self::ml_string::Text::from_units(vec![91]))) {
                            self::ml_string::Text::from_units(vec![93])
                        } else {
                            if (value == (self::ml_string::Text::from_units(vec![123]))) {
                                self::ml_string::Text::from_units(vec![125])
                            } else {
                                self::ml_string::Text::from_units(vec![])
                            }
                        }
                    };
                    if (closing != (self::ml_string::Text::from_units(vec![]))) {
                        if (((expected.len() as f64)) >= (128f64)) {
                            self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return((-1f64))
                        } else {
                            {
                                let expected_2 = {
                                    let source_ml_source2_concat_value128 = expected;
                                    {
                                        let source_ml_source2_concat_value129 = vec![closing.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value128), source_ml_source2_concat_value129)
                                    }
                                };
                                {
                                    let at_2 = (at + 1f64);
                                    {
                                        (expected, at) = (expected_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        if (((value == (self::ml_string::Text::from_units(vec![41]))) || (value == (self::ml_string::Text::from_units(vec![93])))) || (value == (self::ml_string::Text::from_units(vec![125])))) {
                            if (value != ((self::ml_array::optional_at(&expected, self::ml_array::number_index(((expected.len() as f64) - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))) {
                                self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return((-1f64))
                            } else {
                                {
                                    let expected_3 = self::ml_array::slice(&expected, 0f64, (-1f64));
                                    if (((expected_3.len() as f64)) == (0f64)) {
                                        self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Return(at)
                                    } else {
                                        {
                                            let at_3 = (at + 1f64);
                                            {
                                                (expected, at) = (expected_3, at_3);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let at_4 = (at + 1f64);
                                {
                                    at = at_4;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Result::MlSource1TemporaryFindJavaScriptClassGroupEndLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result {
    MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Done(Vec<f64>),
    MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3 {
    MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(f64),
    MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_read_java_script_closed_class_parameter_positions_with_collections_loop2(mut values: Vec<self::ml_string::Text>, mut kinds: Vec<self::ml_string::Text>, mut close: f64, mut collections: bool, mut result: Vec<f64>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result {
    loop {
        return if (at < close) {
            if (((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114]))) {
                self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Return(vec![(-1f64), at])
            } else {
                {
                    let result_2 = {
                        let source_ml_source2_concat_value130 = result;
                        {
                            let source_ml_source2_concat_value131 = vec![at];
                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value130), source_ml_source2_concat_value131)
                        }
                    };
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_source2_s132 = if (((self::ml_array::optional_at(&values, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61]))) {
                                {
                                    let at_3 = (at_2 + 1f64);
                                    {
                                        let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                                        if ((value == (self::ml_string::Text::from_units(vec![43]))) || (value == (self::ml_string::Text::from_units(vec![45])))) {
                                            {
                                                let at_4 = (at_3 + 1f64);
                                                if ((((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at_4))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) && !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![78, 97, 78]), self::ml_string::Text::from_units(vec![73, 110, 102, 105, 110, 105, 116, 121])], &(self::ml_array::optional_at(&values, self::ml_array::number_index(at_4))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b))) {
                                                    self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Return(vec![(-1f64), at_4])
                                                } else {
                                                    {
                                                        let at_5 = (at_4 + 1f64);
                                                        self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_5)
                                                    }
                                                }
                                            }
                                        } else {
                                            if (((((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) || (((self::ml_array::optional_at(&kinds, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![116, 114, 117, 101]), self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101]), self::ml_string::Text::from_units(vec![110, 117, 108, 108]), self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]), self::ml_string::Text::from_units(vec![78, 97, 78]), self::ml_string::Text::from_units(vec![73, 110, 102, 105, 110, 105, 116, 121])], &value.clone(), 0f64, false, |a, b| a == b)) {
                                                {
                                                    let at_6 = (at_3 + 1f64);
                                                    self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_6)
                                                }
                                            } else {
                                                if (((value == (self::ml_string::Text::from_units(vec![91]))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![93])))) || ((value == (self::ml_string::Text::from_units(vec![123]))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![125]))))) {
                                                    {
                                                        let at_7 = (at_3 + 2f64);
                                                        self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_7)
                                                    }
                                                } else {
                                                    if ((((collections && (value == (self::ml_string::Text::from_units(vec![110, 101, 119])))) && self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])], &(self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![40])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 3f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![41])))) {
                                                        {
                                                            let at_8 = (at_3 + 4f64);
                                                            self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_8)
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Return(vec![(-1f64), at_3])
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_2)
                            };
                            match source_ml_source2_s132.clone() {
                                self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Next(at_9) => {
                                    if (at_9 < close) {
                                        if (((self::ml_array::optional_at(&values, self::ml_array::number_index(at_9))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![44]))) {
                                            self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Return(vec![(-1f64), at_9])
                                        } else {
                                            {
                                                let at_10 = (at_9 + 1f64);
                                                {
                                                    (result, at) = (result_2, at_10);
                                                    continue;
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            (result, at) = (result_2, at_9);
                                            continue;
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsJoin3Return(source_ml_source1_temporary_result3) => {
                                    self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Return(source_ml_source1_temporary_result3)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Result::MlSource1TemporaryReadJavaScriptClosedClassParameterPositionsWithCollectionsLoop2Done(result)
        };
    }
}

pub fn source_ml_source1_temporary_render_java_script_closed_class_parameter_arguments_loop4(mut source: self::ml_string::Text, mut values: Vec<self::ml_string::Text>, mut starts: Vec<f64>, mut ends: Vec<f64>, mut close: f64, mut result: Vec<self::ml_string::Text>, mut at: f64) -> Vec<self::ml_string::Text> {
    loop {
        return if (at < close) {
            {
                let result_2 = {
                    let source_ml_source2_concat_value138 = result;
                    {
                        let source_ml_source2_concat_value139 = vec![{
                            let source_ml_source2_string_range_operand133 = source.clone();
                            {
                                let source_ml_source2_string_range_operand134 = (self::ml_array::optional_at(&starts, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                                {
                                    let source_ml_source2_string_range_operand135 = (self::ml_array::optional_at(&ends, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                                    {
                                        let source_ml_source2_string_range_position136 = source_ml_source2_string_range_operand134;
                                        {
                                            let source_ml_source2_string_range_position137 = source_ml_source2_string_range_operand135;
                                            source_ml_source2_string_range_operand133.slice(source_ml_source2_string_range_position136, source_ml_source2_string_range_position137)
                                        }
                                    }
                                }
                            }
                        }];
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value138), source_ml_source2_concat_value139)
                    }
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        let at_8 = if (((self::ml_array::optional_at(&values, self::ml_array::number_index(at_2))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61]))) {
                            {
                                let at_3 = (at_2 + 1f64);
                                {
                                    let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at_3))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                                    if ((value == (self::ml_string::Text::from_units(vec![91]))) || (value == (self::ml_string::Text::from_units(vec![123])))) {
                                        {
                                            let at_4 = (at_3 + 2f64);
                                            at_4
                                        }
                                    } else {
                                        if ((value == (self::ml_string::Text::from_units(vec![43]))) || (value == (self::ml_string::Text::from_units(vec![45])))) {
                                            {
                                                let at_5 = (at_3 + 2f64);
                                                at_5
                                            }
                                        } else {
                                            if ((((value == (self::ml_string::Text::from_units(vec![110, 101, 119]))) && self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])], &(self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b)) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 2f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![40])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at_3 + 3f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![41])))) {
                                                {
                                                    let at_6 = (at_3 + 4f64);
                                                    at_6
                                                }
                                            } else {
                                                {
                                                    let at_7 = (at_3 + 1f64);
                                                    at_7
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            at_2
                        };
                        if (at_8 < close) {
                            {
                                let at_9 = (at_8 + 1f64);
                                {
                                    (result, at) = (result_2, at_9);
                                    continue;
                                }
                            }
                        } else {
                            {
                                (result, at) = (result_2, at_8);
                                continue;
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Result {
    MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Done,
    MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Return(f64),
}

pub fn source_ml_source1_temporary_find_java_script_closed_class_initialization_start_loop5(mut values: Vec<self::ml_string::Text>, mut first: f64, mut last: f64, mut depth: f64, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Result {
    loop {
        return if (at < last) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let previous = (self::ml_array::optional_at(&values, self::ml_array::number_index((at - 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if (((value == (self::ml_string::Text::from_units(vec![116, 104, 105, 115]))) && (previous != (self::ml_string::Text::from_units(vec![46])))) && (previous != (self::ml_string::Text::from_units(vec![63, 46])))) {
                        {
                            let boundary = (((at == first) || (previous == (self::ml_string::Text::from_units(vec![59])))) || (previous == (self::ml_string::Text::from_units(vec![125]))));
                            self::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Result::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Return(if ((((depth == (0f64)) && boundary) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![46])))) && (((self::ml_array::optional_at(&values, self::ml_array::number_index((at + 3f64)))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) == (self::ml_string::Text::from_units(vec![61])))) {
                                at
                            } else {
                                first
                            })
                        }
                    } else {
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
                                        let at_2 = (at + 1f64);
                                        {
                                            (depth, at) = (depth_4, at_2);
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let at_3 = (at + 1f64);
                                    {
                                        (depth, at) = (depth_3, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Result::MlSource1TemporaryFindJavaScriptClosedClassInitializationStartLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Result {
    MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Done,
    MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop6Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Result {
    MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Done,
    MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Return(f64),
}

pub fn source_ml_source1_temporary_find_java_script_closed_class_field_end_loop7(mut values: Vec<self::ml_string::Text>, mut end: f64, mut inner: f64) -> self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Result {
    loop {
        return if (inner < end) {
            {
                let token = (self::ml_array::optional_at(&values, self::ml_array::number_index(inner))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![116, 104, 105, 115]), self::ml_string::Text::from_units(vec![115, 117, 112, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115]), self::ml_string::Text::from_units(vec![97, 119, 97, 105, 116]), self::ml_string::Text::from_units(vec![121, 105, 101, 108, 100]), self::ml_string::Text::from_units(vec![61, 62]), self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115])], &token.clone(), 0f64, false, |a, b| a == b) {
                    self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Return((-1f64))
                } else {
                    {
                        let inner_2 = (inner + 1f64);
                        {
                            inner = inner_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Result::MlSource1TemporaryFindJavaScriptClosedClassFieldEndLoop7Done
        };
    }
}
