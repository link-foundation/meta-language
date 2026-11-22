// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptArrayTransformCall;
use super::JavaScriptArrayTransformState;
use super::JavaScriptArrayTransformTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::begin_java_script_array_species_selection;
use super::continue_java_script_array_transform_species;
use super::ml_string;
use super::read_java_script_array_like_length;
use super::read_java_script_array_transform_state_error;
use super::request_java_script_array_transform_call;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_array_transform_length(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArrayTransformState,
    transition: self::JavaScriptScalarConversionTransition,
) -> self::JavaScriptArrayTransformTransition {
    {
        let source_ml_source1_temporary_binding242 = state.clone();
        {
            let prior_call = match source_ml_source1_temporary_binding242.clone() {
                self::JavaScriptArrayTransformState::ArrayTransformState(
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
                    source_ml_source2_data_property_value5102,
                ) => ((source_ml_source2_data_property_value5102
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,)))
                .0)
                    .clone(),
            };
            {
                let prior_conversion = match source_ml_source1_temporary_binding242.clone() {
                    self::JavaScriptArrayTransformState::ArrayTransformState(
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
                        source_ml_source2_data_property_value5103,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => ((source_ml_source2_data_property_value5103
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,)))
                    .0)
                        .clone(),
                };
                {
                    let base = match source_ml_source1_temporary_binding242.clone() {
                        self::JavaScriptArrayTransformState::ArrayTransformState(
                            source_ml_source2_nominal_rest_field5104,
                            source_ml_source2_nominal_rest_field5105,
                            source_ml_source2_nominal_rest_field5106,
                            source_ml_source2_nominal_rest_field5107,
                            source_ml_source2_nominal_rest_field5108,
                            source_ml_source2_nominal_rest_field5109,
                            source_ml_source2_nominal_rest_field5110,
                            source_ml_source2_nominal_rest_field5111,
                            source_ml_source2_nominal_rest_field5112,
                            source_ml_source2_nominal_rest_field5113,
                            source_ml_source2_nominal_rest_field5114,
                            source_ml_source2_nominal_rest_field5115,
                            source_ml_source2_nominal_rest_field5116,
                            _,
                            source_ml_source2_nominal_rest_field5117,
                            source_ml_source2_nominal_rest_field5118,
                            source_ml_source2_nominal_rest_field5119,
                            source_ml_source2_nominal_rest_field5120,
                            _,
                        ) => {
                            let source_ml_source2_nominal_rest_field5105 =
                                *source_ml_source2_nominal_rest_field5105;
                            let source_ml_source2_nominal_rest_field5106 =
                                *source_ml_source2_nominal_rest_field5106;
                            let source_ml_source2_nominal_rest_field5107 =
                                *source_ml_source2_nominal_rest_field5107;
                            let source_ml_source2_nominal_rest_field5108 =
                                *source_ml_source2_nominal_rest_field5108;
                            let source_ml_source2_nominal_rest_field5112 =
                                *source_ml_source2_nominal_rest_field5112;
                            let source_ml_source2_nominal_rest_field5113 =
                                *source_ml_source2_nominal_rest_field5113;
                            let source_ml_source2_nominal_rest_field5114 =
                                *source_ml_source2_nominal_rest_field5114;
                            let source_ml_source2_nominal_rest_field5115 =
                                *source_ml_source2_nominal_rest_field5115;
                            self::JavaScriptArrayTransformState::ArrayTransformState(
                                source_ml_source2_nominal_rest_field5104.clone(),
                                Box::new(source_ml_source2_nominal_rest_field5105.clone()),
                                Box::new(source_ml_source2_nominal_rest_field5106.clone()),
                                Box::new(source_ml_source2_nominal_rest_field5107.clone()),
                                Box::new(source_ml_source2_nominal_rest_field5108.clone()),
                                source_ml_source2_nominal_rest_field5109,
                                source_ml_source2_nominal_rest_field5110,
                                source_ml_source2_nominal_rest_field5111,
                                Box::new(source_ml_source2_nominal_rest_field5112.clone()),
                                Box::new(source_ml_source2_nominal_rest_field5113.clone()),
                                Box::new(source_ml_source2_nominal_rest_field5114.clone()),
                                Box::new(source_ml_source2_nominal_rest_field5115.clone()),
                                source_ml_source2_nominal_rest_field5116.clone(),
                                None::<(Option<self::JavaScriptPrimitiveConversionState>,)>,
                                source_ml_source2_nominal_rest_field5117.clone(),
                                source_ml_source2_nominal_rest_field5118.clone(),
                                source_ml_source2_nominal_rest_field5119.clone(),
                                source_ml_source2_nominal_rest_field5120.clone(),
                                None::<(Option<self::JavaScriptArrayTransformCall>,)>,
                            )
                        }
                    };
                    match base.clone() {
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
                            match transition.clone() {
                                self::JavaScriptScalarConversionTransition::ScalarContinuation(state_2) => {
                                    let state_2 = *state_2;
                                    self::JavaScriptArrayTransformTransition::ArrayTransformContinue(Box::new(heap.clone()), Box::new({
                                        let source_ml_source2_nominal_field5122 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field5124 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field5126 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field5128 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field5130 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field5132 = shared_constructor_field6;
                                                            {
                                                                let source_ml_source2_nominal_field5134 = shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field5136 = shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field5138 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field5140 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field5142 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field5144 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field5146 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field5148 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field5150 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field5152 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field5154 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field5156 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field5158 = shared_constructor_field19.clone();
                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field5122.clone(), Box::new(source_ml_source2_nominal_field5124.clone()), Box::new(source_ml_source2_nominal_field5126.clone()), Box::new(source_ml_source2_nominal_field5128.clone()), Box::new(source_ml_source2_nominal_field5130.clone()), source_ml_source2_nominal_field5132, source_ml_source2_nominal_field5134, source_ml_source2_nominal_field5136, Box::new(source_ml_source2_nominal_field5138.clone()), Box::new(source_ml_source2_nominal_field5140.clone()), Box::new(source_ml_source2_nominal_field5142.clone()), Box::new(source_ml_source2_nominal_field5144.clone()), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field5150.clone(), source_ml_source2_nominal_field5152.clone(), source_ml_source2_nominal_field5154.clone(), source_ml_source2_nominal_field5156.clone(), source_ml_source2_nominal_field5158.clone())
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
                                    }))
                                }
                                self::JavaScriptScalarConversionTransition::ScalarCall(state_3, callee, receiver, arguments) => {
                                    let state_3 = *state_3;
                                    let callee = *callee;
                                    let receiver = *receiver;
                                    self::request_java_script_array_transform_call(heap.clone(), {
                                        let source_ml_source2_nominal_field5163 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field5165 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field5167 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field5169 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field5171 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field5173 = shared_constructor_field6;
                                                            {
                                                                let source_ml_source2_nominal_field5175 = shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field5177 = shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field5179 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field5181 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field5183 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field5185 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field5187 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field5189 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field5191 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field5193 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field5195 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field5197 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field5199 = shared_constructor_field19.clone();
                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field5163.clone(), Box::new(source_ml_source2_nominal_field5165.clone()), Box::new(source_ml_source2_nominal_field5167.clone()), Box::new(source_ml_source2_nominal_field5169.clone()), Box::new(source_ml_source2_nominal_field5171.clone()), source_ml_source2_nominal_field5173, source_ml_source2_nominal_field5175, source_ml_source2_nominal_field5177, Box::new(source_ml_source2_nominal_field5179.clone()), Box::new(source_ml_source2_nominal_field5181.clone()), Box::new(source_ml_source2_nominal_field5183.clone()), Box::new(source_ml_source2_nominal_field5185.clone()), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field5191.clone(), source_ml_source2_nominal_field5193.clone(), source_ml_source2_nominal_field5195.clone(), source_ml_source2_nominal_field5197.clone(), source_ml_source2_nominal_field5199.clone())
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
                                    }, callee.clone(), receiver.clone(), arguments.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarCompleted(primitive) => {
                                    let primitive = *primitive;
                                    match primitive.clone() {
                                        self::JavaScriptSemanticValue::Number(value) => {
                                            if ({
                                                let source_ml_source2_s5204 = match state.clone() {
                                                    self::JavaScriptArrayTransformState::ArrayTransformState(_, _, source_ml_source2_data_property_value5207, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value5207 = *source_ml_source2_data_property_value5207;
                                                        source_ml_source2_data_property_value5207.clone()
                                                    }
                                                };
                                                match source_ml_source2_s5204.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            } || !(((match match state.clone() {
                                                self::JavaScriptArrayTransformState::ArrayTransformState(_, _, source_ml_source2_data_property_value5208, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value5208 = *source_ml_source2_data_property_value5208;
                                                    source_ml_source2_data_property_value5208.clone()
                                                }
                                            } {
                                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value5209) => {
                                                    Some((source_ml_source2_data_property_value5209,))
                                                }
                                                _ => {
                                                    None::<(bool,)>
                                                }
                                            }).clone().expect("a proven present value")).0)) {
                                                self::JavaScriptArrayTransformTransition::ArrayTransformError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101]))
                                            } else {
                                                {
                                                    let length = self::read_java_script_array_like_length(value);
                                                    {
                                                        let next = {
                                                            let source_ml_source2_nominal_field5211 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field5213 = shared_constructor_field2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field5215 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field5217 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field5219 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field5221 = shared_constructor_field6;
                                                                                {
                                                                                    let source_ml_source2_nominal_field5223 = shared_constructor_field7;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field5225 = shared_constructor_field8;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field5227 = shared_constructor_field9.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field5229 = shared_constructor_field10.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field5231 = shared_constructor_field11.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field5233 = shared_constructor_field12.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field5235 = shared_constructor_field13.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field5237 = shared_constructor_field14.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field5239 = shared_constructor_field15.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field5241 = shared_constructor_field16.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field5243 = shared_constructor_field17.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field5245 = shared_constructor_field18.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field5247 = shared_constructor_field19.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override5250 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field5211.clone(), Box::new(source_ml_source2_nominal_field5213.clone()), Box::new(source_ml_source2_nominal_field5215.clone()), Box::new(source_ml_source2_nominal_field5217.clone()), Box::new(source_ml_source2_nominal_field5219.clone()), length, source_ml_source2_nominal_field5223, source_ml_source2_nominal_field5225, Box::new(source_ml_source2_nominal_field5227.clone()), Box::new(source_ml_source2_nominal_field5229.clone()), Box::new(source_ml_source2_nominal_field5231.clone()), Box::new(source_ml_source2_nominal_override5250.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), source_ml_source2_nominal_field5237.clone(), source_ml_source2_nominal_field5239.clone(), source_ml_source2_nominal_field5241.clone(), source_ml_source2_nominal_field5243.clone(), source_ml_source2_nominal_field5245.clone(), source_ml_source2_nominal_field5247.clone())
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
                                                        };
                                                        self::continue_java_script_array_transform_species(heap.clone(), next.clone(), self::begin_java_script_array_species_selection(heap.clone(), directory.clone(), match state.clone() {
                                                            self::JavaScriptArrayTransformState::ArrayTransformState(_, source_ml_source2_data_property_value5252, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value5252 = *source_ml_source2_data_property_value5252;
                                                                source_ml_source2_data_property_value5252.clone()
                                                            }
                                                        }, if ((match state.clone() {
                                                            self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_data_property_value5254, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value5254.clone()
                                                            }
                                                        }) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 109, 97, 112]))) {
                                                            length
                                                        } else {
                                                            0f64
                                                        }))
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                                        }
                                    }
                                }
                                self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                                    let value_2 = *value_2;
                                    self::JavaScriptArrayTransformTransition::ArrayTransformThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
                                }
                                self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class, reason) => {
                                    self::JavaScriptArrayTransformTransition::ArrayTransformError(Box::new(heap.clone()), error_class.clone(), reason.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(Box::new(heap.clone()), reason_2.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarResourceLimit(value_3, reason_3) => {
                                    let value_3 = *value_3;
                                    self::JavaScriptArrayTransformTransition::ArrayTransformResourceLimit(Box::new(heap.clone()), Box::new({
                                        let source_ml_source2_nominal_field5256 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field5258 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field5260 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field5262 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field5264 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field5266 = shared_constructor_field6;
                                                            {
                                                                let source_ml_source2_nominal_field5268 = shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field5270 = shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field5272 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field5274 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field5276 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field5278 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field5280 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field5282 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field5284 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field5286 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field5288 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field5290 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field5292 = shared_constructor_field19.clone();
                                                                                                                self::JavaScriptArrayTransformState::ArrayTransformState(source_ml_source2_nominal_field5256.clone(), Box::new(source_ml_source2_nominal_field5258.clone()), Box::new(source_ml_source2_nominal_field5260.clone()), Box::new(source_ml_source2_nominal_field5262.clone()), Box::new(source_ml_source2_nominal_field5264.clone()), source_ml_source2_nominal_field5266, source_ml_source2_nominal_field5268, source_ml_source2_nominal_field5270, Box::new(source_ml_source2_nominal_field5272.clone()), Box::new(source_ml_source2_nominal_field5274.clone()), Box::new(source_ml_source2_nominal_field5276.clone()), Box::new(value_3.clone()), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101]), source_ml_source2_nominal_field5282.clone(), source_ml_source2_nominal_field5284.clone(), source_ml_source2_nominal_field5286.clone(), source_ml_source2_nominal_field5288.clone(), source_ml_source2_nominal_field5290.clone(), source_ml_source2_nominal_field5292.clone())
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_array_transform(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    operation: &self::ml_string::Text,
    receiver: self::JavaScriptSemanticValue,
    callback: self::JavaScriptSemanticValue,
    this_argument: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayTransformTransition {
    if (match receiver.clone() {
        self::JavaScriptSemanticValue::Null => true,
        _ => false,
    } || match receiver.clone() {
        self::JavaScriptSemanticValue::Undefined => true,
        _ => false,
    }) {
        self::JavaScriptArrayTransformTransition::ArrayTransformError(
            Box::new(heap.clone()),
            self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116,
                105, 111, 110, 32, 99, 97, 108, 108, 101, 100, 32, 111, 110, 32, 110, 117, 108,
                108, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100,
            ]),
        )
    } else {
        {
            let state = {
                let source_ml_source2_constructor_field5301 = 0f64;
                {
                    let source_ml_source2_constructor_field5302 = 0f64;
                    {
                        let source_ml_source2_constructor_field5303 = 0f64;
                        {
                            let source_ml_source2_constructor_field5304 =
                                self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source2_constructor_field5305 =
                                    self::JavaScriptSemanticValue::Undefined;
                                {
                                    let source_ml_source2_constructor_field5306 =
                                        self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source2_constructor_field5307 =
                                            self::JavaScriptSemanticValue::Undefined;
                                        self::JavaScriptArrayTransformState::ArrayTransformState(
                                            operation.clone(),
                                            Box::new(receiver.clone()),
                                            Box::new(callback.clone()),
                                            Box::new(this_argument.clone()),
                                            Box::new(exotic_key.clone()),
                                            source_ml_source2_constructor_field5301,
                                            source_ml_source2_constructor_field5302,
                                            source_ml_source2_constructor_field5303,
                                            Box::new(
                                                source_ml_source2_constructor_field5304.clone(),
                                            ),
                                            Box::new(
                                                source_ml_source2_constructor_field5305.clone(),
                                            ),
                                            Box::new(
                                                source_ml_source2_constructor_field5306.clone(),
                                            ),
                                            Box::new(
                                                source_ml_source2_constructor_field5307.clone(),
                                            ),
                                            self::ml_string::Text::from_units(vec![
                                                108, 101, 110, 103, 116, 104,
                                            ]),
                                            None::<(
                                                Option<self::JavaScriptPrimitiveConversionState>,
                                            )>,
                                            None::<(Option<self::JavaScriptArraySpeciesState>,)>,
                                            None::<(Option<self::JavaScriptSemanticValue>,)>,
                                            None::<(Option<self::JavaScriptSemanticValue>,)>,
                                            None::<(Option<self::JavaScriptArrayConstructorFact>,)>,
                                            None::<(Option<self::JavaScriptArrayTransformCall>,)>,
                                        )
                                    }
                                }
                            }
                        }
                    }
                }
            };
            {
                let error = self::read_java_script_array_transform_state_error(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error == (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayTransformTransition::ArrayTransformContinue(
                        Box::new(heap.clone()),
                        Box::new(state.clone()),
                    )
                } else {
                    self::JavaScriptArrayTransformTransition::ArrayTransformInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                }
            }
        }
    }
}
