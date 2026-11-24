// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConcatenationCall;
use super::JavaScriptArrayConcatenationState;
use super::JavaScriptArrayConcatenationTransition;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptArraySpeciesTransition;
use super::JavaScriptOrdinaryHeap;
use super::copy_java_script_array_constructor_fact;
use super::ml_string;
use super::request_java_script_array_concatenation_allocation;
use super::request_java_script_array_concatenation_call;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_array_concatenation_species(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayConcatenationState,
    transition: self::JavaScriptArraySpeciesTransition,
) -> self::JavaScriptArrayConcatenationTransition {
    {
        let source_ml_source1_temporary_binding338 = state.clone();
        {
            let prior_call = match source_ml_source1_temporary_binding338.clone() {
                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
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
                    source_ml_source2_data_property_value9059,
                ) => ((source_ml_source2_data_property_value9059
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,)))
                .0)
                    .clone(),
            };
            {
                let base = match source_ml_source1_temporary_binding338.clone() {
                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
                        source_ml_source2_nominal_rest_field9060,
                        source_ml_source2_nominal_rest_field9061,
                        source_ml_source2_nominal_rest_field9062,
                        source_ml_source2_nominal_rest_field9063,
                        source_ml_source2_nominal_rest_field9064,
                        source_ml_source2_nominal_rest_field9065,
                        source_ml_source2_nominal_rest_field9066,
                        source_ml_source2_nominal_rest_field9067,
                        source_ml_source2_nominal_rest_field9068,
                        source_ml_source2_nominal_rest_field9069,
                        source_ml_source2_nominal_rest_field9070,
                        source_ml_source2_nominal_rest_field9071,
                        source_ml_source2_nominal_rest_field9072,
                        source_ml_source2_nominal_rest_field9073,
                        source_ml_source2_nominal_rest_field9074,
                        source_ml_source2_nominal_rest_field9075,
                        source_ml_source2_nominal_rest_field9076,
                        source_ml_source2_nominal_rest_field9077,
                        source_ml_source2_nominal_rest_field9078,
                        source_ml_source2_nominal_rest_field9079,
                        source_ml_source2_nominal_rest_field9080,
                        _,
                    ) => {
                        let source_ml_source2_nominal_rest_field9060 =
                            *source_ml_source2_nominal_rest_field9060;
                        let source_ml_source2_nominal_rest_field9062 =
                            *source_ml_source2_nominal_rest_field9062;
                        let source_ml_source2_nominal_rest_field9063 =
                            *source_ml_source2_nominal_rest_field9063;
                        let source_ml_source2_nominal_rest_field9065 =
                            *source_ml_source2_nominal_rest_field9065;
                        let source_ml_source2_nominal_rest_field9071 =
                            *source_ml_source2_nominal_rest_field9071;
                        let source_ml_source2_nominal_rest_field9072 =
                            *source_ml_source2_nominal_rest_field9072;
                        let source_ml_source2_nominal_rest_field9073 =
                            *source_ml_source2_nominal_rest_field9073;
                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
                            Box::new(source_ml_source2_nominal_rest_field9060.clone()),
                            source_ml_source2_nominal_rest_field9061.clone(),
                            Box::new(source_ml_source2_nominal_rest_field9062.clone()),
                            Box::new(source_ml_source2_nominal_rest_field9063.clone()),
                            source_ml_source2_nominal_rest_field9064,
                            Box::new(source_ml_source2_nominal_rest_field9065.clone()),
                            source_ml_source2_nominal_rest_field9066,
                            source_ml_source2_nominal_rest_field9067,
                            source_ml_source2_nominal_rest_field9068,
                            source_ml_source2_nominal_rest_field9069,
                            source_ml_source2_nominal_rest_field9070,
                            Box::new(source_ml_source2_nominal_rest_field9071.clone()),
                            Box::new(source_ml_source2_nominal_rest_field9072.clone()),
                            Box::new(source_ml_source2_nominal_rest_field9073.clone()),
                            source_ml_source2_nominal_rest_field9074.clone(),
                            source_ml_source2_nominal_rest_field9075.clone(),
                            source_ml_source2_nominal_rest_field9076.clone(),
                            source_ml_source2_nominal_rest_field9077.clone(),
                            source_ml_source2_nominal_rest_field9078.clone(),
                            source_ml_source2_nominal_rest_field9079.clone(),
                            source_ml_source2_nominal_rest_field9080.clone(),
                            None::<(Option<self::JavaScriptArrayConcatenationCall>,)>,
                        )
                    }
                };
                match base.clone() {
                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
                        shared_constructor_field28,
                        shared_constructor_field29,
                        shared_constructor_field30,
                        shared_constructor_field31,
                        shared_constructor_field32,
                        shared_constructor_field33,
                        shared_constructor_field34,
                        shared_constructor_field35,
                        shared_constructor_field36,
                        shared_constructor_field37,
                        shared_constructor_field38,
                        shared_constructor_field39,
                        shared_constructor_field40,
                        shared_constructor_field41,
                        shared_constructor_field42,
                        shared_constructor_field43,
                        shared_constructor_field44,
                        shared_constructor_field45,
                        shared_constructor_field46,
                        shared_constructor_field47,
                        shared_constructor_field48,
                        shared_constructor_field49,
                    ) => {
                        let shared_constructor_field28 = *shared_constructor_field28;
                        let shared_constructor_field30 = *shared_constructor_field30;
                        let shared_constructor_field31 = *shared_constructor_field31;
                        let shared_constructor_field33 = *shared_constructor_field33;
                        let shared_constructor_field39 = *shared_constructor_field39;
                        let shared_constructor_field40 = *shared_constructor_field40;
                        let shared_constructor_field41 = *shared_constructor_field41;
                        match transition.clone() {
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesContinue(
                                heap_2,
                                state_2,
                            ) => {
                                let heap_2 = *heap_2;
                                let state_2 = *state_2;
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap_2.clone()), Box::new({
                                    let source_ml_source2_nominal_field9083 = shared_constructor_field28.clone();
                                    {
                                        let source_ml_source2_nominal_field9085 = shared_constructor_field29.clone();
                                        {
                                            let source_ml_source2_nominal_field9087 = shared_constructor_field30.clone();
                                            {
                                                let source_ml_source2_nominal_field9089 = shared_constructor_field31.clone();
                                                {
                                                    let source_ml_source2_nominal_field9091 = shared_constructor_field32;
                                                    {
                                                        let source_ml_source2_nominal_field9093 = shared_constructor_field33.clone();
                                                        {
                                                            let source_ml_source2_nominal_field9095 = shared_constructor_field34;
                                                            {
                                                                let source_ml_source2_nominal_field9097 = shared_constructor_field35;
                                                                {
                                                                    let source_ml_source2_nominal_field9099 = shared_constructor_field36;
                                                                    {
                                                                        let source_ml_source2_nominal_field9101 = shared_constructor_field37;
                                                                        {
                                                                            let source_ml_source2_nominal_field9103 = shared_constructor_field38;
                                                                            {
                                                                                let source_ml_source2_nominal_field9105 = shared_constructor_field39.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field9107 = shared_constructor_field40.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9109 = shared_constructor_field41.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9111 = shared_constructor_field42.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9113 = shared_constructor_field43.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9115 = shared_constructor_field44.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9117 = shared_constructor_field45.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9119 = shared_constructor_field46.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9121 = shared_constructor_field47.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9123 = shared_constructor_field48.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9125 = shared_constructor_field49.clone();
                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9083.clone()), source_ml_source2_nominal_field9085.clone(), Box::new(source_ml_source2_nominal_field9087.clone()), Box::new(source_ml_source2_nominal_field9089.clone()), source_ml_source2_nominal_field9091, Box::new(source_ml_source2_nominal_field9093.clone()), source_ml_source2_nominal_field9095, source_ml_source2_nominal_field9097, source_ml_source2_nominal_field9099, source_ml_source2_nominal_field9101, source_ml_source2_nominal_field9103, Box::new(source_ml_source2_nominal_field9105.clone()), Box::new(source_ml_source2_nominal_field9107.clone()), Box::new(source_ml_source2_nominal_field9109.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), source_ml_source2_nominal_field9113.clone(), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field9117.clone(), source_ml_source2_nominal_field9119.clone(), source_ml_source2_nominal_field9121.clone(), source_ml_source2_nominal_field9123.clone(), source_ml_source2_nominal_field9125.clone())
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
                                        }
                                    }
                                }))
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesCall(
                                heap_3,
                                state_3,
                                callee,
                                receiver,
                                arguments,
                            ) => {
                                let heap_3 = *heap_3;
                                let state_3 = *state_3;
                                let callee = *callee;
                                let receiver = *receiver;
                                self::request_java_script_array_concatenation_call(
                                    heap_3.clone(),
                                    {
                                        let source_ml_source2_nominal_field9130 =
                                            shared_constructor_field28.clone();
                                        {
                                            let source_ml_source2_nominal_field9132 =
                                                shared_constructor_field29.clone();
                                            {
                                                let source_ml_source2_nominal_field9134 =
                                                    shared_constructor_field30.clone();
                                                {
                                                    let source_ml_source2_nominal_field9136 =
                                                        shared_constructor_field31.clone();
                                                    {
                                                        let source_ml_source2_nominal_field9138 =
                                                            shared_constructor_field32;
                                                        {
                                                            let source_ml_source2_nominal_field9140 =
                                                                shared_constructor_field33.clone();
                                                            {
                                                                let source_ml_source2_nominal_field9142 =
                                                                    shared_constructor_field34;
                                                                {
                                                                    let source_ml_source2_nominal_field9144 =
                                                                        shared_constructor_field35;
                                                                    {
                                                                        let source_ml_source2_nominal_field9146 = shared_constructor_field36;
                                                                        {
                                                                            let source_ml_source2_nominal_field9148 = shared_constructor_field37;
                                                                            {
                                                                                let source_ml_source2_nominal_field9150 = shared_constructor_field38;
                                                                                {
                                                                                    let source_ml_source2_nominal_field9152 = shared_constructor_field39.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9154 = shared_constructor_field40.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9156 = shared_constructor_field41.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9158 = shared_constructor_field42.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9160 = shared_constructor_field43.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9162 = shared_constructor_field44.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9164 = shared_constructor_field45.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9166 = shared_constructor_field46.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9168 = shared_constructor_field47.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9170 = shared_constructor_field48.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9172 = shared_constructor_field49.clone();
                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9130.clone()), source_ml_source2_nominal_field9132.clone(), Box::new(source_ml_source2_nominal_field9134.clone()), Box::new(source_ml_source2_nominal_field9136.clone()), source_ml_source2_nominal_field9138, Box::new(source_ml_source2_nominal_field9140.clone()), source_ml_source2_nominal_field9142, source_ml_source2_nominal_field9144, source_ml_source2_nominal_field9146, source_ml_source2_nominal_field9148, source_ml_source2_nominal_field9150, Box::new(source_ml_source2_nominal_field9152.clone()), Box::new(source_ml_source2_nominal_field9154.clone()), Box::new(source_ml_source2_nominal_field9156.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), source_ml_source2_nominal_field9160.clone(), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field9164.clone(), source_ml_source2_nominal_field9166.clone(), source_ml_source2_nominal_field9168.clone(), source_ml_source2_nominal_field9170.clone(), source_ml_source2_nominal_field9172.clone())
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
                                            }
                                        }
                                    },
                                    callee.clone(),
                                    receiver.clone(),
                                    arguments.clone(),
                                )
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesNeedAuthority(
                                heap_4,
                                state_4,
                                constructor,
                            ) => {
                                let heap_4 = *heap_4;
                                let state_4 = *state_4;
                                let constructor = *constructor;
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationNeedAuthority(Box::new(heap_4.clone()), Box::new({
                                    let source_ml_source2_nominal_field9177 = shared_constructor_field28.clone();
                                    {
                                        let source_ml_source2_nominal_field9179 = shared_constructor_field29.clone();
                                        {
                                            let source_ml_source2_nominal_field9181 = shared_constructor_field30.clone();
                                            {
                                                let source_ml_source2_nominal_field9183 = shared_constructor_field31.clone();
                                                {
                                                    let source_ml_source2_nominal_field9185 = shared_constructor_field32;
                                                    {
                                                        let source_ml_source2_nominal_field9187 = shared_constructor_field33.clone();
                                                        {
                                                            let source_ml_source2_nominal_field9189 = shared_constructor_field34;
                                                            {
                                                                let source_ml_source2_nominal_field9191 = shared_constructor_field35;
                                                                {
                                                                    let source_ml_source2_nominal_field9193 = shared_constructor_field36;
                                                                    {
                                                                        let source_ml_source2_nominal_field9195 = shared_constructor_field37;
                                                                        {
                                                                            let source_ml_source2_nominal_field9197 = shared_constructor_field38;
                                                                            {
                                                                                let source_ml_source2_nominal_field9199 = shared_constructor_field39.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field9201 = shared_constructor_field40.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9203 = shared_constructor_field41.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9205 = shared_constructor_field42.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9207 = shared_constructor_field43.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9209 = shared_constructor_field44.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9211 = shared_constructor_field45.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9213 = shared_constructor_field46.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9215 = shared_constructor_field47.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9217 = shared_constructor_field48.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9219 = shared_constructor_field49.clone();
                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9177.clone()), source_ml_source2_nominal_field9179.clone(), Box::new(source_ml_source2_nominal_field9181.clone()), Box::new(source_ml_source2_nominal_field9183.clone()), source_ml_source2_nominal_field9185, Box::new(source_ml_source2_nominal_field9187.clone()), source_ml_source2_nominal_field9189, source_ml_source2_nominal_field9191, source_ml_source2_nominal_field9193, source_ml_source2_nominal_field9195, source_ml_source2_nominal_field9197, Box::new(source_ml_source2_nominal_field9199.clone()), Box::new(source_ml_source2_nominal_field9201.clone()), Box::new(source_ml_source2_nominal_field9203.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), source_ml_source2_nominal_field9207.clone(), Some((Some(state_4.clone()),)), source_ml_source2_nominal_field9211.clone(), source_ml_source2_nominal_field9213.clone(), source_ml_source2_nominal_field9215.clone(), source_ml_source2_nominal_field9217.clone(), source_ml_source2_nominal_field9219.clone())
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
                                        }
                                    }
                                }), Box::new(constructor.clone()))
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesAllocate(
                                heap_5,
                                state_5,
                                prototype,
                                _,
                            ) => {
                                let heap_5 = *heap_5;
                                let state_5 = *state_5;
                                let prototype = *prototype;
                                self::request_java_script_array_concatenation_allocation(
                                    heap_5.clone(),
                                    {
                                        let source_ml_source2_nominal_field9224 =
                                            shared_constructor_field28.clone();
                                        {
                                            let source_ml_source2_nominal_field9226 =
                                                shared_constructor_field29.clone();
                                            {
                                                let source_ml_source2_nominal_field9228 =
                                                    shared_constructor_field30.clone();
                                                {
                                                    let source_ml_source2_nominal_field9230 =
                                                        shared_constructor_field31.clone();
                                                    {
                                                        let source_ml_source2_nominal_field9232 =
                                                            shared_constructor_field32;
                                                        {
                                                            let source_ml_source2_nominal_field9234 =
                                                                shared_constructor_field33.clone();
                                                            {
                                                                let source_ml_source2_nominal_field9236 =
                                                                    shared_constructor_field34;
                                                                {
                                                                    let source_ml_source2_nominal_field9238 =
                                                                        shared_constructor_field35;
                                                                    {
                                                                        let source_ml_source2_nominal_field9240 = shared_constructor_field36;
                                                                        {
                                                                            let source_ml_source2_nominal_field9242 = shared_constructor_field37;
                                                                            {
                                                                                let source_ml_source2_nominal_field9244 = shared_constructor_field38;
                                                                                {
                                                                                    let source_ml_source2_nominal_field9246 = shared_constructor_field39.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9248 = shared_constructor_field40.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9250 = shared_constructor_field41.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9252 = shared_constructor_field42.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9254 = shared_constructor_field43.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9256 = shared_constructor_field44.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9258 = shared_constructor_field45.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9260 = shared_constructor_field46.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9262 = shared_constructor_field47.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9264 = shared_constructor_field48.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9266 = shared_constructor_field49.clone();
                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9224.clone()), source_ml_source2_nominal_field9226.clone(), Box::new(source_ml_source2_nominal_field9228.clone()), Box::new(source_ml_source2_nominal_field9230.clone()), source_ml_source2_nominal_field9232, Box::new(source_ml_source2_nominal_field9234.clone()), source_ml_source2_nominal_field9236, source_ml_source2_nominal_field9238, source_ml_source2_nominal_field9240, source_ml_source2_nominal_field9242, source_ml_source2_nominal_field9244, Box::new(source_ml_source2_nominal_field9246.clone()), Box::new(source_ml_source2_nominal_field9248.clone()), Box::new(source_ml_source2_nominal_field9250.clone()), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), source_ml_source2_nominal_field9254.clone(), Some((Some(state_5.clone()),)), Some((Some(prototype.clone()),)), source_ml_source2_nominal_field9260.clone(), source_ml_source2_nominal_field9262.clone(), source_ml_source2_nominal_field9264.clone(), source_ml_source2_nominal_field9266.clone())
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
                                            }
                                        }
                                    },
                                )
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesConstruct(
                                heap_6,
                                state_6,
                                constructor_2,
                                arguments_2,
                                fact,
                            ) => {
                                let heap_6 = *heap_6;
                                let state_6 = *state_6;
                                let constructor_2 = *constructor_2;
                                let fact = *fact;
                                match fact.clone() {
                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                        _,
                                        _,
                                        shared_constructor_field26,
                                        shared_constructor_field27,
                                        _,
                                    ) => match state_6.clone() {
                                        self::JavaScriptArraySpeciesState::ArraySpeciesState(
                                            _,
                                            _,
                                            shared_constructor_field23,
                                            shared_constructor_field24,
                                            shared_constructor_field25,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let shared_constructor_field24 =
                                                *shared_constructor_field24;
                                            let shared_constructor_field25 =
                                                *shared_constructor_field25;
                                            {
                                                let selected = {
                                                    let source_ml_source2_nominal_field9272 =
                                                        shared_constructor_field28.clone();
                                                    {
                                                        let source_ml_source2_nominal_field9274 =
                                                            shared_constructor_field29.clone();
                                                        {
                                                            let source_ml_source2_nominal_field9276 =
                                                                shared_constructor_field30.clone();
                                                            {
                                                                let source_ml_source2_nominal_field9278 =
                                                                    shared_constructor_field31
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field9280 =
                                                                        shared_constructor_field32;
                                                                    {
                                                                        let source_ml_source2_nominal_field9282 = shared_constructor_field33.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field9284 = shared_constructor_field34;
                                                                            {
                                                                                let source_ml_source2_nominal_field9286 = shared_constructor_field35;
                                                                                {
                                                                                    let source_ml_source2_nominal_field9288 = shared_constructor_field36;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9290 = shared_constructor_field37;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9292 = shared_constructor_field38;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9294 = shared_constructor_field39.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9296 = shared_constructor_field40.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9298 = shared_constructor_field41.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9300 = shared_constructor_field42.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9302 = shared_constructor_field43.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9304 = shared_constructor_field44.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9306 = shared_constructor_field45.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9308 = shared_constructor_field46.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field9310 = shared_constructor_field47.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field9312 = shared_constructor_field48.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field9314 = shared_constructor_field49.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override9318 = self::copy_java_script_array_constructor_fact(fact.clone());
                                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9272.clone()), source_ml_source2_nominal_field9274.clone(), Box::new(source_ml_source2_nominal_field9276.clone()), Box::new(source_ml_source2_nominal_field9278.clone()), source_ml_source2_nominal_field9280, Box::new(source_ml_source2_nominal_field9282.clone()), source_ml_source2_nominal_field9284, source_ml_source2_nominal_field9286, source_ml_source2_nominal_field9288, source_ml_source2_nominal_field9290, source_ml_source2_nominal_field9292, Box::new(source_ml_source2_nominal_field9294.clone()), Box::new(source_ml_source2_nominal_field9296.clone()), Box::new(source_ml_source2_nominal_field9298.clone()), source_ml_source2_nominal_field9300.clone(), source_ml_source2_nominal_field9302.clone(), Some((Some(state_6.clone()),)), source_ml_source2_nominal_field9306.clone(), Some((Some(constructor_2.clone()),)), Some((Some(source_ml_source2_nominal_override9318.clone()),)), source_ml_source2_nominal_field9312.clone(), source_ml_source2_nominal_field9314.clone())
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
                                                            }
                                                        }
                                                    }
                                                };
                                                match selected.clone() {
                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11, shared_constructor_field12, shared_constructor_field13, shared_constructor_field14, shared_constructor_field15, shared_constructor_field16, shared_constructor_field17, shared_constructor_field18, shared_constructor_field19, shared_constructor_field20, shared_constructor_field21, shared_constructor_field22) => {
                                                            let shared_constructor_field1 = *shared_constructor_field1;
                                                            let shared_constructor_field3 = *shared_constructor_field3;
                                                            let shared_constructor_field4 = *shared_constructor_field4;
                                                            let shared_constructor_field6 = *shared_constructor_field6;
                                                            let shared_constructor_field12 = *shared_constructor_field12;
                                                            let shared_constructor_field13 = *shared_constructor_field13;
                                                            let shared_constructor_field14 = *shared_constructor_field14;
                                                            if ((shared_constructor_field27 && (shared_constructor_field26 == shared_constructor_field23)) && self::test_java_script_semantic_same_value(constructor_2.clone(), shared_constructor_field24.clone())) {
                                                                self::request_java_script_array_concatenation_allocation(heap_6.clone(), {
                                                                    let source_ml_source2_nominal_field9327 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field9329 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field9331 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field9333 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field9335 = shared_constructor_field5;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9337 = shared_constructor_field6.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9339 = shared_constructor_field7;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9341 = shared_constructor_field8;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9343 = shared_constructor_field9;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9345 = shared_constructor_field10;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9347 = shared_constructor_field11;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9349 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9351 = shared_constructor_field13.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9353 = shared_constructor_field14.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9355 = shared_constructor_field15.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field9357 = shared_constructor_field16.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field9359 = shared_constructor_field17.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field9361 = shared_constructor_field18.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field9363 = shared_constructor_field19.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field9365 = shared_constructor_field20.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field9367 = shared_constructor_field21.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field9369 = shared_constructor_field22.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_override9372 = shared_constructor_field25.clone();
                                                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9327.clone()), source_ml_source2_nominal_field9329.clone(), Box::new(source_ml_source2_nominal_field9331.clone()), Box::new(source_ml_source2_nominal_field9333.clone()), source_ml_source2_nominal_field9335, Box::new(source_ml_source2_nominal_field9337.clone()), source_ml_source2_nominal_field9339, source_ml_source2_nominal_field9341, source_ml_source2_nominal_field9343, source_ml_source2_nominal_field9345, source_ml_source2_nominal_field9347, Box::new(source_ml_source2_nominal_field9349.clone()), Box::new(source_ml_source2_nominal_field9351.clone()), Box::new(source_ml_source2_nominal_field9353.clone()), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), source_ml_source2_nominal_field9357.clone(), source_ml_source2_nominal_field9359.clone(), Some((Some(source_ml_source2_nominal_override9372.clone()),)), source_ml_source2_nominal_field9363.clone(), source_ml_source2_nominal_field9365.clone(), source_ml_source2_nominal_field9367.clone(), source_ml_source2_nominal_field9369.clone())
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
                                                                            }
                                                                        }
                                                                    }
                                                                })
                                                            } else {
                                                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationConstruct(Box::new(heap_6.clone()), Box::new({
                                                                    let source_ml_source2_nominal_field9374 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field9376 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field9378 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field9380 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field9382 = shared_constructor_field5;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9384 = shared_constructor_field6.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9386 = shared_constructor_field7;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9388 = shared_constructor_field8;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9390 = shared_constructor_field9;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9392 = shared_constructor_field10;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9394 = shared_constructor_field11;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9396 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9398 = shared_constructor_field13.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9400 = shared_constructor_field14.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9402 = shared_constructor_field15.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field9404 = shared_constructor_field16.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field9406 = shared_constructor_field17.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field9408 = shared_constructor_field18.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field9410 = shared_constructor_field19.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field9412 = shared_constructor_field20.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field9414 = shared_constructor_field21.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field9416 = shared_constructor_field22.clone();
                                                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9374.clone()), source_ml_source2_nominal_field9376.clone(), Box::new(source_ml_source2_nominal_field9378.clone()), Box::new(source_ml_source2_nominal_field9380.clone()), source_ml_source2_nominal_field9382, Box::new(source_ml_source2_nominal_field9384.clone()), source_ml_source2_nominal_field9386, source_ml_source2_nominal_field9388, source_ml_source2_nominal_field9390, source_ml_source2_nominal_field9392, source_ml_source2_nominal_field9394, Box::new(source_ml_source2_nominal_field9396.clone()), Box::new(source_ml_source2_nominal_field9398.clone()), Box::new(source_ml_source2_nominal_field9400.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]), source_ml_source2_nominal_field9404.clone(), source_ml_source2_nominal_field9406.clone(), source_ml_source2_nominal_field9408.clone(), source_ml_source2_nominal_field9410.clone(), source_ml_source2_nominal_field9412.clone(), source_ml_source2_nominal_field9414.clone(), source_ml_source2_nominal_field9416.clone())
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
                                                                        }
                                                                    }
                                                                }), Box::new(constructor_2.clone()), arguments_2.clone(), Box::new(self::copy_java_script_array_constructor_fact(fact.clone())))
                                                            }
                                                        }
                                                    }
                                            }
                                        }
                                    },
                                }
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesThrow(
                                heap_7,
                                value,
                            ) => {
                                let heap_7 = *heap_7;
                                let value = *value;
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationThrow(Box::new(heap_7.clone()), Box::new(value.clone()))
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesError(
                                heap_8,
                                error_class,
                                reason,
                            ) => {
                                let heap_8 = *heap_8;
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationError(Box::new(heap_8.clone()), error_class.clone(), reason.clone())
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesInvalid(
                                heap_9,
                                reason_2,
                            ) => {
                                let heap_9 = *heap_9;
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap_9.clone()), reason_2.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
