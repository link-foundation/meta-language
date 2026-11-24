// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptObjectFromEntriesCall;
use super::JavaScriptObjectFromEntriesState;
use super::JavaScriptObjectFromEntriesTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::accept_java_script_object_from_entries_entry_value;
use super::accept_java_script_object_from_entries_iterator;
use super::accept_java_script_object_from_entries_key;
use super::begin_java_script_synchronous_iterator;
use super::begin_java_script_synchronous_iterator_step;
use super::create_java_script_ordinary_set_descriptor;
use super::define_java_script_ordinary_property;
use super::fail_java_script_object_from_entries;
use super::insert_java_script_ordinary_object;
use super::ml;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::provide_java_script_iterator_primitive_prototype;
use super::read_java_script_object_from_entries_state_error;
use super::read_java_script_ordinary_value_error;
use super::resume_java_script_iterator_protocol_call;
use super::resume_java_script_property_key_conversion;
use super::step_java_script_iterator_protocol;
use super::step_java_script_property_key_conversion;
use super::test_java_script_reflection_property_budget;
use super::throw_java_script_object_from_entries;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_object_from_entries(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectFromEntriesState,
) -> self::JavaScriptObjectFromEntriesTransition {
    match state.clone() {
        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
        ) => {
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field13 = *shared_constructor_field13;
            {
                let error = self::read_java_script_object_from_entries_state_error(
                    heap.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if !(match (
                        match state.clone() {
                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                                source_ml_source2_data_property_value21082,
                                _,
                                _,
                                _,
                            ) => ((source_ml_source2_data_property_value21082
                                .clone()
                                .unwrap_or_else(|| {
                                    (None::<self::JavaScriptObjectFromEntriesCall>,)
                                }))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesCall(Box::new(heap.clone()), Box::new(state.clone()), Box::new(match (match state.clone() {
                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21083, _, _, _) => {
                                ((source_ml_source2_data_property_value21083.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesCall>,))).0).clone()
                            }
                        }).clone().expect("a proven present value") {
                            self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(source_ml_source2_data_property_value21084, _, _) => {
                                let source_ml_source2_data_property_value21084 = *source_ml_source2_data_property_value21084;
                                source_ml_source2_data_property_value21084.clone()
                            }
                        }), Box::new(match (match state.clone() {
                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21085, _, _, _) => {
                                ((source_ml_source2_data_property_value21085.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesCall>,))).0).clone()
                            }
                        }).clone().expect("a proven present value") {
                            self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(_, source_ml_source2_data_property_value21086, _) => {
                                let source_ml_source2_data_property_value21086 = *source_ml_source2_data_property_value21086;
                                source_ml_source2_data_property_value21086.clone()
                            }
                        }), match (match state.clone() {
                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21087, _, _, _) => {
                                ((source_ml_source2_data_property_value21087.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesCall>,))).0).clone()
                            }
                        }).clone().expect("a proven present value") {
                            self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(_, _, source_ml_source2_data_property_value21088) => {
                                source_ml_source2_data_property_value21088.clone()
                            }
                        })
                    } else {
                        if (shared_constructor_field9
                            == (self::ml_string::Text::from_units(vec![
                                97, 108, 108, 111, 99, 97, 116, 101,
                            ])))
                        {
                            if (({
                                let ml_reference_array1051 = match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        source_ml_source2_data_property_value21092,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value21092.clone(),
                                };
                                let ml_reference_projection1051 =
                                    ml_reference_array1051.borrow().len() as f64;
                                ml_reference_projection1051
                            }) >= (4096f64))
                            {
                                self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 115, 117, 108, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesAllocate(Box::new(heap.clone()), Box::new(state.clone()), Box::new(shared_constructor_field5.clone()))
                            }
                        } else {
                            if (shared_constructor_field9
                                == (self::ml_string::Text::from_units(vec![
                                    101, 114, 114, 111, 114,
                                ])))
                            {
                                self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesNeedError(Box::new(heap.clone()), Box::new(state.clone()), (match state.clone() {
                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21096, _) => {
                                        ((source_ml_source2_data_property_value21096.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                    }
                                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), (match state.clone() {
                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21097) => {
                                        ((source_ml_source2_data_property_value21097.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                    }
                                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                            } else {
                                if !(match (match state.clone() {
                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21098, _, _, _, _, _) => {
                                        ((source_ml_source2_data_property_value21098.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                    }
                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    self::accept_java_script_object_from_entries_iterator(heap.clone(), state.clone(), self::step_java_script_iterator_protocol(heap.clone(), (match state.clone() {
                                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21099, _, _, _, _, _) => {
                                            ((source_ml_source2_data_property_value21099.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                        }
                                    }).clone().expect("a proven present value")))
                                } else {
                                    if (shared_constructor_field9 == (self::ml_string::Text::from_units(vec![115, 116, 101, 112]))) {
                                        if match (match state.clone() {
                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21102, _, _, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value21102.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 99, 104, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114]))
                                        } else {
                                            self::accept_java_script_object_from_entries_iterator(heap.clone(), state.clone(), self::begin_java_script_synchronous_iterator_step(heap.clone(), (match state.clone() {
                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21103, _, _, _, _, _, _) => {
                                                    ((source_ml_source2_data_property_value21103.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value"), true))
                                        }
                                    } else {
                                        if ((shared_constructor_field9 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 107, 101, 121]))) && !(match (match state.clone() {
                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21106, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value21106.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                            self::accept_java_script_object_from_entries_key(heap.clone(), state.clone(), self::step_java_script_property_key_conversion(heap.clone(), (match state.clone() {
                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21107, _, _, _, _) => {
                                                    ((source_ml_source2_data_property_value21107.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value")))
                                        } else {
                                            if ((shared_constructor_field9 == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 107, 101, 121]))) || (shared_constructor_field9 == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 118, 97, 108, 117, 101])))) {
                                                if {
                                                    let source_ml_source2_s21113 = shared_constructor_field10.clone();
                                                    match source_ml_source2_s21113.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }
                                                } {
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 101, 110, 116, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 97, 99, 99, 101, 115, 115, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116]))
                                                } else {
                                                    {
                                                        let key = self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![if (shared_constructor_field9 == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 107, 101, 121]))) {
                                                            48f64
                                                        } else {
                                                            49f64
                                                        }])));
                                                        {
                                                            let get = self::plan_java_script_ordinary_get(heap.clone(), (((match shared_constructor_field10.clone() {
                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value21120, _) => {
                                                                    Some((source_ml_source2_data_property_value21120.clone(),))
                                                                }
                                                                _ => {
                                                                    None::<(self::ml::Big,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0).clone(), key.clone(), shared_constructor_field10.clone());
                                                            if (((({ let ml_reference_value1052 = get.borrow().clone(); ml_reference_value1052 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(Box::new(heap.clone()), (({ let ml_reference_value1053 = get.borrow().clone(); ml_reference_value1053 }).0).3)
                                                            } else {
                                                                if (((({ let ml_reference_value1054 = get.borrow().clone(); ml_reference_value1054 }).0).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                    self::accept_java_script_object_from_entries_entry_value(heap.clone(), state.clone(), (({ let ml_reference_value1055 = get.borrow().clone(); ml_reference_value1055 }).0).5)
                                                                } else {
                                                                    if (((({ let ml_reference_value1056 = get.borrow().clone(); ml_reference_value1056 }).0).0) != (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 101, 110, 116, 114, 121, 32, 71, 101, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114]))
                                                                    } else {
                                                                        {
                                                                            let call = self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(Box::new((({ let ml_reference_value1057 = get.borrow().clone(); ml_reference_value1057 }).0).2), Box::new((({ let ml_reference_value1058 = get.borrow().clone(); ml_reference_value1058 }).0).4), (({ let ml_reference_value1059 = get.borrow().clone(); ml_reference_value1059 }).0).1);
                                                                            match call.clone() {
                                                                                self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                                                    let shared_constructor_field1 = *shared_constructor_field1;
                                                                                    let shared_constructor_field2 = *shared_constructor_field2;
                                                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesCall(Box::new(heap.clone()), Box::new({
                                                                                        let source_ml_source2_nominal_field21124 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field21126 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field21128 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field21130 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field21132 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field21134 = shared_constructor_field9.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field21136 = shared_constructor_field10.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field21138 = shared_constructor_field11.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field21140 = shared_constructor_field12.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field21142 = shared_constructor_field13.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field21144 = shared_constructor_field14.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field21146 = shared_constructor_field15.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field21148 = shared_constructor_field16.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field21150 = shared_constructor_field17.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field21152 = shared_constructor_field18.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field21154 = shared_constructor_field19.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field21156 = shared_constructor_field20.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_override21158 = shared_constructor_field9.concat(&self::ml_string::Text::from_units(vec![32, 103, 101, 116, 116, 101, 114]));
                                                                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field21124.clone()), Box::new(source_ml_source2_nominal_field21126.clone()), Box::new(source_ml_source2_nominal_field21128.clone()), Box::new(source_ml_source2_nominal_field21130.clone()), Box::new(source_ml_source2_nominal_field21132.clone()), source_ml_source2_nominal_override21158.clone(), Box::new(source_ml_source2_nominal_field21136.clone()), Box::new(source_ml_source2_nominal_field21138.clone()), Box::new(source_ml_source2_nominal_field21140.clone()), Box::new(source_ml_source2_nominal_field21142.clone()), source_ml_source2_nominal_field21144.clone(), source_ml_source2_nominal_field21146.clone(), source_ml_source2_nominal_field21148.clone(), Some((Some(call.clone()),)), source_ml_source2_nominal_field21152.clone(), source_ml_source2_nominal_field21154.clone(), source_ml_source2_nominal_field21156.clone())
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
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
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if (shared_constructor_field9 == (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]))) {
                                                    if {
                                                        let source_ml_source2_s21166 = shared_constructor_field8.clone();
                                                        match source_ml_source2_s21166.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    } {
                                                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 112, 117, 98, 108, 105, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 114, 101, 115, 117, 108, 116]))
                                                    } else {
                                                        if !(self::test_java_script_reflection_property_budget(heap.clone(), shared_constructor_field8.clone(), shared_constructor_field13.clone())) {
                                                            self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                        } else {
                                                            {
                                                                let definition = self::define_java_script_ordinary_property(heap.clone(), (((match shared_constructor_field8.clone() {
                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value21173, _) => {
                                                                        Some((source_ml_source2_data_property_value21173.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::ml::Big,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone(), shared_constructor_field13.clone(), self::create_java_script_ordinary_set_descriptor(shared_constructor_field12.clone(), true));
                                                                if (((({ let ml_reference_value1060 = definition.borrow().clone(); ml_reference_value1060 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(Box::new((({ let ml_reference_value1061 = definition.borrow().clone(); ml_reference_value1061 }).0).1), (({ let ml_reference_value1062 = definition.borrow().clone(); ml_reference_value1062 }).0).0)
                                                                } else {
                                                                    if !((({ let ml_reference_value1063 = definition.borrow().clone(); ml_reference_value1063 }).0).3) {
                                                                        self::fail_java_script_object_from_entries((({ let ml_reference_value1064 = definition.borrow().clone(); ml_reference_value1064 }).0).1, state.clone(), &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 99, 111, 117, 108, 100, 32, 110, 111, 116, 32, 99, 114, 101, 97, 116, 101, 32, 105, 116, 115, 32, 111, 119, 110, 32, 100, 97, 116, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121])))
                                                                    } else {
                                                                        self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesContinue(Box::new((({ let ml_reference_value1065 = definition.borrow().clone(); ml_reference_value1065 }).0).1), Box::new({
                                                                            let source_ml_source2_nominal_field21177 = shared_constructor_field4.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field21179 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field21181 = shared_constructor_field6.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field21183 = shared_constructor_field7.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field21185 = shared_constructor_field8.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field21187 = shared_constructor_field9.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field21189 = shared_constructor_field10.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field21191 = shared_constructor_field11.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field21193 = shared_constructor_field12.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field21195 = shared_constructor_field13.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field21197 = shared_constructor_field14.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field21199 = shared_constructor_field15.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field21201 = shared_constructor_field16.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field21203 = shared_constructor_field17.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field21205 = shared_constructor_field18.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field21207 = shared_constructor_field19.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field21209 = shared_constructor_field20.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_override21212 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_override21213 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_override21214 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_override21215 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field21177.clone()), Box::new(source_ml_source2_nominal_field21179.clone()), Box::new(source_ml_source2_nominal_field21181.clone()), Box::new(source_ml_source2_nominal_field21183.clone()), Box::new(source_ml_source2_nominal_field21185.clone()), self::ml_string::Text::from_units(vec![115, 116, 101, 112]), Box::new(source_ml_source2_nominal_override21212.clone()), Box::new(source_ml_source2_nominal_override21213.clone()), Box::new(source_ml_source2_nominal_override21214.clone()), Box::new(source_ml_source2_nominal_override21215.clone()), source_ml_source2_nominal_field21197.clone(), source_ml_source2_nominal_field21199.clone(), source_ml_source2_nominal_field21201.clone(), source_ml_source2_nominal_field21203.clone(), source_ml_source2_nominal_field21205.clone(), source_ml_source2_nominal_field21207.clone(), source_ml_source2_nominal_field21209.clone())
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
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
                                                } else {
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                }
                                            }
                                        }
                                    }
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
pub fn resume_java_script_object_from_entries_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectFromEntriesState,
    identity: self::ml::Big,
) -> self::JavaScriptObjectFromEntriesTransition {
    match state.clone() {
        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            {
                let request =
                    self::step_java_script_object_from_entries(heap.clone(), state.clone());
                match request.clone() {
                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesAllocate(_, _, _) => {
                        {
                            let allocated = self::insert_java_script_ordinary_object(heap.clone(), identity.clone(), shared_constructor_field2.clone());
                            if !((({ let ml_reference_value1066 = allocated.borrow().clone(); ml_reference_value1066 }).0).2) {
                                self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(Box::new((({ let ml_reference_value1067 = allocated.borrow().clone(); ml_reference_value1067 }).0).1), (({ let ml_reference_value1068 = allocated.borrow().clone(); ml_reference_value1068 }).0).0)
                            } else {
                                {
                                    let next = {
                                        let source_ml_source2_nominal_field21221 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field21223 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field21225 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field21227 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field21229 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field21231 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field21233 = shared_constructor_field7.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field21235 = shared_constructor_field8.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field21237 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field21239 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field21241 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field21243 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field21245 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field21247 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field21249 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field21251 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field21253 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override21255 = self::JavaScriptSemanticValue::Object(identity.clone(), false);
                                                                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_field21221.clone()), Box::new(source_ml_source2_nominal_field21223.clone()), Box::new(source_ml_source2_nominal_field21225.clone()), Box::new(source_ml_source2_nominal_field21227.clone()), Box::new(source_ml_source2_nominal_override21255.clone()), self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]), Box::new(source_ml_source2_nominal_field21233.clone()), Box::new(source_ml_source2_nominal_field21235.clone()), Box::new(source_ml_source2_nominal_field21237.clone()), Box::new(source_ml_source2_nominal_field21239.clone()), source_ml_source2_nominal_field21241.clone(), source_ml_source2_nominal_field21243.clone(), source_ml_source2_nominal_field21245.clone(), source_ml_source2_nominal_field21247.clone(), source_ml_source2_nominal_field21249.clone(), source_ml_source2_nominal_field21251.clone(), source_ml_source2_nominal_field21253.clone())
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
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
                                    self::accept_java_script_object_from_entries_iterator((({ let ml_reference_value1069 = allocated.borrow().clone(); ml_reference_value1069 }).0).1, next.clone(), self::begin_java_script_synchronous_iterator((({ let ml_reference_value1070 = allocated.borrow().clone(); ml_reference_value1070 }).0).1, shared_constructor_field1.clone(), shared_constructor_field3.clone()))
                                }
                            }
                        }
                    }
                    _ => {
                        if match request.clone() {
                            self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesResourceLimit(_, _, _) => {
                                true
                            }
                            _ => {
                                false
                            }
                        } {
                            request.clone()
                        } else {
                            self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 115, 112, 111, 110, 115, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 102, 114, 101, 115, 104, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 101, 115, 116]))
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_object_from_entries_call(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectFromEntriesState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptObjectFromEntriesTransition {
    match state.clone() {
        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field1,
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
            let error = {
                let source_ml_source2_logical21264 =
                    self::read_java_script_object_from_entries_state_error(
                        heap.clone(),
                        state.clone(),
                    );
                if !source_ml_source2_logical21264.0.is_empty() {
                    source_ml_source2_logical21264.clone()
                } else {
                    self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        match completion.clone() {
                            self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                                source_ml_source2_data_property_value21262,
                            ) => {
                                let source_ml_source2_data_property_value21262 =
                                    *source_ml_source2_data_property_value21262;
                                source_ml_source2_data_property_value21262.clone()
                            }
                            self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                                source_ml_source2_data_property_value21263,
                            ) => {
                                let source_ml_source2_data_property_value21263 =
                                    *source_ml_source2_data_property_value21263;
                                source_ml_source2_data_property_value21263.clone()
                            }
                        },
                    )
                }
            };
            if ((error != (self::ml_string::Text::from_units(vec![])))
                || match (
                    match state.clone() {
                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                            source_ml_source2_data_property_value21265,
                            _,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value21265
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesCall>,)))
                        .0)
                            .clone(),
                    },
                    None::<()>,
                ) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                })
            {
                self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(
                    Box::new(heap.clone()),
                    if !error.0.is_empty() {
                        error.clone()
                    } else {
                        self::ml_string::Text::from_units(vec![
                            79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114,
                            105, 101, 115, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110,
                            108, 121, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100,
                            32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 99, 104, 105, 108, 100,
                        ])
                    },
                )
            } else {
                if !(match (
                    match state.clone() {
                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                            source_ml_source2_data_property_value21267,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value21267
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,)))
                        .0)
                            .clone(),
                    },
                    None::<()>,
                ) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) {
                    self::accept_java_script_object_from_entries_iterator(
                        heap.clone(),
                        state.clone(),
                        self::resume_java_script_iterator_protocol_call(
                            heap.clone(),
                            (match state.clone() {
                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                                    source_ml_source2_data_property_value21268,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value21268
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptIteratorProtocolState>,)
                                    }))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .expect("a proven present value"),
                            completion.clone(),
                        ),
                    )
                } else {
                    if !(match (
                        match state.clone() {
                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                                source_ml_source2_data_property_value21269,
                                _,
                                _,
                                _,
                                _,
                            ) => ((source_ml_source2_data_property_value21269
                                .clone()
                                .unwrap_or_else(|| {
                                    (None::<self::JavaScriptPrimitiveConversionState>,)
                                }))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::accept_java_script_object_from_entries_key(heap.clone(), state.clone(), self::resume_java_script_property_key_conversion(heap.clone(), (match state.clone() {
                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value21270, _, _, _, _) => {
                                    ((source_ml_source2_data_property_value21270.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                }
                            }).clone().expect("a proven present value"), completion.clone()))
                    } else {
                        match completion.clone() {
                            self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                                value,
                            ) => {
                                let value = *value;
                                if ((shared_constructor_field1
                                    == (self::ml_string::Text::from_units(vec![
                                        101, 110, 116, 114, 121, 32, 107, 101, 121, 32, 103, 101,
                                        116, 116, 101, 114,
                                    ])))
                                    || (shared_constructor_field1
                                        == (self::ml_string::Text::from_units(vec![
                                            101, 110, 116, 114, 121, 32, 118, 97, 108, 117, 101,
                                            32, 103, 101, 116, 116, 101, 114,
                                        ]))))
                                {
                                    self::accept_java_script_object_from_entries_entry_value(
                                        heap.clone(),
                                        state.clone(),
                                        value.clone(),
                                    )
                                } else {
                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 110, 116, 114, 121, 32, 71, 101, 116, 32, 111, 114, 32, 110, 101, 115, 116, 101, 100, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 112, 104, 97, 115, 101]))
                                }
                            }
                            self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                                value_2,
                            ) => {
                                let value_2 = *value_2;
                                self::throw_java_script_object_from_entries(
                                    heap.clone(),
                                    state.clone(),
                                    value_2.clone(),
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
pub fn provide_java_script_object_from_entries_primitive_prototype(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectFromEntriesState,
    prototype: self::JavaScriptSemanticValue,
) -> self::JavaScriptObjectFromEntriesTransition {
    {
        let request = self::step_java_script_object_from_entries(heap.clone(), state.clone());
        if (match request.clone() {
            self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesPrimitiveGet(
                _,
                _,
                _,
                _,
            ) => false,
            _ => true,
        } || match (
            match state.clone() {
                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                    source_ml_source2_data_property_value21279,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => ((source_ml_source2_data_property_value21279
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,)))
                .0)
                    .clone(),
            },
            None::<()>,
        ) {
            (Some(_ml_left), Some(_ml_right)) => false,
            (None, None) => true,
            _ => false,
        }) {
            self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(
                Box::new(heap.clone()),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101,
                    115, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116,
                    111, 116, 121, 112, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                    115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 113, 117, 105, 115, 105, 116,
                    105, 111, 110, 32, 114, 101, 113, 117, 101, 115, 116,
                ]),
            )
        } else {
            self::accept_java_script_object_from_entries_iterator(
                heap.clone(),
                state.clone(),
                self::provide_java_script_iterator_primitive_prototype(
                    heap.clone(),
                    (match state.clone() {
                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                            source_ml_source2_data_property_value21280,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value21280
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,)))
                        .0)
                            .clone(),
                    })
                    .clone()
                    .expect("a proven present value"),
                    prototype.clone(),
                ),
            )
        }
    }
}
