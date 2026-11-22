// Generated from complete js/src/translation/javascript-arrays.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Result;
use super::compare_java_script_collection_keys;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_unit;
use super::test_java_script_array_length_key;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_java_script_array_property_definition_loop30(
    mut index: f64,
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut source_ml_temporary_values29: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_temporary_index29: self::ml::Big,
) -> self::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Result {
    loop {
        if source_ml_temporary_index29
            < ({
                let ml_reference_array225 = source_ml_temporary_values29.clone();
                self::ml::Big::from_u128(ml_reference_array225.borrow().len() as u128)
            })
        {
            {
                let property = {
                    let ml_reference_array226 = source_ml_temporary_values29.clone();
                    let ml_reference_position226 = source_ml_temporary_index29.clone();
                    self::ml_array::at(
                        &*ml_reference_array226.borrow(),
                        ml_reference_position226.to_index(),
                    )
                };
                {
                    let source_ml_temporary_plan_java_script_array_property_definition_ignored31 = {
                        let ml_method_owner227 = properties.clone();
                        let ml_method_argument227_0 = if self::test_java_script_array_length_key(
                            match property.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    source_ml_source1_data_property_value18192,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value18192 =
                                        *source_ml_source1_data_property_value18192;
                                    source_ml_source1_data_property_value18192.clone()
                                }
                            },
                        ) {
                            {
                                let source_ml_source1_nominal_field18209 = match property.clone() {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        source_ml_source1_data_property_value18210,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value18210 =
                                            *source_ml_source1_data_property_value18210;
                                        source_ml_source1_data_property_value18210.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_nominal_field18211 =
                                        match property.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                                _,
                                                source_ml_source1_data_property_value18212,
                                            ) => source_ml_source1_data_property_value18212.clone(),
                                        };
                                    {
                                        let source_ml_source1_nominal_override18213 =
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source1_spread18194 = {
                                                    (match property.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value18193) => {
                                                source_ml_source1_data_property_value18193.clone()
                                            }
                                        }).borrow().clone()
                                                };
                                                {
                                                    let source_ml_source1_spread_field18195 =
                                                        (source_ml_source1_spread18194.clone()).0;
                                                    {
                                                        let source_ml_source1_spread_field18196 =
                                                            (source_ml_source1_spread18194.clone())
                                                                .1;
                                                        {
                                                            let source_ml_source1_spread_field18197 =
                                                                (source_ml_source1_spread18194
                                                                    .clone())
                                                                .2;
                                                            {
                                                                let source_ml_source1_spread_field18198 =
                                                                    (source_ml_source1_spread18194
                                                                        .clone())
                                                                    .3;
                                                                {
                                                                    let source_ml_source1_spread_field18199 = (source_ml_source1_spread18194.clone()).4;
                                                                    {
                                                                        let source_ml_source1_spread_field18200 = (source_ml_source1_spread18194.clone()).5;
                                                                        {
                                                                            let source_ml_source1_spread_field18201 = (source_ml_source1_spread18194.clone()).6;
                                                                            {
                                                                                let source_ml_source1_spread_field18202 = (source_ml_source1_spread18194.clone()).7;
                                                                                {
                                                                                    let source_ml_source1_spread_field18203 = (source_ml_source1_spread18194.clone()).8;
                                                                                    {
                                                                                        let source_ml_source1_spread_field18204 = (source_ml_source1_spread18194.clone()).9;
                                                                                        {
                                                                                            let source_ml_source1_spread_field18205 = (source_ml_source1_spread18194.clone()).10;
                                                                                            {
                                                                                                let source_ml_source1_spread_field18206 = (source_ml_source1_spread18194.clone()).11;
                                                                                                {
                                                                                                    let source_ml_source1_field18207 = self::JavaScriptSemanticValue::Number(index + 1f64);
                                                                                                    (source_ml_source1_spread_field18195, source_ml_source1_spread_field18196, source_ml_source1_spread_field18197, source_ml_source1_spread_field18198, source_ml_source1_spread_field18199, source_ml_source1_spread_field18200, source_ml_source1_spread_field18201, source_ml_source1_spread_field18202, source_ml_source1_field18207, source_ml_source1_spread_field18204, source_ml_source1_spread_field18205, source_ml_source1_spread_field18206)
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
                                            }));
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                            Box::new(source_ml_source1_nominal_field18209),
                                            source_ml_source1_nominal_override18213,
                                        )
                                    }
                                }
                            }
                        } else {
                            property.clone()
                        };
                        let ml_inserted = vec![ml_method_argument227_0];
                        let mut ml_method_values227 = ml_method_owner227.borrow_mut();
                        let ml_length = ml_method_values227
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values227.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let source_ml_temporary_index29_2 =
                            source_ml_temporary_index29.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index29 = source_ml_temporary_index29_2;
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Result::MlTemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptCollectionKeyJoin32 {
    MlTemporaryTestJavaScriptCollectionKeyJoin32Next,
    MlTemporaryTestJavaScriptCollectionKeyJoin32Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareJavaScriptCollectionKeysLoop35Result {
    MlTemporaryCompareJavaScriptCollectionKeysLoop35Done,
    MlTemporaryCompareJavaScriptCollectionKeysLoop35Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_java_script_collection_keys_loop35(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut index: f64,
) -> self::MlTemporaryCompareJavaScriptCollectionKeysLoop35Result {
    loop {
        return if index < length {
            {
                let x = self::read_java_script_semantic_text_unit(left.clone(), index);
                {
                    let y = self::read_java_script_semantic_text_unit(right.clone(), index);
                    if x == y {
                        {
                            let index_2 = index + 1f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryCompareJavaScriptCollectionKeysLoop35Result::MlTemporaryCompareJavaScriptCollectionKeysLoop35Return(if x < y {
                            -1f64
                        } else {
                            1f64
                        })
                    }
                }
            }
        } else {
            self::MlTemporaryCompareJavaScriptCollectionKeysLoop35Result::MlTemporaryCompareJavaScriptCollectionKeysLoop35Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareJavaScriptCollectionKeysJoin34 {
    MlTemporaryCompareJavaScriptCollectionKeysJoin34Next,
    MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(f64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryOrderJavaScriptCollectionKeysLoop37Result {
    MlTemporaryOrderJavaScriptCollectionKeysLoop37Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryOrderJavaScriptCollectionKeysLoop38Result {
    MlTemporaryOrderJavaScriptCollectionKeysLoop38Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryOrderJavaScriptCollectionKeysJoin39 {
    MlTemporaryOrderJavaScriptCollectionKeysJoin39Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryOrderJavaScriptCollectionKeysJoin41 {
    MlTemporaryOrderJavaScriptCollectionKeysJoin41Next,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_collection_keys_loop38(
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut middle: f64,
    mut end: f64,
    mut left: f64,
    mut right: f64,
) -> self::MlTemporaryOrderJavaScriptCollectionKeysLoop38Result {
    loop {
        return if (left < middle) || (right < end) {
            {
                let a = {
                    let ml_reference_array229 = source.clone();
                    let ml_reference_position229 = left;
                    self::ml_array::optional_at(
                        &*ml_reference_array229.borrow(),
                        self::ml_array::number_index(ml_reference_position229),
                    )
                };
                {
                    let b = {
                        let ml_reference_array230 = source.clone();
                        let ml_reference_position230 = right;
                        self::ml_array::optional_at(
                            &*ml_reference_array230.borrow(),
                            self::ml_array::number_index(ml_reference_position230),
                        )
                    };
                    {
                        let take_left = right >= end;
                        {
                            let take_left_3 = if !take_left && (left < middle) {
                                if (match (a.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    take_left
                                } else if (match (b.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    take_left
                                } else {
                                    {
                                        (self::compare_java_script_collection_keys(
                                            a.clone().expect("a proven present value"),
                                            b.clone().expect("a proven present value"),
                                        )) <= 0f64
                                    }
                                }
                            } else {
                                take_left
                            };
                            if take_left_3 {
                                {
                                    let source_ml_source1_s18218 = if (match (a.clone(), None::<()>)
                                    {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        self::MlTemporaryOrderJavaScriptCollectionKeysJoin39::MlTemporaryOrderJavaScriptCollectionKeysJoin39Next
                                    } else {
                                        {
                                            let source_ml_temporary_order_java_script_collection_keys_ignored40 = {
                                                let ml_method_owner231 = target.clone();
                                                let ml_method_argument231_0 =
                                                    a.clone().expect("a proven present value");
                                                let ml_inserted = vec![ml_method_argument231_0];
                                                let mut ml_method_values231 =
                                                    ml_method_owner231.borrow_mut();
                                                let ml_length = ml_method_values231
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4_294_967_295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values231.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            self::MlTemporaryOrderJavaScriptCollectionKeysJoin39::MlTemporaryOrderJavaScriptCollectionKeysJoin39Next
                                        }
                                    };
                                    match source_ml_source1_s18218 {
                                        self::MlTemporaryOrderJavaScriptCollectionKeysJoin39::MlTemporaryOrderJavaScriptCollectionKeysJoin39Next => {
                                            {
                                                let left_2 = left + 1f64;
                                                {
                                                    left = left_2;
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let source_ml_source1_s18219 = if (match (b.clone(), None::<()>)
                                    {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        self::MlTemporaryOrderJavaScriptCollectionKeysJoin41::MlTemporaryOrderJavaScriptCollectionKeysJoin41Next
                                    } else {
                                        {
                                            let source_ml_temporary_order_java_script_collection_keys_ignored42 = {
                                                let ml_method_owner232 = target.clone();
                                                let ml_method_argument232_0 =
                                                    b.clone().expect("a proven present value");
                                                let ml_inserted = vec![ml_method_argument232_0];
                                                let mut ml_method_values232 =
                                                    ml_method_owner232.borrow_mut();
                                                let ml_length = ml_method_values232
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4_294_967_295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values232.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            self::MlTemporaryOrderJavaScriptCollectionKeysJoin41::MlTemporaryOrderJavaScriptCollectionKeysJoin41Next
                                        }
                                    };
                                    match source_ml_source1_s18219 {
                                        self::MlTemporaryOrderJavaScriptCollectionKeysJoin41::MlTemporaryOrderJavaScriptCollectionKeysJoin41Next => {
                                            {
                                                let right_2 = right + 1f64;
                                                {
                                                    right = right_2;
                                                    continue;
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
            self::MlTemporaryOrderJavaScriptCollectionKeysLoop38Result::MlTemporaryOrderJavaScriptCollectionKeysLoop38Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_collection_keys_loop37(
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut width: f64,
    mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut start: f64,
) -> self::MlTemporaryOrderJavaScriptCollectionKeysLoop37Result {
    loop {
        return if start
            < ({
                let ml_reference_array233 = source.clone();
                ml_reference_array233.borrow().len() as f64
            }) {
            {
                let middle = self::ml_math::min((start + width), {
                    let ml_reference_array234 = source.clone();
                    ml_reference_array234.borrow().len() as f64
                });
                {
                    let end = self::ml_math::min((start + (width * 2f64)), {
                        let ml_reference_array235 = source.clone();
                        ml_reference_array235.borrow().len() as f64
                    });
                    {
                        let left = start;
                        {
                            let source_ml_source1_s18220 =
                                self::source_ml_temporary_order_java_script_collection_keys_loop38(
                                    source.clone(),
                                    target.clone(),
                                    middle,
                                    end,
                                    left,
                                    middle,
                                );
                            match source_ml_source1_s18220 {
                                self::MlTemporaryOrderJavaScriptCollectionKeysLoop38Result::MlTemporaryOrderJavaScriptCollectionKeysLoop38Done => {
                                    {
                                        let start_2 = start + (width * 2f64);
                                        {
                                            start = start_2;
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryOrderJavaScriptCollectionKeysLoop37Result::MlTemporaryOrderJavaScriptCollectionKeysLoop37Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_collection_keys_loop36(
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut width: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    loop {
        return if width
            < ({
                let ml_reference_array236 = source.clone();
                ml_reference_array236.borrow().len() as f64
            }) {
            {
                let target = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                    self::JavaScriptSemanticValue,
                >::new()));
                {
                    let start = 0f64;
                    {
                        let source_ml_source1_s18221 =
                            self::source_ml_temporary_order_java_script_collection_keys_loop37(
                                source.clone(),
                                width,
                                target.clone(),
                                start,
                            );
                        match source_ml_source1_s18221 {
                            self::MlTemporaryOrderJavaScriptCollectionKeysLoop37Result::MlTemporaryOrderJavaScriptCollectionKeysLoop37Done => {
                                {
                                    let width_2 = width * 2f64;
                                    {
                                        (source, width) = (target.clone(), width_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            source.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44 {
    MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44Next,
    MlTemporaryReadJavaScriptOpaqueObjectErrorJoin44Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result {
    MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Done,
    MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_opaque_object_error_loop45(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values43: std::rc::Rc<
        std::cell::RefCell<Vec<Option<self::JavaScriptSemanticValue>>>,
    >,
    mut source_ml_temporary_index43: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result {
    loop {
        return if source_ml_temporary_index43
            < ({
                let ml_reference_array237 = source_ml_temporary_values43.clone();
                self::ml::Big::from_u128(ml_reference_array237.borrow().len() as u128)
            }) {
            {
                let hook = {
                    let ml_reference_array238 = source_ml_temporary_values43.clone();
                    let ml_reference_position238 = source_ml_temporary_index43.clone();
                    self::ml_array::at(
                        &*ml_reference_array238.borrow(),
                        ml_reference_position238.to_index(),
                    )
                };
                if match (hook.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 97, 99, 99, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 97, 108, 108, 32, 104, 111, 111, 107, 115]))
                } else {
                    {
                        let error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            hook.clone().expect("a proven present value"),
                        );
                        {
                            let ml_library_inequality_1 =
                                error != (self::ml_string::Text::from_units(vec![]));
                            if ml_library_inequality_1 {
                                self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(error.clone())
                            } else if match hook.clone().expect("a proven present value") {
                                self::JavaScriptSemanticValue::Object(_, _) => false,
                                _ => true,
                            } || !(match match hook
                                .clone()
                                .expect("a proven present value")
                            {
                                self::JavaScriptSemanticValue::Undefined => None::<bool>,
                                self::JavaScriptSemanticValue::Null => None::<bool>,
                                self::JavaScriptSemanticValue::Boolean(_) => None::<bool>,
                                self::JavaScriptSemanticValue::Number(_) => None::<bool>,
                                self::JavaScriptSemanticValue::Integer(_) => None::<bool>,
                                self::JavaScriptSemanticValue::Text(_) => None::<bool>,
                                self::JavaScriptSemanticValue::SourceText(_, _) => None::<bool>,
                                self::JavaScriptSemanticValue::Symbol(_) => None::<bool>,
                                self::JavaScriptSemanticValue::Object(
                                    _,
                                    source_ml_source1_data_property_value18233,
                                ) => Some(source_ml_source1_data_property_value18233),
                            } {
                                Some(ml_present) => ml_present,
                                None => false,
                            }) {
                                self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 97, 99, 99, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 97, 108, 108, 32, 104, 111, 111, 107, 115]))
                            } else {
                                {
                                    let source_ml_temporary_index43_2 = source_ml_temporary_index43
                                        .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index43 = source_ml_temporary_index43_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop45Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptClosureObjectErrorJoin47 {
    MlTemporaryReadJavaScriptClosureObjectErrorJoin47Next,
    MlTemporaryReadJavaScriptClosureObjectErrorJoin47Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptClosureObjectErrorLoop49Result {
    MlTemporaryReadJavaScriptClosureObjectErrorLoop49Done,
    MlTemporaryReadJavaScriptClosureObjectErrorLoop49Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_closure_object_error_loop49(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values46: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index46: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Result {
    loop {
        return if source_ml_temporary_index46
            < ({
                let ml_reference_array239 = source_ml_temporary_values46.clone();
                self::ml::Big::from_u128(ml_reference_array239.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array240 = source_ml_temporary_values46.clone();
                    let ml_reference_position240 = source_ml_temporary_index46.clone();
                    self::ml_array::at(
                        &*ml_reference_array240.borrow(),
                        ml_reference_position240.to_index(),
                    )
                };
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if error == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_temporary_index46_2 =
                                source_ml_temporary_index46.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index46 = source_ml_temporary_index46_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Return(error.clone())
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptBoundObjectErrorJoin51 {
    MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next,
    MlTemporaryReadJavaScriptBoundObjectErrorJoin51Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptBoundObjectErrorLoop53Result {
    MlTemporaryReadJavaScriptBoundObjectErrorLoop53Done,
    MlTemporaryReadJavaScriptBoundObjectErrorLoop53Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_bound_object_error_loop53(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values50: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index50: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Result {
    loop {
        return if source_ml_temporary_index50
            < ({
                let ml_reference_array241 = source_ml_temporary_values50.clone();
                self::ml::Big::from_u128(ml_reference_array241.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array242 = source_ml_temporary_values50.clone();
                    let ml_reference_position242 = source_ml_temporary_index50.clone();
                    self::ml_array::at(
                        &*ml_reference_array242.borrow(),
                        ml_reference_position242.to_index(),
                    )
                };
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if error == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_temporary_index50_2 =
                                source_ml_temporary_index50.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index50 = source_ml_temporary_index50_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Return(error.clone())
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54 {
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next,
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55 {
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next,
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58 {
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Next,
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result {
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Done,
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text),
}
