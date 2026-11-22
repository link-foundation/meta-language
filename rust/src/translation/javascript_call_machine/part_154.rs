// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallArgumentListsUpdate;
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1301;
use super::MlSource1TemporaryReplaceUtf16TextLoop1309Result;
use super::dependency_function2;
use super::execute_java_script_call_number_conversion;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_value_error;
use super::read_utf16_text_integer;
use super::replace_java_script_call_frame;
use super::source_ml_source1_temporary_accept_java_script_call_argument_lists_update_loop1312;
use super::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop1311;
use super::source_ml_source1_temporary_plan_utf16_text_substitution_loop1295;
use super::source_ml_source1_temporary_replace_utf16_text_loop1303;
use super::source_ml_source1_temporary_replace_utf16_text_loop1309;
use super::test_java_script_standard_primitive_method_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_utf16_text_substitution(
    replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let start = 0f64;
            {
                let at = 0f64;
                {
                    let start_2 =
                        self::source_ml_source1_temporary_plan_utf16_text_substitution_loop1295(
                            replacement.clone(),
                            result.clone(),
                            start,
                            at,
                        );
                    {
                        let source_ml_source2_s29127 = if (start_2
                            < ({
                                let ml_reference_array1624 = replacement.clone();
                                let ml_reference_projection1624 =
                                    ml_reference_array1624.borrow().len() as f64;
                                ml_reference_projection1624
                            })) {
                            {
                                let source_ml_source1_temporary_plan_utf16_text_substitution_ignored1302 = {
                                    let ml_method_owner1625 = result.clone();
                                    let ml_method_argument1625_0 = 0f64;
                                    let ml_method_argument1625_1 = start_2;
                                    let ml_method_argument1625_2 = {
                                        let ml_reference_array1626 = replacement.clone();
                                        let ml_reference_projection1626 =
                                            ml_reference_array1626.borrow().len() as f64;
                                        ml_reference_projection1626
                                    };
                                    let ml_inserted = vec![
                                        ml_method_argument1625_0,
                                        ml_method_argument1625_1,
                                        ml_method_argument1625_2,
                                    ];
                                    let mut ml_method_values1625 = ml_method_owner1625.borrow_mut();
                                    let ml_length = ml_method_values1625
                                        .len()
                                        .checked_add(ml_inserted.len())
                                        .filter(|length| *length <= 4294967295usize)
                                        .expect("Invalid array length");
                                    ml_method_values1625.extend(ml_inserted);
                                    ml_length as f64
                                };
                                self::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1301::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1301Next
                            }
                        } else {
                            self::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1301::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1301Next
                        };
                        match source_ml_source2_s29127.clone() {
                            self::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1301::MlSource1TemporaryPlanUtf16TextSubstitutionJoin1301Next => {
                                result.clone()
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
pub fn replace_utf16_text(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    all: bool,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let plan = self::plan_utf16_text_substitution(replacement.clone());
            {
                let start = 0f64;
                {
                    let from = 0f64;
                    {
                        let start_2 = self::source_ml_source1_temporary_replace_utf16_text_loop1303(
                            text.clone(),
                            search.clone(),
                            replacement.clone(),
                            all,
                            result.clone(),
                            plan.clone(),
                            start,
                            from,
                            true,
                        );
                        {
                            let source_ml_source2_s29128 =
                                self::source_ml_source1_temporary_replace_utf16_text_loop1309(
                                    text.clone(),
                                    result.clone(),
                                    start_2,
                                );
                            match source_ml_source2_s29128.clone() {
                                self::MlSource1TemporaryReplaceUtf16TextLoop1309Result::MlSource1TemporaryReplaceUtf16TextLoop1309Done => {
                                    result.clone()
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
pub fn test_java_script_primitive_method_operation(operation: &self::ml_string::Text) -> bool {
    self::test_java_script_standard_primitive_method_operation(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_primitive_method(
    receiver: self::JavaScriptSemanticValue,
    operation: &self::ml_string::Text,
    args: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    if !(self::test_java_script_primitive_method_operation(operation)) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(
            vec![
                112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 109, 101, 116, 104, 111, 100, 115,
                32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99,
                116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111,
                112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114,
            ],
        ))
    } else {
        if (({
            let ml_reference_array1627 = args.clone();
            let ml_reference_projection1627 = ml_reference_array1627.borrow().len() as f64;
            ml_reference_projection1627
        }) > (4096f64))
        {
            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(
                self::ml_string::Text::from_units(vec![
                    112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 109, 101, 116, 104, 111, 100,
                    32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 101, 120, 99, 101, 101,
                    100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116,
                    32, 99, 97, 108, 108, 32, 98, 117, 100, 103, 101, 116,
                ]),
            )
        } else {
            if (match receiver.clone() {
                self::JavaScriptSemanticValue::Null => true,
                _ => false,
            } || match receiver.clone() {
                self::JavaScriptSemanticValue::Undefined => true,
                _ => false,
            }) {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        83, 116, 114, 105, 110, 103, 46, 112, 114, 111, 116, 111, 116, 121, 112,
                        101, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104, 32, 99, 97, 108,
                        108, 101, 100, 32, 111, 110, 32, 110, 117, 108, 108, 32, 111, 114, 32, 117,
                        110, 100, 101, 102, 105, 110, 101, 100,
                    ]),
                )
            } else {
                match receiver.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                            self::ml_string::Text::from_units(vec![
                                116, 104, 105, 115, 32, 83, 116, 114, 105, 110, 103, 32, 109, 101,
                                116, 104, 111, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                                105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99,
                                101, 105, 118, 101, 114, 32, 84, 111, 80, 114, 105, 109, 105, 116,
                                105, 118, 101, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105,
                                111, 110,
                            ]),
                        )
                    }
                    _ => {
                        let receiver_error =
                            self::read_java_script_semantic_value_error(receiver.clone());
                        if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                receiver_error.clone(),
                            )
                        } else {
                            {
                                let text = self::dependency_function2(receiver.clone());
                                match text.clone() {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                                        let value = *value;
                                        self::execute_java_script_primitive_prefix_search(
                                            value.clone(),
                                            args.clone(),
                                        )
                                    }
                                    _ => text.clone(),
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
pub fn execute_java_script_primitive_prefix_search(
    receiver_text: self::JavaScriptSemanticValue,
    args: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let search = ({
            let source_ml_source2_array_read_index29132 = 0f64;
            {
                let ml_reference_array1628 = args.clone();
                let ml_reference_position1628 = source_ml_source2_array_read_index29132;
                let ml_reference_projection1628 = self::ml_array::optional_at(
                    &*ml_reference_array1628.borrow(),
                    self::ml_array::number_index(ml_reference_position1628),
                );
                ml_reference_projection1628
            }
        })
        .clone()
        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        match search.clone() {
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                    self::ml_string::Text::from_units(vec![
                        83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 114, 101,
                        113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                        108, 32, 73, 115, 82, 101, 103, 69, 120, 112, 32, 97, 110, 100, 32, 84,
                        111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116,
                        111, 99, 111, 108, 115,
                    ]),
                )
            }
            _ => {
                let search_error = self::read_java_script_semantic_value_error(search.clone());
                if (search_error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error.clone())
                } else {
                    {
                        let converted = self::dependency_function2(search.clone());
                        match converted.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                                let value = *value;
                                self::execute_java_script_primitive_prefix_position(
                                    receiver_text.clone(),
                                    value.clone(),
                                    ({
                                        let source_ml_source2_array_read_index29136 = 1f64;
                                        {
                                            let ml_reference_array1629 = args.clone();
                                            let ml_reference_position1629 =
                                                source_ml_source2_array_read_index29136;
                                            let ml_reference_projection1629 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array1629.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position1629,
                                                    ),
                                                );
                                            ml_reference_projection1629
                                        }
                                    })
                                    .clone()
                                    .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                                )
                            }
                            _ => converted.clone(),
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_primitive_prefix_position(
    receiver_text: self::JavaScriptSemanticValue,
    search_text: self::JavaScriptSemanticValue,
    position: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallPrimitiveResult {
    match position.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                self::ml_string::Text::from_units(vec![
                    83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115,
                    105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105,
                    116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 80, 114, 105, 109, 105,
                    116, 105, 118, 101, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111,
                    110,
                ]),
            )
        }
        _ => {
            let position_error = self::read_java_script_semantic_value_error(position.clone());
            if (position_error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error.clone())
            } else {
                {
                    let number = self::execute_java_script_call_number_conversion(
                        &(self::ml_string::Text::from_units(vec![
                            99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118,
                            101, 32, 110, 117, 109, 98, 101, 114,
                        ])),
                        position.clone(),
                    );
                    match number.clone() {
                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value) => {
                            let position_value = *position_value;
                            match position_value.clone() {
                                self::JavaScriptSemanticValue::Number(value) => {
                                    let length = self::read_java_script_semantic_text_length(
                                        receiver_text.clone(),
                                    );
                                    {
                                        let search_length =
                                            self::read_java_script_semantic_text_length(
                                                search_text.clone(),
                                            );
                                        {
                                            let start = self::ml_math::min(
                                                length,
                                                self::ml_math::max(
                                                    0f64,
                                                    self::read_utf16_text_integer(value),
                                                ),
                                            );
                                            if (search_length > (1048576f64)) {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                            } else {
                                                {
                                                    let matches =
                                                        ((start + search_length) <= length);
                                                    {
                                                        let at = 0f64;
                                                        {
                                                            let matches_2 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop1311(receiver_text.clone(), search_text.clone(), search_length, start, matches, at);
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_2)))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                    self::ml_string::Text::from_units(vec![
                                        83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99,
                                        104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114,
                                        101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99,
                                        116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99,
                                        111, 110, 118, 101, 114, 115, 105, 111, 110,
                                    ]),
                                ),
                            }
                        }
                        _ => number.clone(),
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn retain_java_script_call_resource_limit_machine(
    machine: self::JavaScriptCallMachine,
    transition: self::JavaScriptCallMachineTransition,
) -> self::JavaScriptCallMachineTransition {
    match transition.clone() {
        self::JavaScriptCallMachineTransition::MachineResourceLimit(_, reason) => {
            self::JavaScriptCallMachineTransition::MachineResourceLimit(
                Box::new(machine.clone()),
                reason.clone(),
            )
        }
        _ => transition.clone(),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_argument_lists_update(
    machine: self::JavaScriptCallMachine,
    next: self::JavaScriptCallFrame,
    update: self::JavaScriptCallArgumentListsUpdate,
) -> self::JavaScriptCallMachineTransition {
    match next.clone() {
        self::JavaScriptCallFrame::CallFrame(
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
            let shared_constructor_field8 = *shared_constructor_field8;
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(
                    shared_constructor_field1,
                    shared_constructor_field2,
                ) => {
                    let shared_constructor_field1 = *shared_constructor_field1;
                    match update.clone() {
                        self::JavaScriptCallArgumentListsUpdate::ArgumentListsUpdated(
                            updated_lists,
                        ) => {
                            let count = 0f64;
                            {
                                let position = 0f64;
                                {
                                    let count_2 = self::source_ml_source1_temporary_accept_java_script_call_argument_lists_update_loop1312(machine.clone(), updated_lists.clone(), count, position);
                                    if (count_2 > (1048576f64)) {
                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(
                                            Box::new(machine.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105,
                                                115, 116, 115, 32, 115, 104, 97, 114, 101, 32, 116,
                                                104, 101, 32, 99, 111, 109, 112, 108, 101, 116,
                                                101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116,
                                                97, 99, 107, 32, 97, 114, 103, 117, 109, 101, 110,
                                                116, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98,
                                                117, 100, 103, 101, 116,
                                            ]),
                                        )
                                    } else {
                                        self::JavaScriptCallMachineTransition::MachineContinue(
                                            Box::new(self::replace_java_script_call_frame(
                                                shared_constructor_field1.clone(),
                                                shared_constructor_field2.clone(),
                                                {
                                                    let source_ml_source2_nominal_field29143 =
                                                        shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field29145 =
                                                            shared_constructor_field4;
                                                        {
                                                            let source_ml_source2_nominal_field29147 =
                                                                shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field29149 =
                                                                    shared_constructor_field6
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field29151 =
                                                                        shared_constructor_field7
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field29153 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field29155 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field29157 = shared_constructor_field10.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field29159 = shared_constructor_field11.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field29161 = shared_constructor_field12.clone();
                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field29143.clone(), source_ml_source2_nominal_field29145, source_ml_source2_nominal_field29147.clone(), source_ml_source2_nominal_field29149.clone(), source_ml_source2_nominal_field29151.clone(), Box::new(source_ml_source2_nominal_field29153.clone()), source_ml_source2_nominal_field29155.clone(), Some((Some(updated_lists.clone()),)), source_ml_source2_nominal_field29159.clone(), source_ml_source2_nominal_field29161.clone())
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                },
                                            )),
                                        )
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallArgumentListsUpdate::ArgumentListsInvalid(reason) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                reason.clone(),
                            )
                        }
                        self::JavaScriptCallArgumentListsUpdate::ArgumentListsResourceLimit(
                            reason_2,
                        ) => self::JavaScriptCallMachineTransition::MachineResourceLimit(
                            Box::new(machine.clone()),
                            reason_2.clone(),
                        ),
                    }
                }
            }
        }
    }
}
