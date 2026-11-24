// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySliceCall;
use super::JavaScriptArraySliceState;
use super::JavaScriptArraySliceTransition;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptObjectSpreadTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::ml_string;
use super::read_java_script_array_slice_state_error;
use super::request_call;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_final_set(
    state: self::JavaScriptArraySliceState,
    transition: self::JavaScriptObjectSpreadTransition,
) -> self::JavaScriptArraySliceTransition {
    {
        let source_ml_source1_temporary_binding365 = state.clone();
        {
            let prior_call = match source_ml_source1_temporary_binding365.clone() {
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
                    source_ml_source2_data_property_value12149,
                ) => ((source_ml_source2_data_property_value12149
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,)))
                .0)
                    .clone(),
            };
            {
                let base = match source_ml_source1_temporary_binding365.clone() {
                    self::JavaScriptArraySliceState::ArraySliceState(
                        source_ml_source2_nominal_rest_field12150,
                        source_ml_source2_nominal_rest_field12151,
                        source_ml_source2_nominal_rest_field12152,
                        source_ml_source2_nominal_rest_field12153,
                        source_ml_source2_nominal_rest_field12154,
                        source_ml_source2_nominal_rest_field12155,
                        source_ml_source2_nominal_rest_field12156,
                        source_ml_source2_nominal_rest_field12157,
                        source_ml_source2_nominal_rest_field12158,
                        source_ml_source2_nominal_rest_field12159,
                        source_ml_source2_nominal_rest_field12160,
                        source_ml_source2_nominal_rest_field12161,
                        source_ml_source2_nominal_rest_field12162,
                        source_ml_source2_nominal_rest_field12163,
                        source_ml_source2_nominal_rest_field12164,
                        source_ml_source2_nominal_rest_field12165,
                        source_ml_source2_nominal_rest_field12166,
                        source_ml_source2_nominal_rest_field12167,
                        source_ml_source2_nominal_rest_field12168,
                        source_ml_source2_nominal_rest_field12169,
                        source_ml_source2_nominal_rest_field12170,
                        source_ml_source2_nominal_rest_field12171,
                        _,
                    ) => {
                        let source_ml_source2_nominal_rest_field12150 =
                            *source_ml_source2_nominal_rest_field12150;
                        let source_ml_source2_nominal_rest_field12151 =
                            *source_ml_source2_nominal_rest_field12151;
                        let source_ml_source2_nominal_rest_field12152 =
                            *source_ml_source2_nominal_rest_field12152;
                        let source_ml_source2_nominal_rest_field12154 =
                            *source_ml_source2_nominal_rest_field12154;
                        let source_ml_source2_nominal_rest_field12161 =
                            *source_ml_source2_nominal_rest_field12161;
                        let source_ml_source2_nominal_rest_field12162 =
                            *source_ml_source2_nominal_rest_field12162;
                        let source_ml_source2_nominal_rest_field12163 =
                            *source_ml_source2_nominal_rest_field12163;
                        self::JavaScriptArraySliceState::ArraySliceState(
                            Box::new(source_ml_source2_nominal_rest_field12150.clone()),
                            Box::new(source_ml_source2_nominal_rest_field12151.clone()),
                            Box::new(source_ml_source2_nominal_rest_field12152.clone()),
                            source_ml_source2_nominal_rest_field12153,
                            Box::new(source_ml_source2_nominal_rest_field12154.clone()),
                            source_ml_source2_nominal_rest_field12155,
                            source_ml_source2_nominal_rest_field12156,
                            source_ml_source2_nominal_rest_field12157,
                            source_ml_source2_nominal_rest_field12158,
                            source_ml_source2_nominal_rest_field12159,
                            source_ml_source2_nominal_rest_field12160,
                            Box::new(source_ml_source2_nominal_rest_field12161.clone()),
                            Box::new(source_ml_source2_nominal_rest_field12162.clone()),
                            Box::new(source_ml_source2_nominal_rest_field12163.clone()),
                            source_ml_source2_nominal_rest_field12164.clone(),
                            source_ml_source2_nominal_rest_field12165.clone(),
                            source_ml_source2_nominal_rest_field12166.clone(),
                            source_ml_source2_nominal_rest_field12167.clone(),
                            source_ml_source2_nominal_rest_field12168.clone(),
                            source_ml_source2_nominal_rest_field12169.clone(),
                            source_ml_source2_nominal_rest_field12170.clone(),
                            source_ml_source2_nominal_rest_field12171.clone(),
                            None::<(Option<self::JavaScriptArraySliceCall>,)>,
                        )
                    }
                };
                match base.clone() {
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
                        shared_constructor_field17,
                        shared_constructor_field18,
                        shared_constructor_field19,
                        shared_constructor_field20,
                        shared_constructor_field21,
                        shared_constructor_field22,
                        shared_constructor_field23,
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        let shared_constructor_field2 = *shared_constructor_field2;
                        let shared_constructor_field3 = *shared_constructor_field3;
                        let shared_constructor_field5 = *shared_constructor_field5;
                        let shared_constructor_field12 = *shared_constructor_field12;
                        let shared_constructor_field13 = *shared_constructor_field13;
                        let shared_constructor_field14 = *shared_constructor_field14;
                        match transition.clone() {
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(
                                heap,
                                state_2,
                            ) => {
                                let heap = *heap;
                                let state_2 = *state_2;
                                self::JavaScriptArraySliceTransition::ArraySliceContinue(
                                    Box::new(heap.clone()),
                                    Box::new({
                                        let source_ml_source2_nominal_field12174 =
                                            shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field12176 =
                                                shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field12178 =
                                                    shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field12180 =
                                                        shared_constructor_field4;
                                                    {
                                                        let source_ml_source2_nominal_field12182 =
                                                            shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field12184 =
                                                                shared_constructor_field6;
                                                            {
                                                                let source_ml_source2_nominal_field12186 =
                                                                    shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field12188 =
                                                                        shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field12190 = shared_constructor_field9;
                                                                        {
                                                                            let source_ml_source2_nominal_field12192 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field12194 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field12196 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12198 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12200 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12202 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12204 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12206 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12208 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12210 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12212 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12214 = shared_constructor_field21.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12216 = shared_constructor_field22.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12218 = shared_constructor_field23.clone();
                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12174.clone()), Box::new(source_ml_source2_nominal_field12176.clone()), Box::new(source_ml_source2_nominal_field12178.clone()), source_ml_source2_nominal_field12180, Box::new(source_ml_source2_nominal_field12182.clone()), source_ml_source2_nominal_field12184, source_ml_source2_nominal_field12186, source_ml_source2_nominal_field12188, source_ml_source2_nominal_field12190, source_ml_source2_nominal_field12192, source_ml_source2_nominal_field12194, Box::new(source_ml_source2_nominal_field12196.clone()), Box::new(source_ml_source2_nominal_field12198.clone()), Box::new(source_ml_source2_nominal_field12200.clone()), source_ml_source2_nominal_field12202.clone(), source_ml_source2_nominal_field12204.clone(), source_ml_source2_nominal_field12206.clone(), source_ml_source2_nominal_field12208.clone(), source_ml_source2_nominal_field12210.clone(), source_ml_source2_nominal_field12212.clone(), source_ml_source2_nominal_field12214.clone(), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field12218.clone())
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
                                    }),
                                )
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadCall(
                                heap_2,
                                state_3,
                                callee,
                                receiver,
                                arguments,
                            ) => {
                                let heap_2 = *heap_2;
                                let state_3 = *state_3;
                                let callee = *callee;
                                let receiver = *receiver;
                                self::request_call(
                                    heap_2.clone(),
                                    {
                                        let source_ml_source2_nominal_field12222 =
                                            shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field12224 =
                                                shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field12226 =
                                                    shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field12228 =
                                                        shared_constructor_field4;
                                                    {
                                                        let source_ml_source2_nominal_field12230 =
                                                            shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field12232 =
                                                                shared_constructor_field6;
                                                            {
                                                                let source_ml_source2_nominal_field12234 =
                                                                    shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field12236 =
                                                                        shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field12238 = shared_constructor_field9;
                                                                        {
                                                                            let source_ml_source2_nominal_field12240 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field12242 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field12244 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12246 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12248 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12250 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12252 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12254 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12256 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12258 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12260 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12262 = shared_constructor_field21.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12264 = shared_constructor_field22.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12266 = shared_constructor_field23.clone();
                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12222.clone()), Box::new(source_ml_source2_nominal_field12224.clone()), Box::new(source_ml_source2_nominal_field12226.clone()), source_ml_source2_nominal_field12228, Box::new(source_ml_source2_nominal_field12230.clone()), source_ml_source2_nominal_field12232, source_ml_source2_nominal_field12234, source_ml_source2_nominal_field12236, source_ml_source2_nominal_field12238, source_ml_source2_nominal_field12240, source_ml_source2_nominal_field12242, Box::new(source_ml_source2_nominal_field12244.clone()), Box::new(source_ml_source2_nominal_field12246.clone()), Box::new(source_ml_source2_nominal_field12248.clone()), source_ml_source2_nominal_field12250.clone(), source_ml_source2_nominal_field12252.clone(), source_ml_source2_nominal_field12254.clone(), source_ml_source2_nominal_field12256.clone(), source_ml_source2_nominal_field12258.clone(), source_ml_source2_nominal_field12260.clone(), source_ml_source2_nominal_field12262.clone(), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field12266.clone())
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
                                    },
                                    callee.clone(),
                                    receiver.clone(),
                                    arguments.clone(),
                                )
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadCompleted(
                                heap_3,
                                _,
                            ) => {
                                let heap_3 = *heap_3;
                                self::JavaScriptArraySliceTransition::ArraySliceCompleted(
                                    Box::new(heap_3.clone()),
                                    Box::new(match state.clone() {
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
                                            source_ml_source2_data_property_value12269,
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
                                        ) => {
                                            let source_ml_source2_data_property_value12269 =
                                                *source_ml_source2_data_property_value12269;
                                            source_ml_source2_data_property_value12269.clone()
                                        }
                                    }),
                                )
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadThrow(
                                heap_4,
                                value,
                            ) => {
                                let heap_4 = *heap_4;
                                let value = *value;
                                self::JavaScriptArraySliceTransition::ArraySliceThrow(
                                    Box::new(heap_4.clone()),
                                    Box::new(value.clone()),
                                )
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadError(
                                heap_5,
                                reason,
                                error_class,
                            ) => {
                                let heap_5 = *heap_5;
                                self::JavaScriptArraySliceTransition::ArraySliceError(
                                    Box::new(heap_5.clone()),
                                    (((error_class
                                        .clone()
                                        .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                    .0)
                                        .clone())
                                    .clone()
                                    .unwrap_or_else(|| {
                                        self::ml_string::Text::from_units(vec![
                                            84, 121, 112, 101, 69, 114, 114, 111, 114,
                                        ])
                                    }),
                                    reason.clone(),
                                )
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                                heap_6,
                                reason_2,
                            ) => {
                                let heap_6 = *heap_6;
                                self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                                    Box::new(heap_6.clone()),
                                    reason_2.clone(),
                                )
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadResourceLimit(
                                heap_7,
                                state_4,
                                reason_3,
                            ) => {
                                let heap_7 = *heap_7;
                                let state_4 = *state_4;
                                self::JavaScriptArraySliceTransition::ArraySliceResourceLimit(
                                    Box::new(heap_7.clone()),
                                    Box::new({
                                        let source_ml_source2_nominal_field12271 =
                                            shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field12273 =
                                                shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field12275 =
                                                    shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field12277 =
                                                        shared_constructor_field4;
                                                    {
                                                        let source_ml_source2_nominal_field12279 =
                                                            shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field12281 =
                                                                shared_constructor_field6;
                                                            {
                                                                let source_ml_source2_nominal_field12283 =
                                                                    shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field12285 =
                                                                        shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field12287 = shared_constructor_field9;
                                                                        {
                                                                            let source_ml_source2_nominal_field12289 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field12291 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field12293 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12295 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12297 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12299 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12301 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12303 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12305 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12307 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12309 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12311 = shared_constructor_field21.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12313 = shared_constructor_field22.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12315 = shared_constructor_field23.clone();
                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12271.clone()), Box::new(source_ml_source2_nominal_field12273.clone()), Box::new(source_ml_source2_nominal_field12275.clone()), source_ml_source2_nominal_field12277, Box::new(source_ml_source2_nominal_field12279.clone()), source_ml_source2_nominal_field12281, source_ml_source2_nominal_field12283, source_ml_source2_nominal_field12285, source_ml_source2_nominal_field12287, source_ml_source2_nominal_field12289, source_ml_source2_nominal_field12291, Box::new(source_ml_source2_nominal_field12293.clone()), Box::new(source_ml_source2_nominal_field12295.clone()), Box::new(source_ml_source2_nominal_field12297.clone()), source_ml_source2_nominal_field12299.clone(), source_ml_source2_nominal_field12301.clone(), source_ml_source2_nominal_field12303.clone(), source_ml_source2_nominal_field12305.clone(), source_ml_source2_nominal_field12307.clone(), source_ml_source2_nominal_field12309.clone(), source_ml_source2_nominal_field12311.clone(), Some((Some(state_4.clone()),)), source_ml_source2_nominal_field12315.clone())
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
                                    }),
                                    reason_3.clone(),
                                )
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
pub fn begin_java_script_array_slice(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    receiver: self::JavaScriptSemanticValue,
    start: self::JavaScriptSemanticValue,
    end: self::JavaScriptSemanticValue,
    end_present: bool,
    exotic_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptArraySliceTransition {
    {
        let state = {
            let source_ml_source2_constructor_field12323 = 0f64;
            {
                let source_ml_source2_constructor_field12324 = 0f64;
                {
                    let source_ml_source2_constructor_field12325 = 0f64;
                    {
                        let source_ml_source2_constructor_field12326 = 0f64;
                        {
                            let source_ml_source2_constructor_field12327 = 0f64;
                            {
                                let source_ml_source2_constructor_field12328 = 0f64;
                                {
                                    let source_ml_source2_constructor_field12329 =
                                        self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source2_constructor_field12330 =
                                            self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source2_constructor_field12331 =
                                                self::JavaScriptSemanticValue::Undefined;
                                            self::JavaScriptArraySliceState::ArraySliceState(
                                                Box::new(receiver.clone()),
                                                Box::new(start.clone()),
                                                Box::new(end.clone()),
                                                end_present,
                                                Box::new(exotic_key.clone()),
                                                source_ml_source2_constructor_field12323,
                                                source_ml_source2_constructor_field12324,
                                                source_ml_source2_constructor_field12325,
                                                source_ml_source2_constructor_field12326,
                                                source_ml_source2_constructor_field12327,
                                                source_ml_source2_constructor_field12328,
                                                Box::new(
                                                    source_ml_source2_constructor_field12329
                                                        .clone(),
                                                ),
                                                Box::new(
                                                    source_ml_source2_constructor_field12330
                                                        .clone(),
                                                ),
                                                Box::new(
                                                    source_ml_source2_constructor_field12331
                                                        .clone(),
                                                ),
                                                self::ml_string::Text::from_units(vec![
                                                    108, 101, 110, 103, 116, 104,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    108, 101, 110, 103, 116, 104,
                                                ]),
                                                None::<(
                                                    Option<
                                                        self::JavaScriptPrimitiveConversionState,
                                                    >,
                                                )>,
                                                None::<(Option<self::JavaScriptArraySpeciesState>,)>,
                                                None::<(Option<self::JavaScriptSemanticValue>,)>,
                                                None::<(Option<self::JavaScriptSemanticValue>,)>,
                                                None::<(
                                                    Option<self::JavaScriptArrayConstructorFact>,
                                                )>,
                                                None::<(Option<self::JavaScriptObjectSpreadState>,)>,
                                                None::<(Option<self::JavaScriptArraySliceCall>,)>,
                                            )
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
            let error = self::read_java_script_array_slice_state_error(
                heap.clone(),
                directory.clone(),
                state.clone(),
            );
            if (error == (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptArraySliceTransition::ArraySliceContinue(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                )
            } else {
                self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            }
        }
    }
}
