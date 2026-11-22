// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::finish_java_script_call_source_control_scope;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call_source_control_scope;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_find_java_script_call_source_instruction_position_loop65;
use super::source_ml_source1_temporary_lower_java_script_call_source_switch_loop63;
use super::source_ml_source1_temporary_lower_java_script_call_source_switch_loop64;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_switch(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, builder: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    {
        let directory_error = self::ml_string::Text::from_units(vec![115, 119, 105, 116, 99, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 97, 115, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]);
        match source.clone() {
            self::JavaScriptCallSource::SourceConstant(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceNewTarget => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceReceiver => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceArgument(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceDerivedReceiver => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceErrorIntrinsic => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceConstruct(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceThrow(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceReturn(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceControlJump(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceSwitch(target, value, clauses, after) => {
                let value = *value;
                let after = *after;
                if (({ let ml_reference_array173 = clauses.clone(); let ml_reference_projection173 = ml_reference_array173.borrow().len() as f64; ml_reference_projection173 }) > (4096f64)) {
                    self::refuse_java_script_call_source(builder.clone(), &(directory_error))
                } else {
                    {
                        let input = self::lower_java_script_call_source_expression(value.clone(), bindings.clone(), builder.clone(), (depth + 1f64));
                        {
                            let saved = self::allocate_java_script_call_source_register(match input.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13685, _) => {
                                    let source_ml_source2_data_property_value13685 = *source_ml_source2_data_property_value13685;
                                    source_ml_source2_data_property_value13685.clone()
                                }
                            });
                            {
                                let current = self::append_java_script_call_source_instruction(match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value13686, _) => {
                                        let source_ml_source2_data_property_value13686 = *source_ml_source2_data_property_value13686;
                                        source_ml_source2_data_property_value13686.clone()
                                    }
                                }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value13687) => {
                                        source_ml_source2_data_property_value13687
                                    }
                                }, match input.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value13688) => {
                                        source_ml_source2_data_property_value13688
                                    }
                                }));
                                {
                                    let current_2 = self::push_java_script_call_source_control_scope(current.clone(), target, &(self::ml_string::Text::from_units(vec![115, 119, 105, 116, 99, 104])));
                                    {
                                        let branches = std::rc::Rc::new(std::cell::RefCell::new(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))));
                                        {
                                            let fallback = (-1f64);
                                            {
                                                let index = 0f64;
                                                {
                                                    let source_ml_source2_s13689 = self::source_ml_source1_temporary_lower_java_script_call_source_switch_loop63(source.clone(), bindings.clone(), depth, saved.clone(), current_2.clone(), branches.clone(), fallback, index);
                                                    match source_ml_source2_s13689.clone() {
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Done(current_3, fallback_2) => {
                                                            let current_3 = *current_3;
                                                            {
                                                                let dispatch = { let ml_reference_array174 = match current_3.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13690, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value13690.clone()
                                                                    }
                                                                }; let ml_reference_projection174 = ml_reference_array174.borrow().len() as f64; ml_reference_projection174 };
                                                                {
                                                                    let current_4 = self::append_java_script_call_source_instruction(current_3.clone(), self::JavaScriptCallInstruction::Jump(0f64));
                                                                    {
                                                                        let bodies = std::rc::Rc::new(std::cell::RefCell::new(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))));
                                                                        {
                                                                            let source_ml_source1_temporary_values62 = clauses.clone();
                                                                            {
                                                                                let source_ml_source1_temporary_index62 = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let current_5 = self::source_ml_source1_temporary_lower_java_script_call_source_switch_loop64(bindings.clone(), depth, current_4.clone(), bodies.clone(), source_ml_source1_temporary_values62.clone(), source_ml_source1_temporary_index62.clone());
                                                                                    {
                                                                                        let end = { let ml_reference_array175 = match current_5.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13691, _, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value13691.clone()
                                                                                            }
                                                                                        }; let ml_reference_projection175 = ml_reference_array175.borrow().len() as f64; ml_reference_projection175 };
                                                                                        {
                                                                                            let positions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values176 = { let ml_reference_value177 = ({ let ml_cell_value = branches.borrow().clone(); ml_cell_value }).borrow().clone(); ml_reference_value177 }; let ml_callback176 = std::rc::Rc::new(move |position: f64| -> bool { (position >= (0f64)) }); ml_values176.iter().cloned().enumerate().filter_map(|(ml_index176, ml_value176)| if ml_callback176(ml_value176.clone()) { Some(ml_value176) } else { None }).collect::<Vec<_>>() }));
                                                                                            {
                                                                                                let indices = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values178 = { let ml_reference_value179 = (std::rc::Rc::new(std::cell::RefCell::new({ let ml_values180 = { let ml_reference_value181 = ({ let ml_cell_value = branches.borrow().clone(); ml_cell_value }).borrow().clone(); ml_reference_value181 }; let ml_callback180 = std::rc::Rc::new(move |x_: f64, index_2: f64| -> f64 { index_2 }); ml_values180.iter().cloned().enumerate().map(|(ml_index180, ml_value180)| ml_callback180(ml_value180, (ml_index180 as f64))).collect::<Vec<_>>() }))).borrow().clone(); ml_reference_value179 }; let ml_callback178 = { let branches = branches.clone(); std::rc::Rc::new(move |index_2: f64| -> bool { ((({
                                                                                                    let source_ml_source2_array_read_value13694 = { let ml_cell_value = branches.borrow().clone(); ml_cell_value };
                                                                                                    {
                                                                                                        let source_ml_source2_array_read_index13695 = index_2;
                                                                                                        { let ml_reference_array182 = source_ml_source2_array_read_value13694.clone(); let ml_reference_position182 = source_ml_source2_array_read_index13695; let ml_reference_projection182 = self::ml_array::optional_at(&*ml_reference_array182.borrow(), self::ml_array::number_index(ml_reference_position182)); ml_reference_projection182 }
                                                                                                    }
                                                                                                }).clone().unwrap_or_else(|| (-1f64))) >= (0f64)) }) }; ml_values178.iter().cloned().enumerate().filter_map(|(ml_index178, ml_value178)| if ml_callback178(ml_value178.clone()) { Some(ml_value178) } else { None }).collect::<Vec<_>>() }));
                                                                                                {
                                                                                                    let fallback_target = if (fallback_2 < (0f64)) {
                                                                                                        end
                                                                                                    } else {
                                                                                                        ({
                                                                                                            let source_ml_source2_array_read_value13696 = { let ml_cell_value = bodies.borrow().clone(); ml_cell_value };
                                                                                                            { let ml_reference_array183 = source_ml_source2_array_read_value13696.clone(); let ml_reference_position183 = fallback_2; let ml_reference_projection183 = self::ml_array::optional_at(&*ml_reference_array183.borrow(), self::ml_array::number_index(ml_reference_position183)); ml_reference_projection183 }
                                                                                                        }).clone().unwrap_or_else(|| end)
                                                                                                    };
                                                                                                    {
                                                                                                        let current_6 = {
                                                                                                            let source_ml_source2_nominal_spread13950 = current_5.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field13951 = match source_ml_source2_nominal_spread13950.clone() {
                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13952, _, _, _, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value13952.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field13953 = match source_ml_source2_nominal_spread13950.clone() {
                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value13954, _, _, _, _, _, _, _, _) => {
                                                                                                                            source_ml_source2_data_property_value13954
                                                                                                                        }
                                                                                                                    };
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field13955 = match source_ml_source2_nominal_spread13950.clone() {
                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value13956, _, _, _, _, _, _, _) => {
                                                                                                                                source_ml_source2_data_property_value13956
                                                                                                                            }
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field13957 = match source_ml_source2_nominal_spread13950.clone() {
                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value13958, _, _, _, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value13958
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field13959 = match source_ml_source2_nominal_spread13950.clone() {
                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value13960, _, _, _, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value13960
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field13961 = match source_ml_source2_nominal_spread13950.clone() {
                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value13962, _, _, _, _) => {
                                                                                                                                            source_ml_source2_data_property_value13962.clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field13963 = match source_ml_source2_nominal_spread13950.clone() {
                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value13964, _, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value13964.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field13965 = match source_ml_source2_nominal_spread13950.clone() {
                                                                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value13966, _, _) => {
                                                                                                                                                    source_ml_source2_data_property_value13966.clone()
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field13967 = match source_ml_source2_nominal_spread13950.clone() {
                                                                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13968, _) => {
                                                                                                                                                        source_ml_source2_data_property_value13968.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field13969 = match source_ml_source2_nominal_spread13950.clone() {
                                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value13970) => {
                                                                                                                                                            source_ml_source2_data_property_value13970.clone()
                                                                                                                                                        }
                                                                                                                                                    };
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_override13971 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values184 = { let ml_reference_value185 = (match current_5.clone() {
                                                                                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13698, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                source_ml_source2_data_property_value13698.clone()
                                                                                                                                                            }
                                                                                                                                                        }).borrow().clone(); ml_reference_value185 }; let ml_callback184 = { let dispatch = dispatch; let fallback_target = fallback_target; let positions = positions.clone(); let indices = indices.clone(); let bodies = bodies.clone(); let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index_2: f64| -> self::JavaScriptCallInstruction { if (index_2 == dispatch) {
                                                                                                                                                            self::JavaScriptCallInstruction::Jump(fallback_target)
                                                                                                                                                        } else {
                                                                                                                                                            {
                                                                                                                                                                let selected = self::find_java_script_call_source_instruction_position(positions.clone(), index_2);
                                                                                                                                                                if (selected < (0f64)) {
                                                                                                                                                                    instruction.clone()
                                                                                                                                                                } else {
                                                                                                                                                                    {
                                                                                                                                                                        let clause = {
                                                                                                                                                                            let source_ml_source2_array_read_value13699 = indices.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_array_read_index13700 = selected;
                                                                                                                                                                                { let ml_reference_array186 = source_ml_source2_array_read_value13699.clone(); let ml_reference_position186 = source_ml_source2_array_read_index13700; let ml_reference_projection186 = self::ml_array::optional_at(&*ml_reference_array186.borrow(), self::ml_array::number_index(ml_reference_position186)); ml_reference_projection186 }
                                                                                                                                                                            }
                                                                                                                                                                        };
                                                                                                                                                                        if {
                                                                                                                                                                            let source_ml_source2_comparison_operand13701 = clause.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_comparison_operand13702 = None::<()>;
                                                                                                                                                                                match (source_ml_source2_comparison_operand13701.clone(), source_ml_source2_comparison_operand13702.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                                                                                                            }
                                                                                                                                                                        } {
                                                                                                                                                                            instruction.clone()
                                                                                                                                                                        } else {
                                                                                                                                                                            match instruction.clone() {
                                                                                                                                                                                self::JavaScriptCallInstruction::Constant(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Copy(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Argument(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArgumentCount(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArgumentAt(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::FunctionBind(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::BoundFunctionCall(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::FunctionToString(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PushFinally(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::FinishFinally => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CompleteJump(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Jump(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Branch(condition, _, no) => {
                                                                                                                                                                                    self::JavaScriptCallInstruction::Branch(condition, ({
                                                                                                                                                                                        let source_ml_source2_array_read_value13948 = { let ml_cell_value = bodies.borrow().clone(); ml_cell_value };
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_source2_array_read_index13949 = clause.clone().expect("a proven present value");
                                                                                                                                                                                            { let ml_reference_array187 = source_ml_source2_array_read_value13948.clone(); let ml_reference_position187 = source_ml_source2_array_read_index13949; let ml_reference_projection187 = self::ml_array::optional_at(&*ml_reference_array187.borrow(), self::ml_array::number_index(ml_reference_position187)); ml_reference_projection187 }
                                                                                                                                                                                        }
                                                                                                                                                                                    }).clone().unwrap_or_else(|| end), no)
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArgumentListOpen(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArgumentListAppend(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArgumentListCall(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Construct(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ConstructorCheck(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ReadNewTarget(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayJoinEnter(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayJoinLeave(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ConstructErrorClass(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CallClassConstructor(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ConstructCollection(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CollectionAllocate(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArgumentRest(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CollectionOperation(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PrimitiveMethod(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CollectionForEach(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayConcatenation(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArraySlice(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayCreation(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayCallback(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArraySearch(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayTransform(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ObjectEnumeration(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PropertyDefinitions(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PropertyReflection(_, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::InstanceOf(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ObjectObservation(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::CollectionCopy(_, _, _, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ReadDerivedReceiver(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::InvokeErrorIntrinsic(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::InvokeHostCapability(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Return(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Throw(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PushHandler(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PopHandler => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::RequireWrite(_) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::Primitive(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinaryAllocate(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::LiteralPrototype(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::LiteralAccessor(_, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ObjectFromEntries(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PrimitiveWrapper(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::SymbolAllocate(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::SymbolDescription(_, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::SymbolRegistry(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::FunctionName(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArrayAllocate(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ClosureAllocate(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ArraySpread(_, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::ObjectSpread(_, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinaryDataProperty(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinaryDelete(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinaryHas(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::OrdinaryDefine(_, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                                self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _, _) => {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        } }) }; ml_values184.iter().cloned().enumerate().map(|(ml_index184, ml_value184)| ml_callback184(ml_value184, (ml_index184 as f64))).collect::<Vec<_>>() }));
                                                                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_override13971.clone(), source_ml_source2_nominal_field13953, source_ml_source2_nominal_field13955, source_ml_source2_nominal_field13957, source_ml_source2_nominal_field13959, source_ml_source2_nominal_field13961.clone(), source_ml_source2_nominal_field13963.clone(), source_ml_source2_nominal_field13965.clone(), source_ml_source2_nominal_field13967.clone(), source_ml_source2_nominal_field13969.clone())
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
                                                                                                            let current_7 = self::finish_java_script_call_source_control_scope(current_6.clone(), builder.clone(), target, end, (-1f64));
                                                                                                            self::lower_java_script_call_source_expression(after.clone(), bindings.clone(), current_7.clone(), (depth + 1f64))
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
                                                        self::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Return(source_ml_source1_temporary_result64) => {
                                                            let source_ml_source1_temporary_result64 = *source_ml_source1_temporary_result64;
                                                            source_ml_source1_temporary_result64.clone()
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
            self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceFinally(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceTry(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceBinding(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceLet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceIf(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceSameValue(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceRecur(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceAllocate(_) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceObject(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceSet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceGet(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceDelete(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceHas(_, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
            self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
                self::refuse_java_script_call_source(builder.clone(), &(directory_error))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_source_instruction_position(positions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, position: f64) -> f64 {
    {
        let low = 0f64;
        {
            let high = { let ml_reference_array188 = positions.clone(); let ml_reference_projection188 = ml_reference_array188.borrow().len() as f64; ml_reference_projection188 };
            {
                let source_ml_source2_s13972 = self::source_ml_source1_temporary_find_java_script_call_source_instruction_position_loop65(positions.clone(), position, low, high);
                match source_ml_source2_s13972.clone() {
                    self::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Result::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Result::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Return(source_ml_source1_temporary_result65) => {
                        source_ml_source1_temporary_result65
                    }
                }
            }
        }
    }
}
