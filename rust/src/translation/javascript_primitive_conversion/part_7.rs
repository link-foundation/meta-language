// Generated from complete js/src/translation/javascript-primitive-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptSemanticValue;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_insert_java_script_ordinary_object_loop26;
use super::source_ml_temporary_register_java_script_ordinary_symbol_loop28;
use super::test_java_script_ordinary_symbol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn insert_java_script_ordinary_object(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
) -> (String, self::JavaScriptOrdinaryHeap, bool) {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if !(heap_error).is_empty() {
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
                        let source_ml_source1_field717 = String::from(
                            "ordinary heap identity directory exceeds its explicit resource budget",
                        );
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
                        let source_ml_source1_field723 = String::from(
                            "a fresh ordinary Object requires its distinct externally allocated unsigned64 global identity",
                        );
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
                            String::from("an ordinary prototype must be null or a declared Object");
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
                let error =
                    self::read_java_script_ordinary_value_error(heap.clone(), prototype.clone());
                if !(error).is_empty() {
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
                                let source_ml_temporary_index25 = self::ml::Big::from_i128(0);
                                {
                                    let at_2 = self::source_ml_temporary_insert_java_script_ordinary_object_loop26(identity.clone(), at, source_ml_temporary_values25, source_ml_temporary_index25);
                                    {
                                        let object = self::JavaScriptOrdinaryObject::OrdinaryObject(
                                            identity.clone(),
                                            Box::new(prototype.clone()),
                                            true,
                                            false,
                                            Vec::<self::JavaScriptOrdinaryProperty>::new(),
                                        );
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
                                                        let source_ml_source1_field755 =
                                                            String::new();
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn register_java_script_ordinary_symbol(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
) -> (String, self::JavaScriptOrdinaryHeap, bool) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if !(error).is_empty() {
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
                        let source_ml_source1_field763 = String::from(
                            "ordinary heap identity directory exceeds its explicit resource budget",
                        );
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
                        let source_ml_source1_field769 = String::from(
                            "a fresh Symbol requires its distinct externally allocated unsigned64 global identity",
                        );
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
                                    let source_ml_source1_concat_value775 = self::ml_array::slice(
                                        (match &heap {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                _,
                                                source_ml_source1_data_property_value772,
                                            ) => {
                                                source_ml_source1_data_property_value772.as_slice()
                                            }
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
                                            let source_ml_source1_field781 = String::new();
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_conversion_spelling(name: &str) -> self::JavaScriptSemanticValue {
    if name == "default" {
        self::JavaScriptSemanticValue::Text(vec![
            100f64, 101f64, 102f64, 97f64, 117f64, 108f64, 116f64,
        ])
    } else if name == "number" {
        self::JavaScriptSemanticValue::Text(vec![110f64, 117f64, 109f64, 98f64, 101f64, 114f64])
    } else if name == "string" {
        self::JavaScriptSemanticValue::Text(vec![115f64, 116f64, 114f64, 105f64, 110f64, 103f64])
    } else if name == "valueOf" {
        self::JavaScriptSemanticValue::Text(vec![
            118f64, 97f64, 108f64, 117f64, 101f64, 79f64, 102f64,
        ])
    } else if name == "toString" {
        self::JavaScriptSemanticValue::Text(vec![
            116f64, 111f64, 83f64, 116f64, 114f64, 105f64, 110f64, 103f64,
        ])
    } else {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (String::from(
                "a primitive conversion spelling needs its actual registered name",
            ))
            .encode_utf16()
            .collect(),
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
) -> String {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if !(error).is_empty() {
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
            String::from("a primitive conversion continuation retains its actual Object receiver")
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
                if !(receiver_error).is_empty() {
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f795) => true,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f796) => true,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f797) => true,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f798) => true,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f799) => false,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f800,
                            source_ml_source1_f801,
                        ) => true,
                    }
                } {
                    String::from(
                        "the exotic method key must be the actual registered well-known Symbol",
                    )
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                    _,
                                    source_ml_source1_data_property_value803,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value803 =
                                        *source_ml_source1_data_property_value803;
                                    source_ml_source1_data_property_value803.clone()
                                }
                            },
                        );
                        if !(key_error).is_empty() {
                            key_error.clone()
                        } else if (((match state.clone() {
                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                _,
                                _,
                                source_ml_source1_data_property_value805,
                                _,
                            ) => source_ml_source1_data_property_value805.clone(),
                        }) != "default")
                            && ((match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                    _,
                                    _,
                                    source_ml_source1_data_property_value807,
                                    _,
                                ) => source_ml_source1_data_property_value807.clone(),
                            }) != "number"))
                            && ((match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                    _,
                                    _,
                                    source_ml_source1_data_property_value809,
                                    _,
                                ) => source_ml_source1_data_property_value809.clone(),
                            }) != "string")
                        {
                            String::from(
                                "a primitive conversion requires an exact preferred-type hint",
                            )
                        } else if (self::ml_array::includes(
                            &vec![
                                String::from("exoticGet"),
                                String::from("firstGet"),
                                String::from("secondGet"),
                                String::from("exoticGetter"),
                                String::from("firstGetter"),
                                String::from("secondGetter"),
                                String::from("exoticCall"),
                                String::from("firstCall"),
                                String::from("secondCall"),
                            ],
                            &match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value810,
                                ) => source_ml_source1_data_property_value810.clone(),
                            },
                            0f64,
                            false,
                            |a, b| a == b,
                        )) {
                            String::new()
                        } else {
                            String::from(
                                "a primitive conversion requires its exact method continuation",
                            )
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
    phase: &str,
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
            phase.to_owned(),
        ),
    ))
}
