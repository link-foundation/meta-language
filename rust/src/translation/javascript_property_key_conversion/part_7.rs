// Generated from complete js/src/translation/javascript-property-key-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::create_java_script_ordinary_set_descriptor;
use super::define_java_script_ordinary_property;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_ordinary_set(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
    value: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
) -> (
    self::ml_string::Text,
    Vec<self::JavaScriptSemanticValue>,
    self::JavaScriptSemanticValue,
    self::ml_string::Text,
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
                        let error = {
                            let ml_library_inequality_5 =
                                heap_error != (self::ml_string::Text::from_units(vec![]));
                            if ml_library_inequality_5 {
                                heap_error.clone()
                            } else {
                                let ml_library_inequality_4 =
                                    key_error != (self::ml_string::Text::from_units(vec![]));
                                if ml_library_inequality_4 {
                                    key_error.clone()
                                } else {
                                    let ml_library_inequality_3 =
                                        value_error != (self::ml_string::Text::from_units(vec![]));
                                    if ml_library_inequality_3 {
                                        value_error.clone()
                                    } else {
                                        let ml_library_inequality_2 = receiver_error
                                            != (self::ml_string::Text::from_units(vec![]));
                                        if ml_library_inequality_2 {
                                            receiver_error.clone()
                                        } else if (self::find_java_script_ordinary_object(
                                            match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                    source_ml_source1_data_property_value527,
                                                    _,
                                                ) => source_ml_source1_data_property_value527,
                                            },
                                            identity.clone(),
                                        )) < 0f64
                                        {
                                            self::ml_string::Text::from_units(vec![
                                                97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121,
                                                32, 83, 101, 116, 32, 110, 101, 101, 100, 115, 32,
                                                105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                                79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110,
                                                116, 105, 116, 121,
                                            ])
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                    }
                                }
                            }
                        };
                        {
                            let ml_library_inequality_1 =
                                error != (self::ml_string::Text::from_units(vec![]));
                            if ml_library_inequality_1 {
                                {
                                    let source_ml_source1_field528 = heap.clone();
                                    {
                                        let source_ml_source1_field529 =
                                            self::ml_string::Text::from_units(vec![
                                                101, 114, 114, 111, 114,
                                            ]);
                                        {
                                            let source_ml_source1_field530 = error.clone();
                                            {
                                                let source_ml_source1_field531 = false;
                                                {
                                                    let source_ml_source1_field532 =
                                                        self::JavaScriptSemanticValue::Undefined;
                                                    {
                                                        let source_ml_source1_field533 =
                                                            receiver.clone();
                                                        {
                                                            let source_ml_source1_field534 = Vec::<
                                                                self::JavaScriptSemanticValue,
                                                            >::new(
                                                            );
                                                            (
                                                                source_ml_source1_field529,
                                                                source_ml_source1_field534,
                                                                source_ml_source1_field532,
                                                                source_ml_source1_field530,
                                                                source_ml_source1_field528,
                                                                source_ml_source1_field533,
                                                                source_ml_source1_field531,
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
                                                        let source_ml_source1_field542 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field543 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![
                                                                        99, 111, 109, 112, 108,
                                                                        101, 116, 101,
                                                                    ],
                                                                );
                                                            {
                                                                let source_ml_source1_field544 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field545 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field546 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_field547 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field548 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                (source_ml_source1_field543, source_ml_source1_field548, source_ml_source1_field546, source_ml_source1_field544, source_ml_source1_field542, source_ml_source1_field547, source_ml_source1_field545)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        let source_ml_source1_field549 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field550 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![99, 97, 108, 108],
                                                                );
                                                            {
                                                                let source_ml_source1_field551 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field552 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field553 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field554 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field555 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field550, source_ml_source1_field555, source_ml_source1_field553, source_ml_source1_field551, source_ml_source1_field549, source_ml_source1_field554, source_ml_source1_field552)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(
                                                        source_ml_source1_f535,
                                                    ) => {
                                                        let source_ml_source1_field556 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field557 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![99, 97, 108, 108],
                                                                );
                                                            {
                                                                let source_ml_source1_field558 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field559 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field560 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field561 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field562 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field557, source_ml_source1_field562, source_ml_source1_field560, source_ml_source1_field558, source_ml_source1_field556, source_ml_source1_field561, source_ml_source1_field559)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Number(
                                                        source_ml_source1_f536,
                                                    ) => {
                                                        let source_ml_source1_field563 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field564 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![99, 97, 108, 108],
                                                                );
                                                            {
                                                                let source_ml_source1_field565 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field566 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field567 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field568 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field569 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field564, source_ml_source1_field569, source_ml_source1_field567, source_ml_source1_field565, source_ml_source1_field563, source_ml_source1_field568, source_ml_source1_field566)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(
                                                        source_ml_source1_f537,
                                                    ) => {
                                                        let source_ml_source1_field570 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field571 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![99, 97, 108, 108],
                                                                );
                                                            {
                                                                let source_ml_source1_field572 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field573 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field574 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field575 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field576 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field571, source_ml_source1_field576, source_ml_source1_field574, source_ml_source1_field572, source_ml_source1_field570, source_ml_source1_field575, source_ml_source1_field573)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Text(
                                                        source_ml_source1_f538,
                                                    ) => {
                                                        let source_ml_source1_field577 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field578 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![99, 97, 108, 108],
                                                                );
                                                            {
                                                                let source_ml_source1_field579 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field580 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field581 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field582 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field583 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field578, source_ml_source1_field583, source_ml_source1_field581, source_ml_source1_field579, source_ml_source1_field577, source_ml_source1_field582, source_ml_source1_field580)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(
                                                        source_ml_source1_f539,
                                                    ) => {
                                                        let source_ml_source1_field584 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field585 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![99, 97, 108, 108],
                                                                );
                                                            {
                                                                let source_ml_source1_field586 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field587 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field588 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field589 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field590 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field585, source_ml_source1_field590, source_ml_source1_field588, source_ml_source1_field586, source_ml_source1_field584, source_ml_source1_field589, source_ml_source1_field587)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Object(
                                                        source_ml_source1_f540,
                                                        source_ml_source1_f541,
                                                    ) => {
                                                        let source_ml_source1_field591 =
                                                            heap.clone();
                                                        {
                                                            let source_ml_source1_field592 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![99, 97, 108, 108],
                                                                );
                                                            {
                                                                let source_ml_source1_field593 = self::ml_string::Text::from_units(vec![]);
                                                                {
                                                                    let source_ml_source1_field594 =
                                                                        false;
                                                                    {
                                                                        let source_ml_source1_field595 =
                                                                            setter.clone();
                                                                        {
                                                                            let source_ml_source1_field596 =
                                                                                receiver.clone();
                                                                            {
                                                                                let source_ml_source1_field597 = vec![
                                                                                    value.clone(),
                                                                                ];
                                                                                (source_ml_source1_field592, source_ml_source1_field597, source_ml_source1_field595, source_ml_source1_field593, source_ml_source1_field591, source_ml_source1_field596, source_ml_source1_field594)
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
                                                    let source_ml_source1_field611 = heap.clone();
                                                    {
                                                        let source_ml_source1_field612 =
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 109, 112, 108, 101,
                                                                    116, 101,
                                                                ],
                                                            );
                                                        {
                                                            let source_ml_source1_field613 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![],
                                                                );
                                                            {
                                                                let source_ml_source1_field614 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field615 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field616 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field617 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field612, source_ml_source1_field617, source_ml_source1_field615, source_ml_source1_field613, source_ml_source1_field611, source_ml_source1_field616, source_ml_source1_field614)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    let source_ml_source1_field618 = heap.clone();
                                                    {
                                                        let source_ml_source1_field619 =
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 109, 112, 108, 101,
                                                                    116, 101,
                                                                ],
                                                            );
                                                        {
                                                            let source_ml_source1_field620 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![],
                                                                );
                                                            {
                                                                let source_ml_source1_field621 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field622 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field623 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field624 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field619, source_ml_source1_field624, source_ml_source1_field622, source_ml_source1_field620, source_ml_source1_field618, source_ml_source1_field623, source_ml_source1_field621)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Boolean(
                                                    source_ml_source1_f605,
                                                ) => {
                                                    let source_ml_source1_field625 = heap.clone();
                                                    {
                                                        let source_ml_source1_field626 =
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 109, 112, 108, 101,
                                                                    116, 101,
                                                                ],
                                                            );
                                                        {
                                                            let source_ml_source1_field627 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![],
                                                                );
                                                            {
                                                                let source_ml_source1_field628 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field629 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field630 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field631 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field626, source_ml_source1_field631, source_ml_source1_field629, source_ml_source1_field627, source_ml_source1_field625, source_ml_source1_field630, source_ml_source1_field628)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Number(
                                                    source_ml_source1_f606,
                                                ) => {
                                                    let source_ml_source1_field632 = heap.clone();
                                                    {
                                                        let source_ml_source1_field633 =
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 109, 112, 108, 101,
                                                                    116, 101,
                                                                ],
                                                            );
                                                        {
                                                            let source_ml_source1_field634 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![],
                                                                );
                                                            {
                                                                let source_ml_source1_field635 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field636 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field637 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field638 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field633, source_ml_source1_field638, source_ml_source1_field636, source_ml_source1_field634, source_ml_source1_field632, source_ml_source1_field637, source_ml_source1_field635)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Integer(
                                                    source_ml_source1_f607,
                                                ) => {
                                                    let source_ml_source1_field639 = heap.clone();
                                                    {
                                                        let source_ml_source1_field640 =
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 109, 112, 108, 101,
                                                                    116, 101,
                                                                ],
                                                            );
                                                        {
                                                            let source_ml_source1_field641 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![],
                                                                );
                                                            {
                                                                let source_ml_source1_field642 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field643 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field644 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field645 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field640, source_ml_source1_field645, source_ml_source1_field643, source_ml_source1_field641, source_ml_source1_field639, source_ml_source1_field644, source_ml_source1_field642)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Text(
                                                    source_ml_source1_f608,
                                                ) => {
                                                    let source_ml_source1_field646 = heap.clone();
                                                    {
                                                        let source_ml_source1_field647 =
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 109, 112, 108, 101,
                                                                    116, 101,
                                                                ],
                                                            );
                                                        {
                                                            let source_ml_source1_field648 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![],
                                                                );
                                                            {
                                                                let source_ml_source1_field649 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field650 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field651 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field652 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field647, source_ml_source1_field652, source_ml_source1_field650, source_ml_source1_field648, source_ml_source1_field646, source_ml_source1_field651, source_ml_source1_field649)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(
                                                    source_ml_source1_f609,
                                                ) => {
                                                    let source_ml_source1_field653 = heap.clone();
                                                    {
                                                        let source_ml_source1_field654 =
                                                            self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 109, 112, 108, 101,
                                                                    116, 101,
                                                                ],
                                                            );
                                                        {
                                                            let source_ml_source1_field655 =
                                                                self::ml_string::Text::from_units(
                                                                    vec![],
                                                                );
                                                            {
                                                                let source_ml_source1_field656 =
                                                                    false;
                                                                {
                                                                    let source_ml_source1_field657 = self::JavaScriptSemanticValue::Undefined;
                                                                    {
                                                                        let source_ml_source1_field658 =
                                                                            receiver.clone();
                                                                        {
                                                                            let source_ml_source1_field659 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                            (source_ml_source1_field654, source_ml_source1_field659, source_ml_source1_field657, source_ml_source1_field655, source_ml_source1_field653, source_ml_source1_field658, source_ml_source1_field656)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Object(
                                                    identity_2,
                                                    source_ml_source1_f610,
                                                ) => {
                                                    let index = self::find_java_script_ordinary_object(match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value660, _) => {
                                                                source_ml_source1_data_property_value660
                                                            }
                                                        }, identity_2.clone());
                                                    {
                                                        let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value661, _) => source_ml_source1_data_property_value661.as_slice() }), self::ml_array::number_index(index));
                                                        if match (object.clone(), None::<()>) {
                                                            (Some(_ml_left), Some(_ml_right)) => {
                                                                false
                                                            }
                                                            (None, None) => true,
                                                            _ => false,
                                                        } {
                                                            {
                                                                let source_ml_source1_field662 =
                                                                    heap.clone();
                                                                {
                                                                    let source_ml_source1_field663 = self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]);
                                                                    {
                                                                        let source_ml_source1_field664 = self::ml_string::Text::from_units(vec![83, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]);
                                                                        {
                                                                            let source_ml_source1_field665 =
                                                                                false;
                                                                            {
                                                                                let source_ml_source1_field666 = self::JavaScriptSemanticValue::Undefined;
                                                                                {
                                                                                    let source_ml_source1_field667 =
                                                                                        receiver
                                                                                            .clone();
                                                                                    {
                                                                                        let source_ml_source1_field668 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                        (source_ml_source1_field663, source_ml_source1_field668, source_ml_source1_field666, source_ml_source1_field664, source_ml_source1_field662, source_ml_source1_field667, source_ml_source1_field665)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let at = self::find_java_script_ordinary_property({
                                                                        let source_ml_source1_nullish_field_receiver669 = (object.clone(),);
                                                                        if match (source_ml_source1_nullish_field_receiver669.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source1_nullish_field_present670 = ((source_ml_source1_nullish_field_receiver669.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source1_nullish_field_present670 {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value671) => {
                                                                                        source_ml_source1_data_property_value671
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    }, key.clone());
                                                                {
                                                                    let property = self::ml_array::optional_at(&{
                                                                            let source_ml_source1_nullish_field_receiver672 = (object.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver672.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present673 = ((source_ml_source1_nullish_field_receiver672.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present673 {
                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value674) => {
                                                                                            source_ml_source1_data_property_value674
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }, self::ml_array::number_index(at));
                                                                    if !(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (!(({
                                                                            let source_ml_source1_nullish_field_receiver675 = (property.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver675.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present676 = ((source_ml_source1_nullish_field_receiver675.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present676 {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value677) => {
                                                                                            source_ml_source1_data_property_value677.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).9) || !(({
                                                                            let source_ml_source1_nullish_field_receiver678 = (property.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver678.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present679 = ((source_ml_source1_nullish_field_receiver678.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present679 {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value680) => {
                                                                                            source_ml_source1_data_property_value680.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).10)) {
                                                                            {
                                                                                let source_ml_source1_field681 = heap.clone();
                                                                                {
                                                                                    let source_ml_source1_field682 = self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101]);
                                                                                    {
                                                                                        let source_ml_source1_field683 = self::ml_string::Text::from_units(vec![]);
                                                                                        {
                                                                                            let source_ml_source1_field684 = false;
                                                                                            {
                                                                                                let source_ml_source1_field685 = self::JavaScriptSemanticValue::Undefined;
                                                                                                {
                                                                                                    let source_ml_source1_field686 = receiver.clone();
                                                                                                    {
                                                                                                        let source_ml_source1_field687 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                        (source_ml_source1_field682, source_ml_source1_field687, source_ml_source1_field685, source_ml_source1_field683, source_ml_source1_field681, source_ml_source1_field686, source_ml_source1_field684)
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
                                                                                        let source_ml_source1_field688 = (transition.clone()).1;
                                                                                        {
                                                                                            let source_ml_source1_field689 = if ((transition.clone()).0) == (self::ml_string::Text::from_units(vec![])) {
                                                                                                self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101])
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_field690 = (transition.clone()).0;
                                                                                                {
                                                                                                    let source_ml_source1_field691 = (transition.clone()).2;
                                                                                                    {
                                                                                                        let source_ml_source1_field692 = self::JavaScriptSemanticValue::Undefined;
                                                                                                        {
                                                                                                            let source_ml_source1_field693 = receiver.clone();
                                                                                                            {
                                                                                                                let source_ml_source1_field694 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                                                                (source_ml_source1_field689, source_ml_source1_field694, source_ml_source1_field692, source_ml_source1_field690, source_ml_source1_field688, source_ml_source1_field693, source_ml_source1_field691)
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
                                                let source_ml_source1_field598 = heap.clone();
                                                {
                                                    let source_ml_source1_field599 =
                                                        self::ml_string::Text::from_units(vec![
                                                            99, 111, 109, 112, 108, 101, 116, 101,
                                                        ]);
                                                    {
                                                        let source_ml_source1_field600 =
                                                            self::ml_string::Text::from_units(
                                                                vec![],
                                                            );
                                                        {
                                                            let source_ml_source1_field601 = false;
                                                            {
                                                                let source_ml_source1_field602 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source1_field603 =
                                                                        receiver.clone();
                                                                    {
                                                                        let source_ml_source1_field604 = Vec::<self::JavaScriptSemanticValue>::new();
                                                                        (source_ml_source1_field599, source_ml_source1_field604, source_ml_source1_field602, source_ml_source1_field600, source_ml_source1_field598, source_ml_source1_field603, source_ml_source1_field601)
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
