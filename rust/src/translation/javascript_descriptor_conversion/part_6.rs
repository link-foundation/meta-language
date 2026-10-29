// Generated from complete js/src/translation/javascript-descriptor-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result;
use super::create_java_script_ordinary_set_descriptor;
use super::define_java_script_ordinary_property;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_read_java_script_ordinary_own_keys_loop23;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_ordinary_set(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    value: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
) -> (
    String,
    Vec<self::JavaScriptSemanticValue>,
    self::JavaScriptSemanticValue,
    String,
    self::JavaScriptOrdinaryHeap,
    self::JavaScriptSemanticValue,
    bool,
) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let key_error = self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
            {
                let value_error =
                    self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                {
                    let receiver_error =
                        self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                    {
                        let error = if !(heap_error).is_empty() {
                            heap_error.clone()
                        } else if !(key_error).is_empty() {
                            key_error.clone()
                        } else if !(value_error).is_empty() {
                            value_error.clone()
                        } else if !(receiver_error).is_empty() {
                            receiver_error.clone()
                        } else if (self::find_java_script_ordinary_object(
                            match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    source_ml_source1_data_property_value467,
                                    _,
                                ) => source_ml_source1_data_property_value467,
                            },
                            identity.clone(),
                        )) < 0f64
                        {
                            String::from("an ordinary Set needs its actual Object identity")
                        } else {
                            String::new()
                        };
                        {
                            let ml_library_inequality_1 = error != "";
                            if ml_library_inequality_1 {
                                {
                                    let source_ml_source1_field468 = heap.clone();
                                    {
                                        let source_ml_source1_field469 = String::from("error");
                                        {
                                            let source_ml_source1_field470 = error.clone();
                                            {
                                                let source_ml_source1_field471 = false;
                                                {
                                                    let source_ml_source1_field472 =
                                                        self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source1_field473 =
                                                            receiver.clone();
                                                        {
                                                            let source_ml_source1_field474 = Vec::<
                                                                self::JavaScriptSemanticValue,
                                                            >::new(
                                                            );
                                                            (
                                                                source_ml_source1_field469,
                                                                source_ml_source1_field474,
                                                                source_ml_source1_field472,
                                                                source_ml_source1_field470,
                                                                source_ml_source1_field468,
                                                                source_ml_source1_field473,
                                                                source_ml_source1_field471,
                                                            )
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                {
                                    let located = self::locate_java_script_ordinary_property(
                                        heap.clone(),
                                        identity.clone(),
                                        key.clone(),
                                    );
                                    {
                                        let own = if (located.clone()).1 {
                                            (located.clone()).0
                                        } else {
                                            self::create_java_script_ordinary_set_descriptor(
                                                self::JavaScriptSemanticValue::Undefined,
                                                true,
                                            )
                                        };
                                        if (own.clone()).5 {
                                            {
                                                let setter = (own.clone()).6;
                                                match setter.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        let source_ml_source1_field482 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field483 =
                                                                String::from("complete");
                                                            {
                                                                let source_ml_source1_field484 =
                                                                    String::new();
                                                                {
                                                                    let source_ml_source1_field485 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field486 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field487 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field488 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field483, source_ml_source1_field488, source_ml_source1_field486, source_ml_source1_field484, source_ml_source1_field482, source_ml_source1_field487, source_ml_source1_field485)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        let source_ml_source1_field489 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field490 =
                                                                String::from("call");
                                                            {
                                                                let source_ml_source1_field491 =
                                                                    String::new();
                                                                {
                                                                    let source_ml_source1_field492 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field493 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field494 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field495 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field490, source_ml_source1_field495, source_ml_source1_field493, source_ml_source1_field491, source_ml_source1_field489, source_ml_source1_field494, source_ml_source1_field492)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(
                                                        source_ml_source1_f475,
                                                    ) => {
                                                        let source_ml_source1_field496 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field497 =
                                                                String::from("call");
                                                            {
                                                                let source_ml_source1_field498 =
                                                                    String::new();
                                                                {
                                                                    let source_ml_source1_field499 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field500 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field501 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field502 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field497, source_ml_source1_field502, source_ml_source1_field500, source_ml_source1_field498, source_ml_source1_field496, source_ml_source1_field501, source_ml_source1_field499)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Number(
                                                        source_ml_source1_f476,
                                                    ) => {
                                                        let source_ml_source1_field503 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field504 =
                                                                String::from("call");
                                                            {
                                                                let source_ml_source1_field505 =
                                                                    String::new();
                                                                {
                                                                    let source_ml_source1_field506 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field507 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field508 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field509 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field504, source_ml_source1_field509, source_ml_source1_field507, source_ml_source1_field505, source_ml_source1_field503, source_ml_source1_field508, source_ml_source1_field506)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(
                                                        source_ml_source1_f477,
                                                    ) => {
                                                        let source_ml_source1_field510 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field511 =
                                                                String::from("call");
                                                            {
                                                                let source_ml_source1_field512 =
                                                                    String::new();
                                                                {
                                                                    let source_ml_source1_field513 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field514 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field515 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field516 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field511, source_ml_source1_field516, source_ml_source1_field514, source_ml_source1_field512, source_ml_source1_field510, source_ml_source1_field515, source_ml_source1_field513)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Text(
                                                        source_ml_source1_f478,
                                                    ) => {
                                                        let source_ml_source1_field517 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field518 =
                                                                String::from("call");
                                                            {
                                                                let source_ml_source1_field519 =
                                                                    String::new();
                                                                {
                                                                    let source_ml_source1_field520 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field521 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field522 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field523 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field518, source_ml_source1_field523, source_ml_source1_field521, source_ml_source1_field519, source_ml_source1_field517, source_ml_source1_field522, source_ml_source1_field520)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(
                                                        source_ml_source1_f479,
                                                    ) => {
                                                        let source_ml_source1_field524 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field525 =
                                                                String::from("call");
                                                            {
                                                                let source_ml_source1_field526 =
                                                                    String::new();
                                                                {
                                                                    let source_ml_source1_field527 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field528 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field529 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field530 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field525, source_ml_source1_field530, source_ml_source1_field528, source_ml_source1_field526, source_ml_source1_field524, source_ml_source1_field529, source_ml_source1_field527)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Object(
                                                        source_ml_source1_f480,
                                                        source_ml_source1_f481,
                                                    ) => {
                                                        let source_ml_source1_field531 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field532 =
                                                                String::from("call");
                                                            {
                                                                let source_ml_source1_field533 =
                                                                    String::new();
                                                                {
                                                                    let source_ml_source1_field534 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field535 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field536 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field537 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field532, source_ml_source1_field537, source_ml_source1_field535, source_ml_source1_field533, source_ml_source1_field531, source_ml_source1_field536, source_ml_source1_field534)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else if ((own.clone()).10) {
                                            match receiver.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    let source_ml_source1_field551 = heap.clone();
                                                    {
                                                        let source_ml_source1_field552 =
                                                            String::from("complete");
                                                        {
                                                            let source_ml_source1_field553 =
                                                                String::new();
                                                            {
                                                                let source_ml_source1_field554 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field555 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field556 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field557 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field552, source_ml_source1_field557, source_ml_source1_field555, source_ml_source1_field553, source_ml_source1_field551, source_ml_source1_field556, source_ml_source1_field554)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    let source_ml_source1_field558 = heap.clone();
                                                    {
                                                        let source_ml_source1_field559 =
                                                            String::from("complete");
                                                        {
                                                            let source_ml_source1_field560 =
                                                                String::new();
                                                            {
                                                                let source_ml_source1_field561 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field562 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field563 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field564 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field559, source_ml_source1_field564, source_ml_source1_field562, source_ml_source1_field560, source_ml_source1_field558, source_ml_source1_field563, source_ml_source1_field561)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(
                                                    source_ml_source1_f545,
                                                ) => {
                                                    let source_ml_source1_field565 = heap.clone();
                                                    {
                                                        let source_ml_source1_field566 =
                                                            String::from("complete");
                                                        {
                                                            let source_ml_source1_field567 =
                                                                String::new();
                                                            {
                                                                let source_ml_source1_field568 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field569 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field570 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field571 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field566, source_ml_source1_field571, source_ml_source1_field569, source_ml_source1_field567, source_ml_source1_field565, source_ml_source1_field570, source_ml_source1_field568)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(
                                                    source_ml_source1_f546,
                                                ) => {
                                                    let source_ml_source1_field572 = heap.clone();
                                                    {
                                                        let source_ml_source1_field573 =
                                                            String::from("complete");
                                                        {
                                                            let source_ml_source1_field574 =
                                                                String::new();
                                                            {
                                                                let source_ml_source1_field575 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field576 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field577 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field578 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field573, source_ml_source1_field578, source_ml_source1_field576, source_ml_source1_field574, source_ml_source1_field572, source_ml_source1_field577, source_ml_source1_field575)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(
                                                    source_ml_source1_f547,
                                                ) => {
                                                    let source_ml_source1_field579 = heap.clone();
                                                    {
                                                        let source_ml_source1_field580 =
                                                            String::from("complete");
                                                        {
                                                            let source_ml_source1_field581 =
                                                                String::new();
                                                            {
                                                                let source_ml_source1_field582 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field583 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field584 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field585 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field580, source_ml_source1_field585, source_ml_source1_field583, source_ml_source1_field581, source_ml_source1_field579, source_ml_source1_field584, source_ml_source1_field582)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(
                                                    source_ml_source1_f548,
                                                ) => {
                                                    let source_ml_source1_field586 = heap.clone();
                                                    {
                                                        let source_ml_source1_field587 =
                                                            String::from("complete");
                                                        {
                                                            let source_ml_source1_field588 =
                                                                String::new();
                                                            {
                                                                let source_ml_source1_field589 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field590 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field591 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field592 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field587, source_ml_source1_field592, source_ml_source1_field590, source_ml_source1_field588, source_ml_source1_field586, source_ml_source1_field591, source_ml_source1_field589)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(
                                                    source_ml_source1_f549,
                                                ) => {
                                                    let source_ml_source1_field593 = heap.clone();
                                                    {
                                                        let source_ml_source1_field594 =
                                                            String::from("complete");
                                                        {
                                                            let source_ml_source1_field595 =
                                                                String::new();
                                                            {
                                                                let source_ml_source1_field596 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field597 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field598 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field599 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field594, source_ml_source1_field599, source_ml_source1_field597, source_ml_source1_field595, source_ml_source1_field593, source_ml_source1_field598, source_ml_source1_field596)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(
                                                    identity_2,
                                                    source_ml_source1_f550,
                                                ) => {
                                                    let index = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value600, _) => {
                                                                source_ml_source1_data_property_value600
                                                            }
                                                        }, identity_2.clone());
                                                    {
                                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value601, _) => source_ml_source1_data_property_value601.as_slice() }), self::ml_array::number_index(index));
                                                        if match (object.clone(), None::<()>) {
                                                            (Some(_ml_left), Some(_ml_right)) => {
                                                                false
                                                            }
                                                            (None, None) => true,
                                                            _ => false,
                                                        } {
                                                            {
                                                                let source_ml_source1_field602 =
                                                                    heap.clone();
                                                                {
                                                                    let source_ml_source1_field603 =
                                                                        String::from("error");
                                                                    {
                                                                        let source_ml_source1_field604 =
                                                                            String::from(
                                                                                "Set needs its actual receiver Object",
                                                                            );
                                                                        {
                                                                            let source_ml_source1_field605 =
                                                                                false;
                                                                            {
                                                                                let source_ml_source1_field606 = self::JavaScriptSemanticValue::Undefined;
                                                                                {
                                                                                    let source_ml_source1_field607 =
                                                                                        receiver
                                                                                            .clone();
                                                                                    {
                                                                                        let source_ml_source1_field608 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                        (source_ml_source1_field603, source_ml_source1_field608, source_ml_source1_field606, source_ml_source1_field604, source_ml_source1_field602, source_ml_source1_field607, source_ml_source1_field605)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value609) => {
                                                                            source_ml_source1_data_property_value609
                                                                        }
                                                                    }, key.clone());
                                                                {
                                                                    let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value610) => {
                                                                                source_ml_source1_data_property_value610
                                                                            }
                                                                        }, self::ml_array::number_index(at));
                                                                    if !(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!(({
                                                                            let source_ml_source1_nullish_field_receiver611 = (property.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver611.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present612 = ((source_ml_source1_nullish_field_receiver611.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present612 {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value613) => {
                                                                                            source_ml_source1_data_property_value613.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'descriptor')")).encode_utf16().collect(), message_present: true }).raise()
                                                                            }
                                                                        }).9) || !(({
                                                                            let source_ml_source1_nullish_field_receiver614 = (property.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver614.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present615 = ((source_ml_source1_nullish_field_receiver614.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present615 {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value616) => {
                                                                                            source_ml_source1_data_property_value616.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'descriptor')")).encode_utf16().collect(), message_present: true }).raise()
                                                                            }
                                                                        }).10)) {
                                                                            {
                                                                                let source_ml_source1_field617 = heap.clone();
                                                                                {
                                                                                    let source_ml_source1_field618 = String::from("complete");
                                                                                    {
                                                                                        let source_ml_source1_field619 = String::new();
                                                                                        {
                                                                                            let source_ml_source1_field620 = false;
                                                                                            {
                                                                                                let source_ml_source1_field621 = self::JavaScriptSemanticValue::Undefined;
                                                                                                {
                                                                                                    let source_ml_source1_field622 = receiver.clone();
                                                                                                    {
                                                                                                        let source_ml_source1_field623 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                        (source_ml_source1_field618, source_ml_source1_field623, source_ml_source1_field621, source_ml_source1_field619, source_ml_source1_field617, source_ml_source1_field622, source_ml_source1_field620)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            {
                                                                                let proposed = self::create_java_script_ordinary_set_descriptor(value.clone(), match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false });
                                                                                {
                                                                                    let transition = self::define_java_script_ordinary_property(heap.clone(), identity_2.clone(), key.clone(), proposed);
                                                                                    {
                                                                                        let source_ml_source1_field624 = (transition.clone()).1;
                                                                                        {
                                                                                            let source_ml_source1_field625 = if ((transition.clone()).0) == "" {
                                                                                                String::from("complete")
                                                                                            } else {
                                                                                                String::from("error")
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_field626 = (transition.clone()).0;
                                                                                                {
                                                                                                    let source_ml_source1_field627 = (transition.clone()).2;
                                                                                                    {
                                                                                                        let source_ml_source1_field628 = self::JavaScriptSemanticValue::Undefined;
                                                                                                        {
                                                                                                            let source_ml_source1_field629 = receiver.clone();
                                                                                                            {
                                                                                                                let source_ml_source1_field630 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                                (source_ml_source1_field625, source_ml_source1_field630, source_ml_source1_field628, source_ml_source1_field626, source_ml_source1_field624, source_ml_source1_field629, source_ml_source1_field627)
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
                                        } else {
                                            {
                                                let source_ml_source1_field538 = heap.clone();
                                                {
                                                    let source_ml_source1_field539 =
                                                        String::from("complete");
                                                    {
                                                        let source_ml_source1_field540 =
                                                            String::new();
                                                        {
                                                            let source_ml_source1_field541 = false;
                                                            {
                                                                let source_ml_source1_field542 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source1_field543 =
                                                                        receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field544 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field539, source_ml_source1_field544, source_ml_source1_field542, source_ml_source1_field540, source_ml_source1_field538, source_ml_source1_field543, source_ml_source1_field541)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_own_keys(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
) -> (String, Vec<self::JavaScriptSemanticValue>) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if !(error).is_empty() {
            {
                let source_ml_source1_field631 = Vec::<self::JavaScriptSemanticValue>::new();
                {
                    let source_ml_source1_field632 = error.clone();
                    (source_ml_source1_field632, source_ml_source1_field631)
                }
            }
        } else {
            {
                let index = self::find_java_script_ordinary_object(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value633,
                            _,
                        ) => source_ml_source1_data_property_value633,
                    },
                    identity.clone(),
                );
                {
                    let object = self::ml_array::optional_at(
                        (match &heap {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value634,
                                _,
                            ) => source_ml_source1_data_property_value634.as_slice(),
                        }),
                        self::ml_array::number_index(index),
                    );
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        {
                            let source_ml_source1_field635 =
                                Vec::<self::JavaScriptSemanticValue>::new();
                            {
                                let source_ml_source1_field636 = String::from(
                                    "OwnPropertyKeys needs its actual Object identity",
                                );
                                (source_ml_source1_field636, source_ml_source1_field635)
                            }
                        }
                    } else {
                        {
                            let numeric = Vec::<self::JavaScriptSemanticValue>::new();
                            {
                                let positions = Vec::<f64>::new();
                                {
                                    let text = Vec::<self::JavaScriptSemanticValue>::new();
                                    {
                                        let symbols = Vec::<self::JavaScriptSemanticValue>::new();
                                        {
                                            let source_ml_temporary_values22 = match object
                                                .clone()
                                                .expect("a proven present value")
                                            {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source1_data_property_value637,
                                                ) => source_ml_source1_data_property_value637,
                                            };
                                            {
                                                let source_ml_temporary_index22 =
                                                    self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_s638 = self::source_ml_temporary_read_java_script_ordinary_own_keys_loop23(numeric, positions, text, symbols, source_ml_temporary_values22, source_ml_temporary_index22);
                                                    match source_ml_source1_s638 {
                                                        self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Done(numeric_2, text_2, symbols_2) => {
                                                            {
                                                                let source_ml_source1_field642 = {
                                                                    let source_ml_source1_concat_value639 = numeric_2;
                                                                    {
                                                                        let source_ml_source1_concat_value640 = text_2;
                                                                        {
                                                                            let source_ml_source1_concat_value641 = symbols_2;
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value639), source_ml_source1_concat_value640), source_ml_source1_concat_value641)
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_field643 = String::new();
                                                                    (source_ml_source1_field643, source_ml_source1_field642)
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
