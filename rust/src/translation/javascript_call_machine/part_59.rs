// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConcatenationCall;
use super::JavaScriptArrayConcatenationState;
use super::JavaScriptArrayConcatenationTransition;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptArrayConcatenationElementLoop342Result;
use super::begin_java_script_scalar_protocol;
use super::continue_java_script_array_concatenation_final_set;
use super::continue_java_script_array_concatenation_length;
use super::continue_java_script_array_concatenation_species;
use super::continue_java_script_array_concatenation_spreadability;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_array_concatenation_state_error;
use super::read_java_script_indexed_property_presence;
use super::read_java_script_ordinary_value_error;
use super::request_java_script_array_concatenation_call;
use super::resume_java_script_array_species_getter;
use super::resume_java_script_object_spread;
use super::resume_java_script_scalar_protocol;
use super::source_ml_source1_temporary_step_java_script_array_concatenation_element_loop342;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_concatenation_element(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayConcatenationState,
) -> self::JavaScriptArrayConcatenationTransition {
    match state.clone() {
        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
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
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field13 = *shared_constructor_field13;
            let shared_constructor_field14 = *shared_constructor_field14;
            if ((shared_constructor_field15
                != (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121])))
                || {
                    let source_ml_source2_s10346 = shared_constructor_field6.clone();
                    match source_ml_source2_s10346.clone() {
                        self::JavaScriptSemanticValue::Object(_, _) => false,
                        _ => true,
                    }
                })
            {
                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101,
                        115, 117, 109, 101, 115, 32, 111, 110, 108, 121, 32, 105, 116, 115, 32, 97,
                        99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99,
                        104, 105, 108, 100,
                    ]),
                )
            } else {
                if (shared_constructor_field9 == shared_constructor_field8) {
                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(
                        Box::new(heap.clone()),
                        Box::new({
                            let source_ml_source2_nominal_field10356 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_nominal_field10358 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_nominal_field10360 =
                                        shared_constructor_field3.clone();
                                    {
                                        let source_ml_source2_nominal_field10362 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field10364 =
                                                shared_constructor_field5;
                                            {
                                                let source_ml_source2_nominal_field10366 =
                                                    shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field10368 =
                                                        shared_constructor_field7;
                                                    {
                                                        let source_ml_source2_nominal_field10370 =
                                                            shared_constructor_field8;
                                                        {
                                                            let source_ml_source2_nominal_field10372 =
                                                                shared_constructor_field9;
                                                            {
                                                                let source_ml_source2_nominal_field10374 =
                                                                    shared_constructor_field10;
                                                                {
                                                                    let source_ml_source2_nominal_field10376 =
                                                                        shared_constructor_field11;
                                                                    {
                                                                        let source_ml_source2_nominal_field10378 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field10380 = shared_constructor_field13.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field10382 = shared_constructor_field14.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field10384 = shared_constructor_field15.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10386 = shared_constructor_field16.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10388 = shared_constructor_field17.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10390 = shared_constructor_field18.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10392 = shared_constructor_field19.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10394 = shared_constructor_field20.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10396 = shared_constructor_field21.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10398 = shared_constructor_field22.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override10401 = (shared_constructor_field5 + 1f64);
                                                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10356.clone()), source_ml_source2_nominal_field10358.clone(), Box::new(source_ml_source2_nominal_field10360.clone()), Box::new(source_ml_source2_nominal_field10362.clone()), source_ml_source2_nominal_override10401, Box::new(source_ml_source2_nominal_field10366.clone()), source_ml_source2_nominal_field10368, source_ml_source2_nominal_field10370, source_ml_source2_nominal_field10372, source_ml_source2_nominal_field10374, source_ml_source2_nominal_field10376, Box::new(source_ml_source2_nominal_field10378.clone()), Box::new(source_ml_source2_nominal_field10380.clone()), Box::new(source_ml_source2_nominal_field10382.clone()), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116]), source_ml_source2_nominal_field10386.clone(), source_ml_source2_nominal_field10388.clone(), source_ml_source2_nominal_field10390.clone(), source_ml_source2_nominal_field10392.clone(), source_ml_source2_nominal_field10394.clone(), source_ml_source2_nominal_field10396.clone(), source_ml_source2_nominal_field10398.clone())
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                } else {
                    {
                        let name = {
                            let source_ml_source2_string_argument10403 = shared_constructor_field9;
                            self::ml_string::Text::from_text(&self::ml_number::js_number(
                                source_ml_source2_string_argument10403,
                            ))
                        };
                        {
                            let units =
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                            {
                                let index = 0f64;
                                {
                                    let source_ml_source2_s10404 = self::source_ml_source1_temporary_step_java_script_array_concatenation_element_loop342(&(name), units.clone(), index);
                                    match source_ml_source2_s10404.clone() {
                                        self::MlSource1TemporaryStepJavaScriptArrayConcatenationElementLoop342Result::MlSource1TemporaryStepJavaScriptArrayConcatenationElementLoop342Done => {
                                            {
                                                let key = self::JavaScriptSemanticValue::Text(units.clone());
                                                {
                                                    let presence = self::read_java_script_indexed_property_presence(heap.clone(), (((match shared_constructor_field6.clone() {
                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value10406, _) => {
                                                            Some((source_ml_source2_data_property_value10406.clone(),))
                                                        }
                                                        _ => {
                                                            None::<(self::ml::Big,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone(), key.clone());
                                                    if (((({ let ml_reference_value721 = presence.borrow().clone(); ml_reference_value721 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), (({ let ml_reference_value722 = presence.borrow().clone(); ml_reference_value722 }).0).0)
                                                    } else {
                                                        if !((({ let ml_reference_value723 = presence.borrow().clone(); ml_reference_value723 }).0).1) {
                                                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap.clone()), Box::new({
                                                                let source_ml_source2_nominal_field10410 = shared_constructor_field1.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field10412 = shared_constructor_field2.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field10414 = shared_constructor_field3.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field10416 = shared_constructor_field4.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field10418 = shared_constructor_field5;
                                                                                {
                                                                                    let source_ml_source2_nominal_field10420 = shared_constructor_field6.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10422 = shared_constructor_field7;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10424 = shared_constructor_field8;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10426 = shared_constructor_field9;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10428 = shared_constructor_field10;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10430 = shared_constructor_field11;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10432 = shared_constructor_field12.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10434 = shared_constructor_field13.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10436 = shared_constructor_field14.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field10438 = shared_constructor_field15.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field10440 = shared_constructor_field16.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field10442 = shared_constructor_field17.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field10444 = shared_constructor_field18.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field10446 = shared_constructor_field19.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field10448 = shared_constructor_field20.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field10450 = shared_constructor_field21.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field10452 = shared_constructor_field22.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_override10454 = (shared_constructor_field9 + 1f64);
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_override10455 = (shared_constructor_field10 + 1f64);
                                                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10410.clone()), source_ml_source2_nominal_field10412.clone(), Box::new(source_ml_source2_nominal_field10414.clone()), Box::new(source_ml_source2_nominal_field10416.clone()), source_ml_source2_nominal_field10418, Box::new(source_ml_source2_nominal_field10420.clone()), source_ml_source2_nominal_field10422, source_ml_source2_nominal_field10424, source_ml_source2_nominal_override10454, source_ml_source2_nominal_override10455, source_ml_source2_nominal_field10430, Box::new(source_ml_source2_nominal_field10432.clone()), Box::new(source_ml_source2_nominal_field10434.clone()), Box::new(source_ml_source2_nominal_field10436.clone()), source_ml_source2_nominal_field10438.clone(), source_ml_source2_nominal_field10440.clone(), source_ml_source2_nominal_field10442.clone(), source_ml_source2_nominal_field10444.clone(), source_ml_source2_nominal_field10446.clone(), source_ml_source2_nominal_field10448.clone(), source_ml_source2_nominal_field10450.clone(), source_ml_source2_nominal_field10452.clone())
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
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
                                                                let get = self::plan_java_script_ordinary_get(heap.clone(), (((match shared_constructor_field6.clone() {
                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value10457, _) => {
                                                                        Some((source_ml_source2_data_property_value10457.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::ml::Big,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone(), key.clone(), shared_constructor_field6.clone());
                                                                if (((({ let ml_reference_value724 = get.borrow().clone(); ml_reference_value724 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), (({ let ml_reference_value725 = get.borrow().clone(); ml_reference_value725 }).0).3)
                                                                } else {
                                                                    if (((({ let ml_reference_value726 = get.borrow().clone(); ml_reference_value726 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                        self::request_java_script_array_concatenation_call(heap.clone(), {
                                                                            let source_ml_source2_nominal_field10460 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field10462 = shared_constructor_field2.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field10464 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10466 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10468 = shared_constructor_field5;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10470 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10472 = shared_constructor_field7;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10474 = shared_constructor_field8;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10476 = shared_constructor_field9;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10478 = shared_constructor_field10;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10480 = shared_constructor_field11;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field10482 = shared_constructor_field12.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field10484 = shared_constructor_field13.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field10486 = shared_constructor_field14.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field10488 = shared_constructor_field15.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field10490 = shared_constructor_field16.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field10492 = shared_constructor_field17.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field10494 = shared_constructor_field18.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field10496 = shared_constructor_field19.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field10498 = shared_constructor_field20.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field10500 = shared_constructor_field21.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field10502 = shared_constructor_field22.clone();
                                                                                                                                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10460.clone()), source_ml_source2_nominal_field10462.clone(), Box::new(source_ml_source2_nominal_field10464.clone()), Box::new(source_ml_source2_nominal_field10466.clone()), source_ml_source2_nominal_field10468, Box::new(source_ml_source2_nominal_field10470.clone()), source_ml_source2_nominal_field10472, source_ml_source2_nominal_field10474, source_ml_source2_nominal_field10476, source_ml_source2_nominal_field10478, source_ml_source2_nominal_field10480, Box::new(source_ml_source2_nominal_field10482.clone()), Box::new(source_ml_source2_nominal_field10484.clone()), Box::new(source_ml_source2_nominal_field10486.clone()), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), source_ml_source2_nominal_field10490.clone(), source_ml_source2_nominal_field10492.clone(), source_ml_source2_nominal_field10494.clone(), source_ml_source2_nominal_field10496.clone(), source_ml_source2_nominal_field10498.clone(), source_ml_source2_nominal_field10500.clone(), source_ml_source2_nominal_field10502.clone())
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }, (({ let ml_reference_value727 = get.borrow().clone(); ml_reference_value727 }).0).2, (({ let ml_reference_value728 = get.borrow().clone(); ml_reference_value728 }).0).4, (({ let ml_reference_value729 = get.borrow().clone(); ml_reference_value729 }).0).1)
                                                                    } else {
                                                                        if (((({ let ml_reference_value730 = get.borrow().clone(); ml_reference_value730 }).0).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap.clone()), Box::new({
                                                                                let source_ml_source2_nominal_field10506 = shared_constructor_field1.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field10508 = shared_constructor_field2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10510 = shared_constructor_field3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10512 = shared_constructor_field4.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10514 = shared_constructor_field5;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10516 = shared_constructor_field6.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10518 = shared_constructor_field7;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10520 = shared_constructor_field8;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10522 = shared_constructor_field9;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10524 = shared_constructor_field10;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field10526 = shared_constructor_field11;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field10528 = shared_constructor_field12.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field10530 = shared_constructor_field13.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field10532 = shared_constructor_field14.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field10534 = shared_constructor_field15.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field10536 = shared_constructor_field16.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field10538 = shared_constructor_field17.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field10540 = shared_constructor_field18.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field10542 = shared_constructor_field19.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field10544 = shared_constructor_field20.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field10546 = shared_constructor_field21.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field10548 = shared_constructor_field22.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_override10551 = (({ let ml_reference_value731 = get.borrow().clone(); ml_reference_value731 }).0).5;
                                                                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10506.clone()), source_ml_source2_nominal_field10508.clone(), Box::new(source_ml_source2_nominal_field10510.clone()), Box::new(source_ml_source2_nominal_field10512.clone()), source_ml_source2_nominal_field10514, Box::new(source_ml_source2_nominal_field10516.clone()), source_ml_source2_nominal_field10518, source_ml_source2_nominal_field10520, source_ml_source2_nominal_field10522, source_ml_source2_nominal_field10524, source_ml_source2_nominal_field10526, Box::new(source_ml_source2_nominal_field10528.clone()), Box::new(source_ml_source2_nominal_override10551.clone()), Box::new(source_ml_source2_nominal_field10532.clone()), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), source_ml_source2_nominal_field10536.clone(), source_ml_source2_nominal_field10538.clone(), source_ml_source2_nominal_field10540.clone(), source_ml_source2_nominal_field10542.clone(), source_ml_source2_nominal_field10544.clone(), source_ml_source2_nominal_field10546.clone(), source_ml_source2_nominal_field10548.clone())
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
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
                                                                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 105, 110, 100, 101, 120, 101, 100, 32, 71, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
pub fn resume_java_script_array_concatenation_call(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArrayConcatenationState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArrayConcatenationTransition {
    match state.clone() {
        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
            _,
            _,
            _,
            shared_constructor_field23,
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
            shared_constructor_field24,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field23 = *shared_constructor_field23;
            {
                let error = self::read_java_script_array_concatenation_state_error(
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if (match (
                        match state.clone() {
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
                                source_ml_source2_data_property_value10552,
                            ) => ((source_ml_source2_data_property_value10552
                                .clone()
                                .unwrap_or_else(|| {
                                    (None::<self::JavaScriptArrayConcatenationCall>,)
                                }))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } || !(self::ml_array::includes(
                        &{
                            let ml_reference_value732 =
                                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    self::ml_string::Text::from_units(vec![
                                        110, 111, 114, 109, 97, 108, 67, 111, 109, 112, 108, 101,
                                        116, 105, 111, 110,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        116, 104, 114, 111, 119, 67, 111, 109, 112, 108, 101, 116,
                                        105, 111, 110,
                                    ]),
                                ])))
                                .borrow()
                                .clone();
                            ml_reference_value732
                        },
                        &match completion.clone() {
                            self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(_) => {
                                self::ml_string::Text::from_units(vec![
                                    110, 111, 114, 109, 97, 108, 67, 111, 109, 112, 108, 101, 116,
                                    105, 111, 110,
                                ])
                            }
                            self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(_) => {
                                self::ml_string::Text::from_units(vec![
                                    116, 104, 114, 111, 119, 67, 111, 109, 112, 108, 101, 116, 105,
                                    111, 110,
                                ])
                            }
                        },
                        0f64,
                        false,
                        |a, b| a == b,
                    ))) {
                        self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32,
                                114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101,
                                116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117,
                                97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97,
                                108, 108, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116,
                                105, 111, 110, 32, 107, 105, 110, 100,
                            ]),
                        )
                    } else {
                        {
                            let invalid = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value10553) => {
                                    let source_ml_source2_data_property_value10553 = *source_ml_source2_data_property_value10553;
                                    source_ml_source2_data_property_value10553.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value10554) => {
                                    let source_ml_source2_data_property_value10554 = *source_ml_source2_data_property_value10554;
                                    source_ml_source2_data_property_value10554.clone()
                                }
                            });
                            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), invalid.clone())
                            } else {
                                match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        {
                                            let prior_call = match state.clone() {
                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10555) => {
                                                    ((source_ml_source2_data_property_value10555.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                }
                                            };
                                            {
                                                let base = match state.clone() {
                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(source_ml_source2_nominal_rest_field10556, source_ml_source2_nominal_rest_field10557, source_ml_source2_nominal_rest_field10558, source_ml_source2_nominal_rest_field10559, source_ml_source2_nominal_rest_field10560, source_ml_source2_nominal_rest_field10561, source_ml_source2_nominal_rest_field10562, source_ml_source2_nominal_rest_field10563, source_ml_source2_nominal_rest_field10564, source_ml_source2_nominal_rest_field10565, source_ml_source2_nominal_rest_field10566, source_ml_source2_nominal_rest_field10567, source_ml_source2_nominal_rest_field10568, source_ml_source2_nominal_rest_field10569, source_ml_source2_nominal_rest_field10570, source_ml_source2_nominal_rest_field10571, source_ml_source2_nominal_rest_field10572, source_ml_source2_nominal_rest_field10573, source_ml_source2_nominal_rest_field10574, source_ml_source2_nominal_rest_field10575, source_ml_source2_nominal_rest_field10576, _) => {
                                                        let source_ml_source2_nominal_rest_field10556 = *source_ml_source2_nominal_rest_field10556;
                                                        let source_ml_source2_nominal_rest_field10558 = *source_ml_source2_nominal_rest_field10558;
                                                        let source_ml_source2_nominal_rest_field10559 = *source_ml_source2_nominal_rest_field10559;
                                                        let source_ml_source2_nominal_rest_field10561 = *source_ml_source2_nominal_rest_field10561;
                                                        let source_ml_source2_nominal_rest_field10567 = *source_ml_source2_nominal_rest_field10567;
                                                        let source_ml_source2_nominal_rest_field10568 = *source_ml_source2_nominal_rest_field10568;
                                                        let source_ml_source2_nominal_rest_field10569 = *source_ml_source2_nominal_rest_field10569;
                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_rest_field10556.clone()), source_ml_source2_nominal_rest_field10557.clone(), Box::new(source_ml_source2_nominal_rest_field10558.clone()), Box::new(source_ml_source2_nominal_rest_field10559.clone()), source_ml_source2_nominal_rest_field10560, Box::new(source_ml_source2_nominal_rest_field10561.clone()), source_ml_source2_nominal_rest_field10562, source_ml_source2_nominal_rest_field10563, source_ml_source2_nominal_rest_field10564, source_ml_source2_nominal_rest_field10565, source_ml_source2_nominal_rest_field10566, Box::new(source_ml_source2_nominal_rest_field10567.clone()), Box::new(source_ml_source2_nominal_rest_field10568.clone()), Box::new(source_ml_source2_nominal_rest_field10569.clone()), source_ml_source2_nominal_rest_field10570.clone(), source_ml_source2_nominal_rest_field10571.clone(), source_ml_source2_nominal_rest_field10572.clone(), source_ml_source2_nominal_rest_field10573.clone(), source_ml_source2_nominal_rest_field10574.clone(), source_ml_source2_nominal_rest_field10575.clone(), source_ml_source2_nominal_rest_field10576.clone(), None::<(Option<self::JavaScriptArrayConcatenationCall>,)>)
                                                    }
                                                };
                                                match base.clone() {
                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11, shared_constructor_field12, shared_constructor_field13, shared_constructor_field14, shared_constructor_field15, shared_constructor_field16, shared_constructor_field17, shared_constructor_field18, shared_constructor_field19, shared_constructor_field20, shared_constructor_field21, shared_constructor_field22) => {
                                                        let shared_constructor_field1 = *shared_constructor_field1;
                                                        let shared_constructor_field3 = *shared_constructor_field3;
                                                        let shared_constructor_field4 = *shared_constructor_field4;
                                                        let shared_constructor_field6 = *shared_constructor_field6;
                                                        let shared_constructor_field12 = *shared_constructor_field12;
                                                        let shared_constructor_field13 = *shared_constructor_field13;
                                                        let shared_constructor_field14 = *shared_constructor_field14;
                                                        if ((shared_constructor_field24 == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && !(match (match state.clone() {
                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10579, _, _, _, _, _) => {
                                                                ((source_ml_source2_data_property_value10579.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                            }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                            self::continue_java_script_array_concatenation_species(heap.clone(), base.clone(), self::resume_java_script_array_species_getter(heap.clone(), directory.clone(), (match state.clone() {
                                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10580, _, _, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value10580.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value"), completion.clone()))
                                                        } else {
                                                            if (shared_constructor_field24 == (self::ml_string::Text::from_units(vec![115, 112, 114, 101, 97, 100, 97, 98, 105, 108, 105, 116, 121, 32, 103, 101, 116, 116, 101, 114]))) {
                                                                self::continue_java_script_array_concatenation_spreadability(heap.clone(), base.clone(), value.clone())
                                                            } else {
                                                                if (shared_constructor_field24 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]))) {
                                                                    self::continue_java_script_array_concatenation_length(heap.clone(), {
                                                                        let source_ml_source2_nominal_field10586 = shared_constructor_field1.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field10588 = shared_constructor_field2.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field10590 = shared_constructor_field3.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field10592 = shared_constructor_field4.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10594 = shared_constructor_field5;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10596 = shared_constructor_field6.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10598 = shared_constructor_field7;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10600 = shared_constructor_field8;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10602 = shared_constructor_field9;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10604 = shared_constructor_field10;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10606 = shared_constructor_field11;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10608 = shared_constructor_field12.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field10610 = shared_constructor_field13.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field10612 = shared_constructor_field14.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field10614 = shared_constructor_field15.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field10616 = shared_constructor_field16.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field10618 = shared_constructor_field17.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field10620 = shared_constructor_field18.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field10622 = shared_constructor_field19.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field10624 = shared_constructor_field20.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field10626 = shared_constructor_field21.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field10628 = shared_constructor_field22.clone();
                                                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10586.clone()), source_ml_source2_nominal_field10588.clone(), Box::new(source_ml_source2_nominal_field10590.clone()), Box::new(source_ml_source2_nominal_field10592.clone()), source_ml_source2_nominal_field10594, Box::new(source_ml_source2_nominal_field10596.clone()), source_ml_source2_nominal_field10598, source_ml_source2_nominal_field10600, source_ml_source2_nominal_field10602, source_ml_source2_nominal_field10604, source_ml_source2_nominal_field10606, Box::new(source_ml_source2_nominal_field10608.clone()), Box::new(source_ml_source2_nominal_field10610.clone()), Box::new(value.clone()), source_ml_source2_nominal_field10614.clone(), source_ml_source2_nominal_field10616.clone(), source_ml_source2_nominal_field10618.clone(), source_ml_source2_nominal_field10620.clone(), source_ml_source2_nominal_field10622.clone(), source_ml_source2_nominal_field10624.clone(), source_ml_source2_nominal_field10626.clone(), source_ml_source2_nominal_field10628.clone())
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }, self::begin_java_script_scalar_protocol(heap.clone(), value.clone(), shared_constructor_field23.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                                } else {
                                                                    if ((shared_constructor_field24 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && !(match (match state.clone() {
                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10634, _, _, _, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value10634.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                        }
                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                        self::continue_java_script_array_concatenation_length(heap.clone(), base.clone(), self::resume_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10635, _, _, _, _, _, _) => {
                                                                                ((source_ml_source2_data_property_value10635.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value"), completion.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                                    } else {
                                                                        if (shared_constructor_field24 == (self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]))) {
                                                                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap.clone()), Box::new({
                                                                                let source_ml_source2_nominal_field10639 = shared_constructor_field1.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field10641 = shared_constructor_field2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field10643 = shared_constructor_field3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field10645 = shared_constructor_field4.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field10647 = shared_constructor_field5;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field10649 = shared_constructor_field6.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field10651 = shared_constructor_field7;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field10653 = shared_constructor_field8;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field10655 = shared_constructor_field9;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field10657 = shared_constructor_field10;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field10659 = shared_constructor_field11;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field10661 = shared_constructor_field12.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field10663 = shared_constructor_field13.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field10665 = shared_constructor_field14.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field10667 = shared_constructor_field15.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field10669 = shared_constructor_field16.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field10671 = shared_constructor_field17.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field10673 = shared_constructor_field18.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field10675 = shared_constructor_field19.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field10677 = shared_constructor_field20.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field10679 = shared_constructor_field21.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field10681 = shared_constructor_field22.clone();
                                                                                                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field10639.clone()), source_ml_source2_nominal_field10641.clone(), Box::new(source_ml_source2_nominal_field10643.clone()), Box::new(source_ml_source2_nominal_field10645.clone()), source_ml_source2_nominal_field10647, Box::new(source_ml_source2_nominal_field10649.clone()), source_ml_source2_nominal_field10651, source_ml_source2_nominal_field10653, source_ml_source2_nominal_field10655, source_ml_source2_nominal_field10657, source_ml_source2_nominal_field10659, Box::new(source_ml_source2_nominal_field10661.clone()), Box::new(value.clone()), Box::new(source_ml_source2_nominal_field10665.clone()), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), source_ml_source2_nominal_field10669.clone(), source_ml_source2_nominal_field10671.clone(), source_ml_source2_nominal_field10673.clone(), source_ml_source2_nominal_field10675.clone(), source_ml_source2_nominal_field10677.clone(), source_ml_source2_nominal_field10679.clone(), source_ml_source2_nominal_field10681.clone())
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
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
                                                                            if ((shared_constructor_field24 == (self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104]))) && !(match (match state.clone() {
                                                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10687, _) => {
                                                                                    ((source_ml_source2_data_property_value10687.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                                self::continue_java_script_array_concatenation_final_set(base.clone(), self::resume_java_script_object_spread(heap.clone(), (match state.clone() {
                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value10688, _) => {
                                                                                        ((source_ml_source2_data_property_value10688.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value"), completion.clone()))
                                                                            } else {
                                                                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 104, 97, 115, 101]))
                                                                            }
                                                                        }
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
                                        self::JavaScriptArrayConcatenationTransition::ArrayConcatenationThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
