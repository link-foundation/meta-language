// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorClassification;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptConstructionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptConstructionJoin985;
use super::accept_java_script_construction_conversion;
use super::adapt_java_script_bound_construction;
use super::begin_java_script_construction;
use super::begin_java_script_scalar_protocol;
use super::classify_java_script_array_species_constructor;
use super::convert_java_script_semantic_boolean;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_bound_construction_operands;
use super::read_java_script_collection_constructor_kind;
use super::read_java_script_constructed_collection_allocation_kind;
use super::read_java_script_construction_conversion_operation;
use super::read_java_script_construction_mode;
use super::read_java_script_construction_state_error;
use super::read_java_script_executable_call_function;
use super::read_java_script_known_array_constructor_fact;
use super::request_java_script_construction_implementation;
use super::select_java_script_construction_root;
use super::step_java_script_construction_allocation;
use super::test_java_script_constructed_collection_kind;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_construction(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptConstructionState,
) -> self::JavaScriptConstructionTransition {
    match state.clone() {
        self::JavaScriptConstructionState::ConstructionState(
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
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            {
                let error = self::read_java_script_construction_state_error(
                    program.clone(),
                    heap.clone(),
                    directory.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptConstructionTransition::ConstructionInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if !(match (
                        match state.clone() {
                            self::JavaScriptConstructionState::ConstructionState(
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
                                source_ml_source2_data_property_value19364,
                            ) => ((source_ml_source2_data_property_value19364
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
                        self::adapt_java_script_bound_construction(
                            state.clone(),
                            self::step_java_script_construction(
                                program.clone(),
                                heap.clone(),
                                directory.clone(),
                                (match state.clone() {
                                    self::JavaScriptConstructionState::ConstructionState(
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
                                        source_ml_source2_data_property_value19365,
                                    ) => ((source_ml_source2_data_property_value19365
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptConstructionState>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value"),
                            ),
                        )
                    } else {
                        if (({
                            let ml_reference_array991 = shared_constructor_field3.clone();
                            let ml_reference_projection991 =
                                ml_reference_array991.borrow().len() as f64;
                            ml_reference_projection991
                        }) > (4096f64))
                        {
                            self::JavaScriptConstructionTransition::ConstructionResourceLimit(
                                Box::new(heap.clone()),
                                Box::new(state.clone()),
                                self::ml_string::Text::from_units(vec![
                                    99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32,
                                    101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115,
                                    101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 32, 97,
                                    114, 103, 117, 109, 101, 110, 116, 32, 119, 111, 114, 107, 32,
                                    112, 114, 111, 102, 105, 108, 101,
                                ]),
                            )
                        } else {
                            if !(match (
                                match state.clone() {
                                    self::JavaScriptConstructionState::ConstructionState(
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
                                        source_ml_source2_data_property_value19368,
                                        _,
                                    ) => ((source_ml_source2_data_property_value19368
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptConstructionCall>,)
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
                                self::JavaScriptConstructionTransition::ConstructionCall(Box::new(heap.clone()), Box::new(state.clone()), Box::new(match (match state.clone() {
                                    self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19369, _) => {
                                        ((source_ml_source2_data_property_value19369.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value") {
                                    self::JavaScriptConstructionCall::ConstructionCallRecord(source_ml_source2_data_property_value19370, _, _, _, _) => {
                                        let source_ml_source2_data_property_value19370 = *source_ml_source2_data_property_value19370;
                                        source_ml_source2_data_property_value19370.clone()
                                    }
                                }), Box::new(match (match state.clone() {
                                    self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19371, _) => {
                                        ((source_ml_source2_data_property_value19371.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value") {
                                    self::JavaScriptConstructionCall::ConstructionCallRecord(_, source_ml_source2_data_property_value19372, _, _, _) => {
                                        let source_ml_source2_data_property_value19372 = *source_ml_source2_data_property_value19372;
                                        source_ml_source2_data_property_value19372.clone()
                                    }
                                }), match (match state.clone() {
                                    self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19373, _) => {
                                        ((source_ml_source2_data_property_value19373.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value") {
                                    self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, source_ml_source2_data_property_value19374, _, _) => {
                                        source_ml_source2_data_property_value19374.clone()
                                    }
                                }, match (match state.clone() {
                                    self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19375, _) => {
                                        ((source_ml_source2_data_property_value19375.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value") {
                                    self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, source_ml_source2_data_property_value19376, _) => {
                                        source_ml_source2_data_property_value19376
                                    }
                                }, Box::new(match (match state.clone() {
                                    self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19377, _) => {
                                        ((source_ml_source2_data_property_value19377.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value") {
                                    self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, _, source_ml_source2_data_property_value19378) => {
                                        let source_ml_source2_data_property_value19378 = *source_ml_source2_data_property_value19378;
                                        source_ml_source2_data_property_value19378.clone()
                                    }
                                }))
                            } else {
                                if ((shared_constructor_field4
                                    == (self::ml_string::Text::from_units(vec![
                                        116, 97, 114, 103, 101, 116,
                                    ])))
                                    || (shared_constructor_field4
                                        == (self::ml_string::Text::from_units(vec![
                                            110, 101, 119, 84, 97, 114, 103, 101, 116,
                                        ]))))
                                {
                                    {
                                        let candidate = if (shared_constructor_field4
                                            == (self::ml_string::Text::from_units(vec![
                                                116, 97, 114, 103, 101, 116,
                                            ]))) {
                                            shared_constructor_field1.clone()
                                        } else {
                                            shared_constructor_field2.clone()
                                        };
                                        if (match candidate.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => false,
                                            _ => true,
                                        } || !(((match candidate.clone() {
                                            self::JavaScriptSemanticValue::Object(
                                                _,
                                                source_ml_source2_data_property_value19389,
                                            ) => {
                                                Some((source_ml_source2_data_property_value19389,))
                                            }
                                            _ => None::<(bool,)>,
                                        })
                                        .clone()
                                        .expect("a proven present value"))
                                        .0))
                                        {
                                            {
                                                let source_ml_source2_constructor_field19394 =
                                                    if (shared_constructor_field4
                                                        == (self::ml_string::Text::from_units(
                                                            vec![116, 97, 114, 103, 101, 116],
                                                        )))
                                                    {
                                                        self::ml_string::Text::from_units(vec![
                                                            116, 97, 114, 103, 101, 116, 32, 105,
                                                            115, 32, 110, 111, 116, 32, 97, 32, 99,
                                                            111, 110, 115, 116, 114, 117, 99, 116,
                                                            111, 114,
                                                        ])
                                                    } else {
                                                        self::ml_string::Text::from_units(vec![
                                                            110, 101, 119, 84, 97, 114, 103, 101,
                                                            116, 32, 105, 115, 32, 110, 111, 116,
                                                            32, 97, 32, 99, 111, 110, 115, 116,
                                                            114, 117, 99, 116, 111, 114,
                                                        ])
                                                    };
                                                self::JavaScriptConstructionTransition::ConstructionError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), source_ml_source2_constructor_field19394.clone(), None::<(Option<self::ml::Big>,)>)
                                            }
                                        } else {
                                            {
                                                let classification = self::classify_java_script_array_species_constructor(heap.clone(), directory.clone(), candidate.clone());
                                                match classification.clone() {
                                                    self::JavaScriptArrayConstructorClassification::ArrayConstructorNotConstructable => {
                                                        {
                                                            let source_ml_source2_constructor_field19399 = if (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116]))) {
                                                                self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])
                                                            } else {
                                                                self::ml_string::Text::from_units(vec![110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])
                                                            };
                                                            self::JavaScriptConstructionTransition::ConstructionError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), source_ml_source2_constructor_field19399.clone(), None::<(Option<self::ml::Big>,)>)
                                                        }
                                                    }
                                                    _ => {
                                                        {
                                                            let fact = self::read_java_script_known_array_constructor_fact(classification.clone());
                                                            if match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::JavaScriptConstructionTransition::ConstructionNeedAuthority(Box::new(heap.clone()), Box::new(state.clone()), Box::new(match classification.clone() {
                                                                    self::JavaScriptArrayConstructorClassification::ArrayConstructorUnknown(constructor) => {
                                                                        (((constructor.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()).clone().unwrap_or_else(|| candidate.clone())
                                                                    }
                                                                    _ => {
                                                                        candidate.clone()
                                                                    }
                                                                }))
                                                            } else {
                                                                if !(match fact.clone().expect("a proven present value") {
                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value19400, _, _, _) => {
                                                                        source_ml_source2_data_property_value19400
                                                                    }
                                                                }) {
                                                                    {
                                                                        let source_ml_source2_constructor_field19405 = if (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116]))) {
                                                                            self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])
                                                                        };
                                                                        self::JavaScriptConstructionTransition::ConstructionError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), source_ml_source2_constructor_field19405.clone(), None::<(Option<self::ml::Big>,)>)
                                                                    }
                                                                } else {
                                                                    if (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![110, 101, 119, 84, 97, 114, 103, 101, 116]))) {
                                                                        {
                                                                            let mode = shared_constructor_field5.clone();
                                                                            if !(self::ml_array::includes(&{ let ml_reference_value992 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])]))).borrow().clone(); ml_reference_value992 }, &mode.clone(), 0f64, false, |a, b| a == b)) {
                                                                                self::request_java_script_construction_implementation(heap.clone(), state.clone(), &(self::ml_string::Text::from_units(vec![67, 111, 110, 115, 116, 114, 117, 99, 116])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 107, 105, 110, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 111, 119, 110, 32, 100, 101, 114, 105, 118, 101, 100, 32, 111, 114, 32, 101, 120, 111, 116, 105, 99, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 97, 116, 105, 111, 110])))
                                                                            } else {
                                                                                {
                                                                                    let fn_ = self::read_java_script_executable_call_function(program.clone(), heap.clone(), match fact.clone().expect("a proven present value") {
                                                                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(source_ml_source2_data_property_value19409, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value19409.clone()
                                                                                        }
                                                                                    });
                                                                                    {
                                                                                        let kind = self::read_java_script_construction_mode(program.clone(), heap.clone(), fact.clone().expect("a proven present value"));
                                                                                        if (({
                                                                                            let source_ml_source2_comparison_operand19411 = match fact.clone().expect("a proven present value") {
                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value19410) => {
                                                                                                    ((source_ml_source2_data_property_value19410.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_comparison_operand19412 = None::<()>;
                                                                                                !(match (source_ml_source2_comparison_operand19411.clone(), source_ml_source2_comparison_operand19412.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                            }
                                                                                        } && !(self::ml_array::includes(&{ let ml_reference_value993 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])]))).borrow().clone(); ml_reference_value993 }, &kind.clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![])), 0f64, false, |a, b| a == b))) || ((((!(match (kind.clone(), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }) && {
                                                                                            let source_ml_source2_comparison_operand19416 = match fact.clone().expect("a proven present value") {
                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, _, source_ml_source2_data_property_value19415) => {
                                                                                                    ((source_ml_source2_data_property_value19415.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_comparison_operand19417 = None::<()>;
                                                                                                match (source_ml_source2_comparison_operand19416.clone(), source_ml_source2_comparison_operand19417.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                            }
                                                                                        }) && !(match fact.clone().expect("a proven present value") {
                                                                                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value19418, _) => {
                                                                                                source_ml_source2_data_property_value19418
                                                                                            }
                                                                                        })) && !(match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) && !(self::ml_array::includes(&{ let ml_reference_value994 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99])]))).borrow().clone(); ml_reference_value994 }, &match fn_.clone().expect("a proven present value") {
                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value19419, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value19419.clone()
                                                                                            }
                                                                                        }, 0f64, false, |a, b| a == b)))) {
                                                                                            self::request_java_script_construction_implementation(heap.clone(), state.clone(), &(self::ml_string::Text::from_units(vec![110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 107, 105, 110, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 100, 101, 114, 105, 118, 101, 100, 32, 111, 114, 32, 80, 114, 111, 120, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 47, 114, 101, 97, 108, 109, 32, 112, 114, 111, 116, 111, 99, 111, 108])))
                                                                                        } else {
                                                                                            self::JavaScriptConstructionTransition::ConstructionContinue(Box::new(heap.clone()), Box::new({
                                                                                                let source_ml_source2_nominal_field19421 = shared_constructor_field1.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field19423 = shared_constructor_field2.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field19425 = shared_constructor_field3.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field19427 = shared_constructor_field4.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field19429 = shared_constructor_field5.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field19431 = shared_constructor_field6.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field19433 = shared_constructor_field7.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field19435 = shared_constructor_field8.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field19437 = shared_constructor_field9;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field19439 = shared_constructor_field10.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field19441 = shared_constructor_field11.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field19443 = shared_constructor_field12.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field19445 = shared_constructor_field13.clone();
                                                                                                                                                self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19421.clone()), Box::new(source_ml_source2_nominal_field19423.clone()), source_ml_source2_nominal_field19425.clone(), self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101]), source_ml_source2_nominal_field19429.clone(), Box::new(source_ml_source2_nominal_field19431.clone()), Box::new(source_ml_source2_nominal_field19433.clone()), Box::new(source_ml_source2_nominal_field19435.clone()), source_ml_source2_nominal_field19437, source_ml_source2_nominal_field19439.clone(), source_ml_source2_nominal_field19441.clone(), source_ml_source2_nominal_field19443.clone(), source_ml_source2_nominal_field19445.clone())
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
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
                                                                        self::JavaScriptConstructionTransition::ConstructionContinue(Box::new(heap.clone()), Box::new({
                                                                            let source_ml_source2_nominal_field19449 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field19451 = shared_constructor_field2.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field19453 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field19455 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field19457 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field19459 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field19461 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field19463 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field19465 = shared_constructor_field9;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field19467 = shared_constructor_field10.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field19469 = shared_constructor_field11.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field19471 = shared_constructor_field12.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field19473 = shared_constructor_field13.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override19476 = (self::read_java_script_construction_mode(program.clone(), heap.clone(), fact.clone().expect("a proven present value"))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                                                                                                                                self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19449.clone()), Box::new(source_ml_source2_nominal_field19451.clone()), source_ml_source2_nominal_field19453.clone(), self::ml_string::Text::from_units(vec![110, 101, 119, 84, 97, 114, 103, 101, 116]), source_ml_source2_nominal_override19476.clone(), Box::new(source_ml_source2_nominal_field19459.clone()), Box::new(source_ml_source2_nominal_field19461.clone()), Box::new(source_ml_source2_nominal_field19463.clone()), source_ml_source2_nominal_field19465, source_ml_source2_nominal_field19467.clone(), source_ml_source2_nominal_field19469.clone(), source_ml_source2_nominal_field19471.clone(), source_ml_source2_nominal_field19473.clone())
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
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
                                } else {
                                    if ((shared_constructor_field4
                                        == (self::ml_string::Text::from_units(vec![
                                            105, 110, 105, 116, 105, 97, 108, 105, 122, 101,
                                        ])))
                                        && (shared_constructor_field5
                                            == (self::ml_string::Text::from_units(vec![
                                                98, 111, 117, 110, 100, 32, 102, 117, 110, 99, 116,
                                                105, 111, 110,
                                            ]))))
                                    {
                                        {
                                            let operands =
                                                self::read_java_script_bound_construction_operands(
                                                    heap.clone(),
                                                    state.clone(),
                                                );
                                            if (((({
                                                let ml_reference_value995 =
                                                    operands.borrow().clone();
                                                ml_reference_value995
                                            })
                                            .0)
                                                .1)
                                                != (self::ml_string::Text::from_units(vec![])))
                                            {
                                                if (({
                                                    let ml_reference_value996 =
                                                        operands.borrow().clone();
                                                    ml_reference_value996
                                                })
                                                .0)
                                                    .3
                                                {
                                                    self::JavaScriptConstructionTransition::ConstructionResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), (({ let ml_reference_value997 = operands.borrow().clone(); ml_reference_value997 }).0).1)
                                                } else {
                                                    self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap.clone()), (({ let ml_reference_value998 = operands.borrow().clone(); ml_reference_value998 }).0).1)
                                                }
                                            } else {
                                                self::adapt_java_script_bound_construction(
                                                    state.clone(),
                                                    self::begin_java_script_construction(
                                                        program.clone(),
                                                        heap.clone(),
                                                        directory.clone(),
                                                        (({
                                                            let ml_reference_value999 =
                                                                operands.borrow().clone();
                                                            ml_reference_value999
                                                        })
                                                        .0)
                                                            .4,
                                                        (({
                                                            let ml_reference_value1000 =
                                                                operands.borrow().clone();
                                                            ml_reference_value1000
                                                        })
                                                        .0)
                                                            .2,
                                                        (({
                                                            let ml_reference_value1001 =
                                                                operands.borrow().clone();
                                                            ml_reference_value1001
                                                        })
                                                        .0)
                                                            .0,
                                                    ),
                                                )
                                            }
                                        }
                                    } else {
                                        if !(self::ml_array::includes(
                                            &{
                                                let ml_reference_value1002 = (std::rc::Rc::new(
                                                    std::cell::RefCell::new(vec![
                                                        self::ml_string::Text::from_units(vec![
                                                            115, 116, 114, 105, 99, 116,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            111, 114, 100, 105, 110, 97, 114, 121,
                                                            32, 99, 108, 97, 115, 115,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            65, 114, 114, 97, 121,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            78, 117, 109, 98, 101, 114,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            66, 111, 111, 108, 101, 97, 110,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            83, 116, 114, 105, 110, 103,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            79, 98, 106, 101, 99, 116,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            77, 97, 112,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            83, 101, 116,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            87, 101, 97, 107, 77, 97, 112,
                                                        ]),
                                                        self::ml_string::Text::from_units(vec![
                                                            87, 101, 97, 107, 83, 101, 116,
                                                        ]),
                                                    ]),
                                                ))
                                                .borrow()
                                                .clone();
                                                ml_reference_value1002
                                            },
                                            &shared_constructor_field5.clone(),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        )) {
                                            self::request_java_script_construction_implementation(
                                                heap.clone(),
                                                state.clone(),
                                                &(self::ml_string::Text::from_units(vec![
                                                    67, 111, 110, 115, 116, 114, 117, 99, 116,
                                                ])),
                                                &(self::ml_string::Text::from_units(vec![
                                                    116, 104, 101, 32, 97, 99, 116, 117, 97, 108,
                                                    32, 99, 111, 110, 115, 116, 114, 117, 99, 116,
                                                    111, 114, 32, 107, 105, 110, 100, 32, 114, 101,
                                                    113, 117, 105, 114, 101, 115, 32, 105, 116,
                                                    115, 32, 111, 119, 110, 32, 100, 101, 114, 105,
                                                    118, 101, 100, 32, 111, 114, 32, 101, 120, 111,
                                                    116, 105, 99, 32, 105, 109, 112, 108, 101, 109,
                                                    101, 110, 116, 97, 116, 105, 111, 110,
                                                ])),
                                            )
                                        } else {
                                            if (shared_constructor_field4
                                                == (self::ml_string::Text::from_units(vec![
                                                    105, 110, 105, 116, 105, 97, 108, 105, 122, 101,
                                                ])))
                                            {
                                                {
                                                    let source_ml_source2_s19489 = if self::test_java_script_constructed_collection_kind(&(shared_constructor_field5)) {
                                                        {
                                                            let target = shared_constructor_field1.clone();
                                                            match target.clone() {
                                                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                                                    {
                                                                        let fn_2 = self::read_java_script_executable_call_function(program.clone(), heap.clone(), identity.clone());
                                                                        if match (fn_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                            self::MlSource1TemporaryStepJavaScriptConstructionJoin985::MlSource1TemporaryStepJavaScriptConstructionJoin985Return(Box::new(self::request_java_script_construction_implementation(heap.clone(), state.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 112, 111, 114, 116, 97, 98, 108, 101, 32, 105, 116, 101, 114, 97, 98, 108, 101, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 98, 111, 100, 121])))))
                                                                        } else {
                                                                            if ((self::read_java_script_collection_constructor_kind(fn_2.clone().expect("a proven present value"))) != shared_constructor_field5) {
                                                                                self::MlSource1TemporaryStepJavaScriptConstructionJoin985::MlSource1TemporaryStepJavaScriptConstructionJoin985Return(Box::new(self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 97, 99, 116, 115, 32, 97, 110, 100, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 99, 111, 100, 101, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 107, 105, 110, 100]))))
                                                                            } else {
                                                                                self::MlSource1TemporaryStepJavaScriptConstructionJoin985::MlSource1TemporaryStepJavaScriptConstructionJoin985Next
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                _ => {
                                                                    self::MlSource1TemporaryStepJavaScriptConstructionJoin985::MlSource1TemporaryStepJavaScriptConstructionJoin985Return(Box::new(self::JavaScriptConstructionTransition::ConstructionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 98, 106, 101, 99, 116]))))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryStepJavaScriptConstructionJoin985::MlSource1TemporaryStepJavaScriptConstructionJoin985Next
                                                    };
                                                    match source_ml_source2_s19489.clone() {
                                                        self::MlSource1TemporaryStepJavaScriptConstructionJoin985::MlSource1TemporaryStepJavaScriptConstructionJoin985Next => {
                                                            {
                                                                let input = ({
                                                                    let source_ml_source2_array_read_value19496 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index19497 = 0f64;
                                                                        { let ml_reference_array1003 = source_ml_source2_array_read_value19496.clone(); let ml_reference_position1003 = source_ml_source2_array_read_index19497; let ml_reference_projection1003 = self::ml_array::optional_at(&*ml_reference_array1003.borrow(), self::ml_array::number_index(ml_reference_position1003)); ml_reference_projection1003 }
                                                                    }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                if ((shared_constructor_field5 == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116]))) && self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), shared_constructor_field2.clone())) {
                                                                    match input.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            self::JavaScriptConstructionTransition::ConstructionCompleted(Box::new(heap.clone()), Box::new(input.clone()))
                                                                        }
                                                                        _ => {
                                                                            self::select_java_script_construction_root(heap.clone(), directory.clone(), {
                                                                                let source_ml_source2_nominal_field19505 = shared_constructor_field1.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field19507 = shared_constructor_field2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field19509 = shared_constructor_field3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field19511 = shared_constructor_field4.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field19513 = shared_constructor_field5.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field19515 = shared_constructor_field6.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field19517 = shared_constructor_field7.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field19519 = shared_constructor_field8.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field19521 = shared_constructor_field9;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field19523 = shared_constructor_field10.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field19525 = shared_constructor_field11.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field19527 = shared_constructor_field12.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field19529 = shared_constructor_field13.clone();
                                                                                                                                self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19505.clone()), Box::new(source_ml_source2_nominal_field19507.clone()), source_ml_source2_nominal_field19509.clone(), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 114, 111, 111, 116]), source_ml_source2_nominal_field19513.clone(), Box::new(source_ml_source2_nominal_field19515.clone()), Box::new(source_ml_source2_nominal_field19517.clone()), Box::new(input.clone()), source_ml_source2_nominal_field19521, self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114]), source_ml_source2_nominal_field19525.clone(), source_ml_source2_nominal_field19527.clone(), source_ml_source2_nominal_field19529.clone())
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }, true)
                                                                        }
                                                                    }
                                                                } else {
                                                                    if ((shared_constructor_field5 == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]))) || (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])))) {
                                                                        if (({ let ml_reference_array1004 = shared_constructor_field3.clone(); let ml_reference_projection1004 = ml_reference_array1004.borrow().len() as f64; ml_reference_projection1004 }) == (0f64)) {
                                                                            self::JavaScriptConstructionTransition::ConstructionContinue(Box::new(heap.clone()), Box::new({
                                                                                let source_ml_source2_nominal_field19543 = shared_constructor_field1.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field19545 = shared_constructor_field2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field19547 = shared_constructor_field3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field19549 = shared_constructor_field4.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field19551 = shared_constructor_field5.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field19553 = shared_constructor_field6.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field19555 = shared_constructor_field7.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field19557 = shared_constructor_field8.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field19559 = shared_constructor_field9;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field19561 = shared_constructor_field10.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field19563 = shared_constructor_field11.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field19565 = shared_constructor_field12.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field19567 = shared_constructor_field13.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_override19571 = if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]))) {
                                                                                                                                        self::JavaScriptSemanticValue::Number(0f64)
                                                                                                                                    } else {
                                                                                                                                        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                                                                                                                                    };
                                                                                                                                    self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19543.clone()), Box::new(source_ml_source2_nominal_field19545.clone()), source_ml_source2_nominal_field19547.clone(), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101]), source_ml_source2_nominal_field19551.clone(), Box::new(source_ml_source2_nominal_field19553.clone()), Box::new(source_ml_source2_nominal_field19555.clone()), Box::new(source_ml_source2_nominal_override19571.clone()), source_ml_source2_nominal_field19559, self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114]), source_ml_source2_nominal_field19563.clone(), source_ml_source2_nominal_field19565.clone(), source_ml_source2_nominal_field19567.clone())
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
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
                                                                            self::accept_java_script_construction_conversion(heap.clone(), directory.clone(), state.clone(), self::begin_java_script_scalar_protocol(heap.clone(), input.clone(), match program.clone() {
                                                                                self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value19572, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value19572 = *source_ml_source2_data_property_value19572;
                                                                                    source_ml_source2_data_property_value19572.clone()
                                                                                }
                                                                            }, &(self::read_java_script_construction_conversion_operation(state.clone()))))
                                                                        }
                                                                    } else {
                                                                        if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110]))) {
                                                                            self::JavaScriptConstructionTransition::ConstructionContinue(Box::new(heap.clone()), Box::new({
                                                                                let source_ml_source2_nominal_field19576 = shared_constructor_field1.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field19578 = shared_constructor_field2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field19580 = shared_constructor_field3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field19582 = shared_constructor_field4.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field19584 = shared_constructor_field5.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field19586 = shared_constructor_field6.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field19588 = shared_constructor_field7.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field19590 = shared_constructor_field8.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field19592 = shared_constructor_field9;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field19594 = shared_constructor_field10.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field19596 = shared_constructor_field11.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field19598 = shared_constructor_field12.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field19600 = shared_constructor_field13.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_override19604 = self::JavaScriptSemanticValue::Boolean(self::convert_java_script_semantic_boolean(input.clone()));
                                                                                                                                    self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19576.clone()), Box::new(source_ml_source2_nominal_field19578.clone()), source_ml_source2_nominal_field19580.clone(), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101]), source_ml_source2_nominal_field19584.clone(), Box::new(source_ml_source2_nominal_field19586.clone()), Box::new(source_ml_source2_nominal_field19588.clone()), Box::new(source_ml_source2_nominal_override19604.clone()), source_ml_source2_nominal_field19592, self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114]), source_ml_source2_nominal_field19596.clone(), source_ml_source2_nominal_field19598.clone(), source_ml_source2_nominal_field19600.clone())
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
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
                                                                            self::JavaScriptConstructionTransition::ConstructionContinue(Box::new(heap.clone()), Box::new({
                                                                                let source_ml_source2_nominal_field19610 = shared_constructor_field1.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field19612 = shared_constructor_field2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field19614 = shared_constructor_field3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field19616 = shared_constructor_field4.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field19618 = shared_constructor_field5.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field19620 = shared_constructor_field6.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field19622 = shared_constructor_field7.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field19624 = shared_constructor_field8.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field19626 = shared_constructor_field9;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field19628 = shared_constructor_field10.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field19630 = shared_constructor_field11.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field19632 = shared_constructor_field12.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field19634 = shared_constructor_field13.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_override19637 = if self::test_java_script_constructed_collection_kind(&(shared_constructor_field5)) {
                                                                                                                                        self::read_java_script_constructed_collection_allocation_kind(&(shared_constructor_field5))
                                                                                                                                    } else {
                                                                                                                                        if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) {
                                                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])
                                                                                                                                        } else {
                                                                                                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121])
                                                                                                                                        }
                                                                                                                                    };
                                                                                                                                    self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field19610.clone()), Box::new(source_ml_source2_nominal_field19612.clone()), source_ml_source2_nominal_field19614.clone(), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101]), source_ml_source2_nominal_field19618.clone(), Box::new(source_ml_source2_nominal_field19620.clone()), Box::new(source_ml_source2_nominal_field19622.clone()), Box::new(source_ml_source2_nominal_field19624.clone()), source_ml_source2_nominal_field19626, source_ml_source2_nominal_override19637.clone(), source_ml_source2_nominal_field19630.clone(), source_ml_source2_nominal_field19632.clone(), source_ml_source2_nominal_field19634.clone())
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
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
                                                        self::MlSource1TemporaryStepJavaScriptConstructionJoin985::MlSource1TemporaryStepJavaScriptConstructionJoin985Return(source_ml_source1_temporary_result985) => {
                                                            let source_ml_source1_temporary_result985 = *source_ml_source1_temporary_result985;
                                                            source_ml_source1_temporary_result985.clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::step_java_script_construction_allocation(
                                                    heap.clone(),
                                                    directory.clone(),
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
