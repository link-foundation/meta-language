// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
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
use super::refuse_java_script_call_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_binding_write_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
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
                self::refuse_java_script_call_source(state.clone(), "mutable source bindings require their snapshot compilation pass")
            } else {
                {
                    let binding = { let ml_values40 = { let ml_reference_value41 = bindings.borrow().clone(); ml_reference_value41 }; let ml_callback40 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4106, _, _, _) => {
                            source_ml_source2_data_property_value4106
                        }
                    }) == identity) }) }; ml_values40.iter().cloned().enumerate().find_map(|(ml_index40, ml_value40)| if ml_callback40(ml_value40.clone()) { Some(ml_value40) } else { None }) };
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
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'register')")).encode_utf16().collect(), message_present: true }).raise()
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
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'mutable')")).encode_utf16().collect(), message_present: true }).raise()
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
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'initializing')")).encode_utf16().collect(), message_present: true }).raise()
                            }
                        };
                        match (source_ml_source2_comparison_operand4119.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    } && !(self::ml_array::includes(&{ let ml_reference_value42 = ((match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4122, _, _) => {
                            ((source_ml_source2_data_property_value4122.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                        }
                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value42 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))))) {
                        self::refuse_java_script_call_source(state.clone(), "a mutable assignment needs its actual mutable local lexical register")
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
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'register')")).encode_utf16().collect(), message_present: true }).raise()
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
                                        }, String::from("raiseJavaScriptTypeError"), std::rc::Rc::new(std::cell::RefCell::new(vec![match message.clone() {
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
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(identity_2, realm, lexical) => {
            {
                let binding_2 = { let ml_values43 = { let ml_reference_value44 = bindings.borrow().clone(); ml_reference_value44 }; let ml_callback43 = { let identity_2 = identity_2; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4148, _, _, _) => {
                        source_ml_source2_data_property_value4148
                    }
                }) == identity_2) }) }; ml_values43.iter().cloned().enumerate().find_map(|(ml_index43, ml_value43)| if ml_callback43(ml_value43.clone()) { Some(ml_value43) } else { None }) };
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
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'register')")).encode_utf16().collect(), message_present: true }).raise()
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
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'initializing')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    };
                    match (source_ml_source2_comparison_operand4161.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                }) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::refuse_java_script_call_source(state.clone(), "iteration cloning requires its actual initialized lexical cell and source realm")
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
                                }], { let ml_reference_value45 = (if match ((lexical.clone().unwrap_or_else(|| (None::<bool>,))).0).clone() { Some(ml_present) => ml_present, None => false } {
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
                                }).borrow().clone(); ml_reference_value45 })));
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
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'register')")).encode_utf16().collect(), message_present: true }).raise()
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
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceWriteExisting(identity_3, cell_2, source_value_3, body) => {
            let source_value_3 = *source_value_3;
            let body = *body;
            {
                let binding_3 = { let ml_values46 = { let ml_reference_value47 = bindings.borrow().clone(); ml_reference_value47 }; let ml_callback46 = { let identity_3 = identity_3; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4186, _, _, _) => {
                        source_ml_source2_data_property_value4186
                    }
                }) == identity_3) }) }; ml_values46.iter().cloned().enumerate().find_map(|(ml_index46, ml_value46)| if ml_callback46(ml_value46.clone()) { Some(ml_value46) } else { None }) };
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
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'register')")).encode_utf16().collect(), message_present: true }).raise()
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
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'mutable')")).encode_utf16().collect(), message_present: true }).raise()
                    }
                }))) {
                    self::refuse_java_script_call_source(state.clone(), "existing binding writes retain an actual mutable register or private captured cell")
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
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'register')")).encode_utf16().collect(), message_present: true }).raise()
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
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'register')")).encode_utf16().collect(), message_present: true }).raise()
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
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'register')")).encode_utf16().collect(), message_present: true }).raise()
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
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInitializeBinding(identity_4, source_value_4, body_2) => {
            let source_value_4 = *source_value_4;
            let body_2 = *body_2;
            {
                let binding_4 = { let ml_values48 = { let ml_reference_value49 = bindings.borrow().clone(); ml_reference_value49 }; let ml_callback48 = { let identity_4 = identity_4; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value4221, _, _, _) => {
                        source_ml_source2_data_property_value4221
                    }
                }) == identity_4) }) }; ml_values48.iter().cloned().enumerate().find_map(|(ml_index48, ml_value48)| if ml_callback48(ml_value48.clone()) { Some(ml_value48) } else { None }) };
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
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'initializing')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    };
                    !(match (source_ml_source2_comparison_operand4225.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }) || self::ml_array::includes(&{ let ml_reference_value50 = ((match state.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value4228, _, _) => {
                        ((source_ml_source2_data_property_value4228.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value50 }, &identity_4, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) {
                    self::refuse_java_script_call_source(state.clone(), "binding initialization retains its actual uninitialized owned register")
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
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'register')")).encode_utf16().collect(), message_present: true }).raise()
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
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value51 = source_ml_source2_concat_value4235.borrow().clone(); ml_reference_value51 }), { let ml_reference_value52 = source_ml_source2_concat_value4236.borrow().clone(); ml_reference_value52 })
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
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), "source bindingwrite lowering requires its actual expression kind")
        }
    }
}
