// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::refuse_java_script_call_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_binding_write_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceAssignBinding(identity, source_value) => {
            let source_value = *source_value;
            if {
                let source_ml_source2_comparison_operand4103 = match state.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4102, _, _, _) => {
                        ((source_ml_source2_data_property_value4102.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                !(match (source_ml_source2_comparison_operand4103.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
            } {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![109, 117, 116, 97, 98, 108, 101, 32, 115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 99, 111, 109, 112, 105, 108, 97, 116, 105, 111, 110, 32, 112, 97, 115, 115])))
            } else {
                {
                    let binding = { let ml_values41 = { let ml_reference_value42 = bindings.borrow().clone(); ml_reference_value42 }; let ml_callback41 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4106, _, _, _) => {
                            source_ml_source2_data_property_value4106
                        }
                    }) == identity) }) }; ml_values41.iter().cloned().enumerate().find_map(|(ml_index41, ml_value41)| if ml_callback41(ml_value41.clone()) { Some(ml_value41) } else { None }) };
                    if (((match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || (({
                        let source_ml_source2_nullish_field_receiver4110 = (binding.clone(),);
                        if match (source_ml_source2_nullish_field_receiver4110.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present4111 = ((source_ml_source2_nullish_field_receiver4110.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present4111.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4112, _, _) => {
                                        source_ml_source2_data_property_value4112
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 39, 41])).0, message_present: true }).raise()
                        }
                    }) == (0f64))) || !({
                        let source_ml_source2_nullish_field_receiver4113 = (binding.clone(),);
                        if match (source_ml_source2_nullish_field_receiver4113.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present4114 = ((source_ml_source2_nullish_field_receiver4113.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present4114.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, source_ml_source2_data_property_value4115, _) => {
                                        source_ml_source2_data_property_value4115
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 109, 117, 116, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                        }
                    })) || ({
                        let source_ml_source2_comparison_operand4119 = {
                            let source_ml_source2_nullish_field_receiver4116 = (binding.clone(),);
                            if match (source_ml_source2_nullish_field_receiver4116.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                {
                                    let source_ml_source2_nullish_field_present4117 = ((source_ml_source2_nullish_field_receiver4116.clone()).0).clone().expect("a proven present value");
                                    match source_ml_source2_nullish_field_present4117.clone() {
                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, _, source_ml_source2_data_property_value4118) => {
                                            ((source_ml_source2_data_property_value4118.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 110, 105, 116, 105, 97, 108, 105, 122, 105, 110, 103, 39, 41])).0, message_present: true }).raise()
                            }
                        };
                        match (source_ml_source2_comparison_operand4119.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    } && !(self::ml_array::includes(&{ let ml_reference_value43 = ((match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4122, _, _) => {
                            ((source_ml_source2_data_property_value4122.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                        }
                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value43 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))))) {
                        self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 109, 117, 116, 97, 98, 108, 101, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 108, 111, 99, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114])))
                    } else {
                        {
                            let value = self::lower_java_script_call_source_expression(source_value.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let source_ml_source2_nominal_field4129 = match value.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4130, _) => {
                                        let source_ml_source2_data_property_value4130 = *source_ml_source2_data_property_value4130;
                                        source_ml_source2_data_property_value4130.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field4131 = match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4132) => {
                                            source_ml_source2_data_property_value4132
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override4133 = self::append_java_script_call_source_instruction(match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4123, _) => {
                                                let source_ml_source2_data_property_value4123 = *source_ml_source2_data_property_value4123;
                                                source_ml_source2_data_property_value4123.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy({
                                            let source_ml_source2_nullish_field_receiver4124 = (binding.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver4124.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present4125 = ((source_ml_source2_nullish_field_receiver4124.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present4125.clone() {
                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4126, _, _) => {
                                                            source_ml_source2_data_property_value4126
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }, match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4127) => {
                                                source_ml_source2_data_property_value4127
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override4133.clone()), source_ml_source2_nominal_field4131)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(source_value_2) => {
            let source_value_2 = *source_value_2;
            {
                let value_2 = self::lower_java_script_call_source_expression(source_value_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let message = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![65f64, 115f64, 115f64, 105f64, 103f64, 110f64, 109f64, 101f64, 110f64, 116f64, 32f64, 116f64, 111f64, 32f64, 99f64, 111f64, 110f64, 115f64, 116f64, 97f64, 110f64, 116f64, 32f64, 118f64, 97f64, 114f64, 105f64, 97f64, 98f64, 108f64, 101f64, 46f64]))))), bindings.clone(), match value_2.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4134, _) => {
                            let source_ml_source2_data_property_value4134 = *source_ml_source2_data_property_value4134;
                            source_ml_source2_data_property_value4134.clone()
                        }
                    }, (depth + 1f64));
                    {
                        let present = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Boolean(true))), bindings.clone(), match message.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4135, _) => {
                                let source_ml_source2_data_property_value4135 = *source_ml_source2_data_property_value4135;
                                source_ml_source2_data_property_value4135.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let result = self::allocate_java_script_call_source_register(match present.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4136, _) => {
                                    let source_ml_source2_data_property_value4136 = *source_ml_source2_data_property_value4136;
                                    source_ml_source2_data_property_value4136.clone()
                                }
                            });
                            {
                                let source_ml_source2_nominal_field4142 = match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4143, _) => {
                                        let source_ml_source2_data_property_value4143 = *source_ml_source2_data_property_value4143;
                                        source_ml_source2_data_property_value4143.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field4144 = match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4145) => {
                                            source_ml_source2_data_property_value4145
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override4146 = self::append_java_script_call_source_instruction(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4137, _) => {
                                                let source_ml_source2_data_property_value4137 = *source_ml_source2_data_property_value4137;
                                                source_ml_source2_data_property_value4137.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Primitive(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4138) => {
                                                source_ml_source2_data_property_value4138
                                            }
                                        }, self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![match message.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4139) => {
                                                source_ml_source2_data_property_value4139
                                            }
                                        }, match present.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4140) => {
                                                source_ml_source2_data_property_value4140
                                            }
                                        }]))));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override4146.clone()), source_ml_source2_nominal_field4144)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(identity_2, realm, lexical) => {
            {
                let binding_2 = { let ml_values44 = { let ml_reference_value45 = bindings.borrow().clone(); ml_reference_value45 }; let ml_callback44 = { let identity_2 = identity_2; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4148, _, _, _) => {
                        source_ml_source2_data_property_value4148
                    }
                }) == identity_2) }) }; ml_values44.iter().cloned().enumerate().find_map(|(ml_index44, ml_value44)| if ml_callback44(ml_value44.clone()) { Some(ml_value44) } else { None }) };
                if ((((({
                    let source_ml_source2_comparison_operand4150 = match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4149, _, _, _) => {
                            ((source_ml_source2_data_property_value4149.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                        }
                    };
                    !(match (source_ml_source2_comparison_operand4150.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                } || match (binding_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || (({
                    let source_ml_source2_nullish_field_receiver4155 = (binding_2.clone(),);
                    if match (source_ml_source2_nullish_field_receiver4155.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present4156 = ((source_ml_source2_nullish_field_receiver4155.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present4156.clone() {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4157, _, _) => {
                                    source_ml_source2_data_property_value4157
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 39, 41])).0, message_present: true }).raise()
                    }
                }) == (0f64))) || {
                    let source_ml_source2_comparison_operand4161 = {
                        let source_ml_source2_nullish_field_receiver4158 = (binding_2.clone(),);
                        if match (source_ml_source2_nullish_field_receiver4158.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present4159 = ((source_ml_source2_nullish_field_receiver4158.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present4159.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, _, source_ml_source2_data_property_value4160) => {
                                        ((source_ml_source2_data_property_value4160.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 110, 105, 116, 105, 97, 108, 105, 122, 105, 110, 103, 39, 41])).0, message_present: true }).raise()
                        }
                    };
                    match (source_ml_source2_comparison_operand4161.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                }) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 99, 108, 111, 110, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 108, 101, 120, 105, 99, 97, 108, 32, 99, 101, 108, 108, 32, 97, 110, 100, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109])))
                } else {
                    {
                        let cell = {
                            let source_ml_source2_constructor_field4163 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Null));
                            {
                                let source_ml_source2_constructor_field4173 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![{
                                    let source_ml_source2_constructor_field4165 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![118f64, 97f64, 108f64, 117f64, 101f64])))));
                                    {
                                        let source_ml_source2_constructor_field4168 = {
                                            let source_ml_source2_constructor_field4166 = self::JavaScriptCallSource::SourceBinding(identity_2);
                                            {
                                                let source_ml_source2_constructor_field4167 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![118f64, 97f64, 108f64, 117f64, 101f64])))));
                                                self::JavaScriptCallSource::SourceGet(Box::new(source_ml_source2_constructor_field4166.clone()), Box::new(source_ml_source2_constructor_field4167.clone()), None::<(Option<self::JavaScriptCallSource>,)>)
                                            }
                                        };
                                        self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field4165.clone()), Box::new(source_ml_source2_constructor_field4168.clone()), None::<(Option<bool>,)>)
                                    }
                                }], { let ml_reference_value46 = (if match ((lexical.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                        let source_ml_source2_constructor_field4169 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![105f64, 110f64, 105f64, 116f64, 105f64, 97f64, 108f64, 105f64, 122f64, 101f64, 100f64])))));
                                        {
                                            let source_ml_source2_constructor_field4172 = {
                                                let source_ml_source2_constructor_field4170 = self::JavaScriptCallSource::SourceBinding(identity_2);
                                                {
                                                    let source_ml_source2_constructor_field4171 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![105f64, 110f64, 105f64, 116f64, 105f64, 97f64, 108f64, 105f64, 122f64, 101f64, 100f64])))));
                                                    self::JavaScriptCallSource::SourceGet(Box::new(source_ml_source2_constructor_field4170.clone()), Box::new(source_ml_source2_constructor_field4171.clone()), None::<(Option<self::JavaScriptCallSource>,)>)
                                                }
                                            };
                                            self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field4169.clone()), Box::new(source_ml_source2_constructor_field4172.clone()), None::<(Option<bool>,)>)
                                        }
                                    }]))
                                } else {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceProperty>::new()))
                                }).borrow().clone(); ml_reference_value46 })));
                                self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field4163.clone()), source_ml_source2_constructor_field4173.clone(), realm.clone())
                            }
                        };
                        {
                            let copied = self::lower_java_script_call_source_expression(cell.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                            {
                                let source_ml_source2_nominal_field4180 = match copied.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4181, _) => {
                                        let source_ml_source2_data_property_value4181 = *source_ml_source2_data_property_value4181;
                                        source_ml_source2_data_property_value4181.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field4182 = match copied.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4183) => {
                                            source_ml_source2_data_property_value4183
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override4184 = self::append_java_script_call_source_instruction(match copied.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4174, _) => {
                                                let source_ml_source2_data_property_value4174 = *source_ml_source2_data_property_value4174;
                                                source_ml_source2_data_property_value4174.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy({
                                            let source_ml_source2_nullish_field_receiver4175 = (binding_2.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver4175.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present4176 = ((source_ml_source2_nullish_field_receiver4175.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present4176.clone() {
                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4177, _, _) => {
                                                            source_ml_source2_data_property_value4177
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }, match copied.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4178) => {
                                                source_ml_source2_data_property_value4178
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override4184.clone()), source_ml_source2_nominal_field4182)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceWriteExisting(identity_3, cell_2, source_value_3, body) => {
            let source_value_3 = *source_value_3;
            let body = *body;
            {
                let binding_3 = { let ml_values47 = { let ml_reference_value48 = bindings.borrow().clone(); ml_reference_value48 }; let ml_callback47 = { let identity_3 = identity_3; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4186, _, _, _) => {
                        source_ml_source2_data_property_value4186
                    }
                }) == identity_3) }) }; ml_values47.iter().cloned().enumerate().find_map(|(ml_index47, ml_value47)| if ml_callback47(ml_value47.clone()) { Some(ml_value47) } else { None }) };
                if ((({
                    let source_ml_source2_comparison_operand4188 = match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4187, _, _, _) => {
                            ((source_ml_source2_data_property_value4187.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                        }
                    };
                    !(match (source_ml_source2_comparison_operand4188.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                } || match (binding_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || (({
                    let source_ml_source2_nullish_field_receiver4193 = (binding_3.clone(),);
                    if match (source_ml_source2_nullish_field_receiver4193.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present4194 = ((source_ml_source2_nullish_field_receiver4193.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present4194.clone() {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4195, _, _) => {
                                    source_ml_source2_data_property_value4195
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 39, 41])).0, message_present: true }).raise()
                    }
                }) == (0f64))) || (!cell_2 && !({
                    let source_ml_source2_nullish_field_receiver4196 = (binding_3.clone(),);
                    if match (source_ml_source2_nullish_field_receiver4196.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present4197 = ((source_ml_source2_nullish_field_receiver4196.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present4197.clone() {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, source_ml_source2_data_property_value4198, _) => {
                                    source_ml_source2_data_property_value4198
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 109, 117, 116, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                    }
                }))) {
                    self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![101, 120, 105, 115, 116, 105, 110, 103, 32, 98, 105, 110, 100, 105, 110, 103, 32, 119, 114, 105, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 109, 117, 116, 97, 98, 108, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 111, 114, 32, 112, 114, 105, 118, 97, 116, 101, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 99, 101, 108, 108])))
                } else {
                    {
                        let value_3 = self::lower_java_script_call_source_expression(source_value_3.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                        {
                            let held = self::allocate_java_script_call_source_register(match value_3.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4199, _) => {
                                    let source_ml_source2_data_property_value4199 = *source_ml_source2_data_property_value4199;
                                    source_ml_source2_data_property_value4199.clone()
                                }
                            });
                            {
                                let current = self::append_java_script_call_source_instruction(match held.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4200, _) => {
                                        let source_ml_source2_data_property_value4200 = *source_ml_source2_data_property_value4200;
                                        source_ml_source2_data_property_value4200.clone()
                                    }
                                }, self::JavaScriptCallInstruction::Copy(match held.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4201) => {
                                        source_ml_source2_data_property_value4201
                                    }
                                }, match value_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4202) => {
                                        source_ml_source2_data_property_value4202
                                    }
                                }));
                                {
                                    let current_6 = if cell_2 {
                                        {
                                            let key = self::allocate_java_script_call_source_register(current.clone());
                                            {
                                                let written = self::allocate_java_script_call_source_register(match key.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4203, _) => {
                                                        let source_ml_source2_data_property_value4203 = *source_ml_source2_data_property_value4203;
                                                        source_ml_source2_data_property_value4203.clone()
                                                    }
                                                });
                                                {
                                                    let current_2 = self::append_java_script_call_source_instruction(match written.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4204, _) => {
                                                            let source_ml_source2_data_property_value4204 = *source_ml_source2_data_property_value4204;
                                                            source_ml_source2_data_property_value4204.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Constant(match key.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4205) => {
                                                            source_ml_source2_data_property_value4205
                                                        }
                                                    }, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![118f64, 97f64, 108f64, 117f64, 101f64]))))));
                                                    {
                                                        let current_3 = self::append_java_script_call_source_instruction(current_2.clone(), self::JavaScriptCallInstruction::OrdinarySet(match written.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4206) => {
                                                                source_ml_source2_data_property_value4206
                                                            }
                                                        }, {
                                                            let source_ml_source2_nullish_field_receiver4207 = (binding_3.clone(),);
                                                            if match (source_ml_source2_nullish_field_receiver4207.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source2_nullish_field_present4208 = ((source_ml_source2_nullish_field_receiver4207.clone()).0).clone().expect("a proven present value");
                                                                    match source_ml_source2_nullish_field_present4208.clone() {
                                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4209, _, _) => {
                                                                            source_ml_source2_data_property_value4209
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 39, 41])).0, message_present: true }).raise()
                                                            }
                                                        }, match key.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4210) => {
                                                                source_ml_source2_data_property_value4210
                                                            }
                                                        }, match held.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4211) => {
                                                                source_ml_source2_data_property_value4211
                                                            }
                                                        }, {
                                                            let source_ml_source2_nullish_field_receiver4212 = (binding_3.clone(),);
                                                            if match (source_ml_source2_nullish_field_receiver4212.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                {
                                                                    let source_ml_source2_nullish_field_present4213 = ((source_ml_source2_nullish_field_receiver4212.clone()).0).clone().expect("a proven present value");
                                                                    match source_ml_source2_nullish_field_present4213.clone() {
                                                                        self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4214, _, _) => {
                                                                            source_ml_source2_data_property_value4214
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 39, 41])).0, message_present: true }).raise()
                                                            }
                                                        }));
                                                        {
                                                            let current_4 = self::append_java_script_call_source_instruction(current_3.clone(), self::JavaScriptCallInstruction::RequireWrite(match written.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4215) => {
                                                                    source_ml_source2_data_property_value4215
                                                                }
                                                            }));
                                                            current_4.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let current_5 = self::append_java_script_call_source_instruction(current.clone(), self::JavaScriptCallInstruction::Copy({
                                                let source_ml_source2_nullish_field_receiver4216 = (binding_3.clone(),);
                                                if match (source_ml_source2_nullish_field_receiver4216.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source2_nullish_field_present4217 = ((source_ml_source2_nullish_field_receiver4216.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source2_nullish_field_present4217.clone() {
                                                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4218, _, _) => {
                                                                source_ml_source2_data_property_value4218
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 39, 41])).0, message_present: true }).raise()
                                                }
                                            }, match held.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4219) => {
                                                    source_ml_source2_data_property_value4219
                                                }
                                            }));
                                            current_5.clone()
                                        }
                                    };
                                    self::lower_java_script_call_source_expression(body.clone(), bindings.clone(), current_6.clone(), (depth + 1f64))
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceInitializeBinding(identity_4, source_value_4, body_2) => {
            let source_value_4 = *source_value_4;
            let body_2 = *body_2;
            {
                let binding_4 = { let ml_values49 = { let ml_reference_value50 = bindings.borrow().clone(); ml_reference_value50 }; let ml_callback49 = { let identity_4 = identity_4; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4221, _, _, _) => {
                        source_ml_source2_data_property_value4221
                    }
                }) == identity_4) }) }; ml_values49.iter().cloned().enumerate().find_map(|(ml_index49, ml_value49)| if ml_callback49(ml_value49.clone()) { Some(ml_value49) } else { None }) };
                if ((match (binding_4.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || {
                    let source_ml_source2_comparison_operand4225 = {
                        let source_ml_source2_nullish_field_receiver4222 = (binding_4.clone(),);
                        if match (source_ml_source2_nullish_field_receiver4222.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present4223 = ((source_ml_source2_nullish_field_receiver4222.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present4223.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, _, source_ml_source2_data_property_value4224) => {
                                        ((source_ml_source2_data_property_value4224.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 110, 105, 116, 105, 97, 108, 105, 122, 105, 110, 103, 39, 41])).0, message_present: true }).raise()
                        }
                    };
                    !(match (source_ml_source2_comparison_operand4225.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }) || self::ml_array::includes(&{ let ml_reference_value51 = ((match state.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4228, _, _) => {
                        ((source_ml_source2_data_property_value4228.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value51 }, &identity_4, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                    self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 111, 119, 110, 101, 100, 32, 114, 101, 103, 105, 115, 116, 101, 114])))
                } else {
                    {
                        let value_4 = self::lower_java_script_call_source_expression(source_value_4.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                        {
                            let copied_2 = self::append_java_script_call_source_instruction(match value_4.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value4229, _) => {
                                    let source_ml_source2_data_property_value4229 = *source_ml_source2_data_property_value4229;
                                    source_ml_source2_data_property_value4229.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy({
                                let source_ml_source2_nullish_field_receiver4230 = (binding_4.clone(),);
                                if match (source_ml_source2_nullish_field_receiver4230.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present4231 = ((source_ml_source2_nullish_field_receiver4230.clone()).0).clone().expect("a proven present value");
                                        match source_ml_source2_nullish_field_present4231.clone() {
                                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value4232, _, _) => {
                                                source_ml_source2_data_property_value4232
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 39, 41])).0, message_present: true }).raise()
                                }
                            }, match value_4.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value4233) => {
                                    source_ml_source2_data_property_value4233
                                }
                            }));
                            self::lower_java_script_call_source_expression(body_2.clone(), bindings.clone(), {
                                let source_ml_source2_nominal_field4238 = match copied_2.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value4239, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value4239.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field4240 = match copied_2.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value4241, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value4241
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field4242 = match copied_2.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value4243, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value4243
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field4244 = match copied_2.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value4245, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value4245
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field4246 = match copied_2.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value4247, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value4247
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field4248 = match copied_2.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value4249, _, _, _, _) => {
                                                            source_ml_source2_data_property_value4249.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field4250 = match copied_2.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value4251, _, _, _) => {
                                                                source_ml_source2_data_property_value4251.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field4252 = match copied_2.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4253, _, _) => {
                                                                    source_ml_source2_data_property_value4253.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field4254 = match copied_2.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4255, _) => {
                                                                        source_ml_source2_data_property_value4255.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field4256 = match copied_2.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4257) => {
                                                                            source_ml_source2_data_property_value4257.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_override4258 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                            let source_ml_source2_concat_value4235 = (match copied_2.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4234, _, _) => {
                                                                                    ((source_ml_source2_data_property_value4234.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                                                                                }
                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                                                                            {
                                                                                let source_ml_source2_concat_value4236 = std::rc::Rc::new(std::cell::RefCell::new(vec![identity_4]));
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value52 = source_ml_source2_concat_value4235.borrow().clone(); ml_reference_value52 }), { let ml_reference_value53 = source_ml_source2_concat_value4236.borrow().clone(); ml_reference_value53 })
                                                                            }
                                                                        }));
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field4238.clone(), source_ml_source2_nominal_field4240, source_ml_source2_nominal_field4242, source_ml_source2_nominal_field4244, source_ml_source2_nominal_field4246, source_ml_source2_nominal_field4248.clone(), source_ml_source2_nominal_field4250.clone(), Some((Some(source_ml_source2_nominal_override4258.clone()),)), source_ml_source2_nominal_field4254.clone(), source_ml_source2_nominal_field4256.clone())
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
                            }, (depth + 1f64))
                        }
                    }
                }
            }
        }
        _ => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 98, 105, 110, 100, 105, 110, 103, 119, 114, 105, 116, 101, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}
