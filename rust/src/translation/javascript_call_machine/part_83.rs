// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::clear_java_script_array_creation_child;
use super::fail_java_script_array_creation;
use super::ml_string;
use super::read_java_script_array_like_length;
use super::request_java_script_array_creation_call;
use super::throw_java_script_array_creation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_creation_length(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
    transition: self::JavaScriptScalarConversionTransition,
) -> self::JavaScriptArrayCreationTransition {
    {
        let base = self::clear_java_script_array_creation_child(state.clone());
        match base.clone() {
            self::JavaScriptArrayCreationState::ArrayCreationState(
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
                shared_constructor_field24,
                shared_constructor_field25,
                shared_constructor_field26,
                shared_constructor_field27,
                shared_constructor_field28,
                shared_constructor_field29,
                shared_constructor_field30,
            ) => {
                let shared_constructor_field2 = *shared_constructor_field2;
                let shared_constructor_field4 = *shared_constructor_field4;
                let shared_constructor_field5 = *shared_constructor_field5;
                let shared_constructor_field6 = *shared_constructor_field6;
                let shared_constructor_field7 = *shared_constructor_field7;
                let shared_constructor_field8 = *shared_constructor_field8;
                let shared_constructor_field9 = *shared_constructor_field9;
                let shared_constructor_field15 = *shared_constructor_field15;
                let shared_constructor_field16 = *shared_constructor_field16;
                let shared_constructor_field17 = *shared_constructor_field17;
                let shared_constructor_field18 = *shared_constructor_field18;
                let shared_constructor_field19 = *shared_constructor_field19;
                match transition.clone() {
                    self::JavaScriptScalarConversionTransition::ScalarContinuation(state_2) => {
                        let state_2 = *state_2;
                        self::JavaScriptArrayCreationTransition::ArrayCreationContinue(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field16140 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_nominal_field16142 =
                                        shared_constructor_field2.clone();
                                    {
                                        let source_ml_source2_nominal_field16144 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_nominal_field16146 =
                                                shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field16148 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field16150 =
                                                        shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field16152 =
                                                            shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field16154 =
                                                                shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16156 =
                                                                    shared_constructor_field9
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field16158 =
                                                                        shared_constructor_field10;
                                                                    {
                                                                        let source_ml_source2_nominal_field16160 = shared_constructor_field11;
                                                                        {
                                                                            let source_ml_source2_nominal_field16162 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field16164 = shared_constructor_field13;
                                                                                {
                                                                                    let source_ml_source2_nominal_field16166 = shared_constructor_field14;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16168 = shared_constructor_field15.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16170 = shared_constructor_field16.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16172 = shared_constructor_field17.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16174 = shared_constructor_field18.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16176 = shared_constructor_field19.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16178 = shared_constructor_field20.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16180 = shared_constructor_field21.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16182 = shared_constructor_field22.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16184 = shared_constructor_field23.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field16186 = shared_constructor_field24.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field16188 = shared_constructor_field25.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16190 = shared_constructor_field26.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16192 = shared_constructor_field27.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16194 = shared_constructor_field28.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16196 = shared_constructor_field29.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field16198 = shared_constructor_field30.clone();
                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16140.clone(), Box::new(source_ml_source2_nominal_field16142.clone()), source_ml_source2_nominal_field16144.clone(), Box::new(source_ml_source2_nominal_field16146.clone()), Box::new(source_ml_source2_nominal_field16148.clone()), Box::new(source_ml_source2_nominal_field16150.clone()), Box::new(source_ml_source2_nominal_field16152.clone()), Box::new(source_ml_source2_nominal_field16154.clone()), Box::new(source_ml_source2_nominal_field16156.clone()), source_ml_source2_nominal_field16158, source_ml_source2_nominal_field16160, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), source_ml_source2_nominal_field16164, source_ml_source2_nominal_field16166, Box::new(source_ml_source2_nominal_field16168.clone()), Box::new(source_ml_source2_nominal_field16170.clone()), Box::new(source_ml_source2_nominal_field16172.clone()), Box::new(source_ml_source2_nominal_field16174.clone()), Box::new(source_ml_source2_nominal_field16176.clone()), source_ml_source2_nominal_field16178.clone(), source_ml_source2_nominal_field16180.clone(), source_ml_source2_nominal_field16182.clone(), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field16186.clone(), source_ml_source2_nominal_field16188.clone(), source_ml_source2_nominal_field16190.clone(), source_ml_source2_nominal_field16192.clone(), source_ml_source2_nominal_field16194.clone(), source_ml_source2_nominal_field16196.clone(), source_ml_source2_nominal_field16198.clone())
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
                    self::JavaScriptScalarConversionTransition::ScalarCall(
                        state_3,
                        callee,
                        receiver,
                        arguments,
                    ) => {
                        let state_3 = *state_3;
                        let callee = *callee;
                        let receiver = *receiver;
                        self::request_java_script_array_creation_call(
                            heap.clone(),
                            {
                                let source_ml_source2_nominal_field16203 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_nominal_field16205 =
                                        shared_constructor_field2.clone();
                                    {
                                        let source_ml_source2_nominal_field16207 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_nominal_field16209 =
                                                shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field16211 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field16213 =
                                                        shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field16215 =
                                                            shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field16217 =
                                                                shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16219 =
                                                                    shared_constructor_field9
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field16221 =
                                                                        shared_constructor_field10;
                                                                    {
                                                                        let source_ml_source2_nominal_field16223 = shared_constructor_field11;
                                                                        {
                                                                            let source_ml_source2_nominal_field16225 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field16227 = shared_constructor_field13;
                                                                                {
                                                                                    let source_ml_source2_nominal_field16229 = shared_constructor_field14;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16231 = shared_constructor_field15.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16233 = shared_constructor_field16.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16235 = shared_constructor_field17.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16237 = shared_constructor_field18.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16239 = shared_constructor_field19.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16241 = shared_constructor_field20.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16243 = shared_constructor_field21.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16245 = shared_constructor_field22.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16247 = shared_constructor_field23.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field16249 = shared_constructor_field24.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field16251 = shared_constructor_field25.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16253 = shared_constructor_field26.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16255 = shared_constructor_field27.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16257 = shared_constructor_field28.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16259 = shared_constructor_field29.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field16261 = shared_constructor_field30.clone();
                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16203.clone(), Box::new(source_ml_source2_nominal_field16205.clone()), source_ml_source2_nominal_field16207.clone(), Box::new(source_ml_source2_nominal_field16209.clone()), Box::new(source_ml_source2_nominal_field16211.clone()), Box::new(source_ml_source2_nominal_field16213.clone()), Box::new(source_ml_source2_nominal_field16215.clone()), Box::new(source_ml_source2_nominal_field16217.clone()), Box::new(source_ml_source2_nominal_field16219.clone()), source_ml_source2_nominal_field16221, source_ml_source2_nominal_field16223, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), source_ml_source2_nominal_field16227, source_ml_source2_nominal_field16229, Box::new(source_ml_source2_nominal_field16231.clone()), Box::new(source_ml_source2_nominal_field16233.clone()), Box::new(source_ml_source2_nominal_field16235.clone()), Box::new(source_ml_source2_nominal_field16237.clone()), Box::new(source_ml_source2_nominal_field16239.clone()), source_ml_source2_nominal_field16241.clone(), source_ml_source2_nominal_field16243.clone(), source_ml_source2_nominal_field16245.clone(), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field16249.clone(), source_ml_source2_nominal_field16251.clone(), source_ml_source2_nominal_field16253.clone(), source_ml_source2_nominal_field16255.clone(), source_ml_source2_nominal_field16257.clone(), source_ml_source2_nominal_field16259.clone(), source_ml_source2_nominal_field16261.clone())
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
                    self::JavaScriptScalarConversionTransition::ScalarCompleted(primitive) => {
                        let primitive = *primitive;
                        match primitive.clone() {
                            self::JavaScriptSemanticValue::Number(value) => {
                                self::JavaScriptArrayCreationTransition::ArrayCreationContinue(
                                    Box::new(heap.clone()),
                                    Box::new({
                                        let source_ml_source2_nominal_field16266 =
                                            shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field16268 =
                                                shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field16270 =
                                                    shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field16272 =
                                                        shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field16274 =
                                                            shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field16276 =
                                                                shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16278 =
                                                                    shared_constructor_field7
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field16280 =
                                                                        shared_constructor_field8
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field16282 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field16284 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field16286 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field16288 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16290 = shared_constructor_field13;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16292 = shared_constructor_field14;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16294 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16296 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16298 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16300 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16302 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16304 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16306 = shared_constructor_field21.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field16308 = shared_constructor_field22.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field16310 = shared_constructor_field23.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16312 = shared_constructor_field24.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16314 = shared_constructor_field25.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16316 = shared_constructor_field26.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16318 = shared_constructor_field27.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field16320 = shared_constructor_field28.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field16322 = shared_constructor_field29.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field16324 = shared_constructor_field30.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_override16326 = self::read_java_script_array_like_length(value);
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_override16327 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16266.clone(), Box::new(source_ml_source2_nominal_field16268.clone()), source_ml_source2_nominal_field16270.clone(), Box::new(source_ml_source2_nominal_field16272.clone()), Box::new(source_ml_source2_nominal_field16274.clone()), Box::new(source_ml_source2_nominal_field16276.clone()), Box::new(source_ml_source2_nominal_field16278.clone()), Box::new(source_ml_source2_nominal_field16280.clone()), Box::new(source_ml_source2_nominal_field16282.clone()), source_ml_source2_nominal_field16284, source_ml_source2_nominal_field16286, self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]), source_ml_source2_nominal_field16290, source_ml_source2_nominal_override16326, Box::new(source_ml_source2_nominal_field16294.clone()), Box::new(source_ml_source2_nominal_field16296.clone()), Box::new(source_ml_source2_nominal_field16298.clone()), Box::new(source_ml_source2_nominal_field16300.clone()), Box::new(source_ml_source2_nominal_override16327.clone()), source_ml_source2_nominal_field16304.clone(), source_ml_source2_nominal_field16306.clone(), source_ml_source2_nominal_field16308.clone(), source_ml_source2_nominal_field16310.clone(), source_ml_source2_nominal_field16312.clone(), source_ml_source2_nominal_field16314.clone(), source_ml_source2_nominal_field16316.clone(), source_ml_source2_nominal_field16318.clone(), source_ml_source2_nominal_field16320.clone(), source_ml_source2_nominal_field16322.clone(), source_ml_source2_nominal_field16324.clone())
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
                            _ => self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 84, 111, 76,
                                    101, 110, 103, 116, 104, 32, 114, 101, 116, 97, 105, 110, 115,
                                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111,
                                    109, 112, 108, 101, 116, 101, 100, 32, 78, 117, 109, 98, 101,
                                    114,
                                ]),
                            ),
                        }
                    }
                    self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                        let value_2 = *value_2;
                        self::throw_java_script_array_creation(
                            heap.clone(),
                            base.clone(),
                            value_2.clone(),
                        )
                    }
                    self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(
                        error_class,
                        reason,
                    ) => self::fail_java_script_array_creation(
                        heap.clone(),
                        base.clone(),
                        &(error_class),
                        &(reason),
                    ),
                    self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                        self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                            Box::new(heap.clone()),
                            reason_2.clone(),
                        )
                    }
                    self::JavaScriptScalarConversionTransition::ScalarResourceLimit(
                        value_3,
                        reason_3,
                    ) => {
                        let value_3 = *value_3;
                        self::JavaScriptArrayCreationTransition::ArrayCreationResourceLimit(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field16330 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_nominal_field16332 =
                                        shared_constructor_field2.clone();
                                    {
                                        let source_ml_source2_nominal_field16334 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_nominal_field16336 =
                                                shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field16338 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field16340 =
                                                        shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field16342 =
                                                            shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field16344 =
                                                                shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16346 =
                                                                    shared_constructor_field9
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field16348 =
                                                                        shared_constructor_field10;
                                                                    {
                                                                        let source_ml_source2_nominal_field16350 = shared_constructor_field11;
                                                                        {
                                                                            let source_ml_source2_nominal_field16352 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field16354 = shared_constructor_field13;
                                                                                {
                                                                                    let source_ml_source2_nominal_field16356 = shared_constructor_field14;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16358 = shared_constructor_field15.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16360 = shared_constructor_field16.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16362 = shared_constructor_field17.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16364 = shared_constructor_field18.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16366 = shared_constructor_field19.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16368 = shared_constructor_field20.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16370 = shared_constructor_field21.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16372 = shared_constructor_field22.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16374 = shared_constructor_field23.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field16376 = shared_constructor_field24.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field16378 = shared_constructor_field25.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16380 = shared_constructor_field26.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16382 = shared_constructor_field27.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16384 = shared_constructor_field28.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16386 = shared_constructor_field29.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field16388 = shared_constructor_field30.clone();
                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16330.clone(), Box::new(source_ml_source2_nominal_field16332.clone()), source_ml_source2_nominal_field16334.clone(), Box::new(source_ml_source2_nominal_field16336.clone()), Box::new(source_ml_source2_nominal_field16338.clone()), Box::new(source_ml_source2_nominal_field16340.clone()), Box::new(source_ml_source2_nominal_field16342.clone()), Box::new(source_ml_source2_nominal_field16344.clone()), Box::new(source_ml_source2_nominal_field16346.clone()), source_ml_source2_nominal_field16348, source_ml_source2_nominal_field16350, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), source_ml_source2_nominal_field16354, source_ml_source2_nominal_field16356, Box::new(source_ml_source2_nominal_field16358.clone()), Box::new(source_ml_source2_nominal_field16360.clone()), Box::new(source_ml_source2_nominal_field16362.clone()), Box::new(source_ml_source2_nominal_field16364.clone()), Box::new(source_ml_source2_nominal_field16366.clone()), source_ml_source2_nominal_field16368.clone(), source_ml_source2_nominal_field16370.clone(), source_ml_source2_nominal_field16372.clone(), source_ml_source2_nominal_field16374.clone(), Some((Some(value_3.clone()),)), source_ml_source2_nominal_field16378.clone(), source_ml_source2_nominal_field16380.clone(), source_ml_source2_nominal_field16382.clone(), source_ml_source2_nominal_field16384.clone(), source_ml_source2_nominal_field16386.clone(), source_ml_source2_nominal_field16388.clone())
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
