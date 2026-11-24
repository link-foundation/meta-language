// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptCollectionIteratorResultCompletion;
use super::JavaScriptCollectionIteratorResultState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::allocate_java_script_array;
use super::create_java_script_iterator_result_descriptor;
use super::define_java_script_ordinary_property;
use super::insert_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_string;
use super::read_java_script_semantic_text_unit;
use super::step_java_script_collection_iterator_result;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_collection_iterator_result_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptCollectionIteratorResultState,
    identity: self::ml::Big,
) -> self::JavaScriptCollectionIteratorResultCompletion {
    {
        let request =
            self::step_java_script_collection_iterator_result(heap.clone(), state.clone());
        match request.clone() {
            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultContinue(_, _) => {
                request.clone()
            }
            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultAllocate(
                _,
                _,
                _,
                _,
            ) => {
                if (identity <= (self::ml::Big::from_i128(0))) || (identity > (self::ml::Big::from_i128(18_446_744_073_709_551_615))) {
                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                } else if (match state.clone() {
                        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(source_ml_source1_data_property_value18050, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value18050.clone()
                        }
                    }) == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121])) {
                        {
                            let allocated = self::allocate_java_script_array(heap.clone(), identity.clone(), match state.clone() {
                                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, source_ml_source1_data_property_value18051, _, _, _, _) => {
                                    let source_ml_source1_data_property_value18051 = *source_ml_source1_data_property_value18051;
                                    source_ml_source1_data_property_value18051.clone()
                                }
                            }, 2f64);
                            match allocated {
                                self::JavaScriptArrayCompletion::ArrayNormal(source_ml_source1_f18052, source_ml_source1_f18053) => {
                                    let source_ml_source1_f18052 = *source_ml_source1_f18052;
                                    let source_ml_source1_f18053 = *source_ml_source1_f18053;
                                    {
                                        let first = self::define_java_script_ordinary_property(source_ml_source1_f18052.clone(), identity.clone(), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![48f64]))), self::create_java_script_iterator_result_descriptor(match state.clone() {
                                            self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, source_ml_source1_data_property_value18085, _, _) => {
                                                let source_ml_source1_data_property_value18085 = *source_ml_source1_data_property_value18085;
                                                source_ml_source1_data_property_value18085.clone()
                                            }
                                        }));
                                        {
                                            let second = {
let ml_product_condition_2 = { first.borrow().clone() };
if ml_product_condition_2 {
                                                self::define_java_script_ordinary_property(({ first.borrow().clone() }).1, identity.clone(), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![49f64]))), self::create_java_script_iterator_result_descriptor(match state.clone() {
                                                    self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, _, source_ml_source1_data_property_value18086, _) => {
                                                        let source_ml_source1_data_property_value18086 = *source_ml_source1_data_property_value18086;
                                                        source_ml_source1_data_property_value18086.clone()
                                                    }
                                                }))
                                            } else {
                                                first.clone()
                                            }
};
                                            if ((!(({ first.borrow().clone() }).3) || !(({ second.borrow().clone() }).3)) || ((({ first.borrow().clone() }).0) != (self::ml_string::Text::from_units(vec![])))) || ((({ second.borrow().clone() }).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104, 32, 101, 110, 116, 114, 121, 32, 65, 114, 114, 97, 121, 32, 102, 105, 101, 108, 100, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115]))
                                            } else {
                                                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultContinue(Box::new(({ second.borrow().clone() }).1), Box::new({
                                                    let source_ml_source1_nominal_field18088 = match state.clone() {
                                                        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(source_ml_source1_data_property_value18089, _, _, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value18089.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field18090 = match state.clone() {
                                                            self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, source_ml_source1_data_property_value18091, _, _, _, _, _, _) => {
                                                                let source_ml_source1_data_property_value18091 = *source_ml_source1_data_property_value18091;
                                                                source_ml_source1_data_property_value18091.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field18092 = match state.clone() {
                                                                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, source_ml_source1_data_property_value18093, _, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value18093.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field18094 = match state.clone() {
                                                                    self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, source_ml_source1_data_property_value18095, _, _, _, _) => {
                                                                        let source_ml_source1_data_property_value18095 = *source_ml_source1_data_property_value18095;
                                                                        source_ml_source1_data_property_value18095.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field18096 = match state.clone() {
                                                                        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, source_ml_source1_data_property_value18097, _, _, _) => {
                                                                            let source_ml_source1_data_property_value18097 = *source_ml_source1_data_property_value18097;
                                                                            source_ml_source1_data_property_value18097.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field18098 = match state.clone() {
                                                                            self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, source_ml_source1_data_property_value18099, _, _) => {
                                                                                let source_ml_source1_data_property_value18099 = *source_ml_source1_data_property_value18099;
                                                                                source_ml_source1_data_property_value18099.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_nominal_field18100 = match state.clone() {
                                                                                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, _, source_ml_source1_data_property_value18101, _) => {
                                                                                    let source_ml_source1_data_property_value18101 = *source_ml_source1_data_property_value18101;
                                                                                    source_ml_source1_data_property_value18101.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source1_nominal_field18102 = match state.clone() {
                                                                                    self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, _, _, source_ml_source1_data_property_value18103) => {
                                                                                        source_ml_source1_data_property_value18103
                                                                                    }
                                                                                };
                                                                                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(self::ml_string::Text::from_units(vec![114, 101, 115, 117, 108, 116]), Box::new(source_ml_source1_nominal_field18090), source_ml_source1_nominal_field18092, Box::new(source_ml_source1_nominal_field18094), Box::new(source_ml_source1_nominal_field18096), Box::new(source_ml_source1_nominal_field18098), Box::new(source_ml_source1_f18053.clone()), source_ml_source1_nominal_field18102)
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
                                self::JavaScriptArrayCompletion::ArrayPropertyComplete(_, _, _) => {
                                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 110, 116, 114, 105, 101, 115, 32, 112, 97, 105, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 103, 108, 111, 98, 97, 108, 32, 65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                                }
                                self::JavaScriptArrayCompletion::ArrayNumberRequest(_, _, _) => {
                                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 110, 116, 114, 105, 101, 115, 32, 112, 97, 105, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 103, 108, 111, 98, 97, 108, 32, 65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                                }
                                self::JavaScriptArrayCompletion::ArrayDefinitionContinue(_, _) => {
                                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 110, 116, 114, 105, 101, 115, 32, 112, 97, 105, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 103, 108, 111, 98, 97, 108, 32, 65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                                }
                                self::JavaScriptArrayCompletion::ArrayInvalid(_, _) => {
                                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 110, 116, 114, 105, 101, 115, 32, 112, 97, 105, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 103, 108, 111, 98, 97, 108, 32, 65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                                }
                                self::JavaScriptArrayCompletion::ArrayError(_, _, _) => {
                                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 110, 116, 114, 105, 101, 115, 32, 112, 97, 105, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 103, 108, 111, 98, 97, 108, 32, 65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                                }
                                self::JavaScriptArrayCompletion::ArrayResourceLimit(_, reason, _) => {
                                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), reason.clone())
                                }
                            }
                        }
                    } else {
                        {
                            let allocated_2 = self::insert_java_script_ordinary_object(heap.clone(), identity.clone(), match state.clone() {
                                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, source_ml_source1_data_property_value18186, _, _, _) => {
                                    let source_ml_source1_data_property_value18186 = *source_ml_source1_data_property_value18186;
                                    source_ml_source1_data_property_value18186.clone()
                                }
                            });
                            if (({ allocated_2.borrow().clone() }).2) {
                                {
                                    let value = self::define_java_script_ordinary_property(({ allocated_2.borrow().clone() }).1, identity.clone(), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![118f64, 97f64, 108f64, 117f64, 101f64]))), self::create_java_script_iterator_result_descriptor(match state.clone() {
                                        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, _, source_ml_source1_data_property_value18187, _) => {
                                            let source_ml_source1_data_property_value18187 = *source_ml_source1_data_property_value18187;
                                            source_ml_source1_data_property_value18187.clone()
                                        }
                                    }));
                                    {
                                        let done = {
let ml_product_condition_1 = { value.borrow().clone() };
if ml_product_condition_1 {
                                            self::define_java_script_ordinary_property(({ value.borrow().clone() }).1, identity.clone(), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![100f64, 111f64, 110f64, 101f64]))), self::create_java_script_iterator_result_descriptor(self::JavaScriptSemanticValue::Boolean(match state.clone() {
                                                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, _, _, source_ml_source1_data_property_value18188) => {
                                                    source_ml_source1_data_property_value18188
                                                }
                                            })))
                                        } else {
                                            value.clone()
                                        }
};
                                        if ((!(({ value.borrow().clone() }).3) || !(({ done.borrow().clone() }).3)) || ((({ value.borrow().clone() }).0) != (self::ml_string::Text::from_units(vec![])))) || ((({ done.borrow().clone() }).0) != (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![97, 32, 102, 114, 101, 115, 104, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 101, 114, 101, 100, 32, 118, 97, 108, 117, 101, 47, 100, 111, 110, 101, 32, 100, 97, 116, 97, 32, 102, 105, 101, 108, 100, 115]))
                                        } else {
                                            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultNormal(Box::new(({ done.borrow().clone() }).1), Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), false)))
                                        }
                                    }
                                }
                            } else {
                                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), ({ allocated_2.borrow().clone() }).0)
                            }
                        }
                    }
            }
            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultNormal(_, _) => {
                request.clone()
            }
            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(_, _) => {
                request.clone()
            }
            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultTypeError(_, _) => {
                request.clone()
            }
            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultResourceLimit(
                _,
                _,
                _,
            ) => request.clone(),
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_weak_key_eligibility_loop2(
    mut value: self::JavaScriptSemanticValue,
    mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>,
    mut low: f64,
    mut high: f64,
) -> f64 {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let row = {
                        let ml_reference_array243 = facts.clone();
                        let ml_reference_position243 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array243.borrow(),
                            self::ml_array::number_index(ml_reference_position243),
                        )
                    };
                    if !(match (row.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && {
                        let source_ml_source1_comparison_operand18194 =
                            match row.clone().expect("a proven present value") {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    source_ml_source1_data_property_value18191,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value18191.clone(),
                            };
                        {
                            let source_ml_source1_comparison_operand18195 = match value.clone() {
                                self::JavaScriptSemanticValue::Undefined => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Null => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Number(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Integer(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Text(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    None::<self::ml::Big>
                                }
                                self::JavaScriptSemanticValue::Symbol(
                                    source_ml_source1_data_property_value18192,
                                ) => Some(source_ml_source1_data_property_value18192.clone()),
                                self::JavaScriptSemanticValue::Object(
                                    source_ml_source1_data_property_value18193,
                                    _,
                                ) => Some(source_ml_source1_data_property_value18193.clone()),
                            };
                            if match (
                                source_ml_source1_comparison_operand18195.clone(),
                                None::<self::ml::Big>,
                            ) {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                {
                                    let source_ml_source1_comparison_order18196 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source1_comparison_operand18194.clone());
                                    (source_ml_source1_comparison_order18196 == 1f64)
                                }
                            } else {
                                source_ml_source1_comparison_operand18194
                                    < (source_ml_source1_comparison_operand18195
                                        .clone()
                                        .expect("a proven present value"))
                            }
                        }
                    } {
                        {
                            let low_2 = middle + 1f64;
                            {
                                low = low_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            high = middle;
                            continue;
                        }
                    }
                }
            }
        } else {
            low
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop3Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop3Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop3(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result {
    loop {
        return if at
            < ({
                let ml_reference_array244 = units.clone();
                ml_reference_array244.borrow().len() as f64
            }) {
            {
                let unit = ({
                    let ml_reference_array245 = units.clone();
                    let ml_reference_position245 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array245.borrow(),
                        self::ml_array::number_index(ml_reference_position245),
                    )
                })
                .clone()
                .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result {
    MlTemporaryTestJavaScriptSemanticTextEqualLoop4Done,
    MlTemporaryTestJavaScriptSemanticTextEqualLoop4Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_text_equal_loop4(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut at: f64,
) -> self::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result {
    loop {
        return if at < length {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Text(units) => ({
                        let ml_reference_array246 = units.clone();
                        let ml_reference_position246 = at;
                        self::ml_array::optional_at(
                            &*ml_reference_array246.borrow(),
                            self::ml_array::number_index(ml_reference_position246),
                        )
                    })
                    .clone()
                    .unwrap_or_else(|| -1f64),
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_f18203) => {
                        let text = source_ml_source1_f18203.clone();
                        {
                            let source_ml_source1_string_unit_operand18257 = text;
                            {
                                let source_ml_source1_string_unit_operand18258 = at;
                                (std::rc::Rc::new(
                                    move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                        {
                                            let index = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if (index < 0f64) || (index >= (text_2.length())) {
                                                f64::NAN
                                            } else {
                                                ({
                                                let source_ml_source1_string_unit_operand18264 = {
                                                    let source_ml_source1_string_range_operand18259 = text_2.clone();
                                                    {
                                                        let source_ml_source1_string_range_operand18260 = index;
                                                        {
                                                            let source_ml_source1_string_range_operand18261 = index + 1f64;
                                                            {
                                                                let source_ml_source1_string_range_position18262 = source_ml_source1_string_range_operand18260;
                                                                {
                                                                    let source_ml_source1_string_range_position18263 = source_ml_source1_string_range_operand18261;
                                                                    source_ml_source1_string_range_operand18259.slice(source_ml_source1_string_range_position18262, source_ml_source1_string_range_position18263)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_string_unit_operand18265 = 0f64;
                                                    source_ml_source1_string_unit_operand18264.code_point_at(source_ml_source1_string_unit_operand18265)
                                                }
                                            }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        }
                                    },
                                ))(
                                    source_ml_source1_string_unit_operand18257,
                                    source_ml_source1_string_unit_operand18258,
                                )
                            }
                        }
                    }
                    _ => -1f64,
                };
                {
                    let b = match right.clone() {
                        self::JavaScriptSemanticValue::Text(units_2) => ({
                            let ml_reference_array247 = units_2.clone();
                            let ml_reference_position247 = at;
                            self::ml_array::optional_at(
                                &*ml_reference_array247.borrow(),
                                self::ml_array::number_index(ml_reference_position247),
                            )
                        })
                        .clone()
                        .unwrap_or_else(|| -1f64),
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_f18286) => {
                            let text_2 = source_ml_source1_f18286.clone();
                            {
                                let source_ml_source1_string_unit_operand18340 = text_2.clone();
                                {
                                    let source_ml_source1_string_unit_operand18341 = at;
                                    (std::rc::Rc::new(
                                        move |text_3: self::ml_string::Text,
                                              position: f64|
                                              -> f64 {
                                            {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if (index < 0f64) || (index >= (text_3.length())) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                    let source_ml_source1_string_unit_operand18347 = {
                                                        let source_ml_source1_string_range_operand18342 = text_3.clone();
                                                        {
                                                            let source_ml_source1_string_range_operand18343 = index;
                                                            {
                                                                let source_ml_source1_string_range_operand18344 = index + 1f64;
                                                                {
                                                                    let source_ml_source1_string_range_position18345 = source_ml_source1_string_range_operand18343;
                                                                    {
                                                                        let source_ml_source1_string_range_position18346 = source_ml_source1_string_range_operand18344;
                                                                        source_ml_source1_string_range_operand18342.slice(source_ml_source1_string_range_position18345, source_ml_source1_string_range_position18346)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_string_unit_operand18348 = 0f64;
                                                        source_ml_source1_string_unit_operand18347.code_point_at(source_ml_source1_string_unit_operand18348)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            }
                                        },
                                    ))(
                                        source_ml_source1_string_unit_operand18340,
                                        source_ml_source1_string_unit_operand18341,
                                    )
                                }
                            }
                        }
                        _ => -1f64,
                    };
                    {
                        let ml_library_inequality_1 = a != b;
                        if ml_library_inequality_1 {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Return(false)
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
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result {
    MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop7(
    mut value: self::JavaScriptSemanticValue,
    mut length: f64,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result {
    loop {
        if at < length {
            {
                let source_ml_temporary_construct_java_script_semantic_text_units_ignored8 = {
                    let ml_method_owner248 = units.clone();
                    let ml_method_argument248_0 =
                        self::read_java_script_semantic_text_unit(value.clone(), at);
                    let ml_inserted = vec![ml_method_argument248_0];
                    let mut ml_method_values248 = ml_method_owner248.borrow_mut();
                    let ml_length = ml_method_values248
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values248.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        }
        return {
            self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result {
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Done,
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_property_descriptor_entry_loop9(
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut key: self::JavaScriptSemanticValue,
    mut at: f64,
) -> self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result {
    loop {
        return if at
            < ({
                let ml_reference_array249 = properties.clone();
                ml_reference_array249.borrow().len() as f64
            }) {
            {
                let property = {
                    let ml_reference_array250 = properties.clone();
                    let ml_reference_position250 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array250.borrow(),
                        self::ml_array::number_index(ml_reference_position250),
                    )
                };
                if !(match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && self::test_java_script_semantic_same_value(
                    match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value18370,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value18370 =
                                *source_ml_source1_data_property_value18370;
                            source_ml_source1_data_property_value18370.clone()
                        }
                    },
                    key.clone(),
                ) {
                    self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Return(at)
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
            self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin10 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin10Return(
        std::rc::Rc<
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
        >,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_literal_property_index_loop11(
    mut key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut value: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result {
    loop {
        return if at
            < ({
                let ml_reference_array251 = key.clone();
                ml_reference_array251.borrow().len() as f64
            }) {
            {
                let digit = match {
                    let ml_reference_array252 = key.clone();
                    let ml_reference_position252 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array252.borrow(),
                        self::ml_array::number_index(ml_reference_position252),
                    )
                } {
                    Some(ml_present) => ml_present,
                    None => f64::NAN,
                } - 48f64;
                if (!(self::ml_math::is_integer(digit)) || (digit < 0f64)) || (digit > 9f64) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Return(-1f64)
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
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result {
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Done,
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(f64),
}
