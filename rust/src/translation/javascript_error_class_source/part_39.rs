// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallBindingDeclaration;
use super::JavaScriptCallSource;
use super::JavaScriptCallTemplate;
use super::JavaScriptErrorArrayBindingPart;
use super::JavaScriptErrorParameter;
use super::JavaScriptErrorParameterField;
use super::JavaScriptErrorRealmOperations;
use super::JavaScriptErrorSourceContext;
use super::JavaScriptErrorSourceExpression;
use super::JavaScriptPatternBindingDeclarations;
use super::JavaScriptSemanticValue;
use super::JavaScriptSourceBindingTarget;
use super::JavaScriptSourceLexicalBinding;
use super::JavaScriptSourceReference;
use super::JavaScriptSourceReferenceCapture;
use super::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Result;
use super::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result;
use super::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result;
use super::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Result;
use super::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Result;
use super::append_java_script_pattern_binding_declarations;
use super::construct_java_script_error_constant;
use super::construct_java_script_error_local_binding;
use super::construct_java_script_source_pattern_binding;
use super::lower_java_script_error_expression;
use super::lower_java_script_source_binding_pattern;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::source_ml_source1_temporary_order_java_script_error_call_templates_loop204;
use super::source_ml_source1_temporary_order_java_script_error_call_templates_loop205;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_error_call_templates_loop203(mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut middle: f64, mut end: f64, mut left: f64, mut right: f64) -> self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result {
    loop {
        return if ((left < middle) && (right < end)) {
            {
                let first = { let ml_reference_array643 = ordered.clone(); let ml_reference_position643 = left; let ml_reference_projection643 = self::ml_array::optional_at(&*ml_reference_array643.borrow(), self::ml_array::number_index(ml_reference_position643)); ml_reference_projection643 };
                {
                    let second = { let ml_reference_array644 = ordered.clone(); let ml_reference_position644 = right; let ml_reference_projection644 = self::ml_array::optional_at(&*ml_reference_array644.borrow(), self::ml_array::number_index(ml_reference_position644)); ml_reference_projection644 };
                    if match (first.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value645 = templates.borrow().clone(); ml_reference_value645 }, 0f64, f64::INFINITY))))
                    } else {
                        if match (second.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value646 = templates.borrow().clone(); ml_reference_value646 }, 0f64, f64::INFINITY))))
                        } else {
                            if (((match first.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(source_ml_source2_data_property_value22010, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22010.clone()
                                }
                            }) < (match second.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(source_ml_source2_data_property_value22011, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22011.clone()
                                }
                            })) || (((match first.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(source_ml_source2_data_property_value22014, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22014.clone()
                                }
                            }) == (match second.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(source_ml_source2_data_property_value22015, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22015.clone()
                                }
                            })) && ((match first.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(_, source_ml_source2_data_property_value22018, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22018
                                }
                            }) <= (match second.clone().expect("a proven present value") {
                                self::JavaScriptCallTemplate::CallTemplate(_, source_ml_source2_data_property_value22019, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22019
                                }
                            })))) {
                                {
                                    let source_ml_source1_temporary_order_java_script_error_call_templates_ignored208 = { let ml_method_owner647 = merged.clone(); let ml_method_argument647_0 = first.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument647_0]; let mut ml_method_values647 = ml_method_owner647.borrow_mut(); let ml_length = ml_method_values647.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values647.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let left_2 = (left + 1f64);
                                        {
                                            left = left_2;
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let source_ml_source1_temporary_order_java_script_error_call_templates_ignored209 = { let ml_method_owner648 = merged.clone(); let ml_method_argument648_0 = second.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument648_0]; let mut ml_method_values648 = ml_method_owner648.borrow_mut(); let ml_length = ml_method_values648.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values648.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let right_2 = (right + 1f64);
                                        {
                                            right = right_2;
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
            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Done(left, right)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_error_call_templates_loop202(mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut width: f64, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut start: f64) -> self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result {
    loop {
        return if (start < ({ let ml_reference_array649 = ordered.clone(); let ml_reference_projection649 = ml_reference_array649.borrow().len() as f64; ml_reference_projection649 })) {
            {
                let middle = self::ml_math::min((start + width), { let ml_reference_array650 = ordered.clone(); let ml_reference_projection650 = ml_reference_array650.borrow().len() as f64; ml_reference_projection650 });
                {
                    let end = self::ml_math::min((start + (2f64 * width)), { let ml_reference_array651 = ordered.clone(); let ml_reference_projection651 = ml_reference_array651.borrow().len() as f64; ml_reference_projection651 });
                    {
                        let left = start;
                        {
                            let right = middle;
                            {
                                let source_ml_source2_s22020 = self::source_ml_source1_temporary_order_java_script_error_call_templates_loop203(templates.clone(), ordered.clone(), merged.clone(), middle, end, left, right);
                                match source_ml_source2_s22020.clone() {
                                    self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Done(left_2, right_2) => {
                                        {
                                            let source_ml_source2_s22021 = self::source_ml_source1_temporary_order_java_script_error_call_templates_loop204(templates.clone(), ordered.clone(), merged.clone(), middle, left_2);
                                            match source_ml_source2_s22021.clone() {
                                                self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Done => {
                                                    {
                                                        let source_ml_source2_s22022 = self::source_ml_source1_temporary_order_java_script_error_call_templates_loop205(templates.clone(), ordered.clone(), merged.clone(), end, right_2);
                                                        match source_ml_source2_s22022.clone() {
                                                            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Done => {
                                                                {
                                                                    let start_2 = (start + (2f64 * width));
                                                                    {
                                                                        start = start_2;
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop205Return(source_ml_source1_temporary_result205) => {
                                                                self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Return(source_ml_source1_temporary_result205.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop204Return(source_ml_source1_temporary_result206) => {
                                                    self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Return(source_ml_source1_temporary_result206.clone())
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop203Return(source_ml_source1_temporary_result207) => {
                                        self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Return(source_ml_source1_temporary_result207.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_error_call_templates_loop201(mut templates: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>, mut width: f64) -> self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Result {
    loop {
        return if (width < ({ let ml_reference_array652 = ordered.clone(); let ml_reference_projection652 = ml_reference_array652.borrow().len() as f64; ml_reference_projection652 })) {
            {
                let merged = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallTemplate>::new()));
                {
                    let start = 0f64;
                    {
                        let source_ml_source2_s22023 = self::source_ml_source1_temporary_order_java_script_error_call_templates_loop202(templates.clone(), ordered.clone(), width, merged.clone(), start);
                        match source_ml_source2_s22023.clone() {
                            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Done => {
                                {
                                    let width_2 = (width * 2f64);
                                    {
                                        (ordered, width) = (merged.clone(), width_2);
                                        continue;
                                    }
                                }
                            }
                            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop202Return(source_ml_source1_temporary_result202) => {
                                self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Return(source_ml_source1_temporary_result202.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Result::MlSource1TemporaryOrderJavaScriptErrorCallTemplatesLoop201Done(ordered.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_source_reference_captures_loop210(mut reference: self::JavaScriptSourceReference, mut context: self::JavaScriptErrorSourceContext, mut source: self::JavaScriptCallSource, mut at: f64) -> self::JavaScriptCallSource {
    loop {
        return if (at >= (0f64)) {
            {
                let capture = {
                    let source_ml_source2_array_read_value22025 = match reference.clone() {
                        self::JavaScriptSourceReference::SourceReference(_, _, _, _, _, source_ml_source2_data_property_value22024, _) => {
                            source_ml_source2_data_property_value22024.clone()
                        }
                    };
                    { let ml_reference_array653 = source_ml_source2_array_read_value22025.clone(); let ml_reference_position653 = at; let ml_reference_projection653 = self::ml_array::optional_at(&*ml_reference_array653.borrow(), self::ml_array::number_index(ml_reference_position653)); ml_reference_projection653 }
                };
                if !(match (capture.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                    {
                        let source_2 = self::construct_java_script_error_local_binding(context.clone(), match capture.clone().expect("a proven present value") {
                            self::JavaScriptSourceReferenceCapture::SourceReferenceCapture(source_ml_source2_data_property_value22027, _) => {
                                source_ml_source2_data_property_value22027
                            }
                        }, match capture.clone().expect("a proven present value") {
                            self::JavaScriptSourceReferenceCapture::SourceReferenceCapture(_, source_ml_source2_data_property_value22028) => {
                                let source_ml_source2_data_property_value22028 = *source_ml_source2_data_property_value22028;
                                source_ml_source2_data_property_value22028.clone()
                            }
                        }, source.clone(), false);
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213 {
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Next,
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin213Return(Box<self::JavaScriptPatternBindingDeclarations>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin215 {
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin215Next,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Result {
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Done,
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Return(Box<self::JavaScriptPatternBindingDeclarations>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_collect_java_script_pattern_binding_declarations_loop214(mut mutable: bool, mut depth: f64, mut shared: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, mut source_ml_source1_temporary_values211: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorParameterField>>>, mut source_ml_source1_temporary_index211: self::ml::Big) -> self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Result {
    loop {
        return if (source_ml_source1_temporary_index211 < ({ let ml_reference_array654 = source_ml_source1_temporary_values211.clone(); let ml_reference_projection654 = self::ml::Big::from_u128(ml_reference_array654.borrow().len() as u128); ml_reference_projection654 })) {
            {
                let field = { let ml_reference_array655 = source_ml_source1_temporary_values211.clone(); let ml_reference_position655 = source_ml_source1_temporary_index211.clone(); let ml_reference_projection655 = self::ml_array::at(&*ml_reference_array655.borrow(), ml_reference_position655.to_index()); ml_reference_projection655 };
                {
                    let child = self::append_java_script_pattern_binding_declarations(declarations.clone(), match field.clone() {
                        self::JavaScriptErrorParameterField::ErrorParameterField(source_ml_source2_data_property_value22031, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value22031
                        }
                    }, match field.clone() {
                        self::JavaScriptErrorParameterField::ErrorParameterField(_, _, _, _, _, source_ml_source2_data_property_value22032, _) => {
                            ((source_ml_source2_data_property_value22032.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone()
                        }
                    }, mutable, depth, shared.clone());
                    if ((match child.clone() {
                        self::JavaScriptPatternBindingDeclarations::BindingDeclarations(_, source_ml_source2_data_property_value22034) => {
                            source_ml_source2_data_property_value22034.clone()
                        }
                    }) != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Result::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Return(Box::new(child.clone()))
                    } else {
                        {
                            let source_ml_source1_temporary_index211_2 = source_ml_source1_temporary_index211.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index211 = source_ml_source1_temporary_index211_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Result::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop214Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218 {
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218Next,
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsJoin218Return(Box<self::JavaScriptPatternBindingDeclarations>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Result {
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Done,
    MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Return(Box<self::JavaScriptPatternBindingDeclarations>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_collect_java_script_pattern_binding_declarations_loop217(mut mutable: bool, mut depth: f64, mut shared: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut declarations: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, mut source_ml_source1_temporary_values212: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayBindingPart>>>, mut source_ml_source1_temporary_index212: self::ml::Big) -> self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Result {
    loop {
        return if (source_ml_source1_temporary_index212 < ({ let ml_reference_array656 = source_ml_source1_temporary_values212.clone(); let ml_reference_projection656 = self::ml::Big::from_u128(ml_reference_array656.borrow().len() as u128); ml_reference_projection656 })) {
            {
                let field = { let ml_reference_array657 = source_ml_source1_temporary_values212.clone(); let ml_reference_position657 = source_ml_source1_temporary_index212.clone(); let ml_reference_projection657 = self::ml_array::at(&*ml_reference_array657.borrow(), ml_reference_position657.to_index()); ml_reference_projection657 };
                match field.clone() {
                    self::JavaScriptErrorArrayBindingPart::ErrorArrayBindingElement(identity, _, _, pattern, _) => {
                        {
                            let child = self::append_java_script_pattern_binding_declarations(declarations.clone(), identity, ((pattern.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone(), mutable, depth, shared.clone());
                            if ((match child.clone() {
                                self::JavaScriptPatternBindingDeclarations::BindingDeclarations(_, source_ml_source2_data_property_value22041) => {
                                    source_ml_source2_data_property_value22041.clone()
                                }
                            }) != (self::ml_string::Text::from_units(vec![]))) {
                                self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Result::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Return(Box::new(child.clone()))
                            } else {
                                {
                                    let source_ml_source1_temporary_index212_2 = source_ml_source1_temporary_index212.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index212 = source_ml_source1_temporary_index212_2.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptErrorArrayBindingPart::ErrorArrayElision => {
                        {
                            let source_ml_source1_temporary_index212_3 = source_ml_source1_temporary_index212.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index212 = source_ml_source1_temporary_index212_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Result::MlSource1TemporaryCollectJavaScriptPatternBindingDeclarationsLoop217Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221 {
    MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221Next,
    MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsJoin221Return(Box<self::JavaScriptPatternBindingDeclarations>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsLoop222Result {
    MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsLoop222Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_append_java_script_pattern_binding_declarations_loop222(mut declarations: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, mut source_ml_source1_temporary_values219: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, mut source_ml_source1_temporary_index219: self::ml::Big) -> self::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsLoop222Result {
    loop {
        return if (source_ml_source1_temporary_index219 < ({ let ml_reference_array658 = source_ml_source1_temporary_values219.clone(); let ml_reference_projection658 = self::ml::Big::from_u128(ml_reference_array658.borrow().len() as u128); ml_reference_projection658 })) {
            {
                let declaration = { let ml_reference_array659 = source_ml_source1_temporary_values219.clone(); let ml_reference_position659 = source_ml_source1_temporary_index219.clone(); let ml_reference_projection659 = self::ml_array::at(&*ml_reference_array659.borrow(), ml_reference_position659.to_index()); ml_reference_projection659 };
                {
                    let source_ml_source1_temporary_append_java_script_pattern_binding_declarations_ignored223 = { let ml_method_owner660 = declarations.clone(); let ml_method_argument660_0 = declaration.clone(); let ml_inserted = vec![ml_method_argument660_0]; let mut ml_method_values660 = ml_method_owner660.borrow_mut(); let ml_length = ml_method_values660.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values660.extend(ml_inserted); ml_length as f64 };
                    {
                        let source_ml_source1_temporary_index219_2 = source_ml_source1_temporary_index219.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index219 = source_ml_source1_temporary_index219_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsLoop222Result::MlSource1TemporaryAppendJavaScriptPatternBindingDeclarationsLoop222Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_source_array_binding_loop224(pattern: self::JavaScriptErrorParameter, mutable: bool, depth: f64, context: std::rc::Rc<std::cell::RefCell<self::JavaScriptErrorSourceContext>>, source: self::JavaScriptCallSource, at: f64) -> self::JavaScriptCallSource {
    if (at >= (0f64)) {
        {
            let element = {
                let source_ml_source2_nullish_field_receiver22045 = (match pattern.clone() {
                    self::JavaScriptErrorParameter::ErrorRestParameter(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayBindingPart>>>>
                    }
                    self::JavaScriptErrorParameter::ErrorRequiredParameter(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayBindingPart>>>>
                    }
                    self::JavaScriptErrorParameter::ErrorDefaultParameter(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayBindingPart>>>>
                    }
                    self::JavaScriptErrorParameter::ErrorObjectParameter(_, _, _, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptErrorArrayBindingPart>>>>
                    }
                    self::JavaScriptErrorParameter::ErrorArrayParameter(_, _, _, _, source_ml_source2_data_property_value22044, _) => {
                        Some(source_ml_source2_data_property_value22044.clone())
                    }
                }, at);
                {
                    let source_ml_source2_computed_property_key22046 = (source_ml_source2_nullish_field_receiver22045.clone()).1;
                    if match (source_ml_source2_nullish_field_receiver22045.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present22047 = ((source_ml_source2_nullish_field_receiver22045.clone()).0).clone().expect("a proven present value");
                            { let ml_reference_array661 = source_ml_source2_nullish_field_present22047.clone(); let ml_reference_position661 = source_ml_source2_computed_property_key22046; let ml_reference_projection661 = self::ml_array::optional_at(&*ml_reference_array661.borrow(), self::ml_array::number_index(ml_reference_position661)); ml_reference_projection661 }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (((self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number((source_ml_source2_nullish_field_receiver22045.clone()).1)))).concat(&self::ml_string::Text::from_units(vec![39, 41]))).0, message_present: true }).raise()
                    }
                }
            };
            if match (element.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                {
                    let at_2 = (at - 1f64);
                    self::source_ml_source1_temporary_lower_java_script_source_array_binding_loop224(pattern.clone(), mutable, depth, context.clone(), source.clone(), at_2)
                }
            } else {
                match element.clone().expect("a proven present value") {
                    self::JavaScriptErrorArrayBindingPart::ErrorArrayBindingElement(element_identity, has_default, element_value, element_pattern, target) => {
                        let element_value = *element_value;
                        {
                            let nested_pattern = ((element_pattern.clone().unwrap_or_else(|| (None::<self::JavaScriptErrorParameter>,))).0).clone();
                            {
                                let source_3 = if !(match (nested_pattern.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    {
                                        let nested = self::lower_java_script_source_binding_pattern(nested_pattern.clone().expect("a proven present value"), { let ml_cell_value = context.borrow().clone(); ml_cell_value }, source.clone(), mutable, (depth + 1f64), true);
                                        { let ml_cell662 = context.clone(); let ml_value662 = match nested.clone() {
                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value22050) => {
                                                let source_ml_source2_data_property_value22050 = *source_ml_source2_data_property_value22050;
                                                source_ml_source2_data_property_value22050.clone()
                                            }
                                        }; *ml_cell662.borrow_mut() = ml_value662; {
                                            let source_2 = match nested.clone() {
                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value22051, _) => {
                                                    let source_ml_source2_data_property_value22051 = *source_ml_source2_data_property_value22051;
                                                    source_ml_source2_data_property_value22051.clone()
                                                }
                                            };
                                            source_2.clone()
                                        } }
                                    }
                                } else {
                                    source.clone()
                                };
                                {
                                    let identity = match { let ml_cell_value = context.borrow().clone(); ml_cell_value } {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value22052, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value22052
                                        }
                                    };
                                    { let ml_cell663 = context.clone(); let ml_value663 = {
                                        let source_ml_source2_nominal_spread22053 = { let ml_cell_value = context.borrow().clone(); ml_cell_value };
                                        {
                                            let source_ml_source2_nominal_field22054 = match source_ml_source2_nominal_spread22053.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value22055, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value22055 = *source_ml_source2_data_property_value22055;
                                                    source_ml_source2_data_property_value22055.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field22056 = match source_ml_source2_nominal_spread22053.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value22057, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value22057 = *source_ml_source2_data_property_value22057;
                                                        source_ml_source2_data_property_value22057.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field22058 = match source_ml_source2_nominal_spread22053.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value22059, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value22059.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field22060 = match source_ml_source2_nominal_spread22053.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value22061, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value22061
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field22062 = match source_ml_source2_nominal_spread22053.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value22063, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value22063
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field22064 = match source_ml_source2_nominal_spread22053.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value22065, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value22065.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field22066 = match source_ml_source2_nominal_spread22053.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value22067, _, _, _) => {
                                                                            source_ml_source2_data_property_value22067
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field22068 = match source_ml_source2_nominal_spread22053.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value22069, _, _) => {
                                                                                source_ml_source2_data_property_value22069.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field22070 = match source_ml_source2_nominal_spread22053.clone() {
                                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value22071, _) => {
                                                                                    source_ml_source2_data_property_value22071.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field22072 = match source_ml_source2_nominal_spread22053.clone() {
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22073) => {
                                                                                        source_ml_source2_data_property_value22073.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_override22074 = (identity + 1f64);
                                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field22054.clone()), Box::new(source_ml_source2_nominal_field22056.clone()), source_ml_source2_nominal_field22058.clone(), source_ml_source2_nominal_override22074, source_ml_source2_nominal_field22062, source_ml_source2_nominal_field22064.clone(), source_ml_source2_nominal_field22066, source_ml_source2_nominal_field22068.clone(), source_ml_source2_nominal_field22070.clone(), source_ml_source2_nominal_field22072.clone())
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
                                    }; *ml_cell663.borrow_mut() = ml_value663; {
                                        let saved = self::JavaScriptCallSource::SourceBinding(identity);
                                        {
                                            let value = saved.clone();
                                            {
                                                let value_3 = if has_default {
                                                    {
                                                        let lowered = self::lower_java_script_error_expression(element_value.clone(), { let ml_cell_value = context.borrow().clone(); ml_cell_value }, 0f64);
                                                        { let ml_cell664 = context.clone(); let ml_value664 = match lowered.clone() {
                                                            self::JavaScriptErrorSourceExpression::ErrorSourceExpression(_, source_ml_source2_data_property_value22075) => {
                                                                let source_ml_source2_data_property_value22075 = *source_ml_source2_data_property_value22075;
                                                                source_ml_source2_data_property_value22075.clone()
                                                            }
                                                        }; *ml_cell664.borrow_mut() = ml_value664; {
                                                            let value_2 = self::JavaScriptCallSource::SourceIf(Box::new(self::JavaScriptCallSource::SourceStrictEqual(Box::new(saved.clone()), Box::new(self::construct_java_script_error_constant(self::JavaScriptSemanticValue::Undefined)))), Box::new(match lowered.clone() {
                                                                self::JavaScriptErrorSourceExpression::ErrorSourceExpression(source_ml_source2_data_property_value22076, _) => {
                                                                    let source_ml_source2_data_property_value22076 = *source_ml_source2_data_property_value22076;
                                                                    source_ml_source2_data_property_value22076.clone()
                                                                }
                                                            }), Box::new(saved.clone()));
                                                            value_2.clone()
                                                        } }
                                                    }
                                                } else {
                                                    value.clone()
                                                };
                                                {
                                                    let source_4 = self::construct_java_script_error_local_binding({ let ml_cell_value = context.borrow().clone(); ml_cell_value }, identity, self::JavaScriptCallSource::SourceIteratorStep(match pattern.clone() {
                                                        self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value22077, _, _) => {
                                                            source_ml_source2_data_property_value22077
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value22078, _) => {
                                                            source_ml_source2_data_property_value22078
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value22079, _, _) => {
                                                            source_ml_source2_data_property_value22079
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value22080, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value22080
                                                        }
                                                        self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value22081, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value22081
                                                        }
                                                    }, match match { let ml_cell_value = context.borrow().clone(); ml_cell_value } {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value22082, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value22082 = *source_ml_source2_data_property_value22082;
                                                            source_ml_source2_data_property_value22082.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value22083, _, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value22083.clone()
                                                        }
                                                    }, true), self::construct_java_script_source_pattern_binding({ let ml_cell_value = context.borrow().clone(); ml_cell_value }, element_identity, value_3.clone(), source_3.clone(), mutable, true, ((target.clone().unwrap_or_else(|| (None::<self::JavaScriptSourceBindingTarget>,))).0).clone()), false);
                                                    {
                                                        let at_3 = (at - 1f64);
                                                        self::source_ml_source1_temporary_lower_java_script_source_array_binding_loop224(pattern.clone(), mutable, depth, context.clone(), source_4.clone(), at_3)
                                                    }
                                                }
                                            }
                                        }
                                    } }
                                }
                            }
                        }
                    }
                    self::JavaScriptErrorArrayBindingPart::ErrorArrayElision => {
                        {
                            let identity_2 = match { let ml_cell_value = context.borrow().clone(); ml_cell_value } {
                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value22084, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value22084
                                }
                            };
                            { let ml_cell665 = context.clone(); let ml_value665 = {
                                let source_ml_source2_nominal_spread22085 = { let ml_cell_value = context.borrow().clone(); ml_cell_value };
                                {
                                    let source_ml_source2_nominal_field22086 = match source_ml_source2_nominal_spread22085.clone() {
                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(source_ml_source2_data_property_value22087, _, _, _, _, _, _, _, _, _) => {
                                            let source_ml_source2_data_property_value22087 = *source_ml_source2_data_property_value22087;
                                            source_ml_source2_data_property_value22087.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field22088 = match source_ml_source2_nominal_spread22085.clone() {
                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value22089, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value22089 = *source_ml_source2_data_property_value22089;
                                                source_ml_source2_data_property_value22089.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field22090 = match source_ml_source2_nominal_spread22085.clone() {
                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, source_ml_source2_data_property_value22091, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value22091.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field22092 = match source_ml_source2_nominal_spread22085.clone() {
                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, source_ml_source2_data_property_value22093, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value22093
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field22094 = match source_ml_source2_nominal_spread22085.clone() {
                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, source_ml_source2_data_property_value22095, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value22095
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field22096 = match source_ml_source2_nominal_spread22085.clone() {
                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, source_ml_source2_data_property_value22097, _, _, _, _) => {
                                                                source_ml_source2_data_property_value22097.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field22098 = match source_ml_source2_nominal_spread22085.clone() {
                                                                self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, source_ml_source2_data_property_value22099, _, _, _) => {
                                                                    source_ml_source2_data_property_value22099
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field22100 = match source_ml_source2_nominal_spread22085.clone() {
                                                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, source_ml_source2_data_property_value22101, _, _) => {
                                                                        source_ml_source2_data_property_value22101.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field22102 = match source_ml_source2_nominal_spread22085.clone() {
                                                                        self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value22103, _) => {
                                                                            source_ml_source2_data_property_value22103.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field22104 = match source_ml_source2_nominal_spread22085.clone() {
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22105) => {
                                                                                source_ml_source2_data_property_value22105.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_override22106 = (identity_2 + 1f64);
                                                                            self::JavaScriptErrorSourceContext::ErrorSourceContext(Box::new(source_ml_source2_nominal_field22086.clone()), Box::new(source_ml_source2_nominal_field22088.clone()), source_ml_source2_nominal_field22090.clone(), source_ml_source2_nominal_override22106, source_ml_source2_nominal_field22094, source_ml_source2_nominal_field22096.clone(), source_ml_source2_nominal_field22098, source_ml_source2_nominal_field22100.clone(), source_ml_source2_nominal_field22102.clone(), source_ml_source2_nominal_field22104.clone())
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
                            }; *ml_cell665.borrow_mut() = ml_value665; {
                                let source_5 = self::construct_java_script_error_local_binding({ let ml_cell_value = context.borrow().clone(); ml_cell_value }, identity_2, self::JavaScriptCallSource::SourceIteratorStep(match pattern.clone() {
                                    self::JavaScriptErrorParameter::ErrorRestParameter(source_ml_source2_data_property_value22107, _, _) => {
                                        source_ml_source2_data_property_value22107
                                    }
                                    self::JavaScriptErrorParameter::ErrorRequiredParameter(source_ml_source2_data_property_value22108, _) => {
                                        source_ml_source2_data_property_value22108
                                    }
                                    self::JavaScriptErrorParameter::ErrorDefaultParameter(source_ml_source2_data_property_value22109, _, _) => {
                                        source_ml_source2_data_property_value22109
                                    }
                                    self::JavaScriptErrorParameter::ErrorObjectParameter(source_ml_source2_data_property_value22110, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value22110
                                    }
                                    self::JavaScriptErrorParameter::ErrorArrayParameter(source_ml_source2_data_property_value22111, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value22111
                                    }
                                }, match match { let ml_cell_value = context.borrow().clone(); ml_cell_value } {
                                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, source_ml_source2_data_property_value22112, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value22112 = *source_ml_source2_data_property_value22112;
                                        source_ml_source2_data_property_value22112.clone()
                                    }
                                } {
                                    self::JavaScriptErrorRealmOperations::ErrorRealmOperations(source_ml_source2_data_property_value22113, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value22113.clone()
                                    }
                                }, false), source.clone(), false);
                                {
                                    let at_4 = (at - 1f64);
                                    self::source_ml_source1_temporary_lower_java_script_source_array_binding_loop224(pattern.clone(), mutable, depth, context.clone(), source_5.clone(), at_4)
                                }
                            } }
                        }
                    }
                }
            }
        }
    } else {
        source.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructJavaScriptLexicalCellFailureLoop225Result {
    MlSource1TemporaryConstructJavaScriptLexicalCellFailureLoop225Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_lexical_cell_failure_loop225(message: &self::ml_string::Text, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryConstructJavaScriptLexicalCellFailureLoop225Result {
    loop {
        return if (at < (message.length())) {
            {
                let source_ml_source1_temporary_construct_java_script_lexical_cell_failure_ignored226 = { let ml_method_owner666 = units.clone(); let ml_method_argument666_0 = {
                    let source_ml_source2_string_unit_operand22114 = message.clone();
                    {
                        let source_ml_source2_string_unit_operand22115 = at;
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
                                    let source_ml_source2_string_unit_operand22121 = {
                                        let source_ml_source2_string_range_operand22116 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand22117 = index;
                                            {
                                                let source_ml_source2_string_range_operand22118 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position22119 = source_ml_source2_string_range_operand22117;
                                                    {
                                                        let source_ml_source2_string_range_position22120 = source_ml_source2_string_range_operand22118;
                                                        source_ml_source2_string_range_operand22116.slice(source_ml_source2_string_range_position22119, source_ml_source2_string_range_position22120)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand22122 = 0f64;
                                        source_ml_source2_string_unit_operand22121.code_point_at(source_ml_source2_string_unit_operand22122)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand22114.clone(), source_ml_source2_string_unit_operand22115)
                    }
                }; let ml_inserted = vec![ml_method_argument666_0]; let mut ml_method_values666 = ml_method_owner666.borrow_mut(); let ml_length = ml_method_values666.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values666.extend(ml_inserted); ml_length as f64 };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConstructJavaScriptLexicalCellFailureLoop225Result::MlSource1TemporaryConstructJavaScriptLexicalCellFailureLoop225Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop230Result {
    MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop230Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop231Result {
    MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop231Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop231(mut declarations: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values228: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index228: self::ml::Big) -> self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop231Result {
    loop {
        return if (source_ml_source1_temporary_index228 < ({ let ml_reference_array667 = source_ml_source1_temporary_values228.clone(); let ml_reference_projection667 = self::ml::Big::from_u128(ml_reference_array667.borrow().len() as u128); ml_reference_projection667 })) {
            {
                let identity = { let ml_reference_array668 = source_ml_source1_temporary_values228.clone(); let ml_reference_position668 = source_ml_source1_temporary_index228.clone(); let ml_reference_projection668 = self::ml_array::at(&*ml_reference_array668.borrow(), ml_reference_position668.to_index()); ml_reference_projection668 };
                {
                    let source_ml_source1_temporary_collect_java_script_source_lexical_declarations_ignored232 = { let ml_method_owner669 = declarations.clone(); let ml_method_argument669_0 = identity; let ml_inserted = vec![ml_method_argument669_0]; let mut ml_method_values669 = ml_method_owner669.borrow_mut(); let ml_length = ml_method_values669.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values669.extend(ml_inserted); ml_length as f64 };
                    {
                        let source_ml_source1_temporary_index228_2 = source_ml_source1_temporary_index228.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index228 = source_ml_source1_temporary_index228_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop231Result::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop231Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233 {
    MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin233Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235 {
    MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsJoin235Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Result {
    MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_collect_java_script_source_lexical_declarations_loop236(mut context: self::JavaScriptErrorSourceContext, mut declarations: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values229: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, mut source_ml_source1_temporary_index229: self::ml::Big) -> self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Result {
    loop {
        return if (source_ml_source1_temporary_index229 < ({ let ml_reference_array670 = source_ml_source1_temporary_values229.clone(); let ml_reference_projection670 = self::ml::Big::from_u128(ml_reference_array670.borrow().len() as u128); ml_reference_projection670 })) {
            {
                let row = { let ml_reference_array671 = source_ml_source1_temporary_values229.clone(); let ml_reference_position671 = source_ml_source1_temporary_index229.clone(); let ml_reference_projection671 = self::ml_array::at(&*ml_reference_array671.borrow(), ml_reference_position671.to_index()); ml_reference_projection671 };
                if { let ml_values672 = { let ml_reference_value673 = ((match context.clone() {
                    self::JavaScriptErrorSourceContext::ErrorSourceContext(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value22127) => {
                        ((source_ml_source2_data_property_value22127.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSourceLexicalBinding>>>>,))).0).clone()
                    }
                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSourceLexicalBinding>::new())))).borrow().clone(); ml_reference_value673 }; let ml_callback672 = { let row = row.clone(); std::rc::Rc::new(move |binding: self::JavaScriptSourceLexicalBinding| -> bool { ((match binding.clone() {
                    self::JavaScriptSourceLexicalBinding::SourceLexicalBinding(source_ml_source2_data_property_value22130, _, _, _, _) => {
                        source_ml_source2_data_property_value22130
                    }
                }) == (match row.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value22131, _) => {
                        source_ml_source2_data_property_value22131
                    }
                })) }) }; ml_values672.iter().cloned().enumerate().any(|(ml_index672, ml_value672)| ml_callback672(ml_value672)) } {
                    {
                        let source_ml_source1_temporary_collect_java_script_source_lexical_declarations_ignored237 = { let ml_method_owner674 = declarations.clone(); let ml_method_argument674_0 = match row.clone() {
                            self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value22132, _) => {
                                source_ml_source2_data_property_value22132
                            }
                        }; let ml_inserted = vec![ml_method_argument674_0]; let mut ml_method_values674 = ml_method_owner674.borrow_mut(); let ml_length = ml_method_values674.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values674.extend(ml_inserted); ml_length as f64 };
                        {
                            let source_ml_source1_temporary_index229_2 = source_ml_source1_temporary_index229.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index229 = source_ml_source1_temporary_index229_2.clone();
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index229_3 = source_ml_source1_temporary_index229.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index229 = source_ml_source1_temporary_index229_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Result::MlSource1TemporaryCollectJavaScriptSourceLexicalDeclarationsLoop236Done
        };
    }
}
