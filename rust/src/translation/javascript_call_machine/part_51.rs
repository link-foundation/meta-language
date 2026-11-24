// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayAssignmentTransition;
use super::JavaScriptArrayConcatenationCall;
use super::JavaScriptObjectCopyCall;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptObjectSpreadTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::begin_java_script_array_assignment;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_set_with_array_definition;
use super::test_java_script_reflection_property_budget;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_object_assignment_property(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectSpreadState,
) -> self::JavaScriptObjectSpreadTransition {
    match state.clone() {
        self::JavaScriptObjectSpreadState::ObjectSpreadState(
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
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(
                Box::new(heap.clone()),
                Box::new({
                    let source_ml_source2_nominal_field8306 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_nominal_field8308 = shared_constructor_field2.clone();
                        {
                            let source_ml_source2_nominal_field8310 =
                                shared_constructor_field3.clone();
                            {
                                let source_ml_source2_nominal_field8312 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_nominal_field8314 =
                                        shared_constructor_field5;
                                    {
                                        let source_ml_source2_nominal_field8316 =
                                            shared_constructor_field6.clone();
                                        {
                                            let source_ml_source2_nominal_field8318 =
                                                shared_constructor_field7.clone();
                                            {
                                                let source_ml_source2_nominal_field8320 =
                                                    shared_constructor_field8.clone();
                                                {
                                                    let source_ml_source2_nominal_field8322 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_nominal_field8324 =
                                                            shared_constructor_field10.clone();
                                                        {
                                                            let source_ml_source2_nominal_field8326 =
                                                                shared_constructor_field11.clone();
                                                            {
                                                                let source_ml_source2_nominal_field8328 =
                                                                    shared_constructor_field12
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_override8331 =
                                                                        (shared_constructor_field5
                                                                            + 1f64);
                                                                    {
                                                                        let source_ml_source2_nominal_override8332 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source2_nominal_override8333 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source2_nominal_override8334 = None::<self::JavaScriptObjectCopyCall>;
                                                                                {
                                                                                    let source_ml_source2_nominal_override8335 = None::<self::JavaScriptArrayAssignmentState>;
                                                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field8306.clone()), Box::new(source_ml_source2_nominal_field8308.clone()), source_ml_source2_nominal_field8310.clone(), source_ml_source2_nominal_field8312.clone(), source_ml_source2_nominal_override8331, self::ml_string::Text::from_units(vec![114, 101, 97, 100]), Box::new(source_ml_source2_nominal_override8332.clone()), Box::new(source_ml_source2_nominal_override8333.clone()), source_ml_source2_nominal_field8322.clone(), source_ml_source2_nominal_field8324.clone(), Some((source_ml_source2_nominal_override8335.clone(),)), Some((source_ml_source2_nominal_override8334.clone(),)))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_object_assignment_array(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectSpreadState,
    transition: self::JavaScriptArrayAssignmentTransition,
) -> self::JavaScriptObjectSpreadTransition {
    match state.clone() {
        self::JavaScriptObjectSpreadState::ObjectSpreadState(
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
        ) => {
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            match transition.clone() {
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentContinue(
                    heap_2,
                    transition_state,
                ) => {
                    let heap_2 = *heap_2;
                    let transition_state = *transition_state;
                    self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(
                        Box::new(heap_2.clone()),
                        Box::new({
                            let source_ml_source2_nominal_field8338 =
                                shared_constructor_field4.clone();
                            {
                                let source_ml_source2_nominal_field8340 =
                                    shared_constructor_field5.clone();
                                {
                                    let source_ml_source2_nominal_field8342 =
                                        shared_constructor_field6.clone();
                                    {
                                        let source_ml_source2_nominal_field8344 =
                                            shared_constructor_field7.clone();
                                        {
                                            let source_ml_source2_nominal_field8346 =
                                                shared_constructor_field8;
                                            {
                                                let source_ml_source2_nominal_field8348 =
                                                    shared_constructor_field9.clone();
                                                {
                                                    let source_ml_source2_nominal_field8350 =
                                                        shared_constructor_field10.clone();
                                                    {
                                                        let source_ml_source2_nominal_field8352 =
                                                            shared_constructor_field11.clone();
                                                        {
                                                            let source_ml_source2_nominal_field8354 =
                                                                shared_constructor_field12.clone();
                                                            {
                                                                let source_ml_source2_nominal_field8356 =
                                                                    shared_constructor_field13
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field8358 =
                                                                        shared_constructor_field14
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field8360 = shared_constructor_field15.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_override8364 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source2_nominal_override8365 = None::<self::JavaScriptObjectCopyCall>;
                                                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field8338.clone()), Box::new(source_ml_source2_nominal_field8340.clone()), source_ml_source2_nominal_field8342.clone(), source_ml_source2_nominal_field8344.clone(), source_ml_source2_nominal_field8346, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]), Box::new(source_ml_source2_nominal_field8350.clone()), Box::new(source_ml_source2_nominal_override8364.clone()), source_ml_source2_nominal_field8354.clone(), source_ml_source2_nominal_field8356.clone(), Some((Some(transition_state.clone()),)), Some((source_ml_source2_nominal_override8365.clone(),)))
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCall(
                    heap_3,
                    transition_state_2,
                    callee,
                    receiver,
                    arguments,
                ) => {
                    let heap_3 = *heap_3;
                    let transition_state_2 = *transition_state_2;
                    let callee = *callee;
                    let receiver = *receiver;
                    {
                        let call = self::JavaScriptObjectCopyCall::ObjectCopyCall(
                            Box::new(callee.clone()),
                            Box::new(receiver.clone()),
                            arguments.clone(),
                        );
                        match call.clone() {
                            self::JavaScriptObjectCopyCall::ObjectCopyCall(
                                shared_constructor_field1,
                                shared_constructor_field2,
                                shared_constructor_field3,
                            ) => {
                                let shared_constructor_field1 = *shared_constructor_field1;
                                let shared_constructor_field2 = *shared_constructor_field2;
                                self::JavaScriptObjectSpreadTransition::ObjectSpreadCall(
                                    Box::new(heap_3.clone()),
                                    Box::new({
                                        let source_ml_source2_nominal_field8368 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field8370 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field8372 =
                                                    shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field8374 =
                                                        shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_nominal_field8376 =
                                                            shared_constructor_field8;
                                                        {
                                                            let source_ml_source2_nominal_field8378 =
                                                                shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_nominal_field8380 =
                                                                    shared_constructor_field10
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field8382 =
                                                                        shared_constructor_field11
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field8384 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field8386 = shared_constructor_field13.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field8388 = shared_constructor_field14.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field8390 = shared_constructor_field15.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_override8394 = shared_constructor_field1.clone();
                                                                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field8368.clone()), Box::new(source_ml_source2_nominal_field8370.clone()), source_ml_source2_nominal_field8372.clone(), source_ml_source2_nominal_field8374.clone(), source_ml_source2_nominal_field8376, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]), Box::new(source_ml_source2_nominal_field8380.clone()), Box::new(source_ml_source2_nominal_override8394.clone()), source_ml_source2_nominal_field8384.clone(), source_ml_source2_nominal_field8386.clone(), Some((Some(transition_state_2.clone()),)), Some((Some(call.clone()),)))
                                                                                    }
                                                                                }
                                                                            }
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
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCompleted(
                    heap_4,
                    success,
                    _,
                ) => {
                    let heap_4 = *heap_4;
                    if success {
                        self::finish_java_script_object_assignment_property(
                            heap_4.clone(),
                            state.clone(),
                        )
                    } else {
                        self::JavaScriptObjectSpreadTransition::ObjectSpreadError(
                            Box::new(heap_4.clone()),
                            self::ml_string::Text::from_units(vec![
                                79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110, 32, 99,
                                97, 110, 110, 111, 116, 32, 115, 101, 116, 32, 105, 116, 115, 32,
                                115, 101, 108, 101, 99, 116, 101, 100, 32, 65, 114, 114, 97, 121,
                                32, 112, 114, 111, 112, 101, 114, 116, 121,
                            ]),
                            None::<(Option<self::ml_string::Text>,)>,
                        )
                    }
                }
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentThrow(heap_5, value) => {
                    let heap_5 = *heap_5;
                    let value = *value;
                    self::JavaScriptObjectSpreadTransition::ObjectSpreadThrow(
                        Box::new(heap_5.clone()),
                        Box::new(value.clone()),
                    )
                }
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentError(
                    heap_6,
                    error_class,
                    reason,
                ) => {
                    let heap_6 = *heap_6;
                    {
                        let source_ml_source2_constructor_field8402 = Some(error_class.clone());
                        self::JavaScriptObjectSpreadTransition::ObjectSpreadError(
                            Box::new(heap_6.clone()),
                            reason.clone(),
                            Some((source_ml_source2_constructor_field8402.clone(),)),
                        )
                    }
                }
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                    heap_7,
                    reason_2,
                ) => {
                    let heap_7 = *heap_7;
                    self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                        Box::new(heap_7.clone()),
                        reason_2.clone(),
                    )
                }
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentResourceLimit(
                    heap_8,
                    transition_state_3,
                    reason_3,
                ) => {
                    let heap_8 = *heap_8;
                    let transition_state_3 = *transition_state_3;
                    self::JavaScriptObjectSpreadTransition::ObjectSpreadResourceLimit(
                        Box::new(heap_8.clone()),
                        Box::new({
                            let source_ml_source2_nominal_field8405 =
                                shared_constructor_field4.clone();
                            {
                                let source_ml_source2_nominal_field8407 =
                                    shared_constructor_field5.clone();
                                {
                                    let source_ml_source2_nominal_field8409 =
                                        shared_constructor_field6.clone();
                                    {
                                        let source_ml_source2_nominal_field8411 =
                                            shared_constructor_field7.clone();
                                        {
                                            let source_ml_source2_nominal_field8413 =
                                                shared_constructor_field8;
                                            {
                                                let source_ml_source2_nominal_field8415 =
                                                    shared_constructor_field9.clone();
                                                {
                                                    let source_ml_source2_nominal_field8417 =
                                                        shared_constructor_field10.clone();
                                                    {
                                                        let source_ml_source2_nominal_field8419 =
                                                            shared_constructor_field11.clone();
                                                        {
                                                            let source_ml_source2_nominal_field8421 =
                                                                shared_constructor_field12.clone();
                                                            {
                                                                let source_ml_source2_nominal_field8423 =
                                                                    shared_constructor_field13
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field8425 =
                                                                        shared_constructor_field14
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field8427 = shared_constructor_field15.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_override8431 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source2_nominal_override8432 = None::<self::JavaScriptObjectCopyCall>;
                                                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field8405.clone()), Box::new(source_ml_source2_nominal_field8407.clone()), source_ml_source2_nominal_field8409.clone(), source_ml_source2_nominal_field8411.clone(), source_ml_source2_nominal_field8413, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]), Box::new(source_ml_source2_nominal_field8417.clone()), Box::new(source_ml_source2_nominal_override8431.clone()), source_ml_source2_nominal_field8421.clone(), source_ml_source2_nominal_field8423.clone(), Some((Some(transition_state_3.clone()),)), Some((source_ml_source2_nominal_override8432.clone(),)))
                                                                            }
                                                                        }
                                                                    }
                                                                }
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn write_java_script_object_assignment_property(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectSpreadState,
    key: self::JavaScriptSemanticValue,
) -> self::JavaScriptObjectSpreadTransition {
    match state.clone() {
        self::JavaScriptObjectSpreadState::ObjectSpreadState(
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
        ) => {
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            if ({
                let source_ml_source2_s8434 = shared_constructor_field4.clone();
                match source_ml_source2_s8434.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => false,
                    _ => true,
                }
            } || match (
                match state.clone() {
                    self::JavaScriptObjectSpreadState::ObjectSpreadState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value8438,
                        _,
                        _,
                    ) => ((source_ml_source2_data_property_value8438
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }) {
                self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110, 32, 114, 101,
                        116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114,
                        103, 101, 116, 32, 97, 110, 100, 32, 112, 114, 105, 109, 105, 116, 105,
                        118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 97, 117,
                        116, 104, 111, 114, 105, 116, 121,
                    ]),
                )
            } else {
                {
                    let plan = self::plan_java_script_ordinary_set_with_array_definition(
                        heap.clone(),
                        (((match shared_constructor_field4.clone() {
                            self::JavaScriptSemanticValue::Object(
                                source_ml_source2_data_property_value8440,
                                _,
                            ) => Some((source_ml_source2_data_property_value8440.clone(),)),
                            _ => None::<(self::ml::Big,)>,
                        })
                        .clone()
                        .expect("a proven present value"))
                        .0)
                            .clone(),
                        key.clone(),
                        shared_constructor_field10.clone(),
                        shared_constructor_field4.clone(),
                    );
                    if (((({
                        let ml_reference_value658 = plan.borrow().clone();
                        ml_reference_value658
                    })
                    .0)
                        .4)
                        != (self::ml_string::Text::from_units(vec![])))
                    {
                        self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                            Box::new(
                                (({
                                    let ml_reference_value659 = plan.borrow().clone();
                                    ml_reference_value659
                                })
                                .0)
                                    .5,
                            ),
                            (({
                                let ml_reference_value660 = plan.borrow().clone();
                                ml_reference_value660
                            })
                            .0)
                                .4,
                        )
                    } else {
                        {
                            let definition = ((((({
                                let ml_reference_value661 = plan.borrow().clone();
                                ml_reference_value661
                            })
                            .0)
                                .3)
                                .clone()
                                .unwrap_or_else(|| {
                                    (None::<
                                        std::rc::Rc<
                                            std::cell::RefCell<(
                                                (
                                                    bool,
                                                    bool,
                                                    bool,
                                                    bool,
                                                    self::JavaScriptSemanticValue,
                                                    bool,
                                                    self::JavaScriptSemanticValue,
                                                    bool,
                                                    self::JavaScriptSemanticValue,
                                                    bool,
                                                    bool,
                                                    bool,
                                                ),
                                                Vec<f64>,
                                            )>,
                                        >,
                                    >,)
                                }))
                            .0)
                                .clone();
                            if ((((({
                                let ml_reference_value662 = plan.borrow().clone();
                                ml_reference_value662
                            })
                            .0)
                                .0)
                                == (self::ml_string::Text::from_units(vec![
                                    97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116,
                                    105, 111, 110,
                                ])))
                                && !(match (definition.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }))
                            {
                                self::accept_java_script_object_assignment_array((({ let ml_reference_value663 = plan.borrow().clone(); ml_reference_value663 }).0).5, state.clone(), self::begin_java_script_array_assignment((({ let ml_reference_value664 = plan.borrow().clone(); ml_reference_value664 }).0).5, (((match shared_constructor_field4.clone() {
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value8444, _) => {
                                        Some((source_ml_source2_data_property_value8444.clone(),))
                                    }
                                    _ => {
                                        None::<(self::ml::Big,)>
                                    }
                                }).clone().expect("a proven present value")).0).clone(), key.clone(), definition.clone().expect("a proven present value"), (match state.clone() {
                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8445, _, _) => {
                                        ((source_ml_source2_data_property_value8445.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value")))
                            } else {
                                if (((({
                                    let ml_reference_value665 = plan.borrow().clone();
                                    ml_reference_value665
                                })
                                .0)
                                    .0)
                                    == (self::ml_string::Text::from_units(vec![99, 97, 108, 108])))
                                {
                                    {
                                        let call = self::JavaScriptObjectCopyCall::ObjectCopyCall(
                                            Box::new(
                                                (({
                                                    let ml_reference_value666 =
                                                        plan.borrow().clone();
                                                    ml_reference_value666
                                                })
                                                .0)
                                                    .2,
                                            ),
                                            Box::new(
                                                (({
                                                    let ml_reference_value667 =
                                                        plan.borrow().clone();
                                                    ml_reference_value667
                                                })
                                                .0)
                                                    .6,
                                            ),
                                            (({
                                                let ml_reference_value668 = plan.borrow().clone();
                                                ml_reference_value668
                                            })
                                            .0)
                                                .1,
                                        );
                                        match call.clone() {
                                            self::JavaScriptObjectCopyCall::ObjectCopyCall(
                                                shared_constructor_field1,
                                                shared_constructor_field2,
                                                shared_constructor_field3,
                                            ) => {
                                                let shared_constructor_field1 =
                                                    *shared_constructor_field1;
                                                let shared_constructor_field2 =
                                                    *shared_constructor_field2;
                                                self::JavaScriptObjectSpreadTransition::ObjectSpreadCall(Box::new((({ let ml_reference_value669 = plan.borrow().clone(); ml_reference_value669 }).0).5), Box::new({
                                                    let source_ml_source2_nominal_field8448 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field8450 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field8452 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field8454 = shared_constructor_field7.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field8456 = shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field8458 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field8460 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field8462 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field8464 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field8466 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field8468 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field8470 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override8473 = shared_constructor_field1.clone();
                                                                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_nominal_field8448.clone()), Box::new(source_ml_source2_nominal_field8450.clone()), source_ml_source2_nominal_field8452.clone(), source_ml_source2_nominal_field8454.clone(), source_ml_source2_nominal_field8456, self::ml_string::Text::from_units(vec![115, 101, 116, 116, 101, 114]), Box::new(source_ml_source2_nominal_field8460.clone()), Box::new(source_ml_source2_nominal_override8473.clone()), source_ml_source2_nominal_field8464.clone(), source_ml_source2_nominal_field8466.clone(), source_ml_source2_nominal_field8468.clone(), Some((Some(call.clone()),)))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }), Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone())
                                            }
                                        }
                                    }
                                } else {
                                    if (((({
                                        let ml_reference_value670 = plan.borrow().clone();
                                        ml_reference_value670
                                    })
                                    .0)
                                        .0)
                                        != (self::ml_string::Text::from_units(vec![
                                            99, 111, 109, 112, 108, 101, 116, 101,
                                        ])))
                                    {
                                        self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                                            Box::new(
                                                (({
                                                    let ml_reference_value671 =
                                                        plan.borrow().clone();
                                                    ml_reference_value671
                                                })
                                                .0)
                                                    .5,
                                            ),
                                            self::ml_string::Text::from_units(vec![
                                                79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105,
                                                103, 110, 32, 83, 101, 116, 32, 114, 101, 116, 97,
                                                105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                                117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114,
                                                121, 32, 114, 101, 115, 117, 108, 116, 32, 111,
                                                114, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                                                115, 101, 116, 116, 101, 114,
                                            ]),
                                        )
                                    } else {
                                        if !((({
                                            let ml_reference_value672 = plan.borrow().clone();
                                            ml_reference_value672
                                        })
                                        .0)
                                            .7)
                                        {
                                            {
                                                let source_ml_source2_constructor_field8478 = (({
                                                    let ml_reference_value673 =
                                                        plan.borrow().clone();
                                                    ml_reference_value673
                                                })
                                                .0)
                                                    .5;
                                                self::JavaScriptObjectSpreadTransition::ObjectSpreadError(Box::new(source_ml_source2_constructor_field8478.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 101, 116, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121]), None::<(Option<self::ml_string::Text>,)>)
                                            }
                                        } else {
                                            if !(self::test_java_script_reflection_property_budget(
                                                heap.clone(),
                                                shared_constructor_field4.clone(),
                                                key.clone(),
                                            )) {
                                                self::JavaScriptObjectSpreadTransition::ObjectSpreadResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 104, 101, 97, 112, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 98, 117, 100, 103, 101, 116]))
                                            } else {
                                                self::finish_java_script_object_assignment_property(
                                                    (({
                                                        let ml_reference_value674 =
                                                            plan.borrow().clone();
                                                        ml_reference_value674
                                                    })
                                                    .0)
                                                        .5,
                                                    state.clone(),
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_concatenation_call(
    actual: self::JavaScriptArrayConcatenationCall,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> bool {
    match actual.clone() {
        self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            (((self::test_java_script_semantic_same_value(
                shared_constructor_field1.clone(),
                callee.clone(),
            ) && self::test_java_script_semantic_same_value(
                shared_constructor_field2.clone(),
                receiver.clone(),
            )) && (({
                let ml_reference_array675 = shared_constructor_field3.clone();
                let ml_reference_projection675 = ml_reference_array675.borrow().len() as f64;
                ml_reference_projection675
            }) == ({
                let ml_reference_array676 = arguments_.clone();
                let ml_reference_projection676 = ml_reference_array676.borrow().len() as f64;
                ml_reference_projection676
            }))) && {
                let ml_values677 = {
                    let ml_reference_value678 = shared_constructor_field3.borrow().clone();
                    ml_reference_value678
                };
                let ml_callback677 = {
                    let arguments_ = arguments_.clone();
                    std::rc::Rc::new(
                        move |value: self::JavaScriptSemanticValue, index: f64| -> bool {
                            self::test_java_script_semantic_same_value(
                                value.clone(),
                                ({
                                    let source_ml_source2_array_read_value8486 = arguments_.clone();
                                    {
                                        let source_ml_source2_array_read_index8487 = index;
                                        {
                                            let ml_reference_array679 =
                                                source_ml_source2_array_read_value8486.clone();
                                            let ml_reference_position679 =
                                                source_ml_source2_array_read_index8487;
                                            let ml_reference_projection679 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array679.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position679,
                                                    ),
                                                );
                                            ml_reference_projection679
                                        }
                                    }
                                })
                                .clone()
                                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                            )
                        },
                    )
                };
                ml_values677
                    .iter()
                    .cloned()
                    .enumerate()
                    .all(|(ml_index677, ml_value677)| {
                        ml_callback677(ml_value677, (ml_index677 as f64))
                    })
            })
        }
    }
}
