// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptInstanceOfCall;
use super::JavaScriptInstanceOfState;
use super::JavaScriptInstanceOfTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_instance_of_method(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptInstanceOfState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptInstanceOfTransition {
    match state.clone() {
        self::JavaScriptInstanceOfState::InstanceOfState(
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
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            {
                let right = (match state.clone() {
                    self::JavaScriptInstanceOfState::InstanceOfState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value26133,
                        _,
                    ) => ((source_ml_source2_data_property_value26133
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                    .0)
                        .clone(),
                })
                .clone()
                .unwrap_or_else(|| shared_constructor_field6.clone());
                if (match value.clone() {
                    self::JavaScriptSemanticValue::Null => true,
                    _ => false,
                } || match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => true,
                    _ => false,
                }) {
                    if (match right.clone() {
                        self::JavaScriptSemanticValue::Object(_, _) => false,
                        _ => true,
                    } || !(((match right.clone() {
                        self::JavaScriptSemanticValue::Object(
                            _,
                            source_ml_source2_data_property_value26137,
                        ) => Some((source_ml_source2_data_property_value26137,)),
                        _ => None::<(bool,)>,
                    })
                    .clone()
                    .expect("a proven present value"))
                    .0))
                    {
                        self::JavaScriptInstanceOfTransition::InstanceOfError(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                84, 121, 112, 101, 69, 114, 114, 111, 114,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                82, 105, 103, 104, 116, 45, 104, 97, 110, 100, 32, 115, 105, 100,
                                101, 32, 111, 102, 32, 105, 110, 115, 116, 97, 110, 99, 101, 111,
                                102, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98,
                                108, 101,
                            ]),
                        )
                    } else {
                        self::JavaScriptInstanceOfTransition::InstanceOfContinue(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field26139 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_nominal_field26141 =
                                        shared_constructor_field5.clone();
                                    {
                                        let source_ml_source2_nominal_field26143 =
                                            shared_constructor_field6.clone();
                                        {
                                            let source_ml_source2_nominal_field26145 =
                                                shared_constructor_field7.clone();
                                            {
                                                let source_ml_source2_nominal_field26147 =
                                                    shared_constructor_field8.clone();
                                                {
                                                    let source_ml_source2_nominal_field26149 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_nominal_field26151 =
                                                            shared_constructor_field10.clone();
                                                        {
                                                            let source_ml_source2_nominal_field26153 =
                                                                shared_constructor_field11.clone();
                                                            {
                                                                let source_ml_source2_nominal_field26155 =
                                                                    shared_constructor_field12
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field26157 =
                                                                        shared_constructor_field13
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field26159 = shared_constructor_field14.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_override26162 = None::<self::JavaScriptInstanceOfCall>;
                                                                            self::JavaScriptInstanceOfState::InstanceOfState(source_ml_source2_nominal_field26139.clone(), Box::new(source_ml_source2_nominal_field26141.clone()), Box::new(source_ml_source2_nominal_field26143.clone()), Box::new(source_ml_source2_nominal_field26145.clone()), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101]), Box::new(source_ml_source2_nominal_field26149.clone()), Box::new(source_ml_source2_nominal_field26151.clone()), source_ml_source2_nominal_field26153.clone(), Some((source_ml_source2_nominal_override26162.clone(),)), source_ml_source2_nominal_field26157.clone(), source_ml_source2_nominal_field26159.clone())
                                                                        }
                                                                    }
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
                    if (match value.clone() {
                        self::JavaScriptSemanticValue::Object(_, _) => false,
                        _ => true,
                    } || !(((match value.clone() {
                        self::JavaScriptSemanticValue::Object(
                            _,
                            source_ml_source2_data_property_value26165,
                        ) => Some((source_ml_source2_data_property_value26165,)),
                        _ => None::<(bool,)>,
                    })
                    .clone()
                    .expect("a proven present value"))
                    .0))
                    {
                        self::JavaScriptInstanceOfTransition::InstanceOfError(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                84, 121, 112, 101, 69, 114, 114, 111, 114,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                83, 121, 109, 98, 111, 108, 46, 104, 97, 115, 73, 110, 115, 116,
                                97, 110, 99, 101, 32, 109, 101, 116, 104, 111, 100, 32, 105, 115,
                                32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101,
                            ]),
                        )
                    } else {
                        {
                            let call = self::JavaScriptInstanceOfCall::InstanceOfCall(
                                Box::new(value.clone()),
                                Box::new(right.clone()),
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    shared_constructor_field5.clone(),
                                ])),
                            );
                            match call.clone() {
                                self::JavaScriptInstanceOfCall::InstanceOfCall(
                                    shared_constructor_field1,
                                    shared_constructor_field2,
                                    shared_constructor_field3,
                                ) => {
                                    let shared_constructor_field1 = *shared_constructor_field1;
                                    let shared_constructor_field2 = *shared_constructor_field2;
                                    self::JavaScriptInstanceOfTransition::InstanceOfCall(
                                        Box::new(heap.clone()),
                                        Box::new({
                                            let source_ml_source2_nominal_field26168 =
                                                shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field26170 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field26172 =
                                                        shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field26174 =
                                                            shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field26176 =
                                                                shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field26178 =
                                                                    shared_constructor_field9
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field26180 =
                                                                        shared_constructor_field10
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field26182 = shared_constructor_field11.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field26184 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field26186 = shared_constructor_field13.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field26188 = shared_constructor_field14.clone();
                                                                                    self::JavaScriptInstanceOfState::InstanceOfState(source_ml_source2_nominal_field26168.clone(), Box::new(source_ml_source2_nominal_field26170.clone()), Box::new(source_ml_source2_nominal_field26172.clone()), Box::new(source_ml_source2_nominal_field26174.clone()), self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 32, 118, 97, 108, 117, 101]), Box::new(source_ml_source2_nominal_field26178.clone()), Box::new(source_ml_source2_nominal_field26180.clone()), source_ml_source2_nominal_field26182.clone(), Some((Some(call.clone()),)), source_ml_source2_nominal_field26186.clone(), source_ml_source2_nominal_field26188.clone())
                                                                                }
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
                                        Box::new(shared_constructor_field1.clone()),
                                        Box::new(shared_constructor_field2.clone()),
                                        shared_constructor_field3.clone(),
                                    )
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
pub fn accept_java_script_instance_of_prototype(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptInstanceOfState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptInstanceOfTransition {
    match state.clone() {
        self::JavaScriptInstanceOfState::InstanceOfState(
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
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            match value.clone() {
                self::JavaScriptSemanticValue::Object(_, _) => {
                    self::JavaScriptInstanceOfTransition::InstanceOfContinue(
                        Box::new(heap.clone()),
                        Box::new({
                            let source_ml_source2_nominal_field26199 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_nominal_field26201 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_nominal_field26203 =
                                        shared_constructor_field3.clone();
                                    {
                                        let source_ml_source2_nominal_field26205 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field26207 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field26209 =
                                                    shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field26211 =
                                                        shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_nominal_field26213 =
                                                            shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field26215 =
                                                                shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_nominal_field26217 =
                                                                    shared_constructor_field10
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field26219 =
                                                                        shared_constructor_field11
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_override26223 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_override26224 = None::<self::JavaScriptInstanceOfCall>;
                                                                            self::JavaScriptInstanceOfState::InstanceOfState(source_ml_source2_nominal_field26199.clone(), Box::new(source_ml_source2_nominal_field26201.clone()), Box::new(source_ml_source2_nominal_field26203.clone()), Box::new(source_ml_source2_nominal_field26205.clone()), self::ml_string::Text::from_units(vec![119, 97, 108, 107]), Box::new(value.clone()), Box::new(source_ml_source2_nominal_override26223.clone()), source_ml_source2_nominal_field26213.clone(), Some((source_ml_source2_nominal_override26224.clone(),)), source_ml_source2_nominal_field26217.clone(), source_ml_source2_nominal_field26219.clone())
                                                                        }
                                                                    }
                                                                }
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
                _ => self::JavaScriptInstanceOfTransition::InstanceOfError(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        70, 117, 110, 99, 116, 105, 111, 110, 32, 104, 97, 115, 32, 110, 111, 110,
                        45, 111, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112,
                        101, 32, 105, 110, 32, 105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32,
                        99, 104, 101, 99, 107,
                    ]),
                ),
            }
        }
    }
}
