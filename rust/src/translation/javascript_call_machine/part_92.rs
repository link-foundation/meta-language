// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptConstructionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Result;
use super::classify_java_script_array_species_constructor;
use super::dependency_function5;
use super::find_java_script_array_constructor_fact;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::normalize_java_script_signed_integer;
use super::read_java_script_executable_call_function;
use super::read_java_script_known_array_constructor_fact;
use super::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop956;
use super::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop957;
use super::source_ml_source1_temporary_count_java_script_leading_zero_bits_loop958;
use super::source_ml_source1_temporary_read_java_script_bound_construction_operands_loop962;
use super::source_ml_source1_temporary_round_java_script_binary32_loop959;
use super::source_ml_source3_record_order;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_number_bitwise_operation(operation: f64, left: f64, right: f64) -> f64 {
    if (operation == (6f64)) {
        ((-1f64) - self::normalize_java_script_signed_integer(left))
    } else {
        if (((operation == (3f64)) || (operation == (4f64))) || (operation == (5f64))) {
            {
                let count = (self::dependency_function5(right) % 32f64);
                {
                    let factor = 1f64;
                    {
                        let position = 0f64;
                        {
                            let factor_2 = self::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop956(count, factor, position);
                            if (operation == (3f64)) {
                                self::normalize_java_script_signed_integer(
                                    (self::dependency_function5(left) * factor_2),
                                )
                            } else {
                                if (operation == (4f64)) {
                                    (self::normalize_java_script_signed_integer(left) / factor_2)
                                        .floor()
                                } else {
                                    (self::dependency_function5(left) / factor_2).floor()
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if !(((operation == (0f64)) || (operation == (1f64))) || (operation == (2f64))) {
                f64::NAN
            } else {
                {
                    let first = self::dependency_function5(left);
                    {
                        let second = self::dependency_function5(right);
                        {
                            let result = 0f64;
                            {
                                let weight = 1f64;
                                {
                                    let position_2 = 0f64;
                                    {
                                        let result_2 = self::source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop957(operation, first, second, result, weight, position_2);
                                        self::normalize_java_script_signed_integer(result_2)
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
pub fn multiply_java_script_signed_integer_bits(left: f64, right: f64) -> f64 {
    {
        let first = self::dependency_function5(left);
        {
            let second = self::dependency_function5(right);
            {
                let low_first = (first % 65536f64);
                {
                    let low_second = (second % 65536f64);
                    {
                        let high_first = (first / 65536f64).floor();
                        {
                            let high_second = (second / 65536f64).floor();
                            self::normalize_java_script_signed_integer(
                                ((low_first * low_second)
                                    + ((((high_first * low_second) + (low_first * high_second))
                                        % 65536f64)
                                        * 65536f64)),
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn count_java_script_leading_zero_bits(value: f64) -> f64 {
    {
        let integer = self::dependency_function5(value);
        {
            let count = 32f64;
            {
                let count_2 =
                    self::source_ml_source1_temporary_count_java_script_leading_zero_bits_loop958(
                        integer, count,
                    );
                count_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn round_java_script_binary32(value: f64) -> f64 {
    if ((value.is_nan() || (value == (0f64))) || !(value.is_finite())) {
        value
    } else {
        {
            let negative = (value < (0f64));
            {
                let magnitude = if negative { (-value) } else { value };
                if (magnitude >= (3.402823669209385e+38f64)) {
                    if negative {
                        f64::NEG_INFINITY
                    } else {
                        f64::INFINITY
                    }
                } else {
                    {
                        let lower = 1.1754943508222875e-38f64;
                        {
                            let spacing = 1.401298464324817e-45f64;
                            {
                                let spacing_2 = self::source_ml_source1_temporary_round_java_script_binary32_loop959(magnitude, lower, spacing);
                                {
                                    let quotient = (magnitude / spacing_2);
                                    {
                                        let integral = quotient.floor();
                                        {
                                            let fraction = (quotient - integral);
                                            {
                                                let rounded = if ((fraction > (0.5f64))
                                                    || ((fraction == (0.5f64))
                                                        && ((integral % 2f64) == (1f64))))
                                                {
                                                    (integral + 1f64)
                                                } else {
                                                    integral
                                                };
                                                {
                                                    let result = (rounded * spacing_2);
                                                    {
                                                        let output = if (result
                                                            >= (3.402823669209385e+38f64))
                                                        {
                                                            f64::INFINITY
                                                        } else {
                                                            result
                                                        };
                                                        if negative { (-output) } else { output }
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
pub fn read_java_script_construction_fact(
    directory: self::JavaScriptArraySpeciesDirectory,
    candidate: self::JavaScriptSemanticValue,
    heap: Option<self::JavaScriptOrdinaryHeap>,
) -> Option<self::JavaScriptArrayConstructorFact> {
    match directory.clone() {
        self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(
            _,
            _,
            _,
            _,
            shared_constructor_field1,
            _,
        ) => {
            if !(match (heap.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }) {
                {
                    let classification = self::classify_java_script_array_species_constructor(
                        heap.clone().expect("a proven present value"),
                        directory.clone(),
                        candidate.clone(),
                    );
                    self::read_java_script_known_array_constructor_fact(classification.clone())
                }
            } else {
                if (match candidate.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => true,
                    _ => false,
                } && ((match candidate.clone() {
                    self::JavaScriptSemanticValue::Object(
                        _,
                        source_ml_source2_data_property_value18444,
                    ) => Some((source_ml_source2_data_property_value18444,)),
                    _ => None::<(bool,)>,
                })
                .clone()
                .expect("a proven present value"))
                .0)
                {
                    {
                        let source_ml_source2_array_read_value18448 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_array_read_index18449 =
                                self::find_java_script_array_constructor_fact(
                                    shared_constructor_field1.clone(),
                                    (((match candidate.clone() {
                                        self::JavaScriptSemanticValue::Object(
                                            source_ml_source2_data_property_value18447,
                                            _,
                                        ) => Some((
                                            source_ml_source2_data_property_value18447.clone(),
                                        )),
                                        _ => None::<(self::ml::Big,)>,
                                    })
                                    .clone()
                                    .expect("a proven present value"))
                                    .0)
                                        .clone(),
                                );
                            {
                                let ml_reference_array945 =
                                    source_ml_source2_array_read_value18448.clone();
                                let ml_reference_position945 =
                                    source_ml_source2_array_read_index18449;
                                let ml_reference_projection945 = self::ml_array::optional_at(
                                    &*ml_reference_array945.borrow(),
                                    self::ml_array::number_index(ml_reference_position945),
                                );
                                ml_reference_projection945
                            }
                        }
                    }
                } else {
                    None::<self::JavaScriptArrayConstructorFact>
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_bound_construction_operands(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptConstructionState,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (
            std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
            self::ml_string::Text,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
        ),
        Vec<f64>,
    )>,
> {
    match state.clone() {
        self::JavaScriptConstructionState::ConstructionState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
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
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let target = shared_constructor_field1.clone();
                {
                    let new_target = shared_constructor_field2.clone();
                    {
                        let total = {
                            let ml_reference_array946 = shared_constructor_field3.clone();
                            let ml_reference_projection946 =
                                ml_reference_array946.borrow().len() as f64;
                            ml_reference_projection946
                        };
                        {
                            let prefixes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
                            >::new(
                            )));
                            {
                                let depth = 0f64;
                                {
                                    let source_ml_source2_s18453 = self::source_ml_source1_temporary_read_java_script_bound_construction_operands_loop962(heap.clone(), state.clone(), target.clone(), new_target.clone(), total, prefixes.clone(), depth);
                                    match source_ml_source2_s18453.clone() {
                                        self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Result::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Done(target_2, new_target_2) => {
                                            let target_2 = *target_2;
                                            let new_target_2 = *new_target_2;
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source2_field18456 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                {
                                                    let source_ml_source3_record_field434 = source_ml_source2_field18456.clone();
                                                    {
                                                        let source_ml_source3_record_field435 = self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 97, 99, 121, 99, 108, 105, 99, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 32, 103, 114, 97, 112, 104]);
                                                        {
                                                            let source_ml_source3_record_field436 = new_target_2.clone();
                                                            {
                                                                let source_ml_source3_record_field437 = false;
                                                                {
                                                                    let source_ml_source3_record_field438 = target_2.clone();
                                                                    ((source_ml_source3_record_field434.clone(), source_ml_source3_record_field435.clone(), source_ml_source3_record_field436.clone(), source_ml_source3_record_field437, source_ml_source3_record_field438.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![97f64, 114f64, 103f64, 117f64, 109f64, 101f64, 110f64, 116f64, 115f64], vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![110f64, 101f64, 119f64, 84f64, 97f64, 114f64, 103f64, 101f64, 116f64], vec![114f64, 101f64, 115f64, 111f64, 117f64, 114f64, 99f64, 101f64], vec![116f64, 97f64, 114f64, 103f64, 101f64, 116f64]], vec![4f64, 2f64, 0f64, 1f64, 3f64]))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }))
                                        }
                                        self::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Result::MlSource1TemporaryReadJavaScriptBoundConstructionOperandsLoop962Return(source_ml_source1_temporary_result962) => {
                                            source_ml_source1_temporary_result962.clone()
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
pub fn read_java_script_active_construction(
    state: self::JavaScriptConstructionState,
) -> self::JavaScriptConstructionState {
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
            source_ml_source2_data_property_value18459,
        ) => ((source_ml_source2_data_property_value18459
            .clone()
            .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
        .0)
            .clone(),
    })
    .clone()
    .unwrap_or_else(|| state.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn adapt_java_script_bound_construction(
    state: self::JavaScriptConstructionState,
    transition: self::JavaScriptConstructionTransition,
) -> self::JavaScriptConstructionTransition {
    if (((match transition.clone() {
        self::JavaScriptConstructionTransition::ConstructionCompleted(_, _) => true,
        _ => false,
    } || match transition.clone() {
        self::JavaScriptConstructionTransition::ConstructionThrow(_, _) => true,
        _ => false,
    }) || match transition.clone() {
        self::JavaScriptConstructionTransition::ConstructionError(_, _, _, _) => true,
        _ => false,
    }) || match transition.clone() {
        self::JavaScriptConstructionTransition::ConstructionInvalid(_, _) => true,
        _ => false,
    }) {
        transition.clone()
    } else {
        {
            let prior = match state.clone() {
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
                    source_ml_source2_data_property_value18470,
                    _,
                ) => ((source_ml_source2_data_property_value18470
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,)))
                .0)
                    .clone(),
            };
            {
                let base = match state.clone() {
                    self::JavaScriptConstructionState::ConstructionState(
                        source_ml_source2_nominal_rest_field18471,
                        source_ml_source2_nominal_rest_field18472,
                        source_ml_source2_nominal_rest_field18473,
                        source_ml_source2_nominal_rest_field18474,
                        source_ml_source2_nominal_rest_field18475,
                        source_ml_source2_nominal_rest_field18476,
                        source_ml_source2_nominal_rest_field18477,
                        source_ml_source2_nominal_rest_field18478,
                        source_ml_source2_nominal_rest_field18479,
                        source_ml_source2_nominal_rest_field18480,
                        source_ml_source2_nominal_rest_field18481,
                        _,
                        source_ml_source2_nominal_rest_field18482,
                    ) => {
                        let source_ml_source2_nominal_rest_field18471 =
                            *source_ml_source2_nominal_rest_field18471;
                        let source_ml_source2_nominal_rest_field18472 =
                            *source_ml_source2_nominal_rest_field18472;
                        let source_ml_source2_nominal_rest_field18476 =
                            *source_ml_source2_nominal_rest_field18476;
                        let source_ml_source2_nominal_rest_field18477 =
                            *source_ml_source2_nominal_rest_field18477;
                        let source_ml_source2_nominal_rest_field18478 =
                            *source_ml_source2_nominal_rest_field18478;
                        self::JavaScriptConstructionState::ConstructionState(
                            Box::new(source_ml_source2_nominal_rest_field18471.clone()),
                            Box::new(source_ml_source2_nominal_rest_field18472.clone()),
                            source_ml_source2_nominal_rest_field18473.clone(),
                            source_ml_source2_nominal_rest_field18474.clone(),
                            source_ml_source2_nominal_rest_field18475.clone(),
                            Box::new(source_ml_source2_nominal_rest_field18476.clone()),
                            Box::new(source_ml_source2_nominal_rest_field18477.clone()),
                            Box::new(source_ml_source2_nominal_rest_field18478.clone()),
                            source_ml_source2_nominal_rest_field18479,
                            source_ml_source2_nominal_rest_field18480.clone(),
                            source_ml_source2_nominal_rest_field18481.clone(),
                            None::<(Option<self::JavaScriptConstructionCall>,)>,
                            source_ml_source2_nominal_rest_field18482.clone(),
                        )
                    }
                };
                match base.clone() {
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
                            let operation = (((match transition.clone() {
                                self::JavaScriptConstructionTransition::ConstructionContinue(_, source_ml_source2_data_property_value18483) => {
                                    let source_ml_source2_data_property_value18483 = *source_ml_source2_data_property_value18483;
                                    Some((source_ml_source2_data_property_value18483.clone(),))
                                }
                                self::JavaScriptConstructionTransition::ConstructionCall(_, source_ml_source2_data_property_value18484, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value18484 = *source_ml_source2_data_property_value18484;
                                    Some((source_ml_source2_data_property_value18484.clone(),))
                                }
                                self::JavaScriptConstructionTransition::ConstructionAllocate(_, source_ml_source2_data_property_value18485, _) => {
                                    let source_ml_source2_data_property_value18485 = *source_ml_source2_data_property_value18485;
                                    Some((source_ml_source2_data_property_value18485.clone(),))
                                }
                                self::JavaScriptConstructionTransition::ConstructionNeedAuthority(_, source_ml_source2_data_property_value18486, _) => {
                                    let source_ml_source2_data_property_value18486 = *source_ml_source2_data_property_value18486;
                                    Some((source_ml_source2_data_property_value18486.clone(),))
                                }
                                self::JavaScriptConstructionTransition::ConstructionNeedImplementation(_, source_ml_source2_data_property_value18487, _, _, _, _, _) => {
                                    let source_ml_source2_data_property_value18487 = *source_ml_source2_data_property_value18487;
                                    Some((source_ml_source2_data_property_value18487.clone(),))
                                }
                                self::JavaScriptConstructionTransition::ConstructionResourceLimit(_, source_ml_source2_data_property_value18488, _) => {
                                    let source_ml_source2_data_property_value18488 = *source_ml_source2_data_property_value18488;
                                    Some((source_ml_source2_data_property_value18488.clone(),))
                                }
                                _ => {
                                    None::<(self::JavaScriptConstructionState,)>
                                }
                            }).clone().expect("a proven present value")).0).clone();
                            {
                                let call = match operation.clone() {
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
                                        source_ml_source2_data_property_value18489,
                                        _,
                                    ) => ((source_ml_source2_data_property_value18489
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptConstructionCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                };
                                {
                                    let next = if match (call.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        {
                                            let source_ml_source2_nominal_field18491 =
                                                shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field18493 =
                                                    shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_field18495 =
                                                        shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field18497 =
                                                            shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field18499 =
                                                                shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field18501 =
                                                                    shared_constructor_field6
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field18503 =
                                                                        shared_constructor_field7
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field18505 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field18507 = shared_constructor_field9;
                                                                            {
                                                                                let source_ml_source2_nominal_field18509 = shared_constructor_field10.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field18511 = shared_constructor_field11.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18513 = shared_constructor_field12.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18515 = shared_constructor_field13.clone();
                                                                                            self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field18491.clone()), Box::new(source_ml_source2_nominal_field18493.clone()), source_ml_source2_nominal_field18495.clone(), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]), source_ml_source2_nominal_field18499.clone(), Box::new(source_ml_source2_nominal_field18501.clone()), Box::new(source_ml_source2_nominal_field18503.clone()), Box::new(source_ml_source2_nominal_field18505.clone()), source_ml_source2_nominal_field18507, source_ml_source2_nominal_field18509.clone(), source_ml_source2_nominal_field18511.clone(), source_ml_source2_nominal_field18513.clone(), Some((Some(operation.clone()),)))
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
                                    } else {
                                        {
                                            let source_ml_source2_nominal_field18520 =
                                                shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field18522 =
                                                    shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_field18524 =
                                                        shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field18526 =
                                                            shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field18528 =
                                                                shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field18530 =
                                                                    shared_constructor_field6
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field18532 =
                                                                        shared_constructor_field7
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field18534 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field18536 = shared_constructor_field9;
                                                                            {
                                                                                let source_ml_source2_nominal_field18538 = shared_constructor_field10.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field18540 = shared_constructor_field11.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field18542 = shared_constructor_field12.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field18544 = shared_constructor_field13.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override18548 = call.clone().expect("a proven present value");
                                                                                                self::JavaScriptConstructionState::ConstructionState(Box::new(source_ml_source2_nominal_field18520.clone()), Box::new(source_ml_source2_nominal_field18522.clone()), source_ml_source2_nominal_field18524.clone(), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]), source_ml_source2_nominal_field18528.clone(), Box::new(source_ml_source2_nominal_field18530.clone()), Box::new(source_ml_source2_nominal_field18532.clone()), Box::new(source_ml_source2_nominal_field18534.clone()), source_ml_source2_nominal_field18536, source_ml_source2_nominal_field18538.clone(), source_ml_source2_nominal_field18540.clone(), Some((Some(source_ml_source2_nominal_override18548.clone()),)), Some((Some(operation.clone()),)))
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
                                    {
                                        let heap = (((match transition.clone() {
                                            self::JavaScriptConstructionTransition::ConstructionContinue(source_ml_source2_data_property_value18549, _) => {
                                                let source_ml_source2_data_property_value18549 = *source_ml_source2_data_property_value18549;
                                                Some((source_ml_source2_data_property_value18549.clone(),))
                                            }
                                            self::JavaScriptConstructionTransition::ConstructionCall(source_ml_source2_data_property_value18550, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value18550 = *source_ml_source2_data_property_value18550;
                                                Some((source_ml_source2_data_property_value18550.clone(),))
                                            }
                                            self::JavaScriptConstructionTransition::ConstructionAllocate(source_ml_source2_data_property_value18551, _, _) => {
                                                let source_ml_source2_data_property_value18551 = *source_ml_source2_data_property_value18551;
                                                Some((source_ml_source2_data_property_value18551.clone(),))
                                            }
                                            self::JavaScriptConstructionTransition::ConstructionNeedAuthority(source_ml_source2_data_property_value18552, _, _) => {
                                                let source_ml_source2_data_property_value18552 = *source_ml_source2_data_property_value18552;
                                                Some((source_ml_source2_data_property_value18552.clone(),))
                                            }
                                            self::JavaScriptConstructionTransition::ConstructionNeedImplementation(source_ml_source2_data_property_value18553, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value18553 = *source_ml_source2_data_property_value18553;
                                                Some((source_ml_source2_data_property_value18553.clone(),))
                                            }
                                            self::JavaScriptConstructionTransition::ConstructionResourceLimit(source_ml_source2_data_property_value18554, _, _) => {
                                                let source_ml_source2_data_property_value18554 = *source_ml_source2_data_property_value18554;
                                                Some((source_ml_source2_data_property_value18554.clone(),))
                                            }
                                            _ => {
                                                None::<(self::JavaScriptOrdinaryHeap,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone();
                                        match transition.clone() {
                                            self::JavaScriptConstructionTransition::ConstructionContinue(_, _) => {
                                                self::JavaScriptConstructionTransition::ConstructionContinue(Box::new(heap.clone()), Box::new(next.clone()))
                                            }
                                            self::JavaScriptConstructionTransition::ConstructionCall(_, _, callee, receiver, arguments, constructing, new_target) => {
                                                let callee = *callee;
                                                let receiver = *receiver;
                                                let new_target = *new_target;
                                                self::JavaScriptConstructionTransition::ConstructionCall(Box::new(heap.clone()), Box::new(next.clone()), Box::new(callee.clone()), Box::new(receiver.clone()), arguments.clone(), constructing, Box::new(new_target.clone()))
                                            }
                                            self::JavaScriptConstructionTransition::ConstructionAllocate(_, _, prototype) => {
                                                let prototype = *prototype;
                                                self::JavaScriptConstructionTransition::ConstructionAllocate(Box::new(heap.clone()), Box::new(next.clone()), Box::new(prototype.clone()))
                                            }
                                            self::JavaScriptConstructionTransition::ConstructionNeedAuthority(_, _, constructor) => {
                                                let constructor = *constructor;
                                                self::JavaScriptConstructionTransition::ConstructionNeedAuthority(Box::new(heap.clone()), Box::new(next.clone()), Box::new(constructor.clone()))
                                            }
                                            self::JavaScriptConstructionTransition::ConstructionNeedImplementation(_, _, constructor_2, new_target_2, arguments_2, operation_2, reason) => {
                                                let constructor_2 = *constructor_2;
                                                let new_target_2 = *new_target_2;
                                                self::JavaScriptConstructionTransition::ConstructionNeedImplementation(Box::new(heap.clone()), Box::new(next.clone()), Box::new(constructor_2.clone()), Box::new(new_target_2.clone()), arguments_2.clone(), operation_2.clone(), reason.clone())
                                            }
                                            self::JavaScriptConstructionTransition::ConstructionResourceLimit(_, _, reason_2) => {
                                                self::JavaScriptConstructionTransition::ConstructionResourceLimit(Box::new(heap.clone()), Box::new(next.clone()), reason_2.clone())
                                            }
                                            _ => {
                                                transition.clone()
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
pub fn read_java_script_construction_mode(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    fact: self::JavaScriptArrayConstructorFact,
) -> Option<self::ml_string::Text> {
    match fact.clone() {
        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
            shared_constructor_field2,
            _,
            _,
            shared_constructor_field3,
            _,
        ) => {
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => Some({
                    let actual = {
                        let source_ml_source2_array_read_value18580 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_array_read_index18581 =
                                self::find_java_script_ordinary_object(
                                    shared_constructor_field1.clone(),
                                    shared_constructor_field2.clone(),
                                );
                            {
                                let ml_reference_array947 =
                                    source_ml_source2_array_read_value18580.clone();
                                let ml_reference_position947 =
                                    source_ml_source2_array_read_index18581;
                                let ml_reference_projection947 = self::ml_array::optional_at(
                                    &*ml_reference_array947.borrow(),
                                    self::ml_array::number_index(ml_reference_position947),
                                );
                                ml_reference_projection947
                            }
                        }
                    };
                    if {
                        let source_ml_source2_comparison_operand18583 = {
                            let source_ml_source2_optional_chain_receiver18582 = (actual.clone(),);
                            if match (source_ml_source2_optional_chain_receiver18582.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                Some({
                                    let source_ml_source1_temporary_optional_chain970 =
                                        ((source_ml_source2_optional_chain_receiver18582.clone())
                                            .0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source1_temporary_optional_chain970.clone() {
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
                        match (
                            source_ml_source2_comparison_operand18583.clone(),
                            self::ml_string::Text::from_units(vec![
                                98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79,
                                98, 106, 101, 99, 116,
                            ]),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    } {
                        self::ml_string::Text::from_units(vec![
                            98, 111, 117, 110, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110,
                        ])
                    } else {
                        {
                            let intrinsic_kind = match fact.clone() {
                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value18585,
                                ) => ((source_ml_source2_data_property_value18585
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                .0)
                                    .clone(),
                            };
                            if {
                                let source_ml_source2_comparison_operand18587 = None::<()>;
                                !(match (
                                    intrinsic_kind.clone(),
                                    source_ml_source2_comparison_operand18587.clone(),
                                ) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                })
                            } {
                                intrinsic_kind.clone().expect("a proven present value")
                            } else {
                                if shared_constructor_field3 {
                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121])
                                } else {
                                    ({
                                            let source_ml_source2_optional_chain_receiver18590 = (self::read_java_script_executable_call_function(program.clone(), heap.clone(), shared_constructor_field2.clone()),);
                                            if match (source_ml_source2_optional_chain_receiver18590.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                Some({
                                                    let source_ml_source1_temporary_optional_chain971 = ((source_ml_source2_optional_chain_receiver18590.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_temporary_optional_chain971.clone() {
                                                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value18591, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value18591.clone()
                                                        }
                                                    }
                                                })
                                            } else {
                                                None::<self::ml_string::Text>
                                            }
                                        }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))
                                }
                            }
                        }
                    }
                }),
            }
        }
    }
}
