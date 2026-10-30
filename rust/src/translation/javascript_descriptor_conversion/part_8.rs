// Generated from complete js/src/translation/javascript-descriptor-conversion.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result;
use super::convert_java_script_semantic_boolean;
use super::create_java_script_empty_descriptor;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::read_java_script_descriptor_conversion_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::source_ml_temporary_step_java_script_descriptor_conversion_loop30;
use super::test_java_script_semantic_accessor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_descriptor_field(
    state: self::JavaScriptDescriptorConversionState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptDescriptorConversionTransition {
    if (((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(
            _,
            source_ml_source1_data_property_value779,
            _,
            _,
        ) => source_ml_source1_data_property_value779,
    }) == 4f64)
        || ((match state.clone() {
            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                _,
                source_ml_source1_data_property_value781,
                _,
                _,
            ) => source_ml_source1_data_property_value781,
        }) == 5f64))
        && !(self::test_java_script_semantic_accessor(value.clone()))
    {
        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
            String::from("TypeError"),
            String::from("a descriptor accessor must be callable or undefined"),
        )
    } else {
        {
            let before = match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(
                    _,
                    _,
                    source_ml_source1_data_property_value782,
                    _,
                ) => source_ml_source1_data_property_value782.clone(),
            };
            {
                let enumerable = (match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        source_ml_source1_data_property_value784,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value784,
                }) == 0f64;
                {
                    let configurable = (match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(
                            _,
                            source_ml_source1_data_property_value786,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value786,
                    }) == 1f64;
                    {
                        let data_value = (match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                _,
                                source_ml_source1_data_property_value788,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value788,
                        }) == 2f64;
                        {
                            let writable = (match state.clone() {
                                self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                    _,
                                    source_ml_source1_data_property_value790,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value790,
                            }) == 3f64;
                            {
                                let getter = (match state.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value792, _, _) => {
                                        source_ml_source1_data_property_value792
                                    }
                                }) == 4f64;
                                {
                                    let setter = !((((enumerable || configurable) || data_value)
                                        || writable)
                                        || getter);
                                    {
                                        let flag = if (enumerable || configurable) || writable {
                                            self::convert_java_script_semantic_boolean(
                                                value.clone(),
                                            )
                                        } else {
                                            false
                                        };
                                        {
                                            let descriptor = {
                                                let source_ml_source1_spread793 = before.clone();
                                                {
                                                    let source_ml_source1_spread_field794 =
                                                        (source_ml_source1_spread793.clone()).0;
                                                    {
                                                        let source_ml_source1_spread_field795 =
                                                            (source_ml_source1_spread793.clone()).1;
                                                        {
                                                            let source_ml_source1_spread_field796 =
                                                                (source_ml_source1_spread793
                                                                    .clone())
                                                                .2;
                                                            {
                                                                let source_ml_source1_spread_field797 =
                                                                    (source_ml_source1_spread793
                                                                        .clone())
                                                                    .3;
                                                                {
                                                                    let source_ml_source1_spread_field798 = (source_ml_source1_spread793.clone()).4;
                                                                    {
                                                                        let source_ml_source1_spread_field799 = (source_ml_source1_spread793.clone()).5;
                                                                        {
                                                                            let source_ml_source1_spread_field800 = (source_ml_source1_spread793.clone()).6;
                                                                            {
                                                                                let source_ml_source1_spread_field801 = (source_ml_source1_spread793.clone()).7;
                                                                                {
                                                                                    let source_ml_source1_spread_field802 = (source_ml_source1_spread793.clone()).8;
                                                                                    {
                                                                                        let source_ml_source1_spread_field803 = (source_ml_source1_spread793.clone()).9;
                                                                                        {
                                                                                            let source_ml_source1_spread_field804 = (source_ml_source1_spread793.clone()).10;
                                                                                            {
                                                                                                let source_ml_source1_spread_field805 = (source_ml_source1_spread793.clone()).11;
                                                                                                {
                                                                                                    let source_ml_source1_field806 = enumerable || (before.clone()).3;
                                                                                                    {
                                                                                                        let source_ml_source1_field807 = if enumerable {
                                                                                                            flag
                                                                                                        } else {
                                                                                                            (before.clone()).2
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source1_field808 = configurable || (before.clone()).1;
                                                                                                            {
                                                                                                                let source_ml_source1_field809 = if configurable {
                                                                                                                    flag
                                                                                                                } else {
                                                                                                                    (before.clone()).0
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source1_field810 = data_value || (before.clone()).9;
                                                                                                                    {
                                                                                                                        let source_ml_source1_field811 = if data_value {
                                                                                                                            value.clone()
                                                                                                                        } else {
                                                                                                                            (before.clone()).8
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source1_field812 = writable || (before.clone()).11;
                                                                                                                            {
                                                                                                                                let source_ml_source1_field813 = if writable {
                                                                                                                                    flag
                                                                                                                                } else {
                                                                                                                                    (before.clone()).10
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source1_field814 = getter || (before.clone()).5;
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_field815 = if getter {
                                                                                                                                            value.clone()
                                                                                                                                        } else {
                                                                                                                                            (before.clone()).4
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_field816 = setter || (before.clone()).7;
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_field817 = if setter {
                                                                                                                                                    value.clone()
                                                                                                                                                } else {
                                                                                                                                                    (before.clone()).6
                                                                                                                                                };
                                                                                                                                                (source_ml_source1_field809, source_ml_source1_field808, source_ml_source1_field807, source_ml_source1_field806, source_ml_source1_field815, source_ml_source1_field814, source_ml_source1_field817, source_ml_source1_field816, source_ml_source1_field811, source_ml_source1_field810, source_ml_source1_field813, source_ml_source1_field812)
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
                                                }
                                            };
                                            self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(match state.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value818, _, _, _) => {
                                                    let source_ml_source1_data_property_value818 = *source_ml_source1_data_property_value818;
                                                    source_ml_source1_data_property_value818.clone()
                                                }
                                            }), (match state.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value819, _, _) => {
                                                    source_ml_source1_data_property_value819
                                                }
                                            } + 1f64), descriptor, false)))
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
pub fn step_java_script_descriptor_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptDescriptorConversionState,
) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error =
            self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if !(error).is_empty() {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else if match state.clone() {
            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                _,
                _,
                _,
                source_ml_source1_data_property_value820,
            ) => source_ml_source1_data_property_value820,
        } {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from(
                "a pending descriptor Get must be completed before its next step",
            ))
        } else {
            {
                let current = state.clone();
                {
                    let source_ml_source1_s821 =
                        self::source_ml_temporary_step_java_script_descriptor_conversion_loop30(
                            heap.clone(),
                            current,
                        );
                    match source_ml_source1_s821 {
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Done(current_2) => {
                                let current_2 = *current_2;
                                if (self::read_java_script_property_descriptor_error(match current_2.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value823, _) => {
                                        source_ml_source1_data_property_value823.clone()
                                    }
                                })) != "" {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(String::from("TypeError"), String::from("a descriptor cannot contain both data and accessor attributes"))
                                } else {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(match current_2.clone() {
                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value824, _) => {
                                            source_ml_source1_data_property_value824.clone()
                                        }
                                    })
                                }
                            }
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(source_ml_temporary_result30) => {
                                let source_ml_temporary_result30 = *source_ml_temporary_result30;
                                source_ml_temporary_result30.clone()
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_descriptor_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
) -> self::JavaScriptDescriptorConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if !(heap_error).is_empty() {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(heap_error.clone())
        } else {
            {
                let error =
                    self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                if !(error).is_empty() {
                    self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
                } else {
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f825) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f826) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f827) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f828) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f829) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f830,
                            source_ml_source1_f831,
                        ) => self::step_java_script_descriptor_conversion(
                            heap.clone(),
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                Box::new(receiver.clone()),
                                0f64,
                                self::create_java_script_empty_descriptor(),
                                false,
                            ),
                        ),
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_descriptor_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptDescriptorConversionState,
    completion: self::JavaScriptSemanticCompletion,
) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error =
            self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if !(error).is_empty() {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else if (match state.clone() {
            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                _,
                _,
                _,
                source_ml_source1_data_property_value832,
            ) => source_ml_source1_data_property_value832,
        }) {
            {
                let value_error = self::read_java_script_ordinary_value_error(
                    heap.clone(),
                    match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(
                            source_ml_source1_data_property_value833,
                        ) => {
                            let source_ml_source1_data_property_value833 =
                                *source_ml_source1_data_property_value833;
                            source_ml_source1_data_property_value833.clone()
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(
                            source_ml_source1_data_property_value834,
                        ) => {
                            let source_ml_source1_data_property_value834 =
                                *source_ml_source1_data_property_value834;
                            source_ml_source1_data_property_value834.clone()
                        }
                    },
                );
                if !(value_error).is_empty() {
                    self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(
                        value_error.clone(),
                    )
                } else {
                    match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(value) => {
                            let value = *value;
                            {
                                let applied = self::apply_java_script_descriptor_field(
                                    state.clone(),
                                    value.clone(),
                                );
                                match applied.clone() {
                                        self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(state_2) => {
                                            let state_2 = *state_2;
                                            self::step_java_script_descriptor_conversion(heap.clone(), state_2.clone())
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCall(source_ml_source1_f835, source_ml_source1_f836, source_ml_source1_f837, source_ml_source1_f838) => {
                                            let source_ml_source1_f835 = *source_ml_source1_f835;
                                            let source_ml_source1_f836 = *source_ml_source1_f836;
                                            let source_ml_source1_f837 = *source_ml_source1_f837;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(source_ml_source1_f839) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorThrow(source_ml_source1_f840) => {
                                            let source_ml_source1_f840 = *source_ml_source1_f840;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(source_ml_source1_f841, source_ml_source1_f842) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(source_ml_source1_f843) => {
                                            applied.clone()
                                        }
                                    }
                            }
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(value_2) => {
                            let value_2 = *value_2;
                            self::JavaScriptDescriptorConversionTransition::DescriptorThrow(
                                Box::new(value_2.clone()),
                            )
                        }
                    }
                }
            }
        } else {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from(
                "descriptor resumption needs its actual pending Get",
            ))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(
    units: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if at < (units.len() as f64) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String::from("semantic text requires complete ordered UTF16 code units"))
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_same_value_loop2(
    left_units: &[f64],
    right_units: &[f64],
    mut at: f64,
) -> self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    loop {
        return if at < (left_units.len() as f64) {
            {
                let ml_library_inequality_1 =
                    ((self::ml_array::optional_at(left_units, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| -1f64))
                        != ((self::ml_array::optional_at(
                            right_units,
                            self::ml_array::number_index(at),
                        ))
                        .unwrap_or_else(|| -1f64));
                if ml_library_inequality_1 {
                    self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(false)
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin3 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(
        (
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
            String,
            String,
            Vec<String>,
        ),
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_literal_property_index_loop4(
    key: &[f64],
    mut value: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result {
    loop {
        return if at < (key.len() as f64) {
            {
                let digit = (self::ml_array::optional_at(key, self::ml_array::number_index(at)))
                    .unwrap_or(f64::NAN)
                    - 48f64;
                if (!(self::ml_math::is_integer(digit)) || (digit < 0f64)) || (digit > 9f64) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Return(-1f64)
                } else {
                    {
                        let value_2 = (value * 10f64) + digit;
                        {
                            let at_2 = at + 1f64;
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result {
    MlTemporaryFindJavaScriptOrdinaryObjectLoop5Done,
    MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_ordinary_object_loop5(
    mut objects: Vec<self::JavaScriptOrdinaryObject>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let object =
                        self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(-1f64)
                    } else if (match object.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            source_ml_source1_data_property_value845,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value845.clone(),
                    }) == identity
                    {
                        self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(middle)
                    } else {
                        if (match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value847,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value847.clone(),
                        }) < identity
                        {
                            {
                                let low_2 = middle + 1f64;
                                {
                                    low = low_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let high_2 = middle;
                                {
                                    high = high_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result::MlTemporaryFindJavaScriptOrdinaryObjectLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result {
    MlTemporaryTestJavaScriptOrdinarySymbolLoop6Done,
    MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_ordinary_symbol_loop6(
    mut symbols: Vec<self::ml::Big>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let symbol =
                        self::ml_array::optional_at(&symbols, self::ml_array::number_index(middle));
                    if {
                        let source_ml_source1_comparison_operand848 = symbol.clone();
                        {
                            let source_ml_source1_comparison_operand849 = None::<()>;
                            match (
                                source_ml_source1_comparison_operand848,
                                source_ml_source1_comparison_operand849,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }
                        }
                    } {
                        self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(false)
                    } else if (symbol.clone().expect("a proven present value")) == identity {
                        self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Return(true)
                    } else {
                        if (symbol.clone().expect("a proven present value")) < identity {
                            {
                                let low_2 = middle + 1f64;
                                {
                                    low = low_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let high_2 = middle;
                                {
                                    high = high_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Result::MlTemporaryTestJavaScriptOrdinarySymbolLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result {
    MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Done,
    MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_ordinary_property_loop7(
    mut properties: Vec<self::JavaScriptOrdinaryProperty>,
    mut key: self::JavaScriptSemanticValue,
    mut at: f64,
) -> self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result {
    loop {
        return if at < (properties.len() as f64) {
            {
                let property =
                    self::ml_array::optional_at(&properties, self::ml_array::number_index(at));
                if !(match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && self::test_java_script_semantic_same_value(
                    {
                        let source_ml_source1_nullish_field_receiver850 = (property.clone(),);
                        if match (source_ml_source1_nullish_field_receiver850.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present851 =
                                    ((source_ml_source1_nullish_field_receiver850.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_nullish_field_present851 {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        source_ml_source1_data_property_value852,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value852 =
                                            *source_ml_source1_data_property_value852;
                                        source_ml_source1_data_property_value852.clone()
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (String::from(
                                    "Cannot read properties of undefined (reading 'key')",
                                ))
                                .encode_utf16()
                                .collect(),
                                message_present: true,
                            })
                            .raise()
                        }
                    },
                    key.clone(),
                ) {
                    self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Return(at)
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Result::MlTemporaryFindJavaScriptOrdinaryPropertyLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(String),
}
