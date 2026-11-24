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
                let source_ml_source1_field644 = heap.clone();
                {
                    let source_ml_source1_field645 = false;
                    {
                        let source_ml_source1_field646 = heap_error.clone();
                        (
                            source_ml_source1_field646,
                            source_ml_source1_field644,
                            source_ml_source1_field645,
                        )
                    }
                }
            }
        } else if ((match &heap {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                source_ml_source1_data_property_value648,
                _,
            ) => source_ml_source1_data_property_value648.as_slice(),
        })
        .len() as f64)
            >= 4096f64
        {
            {
                let source_ml_source1_field649 = heap.clone();
                {
                    let source_ml_source1_field650 = false;
                    {
                        let source_ml_source1_field651 = String::from(
                            "ordinary heap identity directory exceeds its explicit resource budget",
                        );
                        (
                            source_ml_source1_field651,
                            source_ml_source1_field649,
                            source_ml_source1_field650,
                        )
                    }
                }
            }
        } else if (((identity <= (self::ml::Big::from_i128(0)))
            || (identity > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
            || ((self::find_java_script_ordinary_object(
                match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value653,
                        _,
                    ) => source_ml_source1_data_property_value653,
                },
                identity.clone(),
            )) >= 0f64))
            || self::test_java_script_ordinary_symbol(
                match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        _,
                        source_ml_source1_data_property_value654,
                    ) => source_ml_source1_data_property_value654,
                },
                identity.clone(),
            )
        {
            {
                let source_ml_source1_field655 = heap.clone();
                {
                    let source_ml_source1_field656 = false;
                    {
                        let source_ml_source1_field657 = String::from(
                            "a fresh ordinary Object requires its distinct externally allocated unsigned64 global identity",
                        );
                        (
                            source_ml_source1_field657,
                            source_ml_source1_field655,
                            source_ml_source1_field656,
                        )
                    }
                }
            }
        } else if match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => true,
            self::JavaScriptSemanticValue::Null => false,
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f658) => true,
            self::JavaScriptSemanticValue::Number(source_ml_source1_f659) => true,
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f660) => true,
            self::JavaScriptSemanticValue::Text(source_ml_source1_f661) => true,
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f662) => true,
            self::JavaScriptSemanticValue::Object(
                source_ml_source1_f663,
                source_ml_source1_f664,
            ) => true,
        } && match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => true,
            self::JavaScriptSemanticValue::Null => true,
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f665) => true,
            self::JavaScriptSemanticValue::Number(source_ml_source1_f666) => true,
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f667) => true,
            self::JavaScriptSemanticValue::Text(source_ml_source1_f668) => true,
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f669) => true,
            self::JavaScriptSemanticValue::Object(
                source_ml_source1_f670,
                source_ml_source1_f671,
            ) => false,
        } {
            {
                let source_ml_source1_field672 = heap.clone();
                {
                    let source_ml_source1_field673 = false;
                    {
                        let source_ml_source1_field674 =
                            String::from("an ordinary prototype must be null or a declared Object");
                        (
                            source_ml_source1_field674,
                            source_ml_source1_field672,
                            source_ml_source1_field673,
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
                        let source_ml_source1_field675 = heap.clone();
                        {
                            let source_ml_source1_field676 = false;
                            {
                                let source_ml_source1_field677 = error.clone();
                                (
                                    source_ml_source1_field677,
                                    source_ml_source1_field675,
                                    source_ml_source1_field676,
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
                                    source_ml_source1_data_property_value678,
                                    _,
                                ) => source_ml_source1_data_property_value678,
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
                                                let source_ml_source1_concat_value683 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value680, _) => source_ml_source1_data_property_value680.as_slice() }), 0f64, at_2);
                                                {
                                                    let source_ml_source1_concat_value684 =
                                                        vec![object];
                                                    {
                                                        let source_ml_source1_concat_value685 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value682, _) => source_ml_source1_data_property_value682.as_slice() }), at_2, f64::INFINITY);
                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), source_ml_source1_concat_value683), source_ml_source1_concat_value684), source_ml_source1_concat_value685)
                                                    }
                                                }
                                            };
                                            {
                                                let source_ml_source1_field687 = self::JavaScriptOrdinaryHeap::OrdinaryHeap(objects, match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value686) => {
                                                                    source_ml_source1_data_property_value686
                                                                }
                                                            });
                                                {
                                                    let source_ml_source1_field688 = true;
                                                    {
                                                        let source_ml_source1_field689 =
                                                            String::new();
                                                        (
                                                            source_ml_source1_field689,
                                                            source_ml_source1_field687,
                                                            source_ml_source1_field688,
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
                let source_ml_source1_field690 = heap.clone();
                {
                    let source_ml_source1_field691 = false;
                    {
                        let source_ml_source1_field692 = error.clone();
                        (
                            source_ml_source1_field692,
                            source_ml_source1_field690,
                            source_ml_source1_field691,
                        )
                    }
                }
            }
        } else if ((match &heap {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                _,
                source_ml_source1_data_property_value694,
            ) => source_ml_source1_data_property_value694.as_slice(),
        })
        .len() as f64)
            >= 16_384f64
        {
            {
                let source_ml_source1_field695 = heap.clone();
                {
                    let source_ml_source1_field696 = false;
                    {
                        let source_ml_source1_field697 = String::from(
                            "ordinary heap identity directory exceeds its explicit resource budget",
                        );
                        (
                            source_ml_source1_field697,
                            source_ml_source1_field695,
                            source_ml_source1_field696,
                        )
                    }
                }
            }
        } else if (((identity <= (self::ml::Big::from_i128(0)))
            || (identity > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
            || ((self::find_java_script_ordinary_object(
                match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value699,
                        _,
                    ) => source_ml_source1_data_property_value699,
                },
                identity.clone(),
            )) >= 0f64))
            || self::test_java_script_ordinary_symbol(
                match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        _,
                        source_ml_source1_data_property_value700,
                    ) => source_ml_source1_data_property_value700,
                },
                identity.clone(),
            )
        {
            {
                let source_ml_source1_field701 = heap.clone();
                {
                    let source_ml_source1_field702 = false;
                    {
                        let source_ml_source1_field703 = String::from(
                            "a fresh Symbol requires its distinct externally allocated unsigned64 global identity",
                        );
                        (
                            source_ml_source1_field703,
                            source_ml_source1_field701,
                            source_ml_source1_field702,
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
                            source_ml_source1_data_property_value704,
                        ) => source_ml_source1_data_property_value704,
                    };
                    {
                        let source_ml_temporary_index27 = self::ml::Big::from_i128(0);
                        {
                            let at_2 = self::source_ml_temporary_register_java_script_ordinary_symbol_loop28(identity.clone(), at, source_ml_temporary_values27, source_ml_temporary_index27);
                            {
                                let symbols = {
                                    let source_ml_source1_concat_value709 = self::ml_array::slice(
                                        (match &heap {
                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                _,
                                                source_ml_source1_data_property_value706,
                                            ) => {
                                                source_ml_source1_data_property_value706.as_slice()
                                            }
                                        }),
                                        0f64,
                                        at_2,
                                    );
                                    {
                                        let source_ml_source1_concat_value710 =
                                            vec![identity.clone()];
                                        {
                                            let source_ml_source1_concat_value711 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value708) => source_ml_source1_data_property_value708.as_slice() }), at_2, f64::INFINITY);
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<self::ml::Big>::new(),
                                                        source_ml_source1_concat_value709,
                                                    ),
                                                    source_ml_source1_concat_value710,
                                                ),
                                                source_ml_source1_concat_value711,
                                            )
                                        }
                                    }
                                };
                                {
                                    let source_ml_source1_field713 =
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                    source_ml_source1_data_property_value712,
                                                    _,
                                                ) => source_ml_source1_data_property_value712,
                                            },
                                            symbols,
                                        );
                                    {
                                        let source_ml_source1_field714 = true;
                                        {
                                            let source_ml_source1_field715 = String::new();
                                            (
                                                source_ml_source1_field715,
                                                source_ml_source1_field713,
                                                source_ml_source1_field714,
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
            let source_ml_source1_field716 = String::from("enumerable");
            {
                let source_ml_source1_field717 = String::from("boolean");
                {
                    let source_ml_source1_field718 = vec![
                        101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64,
                        101f64,
                    ];
                    (
                        source_ml_source1_field717,
                        source_ml_source1_field716,
                        source_ml_source1_field718,
                    )
                }
            }
        }
    } else if index == 1f64 {
        {
            let source_ml_source1_field719 = String::from("configurable");
            {
                let source_ml_source1_field720 = String::from("boolean");
                {
                    let source_ml_source1_field721 = vec![
                        99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64,
                        98f64, 108f64, 101f64,
                    ];
                    (
                        source_ml_source1_field720,
                        source_ml_source1_field719,
                        source_ml_source1_field721,
                    )
                }
            }
        }
    } else if index == 2f64 {
        {
            let source_ml_source1_field722 = String::from("value");
            {
                let source_ml_source1_field723 = String::from("value");
                {
                    let source_ml_source1_field724 = vec![118f64, 97f64, 108f64, 117f64, 101f64];
                    (
                        source_ml_source1_field723,
                        source_ml_source1_field722,
                        source_ml_source1_field724,
                    )
                }
            }
        }
    } else if index == 3f64 {
        {
            let source_ml_source1_field725 = String::from("writable");
            {
                let source_ml_source1_field726 = String::from("boolean");
                {
                    let source_ml_source1_field727 =
                        vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64];
                    (
                        source_ml_source1_field726,
                        source_ml_source1_field725,
                        source_ml_source1_field727,
                    )
                }
            }
        }
    } else if index == 4f64 {
        {
            let source_ml_source1_field728 = String::from("get");
            {
                let source_ml_source1_field729 = String::from("accessor");
                {
                    let source_ml_source1_field730 = vec![103f64, 101f64, 116f64];
                    (
                        source_ml_source1_field729,
                        source_ml_source1_field728,
                        source_ml_source1_field730,
                    )
                }
            }
        }
    } else if index == 5f64 {
        {
            let source_ml_source1_field731 = String::from("set");
            {
                let source_ml_source1_field732 = String::from("accessor");
                {
                    let source_ml_source1_field733 = vec![115f64, 101f64, 116f64];
                    (
                        source_ml_source1_field732,
                        source_ml_source1_field731,
                        source_ml_source1_field733,
                    )
                }
            }
        }
    } else {
        {
            let source_ml_source1_field734 = String::new();
            {
                let source_ml_source1_field735 = String::from("invalid");
                {
                    let source_ml_source1_field736 = Vec::<f64>::new();
                    (
                        source_ml_source1_field735,
                        source_ml_source1_field734,
                        source_ml_source1_field736,
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
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f737) => true,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f738,
                    source_ml_source1_f739,
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
        let source_ml_source1_field740 = false;
        {
            let source_ml_source1_field741 = false;
            {
                let source_ml_source1_field742 = false;
                {
                    let source_ml_source1_field743 = false;
                    {
                        let source_ml_source1_field744 = false;
                        {
                            let source_ml_source1_field745 =
                                self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field746 = false;
                                {
                                    let source_ml_source1_field747 = false;
                                    {
                                        let source_ml_source1_field748 = false;
                                        {
                                            let source_ml_source1_field749 =
                                                self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field750 = false;
                                                {
                                                    let source_ml_source1_field751 =
                                                        self::JavaScriptSemanticValue::Undefined;
                                                    (
                                                        source_ml_source1_field741,
                                                        source_ml_source1_field740,
                                                        source_ml_source1_field743,
                                                        source_ml_source1_field742,
                                                        source_ml_source1_field749,
                                                        source_ml_source1_field748,
                                                        source_ml_source1_field751,
                                                        source_ml_source1_field750,
                                                        source_ml_source1_field745,
                                                        source_ml_source1_field744,
                                                        source_ml_source1_field747,
                                                        source_ml_source1_field746,
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
            let source_ml_source1_s753 = match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(
                    source_ml_source1_data_property_value761,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source1_data_property_value761 =
                        *source_ml_source1_data_property_value761;
                    source_ml_source1_data_property_value761.clone()
                }
            };
            match source_ml_source1_s753 {
                self::JavaScriptSemanticValue::Undefined => true,
                self::JavaScriptSemanticValue::Null => true,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f754) => true,
                self::JavaScriptSemanticValue::Number(source_ml_source1_f755) => true,
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f756) => true,
                self::JavaScriptSemanticValue::Text(source_ml_source1_f757) => true,
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f758) => true,
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f759,
                    source_ml_source1_f760,
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
                            source_ml_source1_data_property_value762,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value762 =
                                *source_ml_source1_data_property_value762;
                            source_ml_source1_data_property_value762.clone()
                        }
                    },
                );
                if !(receiver_error).is_empty() {
                    receiver_error.clone()
                } else if (((!(self::ml_math::is_safe_integer(match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        source_ml_source1_data_property_value763,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value763,
                })) || ((match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        source_ml_source1_data_property_value765,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value765,
                }) < 0f64))
                    || ((match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(
                            _,
                            source_ml_source1_data_property_value767,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value767,
                    }) > 6f64))
                    || (((match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(
                            _,
                            source_ml_source1_data_property_value769,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value769,
                    }) == 0f64)
                        && ((1f64
                            / match state.clone() {
                                self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                    _,
                                    source_ml_source1_data_property_value771,
                                    _,
                                    _,
                                ) => source_ml_source1_data_property_value771,
                            })
                            < 0f64)))
                    || (match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value772,
                        ) => source_ml_source1_data_property_value772,
                    } && ((match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(
                            _,
                            source_ml_source1_data_property_value774,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value774,
                    }) == 6f64))
                {
                    String::from("descriptor conversion needs its exact ordered field continuation")
                } else {
                    {
                        let index = match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                _,
                                source_ml_source1_data_property_value775,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value775,
                        };
                        {
                            let source_ml_source1_s776 = self::source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop29(state.clone(), index);
                            match source_ml_source1_s776 {
                                        self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Done => {
                                            {
                                                let descriptor = match state.clone() {
                                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value777, _) => {
                                                        source_ml_source1_data_property_value777.clone()
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
