// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallArrayPart;
use super::JavaScriptCallCellInitializer;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceArrayElement;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptErrorArrayElement;
use super::JavaScriptErrorArrayPart;
use super::JavaScriptErrorExpression;
use super::JavaScriptErrorObjectProperty;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptErrorTemplatePart;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryConstructJavaScriptErrorPropertyReadLoop77Result;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_source_variable_initializer;
use super::lower_java_script_error_expression;
use super::lower_java_script_error_object_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::source_ml_source1_temporary_construct_java_script_error_property_read_loop77;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_error_property_read_loop76(mut receiver: self::JavaScriptCallSource, mut key: self::JavaScriptCallSource, mut primitive: self::JavaScriptCallSource, mut source_ml_source1_temporary_values75: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, Option<self::JavaScriptSemanticValue>)>>>>>, mut source_ml_source1_temporary_index75: self::ml::Big) -> self::JavaScriptCallSource {
    loop {
        return if (source_ml_source1_temporary_index75 < ({ let ml_reference_array467 = source_ml_source1_temporary_values75.clone(); let ml_reference_projection467 = self::ml::Big::from_u128(ml_reference_array467.borrow().len() as u128); ml_reference_projection467 })) {
            {
                let root = { let ml_reference_array468 = source_ml_source1_temporary_values75.clone(); let ml_reference_position468 = source_ml_source1_temporary_index75.clone(); let ml_reference_projection468 = self::ml_array::at(&*ml_reference_array468.borrow(), ml_reference_position468.to_index()); ml_reference_projection468 };
                {
                    let prototype = ({ let ml_reference_value469 = root.borrow().clone(); ml_reference_value469 }).1;
                    if match (prototype.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        {
                            let source_ml_source1_temporary_index75_2 = source_ml_source1_temporary_index75.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index75 = source_ml_source1_temporary_index75_2.clone();
                                continue;
                            }
                        }
                    } else {
                        {
                            let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                            {
                                let at = 0f64;
                                {
                                    let source_ml_source2_s19795 = self::source_ml_source1_temporary_construct_java_script_error_property_read_loop77(root.clone(), units.clone(), at);
                                    match source_ml_source2_s19795.clone() {
                                        self::MlSource1TemporaryConstructJavaScriptErrorPropertyReadLoop77Result::MlSource1TemporaryConstructJavaScriptErrorPropertyReadLoop77Done => {
                                            {
                                                let primitive_2 = self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 121, 112, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![receiver.clone()])))), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Text(units.clone()))))), Box::new({
                                                    let source_ml_source2_constructor_field19796 = self::construct_java_script_error_constant(prototype.clone().expect("a proven present value"));
                                                    {
                                                        let source_ml_source2_constructor_field19798 = Some(receiver.clone());
                                                        self::JavaScriptCallSource::SourceGet(Box::new(source_ml_source2_constructor_field19796.clone()), Box::new(key.clone()), Some((source_ml_source2_constructor_field19798.clone(),)))
                                                    }
                                                }), Box::new(primitive.clone()));
                                                {
                                                    let source_ml_source1_temporary_index75_3 = source_ml_source1_temporary_index75.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (primitive, source_ml_source1_temporary_index75) = (primitive_2.clone(), source_ml_source1_temporary_index75_3.clone());
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
            primitive.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructJavaScriptSourceFieldKeyLoop79Result {
    MlSource1TemporaryConstructJavaScriptSourceFieldKeyLoop79Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_source_field_key_loop79(key: &self::ml_string::Text, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryConstructJavaScriptSourceFieldKeyLoop79Result {
    loop {
        return if (at < (key.length())) {
            {
                let source_ml_source1_temporary_construct_java_script_source_field_key_ignored80 = { let ml_method_owner470 = units.clone(); let ml_method_argument470_0 = {
                    let source_ml_source2_string_unit_operand19799 = key.clone();
                    {
                        let source_ml_source2_string_unit_operand19800 = at;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index < (0f64)) || (index >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand19806 = {
                                        let source_ml_source2_string_range_operand19801 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand19802 = index;
                                            {
                                                let source_ml_source2_string_range_operand19803 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position19804 = source_ml_source2_string_range_operand19802;
                                                    {
                                                        let source_ml_source2_string_range_position19805 = source_ml_source2_string_range_operand19803;
                                                        source_ml_source2_string_range_operand19801.slice(source_ml_source2_string_range_position19804, source_ml_source2_string_range_position19805)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand19807 = 0f64;
                                        source_ml_source2_string_unit_operand19806.code_point_at(source_ml_source2_string_unit_operand19807)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand19799.clone(), source_ml_source2_string_unit_operand19800)
                    }
                }; let ml_inserted = vec![ml_method_argument470_0]; let mut ml_method_values470 = ml_method_owner470.borrow_mut(); let ml_length = ml_method_values470.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values470.extend(ml_inserted); ml_length as f64 };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConstructJavaScriptSourceFieldKeyLoop79Result::MlSource1TemporaryConstructJavaScriptSourceFieldKeyLoop79Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_error_construction_expression_loop86(mut depth: f64, mut current: self::JavaScriptErrorSourceContext, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_values81: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>, mut source_ml_source1_temporary_index81: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index81 < ({ let ml_reference_array471 = source_ml_source1_temporary_values81.clone(); let ml_reference_projection471 = self::ml::Big::from_u128(ml_reference_array471.borrow().len() as u128); ml_reference_projection471 })) {
            {
                let argument = { let ml_reference_array472 = source_ml_source1_temporary_values81.clone(); let ml_reference_position472 = source_ml_source1_temporary_index81.clone(); let ml_reference_projection472 = self::ml_array::at(&*ml_reference_array472.borrow(), ml_reference_position472.to_index()); ml_reference_projection472 };
                {
                    let value = self::lower_java_script_error_expression(argument.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19810) => {
                                let source_ml_source2_data_property_value19810 = *source_ml_source2_data_property_value19810;
                                source_ml_source2_data_property_value19810.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_lower_java_script_error_construction_expression_ignored87 = { let ml_method_owner473 = arguments_.clone(); let ml_method_argument473_0 = match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19811, _) => {
                                    let source_ml_source2_data_property_value19811 = *source_ml_source2_data_property_value19811;
                                    source_ml_source2_data_property_value19811.clone()
                                }
                            }; let ml_inserted = vec![ml_method_argument473_0]; let mut ml_method_values473 = ml_method_owner473.borrow_mut(); let ml_length = ml_method_values473.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values473.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_index81_2 = source_ml_source1_temporary_index81.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, source_ml_source1_temporary_index81) = (current_2.clone(), source_ml_source1_temporary_index81_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_error_construction_expression_loop88(mut depth: f64, mut current: self::JavaScriptErrorSourceContext, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_values82: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>, mut source_ml_source1_temporary_index82: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index82 < ({ let ml_reference_array474 = source_ml_source1_temporary_values82.clone(); let ml_reference_projection474 = self::ml::Big::from_u128(ml_reference_array474.borrow().len() as u128); ml_reference_projection474 })) {
            {
                let argument = { let ml_reference_array475 = source_ml_source1_temporary_values82.clone(); let ml_reference_position475 = source_ml_source1_temporary_index82.clone(); let ml_reference_projection475 = self::ml_array::at(&*ml_reference_array475.borrow(), ml_reference_position475.to_index()); ml_reference_projection475 };
                {
                    let lowered = self::lower_java_script_error_expression(argument.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match lowered.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19814) => {
                                let source_ml_source2_data_property_value19814 = *source_ml_source2_data_property_value19814;
                                source_ml_source2_data_property_value19814.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_lower_java_script_error_construction_expression_ignored89 = { let ml_method_owner476 = arguments_.clone(); let ml_method_argument476_0 = match lowered.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19815, _) => {
                                    let source_ml_source2_data_property_value19815 = *source_ml_source2_data_property_value19815;
                                    source_ml_source2_data_property_value19815.clone()
                                }
                            }; let ml_inserted = vec![ml_method_argument476_0]; let mut ml_method_values476 = ml_method_owner476.borrow_mut(); let ml_length = ml_method_values476.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values476.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_index82_2 = source_ml_source1_temporary_index82.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, source_ml_source1_temporary_index82) = (current_2.clone(), source_ml_source1_temporary_index82_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_error_construction_expression_loop90(mut depth: f64, mut current: self::JavaScriptErrorSourceContext, mut captures: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64, self::JavaScriptCallSource)>>>>>, mut source_ml_source1_temporary_values83: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>, mut source_ml_source1_temporary_index83: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index83 < ({ let ml_reference_array477 = source_ml_source1_temporary_values83.clone(); let ml_reference_projection477 = self::ml::Big::from_u128(ml_reference_array477.borrow().len() as u128); ml_reference_projection477 })) {
            {
                let argument = { let ml_reference_array478 = source_ml_source1_temporary_values83.clone(); let ml_reference_position478 = source_ml_source1_temporary_index83.clone(); let ml_reference_projection478 = self::ml_array::at(&*ml_reference_array478.borrow(), ml_reference_position478.to_index()); ml_reference_projection478 };
                {
                    let value = self::lower_java_script_error_expression(argument.clone(), current.clone(), (depth + 1f64));
                    {
                        let identity = match match value.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19818) => {
                                let source_ml_source2_data_property_value19818 = *source_ml_source2_data_property_value19818;
                                source_ml_source2_data_property_value19818.clone()
                            }
                        } {
                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value19819, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value19819
                            }
                        };
                        {
                            let current_2 = {
                                let source_ml_source2_nominal_spread19821 = match value.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19820) => {
                                        let source_ml_source2_data_property_value19820 = *source_ml_source2_data_property_value19820;
                                        source_ml_source2_data_property_value19820.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field19822 = match source_ml_source2_nominal_spread19821.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value19823, _, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value19823 = *source_ml_source2_data_property_value19823;
                                            source_ml_source2_data_property_value19823.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field19824 = match source_ml_source2_nominal_spread19821.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value19825, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value19825 = *source_ml_source2_data_property_value19825;
                                                source_ml_source2_data_property_value19825.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field19826 = match source_ml_source2_nominal_spread19821.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value19827, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value19827.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field19828 = match source_ml_source2_nominal_spread19821.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value19829, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value19829
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field19830 = match source_ml_source2_nominal_spread19821.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value19831, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value19831
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field19832 = match source_ml_source2_nominal_spread19821.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value19833, _, _, _, _) => {
                                                                source_ml_source2_data_property_value19833.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field19834 = match source_ml_source2_nominal_spread19821.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value19835, _, _, _) => {
                                                                    source_ml_source2_data_property_value19835
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field19836 = match source_ml_source2_nominal_spread19821.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value19837, _, _) => {
                                                                        source_ml_source2_data_property_value19837.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field19838 = match source_ml_source2_nominal_spread19821.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value19839, _) => {
                                                                            source_ml_source2_data_property_value19839.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field19840 = match source_ml_source2_nominal_spread19821.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19841) => {
                                                                                source_ml_source2_data_property_value19841.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override19842 = (identity + 1f64);
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field19822.clone()), Box::new(source_ml_source2_nominal_field19824.clone()), source_ml_source2_nominal_field19826.clone(), source_ml_source2_nominal_override19842, source_ml_source2_nominal_field19830, source_ml_source2_nominal_field19832.clone(), source_ml_source2_nominal_field19834, source_ml_source2_nominal_field19836.clone(), source_ml_source2_nominal_field19838.clone(), source_ml_source2_nominal_field19840.clone())
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
                                let source_ml_source1_temporary_lower_java_script_error_construction_expression_ignored91 = { let ml_method_owner479 = captures.clone(); let ml_method_argument479_0 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field19845 = match value.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19844, _) => {
                                            let source_ml_source2_data_property_value19844 = *source_ml_source2_data_property_value19844;
                                            source_ml_source2_data_property_value19844.clone()
                                        }
                                    };
                                    (identity, source_ml_source2_field19845.clone())
                                })); let ml_inserted = vec![ml_method_argument479_0]; let mut ml_method_values479 = ml_method_owner479.borrow_mut(); let ml_length = ml_method_values479.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values479.extend(ml_inserted); ml_length as f64 };
                                {
                                    let source_ml_source1_temporary_index83_2 = source_ml_source1_temporary_index83.add(&self::ml::Big::from_i128(1));
                                    {
                                        (current, source_ml_source1_temporary_index83) = (current_2.clone(), source_ml_source1_temporary_index83_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_error_construction_expression_loop92(mut current: self::JavaScriptErrorSourceContext, mut captures: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(f64, self::JavaScriptCallSource)>>>>>, mut source: self::JavaScriptCallSource, mut at: f64) -> self::JavaScriptCallSource {
    loop {
        return if (at >= (0f64)) {
            {
                let capture = { let ml_reference_array480 = captures.clone(); let ml_reference_position480 = at; let ml_reference_projection480 = self::ml_array::optional_at(&*ml_reference_array480.borrow(), self::ml_array::number_index(ml_reference_position480)); ml_reference_projection480 };
                if !(match (capture.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                    {
                        let source_2 = self::construct_java_script_error_local_binding(current.clone(), ({ let ml_reference_value481 = (capture.clone().expect("a proven present value")).borrow().clone(); ml_reference_value481 }).0, ({ let ml_reference_value482 = (capture.clone().expect("a proven present value")).borrow().clone(); ml_reference_value482 }).1, source.clone(), false);
                        {
                            let at_2 = (at - 1f64);
                            {
                                (source, at) = (source_2.clone(), at_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let at_3 = (at - 1f64);
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            source.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_error_construction_expression_loop93(mut depth: f64, mut current: self::JavaScriptErrorSourceContext, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_values84: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>, mut source_ml_source1_temporary_index84: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index84 < ({ let ml_reference_array483 = source_ml_source1_temporary_values84.clone(); let ml_reference_projection483 = self::ml::Big::from_u128(ml_reference_array483.borrow().len() as u128); ml_reference_projection483 })) {
            {
                let argument = { let ml_reference_array484 = source_ml_source1_temporary_values84.clone(); let ml_reference_position484 = source_ml_source1_temporary_index84.clone(); let ml_reference_projection484 = self::ml_array::at(&*ml_reference_array484.borrow(), ml_reference_position484.to_index()); ml_reference_projection484 };
                {
                    let value = self::lower_java_script_error_expression(argument.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19850) => {
                                let source_ml_source2_data_property_value19850 = *source_ml_source2_data_property_value19850;
                                source_ml_source2_data_property_value19850.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_lower_java_script_error_construction_expression_ignored94 = { let ml_method_owner485 = arguments_.clone(); let ml_method_argument485_0 = match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19851, _) => {
                                    let source_ml_source2_data_property_value19851 = *source_ml_source2_data_property_value19851;
                                    source_ml_source2_data_property_value19851.clone()
                                }
                            }; let ml_inserted = vec![ml_method_argument485_0]; let mut ml_method_values485 = ml_method_owner485.borrow_mut(); let ml_length = ml_method_values485.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values485.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_index84_2 = source_ml_source1_temporary_index84.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, source_ml_source1_temporary_index84) = (current_2.clone(), source_ml_source1_temporary_index84_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_error_construction_expression_loop95(mut depth: f64, mut current: self::JavaScriptErrorSourceContext, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_values85: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>, mut source_ml_source1_temporary_index85: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index85 < ({ let ml_reference_array486 = source_ml_source1_temporary_values85.clone(); let ml_reference_projection486 = self::ml::Big::from_u128(ml_reference_array486.borrow().len() as u128); ml_reference_projection486 })) {
            {
                let argument = { let ml_reference_array487 = source_ml_source1_temporary_values85.clone(); let ml_reference_position487 = source_ml_source1_temporary_index85.clone(); let ml_reference_projection487 = self::ml_array::at(&*ml_reference_array487.borrow(), ml_reference_position487.to_index()); ml_reference_projection487 };
                {
                    let value = self::lower_java_script_error_expression(argument.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19854) => {
                                let source_ml_source2_data_property_value19854 = *source_ml_source2_data_property_value19854;
                                source_ml_source2_data_property_value19854.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_lower_java_script_error_construction_expression_ignored96 = { let ml_method_owner488 = arguments_.clone(); let ml_method_argument488_0 = match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19855, _) => {
                                    let source_ml_source2_data_property_value19855 = *source_ml_source2_data_property_value19855;
                                    source_ml_source2_data_property_value19855.clone()
                                }
                            }; let ml_inserted = vec![ml_method_argument488_0]; let mut ml_method_values488 = ml_method_owner488.borrow_mut(); let ml_length = ml_method_values488.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values488.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_index85_2 = source_ml_source1_temporary_index85.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, source_ml_source1_temporary_index85) = (current_2.clone(), source_ml_source1_temporary_index85_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptErrorPropertyExpressionJoin99 {
    MlSource1TemporaryLowerJavaScriptErrorPropertyExpressionJoin99Next(Box<self::JavaScriptErrorSourceContext>, Box<self::JavaScriptCallSource>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_error_property_expression_loop100(mut depth: f64, mut current: self::JavaScriptErrorSourceContext, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_values97: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>, mut source_ml_source1_temporary_index97: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index97 < ({ let ml_reference_array489 = source_ml_source1_temporary_values97.clone(); let ml_reference_projection489 = self::ml::Big::from_u128(ml_reference_array489.borrow().len() as u128); ml_reference_projection489 })) {
            {
                let argument = { let ml_reference_array490 = source_ml_source1_temporary_values97.clone(); let ml_reference_position490 = source_ml_source1_temporary_index97.clone(); let ml_reference_projection490 = self::ml_array::at(&*ml_reference_array490.borrow(), ml_reference_position490.to_index()); ml_reference_projection490 };
                {
                    let value = self::lower_java_script_error_expression(argument.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19858) => {
                                let source_ml_source2_data_property_value19858 = *source_ml_source2_data_property_value19858;
                                source_ml_source2_data_property_value19858.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_lower_java_script_error_property_expression_ignored101 = { let ml_method_owner491 = arguments_.clone(); let ml_method_argument491_0 = match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19859, _) => {
                                    let source_ml_source2_data_property_value19859 = *source_ml_source2_data_property_value19859;
                                    source_ml_source2_data_property_value19859.clone()
                                }
                            }; let ml_inserted = vec![ml_method_argument491_0]; let mut ml_method_values491 = ml_method_owner491.borrow_mut(); let ml_length = ml_method_values491.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values491.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_index97_2 = source_ml_source1_temporary_index97.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, source_ml_source1_temporary_index97) = (current_2.clone(), source_ml_source1_temporary_index97_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_error_property_expression_loop102(mut depth: f64, mut current: self::JavaScriptErrorSourceContext, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_values98: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>, mut source_ml_source1_temporary_index98: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index98 < ({ let ml_reference_array492 = source_ml_source1_temporary_values98.clone(); let ml_reference_projection492 = self::ml::Big::from_u128(ml_reference_array492.borrow().len() as u128); ml_reference_projection492 })) {
            {
                let argument = { let ml_reference_array493 = source_ml_source1_temporary_values98.clone(); let ml_reference_position493 = source_ml_source1_temporary_index98.clone(); let ml_reference_projection493 = self::ml_array::at(&*ml_reference_array493.borrow(), ml_reference_position493.to_index()); ml_reference_projection493 };
                {
                    let value = self::lower_java_script_error_expression(argument.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19862) => {
                                let source_ml_source2_data_property_value19862 = *source_ml_source2_data_property_value19862;
                                source_ml_source2_data_property_value19862.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_lower_java_script_error_property_expression_ignored103 = { let ml_method_owner494 = arguments_.clone(); let ml_method_argument494_0 = match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19863, _) => {
                                    let source_ml_source2_data_property_value19863 = *source_ml_source2_data_property_value19863;
                                    source_ml_source2_data_property_value19863.clone()
                                }
                            }; let ml_inserted = vec![ml_method_argument494_0]; let mut ml_method_values494 = ml_method_owner494.borrow_mut(); let ml_length = ml_method_values494.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values494.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_index98_2 = source_ml_source1_temporary_index98.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, source_ml_source1_temporary_index98) = (current_2.clone(), source_ml_source1_temporary_index98_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_error_literal_expression_loop109(mut depth: f64, mut current: self::JavaScriptErrorSourceContext, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_values104: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorExpression>>>, mut source_ml_source1_temporary_index104: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index104 < ({ let ml_reference_array495 = source_ml_source1_temporary_values104.clone(); let ml_reference_projection495 = self::ml::Big::from_u128(ml_reference_array495.borrow().len() as u128); ml_reference_projection495 })) {
            {
                let item = { let ml_reference_array496 = source_ml_source1_temporary_values104.clone(); let ml_reference_position496 = source_ml_source1_temporary_index104.clone(); let ml_reference_projection496 = self::ml_array::at(&*ml_reference_array496.borrow(), ml_reference_position496.to_index()); ml_reference_projection496 };
                {
                    let value = self::lower_java_script_error_expression(item.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19866) => {
                                let source_ml_source2_data_property_value19866 = *source_ml_source2_data_property_value19866;
                                source_ml_source2_data_property_value19866.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_lower_java_script_error_literal_expression_ignored110 = { let ml_method_owner497 = arguments_.clone(); let ml_method_argument497_0 = match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19867, _) => {
                                    let source_ml_source2_data_property_value19867 = *source_ml_source2_data_property_value19867;
                                    source_ml_source2_data_property_value19867.clone()
                                }
                            }; let ml_inserted = vec![ml_method_argument497_0]; let mut ml_method_values497 = ml_method_owner497.borrow_mut(); let ml_length = ml_method_values497.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values497.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_index104_2 = source_ml_source1_temporary_index104.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, source_ml_source1_temporary_index104) = (current_2.clone(), source_ml_source1_temporary_index104_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_error_literal_expression_loop111(mut depth: f64, mut current: self::JavaScriptErrorSourceContext, mut parts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArrayPart>>>, mut source_ml_source1_temporary_values105: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayPart>>>, mut source_ml_source1_temporary_index105: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index105 < ({ let ml_reference_array498 = source_ml_source1_temporary_values105.clone(); let ml_reference_projection498 = self::ml::Big::from_u128(ml_reference_array498.borrow().len() as u128); ml_reference_projection498 })) {
            {
                let part = { let ml_reference_array499 = source_ml_source1_temporary_values105.clone(); let ml_reference_position499 = source_ml_source1_temporary_index105.clone(); let ml_reference_projection499 = self::ml_array::at(&*ml_reference_array499.borrow(), ml_reference_position499.to_index()); ml_reference_projection499 };
                match part.clone() {
                    self::JavaScriptErrorArrayPart::ErrorArrayValue(part_value) => {
                        let part_value = *part_value;
                        {
                            let value = self::lower_java_script_error_expression(part_value.clone(), current.clone(), (depth + 1f64));
                            {
                                let current_2 = match value.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19870) => {
                                        let source_ml_source2_data_property_value19870 = *source_ml_source2_data_property_value19870;
                                        source_ml_source2_data_property_value19870.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_lower_java_script_error_literal_expression_ignored113 = { let ml_method_owner500 = parts.clone(); let ml_method_argument500_0 = self::JavaScriptCallArrayPart::SourceArrayValue(Box::new(match value.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19874, _) => {
                                            let source_ml_source2_data_property_value19874 = *source_ml_source2_data_property_value19874;
                                            source_ml_source2_data_property_value19874.clone()
                                        }
                                    })); let ml_inserted = vec![ml_method_argument500_0]; let mut ml_method_values500 = ml_method_owner500.borrow_mut(); let ml_length = ml_method_values500.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values500.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let source_ml_source1_temporary_index105_2 = source_ml_source1_temporary_index105.add(&self::ml::Big::from_i128(1));
                                        {
                                            (current, source_ml_source1_temporary_index105) = (current_2.clone(), source_ml_source1_temporary_index105_2.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptErrorArrayPart::ErrorArraySpread(part_value_2) => {
                        let part_value_2 = *part_value_2;
                        {
                            let value_2 = self::lower_java_script_error_expression(part_value_2.clone(), current.clone(), (depth + 1f64));
                            {
                                let current_3 = match value_2.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19875) => {
                                        let source_ml_source2_data_property_value19875 = *source_ml_source2_data_property_value19875;
                                        source_ml_source2_data_property_value19875.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_lower_java_script_error_literal_expression_ignored113_2 = { let ml_method_owner501 = parts.clone(); let ml_method_argument501_0 = self::JavaScriptCallArrayPart::SourceArraySpread(Box::new(match value_2.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19878, _) => {
                                            let source_ml_source2_data_property_value19878 = *source_ml_source2_data_property_value19878;
                                            source_ml_source2_data_property_value19878.clone()
                                        }
                                    })); let ml_inserted = vec![ml_method_argument501_0]; let mut ml_method_values501 = ml_method_owner501.borrow_mut(); let ml_length = ml_method_values501.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values501.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let source_ml_source1_temporary_index105_3 = source_ml_source1_temporary_index105.add(&self::ml::Big::from_i128(1));
                                        {
                                            (current, source_ml_source1_temporary_index105) = (current_3.clone(), source_ml_source1_temporary_index105_3.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptErrorArrayPart::ErrorArrayHole => {
                        {
                            let source_ml_source1_temporary_lower_java_script_error_literal_expression_ignored112 = { let ml_method_owner502 = parts.clone(); let ml_method_argument502_0 = self::JavaScriptCallArrayPart::SourceArrayHole; let ml_inserted = vec![ml_method_argument502_0]; let mut ml_method_values502 = ml_method_owner502.borrow_mut(); let ml_length = ml_method_values502.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values502.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_index105_4 = source_ml_source1_temporary_index105.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index105 = source_ml_source1_temporary_index105_4.clone();
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_error_literal_expression_loop114(mut depth: f64, mut current: self::JavaScriptErrorSourceContext, mut elements: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceArrayElement>>>, mut source_ml_source1_temporary_values106: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayElement>>>, mut source_ml_source1_temporary_index106: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index106 < ({ let ml_reference_array503 = source_ml_source1_temporary_values106.clone(); let ml_reference_projection503 = self::ml::Big::from_u128(ml_reference_array503.borrow().len() as u128); ml_reference_projection503 })) {
            {
                let element = { let ml_reference_array504 = source_ml_source1_temporary_values106.clone(); let ml_reference_position504 = source_ml_source1_temporary_index106.clone(); let ml_reference_projection504 = self::ml_array::at(&*ml_reference_array504.borrow(), ml_reference_position504.to_index()); ml_reference_projection504 };
                {
                    let value = self::lower_java_script_error_expression(match element.clone() {
                        self::JavaScriptErrorArrayElement::ErrorArrayElement(_, source_ml_source2_data_property_value19882) => {
                            let source_ml_source2_data_property_value19882 = *source_ml_source2_data_property_value19882;
                            source_ml_source2_data_property_value19882.clone()
                        }
                    }, current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19883) => {
                                let source_ml_source2_data_property_value19883 = *source_ml_source2_data_property_value19883;
                                source_ml_source2_data_property_value19883.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_lower_java_script_error_literal_expression_ignored115 = { let ml_method_owner505 = elements.clone(); let ml_method_argument505_0 = self::JavaScriptCallSourceArrayElement::SourceArrayElement(match element.clone() {
                                self::JavaScriptErrorArrayElement::ErrorArrayElement(source_ml_source2_data_property_value19884, _) => {
                                    source_ml_source2_data_property_value19884
                                }
                            }, Box::new(match value.clone() {
                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19885, _) => {
                                    let source_ml_source2_data_property_value19885 = *source_ml_source2_data_property_value19885;
                                    source_ml_source2_data_property_value19885.clone()
                                }
                            })); let ml_inserted = vec![ml_method_argument505_0]; let mut ml_method_values505 = ml_method_owner505.borrow_mut(); let ml_length = ml_method_values505.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values505.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_index106_2 = source_ml_source1_temporary_index106.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, source_ml_source1_temporary_index106) = (current_2.clone(), source_ml_source1_temporary_index106_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_error_literal_expression_loop116(mut depth: f64, mut current: self::JavaScriptErrorSourceContext, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceProperty>>>, mut source_ml_source1_temporary_values107: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorObjectProperty>>>, mut source_ml_source1_temporary_index107: self::ml::Big) -> self::JavaScriptErrorSourceContext {
    loop {
        return if (source_ml_source1_temporary_index107 < ({ let ml_reference_array506 = source_ml_source1_temporary_values107.clone(); let ml_reference_projection506 = self::ml::Big::from_u128(ml_reference_array506.borrow().len() as u128); ml_reference_projection506 })) {
            {
                let property = { let ml_reference_array507 = source_ml_source1_temporary_values107.clone(); let ml_reference_position507 = source_ml_source1_temporary_index107.clone(); let ml_reference_projection507 = self::ml_array::at(&*ml_reference_array507.borrow(), ml_reference_position507.to_index()); ml_reference_projection507 };
                {
                    let lowered = self::lower_java_script_error_object_property(property.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = ({ let ml_reference_value508 = lowered.borrow().clone(); ml_reference_value508 }).0;
                        {
                            let source_ml_source1_temporary_lower_java_script_error_literal_expression_ignored117 = { let ml_method_owner509 = properties.clone(); let ml_method_argument509_0 = ({ let ml_reference_value510 = lowered.borrow().clone(); ml_reference_value510 }).1; let ml_inserted = vec![ml_method_argument509_0]; let mut ml_method_values509 = ml_method_owner509.borrow_mut(); let ml_length = ml_method_values509.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values509.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_index107_2 = source_ml_source1_temporary_index107.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, source_ml_source1_temporary_index107) = (current_2.clone(), source_ml_source1_temporary_index107_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptErrorLiteralExpressionLoop118Result {
    MlSource1TemporaryLowerJavaScriptErrorLiteralExpressionLoop118Done(Box<self::JavaScriptErrorSourceContext>, Box<self::JavaScriptCallSource>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_error_literal_expression_loop118(mut depth: f64, mut constant: std::rc::Rc<dyn Fn(self::JavaScriptSemanticValue) -> self::JavaScriptCallSource>, mut current: self::JavaScriptErrorSourceContext, mut source: self::JavaScriptCallSource, mut source_ml_source1_temporary_values108: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorTemplatePart>>>, mut source_ml_source1_temporary_index108: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptErrorLiteralExpressionLoop118Result {
    loop {
        return if (source_ml_source1_temporary_index108 < ({ let ml_reference_array511 = source_ml_source1_temporary_values108.clone(); let ml_reference_projection511 = self::ml::Big::from_u128(ml_reference_array511.borrow().len() as u128); ml_reference_projection511 })) {
            {
                let part = { let ml_reference_array512 = source_ml_source1_temporary_values108.clone(); let ml_reference_position512 = source_ml_source1_temporary_index108.clone(); let ml_reference_projection512 = self::ml_array::at(&*ml_reference_array512.borrow(), ml_reference_position512.to_index()); ml_reference_projection512 };
                match part.clone() {
                    self::JavaScriptErrorTemplatePart::ErrorTemplateText(units) => {
                        {
                            let source_2 = self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![source.clone(), (constant.clone())(self::JavaScriptSemanticValue::Text(units.clone()))])));
                            {
                                let source_ml_source1_temporary_index108_2 = source_ml_source1_temporary_index108.add(&self::ml::Big::from_i128(1));
                                {
                                    (source, source_ml_source1_temporary_index108) = (source_2.clone(), source_ml_source1_temporary_index108_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                    self::JavaScriptErrorTemplatePart::ErrorTemplateValue(part_value) => {
                        let part_value = *part_value;
                        {
                            let value = self::lower_java_script_error_expression(part_value.clone(), current.clone(), (depth + 1f64));
                            {
                                let current_2 = match value.clone() {
                                    self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19890) => {
                                        let source_ml_source2_data_property_value19890 = *source_ml_source2_data_property_value19890;
                                        source_ml_source2_data_property_value19890.clone()
                                    }
                                };
                                {
                                    let source_3 = self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![source.clone(), self::JavaScriptCallSource::SourcePrimitive(self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![match value.clone() {
                                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value19891, _) => {
                                            let source_ml_source2_data_property_value19891 = *source_ml_source2_data_property_value19891;
                                            source_ml_source2_data_property_value19891.clone()
                                        }
                                    }])))])));
                                    {
                                        let source_ml_source1_temporary_index108_3 = source_ml_source1_temporary_index108.add(&self::ml::Big::from_i128(1));
                                        {
                                            (current, source, source_ml_source1_temporary_index108) = (current_2.clone(), source_3.clone(), source_ml_source1_temporary_index108_3.clone());
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
            self::MlSource1TemporaryLowerJavaScriptErrorLiteralExpressionLoop118Result::MlSource1TemporaryLowerJavaScriptErrorLiteralExpressionLoop118Done(Box::new(current.clone()), Box::new(source.clone()))
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119 {
    MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119Next(Box<self::JavaScriptCallSource>),
    MlSource1TemporaryLowerJavaScriptSourceBinaryJoin119Return(Box<self::JavaScriptErrorSourceExpression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123 {
    MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123Next,
    MlSource1TemporaryLowerJavaScriptSourceStatementsJoin123Return(Box<self::JavaScriptErrorSourceExpression>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceStatementsLoop124Result {
    MlSource1TemporaryLowerJavaScriptSourceStatementsLoop124Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_source_statements_loop124(mut body: self::JavaScriptErrorSourceExpression, mut initializers: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>, mut source_ml_source1_temporary_values120: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index120: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptSourceStatementsLoop124Result {
    loop {
        return if (source_ml_source1_temporary_index120 < ({ let ml_reference_array513 = source_ml_source1_temporary_values120.clone(); let ml_reference_projection513 = self::ml::Big::from_u128(ml_reference_array513.borrow().len() as u128); ml_reference_projection513 })) {
            {
                let identity = { let ml_reference_array514 = source_ml_source1_temporary_values120.clone(); let ml_reference_position514 = source_ml_source1_temporary_index120.clone(); let ml_reference_projection514 = self::ml_array::at(&*ml_reference_array514.borrow(), ml_reference_position514.to_index()); ml_reference_projection514 };
                {
                    let source_ml_source1_temporary_lower_java_script_source_statements_ignored125 = { let ml_method_owner515 = initializers.clone(); let ml_method_argument515_0 = self::construct_java_script_source_variable_initializer(match body.clone() {
                        self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value19894) => {
                            let source_ml_source2_data_property_value19894 = *source_ml_source2_data_property_value19894;
                            source_ml_source2_data_property_value19894.clone()
                        }
                    }, identity); let ml_inserted = vec![ml_method_argument515_0]; let mut ml_method_values515 = ml_method_owner515.borrow_mut(); let ml_length = ml_method_values515.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values515.extend(ml_inserted); ml_length as f64 };
                    {
                        let source_ml_source1_temporary_index120_2 = source_ml_source1_temporary_index120.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index120 = source_ml_source1_temporary_index120_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptSourceStatementsLoop124Result::MlSource1TemporaryLowerJavaScriptSourceStatementsLoop124Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptSourceIterationContinuationLoop138Result {
    MlSource1TemporaryLowerJavaScriptSourceIterationContinuationLoop138Done(Box<self::JavaScriptErrorSourceContext>, Option<self::JavaScriptCallSource>),
}
