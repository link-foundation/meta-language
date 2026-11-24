// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentTransition;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPropertyDefinitionCall;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticValue;
use super::begin_java_script_array_assignment;
use super::begin_java_script_descriptor_conversion;
use super::begin_java_script_property_key_conversion;
use super::create_java_script_property_definition_call;
use super::define_java_script_ordinary_property;
use super::find_java_script_ordinary_object;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::retain_java_script_property_definition_context;
use super::test_java_script_array_length_key;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_property_array_definition(
    target: self::JavaScriptSemanticValue,
    key: self::JavaScriptSemanticValue,
    transition: self::JavaScriptArrayAssignmentTransition,
    operation: Option<self::ml_string::Text>,
) -> self::JavaScriptPropertyDefinitionTransition {
    match transition.clone() {
        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentContinue(heap, state) => {
            let heap = *heap;
            let state = *state;
            {
                let source_ml_source2_constructor_field21592 = Some(heap.clone());
                {
                    let source_ml_source2_constructor_field21596 =
                        self::retain_java_script_property_definition_context(
                            self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                                Box::new(target.clone()),
                                Box::new(key.clone()),
                                Box::new(state.clone()),
                                None::<(Option<self::ml_string::Text>,)>,
                                None::<(Option<self::JavaScriptPropertyDefinitionCall>,)>,
                            ),
                            operation.clone(),
                            None::<self::JavaScriptPropertyDefinitionCall>,
                        );
                    self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(
                        Box::new(source_ml_source2_constructor_field21596.clone()),
                        Some((source_ml_source2_constructor_field21592.clone(),)),
                    )
                }
            }
        }
        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCall(
            heap_2,
            state_2,
            callee,
            receiver,
            arguments,
        ) => {
            let heap_2 = *heap_2;
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            {
                let source_ml_source2_constructor_field21597 = Some(heap_2.clone());
                {
                    let source_ml_source2_constructor_field21601 =
                        self::retain_java_script_property_definition_context(
                            self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                                Box::new(target.clone()),
                                Box::new(key.clone()),
                                Box::new(state_2.clone()),
                                None::<(Option<self::ml_string::Text>,)>,
                                None::<(Option<self::JavaScriptPropertyDefinitionCall>,)>,
                            ),
                            operation.clone(),
                            Some(self::create_java_script_property_definition_call(
                                callee.clone(),
                                receiver.clone(),
                                arguments.clone(),
                            )),
                        );
                    self::JavaScriptPropertyDefinitionTransition::DefinitionCall(
                        Box::new(source_ml_source2_constructor_field21601.clone()),
                        Box::new(callee.clone()),
                        Box::new(receiver.clone()),
                        arguments.clone(),
                        Some((source_ml_source2_constructor_field21597.clone(),)),
                    )
                }
            }
        }
        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCompleted(
            heap_3,
            success,
            writes,
        ) => {
            let heap_3 = *heap_3;
            if (match (
                operation.clone(),
                self::ml_string::Text::from_units(vec![
                    82, 101, 102, 108, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114,
                    111, 112, 101, 114, 116, 121,
                ]),
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            } || success)
            {
                self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(
                    Box::new(heap_3.clone()),
                    Box::new(
                        if match (
                            operation.clone(),
                            self::ml_string::Text::from_units(vec![
                                82, 101, 102, 108, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101,
                                80, 114, 111, 112, 101, 114, 116, 121,
                            ]),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        } {
                            self::JavaScriptSemanticValue::Boolean(success)
                        } else {
                            target.clone()
                        },
                    ),
                    (((writes.clone().unwrap_or_else(|| {
                        (None::<std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>>,)
                    }))
                    .0)
                        .clone())
                    .clone()
                    .unwrap_or_else(|| {
                        std::rc::Rc::new(std::cell::RefCell::new(
                            Vec::<self::ml_string::Text>::new(),
                        ))
                    }),
                )
            } else {
                {
                    let source_ml_source2_constructor_field21609 = Some(heap_3.clone());
                    {
                        let source_ml_source2_constructor_field21610 =
                            Some(
                                (((writes.clone().unwrap_or_else(|| {
                                    (None::<
                                        std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
                                    >,)
                                }))
                                .0)
                                    .clone())
                                .clone()
                                .unwrap_or_else(|| {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::ml_string::Text,
                                    >::new(
                                    )))
                                }),
                            );
                        self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(
                            self::ml_string::Text::from_units(vec![
                                84, 121, 112, 101, 69, 114, 114, 111, 114,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 68, 101, 102, 105, 110, 101, 79, 119,
                                110, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 106, 101,
                                99, 116, 101, 100, 32, 116, 104, 101, 32, 114, 101, 113, 117, 101,
                                115, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111,
                                114,
                            ]),
                            Some((source_ml_source2_constructor_field21609.clone(),)),
                            Some((source_ml_source2_constructor_field21610.clone(),)),
                        )
                    }
                }
            }
        }
        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentThrow(heap_4, value) => {
            let heap_4 = *heap_4;
            let value = *value;
            {
                let source_ml_source2_constructor_field21613 = Some(heap_4.clone());
                self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(
                    Box::new(value.clone()),
                    Some((source_ml_source2_constructor_field21613.clone(),)),
                )
            }
        }
        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentError(
            heap_5,
            error_class,
            reason,
        ) => {
            let heap_5 = *heap_5;
            {
                let source_ml_source2_constructor_field21615 = Some(heap_5.clone());
                self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(
                    error_class.clone(),
                    reason.clone(),
                    Some((source_ml_source2_constructor_field21615.clone(),)),
                    None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>>,)>,
                )
            }
        }
        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(heap_6, reason_2) => {
            let heap_6 = *heap_6;
            {
                let source_ml_source2_constructor_field21618 = Some(heap_6.clone());
                self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                    reason_2.clone(),
                    Some((source_ml_source2_constructor_field21618.clone(),)),
                )
            }
        }
        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentResourceLimit(
            heap_7,
            state_3,
            reason_3,
        ) => {
            let heap_7 = *heap_7;
            let state_3 = *state_3;
            self::JavaScriptPropertyDefinitionTransition::DefinitionResourceLimit(
                Box::new(heap_7.clone()),
                Box::new(self::retain_java_script_property_definition_context(
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                        Box::new(target.clone()),
                        Box::new(key.clone()),
                        Box::new(state_3.clone()),
                        None::<(Option<self::ml_string::Text>,)>,
                        None::<(Option<self::JavaScriptPropertyDefinitionCall>,)>,
                    ),
                    operation.clone(),
                    None::<self::JavaScriptPropertyDefinitionCall>,
                )),
                reason_3.clone(),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn complete_java_script_property_definition_with_symbol(
    heap: self::JavaScriptOrdinaryHeap,
    target: self::JavaScriptSemanticValue,
    key: self::JavaScriptSemanticValue,
    descriptor: std::rc::Rc<
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
    exotic_key: Option<self::JavaScriptSemanticValue>,
    operation: Option<self::ml_string::Text>,
) -> self::JavaScriptPropertyDefinitionTransition {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            if ({
                let source_ml_source2_comparison_operand21624 = None::<()>;
                !(match (
                    operation.clone(),
                    source_ml_source2_comparison_operand21624.clone(),
                ) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                })
            } && !(self::ml_array::includes(
                &{
                    let ml_reference_value1089 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                        self::ml_string::Text::from_units(vec![
                            79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114,
                            111, 112, 101, 114, 116, 121,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            82, 101, 102, 108, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80,
                            114, 111, 112, 101, 114, 116, 121,
                        ]),
                    ])))
                    .borrow()
                    .clone();
                    ml_reference_value1089
                },
                &operation.clone().expect("a proven present value"),
                0f64,
                false,
                |a, b| a == b,
            ))) {
                {
                    let source_ml_source2_constructor_field21625 = Some(heap.clone());
                    self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                        self::ml_string::Text::from_units(vec![
                            97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 112, 114, 111,
                            112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105,
                            111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                            97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 114,
                            32, 82, 101, 102, 108, 101, 99, 116, 32, 114, 101, 115, 117, 108, 116,
                            32, 112, 111, 108, 105, 99, 121,
                        ]),
                        Some((source_ml_source2_constructor_field21625.clone(),)),
                    )
                }
            } else {
                {
                    let error = self::read_java_script_ordinary_heap_error(heap.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                            error.clone(),
                            None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
                        )
                    } else {
                        {
                            let value_error = self::read_java_script_ordinary_value_error(
                                heap.clone(),
                                target.clone(),
                            );
                            if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                                    value_error.clone(),
                                    None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
                                )
                            } else {
                                match target.clone() {
                                    self::JavaScriptSemanticValue::Object(identity, _) => {
                                        {
                                            let object = {
                                                let source_ml_source2_array_read_value21632 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_array_read_index21633 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                                    { let ml_reference_array1090 = source_ml_source2_array_read_value21632.clone(); let ml_reference_position1090 = source_ml_source2_array_read_index21633; let ml_reference_projection1090 = self::ml_array::optional_at(&*ml_reference_array1090.borrow(), self::ml_array::number_index(ml_reference_position1090)); ml_reference_projection1090 }
                                                }
                                            };
                                            if (({
                                                let source_ml_source2_comparison_operand21635 = {
                                                    let source_ml_source2_optional_chain_receiver21634 = (object.clone(),);
                                                    if match (source_ml_source2_optional_chain_receiver21634.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        Some({
                                                            let source_ml_source1_temporary_optional_chain1016 = ((source_ml_source2_optional_chain_receiver21634.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_source1_temporary_optional_chain1016.clone() {
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
                                                            }
                                                        })
                                                    } else {
                                                        None::<self::ml_string::Text>
                                                    }
                                                };
                                                match (source_ml_source2_comparison_operand21635.clone(), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                            } && self::test_java_script_array_length_key(key.clone())) && !(match (exotic_key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                self::continue_java_script_property_array_definition(target.clone(), key.clone(), self::begin_java_script_array_assignment(heap.clone(), identity.clone(), key.clone(), descriptor.clone(), exotic_key.clone().expect("a proven present value")), operation.clone())
                                            } else {
                                                {
                                                    let transition = self::define_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone(), descriptor.clone());
                                                    if !((({ let ml_reference_value1091 = transition.borrow().clone(); ml_reference_value1091 }).0).3) {
                                                        if {
                                                            let source_ml_source2_comparison_operand21637 = ((((({ let ml_reference_value1092 = transition.borrow().clone(); ml_reference_value1092 }).0).2).clone().unwrap_or_else(|| (None::<bool>,))).0).clone();
                                                            match (source_ml_source2_comparison_operand21637.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                        } {
                                                            self::JavaScriptPropertyDefinitionTransition::DefinitionResourceLimit(Box::new((({ let ml_reference_value1093 = transition.borrow().clone(); ml_reference_value1093 }).0).1), Box::new(self::retain_java_script_property_definition_context(if match (exotic_key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(Box::new(target.clone()), Box::new(key.clone()), descriptor.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<self::JavaScriptPropertyDefinitionCall>,)>)
                                                            } else {
                                                                {
                                                                    let source_ml_source2_constructor_field21645 = Some(exotic_key.clone().expect("a proven present value"));
                                                                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(Box::new(target.clone()), Box::new(key.clone()), descriptor.clone(), Some((source_ml_source2_constructor_field21645.clone(),)), None::<(Option<self::ml_string::Text>,)>, None::<(Option<self::JavaScriptPropertyDefinitionCall>,)>)
                                                                }
                                                            }, operation.clone(), None::<self::JavaScriptPropertyDefinitionCall>)), (({ let ml_reference_value1094 = transition.borrow().clone(); ml_reference_value1094 }).0).0)
                                                        } else {
                                                            if (((({ let ml_reference_value1095 = transition.borrow().clone(); ml_reference_value1095 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                                {
                                                                    let source_ml_source2_constructor_field21646 = (({ let ml_reference_value1096 = transition.borrow().clone(); ml_reference_value1096 }).0).0;
                                                                    self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(source_ml_source2_constructor_field21646.clone(), None::<(Option<self::JavaScriptOrdinaryHeap>,)>)
                                                                }
                                                            } else {
                                                                if match (operation.clone(), self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } {
                                                                    self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(Box::new((({ let ml_reference_value1097 = transition.borrow().clone(); ml_reference_value1097 }).0).1), Box::new(self::JavaScriptSemanticValue::Boolean(false)), (({ let ml_reference_value1098 = transition.borrow().clone(); ml_reference_value1098 }).0).4)
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_constructor_field21649 = Some((({ let ml_reference_value1099 = transition.borrow().clone(); ml_reference_value1099 }).0).1);
                                                                        self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 68, 101, 102, 105, 110, 101, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 106, 101, 99, 116, 101, 100, 32, 116, 104, 101, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114]), Some((source_ml_source2_constructor_field21649.clone(),)), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>>,)>)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(Box::new((({ let ml_reference_value1100 = transition.borrow().clone(); ml_reference_value1100 }).0).1), Box::new(if match (operation.clone(), self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false } {
                                                            self::JavaScriptSemanticValue::Boolean(true)
                                                        } else {
                                                            target.clone()
                                                        }), (({ let ml_reference_value1101 = transition.borrow().clone(); ml_reference_value1101 }).0).4)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]), None::<(Option<self::JavaScriptOrdinaryHeap>,)>)
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
pub fn continue_java_script_property_descriptor_definition_with_symbol(
    heap: self::JavaScriptOrdinaryHeap,
    target: self::JavaScriptSemanticValue,
    key: self::JavaScriptSemanticValue,
    transition: self::JavaScriptDescriptorConversionTransition,
    exotic_key: Option<self::JavaScriptSemanticValue>,
    operation: Option<self::ml_string::Text>,
) -> self::JavaScriptPropertyDefinitionTransition {
    match transition.clone() {
        self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(state) => {
            let state = *state;
            {
                let source_ml_source2_constructor_field21662 =
                    self::retain_java_script_property_definition_context(
                        if match (exotic_key.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                                Box::new(target.clone()),
                                Box::new(key.clone()),
                                Box::new(state.clone()),
                                None::<(Option<self::JavaScriptSemanticValue>,)>,
                                None::<(Option<self::ml_string::Text>,)>,
                                None::<(Option<self::JavaScriptPropertyDefinitionCall>,)>,
                            )
                        } else {
                            {
                                let source_ml_source2_constructor_field21661 =
                                    Some(exotic_key.clone().expect("a proven present value"));
                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(Box::new(target.clone()), Box::new(key.clone()), Box::new(state.clone()), Some((source_ml_source2_constructor_field21661.clone(),)), None::<(Option<self::ml_string::Text>,)>, None::<(Option<self::JavaScriptPropertyDefinitionCall>,)>)
                            }
                        },
                        operation.clone(),
                        None::<self::JavaScriptPropertyDefinitionCall>,
                    );
                self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(
                    Box::new(source_ml_source2_constructor_field21662.clone()),
                    None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
                )
            }
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorCall(
            state_2,
            callee,
            receiver,
            arguments,
        ) => {
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            {
                let source_ml_source2_constructor_field21670 =
                    self::retain_java_script_property_definition_context(
                        if match (exotic_key.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                                Box::new(target.clone()),
                                Box::new(key.clone()),
                                Box::new(state_2.clone()),
                                None::<(Option<self::JavaScriptSemanticValue>,)>,
                                None::<(Option<self::ml_string::Text>,)>,
                                None::<(Option<self::JavaScriptPropertyDefinitionCall>,)>,
                            )
                        } else {
                            {
                                let source_ml_source2_constructor_field21669 =
                                    Some(exotic_key.clone().expect("a proven present value"));
                                self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(Box::new(target.clone()), Box::new(key.clone()), Box::new(state_2.clone()), Some((source_ml_source2_constructor_field21669.clone(),)), None::<(Option<self::ml_string::Text>,)>, None::<(Option<self::JavaScriptPropertyDefinitionCall>,)>)
                            }
                        },
                        operation.clone(),
                        Some(self::create_java_script_property_definition_call(
                            callee.clone(),
                            receiver.clone(),
                            arguments.clone(),
                        )),
                    );
                self::JavaScriptPropertyDefinitionTransition::DefinitionCall(
                    Box::new(source_ml_source2_constructor_field21670.clone()),
                    Box::new(callee.clone()),
                    Box::new(receiver.clone()),
                    arguments.clone(),
                    None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
                )
            }
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(descriptor) => {
            self::complete_java_script_property_definition_with_symbol(
                heap.clone(),
                target.clone(),
                key.clone(),
                descriptor.clone(),
                exotic_key.clone(),
                operation.clone(),
            )
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorThrow(value) => {
            let value = *value;
            self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(
                Box::new(value.clone()),
                None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
            )
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
            error_class,
            reason,
        ) => self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(
            error_class.clone(),
            reason.clone(),
            None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
            None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>>,)>,
        ),
        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(reason_2) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                reason_2.clone(),
                None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_property_key_definition_with_symbol(
    heap: self::JavaScriptOrdinaryHeap,
    target: self::JavaScriptSemanticValue,
    attributes: self::JavaScriptSemanticValue,
    transition: self::JavaScriptPropertyKeyConversionTransition,
    exotic_key: Option<self::JavaScriptSemanticValue>,
    operation: Option<self::ml_string::Text>,
) -> self::JavaScriptPropertyDefinitionTransition {
    match transition.clone() {
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(state) => {
            let state = *state;
            {
                let source_ml_source2_constructor_field21681 =
                    self::retain_java_script_property_definition_context(
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                            Box::new(target.clone()),
                            Box::new(attributes.clone()),
                            Box::new(state.clone()),
                            None::<(Option<self::ml_string::Text>,)>,
                            None::<(Option<self::JavaScriptPropertyDefinitionCall>,)>,
                        ),
                        operation.clone(),
                        None::<self::JavaScriptPropertyDefinitionCall>,
                    );
                self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(
                    Box::new(source_ml_source2_constructor_field21681.clone()),
                    None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
                )
            }
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(
            state_2,
            callee,
            receiver,
            arguments,
        ) => {
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            {
                let source_ml_source2_constructor_field21685 =
                    self::retain_java_script_property_definition_context(
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                            Box::new(target.clone()),
                            Box::new(attributes.clone()),
                            Box::new(state_2.clone()),
                            None::<(Option<self::ml_string::Text>,)>,
                            None::<(Option<self::JavaScriptPropertyDefinitionCall>,)>,
                        ),
                        operation.clone(),
                        Some(self::create_java_script_property_definition_call(
                            callee.clone(),
                            receiver.clone(),
                            arguments.clone(),
                        )),
                    );
                self::JavaScriptPropertyDefinitionTransition::DefinitionCall(
                    Box::new(source_ml_source2_constructor_field21685.clone()),
                    Box::new(callee.clone()),
                    Box::new(receiver.clone()),
                    arguments.clone(),
                    None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
                )
            }
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(key) => {
            let key = *key;
            self::continue_java_script_property_descriptor_definition_with_symbol(
                heap.clone(),
                target.clone(),
                key.clone(),
                self::begin_java_script_descriptor_conversion(heap.clone(), attributes.clone()),
                exotic_key.clone(),
                operation.clone(),
            )
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(value) => {
            let value = *value;
            self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(
                Box::new(value.clone()),
                None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
            )
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(
            error_class,
            reason,
        ) => self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(
            error_class.clone(),
            reason.clone(),
            None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
            None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>>,)>,
        ),
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                reason_2.clone(),
                None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_property_definition(
    heap: self::JavaScriptOrdinaryHeap,
    target: self::JavaScriptSemanticValue,
    key_input: self::JavaScriptSemanticValue,
    attributes: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
    operation: Option<self::ml_string::Text>,
) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                error.clone(),
                None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
            )
        } else {
            {
                let value_error =
                    self::read_java_script_ordinary_value_error(heap.clone(), target.clone());
                if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                        value_error.clone(),
                        None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
                    )
                } else {
                    if ({
                        let source_ml_source2_comparison_operand21696 = None::<()>;
                        !(match (
                            operation.clone(),
                            source_ml_source2_comparison_operand21696.clone(),
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        })
                    } && !(self::ml_array::includes(
                        &{
                            let ml_reference_value1102 =
                                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    self::ml_string::Text::from_units(vec![
                                        79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110,
                                        101, 80, 114, 111, 112, 101, 114, 116, 121,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        82, 101, 102, 108, 101, 99, 116, 46, 100, 101, 102, 105,
                                        110, 101, 80, 114, 111, 112, 101, 114, 116, 121,
                                    ]),
                                ])))
                                .borrow()
                                .clone();
                            ml_reference_value1102
                        },
                        &operation.clone().expect("a proven present value"),
                        0f64,
                        false,
                        |a, b| a == b,
                    ))) {
                        self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(
                            self::ml_string::Text::from_units(vec![
                                97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102,
                                105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105,
                                114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 79, 98, 106, 101,
                                99, 116, 32, 111, 114, 32, 82, 101, 102, 108, 101, 99, 116, 32,
                                111, 112, 101, 114, 97, 116, 105, 111, 110,
                            ]),
                            None::<(Option<self::JavaScriptOrdinaryHeap>,)>,
                        )
                    } else {
                        match target.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                self::continue_java_script_property_key_definition_with_symbol(
                                    heap.clone(),
                                    target.clone(),
                                    attributes.clone(),
                                    self::begin_java_script_property_key_conversion(
                                        heap.clone(),
                                        key_input.clone(),
                                        exotic_key.clone(),
                                    ),
                                    Some(exotic_key.clone()),
                                    operation.clone(),
                                )
                            }
                            _ => {
                                let source_ml_source2_constructor_field21701 = (operation
                                    .clone()
                                    .unwrap_or_else(|| {
                                        self::ml_string::Text::from_units(vec![
                                            79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110,
                                            101, 80, 114, 111, 112, 101, 114, 116, 121,
                                        ])
                                    }))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32,
                                    79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116,
                                ]));
                                self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), source_ml_source2_constructor_field21701.clone(), None::<(Option<self::JavaScriptOrdinaryHeap>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>>,)>)
                            }
                        }
                    }
                }
            }
        }
    }
}
