// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCallbackCall;
use super::JavaScriptArrayCallbackState;
use super::JavaScriptArrayCallbackTransition;
use super::JavaScriptErrorAllocationDirectory;
use super::JavaScriptErrorAllocationIdentity;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Result;
use super::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Result;
use super::begin_java_script_array_callback_element;
use super::begin_java_script_scalar_protocol;
use super::continue_java_script_array_callback_length;
use super::convert_java_script_semantic_boolean;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_callback_state_error;
use super::read_java_script_ordinary_value_error;
use super::resume_java_script_scalar_protocol;
use super::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop1256;
use super::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop1257;
use super::test_java_script_array_callback_reduction;
use super::test_java_script_array_callback_reverse;
use super::test_java_script_array_callback_visit_holes;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_array_callback(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCallbackState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArrayCallbackTransition {
    match state.clone() {
        self::JavaScriptArrayCallbackState::ArrayCallbackState(
            shared_constructor_field15,
            _,
            _,
            _,
            shared_constructor_field16,
            _,
            shared_constructor_field17,
            shared_constructor_field18,
            _,
            _,
            _,
            shared_constructor_field19,
            _,
            _,
        ) => {
            let shared_constructor_field16 = *shared_constructor_field16;
            let shared_constructor_field19 = *shared_constructor_field19;
            {
                let error =
                    self::read_java_script_array_callback_state_error(heap.clone(), state.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if match (
                        match state.clone() {
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
                                source_ml_source2_data_property_value28836,
                            ) => ((source_ml_source2_data_property_value28836
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32,
                                114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101,
                                101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108,
                                100, 32, 99, 97, 108, 108,
                            ]),
                        )
                    } else {
                        {
                            let invalid = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value28837) => {
                                    let source_ml_source2_data_property_value28837 = *source_ml_source2_data_property_value28837;
                                    source_ml_source2_data_property_value28837.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value28838) => {
                                    let source_ml_source2_data_property_value28838 = *source_ml_source2_data_property_value28838;
                                    source_ml_source2_data_property_value28838.clone()
                                }
                            });
                            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(
                                    Box::new(heap.clone()),
                                    invalid.clone(),
                                )
                            } else {
                                match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        {
                                            let prior_call = match state.clone() {
                                                self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28839) => {
                                                    ((source_ml_source2_data_property_value28839.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                                                }
                                            };
                                            {
                                                let base = match state.clone() {
                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_rest_field28840, source_ml_source2_nominal_rest_field28841, source_ml_source2_nominal_rest_field28842, source_ml_source2_nominal_rest_field28843, source_ml_source2_nominal_rest_field28844, source_ml_source2_nominal_rest_field28845, source_ml_source2_nominal_rest_field28846, source_ml_source2_nominal_rest_field28847, source_ml_source2_nominal_rest_field28848, source_ml_source2_nominal_rest_field28849, source_ml_source2_nominal_rest_field28850, source_ml_source2_nominal_rest_field28851, source_ml_source2_nominal_rest_field28852, _) => {
                                                        let source_ml_source2_nominal_rest_field28841 = *source_ml_source2_nominal_rest_field28841;
                                                        let source_ml_source2_nominal_rest_field28842 = *source_ml_source2_nominal_rest_field28842;
                                                        let source_ml_source2_nominal_rest_field28843 = *source_ml_source2_nominal_rest_field28843;
                                                        let source_ml_source2_nominal_rest_field28844 = *source_ml_source2_nominal_rest_field28844;
                                                        let source_ml_source2_nominal_rest_field28850 = *source_ml_source2_nominal_rest_field28850;
                                                        let source_ml_source2_nominal_rest_field28851 = *source_ml_source2_nominal_rest_field28851;
                                                        self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_rest_field28840.clone(), Box::new(source_ml_source2_nominal_rest_field28841.clone()), Box::new(source_ml_source2_nominal_rest_field28842.clone()), Box::new(source_ml_source2_nominal_rest_field28843.clone()), Box::new(source_ml_source2_nominal_rest_field28844.clone()), source_ml_source2_nominal_rest_field28845, source_ml_source2_nominal_rest_field28846, source_ml_source2_nominal_rest_field28847.clone(), source_ml_source2_nominal_rest_field28848, source_ml_source2_nominal_rest_field28849, Box::new(source_ml_source2_nominal_rest_field28850.clone()), Box::new(source_ml_source2_nominal_rest_field28851.clone()), source_ml_source2_nominal_rest_field28852.clone(), None::<(Option<self::JavaScriptArrayCallbackCall>,)>)
                                                    }
                                                };
                                                match base.clone() {
                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11, shared_constructor_field12, shared_constructor_field13, shared_constructor_field14) => {
                                                        let shared_constructor_field2 = *shared_constructor_field2;
                                                        let shared_constructor_field3 = *shared_constructor_field3;
                                                        let shared_constructor_field4 = *shared_constructor_field4;
                                                        let shared_constructor_field5 = *shared_constructor_field5;
                                                        let shared_constructor_field11 = *shared_constructor_field11;
                                                        let shared_constructor_field12 = *shared_constructor_field12;
                                                        if (shared_constructor_field18 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]))) {
                                                            self::continue_java_script_array_callback_length(heap.clone(), {
                                                                let source_ml_source2_nominal_field28856 = shared_constructor_field1.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field28858 = shared_constructor_field2.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field28860 = shared_constructor_field3.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field28862 = shared_constructor_field4.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field28864 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field28866 = shared_constructor_field6;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field28868 = shared_constructor_field7;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field28870 = shared_constructor_field8.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field28872 = shared_constructor_field9;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field28874 = shared_constructor_field10;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field28876 = shared_constructor_field11.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field28878 = shared_constructor_field12.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field28880 = shared_constructor_field13.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field28882 = shared_constructor_field14.clone();
                                                                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28856.clone(), Box::new(source_ml_source2_nominal_field28858.clone()), Box::new(source_ml_source2_nominal_field28860.clone()), Box::new(source_ml_source2_nominal_field28862.clone()), Box::new(source_ml_source2_nominal_field28864.clone()), source_ml_source2_nominal_field28866, source_ml_source2_nominal_field28868, source_ml_source2_nominal_field28870.clone(), source_ml_source2_nominal_field28872, source_ml_source2_nominal_field28874, Box::new(source_ml_source2_nominal_field28876.clone()), Box::new(value.clone()), source_ml_source2_nominal_field28880.clone(), source_ml_source2_nominal_field28882.clone())
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
                                                            }, self::begin_java_script_scalar_protocol(heap.clone(), value.clone(), shared_constructor_field16.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                        } else {
                                                            if (shared_constructor_field18 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) {
                                                                if match (match state.clone() {
                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28888, _) => {
                                                                        ((source_ml_source2_data_property_value28888.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 115, 116, 97, 116, 101]))
                                                                } else {
                                                                    self::continue_java_script_array_callback_length(heap.clone(), base.clone(), self::resume_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                                                        self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28889, _) => {
                                                                            ((source_ml_source2_data_property_value28889.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value"), completion.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                                }
                                                            } else {
                                                                if (shared_constructor_field18 == (self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]))) {
                                                                    self::begin_java_script_array_callback_element(heap.clone(), base.clone(), value.clone())
                                                                } else {
                                                                    if (shared_constructor_field18 != (self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107]))) {
                                                                        self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 110, 103, 116, 104, 44, 32, 101, 108, 101, 109, 101, 110, 116, 32, 111, 114, 32, 112, 114, 101, 100, 105, 99, 97, 116, 101, 32, 99, 97, 108, 108]))
                                                                    } else {
                                                                        if self::test_java_script_array_callback_reduction(&(shared_constructor_field15)) {
                                                                            self::JavaScriptArrayCallbackTransition::ArrayCallbackContinue(Box::new(heap.clone()), Box::new({
                                                                                let source_ml_source2_nominal_field28898 = shared_constructor_field1.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field28900 = shared_constructor_field2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field28902 = shared_constructor_field3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field28904 = shared_constructor_field4.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field28906 = shared_constructor_field5.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field28908 = shared_constructor_field6;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field28910 = shared_constructor_field7;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field28912 = shared_constructor_field8.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field28914 = shared_constructor_field9;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field28916 = shared_constructor_field10;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field28918 = shared_constructor_field11.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field28920 = shared_constructor_field12.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field28922 = shared_constructor_field13.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field28924 = shared_constructor_field14.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override28929 = (shared_constructor_field17 + if self::test_java_script_array_callback_reverse(&(shared_constructor_field15)) {
                                                                                                                                            (-1f64)
                                                                                                                                        } else {
                                                                                                                                            1f64
                                                                                                                                        });
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override28930 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                            self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28898.clone(), Box::new(source_ml_source2_nominal_field28900.clone()), Box::new(source_ml_source2_nominal_field28902.clone()), Box::new(source_ml_source2_nominal_field28904.clone()), Box::new(source_ml_source2_nominal_field28906.clone()), source_ml_source2_nominal_field28908, source_ml_source2_nominal_override28929, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), source_ml_source2_nominal_field28914, true, Box::new(value.clone()), Box::new(source_ml_source2_nominal_override28930.clone()), source_ml_source2_nominal_field28922.clone(), source_ml_source2_nominal_field28924.clone())
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
                                                                        } else {
                                                                            {
                                                                                let predicate = self::convert_java_script_semantic_boolean(value.clone());
                                                                                {
                                                                                    let find = self::test_java_script_array_callback_visit_holes(&(shared_constructor_field15));
                                                                                    if (((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 115, 111, 109, 101]))) && predicate) || ((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 101, 118, 101, 114, 121]))) && !predicate)) {
                                                                                        self::JavaScriptArrayCallbackTransition::ArrayCallbackCompleted(Box::new(heap.clone()), Box::new(self::JavaScriptSemanticValue::Boolean(predicate)))
                                                                                    } else {
                                                                                        if (find && predicate) {
                                                                                            self::JavaScriptArrayCallbackTransition::ArrayCallbackCompleted(Box::new(heap.clone()), Box::new(if ((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 73, 110, 100, 101, 120]))) || (shared_constructor_field15 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 76, 97, 115, 116, 73, 110, 100, 101, 120])))) {
                                                                                                self::JavaScriptSemanticValue::Number(shared_constructor_field17)
                                                                                            } else {
                                                                                                shared_constructor_field19.clone()
                                                                                            }))
                                                                                        } else {
                                                                                            self::JavaScriptArrayCallbackTransition::ArrayCallbackContinue(Box::new(heap.clone()), Box::new({
                                                                                                let source_ml_source2_nominal_field28945 = shared_constructor_field1.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field28947 = shared_constructor_field2.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field28949 = shared_constructor_field3.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field28951 = shared_constructor_field4.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field28953 = shared_constructor_field5.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field28955 = shared_constructor_field6;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field28957 = shared_constructor_field7;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field28959 = shared_constructor_field8.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field28961 = shared_constructor_field9;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field28963 = shared_constructor_field10;
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field28965 = shared_constructor_field11.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field28967 = shared_constructor_field12.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field28969 = shared_constructor_field13.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field28971 = shared_constructor_field14.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_override28974 = (shared_constructor_field17 + if self::test_java_script_array_callback_reverse(&(shared_constructor_field15)) {
                                                                                                                                                            (-1f64)
                                                                                                                                                        } else {
                                                                                                                                                            1f64
                                                                                                                                                        });
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_override28975 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                            self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28945.clone(), Box::new(source_ml_source2_nominal_field28947.clone()), Box::new(source_ml_source2_nominal_field28949.clone()), Box::new(source_ml_source2_nominal_field28951.clone()), Box::new(source_ml_source2_nominal_field28953.clone()), source_ml_source2_nominal_field28955, source_ml_source2_nominal_override28974, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), source_ml_source2_nominal_field28961, source_ml_source2_nominal_field28963, Box::new(source_ml_source2_nominal_field28965.clone()), Box::new(source_ml_source2_nominal_override28975.clone()), source_ml_source2_nominal_field28969.clone(), source_ml_source2_nominal_field28971.clone())
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
                                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                        let value_2 = *value_2;
                                        self::JavaScriptArrayCallbackTransition::ArrayCallbackThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
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
pub fn read_java_script_source_exception_operation(
    name: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*name) == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]))) {
        self::ml_string::Text::from_units(vec![
            114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111,
            114,
        ])
    } else {
        if ((*name)
            == (self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])))
        {
            self::ml_string::Text::from_units(vec![
                114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112,
                101, 69, 114, 114, 111, 114,
            ])
        } else {
            if ((*name)
                == (self::ml_string::Text::from_units(vec![
                    82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                ])))
            {
                self::ml_string::Text::from_units(vec![
                    114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97,
                    110, 103, 101, 69, 114, 114, 111, 114,
                ])
            } else {
                if ((*name)
                    == (self::ml_string::Text::from_units(vec![
                        83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114,
                    ])))
                {
                    self::ml_string::Text::from_units(vec![
                        114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83,
                        121, 110, 116, 97, 120, 69, 114, 114, 111, 114,
                    ])
                } else {
                    if ((*name)
                        == (self::ml_string::Text::from_units(vec![
                            82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114,
                        ])))
                    {
                        self::ml_string::Text::from_units(vec![
                            114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
                            82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114,
                        ])
                    } else {
                        if ((*name)
                            == (self::ml_string::Text::from_units(vec![
                                85, 82, 73, 69, 114, 114, 111, 114,
                            ])))
                        {
                            self::ml_string::Text::from_units(vec![
                                114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112,
                                116, 85, 82, 73, 69, 114, 114, 111, 114,
                            ])
                        } else {
                            if ((*name)
                                == (self::ml_string::Text::from_units(vec![
                                    69, 118, 97, 108, 69, 114, 114, 111, 114,
                                ])))
                            {
                                self::ml_string::Text::from_units(vec![
                                    114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112,
                                    116, 69, 118, 97, 108, 69, 114, 114, 111, 114,
                                ])
                            } else {
                                self::ml_string::Text::from_units(vec![])
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
pub fn read_java_script_source_exception_class(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*operation)
        == (self::ml_string::Text::from_units(vec![
            114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111,
            114,
        ])))
    {
        self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114])
    } else {
        if ((*operation)
            == (self::ml_string::Text::from_units(vec![
                114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112,
                101, 69, 114, 114, 111, 114,
            ])))
        {
            self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
        } else {
            if ((*operation)
                == (self::ml_string::Text::from_units(vec![
                    114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97,
                    110, 103, 101, 69, 114, 114, 111, 114,
                ])))
            {
                self::ml_string::Text::from_units(vec![
                    82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                ])
            } else {
                if ((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83,
                        121, 110, 116, 97, 120, 69, 114, 114, 111, 114,
                    ])))
                {
                    self::ml_string::Text::from_units(vec![
                        83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114,
                    ])
                } else {
                    if ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
                            82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114,
                        ])))
                    {
                        self::ml_string::Text::from_units(vec![
                            82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114,
                        ])
                    } else {
                        if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112,
                                116, 85, 82, 73, 69, 114, 114, 111, 114,
                            ])))
                        {
                            self::ml_string::Text::from_units(vec![
                                85, 82, 73, 69, 114, 114, 111, 114,
                            ])
                        } else {
                            if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112,
                                    116, 69, 118, 97, 108, 69, 114, 114, 111, 114,
                                ])))
                            {
                                self::ml_string::Text::from_units(vec![
                                    69, 118, 97, 108, 69, 114, 114, 111, 114,
                                ])
                            } else {
                                self::ml_string::Text::from_units(vec![])
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
pub fn test_java_script_error_constructor_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode)
        == (self::ml_string::Text::from_units(vec![
            100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114,
        ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_error_intrinsic_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode)
        == (self::ml_string::Text::from_units(vec![
            69, 114, 114, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99,
        ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_construction_kind_error(
    error_class: &self::ml_string::Text,
    bind_receiver: bool,
    mode: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((self::read_java_script_source_exception_operation(error_class))
        == (self::ml_string::Text::from_units(vec![])))
    {
        self::ml_string::Text::from_units(vec![
            69, 114, 114, 111, 114, 32, 118, 97, 108, 117, 101, 32, 99, 111, 110, 115, 116, 114,
            117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97,
            99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 105, 110, 116,
            114, 105, 110, 115, 105, 99, 32, 99, 108, 97, 115, 115,
        ])
    } else {
        if (bind_receiver && !(self::test_java_script_error_constructor_mode(mode))) {
            self::ml_string::Text::from_units(vec![
                111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 114,
                105, 118, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32,
                98, 105, 110, 100, 115, 32, 116, 104, 101, 32, 105, 110, 105, 116, 105, 97, 108,
                105, 122, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114,
            ])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_allocation_directory(
    heap: self::JavaScriptOrdinaryHeap,
) -> self::JavaScriptErrorAllocationDirectory {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
            shared_constructor_field1,
            shared_constructor_field2,
            _,
            _,
            _,
            _,
        ) => self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(
            std::rc::Rc::new(std::cell::RefCell::new({
                let ml_values1587 = {
                    let ml_reference_value1588 = shared_constructor_field1.borrow().clone();
                    ml_reference_value1588
                };
                let ml_callback1587 = std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptErrorAllocationIdentity { self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value29009, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29009.clone()
                }
                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value29010, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29010.clone()
                }
                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value29011, _, _, _, _) => {
                    source_ml_source2_data_property_value29011.clone()
                }
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value29012, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29012.clone()
                }
                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value29013, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29013.clone()
                }
                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value29014, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29014.clone()
                }
                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value29015, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29015.clone()
                }
                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value29016, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29016.clone()
                }
                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value29017, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29017.clone()
                }
                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value29018, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29018.clone()
                }
                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value29019, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29019.clone()
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value29020, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29020.clone()
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value29021, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29021.clone()
                }
            }, match object.clone() {
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
            }, match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value29022, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29022
                }
                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value29023, _, _, _, _) => {
                    source_ml_source2_data_property_value29023
                }
                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value29024, _) => {
                    source_ml_source2_data_property_value29024
                }
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value29025, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value29025
                }
                self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value29026, _, _) => {
                    source_ml_source2_data_property_value29026
                }
                self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value29027, _, _) => {
                    source_ml_source2_data_property_value29027
                }
                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value29028, _, _) => {
                    source_ml_source2_data_property_value29028
                }
                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value29029, _, _) => {
                    source_ml_source2_data_property_value29029
                }
                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value29030, _) => {
                    source_ml_source2_data_property_value29030
                }
                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value29031, _, _, _) => {
                    source_ml_source2_data_property_value29031
                }
                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value29032, _, _, _) => {
                    source_ml_source2_data_property_value29032
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value29033, _, _, _) => {
                    source_ml_source2_data_property_value29033
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value29034, _, _, _) => {
                    source_ml_source2_data_property_value29034
                }
            }) });
                ml_values1587
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(ml_index1587, ml_value1587)| ml_callback1587(ml_value1587))
                    .collect::<Vec<_>>()
            })),
            std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(
                Vec::<self::ml::Big>::new(),
                {
                    let ml_reference_value1589 = shared_constructor_field2.borrow().clone();
                    ml_reference_value1589
                },
            ))),
        ),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_prior_directory_error(
    heap: self::JavaScriptOrdinaryHeap,
    before: self::JavaScriptErrorAllocationDirectory,
) -> self::ml_string::Text {
    match before.clone() {
        self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            if ((({
                let ml_reference_array1590 = shared_constructor_field1.clone();
                let ml_reference_projection1590 = ml_reference_array1590.borrow().len() as f64;
                ml_reference_projection1590
            }) > (4096f64))
                || (({
                    let ml_reference_array1591 = shared_constructor_field2.clone();
                    let ml_reference_projection1591 = ml_reference_array1591.borrow().len() as f64;
                    ml_reference_projection1591
                }) > (16384f64)))
            {
                self::ml_string::Text::from_units(vec![
                    116, 104, 101, 32, 112, 114, 105, 111, 114, 32, 69, 114, 114, 111, 114, 32, 97,
                    108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 100, 105, 114, 101, 99, 116,
                    111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32,
                    101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 111, 114, 97, 103, 101,
                    32, 98, 117, 100, 103, 101, 116,
                ])
            } else {
                {
                    let previous = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_temporary_values1254 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source1_temporary_index1254 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s29041 = self::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop1256(heap.clone(), previous.clone(), source_ml_source1_temporary_values1254.clone(), source_ml_source1_temporary_index1254.clone());
                                match source_ml_source2_s29041.clone() {
                                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Done(previous_2) => {
                                        {
                                            let previous_3 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_temporary_values1255 = shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source1_temporary_index1255 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s29043 = self::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop1257(heap.clone(), before.clone(), previous_3.clone(), source_ml_source1_temporary_values1255.clone(), source_ml_source1_temporary_index1255.clone());
                                                        match source_ml_source2_s29043.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Done => {
                                                                self::ml_string::Text::from_units(vec![])
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1257Return(source_ml_source1_temporary_result1257) => {
                                                                source_ml_source1_temporary_result1257.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop1256Return(source_ml_source1_temporary_result1257_2) => {
                                        source_ml_source1_temporary_result1257_2.clone()
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
