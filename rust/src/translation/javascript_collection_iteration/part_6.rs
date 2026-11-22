// Generated from complete js/src/translation/javascript-collection-iteration.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryCompareJavaScriptCollectionKeysJoin34;
use super::MlTemporaryCompareJavaScriptCollectionKeysLoop35Result;
use super::find_java_script_heap_directory_object;
use super::find_java_script_heap_directory_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_collection_key_rank;
use super::read_java_script_heap_directory_key_error;
use super::read_java_script_heap_directory_value_error;
use super::read_java_script_semantic_text_length;
use super::source_ml_temporary_compare_java_script_collection_keys_loop35;
use super::source_ml_temporary_order_java_script_collection_keys_loop36;
use super::test_java_script_collection_key;
use super::test_java_script_heap_directory_symbol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_java_script_collection_keys(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> f64 {
    {
        let a = self::read_java_script_collection_key_rank(left.clone());
        {
            let b = self::read_java_script_collection_key_rank(right.clone());
            {
                let ml_library_inequality_1 = a != b;
                if ml_library_inequality_1 {
                    if a < b { -1f64 } else { 1f64 }
                } else {
                    {
                        let source_ml_source1_s1443 = match left.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Next
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Next
                        }
                        self::JavaScriptSemanticValue::Boolean(left_value) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Boolean(value) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(if left_value == value {
                                        0f64
                                    } else if left_value {
                                            1f64
                                        } else {
                                            -1f64
                                        })
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Number(left_value_2) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Number(value_2) => {
                                    if self::test_java_script_collection_key(left.clone(), right.clone()) {
                                        self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                    } else if left_value_2.is_nan() {
                                            self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(1f64)
                                        } else if value_2.is_nan() {
                                                self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(-1f64)
                                            } else {
                                                self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(if left_value_2 < value_2 {
                                                    -1f64
                                                } else {
                                                    1f64
                                                })
                                            }
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(left_value_3) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Integer(value_3) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(if left_value_3 == value_3 {
                                        0f64
                                    } else if left_value_3 < value_3 {
                                            -1f64
                                        } else {
                                            1f64
                                        })
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Next
                        }
                        self::JavaScriptSemanticValue::SourceText(_, left_text) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Next
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Next
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Next
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Next
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Next
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Next
                                }
                                self::JavaScriptSemanticValue::SourceText(_, text) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(if left_text == text {
                                        0f64
                                    } else if <self::ml_string::Text as std::cmp::PartialOrd>::lt(&left_text, &text) {
                                            -1f64
                                        } else {
                                            1f64
                                        })
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Next
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Next
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(left_identity) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(identity) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(if left_identity == identity {
                                        0f64
                                    } else if left_identity < identity {
                                            -1f64
                                        } else {
                                            1f64
                                        })
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Object(left_identity_2, _) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(if left_identity_2 == identity_2 {
                                        0f64
                                    } else if left_identity_2 < identity_2 {
                                            -1f64
                                        } else {
                                            1f64
                                        })
                                }
                            }
                        }
                    };
                        match source_ml_source1_s1443 {
                        self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Next => {
                            if a == 5f64 {
                                {
                                    let first = self::read_java_script_semantic_text_length(left.clone());
                                    {
                                        let second = self::read_java_script_semantic_text_length(right.clone());
                                        {
                                            let length = self::ml_math::min(first, second);
                                            {
                                                let index = 0f64;
                                                {
                                                    let source_ml_source1_s1495 = self::source_ml_temporary_compare_java_script_collection_keys_loop35(left.clone(), right.clone(), length, index);
                                                    match source_ml_source1_s1495 {
                                                        self::MlTemporaryCompareJavaScriptCollectionKeysLoop35Result::MlTemporaryCompareJavaScriptCollectionKeysLoop35Done => {
                                                            if first == second {
                                                                0f64
                                                            } else if first < second {
                                                                    -1f64
                                                                } else {
                                                                    1f64
                                                                }
                                                        }
                                                        self::MlTemporaryCompareJavaScriptCollectionKeysLoop35Result::MlTemporaryCompareJavaScriptCollectionKeysLoop35Return(source_ml_temporary_result35) => {
                                                            source_ml_temporary_result35
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                0f64
                            }
                        }
                        self::MlTemporaryCompareJavaScriptCollectionKeysJoin34::MlTemporaryCompareJavaScriptCollectionKeysJoin34Return(source_ml_temporary_result35_2) => {
                            source_ml_temporary_result35_2
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
pub fn order_java_script_collection_keys(
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    {
        let source = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(
            &{ values.borrow().clone() },
            0f64,
            f64::INFINITY,
        )));
        {
            let width = 1f64;
            { self::source_ml_temporary_order_java_script_collection_keys_loop36(source, width) }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_ordinary_object(
    objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>,
    identity: self::ml::Big,
) -> f64 {
    self::find_java_script_heap_directory_object(objects.clone(), identity.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_ordinary_symbol(
    symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    identity: self::ml::Big,
) -> bool {
    self::test_java_script_heap_directory_symbol(symbols.clone(), identity.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_value_error(
    heap: self::JavaScriptOrdinaryHeap,
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    self::read_java_script_heap_directory_value_error(heap.clone(), value.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_key_error(
    heap: self::JavaScriptOrdinaryHeap,
    key: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    self::read_java_script_heap_directory_key_error(heap.clone(), key.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_ordinary_property(
    properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    self::find_java_script_heap_directory_property(properties.clone(), key.clone())
}
