// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallArrayPart;
use super::JavaScriptCallCellInitializer;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceArrayElement;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::refuse_java_script_call_source;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_constructor_expression_loop16(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values14: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index14: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Result {
    loop {
        return if (source_ml_source1_temporary_index14 < ({ let ml_reference_array227 = source_ml_source1_temporary_values14.clone(); let ml_reference_projection227 = self::ml::Big::from_u128(ml_reference_array227.borrow().len() as u128); ml_reference_projection227 })) {
            {
                let argument = { let ml_reference_array228 = source_ml_source1_temporary_values14.clone(); let ml_reference_position228 = source_ml_source1_temporary_index14.clone(); let ml_reference_projection228 = self::ml_array::at(&*ml_reference_array228.borrow(), ml_reference_position228.to_index()); ml_reference_projection228 };
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let saved = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15051, _) => {
                                let source_ml_source2_data_property_value15051 = *source_ml_source2_data_property_value15051;
                                source_ml_source2_data_property_value15051.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15052, _) => {
                                    let source_ml_source2_data_property_value15052 = *source_ml_source2_data_property_value15052;
                                    source_ml_source2_data_property_value15052.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15053) => {
                                    source_ml_source2_data_property_value15053
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15054) => {
                                    source_ml_source2_data_property_value15054
                                }
                            }));
                            {
                                let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value229 = arguments_.borrow().clone(); ml_reference_value229 }, vec![match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15055) => {
                                        source_ml_source2_data_property_value15055
                                    }
                                }])));
                                {
                                    let source_ml_source1_temporary_index14_2 = source_ml_source1_temporary_index14.add(&self::ml::Big::from_i128(1));
                                    {
                                        (current, arguments_, source_ml_source1_temporary_index14) = (current_2.clone(), arguments__2.clone(), source_ml_source1_temporary_index14_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Result::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Done(Box::new(current.clone()), arguments_.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Result {
    MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>),
    MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_lexical_cell_expression_loop17(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut realm: self::ml::Big, mut identities: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut existing: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut initializers: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>, mut current: self::JavaScriptCallSourceBuilder, mut entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut at: f64) -> self::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Result {
    loop {
        return if (at < ({ let ml_reference_array230 = identities.clone(); let ml_reference_projection230 = ml_reference_array230.borrow().len() as f64; ml_reference_projection230 })) {
            {
                let identity = ({ let ml_reference_array231 = identities.clone(); let ml_reference_position231 = at; let ml_reference_projection231 = self::ml_array::optional_at(&*ml_reference_array231.borrow(), self::ml_array::number_index(ml_reference_position231)); ml_reference_projection231 }).clone().unwrap_or_else(|| (-1f64));
                {
                    let binding = { let ml_values232 = { let ml_reference_value233 = bindings.borrow().clone(); ml_reference_value233 }; let ml_callback232 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value15059, _, _, _) => {
                            source_ml_source2_data_property_value15059
                        }
                    }) == identity) }) }; ml_values232.iter().cloned().enumerate().find_map(|(ml_index232, ml_value232)| if ml_callback232(ml_value232.clone()) { Some(ml_value232) } else { None }) };
                    {
                        let replace = self::ml_array::includes(&{ let ml_reference_value234 = existing.borrow().clone(); ml_reference_value234 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()));
                        if ((!(self::test_java_script_call_code_coordinate(identity, 65536f64)) || ((self::ml_array::index_of(&{ let ml_reference_value235 = identities.borrow().clone(); ml_reference_value235 }, &identity, 0f64, false, |a, b| a == b)) != at)) || if replace {
                            ((match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match binding.clone().expect("a proven present value") {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value15061, _, _) => {
                                    source_ml_source2_data_property_value15061
                                }
                            }) == (0f64))) || !(match binding.clone().expect("a proven present value") {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, source_ml_source2_data_property_value15062, _) => {
                                    source_ml_source2_data_property_value15062
                                }
                            }))
                        } else {
                            !(match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                        }) {
                            self::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Result::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Return(Box::new(self::refuse_java_script_call_source(current.clone(), "lexical cell coordinates retain distinct actual owned scope or formal bindings")))
                        } else {
                            {
                                let initial = { let ml_values236 = { let ml_reference_value237 = initializers.borrow().clone(); ml_reference_value237 }; let ml_callback236 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptCallCellInitializer| -> bool { ((match row.clone() {
                                    self::JavaScriptCallCellInitializer::SourceCellInitializer(source_ml_source2_data_property_value15064, _, _) => {
                                        source_ml_source2_data_property_value15064
                                    }
                                }) == identity) }) }; ml_values236.iter().cloned().enumerate().find_map(|(ml_index236, ml_value236)| if ml_callback236(ml_value236.clone()) { Some(ml_value236) } else { None }) };
                                {
                                    let missing = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Undefined));
                                    {
                                        let initial_value = if match (initial.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            missing.clone()
                                        } else {
                                            match initial.clone().expect("a proven present value") {
                                                self::JavaScriptCallCellInitializer::SourceCellInitializer(_, source_ml_source2_data_property_value15065, _) => {
                                                    let source_ml_source2_data_property_value15065 = *source_ml_source2_data_property_value15065;
                                                    source_ml_source2_data_property_value15065.clone()
                                                }
                                            }
                                        };
                                        {
                                            let initialized = if match (initial.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                false
                                            } else {
                                                match initial.clone().expect("a proven present value") {
                                                    self::JavaScriptCallCellInitializer::SourceCellInitializer(_, _, source_ml_source2_data_property_value15066) => {
                                                        source_ml_source2_data_property_value15066
                                                    }
                                                }
                                            };
                                            {
                                                let cell = {
                                                    let source_ml_source2_constructor_field15067 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Null));
                                                    {
                                                        let source_ml_source2_constructor_field15073 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                            let source_ml_source2_constructor_field15069 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![118f64, 97f64, 108f64, 117f64, 101f64])))));
                                                            self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field15069.clone()), Box::new(initial_value.clone()), None::<(Option<bool>,)>)
                                                        }, {
                                                            let source_ml_source2_constructor_field15071 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![105f64, 110f64, 105f64, 116f64, 105f64, 97f64, 108f64, 105f64, 122f64, 101f64, 100f64])))));
                                                            {
                                                                let source_ml_source2_constructor_field15072 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Boolean(initialized)));
                                                                self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field15071.clone()), Box::new(source_ml_source2_constructor_field15072.clone()), None::<(Option<bool>,)>)
                                                            }
                                                        }]));
                                                        self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field15067.clone()), source_ml_source2_constructor_field15073.clone(), realm.clone())
                                                    }
                                                };
                                                {
                                                    let allocated = self::lower_java_script_call_source_expression(cell.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                                                    {
                                                        let current_2 = match allocated.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15074, _) => {
                                                                let source_ml_source2_data_property_value15074 = *source_ml_source2_data_property_value15074;
                                                                source_ml_source2_data_property_value15074.clone()
                                                            }
                                                        };
                                                        if (replace && !(match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                            {
                                                                let current_3 = self::append_java_script_call_source_instruction(current_2.clone(), self::JavaScriptCallInstruction::Copy(match binding.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value15075, _, _) => {
                                                                        source_ml_source2_data_property_value15075
                                                                    }
                                                                }, match allocated.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15076) => {
                                                                        source_ml_source2_data_property_value15076
                                                                    }
                                                                }));
                                                                {
                                                                    let at_2 = (at + 1f64);
                                                                    {
                                                                        (current, at) = (current_3.clone(), at_2);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let entries_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value238 = entries.borrow().clone(); ml_reference_value238 }, vec![{
                                                                    let source_ml_source2_constructor_field15079 = match allocated.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15078) => {
                                                                            source_ml_source2_data_property_value15078
                                                                        }
                                                                    };
                                                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, source_ml_source2_constructor_field15079, false, None::<(Option<bool>,)>)
                                                                }])));
                                                                {
                                                                    let at_3 = (at + 1f64);
                                                                    {
                                                                        (current, entries, at) = (current_2.clone(), entries_2.clone(), at_3);
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
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Result::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Done(Box::new(current.clone()), entries.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result {
    MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Done,
    MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_binding_read_expression_loop18(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut state: self::JavaScriptCallSourceBuilder, mut identity: f64, mut index: f64) -> self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result {
    loop {
        return if (index >= (0f64)) {
            {
                let entry = { let ml_reference_array239 = bindings.clone(); let ml_reference_position239 = index; let ml_reference_projection239 = self::ml_array::optional_at(&*ml_reference_array239.borrow(), self::ml_array::number_index(ml_reference_position239)); ml_reference_projection239 };
                if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match entry.clone().expect("a proven present value") {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value15084, _, _, _) => {
                        source_ml_source2_data_property_value15084
                    }
                }) == identity)) {
                    if ({
                        let source_ml_source2_comparison_operand15086 = match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, _, source_ml_source2_data_property_value15085) => {
                                ((source_ml_source2_data_property_value15085.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                            }
                        };
                        match (source_ml_source2_comparison_operand15086.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    } && !(self::ml_array::includes(&{ let ml_reference_value240 = ((match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value15089, _, _) => {
                            ((source_ml_source2_data_property_value15089.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                        }
                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value240 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) {
                        self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Return(Box::new(self::refuse_java_script_call_source(state.clone(), "uninitialized binding reads require their actual temporal-dead-zone environment protocol")))
                    } else {
                        if ({
                            let source_ml_source2_comparison_operand15091 = match state.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value15090, _, _, _) => {
                                    ((source_ml_source2_data_property_value15090.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                }
                            };
                            match (source_ml_source2_comparison_operand15091.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        } && match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, source_ml_source2_data_property_value15093, _) => {
                                source_ml_source2_data_property_value15093
                            }
                        }) {
                            {
                                let saved = self::allocate_java_script_call_source_register(state.clone());
                                self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Return(Box::new({
                                    let source_ml_source2_nominal_field15098 = match saved.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15099, _) => {
                                            let source_ml_source2_data_property_value15099 = *source_ml_source2_data_property_value15099;
                                            source_ml_source2_data_property_value15099.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field15100 = match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15101) => {
                                                source_ml_source2_data_property_value15101
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override15102 = self::append_java_script_call_source_instruction(match saved.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15094, _) => {
                                                    let source_ml_source2_data_property_value15094 = *source_ml_source2_data_property_value15094;
                                                    source_ml_source2_data_property_value15094.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15095) => {
                                                    source_ml_source2_data_property_value15095
                                                }
                                            }, match entry.clone().expect("a proven present value") {
                                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value15096, _, _) => {
                                                    source_ml_source2_data_property_value15096
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override15102.clone()), source_ml_source2_nominal_field15100)
                                        }
                                    }
                                }))
                            }
                        } else {
                            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Return(Box::new(self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value15103, _, _) => {
                                    source_ml_source2_data_property_value15103
                                }
                            })))
                        }
                    }
                } else {
                    {
                        let index_2 = (index - 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Result {
    MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop28(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values26: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index26: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Result {
    loop {
        return if (source_ml_source1_temporary_index26 < ({ let ml_reference_array241 = source_ml_source1_temporary_values26.clone(); let ml_reference_projection241 = self::ml::Big::from_u128(ml_reference_array241.borrow().len() as u128); ml_reference_projection241 })) {
            {
                let argument = { let ml_reference_array242 = source_ml_source1_temporary_values26.clone(); let ml_reference_position242 = source_ml_source1_temporary_index26.clone(); let ml_reference_projection242 = self::ml_array::at(&*ml_reference_array242.borrow(), ml_reference_position242.to_index()); ml_reference_projection242 };
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15106, _) => {
                                let source_ml_source2_data_property_value15106 = *source_ml_source2_data_property_value15106;
                                source_ml_source2_data_property_value15106.clone()
                            }
                        };
                        {
                            let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value243 = arguments_.borrow().clone(); ml_reference_value243 }, vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15107) => {
                                    source_ml_source2_data_property_value15107
                                }
                            }])));
                            {
                                let source_ml_source1_temporary_index26_2 = source_ml_source1_temporary_index26.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index26) = (current_2.clone(), arguments__2.clone(), source_ml_source1_temporary_index26_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Result::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Done(Box::new(current.clone()), arguments_.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop30(mut current: self::JavaScriptCallSourceBuilder, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::JavaScriptCallSourceBuilder {
    loop {
        return if (index < ({ let ml_reference_array244 = arguments_.clone(); let ml_reference_projection244 = ml_reference_array244.borrow().len() as f64; ml_reference_projection244 })) {
            {
                let current_2 = self::append_java_script_call_source_instruction(current.clone(), self::JavaScriptCallInstruction::Copy((index + 1f64), ({ let ml_reference_array245 = arguments_.clone(); let ml_reference_position245 = index; let ml_reference_projection245 = self::ml_array::optional_at(&*ml_reference_array245.borrow(), self::ml_array::number_index(ml_reference_position245)); ml_reference_projection245 }).clone().unwrap_or_else(|| 0f64)));
                {
                    let index_2 = (index + 1f64);
                    {
                        (current, index) = (current_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Result {
    MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop29(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values27: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index27: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Result {
    loop {
        return if (source_ml_source1_temporary_index27 < ({ let ml_reference_array246 = source_ml_source1_temporary_values27.clone(); let ml_reference_projection246 = self::ml::Big::from_u128(ml_reference_array246.borrow().len() as u128); ml_reference_projection246 })) {
            {
                let argument = { let ml_reference_array247 = source_ml_source1_temporary_values27.clone(); let ml_reference_position247 = source_ml_source1_temporary_index27.clone(); let ml_reference_projection247 = self::ml_array::at(&*ml_reference_array247.borrow(), ml_reference_position247.to_index()); ml_reference_projection247 };
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let captured = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15112, _) => {
                                let source_ml_source2_data_property_value15112 = *source_ml_source2_data_property_value15112;
                                source_ml_source2_data_property_value15112.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15113, _) => {
                                    let source_ml_source2_data_property_value15113 = *source_ml_source2_data_property_value15113;
                                    source_ml_source2_data_property_value15113.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15114) => {
                                    source_ml_source2_data_property_value15114
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15115) => {
                                    source_ml_source2_data_property_value15115
                                }
                            }));
                            {
                                let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value248 = arguments_.borrow().clone(); ml_reference_value248 }, vec![match captured.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15116) => {
                                        source_ml_source2_data_property_value15116
                                    }
                                }])));
                                {
                                    let source_ml_source1_temporary_index27_2 = source_ml_source1_temporary_index27.add(&self::ml::Big::from_i128(1));
                                    {
                                        (current, arguments_, source_ml_source1_temporary_index27) = (current_2.clone(), arguments__2.clone(), source_ml_source1_temporary_index27_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Result::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Done(Box::new(current.clone()), arguments_.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result {
    MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Done(Box<self::JavaScriptCallSourceBuilder>),
    MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop33(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut realm: self::ml::Big, mut object: self::JavaScriptCallSourceResult, mut current: self::JavaScriptCallSourceBuilder, mut saved_key: self::JavaScriptCallSourceResult, mut index: self::JavaScriptCallSourceResult, mut source_ml_source1_temporary_values31: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArrayPart>>>, mut source_ml_source1_temporary_index31: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result {
    loop {
        return if (source_ml_source1_temporary_index31 < ({ let ml_reference_array249 = source_ml_source1_temporary_values31.clone(); let ml_reference_projection249 = self::ml::Big::from_u128(ml_reference_array249.borrow().len() as u128); ml_reference_projection249 })) {
            {
                let part = { let ml_reference_array250 = source_ml_source1_temporary_values31.clone(); let ml_reference_position250 = source_ml_source1_temporary_index31.clone(); let ml_reference_projection250 = self::ml_array::at(&*ml_reference_array250.borrow(), ml_reference_position250.to_index()); ml_reference_projection250 };
                if !(self::ml_array::includes(&{ let ml_reference_value251 = (std::rc::Rc::new(std::cell::RefCell::new(vec![String::from("sourceArraySpread"), String::from("sourceArrayValue"), String::from("sourceArrayHole")]))).borrow().clone(); ml_reference_value251 }, &match part.clone() {
                    self::JavaScriptCallArrayPart::SourceArrayValue(_) => {
                        String::from("sourceArrayValue")
                    }
                    self::JavaScriptCallArrayPart::SourceArraySpread(_) => {
                        String::from("sourceArraySpread")
                    }
                    self::JavaScriptCallArrayPart::SourceArrayHole => {
                        String::from("sourceArrayHole")
                    }
                }, 0f64, false, |a, b| a == b)) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Return(Box::new(self::refuse_java_script_call_source(current.clone(), "Array accumulation retains every actual part kind")))
                } else {
                    match part.clone() {
                        self::JavaScriptCallArrayPart::SourceArrayValue(source_ml_source2_f15119) => {
                            let source_ml_source2_f15119 = *source_ml_source2_f15119;
                            {
                                let current_3 = {
                                    let key = self::allocate_java_script_call_source_register(current.clone());
                                    {
                                        let keyed = self::append_java_script_call_source_instruction(match key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15121, _) => {
                                                let source_ml_source2_data_property_value15121 = *source_ml_source2_data_property_value15121;
                                                source_ml_source2_data_property_value15121.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::PropertyKey(match key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15122) => {
                                                source_ml_source2_data_property_value15122
                                            }
                                        }, match index.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15123) => {
                                                source_ml_source2_data_property_value15123
                                            }
                                        }, realm.clone()));
                                        {
                                            let value = self::lower_java_script_call_source_expression(source_ml_source2_f15119.clone(), bindings.clone(), keyed.clone(), (depth + 1f64));
                                            {
                                                let written = self::allocate_java_script_call_source_register(match value.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15124, _) => {
                                                        let source_ml_source2_data_property_value15124 = *source_ml_source2_data_property_value15124;
                                                        source_ml_source2_data_property_value15124.clone()
                                                    }
                                                });
                                                {
                                                    let current_2 = self::append_java_script_call_source_instruction(match written.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15125, _) => {
                                                            let source_ml_source2_data_property_value15125 = *source_ml_source2_data_property_value15125;
                                                            source_ml_source2_data_property_value15125.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryDataProperty(match written.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15126) => {
                                                            source_ml_source2_data_property_value15126
                                                        }
                                                    }, match object.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15127) => {
                                                            source_ml_source2_data_property_value15127
                                                        }
                                                    }, match key.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15128) => {
                                                            source_ml_source2_data_property_value15128
                                                        }
                                                    }, match value.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15129) => {
                                                            source_ml_source2_data_property_value15129
                                                        }
                                                    }, realm.clone()));
                                                    current_2.clone()
                                                }
                                            }
                                        }
                                    }
                                };
                                {
                                    let current_4 = self::append_java_script_call_source_instruction(current_3.clone(), self::JavaScriptCallInstruction::Primitive(match index.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15130) => {
                                            source_ml_source2_data_property_value15130
                                        }
                                    }, String::from("increment numeric value"), std::rc::Rc::new(std::cell::RefCell::new(vec![match index.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15131) => {
                                            source_ml_source2_data_property_value15131
                                        }
                                    }]))));
                                    {
                                        let source_ml_source1_temporary_index31_2 = source_ml_source1_temporary_index31.add(&self::ml::Big::from_i128(1));
                                        {
                                            (current, source_ml_source1_temporary_index31) = (current_4.clone(), source_ml_source1_temporary_index31_2.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallArrayPart::SourceArraySpread(value_2) => {
                            let value_2 = *value_2;
                            {
                                let input = self::lower_java_script_call_source_expression(value_2.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                                {
                                    let saved = self::allocate_java_script_call_source_register(match input.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15134, _) => {
                                            let source_ml_source2_data_property_value15134 = *source_ml_source2_data_property_value15134;
                                            source_ml_source2_data_property_value15134.clone()
                                        }
                                    });
                                    {
                                        let held = self::append_java_script_call_source_instruction(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15135, _) => {
                                                let source_ml_source2_data_property_value15135 = *source_ml_source2_data_property_value15135;
                                                source_ml_source2_data_property_value15135.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15136) => {
                                                source_ml_source2_data_property_value15136
                                            }
                                        }, match input.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15137) => {
                                                source_ml_source2_data_property_value15137
                                            }
                                        }));
                                        {
                                            let current_5 = self::append_java_script_call_source_instruction(held.clone(), self::JavaScriptCallInstruction::ArraySpread(match index.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15138) => {
                                                    source_ml_source2_data_property_value15138
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15139) => {
                                                    source_ml_source2_data_property_value15139
                                                }
                                            }, match saved.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15140) => {
                                                    source_ml_source2_data_property_value15140
                                                }
                                            }, match index.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15141) => {
                                                    source_ml_source2_data_property_value15141
                                                }
                                            }, match saved_key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15142) => {
                                                    source_ml_source2_data_property_value15142
                                                }
                                            }, realm.clone()));
                                            {
                                                let source_ml_source1_temporary_index31_3 = source_ml_source1_temporary_index31.add(&self::ml::Big::from_i128(1));
                                                {
                                                    (current, source_ml_source1_temporary_index31) = (current_5.clone(), source_ml_source1_temporary_index31_3.clone());
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallArrayPart::SourceArrayHole => {
                            {
                                let current_6 = current.clone();
                                {
                                    let current_7 = self::append_java_script_call_source_instruction(current_6.clone(), self::JavaScriptCallInstruction::Primitive(match index.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15144) => {
                                            source_ml_source2_data_property_value15144
                                        }
                                    }, String::from("increment numeric value"), std::rc::Rc::new(std::cell::RefCell::new(vec![match index.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15145) => {
                                            source_ml_source2_data_property_value15145
                                        }
                                    }]))));
                                    {
                                        let key_2 = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64]))))), bindings.clone(), current_7.clone(), (depth + 1f64));
                                        {
                                            let written_2 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15148, _) => {
                                                    let source_ml_source2_data_property_value15148 = *source_ml_source2_data_property_value15148;
                                                    source_ml_source2_data_property_value15148.clone()
                                                }
                                            });
                                            {
                                                let current_8 = self::append_java_script_call_source_instruction(match written_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15149, _) => {
                                                        let source_ml_source2_data_property_value15149 = *source_ml_source2_data_property_value15149;
                                                        source_ml_source2_data_property_value15149.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::OrdinarySet(match written_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15150) => {
                                                        source_ml_source2_data_property_value15150
                                                    }
                                                }, match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15151) => {
                                                        source_ml_source2_data_property_value15151
                                                    }
                                                }, match key_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15152) => {
                                                        source_ml_source2_data_property_value15152
                                                    }
                                                }, match index.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15153) => {
                                                        source_ml_source2_data_property_value15153
                                                    }
                                                }, match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15154) => {
                                                        source_ml_source2_data_property_value15154
                                                    }
                                                }));
                                                {
                                                    let current_9 = self::append_java_script_call_source_instruction(current_8.clone(), self::JavaScriptCallInstruction::RequireWrite(match written_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15155) => {
                                                            source_ml_source2_data_property_value15155
                                                        }
                                                    }));
                                                    {
                                                        let source_ml_source1_temporary_index31_4 = source_ml_source1_temporary_index31.add(&self::ml::Big::from_i128(1));
                                                        {
                                                            (current, source_ml_source1_temporary_index31) = (current_9.clone(), source_ml_source1_temporary_index31_4.clone());
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
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Done(Box::new(current.clone()))
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result {
    MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Done(Box<self::JavaScriptCallSourceBuilder>),
    MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_array_expression_loop34(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut realm: self::ml::Big, mut length: f64, mut object: self::JavaScriptCallSourceResult, mut current: self::JavaScriptCallSourceBuilder, mut previous: f64, mut source_ml_source1_temporary_values32: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceArrayElement>>>, mut source_ml_source1_temporary_index32: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result {
    loop {
        return if (source_ml_source1_temporary_index32 < ({ let ml_reference_array252 = source_ml_source1_temporary_values32.clone(); let ml_reference_projection252 = self::ml::Big::from_u128(ml_reference_array252.borrow().len() as u128); ml_reference_projection252 })) {
            {
                let element = { let ml_reference_array253 = source_ml_source1_temporary_values32.clone(); let ml_reference_position253 = source_ml_source1_temporary_index32.clone(); let ml_reference_projection253 = self::ml_array::at(&*ml_reference_array253.borrow(), ml_reference_position253.to_index()); ml_reference_projection253 };
                if (((!(self::ml_math::is_safe_integer(match element.clone() {
                    self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value15158, _) => {
                        source_ml_source2_data_property_value15158
                    }
                })) || {
                    let source_ml_source2_same_value_operand15160 = match element.clone() {
                        self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value15159, _) => {
                            source_ml_source2_data_property_value15159
                        }
                    };
                    {
                        let source_ml_source2_same_value_operand15161 = (-0f64);
                        (((source_ml_source2_same_value_operand15160 == source_ml_source2_same_value_operand15161) && ((source_ml_source2_same_value_operand15160 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand15160)) == ((1f64 / source_ml_source2_same_value_operand15161))))) || ((source_ml_source2_same_value_operand15160 != source_ml_source2_same_value_operand15160) && (source_ml_source2_same_value_operand15161 != source_ml_source2_same_value_operand15161)))
                    }
                }) || ((match element.clone() {
                    self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value15163, _) => {
                        source_ml_source2_data_property_value15163
                    }
                }) <= previous)) || ((match element.clone() {
                    self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value15165, _) => {
                        source_ml_source2_data_property_value15165
                    }
                }) >= length)) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Return(Box::new(self::refuse_java_script_call_source(current.clone(), "source Array elements retain their actual strictly increasing literal coordinates")))
                } else {
                    {
                        let previous_2 = match element.clone() {
                            self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value15166, _) => {
                                source_ml_source2_data_property_value15166
                            }
                        };
                        {
                            let key = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourcePropertyKey(Box::new(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Number(match element.clone() {
                                self::JavaScriptCallSourceArrayElement::SourceArrayElement(source_ml_source2_data_property_value15167, _) => {
                                    source_ml_source2_data_property_value15167
                                }
                            })))), realm.clone()), bindings.clone(), current.clone(), (depth + 1f64));
                            {
                                let value = self::lower_java_script_call_source_expression(match element.clone() {
                                    self::JavaScriptCallSourceArrayElement::SourceArrayElement(_, source_ml_source2_data_property_value15168) => {
                                        let source_ml_source2_data_property_value15168 = *source_ml_source2_data_property_value15168;
                                        source_ml_source2_data_property_value15168.clone()
                                    }
                                }, bindings.clone(), match key.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15169, _) => {
                                        let source_ml_source2_data_property_value15169 = *source_ml_source2_data_property_value15169;
                                        source_ml_source2_data_property_value15169.clone()
                                    }
                                }, (depth + 1f64));
                                {
                                    let result = self::allocate_java_script_call_source_register(match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15170, _) => {
                                            let source_ml_source2_data_property_value15170 = *source_ml_source2_data_property_value15170;
                                            source_ml_source2_data_property_value15170.clone()
                                        }
                                    });
                                    {
                                        let current_2 = self::append_java_script_call_source_instruction(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15171, _) => {
                                                let source_ml_source2_data_property_value15171 = *source_ml_source2_data_property_value15171;
                                                source_ml_source2_data_property_value15171.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::OrdinaryDataProperty(match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15172) => {
                                                source_ml_source2_data_property_value15172
                                            }
                                        }, match object.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15173) => {
                                                source_ml_source2_data_property_value15173
                                            }
                                        }, match key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15174) => {
                                                source_ml_source2_data_property_value15174
                                            }
                                        }, match value.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15175) => {
                                                source_ml_source2_data_property_value15175
                                            }
                                        }, realm.clone()));
                                        {
                                            let source_ml_source1_temporary_index32_2 = source_ml_source1_temporary_index32.add(&self::ml::Big::from_i128(1));
                                            {
                                                (current, previous, source_ml_source1_temporary_index32) = (current_2.clone(), previous_2, source_ml_source1_temporary_index32_2.clone());
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
            self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop34Done(Box::new(current.clone()))
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Result {
    MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Done(Box<self::JavaScriptCallSourceBuilder>),
    MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop41Return(Box<self::JavaScriptCallSourceResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Result {
    MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_object_expression_loop42(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut realm: self::ml::Big, mut current: self::JavaScriptCallSourceBuilder, mut excluded: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values36: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index36: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Result {
    loop {
        return if (source_ml_source1_temporary_index36 < ({ let ml_reference_array254 = source_ml_source1_temporary_values36.clone(); let ml_reference_projection254 = self::ml::Big::from_u128(ml_reference_array254.borrow().len() as u128); ml_reference_projection254 })) {
            {
                let expression = { let ml_reference_array255 = source_ml_source1_temporary_values36.clone(); let ml_reference_position255 = source_ml_source1_temporary_index36.clone(); let ml_reference_projection255 = self::ml_array::at(&*ml_reference_array255.borrow(), ml_reference_position255.to_index()); ml_reference_projection255 };
                {
                    let key = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourcePropertyKey(Box::new(expression.clone()), realm.clone()), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let saved = self::allocate_java_script_call_source_register(match key.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15178, _) => {
                                let source_ml_source2_data_property_value15178 = *source_ml_source2_data_property_value15178;
                                source_ml_source2_data_property_value15178.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15179, _) => {
                                    let source_ml_source2_data_property_value15179 = *source_ml_source2_data_property_value15179;
                                    source_ml_source2_data_property_value15179.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15180) => {
                                    source_ml_source2_data_property_value15180
                                }
                            }, match key.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15181) => {
                                    source_ml_source2_data_property_value15181
                                }
                            }));
                            {
                                let excluded_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value256 = excluded.borrow().clone(); ml_reference_value256 }, vec![match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15182) => {
                                        source_ml_source2_data_property_value15182
                                    }
                                }])));
                                {
                                    let source_ml_source1_temporary_index36_2 = source_ml_source1_temporary_index36.add(&self::ml::Big::from_i128(1));
                                    {
                                        (current, excluded, source_ml_source1_temporary_index36) = (current_2.clone(), excluded_2.clone(), source_ml_source1_temporary_index36_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Result::MlSource1TemporaryLowerJavaScriptCallSourceObjectExpressionLoop42Done(Box::new(current.clone()), excluded.clone())
        };
    }
}
