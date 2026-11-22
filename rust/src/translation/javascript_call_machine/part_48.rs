// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptObjectCopyCall;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptObjectSpreadTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304;
use super::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305;
use super::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306;
use super::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307;
use super::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop303Result;
use super::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop308Result;
use super::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop309Result;
use super::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop310Result;
use super::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop311Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_assignment_state_error;
use super::read_java_script_object_spread_key_name;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::source_ml_source1_temporary_read_java_script_object_spread_state_error_loop303;
use super::source_ml_source1_temporary_read_java_script_object_spread_state_error_loop308;
use super::source_ml_source1_temporary_read_java_script_object_spread_state_error_loop309;
use super::source_ml_source1_temporary_read_java_script_object_spread_state_error_loop310;
use super::source_ml_source1_temporary_read_java_script_object_spread_state_error_loop311;
use super::test_duplicate_java_script_object_spread_key_name;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_spread_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectSpreadState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptObjectSpreadState::ObjectSpreadState(
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field4,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                    if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                        heap_error.clone()
                    } else {
                        {
                            let source_ml_source1_temporary_values299 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    shared_constructor_field5.clone(),
                                    shared_constructor_field6.clone(),
                                    shared_constructor_field11.clone(),
                                    shared_constructor_field12.clone(),
                                ]));
                            {
                                let source_ml_source1_temporary_index299 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s7493 = self::source_ml_source1_temporary_read_java_script_object_spread_state_error_loop303(heap.clone(), source_ml_source1_temporary_values299.clone(), source_ml_source1_temporary_index299.clone());
                                    match source_ml_source2_s7493.clone() {
                                            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop303Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop303Done => {
                                                if {
                                                    let source_ml_source2_s7495 = shared_constructor_field5.clone();
                                                    match source_ml_source2_s7495.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }
                                                } {
                                                    self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 108, 105, 116, 101, 114, 97, 108, 32, 116, 97, 114, 103, 101, 116])
                                                } else {
                                                    {
                                                        let assignment = {
                                                            let source_ml_source2_comparison_operand7500 = match state.clone() {
                                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value7499, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value7499.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                }
                                                            };
                                                            match (source_ml_source2_comparison_operand7500.clone(), self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                        };
                                                        if ({
                                                            let source_ml_source2_comparison_operand7503 = match state.clone() {
                                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value7502, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value7502.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_comparison_operand7504 = None::<()>;
                                                                !(match (source_ml_source2_comparison_operand7503.clone(), source_ml_source2_comparison_operand7504.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                            }
                                                        } && !assignment) {
                                                            self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 99, 111, 112, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 67, 114, 101, 97, 116, 101, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121, 32, 111, 114, 32, 83, 101, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
                                                        } else {
                                                            {
                                                                let target = {
                                                                    let source_ml_source2_array_read_value7509 = shared_constructor_field4.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index7510 = self::find_java_script_ordinary_object(shared_constructor_field4.clone(), (((match shared_constructor_field5.clone() {
                                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value7508, _) => {
                                                                                Some((source_ml_source2_data_property_value7508.clone(),))
                                                                            }
                                                                            _ => {
                                                                                None::<(self::ml::Big,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0).clone());
                                                                        { let ml_reference_array605 = source_ml_source2_array_read_value7509.clone(); let ml_reference_position605 = source_ml_source2_array_read_index7510; let ml_reference_projection605 = self::ml_array::optional_at(&*ml_reference_array605.borrow(), self::ml_array::number_index(ml_reference_position605)); ml_reference_projection605 }
                                                                    }
                                                                };
                                                                if (match (target.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || if assignment {
                                                                    !(self::ml_array::includes(&{ let ml_reference_value606 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value606 }, &match target.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                    }, 0f64, false, |a, b| a == b))
                                                                } else {
                                                                    (match target.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    } || ((match target.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value7520, _) => {
                                                                            Some((source_ml_source2_data_property_value7520,))
                                                                        }
                                                                        _ => {
                                                                            None::<(bool,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0)
                                                                }) {
                                                                    self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 99, 111, 112, 121, 32, 116, 97, 114, 103, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 112, 111, 114, 116, 101, 100, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 107, 105, 110, 100])
                                                                } else {
                                                                    if (assignment && ((match (match state.clone() {
                                                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7521, _, _) => {
                                                                            ((source_ml_source2_data_property_value7521.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                        }
                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || {
                                                                        let source_ml_source2_s7523 = (match state.clone() {
                                                                            self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7525, _, _) => {
                                                                                ((source_ml_source2_data_property_value7525.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value");
                                                                        match source_ml_source2_s7523.clone() {
                                                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                                                false
                                                                            }
                                                                            _ => {
                                                                                true
                                                                            }
                                                                        }
                                                                    }) || ((self::read_java_script_ordinary_value_error(heap.clone(), (match state.clone() {
                                                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7527, _, _) => {
                                                                            ((source_ml_source2_data_property_value7527.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![]))))) {
                                                                        self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108])
                                                                    } else {
                                                                        if (((((!(self::ml_array::includes(&{ let ml_reference_value607 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![107, 101, 121, 115]), self::ml_string::Text::from_units(vec![114, 101, 97, 100]), self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), self::ml_string::Text::from_units(vec![115, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])]))).borrow().clone(); ml_reference_value607 }, &shared_constructor_field10.clone(), 0f64, false, |a, b| a == b)) || !(self::ml_math::is_safe_integer(shared_constructor_field9))) || (shared_constructor_field9 < (0f64))) || {
                                                                            let source_ml_source2_same_value_operand7533 = shared_constructor_field9;
                                                                            {
                                                                                let source_ml_source2_same_value_operand7534 = (-0f64);
                                                                                (((source_ml_source2_same_value_operand7533 == source_ml_source2_same_value_operand7534) && ((source_ml_source2_same_value_operand7533 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand7533)) == ((1f64 / source_ml_source2_same_value_operand7534))))) || ((source_ml_source2_same_value_operand7533 != source_ml_source2_same_value_operand7533) && (source_ml_source2_same_value_operand7534 != source_ml_source2_same_value_operand7534)))
                                                                            }
                                                                        }) || (shared_constructor_field9 > ({ let ml_reference_array608 = shared_constructor_field8.clone(); let ml_reference_projection608 = ml_reference_array608.borrow().len() as f64; ml_reference_projection608 }))) || (({ let ml_reference_array609 = shared_constructor_field8.clone(); let ml_reference_projection609 = ml_reference_array609.borrow().len() as f64; ml_reference_projection609 }) > (4096f64))) {
                                                                            self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 98, 111, 117, 110, 100, 101, 100, 32, 107, 101, 121, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 97, 110, 100, 32, 101, 120, 97, 99, 116, 32, 99, 117, 114, 115, 111, 114])
                                                                        } else {
                                                                            if (((shared_constructor_field10 == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) && ((({ let ml_reference_array610 = shared_constructor_field8.clone(); let ml_reference_projection610 = ml_reference_array610.borrow().len() as f64; ml_reference_projection610 }) != (0f64)) || (shared_constructor_field9 != (0f64)))) || (self::ml_array::includes(&{ let ml_reference_value611 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), self::ml_string::Text::from_units(vec![115, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])]))).borrow().clone(); ml_reference_value611 }, &shared_constructor_field10.clone(), 0f64, false, |a, b| a == b) && (shared_constructor_field9 >= ({ let ml_reference_array612 = shared_constructor_field8.clone(); let ml_reference_projection612 = ml_reference_array612.borrow().len() as f64; ml_reference_projection612 })))) {
                                                                                self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 112, 104, 97, 115, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121])
                                                                            } else {
                                                                                if ((!(self::ml_array::includes(&{ let ml_reference_value613 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), self::ml_string::Text::from_units(vec![115, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])]))).borrow().clone(); ml_reference_value613 }, &shared_constructor_field10.clone(), 0f64, false, |a, b| a == b)) && {
                                                                                    let source_ml_source2_s7554 = shared_constructor_field11.clone();
                                                                                    match source_ml_source2_s7554.clone() {
                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                            false
                                                                                        }
                                                                                        _ => {
                                                                                            true
                                                                                        }
                                                                                    }
                                                                                }) || (!(self::ml_array::includes(&{ let ml_reference_value614 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])]))).borrow().clone(); ml_reference_value614 }, &shared_constructor_field10.clone(), 0f64, false, |a, b| a == b)) && {
                                                                                    let source_ml_source2_s7558 = shared_constructor_field12.clone();
                                                                                    match source_ml_source2_s7558.clone() {
                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                            false
                                                                                        }
                                                                                        _ => {
                                                                                            true
                                                                                        }
                                                                                    }
                                                                                })) {
                                                                                    self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 111, 110, 108, 121, 32, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 104, 97, 115, 101])
                                                                                } else {
                                                                                    if ((shared_constructor_field10 == (self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]))) && (({
                                                                                        let source_ml_source2_s7563 = shared_constructor_field6.clone();
                                                                                        match source_ml_source2_s7563.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                false
                                                                                            }
                                                                                            _ => {
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    } || {
                                                                                        let source_ml_source2_s7568 = shared_constructor_field12.clone();
                                                                                        match source_ml_source2_s7568.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                false
                                                                                            }
                                                                                            _ => {
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    }) || !(((match shared_constructor_field12.clone() {
                                                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value7573) => {
                                                                                            Some((source_ml_source2_data_property_value7573,))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(bool,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0))) {
                                                                                        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 103, 101, 116, 116, 101, 114, 32, 115, 117, 115, 112, 101, 110, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 110, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                                                                                    } else {
                                                                                        if ((((shared_constructor_field10 == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])))) != (!(match (match state.clone() {
                                                                                            self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7579, _) => {
                                                                                                ((source_ml_source2_data_property_value7579.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) || (!assignment && self::ml_array::includes(&{ let ml_reference_value615 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])]))).borrow().clone(); ml_reference_value615 }, &shared_constructor_field10.clone(), 0f64, false, |a, b| a == b))) {
                                                                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 111, 110, 108, 121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 115, 116, 101, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 112, 104, 97, 115, 101])
                                                                                        } else {
                                                                                            {
                                                                                                let source_ml_source2_s7607 = if !(match (match state.clone() {
                                                                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7608, _) => {
                                                                                                        ((source_ml_source2_data_property_value7608.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                                                                    }
                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                    {
                                                                                                        let invalid = self::read_java_script_array_assignment_state_error(heap.clone(), (match state.clone() {
                                                                                                            self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7609, _) => {
                                                                                                                ((source_ml_source2_data_property_value7609.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                                                                            }
                                                                                                        }).clone().expect("a proven present value"));
                                                                                                        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304Return(invalid.clone())
                                                                                                        } else {
                                                                                                            {
                                                                                                                let exotic_key = match state.clone() {
                                                                                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7610, _, _) => {
                                                                                                                        ((source_ml_source2_data_property_value7610.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                if ((({
                                                                                                                    let source_ml_source2_s7612 = shared_constructor_field5.clone();
                                                                                                                    match source_ml_source2_s7612.clone() {
                                                                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                            false
                                                                                                                        }
                                                                                                                        _ => {
                                                                                                                            true
                                                                                                                        }
                                                                                                                    }
                                                                                                                } || ((match match (match state.clone() {
                                                                                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7623, _) => {
                                                                                                                        ((source_ml_source2_data_property_value7623.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                                                                                    }
                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(source_ml_source2_data_property_value7624, _) => {
                                                                                                                        let source_ml_source2_data_property_value7624 = *source_ml_source2_data_property_value7624;
                                                                                                                        source_ml_source2_data_property_value7624.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(source_ml_source2_data_property_value7625, _, _) => {
                                                                                                                        let source_ml_source2_data_property_value7625 = *source_ml_source2_data_property_value7625;
                                                                                                                        source_ml_source2_data_property_value7625.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(source_ml_source2_data_property_value7626, _, _) => {
                                                                                                                        let source_ml_source2_data_property_value7626 = *source_ml_source2_data_property_value7626;
                                                                                                                        source_ml_source2_data_property_value7626.clone()
                                                                                                                    }
                                                                                                                } {
                                                                                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source2_data_property_value7627, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value7627.clone()
                                                                                                                    }
                                                                                                                }) != ((((match shared_constructor_field5.clone() {
                                                                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value7629, _) => {
                                                                                                                        Some((source_ml_source2_data_property_value7629.clone(),))
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        None::<(self::ml::Big,)>
                                                                                                                    }
                                                                                                                }).clone().expect("a proven present value")).0).clone()))) || match (exotic_key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7630, _) => {
                                                                                                                        ((source_ml_source2_data_property_value7630.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                                                                                    }
                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(_, source_ml_source2_data_property_value7631) => {
                                                                                                                        let source_ml_source2_data_property_value7631 = *source_ml_source2_data_property_value7631;
                                                                                                                        source_ml_source2_data_property_value7631.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(_, source_ml_source2_data_property_value7632, _) => {
                                                                                                                        let source_ml_source2_data_property_value7632 = *source_ml_source2_data_property_value7632;
                                                                                                                        source_ml_source2_data_property_value7632.clone()
                                                                                                                    }
                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(_, source_ml_source2_data_property_value7633, _) => {
                                                                                                                        let source_ml_source2_data_property_value7633 = *source_ml_source2_data_property_value7633;
                                                                                                                        source_ml_source2_data_property_value7633.clone()
                                                                                                                    }
                                                                                                                }, exotic_key.clone().expect("a proven present value")))) {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110, 32, 65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 32, 97, 110, 100, 32, 83, 121, 109, 98, 111, 108]))
                                                                                                                } else {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304Next
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304Next
                                                                                                };
                                                                                                match source_ml_source2_s7607.clone() {
                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304Next => {
                                                                                                        {
                                                                                                            let source_ml_source2_s7653 = if !(match (match state.clone() {
                                                                                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7654) => {
                                                                                                                    ((source_ml_source2_data_property_value7654.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectCopyCall>,))).0).clone()
                                                                                                                }
                                                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                                {
                                                                                                                    let call = (match state.clone() {
                                                                                                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7655) => {
                                                                                                                            ((source_ml_source2_data_property_value7655.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectCopyCall>,))).0).clone()
                                                                                                                        }
                                                                                                                    }).clone().expect("a proven present value");
                                                                                                                    match call.clone() {
                                                                                                                        self::JavaScriptObjectCopyCall::ObjectCopyCall(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                                                                                            let shared_constructor_field1 = *shared_constructor_field1;
                                                                                                                            let shared_constructor_field2 = *shared_constructor_field2;
                                                                                                                            {
                                                                                                                                let source_ml_source1_temporary_values300 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![shared_constructor_field1.clone(), shared_constructor_field2.clone()], { let ml_reference_value616 = shared_constructor_field3.borrow().clone(); ml_reference_value616 })));
                                                                                                                                {
                                                                                                                                    let source_ml_source1_temporary_index300 = self::ml::Big::from_i128(0);
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_s7659 = self::source_ml_source1_temporary_read_java_script_object_spread_state_error_loop311(heap.clone(), source_ml_source1_temporary_values300.clone(), source_ml_source1_temporary_index300.clone());
                                                                                                                                        match source_ml_source2_s7659.clone() {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop311Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop311Done => {
                                                                                                                                                if (!(self::ml_array::includes(&{ let ml_reference_value617 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])]))).borrow().clone(); ml_reference_value617 }, &shared_constructor_field10.clone(), 0f64, false, |a, b| a == b)) || !(self::test_java_script_semantic_same_value(shared_constructor_field12.clone(), shared_constructor_field1.clone()))) {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305Return(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 99, 111, 112, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 32, 112, 104, 97, 115, 101]))
                                                                                                                                                } else {
                                                                                                                                                    if (shared_constructor_field10 == (self::ml_string::Text::from_units(vec![115, 101, 116, 116, 101, 114]))) {
                                                                                                                                                        {
                                                                                                                                                            let argument = {
                                                                                                                                                                let source_ml_source2_array_read_value7666 = shared_constructor_field3.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_array_read_index7667 = 0f64;
                                                                                                                                                                    { let ml_reference_array618 = source_ml_source2_array_read_value7666.clone(); let ml_reference_position618 = source_ml_source2_array_read_index7667; let ml_reference_projection618 = self::ml_array::optional_at(&*ml_reference_array618.borrow(), self::ml_array::number_index(ml_reference_position618)); ml_reference_projection618 }
                                                                                                                                                                }
                                                                                                                                                            };
                                                                                                                                                            if (((!(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), shared_constructor_field5.clone())) || (({ let ml_reference_array619 = shared_constructor_field3.clone(); let ml_reference_projection619 = ml_reference_array619.borrow().len() as f64; ml_reference_projection619 }) != (1f64))) || match (argument.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::test_java_script_semantic_same_value(argument.clone().expect("a proven present value"), shared_constructor_field11.clone()))) {
                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 116, 116, 101, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 104, 101, 108, 100, 32, 118, 97, 108, 117, 101]))
                                                                                                                                                            } else {
                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305Next
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305Next
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop311Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop311Return(source_ml_source1_temporary_result311) => {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305Return(source_ml_source1_temporary_result311.clone())
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305Next
                                                                                                            };
                                                                                                            match source_ml_source2_s7653.clone() {
                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305Next => {
                                                                                                                    {
                                                                                                                        let source_ml_source2_s7683 = if !(match (match state.clone() {
                                                                                                                            self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7684, _) => {
                                                                                                                                ((source_ml_source2_data_property_value7684.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                                                                                            }
                                                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                                            {
                                                                                                                                let key = {
                                                                                                                                    let source_ml_source2_array_read_value7687 = shared_constructor_field8.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_array_read_index7688 = shared_constructor_field9;
                                                                                                                                        { let ml_reference_array620 = source_ml_source2_array_read_value7687.clone(); let ml_reference_position620 = source_ml_source2_array_read_index7688; let ml_reference_projection620 = self::ml_array::optional_at(&*ml_reference_array620.borrow(), self::ml_array::number_index(ml_reference_position620)); ml_reference_projection620 }
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                if (match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(match match (match state.clone() {
                                                                                                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7689, _) => {
                                                                                                                                        ((source_ml_source2_data_property_value7689.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(source_ml_source2_data_property_value7690, _) => {
                                                                                                                                        let source_ml_source2_data_property_value7690 = *source_ml_source2_data_property_value7690;
                                                                                                                                        source_ml_source2_data_property_value7690.clone()
                                                                                                                                    }
                                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(source_ml_source2_data_property_value7691, _, _) => {
                                                                                                                                        let source_ml_source2_data_property_value7691 = *source_ml_source2_data_property_value7691;
                                                                                                                                        source_ml_source2_data_property_value7691.clone()
                                                                                                                                    }
                                                                                                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(source_ml_source2_data_property_value7692, _, _) => {
                                                                                                                                        let source_ml_source2_data_property_value7692 = *source_ml_source2_data_property_value7692;
                                                                                                                                        source_ml_source2_data_property_value7692.clone()
                                                                                                                                    }
                                                                                                                                } {
                                                                                                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source2_data_property_value7693, _, _, _, _) => {
                                                                                                                                        let source_ml_source2_data_property_value7693 = *source_ml_source2_data_property_value7693;
                                                                                                                                        source_ml_source2_data_property_value7693.clone()
                                                                                                                                    }
                                                                                                                                }, key.clone().expect("a proven present value")))) {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110, 32, 65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121]))
                                                                                                                                } else {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306Next
                                                                                                                                }
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306Next
                                                                                                                        };
                                                                                                                        match source_ml_source2_s7683.clone() {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306Next => {
                                                                                                                                if ((shared_constructor_field10 == (self::ml_string::Text::from_units(vec![115, 101, 116, 116, 101, 114]))) && match (match state.clone() {
                                                                                                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value7696) => {
                                                                                                                                        ((source_ml_source2_data_property_value7696.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectCopyCall>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                                                    self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110, 32, 115, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108])
                                                                                                                                } else {
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_s7717 = if ((shared_constructor_field10 != (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) && {
                                                                                                                                            let source_ml_source2_s7721 = shared_constructor_field6.clone();
                                                                                                                                            match source_ml_source2_s7721.clone() {
                                                                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                                                    false
                                                                                                                                                }
                                                                                                                                                _ => {
                                                                                                                                                    true
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }) {
                                                                                                                                            {
                                                                                                                                                let length = if ({
                                                                                                                                                    let source_ml_source2_s7726 = shared_constructor_field6.clone();
                                                                                                                                                    match source_ml_source2_s7726.clone() {
                                                                                                                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                                                                                                                            true
                                                                                                                                                        }
                                                                                                                                                        _ => {
                                                                                                                                                            false
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                } || {
                                                                                                                                                    let source_ml_source2_s7730 = shared_constructor_field6.clone();
                                                                                                                                                    match source_ml_source2_s7730.clone() {
                                                                                                                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                                                                                                            true
                                                                                                                                                        }
                                                                                                                                                        _ => {
                                                                                                                                                            false
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }) {
                                                                                                                                                    self::read_java_script_semantic_text_length(shared_constructor_field6.clone())
                                                                                                                                                } else {
                                                                                                                                                    0f64
                                                                                                                                                };
                                                                                                                                                if (length != ({ let ml_reference_array621 = shared_constructor_field8.clone(); let ml_reference_projection621 = ml_reference_array621.borrow().len() as f64; ml_reference_projection621 })) {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307Return(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 115, 112, 114, 101, 97, 100, 32, 115, 110, 97, 112, 115, 104, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 108, 108, 32, 97, 99, 116, 117, 97, 108, 32, 83, 116, 114, 105, 110, 103, 32, 105, 110, 100, 105, 99, 101, 115, 32, 97, 110, 100, 32, 110, 111, 32, 105, 110, 118, 101, 110, 116, 101, 100, 32, 107, 101, 121, 115]))
                                                                                                                                                } else {
                                                                                                                                                    {
                                                                                                                                                        let at = 0f64;
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_s7737 = self::source_ml_source1_temporary_read_java_script_object_spread_state_error_loop310(state.clone(), at);
                                                                                                                                                            match source_ml_source2_s7737.clone() {
                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop310Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop310Done => {
                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307Next
                                                                                                                                                                }
                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop310Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop310Return(source_ml_source1_temporary_result310) => {
                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307Return(source_ml_source1_temporary_result310.clone())
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        } else {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307Next
                                                                                                                                        };
                                                                                                                                        match source_ml_source2_s7717.clone() {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307Next => {
                                                                                                                                                if (({ let ml_reference_array622 = shared_constructor_field7.clone(); let ml_reference_projection622 = ml_reference_array622.borrow().len() as f64; ml_reference_projection622 }) > (4096f64)) {
                                                                                                                                                    self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 101, 120, 99, 108, 117, 100, 101, 100, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 98, 117, 100, 103, 101, 116])
                                                                                                                                                } else {
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source1_temporary_values301 = shared_constructor_field7.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source1_temporary_index301 = self::ml::Big::from_i128(0);
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_s7741 = self::source_ml_source1_temporary_read_java_script_object_spread_state_error_loop308(heap.clone(), source_ml_source1_temporary_values301.clone(), source_ml_source1_temporary_index301.clone());
                                                                                                                                                                match source_ml_source2_s7741.clone() {
                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop308Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop308Done => {
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_temporary_values302 = shared_constructor_field8.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source1_temporary_index302 = self::ml::Big::from_i128(0);
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_s7743 = self::source_ml_source1_temporary_read_java_script_object_spread_state_error_loop309(heap.clone(), source_ml_source1_temporary_values302.clone(), source_ml_source1_temporary_index302.clone());
                                                                                                                                                                                    match source_ml_source2_s7743.clone() {
                                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop309Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop309Done => {
                                                                                                                                                                                            if self::test_duplicate_java_script_object_spread_key_name(std::rc::Rc::new(std::cell::RefCell::new({ let ml_values623 = { let ml_reference_value624 = shared_constructor_field8.borrow().clone(); ml_reference_value624 }; let ml_callback623 = std::rc::Rc::new(move |key_2: self::JavaScriptSemanticValue| -> self::ml_string::Text { self::read_java_script_object_spread_key_name(key_2.clone()) }); ml_values623.iter().cloned().enumerate().map(|(ml_index623, ml_value623)| ml_callback623(ml_value623)).collect::<Vec<_>>() }))) {
                                                                                                                                                                                                self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 115, 112, 114, 101, 97, 100, 32, 111, 119, 110, 45, 107, 101, 121, 32, 115, 110, 97, 112, 115, 104, 111, 116, 115, 32, 99, 97, 110, 110, 111, 116, 32, 100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 107, 101, 121])
                                                                                                                                                                                            } else {
                                                                                                                                                                                                self::ml_string::Text::from_units(vec![])
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop309Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop309Return(source_ml_source1_temporary_result309) => {
                                                                                                                                                                                            source_ml_source1_temporary_result309.clone()
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop308Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop308Return(source_ml_source1_temporary_result309_2) => {
                                                                                                                                                                        source_ml_source1_temporary_result309_2.clone()
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307Return(source_ml_source1_temporary_result309_3) => {
                                                                                                                                                source_ml_source1_temporary_result309_3.clone()
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306Return(source_ml_source1_temporary_result310_2) => {
                                                                                                                                source_ml_source1_temporary_result310_2.clone()
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305Return(source_ml_source1_temporary_result310_3) => {
                                                                                                                    source_ml_source1_temporary_result310_3.clone()
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304Return(source_ml_source1_temporary_result311_2) => {
                                                                                                        source_ml_source1_temporary_result311_2.clone()
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
                                            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop303Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop303Return(source_ml_source1_temporary_result311_3) => {
                                                source_ml_source1_temporary_result311_3.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_object_spread(
    heap: self::JavaScriptOrdinaryHeap,
    target: self::JavaScriptSemanticValue,
    input: self::JavaScriptSemanticValue,
    excluded: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptObjectSpreadTransition {
    {
        let state = {
            let source_ml_source2_constructor_field7748 = std::rc::Rc::new(
                std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()),
            );
            {
                let source_ml_source2_constructor_field7749 = 0f64;
                {
                    let source_ml_source2_constructor_field7751 =
                        self::JavaScriptSemanticValue::Undefined;
                    {
                        let source_ml_source2_constructor_field7752 =
                            self::JavaScriptSemanticValue::Undefined;
                        self::JavaScriptObjectSpreadState::ObjectSpreadState(
                            Box::new(target.clone()),
                            Box::new(input.clone()),
                            excluded.clone(),
                            source_ml_source2_constructor_field7748.clone(),
                            source_ml_source2_constructor_field7749,
                            self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
                            Box::new(source_ml_source2_constructor_field7751.clone()),
                            Box::new(source_ml_source2_constructor_field7752.clone()),
                            None::<(Option<self::ml_string::Text>,)>,
                            None::<(Option<self::JavaScriptSemanticValue>,)>,
                            None::<(Option<self::JavaScriptArrayAssignmentState>,)>,
                            None::<(Option<self::JavaScriptObjectCopyCall>,)>,
                        )
                    }
                }
            }
        };
        {
            let error =
                self::read_java_script_object_spread_state_error(heap.clone(), state.clone());
            if (error == (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                )
            } else {
                self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            }
        }
    }
}
