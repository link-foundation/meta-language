// Generated from complete js/src/translation/javascript-descriptor-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_temporary_set_java_script_ordinary_prototype_loop21;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn set_java_script_ordinary_prototype(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
) -> (String, self::JavaScriptOrdinaryHeap, bool) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if !(heap_error).is_empty() {
            {
                let source_ml_source1_field410 = heap.clone();
                {
                    let source_ml_source1_field411 = false;
                    {
                        let source_ml_source1_field412 = heap_error.clone();
                        (
                            source_ml_source1_field412,
                            source_ml_source1_field410,
                            source_ml_source1_field411,
                        )
                    }
                }
            }
        } else if match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => true,
            self::JavaScriptSemanticValue::Null => false,
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f413) => true,
            self::JavaScriptSemanticValue::Number(source_ml_source1_f414) => true,
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f415) => true,
            self::JavaScriptSemanticValue::Text(source_ml_source1_f416) => true,
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f417) => true,
            self::JavaScriptSemanticValue::Object(
                source_ml_source1_f418,
                source_ml_source1_f419,
            ) => true,
        } && match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => true,
            self::JavaScriptSemanticValue::Null => true,
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f420) => true,
            self::JavaScriptSemanticValue::Number(source_ml_source1_f421) => true,
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f422) => true,
            self::JavaScriptSemanticValue::Text(source_ml_source1_f423) => true,
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f424) => true,
            self::JavaScriptSemanticValue::Object(
                source_ml_source1_f425,
                source_ml_source1_f426,
            ) => false,
        } {
            {
                let source_ml_source1_field427 = heap.clone();
                {
                    let source_ml_source1_field428 = false;
                    {
                        let source_ml_source1_field429 =
                            String::from("an ordinary prototype must be null or a declared Object");
                        (
                            source_ml_source1_field429,
                            source_ml_source1_field427,
                            source_ml_source1_field428,
                        )
                    }
                }
            }
        } else {
            {
                let error =
                    self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
                if !(error).is_empty() {
                    {
                        let source_ml_source1_field430 = heap.clone();
                        {
                            let source_ml_source1_field431 = false;
                            {
                                let source_ml_source1_field432 = error.clone();
                                (
                                    source_ml_source1_field432,
                                    source_ml_source1_field430,
                                    source_ml_source1_field431,
                                )
                            }
                        }
                    }
                } else {
                    {
                        let index = self::find_java_script_ordinary_object(
                            match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    source_ml_source1_data_property_value433,
                                    _,
                                ) => source_ml_source1_data_property_value433,
                            },
                            identity.clone(),
                        );
                        {
                            let object = self::ml_array::optional_at(
                                (match &heap {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        source_ml_source1_data_property_value434,
                                        _,
                                    ) => source_ml_source1_data_property_value434.as_slice(),
                                }),
                                self::ml_array::number_index(index),
                            );
                            if match (object.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                {
                                    let source_ml_source1_field435 = heap.clone();
                                    {
                                        let source_ml_source1_field436 = false;
                                        {
                                            let source_ml_source1_field437 = String::from(
                                                "SetPrototypeOf needs its actual Object identity",
                                            );
                                            (
                                                source_ml_source1_field437,
                                                source_ml_source1_field435,
                                                source_ml_source1_field436,
                                            )
                                        }
                                    }
                                }
                            } else if self::test_java_script_semantic_same_value(
                                match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        source_ml_source1_data_property_value438,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value438 =
                                            *source_ml_source1_data_property_value438;
                                        source_ml_source1_data_property_value438.clone()
                                    }
                                },
                                prototype.clone(),
                            ) {
                                {
                                    let source_ml_source1_field439 = heap.clone();
                                    {
                                        let source_ml_source1_field440 = true;
                                        {
                                            let source_ml_source1_field441 = String::new();
                                            (
                                                source_ml_source1_field441,
                                                source_ml_source1_field439,
                                                source_ml_source1_field440,
                                            )
                                        }
                                    }
                                }
                            } else if (match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    source_ml_source1_data_property_value442,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value442,
                            }) {
                                {
                                    let next = prototype.clone();
                                    {
                                        let depth = 0f64;
                                        {
                                            let source_ml_source1_s446 = self::source_ml_temporary_set_java_script_ordinary_prototype_loop21(heap.clone(), identity.clone(), next, depth);
                                            match source_ml_source1_s446 {
                                                            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done => {
                                                                {
                                                                    let replacement = self::JavaScriptOrdinaryObject::OrdinaryObject(match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value447, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value447.clone()
                                                                        }
                                                                    }, Box::new(prototype.clone()), match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source1_data_property_value448, _, _) => {
                                                                            source_ml_source1_data_property_value448
                                                                        }
                                                                    }, match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, source_ml_source1_data_property_value449, _) => {
                                                                            source_ml_source1_data_property_value449
                                                                        }
                                                                    }, match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value450) => {
                                                                            source_ml_source1_data_property_value450
                                                                        }
                                                                    });
                                                                    {
                                                                        let source_ml_source1_field451 = self::replace_java_script_ordinary_object(heap.clone(), replacement);
                                                                        {
                                                                            let source_ml_source1_field452 = true;
                                                                            {
                                                                                let source_ml_source1_field453 = String::new();
                                                                                (source_ml_source1_field453, source_ml_source1_field451, source_ml_source1_field452)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return(source_ml_temporary_result21) => {
                                                                source_ml_temporary_result21.clone()
                                                            }
                                                        }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let source_ml_source1_field443 = heap.clone();
                                    {
                                        let source_ml_source1_field444 = false;
                                        {
                                            let source_ml_source1_field445 = String::new();
                                            (
                                                source_ml_source1_field445,
                                                source_ml_source1_field443,
                                                source_ml_source1_field444,
                                            )
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
pub fn create_java_script_ordinary_set_descriptor(
    value: self::JavaScriptSemanticValue,
    creation: bool,
) -> (
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
) {
    {
        let source_ml_source1_field454 = creation;
        {
            let source_ml_source1_field455 = creation;
            {
                let source_ml_source1_field456 = creation;
                {
                    let source_ml_source1_field457 = creation;
                    {
                        let source_ml_source1_field458 = true;
                        {
                            let source_ml_source1_field459 = value.clone();
                            {
                                let source_ml_source1_field460 = creation;
                                {
                                    let source_ml_source1_field461 = creation;
                                    {
                                        let source_ml_source1_field462 = false;
                                        {
                                            let source_ml_source1_field463 =
                                                self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field464 = false;
                                                {
                                                    let source_ml_source1_field465 =
                                                        self::JavaScriptSemanticValue::Undefined;
                                                    (
                                                        source_ml_source1_field455,
                                                        source_ml_source1_field454,
                                                        source_ml_source1_field457,
                                                        source_ml_source1_field456,
                                                        source_ml_source1_field463,
                                                        source_ml_source1_field462,
                                                        source_ml_source1_field465,
                                                        source_ml_source1_field464,
                                                        source_ml_source1_field459,
                                                        source_ml_source1_field458,
                                                        source_ml_source1_field461,
                                                        source_ml_source1_field460,
                                                    )
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
