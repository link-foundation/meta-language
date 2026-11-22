// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
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
use super::source_ml_source3_record_order;
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
        return if (source_ml_source1_temporary_index29
            < ({
                let ml_reference_array2543 = source_ml_source1_temporary_values29.clone();
                let ml_reference_projection2543 =
                    self::ml::Big::from_u128(ml_reference_array2543.borrow().len() as u128);
                ml_reference_projection2543
            })) {
            {
                let property = {
                    let ml_reference_array2544 = source_ml_source1_temporary_values29.clone();
                    let ml_reference_position2544 = source_ml_source1_temporary_index29.clone();
                    let ml_reference_projection2544 = self::ml_array::at(
                        &*ml_reference_array2544.borrow(),
                        ml_reference_position2544.to_index(),
                    );
                    ml_reference_projection2544
                };
                match property.clone() {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                        shared_constructor_field1,
                        shared_constructor_field2,
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        {
                            let source_ml_source1_temporary_plan_java_script_array_property_definition_ignored31 = {
                                let ml_method_owner2545 = properties.clone();
                                let ml_method_argument2545_0 =
                                    if self::test_java_script_array_length_key(
                                        shared_constructor_field1.clone(),
                                    ) {
                                        {
                                            let source_ml_source2_nominal_field39122 =
                                                shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field39124 =
                                                    shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_override39126 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            {
                                                                let source_ml_source2_spread39107 = {
                                                                    let ml_reference_value2546 =
                                                                        shared_constructor_field2
                                                                            .borrow()
                                                                            .clone();
                                                                    ml_reference_value2546
                                                                };
                                                                {
                                                                    let source_ml_source2_spread_field39108 = ((source_ml_source2_spread39107.clone()).0).0;
                                                                    {
                                                                        let source_ml_source2_spread_field39109 = ((source_ml_source2_spread39107.clone()).0).1;
                                                                        {
                                                                            let source_ml_source2_spread_field39110 = ((source_ml_source2_spread39107.clone()).0).2;
                                                                            {
                                                                                let source_ml_source2_spread_field39111 = ((source_ml_source2_spread39107.clone()).0).3;
                                                                                {
                                                                                    let source_ml_source2_spread_field39112 = ((source_ml_source2_spread39107.clone()).0).4;
                                                                                    {
                                                                                        let source_ml_source2_spread_field39113 = ((source_ml_source2_spread39107.clone()).0).5;
                                                                                        {
                                                                                            let source_ml_source2_spread_field39114 = ((source_ml_source2_spread39107.clone()).0).6;
                                                                                            {
                                                                                                let source_ml_source2_spread_field39115 = ((source_ml_source2_spread39107.clone()).0).7;
                                                                                                {
                                                                                                    let source_ml_source2_spread_field39116 = ((source_ml_source2_spread39107.clone()).0).8;
                                                                                                    {
                                                                                                        let source_ml_source2_spread_field39117 = ((source_ml_source2_spread39107.clone()).0).9;
                                                                                                        {
                                                                                                            let source_ml_source2_spread_field39118 = ((source_ml_source2_spread39107.clone()).0).10;
                                                                                                            {
                                                                                                                let source_ml_source2_spread_field39119 = ((source_ml_source2_spread39107.clone()).0).11;
                                                                                                                {
                                                                                                                    let source_ml_source2_field39120 = self::JavaScriptSemanticValue::Number((index + 1f64));
                                                                                                                    {
                                                                                                                        let source_ml_source3_record_field481 = source_ml_source2_spread_field39108;
                                                                                                                        {
                                                                                                                            let source_ml_source3_record_field482 = source_ml_source2_spread_field39109;
                                                                                                                            {
                                                                                                                                let source_ml_source3_record_field483 = source_ml_source2_spread_field39110;
                                                                                                                                {
                                                                                                                                    let source_ml_source3_record_field484 = source_ml_source2_spread_field39111;
                                                                                                                                    {
                                                                                                                                        let source_ml_source3_record_field485 = source_ml_source2_spread_field39112.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source3_record_field486 = source_ml_source2_spread_field39113;
                                                                                                                                            {
                                                                                                                                                let source_ml_source3_record_field487 = source_ml_source2_spread_field39114.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source3_record_field488 = source_ml_source2_spread_field39115;
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source3_record_field489 = source_ml_source2_field39120.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source3_record_field490 = source_ml_source2_spread_field39117;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source3_record_field491 = source_ml_source2_spread_field39118;
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source3_record_field492 = source_ml_source2_spread_field39119;
                                                                                                                                                                    ((source_ml_source3_record_field481, source_ml_source3_record_field482, source_ml_source3_record_field483, source_ml_source3_record_field484, source_ml_source3_record_field485.clone(), source_ml_source3_record_field486, source_ml_source3_record_field487.clone(), source_ml_source3_record_field488, source_ml_source3_record_field489.clone(), source_ml_source3_record_field490, source_ml_source3_record_field491, source_ml_source3_record_field492), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], self::source_ml_source3_record_order::merge_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![self::source_ml_source3_record_order::rebase_java_script_record_property_order(vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], vec![vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64], vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![103f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64], vec![115f64, 101f64, 116f64, 116f64, 101f64, 114f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![118f64, 97f64, 108f64, 117f64, 101f64], vec![118f64, 97f64, 108f64, 117f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64], vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64, 80f64, 114f64, 101f64, 115f64, 101f64, 110f64, 116f64]], (source_ml_source2_spread39107.clone()).1.clone()), vec![8f64]])))
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
                                                                }
                                                            },
                                                        ));
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source2_nominal_field39122.clone()), source_ml_source2_nominal_override39126.clone())
                                                }
                                            }
                                        }
                                    } else {
                                        property.clone()
                                    };
                                let ml_inserted = vec![ml_method_argument2545_0];
                                let mut ml_method_values2545 = ml_method_owner2545.borrow_mut();
                                let ml_length = ml_method_values2545
                                    .len()
                                    .checked_add(ml_inserted.len())
                                    .filter(|length| *length <= 4294967295usize)
                                    .expect("Invalid array length");
                                ml_method_values2545.extend(ml_inserted);
                                ml_length as f64
                            };
                            {
                                let source_ml_source1_temporary_index29_2 =
                                    source_ml_source1_temporary_index29
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index29 =
                                        source_ml_source1_temporary_index29_2.clone();
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
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
pub enum MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33 {
    MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33Next,
    MlSource1TemporaryNormalizeJavaScriptCollectionKeyJoin33Return(
        Box<self::JavaScriptSemanticValue>,
    ),
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
        return if (index < length) {
            {
                let x = self::read_java_script_semantic_text_unit(left.clone(), index);
                {
                    let y = self::read_java_script_semantic_text_unit(right.clone(), index);
                    if (x != y) {
                        self::MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Result::MlSource1TemporaryCompareJavaScriptCollectionKeysLoop35Return(if (x < y) {
                            (-1f64)
                        } else {
                            1f64
                        })
                    } else {
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
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
        return if ((left < middle) || (right < end)) {
            {
                let a = {
                    let ml_reference_array2547 = source.clone();
                    let ml_reference_position2547 = left;
                    let ml_reference_projection2547 = self::ml_array::optional_at(
                        &*ml_reference_array2547.borrow(),
                        self::ml_array::number_index(ml_reference_position2547),
                    );
                    ml_reference_projection2547
                };
                {
                    let b = {
                        let ml_reference_array2548 = source.clone();
                        let ml_reference_position2548 = right;
                        let ml_reference_projection2548 = self::ml_array::optional_at(
                            &*ml_reference_array2548.borrow(),
                            self::ml_array::number_index(ml_reference_position2548),
                        );
                        ml_reference_projection2548
                    };
                    {
                        let take_left = (right >= end);
                        {
                            let take_left_3 = if (!take_left && (left < middle)) {
                                if !(match (a.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    if !(match (b.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        {
                                            let take_left_2 =
                                                ((self::compare_java_script_collection_keys(
                                                    a.clone().expect("a proven present value"),
                                                    b.clone().expect("a proven present value"),
                                                )) <= (0f64));
                                            take_left_2
                                        }
                                    } else {
                                        take_left
                                    }
                                } else {
                                    take_left
                                }
                            } else {
                                take_left
                            };
                            if take_left_3 {
                                {
                                    let source_ml_source2_s39131 = if !(match (
                                        a.clone(),
                                        None::<()>,
                                    ) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        {
                                            let source_ml_source1_temporary_order_java_script_collection_keys_ignored40 = {
                                                let ml_method_owner2549 = target.clone();
                                                let ml_method_argument2549_0 =
                                                    a.clone().expect("a proven present value");
                                                let ml_inserted = vec![ml_method_argument2549_0];
                                                let mut ml_method_values2549 =
                                                    ml_method_owner2549.borrow_mut();
                                                let ml_length = ml_method_values2549
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4294967295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values2549.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            self::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39Next
                                        }
                                    } else {
                                        self::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39Next
                                    };
                                    match source_ml_source2_s39131.clone() {
                                        self::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin39Next => {
                                            {
                                                let left_2 = (left + 1f64);
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
                                    let source_ml_source2_s39132 = if !(match (
                                        b.clone(),
                                        None::<()>,
                                    ) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        {
                                            let source_ml_source1_temporary_order_java_script_collection_keys_ignored42 = {
                                                let ml_method_owner2550 = target.clone();
                                                let ml_method_argument2550_0 =
                                                    b.clone().expect("a proven present value");
                                                let ml_inserted = vec![ml_method_argument2550_0];
                                                let mut ml_method_values2550 =
                                                    ml_method_owner2550.borrow_mut();
                                                let ml_length = ml_method_values2550
                                                    .len()
                                                    .checked_add(ml_inserted.len())
                                                    .filter(|length| *length <= 4294967295usize)
                                                    .expect("Invalid array length");
                                                ml_method_values2550.extend(ml_inserted);
                                                ml_length as f64
                                            };
                                            self::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41Next
                                        }
                                    } else {
                                        self::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41Next
                                    };
                                    match source_ml_source2_s39132.clone() {
                                        self::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41::MlSource1TemporaryOrderJavaScriptCollectionKeysJoin41Next => {
                                            {
                                                let right_2 = (right + 1f64);
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
        return if (start
            < ({
                let ml_reference_array2551 = source.clone();
                let ml_reference_projection2551 = ml_reference_array2551.borrow().len() as f64;
                ml_reference_projection2551
            })) {
            {
                let middle = self::ml_math::min((start + width), {
                    let ml_reference_array2552 = source.clone();
                    let ml_reference_projection2552 = ml_reference_array2552.borrow().len() as f64;
                    ml_reference_projection2552
                });
                {
                    let end = self::ml_math::min((start + (width * 2f64)), {
                        let ml_reference_array2553 = source.clone();
                        let ml_reference_projection2553 =
                            ml_reference_array2553.borrow().len() as f64;
                        ml_reference_projection2553
                    });
                    {
                        let left = start;
                        {
                            let source_ml_source2_s39133 = self::source_ml_source1_temporary_order_java_script_collection_keys_loop38(source.clone(), target.clone(), middle, end, left, middle);
                            match source_ml_source2_s39133.clone() {
                                self::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop38Result::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop38Done => {
                                    {
                                        let start_2 = (start + (width * 2f64));
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
        return if (width
            < ({
                let ml_reference_array2554 = source.clone();
                let ml_reference_projection2554 = ml_reference_array2554.borrow().len() as f64;
                ml_reference_projection2554
            })) {
            {
                let target = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                    self::JavaScriptSemanticValue,
                >::new()));
                {
                    let start = 0f64;
                    {
                        let source_ml_source2_s39134 = self::source_ml_source1_temporary_order_java_script_collection_keys_loop37(source.clone(), width, target.clone(), start);
                        match source_ml_source2_s39134.clone() {
                            self::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop37Result::MlSource1TemporaryOrderJavaScriptCollectionKeysLoop37Done => {
                                {
                                    let width_2 = (width * 2f64);
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
        return if (source_ml_source1_temporary_index43
            < ({
                let ml_reference_array2555 = source_ml_source1_temporary_values43.clone();
                let ml_reference_projection2555 =
                    self::ml::Big::from_u128(ml_reference_array2555.borrow().len() as u128);
                ml_reference_projection2555
            })) {
            {
                let hook = {
                    let ml_reference_array2556 = source_ml_source1_temporary_values43.clone();
                    let ml_reference_position2556 = source_ml_source1_temporary_index43.clone();
                    let ml_reference_projection2556 = self::ml_array::at(
                        &*ml_reference_array2556.borrow(),
                        ml_reference_position2556.to_index(),
                    );
                    ml_reference_projection2556
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
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(error.clone())
                        } else {
                            if (match hook.clone().expect("a proven present value") {
                                self::JavaScriptSemanticValue::Object(_, _) => false,
                                _ => true,
                            } || !(((match hook.clone().expect("a proven present value") {
                                self::JavaScriptSemanticValue::Object(
                                    _,
                                    source_ml_source2_data_property_value39139,
                                ) => Some((source_ml_source2_data_property_value39139,)),
                                _ => None::<(bool,)>,
                            })
                            .clone()
                            .expect("a proven present value"))
                            .0))
                            {
                                self::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Result::MlSource1TemporaryReadJavaScriptOpaqueObjectErrorLoop45Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 97, 99, 99, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 97, 108, 108, 32, 104, 111, 111, 107, 115]))
                            } else {
                                {
                                    let source_ml_source1_temporary_index43_2 =
                                        source_ml_source1_temporary_index43
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index43 =
                                            source_ml_source1_temporary_index43_2.clone();
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
        return if (source_ml_source1_temporary_index46
            < ({
                let ml_reference_array2557 = source_ml_source1_temporary_values46.clone();
                let ml_reference_projection2557 =
                    self::ml::Big::from_u128(ml_reference_array2557.borrow().len() as u128);
                ml_reference_projection2557
            })) {
            {
                let value = {
                    let ml_reference_array2558 = source_ml_source1_temporary_values46.clone();
                    let ml_reference_position2558 = source_ml_source1_temporary_index46.clone();
                    let ml_reference_projection2558 = self::ml_array::at(
                        &*ml_reference_array2558.borrow(),
                        ml_reference_position2558.to_index(),
                    );
                    ml_reference_projection2558
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Result::MlSource1TemporaryReadJavaScriptClosureObjectErrorLoop49Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index46_2 =
                                source_ml_source1_temporary_index46
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index46 =
                                    source_ml_source1_temporary_index46_2.clone();
                                continue;
                            }
                        }
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
        return if (source_ml_source1_temporary_index50
            < ({
                let ml_reference_array2559 = source_ml_source1_temporary_values50.clone();
                let ml_reference_projection2559 =
                    self::ml::Big::from_u128(ml_reference_array2559.borrow().len() as u128);
                ml_reference_projection2559
            })) {
            {
                let value = {
                    let ml_reference_array2560 = source_ml_source1_temporary_values50.clone();
                    let ml_reference_position2560 = source_ml_source1_temporary_index50.clone();
                    let ml_reference_projection2560 = self::ml_array::at(
                        &*ml_reference_array2560.borrow(),
                        ml_reference_position2560.to_index(),
                    );
                    ml_reference_projection2560
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Result::MlSource1TemporaryReadJavaScriptBoundObjectErrorLoop53Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index50_2 =
                                source_ml_source1_temporary_index50
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index50 =
                                    source_ml_source1_temporary_index50_2.clone();
                                continue;
                            }
                        }
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
