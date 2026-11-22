// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result;
use super::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10;
use super::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result;
use super::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result;
use super::apply_java_script_property_descriptor_fields;
use super::construct_java_script_semantic_text_units;
use super::create_java_script_property_descriptor;
use super::create_java_script_property_transition;
use super::find_java_script_property_descriptor_entry;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_property_descriptor_kind;
use super::read_java_script_property_descriptor_writes;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_find_java_script_heap_directory_object_loop12;
use super::source_ml_temporary_read_java_script_literal_property_index_loop11;
use super::source_ml_temporary_test_java_script_heap_directory_symbol_loop13;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn plan_java_script_property_definition(
    exists: bool,
    extensible: bool,
    current: std::rc::Rc<
        std::cell::RefCell<(
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
        )>,
    >,
    proposed: std::rc::Rc<
        std::cell::RefCell<(
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
        )>,
    >,
) -> std::rc::Rc<
    std::cell::RefCell<(
        std::rc::Rc<
            std::cell::RefCell<(
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
            )>,
        >,
        self::ml_string::Text,
        self::ml_string::Text,
        std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
    )>,
> {
    {
        let error = self::read_java_script_property_descriptor_error(proposed.clone());
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                self::create_java_script_property_transition(
                    &(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])),
                    &(error),
                    current.clone(),
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())),
                )
            } else if exists
                && !(self::test_complete_java_script_property_descriptor(current.clone()))
            {
                self::create_java_script_property_transition(
                    &(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])),
                    &(self::ml_string::Text::from_units(vec![
                        73, 110, 118, 97, 108, 105, 100, 67, 117, 114, 114, 101, 110, 116, 68, 101,
                        115, 99, 114, 105, 112, 116, 111, 114,
                    ])),
                    current.clone(),
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())),
                )
            } else {
                {
                    let kind = self::read_java_script_property_descriptor_kind(proposed.clone());
                    if exists {
                        if ((((!(({ proposed.borrow().clone() }).1)
                            && !(({ proposed.borrow().clone() }).3))
                            && !(({ proposed.borrow().clone() }).9))
                            && !(({ proposed.borrow().clone() }).11))
                            && !(({ proposed.borrow().clone() }).5))
                            && !(({ proposed.borrow().clone() }).7)
                        {
                            self::create_java_script_property_transition(
                                &(self::ml_string::Text::from_units(vec![107, 101, 101, 112])),
                                &(self::ml_string::Text::from_units(vec![])),
                                current.clone(),
                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                    self::ml_string::Text,
                                >::new(
                                ))),
                            )
                        } else {
                            {
                                let original = self::read_java_script_property_descriptor_kind(
                                    current.clone(),
                                );
                                {
                                    let source_ml_source1_s532 = if (({ current.borrow().clone() })
                                        .0)
                                    {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Next
                                    } else {
                                        let ml_product_condition_6 = { proposed.borrow().clone() };
                                        if ml_product_condition_6 {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                        } else {
                                            let ml_product_condition_5 =
                                                { proposed.borrow().clone() };
                                            if ml_product_condition_5 {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                            } else if (kind
                                                != (self::ml_string::Text::from_units(vec![
                                                    103, 101, 110, 101, 114, 105, 99,
                                                ])))
                                                && (kind != original)
                                            {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                            } else if original
                                                == (self::ml_string::Text::from_units(vec![
                                                    97, 99, 99, 101, 115, 115, 111, 114,
                                                ]))
                                            {
                                                {
                                                    let ml_product_condition_4 =
                                                        { proposed.borrow().clone() };
                                                    if ml_product_condition_4 {
                                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                    } else {
                                                        let ml_product_condition_3 =
                                                            { proposed.borrow().clone() };
                                                        if ml_product_condition_3 {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                        } else {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Next
                                                        }
                                                    }
                                                }
                                            } else if (({ current.borrow().clone() }).10) {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Next
                                            } else {
                                                let ml_product_condition_2 =
                                                    { proposed.borrow().clone() };
                                                if ml_product_condition_2 {
                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                } else {
                                                    let ml_product_condition_1 =
                                                        { proposed.borrow().clone() };
                                                    if ml_product_condition_1 {
                                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(self::create_java_script_property_transition(&(if self::test_java_script_semantic_same_value(({ proposed.borrow().clone() }).8, ({ current.borrow().clone() }).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }), &(self::ml_string::Text::from_units(vec![])), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                    } else {
                                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Next
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    match source_ml_source1_s532 {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Next => {
                                            {
                                                let base_2 = if (kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original) {
                                                    self::create_java_script_property_descriptor(&(kind), ({ current.borrow().clone() }).0, ({ current.borrow().clone() }).2)
                                                } else {
                                                    current.clone()
                                                };
                                                {
                                                    let descriptor_2 = self::apply_java_script_property_descriptor_fields(base_2, proposed.clone());
                                                    {
                                                        let conversion = (kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original);
                                                        self::create_java_script_property_transition(&(if conversion {
                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![117, 112, 100, 97, 116, 101])
                                                        }), &(self::ml_string::Text::from_units(vec![])), descriptor_2.clone(), self::read_java_script_property_descriptor_writes(if conversion {
                                                            descriptor_2.clone()
                                                        } else {
                                                            proposed.clone()
                                                        }))
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10::MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(source_ml_temporary_result10) => {
                                            source_ml_temporary_result10.clone()
                                        }
                                    }
                                }
                            }
                        }
                    } else if extensible {
                        {
                            let base =
                                self::create_java_script_property_descriptor(&(kind), false, false);
                            {
                                let descriptor = self::apply_java_script_property_descriptor_fields(
                                    base,
                                    proposed.clone(),
                                );
                                self::create_java_script_property_transition(
                                    &(self::ml_string::Text::from_units(vec![
                                        99, 114, 101, 97, 116, 101,
                                    ])),
                                    &(self::ml_string::Text::from_units(vec![])),
                                    descriptor.clone(),
                                    self::read_java_script_property_descriptor_writes(
                                        descriptor.clone(),
                                    ),
                                )
                            }
                        }
                    } else {
                        self::create_java_script_property_transition(
                            &(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])),
                            &(self::ml_string::Text::from_units(vec![])),
                            current.clone(),
                            std::rc::Rc::new(std::cell::RefCell::new(
                                Vec::<self::ml_string::Text>::new(),
                            )),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_literal_property_index(
    key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> f64 {
    if (({
        let ml_reference_array94 = key.clone();
        ml_reference_array94.borrow().len() as f64
    }) == 0f64)
        || (({
            let ml_reference_array95 = key.clone();
            ml_reference_array95.borrow().len() as f64
        }) > 10f64)
    {
        -1f64
    } else {
        if (({
            let ml_reference_array96 = key.clone();
            ml_reference_array96.borrow().len() as f64
        }) > 1f64)
            && {
                let source_ml_source1_comparison_operand535 = {
                    let source_ml_source1_array_read_index534 = 0f64;
                    {
                        let ml_reference_array97 = key.clone();
                        let ml_reference_position97 = source_ml_source1_array_read_index534;
                        self::ml_array::optional_at(
                            &*ml_reference_array97.borrow(),
                            self::ml_array::number_index(ml_reference_position97),
                        )
                    }
                };
                {
                    let source_ml_source1_comparison_operand536 = 48f64;
                    match (
                        source_ml_source1_comparison_operand535,
                        source_ml_source1_comparison_operand536,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            }
        {
            -1f64
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s537 = self::source_ml_temporary_read_java_script_literal_property_index_loop11(key.clone(), value, at);
                        match source_ml_source1_s537 {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(value_2) => {
                                if value_2 <= 4_294_967_294f64 {
                                    value_2
                                } else {
                                    -1f64
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Return(source_ml_temporary_result11) => {
                                source_ml_temporary_result11
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
pub fn find_java_script_heap_directory_object(
    objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>,
    identity: self::ml::Big,
) -> f64 {
    {
        let low = 0f64;
        {
            let high = {
                let ml_reference_array98 = objects.clone();
                ml_reference_array98.borrow().len() as f64
            };
            {
                let source_ml_source1_s538 =
                    self::source_ml_temporary_find_java_script_heap_directory_object_loop12(
                        objects.clone(),
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s538 {
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Done => {
                        -1f64
                    }
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(source_ml_temporary_result12) => {
                        source_ml_temporary_result12
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_heap_directory_symbol(
    symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    identity: self::ml::Big,
) -> bool {
    {
        let low = 0f64;
        {
            let high = {
                let ml_reference_array99 = symbols.clone();
                ml_reference_array99.borrow().len() as f64
            };
            {
                let source_ml_source1_s539 =
                    self::source_ml_temporary_test_java_script_heap_directory_symbol_loop13(
                        symbols.clone(),
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s539 {
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Done => {
                        false
                    }
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Return(source_ml_temporary_result13) => {
                        source_ml_temporary_result13
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_heap_directory_value_error(
    heap: self::JavaScriptOrdinaryHeap,
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    {
        let error = self::read_java_script_semantic_value_error(value.clone());
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                error.clone()
            } else {
                match value.clone() {
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        if self::test_java_script_heap_directory_symbol(
                            match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    _,
                                    source_ml_source1_data_property_value597,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value597.clone(),
                            },
                            identity.clone(),
                        ) {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            self::ml_string::Text::from_units(vec![
                                97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98,
                                111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103,
                                32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101,
                                116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110,
                                116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121,
                            ])
                        }
                    }
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f546,
                        source_ml_source1_f547,
                    ) => {
                        let at = self::find_java_script_heap_directory_object(
                            match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    source_ml_source1_data_property_value605,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value605.clone(),
                            },
                            source_ml_source1_f546.clone(),
                        );
                        if at < 0f64 {
                            self::ml_string::Text::from_units(vec![
                                97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101,
                                99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103,
                                32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101,
                                116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110,
                                116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121,
                            ])
                        } else {
                            {
                                let object = {
                                    let source_ml_source1_array_read_value607 = match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            source_ml_source1_data_property_value606,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source1_data_property_value606.clone(),
                                    };
                                    {
                                        let ml_reference_array100 =
                                            source_ml_source1_array_read_value607;
                                        let ml_reference_position100 = at;
                                        self::ml_array::optional_at(
                                            &*ml_reference_array100.borrow(),
                                            self::ml_array::number_index(ml_reference_position100),
                                        )
                                    }
                                };
                                if !(match (object.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) && ((match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value622,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value622,
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value623,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value623,
                                    self::JavaScriptOrdinaryObject::ArrayObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value624,
                                        _,
                                    ) => source_ml_source1_data_property_value624,
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value625,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value625,
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value626,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value626,
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value627,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value627,
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value628,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value628,
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value629,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value629,
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value630,
                                        _,
                                    ) => source_ml_source1_data_property_value630,
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value631,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value631,
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value632,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value632,
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value633,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value633,
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value634,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source1_data_property_value634,
                                }) == source_ml_source1_f547)
                                {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    self::ml_string::Text::from_units(vec![
                                        79, 98, 106, 101, 99, 116, 32, 99, 97, 108, 108, 97, 98,
                                        105, 108, 105, 116, 121, 32, 109, 117, 115, 116, 32, 97,
                                        103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 105, 116,
                                        115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32,
                                        103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116,
                                        105, 116, 121,
                                    ])
                                }
                            }
                        }
                    }
                    _ => self::ml_string::Text::from_units(vec![]),
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_heap_directory_key_error(
    heap: self::JavaScriptOrdinaryHeap,
    key: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    if (match key.clone() {
        self::JavaScriptSemanticValue::Text(_) => false,
        _ => true,
    } && match key.clone() {
        self::JavaScriptSemanticValue::SourceText(_, _) => false,
        _ => true,
    }) && match key.clone() {
        self::JavaScriptSemanticValue::Symbol(_) => false,
        _ => true,
    } {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114,
            116, 121, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 101, 120,
            116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 104, 111, 117,
            116, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 99, 111, 101, 114, 99, 105, 111,
            110,
        ])
    } else {
        self::read_java_script_heap_directory_value_error(heap.clone(), key.clone())
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_heap_directory_property(
    properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    self::find_java_script_property_descriptor_entry(properties.clone(), key.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_length_key() -> self::JavaScriptSemanticValue {
    self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![
        108f64, 101f64, 110f64, 103f64, 116f64, 104f64,
    ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_length_key(key: self::JavaScriptSemanticValue) -> bool {
    self::test_java_script_semantic_same_value(
        key.clone(),
        self::read_java_script_array_length_key(),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_index(key: self::JavaScriptSemanticValue) -> f64 {
    {
        let length = self::read_java_script_semantic_text_length(key.clone());
        if (length < 0f64) || (length > 10f64) {
            -1f64
        } else {
            self::read_java_script_literal_property_index(
                self::construct_java_script_semantic_text_units(key.clone(), 10f64),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_array_property(
    properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    self::find_java_script_heap_directory_property(properties.clone(), key.clone())
}
