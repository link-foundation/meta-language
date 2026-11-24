// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallArrayPart;
use super::JavaScriptCallCellInitializer;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::compile_java_script_call_source;
use super::lower_java_script_call_source_expression;
use super::lower_java_script_call_source_operand;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_plan_java_script_natural_pattern_branches_loop82;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_natural_cast_plan(from: &self::ml_string::Text, to: &self::ml_string::Text, flavor: &self::ml_string::Text, retained_message: bool, order: f64, one_realm: bool) -> f64 {
    if ((((*from) == (self::ml_string::Text::from_units(vec![110, 97, 116]))) && ((*to) == (self::ml_string::Text::from_units(vec![105, 110, 116])))) && ((*flavor) == (self::ml_string::Text::from_units(vec![101, 120, 97, 99, 116])))) {
        1f64
    } else {
        if (((*from) != (self::ml_string::Text::from_units(vec![105, 110, 116]))) || ((*to) != (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
            0f64
        } else {
            if ((*flavor) == (self::ml_string::Text::from_units(vec![99, 108, 97, 109, 112]))) {
                2f64
            } else {
                if ((((((*flavor) == (self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100]))) && retained_message) && self::ml_math::is_safe_integer(order)) && (order >= (0f64))) && one_realm) {
                    3f64
                } else {
                    0f64
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_natural_pattern_binding(pattern: &self::ml_string::Text) -> f64 {
    if ((*pattern) == (self::ml_string::Text::from_units(vec![110, 97, 116, 83, 117, 99, 99]))) {
        2f64
    } else {
        if ((*pattern) == (self::ml_string::Text::from_units(vec![98, 105, 110, 100]))) {
            1f64
        } else {
            if (((*pattern) == (self::ml_string::Text::from_units(vec![110, 97, 116, 90, 101, 114, 111]))) || ((*pattern) == (self::ml_string::Text::from_units(vec![119, 105, 108, 100])))) {
                0f64
            } else {
                (-1f64)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_natural_pattern_branches(kind: &self::ml_string::Text, patterns: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (((*kind) != (self::ml_string::Text::from_units(vec![110, 97, 116]))) || (({ let ml_reference_array228 = patterns.clone(); let ml_reference_projection228 = ml_reference_array228.borrow().len() as f64; ml_reference_projection228 }) > (3f64))) {
        std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
    } else {
        {
            let zero = (-1f64);
            {
                let succ = (-1f64);
                {
                    let fallback = (-1f64);
                    {
                        let index = 0f64;
                        {
                            let source_ml_source2_s14850 = self::source_ml_source1_temporary_plan_java_script_natural_pattern_branches_loop82(patterns.clone(), zero, succ, fallback, index);
                            match source_ml_source2_s14850.clone() {
                                self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Done(zero_2, succ_2, fallback_2) => {
                                    {
                                        let zero_3 = if (zero_2 == ((-1f64))) {
                                            fallback_2
                                        } else {
                                            zero_2
                                        };
                                        {
                                            let succ_3 = if (succ_2 == ((-1f64))) {
                                                fallback_2
                                            } else {
                                                succ_2
                                            };
                                            if ((zero_3 < (0f64)) || (succ_3 < (0f64))) {
                                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
                                            } else {
                                                std::rc::Rc::new(std::cell::RefCell::new(vec![zero_3, succ_3]))
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Return(source_ml_source1_temporary_result82) => {
                                    source_ml_source1_temporary_result82.clone()
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
pub fn compile_java_script_natural_source(source: self::JavaScriptCallSource, parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> self::JavaScriptCallSourceCompilation {
    self::compile_java_script_call_source(source.clone(), parameters.clone())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ({ let ml_reference_array229 = units.clone(); let ml_reference_projection229 = ml_reference_array229.borrow().len() as f64; ml_reference_projection229 })) {
            {
                let unit = ({ let ml_reference_array230 = units.clone(); let ml_reference_position230 = at; let ml_reference_projection230 = self::ml_array::optional_at(&*ml_reference_array230.borrow(), self::ml_array::number_index(ml_reference_position230)); ml_reference_projection230 }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
        } else {
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Result {
    MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_constructor_expression_loop15(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut snapshot: bool, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values13: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index13: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Result {
    loop {
        return if (source_ml_source1_temporary_index13 < ({ let ml_reference_array231 = source_ml_source1_temporary_values13.clone(); let ml_reference_projection231 = self::ml::Big::from_u128(ml_reference_array231.borrow().len() as u128); ml_reference_projection231 })) {
            {
                let argument = { let ml_reference_array232 = source_ml_source1_temporary_values13.clone(); let ml_reference_position232 = source_ml_source1_temporary_index13.clone(); let ml_reference_projection232 = self::ml_array::at(&*ml_reference_array232.borrow(), ml_reference_position232.to_index()); ml_reference_projection232 };
                {
                    let value = self::lower_java_script_call_source_operand(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64), snapshot);
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15053, _) => {
                                let source_ml_source2_data_property_value15053 = *source_ml_source2_data_property_value15053;
                                source_ml_source2_data_property_value15053.clone()
                            }
                        };
                        {
                            let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value233 = arguments_.borrow().clone(); ml_reference_value233 }, vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15054) => {
                                    source_ml_source2_data_property_value15054
                                }
                            }])));
                            {
                                let source_ml_source1_temporary_index13_2 = source_ml_source1_temporary_index13.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index13) = (current_2.clone(), arguments__2.clone(), source_ml_source1_temporary_index13_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Result::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Done(Box::new(current.clone()), arguments_.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Result {
    MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_constructor_expression_loop16(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values14: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index14: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Result {
    loop {
        return if (source_ml_source1_temporary_index14 < ({ let ml_reference_array234 = source_ml_source1_temporary_values14.clone(); let ml_reference_projection234 = self::ml::Big::from_u128(ml_reference_array234.borrow().len() as u128); ml_reference_projection234 })) {
            {
                let argument = { let ml_reference_array235 = source_ml_source1_temporary_values14.clone(); let ml_reference_position235 = source_ml_source1_temporary_index14.clone(); let ml_reference_projection235 = self::ml_array::at(&*ml_reference_array235.borrow(), ml_reference_position235.to_index()); ml_reference_projection235 };
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let saved = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15057, _) => {
                                let source_ml_source2_data_property_value15057 = *source_ml_source2_data_property_value15057;
                                source_ml_source2_data_property_value15057.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15058, _) => {
                                    let source_ml_source2_data_property_value15058 = *source_ml_source2_data_property_value15058;
                                    source_ml_source2_data_property_value15058.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15059) => {
                                    source_ml_source2_data_property_value15059
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15060) => {
                                    source_ml_source2_data_property_value15060
                                }
                            }));
                            {
                                let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value236 = arguments_.borrow().clone(); ml_reference_value236 }, vec![match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15061) => {
                                        source_ml_source2_data_property_value15061
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
        return if (at < ({ let ml_reference_array237 = identities.clone(); let ml_reference_projection237 = ml_reference_array237.borrow().len() as f64; ml_reference_projection237 })) {
            {
                let identity = ({ let ml_reference_array238 = identities.clone(); let ml_reference_position238 = at; let ml_reference_projection238 = self::ml_array::optional_at(&*ml_reference_array238.borrow(), self::ml_array::number_index(ml_reference_position238)); ml_reference_projection238 }).clone().unwrap_or_else(|| (-1f64));
                {
                    let binding = { let ml_values239 = { let ml_reference_value240 = bindings.borrow().clone(); ml_reference_value240 }; let ml_callback239 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value15065, _, _, _) => {
                            source_ml_source2_data_property_value15065
                        }
                    }) == identity) }) }; ml_values239.iter().cloned().enumerate().find_map(|(ml_index239, ml_value239)| if ml_callback239(ml_value239.clone()) { Some(ml_value239) } else { None }) };
                    {
                        let replace = self::ml_array::includes(&{ let ml_reference_value241 = existing.borrow().clone(); ml_reference_value241 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()));
                        if ((!(self::test_java_script_call_code_coordinate(identity, 65536f64)) || ((self::ml_array::index_of(&{ let ml_reference_value242 = identities.borrow().clone(); ml_reference_value242 }, &identity, 0f64, false, |a, b| a == b)) != at)) || if replace {
                            ((match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match binding.clone().expect("a proven present value") {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value15067, _, _) => {
                                    source_ml_source2_data_property_value15067
                                }
                            }) == (0f64))) || !(match binding.clone().expect("a proven present value") {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, source_ml_source2_data_property_value15068, _) => {
                                    source_ml_source2_data_property_value15068
                                }
                            }))
                        } else {
                            !(match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                        }) {
                            self::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Result::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Return(Box::new(self::refuse_java_script_call_source(current.clone(), &(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 99, 101, 108, 108, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 115, 99, 111, 112, 101, 32, 111, 114, 32, 102, 111, 114, 109, 97, 108, 32, 98, 105, 110, 100, 105, 110, 103, 115])))))
                        } else {
                            {
                                let initial = { let ml_values243 = { let ml_reference_value244 = initializers.borrow().clone(); ml_reference_value244 }; let ml_callback243 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptCallCellInitializer| -> bool { ((match row.clone() {
                                    self::JavaScriptCallCellInitializer::SourceCellInitializer(source_ml_source2_data_property_value15070, _, _) => {
                                        source_ml_source2_data_property_value15070
                                    }
                                }) == identity) }) }; ml_values243.iter().cloned().enumerate().find_map(|(ml_index243, ml_value243)| if ml_callback243(ml_value243.clone()) { Some(ml_value243) } else { None }) };
                                {
                                    let missing = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Undefined));
                                    {
                                        let initial_value = if match (initial.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            missing.clone()
                                        } else {
                                            match initial.clone().expect("a proven present value") {
                                                self::JavaScriptCallCellInitializer::SourceCellInitializer(_, source_ml_source2_data_property_value15071, _) => {
                                                    let source_ml_source2_data_property_value15071 = *source_ml_source2_data_property_value15071;
                                                    source_ml_source2_data_property_value15071.clone()
                                                }
                                            }
                                        };
                                        {
                                            let initialized = if match (initial.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                false
                                            } else {
                                                match initial.clone().expect("a proven present value") {
                                                    self::JavaScriptCallCellInitializer::SourceCellInitializer(_, _, source_ml_source2_data_property_value15072) => {
                                                        source_ml_source2_data_property_value15072
                                                    }
                                                }
                                            };
                                            {
                                                let cell = {
                                                    let source_ml_source2_constructor_field15073 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Null));
                                                    {
                                                        let source_ml_source2_constructor_field15079 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                            let source_ml_source2_constructor_field15075 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![118f64, 97f64, 108f64, 117f64, 101f64])))));
                                                            self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field15075.clone()), Box::new(initial_value.clone()), None::<(Option<bool>,)>)
                                                        }, {
                                                            let source_ml_source2_constructor_field15077 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![105f64, 110f64, 105f64, 116f64, 105f64, 97f64, 108f64, 105f64, 122f64, 101f64, 100f64])))));
                                                            {
                                                                let source_ml_source2_constructor_field15078 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Boolean(initialized)));
                                                                self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field15077.clone()), Box::new(source_ml_source2_constructor_field15078.clone()), None::<(Option<bool>,)>)
                                                            }
                                                        }]));
                                                        self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field15073.clone()), source_ml_source2_constructor_field15079.clone(), realm.clone())
                                                    }
                                                };
                                                {
                                                    let allocated = self::lower_java_script_call_source_expression(cell.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                                                    {
                                                        let current_2 = match allocated.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15080, _) => {
                                                                let source_ml_source2_data_property_value15080 = *source_ml_source2_data_property_value15080;
                                                                source_ml_source2_data_property_value15080.clone()
                                                            }
                                                        };
                                                        if (replace && !(match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                            {
                                                                let current_3 = self::append_java_script_call_source_instruction(current_2.clone(), self::JavaScriptCallInstruction::Copy(match binding.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value15081, _, _) => {
                                                                        source_ml_source2_data_property_value15081
                                                                    }
                                                                }, match allocated.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15082) => {
                                                                        source_ml_source2_data_property_value15082
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
                                                                let entries_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value245 = entries.borrow().clone(); ml_reference_value245 }, vec![{
                                                                    let source_ml_source2_constructor_field15085 = match allocated.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15084) => {
                                                                            source_ml_source2_data_property_value15084
                                                                        }
                                                                    };
                                                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, source_ml_source2_constructor_field15085, false, None::<(Option<bool>,)>)
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
                let entry = { let ml_reference_array246 = bindings.clone(); let ml_reference_position246 = index; let ml_reference_projection246 = self::ml_array::optional_at(&*ml_reference_array246.borrow(), self::ml_array::number_index(ml_reference_position246)); ml_reference_projection246 };
                if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match entry.clone().expect("a proven present value") {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value15090, _, _, _) => {
                        source_ml_source2_data_property_value15090
                    }
                }) == identity)) {
                    if ({
                        let source_ml_source2_comparison_operand15092 = match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, _, source_ml_source2_data_property_value15091) => {
                                ((source_ml_source2_data_property_value15091.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                            }
                        };
                        match (source_ml_source2_comparison_operand15092.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    } && !(self::ml_array::includes(&{ let ml_reference_value247 = ((match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value15095, _, _) => {
                            ((source_ml_source2_data_property_value15095.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                        }
                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value247 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) {
                        self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Return(Box::new(self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![117, 110, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 97, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 109, 112, 111, 114, 97, 108, 45, 100, 101, 97, 100, 45, 122, 111, 110, 101, 32, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108])))))
                    } else {
                        if ({
                            let source_ml_source2_comparison_operand15097 = match state.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value15096, _, _, _) => {
                                    ((source_ml_source2_data_property_value15096.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                }
                            };
                            match (source_ml_source2_comparison_operand15097.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        } && match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, source_ml_source2_data_property_value15099, _) => {
                                source_ml_source2_data_property_value15099
                            }
                        }) {
                            {
                                let saved = self::allocate_java_script_call_source_register(state.clone());
                                self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Return(Box::new({
                                    let source_ml_source2_nominal_field15104 = match saved.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15105, _) => {
                                            let source_ml_source2_data_property_value15105 = *source_ml_source2_data_property_value15105;
                                            source_ml_source2_data_property_value15105.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field15106 = match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15107) => {
                                                source_ml_source2_data_property_value15107
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override15108 = self::append_java_script_call_source_instruction(match saved.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15100, _) => {
                                                    let source_ml_source2_data_property_value15100 = *source_ml_source2_data_property_value15100;
                                                    source_ml_source2_data_property_value15100.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15101) => {
                                                    source_ml_source2_data_property_value15101
                                                }
                                            }, match entry.clone().expect("a proven present value") {
                                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value15102, _, _) => {
                                                    source_ml_source2_data_property_value15102
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override15108.clone()), source_ml_source2_nominal_field15106)
                                        }
                                    }
                                }))
                            }
                        } else {
                            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Return(Box::new(self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value15109, _, _) => {
                                    source_ml_source2_data_property_value15109
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
        return if (source_ml_source1_temporary_index26 < ({ let ml_reference_array248 = source_ml_source1_temporary_values26.clone(); let ml_reference_projection248 = self::ml::Big::from_u128(ml_reference_array248.borrow().len() as u128); ml_reference_projection248 })) {
            {
                let argument = { let ml_reference_array249 = source_ml_source1_temporary_values26.clone(); let ml_reference_position249 = source_ml_source1_temporary_index26.clone(); let ml_reference_projection249 = self::ml_array::at(&*ml_reference_array249.borrow(), ml_reference_position249.to_index()); ml_reference_projection249 };
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15112, _) => {
                                let source_ml_source2_data_property_value15112 = *source_ml_source2_data_property_value15112;
                                source_ml_source2_data_property_value15112.clone()
                            }
                        };
                        {
                            let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value250 = arguments_.borrow().clone(); ml_reference_value250 }, vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15113) => {
                                    source_ml_source2_data_property_value15113
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
        return if (index < ({ let ml_reference_array251 = arguments_.clone(); let ml_reference_projection251 = ml_reference_array251.borrow().len() as f64; ml_reference_projection251 })) {
            {
                let current_2 = self::append_java_script_call_source_instruction(current.clone(), self::JavaScriptCallInstruction::Copy((index + 1f64), ({ let ml_reference_array252 = arguments_.clone(); let ml_reference_position252 = index; let ml_reference_projection252 = self::ml_array::optional_at(&*ml_reference_array252.borrow(), self::ml_array::number_index(ml_reference_position252)); ml_reference_projection252 }).clone().unwrap_or_else(|| 0f64)));
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
        return if (source_ml_source1_temporary_index27 < ({ let ml_reference_array253 = source_ml_source1_temporary_values27.clone(); let ml_reference_projection253 = self::ml::Big::from_u128(ml_reference_array253.borrow().len() as u128); ml_reference_projection253 })) {
            {
                let argument = { let ml_reference_array254 = source_ml_source1_temporary_values27.clone(); let ml_reference_position254 = source_ml_source1_temporary_index27.clone(); let ml_reference_projection254 = self::ml_array::at(&*ml_reference_array254.borrow(), ml_reference_position254.to_index()); ml_reference_projection254 };
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let captured = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15118, _) => {
                                let source_ml_source2_data_property_value15118 = *source_ml_source2_data_property_value15118;
                                source_ml_source2_data_property_value15118.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15119, _) => {
                                    let source_ml_source2_data_property_value15119 = *source_ml_source2_data_property_value15119;
                                    source_ml_source2_data_property_value15119.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15120) => {
                                    source_ml_source2_data_property_value15120
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15121) => {
                                    source_ml_source2_data_property_value15121
                                }
                            }));
                            {
                                let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value255 = arguments_.borrow().clone(); ml_reference_value255 }, vec![match captured.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15122) => {
                                        source_ml_source2_data_property_value15122
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
        return if (source_ml_source1_temporary_index31 < ({ let ml_reference_array256 = source_ml_source1_temporary_values31.clone(); let ml_reference_projection256 = self::ml::Big::from_u128(ml_reference_array256.borrow().len() as u128); ml_reference_projection256 })) {
            {
                let part = { let ml_reference_array257 = source_ml_source1_temporary_values31.clone(); let ml_reference_position257 = source_ml_source1_temporary_index31.clone(); let ml_reference_projection257 = self::ml_array::at(&*ml_reference_array257.borrow(), ml_reference_position257.to_index()); ml_reference_projection257 };
                if !(self::ml_array::includes(&{ let ml_reference_value258 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 65, 114, 114, 97, 121, 83, 112, 114, 101, 97, 100]), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 65, 114, 114, 97, 121, 86, 97, 108, 117, 101]), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 65, 114, 114, 97, 121, 72, 111, 108, 101])]))).borrow().clone(); ml_reference_value258 }, &match part.clone() {
                    self::JavaScriptCallArrayPart::SourceArrayValue(_) => {
                        self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 65, 114, 114, 97, 121, 86, 97, 108, 117, 101])
                    }
                    self::JavaScriptCallArrayPart::SourceArraySpread(_) => {
                        self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 65, 114, 114, 97, 121, 83, 112, 114, 101, 97, 100])
                    }
                    self::JavaScriptCallArrayPart::SourceArrayHole => {
                        self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 65, 114, 114, 97, 121, 72, 111, 108, 101])
                    }
                }, 0f64, false, |a, b| a == b)) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result::MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Return(Box::new(self::refuse_java_script_call_source(current.clone(), &(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 99, 99, 117, 109, 117, 108, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 112, 97, 114, 116, 32, 107, 105, 110, 100])))))
                } else {
                    match part.clone() {
                        self::JavaScriptCallArrayPart::SourceArrayValue(source_ml_source2_f15125) => {
                            let source_ml_source2_f15125 = *source_ml_source2_f15125;
                            {
                                let current_3 = {
                                    let key = self::allocate_java_script_call_source_register(current.clone());
                                    {
                                        let keyed = self::append_java_script_call_source_instruction(match key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15127, _) => {
                                                let source_ml_source2_data_property_value15127 = *source_ml_source2_data_property_value15127;
                                                source_ml_source2_data_property_value15127.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::PropertyKey(match key.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15128) => {
                                                source_ml_source2_data_property_value15128
                                            }
                                        }, match index.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15129) => {
                                                source_ml_source2_data_property_value15129
                                            }
                                        }, realm.clone()));
                                        {
                                            let value = self::lower_java_script_call_source_expression(source_ml_source2_f15125.clone(), bindings.clone(), keyed.clone(), (depth + 1f64));
                                            {
                                                let written = self::allocate_java_script_call_source_register(match value.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15130, _) => {
                                                        let source_ml_source2_data_property_value15130 = *source_ml_source2_data_property_value15130;
                                                        source_ml_source2_data_property_value15130.clone()
                                                    }
                                                });
                                                {
                                                    let current_2 = self::append_java_script_call_source_instruction(match written.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15131, _) => {
                                                            let source_ml_source2_data_property_value15131 = *source_ml_source2_data_property_value15131;
                                                            source_ml_source2_data_property_value15131.clone()
                                                        }
                                                    }, self::JavaScriptCallInstruction::OrdinaryDataProperty(match written.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15132) => {
                                                            source_ml_source2_data_property_value15132
                                                        }
                                                    }, match object.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15133) => {
                                                            source_ml_source2_data_property_value15133
                                                        }
                                                    }, match key.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15134) => {
                                                            source_ml_source2_data_property_value15134
                                                        }
                                                    }, match value.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15135) => {
                                                            source_ml_source2_data_property_value15135
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
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15136) => {
                                            source_ml_source2_data_property_value15136
                                        }
                                    }, self::ml_string::Text::from_units(vec![105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![match index.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15137) => {
                                            source_ml_source2_data_property_value15137
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
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15140, _) => {
                                            let source_ml_source2_data_property_value15140 = *source_ml_source2_data_property_value15140;
                                            source_ml_source2_data_property_value15140.clone()
                                        }
                                    });
                                    {
                                        let held = self::append_java_script_call_source_instruction(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15141, _) => {
                                                let source_ml_source2_data_property_value15141 = *source_ml_source2_data_property_value15141;
                                                source_ml_source2_data_property_value15141.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15142) => {
                                                source_ml_source2_data_property_value15142
                                            }
                                        }, match input.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15143) => {
                                                source_ml_source2_data_property_value15143
                                            }
                                        }));
                                        {
                                            let current_5 = self::append_java_script_call_source_instruction(held.clone(), self::JavaScriptCallInstruction::ArraySpread(match index.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15144) => {
                                                    source_ml_source2_data_property_value15144
                                                }
                                            }, match object.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15145) => {
                                                    source_ml_source2_data_property_value15145
                                                }
                                            }, match saved.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15146) => {
                                                    source_ml_source2_data_property_value15146
                                                }
                                            }, match index.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15147) => {
                                                    source_ml_source2_data_property_value15147
                                                }
                                            }, match saved_key.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15148) => {
                                                    source_ml_source2_data_property_value15148
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
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15150) => {
                                            source_ml_source2_data_property_value15150
                                        }
                                    }, self::ml_string::Text::from_units(vec![105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![match index.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15151) => {
                                            source_ml_source2_data_property_value15151
                                        }
                                    }]))));
                                    {
                                        let key_2 = self::lower_java_script_call_source_expression(self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64]))))), bindings.clone(), current_7.clone(), (depth + 1f64));
                                        {
                                            let written_2 = self::allocate_java_script_call_source_register(match key_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15154, _) => {
                                                    let source_ml_source2_data_property_value15154 = *source_ml_source2_data_property_value15154;
                                                    source_ml_source2_data_property_value15154.clone()
                                                }
                                            });
                                            {
                                                let current_8 = self::append_java_script_call_source_instruction(match written_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value15155, _) => {
                                                        let source_ml_source2_data_property_value15155 = *source_ml_source2_data_property_value15155;
                                                        source_ml_source2_data_property_value15155.clone()
                                                    }
                                                }, self::JavaScriptCallInstruction::OrdinarySet(match written_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15156) => {
                                                        source_ml_source2_data_property_value15156
                                                    }
                                                }, match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15157) => {
                                                        source_ml_source2_data_property_value15157
                                                    }
                                                }, match key_2.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15158) => {
                                                        source_ml_source2_data_property_value15158
                                                    }
                                                }, match index.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15159) => {
                                                        source_ml_source2_data_property_value15159
                                                    }
                                                }, match object.clone() {
                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15160) => {
                                                        source_ml_source2_data_property_value15160
                                                    }
                                                }));
                                                {
                                                    let current_9 = self::append_java_script_call_source_instruction(current_8.clone(), self::JavaScriptCallInstruction::RequireWrite(match written_2.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value15161) => {
                                                            source_ml_source2_data_property_value15161
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
