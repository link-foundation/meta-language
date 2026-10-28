// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop93Result;
use super::character_utf16_text;
use super::code_point_utf16_text;
use super::compare_utf16_text;
use super::ends_utf16_text;
use super::execute_java_script_call_integer_primitive;
use super::execute_java_script_call_numeric_primitive;
use super::execute_java_script_call_text_primitive;
use super::includes_utf16_text;
use super::index_utf16_text;
use super::last_index_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_java_script_call_integer_arity;
use super::read_java_script_call_primitive_arity_error;
use super::relative_character_utf16_text;
use super::slice_utf16_text;
use super::source_ml_source1_temporary_execute_java_script_call_primitive_loop93;
use super::starts_utf16_text;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_position(operation: &str, text: &[f64], values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let second = (self::ml_array::optional_at(&values, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let third = (self::ml_array::optional_at(&values, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            if (((operation == "read text code point") || (operation == "read text character")) || (operation == "read relative text character")) {
                match second.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text positions require their actual converted Number operand"))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text positions require their actual converted Number operand"))
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f227) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text positions require their actual converted Number operand"))
                    }
                    self::JavaScriptSemanticValue::Number(second_value) => {
                        if (operation == "read text character") {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::character_utf16_text(text, second_value))))
                        } else {
                            if (operation == "read relative text character") {
                                {
                                    let result = self::relative_character_utf16_text(text, second_value);
                                    {
                                        let value = self::ml_array::optional_at(&result, self::ml_array::number_index(0f64));
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(if match (value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::JavaScriptSemanticValue::Undefined
                                        } else {
                                            self::JavaScriptSemanticValue::Text(value.clone().expect("a proven present value"))
                                        }))
                                    }
                                }
                            } else {
                                {
                                    let result_2 = self::code_point_utf16_text(text, second_value);
                                    {
                                        let value_2 = self::ml_array::optional_at(&result_2, self::ml_array::number_index(0f64));
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(if {
                                            let source_ml_source2_comparison_operand233 = value_2.clone();
                                            {
                                                let source_ml_source2_comparison_operand234 = None::<()>;
                                                match (source_ml_source2_comparison_operand233.clone(), source_ml_source2_comparison_operand234.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                            }
                                        } {
                                            self::JavaScriptSemanticValue::Undefined
                                        } else {
                                            self::JavaScriptSemanticValue::Number(value_2.clone().expect("a proven present value"))
                                        }))
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f228) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text positions require their actual converted Number operand"))
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f229) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text positions require their actual converted Number operand"))
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f230) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text positions require their actual converted Number operand"))
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f231, source_ml_source2_f232) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text positions require their actual converted Number operand"))
                    }
                }
            } else {
                if (operation == "slice text") {
                    match second.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f235) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Number(second_value_2) => {
                            match third.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f241) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                                }
                                self::JavaScriptSemanticValue::Number(value_3) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::slice_utf16_text(text, second_value_2, value_3))))
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f242) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source2_f243) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f244) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source2_f245, source_ml_source2_f246) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f236) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f237) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f238) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f239, source_ml_source2_f240) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                        }
                    }
                } else {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("a text position executor needs its actual registered operation"))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_pair(operation: &str, text: &[f64], values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let second = (self::ml_array::optional_at(&values, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let third = (self::ml_array::optional_at(&values, self::ml_array::number_index(2f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            match second.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search and concatenation require the actual converted UTF16 operand"))
                }
                self::JavaScriptSemanticValue::Null => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search and concatenation require the actual converted UTF16 operand"))
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f247) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search and concatenation require the actual converted UTF16 operand"))
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f248) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search and concatenation require the actual converted UTF16 operand"))
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f249) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search and concatenation require the actual converted UTF16 operand"))
                }
                self::JavaScriptSemanticValue::Text(units) => {
                    if (operation == "concatenate text") {
                        if ((((text.len() as f64) + (units.len() as f64))) > (1048576f64)) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(String::from("primitive text output exceeds its explicit UTF16 unit budget"))
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text({
                                let source_ml_source2_concat_value253 = text.to_vec();
                                {
                                    let source_ml_source2_concat_value254 = units;
                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value253), source_ml_source2_concat_value254)
                                }
                            })))
                        }
                    } else {
                        if (operation == "compare text") {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::compare_utf16_text(text, &(units)))))
                        } else {
                            if (operation == "test text prefix") {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::starts_utf16_text(text, &(units)))))
                            } else {
                                if (operation == "test text suffix") {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::ends_utf16_text(text, &(units)))))
                                } else {
                                    if ((((self::ml_math::max(0f64, ((text.len() as f64) - (units.len() as f64))) + 1f64) * (units.len() as f64))) > (67108864f64)) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(String::from("primitive text search exceeds its explicit comparison work quota"))
                                    } else {
                                        if (operation == "test text inclusion") {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::includes_utf16_text(text, &(units)))))
                                        } else {
                                            if ((operation == "find text") || (operation == "find last text")) {
                                                match third.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search positions require their actual converted Number operand"))
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search positions require their actual converted Number operand"))
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f255) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search positions require their actual converted Number operand"))
                                                    }
                                                    self::JavaScriptSemanticValue::Number(value) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (operation == "find text") {
                                                            self::index_utf16_text(text, &(units), value)
                                                        } else {
                                                            self::last_index_utf16_text(text, &(units), value)
                                                        })))
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f256) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search positions require their actual converted Number operand"))
                                                    }
                                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f257) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search positions require their actual converted Number operand"))
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f258) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search positions require their actual converted Number operand"))
                                                    }
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f259, source_ml_source2_f260) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search positions require their actual converted Number operand"))
                                                    }
                                                }
                                            } else {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("the text primitive executor needs its actual registered operation"))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f250) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search and concatenation require the actual converted UTF16 operand"))
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f251, source_ml_source2_f252) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search and concatenation require the actual converted UTF16 operand"))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_primitive(operation: &str, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let arity = self::read_java_script_call_primitive_arity_error(operation, (values.len() as f64));
        if (arity != "") {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(arity.clone())
        } else {
            {
                let source_ml_source1_temporary_values92 = values.clone();
                {
                    let source_ml_source1_temporary_index92 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s261 = self::source_ml_source1_temporary_execute_java_script_call_primitive_loop93(source_ml_source1_temporary_values92, source_ml_source1_temporary_index92.clone());
                        match source_ml_source2_s261.clone() {
                            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop93Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop93Done => {
                                if ((self::read_java_script_call_integer_arity(operation)) != (0f64)) {
                                    self::execute_java_script_call_integer_primitive(operation, values)
                                } else {
                                    if ((((operation == "truncate number") || (operation == "read minimum number")) || (operation == "read maximum number")) || (operation == "construct text units")) {
                                        self::execute_java_script_call_numeric_primitive(operation, values)
                                    } else {
                                        self::execute_java_script_call_text_primitive(operation, values)
                                    }
                                }
                            }
                            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop93Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop93Return(source_ml_source1_temporary_result93) => {
                                let source_ml_source1_temporary_result93 = *source_ml_source1_temporary_result93;
                                source_ml_source1_temporary_result93.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_call_source(builder: self::JavaScriptCallSourceBuilder, error: &str) -> self::JavaScriptCallSourceResult {
    self::JavaScriptCallSourceResult::SourceResult(Box::new({
        let source_ml_source2_nominal_spread262 = builder.clone();
        {
            let source_ml_source2_nominal_field263 = match source_ml_source2_nominal_spread262.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value264, _, _, _, _) => {
                    source_ml_source2_data_property_value264
                }
            };
            {
                let source_ml_source2_nominal_field265 = match source_ml_source2_nominal_spread262.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value266, _, _, _) => {
                        source_ml_source2_data_property_value266
                    }
                };
                {
                    let source_ml_source2_nominal_field267 = match source_ml_source2_nominal_spread262.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value268, _, _) => {
                            source_ml_source2_data_property_value268
                        }
                    };
                    {
                        let source_ml_source2_nominal_field269 = match source_ml_source2_nominal_spread262.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value270, _) => {
                                source_ml_source2_data_property_value270
                            }
                        };
                        {
                            let source_ml_source2_nominal_field271 = match source_ml_source2_nominal_spread262.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value272) => {
                                    source_ml_source2_data_property_value272.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_override273 = error.to_owned();
                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field263, source_ml_source2_nominal_field265, source_ml_source2_nominal_field267, source_ml_source2_nominal_field269, source_ml_source2_nominal_override273.clone())
                            }
                        }
                    }
                }
            }
        }
    }), 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append_java_script_call_source_instruction(builder: self::JavaScriptCallSourceBuilder, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallSourceBuilder {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value275) => {
            source_ml_source2_data_property_value275.clone()
        }
    }) != "") {
        builder.clone()
    } else {
        if ((((match &builder { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value277, _, _, _, _) => source_ml_source2_data_property_value277.as_slice() }).len() as f64)) >= (65536f64)) {
            {
                let source_ml_source2_nominal_spread278 = builder.clone();
                {
                    let source_ml_source2_nominal_field279 = match source_ml_source2_nominal_spread278.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value280, _, _, _, _) => {
                            source_ml_source2_data_property_value280
                        }
                    };
                    {
                        let source_ml_source2_nominal_field281 = match source_ml_source2_nominal_spread278.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value282, _, _, _) => {
                                source_ml_source2_data_property_value282
                            }
                        };
                        {
                            let source_ml_source2_nominal_field283 = match source_ml_source2_nominal_spread278.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value284, _, _) => {
                                    source_ml_source2_data_property_value284
                                }
                            };
                            {
                                let source_ml_source2_nominal_field285 = match source_ml_source2_nominal_spread278.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value286, _) => {
                                        source_ml_source2_data_property_value286
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field287 = match source_ml_source2_nominal_spread278.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value288) => {
                                            source_ml_source2_data_property_value288.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override289 = String::from("source lowering exceeds its instruction budget");
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field279, source_ml_source2_nominal_field281, source_ml_source2_nominal_field283, source_ml_source2_nominal_field285, source_ml_source2_nominal_override289.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            {
                let source_ml_source2_nominal_spread293 = builder.clone();
                {
                    let source_ml_source2_nominal_field294 = match source_ml_source2_nominal_spread293.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value295, _, _, _, _) => {
                            source_ml_source2_data_property_value295
                        }
                    };
                    {
                        let source_ml_source2_nominal_field296 = match source_ml_source2_nominal_spread293.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value297, _, _, _) => {
                                source_ml_source2_data_property_value297
                            }
                        };
                        {
                            let source_ml_source2_nominal_field298 = match source_ml_source2_nominal_spread293.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value299, _, _) => {
                                    source_ml_source2_data_property_value299
                                }
                            };
                            {
                                let source_ml_source2_nominal_field300 = match source_ml_source2_nominal_spread293.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value301, _) => {
                                        source_ml_source2_data_property_value301
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field302 = match source_ml_source2_nominal_spread293.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value303) => {
                                            source_ml_source2_data_property_value303.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override304 = {
                                            let source_ml_source2_concat_value291 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value290, _, _, _, _) => {
                                                    source_ml_source2_data_property_value290
                                                }
                                            };
                                            {
                                                let source_ml_source2_concat_value292 = vec![instruction.clone()];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallInstruction>::new(), source_ml_source2_concat_value291), source_ml_source2_concat_value292)
                                            }
                                        };
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override304, source_ml_source2_nominal_field296, source_ml_source2_nominal_field298, source_ml_source2_nominal_field300, source_ml_source2_nominal_field302.clone())
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_java_script_call_source_register(builder: self::JavaScriptCallSourceBuilder) -> self::JavaScriptCallSourceResult {
    if ((match builder.clone() {
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value306) => {
            source_ml_source2_data_property_value306.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value308, _, _, _) => {
                source_ml_source2_data_property_value308
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source2_nominal_spread310 = builder.clone();
                {
                    let source_ml_source2_nominal_field311 = match source_ml_source2_nominal_spread310.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value312, _, _, _, _) => {
                            source_ml_source2_data_property_value312
                        }
                    };
                    {
                        let source_ml_source2_nominal_field313 = match source_ml_source2_nominal_spread310.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value314, _, _, _) => {
                                source_ml_source2_data_property_value314
                            }
                        };
                        {
                            let source_ml_source2_nominal_field315 = match source_ml_source2_nominal_spread310.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value316, _, _) => {
                                    source_ml_source2_data_property_value316
                                }
                            };
                            {
                                let source_ml_source2_nominal_field317 = match source_ml_source2_nominal_spread310.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value318, _) => {
                                        source_ml_source2_data_property_value318
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field319 = match source_ml_source2_nominal_spread310.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value320) => {
                                            source_ml_source2_data_property_value320.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override321 = (match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value309, _, _, _) => {
                                                source_ml_source2_data_property_value309
                                            }
                                        } + 1f64);
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field311, source_ml_source2_nominal_override321, source_ml_source2_nominal_field315, source_ml_source2_nominal_field317, source_ml_source2_nominal_field319.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value322, _, _, _) => {
                    source_ml_source2_data_property_value322
                }
            })
        }
    }
}
