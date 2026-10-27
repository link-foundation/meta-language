// Generated from complete js/src/translation/javascript-descriptor-conversion.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_insert_java_script_ordinary_object_loop26;
use super::source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop29;
use super::source_ml_temporary_register_java_script_ordinary_symbol_loop28;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_semantic_accessor;

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
pub fn read_java_script_descriptor_field(index: f64) -> (String, String, Vec<f64>) {
    if index == 0f64 {
        {
            let source_ml_source1_field782 = String::from("enumerable");
            {
                let source_ml_source1_field783 = String::from("boolean");
                {
                    let source_ml_source1_field784 = vec![
                        101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64,
                        101f64,
                    ];
                    (
                        source_ml_source1_field783,
                        source_ml_source1_field782,
                        source_ml_source1_field784,
                    )
                }
            }
        }
    } else if index == 1f64 {
        {
            let source_ml_source1_field785 = String::from("configurable");
            {
                let source_ml_source1_field786 = String::from("boolean");
                {
                    let source_ml_source1_field787 = vec![
                        99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64,
                        98f64, 108f64, 101f64,
                    ];
                    (
                        source_ml_source1_field786,
                        source_ml_source1_field785,
                        source_ml_source1_field787,
                    )
                }
            }
        }
    } else if index == 2f64 {
        {
            let source_ml_source1_field788 = String::from("value");
            {
                let source_ml_source1_field789 = String::from("value");
                {
                    let source_ml_source1_field790 = vec![118f64, 97f64, 108f64, 117f64, 101f64];
                    (
                        source_ml_source1_field789,
                        source_ml_source1_field788,
                        source_ml_source1_field790,
                    )
                }
            }
        }
    } else if index == 3f64 {
        {
            let source_ml_source1_field791 = String::from("writable");
            {
                let source_ml_source1_field792 = String::from("boolean");
                {
                    let source_ml_source1_field793 =
                        vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64];
                    (
                        source_ml_source1_field792,
                        source_ml_source1_field791,
                        source_ml_source1_field793,
                    )
                }
            }
        }
    } else if index == 4f64 {
        {
            let source_ml_source1_field794 = String::from("get");
            {
                let source_ml_source1_field795 = String::from("accessor");
                {
                    let source_ml_source1_field796 = vec![103f64, 101f64, 116f64];
                    (
                        source_ml_source1_field795,
                        source_ml_source1_field794,
                        source_ml_source1_field796,
                    )
                }
            }
        }
    } else if index == 5f64 {
        {
            let source_ml_source1_field797 = String::from("set");
            {
                let source_ml_source1_field798 = String::from("accessor");
                {
                    let source_ml_source1_field799 = vec![115f64, 101f64, 116f64];
                    (
                        source_ml_source1_field798,
                        source_ml_source1_field797,
                        source_ml_source1_field799,
                    )
                }
            }
        }
    } else {
        {
            let source_ml_source1_field800 = String::new();
            {
                let source_ml_source1_field801 = String::from("invalid");
                {
                    let source_ml_source1_field802 = Vec::<f64>::new();
                    (
                        source_ml_source1_field801,
                        source_ml_source1_field800,
                        source_ml_source1_field802,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn convert_java_script_semantic_boolean(value: self::JavaScriptSemanticValue) -> bool {
    {
        let ml_library_inequality_1 =
            (self::read_java_script_semantic_value_error(value.clone())) != "";
        if ml_library_inequality_1 {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (String::from("ToBoolean requires a complete semantic value"))
                    .encode_utf16()
                    .collect(),
                message_present: true,
            })
            .raise()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => false,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(value_value) => value_value,
                self::JavaScriptSemanticValue::Number(value_value_2) => {
                    (value_value_2 != 0f64) && !(value_value_2.is_nan())
                }
                self::JavaScriptSemanticValue::Integer(value_value_3) => {
                    value_value_3 != (self::ml::Big::from_i128(0))
                }
                self::JavaScriptSemanticValue::Text(units) => (units.len() as f64) != 0f64,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f803) => true,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f804,
                    source_ml_source1_f805,
                ) => true,
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_empty_descriptor() -> (
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
        let source_ml_source1_field806 = false;
        {
            let source_ml_source1_field807 = false;
            {
                let source_ml_source1_field808 = false;
                {
                    let source_ml_source1_field809 = false;
                    {
                        let source_ml_source1_field810 = false;
                        {
                            let source_ml_source1_field811 =
                                self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field812 = false;
                                {
                                    let source_ml_source1_field813 = false;
                                    {
                                        let source_ml_source1_field814 = false;
                                        {
                                            let source_ml_source1_field815 =
                                                self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field816 = false;
                                                {
                                                    let source_ml_source1_field817 =
                                                        self::JavaScriptSemanticValue::Undefined;
                                                    (
                                                        source_ml_source1_field807,
                                                        source_ml_source1_field806,
                                                        source_ml_source1_field809,
                                                        source_ml_source1_field808,
                                                        source_ml_source1_field815,
                                                        source_ml_source1_field814,
                                                        source_ml_source1_field817,
                                                        source_ml_source1_field816,
                                                        source_ml_source1_field811,
                                                        source_ml_source1_field810,
                                                        source_ml_source1_field813,
                                                        source_ml_source1_field812,
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn test_java_script_descriptor_field_presence(
    descriptor: (
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
    ),
    index: f64,
) -> bool {
    if index == 0f64 {
        (descriptor.clone()).3
    } else if index == 1f64 {
        (descriptor.clone()).1
    } else if index == 2f64 {
        (descriptor.clone()).9
    } else if index == 3f64 {
        (descriptor.clone()).11
    } else if index == 4f64 {
        (descriptor.clone()).5
    } else if index == 5f64 {
        (descriptor.clone()).7
    } else {
        false
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_descriptor_conversion_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptDescriptorConversionState,
) -> String {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if !(heap_error).is_empty() {
            heap_error.clone()
        } else if {
            let source_ml_source1_s819 = match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(
                    source_ml_source1_data_property_value827,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source1_data_property_value827 =
                        *source_ml_source1_data_property_value827;
                    source_ml_source1_data_property_value827.clone()
                }
            };
            match source_ml_source1_s819 {
                self::JavaScriptSemanticValue::Undefined => true,
                self::JavaScriptSemanticValue::Null => true,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f820) => true,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f821) => true,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f822) => true,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f823) => true,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f824) => true,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f825,
                    source_ml_source1_f826,
                ) => false,
            }
        } {
            String::from("descriptor conversion retains its actual Object receiver")
        } else {
            {
                let receiver_error = self::read_java_script_ordinary_value_error(
                    heap.clone(),
                    match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(
                            source_ml_source1_data_property_value828,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value828 =
                                *source_ml_source1_data_property_value828;
                            source_ml_source1_data_property_value828.clone()
                        }
                    },
                );
                if !(receiver_error).is_empty() {
                    receiver_error.clone()
                } else if (((!(self::ml_math::is_safe_integer(match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        source_ml_source1_data_property_value829,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value829,
                })) || ((match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        source_ml_source1_data_property_value831,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value831,
                }) < 0f64))
                    || ((match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(
                            _,
                            source_ml_source1_data_property_value833,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value833,
                    }) > 6f64))
                    || (((match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(
                            _,
                            source_ml_source1_data_property_value835,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value835,
                    }) == 0f64)
                        && ((1f64
                            / match state.clone() {
                                self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                    _,
                                    source_ml_source1_data_property_value837,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value837,
                            })
                            < 0f64)))
                    || (match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value838,
                        ) => source_ml_source1_data_property_value838,
                    } && ((match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(
                            _,
                            source_ml_source1_data_property_value840,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value840,
                    }) == 6f64))
                {
                    String::from("descriptor conversion needs its exact ordered field continuation")
                } else {
                    {
                        let index = match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                _,
                                source_ml_source1_data_property_value841,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value841,
                        };
                        {
                            let source_ml_source1_s842 = self::source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop29(state.clone(), index);
                            match source_ml_source1_s842 {
                                        self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Done => {
                                            {
                                                let descriptor = match state.clone() {
                                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value843, _) => {
                                                        source_ml_source1_data_property_value843.clone()
                                                    }
                                                };
                                                if (descriptor.clone()).9 && (!(self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).8)).is_empty()) {
                                                    String::from("descriptor values belong to the actual latest global context")
                                                } else if (descriptor.clone()).5 && ((!(self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4)).is_empty()) || !(self::test_java_script_semantic_accessor((descriptor.clone()).4))) {
                                                        String::from("a processed descriptor getter retains actual intrinsic callability")
                                                    } else if (descriptor.clone()).7 && ((!(self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6)).is_empty()) || !(self::test_java_script_semantic_accessor((descriptor.clone()).6))) {
                                                            String::from("a processed descriptor setter retains actual intrinsic callability")
                                                        } else {
                                                            String::new()
                                                        }
                                            }
                                        }
                                        self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Return(source_ml_temporary_result29) => {
                                            source_ml_temporary_result29.clone()
                                        }
                                    }
                        }
                    }
                }
            }
        }
    }
}
