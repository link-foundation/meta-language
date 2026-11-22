// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptConstructionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977;
use super::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978;
use super::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979;
use super::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980;
use super::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop981Result;
use super::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop982Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_collection_constructor_kind;
use super::read_java_script_constructed_collection_allocation_kind;
use super::read_java_script_construction_state_error;
use super::read_java_script_executable_call_function;
use super::read_java_script_primitive_conversion_state_error;
use super::source_ml_source1_temporary_read_java_script_construction_completion_state_error_loop981;
use super::source_ml_source1_temporary_read_java_script_construction_completion_state_error_loop982;
use super::test_java_script_constructed_collection_kind;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_construction_completion_state_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptConstructionState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptConstructionState::ConstructionState(
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            shared_constructor_field13,
            shared_constructor_field14,
            _,
            shared_constructor_field15,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field13 = *shared_constructor_field13;
            let shared_constructor_field14 = *shared_constructor_field14;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field6,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    if ((shared_constructor_field11
                        == (self::ml_string::Text::from_units(vec![
                            98, 111, 117, 110, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110,
                        ])))
                        && !(self::ml_array::includes(
                            &{
                                let ml_reference_value969 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            110, 101, 119, 84, 97, 114, 103, 101, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            105, 110, 105, 116, 105, 97, 108, 105, 122, 101,
                                        ]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value969
                            },
                            &shared_constructor_field10.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )))
                    {
                        self::ml_string::Text::from_units(vec![
                            98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116,
                            105, 111, 110, 32, 101, 110, 116, 101, 114, 115, 32, 111, 110, 108,
                            121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97,
                            115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 111, 114, 32,
                            100, 101, 108, 101, 103, 97, 116, 105, 111, 110, 32, 112, 104, 97, 115,
                            101,
                        ])
                    } else {
                        if !(self::ml_array::includes(
                            &{
                                let ml_reference_value970 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![]),
                                        self::ml_string::Text::from_units(vec![
                                            111, 114, 100, 105, 110, 97, 114, 121,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            97, 114, 114, 97, 121,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            119, 114, 97, 112, 112, 101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            99, 111, 108, 108, 101, 99, 116, 105, 111, 110,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            119, 101, 97, 107, 32, 99, 111, 108, 108, 101, 99, 116,
                                            105, 111, 110,
                                        ]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value970
                            },
                            &shared_constructor_field15.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )) {
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114,
                                101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 97,
                                108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 107, 105, 110, 100,
                            ])
                        } else {
                            {
                                let source_ml_source2_s18757 = if self::ml_array::includes(
                                    &{
                                        let ml_reference_value971 =
                                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                self::ml_string::Text::from_units(vec![
                                                    112, 114, 111, 116, 111, 116, 121, 112, 101,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    112, 114, 111, 116, 111, 116, 121, 112, 101,
                                                    32, 103, 101, 116, 116, 101, 114,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    112, 114, 111, 116, 111, 116, 121, 112, 101,
                                                    32, 102, 97, 108, 108, 98, 97, 99, 107,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    79, 98, 106, 101, 99, 116, 32, 114, 111, 111,
                                                    116,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    97, 108, 108, 111, 99, 97, 116, 101,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    97, 114, 114, 97, 121, 32, 108, 101, 110, 103,
                                                    116, 104,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    97, 114, 114, 97, 121, 32, 101, 108, 101, 109,
                                                    101, 110, 116, 115,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    98, 111, 100, 121,
                                                ]),
                                            ])))
                                            .borrow()
                                            .clone();
                                        ml_reference_value971
                                    },
                                    &shared_constructor_field10.clone(),
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                ) {
                                    {
                                        let expected =
                                            if self::test_java_script_constructed_collection_kind(
                                                &(shared_constructor_field11),
                                            ) {
                                                self::read_java_script_constructed_collection_allocation_kind(&(shared_constructor_field11))
                                            } else {
                                                if (shared_constructor_field11
                                                    == (self::ml_string::Text::from_units(vec![
                                                        65, 114, 114, 97, 121,
                                                    ])))
                                                {
                                                    self::ml_string::Text::from_units(vec![
                                                        97, 114, 114, 97, 121,
                                                    ])
                                                } else {
                                                    if (self::ml_array::includes(&{ let ml_reference_value972 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])]))).borrow().clone(); ml_reference_value972 }, &shared_constructor_field11.clone(), 0f64, false, |a, b| a == b) || ((shared_constructor_field11 == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116]))) && self::test_java_script_semantic_same_value(shared_constructor_field7.clone(), shared_constructor_field8.clone()))) {
                                                    self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121])
                                                }
                                                }
                                            };
                                        if (shared_constructor_field15 != expected) {
                                            self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 116, 104, 101, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 107, 105, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 98, 121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
                                        } else {
                                            if ((((shared_constructor_field11
                                                == (self::ml_string::Text::from_units(vec![
                                                    78, 117, 109, 98, 101, 114,
                                                ])))
                                                && {
                                                    let source_ml_source2_s18773 =
                                                        shared_constructor_field14.clone();
                                                    match source_ml_source2_s18773.clone() {
                                                        self::JavaScriptSemanticValue::Number(
                                                            _,
                                                        ) => false,
                                                        _ => true,
                                                    }
                                                })
                                                || ((shared_constructor_field11
                                                    == (self::ml_string::Text::from_units(
                                                        vec![66, 111, 111, 108, 101, 97, 110],
                                                    )))
                                                    && {
                                                        let source_ml_source2_s18779 =
                                                            shared_constructor_field14.clone();
                                                        match source_ml_source2_s18779.clone() {
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                                    }))
                                                || (((shared_constructor_field11
                                                    == (self::ml_string::Text::from_units(
                                                        vec![83, 116, 114, 105, 110, 103],
                                                    )))
                                                    && {
                                                        let source_ml_source2_s18785 =
                                                            shared_constructor_field14.clone();
                                                        match source_ml_source2_s18785.clone() {
                                                            self::JavaScriptSemanticValue::Text(
                                                                _,
                                                            ) => false,
                                                            _ => true,
                                                        }
                                                    })
                                                    && {
                                                        let source_ml_source2_s18789 =
                                                            shared_constructor_field14.clone();
                                                        match source_ml_source2_s18789.clone() {
                                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                                    }))
                                            {
                                                self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977Return(self::ml_string::Text::from_units(vec![98, 111, 120, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 100, 97, 116, 97, 32, 118, 97, 108, 117, 101]))
                                            } else {
                                                if (((shared_constructor_field11 == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116]))) && self::test_java_script_semantic_same_value(shared_constructor_field7.clone(), shared_constructor_field8.clone())) && (!(self::test_java_script_semantic_same_value(shared_constructor_field14.clone(), ({
                                                    let source_ml_source2_array_read_value18799 = shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_array_read_index18800 = 0f64;
                                                        { let ml_reference_array973 = source_ml_source2_array_read_value18799.clone(); let ml_reference_position973 = source_ml_source2_array_read_index18800; let ml_reference_projection973 = self::ml_array::optional_at(&*ml_reference_array973.borrow(), self::ml_array::number_index(ml_reference_position973)); ml_reference_projection973 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) || {
                                                    let source_ml_source2_s18802 = shared_constructor_field14.clone();
                                                    match source_ml_source2_s18802.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    }
                                                })) {
                                                    self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 115, 101, 108, 102, 45, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 99, 111, 101, 114, 99, 101, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 112, 117, 116]))
                                                } else {
                                                    self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977Next
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977Next
                                };
                                match source_ml_source2_s18757.clone() {
                                    self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977Next => {
                                        if (self::ml_array::includes(&{ let ml_reference_value974 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 101, 108, 101, 109, 101, 110, 116, 115])]))).borrow().clone(); ml_reference_value974 }, &shared_constructor_field10.clone(), 0f64, false, |a, b| a == b) && (shared_constructor_field11 != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121])))) {
                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 112, 104, 97, 115, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 116, 97, 114, 103, 101, 116, 32, 107, 105, 110, 100])
                                        } else {
                                            if (self::ml_array::includes(&{ let ml_reference_value975 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 101, 108, 101, 109, 101, 110, 116, 115]), self::ml_string::Text::from_units(vec![98, 111, 100, 121])]))).borrow().clone(); ml_reference_value975 }, &shared_constructor_field10.clone(), 0f64, false, |a, b| a == b) && {
                                                let source_ml_source2_s18811 = shared_constructor_field12.clone();
                                                match source_ml_source2_s18811.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            }) {
                                                self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 71, 101, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 70, 114, 111, 109, 67, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 111, 111, 116])
                                            } else {
                                                {
                                                    let source_ml_source2_s18873 = if self::ml_array::includes(&{ let ml_reference_value976 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![98, 111, 100, 121]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 101, 108, 101, 109, 101, 110, 116, 115])]))).borrow().clone(); ml_reference_value976 }, &shared_constructor_field10.clone(), 0f64, false, |a, b| a == b) {
                                                        if {
                                                            let source_ml_source2_s18876 = shared_constructor_field13.clone();
                                                            match source_ml_source2_s18876.clone() {
                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            }
                                                        } {
                                                            self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 107, 105, 110, 100, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                        } else {
                                                            {
                                                                let receiver = {
                                                                    let source_ml_source2_array_read_value18884 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index18885 = self::find_java_script_ordinary_object(shared_constructor_field6.clone(), (((match shared_constructor_field13.clone() {
                                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value18883, _) => {
                                                                                Some((source_ml_source2_data_property_value18883.clone(),))
                                                                            }
                                                                            _ => {
                                                                                None::<(self::ml::Big,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0).clone());
                                                                        { let ml_reference_array977 = source_ml_source2_array_read_value18884.clone(); let ml_reference_position977 = source_ml_source2_array_read_index18885; let ml_reference_projection977 = self::ml_array::optional_at(&*ml_reference_array977.borrow(), self::ml_array::number_index(ml_reference_position977)); ml_reference_projection977 }
                                                                    }
                                                                };
                                                                {
                                                                    let brand = if (shared_constructor_field11 == (self::ml_string::Text::from_units(vec![77, 97, 112]))) {
                                                                        self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                    } else {
                                                                        if (shared_constructor_field11 == (self::ml_string::Text::from_units(vec![83, 101, 116]))) {
                                                                            self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                        } else {
                                                                            if (shared_constructor_field11 == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]))) {
                                                                                self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                            } else {
                                                                                if (shared_constructor_field11 == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116]))) {
                                                                                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                                } else {
                                                                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    if ((((match (receiver.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((shared_constructor_field10 == (self::ml_string::Text::from_units(vec![98, 111, 100, 121]))) && ((match receiver.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                    }) != brand))) || ((shared_constructor_field10 != (self::ml_string::Text::from_units(vec![98, 111, 100, 121]))) && match receiver.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    })) || match receiver.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value18903, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18903
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value18904, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18904
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value18905, _) => {
                                                                            source_ml_source2_data_property_value18905
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value18906, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value18906
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value18907, _, _) => {
                                                                            source_ml_source2_data_property_value18907
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value18908, _, _) => {
                                                                            source_ml_source2_data_property_value18908
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value18909, _, _) => {
                                                                            source_ml_source2_data_property_value18909
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value18910, _, _) => {
                                                                            source_ml_source2_data_property_value18910
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value18911, _) => {
                                                                            source_ml_source2_data_property_value18911
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value18912, _, _, _) => {
                                                                            source_ml_source2_data_property_value18912
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value18913, _, _, _) => {
                                                                            source_ml_source2_data_property_value18913
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value18914, _, _, _) => {
                                                                            source_ml_source2_data_property_value18914
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value18915, _, _, _) => {
                                                                            source_ml_source2_data_property_value18915
                                                                        }
                                                                    }) || ((shared_constructor_field10 != (self::ml_string::Text::from_units(vec![98, 111, 100, 121]))) && !(self::test_java_script_semantic_same_value(match receiver.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value18918, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18918 = *source_ml_source2_data_property_value18918;
                                                                            source_ml_source2_data_property_value18918.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value18919, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18919 = *source_ml_source2_data_property_value18919;
                                                                            source_ml_source2_data_property_value18919.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value18920, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18920 = *source_ml_source2_data_property_value18920;
                                                                            source_ml_source2_data_property_value18920.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value18921, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18921 = *source_ml_source2_data_property_value18921;
                                                                            source_ml_source2_data_property_value18921.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value18922, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18922 = *source_ml_source2_data_property_value18922;
                                                                            source_ml_source2_data_property_value18922.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value18923, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18923 = *source_ml_source2_data_property_value18923;
                                                                            source_ml_source2_data_property_value18923.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value18924, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18924 = *source_ml_source2_data_property_value18924;
                                                                            source_ml_source2_data_property_value18924.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value18925, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18925 = *source_ml_source2_data_property_value18925;
                                                                            source_ml_source2_data_property_value18925.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value18926, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18926 = *source_ml_source2_data_property_value18926;
                                                                            source_ml_source2_data_property_value18926.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value18927, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18927 = *source_ml_source2_data_property_value18927;
                                                                            source_ml_source2_data_property_value18927.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source2_data_property_value18928, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18928 = *source_ml_source2_data_property_value18928;
                                                                            source_ml_source2_data_property_value18928.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source2_data_property_value18929, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18929 = *source_ml_source2_data_property_value18929;
                                                                            source_ml_source2_data_property_value18929.clone()
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source2_data_property_value18930, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value18930 = *source_ml_source2_data_property_value18930;
                                                                            source_ml_source2_data_property_value18930.clone()
                                                                        }
                                                                    }, shared_constructor_field12.clone())))) {
                                                                        self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 107, 105, 110, 100, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978Next
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978Next
                                                    };
                                                    match source_ml_source2_s18873.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978Next => {
                                                            {
                                                                let source_ml_source2_s18955 = if !(match (match state.clone() {
                                                                    self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18956, _, _) => {
                                                                        ((source_ml_source2_data_property_value18956.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                    {
                                                                        let error = self::read_java_script_primitive_conversion_state_error(heap.clone(), (match state.clone() {
                                                                            self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18957, _, _) => {
                                                                                ((source_ml_source2_data_property_value18957.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value"));
                                                                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979Return(error.clone())
                                                                        } else {
                                                                            if ((((shared_constructor_field10 != (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116]))) || !(self::ml_array::includes(&{ let ml_reference_value978 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])]))).borrow().clone(); ml_reference_value978 }, &shared_constructor_field11.clone(), 0f64, false, |a, b| a == b))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18961, _, _) => {
                                                                                    ((source_ml_source2_data_property_value18961.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value") {
                                                                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value18962, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value18962 = *source_ml_source2_data_property_value18962;
                                                                                    source_ml_source2_data_property_value18962.clone()
                                                                                }
                                                                            }, ({
                                                                                let source_ml_source2_array_read_value18964 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index18965 = 0f64;
                                                                                    { let ml_reference_array979 = source_ml_source2_array_read_value18964.clone(); let ml_reference_position979 = source_ml_source2_array_read_index18965; let ml_reference_projection979 = self::ml_array::optional_at(&*ml_reference_array979.borrow(), self::ml_array::number_index(ml_reference_position979)); ml_reference_projection979 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18966, _, _) => {
                                                                                    ((source_ml_source2_data_property_value18966.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value") {
                                                                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value18967, _, _) => {
                                                                                    let source_ml_source2_data_property_value18967 = *source_ml_source2_data_property_value18967;
                                                                                    source_ml_source2_data_property_value18967.clone()
                                                                                }
                                                                            }, match program.clone() {
                                                                                self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value18968, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value18968 = *source_ml_source2_data_property_value18968;
                                                                                    source_ml_source2_data_property_value18968.clone()
                                                                                }
                                                                            }))) {
                                                                                self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 105, 114, 115, 116, 45, 97, 114, 103, 117, 109, 101, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                                                            } else {
                                                                                if ((match (match state.clone() {
                                                                                    self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value18973, _, _) => {
                                                                                        ((source_ml_source2_data_property_value18973.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value") {
                                                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value18974, _) => {
                                                                                        source_ml_source2_data_property_value18974.clone()
                                                                                    }
                                                                                }) != (if (shared_constructor_field11 == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) {
                                                                                    self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
                                                                                } else {
                                                                                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                                                                                })) {
                                                                                    self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 109, 101, 114, 105, 99, 32, 111, 114, 32, 116, 101, 120, 116, 32, 104, 105, 110, 116]))
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979Next
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if (shared_constructor_field10 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116]))) {
                                                                        self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979Next
                                                                    }
                                                                };
                                                                match source_ml_source2_s18955.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979Next => {
                                                                        {
                                                                            let source_ml_source2_s19044 = if !(match (match state.clone() {
                                                                                self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19045, _) => {
                                                                                    ((source_ml_source2_data_property_value19045.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                {
                                                                                    let call = (match state.clone() {
                                                                                        self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value19046, _) => {
                                                                                            ((source_ml_source2_data_property_value19046.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value");
                                                                                    match call.clone() {
                                                                                        self::JavaScriptConstructionCall::ConstructionCallRecord(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5) => {
                                                                                            let shared_constructor_field1 = *shared_constructor_field1;
                                                                                            let shared_constructor_field2 = *shared_constructor_field2;
                                                                                            let shared_constructor_field5 = *shared_constructor_field5;
                                                                                            if (((((match call.clone() {
                                                                                                self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, _, _) => {
                                                                                                    false
                                                                                                }
                                                                                            } || {
                                                                                                let source_ml_source2_s19053 = shared_constructor_field1.clone();
                                                                                                match source_ml_source2_s19053.clone() {
                                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                        false
                                                                                                    }
                                                                                                    _ => {
                                                                                                        true
                                                                                                    }
                                                                                                }
                                                                                            }) || !(((match shared_constructor_field1.clone() {
                                                                                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value19058) => {
                                                                                                    Some((source_ml_source2_data_property_value19058,))
                                                                                                }
                                                                                                _ => {
                                                                                                    None::<(bool,)>
                                                                                                }
                                                                                            }).clone().expect("a proven present value")).0)) || (({ let ml_reference_array980 = shared_constructor_field3.clone(); let ml_reference_projection980 = ml_reference_array980.borrow().len() as f64; ml_reference_projection980 }) > (1048576f64))) || !(self::ml_array::includes(&{ let ml_reference_value981 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116]), self::ml_string::Text::from_units(vec![98, 111, 100, 121])]))).borrow().clone(); ml_reference_value981 }, &shared_constructor_field10.clone(), 0f64, false, |a, b| a == b))) || (({
                                                                                                let source_ml_source2_type_query_operand19065 = shared_constructor_field4;
                                                                                                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                                                                                            }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) {
                                                                                                self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 111, 114, 32, 98, 111, 100, 121, 32, 99, 97, 108, 108]))
                                                                                            } else {
                                                                                                {
                                                                                                    let source_ml_source1_temporary_values976 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![shared_constructor_field1.clone(), shared_constructor_field2.clone(), shared_constructor_field5.clone()], { let ml_reference_value982 = shared_constructor_field3.borrow().clone(); ml_reference_value982 })));
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_index976 = self::ml::Big::from_i128(0);
                                                                                                        {
                                                                                                            let source_ml_source2_s19070 = self::source_ml_source1_temporary_read_java_script_construction_completion_state_error_loop981(heap.clone(), source_ml_source1_temporary_values976.clone(), source_ml_source1_temporary_index976.clone());
                                                                                                            match source_ml_source2_s19070.clone() {
                                                                                                                self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop981Result::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop981Done => {
                                                                                                                    if (shared_constructor_field10 == (self::ml_string::Text::from_units(vec![98, 111, 100, 121]))) {
                                                                                                                        if {
                                                                                                                            let source_ml_source2_s19074 = shared_constructor_field7.clone();
                                                                                                                            match source_ml_source2_s19074.clone() {
                                                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                                    false
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                    true
                                                                                                                                }
                                                                                                                            }
                                                                                                                        } {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Return(self::ml_string::Text::from_units(vec![98, 97, 115, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let fn_ = self::read_java_script_executable_call_function(program.clone(), heap.clone(), (((match shared_constructor_field7.clone() {
                                                                                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value19079, _) => {
                                                                                                                                        Some((source_ml_source2_data_property_value19079.clone(),))
                                                                                                                                    }
                                                                                                                                    _ => {
                                                                                                                                        None::<(self::ml::Big,)>
                                                                                                                                    }
                                                                                                                                }).clone().expect("a proven present value")).0).clone());
                                                                                                                                {
                                                                                                                                    let collection = self::test_java_script_constructed_collection_kind(&(shared_constructor_field11));
                                                                                                                                    if (collection && (match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((self::read_java_script_collection_constructor_kind(fn_.clone().expect("a proven present value"))) != shared_constructor_field11))) {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 111, 100, 101, 32, 107, 105, 110, 100]))
                                                                                                                                    } else {
                                                                                                                                        if (((((!(self::ml_array::includes(&{ let ml_reference_value983 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])]))).borrow().clone(); ml_reference_value983 }, &shared_constructor_field11.clone(), 0f64, false, |a, b| a == b)) || !shared_constructor_field4) || !(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), shared_constructor_field7.clone()))) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), shared_constructor_field13.clone()))) || !(self::test_java_script_semantic_same_value(shared_constructor_field5.clone(), shared_constructor_field8.clone()))) || (({ let ml_reference_array984 = shared_constructor_field3.clone(); let ml_reference_projection984 = ml_reference_array984.borrow().len() as f64; ml_reference_projection984 }) != ({ let ml_reference_array985 = shared_constructor_field9.clone(); let ml_reference_projection985 = ml_reference_array985.borrow().len() as f64; ml_reference_projection985 }))) {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Return(self::ml_string::Text::from_units(vec![98, 97, 115, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                                                        } else {
                                                                                                                                            {
                                                                                                                                                let index = 0f64;
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_s19095 = self::source_ml_source1_temporary_read_java_script_construction_completion_state_error_loop982(state.clone(), call.clone(), index);
                                                                                                                                                    match source_ml_source2_s19095.clone() {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop982Result::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop982Done => {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Next
                                                                                                                                                        }
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop982Result::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop982Return(source_ml_source1_temporary_result982) => {
                                                                                                                                                            self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Return(source_ml_source1_temporary_result982.clone())
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
                                                                                                                        if ((shared_constructor_field4 || {
                                                                                                                            let source_ml_source2_s19098 = shared_constructor_field5.clone();
                                                                                                                            match source_ml_source2_s19098.clone() {
                                                                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                                                                    false
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                    true
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }) || ((shared_constructor_field10 == (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]))) && (!(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), shared_constructor_field8.clone())) || (({ let ml_reference_array986 = shared_constructor_field3.clone(); let ml_reference_projection986 = ml_reference_array986.borrow().len() as f64; ml_reference_projection986 }) != (0f64))))) {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 115, 101, 116, 117, 112, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 67, 97, 108, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                                        } else {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                                self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop981Result::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorLoop981Return(source_ml_source1_temporary_result982_2) => {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Return(source_ml_source1_temporary_result982_2.clone())
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
                                                                                if ((shared_constructor_field10 == (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]))) || (shared_constructor_field10 == (self::ml_string::Text::from_units(vec![98, 111, 100, 121])))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 97, 110, 100, 32, 98, 111, 100, 121, 32, 115, 116, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108]))
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Next
                                                                                }
                                                                            };
                                                                            match source_ml_source2_s19044.clone() {
                                                                                self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Next => {
                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                }
                                                                                self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin980Return(source_ml_source1_temporary_result980) => {
                                                                                    source_ml_source1_temporary_result980.clone()
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin979Return(source_ml_source1_temporary_result982_3) => {
                                                                        source_ml_source1_temporary_result982_3.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin978Return(source_ml_source1_temporary_result982_4) => {
                                                            source_ml_source1_temporary_result982_4.clone()
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977::MlSource1TemporaryReadJavaScriptConstructionCompletionStateErrorJoin977Return(source_ml_source1_temporary_result982_5) => {
                                        source_ml_source1_temporary_result982_5.clone()
                                    }
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
pub fn begin_java_script_construction(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    target: self::JavaScriptSemanticValue,
    new_target: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptConstructionTransition {
    {
        let state = {
            let source_ml_source2_constructor_field19112 =
                std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(
                    &{
                        let ml_reference_value987 = arguments_.borrow().clone();
                        ml_reference_value987
                    },
                    0f64,
                    f64::INFINITY,
                )));
            {
                let source_ml_source2_constructor_field19115 =
                    self::JavaScriptSemanticValue::Undefined;
                {
                    let source_ml_source2_constructor_field19116 =
                        self::JavaScriptSemanticValue::Undefined;
                    {
                        let source_ml_source2_constructor_field19117 =
                            self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source2_constructor_field19118 = 0f64;
                            self::JavaScriptConstructionState::ConstructionState(
                                Box::new(target.clone()),
                                Box::new(new_target.clone()),
                                source_ml_source2_constructor_field19112.clone(),
                                self::ml_string::Text::from_units(vec![
                                    116, 97, 114, 103, 101, 116,
                                ]),
                                self::ml_string::Text::from_units(vec![]),
                                Box::new(source_ml_source2_constructor_field19115.clone()),
                                Box::new(source_ml_source2_constructor_field19116.clone()),
                                Box::new(source_ml_source2_constructor_field19117.clone()),
                                source_ml_source2_constructor_field19118,
                                self::ml_string::Text::from_units(vec![]),
                                None::<(Option<self::JavaScriptPrimitiveConversionState>,)>,
                                None::<(Option<self::JavaScriptConstructionCall>,)>,
                                None::<(Option<self::JavaScriptConstructionState>,)>,
                            )
                        }
                    }
                }
            }
        };
        {
            let error = self::read_java_script_construction_state_error(
                program.clone(),
                heap.clone(),
                directory.clone(),
                state.clone(),
            );
            if (error == (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptConstructionTransition::ConstructionContinue(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                )
            } else {
                self::JavaScriptConstructionTransition::ConstructionInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            }
        }
    }
}
