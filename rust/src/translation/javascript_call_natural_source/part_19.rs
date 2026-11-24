// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::append_java_script_call_source_instruction;
use super::finish_java_script_call_source_control_scope;
use super::lower_java_script_call_source_expression;
use super::ml_string;
use super::push_java_script_call_source_control_scope;
use super::refuse_java_script_call_source;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_loop(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceWhile(source_condition, source_body, after, source_target, source_update, post_test, iteration) => {
            let source_condition = *source_condition;
            let source_body = *source_body;
            let after = *after;
            {
                let target = ((source_target.clone().unwrap_or_else(|| (None::<f64>,))).0).clone();
                {
                    let iteration_source = ((iteration.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSource>,))).0).clone();
                    {
                        let update_source = ((source_update.clone().unwrap_or_else(|| (None::<self::JavaScriptCallSource>,))).0).clone();
                        {
                            let scoped = if {
                                let source_ml_source2_comparison_operand13453 = None::<()>;
                                match (target.clone(), source_ml_source2_comparison_operand13453.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                            } {
                                builder.clone()
                            } else {
                                self::push_java_script_call_source_control_scope(builder.clone(), target.clone().expect("a proven present value"), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112])))
                            };
                            if {
                                let source_ml_source2_comparison_operand13454 = ((post_test.clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                match (source_ml_source2_comparison_operand13454.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                            } {
                                if (!(match (update_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(match (iteration_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                    self::refuse_java_script_call_source(scoped.clone(), &(self::ml_string::Text::from_units(vec![97, 32, 112, 111, 115, 116, 45, 116, 101, 115, 116, 32, 108, 111, 111, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 112, 97, 114, 97, 116, 101, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 119, 105, 116, 104, 111, 117, 116, 32, 97, 110, 32, 105, 110, 118, 101, 110, 116, 101, 100, 32, 117, 112, 100, 97, 116, 101, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 104, 101, 97, 100, 32, 99, 108, 111, 110, 101])))
                                } else {
                                    {
                                        let start = { let ml_reference_array164 = match scoped.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13456, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value13456.clone()
                                            }
                                        }; let ml_reference_projection164 = ml_reference_array164.borrow().len() as f64; ml_reference_projection164 };
                                        {
                                            let body = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), scoped.clone(), (depth + 1f64));
                                            {
                                                let test = { let ml_reference_array165 = match match body.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13457, _) => {
                                                        let source_ml_source2_data_property_value13457 = *source_ml_source2_data_property_value13457;
                                                        source_ml_source2_data_property_value13457.clone()
                                                    }
                                                } {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13458, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value13458.clone()
                                                    }
                                                }; let ml_reference_projection165 = ml_reference_array165.borrow().len() as f64; ml_reference_projection165 };
                                                {
                                                    let condition = self::lower_java_script_call_source_expression(source_condition.clone(), bindings.clone(), match body.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13459, _) => {
                                                            let source_ml_source2_data_property_value13459 = *source_ml_source2_data_property_value13459;
                                                            source_ml_source2_data_property_value13459.clone()
                                                        }
                                                    }, (depth + 1f64));
                                                    {
                                                        let end = ({ let ml_reference_array166 = match match condition.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13460, _) => {
                                                                let source_ml_source2_data_property_value13460 = *source_ml_source2_data_property_value13460;
                                                                source_ml_source2_data_property_value13460.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13461, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value13461.clone()
                                                            }
                                                        }; let ml_reference_projection166 = ml_reference_array166.borrow().len() as f64; ml_reference_projection166 } + 1f64);
                                                        {
                                                            let finished = self::append_java_script_call_source_instruction(match condition.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13462, _) => {
                                                                    let source_ml_source2_data_property_value13462 = *source_ml_source2_data_property_value13462;
                                                                    source_ml_source2_data_property_value13462.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Branch(match condition.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value13463) => {
                                                                    source_ml_source2_data_property_value13463
                                                                }
                                                            }, start, end));
                                                            {
                                                                let finished_3 = if {
                                                                    let source_ml_source2_comparison_operand13465 = None::<()>;
                                                                    !(match (target.clone(), source_ml_source2_comparison_operand13465.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                } {
                                                                    {
                                                                        let finished_2 = self::finish_java_script_call_source_control_scope(finished.clone(), builder.clone(), target.clone().expect("a proven present value"), end, test);
                                                                        finished_2.clone()
                                                                    }
                                                                } else {
                                                                    finished.clone()
                                                                };
                                                                self::lower_java_script_call_source_expression(after.clone(), bindings.clone(), finished_3.clone(), (depth + 1f64))
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
                                    let initial = if match (iteration_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        scoped.clone()
                                    } else {
                                        match self::lower_java_script_call_source_expression(iteration_source.clone().expect("a proven present value"), bindings.clone(), scoped.clone(), (depth + 1f64)) {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13466, _) => {
                                                let source_ml_source2_data_property_value13466 = *source_ml_source2_data_property_value13466;
                                                source_ml_source2_data_property_value13466.clone()
                                            }
                                        }
                                    };
                                    {
                                        let start_2 = { let ml_reference_array167 = match initial.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13467, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value13467.clone()
                                            }
                                        }; let ml_reference_projection167 = ml_reference_array167.borrow().len() as f64; ml_reference_projection167 };
                                        {
                                            let condition_2 = self::lower_java_script_call_source_expression(source_condition.clone(), bindings.clone(), initial.clone(), (depth + 1f64));
                                            {
                                                let branch = { let ml_reference_array168 = match match condition_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13468, _) => {
                                                        let source_ml_source2_data_property_value13468 = *source_ml_source2_data_property_value13468;
                                                        source_ml_source2_data_property_value13468.clone()
                                                    }
                                                } {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13469, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value13469.clone()
                                                    }
                                                }; let ml_reference_projection168 = ml_reference_array168.borrow().len() as f64; ml_reference_projection168 };
                                                {
                                                    let test_2 = self::append_java_script_call_source_instruction(match condition_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13470, _) => {
                                                            let source_ml_source2_data_property_value13470 = *source_ml_source2_data_property_value13470;
                                                            source_ml_source2_data_property_value13470.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::Branch(match condition_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value13471) => {
                                                            source_ml_source2_data_property_value13471
                                                        }
                                                    }, (branch + 1f64), 0f64));
                                                    {
                                                        let body_2 = self::lower_java_script_call_source_expression(source_body.clone(), bindings.clone(), test_2.clone(), (depth + 1f64));
                                                        {
                                                            let update = { let ml_reference_array169 = match match body_2.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13472, _) => {
                                                                    let source_ml_source2_data_property_value13472 = *source_ml_source2_data_property_value13472;
                                                                    source_ml_source2_data_property_value13472.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13473, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value13473.clone()
                                                                }
                                                            }; let ml_reference_projection169 = ml_reference_array169.borrow().len() as f64; ml_reference_projection169 };
                                                            {
                                                                let cloned = if match (iteration_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    match body_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13474, _) => {
                                                                            let source_ml_source2_data_property_value13474 = *source_ml_source2_data_property_value13474;
                                                                            source_ml_source2_data_property_value13474.clone()
                                                                        }
                                                                    }
                                                                } else {
                                                                    match self::lower_java_script_call_source_expression(iteration_source.clone().expect("a proven present value"), bindings.clone(), match body_2.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13475, _) => {
                                                                            let source_ml_source2_data_property_value13475 = *source_ml_source2_data_property_value13475;
                                                                            source_ml_source2_data_property_value13475.clone()
                                                                        }
                                                                    }, (depth + 1f64)) {
                                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13476, _) => {
                                                                            let source_ml_source2_data_property_value13476 = *source_ml_source2_data_property_value13476;
                                                                            source_ml_source2_data_property_value13476.clone()
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let updated = if match (update_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                        cloned.clone()
                                                                    } else {
                                                                        match self::lower_java_script_call_source_expression(update_source.clone().expect("a proven present value"), bindings.clone(), cloned.clone(), (depth + 1f64)) {
                                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13477, _) => {
                                                                                let source_ml_source2_data_property_value13477 = *source_ml_source2_data_property_value13477;
                                                                                source_ml_source2_data_property_value13477.clone()
                                                                            }
                                                                        }
                                                                    };
                                                                    {
                                                                        let loop_ = self::append_java_script_call_source_instruction(updated.clone(), self::JavaScriptCallInstruction::Jump(start_2));
                                                                        {
                                                                            let end_2 = { let ml_reference_array170 = match loop_.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13478, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value13478.clone()
                                                                                }
                                                                            }; let ml_reference_projection170 = ml_reference_array170.borrow().len() as f64; ml_reference_projection170 };
                                                                            {
                                                                                let finished_4 = {
                                                                                    let source_ml_source2_nominal_spread13481 = loop_.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field13482 = match source_ml_source2_nominal_spread13481.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13483, _, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value13483.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field13484 = match source_ml_source2_nominal_spread13481.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value13485, _, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value13485
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field13486 = match source_ml_source2_nominal_spread13481.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value13487, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value13487
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field13488 = match source_ml_source2_nominal_spread13481.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value13489, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value13489
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field13490 = match source_ml_source2_nominal_spread13481.clone() {
                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value13491, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value13491
                                                                                                            }
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field13492 = match source_ml_source2_nominal_spread13481.clone() {
                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value13493, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value13493.clone()
                                                                                                                }
                                                                                                            };
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field13494 = match source_ml_source2_nominal_spread13481.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value13495, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value13495.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field13496 = match source_ml_source2_nominal_spread13481.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value13497, _, _) => {
                                                                                                                            source_ml_source2_data_property_value13497.clone()
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field13498 = match source_ml_source2_nominal_spread13481.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13499, _) => {
                                                                                                                                source_ml_source2_data_property_value13499.clone()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field13500 = match source_ml_source2_nominal_spread13481.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value13501) => {
                                                                                                                                    source_ml_source2_data_property_value13501.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override13502 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values171 = { let ml_reference_value172 = (match loop_.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13479, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value13479.clone()
                                                                                                                                    }
                                                                                                                                }).borrow().clone(); ml_reference_value172 }; let ml_callback171 = { let branch = branch; let condition_2 = condition_2.clone(); let end_2 = end_2; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == branch) {
                                                                                                                                    self::JavaScriptCallInstruction::Branch(match condition_2.clone() {
                                                                                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value13480) => {
                                                                                                                                            source_ml_source2_data_property_value13480
                                                                                                                                        }
                                                                                                                                    }, (branch + 1f64), end_2)
                                                                                                                                } else {
                                                                                                                                    instruction.clone()
                                                                                                                                } }) }; ml_values171.iter().cloned().enumerate().map(|(ml_index171, ml_value171)| ml_callback171(ml_value171, (ml_index171 as f64))).collect::<Vec<_>>() }));
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override13502.clone(), source_ml_source2_nominal_field13484, source_ml_source2_nominal_field13486, source_ml_source2_nominal_field13488, source_ml_source2_nominal_field13490, source_ml_source2_nominal_field13492.clone(), source_ml_source2_nominal_field13494.clone(), source_ml_source2_nominal_field13496.clone(), source_ml_source2_nominal_field13498.clone(), source_ml_source2_nominal_field13500.clone())
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
                                                                                {
                                                                                    let finished_6 = if {
                                                                                        let source_ml_source2_comparison_operand13504 = None::<()>;
                                                                                        !(match (target.clone(), source_ml_source2_comparison_operand13504.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                    } {
                                                                                        {
                                                                                            let finished_5 = self::finish_java_script_call_source_control_scope(finished_4.clone(), builder.clone(), target.clone().expect("a proven present value"), end_2, if (match (update_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } && match (iteration_source.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                start_2
                                                                                            } else {
                                                                                                update
                                                                                            });
                                                                                            finished_5.clone()
                                                                                        }
                                                                                    } else {
                                                                                        finished_4.clone()
                                                                                    };
                                                                                    self::lower_java_script_call_source_expression(after.clone(), bindings.clone(), finished_6.clone(), (depth + 1f64))
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
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(builder.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 100, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 98, 111, 100, 121])))
        }
    }
}
