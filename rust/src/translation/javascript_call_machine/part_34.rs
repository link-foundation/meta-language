// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesCall;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptArrayTransformCall;
use super::JavaScriptArrayTransformState;
use super::JavaScriptArrayTransformTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234;
use super::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235;
use super::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236;
use super::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237;
use super::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239;
use super::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop233Result;
use super::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Result;
use super::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop240Result;
use super::classify_java_script_array_species_constructor;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_species_directory_error;
use super::read_java_script_array_species_state_error;
use super::read_java_script_known_array_constructor_fact;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::source_ml_source1_temporary_read_java_script_array_transform_state_error_loop233;
use super::source_ml_source1_temporary_read_java_script_array_transform_state_error_loop238;
use super::source_ml_source1_temporary_read_java_script_array_transform_state_error_loop240;
use super::test_java_script_array_transform_operation;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_transform_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArrayTransformState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptArrayTransformState::ArrayTransformState(
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
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            {
                let error = self::read_java_script_array_species_directory_error(
                    heap.clone(),
                    directory.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    if ((!(self::test_java_script_array_transform_operation(
                        &(shared_constructor_field1),
                    )) || {
                        let source_ml_source2_s4361 = shared_constructor_field2.clone();
                        match source_ml_source2_s4361.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        }
                    }) || {
                        let source_ml_source2_s4366 = shared_constructor_field5.clone();
                        match source_ml_source2_s4366.clone() {
                            self::JavaScriptSemanticValue::Symbol(_) => false,
                            _ => true,
                        }
                    }) {
                        self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109,
                            97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97,
                            116, 105, 111, 110, 44, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101,
                            99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 99, 111, 110, 118,
                            101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108,
                        ])
                    } else {
                        {
                            let source_ml_source1_temporary_values229 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    shared_constructor_field2.clone(),
                                    shared_constructor_field3.clone(),
                                    shared_constructor_field4.clone(),
                                    shared_constructor_field5.clone(),
                                    shared_constructor_field9.clone(),
                                    shared_constructor_field10.clone(),
                                    shared_constructor_field11.clone(),
                                    shared_constructor_field12.clone(),
                                ]));
                            {
                                let source_ml_source1_temporary_index229 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s4377 = self::source_ml_source1_temporary_read_java_script_array_transform_state_error_loop233(heap.clone(), source_ml_source1_temporary_values229.clone(), source_ml_source1_temporary_index229.clone());
                                    match source_ml_source2_s4377.clone() {
                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop233Result::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop233Done => {
                                            if ((((((((((!(self::ml_math::is_safe_integer(shared_constructor_field6)) || (shared_constructor_field6 < (0f64))) || {
                                                let source_ml_source2_same_value_operand4382 = shared_constructor_field6;
                                                {
                                                    let source_ml_source2_same_value_operand4383 = (-0f64);
                                                    (((source_ml_source2_same_value_operand4382 == source_ml_source2_same_value_operand4383) && ((source_ml_source2_same_value_operand4382 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand4382)) == ((1f64 / source_ml_source2_same_value_operand4383))))) || ((source_ml_source2_same_value_operand4382 != source_ml_source2_same_value_operand4382) && (source_ml_source2_same_value_operand4383 != source_ml_source2_same_value_operand4383)))
                                                }
                                            }) || !(self::ml_math::is_safe_integer(shared_constructor_field7))) || (shared_constructor_field7 < (0f64))) || {
                                                let source_ml_source2_same_value_operand4388 = shared_constructor_field7;
                                                {
                                                    let source_ml_source2_same_value_operand4389 = (-0f64);
                                                    (((source_ml_source2_same_value_operand4388 == source_ml_source2_same_value_operand4389) && ((source_ml_source2_same_value_operand4388 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand4388)) == ((1f64 / source_ml_source2_same_value_operand4389))))) || ((source_ml_source2_same_value_operand4388 != source_ml_source2_same_value_operand4388) && (source_ml_source2_same_value_operand4389 != source_ml_source2_same_value_operand4389)))
                                                }
                                            }) || (shared_constructor_field7 > shared_constructor_field6)) || !(self::ml_math::is_safe_integer(shared_constructor_field8))) || (shared_constructor_field8 < (0f64))) || {
                                                let source_ml_source2_same_value_operand4398 = shared_constructor_field8;
                                                {
                                                    let source_ml_source2_same_value_operand4399 = (-0f64);
                                                    (((source_ml_source2_same_value_operand4398 == source_ml_source2_same_value_operand4399) && ((source_ml_source2_same_value_operand4398 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand4398)) == ((1f64 / source_ml_source2_same_value_operand4399))))) || ((source_ml_source2_same_value_operand4398 != source_ml_source2_same_value_operand4398) && (source_ml_source2_same_value_operand4399 != source_ml_source2_same_value_operand4399)))
                                                }
                                            }) || (shared_constructor_field8 > shared_constructor_field7)) {
                                                self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 108, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 101, 120, 97, 99, 116, 32, 105, 110, 112, 117, 116, 47, 111, 117, 116, 112, 117, 116, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115])
                                            } else {
                                                if !(self::ml_array::includes(&{ let ml_reference_value408 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101]), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])]))).borrow().clone(); ml_reference_value408 }, &shared_constructor_field13.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 103, 111, 114, 105, 116, 104, 109, 32, 112, 104, 97, 115, 101])
                                                } else {
                                                    {
                                                        let source_ml_source2_s4420 = if (shared_constructor_field13 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) {
                                                            if (((match (match state.clone() {
                                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4423, _, _, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value4423.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match (match state.clone() {
                                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4426, _, _, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value4426.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value") {
                                                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value4427, _) => {
                                                                    source_ml_source2_data_property_value4427.clone()
                                                                }
                                                            }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4428, _, _, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value4428.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value") {
                                                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value4429, _, _, _) => {
                                                                    let source_ml_source2_data_property_value4429 = *source_ml_source2_data_property_value4429;
                                                                    source_ml_source2_data_property_value4429.clone()
                                                                }
                                                            }, shared_constructor_field12.clone()))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4431, _, _, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value4431.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value") {
                                                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value4432, _, _) => {
                                                                    let source_ml_source2_data_property_value4432 = *source_ml_source2_data_property_value4432;
                                                                    source_ml_source2_data_property_value4432.clone()
                                                                }
                                                            }, shared_constructor_field5.clone()))) {
                                                                self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 44, 32, 104, 105, 110, 116, 32, 97, 110, 100, 32, 83, 121, 109, 98, 111, 108]))
                                                            } else {
                                                                {
                                                                    let invalid = self::read_java_script_primitive_conversion_state_error(heap.clone(), (match state.clone() {
                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4434, _, _, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value4434.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value"));
                                                                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234Return(invalid.clone())
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234Next
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if !(match (match state.clone() {
                                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4435, _, _, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value4435.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234Return(self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 32, 115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234Next
                                                            }
                                                        };
                                                        match source_ml_source2_s4420.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234Next => {
                                                                if (!(self::ml_array::includes(&{ let ml_reference_value409 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101])]))).borrow().clone(); ml_reference_value409 }, &shared_constructor_field13.clone(), 0f64, false, |a, b| a == b)) && ({
                                                                    let source_ml_source2_s4438 = shared_constructor_field3.clone();
                                                                    match source_ml_source2_s4438.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }
                                                                } || !(((match shared_constructor_field3.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value4443) => {
                                                                        Some((source_ml_source2_data_property_value4443,))
                                                                    }
                                                                    _ => {
                                                                        None::<(bool,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0))) {
                                                                    self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 99, 97, 108, 108, 98, 97, 99, 107])
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_s4459 = if !(match (match state.clone() {
                                                                            self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4460, _, _, _, _) => {
                                                                                ((source_ml_source2_data_property_value4460.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                            }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                            {
                                                                                let invalid_2 = self::read_java_script_array_species_state_error(heap.clone(), directory.clone(), (match state.clone() {
                                                                                    self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4461, _, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value4461.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value"));
                                                                                if (invalid_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235Return(invalid_2.clone())
                                                                                } else {
                                                                                    if (!(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4462, _, _, _, _) => {
                                                                                            ((source_ml_source2_data_property_value4462.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value") {
                                                                                        self::JavaScriptArraySpeciesState::ArraySpeciesState(source_ml_source2_data_property_value4463, _, _, _, _, _, _, _, _) => {
                                                                                            let source_ml_source2_data_property_value4463 = *source_ml_source2_data_property_value4463;
                                                                                            source_ml_source2_data_property_value4463.clone()
                                                                                        }
                                                                                    }, shared_constructor_field2.clone())) || ((match (match state.clone() {
                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4470, _, _, _, _) => {
                                                                                            ((source_ml_source2_data_property_value4470.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value") {
                                                                                        self::JavaScriptArraySpeciesState::ArraySpeciesState(_, source_ml_source2_data_property_value4471, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value4471
                                                                                        }
                                                                                    }) != (if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 109, 97, 112]))) {
                                                                                        shared_constructor_field6
                                                                                    } else {
                                                                                        0f64
                                                                                    }))) {
                                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 97, 110, 100, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 101, 99, 105, 101, 115, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 108, 101, 110, 103, 116, 104]))
                                                                                    } else {
                                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235Next
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235Next
                                                                        };
                                                                        match source_ml_source2_s4459.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235Next => {
                                                                                if (self::ml_array::includes(&{ let ml_reference_value410 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])]))).borrow().clone(); ml_reference_value410 }, &shared_constructor_field13.clone(), 0f64, false, |a, b| a == b) && match (match state.clone() {
                                                                                    self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4476, _, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value4476.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 114, 101, 115, 117, 108, 116, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                                                                                } else {
                                                                                    if (self::ml_array::includes(&{ let ml_reference_value411 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])]))).borrow().clone(); ml_reference_value411 }, &shared_constructor_field13.clone(), 0f64, false, |a, b| a == b) && {
                                                                                        let source_ml_source2_s4479 = shared_constructor_field9.clone();
                                                                                        match source_ml_source2_s4479.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                false
                                                                                            }
                                                                                            _ => {
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    }) {
                                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 103, 101, 110, 117, 105, 110, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 101, 100, 32, 114, 101, 115, 117, 108, 116, 32, 79, 98, 106, 101, 99, 116])
                                                                                    } else {
                                                                                        if (self::ml_array::includes(&{ let ml_reference_value412 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])]))).borrow().clone(); ml_reference_value412 }, &shared_constructor_field13.clone(), 0f64, false, |a, b| a == b) && (shared_constructor_field7 >= shared_constructor_field6)) {
                                                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 119, 111, 114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 45, 114, 97, 110, 103, 101, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 100, 101, 120])
                                                                                        } else {
                                                                                            if ((shared_constructor_field13 == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]))) && (((match (match state.clone() {
                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4490, _, _, _) => {
                                                                                                    ((source_ml_source2_data_property_value4490.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                }
                                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || {
                                                                                                let source_ml_source2_s4492 = (match state.clone() {
                                                                                                    self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4495, _, _, _) => {
                                                                                                        ((source_ml_source2_data_property_value4495.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                    }
                                                                                                }).clone().expect("a proven present value");
                                                                                                match source_ml_source2_s4492.clone() {
                                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                        false
                                                                                                    }
                                                                                                    _ => {
                                                                                                        true
                                                                                                    }
                                                                                                }
                                                                                            }) || !(self::test_java_script_semantic_same_value((match state.clone() {
                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4496, _, _, _) => {
                                                                                                    ((source_ml_source2_data_property_value4496.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value"), match directory.clone() {
                                                                                                self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(_, _, source_ml_source2_data_property_value4497, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value4497 = *source_ml_source2_data_property_value4497;
                                                                                                    source_ml_source2_data_property_value4497.clone()
                                                                                                }
                                                                                            }))) || ((self::read_java_script_ordinary_value_error(heap.clone(), (match state.clone() {
                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4499, _, _, _) => {
                                                                                                    ((source_ml_source2_data_property_value4499.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![]))))) {
                                                                                                self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 114, 101, 115, 117, 108, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                                                            } else {
                                                                                                {
                                                                                                    let source_ml_source2_s4538 = if (shared_constructor_field13 == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]))) {
                                                                                                        {
                                                                                                            let constructor = if {
                                                                                                                let source_ml_source2_own_property_operands4541 = (state.clone(), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]));
                                                                                                                {
                                                                                                                    let source_ml_source2_own_property_key4542 = ((source_ml_source2_own_property_operands4541.clone()).1,);
                                                                                                                    match (source_ml_source2_own_property_operands4541.clone()).0 {
                                                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_owned_constructor_field4543, source_ml_source2_owned_constructor_field4544, source_ml_source2_owned_constructor_field4545, source_ml_source2_owned_constructor_field4546, source_ml_source2_owned_constructor_field4547, source_ml_source2_owned_constructor_field4548) => {
                                                                                                                            ((((((((((((((((((((((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![36]))) || (((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 116, 105, 111, 110])))) || (((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])))) || (((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107])))) || (((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![116, 104, 105, 115, 65, 114, 103, 117, 109, 101, 110, 116])))) || (((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 75, 101, 121])))) || (((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))) || (((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116])))) || (((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])))) || (((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))) || (((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116])))) || (((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![112, 104, 97, 115, 101])))) || ((((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && match source_ml_source2_owned_constructor_field4543.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && match source_ml_source2_owned_constructor_field4544.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 80, 114, 111, 116, 111, 116, 121, 112, 101]))) && match source_ml_source2_owned_constructor_field4545.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) && match source_ml_source2_owned_constructor_field4546.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 70, 97, 99, 116]))) && match source_ml_source2_owned_constructor_field4547.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key4542.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) && match source_ml_source2_owned_constructor_field4548.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }))
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            } {
                                                                                                                match state.clone() {
                                                                                                                    self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4549, _, _) => {
                                                                                                                        ((source_ml_source2_data_property_value4549.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                                    }
                                                                                                                }
                                                                                                            } else {
                                                                                                                None::<self::JavaScriptSemanticValue>
                                                                                                            };
                                                                                                            {
                                                                                                                let selected_fact = match state.clone() {
                                                                                                                    self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4550, _) => {
                                                                                                                        ((source_ml_source2_data_property_value4550.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConstructorFact>,))).0).clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                if (((((match (constructor.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match (selected_fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match constructor.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                        false
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        true
                                                                                                                    }
                                                                                                                }) || (((((match constructor.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value4555, _) => {
                                                                                                                        Some((source_ml_source2_data_property_value4555.clone(),))
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        None::<(self::ml::Big,)>
                                                                                                                    }
                                                                                                                }).clone().expect("a proven present value")).0).clone()) != (match selected_fact.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(source_ml_source2_data_property_value4556, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value4556.clone()
                                                                                                                    }
                                                                                                                }))) || !(match selected_fact.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value4557, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value4557
                                                                                                                    }
                                                                                                                })) || !(self::test_java_script_semantic_same_value(constructor.clone().expect("a proven present value"), ({
                                                                                                                    let source_ml_source2_optional_chain_receiver4559 = (match state.clone() {
                                                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4558, _, _, _, _) => {
                                                                                                                            ((source_ml_source2_data_property_value4558.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                        }
                                                                                                                    },);
                                                                                                                    if match (source_ml_source2_optional_chain_receiver4559.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                        Some({
                                                                                                                            let source_ml_source1_temporary_optional_chain230 = ((source_ml_source2_optional_chain_receiver4559.clone()).0).clone().expect("a proven present value");
                                                                                                                            match source_ml_source1_temporary_optional_chain230.clone() {
                                                                                                                                self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, source_ml_source2_data_property_value4560, _, _) => {
                                                                                                                                    let source_ml_source2_data_property_value4560 = *source_ml_source2_data_property_value4560;
                                                                                                                                    source_ml_source2_data_property_value4560.clone()
                                                                                                                                }
                                                                                                                            }
                                                                                                                        })
                                                                                                                    } else {
                                                                                                                        None::<self::JavaScriptSemanticValue>
                                                                                                                    }
                                                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 117, 115, 116, 111, 109, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 102, 97, 99, 116]))
                                                                                                                } else {
                                                                                                                    {
                                                                                                                        let classification = self::classify_java_script_array_species_constructor(heap.clone(), directory.clone(), constructor.clone().expect("a proven present value"));
                                                                                                                        {
                                                                                                                            let fact = self::read_java_script_known_array_constructor_fact(classification.clone());
                                                                                                                            if ((((match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match fact.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value4563, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value4563
                                                                                                                                }
                                                                                                                            }) != (match selected_fact.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value4564, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value4564
                                                                                                                                }
                                                                                                                            }))) || ((match fact.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value4567, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value4567.clone()
                                                                                                                                }
                                                                                                                            }) != (match selected_fact.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value4568, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value4568.clone()
                                                                                                                                }
                                                                                                                            }))) || ((match fact.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value4571, _) => {
                                                                                                                                    source_ml_source2_data_property_value4571
                                                                                                                                }
                                                                                                                            }) != (match selected_fact.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value4572, _) => {
                                                                                                                                    source_ml_source2_data_property_value4572
                                                                                                                                }
                                                                                                                            }))) || {
                                                                                                                                let source_ml_source2_comparison_operand4575 = match fact.clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value4573) => {
                                                                                                                                        ((source_ml_source2_data_property_value4573.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_comparison_operand4576 = match selected_fact.clone().expect("a proven present value") {
                                                                                                                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value4574) => {
                                                                                                                                            ((source_ml_source2_data_property_value4574.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    !(match (source_ml_source2_comparison_operand4575.clone(), source_ml_source2_comparison_operand4576.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                                                                                                                                }
                                                                                                                            }) {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 117, 115, 116, 111, 109, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110]))
                                                                                                                            } else {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236Next
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236Next
                                                                                                    };
                                                                                                    match source_ml_source2_s4538.clone() {
                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236Next => {
                                                                                                            if (((shared_constructor_field13 == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) && (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 105, 108, 116, 101, 114])))) && !(self::test_java_script_semantic_same_value(shared_constructor_field11.clone(), shared_constructor_field10.clone()))) {
                                                                                                                self::ml_string::Text::from_units(vec![102, 105, 108, 116, 101, 114, 32, 119, 114, 105, 116, 101, 115, 32, 105, 116, 115, 32, 114, 101, 116, 97, 105, 110, 101, 100, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 101, 108, 101, 109, 101, 110, 116, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 105, 116, 115, 32, 112, 114, 101, 100, 105, 99, 97, 116, 101, 32, 114, 101, 115, 117, 108, 116])
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let source_ml_source2_s4664 = if !(match (match state.clone() {
                                                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4665) => {
                                                                                                                            ((source_ml_source2_data_property_value4665.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                                        }
                                                                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                                        if (((!(self::ml_array::includes(&{ let ml_reference_value413 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107])]))).borrow().clone(); ml_reference_value413 }, &shared_constructor_field13.clone(), 0f64, false, |a, b| a == b)) || {
                                                                                                                            let source_ml_source2_s4669 = match (match state.clone() {
                                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4672) => {
                                                                                                                                    ((source_ml_source2_data_property_value4672.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                                                }
                                                                                                                            }).clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(source_ml_source2_data_property_value4673, _, _) => {
                                                                                                                                    let source_ml_source2_data_property_value4673 = *source_ml_source2_data_property_value4673;
                                                                                                                                    source_ml_source2_data_property_value4673.clone()
                                                                                                                                }
                                                                                                                            };
                                                                                                                            match source_ml_source2_s4669.clone() {
                                                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                                    false
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                    true
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }) || !(((match match (match state.clone() {
                                                                                                                            self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4674) => {
                                                                                                                                ((source_ml_source2_data_property_value4674.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                                            }
                                                                                                                        }).clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(source_ml_source2_data_property_value4675, _, _) => {
                                                                                                                                let source_ml_source2_data_property_value4675 = *source_ml_source2_data_property_value4675;
                                                                                                                                source_ml_source2_data_property_value4675.clone()
                                                                                                                            }
                                                                                                                        } {
                                                                                                                            self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value4676) => {
                                                                                                                                Some((source_ml_source2_data_property_value4676,))
                                                                                                                            }
                                                                                                                            _ => {
                                                                                                                                None::<(bool,)>
                                                                                                                            }
                                                                                                                        }).clone().expect("a proven present value")).0)) || (({ let ml_reference_array414 = match (match state.clone() {
                                                                                                                            self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4679) => {
                                                                                                                                ((source_ml_source2_data_property_value4679.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                                            }
                                                                                                                        }).clone().expect("a proven present value") {
                                                                                                                            self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(_, _, source_ml_source2_data_property_value4680) => {
                                                                                                                                source_ml_source2_data_property_value4680.clone()
                                                                                                                            }
                                                                                                                        }; let ml_reference_projection414 = ml_reference_array414.borrow().len() as f64; ml_reference_projection414 }) > (3f64))) {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107]))
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let source_ml_source1_temporary_values231 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![match (match state.clone() {
                                                                                                                                    self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4681) => {
                                                                                                                                        ((source_ml_source2_data_property_value4681.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(source_ml_source2_data_property_value4682, _, _) => {
                                                                                                                                        let source_ml_source2_data_property_value4682 = *source_ml_source2_data_property_value4682;
                                                                                                                                        source_ml_source2_data_property_value4682.clone()
                                                                                                                                    }
                                                                                                                                }, match (match state.clone() {
                                                                                                                                    self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4683) => {
                                                                                                                                        ((source_ml_source2_data_property_value4683.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(_, source_ml_source2_data_property_value4684, _) => {
                                                                                                                                        let source_ml_source2_data_property_value4684 = *source_ml_source2_data_property_value4684;
                                                                                                                                        source_ml_source2_data_property_value4684.clone()
                                                                                                                                    }
                                                                                                                                }], { let ml_reference_value415 = (match (match state.clone() {
                                                                                                                                    self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4685) => {
                                                                                                                                        ((source_ml_source2_data_property_value4685.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(_, _, source_ml_source2_data_property_value4686) => {
                                                                                                                                        source_ml_source2_data_property_value4686.clone()
                                                                                                                                    }
                                                                                                                                }).borrow().clone(); ml_reference_value415 })));
                                                                                                                                {
                                                                                                                                    let source_ml_source1_temporary_index231 = self::ml::Big::from_i128(0);
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_s4687 = self::source_ml_source1_temporary_read_java_script_array_transform_state_error_loop238(heap.clone(), source_ml_source1_temporary_values231.clone(), source_ml_source1_temporary_index231.clone());
                                                                                                                                        match source_ml_source2_s4687.clone() {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Result::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Done => {
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_s4707 = if (shared_constructor_field13 == (self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107]))) {
                                                                                                                                                        {
                                                                                                                                                            let args = match (match state.clone() {
                                                                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4710) => {
                                                                                                                                                                    ((source_ml_source2_data_property_value4710.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                                                                                }
                                                                                                                                                            }).clone().expect("a proven present value") {
                                                                                                                                                                self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(_, _, source_ml_source2_data_property_value4711) => {
                                                                                                                                                                    source_ml_source2_data_property_value4711.clone()
                                                                                                                                                                }
                                                                                                                                                            };
                                                                                                                                                            if (((((!(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4712) => {
                                                                                                                                                                    ((source_ml_source2_data_property_value4712.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                                                                                }
                                                                                                                                                            }).clone().expect("a proven present value") {
                                                                                                                                                                self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(source_ml_source2_data_property_value4713, _, _) => {
                                                                                                                                                                    let source_ml_source2_data_property_value4713 = *source_ml_source2_data_property_value4713;
                                                                                                                                                                    source_ml_source2_data_property_value4713.clone()
                                                                                                                                                                }
                                                                                                                                                            }, shared_constructor_field3.clone())) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4715) => {
                                                                                                                                                                    ((source_ml_source2_data_property_value4715.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                                                                                }
                                                                                                                                                            }).clone().expect("a proven present value") {
                                                                                                                                                                self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(_, source_ml_source2_data_property_value4716, _) => {
                                                                                                                                                                    let source_ml_source2_data_property_value4716 = *source_ml_source2_data_property_value4716;
                                                                                                                                                                    source_ml_source2_data_property_value4716.clone()
                                                                                                                                                                }
                                                                                                                                                            }, shared_constructor_field4.clone()))) || (({ let ml_reference_array416 = args.clone(); let ml_reference_projection416 = ml_reference_array416.borrow().len() as f64; ml_reference_projection416 }) != (3f64))) || !(self::test_java_script_semantic_same_value(({
                                                                                                                                                                let source_ml_source2_array_read_index4719 = 0f64;
                                                                                                                                                                { let ml_reference_array417 = args.clone(); let ml_reference_position417 = source_ml_source2_array_read_index4719; let ml_reference_projection417 = self::ml_array::optional_at(&*ml_reference_array417.borrow(), self::ml_array::number_index(ml_reference_position417)); ml_reference_projection417 }
                                                                                                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field10.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                                                                                                                                let source_ml_source2_array_read_index4722 = 1f64;
                                                                                                                                                                { let ml_reference_array418 = args.clone(); let ml_reference_position418 = source_ml_source2_array_read_index4722; let ml_reference_projection418 = self::ml_array::optional_at(&*ml_reference_array418.borrow(), self::ml_array::number_index(ml_reference_position418)); ml_reference_projection418 }
                                                                                                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), self::JavaScriptSemanticValue::Number(shared_constructor_field7)))) || !(self::test_java_script_semantic_same_value(({
                                                                                                                                                                let source_ml_source2_array_read_index4725 = 2f64;
                                                                                                                                                                { let ml_reference_array419 = args.clone(); let ml_reference_position419 = source_ml_source2_array_read_index4725; let ml_reference_projection419 = self::ml_array::optional_at(&*ml_reference_array419.borrow(), self::ml_array::number_index(ml_reference_position419)); ml_reference_projection419 }
                                                                                                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2.clone()))) {
                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 101, 108, 101, 109, 101, 110, 116, 44, 32, 105, 110, 100, 101, 120, 44, 32, 115, 111, 117, 114, 99, 101, 32, 97, 110, 100, 32, 116, 104, 105, 115, 65, 114, 103]))
                                                                                                                                                            } else {
                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239Next
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239Next
                                                                                                                                                    };
                                                                                                                                                    match source_ml_source2_s4707.clone() {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239Next => {
                                                                                                                                                            if (shared_constructor_field13 == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) {
                                                                                                                                                                {
                                                                                                                                                                    let call = {
                                                                                                                                                                        let source_ml_source2_optional_chain_receiver4730 = (match state.clone() {
                                                                                                                                                                            self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4729, _, _, _, _) => {
                                                                                                                                                                                ((source_ml_source2_data_property_value4729.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                                                                            }
                                                                                                                                                                        },);
                                                                                                                                                                        match if match (source_ml_source2_optional_chain_receiver4730.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                                                            Some({
                                                                                                                                                                                let source_ml_source1_temporary_optional_chain232 = ((source_ml_source2_optional_chain_receiver4730.clone()).0).clone().expect("a proven present value");
                                                                                                                                                                                match source_ml_source1_temporary_optional_chain232.clone() {
                                                                                                                                                                                    self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value4731) => {
                                                                                                                                                                                        ((source_ml_source2_data_property_value4731.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,))).0).clone()
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            })
                                                                                                                                                                        } else {
                                                                                                                                                                            None::<self::JavaScriptArraySpeciesCall>
                                                                                                                                                                        } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                                                                                                                                    };
                                                                                                                                                                    if (((match (call.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(match call.clone().expect("a proven present value") {
                                                                                                                                                                        self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(source_ml_source2_data_property_value4732, _, _) => {
                                                                                                                                                                            let source_ml_source2_data_property_value4732 = *source_ml_source2_data_property_value4732;
                                                                                                                                                                            source_ml_source2_data_property_value4732.clone()
                                                                                                                                                                        }
                                                                                                                                                                    }, match (match state.clone() {
                                                                                                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4733) => {
                                                                                                                                                                            ((source_ml_source2_data_property_value4733.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                                                                                        }
                                                                                                                                                                    }).clone().expect("a proven present value") {
                                                                                                                                                                        self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(source_ml_source2_data_property_value4734, _, _) => {
                                                                                                                                                                            let source_ml_source2_data_property_value4734 = *source_ml_source2_data_property_value4734;
                                                                                                                                                                            source_ml_source2_data_property_value4734.clone()
                                                                                                                                                                        }
                                                                                                                                                                    }))) || !(self::test_java_script_semantic_same_value(match call.clone().expect("a proven present value") {
                                                                                                                                                                        self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(_, source_ml_source2_data_property_value4735, _) => {
                                                                                                                                                                            let source_ml_source2_data_property_value4735 = *source_ml_source2_data_property_value4735;
                                                                                                                                                                            source_ml_source2_data_property_value4735.clone()
                                                                                                                                                                        }
                                                                                                                                                                    }, match (match state.clone() {
                                                                                                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4736) => {
                                                                                                                                                                            ((source_ml_source2_data_property_value4736.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                                                                                        }
                                                                                                                                                                    }).clone().expect("a proven present value") {
                                                                                                                                                                        self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(_, source_ml_source2_data_property_value4737, _) => {
                                                                                                                                                                            let source_ml_source2_data_property_value4737 = *source_ml_source2_data_property_value4737;
                                                                                                                                                                            source_ml_source2_data_property_value4737.clone()
                                                                                                                                                                        }
                                                                                                                                                                    }))) || (({ let ml_reference_array420 = match call.clone().expect("a proven present value") {
                                                                                                                                                                        self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(_, _, source_ml_source2_data_property_value4741) => {
                                                                                                                                                                            source_ml_source2_data_property_value4741.clone()
                                                                                                                                                                        }
                                                                                                                                                                    }; let ml_reference_projection420 = ml_reference_array420.borrow().len() as f64; ml_reference_projection420 }) != ({ let ml_reference_array421 = match (match state.clone() {
                                                                                                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value4742) => {
                                                                                                                                                                            ((source_ml_source2_data_property_value4742.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                                                                                        }
                                                                                                                                                                    }).clone().expect("a proven present value") {
                                                                                                                                                                        self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(_, _, source_ml_source2_data_property_value4743) => {
                                                                                                                                                                            source_ml_source2_data_property_value4743.clone()
                                                                                                                                                                        }
                                                                                                                                                                    }; let ml_reference_projection421 = ml_reference_array421.borrow().len() as f64; ml_reference_projection421 }))) {
                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 115, 116, 101, 100, 32, 115, 101, 108, 101, 99, 116, 105, 111, 110]))
                                                                                                                                                                    } else {
                                                                                                                                                                        {
                                                                                                                                                                            let at = 0f64;
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_s4744 = self::source_ml_source1_temporary_read_java_script_array_transform_state_error_loop240(state.clone(), Some(call.clone().expect("a proven present value")), at);
                                                                                                                                                                                match source_ml_source2_s4744.clone() {
                                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop240Result::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop240Done => {
                                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237Next
                                                                                                                                                                                    }
                                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop240Result::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop240Return(source_ml_source1_temporary_result240) => {
                                                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237Return(source_ml_source1_temporary_result240.clone())
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            } else {
                                                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237Next
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin239Return(source_ml_source1_temporary_result240_2) => {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237Return(source_ml_source1_temporary_result240_2.clone())
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Result::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Return(source_ml_source1_temporary_result240_3) => {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237Return(source_ml_source1_temporary_result240_3.clone())
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        if self::ml_array::includes(&{ let ml_reference_value422 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107])]))).borrow().clone(); ml_reference_value422 }, &shared_constructor_field13.clone(), 0f64, false, |a, b| a == b) {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 103, 101, 116, 116, 101, 114, 47, 99, 97, 108, 108, 98, 97, 99, 107, 32, 112, 104, 97, 115, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108]))
                                                                                                                        } else {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237Next
                                                                                                                        }
                                                                                                                    };
                                                                                                                    match source_ml_source2_s4664.clone() {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237Next => {
                                                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                                                        }
                                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin237Return(source_ml_source1_temporary_result237) => {
                                                                                                                            source_ml_source1_temporary_result237.clone()
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236Return(source_ml_source1_temporary_result240_4) => {
                                                                                                            source_ml_source1_temporary_result240_4.clone()
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235Return(source_ml_source1_temporary_result240_5) => {
                                                                                source_ml_source1_temporary_result240_5.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234Return(source_ml_source1_temporary_result240_6) => {
                                                                source_ml_source1_temporary_result240_6.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop233Result::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop233Return(source_ml_source1_temporary_result240_7) => {
                                            source_ml_source1_temporary_result240_7.clone()
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
pub fn request_java_script_array_transform_call(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayTransformState,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptArrayTransformTransition {
    match state.clone() {
        self::JavaScriptArrayTransformState::ArrayTransformState(
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
            shared_constructor_field15,
            shared_constructor_field16,
            shared_constructor_field17,
            shared_constructor_field18,
            shared_constructor_field19,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            self::JavaScriptArrayTransformTransition::ArrayTransformCall(
                Box::new(heap.clone()),
                Box::new({
                    let source_ml_source2_nominal_field4747 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_nominal_field4749 = shared_constructor_field2.clone();
                        {
                            let source_ml_source2_nominal_field4751 =
                                shared_constructor_field3.clone();
                            {
                                let source_ml_source2_nominal_field4753 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_nominal_field4755 =
                                        shared_constructor_field5.clone();
                                    {
                                        let source_ml_source2_nominal_field4757 =
                                            shared_constructor_field6;
                                        {
                                            let source_ml_source2_nominal_field4759 =
                                                shared_constructor_field7;
                                            {
                                                let source_ml_source2_nominal_field4761 =
                                                    shared_constructor_field8;
                                                {
                                                    let source_ml_source2_nominal_field4763 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_nominal_field4765 =
                                                            shared_constructor_field10.clone();
                                                        {
                                                            let source_ml_source2_nominal_field4767 =
                                                                shared_constructor_field11.clone();
                                                            {
                                                                let source_ml_source2_nominal_field4769 =
                                                                    shared_constructor_field12
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field4771 =
                                                                        shared_constructor_field13
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field4773 = shared_constructor_field14.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field4775 = shared_constructor_field15.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field4777 = shared_constructor_field16.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field4779 = shared_constructor_field17.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field4781 = shared_constructor_field18.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field4783 = shared_constructor_field19.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override4785 = self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(Box::new(callee.clone()), Box::new(receiver.clone()), arguments_.clone());
                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field4747.clone(), Box::new(source_ml_source2_nominal_field4749.clone()), Box::new(source_ml_source2_nominal_field4751.clone()), Box::new(source_ml_source2_nominal_field4753.clone()), Box::new(source_ml_source2_nominal_field4755.clone()), source_ml_source2_nominal_field4757, source_ml_source2_nominal_field4759, source_ml_source2_nominal_field4761, Box::new(source_ml_source2_nominal_field4763.clone()), Box::new(source_ml_source2_nominal_field4765.clone()), Box::new(source_ml_source2_nominal_field4767.clone()), Box::new(source_ml_source2_nominal_field4769.clone()), source_ml_source2_nominal_field4771.clone(), source_ml_source2_nominal_field4773.clone(), source_ml_source2_nominal_field4775.clone(), source_ml_source2_nominal_field4777.clone(), source_ml_source2_nominal_field4779.clone(), source_ml_source2_nominal_field4781.clone(), Some((Some(source_ml_source2_nominal_override4785.clone()),)))
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
                }),
                Box::new(callee.clone()),
                Box::new(receiver.clone()),
                arguments_.clone(),
            )
        }
    }
}
