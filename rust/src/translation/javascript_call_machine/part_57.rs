// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConcatenationCall;
use super::JavaScriptArrayConcatenationState;
use super::JavaScriptArrayConcatenationTransition;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::begin_java_script_array_species_selection;
use super::continue_java_script_array_concatenation_final_set;
use super::continue_java_script_array_concatenation_length;
use super::continue_java_script_array_concatenation_species;
use super::copy_java_script_array_constructor_fact;
use super::finish_java_script_scalar_primitive;
use super::ml_string;
use super::read_java_script_array_concatenation_state_error;
use super::request_java_script_array_concatenation_allocation;
use super::step_java_script_array_concatenation_items;
use super::step_java_script_array_species_selection;
use super::step_java_script_object_spread;
use super::step_java_script_scalar_protocol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_concatenation(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArrayConcatenationState,
) -> self::JavaScriptArrayConcatenationTransition {
    match state.clone() {
        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
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
            _,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
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
                    if !(match (
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
                                source_ml_source2_data_property_value9910,
                            ) => ((source_ml_source2_data_property_value9910
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
                    }) {
                        self::JavaScriptArrayConcatenationTransition::ArrayConcatenationCall(Box::new(heap.clone()), Box::new(state.clone()), Box::new(match (match state.clone() {
                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9911) => {
                                ((source_ml_source2_data_property_value9911.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                            }
                        }).clone().expect("a proven present value") {
                            self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(source_ml_source2_data_property_value9912, _, _) => {
                                let source_ml_source2_data_property_value9912 = *source_ml_source2_data_property_value9912;
                                source_ml_source2_data_property_value9912.clone()
                            }
                        }), Box::new(match (match state.clone() {
                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9913) => {
                                ((source_ml_source2_data_property_value9913.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                            }
                        }).clone().expect("a proven present value") {
                            self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(_, source_ml_source2_data_property_value9914, _) => {
                                let source_ml_source2_data_property_value9914 = *source_ml_source2_data_property_value9914;
                                source_ml_source2_data_property_value9914.clone()
                            }
                        }), match (match state.clone() {
                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9915) => {
                                ((source_ml_source2_data_property_value9915.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                            }
                        }).clone().expect("a proven present value") {
                            self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(_, _, source_ml_source2_data_property_value9916) => {
                                source_ml_source2_data_property_value9916.clone()
                            }
                        })
                    } else {
                        if (shared_constructor_field3
                            == (self::ml_string::Text::from_units(vec![
                                105, 110, 105, 116, 105, 97, 108, 105, 122, 101,
                            ])))
                        {
                            self::continue_java_script_array_concatenation_species(
                                heap.clone(),
                                state.clone(),
                                self::begin_java_script_array_species_selection(
                                    heap.clone(),
                                    directory.clone(),
                                    shared_constructor_field1.clone(),
                                    0f64,
                                ),
                            )
                        } else {
                            if (shared_constructor_field3
                                == (self::ml_string::Text::from_units(vec![
                                    115, 112, 101, 99, 105, 101, 115,
                                ])))
                            {
                                if match (match state.clone() {
                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9922, _, _, _, _, _) => {
                                        ((source_ml_source2_data_property_value9922.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                    }
                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                } else {
                                    self::continue_java_script_array_concatenation_species(heap.clone(), state.clone(), self::step_java_script_array_species_selection(heap.clone(), directory.clone(), (match state.clone() {
                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9923, _, _, _, _, _) => {
                                            ((source_ml_source2_data_property_value9923.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                        }
                                    }).clone().expect("a proven present value")))
                                }
                            } else {
                                if (shared_constructor_field3
                                    == (self::ml_string::Text::from_units(vec![
                                        97, 108, 108, 111, 99, 97, 116, 101,
                                    ])))
                                {
                                    self::request_java_script_array_concatenation_allocation(
                                        heap.clone(),
                                        state.clone(),
                                    )
                                } else {
                                    if (shared_constructor_field3
                                        == (self::ml_string::Text::from_units(vec![
                                            99, 111, 110, 115, 116, 114, 117, 99, 116,
                                        ])))
                                    {
                                        {
                                            let constructor = if {
                                                let source_ml_source2_own_property_operands9928 = (
                                                    state.clone(),
                                                    self::ml_string::Text::from_units(vec![
                                                        99, 111, 110, 115, 116, 114, 117, 99, 116,
                                                        111, 114,
                                                    ]),
                                                );
                                                {
                                                    let source_ml_source2_own_property_key9929 = ((source_ml_source2_own_property_operands9928.clone()).1,);
                                                    match (source_ml_source2_own_property_operands9928.clone()).0 {
                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_owned_constructor_field9930, source_ml_source2_owned_constructor_field9931, source_ml_source2_owned_constructor_field9932, source_ml_source2_owned_constructor_field9933, source_ml_source2_owned_constructor_field9934, source_ml_source2_owned_constructor_field9935, source_ml_source2_owned_constructor_field9936) => {
                                                            (((((((((((((((((((((((((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![36]))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 114, 101, 97, 100, 97, 98, 108, 101, 75, 101, 121])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 75, 101, 121])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![105, 116, 101, 109, 80, 111, 115, 105, 116, 105, 111, 110])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![105, 116, 101, 109])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![105, 116, 101, 109, 83, 116, 97, 114, 116])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 114, 101, 97, 100])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116])))) || (((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![112, 104, 97, 115, 101])))) || ((((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && match source_ml_source2_owned_constructor_field9930.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && match source_ml_source2_owned_constructor_field9931.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 80, 114, 111, 116, 111, 116, 121, 112, 101]))) && match source_ml_source2_owned_constructor_field9932.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) && match source_ml_source2_owned_constructor_field9933.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 70, 97, 99, 116]))) && match source_ml_source2_owned_constructor_field9934.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116]))) && match source_ml_source2_owned_constructor_field9935.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key9929.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) && match source_ml_source2_owned_constructor_field9936.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }))
                                                        }
                                                    }
                                                }
                                            } {
                                                match state.clone() {
                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9937, _, _, _) => {
                                                        ((source_ml_source2_data_property_value9937.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                    }
                                                }
                                            } else {
                                                None::<self::JavaScriptSemanticValue>
                                            };
                                            {
                                                let fact = match state.clone() {
                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9938, _, _) => {
                                                        ((source_ml_source2_data_property_value9938.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConstructorFact>,))).0).clone()
                                                    }
                                                };
                                                if (match (constructor.clone(), None::<()>) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                } || match (fact.clone(), None::<()>) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                }) {
                                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
                                                } else {
                                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationConstruct(Box::new(heap.clone()), Box::new(state.clone()), Box::new(constructor.clone().expect("a proven present value")), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::Number(0f64)])), Box::new(self::copy_java_script_array_constructor_fact(fact.clone().expect("a proven present value"))))
                                                }
                                            }
                                        }
                                    } else {
                                        if (shared_constructor_field3
                                            == (self::ml_string::Text::from_units(vec![
                                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                                            ])))
                                        {
                                            if match (match state.clone() {
                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9941, _, _, _, _, _, _) => {
                                                    ((source_ml_source2_data_property_value9941.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                }
                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                            } else {
                                                self::continue_java_script_array_concatenation_length(heap.clone(), state.clone(), self::step_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9942, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value9942.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value"), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                            }
                                        } else {
                                            if (shared_constructor_field3
                                                == (self::ml_string::Text::from_units(vec![
                                                    112, 114, 105, 109, 105, 116, 105, 118, 101,
                                                ])))
                                            {
                                                self::continue_java_script_array_concatenation_length(heap.clone(), state.clone(), self::finish_java_script_scalar_primitive(&(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])), shared_constructor_field2.clone()))
                                            } else {
                                                if (shared_constructor_field3
                                                    == (self::ml_string::Text::from_units(vec![
                                                        102, 105, 110, 105, 115, 104,
                                                    ])))
                                                {
                                                    if match (match state.clone() {
                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9948, _) => {
                                                            ((source_ml_source2_data_property_value9948.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                                                        }
                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 102, 105, 110, 97, 108, 32, 83, 101, 116]))
                                                    } else {
                                                        self::continue_java_script_array_concatenation_final_set(state.clone(), self::step_java_script_object_spread(heap.clone(), (match state.clone() {
                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9949, _) => {
                                                                ((source_ml_source2_data_property_value9949.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                                                            }
                                                        }).clone().expect("a proven present value")))
                                                    }
                                                } else {
                                                    self::step_java_script_array_concatenation_items(
                                                        heap.clone(),
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
}
