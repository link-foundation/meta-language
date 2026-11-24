// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCallbackCall;
use super::JavaScriptArrayCallbackState;
use super::JavaScriptArrayCallbackTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245;
use super::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246;
use super::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1244Result;
use super::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1247Result;
use super::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1248Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_like_length;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_primitive_conversion_state_error;
use super::source_ml_source1_temporary_read_java_script_array_callback_state_error_loop1244;
use super::source_ml_source1_temporary_read_java_script_array_callback_state_error_loop1247;
use super::source_ml_source1_temporary_read_java_script_array_callback_state_error_loop1248;
use super::test_java_script_array_callback_operation;
use super::test_java_script_array_callback_reduction;
use super::test_java_script_array_callback_reverse;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_callback_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCallbackState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptArrayCallbackState::ArrayCallbackState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
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
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            {
                let error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    if ((!(self::test_java_script_array_callback_operation(
                        &(shared_constructor_field1),
                    )) || {
                        let source_ml_source2_s28149 = shared_constructor_field2.clone();
                        match source_ml_source2_s28149.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        }
                    }) || {
                        let source_ml_source2_s28154 = shared_constructor_field5.clone();
                        match source_ml_source2_s28154.clone() {
                            self::JavaScriptSemanticValue::Symbol(_) => false,
                            _ => true,
                        }
                    }) {
                        self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32,
                            114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99,
                            116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 44,
                            32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101,
                            114, 32, 97, 110, 100, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                            110, 32, 83, 121, 109, 98, 111, 108,
                        ])
                    } else {
                        {
                            let source_ml_source1_temporary_values1242 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    shared_constructor_field2.clone(),
                                    shared_constructor_field3.clone(),
                                    shared_constructor_field4.clone(),
                                    shared_constructor_field5.clone(),
                                    shared_constructor_field12.clone(),
                                    shared_constructor_field11.clone(),
                                ]));
                            {
                                let source_ml_source1_temporary_index1242 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s28163 = self::source_ml_source1_temporary_read_java_script_array_callback_state_error_loop1244(heap.clone(), source_ml_source1_temporary_values1242.clone(), source_ml_source1_temporary_index1242.clone());
                                    match source_ml_source2_s28163.clone() {
                                        self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1244Result::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1244Done => {
                                            if (((({
                                                let source_ml_source2_type_query_operand28167 = shared_constructor_field9;
                                                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                                            }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]))) || (({
                                                let source_ml_source2_type_query_operand28171 = shared_constructor_field10;
                                                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                                            }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) || (!(self::test_java_script_array_callback_reduction(&(shared_constructor_field1))) && shared_constructor_field10)) {
                                                self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 114, 101, 100, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 105, 116, 105, 97, 108, 45, 118, 97, 108, 117, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101, 32, 97, 110, 100, 32, 97, 99, 99, 117, 109, 117, 108, 97, 116, 111, 114, 32, 115, 116, 97, 116, 101])
                                            } else {
                                                if ((((((!(self::ml_math::is_safe_integer(shared_constructor_field6)) || (shared_constructor_field6 < (0f64))) || {
                                                    let source_ml_source2_same_value_operand28178 = shared_constructor_field6;
                                                    {
                                                        let source_ml_source2_same_value_operand28179 = (-0f64);
                                                        (((source_ml_source2_same_value_operand28178 == source_ml_source2_same_value_operand28179) && ((source_ml_source2_same_value_operand28178 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand28178)) == ((1f64 / source_ml_source2_same_value_operand28179))))) || ((source_ml_source2_same_value_operand28178 != source_ml_source2_same_value_operand28178) && (source_ml_source2_same_value_operand28179 != source_ml_source2_same_value_operand28179)))
                                                    }
                                                }) || !(self::ml_math::is_safe_integer(shared_constructor_field7))) || (shared_constructor_field7 < ((-1f64)))) || (shared_constructor_field7 > shared_constructor_field6)) || {
                                                    let source_ml_source2_same_value_operand28188 = shared_constructor_field7;
                                                    {
                                                        let source_ml_source2_same_value_operand28189 = (-0f64);
                                                        (((source_ml_source2_same_value_operand28188 == source_ml_source2_same_value_operand28189) && ((source_ml_source2_same_value_operand28188 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand28188)) == ((1f64 / source_ml_source2_same_value_operand28189))))) || ((source_ml_source2_same_value_operand28188 != source_ml_source2_same_value_operand28188) && (source_ml_source2_same_value_operand28189 != source_ml_source2_same_value_operand28189)))
                                                    }
                                                }) {
                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 101, 120, 97, 99, 116, 32, 105, 110, 100, 101, 120])
                                                } else {
                                                    if !(self::ml_array::includes(&{ let ml_reference_value1558 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107])]))).borrow().clone(); ml_reference_value1558 }, &shared_constructor_field8.clone(), 0f64, false, |a, b| a == b)) {
                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 112, 104, 97, 115, 101])
                                                    } else {
                                                        {
                                                            let source_ml_source2_s28206 = if (shared_constructor_field8 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) {
                                                                if (((match (match state.clone() {
                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28209, _) => {
                                                                        ((source_ml_source2_data_property_value28209.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match (match state.clone() {
                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28212, _) => {
                                                                        ((source_ml_source2_data_property_value28212.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                    }
                                                                }).clone().expect("a proven present value") {
                                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value28213, _) => {
                                                                        source_ml_source2_data_property_value28213.clone()
                                                                    }
                                                                }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28214, _) => {
                                                                        ((source_ml_source2_data_property_value28214.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                    }
                                                                }).clone().expect("a proven present value") {
                                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value28215, _, _, _) => {
                                                                        let source_ml_source2_data_property_value28215 = *source_ml_source2_data_property_value28215;
                                                                        source_ml_source2_data_property_value28215.clone()
                                                                    }
                                                                }, shared_constructor_field12.clone()))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28217, _) => {
                                                                        ((source_ml_source2_data_property_value28217.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                    }
                                                                }).clone().expect("a proven present value") {
                                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value28218, _, _) => {
                                                                        let source_ml_source2_data_property_value28218 = *source_ml_source2_data_property_value28218;
                                                                        source_ml_source2_data_property_value28218.clone()
                                                                    }
                                                                }, shared_constructor_field5.clone()))) {
                                                                    self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 109, 98, 101, 114, 45, 104, 105, 110, 116, 32, 115, 116, 97, 116, 101]))
                                                                } else {
                                                                    {
                                                                        let invalid = self::read_java_script_primitive_conversion_state_error(heap.clone(), (match state.clone() {
                                                                            self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28220, _) => {
                                                                                ((source_ml_source2_data_property_value28220.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value"));
                                                                        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245Return(invalid.clone())
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245Next
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if !(match (match state.clone() {
                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28221, _) => {
                                                                        ((source_ml_source2_data_property_value28221.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                    self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245Return(self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245Next
                                                                }
                                                            };
                                                            match source_ml_source2_s28206.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245Next => {
                                                                    if (self::ml_array::includes(&{ let ml_reference_value1559 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107])]))).borrow().clone(); ml_reference_value1559 }, &shared_constructor_field8.clone(), 0f64, false, |a, b| a == b) && ((shared_constructor_field7 < (0f64)) || (shared_constructor_field7 >= shared_constructor_field6))) {
                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 101, 108, 101, 109, 101, 110, 116, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 45, 114, 97, 110, 103, 101, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                                                                    } else {
                                                                        if (self::ml_array::includes(&{ let ml_reference_value1560 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107])]))).borrow().clone(); ml_reference_value1560 }, &shared_constructor_field8.clone(), 0f64, false, |a, b| a == b) && ({
                                                                            let source_ml_source2_s28231 = shared_constructor_field3.clone();
                                                                            match source_ml_source2_s28231.clone() {
                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }
                                                                        } || !(((match shared_constructor_field3.clone() {
                                                                            self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value28236) => {
                                                                                Some((source_ml_source2_data_property_value28236,))
                                                                            }
                                                                            _ => {
                                                                                None::<(bool,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0))) {
                                                                            self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 101, 100, 105, 99, 97, 116, 101])
                                                                        } else {
                                                                            {
                                                                                let source_ml_source2_s28281 = if !(match (match state.clone() {
                                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28282) => {
                                                                                        ((source_ml_source2_data_property_value28282.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                    if (((!(self::ml_array::includes(&{ let ml_reference_value1561 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107])]))).borrow().clone(); ml_reference_value1561 }, &shared_constructor_field8.clone(), 0f64, false, |a, b| a == b)) || {
                                                                                        let source_ml_source2_s28286 = match (match state.clone() {
                                                                                            self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28289) => {
                                                                                                ((source_ml_source2_data_property_value28289.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value") {
                                                                                            self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(source_ml_source2_data_property_value28290, _, _) => {
                                                                                                let source_ml_source2_data_property_value28290 = *source_ml_source2_data_property_value28290;
                                                                                                source_ml_source2_data_property_value28290.clone()
                                                                                            }
                                                                                        };
                                                                                        match source_ml_source2_s28286.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                false
                                                                                            }
                                                                                            _ => {
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    }) || !(((match match (match state.clone() {
                                                                                        self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28291) => {
                                                                                            ((source_ml_source2_data_property_value28291.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value") {
                                                                                        self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(source_ml_source2_data_property_value28292, _, _) => {
                                                                                            let source_ml_source2_data_property_value28292 = *source_ml_source2_data_property_value28292;
                                                                                            source_ml_source2_data_property_value28292.clone()
                                                                                        }
                                                                                    } {
                                                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value28293) => {
                                                                                            Some((source_ml_source2_data_property_value28293,))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(bool,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0)) || (({ let ml_reference_array1562 = match (match state.clone() {
                                                                                        self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28296) => {
                                                                                            ((source_ml_source2_data_property_value28296.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value") {
                                                                                        self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(_, _, source_ml_source2_data_property_value28297) => {
                                                                                            source_ml_source2_data_property_value28297.clone()
                                                                                        }
                                                                                    }; let ml_reference_projection1562 = ml_reference_array1562.borrow().len() as f64; ml_reference_projection1562 }) > (4f64))) {
                                                                                        self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source1_temporary_values1243 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![match (match state.clone() {
                                                                                                self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28298) => {
                                                                                                    ((source_ml_source2_data_property_value28298.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value") {
                                                                                                self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(source_ml_source2_data_property_value28299, _, _) => {
                                                                                                    let source_ml_source2_data_property_value28299 = *source_ml_source2_data_property_value28299;
                                                                                                    source_ml_source2_data_property_value28299.clone()
                                                                                                }
                                                                                            }, match (match state.clone() {
                                                                                                self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28300) => {
                                                                                                    ((source_ml_source2_data_property_value28300.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value") {
                                                                                                self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(_, source_ml_source2_data_property_value28301, _) => {
                                                                                                    let source_ml_source2_data_property_value28301 = *source_ml_source2_data_property_value28301;
                                                                                                    source_ml_source2_data_property_value28301.clone()
                                                                                                }
                                                                                            }], { let ml_reference_value1563 = (match (match state.clone() {
                                                                                                self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28302) => {
                                                                                                    ((source_ml_source2_data_property_value28302.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value") {
                                                                                                self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(_, _, source_ml_source2_data_property_value28303) => {
                                                                                                    source_ml_source2_data_property_value28303.clone()
                                                                                                }
                                                                                            }).borrow().clone(); ml_reference_value1563 })));
                                                                                            {
                                                                                                let source_ml_source1_temporary_index1243 = self::ml::Big::from_i128(0);
                                                                                                {
                                                                                                    let source_ml_source2_s28304 = self::source_ml_source1_temporary_read_java_script_array_callback_state_error_loop1247(heap.clone(), source_ml_source1_temporary_values1243.clone(), source_ml_source1_temporary_index1243.clone());
                                                                                                    match source_ml_source2_s28304.clone() {
                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1247Result::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1247Done => {
                                                                                                            if (shared_constructor_field8 == (self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107]))) {
                                                                                                                {
                                                                                                                    let reduction = self::test_java_script_array_callback_reduction(&(shared_constructor_field1));
                                                                                                                    {
                                                                                                                        let expected = if reduction {
                                                                                                                            std::rc::Rc::new(std::cell::RefCell::new(vec![shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::JavaScriptSemanticValue::Number(shared_constructor_field7), shared_constructor_field2.clone()]))
                                                                                                                        } else {
                                                                                                                            std::rc::Rc::new(std::cell::RefCell::new(vec![shared_constructor_field12.clone(), self::JavaScriptSemanticValue::Number(shared_constructor_field7), shared_constructor_field2.clone()]))
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let receiver = if reduction {
                                                                                                                                self::JavaScriptSemanticValue::Undefined
                                                                                                                            } else {
                                                                                                                                shared_constructor_field4.clone()
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let args = match (match state.clone() {
                                                                                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28316) => {
                                                                                                                                        ((source_ml_source2_data_property_value28316.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(_, _, source_ml_source2_data_property_value28317) => {
                                                                                                                                        source_ml_source2_data_property_value28317.clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                if (((!(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28318) => {
                                                                                                                                        ((source_ml_source2_data_property_value28318.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(source_ml_source2_data_property_value28319, _, _) => {
                                                                                                                                        let source_ml_source2_data_property_value28319 = *source_ml_source2_data_property_value28319;
                                                                                                                                        source_ml_source2_data_property_value28319.clone()
                                                                                                                                    }
                                                                                                                                }, shared_constructor_field3.clone())) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28321) => {
                                                                                                                                        ((source_ml_source2_data_property_value28321.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(_, source_ml_source2_data_property_value28322, _) => {
                                                                                                                                        let source_ml_source2_data_property_value28322 = *source_ml_source2_data_property_value28322;
                                                                                                                                        source_ml_source2_data_property_value28322.clone()
                                                                                                                                    }
                                                                                                                                }, receiver.clone()))) || (({ let ml_reference_array1564 = args.clone(); let ml_reference_projection1564 = ml_reference_array1564.borrow().len() as f64; ml_reference_projection1564 }) != ({ let ml_reference_array1565 = expected.clone(); let ml_reference_projection1565 = ml_reference_array1565.borrow().len() as f64; ml_reference_projection1565 }))) || (reduction && !shared_constructor_field10)) {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 112, 114, 101, 100, 105, 99, 97, 116, 101, 47, 114, 101, 100, 117, 99, 101, 114, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 97, 110, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 99, 111, 110, 118, 101, 110, 116, 105, 111, 110]))
                                                                                                                                } else {
                                                                                                                                    {
                                                                                                                                        let at = 0f64;
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_s28324 = self::source_ml_source1_temporary_read_java_script_array_callback_state_error_loop1248(expected.clone(), args.clone(), at);
                                                                                                                                            match source_ml_source2_s28324.clone() {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1248Result::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1248Done => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246Next
                                                                                                                                                }
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1248Result::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1248Return(source_ml_source1_temporary_result1248) => {
                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246Return(source_ml_source1_temporary_result1248.clone())
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
                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246Next
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1247Result::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1247Return(source_ml_source1_temporary_result1248_2) => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246Return(source_ml_source1_temporary_result1248_2.clone())
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    if self::ml_array::includes(&{ let ml_reference_value1566 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107])]))).borrow().clone(); ml_reference_value1566 }, &shared_constructor_field8.clone(), 0f64, false, |a, b| a == b) {
                                                                                        self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 103, 101, 116, 116, 101, 114, 47, 99, 97, 108, 108, 98, 97, 99, 107, 32, 112, 104, 97, 115, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108]))
                                                                                    } else {
                                                                                        self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246Next
                                                                                    }
                                                                                };
                                                                                match source_ml_source2_s28281.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246Next => {
                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1246Return(source_ml_source1_temporary_result1246) => {
                                                                                        source_ml_source1_temporary_result1246.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorJoin1245Return(source_ml_source1_temporary_result1248_3) => {
                                                                    source_ml_source1_temporary_result1248_3.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1244Result::MlSource1TemporaryReadJavaScriptArrayCallbackStateErrorLoop1244Return(source_ml_source1_temporary_result1248_4) => {
                                            source_ml_source1_temporary_result1248_4.clone()
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
pub fn request_java_script_array_callback_call(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCallbackState,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptArrayCallbackTransition {
    match state.clone() {
        self::JavaScriptArrayCallbackState::ArrayCallbackState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            shared_constructor_field13,
            shared_constructor_field14,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            {
                let call = self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(
                    Box::new(callee.clone()),
                    Box::new(receiver.clone()),
                    arguments_.clone(),
                );
                self::JavaScriptArrayCallbackTransition::ArrayCallbackCall(
                    Box::new(heap.clone()),
                    Box::new({
                        let source_ml_source2_nominal_field28327 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_nominal_field28329 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_nominal_field28331 =
                                    shared_constructor_field3.clone();
                                {
                                    let source_ml_source2_nominal_field28333 =
                                        shared_constructor_field4.clone();
                                    {
                                        let source_ml_source2_nominal_field28335 =
                                            shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_nominal_field28337 =
                                                shared_constructor_field6;
                                            {
                                                let source_ml_source2_nominal_field28339 =
                                                    shared_constructor_field7;
                                                {
                                                    let source_ml_source2_nominal_field28341 =
                                                        shared_constructor_field8.clone();
                                                    {
                                                        let source_ml_source2_nominal_field28343 =
                                                            shared_constructor_field9;
                                                        {
                                                            let source_ml_source2_nominal_field28345 =
                                                                shared_constructor_field10;
                                                            {
                                                                let source_ml_source2_nominal_field28347 =
                                                                    shared_constructor_field11
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field28349 =
                                                                        shared_constructor_field12
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field28351 = shared_constructor_field13.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field28353 = shared_constructor_field14.clone();
                                                                            self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28327.clone(), Box::new(source_ml_source2_nominal_field28329.clone()), Box::new(source_ml_source2_nominal_field28331.clone()), Box::new(source_ml_source2_nominal_field28333.clone()), Box::new(source_ml_source2_nominal_field28335.clone()), source_ml_source2_nominal_field28337, source_ml_source2_nominal_field28339, source_ml_source2_nominal_field28341.clone(), source_ml_source2_nominal_field28343, source_ml_source2_nominal_field28345, Box::new(source_ml_source2_nominal_field28347.clone()), Box::new(source_ml_source2_nominal_field28349.clone()), source_ml_source2_nominal_field28351.clone(), Some((Some(call.clone()),)))
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
                    }),
                    Box::new(callee.clone()),
                    Box::new(receiver.clone()),
                    arguments_.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_array_callback_length(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCallbackState,
    transition: self::JavaScriptScalarConversionTransition,
) -> self::JavaScriptArrayCallbackTransition {
    {
        let source_ml_source1_temporary_binding1249 = state.clone();
        {
            let prior_conversion = match source_ml_source1_temporary_binding1249.clone() {
                self::JavaScriptArrayCallbackState::ArrayCallbackState(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value28356,
                    _,
                ) => ((source_ml_source2_data_property_value28356
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,)))
                .0)
                    .clone(),
            };
            {
                let prior_call = match source_ml_source1_temporary_binding1249.clone() {
                    self::JavaScriptArrayCallbackState::ArrayCallbackState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value28357,
                    ) => ((source_ml_source2_data_property_value28357
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,)))
                    .0)
                        .clone(),
                };
                {
                    let base = match source_ml_source1_temporary_binding1249.clone() {
                        self::JavaScriptArrayCallbackState::ArrayCallbackState(
                            source_ml_source2_nominal_rest_field28358,
                            source_ml_source2_nominal_rest_field28359,
                            source_ml_source2_nominal_rest_field28360,
                            source_ml_source2_nominal_rest_field28361,
                            source_ml_source2_nominal_rest_field28362,
                            source_ml_source2_nominal_rest_field28363,
                            source_ml_source2_nominal_rest_field28364,
                            source_ml_source2_nominal_rest_field28365,
                            source_ml_source2_nominal_rest_field28366,
                            source_ml_source2_nominal_rest_field28367,
                            source_ml_source2_nominal_rest_field28368,
                            source_ml_source2_nominal_rest_field28369,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_nominal_rest_field28359 =
                                *source_ml_source2_nominal_rest_field28359;
                            let source_ml_source2_nominal_rest_field28360 =
                                *source_ml_source2_nominal_rest_field28360;
                            let source_ml_source2_nominal_rest_field28361 =
                                *source_ml_source2_nominal_rest_field28361;
                            let source_ml_source2_nominal_rest_field28362 =
                                *source_ml_source2_nominal_rest_field28362;
                            let source_ml_source2_nominal_rest_field28368 =
                                *source_ml_source2_nominal_rest_field28368;
                            let source_ml_source2_nominal_rest_field28369 =
                                *source_ml_source2_nominal_rest_field28369;
                            self::JavaScriptArrayCallbackState::ArrayCallbackState(
                                source_ml_source2_nominal_rest_field28358.clone(),
                                Box::new(source_ml_source2_nominal_rest_field28359.clone()),
                                Box::new(source_ml_source2_nominal_rest_field28360.clone()),
                                Box::new(source_ml_source2_nominal_rest_field28361.clone()),
                                Box::new(source_ml_source2_nominal_rest_field28362.clone()),
                                source_ml_source2_nominal_rest_field28363,
                                source_ml_source2_nominal_rest_field28364,
                                source_ml_source2_nominal_rest_field28365.clone(),
                                source_ml_source2_nominal_rest_field28366,
                                source_ml_source2_nominal_rest_field28367,
                                Box::new(source_ml_source2_nominal_rest_field28368.clone()),
                                Box::new(source_ml_source2_nominal_rest_field28369.clone()),
                                None::<(Option<self::JavaScriptPrimitiveConversionState>,)>,
                                None::<(Option<self::JavaScriptArrayCallbackCall>,)>,
                            )
                        }
                    };
                    match base.clone() {
                        self::JavaScriptArrayCallbackState::ArrayCallbackState(
                            shared_constructor_field1,
                            shared_constructor_field2,
                            shared_constructor_field3,
                            shared_constructor_field4,
                            shared_constructor_field5,
                            shared_constructor_field6,
                            shared_constructor_field7,
                            shared_constructor_field8,
                            shared_constructor_field9,
                            shared_constructor_field10,
                            shared_constructor_field11,
                            shared_constructor_field12,
                            shared_constructor_field13,
                            shared_constructor_field14,
                        ) => {
                            let shared_constructor_field2 = *shared_constructor_field2;
                            let shared_constructor_field3 = *shared_constructor_field3;
                            let shared_constructor_field4 = *shared_constructor_field4;
                            let shared_constructor_field5 = *shared_constructor_field5;
                            let shared_constructor_field11 = *shared_constructor_field11;
                            let shared_constructor_field12 = *shared_constructor_field12;
                            match transition.clone() {
                                self::JavaScriptScalarConversionTransition::ScalarContinuation(state_2) => {
                                    let state_2 = *state_2;
                                    self::JavaScriptArrayCallbackTransition::ArrayCallbackContinue(Box::new(heap.clone()), Box::new({
                                        let source_ml_source2_nominal_field28371 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field28373 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field28375 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field28377 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field28379 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field28381 = shared_constructor_field6;
                                                            {
                                                                let source_ml_source2_nominal_field28383 = shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field28385 = shared_constructor_field8.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field28387 = shared_constructor_field9;
                                                                        {
                                                                            let source_ml_source2_nominal_field28389 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field28391 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field28393 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field28395 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field28397 = shared_constructor_field14.clone();
                                                                                            self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28371.clone(), Box::new(source_ml_source2_nominal_field28373.clone()), Box::new(source_ml_source2_nominal_field28375.clone()), Box::new(source_ml_source2_nominal_field28377.clone()), Box::new(source_ml_source2_nominal_field28379.clone()), source_ml_source2_nominal_field28381, source_ml_source2_nominal_field28383, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), source_ml_source2_nominal_field28387, source_ml_source2_nominal_field28389, Box::new(source_ml_source2_nominal_field28391.clone()), Box::new(source_ml_source2_nominal_field28393.clone()), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field28397.clone())
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
                                    }))
                                }
                                self::JavaScriptScalarConversionTransition::ScalarCall(state_3, callee, receiver, arguments) => {
                                    let state_3 = *state_3;
                                    let callee = *callee;
                                    let receiver = *receiver;
                                    self::request_java_script_array_callback_call(heap.clone(), {
                                        let source_ml_source2_nominal_field28402 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field28404 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field28406 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field28408 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field28410 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field28412 = shared_constructor_field6;
                                                            {
                                                                let source_ml_source2_nominal_field28414 = shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field28416 = shared_constructor_field8.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field28418 = shared_constructor_field9;
                                                                        {
                                                                            let source_ml_source2_nominal_field28420 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field28422 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field28424 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field28426 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field28428 = shared_constructor_field14.clone();
                                                                                            self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28402.clone(), Box::new(source_ml_source2_nominal_field28404.clone()), Box::new(source_ml_source2_nominal_field28406.clone()), Box::new(source_ml_source2_nominal_field28408.clone()), Box::new(source_ml_source2_nominal_field28410.clone()), source_ml_source2_nominal_field28412, source_ml_source2_nominal_field28414, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), source_ml_source2_nominal_field28418, source_ml_source2_nominal_field28420, Box::new(source_ml_source2_nominal_field28422.clone()), Box::new(source_ml_source2_nominal_field28424.clone()), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field28428.clone())
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
                                    }, callee.clone(), receiver.clone(), arguments.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarCompleted(primitive) => {
                                    let primitive = *primitive;
                                    match primitive.clone() {
                                        self::JavaScriptSemanticValue::Number(value) => {
                                            {
                                                let length = self::read_java_script_array_like_length(value);
                                                if ({
                                                    let source_ml_source2_s28433 = match state.clone() {
                                                        self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, source_ml_source2_data_property_value28436, _, _, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value28436 = *source_ml_source2_data_property_value28436;
                                                            source_ml_source2_data_property_value28436.clone()
                                                        }
                                                    };
                                                    match source_ml_source2_s28433.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }
                                                } || !(((match match state.clone() {
                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, source_ml_source2_data_property_value28437, _, _, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value28437 = *source_ml_source2_data_property_value28437;
                                                        source_ml_source2_data_property_value28437.clone()
                                                    }
                                                } {
                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value28438) => {
                                                        Some((source_ml_source2_data_property_value28438,))
                                                    }
                                                    _ => {
                                                        None::<(bool,)>
                                                    }
                                                }).clone().expect("a proven present value")).0)) {
                                                    self::JavaScriptArrayCallbackTransition::ArrayCallbackError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101]))
                                                } else {
                                                    self::JavaScriptArrayCallbackTransition::ArrayCallbackContinue(Box::new(heap.clone()), Box::new({
                                                        let source_ml_source2_nominal_field28441 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_nominal_field28443 = shared_constructor_field2.clone();
                                                            {
                                                                let source_ml_source2_nominal_field28445 = shared_constructor_field3.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field28447 = shared_constructor_field4.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field28449 = shared_constructor_field5.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field28451 = shared_constructor_field6;
                                                                            {
                                                                                let source_ml_source2_nominal_field28453 = shared_constructor_field7;
                                                                                {
                                                                                    let source_ml_source2_nominal_field28455 = shared_constructor_field8.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field28457 = shared_constructor_field9;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field28459 = shared_constructor_field10;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field28461 = shared_constructor_field11.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field28463 = shared_constructor_field12.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field28465 = shared_constructor_field13.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field28467 = shared_constructor_field14.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override28471 = if self::test_java_script_array_callback_reverse(&(match state.clone() {
                                                                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_data_property_value28439, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value28439.clone()
                                                                                                                    }
                                                                                                                })) {
                                                                                                                    (length - 1f64)
                                                                                                                } else {
                                                                                                                    0f64
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override28472 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28441.clone(), Box::new(source_ml_source2_nominal_field28443.clone()), Box::new(source_ml_source2_nominal_field28445.clone()), Box::new(source_ml_source2_nominal_field28447.clone()), Box::new(source_ml_source2_nominal_field28449.clone()), length, source_ml_source2_nominal_override28471, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), source_ml_source2_nominal_field28457, source_ml_source2_nominal_field28459, Box::new(source_ml_source2_nominal_field28461.clone()), Box::new(source_ml_source2_nominal_override28472.clone()), source_ml_source2_nominal_field28465.clone(), source_ml_source2_nominal_field28467.clone())
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
                                                    }))
                                                }
                                            }
                                        }
                                        _ => {
                                            self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 76, 101, 110, 103, 116, 104, 79, 102, 65, 114, 114, 97, 121, 76, 105, 107, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                                        }
                                    }
                                }
                                self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                                    let value_2 = *value_2;
                                    self::JavaScriptArrayCallbackTransition::ArrayCallbackThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
                                }
                                self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class, reason) => {
                                    self::JavaScriptArrayCallbackTransition::ArrayCallbackError(Box::new(heap.clone()), error_class.clone(), reason.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                                    self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(Box::new(heap.clone()), reason_2.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarResourceLimit(value_3, reason_3) => {
                                    let value_3 = *value_3;
                                    self::JavaScriptArrayCallbackTransition::ArrayCallbackResourceLimit(Box::new(heap.clone()), Box::new({
                                        let source_ml_source2_nominal_field28474 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field28476 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field28478 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field28480 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field28482 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field28484 = shared_constructor_field6;
                                                            {
                                                                let source_ml_source2_nominal_field28486 = shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field28488 = shared_constructor_field8.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field28490 = shared_constructor_field9;
                                                                        {
                                                                            let source_ml_source2_nominal_field28492 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field28494 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field28496 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field28498 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field28500 = shared_constructor_field14.clone();
                                                                                            self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28474.clone(), Box::new(source_ml_source2_nominal_field28476.clone()), Box::new(source_ml_source2_nominal_field28478.clone()), Box::new(source_ml_source2_nominal_field28480.clone()), Box::new(source_ml_source2_nominal_field28482.clone()), source_ml_source2_nominal_field28484, source_ml_source2_nominal_field28486, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101]), source_ml_source2_nominal_field28490, source_ml_source2_nominal_field28492, Box::new(source_ml_source2_nominal_field28494.clone()), Box::new(value_3.clone()), source_ml_source2_nominal_field28498.clone(), source_ml_source2_nominal_field28500.clone())
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
                                    }), reason_3.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
