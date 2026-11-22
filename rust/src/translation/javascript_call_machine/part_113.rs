// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptObjectEnumerationState;
use super::JavaScriptObjectEnumerationTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPropertyDefinitionsState;
use super::JavaScriptPropertyDefinitionsTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051;
use super::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052;
use super::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1048Result;
use super::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Result;
use super::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Result;
use super::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1053Result;
use super::find_java_script_ordinary_object;
use super::insert_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_length;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_definitions_state_error;
use super::read_java_script_semantic_text_length;
use super::source_ml_source1_temporary_read_java_script_object_enumeration_state_error_loop1048;
use super::source_ml_source1_temporary_read_java_script_object_enumeration_state_error_loop1049;
use super::source_ml_source1_temporary_read_java_script_object_enumeration_state_error_loop1050;
use super::source_ml_source1_temporary_read_java_script_object_enumeration_state_error_loop1053;
use super::test_java_script_semantic_same_value;
use super::test_java_script_standard_object_enumeration_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_property_definitions_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPropertyDefinitionsState,
    identity: self::ml::Big,
) -> self::JavaScriptPropertyDefinitionsTransition {
    match state.clone() {
        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(
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
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            {
                let error = self::read_java_script_property_definitions_state_error(
                    heap.clone(),
                    state.clone(),
                );
                if ((error != (self::ml_string::Text::from_units(vec![])))
                    || (shared_constructor_field6
                        != (self::ml_string::Text::from_units(vec![
                            97, 108, 108, 111, 99, 97, 116, 101,
                        ]))))
                {
                    self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(
                        Box::new(heap.clone()),
                        if !error.0.is_empty() {
                            error.clone()
                        } else {
                            self::ml_string::Text::from_units(vec![
                                79, 98, 106, 101, 99, 116, 46, 99, 114, 101, 97, 116, 101, 32, 97,
                                108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97,
                                105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                32, 112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99,
                                101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110,
                            ])
                        },
                    )
                } else {
                    if (({
                        let ml_reference_array1135 = match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source2_data_property_value22562,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value22562.clone(),
                        };
                        let ml_reference_projection1135 =
                            ml_reference_array1135.borrow().len() as f64;
                        ml_reference_projection1135
                    }) >= (4096f64))
                    {
                        self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 99, 114, 101, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 98, 117, 100, 103, 101, 116]))
                    } else {
                        {
                            let allocation = self::insert_java_script_ordinary_object(
                                heap.clone(),
                                identity.clone(),
                                shared_constructor_field4.clone(),
                            );
                            if (((({
                                let ml_reference_value1136 = allocation.borrow().clone();
                                ml_reference_value1136
                            })
                            .0)
                                .0)
                                != (self::ml_string::Text::from_units(vec![])))
                            {
                                self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsInvalid(Box::new((({ let ml_reference_value1137 = allocation.borrow().clone(); ml_reference_value1137 }).0).1), (({ let ml_reference_value1138 = allocation.borrow().clone(); ml_reference_value1138 }).0).0)
                            } else {
                                self::JavaScriptPropertyDefinitionsTransition::PropertyDefinitionsContinue(Box::new((({ let ml_reference_value1139 = allocation.borrow().clone(); ml_reference_value1139 }).0).1), Box::new({
                                    let source_ml_source2_nominal_field22565 = shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field22567 = shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field22569 = shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field22571 = shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field22573 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field22575 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field22577 = shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field22579 = shared_constructor_field8;
                                                                {
                                                                    let source_ml_source2_nominal_field22581 = shared_constructor_field9.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field22583 = shared_constructor_field10;
                                                                        {
                                                                            let source_ml_source2_nominal_field22585 = shared_constructor_field11.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field22587 = shared_constructor_field12.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field22589 = shared_constructor_field13.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_override22592 = self::JavaScriptSemanticValue::Object(identity.clone(), false);
                                                                                        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(source_ml_source2_nominal_field22565.clone(), Box::new(source_ml_source2_nominal_override22592.clone()), Box::new(source_ml_source2_nominal_field22569.clone()), Box::new(source_ml_source2_nominal_field22571.clone()), Box::new(source_ml_source2_nominal_field22573.clone()), self::ml_string::Text::from_units(vec![107, 101, 121, 115]), source_ml_source2_nominal_field22577.clone(), source_ml_source2_nominal_field22579, source_ml_source2_nominal_field22581.clone(), source_ml_source2_nominal_field22583, source_ml_source2_nominal_field22585.clone(), source_ml_source2_nominal_field22587.clone(), source_ml_source2_nominal_field22589.clone())
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_object_enumeration_operation(operation: &self::ml_string::Text) -> bool {
    self::test_java_script_standard_object_enumeration_operation(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_enumeration_array(
    heap: self::JavaScriptOrdinaryHeap,
    value: self::JavaScriptSemanticValue,
    prototype: self::JavaScriptSemanticValue,
    length: f64,
) -> bool {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            match value.clone() {
                self::JavaScriptSemanticValue::Object(identity, _) => {
                    let array = {
                        let source_ml_source2_array_read_value22596 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_array_read_index22597 =
                                self::find_java_script_ordinary_object(
                                    shared_constructor_field1.clone(),
                                    identity.clone(),
                                );
                            {
                                let ml_reference_array1140 =
                                    source_ml_source2_array_read_value22596.clone();
                                let ml_reference_position1140 =
                                    source_ml_source2_array_read_index22597;
                                let ml_reference_projection1140 = self::ml_array::optional_at(
                                    &*ml_reference_array1140.borrow(),
                                    self::ml_array::number_index(ml_reference_position1140),
                                );
                                ml_reference_projection1140
                            }
                        }
                    };
                    (((!(match (array.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && match array.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => true,
                        _ => false,
                    }) && ((self::read_java_script_array_length(
                        array.clone().expect("a proven present value"),
                    )) == length))
                        && self::test_java_script_semantic_same_value(
                            (((match array.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    _,
                                    source_ml_source2_data_property_value22603,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value22603 =
                                        *source_ml_source2_data_property_value22603;
                                    Some((source_ml_source2_data_property_value22603.clone(),))
                                }
                                _ => None::<(self::JavaScriptSemanticValue,)>,
                            })
                            .clone()
                            .expect("a proven present value"))
                            .0)
                                .clone(),
                            prototype.clone(),
                        ))
                }
                _ => false,
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_enumeration_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectEnumerationState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptObjectEnumerationState::ObjectEnumeration(
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
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field12 = *shared_constructor_field12;
            {
                let error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    if (((((!(self::test_java_script_object_enumeration_operation(
                        &(shared_constructor_field1),
                    )) || !(self::ml_array::includes(
                        &{
                            let ml_reference_value1141 =
                                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
                                    self::ml_string::Text::from_units(vec![114, 101, 97, 100]),
                                    self::ml_string::Text::from_units(vec![
                                        103, 101, 116, 116, 101, 114,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        112, 97, 105, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                                        111, 110,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        112, 97, 105, 114, 32, 119, 114, 105, 116, 101,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        97, 108, 108, 111, 99, 97, 116, 101,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        111, 117, 116, 112, 117, 116, 32, 119, 114, 105, 116, 101,
                                    ]),
                                ])))
                                .borrow()
                                .clone();
                            ml_reference_value1141
                        },
                        &shared_constructor_field7.clone(),
                        0f64,
                        false,
                        |a, b| a == b,
                    ))) || {
                        let source_ml_source2_s22607 = shared_constructor_field3.clone();
                        match source_ml_source2_s22607.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        }
                    }) || ((match shared_constructor_field3.clone() {
                        self::JavaScriptSemanticValue::Object(
                            _,
                            source_ml_source2_data_property_value22612,
                        ) => Some((source_ml_source2_data_property_value22612,)),
                        _ => None::<(bool,)>,
                    })
                    .clone()
                    .expect("a proven present value"))
                    .0) || {
                        let source_ml_source2_s22614 = shared_constructor_field2.clone();
                        match source_ml_source2_s22614.clone() {
                            self::JavaScriptSemanticValue::Null => true,
                            _ => false,
                        }
                    }) || {
                        let source_ml_source2_s22617 = shared_constructor_field2.clone();
                        match source_ml_source2_s22617.clone() {
                            self::JavaScriptSemanticValue::Undefined => true,
                            _ => false,
                        }
                    }) {
                        self::ml_string::Text::from_units(vec![
                            79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116,
                            105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115,
                            32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 105,
                            111, 110, 32, 105, 110, 112, 117, 116, 32, 65, 114, 114, 97, 121, 32,
                            112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 97, 110, 100, 32, 112,
                            104, 97, 115, 101,
                        ])
                    } else {
                        if ((((((((((((((({
                            let ml_reference_array1142 = shared_constructor_field4.clone();
                            let ml_reference_projection1142 =
                                ml_reference_array1142.borrow().len() as f64;
                            ml_reference_projection1142
                        }) > (4096f64))
                            || (({
                                let ml_reference_array1143 = shared_constructor_field5.clone();
                                let ml_reference_projection1143 =
                                    ml_reference_array1143.borrow().len() as f64;
                                ml_reference_projection1143
                            }) > (4096f64)))
                            || !(self::ml_math::is_safe_integer(
                                shared_constructor_field6,
                            )))
                            || (shared_constructor_field6 < (0f64)))
                            || {
                                let source_ml_source2_same_value_operand22627 =
                                    shared_constructor_field6;
                                {
                                    let source_ml_source2_same_value_operand22628 = (-0f64);
                                    (((source_ml_source2_same_value_operand22627
                                        == source_ml_source2_same_value_operand22628)
                                        && ((source_ml_source2_same_value_operand22627 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand22627)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand22628))))
                                        || ((source_ml_source2_same_value_operand22627
                                            != source_ml_source2_same_value_operand22627)
                                            && (source_ml_source2_same_value_operand22628
                                                != source_ml_source2_same_value_operand22628)))
                                }
                            })
                            || (shared_constructor_field6
                                > ({
                                    let ml_reference_array1144 = shared_constructor_field4.clone();
                                    let ml_reference_projection1144 =
                                        ml_reference_array1144.borrow().len() as f64;
                                    ml_reference_projection1144
                                })))
                            || !(self::ml_math::is_safe_integer(shared_constructor_field13)))
                            || (shared_constructor_field13 < (0f64)))
                            || {
                                let source_ml_source2_same_value_operand22637 =
                                    shared_constructor_field13;
                                {
                                    let source_ml_source2_same_value_operand22638 = (-0f64);
                                    (((source_ml_source2_same_value_operand22637
                                        == source_ml_source2_same_value_operand22638)
                                        && ((source_ml_source2_same_value_operand22637 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand22637)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand22638))))
                                        || ((source_ml_source2_same_value_operand22637
                                            != source_ml_source2_same_value_operand22637)
                                            && (source_ml_source2_same_value_operand22638
                                                != source_ml_source2_same_value_operand22638)))
                                }
                            })
                            || (shared_constructor_field13
                                > ({
                                    let ml_reference_array1145 = shared_constructor_field5.clone();
                                    let ml_reference_projection1145 =
                                        ml_reference_array1145.borrow().len() as f64;
                                    ml_reference_projection1145
                                })))
                            || !(self::ml_math::is_safe_integer(shared_constructor_field11)))
                            || (shared_constructor_field11 < (0f64)))
                            || (shared_constructor_field11 > (2f64)))
                            || {
                                let source_ml_source2_same_value_operand22649 =
                                    shared_constructor_field11;
                                {
                                    let source_ml_source2_same_value_operand22650 = (-0f64);
                                    (((source_ml_source2_same_value_operand22649
                                        == source_ml_source2_same_value_operand22650)
                                        && ((source_ml_source2_same_value_operand22649 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand22649)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand22650))))
                                        || ((source_ml_source2_same_value_operand22649
                                            != source_ml_source2_same_value_operand22649)
                                            && (source_ml_source2_same_value_operand22650
                                                != source_ml_source2_same_value_operand22650)))
                                }
                            })
                        {
                            self::ml_string::Text::from_units(vec![
                                79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97,
                                116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 98,
                                111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32,
                                107, 101, 121, 32, 76, 105, 115, 116, 32, 97, 110, 100, 32, 119,
                                114, 105, 116, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116,
                                101, 115,
                            ])
                        } else {
                            {
                                let source_ml_source1_temporary_values1044 = std::rc::Rc::new(
                                    std::cell::RefCell::new(self::ml_array::append(
                                        vec![
                                            shared_constructor_field2.clone(),
                                            shared_constructor_field3.clone(),
                                            shared_constructor_field8.clone(),
                                            shared_constructor_field9.clone(),
                                            shared_constructor_field10.clone(),
                                            shared_constructor_field12.clone(),
                                        ],
                                        {
                                            let ml_reference_value1146 =
                                                shared_constructor_field5.borrow().clone();
                                            ml_reference_value1146
                                        },
                                    )),
                                );
                                {
                                    let source_ml_source1_temporary_index1044 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s22658 = self::source_ml_source1_temporary_read_java_script_object_enumeration_state_error_loop1048(heap.clone(), source_ml_source1_temporary_values1044.clone(), source_ml_source1_temporary_index1044.clone());
                                        match source_ml_source2_s22658.clone() {
                                            self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1048Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1048Done => {
                                                {
                                                    let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                    {
                                                        let source_ml_source1_temporary_values1045 = shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source1_temporary_index1045 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s22660 = self::source_ml_source1_temporary_read_java_script_object_enumeration_state_error_loop1049(heap.clone(), seen.clone(), source_ml_source1_temporary_values1045.clone(), source_ml_source1_temporary_index1045.clone());
                                                                match source_ml_source2_s22660.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Done => {
                                                                        if (({ let ml_reference_array1147 = shared_constructor_field5.clone(); let ml_reference_projection1147 = ml_reference_array1147.borrow().len() as f64; ml_reference_projection1147 }) > shared_constructor_field6) {
                                                                            self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 76, 105, 115, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 110, 108, 121, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 112, 114, 101, 99, 101, 100, 105, 110, 103, 32, 111, 119, 110, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115])
                                                                        } else {
                                                                            {
                                                                                let source_ml_source1_temporary_values1046 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source1_temporary_index1046 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let source_ml_source2_s22666 = self::source_ml_source1_temporary_read_java_script_object_enumeration_state_error_loop1050(heap.clone(), state.clone(), source_ml_source1_temporary_values1046.clone(), source_ml_source1_temporary_index1046.clone());
                                                                                        match source_ml_source2_s22666.clone() {
                                                                                            self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Done => {
                                                                                                if ((((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]))) || (shared_constructor_field7 == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116, 32, 119, 114, 105, 116, 101])))) && (shared_constructor_field6 != ({ let ml_reference_array1148 = shared_constructor_field4.clone(); let ml_reference_projection1148 = ml_reference_array1148.borrow().len() as f64; ml_reference_projection1148 }))) || ((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]))) && (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 107, 101, 121, 115]))))) {
                                                                                                    self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 102, 111, 108, 108, 111, 119, 115, 32, 97, 108, 108, 32, 111, 119, 110, 32, 107, 101, 121, 115, 32, 97, 110, 100, 32, 107, 101, 121, 45, 111, 110, 108, 121, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 105, 110, 118, 111, 107, 101, 115, 32, 110, 111, 32, 103, 101, 116, 116, 101, 114])
                                                                                                } else {
                                                                                                    {
                                                                                                        let source_ml_source2_s22703 = if ((shared_constructor_field7 != (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) && {
                                                                                                            let source_ml_source2_s22707 = shared_constructor_field2.clone();
                                                                                                            match source_ml_source2_s22707.clone() {
                                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                    false
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    true
                                                                                                                }
                                                                                                            }
                                                                                                        }) {
                                                                                                            {
                                                                                                                let length = if ({
                                                                                                                    let source_ml_source2_s22712 = shared_constructor_field2.clone();
                                                                                                                    match source_ml_source2_s22712.clone() {
                                                                                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                                                                                            true
                                                                                                                        }
                                                                                                                        _ => {
                                                                                                                            false
                                                                                                                        }
                                                                                                                    }
                                                                                                                } || {
                                                                                                                    let source_ml_source2_s22716 = shared_constructor_field2.clone();
                                                                                                                    match source_ml_source2_s22716.clone() {
                                                                                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                                                                            true
                                                                                                                        }
                                                                                                                        _ => {
                                                                                                                            false
                                                                                                                        }
                                                                                                                    }
                                                                                                                }) {
                                                                                                                    self::read_java_script_semantic_text_length(shared_constructor_field2.clone())
                                                                                                                } else {
                                                                                                                    0f64
                                                                                                                };
                                                                                                                if ((({ let ml_reference_array1149 = shared_constructor_field4.clone(); let ml_reference_projection1149 = ml_reference_array1149.borrow().len() as f64; ml_reference_projection1149 }) != length) || (({ let ml_reference_array1150 = shared_constructor_field5.clone(); let ml_reference_projection1150 = ml_reference_array1150.borrow().len() as f64; ml_reference_projection1150 }) != shared_constructor_field6)) {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051Return(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 111, 119, 110, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 120, 97, 99, 116, 108, 121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 83, 116, 114, 105, 110, 103, 32, 105, 110, 100, 105, 99, 101, 115, 32, 111, 114, 32, 101, 109, 112, 116, 121, 32, 107, 101, 121, 32, 115, 112, 97, 99, 101]))
                                                                                                                } else {
                                                                                                                    {
                                                                                                                        let at = 0f64;
                                                                                                                        {
                                                                                                                            let source_ml_source2_s22727 = self::source_ml_source1_temporary_read_java_script_object_enumeration_state_error_loop1053(state.clone(), at);
                                                                                                                            match source_ml_source2_s22727.clone() {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1053Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1053Done => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051Next
                                                                                                                                }
                                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1053Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1053Return(source_ml_source1_temporary_result1053) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051Return(source_ml_source1_temporary_result1053.clone())
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        } else {
                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051Next
                                                                                                        };
                                                                                                        match source_ml_source2_s22703.clone() {
                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051Next => {
                                                                                                                if (((shared_constructor_field7 != (self::ml_string::Text::from_units(vec![112, 97, 105, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))) && (shared_constructor_field7 != (self::ml_string::Text::from_units(vec![112, 97, 105, 114, 32, 119, 114, 105, 116, 101])))) && {
                                                                                                                    let source_ml_source2_s22733 = shared_constructor_field8.clone();
                                                                                                                    match source_ml_source2_s22733.clone() {
                                                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                                                            false
                                                                                                                        }
                                                                                                                        _ => {
                                                                                                                            true
                                                                                                                        }
                                                                                                                    }
                                                                                                                }) {
                                                                                                                    self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 104, 111, 108, 100, 115, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 118, 97, 108, 117, 101, 32, 111, 110, 108, 121, 32, 100, 117, 114, 105, 110, 103, 32, 97, 99, 116, 117, 97, 108, 32, 101, 110, 116, 114, 121, 32, 112, 97, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110])
                                                                                                                } else {
                                                                                                                    if (((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) && (((({ let ml_reference_array1151 = shared_constructor_field4.clone(); let ml_reference_projection1151 = ml_reference_array1151.borrow().len() as f64; ml_reference_projection1151 }) != (0f64)) || (({ let ml_reference_array1152 = shared_constructor_field5.clone(); let ml_reference_projection1152 = ml_reference_array1152.borrow().len() as f64; ml_reference_projection1152 }) != (0f64))) || (shared_constructor_field6 != (0f64)))) || (self::ml_array::includes(&{ let ml_reference_value1153 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![112, 97, 105, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![112, 97, 105, 114, 32, 119, 114, 105, 116, 101])]))).borrow().clone(); ml_reference_value1153 }, &shared_constructor_field7.clone(), 0f64, false, |a, b| a == b) && (shared_constructor_field6 >= ({ let ml_reference_array1154 = shared_constructor_field4.clone(); let ml_reference_projection1154 = ml_reference_array1154.borrow().len() as f64; ml_reference_projection1154 })))) {
                                                                                                                        self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 112, 104, 97, 115, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121])
                                                                                                                    } else {
                                                                                                                        if (((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]))) && (({
                                                                                                                            let source_ml_source2_s22751 = shared_constructor_field2.clone();
                                                                                                                            match source_ml_source2_s22751.clone() {
                                                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                                    false
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                    true
                                                                                                                                }
                                                                                                                            }
                                                                                                                        } || {
                                                                                                                            let source_ml_source2_s22756 = shared_constructor_field9.clone();
                                                                                                                            match source_ml_source2_s22756.clone() {
                                                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                                    false
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                    true
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }) || !(((match shared_constructor_field9.clone() {
                                                                                                                            self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value22761) => {
                                                                                                                                Some((source_ml_source2_data_property_value22761,))
                                                                                                                            }
                                                                                                                            _ => {
                                                                                                                                None::<(bool,)>
                                                                                                                            }
                                                                                                                        }).clone().expect("a proven present value")).0))) || ((shared_constructor_field7 != (self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]))) && {
                                                                                                                            let source_ml_source2_s22765 = shared_constructor_field9.clone();
                                                                                                                            match source_ml_source2_s22765.clone() {
                                                                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                                                                    false
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                    true
                                                                                                                                }
                                                                                                                            }
                                                                                                                        })) {
                                                                                                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 103, 101, 116, 116, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 110, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                                                                                                                        } else {
                                                                                                                            if ((self::ml_array::includes(&{ let ml_reference_value1155 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![112, 97, 105, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![112, 97, 105, 114, 32, 119, 114, 105, 116, 101])]))).borrow().clone(); ml_reference_value1155 }, &shared_constructor_field7.clone(), 0f64, false, |a, b| a == b) && (shared_constructor_field1 != (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 101, 110, 116, 114, 105, 101, 115])))) || ((shared_constructor_field7 != (self::ml_string::Text::from_units(vec![112, 97, 105, 114, 32, 119, 114, 105, 116, 101]))) && ({
                                                                                                                                let source_ml_source2_s22773 = shared_constructor_field10.clone();
                                                                                                                                match source_ml_source2_s22773.clone() {
                                                                                                                                    self::JavaScriptSemanticValue::Undefined => {
                                                                                                                                        false
                                                                                                                                    }
                                                                                                                                    _ => {
                                                                                                                                        true
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            } || (shared_constructor_field11 != (0f64))))) {
                                                                                                                                self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 112, 97, 105, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 101, 108, 111, 110, 103, 115, 32, 111, 110, 108, 121, 32, 116, 111, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 97, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 112, 104, 97, 115, 101])
                                                                                                                            } else {
                                                                                                                                if ((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![112, 97, 105, 114, 32, 119, 114, 105, 116, 101]))) && !(self::test_java_script_enumeration_array(heap.clone(), shared_constructor_field10.clone(), shared_constructor_field3.clone(), 2f64))) {
                                                                                                                                    self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 112, 97, 105, 114, 32, 119, 114, 105, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 108, 101, 110, 103, 116, 104, 45, 116, 119, 111, 32, 65, 114, 114, 97, 121])
                                                                                                                                } else {
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_s22795 = if (shared_constructor_field7 == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116, 32, 119, 114, 105, 116, 101]))) {
                                                                                                                                            if (!(self::test_java_script_enumeration_array(heap.clone(), shared_constructor_field12.clone(), shared_constructor_field3.clone(), { let ml_reference_array1156 = shared_constructor_field5.clone(); let ml_reference_projection1156 = ml_reference_array1156.borrow().len() as f64; ml_reference_projection1156 })) || (shared_constructor_field6 != ({ let ml_reference_array1157 = shared_constructor_field4.clone(); let ml_reference_projection1157 = ml_reference_array1157.borrow().len() as f64; ml_reference_projection1157 }))) {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 111, 117, 116, 112, 117, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 76, 105, 115, 116, 32, 97, 110, 100, 32, 114, 101, 115, 117, 108, 116, 32, 65, 114, 114, 97, 121]))
                                                                                                                                            } else {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052Next
                                                                                                                                            }
                                                                                                                                        } else {
                                                                                                                                            if ({
                                                                                                                                                let source_ml_source2_s22806 = shared_constructor_field12.clone();
                                                                                                                                                match source_ml_source2_s22806.clone() {
                                                                                                                                                    self::JavaScriptSemanticValue::Undefined => {
                                                                                                                                                        false
                                                                                                                                                    }
                                                                                                                                                    _ => {
                                                                                                                                                        true
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            } || (shared_constructor_field13 != (0f64))) {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 101, 115, 32, 105, 116, 115, 32, 102, 105, 110, 97, 108, 32, 65, 114, 114, 97, 121, 32, 111, 110, 108, 121, 32, 97, 102, 116, 101, 114, 32, 99, 111, 108, 108, 101, 99, 116, 105, 110, 103, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 76, 105, 115, 116]))
                                                                                                                                            } else {
                                                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052Next
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        match source_ml_source2_s22795.clone() {
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052Next => {
                                                                                                                                                self::ml_string::Text::from_units(vec![])
                                                                                                                                            }
                                                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052Return(source_ml_source1_temporary_result1052) => {
                                                                                                                                                source_ml_source1_temporary_result1052.clone()
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051Return(source_ml_source1_temporary_result1052_2) => {
                                                                                                                source_ml_source1_temporary_result1052_2.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Return(source_ml_source1_temporary_result1053_2) => {
                                                                                                source_ml_source1_temporary_result1053_2.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Return(source_ml_source1_temporary_result1053_3) => {
                                                                        source_ml_source1_temporary_result1053_3.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1048Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1048Return(source_ml_source1_temporary_result1054) => {
                                                source_ml_source1_temporary_result1054.clone()
                                            }
                                        }
                                    }
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
pub fn begin_java_script_object_enumeration(
    heap: self::JavaScriptOrdinaryHeap,
    operation: &self::ml_string::Text,
    input: self::JavaScriptSemanticValue,
    prototype: self::JavaScriptSemanticValue,
) -> self::JavaScriptObjectEnumerationTransition {
    {
        let invalid = {
            let source_ml_source2_logical22811 = {
                let source_ml_source2_logical22810 =
                    self::read_java_script_ordinary_heap_error(heap.clone());
                if !source_ml_source2_logical22810.0.is_empty() {
                    source_ml_source2_logical22810.clone()
                } else {
                    self::read_java_script_ordinary_value_error(heap.clone(), input.clone())
                }
            };
            if !source_ml_source2_logical22811.0.is_empty() {
                source_ml_source2_logical22811.clone()
            } else {
                self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone())
            }
        };
        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(
                Box::new(heap.clone()),
                invalid.clone(),
            )
        } else {
            if (match input.clone() {
                self::JavaScriptSemanticValue::Null => true,
                _ => false,
            } || match input.clone() {
                self::JavaScriptSemanticValue::Undefined => true,
                _ => false,
            }) {
                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationError(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 117,
                        110, 100, 101, 102, 105, 110, 101, 100, 32, 111, 114, 32, 110, 117, 108,
                        108, 32, 116, 111, 32, 111, 98, 106, 101, 99, 116,
                    ]),
                )
            } else {
                {
                    let absent = self::JavaScriptSemanticValue::Undefined;
                    {
                        let state = self::JavaScriptObjectEnumerationState::ObjectEnumeration(
                            operation.clone(),
                            Box::new(input.clone()),
                            Box::new(prototype.clone()),
                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                self::JavaScriptSemanticValue,
                            >::new(
                            ))),
                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                self::JavaScriptSemanticValue,
                            >::new(
                            ))),
                            0f64,
                            self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
                            Box::new(absent.clone()),
                            Box::new(absent.clone()),
                            Box::new(absent.clone()),
                            0f64,
                            Box::new(absent.clone()),
                            0f64,
                        );
                        {
                            let error = self::read_java_script_object_enumeration_state_error(
                                heap.clone(),
                                state.clone(),
                            );
                            if (error == (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationContinue(Box::new(heap.clone()), Box::new(state.clone()))
                            } else {
                                self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(Box::new(heap.clone()), error.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
