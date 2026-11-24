// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptIteratorCompletion;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorProtocolTransition;
use super::JavaScriptSemanticValue;
use super::accept_java_script_array_creation_iterator_step;
use super::clear_java_script_array_creation_child;
use super::fail_java_script_array_creation;
use super::ml_string;
use super::request_java_script_array_creation_call;
use super::throw_java_script_array_creation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_creation_iterator(
    state: self::JavaScriptArrayCreationState,
    transition: self::JavaScriptIteratorProtocolTransition,
) -> self::JavaScriptArrayCreationTransition {
    {
        let cleared = self::clear_java_script_array_creation_child(state.clone());
        match cleared.clone() {
            self::JavaScriptArrayCreationState::ArrayCreationState(
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
                shared_constructor_field50,
                shared_constructor_field51,
                shared_constructor_field52,
                shared_constructor_field53,
                shared_constructor_field54,
                shared_constructor_field55,
                shared_constructor_field56,
                shared_constructor_field57,
                shared_constructor_field58,
                shared_constructor_field59,
                shared_constructor_field60,
                shared_constructor_field61,
                shared_constructor_field62,
            ) => {
                let shared_constructor_field34 = *shared_constructor_field34;
                let shared_constructor_field36 = *shared_constructor_field36;
                let shared_constructor_field37 = *shared_constructor_field37;
                let shared_constructor_field38 = *shared_constructor_field38;
                let shared_constructor_field39 = *shared_constructor_field39;
                let shared_constructor_field40 = *shared_constructor_field40;
                let shared_constructor_field41 = *shared_constructor_field41;
                let shared_constructor_field47 = *shared_constructor_field47;
                let shared_constructor_field48 = *shared_constructor_field48;
                let shared_constructor_field49 = *shared_constructor_field49;
                let shared_constructor_field50 = *shared_constructor_field50;
                let shared_constructor_field51 = *shared_constructor_field51;
                {
                    let abrupt = match state.clone() {
                        self::JavaScriptArrayCreationState::ArrayCreationState(
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
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value14901,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value14901
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                        .0)
                            .clone(),
                    };
                    {
                        let base = if (((match state.clone() {
                            self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                source_ml_source2_data_property_value14903,
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
                            ) => source_ml_source2_data_property_value14903.clone(),
                        }) == (self::ml_string::Text::from_units(vec![
                            99, 108, 111, 115, 101,
                        ]))) && !(match (abrupt.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        })) {
                            {
                                let source_ml_source2_nominal_field14905 =
                                    shared_constructor_field33.clone();
                                {
                                    let source_ml_source2_nominal_field14907 =
                                        shared_constructor_field34.clone();
                                    {
                                        let source_ml_source2_nominal_field14909 =
                                            shared_constructor_field35.clone();
                                        {
                                            let source_ml_source2_nominal_field14911 =
                                                shared_constructor_field36.clone();
                                            {
                                                let source_ml_source2_nominal_field14913 =
                                                    shared_constructor_field37.clone();
                                                {
                                                    let source_ml_source2_nominal_field14915 =
                                                        shared_constructor_field38.clone();
                                                    {
                                                        let source_ml_source2_nominal_field14917 =
                                                            shared_constructor_field39.clone();
                                                        {
                                                            let source_ml_source2_nominal_field14919 =
                                                                shared_constructor_field40.clone();
                                                            {
                                                                let source_ml_source2_nominal_field14921 =
                                                                    shared_constructor_field41
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field14923 =
                                                                        shared_constructor_field42;
                                                                    {
                                                                        let source_ml_source2_nominal_field14925 = shared_constructor_field43;
                                                                        {
                                                                            let source_ml_source2_nominal_field14927 = shared_constructor_field44.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field14929 = shared_constructor_field45;
                                                                                {
                                                                                    let source_ml_source2_nominal_field14931 = shared_constructor_field46;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field14933 = shared_constructor_field47.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field14935 = shared_constructor_field48.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field14937 = shared_constructor_field49.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field14939 = shared_constructor_field50.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field14941 = shared_constructor_field51.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field14943 = shared_constructor_field52.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field14945 = shared_constructor_field53.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field14947 = shared_constructor_field54.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field14949 = shared_constructor_field55.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field14951 = shared_constructor_field56.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field14953 = shared_constructor_field57.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field14955 = shared_constructor_field58.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field14957 = shared_constructor_field59.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field14959 = shared_constructor_field60.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field14961 = shared_constructor_field61.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field14963 = shared_constructor_field62.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_override14965 = abrupt.clone().expect("a proven present value");
                                                                                                                                                        self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field14905.clone(), Box::new(source_ml_source2_nominal_field14907.clone()), source_ml_source2_nominal_field14909.clone(), Box::new(source_ml_source2_nominal_field14911.clone()), Box::new(source_ml_source2_nominal_field14913.clone()), Box::new(source_ml_source2_nominal_field14915.clone()), Box::new(source_ml_source2_nominal_field14917.clone()), Box::new(source_ml_source2_nominal_field14919.clone()), Box::new(source_ml_source2_nominal_field14921.clone()), source_ml_source2_nominal_field14923, source_ml_source2_nominal_field14925, source_ml_source2_nominal_field14927.clone(), source_ml_source2_nominal_field14929, source_ml_source2_nominal_field14931, Box::new(source_ml_source2_nominal_field14933.clone()), Box::new(source_ml_source2_nominal_field14935.clone()), Box::new(source_ml_source2_nominal_field14937.clone()), Box::new(source_ml_source2_nominal_field14939.clone()), Box::new(source_ml_source2_nominal_field14941.clone()), source_ml_source2_nominal_field14943.clone(), source_ml_source2_nominal_field14945.clone(), source_ml_source2_nominal_field14947.clone(), source_ml_source2_nominal_field14949.clone(), source_ml_source2_nominal_field14951.clone(), source_ml_source2_nominal_field14953.clone(), source_ml_source2_nominal_field14955.clone(), source_ml_source2_nominal_field14957.clone(), Some((Some(source_ml_source2_nominal_override14965.clone()),)), source_ml_source2_nominal_field14961.clone(), source_ml_source2_nominal_field14963.clone())
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                            cleared.clone()
                        };
                        match base.clone() {
                            self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                shared_constructor_field31,
                                shared_constructor_field32,
                            ) => {
                                let shared_constructor_field4 = *shared_constructor_field4;
                                let shared_constructor_field6 = *shared_constructor_field6;
                                let shared_constructor_field7 = *shared_constructor_field7;
                                let shared_constructor_field8 = *shared_constructor_field8;
                                let shared_constructor_field9 = *shared_constructor_field9;
                                let shared_constructor_field10 = *shared_constructor_field10;
                                let shared_constructor_field11 = *shared_constructor_field11;
                                let shared_constructor_field17 = *shared_constructor_field17;
                                let shared_constructor_field18 = *shared_constructor_field18;
                                let shared_constructor_field19 = *shared_constructor_field19;
                                let shared_constructor_field20 = *shared_constructor_field20;
                                let shared_constructor_field21 = *shared_constructor_field21;
                                match transition.clone() {
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(heap, state_2) => {
                                        let heap = *heap;
                                        let state_2 = *state_2;
                                        self::JavaScriptArrayCreationTransition::ArrayCreationContinue(Box::new(heap.clone()), Box::new({
                                            let source_ml_source2_nominal_field14971 = shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field14973 = shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field14975 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field14977 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field14979 = shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field14981 = shared_constructor_field8.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field14983 = shared_constructor_field9.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field14985 = shared_constructor_field10.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field14987 = shared_constructor_field11.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field14989 = shared_constructor_field12;
                                                                                {
                                                                                    let source_ml_source2_nominal_field14991 = shared_constructor_field13;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field14993 = shared_constructor_field14.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field14995 = shared_constructor_field15;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field14997 = shared_constructor_field16;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field14999 = shared_constructor_field17.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15001 = shared_constructor_field18.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15003 = shared_constructor_field19.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15005 = shared_constructor_field20.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15007 = shared_constructor_field21.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15009 = shared_constructor_field22.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15011 = shared_constructor_field23.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15013 = shared_constructor_field24.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15015 = shared_constructor_field25.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15017 = shared_constructor_field26.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15019 = shared_constructor_field27.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field15021 = shared_constructor_field28.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field15023 = shared_constructor_field29.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field15025 = shared_constructor_field30.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field15027 = shared_constructor_field31.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field15029 = shared_constructor_field32.clone();
                                                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field14971.clone(), Box::new(source_ml_source2_nominal_field14973.clone()), source_ml_source2_nominal_field14975.clone(), Box::new(source_ml_source2_nominal_field14977.clone()), Box::new(source_ml_source2_nominal_field14979.clone()), Box::new(source_ml_source2_nominal_field14981.clone()), Box::new(source_ml_source2_nominal_field14983.clone()), Box::new(source_ml_source2_nominal_field14985.clone()), Box::new(source_ml_source2_nominal_field14987.clone()), source_ml_source2_nominal_field14989, source_ml_source2_nominal_field14991, source_ml_source2_nominal_field14993.clone(), source_ml_source2_nominal_field14995, source_ml_source2_nominal_field14997, Box::new(source_ml_source2_nominal_field14999.clone()), Box::new(source_ml_source2_nominal_field15001.clone()), Box::new(source_ml_source2_nominal_field15003.clone()), Box::new(source_ml_source2_nominal_field15005.clone()), Box::new(source_ml_source2_nominal_field15007.clone()), source_ml_source2_nominal_field15009.clone(), source_ml_source2_nominal_field15011.clone(), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field15015.clone(), source_ml_source2_nominal_field15017.clone(), source_ml_source2_nominal_field15019.clone(), source_ml_source2_nominal_field15021.clone(), source_ml_source2_nominal_field15023.clone(), source_ml_source2_nominal_field15025.clone(), source_ml_source2_nominal_field15027.clone(), source_ml_source2_nominal_field15029.clone())
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
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
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(heap_2, state_3, callee, receiver, arguments) => {
                                        let heap_2 = *heap_2;
                                        let state_3 = *state_3;
                                        let callee = *callee;
                                        let receiver = *receiver;
                                        self::request_java_script_array_creation_call(heap_2.clone(), {
                                            let source_ml_source2_nominal_field15033 = shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field15035 = shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field15037 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15039 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15041 = shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field15043 = shared_constructor_field8.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field15045 = shared_constructor_field9.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field15047 = shared_constructor_field10.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field15049 = shared_constructor_field11.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field15051 = shared_constructor_field12;
                                                                                {
                                                                                    let source_ml_source2_nominal_field15053 = shared_constructor_field13;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15055 = shared_constructor_field14.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15057 = shared_constructor_field15;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15059 = shared_constructor_field16;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15061 = shared_constructor_field17.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15063 = shared_constructor_field18.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15065 = shared_constructor_field19.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15067 = shared_constructor_field20.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15069 = shared_constructor_field21.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15071 = shared_constructor_field22.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15073 = shared_constructor_field23.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15075 = shared_constructor_field24.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15077 = shared_constructor_field25.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15079 = shared_constructor_field26.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15081 = shared_constructor_field27.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field15083 = shared_constructor_field28.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field15085 = shared_constructor_field29.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field15087 = shared_constructor_field30.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field15089 = shared_constructor_field31.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field15091 = shared_constructor_field32.clone();
                                                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15033.clone(), Box::new(source_ml_source2_nominal_field15035.clone()), source_ml_source2_nominal_field15037.clone(), Box::new(source_ml_source2_nominal_field15039.clone()), Box::new(source_ml_source2_nominal_field15041.clone()), Box::new(source_ml_source2_nominal_field15043.clone()), Box::new(source_ml_source2_nominal_field15045.clone()), Box::new(source_ml_source2_nominal_field15047.clone()), Box::new(source_ml_source2_nominal_field15049.clone()), source_ml_source2_nominal_field15051, source_ml_source2_nominal_field15053, source_ml_source2_nominal_field15055.clone(), source_ml_source2_nominal_field15057, source_ml_source2_nominal_field15059, Box::new(source_ml_source2_nominal_field15061.clone()), Box::new(source_ml_source2_nominal_field15063.clone()), Box::new(source_ml_source2_nominal_field15065.clone()), Box::new(source_ml_source2_nominal_field15067.clone()), Box::new(source_ml_source2_nominal_field15069.clone()), source_ml_source2_nominal_field15071.clone(), source_ml_source2_nominal_field15073.clone(), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field15077.clone(), source_ml_source2_nominal_field15079.clone(), source_ml_source2_nominal_field15081.clone(), source_ml_source2_nominal_field15083.clone(), source_ml_source2_nominal_field15085.clone(), source_ml_source2_nominal_field15087.clone(), source_ml_source2_nominal_field15089.clone(), source_ml_source2_nominal_field15091.clone())
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
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
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(heap_3, _, _, _) => {
                                        let heap_3 = *heap_3;
                                        self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new(heap_3.clone()), self::ml_string::Text::from_units(vec![99, 97, 99, 104, 101, 100, 32, 65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 101, 97, 116, 32, 105, 116, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 109, 101, 116, 104, 111, 100, 32, 71, 101, 116]))
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolAcquired(heap_4, record) => {
                                        let heap_4 = *heap_4;
                                        let record = *record;
                                        self::JavaScriptArrayCreationTransition::ArrayCreationContinue(Box::new(heap_4.clone()), Box::new({
                                            let source_ml_source2_nominal_field15095 = shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field15097 = shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field15099 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15101 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15103 = shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field15105 = shared_constructor_field8.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field15107 = shared_constructor_field9.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field15109 = shared_constructor_field10.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field15111 = shared_constructor_field11.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field15113 = shared_constructor_field12;
                                                                                {
                                                                                    let source_ml_source2_nominal_field15115 = shared_constructor_field13;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15117 = shared_constructor_field14.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15119 = shared_constructor_field15;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15121 = shared_constructor_field16;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15123 = shared_constructor_field17.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15125 = shared_constructor_field18.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15127 = shared_constructor_field19.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15129 = shared_constructor_field20.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15131 = shared_constructor_field21.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15133 = shared_constructor_field22.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15135 = shared_constructor_field23.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15137 = shared_constructor_field24.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15139 = shared_constructor_field25.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15141 = shared_constructor_field26.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15143 = shared_constructor_field27.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field15145 = shared_constructor_field28.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field15147 = shared_constructor_field29.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field15149 = shared_constructor_field30.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field15151 = shared_constructor_field31.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field15153 = shared_constructor_field32.clone();
                                                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15095.clone(), Box::new(source_ml_source2_nominal_field15097.clone()), source_ml_source2_nominal_field15099.clone(), Box::new(source_ml_source2_nominal_field15101.clone()), Box::new(source_ml_source2_nominal_field15103.clone()), Box::new(source_ml_source2_nominal_field15105.clone()), Box::new(source_ml_source2_nominal_field15107.clone()), Box::new(source_ml_source2_nominal_field15109.clone()), Box::new(source_ml_source2_nominal_field15111.clone()), source_ml_source2_nominal_field15113, source_ml_source2_nominal_field15115, self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 101]), source_ml_source2_nominal_field15119, source_ml_source2_nominal_field15121, Box::new(source_ml_source2_nominal_field15123.clone()), Box::new(source_ml_source2_nominal_field15125.clone()), Box::new(source_ml_source2_nominal_field15127.clone()), Box::new(source_ml_source2_nominal_field15129.clone()), Box::new(source_ml_source2_nominal_field15131.clone()), source_ml_source2_nominal_field15133.clone(), Some((Some(record.clone()),)), source_ml_source2_nominal_field15137.clone(), source_ml_source2_nominal_field15139.clone(), source_ml_source2_nominal_field15141.clone(), source_ml_source2_nominal_field15143.clone(), source_ml_source2_nominal_field15145.clone(), source_ml_source2_nominal_field15147.clone(), source_ml_source2_nominal_field15149.clone(), source_ml_source2_nominal_field15151.clone(), source_ml_source2_nominal_field15153.clone())
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
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
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolStep(heap_5, record_2, done, value) => {
                                        let heap_5 = *heap_5;
                                        let record_2 = *record_2;
                                        let value = *value;
                                        self::accept_java_script_array_creation_iterator_step(heap_5.clone(), base.clone(), record_2.clone(), done, value.clone())
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(heap_6, _, completion) => {
                                        let heap_6 = *heap_6;
                                        let completion = *completion;
                                        match completion.clone() {
                                            self::JavaScriptIteratorCompletion::IteratorCompletion(shared_constructor_field1, shared_constructor_field2, _) => {
                                                let shared_constructor_field2 = *shared_constructor_field2;
                                                if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]))) {
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationThrow(Box::new(heap_6.clone()), Box::new(shared_constructor_field2.clone()))
                                                } else {
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new(heap_6.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 99, 108, 101, 97, 110, 117, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 116, 104, 114, 111, 119]))
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(heap_7, transition_state, value_2) => {
                                        let heap_7 = *heap_7;
                                        let transition_state = *transition_state;
                                        let value_2 = *value_2;
                                        if (((match state.clone() {
                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15161, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value15161.clone()
                                            }
                                        }) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) && !(match (match state.clone() {
                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15162, _, _) => {
                                                ((source_ml_source2_data_property_value15162.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                            self::JavaScriptArrayCreationTransition::ArrayCreationThrow(Box::new(heap_7.clone()), Box::new((match state.clone() {
                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15163, _, _) => {
                                                    ((source_ml_source2_data_property_value15163.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value")))
                                        } else {
                                            self::throw_java_script_array_creation(heap_7.clone(), if ((match state.clone() {
                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15165, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value15165.clone()
                                                }
                                            }) == (self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 101]))) {
                                                {
                                                    let source_ml_source2_nominal_field15168 = shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15170 = shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15172 = shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field15174 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field15176 = shared_constructor_field7.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field15178 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field15180 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field15182 = shared_constructor_field10.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field15184 = shared_constructor_field11.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15186 = shared_constructor_field12;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15188 = shared_constructor_field13;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15190 = shared_constructor_field14.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15192 = shared_constructor_field15;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15194 = shared_constructor_field16;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15196 = shared_constructor_field17.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15198 = shared_constructor_field18.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15200 = shared_constructor_field19.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15202 = shared_constructor_field20.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15204 = shared_constructor_field21.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15206 = shared_constructor_field22.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15208 = shared_constructor_field23.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15210 = shared_constructor_field24.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15212 = shared_constructor_field25.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field15214 = shared_constructor_field26.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field15216 = shared_constructor_field27.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field15218 = shared_constructor_field28.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field15220 = shared_constructor_field29.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field15222 = shared_constructor_field30.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field15224 = shared_constructor_field31.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field15226 = shared_constructor_field32.clone();
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_override15228 = match transition_state.clone() {
                                                                                                                                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value15166, _, _, _, _) => {
                                                                                                                                                                                    let source_ml_source2_data_property_value15166 = *source_ml_source2_data_property_value15166;
                                                                                                                                                                                    source_ml_source2_data_property_value15166.clone()
                                                                                                                                                                                }
                                                                                                                                                                            };
                                                                                                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15168.clone(), Box::new(source_ml_source2_nominal_field15170.clone()), source_ml_source2_nominal_field15172.clone(), Box::new(source_ml_source2_nominal_field15174.clone()), Box::new(source_ml_source2_nominal_field15176.clone()), Box::new(source_ml_source2_nominal_field15178.clone()), Box::new(source_ml_source2_nominal_field15180.clone()), Box::new(source_ml_source2_nominal_field15182.clone()), Box::new(source_ml_source2_nominal_field15184.clone()), source_ml_source2_nominal_field15186, source_ml_source2_nominal_field15188, source_ml_source2_nominal_field15190.clone(), source_ml_source2_nominal_field15192, source_ml_source2_nominal_field15194, Box::new(source_ml_source2_nominal_field15196.clone()), Box::new(source_ml_source2_nominal_field15198.clone()), Box::new(source_ml_source2_nominal_field15200.clone()), Box::new(source_ml_source2_nominal_field15202.clone()), Box::new(source_ml_source2_nominal_field15204.clone()), source_ml_source2_nominal_field15206.clone(), Some((Some(source_ml_source2_nominal_override15228.clone()),)), source_ml_source2_nominal_field15210.clone(), source_ml_source2_nominal_field15212.clone(), source_ml_source2_nominal_field15214.clone(), source_ml_source2_nominal_field15216.clone(), source_ml_source2_nominal_field15218.clone(), source_ml_source2_nominal_field15220.clone(), source_ml_source2_nominal_field15222.clone(), source_ml_source2_nominal_field15224.clone(), source_ml_source2_nominal_field15226.clone())
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
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
                                                base.clone()
                                            }, value_2.clone())
                                        }
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolError(heap_8, transition_state_2, error_class, reason) => {
                                        let heap_8 = *heap_8;
                                        let transition_state_2 = *transition_state_2;
                                        if (((match state.clone() {
                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15230, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value15230.clone()
                                            }
                                        }) == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) && !(match (match state.clone() {
                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15231, _, _) => {
                                                ((source_ml_source2_data_property_value15231.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                            self::JavaScriptArrayCreationTransition::ArrayCreationThrow(Box::new(heap_8.clone()), Box::new((match state.clone() {
                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15232, _, _) => {
                                                    ((source_ml_source2_data_property_value15232.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value")))
                                        } else {
                                            self::fail_java_script_array_creation(heap_8.clone(), if ((match state.clone() {
                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15234, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value15234.clone()
                                                }
                                            }) == (self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 101]))) {
                                                {
                                                    let source_ml_source2_nominal_field15237 = shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15239 = shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15241 = shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field15243 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field15245 = shared_constructor_field7.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field15247 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field15249 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field15251 = shared_constructor_field10.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field15253 = shared_constructor_field11.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15255 = shared_constructor_field12;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15257 = shared_constructor_field13;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15259 = shared_constructor_field14.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15261 = shared_constructor_field15;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15263 = shared_constructor_field16;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15265 = shared_constructor_field17.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15267 = shared_constructor_field18.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15269 = shared_constructor_field19.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15271 = shared_constructor_field20.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15273 = shared_constructor_field21.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15275 = shared_constructor_field22.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15277 = shared_constructor_field23.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15279 = shared_constructor_field24.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15281 = shared_constructor_field25.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field15283 = shared_constructor_field26.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field15285 = shared_constructor_field27.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field15287 = shared_constructor_field28.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field15289 = shared_constructor_field29.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field15291 = shared_constructor_field30.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field15293 = shared_constructor_field31.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field15295 = shared_constructor_field32.clone();
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_override15297 = match transition_state_2.clone() {
                                                                                                                                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value15235, _, _, _, _) => {
                                                                                                                                                                                    let source_ml_source2_data_property_value15235 = *source_ml_source2_data_property_value15235;
                                                                                                                                                                                    source_ml_source2_data_property_value15235.clone()
                                                                                                                                                                                }
                                                                                                                                                                            };
                                                                                                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15237.clone(), Box::new(source_ml_source2_nominal_field15239.clone()), source_ml_source2_nominal_field15241.clone(), Box::new(source_ml_source2_nominal_field15243.clone()), Box::new(source_ml_source2_nominal_field15245.clone()), Box::new(source_ml_source2_nominal_field15247.clone()), Box::new(source_ml_source2_nominal_field15249.clone()), Box::new(source_ml_source2_nominal_field15251.clone()), Box::new(source_ml_source2_nominal_field15253.clone()), source_ml_source2_nominal_field15255, source_ml_source2_nominal_field15257, source_ml_source2_nominal_field15259.clone(), source_ml_source2_nominal_field15261, source_ml_source2_nominal_field15263, Box::new(source_ml_source2_nominal_field15265.clone()), Box::new(source_ml_source2_nominal_field15267.clone()), Box::new(source_ml_source2_nominal_field15269.clone()), Box::new(source_ml_source2_nominal_field15271.clone()), Box::new(source_ml_source2_nominal_field15273.clone()), source_ml_source2_nominal_field15275.clone(), Some((Some(source_ml_source2_nominal_override15297.clone()),)), source_ml_source2_nominal_field15279.clone(), source_ml_source2_nominal_field15281.clone(), source_ml_source2_nominal_field15283.clone(), source_ml_source2_nominal_field15285.clone(), source_ml_source2_nominal_field15287.clone(), source_ml_source2_nominal_field15289.clone(), source_ml_source2_nominal_field15291.clone(), source_ml_source2_nominal_field15293.clone(), source_ml_source2_nominal_field15295.clone())
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
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
                                                base.clone()
                                            }, &(error_class), &(reason))
                                        }
                                    }
                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(heap_9, reason_2) => {
                                        let heap_9 = *heap_9;
                                        self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new(heap_9.clone()), reason_2.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
