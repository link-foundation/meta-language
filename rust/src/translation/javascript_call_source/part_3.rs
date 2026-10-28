// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlTemporaryExecuteJavaScriptCallPrimitiveLoop35Result;
use super::character_utf16_text;
use super::code_point_utf16_text;
use super::compare_utf16_text;
use super::ends_utf16_text;
use super::execute_java_script_call_numeric_primitive;
use super::includes_utf16_text;
use super::index_utf16_text;
use super::last_index_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_java_script_call_primitive_arity_error;
use super::relative_character_utf16_text;
use super::slice_utf16_text;
use super::source_ml_temporary_execute_java_script_call_primitive_loop35;
use super::starts_utf16_text;
use super::trim_utf16_text;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_primitive(operation: &str, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let receiver = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        match receiver.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
            self::JavaScriptSemanticValue::Null => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f146) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f147) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f148) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
            self::JavaScriptSemanticValue::Text(text) => {
                if (operation == "read text length") {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number((text.len() as f64))))
                } else {
                    if (((operation == "trim text") || (operation == "trim text start")) || (operation == "trim text end")) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::trim_utf16_text(&(text), (operation != "trim text end"), (operation != "trim text start")))))
                    } else {
                        if ((((operation == "read text code point") || (operation == "read text character")) || (operation == "read relative text character")) || (operation == "slice text")) {
                            self::execute_java_script_call_text_position(operation, &(text), values)
                        } else {
                            self::execute_java_script_call_text_pair(operation, &(text), values)
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f149) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f150, source_ml_source1_f151) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
        }
    }
}

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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f152) => {
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
                                            let source_ml_source1_comparison_operand158 = value_2.clone();
                                            {
                                                let source_ml_source1_comparison_operand159 = None::<()>;
                                                match (source_ml_source1_comparison_operand158.clone(), source_ml_source1_comparison_operand159.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
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
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f153) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text positions require their actual converted Number operand"))
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f154) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text positions require their actual converted Number operand"))
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f155) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text positions require their actual converted Number operand"))
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f156, source_ml_source1_f157) => {
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f160) => {
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
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f166) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                                }
                                self::JavaScriptSemanticValue::Number(value_3) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::slice_utf16_text(text, second_value_2, value_3))))
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f167) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f168) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f169) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f170, source_ml_source1_f171) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f161) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f162) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f163) => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text slice bounds require their actual converted Number operands"))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f164, source_ml_source1_f165) => {
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f172) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search and concatenation require the actual converted UTF16 operand"))
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f173) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search and concatenation require the actual converted UTF16 operand"))
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f174) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search and concatenation require the actual converted UTF16 operand"))
                }
                self::JavaScriptSemanticValue::Text(units) => {
                    if (operation == "concatenate text") {
                        if ((((text.len() as f64) + (units.len() as f64))) > (1048576f64)) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(String::from("primitive text output exceeds its explicit UTF16 unit budget"))
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text({
                                let source_ml_source1_concat_value178 = text.to_vec();
                                {
                                    let source_ml_source1_concat_value179 = units;
                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value178), source_ml_source1_concat_value179)
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
                                                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f180) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search positions require their actual converted Number operand"))
                                                    }
                                                    self::JavaScriptSemanticValue::Number(value) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (operation == "find text") {
                                                            self::index_utf16_text(text, &(units), value)
                                                        } else {
                                                            self::last_index_utf16_text(text, &(units), value)
                                                        })))
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f181) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search positions require their actual converted Number operand"))
                                                    }
                                                    self::JavaScriptSemanticValue::Text(source_ml_source1_f182) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search positions require their actual converted Number operand"))
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f183) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search positions require their actual converted Number operand"))
                                                    }
                                                    self::JavaScriptSemanticValue::Object(source_ml_source1_f184, source_ml_source1_f185) => {
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
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f175) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text search and concatenation require the actual converted UTF16 operand"))
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f176, source_ml_source1_f177) => {
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
                let source_ml_temporary_values34 = values.clone();
                {
                    let source_ml_temporary_index34 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s186 = self::source_ml_temporary_execute_java_script_call_primitive_loop35(source_ml_temporary_values34, source_ml_temporary_index34.clone());
                        match source_ml_source1_s186.clone() {
                            self::MlTemporaryExecuteJavaScriptCallPrimitiveLoop35Result::MlTemporaryExecuteJavaScriptCallPrimitiveLoop35Done => {
                                if ((((operation == "truncate number") || (operation == "read minimum number")) || (operation == "read maximum number")) || (operation == "construct text units")) {
                                    self::execute_java_script_call_numeric_primitive(operation, values)
                                } else {
                                    self::execute_java_script_call_text_primitive(operation, values)
                                }
                            }
                            self::MlTemporaryExecuteJavaScriptCallPrimitiveLoop35Result::MlTemporaryExecuteJavaScriptCallPrimitiveLoop35Return(source_ml_temporary_result35) => {
                                let source_ml_temporary_result35 = *source_ml_temporary_result35;
                                source_ml_temporary_result35.clone()
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
        let source_ml_source1_nominal_spread187 = builder.clone();
        {
            let source_ml_source1_nominal_field188 = match source_ml_source1_nominal_spread187.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value189, _, _, _, _) => {
                    source_ml_source1_data_property_value189
                }
            };
            {
                let source_ml_source1_nominal_field190 = match source_ml_source1_nominal_spread187.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value191, _, _, _) => {
                        source_ml_source1_data_property_value191
                    }
                };
                {
                    let source_ml_source1_nominal_field192 = match source_ml_source1_nominal_spread187.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value193, _, _) => {
                            source_ml_source1_data_property_value193
                        }
                    };
                    {
                        let source_ml_source1_nominal_field194 = match source_ml_source1_nominal_spread187.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value195, _) => {
                                source_ml_source1_data_property_value195
                            }
                        };
                        {
                            let source_ml_source1_nominal_field196 = match source_ml_source1_nominal_spread187.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value197) => {
                                    source_ml_source1_data_property_value197.clone()
                                }
                            };
                            {
                                let source_ml_source1_nominal_override198 = error.to_owned();
                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_field188, source_ml_source1_nominal_field190, source_ml_source1_nominal_field192, source_ml_source1_nominal_field194, source_ml_source1_nominal_override198.clone())
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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value200) => {
            source_ml_source1_data_property_value200.clone()
        }
    }) != "") {
        builder.clone()
    } else {
        if ((((match &builder { self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value202, _, _, _, _) => source_ml_source1_data_property_value202.as_slice() }).len() as f64)) >= (65536f64)) {
            {
                let source_ml_source1_nominal_spread203 = builder.clone();
                {
                    let source_ml_source1_nominal_field204 = match source_ml_source1_nominal_spread203.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value205, _, _, _, _) => {
                            source_ml_source1_data_property_value205
                        }
                    };
                    {
                        let source_ml_source1_nominal_field206 = match source_ml_source1_nominal_spread203.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value207, _, _, _) => {
                                source_ml_source1_data_property_value207
                            }
                        };
                        {
                            let source_ml_source1_nominal_field208 = match source_ml_source1_nominal_spread203.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value209, _, _) => {
                                    source_ml_source1_data_property_value209
                                }
                            };
                            {
                                let source_ml_source1_nominal_field210 = match source_ml_source1_nominal_spread203.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value211, _) => {
                                        source_ml_source1_data_property_value211
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field212 = match source_ml_source1_nominal_spread203.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value213) => {
                                            source_ml_source1_data_property_value213.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_override214 = String::from("source lowering exceeds its instruction budget");
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_field204, source_ml_source1_nominal_field206, source_ml_source1_nominal_field208, source_ml_source1_nominal_field210, source_ml_source1_nominal_override214.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            {
                let source_ml_source1_nominal_spread218 = builder.clone();
                {
                    let source_ml_source1_nominal_field219 = match source_ml_source1_nominal_spread218.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value220, _, _, _, _) => {
                            source_ml_source1_data_property_value220
                        }
                    };
                    {
                        let source_ml_source1_nominal_field221 = match source_ml_source1_nominal_spread218.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value222, _, _, _) => {
                                source_ml_source1_data_property_value222
                            }
                        };
                        {
                            let source_ml_source1_nominal_field223 = match source_ml_source1_nominal_spread218.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value224, _, _) => {
                                    source_ml_source1_data_property_value224
                                }
                            };
                            {
                                let source_ml_source1_nominal_field225 = match source_ml_source1_nominal_spread218.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value226, _) => {
                                        source_ml_source1_data_property_value226
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field227 = match source_ml_source1_nominal_spread218.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value228) => {
                                            source_ml_source1_data_property_value228.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_override229 = {
                                            let source_ml_source1_concat_value216 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value215, _, _, _, _) => {
                                                    source_ml_source1_data_property_value215
                                                }
                                            };
                                            {
                                                let source_ml_source1_concat_value217 = vec![instruction.clone()];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallInstruction>::new(), source_ml_source1_concat_value216), source_ml_source1_concat_value217)
                                            }
                                        };
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_override229, source_ml_source1_nominal_field221, source_ml_source1_nominal_field223, source_ml_source1_nominal_field225, source_ml_source1_nominal_field227.clone())
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
        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value231) => {
            source_ml_source1_data_property_value231.clone()
        }
    }) != "") {
        self::JavaScriptCallSourceResult::SourceResult(Box::new(builder.clone()), 0f64)
    } else {
        if ((match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value233, _, _, _) => {
                source_ml_source1_data_property_value233
            }
        }) >= (4096f64)) {
            self::refuse_java_script_call_source(builder.clone(), "source lowering exceeds its register budget")
        } else {
            self::JavaScriptCallSourceResult::SourceResult(Box::new({
                let source_ml_source1_nominal_spread235 = builder.clone();
                {
                    let source_ml_source1_nominal_field236 = match source_ml_source1_nominal_spread235.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value237, _, _, _, _) => {
                            source_ml_source1_data_property_value237
                        }
                    };
                    {
                        let source_ml_source1_nominal_field238 = match source_ml_source1_nominal_spread235.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value239, _, _, _) => {
                                source_ml_source1_data_property_value239
                            }
                        };
                        {
                            let source_ml_source1_nominal_field240 = match source_ml_source1_nominal_spread235.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source1_data_property_value241, _, _) => {
                                    source_ml_source1_data_property_value241
                                }
                            };
                            {
                                let source_ml_source1_nominal_field242 = match source_ml_source1_nominal_spread235.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source1_data_property_value243, _) => {
                                        source_ml_source1_data_property_value243
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field244 = match source_ml_source1_nominal_spread235.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value245) => {
                                            source_ml_source1_data_property_value245.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_nominal_override246 = (match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value234, _, _, _) => {
                                                source_ml_source1_data_property_value234
                                            }
                                        } + 1f64);
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_nominal_field236, source_ml_source1_nominal_override246, source_ml_source1_nominal_field240, source_ml_source1_nominal_field242, source_ml_source1_nominal_field244.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }), match builder.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value247, _, _, _) => {
                    source_ml_source1_data_property_value247
                }
            })
        }
    }
}
