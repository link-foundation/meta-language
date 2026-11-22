// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::JavaScriptSpeciesConstructionCall;
use super::JavaScriptSpeciesConstructionState;
use super::JavaScriptSpeciesConstructionTransition;
use super::adapt_java_script_species_construction;
use super::allocate_java_script_array;
use super::continue_java_script_species_prototype;
use super::insert_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_species_construction_state_error;
use super::resume_java_script_construction_allocation;
use super::resume_java_script_construction_call;
use super::source_ml_source1_temporary_step_java_script_species_construction_loop1112;
use super::step_java_script_construction;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_species_construction(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptSpeciesConstructionState,
) -> self::JavaScriptSpeciesConstructionTransition {
    match state.clone() {
        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
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
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let error = self::read_java_script_species_construction_state_error(
                        program.clone(),
                        heap.clone(),
                        directory.clone(),
                        state.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(
                            Box::new(heap.clone()),
                            error.clone(),
                        )
                    } else {
                        if !(match (
                            match state.clone() {
                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value24665,
                                ) => ((source_ml_source2_data_property_value24665
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptConstructionState>,)
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
                            self::adapt_java_script_species_construction(state.clone(), self::step_java_script_construction(program.clone(), heap.clone(), directory.clone(), (match state.clone() {
                                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value24666) => {
                                        ((source_ml_source2_data_property_value24666.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value")))
                        } else {
                            if !(match (match state.clone() {
                                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24667, _, _) => {
                                        ((source_ml_source2_data_property_value24667.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                    }
                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCall(Box::new(heap.clone()), Box::new(state.clone()), Box::new(match (match state.clone() {
                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24668, _, _) => {
                                            ((source_ml_source2_data_property_value24668.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                        }
                                    }).clone().expect("a proven present value") {
                                        self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(source_ml_source2_data_property_value24669, _, _) => {
                                            let source_ml_source2_data_property_value24669 = *source_ml_source2_data_property_value24669;
                                            source_ml_source2_data_property_value24669.clone()
                                        }
                                    }), Box::new(match (match state.clone() {
                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24670, _, _) => {
                                            ((source_ml_source2_data_property_value24670.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                        }
                                    }).clone().expect("a proven present value") {
                                        self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, source_ml_source2_data_property_value24671, _) => {
                                            let source_ml_source2_data_property_value24671 = *source_ml_source2_data_property_value24671;
                                            source_ml_source2_data_property_value24671.clone()
                                        }
                                    }), match (match state.clone() {
                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24672, _, _) => {
                                            ((source_ml_source2_data_property_value24672.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                        }
                                    }).clone().expect("a proven present value") {
                                        self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, _, source_ml_source2_data_property_value24673) => {
                                            source_ml_source2_data_property_value24673.clone()
                                        }
                                    }, (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![98, 111, 100, 121]))))
                                } else {
                                    if !(self::ml_array::includes(&{ let ml_reference_value1277 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99])]))).borrow().clone(); ml_reference_value1277 }, &shared_constructor_field6.clone(), 0f64, false, |a, b| a == b)) {
                                        {
                                            let source_ml_source2_constructor_field24680 = shared_constructor_field2.clone();
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedImplementation(Box::new(heap.clone()), Box::new(state.clone()), Box::new(source_ml_source2_constructor_field24680.clone()), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<self::ml_string::Text>,)>)
                                        }
                                    } else {
                                        if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 102, 97, 108, 108, 98, 97, 99, 107]))) {
                                            self::continue_java_script_species_prototype(heap.clone(), directory.clone(), state.clone(), shared_constructor_field7.clone())
                                        } else {
                                            if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]))) {
                                                if ((shared_constructor_field6 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]))) && (shared_constructor_field4 > (4294967295f64))) {
                                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]), None::<(Option<self::ml::Big>,)>)
                                                } else {
                                                    {
                                                        let properties = 0f64;
                                                        {
                                                            let source_ml_source1_temporary_values1111 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source1_temporary_index1111 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let properties_2 = self::source_ml_source1_temporary_step_java_script_species_construction_loop1112(properties, source_ml_source1_temporary_values1111.clone(), source_ml_source1_temporary_index1111.clone());
                                                                    if ((({ let ml_reference_array1278 = shared_constructor_field1.clone(); let ml_reference_projection1278 = ml_reference_array1278.borrow().len() as f64; ml_reference_projection1278 }) >= (4096f64)) || ((shared_constructor_field6 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]))) && (properties_2 >= (16384f64)))) {
                                                                        self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionAllocate(Box::new(heap.clone()), Box::new(state.clone()), Box::new(shared_constructor_field7.clone()))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if (shared_constructor_field5 != (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101]))) {
                                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                } else {
                                                    if {
                                                        let source_ml_source2_s24702 = shared_constructor_field2.clone();
                                                        match source_ml_source2_s24702.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    } {
                                                        self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 79, 98, 106, 101, 99, 116]))
                                                    } else {
                                                        {
                                                            let get = self::plan_java_script_ordinary_get(heap.clone(), (((match shared_constructor_field2.clone() {
                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value24707, _) => {
                                                                    Some((source_ml_source2_data_property_value24707.clone(),))
                                                                }
                                                                _ => {
                                                                    None::<(self::ml::Big,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0).clone(), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![112f64, 114f64, 111f64, 116f64, 111f64, 116f64, 121f64, 112f64, 101f64]))), shared_constructor_field2.clone());
                                                            if (((({ let ml_reference_value1279 = get.borrow().clone(); ml_reference_value1279 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new(heap.clone()), (({ let ml_reference_value1280 = get.borrow().clone(); ml_reference_value1280 }).0).3)
                                                            } else {
                                                                if (((({ let ml_reference_value1281 = get.borrow().clone(); ml_reference_value1281 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                    {
                                                                        let call = self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(Box::new((({ let ml_reference_value1282 = get.borrow().clone(); ml_reference_value1282 }).0).2), Box::new((({ let ml_reference_value1283 = get.borrow().clone(); ml_reference_value1283 }).0).4), (({ let ml_reference_value1284 = get.borrow().clone(); ml_reference_value1284 }).0).1);
                                                                        self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCall(Box::new(heap.clone()), Box::new({
                                                                            let source_ml_source2_nominal_field24710 = shared_constructor_field2.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field24712 = shared_constructor_field3.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field24714 = shared_constructor_field4;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field24716 = shared_constructor_field5.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field24718 = shared_constructor_field6.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field24720 = shared_constructor_field7.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field24722 = shared_constructor_field8.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field24724 = shared_constructor_field9.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field24726 = shared_constructor_field10.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field24728 = shared_constructor_field11.clone();
                                                                                                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(Box::new(source_ml_source2_nominal_field24710.clone()), Box::new(source_ml_source2_nominal_field24712.clone()), source_ml_source2_nominal_field24714, self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]), source_ml_source2_nominal_field24718.clone(), Box::new(source_ml_source2_nominal_field24720.clone()), Box::new(source_ml_source2_nominal_field24722.clone()), Some((Some(call.clone()),)), source_ml_source2_nominal_field24726.clone(), source_ml_source2_nominal_field24728.clone())
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }), Box::new((({ let ml_reference_value1285 = get.borrow().clone(); ml_reference_value1285 }).0).2), Box::new((({ let ml_reference_value1286 = get.borrow().clone(); ml_reference_value1286 }).0).4), (({ let ml_reference_value1287 = get.borrow().clone(); ml_reference_value1287 }).0).1, false)
                                                                    }
                                                                } else {
                                                                    if (((({ let ml_reference_value1288 = get.borrow().clone(); ml_reference_value1288 }).0).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                        self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 71, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                                    } else {
                                                                        self::continue_java_script_species_prototype(heap.clone(), directory.clone(), state.clone(), (({ let ml_reference_value1289 = get.borrow().clone(); ml_reference_value1289 }).0).5)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
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
pub fn resume_java_script_species_construction_allocation(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptSpeciesConstructionState,
    identity: self::ml::Big,
) -> self::JavaScriptSpeciesConstructionTransition {
    match state.clone() {
        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
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
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            if !(match (
                match state.clone() {
                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value24732,
                    ) => ((source_ml_source2_data_property_value24732
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }) {
                {
                    let error = self::read_java_script_species_construction_state_error(
                        program.clone(),
                        heap.clone(),
                        directory.clone(),
                        state.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(
                            Box::new(heap.clone()),
                            error.clone(),
                        )
                    } else {
                        self::adapt_java_script_species_construction(state.clone(), self::resume_java_script_construction_allocation(program.clone(), heap.clone(), directory.clone(), (match state.clone() {
                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value24733) => {
                                ((source_ml_source2_data_property_value24733.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone()
                            }
                        }).clone().expect("a proven present value"), identity.clone()))
                    }
                }
            } else {
                {
                    let error_2 = self::read_java_script_species_construction_state_error(
                        program.clone(),
                        heap.clone(),
                        directory.clone(),
                        state.clone(),
                    );
                    if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(
                            Box::new(heap.clone()),
                            error_2.clone(),
                        )
                    } else {
                        if (shared_constructor_field4
                            != (self::ml_string::Text::from_units(vec![
                                97, 108, 108, 111, 99, 97, 116, 101,
                            ])))
                        {
                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 114, 101, 113, 117, 101, 115, 116]))
                        } else {
                            if (shared_constructor_field5
                                == (self::ml_string::Text::from_units(vec![
                                    65, 114, 114, 97, 121, 32, 105, 110, 116, 114, 105, 110, 115,
                                    105, 99,
                                ])))
                            {
                                {
                                    let created = self::allocate_java_script_array(
                                        heap.clone(),
                                        identity.clone(),
                                        shared_constructor_field6.clone(),
                                        shared_constructor_field3,
                                    );
                                    match created.clone() {
                                        self::JavaScriptArrayCompletion::ArrayNormal(heap_2, value) => {
                                            let heap_2 = *heap_2;
                                            let value = *value;
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCompleted(Box::new(heap_2.clone()), Box::new(value.clone()))
                                        }
                                        self::JavaScriptArrayCompletion::ArrayPropertyComplete(heap_3, _, _) => {
                                            let heap_3 = *heap_3;
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new(heap_3.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 117, 115, 101, 32, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptArrayCompletion::ArrayNumberRequest(heap_4, _, _) => {
                                            let heap_4 = *heap_4;
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new(heap_4.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 117, 115, 101, 32, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptArrayCompletion::ArrayDefinitionContinue(heap_5, _) => {
                                            let heap_5 = *heap_5;
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new(heap_5.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 117, 115, 101, 32, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptArrayCompletion::ArrayInvalid(heap_6, reason) => {
                                            let heap_6 = *heap_6;
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new(heap_6.clone()), reason.clone())
                                        }
                                        self::JavaScriptArrayCompletion::ArrayError(heap_7, error_class, reason_2) => {
                                            let heap_7 = *heap_7;
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionError(Box::new(heap_7.clone()), error_class.clone(), reason_2.clone(), None::<(Option<self::ml::Big>,)>)
                                        }
                                        self::JavaScriptArrayCompletion::ArrayResourceLimit(heap_8, reason_3, _) => {
                                            let heap_8 = *heap_8;
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionResourceLimit(Box::new(heap_8.clone()), Box::new(state.clone()), reason_3.clone())
                                        }
                                    }
                                }
                            } else {
                                {
                                    let created_2 = self::insert_java_script_ordinary_object(
                                        heap.clone(),
                                        identity.clone(),
                                        shared_constructor_field6.clone(),
                                    );
                                    if !((({
                                        let ml_reference_value1290 = created_2.borrow().clone();
                                        ml_reference_value1290
                                    })
                                    .0)
                                        .2)
                                    {
                                        self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new((({ let ml_reference_value1291 = created_2.borrow().clone(); ml_reference_value1291 }).0).1), (({ let ml_reference_value1292 = created_2.borrow().clone(); ml_reference_value1292 }).0).0)
                                    } else {
                                        {
                                            let receiver = self::JavaScriptSemanticValue::Object(
                                                identity.clone(),
                                                false,
                                            );
                                            {
                                                let call = self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(Box::new(shared_constructor_field1.clone()), Box::new(receiver.clone()), std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::Number(shared_constructor_field3)])));
                                                self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionContinue(Box::new((({ let ml_reference_value1293 = created_2.borrow().clone(); ml_reference_value1293 }).0).1), Box::new({
                                                    let source_ml_source2_nominal_field24753 = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field24755 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field24757 = shared_constructor_field3;
                                                            {
                                                                let source_ml_source2_nominal_field24759 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field24761 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field24763 = shared_constructor_field6.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field24765 = shared_constructor_field7.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field24767 = shared_constructor_field8.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field24769 = shared_constructor_field9.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field24771 = shared_constructor_field10.clone();
                                                                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(Box::new(source_ml_source2_nominal_field24753.clone()), Box::new(source_ml_source2_nominal_field24755.clone()), source_ml_source2_nominal_field24757, self::ml_string::Text::from_units(vec![98, 111, 100, 121]), source_ml_source2_nominal_field24761.clone(), Box::new(source_ml_source2_nominal_field24763.clone()), Box::new(receiver.clone()), Some((Some(call.clone()),)), source_ml_source2_nominal_field24769.clone(), source_ml_source2_nominal_field24771.clone())
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
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_species_construction_call(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptSpeciesConstructionState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptSpeciesConstructionTransition {
    match state.clone() {
        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
            _,
            _,
            _,
            shared_constructor_field1,
            _,
            _,
            shared_constructor_field2,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            if !(match (
                match state.clone() {
                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value24776,
                    ) => ((source_ml_source2_data_property_value24776
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }) {
                {
                    let error = self::read_java_script_species_construction_state_error(
                        program.clone(),
                        heap.clone(),
                        directory.clone(),
                        state.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(
                            Box::new(heap.clone()),
                            error.clone(),
                        )
                    } else {
                        self::adapt_java_script_species_construction(state.clone(), self::resume_java_script_construction_call(program.clone(), heap.clone(), directory.clone(), (match state.clone() {
                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value24777) => {
                                ((source_ml_source2_data_property_value24777.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone()
                            }
                        }).clone().expect("a proven present value"), completion.clone()))
                    }
                }
            } else {
                {
                    let error_2 = self::read_java_script_species_construction_state_error(
                        program.clone(),
                        heap.clone(),
                        directory.clone(),
                        state.clone(),
                    );
                    if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(
                            Box::new(heap.clone()),
                            error_2.clone(),
                        )
                    } else {
                        if match (
                            match state.clone() {
                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value24778,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value24778
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptSpeciesConstructionCall>,)
                                    }))
                                .0)
                                    .clone(),
                            },
                            None::<()>,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100]))
                        } else {
                            if ((self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value24781) => {
                                    let source_ml_source2_data_property_value24781 = *source_ml_source2_data_property_value24781;
                                    source_ml_source2_data_property_value24781.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value24782) => {
                                    let source_ml_source2_data_property_value24782 = *source_ml_source2_data_property_value24782;
                                    source_ml_source2_data_property_value24782.clone()
                                }
                            })) != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 118, 97, 108, 117, 101]))
                            } else {
                                match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]))) {
                                            self::continue_java_script_species_prototype(heap.clone(), directory.clone(), state.clone(), value.clone())
                                        } else {
                                            if (shared_constructor_field1 != (self::ml_string::Text::from_units(vec![98, 111, 100, 121]))) {
                                                self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 100, 121, 32, 111, 114, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 112, 104, 97, 115, 101]))
                                            } else {
                                                self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionCompleted(Box::new(heap.clone()), Box::new(if match value.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        true
                                                    }
                                                    _ => {
                                                        false
                                                    }
                                                } {
                                                    value.clone()
                                                } else {
                                                    shared_constructor_field2.clone()
                                                }))
                                            }
                                        }
                                    }
                                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                        let value_2 = *value_2;
                                        self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
                                    }
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
pub fn retain_java_script_shared_species_construction(
    previous: self::JavaScriptSpeciesConstructionState,
    operation: self::JavaScriptConstructionState,
) -> self::JavaScriptSpeciesConstructionState {
    match operation.clone() {
        self::JavaScriptConstructionState::ConstructionState(
            _,
            _,
            _,
            shared_constructor_field21,
            shared_constructor_field22,
            shared_constructor_field23,
            shared_constructor_field24,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field23 = *shared_constructor_field23;
            let shared_constructor_field24 = *shared_constructor_field24;
            {
                let prior = match previous.clone() {
                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value24790,
                        _,
                        _,
                    ) => ((source_ml_source2_data_property_value24790
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,)))
                    .0)
                        .clone(),
                };
                {
                    let base = match previous.clone() {
                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                            source_ml_source2_nominal_rest_field24791,
                            source_ml_source2_nominal_rest_field24792,
                            source_ml_source2_nominal_rest_field24793,
                            source_ml_source2_nominal_rest_field24794,
                            source_ml_source2_nominal_rest_field24795,
                            source_ml_source2_nominal_rest_field24796,
                            source_ml_source2_nominal_rest_field24797,
                            _,
                            source_ml_source2_nominal_rest_field24798,
                            source_ml_source2_nominal_rest_field24799,
                        ) => {
                            let source_ml_source2_nominal_rest_field24791 =
                                *source_ml_source2_nominal_rest_field24791;
                            let source_ml_source2_nominal_rest_field24792 =
                                *source_ml_source2_nominal_rest_field24792;
                            let source_ml_source2_nominal_rest_field24796 =
                                *source_ml_source2_nominal_rest_field24796;
                            let source_ml_source2_nominal_rest_field24797 =
                                *source_ml_source2_nominal_rest_field24797;
                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                Box::new(source_ml_source2_nominal_rest_field24791.clone()),
                                Box::new(source_ml_source2_nominal_rest_field24792.clone()),
                                source_ml_source2_nominal_rest_field24793,
                                source_ml_source2_nominal_rest_field24794.clone(),
                                source_ml_source2_nominal_rest_field24795.clone(),
                                Box::new(source_ml_source2_nominal_rest_field24796.clone()),
                                Box::new(source_ml_source2_nominal_rest_field24797.clone()),
                                None::<(Option<self::JavaScriptSpeciesConstructionCall>,)>,
                                source_ml_source2_nominal_rest_field24798.clone(),
                                source_ml_source2_nominal_rest_field24799.clone(),
                            )
                        }
                    };
                    match base.clone() {
                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
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
                            let shared_constructor_field11 = *shared_constructor_field11;
                            let shared_constructor_field12 = *shared_constructor_field12;
                            let shared_constructor_field16 = *shared_constructor_field16;
                            let shared_constructor_field17 = *shared_constructor_field17;
                            {
                                let state = {
                                    let source_ml_source2_nominal_field24805 =
                                        shared_constructor_field11.clone();
                                    {
                                        let source_ml_source2_nominal_field24807 =
                                            shared_constructor_field12.clone();
                                        {
                                            let source_ml_source2_nominal_field24809 =
                                                shared_constructor_field13;
                                            {
                                                let source_ml_source2_nominal_field24811 =
                                                    shared_constructor_field14.clone();
                                                {
                                                    let source_ml_source2_nominal_field24813 =
                                                        shared_constructor_field15.clone();
                                                    {
                                                        let source_ml_source2_nominal_field24815 =
                                                            shared_constructor_field16.clone();
                                                        {
                                                            let source_ml_source2_nominal_field24817 =
                                                                shared_constructor_field17.clone();
                                                            {
                                                                let source_ml_source2_nominal_field24819 =
                                                                    shared_constructor_field18
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field24821 =
                                                                        shared_constructor_field19
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field24823 = shared_constructor_field20.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_override24827 = shared_constructor_field21.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_override24828 = shared_constructor_field22.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_override24829 = shared_constructor_field23.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_override24830 = shared_constructor_field24.clone();
                                                                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(Box::new(source_ml_source2_nominal_field24805.clone()), Box::new(source_ml_source2_nominal_field24807.clone()), source_ml_source2_nominal_field24809, source_ml_source2_nominal_override24827.clone(), source_ml_source2_nominal_override24828.clone(), Box::new(source_ml_source2_nominal_override24829.clone()), Box::new(source_ml_source2_nominal_override24830.clone()), source_ml_source2_nominal_field24819.clone(), Some((Some(self::ml_string::Text::from_units(vec![115, 104, 97, 114, 101, 100, 32, 67, 111, 110, 115, 116, 114, 117, 99, 116, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 49])),)), Some((Some(operation.clone()),)))
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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
                                match state.clone() {
                                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                        let shared_constructor_field1 = *shared_constructor_field1;
                                        let shared_constructor_field2 = *shared_constructor_field2;
                                        let shared_constructor_field6 = *shared_constructor_field6;
                                        let shared_constructor_field7 = *shared_constructor_field7;
                                        {
                                            let call = match operation.clone() {
                                                self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value24831, _) => {
                                                    ((source_ml_source2_data_property_value24831.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                                }
                                            };
                                            if match (call.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                state.clone()
                                            } else {
                                                {
                                                    let source_ml_source2_nominal_field24836 = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field24838 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field24840 = shared_constructor_field3;
                                                            {
                                                                let source_ml_source2_nominal_field24842 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field24844 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field24846 = shared_constructor_field6.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field24848 = shared_constructor_field7.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field24850 = shared_constructor_field8.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field24852 = shared_constructor_field9.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field24854 = shared_constructor_field10.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override24856 = self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(Box::new(match call.clone().expect("a proven present value") {
                                                                                                self::JavaScriptConstructionCall::ConstructionCallRecord(source_ml_source2_data_property_value24832, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value24832 = *source_ml_source2_data_property_value24832;
                                                                                                    source_ml_source2_data_property_value24832.clone()
                                                                                                }
                                                                                            }), Box::new(match call.clone().expect("a proven present value") {
                                                                                                self::JavaScriptConstructionCall::ConstructionCallRecord(_, source_ml_source2_data_property_value24833, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value24833 = *source_ml_source2_data_property_value24833;
                                                                                                    source_ml_source2_data_property_value24833.clone()
                                                                                                }
                                                                                            }), std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value1294 = (match call.clone().expect("a proven present value") {
                                                                                                self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, source_ml_source2_data_property_value24834, _, _) => {
                                                                                                    source_ml_source2_data_property_value24834.clone()
                                                                                                }
                                                                                            }).borrow().clone(); ml_reference_value1294 }, 0f64, f64::INFINITY))));
                                                                                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(Box::new(source_ml_source2_nominal_field24836.clone()), Box::new(source_ml_source2_nominal_field24838.clone()), source_ml_source2_nominal_field24840, source_ml_source2_nominal_field24842.clone(), source_ml_source2_nominal_field24844.clone(), Box::new(source_ml_source2_nominal_field24846.clone()), Box::new(source_ml_source2_nominal_field24848.clone()), Some((Some(source_ml_source2_nominal_override24856.clone()),)), source_ml_source2_nominal_field24852.clone(), source_ml_source2_nominal_field24854.clone())
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
pub fn test_java_script_shared_species_child_call(
    selected: self::JavaScriptSpeciesConstructionCall,
    actual: self::JavaScriptConstructionCall,
) -> bool {
    match actual.clone() {
        self::JavaScriptConstructionCall::ConstructionCallRecord(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            _,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            match selected.clone() {
                self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(
                    callee,
                    receiver,
                    arguments,
                ) => {
                    let callee = *callee;
                    let receiver = *receiver;
                    (((self::test_java_script_semantic_same_value(
                        callee.clone(),
                        shared_constructor_field1.clone(),
                    ) && self::test_java_script_semantic_same_value(
                        receiver.clone(),
                        shared_constructor_field2.clone(),
                    )) && (({
                        let ml_reference_array1295 = arguments.clone();
                        let ml_reference_projection1295 =
                            ml_reference_array1295.borrow().len() as f64;
                        ml_reference_projection1295
                    }) == ({
                        let ml_reference_array1296 = shared_constructor_field3.clone();
                        let ml_reference_projection1296 =
                            ml_reference_array1296.borrow().len() as f64;
                        ml_reference_projection1296
                    }))) && {
                        let ml_values1297 = {
                            let ml_reference_value1298 = arguments.borrow().clone();
                            ml_reference_value1298
                        };
                        let ml_callback1297 = {
                            let actual = actual.clone();
                            std::rc::Rc::new(
                                move |value: self::JavaScriptSemanticValue, index: f64| -> bool {
                                    self::test_java_script_semantic_same_value(value.clone(), ({
                        let source_ml_source2_array_read_value24862 = match actual.clone() {
                            self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, source_ml_source2_data_property_value24861, _, _) => {
                                source_ml_source2_data_property_value24861.clone()
                            }
                        };
                        {
                            let source_ml_source2_array_read_index24863 = index;
                            { let ml_reference_array1299 = source_ml_source2_array_read_value24862.clone(); let ml_reference_position1299 = source_ml_source2_array_read_index24863; let ml_reference_projection1299 = self::ml_array::optional_at(&*ml_reference_array1299.borrow(), self::ml_array::number_index(ml_reference_position1299)); ml_reference_projection1299 }
                        }
                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                },
                            )
                        };
                        ml_values1297.iter().cloned().enumerate().all(
                            |(ml_index1297, ml_value1297)| {
                                ml_callback1297(ml_value1297, (ml_index1297 as f64))
                            },
                        )
                    })
                }
            }
        }
    }
}
