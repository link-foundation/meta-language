// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Result;
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
pub fn source_ml_source1_temporary_plan_java_script_array_property_definition_loop30(
    mut index: f64,
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut source_ml_source1_temporary_values29: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_source1_temporary_index29: self::ml::Big,
) -> self::MlSource1TemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Result {
    loop {
        if source_ml_source1_temporary_index29
            < ({
                let ml_reference_array253 = source_ml_source1_temporary_values29.clone();
                self::ml::Big::from_u128(ml_reference_array253.borrow().len() as u128)
            })
        {
            {
                let property = {
                    let ml_reference_array254 = source_ml_source1_temporary_values29.clone();
                    let ml_reference_position254 = source_ml_source1_temporary_index29.clone();
                    self::ml_array::at(
                        &*ml_reference_array254.borrow(),
                        ml_reference_position254.to_index(),
                    )
                };
                {
                    let source_ml_source1_temporary_plan_java_script_array_property_definition_ignored31 = {
                        let ml_method_owner255 = properties.clone();
                        let ml_method_argument255_0 = if self::test_java_script_array_length_key(
                            match property.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    source_ml_source2_data_property_value17295,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value17295 =
                                        *source_ml_source2_data_property_value17295;
                                    source_ml_source2_data_property_value17295.clone()
                                }
                            },
                        ) {
                            {
                                let source_ml_source2_nominal_field17312 = match property.clone() {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        source_ml_source2_data_property_value17313,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value17313 =
                                            *source_ml_source2_data_property_value17313;
                                        source_ml_source2_data_property_value17313.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field17314 =
                                        match property.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                                _,
                                                source_ml_source2_data_property_value17315,
                                            ) => source_ml_source2_data_property_value17315.clone(),
                                        };
                                    {
                                        let source_ml_source2_nominal_override17316 =
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source2_spread17297 = {
                                                    (match property.clone() {
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value17296) => {
                                                source_ml_source2_data_property_value17296.clone()
                                            }
                                        }).borrow().clone()
                                                };
                                                {
                                                    let source_ml_source2_spread_field17298 =
                                                        (source_ml_source2_spread17297.clone()).0;
                                                    {
                                                        let source_ml_source2_spread_field17299 =
                                                            (source_ml_source2_spread17297.clone())
                                                                .1;
                                                        {
                                                            let source_ml_source2_spread_field17300 =
                                                                (source_ml_source2_spread17297
                                                                    .clone())
                                                                .2;
                                                            {
                                                                let source_ml_source2_spread_field17301 =
                                                                    (source_ml_source2_spread17297
                                                                        .clone())
                                                                    .3;
                                                                {
                                                                    let source_ml_source2_spread_field17302 = (source_ml_source2_spread17297.clone()).4;
                                                                    {
                                                                        let source_ml_source2_spread_field17303 = (source_ml_source2_spread17297.clone()).5;
                                                                        {
                                                                            let source_ml_source2_spread_field17304 = (source_ml_source2_spread17297.clone()).6;
                                                                            {
                                                                                let source_ml_source2_spread_field17305 = (source_ml_source2_spread17297.clone()).7;
                                                                                {
                                                                                    let source_ml_source2_spread_field17306 = (source_ml_source2_spread17297.clone()).8;
                                                                                    {
                                                                                        let source_ml_source2_spread_field17307 = (source_ml_source2_spread17297.clone()).9;
                                                                                        {
                                                                                            let source_ml_source2_spread_field17308 = (source_ml_source2_spread17297.clone()).10;
                                                                                            {
                                                                                                let source_ml_source2_spread_field17309 = (source_ml_source2_spread17297.clone()).11;
                                                                                                {
                                                                                                    let source_ml_source2_field17310 = self::JavaScriptSemanticValue::Number(index + 1f64);
                                                                                                    (source_ml_source2_spread_field17298, source_ml_source2_spread_field17299, source_ml_source2_spread_field17300, source_ml_source2_spread_field17301, source_ml_source2_spread_field17302, source_ml_source2_spread_field17303, source_ml_source2_spread_field17304, source_ml_source2_spread_field17305, source_ml_source2_field17310, source_ml_source2_spread_field17307, source_ml_source2_spread_field17308, source_ml_source2_spread_field17309)
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
                                            Box::new(source_ml_source2_nominal_field17312),
                                            source_ml_source2_nominal_override17316,
                                        )
                                    }
                                }
                            }
                        } else {
                            property.clone()
                        };
                        let ml_inserted = vec![ml_method_argument255_0];
                        let mut ml_method_values255 = ml_method_owner255.borrow_mut();
                        let ml_length = ml_method_values255
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4_294_967_295usize)
                            .expect("Invalid array length");
                        ml_method_values255.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let source_ml_source1_temporary_index29_2 =
                            source_ml_source1_temporary_index29.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index29 =
                                source_ml_source1_temporary_index29_2;
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Result::MlSource1TemporaryPlanJavaScriptArrayPropertyDefinitionLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptCollectionKeyJoin32 {
    MlSource1TemporaryTestJavaScriptCollectionKeyJoin32Next,
    MlSource1TemporaryTestJavaScriptCollectionKeyJoin32Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Result {
    MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Done,
    MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compare_java_script_collection_keys_loop35(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut index: f64,
) -> self::MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Result {
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
                        self::MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Result::MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Return(if x < y {
                            -1f64
                        } else {
                            1f64
                        })
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Result::MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34 {
    MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Next,
    MlSource1TemporaryCompareJavaScriptCollectionKeysJoin34Return(f64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryOrderJavaScriptCollectionKeysLoop37Result {
    MlSource1TemporaryOrderJavaScriptCollectionKeysLoop37Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryOrderJavaScriptCollectionKeysLoop38Result {
    MlSource1TemporaryOrderJavaScriptCollectionKeysLoop38Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39 {
    MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41 {
    MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41Next,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_collection_keys_loop38(
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut middle: f64,
    mut end: f64,
    mut left: f64,
    mut right: f64,
) -> self::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop38Result {
    loop {
        return if (left < middle) || (right < end) {
            {
                let a = {
                    let ml_reference_array257 = source.clone();
                    let ml_reference_position257 = left;
                    self::ml_array::optional_at(
                        &*ml_reference_array257.borrow(),
                        self::ml_array::number_index(ml_reference_position257),
                    )
                };
                {
                    let b = {
                        let ml_reference_array258 = source.clone();
                        let ml_reference_position258 = right;
                        self::ml_array::optional_at(
                            &*ml_reference_array258.borrow(),
                            self::ml_array::number_index(ml_reference_position258),
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
                                    let source_ml_source2_s17321 = if (match (a.clone(), None::<()>)
                                    {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        self::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39Next
                                    } else {
                                        {
                                            let source_ml_source1_temporary_order_java_script_collection_keys_ignored40 = {
                                                let ml_method_owner259 = target.clone();
                                                let ml_method_argument259_0 =
                                                    a.clone().expect("a proven present value");
                                                let ml_inserted = vec![ml_method_argument259_0];
                                                let mut ml_method_values259 =
                                                    ml_method_owner259.borrow_mut();
                                                let ml_length = ml_method_values259
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4_294_967_295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values259.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            self::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39Next
                                        }
                                    };
                                    match source_ml_source2_s17321 {
                                        self::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39Next => {
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
                                    let source_ml_source2_s17322 = if (match (b.clone(), None::<()>)
                                    {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        self::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41Next
                                    } else {
                                        {
                                            let source_ml_source1_temporary_order_java_script_collection_keys_ignored42 = {
                                                let ml_method_owner260 = target.clone();
                                                let ml_method_argument260_0 =
                                                    b.clone().expect("a proven present value");
                                                let ml_inserted = vec![ml_method_argument260_0];
                                                let mut ml_method_values260 =
                                                    ml_method_owner260.borrow_mut();
                                                let ml_length = ml_method_values260
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4_294_967_295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values260.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            self::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41Next
                                        }
                                    };
                                    match source_ml_source2_s17322 {
                                        self::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41Next => {
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
            self::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop38Result::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop38Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_collection_keys_loop37(
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut width: f64,
    mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut start: f64,
) -> self::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop37Result {
    loop {
        return if start
            < ({
                let ml_reference_array261 = source.clone();
                ml_reference_array261.borrow().len() as f64
            }) {
            {
                let middle = self::ml_math::min((start + width), {
                    let ml_reference_array262 = source.clone();
                    ml_reference_array262.borrow().len() as f64
                });
                {
                    let end = self::ml_math::min((start + (width * 2f64)), {
                        let ml_reference_array263 = source.clone();
                        ml_reference_array263.borrow().len() as f64
                    });
                    {
                        let left = start;
                        {
                            let source_ml_source2_s17323 = self::source_ml_source1_temporary_order_java_script_collection_keys_loop38(source.clone(), target.clone(), middle, end, left, middle);
                            match source_ml_source2_s17323 {
                                self::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop38Result::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop38Done => {
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
            self::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop37Result::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop37Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_collection_keys_loop36(
    mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut width: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    loop {
        return if width
            < ({
                let ml_reference_array264 = source.clone();
                ml_reference_array264.borrow().len() as f64
            }) {
            {
                let target = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                    self::JavaScriptSemanticValue,
                >::new()));
                {
                    let start = 0f64;
                    {
                        let source_ml_source2_s17324 = self::source_ml_source1_temporary_order_java_script_collection_keys_loop37(source.clone(), width, target.clone(), start);
                        match source_ml_source2_s17324 {
                            self::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop37Result::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop37Done => {
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
pub enum MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44 {
    MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44Next,
    MlSource1TemporaryReadJavaScriptOpaqueObjectErrorJoin44Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Result {
    MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Done,
    MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_opaque_object_error_loop45(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values43: std::rc::Rc<
        std::cell::RefCell<Vec<Option<self::JavaScriptSemanticValue>>>,
    >,
    mut source_ml_source1_temporary_index43: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Result {
    loop {
        return if source_ml_source1_temporary_index43
            < ({
                let ml_reference_array265 = source_ml_source1_temporary_values43.clone();
                self::ml::Big::from_u128(ml_reference_array265.borrow().len() as u128)
            }) {
            {
                let hook = {
                    let ml_reference_array266 = source_ml_source1_temporary_values43.clone();
                    let ml_reference_position266 = source_ml_source1_temporary_index43.clone();
                    self::ml_array::at(
                        &*ml_reference_array266.borrow(),
                        ml_reference_position266.to_index(),
                    )
                };
                if match (hook.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 97, 99, 99, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 97, 108, 108, 32, 104, 111, 111, 107, 115]))
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
                                self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(error.clone())
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
                                    source_ml_source2_data_property_value17336,
                                ) => Some(source_ml_source2_data_property_value17336),
                            } {
                                Some(ml_present) => ml_present,
                                None => false,
                            }) {
                                self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 97, 99, 99, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 97, 108, 108, 32, 104, 111, 111, 107, 115]))
                            } else {
                                {
                                    let source_ml_source1_temporary_index43_2 =
                                        source_ml_source1_temporary_index43
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index43 =
                                            source_ml_source1_temporary_index43_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47 {
    MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47Next,
    MlSource1TemporaryReadJavaScriptClosureObjectErrorJoin47Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Result {
    MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Done,
    MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_closure_object_error_loop49(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values46: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index46: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Result {
    loop {
        return if source_ml_source1_temporary_index46
            < ({
                let ml_reference_array267 = source_ml_source1_temporary_values46.clone();
                self::ml::Big::from_u128(ml_reference_array267.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array268 = source_ml_source1_temporary_values46.clone();
                    let ml_reference_position268 = source_ml_source1_temporary_index46.clone();
                    self::ml_array::at(
                        &*ml_reference_array268.borrow(),
                        ml_reference_position268.to_index(),
                    )
                };
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if error == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_source1_temporary_index46_2 =
                                source_ml_source1_temporary_index46
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index46 =
                                    source_ml_source1_temporary_index46_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Result::MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Return(error.clone())
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Result::MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51 {
    MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51Next,
    MlSource1TemporaryReadJavaScriptBoundObjectErrorJoin51Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Result {
    MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Done,
    MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_bound_object_error_loop53(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values50: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index50: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Result {
    loop {
        return if source_ml_source1_temporary_index50
            < ({
                let ml_reference_array269 = source_ml_source1_temporary_values50.clone();
                self::ml::Big::from_u128(ml_reference_array269.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array270 = source_ml_source1_temporary_values50.clone();
                    let ml_reference_position270 = source_ml_source1_temporary_index50.clone();
                    self::ml_array::at(
                        &*ml_reference_array270.borrow(),
                        ml_reference_position270.to_index(),
                    )
                };
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if error == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_source1_temporary_index50_2 =
                                source_ml_source1_temporary_index50
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index50 =
                                    source_ml_source1_temporary_index50_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Result::MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Return(error.clone())
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Result::MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54 {
    MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next,
    MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55 {
    MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next,
    MlSource1TemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58 {
    MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Next,
    MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result {
    MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Done,
    MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text),
}
