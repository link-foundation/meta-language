// Generated from complete js/src/translation/javascript-collection-rules.js. Do not edit by hand.
use super::JavaScriptSemanticValue;
use super::compare_java_script_collection_keys;
use super::ml_array;
use super::ml_math;
use super::read_java_script_semantic_text_unit;

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareJavaScriptCollectionKeysLoop9Result {
    MlTemporaryCompareJavaScriptCollectionKeysLoop9Done,
    MlTemporaryCompareJavaScriptCollectionKeysLoop9Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_java_script_collection_keys_loop9(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut index: f64,
) -> self::MlTemporaryCompareJavaScriptCollectionKeysLoop9Result {
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
                        self::MlTemporaryCompareJavaScriptCollectionKeysLoop9Result::MlTemporaryCompareJavaScriptCollectionKeysLoop9Return(if x < y {
                            -1f64
                        } else {
                            1f64
                        })
                    }
                }
            }
        } else {
            self::MlTemporaryCompareJavaScriptCollectionKeysLoop9Result::MlTemporaryCompareJavaScriptCollectionKeysLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareJavaScriptCollectionKeysJoin8 {
    MlTemporaryCompareJavaScriptCollectionKeysJoin8Next,
    MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_collection_keys_loop12(
    mut source: Vec<self::JavaScriptSemanticValue>,
    mut target: Vec<self::JavaScriptSemanticValue>,
    mut middle: f64,
    mut end: f64,
    mut left: f64,
    mut right: f64,
) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        if (left < middle) || (right < end) {
            {
                let a = self::ml_array::optional_at(&source, self::ml_array::number_index(left));
                {
                    let b =
                        self::ml_array::optional_at(&source, self::ml_array::number_index(right));
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
                                    let target_3 = if (match (a.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        target
                                    } else {
                                        {
                                            {
                                                let (ml_append_right, ml_append_left) = (
                                                    vec![
                                                        a.clone().expect("a proven present value"),
                                                    ],
                                                    target,
                                                );
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            }
                                        }
                                    };
                                    {
                                        let left_2 = left + 1f64;
                                        {
                                            (target, left) = (target_3, left_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                            {
                                {
                                    let target_5 = if (match (b.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    }) {
                                        target
                                    } else {
                                        {
                                            {
                                                let (ml_append_right, ml_append_left) = (
                                                    vec![
                                                        b.clone().expect("a proven present value"),
                                                    ],
                                                    target,
                                                );
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            }
                                        }
                                    };
                                    {
                                        let right_2 = right + 1f64;
                                        {
                                            (target, right) = (target_5, right_2);
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
        return { target };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_collection_keys_loop11(
    mut source: Vec<self::JavaScriptSemanticValue>,
    mut width: f64,
    mut target: Vec<self::JavaScriptSemanticValue>,
    mut start: f64,
) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        if start < (source.len() as f64) {
            {
                let middle = self::ml_math::min((start + width), (source.len() as f64));
                {
                    let end = self::ml_math::min((start + (width * 2f64)), (source.len() as f64));
                    {
                        let left = start;
                        {
                            let right = middle;
                            {
                                let target_2 = self::source_ml_temporary_order_java_script_collection_keys_loop12(source.clone(), target, middle, end, left, right);
                                {
                                    let start_2 = start + (width * 2f64);
                                    {
                                        (target, start) = (target_2, start_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { target };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_collection_keys_loop10(
    mut source: Vec<self::JavaScriptSemanticValue>,
    mut width: f64,
) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        if width < (source.len() as f64) {
            {
                let target = Vec::<self::JavaScriptSemanticValue>::new();
                {
                    let start = 0f64;
                    {
                        let target_2 =
                            self::source_ml_temporary_order_java_script_collection_keys_loop11(
                                source, width, target, start,
                            );
                        {
                            let source_2 = target_2;
                            {
                                let width_2 = width * 2f64;
                                {
                                    (source, width) = (source_2, width_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { source };
    }
}
