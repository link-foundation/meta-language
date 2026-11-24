// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySliceState;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350;
use super::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351;
use super::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352;
use super::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop348Result;
use super::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop349Result;
use super::classify_java_script_array_species_constructor;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_slice_child_state_error;
use super::read_java_script_array_species_directory_error;
use super::read_java_script_array_species_state_error;
use super::read_java_script_known_array_constructor_fact;
use super::read_java_script_primitive_conversion_state_error;
use super::source_ml_source1_temporary_read_java_script_array_slice_state_error_loop348;
use super::source_ml_source1_temporary_read_java_script_array_slice_state_error_loop349;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_slice_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArraySliceState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptArraySliceState::ArraySliceState(
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
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field13 = *shared_constructor_field13;
            let shared_constructor_field14 = *shared_constructor_field14;
            {
                let directory_error = self::read_java_script_array_species_directory_error(
                    heap.clone(),
                    directory.clone(),
                );
                if (directory_error != (self::ml_string::Text::from_units(vec![]))) {
                    directory_error.clone()
                } else {
                    if ((((match state.clone() {
                        self::JavaScriptArraySliceState::ArraySliceState(
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
                        ) => false,
                    } || {
                        let source_ml_source2_s10839 = shared_constructor_field1.clone();
                        match source_ml_source2_s10839.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        }
                    }) || {
                        let source_ml_source2_s10844 = shared_constructor_field5.clone();
                        match source_ml_source2_s10844.clone() {
                            self::JavaScriptSemanticValue::Symbol(_) => false,
                            _ => true,
                        }
                    }) || (({
                        let source_ml_source2_type_query_operand10850 = shared_constructor_field4;
                        self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                    }) != (self::ml_string::Text::from_units(vec![
                        98, 111, 111, 108, 101, 97, 110,
                    ]))))
                        || (!shared_constructor_field4 && {
                            let source_ml_source2_s10853 = shared_constructor_field3.clone();
                            match source_ml_source2_s10853.clone() {
                                self::JavaScriptSemanticValue::Undefined => false,
                                _ => true,
                            }
                        }))
                    {
                        self::ml_string::Text::from_units(vec![
                            115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116,
                            32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32, 97, 114, 103, 117,
                            109, 101, 110, 116, 32, 112, 114, 101, 115, 101, 110, 99, 101, 32, 97,
                            110, 100, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83,
                            121, 109, 98, 111, 108,
                        ])
                    } else {
                        {
                            let source_ml_source1_temporary_values345 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    shared_constructor_field1.clone(),
                                    shared_constructor_field2.clone(),
                                    shared_constructor_field3.clone(),
                                    shared_constructor_field5.clone(),
                                    shared_constructor_field12.clone(),
                                    shared_constructor_field13.clone(),
                                    shared_constructor_field14.clone(),
                                ]));
                            {
                                let source_ml_source1_temporary_index345 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s10862 = self::source_ml_source1_temporary_read_java_script_array_slice_state_error_loop348(heap.clone(), source_ml_source1_temporary_values345.clone(), source_ml_source1_temporary_index345.clone());
                                    match source_ml_source2_s10862.clone() {
                                        self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop348Result::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop348Done => {
                                            {
                                                let source_ml_source1_temporary_values346 = std::rc::Rc::new(std::cell::RefCell::new(vec![shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11]));
                                                {
                                                    let source_ml_source1_temporary_index346 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s10869 = self::source_ml_source1_temporary_read_java_script_array_slice_state_error_loop349(source_ml_source1_temporary_values346.clone(), source_ml_source1_temporary_index346.clone());
                                                        match source_ml_source2_s10869.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop349Result::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop349Done => {
                                                                if (((((shared_constructor_field7 > shared_constructor_field6) || (shared_constructor_field8 > shared_constructor_field6)) || (shared_constructor_field9 > shared_constructor_field6)) || (shared_constructor_field10 > shared_constructor_field6)) || (shared_constructor_field11 > shared_constructor_field9)) {
                                                                    self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 114, 101, 109, 97, 105, 110, 32, 119, 105, 116, 104, 105, 110, 32, 105, 116, 115, 32, 115, 97, 118, 101, 100, 32, 108, 101, 110, 103, 116, 104])
                                                                } else {
                                                                    if (!(self::ml_array::includes(&{ let ml_reference_value739 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101]), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104])]))).borrow().clone(); ml_reference_value739 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b)) || !(self::ml_array::includes(&{ let ml_reference_value740 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116]), self::ml_string::Text::from_units(vec![101, 110, 100])]))).borrow().clone(); ml_reference_value740 }, &shared_constructor_field16.clone(), 0f64, false, |a, b| a == b))) {
                                                                        self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 104, 97, 115, 101, 32, 97, 110, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 111, 112, 101, 114, 97, 110, 100])
                                                                    } else {
                                                                        {
                                                                            let source_ml_source2_s10907 = if (shared_constructor_field15 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) {
                                                                                if (((match (match state.clone() {
                                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10910, _, _, _, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value10910.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match (match state.clone() {
                                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10913, _, _, _, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value10913.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value") {
                                                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value10914, _) => {
                                                                                        source_ml_source2_data_property_value10914.clone()
                                                                                    }
                                                                                }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10915, _, _, _, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value10915.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value") {
                                                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value10916, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value10916 = *source_ml_source2_data_property_value10916;
                                                                                        source_ml_source2_data_property_value10916.clone()
                                                                                    }
                                                                                }, shared_constructor_field14.clone()))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10918, _, _, _, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value10918.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value") {
                                                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value10919, _, _) => {
                                                                                        let source_ml_source2_data_property_value10919 = *source_ml_source2_data_property_value10919;
                                                                                        source_ml_source2_data_property_value10919.clone()
                                                                                    }
                                                                                }, shared_constructor_field5.clone()))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350Return(self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 44, 32, 104, 105, 110, 116, 32, 97, 110, 100, 32, 83, 121, 109, 98, 111, 108]))
                                                                                } else {
                                                                                    {
                                                                                        let error = self::read_java_script_primitive_conversion_state_error(heap.clone(), (match state.clone() {
                                                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10921, _, _, _, _, _, _) => {
                                                                                                ((source_ml_source2_data_property_value10921.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value"));
                                                                                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350Return(error.clone())
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350Next
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                if !(match (match state.clone() {
                                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10922, _, _, _, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value10922.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                    self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350Return(self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 105, 118, 101, 32, 115, 108, 105, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 99, 97, 108, 97, 114, 32, 99, 104, 105, 108, 100]))
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350Next
                                                                                }
                                                                            };
                                                                            match source_ml_source2_s10907.clone() {
                                                                                self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350Next => {
                                                                                    if ((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101]))) && {
                                                                                        let source_ml_source2_s10926 = shared_constructor_field14.clone();
                                                                                        match source_ml_source2_s10926.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                true
                                                                                            }
                                                                                            _ => {
                                                                                                false
                                                                                            }
                                                                                        }
                                                                                    }) {
                                                                                        self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 115, 117, 115, 112, 101, 110, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 115, 99, 97, 108, 97, 114])
                                                                                    } else {
                                                                                        if (((self::ml_array::includes(&{ let ml_reference_value741 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101])]))).borrow().clone(); ml_reference_value741 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b) && (shared_constructor_field16 != (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))) && !(self::test_java_script_semantic_same_value(shared_constructor_field14.clone(), if (shared_constructor_field16 == (self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116]))) {
                                                                                            shared_constructor_field2.clone()
                                                                                        } else {
                                                                                            shared_constructor_field3.clone()
                                                                                        }))) && (shared_constructor_field15 != (self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101])))) {
                                                                                            self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116])
                                                                                        } else {
                                                                                            {
                                                                                                let initialized = self::ml_array::includes(&{ let ml_reference_value742 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104])]))).borrow().clone(); ml_reference_value742 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b);
                                                                                                if (initialized && ((shared_constructor_field9 != (self::ml_math::max((shared_constructor_field8 - shared_constructor_field7), 0f64))) || (shared_constructor_field10 != ((shared_constructor_field7 + shared_constructor_field11))))) {
                                                                                                    self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 116, 101, 114, 118, 97, 108, 32, 97, 110, 100, 32, 99, 111, 114, 114, 101, 115, 112, 111, 110, 100, 105, 110, 103, 32, 105, 110, 112, 117, 116, 47, 111, 117, 116, 112, 117, 116, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115])
                                                                                                } else {
                                                                                                    if ((self::ml_array::includes(&{ let ml_reference_value743 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])]))).borrow().clone(); ml_reference_value743 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b) && ((shared_constructor_field11 != (0f64)) || {
                                                                                                        let source_ml_source2_s10957 = shared_constructor_field12.clone();
                                                                                                        match source_ml_source2_s10957.clone() {
                                                                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                                                                false
                                                                                                            }
                                                                                                            _ => {
                                                                                                                true
                                                                                                            }
                                                                                                        }
                                                                                                    })) || ((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104]))) && (shared_constructor_field11 != shared_constructor_field9))) {
                                                                                                        self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 112, 114, 101, 99, 101, 100, 101, 115, 32, 101, 108, 101, 109, 101, 110, 116, 32, 119, 111, 114, 107, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 32, 83, 101, 116, 32, 102, 111, 108, 108, 111, 119, 115, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 116, 101, 114, 118, 97, 108])
                                                                                                    } else {
                                                                                                        if ((shared_constructor_field15 != (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]))) && {
                                                                                                            let source_ml_source2_s10968 = shared_constructor_field13.clone();
                                                                                                            match source_ml_source2_s10968.clone() {
                                                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                                                    false
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    true
                                                                                                                }
                                                                                                            }
                                                                                                        }) {
                                                                                                            self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 110, 32, 101, 108, 101, 109, 101, 110, 116, 32, 118, 97, 108, 117, 101, 32, 111, 110, 108, 121, 32, 100, 117, 114, 105, 110, 103, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 117, 116, 112, 117, 116, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])
                                                                                                        } else {
                                                                                                            if (self::ml_array::includes(&{ let ml_reference_value744 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])]))).borrow().clone(); ml_reference_value744 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b) && (shared_constructor_field11 >= shared_constructor_field9)) {
                                                                                                                self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 101, 108, 101, 109, 101, 110, 116, 32, 119, 111, 114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 110, 32, 105, 110, 45, 114, 97, 110, 103, 101, 32, 111, 117, 116, 112, 117, 116, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let source_ml_source2_s10986 = if !(match (match state.clone() {
                                                                                                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10987, _, _, _, _, _) => {
                                                                                                                            ((source_ml_source2_data_property_value10987.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                        }
                                                                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                                        {
                                                                                                                            let error_2 = self::read_java_script_array_species_state_error(heap.clone(), directory.clone(), (match state.clone() {
                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10988, _, _, _, _, _) => {
                                                                                                                                    ((source_ml_source2_data_property_value10988.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                                }
                                                                                                                            }).clone().expect("a proven present value"));
                                                                                                                            if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351Return(error_2.clone())
                                                                                                                            } else {
                                                                                                                                if (!(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10989, _, _, _, _, _) => {
                                                                                                                                        ((source_ml_source2_data_property_value10989.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptArraySpeciesState::ArraySpeciesState(source_ml_source2_data_property_value10990, _, _, _, _, _, _, _, _) => {
                                                                                                                                        let source_ml_source2_data_property_value10990 = *source_ml_source2_data_property_value10990;
                                                                                                                                        source_ml_source2_data_property_value10990.clone()
                                                                                                                                    }
                                                                                                                                }, shared_constructor_field1.clone())) || ((match (match state.clone() {
                                                                                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10995, _, _, _, _, _) => {
                                                                                                                                        ((source_ml_source2_data_property_value10995.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                                    }
                                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptArraySpeciesState::ArraySpeciesState(_, source_ml_source2_data_property_value10996, _, _, _, _, _, _, _) => {
                                                                                                                                        source_ml_source2_data_property_value10996
                                                                                                                                    }
                                                                                                                                }) != shared_constructor_field9)) {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351Return(self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 115, 112, 101, 99, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 97, 110, 100, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 99, 111, 117, 110, 116]))
                                                                                                                                } else {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351Next
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351Next
                                                                                                                    };
                                                                                                                    match source_ml_source2_s10986.clone() {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351Next => {
                                                                                                                            if (initialized && match (match state.clone() {
                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10998, _, _, _, _, _) => {
                                                                                                                                    ((source_ml_source2_data_property_value10998.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                                }
                                                                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                                                self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 114, 101, 115, 117, 108, 116, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 101, 99, 105, 101, 115, 32, 115, 101, 108, 101, 99, 116, 105, 111, 110])
                                                                                                                            } else {
                                                                                                                                if (self::ml_array::includes(&{ let ml_reference_value745 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104])]))).borrow().clone(); ml_reference_value745 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b) && {
                                                                                                                                    let source_ml_source2_s11001 = shared_constructor_field12.clone();
                                                                                                                                    match source_ml_source2_s11001.clone() {
                                                                                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                                            false
                                                                                                                                        }
                                                                                                                                        _ => {
                                                                                                                                            true
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }) {
                                                                                                                                    self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 101, 100, 32, 114, 101, 115, 117, 108, 116, 32, 79, 98, 106, 101, 99, 116])
                                                                                                                                } else {
                                                                                                                                    if ((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]))) && (match (match state.clone() {
                                                                                                                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11007, _, _, _, _) => {
                                                                                                                                            ((source_ml_source2_data_property_value11007.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value((match state.clone() {
                                                                                                                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11008, _, _, _, _) => {
                                                                                                                                            ((source_ml_source2_data_property_value11008.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    }).clone().expect("a proven present value"), match directory.clone() {
                                                                                                                                        self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(_, _, source_ml_source2_data_property_value11009, _, _, _) => {
                                                                                                                                            let source_ml_source2_data_property_value11009 = *source_ml_source2_data_property_value11009;
                                                                                                                                            source_ml_source2_data_property_value11009.clone()
                                                                                                                                        }
                                                                                                                                    })))) {
                                                                                                                                        self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 102, 97, 108, 108, 98, 97, 99, 107, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                                                                                                    } else {
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_s11049 = if (shared_constructor_field15 == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]))) {
                                                                                                                                                {
                                                                                                                                                    let constructor = if {
                                                                                                                                                        let source_ml_source2_own_property_operands11052 = (state.clone(), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]));
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_own_property_key11053 = ((source_ml_source2_own_property_operands11052.clone()).1,);
                                                                                                                                                            match (source_ml_source2_own_property_operands11052.clone()).0 {
                                                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_owned_constructor_field11054, source_ml_source2_owned_constructor_field11055, source_ml_source2_owned_constructor_field11056, source_ml_source2_owned_constructor_field11057, source_ml_source2_owned_constructor_field11058, source_ml_source2_owned_constructor_field11059, source_ml_source2_owned_constructor_field11060) => {
                                                                                                                                                                    ((((((((((((((((((((((((((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![36]))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![101, 110, 100])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![101, 110, 100, 80, 114, 101, 115, 101, 110, 116])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 75, 101, 121])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![101, 110, 100, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 117, 110, 116])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![112, 104, 97, 115, 101])))) || (((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 75, 105, 110, 100])))) || ((((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && match source_ml_source2_owned_constructor_field11054.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && match source_ml_source2_owned_constructor_field11055.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 80, 114, 111, 116, 111, 116, 121, 112, 101]))) && match source_ml_source2_owned_constructor_field11056.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) && match source_ml_source2_owned_constructor_field11057.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 70, 97, 99, 116]))) && match source_ml_source2_owned_constructor_field11058.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116]))) && match source_ml_source2_owned_constructor_field11059.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key11053.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) && match source_ml_source2_owned_constructor_field11060.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }))
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    } {
                                                                                                                                                        match state.clone() {
                                                                                                                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11061, _, _, _) => {
                                                                                                                                                                ((source_ml_source2_data_property_value11061.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        None::<self::JavaScriptSemanticValue>
                                                                                                                                                    };
                                                                                                                                                    {
                                                                                                                                                        let fact = match state.clone() {
                                                                                                                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11062, _, _) => {
                                                                                                                                                                ((source_ml_source2_data_property_value11062.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConstructorFact>,))).0).clone()
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        {
                                                                                                                                                            let classification = self::classify_java_script_array_species_constructor(heap.clone(), directory.clone(), constructor.clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                                                                                                                            {
                                                                                                                                                                let actual = self::read_java_script_known_array_constructor_fact(classification.clone());
                                                                                                                                                                if ((((((((((match (constructor.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match constructor.clone().expect("a proven present value") {
                                                                                                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                                                                        false
                                                                                                                                                                    }
                                                                                                                                                                    _ => {
                                                                                                                                                                        true
                                                                                                                                                                    }
                                                                                                                                                                }) || match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(match fact.clone().expect("a proven present value") {
                                                                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value11065, _, _, _) => {
                                                                                                                                                                        source_ml_source2_data_property_value11065
                                                                                                                                                                    }
                                                                                                                                                                })) || match (actual.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::test_java_script_semantic_same_value(constructor.clone().expect("a proven present value"), ({
                                                                                                                                                                    let source_ml_source2_optional_chain_receiver11067 = (match state.clone() {
                                                                                                                                                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11066, _, _, _, _, _) => {
                                                                                                                                                                            ((source_ml_source2_data_property_value11066.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                                                                                                        }
                                                                                                                                                                    },);
                                                                                                                                                                    if match (source_ml_source2_optional_chain_receiver11067.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                                                        Some({
                                                                                                                                                                            let source_ml_source1_temporary_optional_chain347 = ((source_ml_source2_optional_chain_receiver11067.clone()).0).clone().expect("a proven present value");
                                                                                                                                                                            match source_ml_source1_temporary_optional_chain347.clone() {
                                                                                                                                                                                self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, source_ml_source2_data_property_value11068, _, _) => {
                                                                                                                                                                                    let source_ml_source2_data_property_value11068 = *source_ml_source2_data_property_value11068;
                                                                                                                                                                                    source_ml_source2_data_property_value11068.clone()
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        })
                                                                                                                                                                    } else {
                                                                                                                                                                        None::<self::JavaScriptSemanticValue>
                                                                                                                                                                    }
                                                                                                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || (((((match constructor.clone().expect("a proven present value") {
                                                                                                                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value11071, _) => {
                                                                                                                                                                        Some((source_ml_source2_data_property_value11071.clone(),))
                                                                                                                                                                    }
                                                                                                                                                                    _ => {
                                                                                                                                                                        None::<(self::ml::Big,)>
                                                                                                                                                                    }
                                                                                                                                                                }).clone().expect("a proven present value")).0).clone()) != (match fact.clone().expect("a proven present value") {
                                                                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(source_ml_source2_data_property_value11072, _, _, _, _) => {
                                                                                                                                                                        source_ml_source2_data_property_value11072.clone()
                                                                                                                                                                    }
                                                                                                                                                                }))) || ((match actual.clone().expect("a proven present value") {
                                                                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value11075, _, _, _) => {
                                                                                                                                                                        source_ml_source2_data_property_value11075
                                                                                                                                                                    }
                                                                                                                                                                }) != (match fact.clone().expect("a proven present value") {
                                                                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value11076, _, _, _) => {
                                                                                                                                                                        source_ml_source2_data_property_value11076
                                                                                                                                                                    }
                                                                                                                                                                }))) || ((match actual.clone().expect("a proven present value") {
                                                                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value11079, _, _) => {
                                                                                                                                                                        source_ml_source2_data_property_value11079.clone()
                                                                                                                                                                    }
                                                                                                                                                                }) != (match fact.clone().expect("a proven present value") {
                                                                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value11080, _, _) => {
                                                                                                                                                                        source_ml_source2_data_property_value11080.clone()
                                                                                                                                                                    }
                                                                                                                                                                }))) || ((match actual.clone().expect("a proven present value") {
                                                                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value11083, _) => {
                                                                                                                                                                        source_ml_source2_data_property_value11083
                                                                                                                                                                    }
                                                                                                                                                                }) != (match fact.clone().expect("a proven present value") {
                                                                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value11084, _) => {
                                                                                                                                                                        source_ml_source2_data_property_value11084
                                                                                                                                                                    }
                                                                                                                                                                }))) || {
                                                                                                                                                                    let source_ml_source2_comparison_operand11087 = match actual.clone().expect("a proven present value") {
                                                                                                                                                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value11085) => {
                                                                                                                                                                            ((source_ml_source2_data_property_value11085.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                                                        }
                                                                                                                                                                    };
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_comparison_operand11088 = match fact.clone().expect("a proven present value") {
                                                                                                                                                                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value11086) => {
                                                                                                                                                                                ((source_ml_source2_data_property_value11086.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                                                                                            }
                                                                                                                                                                        };
                                                                                                                                                                        !(match (source_ml_source2_comparison_operand11087.clone(), source_ml_source2_comparison_operand11088.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                                                                                                                                                                    }
                                                                                                                                                                }) {
                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352Return(self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 97, 99, 116]))
                                                                                                                                                                } else {
                                                                                                                                                                    self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352Next
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            } else {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352Next
                                                                                                                                            };
                                                                                                                                            match source_ml_source2_s11049.clone() {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352Next => {
                                                                                                                                                    self::read_java_script_array_slice_child_state_error(heap.clone(), state.clone())
                                                                                                                                                }
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352Return(source_ml_source1_temporary_result352) => {
                                                                                                                                                    source_ml_source1_temporary_result352.clone()
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                        self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351Return(source_ml_source1_temporary_result352_2) => {
                                                                                                                            source_ml_source1_temporary_result352_2.clone()
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
                                                                                self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350::MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350Return(source_ml_source1_temporary_result352_3) => {
                                                                                    source_ml_source1_temporary_result352_3.clone()
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop349Result::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop349Return(source_ml_source1_temporary_result352_4) => {
                                                                source_ml_source1_temporary_result352_4.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop348Result::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop348Return(source_ml_source1_temporary_result352_5) => {
                                            source_ml_source1_temporary_result352_5.clone()
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
