// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayConcatenationState;
use super::JavaScriptArrayConcatenationTransition;
use super::JavaScriptIndexedPropertyCreation;
use super::JavaScriptObjectCopyCall;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::begin_java_script_scalar_protocol;
use super::continue_java_script_array_concatenation_length;
use super::continue_java_script_array_concatenation_spreadability;
use super::create_java_script_indexed_data_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::request_java_script_array_concatenation_call;
use super::step_java_script_array_concatenation_element;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_concatenation_items(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayConcatenationState,
) -> self::JavaScriptArrayConcatenationTransition {
    match state.clone() {
        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
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
        ) => {
            let shared_constructor_field23 = *shared_constructor_field23;
            let shared_constructor_field25 = *shared_constructor_field25;
            let shared_constructor_field26 = *shared_constructor_field26;
            let shared_constructor_field28 = *shared_constructor_field28;
            let shared_constructor_field34 = *shared_constructor_field34;
            let shared_constructor_field35 = *shared_constructor_field35;
            let shared_constructor_field36 = *shared_constructor_field36;
            if (shared_constructor_field37
                == (self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116])))
            {
                if (shared_constructor_field27
                    == ({
                        let ml_reference_array709 = shared_constructor_field24.clone();
                        let ml_reference_projection709 =
                            ml_reference_array709.borrow().len() as f64;
                        ml_reference_projection709
                    } + 1f64))
                {
                    {
                        let assignment = {
                            let source_ml_source2_constructor_field9957 =
                                shared_constructor_field34.clone();
                            {
                                let source_ml_source2_constructor_field9959 =
                                    shared_constructor_field23.clone();
                                {
                                    let source_ml_source2_constructor_field9960 =
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::JavaScriptSemanticValue,
                                        >::new(
                                        )));
                                    {
                                        let source_ml_source2_constructor_field9961 =
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                self::JavaScriptSemanticValue::Text(
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        vec![
                                                            108f64, 101f64, 110f64, 103f64, 116f64,
                                                            104f64,
                                                        ],
                                                    )),
                                                ),
                                            ]));
                                        {
                                            let source_ml_source2_constructor_field9962 = 0f64;
                                            {
                                                let source_ml_source2_constructor_field9965 =
                                                    self::JavaScriptSemanticValue::Number(
                                                        shared_constructor_field32,
                                                    );
                                                {
                                                    let source_ml_source2_constructor_field9966 =
                                                        self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source2_constructor_field9967 =
                                                            Some(self::ml_string::Text::from_units(
                                                                vec![
                                                                    97, 115, 115, 105, 103, 110,
                                                                    109, 101, 110, 116,
                                                                ],
                                                            ));
                                                        {
                                                            let source_ml_source2_constructor_field9969 =
                                                                Some(
                                                                    shared_constructor_field26
                                                                        .clone(),
                                                                );
                                                            self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_constructor_field9957.clone()), Box::new(source_ml_source2_constructor_field9959.clone()), source_ml_source2_constructor_field9960.clone(), source_ml_source2_constructor_field9961.clone(), source_ml_source2_constructor_field9962, self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), Box::new(source_ml_source2_constructor_field9965.clone()), Box::new(source_ml_source2_constructor_field9966.clone()), Some((source_ml_source2_constructor_field9967.clone(),)), Some((source_ml_source2_constructor_field9969.clone(),)), None::<(Option<self::JavaScriptArrayAssignmentState>,)>, None::<(Option<self::JavaScriptObjectCopyCall>,)>)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field9971 =
                                    shared_constructor_field23.clone();
                                {
                                    let source_ml_source2_nominal_field9973 =
                                        shared_constructor_field24.clone();
                                    {
                                        let source_ml_source2_nominal_field9975 =
                                            shared_constructor_field25.clone();
                                        {
                                            let source_ml_source2_nominal_field9977 =
                                                shared_constructor_field26.clone();
                                            {
                                                let source_ml_source2_nominal_field9979 =
                                                    shared_constructor_field27;
                                                {
                                                    let source_ml_source2_nominal_field9981 =
                                                        shared_constructor_field28.clone();
                                                    {
                                                        let source_ml_source2_nominal_field9983 =
                                                            shared_constructor_field29;
                                                        {
                                                            let source_ml_source2_nominal_field9985 =
                                                                shared_constructor_field30;
                                                            {
                                                                let source_ml_source2_nominal_field9987 =
                                                                    shared_constructor_field31;
                                                                {
                                                                    let source_ml_source2_nominal_field9989 =
                                                                        shared_constructor_field32;
                                                                    {
                                                                        let source_ml_source2_nominal_field9991 = shared_constructor_field33;
                                                                        {
                                                                            let source_ml_source2_nominal_field9993 = shared_constructor_field34.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field9995 = shared_constructor_field35.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field9997 = shared_constructor_field36.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9999 = shared_constructor_field37.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10001 = shared_constructor_field38.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10003 = shared_constructor_field39.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10005 = shared_constructor_field40.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10007 = shared_constructor_field41.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10009 = shared_constructor_field42.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10011 = shared_constructor_field43.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10013 = shared_constructor_field44.clone();
                                                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9971.clone()), source_ml_source2_nominal_field9973.clone(), Box::new(source_ml_source2_nominal_field9975.clone()), Box::new(source_ml_source2_nominal_field9977.clone()), source_ml_source2_nominal_field9979, Box::new(source_ml_source2_nominal_field9981.clone()), source_ml_source2_nominal_field9983, source_ml_source2_nominal_field9985, source_ml_source2_nominal_field9987, source_ml_source2_nominal_field9989, source_ml_source2_nominal_field9991, Box::new(source_ml_source2_nominal_field9993.clone()), Box::new(source_ml_source2_nominal_field9995.clone()), Box::new(source_ml_source2_nominal_field9997.clone()), self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104]), source_ml_source2_nominal_field10001.clone(), source_ml_source2_nominal_field10003.clone(), source_ml_source2_nominal_field10005.clone(), source_ml_source2_nominal_field10007.clone(), source_ml_source2_nominal_field10009.clone(), Some((Some(assignment.clone()),)), source_ml_source2_nominal_field10013.clone())
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
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
                } else {
                    {
                        let item = if (shared_constructor_field27 == (0f64)) {
                            shared_constructor_field23.clone()
                        } else {
                            ({
                                let source_ml_source2_array_read_value10022 =
                                    shared_constructor_field24.clone();
                                {
                                    let source_ml_source2_array_read_index10023 =
                                        (shared_constructor_field27 - 1f64);
                                    {
                                        let ml_reference_array710 =
                                            source_ml_source2_array_read_value10022.clone();
                                        let ml_reference_position710 =
                                            source_ml_source2_array_read_index10023;
                                        let ml_reference_projection710 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array710.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position710,
                                                ),
                                            );
                                        ml_reference_projection710
                                    }
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)
                        };
                        {
                            let selected = {
                                let source_ml_source2_nominal_field10026 =
                                    shared_constructor_field23.clone();
                                {
                                    let source_ml_source2_nominal_field10028 =
                                        shared_constructor_field24.clone();
                                    {
                                        let source_ml_source2_nominal_field10030 =
                                            shared_constructor_field25.clone();
                                        {
                                            let source_ml_source2_nominal_field10032 =
                                                shared_constructor_field26.clone();
                                            {
                                                let source_ml_source2_nominal_field10034 =
                                                    shared_constructor_field27;
                                                {
                                                    let source_ml_source2_nominal_field10036 =
                                                        shared_constructor_field28.clone();
                                                    {
                                                        let source_ml_source2_nominal_field10038 =
                                                            shared_constructor_field29;
                                                        {
                                                            let source_ml_source2_nominal_field10040 =
                                                                shared_constructor_field30;
                                                            {
                                                                let source_ml_source2_nominal_field10042 =
                                                                    shared_constructor_field31;
                                                                {
                                                                    let source_ml_source2_nominal_field10044 =
                                                                        shared_constructor_field32;
                                                                    {
                                                                        let source_ml_source2_nominal_field10046 = shared_constructor_field33;
                                                                        {
                                                                            let source_ml_source2_nominal_field10048 = shared_constructor_field34.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field10050 = shared_constructor_field35.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field10052 = shared_constructor_field36.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10054 = shared_constructor_field37.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10056 = shared_constructor_field38.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10058 = shared_constructor_field39.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10060 = shared_constructor_field40.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10062 = shared_constructor_field41.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10064 = shared_constructor_field42.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10066 = shared_constructor_field43.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10068 = shared_constructor_field44.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override10071 = shared_constructor_field32;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override10072 = 0f64;
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override10073 = 0f64;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_override10075 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override10076 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10026.clone()), source_ml_source2_nominal_field10028.clone(), Box::new(source_ml_source2_nominal_field10030.clone()), Box::new(source_ml_source2_nominal_field10032.clone()), source_ml_source2_nominal_field10034, Box::new(item.clone()), source_ml_source2_nominal_override10071, source_ml_source2_nominal_override10072, source_ml_source2_nominal_override10073, source_ml_source2_nominal_field10044, false, Box::new(source_ml_source2_nominal_field10048.clone()), Box::new(source_ml_source2_nominal_override10075.clone()), Box::new(source_ml_source2_nominal_override10076.clone()), source_ml_source2_nominal_field10054.clone(), source_ml_source2_nominal_field10056.clone(), source_ml_source2_nominal_field10058.clone(), source_ml_source2_nominal_field10060.clone(), source_ml_source2_nominal_field10062.clone(), source_ml_source2_nominal_field10064.clone(), source_ml_source2_nominal_field10066.clone(), source_ml_source2_nominal_field10068.clone())
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
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
                                    match item.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap.clone()), Box::new({
                                                let source_ml_source2_nominal_field10080 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_nominal_field10082 = shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source2_nominal_field10084 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field10086 = shared_constructor_field4.clone();
                                                            {
                                                                let source_ml_source2_nominal_field10088 = shared_constructor_field5;
                                                                {
                                                                    let source_ml_source2_nominal_field10090 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field10092 = shared_constructor_field7;
                                                                        {
                                                                            let source_ml_source2_nominal_field10094 = shared_constructor_field8;
                                                                            {
                                                                                let source_ml_source2_nominal_field10096 = shared_constructor_field9;
                                                                                {
                                                                                    let source_ml_source2_nominal_field10098 = shared_constructor_field10;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10100 = shared_constructor_field11;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10102 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10104 = shared_constructor_field13.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10106 = shared_constructor_field14.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10108 = shared_constructor_field15.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10110 = shared_constructor_field16.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10112 = shared_constructor_field17.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10114 = shared_constructor_field18.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field10116 = shared_constructor_field19.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field10118 = shared_constructor_field20.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field10120 = shared_constructor_field21.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field10122 = shared_constructor_field22.clone();
                                                                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10080.clone()), source_ml_source2_nominal_field10082.clone(), Box::new(source_ml_source2_nominal_field10084.clone()), Box::new(source_ml_source2_nominal_field10086.clone()), source_ml_source2_nominal_field10088, Box::new(source_ml_source2_nominal_field10090.clone()), source_ml_source2_nominal_field10092, source_ml_source2_nominal_field10094, source_ml_source2_nominal_field10096, source_ml_source2_nominal_field10098, source_ml_source2_nominal_field10100, Box::new(source_ml_source2_nominal_field10102.clone()), Box::new(source_ml_source2_nominal_field10104.clone()), Box::new(source_ml_source2_nominal_field10106.clone()), self::ml_string::Text::from_units(vec![115, 112, 114, 101, 97, 100, 97, 98, 105, 108, 105, 116, 121]), source_ml_source2_nominal_field10110.clone(), source_ml_source2_nominal_field10112.clone(), source_ml_source2_nominal_field10114.clone(), source_ml_source2_nominal_field10116.clone(), source_ml_source2_nominal_field10118.clone(), source_ml_source2_nominal_field10120.clone(), source_ml_source2_nominal_field10122.clone())
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
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
                                        _ => {
                                            if (shared_constructor_field32 >= (9007199254740991f64)) {
                                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 109, 97, 120, 105, 109, 117, 109, 32, 115, 97, 102, 101, 32, 111, 117, 116, 112, 117, 116, 32, 108, 101, 110, 103, 116, 104]))
                                            } else {
                                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap.clone()), Box::new({
                                                    let source_ml_source2_nominal_field10128 = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field10130 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field10132 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field10134 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field10136 = shared_constructor_field5;
                                                                    {
                                                                        let source_ml_source2_nominal_field10138 = shared_constructor_field6.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field10140 = shared_constructor_field7;
                                                                            {
                                                                                let source_ml_source2_nominal_field10142 = shared_constructor_field8;
                                                                                {
                                                                                    let source_ml_source2_nominal_field10144 = shared_constructor_field9;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10146 = shared_constructor_field10;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10148 = shared_constructor_field11;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10150 = shared_constructor_field12.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10152 = shared_constructor_field13.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10154 = shared_constructor_field14.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10156 = shared_constructor_field15.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10158 = shared_constructor_field16.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10160 = shared_constructor_field17.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field10162 = shared_constructor_field18.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field10164 = shared_constructor_field19.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field10166 = shared_constructor_field20.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field10168 = shared_constructor_field21.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field10170 = shared_constructor_field22.clone();
                                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10128.clone()), source_ml_source2_nominal_field10130.clone(), Box::new(source_ml_source2_nominal_field10132.clone()), Box::new(source_ml_source2_nominal_field10134.clone()), source_ml_source2_nominal_field10136, Box::new(source_ml_source2_nominal_field10138.clone()), source_ml_source2_nominal_field10140, source_ml_source2_nominal_field10142, source_ml_source2_nominal_field10144, source_ml_source2_nominal_field10146, source_ml_source2_nominal_field10148, Box::new(source_ml_source2_nominal_field10150.clone()), Box::new(item.clone()), Box::new(source_ml_source2_nominal_field10154.clone()), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), source_ml_source2_nominal_field10158.clone(), source_ml_source2_nominal_field10160.clone(), source_ml_source2_nominal_field10162.clone(), source_ml_source2_nominal_field10164.clone(), source_ml_source2_nominal_field10166.clone(), source_ml_source2_nominal_field10168.clone(), source_ml_source2_nominal_field10170.clone())
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
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
            } else {
                if ((shared_constructor_field37
                    == (self::ml_string::Text::from_units(vec![
                        115, 112, 114, 101, 97, 100, 97, 98, 105, 108, 105, 116, 121,
                    ])))
                    || (shared_constructor_field37
                        == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]))))
                {
                    if {
                        let source_ml_source2_s10179 = shared_constructor_field28.clone();
                        match source_ml_source2_s10179.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        }
                    } {
                        self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32,
                                112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 97, 100, 115,
                                32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32,
                                79, 98, 106, 101, 99, 116, 32, 105, 116, 101, 109, 115,
                            ]),
                        )
                    } else {
                        {
                            let spreadability = (shared_constructor_field37
                                == (self::ml_string::Text::from_units(vec![
                                    115, 112, 114, 101, 97, 100, 97, 98, 105, 108, 105, 116, 121,
                                ])));
                            {
                                let get = self::plan_java_script_ordinary_get(
                                    heap.clone(),
                                    (((match shared_constructor_field28.clone() {
                                        self::JavaScriptSemanticValue::Object(
                                            source_ml_source2_data_property_value10186,
                                            _,
                                        ) => Some((
                                            source_ml_source2_data_property_value10186.clone(),
                                        )),
                                        _ => None::<(self::ml::Big,)>,
                                    })
                                    .clone()
                                    .expect("a proven present value"))
                                    .0)
                                        .clone(),
                                    if spreadability {
                                        shared_constructor_field25.clone()
                                    } else {
                                        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                            std::cell::RefCell::new(vec![
                                                108f64, 101f64, 110f64, 103f64, 116f64, 104f64,
                                            ]),
                                        ))
                                    },
                                    shared_constructor_field28.clone(),
                                );
                                if (((({
                                    let ml_reference_value711 = get.borrow().clone();
                                    ml_reference_value711
                                })
                                .0)
                                    .3)
                                    != (self::ml_string::Text::from_units(vec![])))
                                {
                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), (({ let ml_reference_value712 = get.borrow().clone(); ml_reference_value712 }).0).3)
                                } else {
                                    if (((({
                                        let ml_reference_value713 = get.borrow().clone();
                                        ml_reference_value713
                                    })
                                    .0)
                                        .0)
                                        == (self::ml_string::Text::from_units(vec![
                                            99, 97, 108, 108,
                                        ])))
                                    {
                                        self::request_java_script_array_concatenation_call(
                                            heap.clone(),
                                            {
                                                let source_ml_source2_nominal_field10190 =
                                                    shared_constructor_field23.clone();
                                                {
                                                    let source_ml_source2_nominal_field10192 =
                                                        shared_constructor_field24.clone();
                                                    {
                                                        let source_ml_source2_nominal_field10194 =
                                                            shared_constructor_field25.clone();
                                                        {
                                                            let source_ml_source2_nominal_field10196 =
                                                                shared_constructor_field26.clone();
                                                            {
                                                                let source_ml_source2_nominal_field10198 =
                                                                    shared_constructor_field27;
                                                                {
                                                                    let source_ml_source2_nominal_field10200 =
                                                                        shared_constructor_field28
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field10202 = shared_constructor_field29;
                                                                        {
                                                                            let source_ml_source2_nominal_field10204 = shared_constructor_field30;
                                                                            {
                                                                                let source_ml_source2_nominal_field10206 = shared_constructor_field31;
                                                                                {
                                                                                    let source_ml_source2_nominal_field10208 = shared_constructor_field32;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10210 = shared_constructor_field33;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10212 = shared_constructor_field34.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10214 = shared_constructor_field35.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10216 = shared_constructor_field36.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10218 = shared_constructor_field37.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10220 = shared_constructor_field38.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10222 = shared_constructor_field39.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10224 = shared_constructor_field40.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field10226 = shared_constructor_field41.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field10228 = shared_constructor_field42.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field10230 = shared_constructor_field43.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field10232 = shared_constructor_field44.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override10234 = if spreadability {
                                                                                                                                        self::ml_string::Text::from_units(vec![115, 112, 114, 101, 97, 100, 97, 98, 105, 108, 105, 116, 121, 32, 103, 101, 116, 116, 101, 114])
                                                                                                                                    } else {
                                                                                                                                        self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114])
                                                                                                                                    };
                                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10190.clone()), source_ml_source2_nominal_field10192.clone(), Box::new(source_ml_source2_nominal_field10194.clone()), Box::new(source_ml_source2_nominal_field10196.clone()), source_ml_source2_nominal_field10198, Box::new(source_ml_source2_nominal_field10200.clone()), source_ml_source2_nominal_field10202, source_ml_source2_nominal_field10204, source_ml_source2_nominal_field10206, source_ml_source2_nominal_field10208, source_ml_source2_nominal_field10210, Box::new(source_ml_source2_nominal_field10212.clone()), Box::new(source_ml_source2_nominal_field10214.clone()), Box::new(source_ml_source2_nominal_field10216.clone()), source_ml_source2_nominal_override10234.clone(), source_ml_source2_nominal_field10220.clone(), source_ml_source2_nominal_field10222.clone(), source_ml_source2_nominal_field10224.clone(), source_ml_source2_nominal_field10226.clone(), source_ml_source2_nominal_field10228.clone(), source_ml_source2_nominal_field10230.clone(), source_ml_source2_nominal_field10232.clone())
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
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
                                            (({
                                                let ml_reference_value714 = get.borrow().clone();
                                                ml_reference_value714
                                            })
                                            .0)
                                                .2,
                                            (({
                                                let ml_reference_value715 = get.borrow().clone();
                                                ml_reference_value715
                                            })
                                            .0)
                                                .4,
                                            (({
                                                let ml_reference_value716 = get.borrow().clone();
                                                ml_reference_value716
                                            })
                                            .0)
                                                .1,
                                        )
                                    } else {
                                        if (((({
                                            let ml_reference_value717 = get.borrow().clone();
                                            ml_reference_value717
                                        })
                                        .0)
                                            .0)
                                            != (self::ml_string::Text::from_units(vec![
                                                118, 97, 108, 117, 101,
                                            ])))
                                        {
                                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 114, 101, 97, 100, 97, 98, 105, 108, 105, 116, 121, 32, 111, 114, 32, 108, 101, 110, 103, 116, 104, 32, 71, 101, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                        } else {
                                            if spreadability {
                                                self::continue_java_script_array_concatenation_spreadability(heap.clone(), state.clone(), (({ let ml_reference_value718 = get.borrow().clone(); ml_reference_value718 }).0).5)
                                            } else {
                                                self::continue_java_script_array_concatenation_length(heap.clone(), {
                                                    let source_ml_source2_nominal_field10236 = shared_constructor_field23.clone();
                                                    {
                                                        let source_ml_source2_nominal_field10238 = shared_constructor_field24.clone();
                                                        {
                                                            let source_ml_source2_nominal_field10240 = shared_constructor_field25.clone();
                                                            {
                                                                let source_ml_source2_nominal_field10242 = shared_constructor_field26.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field10244 = shared_constructor_field27;
                                                                    {
                                                                        let source_ml_source2_nominal_field10246 = shared_constructor_field28.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field10248 = shared_constructor_field29;
                                                                            {
                                                                                let source_ml_source2_nominal_field10250 = shared_constructor_field30;
                                                                                {
                                                                                    let source_ml_source2_nominal_field10252 = shared_constructor_field31;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10254 = shared_constructor_field32;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10256 = shared_constructor_field33;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10258 = shared_constructor_field34.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10260 = shared_constructor_field35.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10262 = shared_constructor_field36.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10264 = shared_constructor_field37.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10266 = shared_constructor_field38.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10268 = shared_constructor_field39.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field10270 = shared_constructor_field40.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field10272 = shared_constructor_field41.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field10274 = shared_constructor_field42.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field10276 = shared_constructor_field43.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field10278 = shared_constructor_field44.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override10280 = (({ let ml_reference_value719 = get.borrow().clone(); ml_reference_value719 }).0).5;
                                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10236.clone()), source_ml_source2_nominal_field10238.clone(), Box::new(source_ml_source2_nominal_field10240.clone()), Box::new(source_ml_source2_nominal_field10242.clone()), source_ml_source2_nominal_field10244, Box::new(source_ml_source2_nominal_field10246.clone()), source_ml_source2_nominal_field10248, source_ml_source2_nominal_field10250, source_ml_source2_nominal_field10252, source_ml_source2_nominal_field10254, source_ml_source2_nominal_field10256, Box::new(source_ml_source2_nominal_field10258.clone()), Box::new(source_ml_source2_nominal_field10260.clone()), Box::new(source_ml_source2_nominal_override10280.clone()), source_ml_source2_nominal_field10264.clone(), source_ml_source2_nominal_field10266.clone(), source_ml_source2_nominal_field10268.clone(), source_ml_source2_nominal_field10270.clone(), source_ml_source2_nominal_field10272.clone(), source_ml_source2_nominal_field10274.clone(), source_ml_source2_nominal_field10276.clone(), source_ml_source2_nominal_field10278.clone())
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }, self::begin_java_script_scalar_protocol(heap.clone(), (({ let ml_reference_value720 = get.borrow().clone(); ml_reference_value720 }).0).5, shared_constructor_field26.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if (shared_constructor_field37
                        == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])))
                    {
                        {
                            let written = self::create_java_script_indexed_data_property(
                                heap.clone(),
                                shared_constructor_field34.clone(),
                                shared_constructor_field32,
                                shared_constructor_field35.clone(),
                            );
                            match written.clone() {
                                self::JavaScriptIndexedPropertyCreation::IndexedPropertyCreated(heap_2) => {
                                    let heap_2 = *heap_2;
                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap_2.clone()), Box::new({
                                        let source_ml_source2_nominal_field10294 = shared_constructor_field23.clone();
                                        {
                                            let source_ml_source2_nominal_field10296 = shared_constructor_field24.clone();
                                            {
                                                let source_ml_source2_nominal_field10298 = shared_constructor_field25.clone();
                                                {
                                                    let source_ml_source2_nominal_field10300 = shared_constructor_field26.clone();
                                                    {
                                                        let source_ml_source2_nominal_field10302 = shared_constructor_field27;
                                                        {
                                                            let source_ml_source2_nominal_field10304 = shared_constructor_field28.clone();
                                                            {
                                                                let source_ml_source2_nominal_field10306 = shared_constructor_field29;
                                                                {
                                                                    let source_ml_source2_nominal_field10308 = shared_constructor_field30;
                                                                    {
                                                                        let source_ml_source2_nominal_field10310 = shared_constructor_field31;
                                                                        {
                                                                            let source_ml_source2_nominal_field10312 = shared_constructor_field32;
                                                                            {
                                                                                let source_ml_source2_nominal_field10314 = shared_constructor_field33;
                                                                                {
                                                                                    let source_ml_source2_nominal_field10316 = shared_constructor_field34.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10318 = shared_constructor_field35.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10320 = shared_constructor_field36.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10322 = shared_constructor_field37.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10324 = shared_constructor_field38.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10326 = shared_constructor_field39.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10328 = shared_constructor_field40.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10330 = shared_constructor_field41.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10332 = shared_constructor_field42.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field10334 = shared_constructor_field43.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field10336 = shared_constructor_field44.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override10338 = if shared_constructor_field33 {
                                                                                                                                    self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121])
                                                                                                                                } else {
                                                                                                                                    self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116])
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_override10339 = if shared_constructor_field33 {
                                                                                                                                        (shared_constructor_field31 + 1f64)
                                                                                                                                    } else {
                                                                                                                                        0f64
                                                                                                                                    };
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override10340 = (shared_constructor_field27 + if shared_constructor_field33 {
                                                                                                                                            0f64
                                                                                                                                        } else {
                                                                                                                                            1f64
                                                                                                                                        });
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override10341 = (shared_constructor_field32 + 1f64);
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_override10342 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10294.clone()), source_ml_source2_nominal_field10296.clone(), Box::new(source_ml_source2_nominal_field10298.clone()), Box::new(source_ml_source2_nominal_field10300.clone()), source_ml_source2_nominal_override10340, Box::new(source_ml_source2_nominal_field10304.clone()), source_ml_source2_nominal_field10306, source_ml_source2_nominal_field10308, source_ml_source2_nominal_override10339, source_ml_source2_nominal_override10341, source_ml_source2_nominal_field10314, Box::new(source_ml_source2_nominal_field10316.clone()), Box::new(source_ml_source2_nominal_override10342.clone()), Box::new(source_ml_source2_nominal_field10320.clone()), source_ml_source2_nominal_override10338.clone(), source_ml_source2_nominal_field10324.clone(), source_ml_source2_nominal_field10326.clone(), source_ml_source2_nominal_field10328.clone(), source_ml_source2_nominal_field10330.clone(), source_ml_source2_nominal_field10332.clone(), source_ml_source2_nominal_field10334.clone(), source_ml_source2_nominal_field10336.clone())
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
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
                                self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(heap_3, reason) => {
                                    let heap_3 = *heap_3;
                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationError(Box::new(heap_3.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), reason.clone())
                                }
                                self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(heap_4, reason_2) => {
                                    let heap_4 = *heap_4;
                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap_4.clone()), reason_2.clone())
                                }
                                self::JavaScriptIndexedPropertyCreation::IndexedPropertyResourceLimit(heap_5, reason_3) => {
                                    let heap_5 = *heap_5;
                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationResourceLimit(Box::new(heap_5.clone()), Box::new(state.clone()), reason_3.clone())
                                }
                            }
                        }
                    } else {
                        self::step_java_script_array_concatenation_element(
                            heap.clone(),
                            state.clone(),
                        )
                    }
                }
            }
        }
    }
}
