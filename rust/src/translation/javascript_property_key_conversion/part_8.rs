// Generated from complete js/src/translation/javascript-property-key-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_insert_java_script_ordinary_object_loop26;
use super::source_ml_temporary_read_java_script_ordinary_own_keys_loop23;
use super::source_ml_temporary_register_java_script_ordinary_symbol_loop28;
use super::test_java_script_ordinary_symbol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_own_keys(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
) -> (self::ml_string::Text, Vec<self::JavaScriptSemanticValue>) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                {
                    let source_ml_source1_field695 = Vec::<self::JavaScriptSemanticValue>::new();
                    {
                        let source_ml_source1_field696 = error.clone();
                        (source_ml_source1_field696, source_ml_source1_field695)
                    }
                }
            } else {
                {
                    let index = self::find_java_script_ordinary_object(
                        match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value697,
                                _,
                            ) => source_ml_source1_data_property_value697,
                        },
                        identity.clone(),
                    );
                    {
                        let object = self::ml_array::optional_at(
                            (match &heap {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    source_ml_source1_data_property_value698,
                                    _,
                                ) => source_ml_source1_data_property_value698.as_slice(),
                            }),
                            self::ml_array::number_index(index),
                        );
                        if match (object.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            {
                                let source_ml_source1_field699 =
                                    Vec::<self::JavaScriptSemanticValue>::new();
                                {
                                    let source_ml_source1_field700 =
                                        self::ml_string::Text::from_units(vec![
                                            79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121,
                                            75, 101, 121, 115, 32, 110, 101, 101, 100, 115, 32,
                                            105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79,
                                            98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116,
                                            105, 116, 121,
                                        ]);
                                    (source_ml_source1_field700, source_ml_source1_field699)
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
                                            let symbols =
                                                Vec::<self::JavaScriptSemanticValue>::new();
                                            {
                                                let source_ml_temporary_values22 = {
                                                    let source_ml_source1_nullish_field_receiver701 =
                                                        (object.clone(),);
                                                    if match (source_ml_source1_nullish_field_receiver701.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source1_nullish_field_present702 = ((source_ml_source1_nullish_field_receiver701.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source1_nullish_field_present702 {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value703) => {
                                                                source_ml_source1_data_property_value703
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 39, 41])).0, message_present: true }).raise()
                                                }
                                                };
                                                {
                                                    let source_ml_temporary_index22 =
                                                        self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s704 = self::source_ml_temporary_read_java_script_ordinary_own_keys_loop23(numeric, positions, text, symbols, source_ml_temporary_values22, source_ml_temporary_index22);
                                                        match source_ml_source1_s704 {
                                                        self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Done(numeric_2, text_2, symbols_2) => {
                                                            {
                                                                let source_ml_source1_field708 = {
                                                                    let source_ml_source1_concat_value705 = numeric_2;
                                                                    {
                                                                        let source_ml_source1_concat_value706 = text_2;
                                                                        {
                                                                            let source_ml_source1_concat_value707 = symbols_2;
                                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value705), source_ml_source1_concat_value706), source_ml_source1_concat_value707)
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_field709 = self::ml_string::Text::from_units(vec![]);
                                                                    (source_ml_source1_field709, source_ml_source1_field708)
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
pub fn insert_java_script_ordinary_object(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_2 = heap_error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_2 {
                {
                    let source_ml_source1_field710 = heap.clone();
                    {
                        let source_ml_source1_field711 = false;
                        {
                            let source_ml_source1_field712 = heap_error.clone();
                            (
                                source_ml_source1_field712,
                                source_ml_source1_field710,
                                source_ml_source1_field711,
                            )
                        }
                    }
                }
            } else if ((match &heap {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    source_ml_source1_data_property_value714,
                    _,
                ) => source_ml_source1_data_property_value714.as_slice(),
            })
            .len() as f64)
                >= 4096f64
            {
                {
                    let source_ml_source1_field715 = heap.clone();
                    {
                        let source_ml_source1_field716 = false;
                        {
                            let source_ml_source1_field717 =
                                self::ml_string::Text::from_units(vec![
                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112,
                                    32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114,
                                    101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100,
                                    115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105,
                                    116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117,
                                    100, 103, 101, 116,
                                ]);
                            (
                                source_ml_source1_field717,
                                source_ml_source1_field715,
                                source_ml_source1_field716,
                            )
                        }
                    }
                }
            } else if (((identity <= (self::ml::Big::from_i128(0)))
                || (identity > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                || ((self::find_java_script_ordinary_object(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value719,
                            _,
                        ) => source_ml_source1_data_property_value719,
                    },
                    identity.clone(),
                )) >= 0f64))
                || self::test_java_script_ordinary_symbol(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            _,
                            source_ml_source1_data_property_value720,
                        ) => source_ml_source1_data_property_value720,
                    },
                    identity.clone(),
                )
            {
                {
                    let source_ml_source1_field721 = heap.clone();
                    {
                        let source_ml_source1_field722 = false;
                        {
                            let source_ml_source1_field723 =
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 102, 114, 101, 115, 104, 32, 111, 114, 100, 105, 110,
                                    97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 113,
                                    117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 100, 105, 115,
                                    116, 105, 110, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97,
                                    108, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100,
                                    32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103,
                                    108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116,
                                    121,
                                ]);
                            (
                                source_ml_source1_field723,
                                source_ml_source1_field721,
                                source_ml_source1_field722,
                            )
                        }
                    }
                }
            } else if match prototype.clone() {
                self::JavaScriptSemanticValue::Undefined => true,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f724) => true,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f725) => true,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f726) => true,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f727) => true,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f728) => true,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f729,
                    source_ml_source1_f730,
                ) => true,
            } && match prototype.clone() {
                self::JavaScriptSemanticValue::Undefined => true,
                self::JavaScriptSemanticValue::Null => true,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f731) => true,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f732) => true,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f733) => true,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f734) => true,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f735) => true,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f736,
                    source_ml_source1_f737,
                ) => false,
            } {
                {
                    let source_ml_source1_field738 = heap.clone();
                    {
                        let source_ml_source1_field739 = false;
                        {
                            let source_ml_source1_field740 =
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112,
                                    114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116,
                                    32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32,
                                    100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99,
                                    116,
                                ]);
                            (
                                source_ml_source1_field740,
                                source_ml_source1_field738,
                                source_ml_source1_field739,
                            )
                        }
                    }
                }
            } else {
                {
                    let error = self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        prototype.clone(),
                    );
                    {
                        let ml_library_inequality_1 =
                            error != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_1 {
                            {
                                let source_ml_source1_field741 = heap.clone();
                                {
                                    let source_ml_source1_field742 = false;
                                    {
                                        let source_ml_source1_field743 = error.clone();
                                        (
                                            source_ml_source1_field743,
                                            source_ml_source1_field741,
                                            source_ml_source1_field742,
                                        )
                                    }
                                }
                            }
                        } else {
                            {
                                let at = 0f64;
                                {
                                    let source_ml_temporary_values25 = match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            source_ml_source1_data_property_value744,
                                            _,
                                        ) => source_ml_source1_data_property_value744,
                                    };
                                    {
                                        let source_ml_temporary_index25 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            let at_2 = self::source_ml_temporary_insert_java_script_ordinary_object_loop26(identity.clone(), at, source_ml_temporary_values25, source_ml_temporary_index25);
                                            {
                                                let object = self::JavaScriptOrdinaryObject::OrdinaryObject(identity.clone(), Box::new(prototype.clone()), true, false, Vec::<self::JavaScriptOrdinaryProperty>::new());
                                                {
                                                    let objects = {
                                                        let source_ml_source1_concat_value749 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value746, _) => source_ml_source1_data_property_value746.as_slice() }), 0f64, at_2);
                                                        {
                                                            let source_ml_source1_concat_value750 =
                                                                vec![object];
                                                            {
                                                                let source_ml_source1_concat_value751 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value748, _) => source_ml_source1_data_property_value748.as_slice() }), at_2, f64::INFINITY);
                                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), source_ml_source1_concat_value749), source_ml_source1_concat_value750), source_ml_source1_concat_value751)
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_field753 = self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects, match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value752) => {
                                                                    source_ml_source1_data_property_value752
                                                                }
                                                            });
                                                        {
                                                            let source_ml_source1_field754 = true;
                                                            {
                                                                let source_ml_source1_field755 = self::ml_string::Text::from_units(vec![]);
                                                                (
                                                                    source_ml_source1_field755,
                                                                    source_ml_source1_field753,
                                                                    source_ml_source1_field754,
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn register_java_script_ordinary_symbol(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                {
                    let source_ml_source1_field756 = heap.clone();
                    {
                        let source_ml_source1_field757 = false;
                        {
                            let source_ml_source1_field758 = error.clone();
                            (
                                source_ml_source1_field758,
                                source_ml_source1_field756,
                                source_ml_source1_field757,
                            )
                        }
                    }
                }
            } else if ((match &heap {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    _,
                    source_ml_source1_data_property_value760,
                ) => source_ml_source1_data_property_value760.as_slice(),
            })
            .len() as f64)
                >= 16_384f64
            {
                {
                    let source_ml_source1_field761 = heap.clone();
                    {
                        let source_ml_source1_field762 = false;
                        {
                            let source_ml_source1_field763 =
                                self::ml_string::Text::from_units(vec![
                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112,
                                    32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114,
                                    101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100,
                                    115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105,
                                    116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117,
                                    100, 103, 101, 116,
                                ]);
                            (
                                source_ml_source1_field763,
                                source_ml_source1_field761,
                                source_ml_source1_field762,
                            )
                        }
                    }
                }
            } else if (((identity <= (self::ml::Big::from_i128(0)))
                || (identity > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                || ((self::find_java_script_ordinary_object(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value765,
                            _,
                        ) => source_ml_source1_data_property_value765,
                    },
                    identity.clone(),
                )) >= 0f64))
                || self::test_java_script_ordinary_symbol(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            _,
                            source_ml_source1_data_property_value766,
                        ) => source_ml_source1_data_property_value766,
                    },
                    identity.clone(),
                )
            {
                {
                    let source_ml_source1_field767 = heap.clone();
                    {
                        let source_ml_source1_field768 = false;
                        {
                            let source_ml_source1_field769 =
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 102, 114, 101, 115, 104, 32, 83, 121, 109, 98, 111,
                                    108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116,
                                    115, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 101, 120,
                                    116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108, 111,
                                    99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101,
                                    100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101,
                                    110, 116, 105, 116, 121,
                                ]);
                            (
                                source_ml_source1_field769,
                                source_ml_source1_field767,
                                source_ml_source1_field768,
                            )
                        }
                    }
                }
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_temporary_values27 = match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                _,
                                source_ml_source1_data_property_value770,
                            ) => source_ml_source1_data_property_value770,
                        };
                        {
                            let source_ml_temporary_index27 = self::ml::Big::from_i128(0);
                            {
                                let at_2 = self::source_ml_temporary_register_java_script_ordinary_symbol_loop28(identity.clone(), at, source_ml_temporary_values27, source_ml_temporary_index27);
                                {
                                    let symbols = {
                                        let source_ml_source1_concat_value775 =
                                            self::ml_array::slice(
                                                (match &heap {
                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                        _,
                                                        source_ml_source1_data_property_value772,
                                                    ) => source_ml_source1_data_property_value772
                                                        .as_slice(),
                                                }),
                                                0f64,
                                                at_2,
                                            );
                                        {
                                            let source_ml_source1_concat_value776 =
                                                vec![identity.clone()];
                                            {
                                                let source_ml_source1_concat_value777 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value774) => source_ml_source1_data_property_value774.as_slice() }), at_2, f64::INFINITY);
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<self::ml::Big>::new(),
                                                            source_ml_source1_concat_value775,
                                                        ),
                                                        source_ml_source1_concat_value776,
                                                    ),
                                                    source_ml_source1_concat_value777,
                                                )
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_field779 =
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                match heap.clone() {
                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                        source_ml_source1_data_property_value778,
                                                        _,
                                                    ) => source_ml_source1_data_property_value778,
                                                },
                                                symbols,
                                            );
                                        {
                                            let source_ml_source1_field780 = true;
                                            {
                                                let source_ml_source1_field781 =
                                                    self::ml_string::Text::from_units(vec![]);
                                                (
                                                    source_ml_source1_field781,
                                                    source_ml_source1_field779,
                                                    source_ml_source1_field780,
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_conversion_spelling(
    name: &self::ml_string::Text,
) -> self::JavaScriptSemanticValue {
    if (*name) == (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])) {
        self::JavaScriptSemanticValue::Text(vec![
            100f64, 101f64, 102f64, 97f64, 117f64, 108f64, 116f64,
        ])
    } else if (*name) == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])) {
        self::JavaScriptSemanticValue::Text(vec![110f64, 117f64, 109f64, 98f64, 101f64, 114f64])
    } else if (*name) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])) {
        self::JavaScriptSemanticValue::Text(vec![115f64, 116f64, 114f64, 105f64, 110f64, 103f64])
    } else if (*name) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102]))
    {
        self::JavaScriptSemanticValue::Text(vec![
            118f64, 97f64, 108f64, 117f64, 101f64, 79f64, 102f64,
        ])
    } else if (*name)
        == (self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]))
    {
        self::JavaScriptSemanticValue::Text(vec![
            116f64, 111f64, 83f64, 116f64, 114f64, 105f64, 110f64, 103f64,
        ])
    } else {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101,
                114, 115, 105, 111, 110, 32, 115, 112, 101, 108, 108, 105, 110, 103, 32, 110, 101,
                101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103,
                105, 115, 116, 101, 114, 101, 100, 32, 110, 97, 109, 101,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_conversion_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
) -> self::ml_string::Text {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_3 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_3 {
                error.clone()
            } else if {
                let source_ml_source1_s783 = match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                        source_ml_source1_data_property_value791,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source1_data_property_value791 =
                            *source_ml_source1_data_property_value791;
                        source_ml_source1_data_property_value791.clone()
                    }
                };
                match source_ml_source1_s783 {
                    self::JavaScriptSemanticValue::Undefined => true,
                    self::JavaScriptSemanticValue::Null => true,
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f784) => true,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f785) => true,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f786) => true,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f787) => true,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f788) => true,
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f789,
                        source_ml_source1_f790,
                    ) => false,
                }
            } {
                self::ml_string::Text::from_units(vec![
                    97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118,
                    101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116,
                    105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                    99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101,
                    105, 118, 101, 114,
                ])
            } else {
                {
                    let receiver_error = self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        match state.clone() {
                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                source_ml_source1_data_property_value792,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source1_data_property_value792 =
                                    *source_ml_source1_data_property_value792;
                                source_ml_source1_data_property_value792.clone()
                            }
                        },
                    );
                    {
                        let ml_library_inequality_2 =
                            receiver_error != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_2 {
                            receiver_error.clone()
                        } else if {
                            let source_ml_source1_s794 = match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                    _,
                                    source_ml_source1_data_property_value802,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value802 =
                                        *source_ml_source1_data_property_value802;
                                    source_ml_source1_data_property_value802.clone()
                                }
                            };
                            match source_ml_source1_s794 {
                                self::JavaScriptSemanticValue::Undefined => true,
                                self::JavaScriptSemanticValue::Null => true,
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f795) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source1_f796) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f797) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f798) => true,
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f799) => {
                                    false
                                }
                                self::JavaScriptSemanticValue::Object(
                                    source_ml_source1_f800,
                                    source_ml_source1_f801,
                                ) => true,
                            }
                        } {
                            self::ml_string::Text::from_units(vec![
                                116, 104, 101, 32, 101, 120, 111, 116, 105, 99, 32, 109, 101, 116,
                                104, 111, 100, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98,
                                101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 114,
                                101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 119, 101, 108,
                                108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108,
                            ])
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value803, _, _) => {
                                        let source_ml_source1_data_property_value803 = *source_ml_source1_data_property_value803;
                                        source_ml_source1_data_property_value803.clone()
                                    }
                                });
                                {
                                    let ml_library_inequality_1 =
                                        key_error != (self::ml_string::Text::from_units(vec![]));
                                    if ml_library_inequality_1 {
                                    key_error.clone()
                                } else if (((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value805, _) => {
                                            source_ml_source1_data_property_value805.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) && ((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value807, _) => {
                                            source_ml_source1_data_property_value807.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) && ((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value809, _) => {
                                            source_ml_source1_data_property_value809.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                                        self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99, 116, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112, 101, 32, 104, 105, 110, 116])
                                    } else if (self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])], &match state.clone() {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value810) => {
                                                source_ml_source1_data_property_value810.clone()
                                            }
                                        }, 0f64, false, |a, b| a == b)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 109, 101, 116, 104, 111, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
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
pub fn continue_java_script_primitive_conversion(
    state: self::JavaScriptPrimitiveConversionState,
    phase: &self::ml_string::Text,
) -> self::JavaScriptPrimitiveConversionTransition {
    self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(Box::new(
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
            Box::new(match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    source_ml_source1_data_property_value811,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source1_data_property_value811 =
                        *source_ml_source1_data_property_value811;
                    source_ml_source1_data_property_value811.clone()
                }
            }),
            Box::new(match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    source_ml_source1_data_property_value812,
                    _,
                    _,
                ) => {
                    let source_ml_source1_data_property_value812 =
                        *source_ml_source1_data_property_value812;
                    source_ml_source1_data_property_value812.clone()
                }
            }),
            match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    source_ml_source1_data_property_value813,
                    _,
                ) => source_ml_source1_data_property_value813.clone(),
            },
            phase.clone(),
        ),
    ))
}
