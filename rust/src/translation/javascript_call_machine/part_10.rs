// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Result;
use super::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop29;
use super::source_ml_temporary_register_java_script_ordinary_symbol_loop28;
use super::source_ml_temporary_step_java_script_descriptor_conversion_loop30;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn register_java_script_ordinary_symbol(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool) {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            {
                let source_ml_source1_field690 = heap.clone();
                {
                    let source_ml_source1_field691 = false;
                    {
                        let source_ml_source1_field692 = error.clone();
                        (source_ml_source1_field692.clone(), source_ml_source1_field690.clone(), source_ml_source1_field691)
                    }
                }
            }
        } else {
            if ((((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value694) => source_ml_source1_data_property_value694.as_slice() }).len() as f64)) >= (16384f64)) {
                {
                    let source_ml_source1_field695 = heap.clone();
                    {
                        let source_ml_source1_field696 = false;
                        {
                            let source_ml_source1_field697 = self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 101, 97, 112, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]);
                            (source_ml_source1_field697.clone(), source_ml_source1_field695.clone(), source_ml_source1_field696)
                        }
                    }
                }
            } else {
                if ((((identity <= (self::ml::Big::from_i128(0))) || (identity > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value699, _) => {
                        source_ml_source1_data_property_value699
                    }
                }, identity.clone())) >= (0f64))) || self::test_java_script_ordinary_symbol(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value700) => {
                        source_ml_source1_data_property_value700
                    }
                }, identity.clone())) {
                    {
                        let source_ml_source1_field701 = heap.clone();
                        {
                            let source_ml_source1_field702 = false;
                            {
                                let source_ml_source1_field703 = self::ml_string::Text::from_units(vec![97, 32, 102, 114, 101, 115, 104, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                (source_ml_source1_field703.clone(), source_ml_source1_field701.clone(), source_ml_source1_field702)
                            }
                        }
                    }
                } else {
                    {
                        let at = 0f64;
                        {
                            let source_ml_temporary_values27 = match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value704) => {
                                    source_ml_source1_data_property_value704
                                }
                            };
                            {
                                let source_ml_temporary_index27 = self::ml::Big::from_i128(0);
                                {
                                    let at_2 = self::source_ml_temporary_register_java_script_ordinary_symbol_loop28(identity.clone(), at, source_ml_temporary_values27, source_ml_temporary_index27.clone());
                                    {
                                        let symbols = {
                                            let source_ml_source1_concat_value709 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value706) => source_ml_source1_data_property_value706.as_slice() }), 0f64, at_2);
                                            {
                                                let source_ml_source1_concat_value710 = vec![identity.clone()];
                                                {
                                                    let source_ml_source1_concat_value711 = self::ml_array::slice((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value708) => source_ml_source1_data_property_value708.as_slice() }), at_2, f64::INFINITY);
                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::ml::Big>::new(), source_ml_source1_concat_value709), source_ml_source1_concat_value710), source_ml_source1_concat_value711)
                                                }
                                            }
                                        };
                                        {
                                            let source_ml_source1_field713 = self::JavaScriptOrdinaryHeap::OrdinaryHeap(match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value712, _) => {
                                                    source_ml_source1_data_property_value712
                                                }
                                            }, symbols);
                                            {
                                                let source_ml_source1_field714 = true;
                                                {
                                                    let source_ml_source1_field715 = self::ml_string::Text::from_units(vec![]);
                                                    (source_ml_source1_field715.clone(), source_ml_source1_field713.clone(), source_ml_source1_field714)
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
pub fn read_java_script_descriptor_field(index: f64) -> (self::ml_string::Text, self::ml_string::Text, Vec<f64>) {
    if (index == (0f64)) {
        {
            let source_ml_source1_field716 = self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101]);
            {
                let source_ml_source1_field717 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                {
                    let source_ml_source1_field718 = vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                    (source_ml_source1_field717.clone(), source_ml_source1_field716.clone(), source_ml_source1_field718)
                }
            }
        }
    } else {
        if (index == (1f64)) {
            {
                let source_ml_source1_field719 = self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101]);
                {
                    let source_ml_source1_field720 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                    {
                        let source_ml_source1_field721 = vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                        (source_ml_source1_field720.clone(), source_ml_source1_field719.clone(), source_ml_source1_field721)
                    }
                }
            }
        } else {
            if (index == (2f64)) {
                {
                    let source_ml_source1_field722 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                    {
                        let source_ml_source1_field723 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                        {
                            let source_ml_source1_field724 = vec![118f64, 97f64, 108f64, 117f64, 101f64];
                            (source_ml_source1_field723.clone(), source_ml_source1_field722.clone(), source_ml_source1_field724)
                        }
                    }
                }
            } else {
                if (index == (3f64)) {
                    {
                        let source_ml_source1_field725 = self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]);
                        {
                            let source_ml_source1_field726 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                            {
                                let source_ml_source1_field727 = vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64];
                                (source_ml_source1_field726.clone(), source_ml_source1_field725.clone(), source_ml_source1_field727)
                            }
                        }
                    }
                } else {
                    if (index == (4f64)) {
                        {
                            let source_ml_source1_field728 = self::ml_string::Text::from_units(vec![103, 101, 116]);
                            {
                                let source_ml_source1_field729 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                {
                                    let source_ml_source1_field730 = vec![103f64, 101f64, 116f64];
                                    (source_ml_source1_field729.clone(), source_ml_source1_field728.clone(), source_ml_source1_field730)
                                }
                            }
                        }
                    } else {
                        if (index == (5f64)) {
                            {
                                let source_ml_source1_field731 = self::ml_string::Text::from_units(vec![115, 101, 116]);
                                {
                                    let source_ml_source1_field732 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                    {
                                        let source_ml_source1_field733 = vec![115f64, 101f64, 116f64];
                                        (source_ml_source1_field732.clone(), source_ml_source1_field731.clone(), source_ml_source1_field733)
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source1_field734 = self::ml_string::Text::from_units(vec![]);
                                {
                                    let source_ml_source1_field735 = self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]);
                                    {
                                        let source_ml_source1_field736 = Vec::<f64>::new();
                                        (source_ml_source1_field735.clone(), source_ml_source1_field734.clone(), source_ml_source1_field736)
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
pub fn convert_java_script_semantic_boolean(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![84, 111, 66, 111, 111, 108, 101, 97, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                false
            }
            self::JavaScriptSemanticValue::Null => {
                false
            }
            self::JavaScriptSemanticValue::Boolean(value_value) => {
                value_value
            }
            self::JavaScriptSemanticValue::Number(value_value_2) => {
                ((value_value_2 != (0f64)) && !(value_value_2.is_nan()))
            }
            self::JavaScriptSemanticValue::Integer(value_value_3) => {
                (value_value_3 != (self::ml::Big::from_i128(0)))
            }
            self::JavaScriptSemanticValue::Text(units) => {
                (((units.len() as f64)) != (0f64))
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f737) => {
                true
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f738, source_ml_source1_f739) => {
                true
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_empty_descriptor() -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
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
                            let source_ml_source1_field745 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field746 = false;
                                {
                                    let source_ml_source1_field747 = false;
                                    {
                                        let source_ml_source1_field748 = false;
                                        {
                                            let source_ml_source1_field749 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field750 = false;
                                                {
                                                    let source_ml_source1_field751 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field741, source_ml_source1_field740, source_ml_source1_field743, source_ml_source1_field742, source_ml_source1_field749.clone(), source_ml_source1_field748, source_ml_source1_field751.clone(), source_ml_source1_field750, source_ml_source1_field745.clone(), source_ml_source1_field744, source_ml_source1_field747, source_ml_source1_field746)
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
pub fn test_java_script_descriptor_field_presence(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), index: f64) -> bool {
    if (index == (0f64)) {
        (descriptor.clone()).3
    } else {
        if (index == (1f64)) {
            (descriptor.clone()).1
        } else {
            if (index == (2f64)) {
                (descriptor.clone()).9
            } else {
                if (index == (3f64)) {
                    (descriptor.clone()).11
                } else {
                    if (index == (4f64)) {
                        (descriptor.clone()).5
                    } else {
                        if (index == (5f64)) {
                            (descriptor.clone()).7
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_descriptor_conversion_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState) -> self::ml_string::Text {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            heap_error.clone()
        } else {
            if {
                let source_ml_source1_s753 = match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value761, _, _, _) => {
                        let source_ml_source1_data_property_value761 = *source_ml_source1_data_property_value761;
                        source_ml_source1_data_property_value761.clone()
                    }
                };
                match source_ml_source1_s753.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f754) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f755) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f756) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f757) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f758) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f759, source_ml_source1_f760) => {
                        false
                    }
                }
            } {
                self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
            } else {
                {
                    let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value762, _, _, _) => {
                            let source_ml_source1_data_property_value762 = *source_ml_source1_data_property_value762;
                            source_ml_source1_data_property_value762.clone()
                        }
                    });
                    if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                        receiver_error.clone()
                    } else {
                        if ((((!(self::ml_math::is_safe_integer(match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value763, _, _) => {
                                source_ml_source1_data_property_value763
                            }
                        })) || ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value765, _, _) => {
                                source_ml_source1_data_property_value765
                            }
                        }) < (0f64))) || ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value767, _, _) => {
                                source_ml_source1_data_property_value767
                            }
                        }) > (6f64))) || (((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value769, _, _) => {
                                source_ml_source1_data_property_value769
                            }
                        }) == (0f64)) && (((1f64 / match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value771, _, _) => {
                                source_ml_source1_data_property_value771
                            }
                        })) < (0f64)))) || (match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value772) => {
                                source_ml_source1_data_property_value772
                            }
                        } && ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value774, _, _) => {
                                source_ml_source1_data_property_value774
                            }
                        }) == (6f64)))) {
                            self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 102, 105, 101, 108, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                        } else {
                            {
                                let index = match state.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value775, _, _) => {
                                        source_ml_source1_data_property_value775
                                    }
                                };
                                {
                                    let source_ml_source1_s776 = self::source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop29(state.clone(), index);
                                    match source_ml_source1_s776.clone() {
                                        self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop29Done => {
                                            {
                                                let descriptor = match state.clone() {
                                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value777, _) => {
                                                        source_ml_source1_data_property_value777.clone()
                                                    }
                                                };
                                                if ((descriptor.clone()).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
                                                    self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 118, 97, 108, 117, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116])
                                                } else {
                                                    if ((descriptor.clone()).5 && (((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).4)))) {
                                                        self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                    } else {
                                                        if ((descriptor.clone()).7 && (((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).6)))) {
                                                            self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                    }
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_descriptor_field(state: self::JavaScriptDescriptorConversionState, value: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    if ((((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value779, _, _) => {
            source_ml_source1_data_property_value779
        }
    }) == (4f64)) || ((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value781, _, _) => {
            source_ml_source1_data_property_value781
        }
    }) == (5f64))) && !(self::test_java_script_semantic_accessor(value.clone()))) {
        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100]))
    } else {
        {
            let before = match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value782, _) => {
                    source_ml_source1_data_property_value782.clone()
                }
            };
            {
                let descriptor = if ((match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value784, _, _) => {
                        source_ml_source1_data_property_value784
                    }
                }) == (0f64)) {
                    {
                        let source_ml_source1_spread785 = before.clone();
                        {
                            let source_ml_source1_spread_field786 = (source_ml_source1_spread785.clone()).0;
                            {
                                let source_ml_source1_spread_field787 = (source_ml_source1_spread785.clone()).1;
                                {
                                    let source_ml_source1_spread_field788 = (source_ml_source1_spread785.clone()).2;
                                    {
                                        let source_ml_source1_spread_field789 = (source_ml_source1_spread785.clone()).3;
                                        {
                                            let source_ml_source1_spread_field790 = (source_ml_source1_spread785.clone()).4;
                                            {
                                                let source_ml_source1_spread_field791 = (source_ml_source1_spread785.clone()).5;
                                                {
                                                    let source_ml_source1_spread_field792 = (source_ml_source1_spread785.clone()).6;
                                                    {
                                                        let source_ml_source1_spread_field793 = (source_ml_source1_spread785.clone()).7;
                                                        {
                                                            let source_ml_source1_spread_field794 = (source_ml_source1_spread785.clone()).8;
                                                            {
                                                                let source_ml_source1_spread_field795 = (source_ml_source1_spread785.clone()).9;
                                                                {
                                                                    let source_ml_source1_spread_field796 = (source_ml_source1_spread785.clone()).10;
                                                                    {
                                                                        let source_ml_source1_spread_field797 = (source_ml_source1_spread785.clone()).11;
                                                                        {
                                                                            let source_ml_source1_field798 = true;
                                                                            {
                                                                                let source_ml_source1_field799 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                (source_ml_source1_spread_field786, source_ml_source1_spread_field787, source_ml_source1_field799, source_ml_source1_field798, source_ml_source1_spread_field790.clone(), source_ml_source1_spread_field791, source_ml_source1_spread_field792.clone(), source_ml_source1_spread_field793, source_ml_source1_spread_field794.clone(), source_ml_source1_spread_field795, source_ml_source1_spread_field796, source_ml_source1_spread_field797)
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
                    if ((match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value801, _, _) => {
                            source_ml_source1_data_property_value801
                        }
                    }) == (1f64)) {
                        {
                            let source_ml_source1_spread802 = before.clone();
                            {
                                let source_ml_source1_spread_field803 = (source_ml_source1_spread802.clone()).0;
                                {
                                    let source_ml_source1_spread_field804 = (source_ml_source1_spread802.clone()).1;
                                    {
                                        let source_ml_source1_spread_field805 = (source_ml_source1_spread802.clone()).2;
                                        {
                                            let source_ml_source1_spread_field806 = (source_ml_source1_spread802.clone()).3;
                                            {
                                                let source_ml_source1_spread_field807 = (source_ml_source1_spread802.clone()).4;
                                                {
                                                    let source_ml_source1_spread_field808 = (source_ml_source1_spread802.clone()).5;
                                                    {
                                                        let source_ml_source1_spread_field809 = (source_ml_source1_spread802.clone()).6;
                                                        {
                                                            let source_ml_source1_spread_field810 = (source_ml_source1_spread802.clone()).7;
                                                            {
                                                                let source_ml_source1_spread_field811 = (source_ml_source1_spread802.clone()).8;
                                                                {
                                                                    let source_ml_source1_spread_field812 = (source_ml_source1_spread802.clone()).9;
                                                                    {
                                                                        let source_ml_source1_spread_field813 = (source_ml_source1_spread802.clone()).10;
                                                                        {
                                                                            let source_ml_source1_spread_field814 = (source_ml_source1_spread802.clone()).11;
                                                                            {
                                                                                let source_ml_source1_field815 = true;
                                                                                {
                                                                                    let source_ml_source1_field816 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                    (source_ml_source1_field816, source_ml_source1_field815, source_ml_source1_spread_field805, source_ml_source1_spread_field806, source_ml_source1_spread_field807.clone(), source_ml_source1_spread_field808, source_ml_source1_spread_field809.clone(), source_ml_source1_spread_field810, source_ml_source1_spread_field811.clone(), source_ml_source1_spread_field812, source_ml_source1_spread_field813, source_ml_source1_spread_field814)
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
                        if ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value818, _, _) => {
                                source_ml_source1_data_property_value818
                            }
                        }) == (2f64)) {
                            {
                                let source_ml_source1_spread819 = before.clone();
                                {
                                    let source_ml_source1_spread_field820 = (source_ml_source1_spread819.clone()).0;
                                    {
                                        let source_ml_source1_spread_field821 = (source_ml_source1_spread819.clone()).1;
                                        {
                                            let source_ml_source1_spread_field822 = (source_ml_source1_spread819.clone()).2;
                                            {
                                                let source_ml_source1_spread_field823 = (source_ml_source1_spread819.clone()).3;
                                                {
                                                    let source_ml_source1_spread_field824 = (source_ml_source1_spread819.clone()).4;
                                                    {
                                                        let source_ml_source1_spread_field825 = (source_ml_source1_spread819.clone()).5;
                                                        {
                                                            let source_ml_source1_spread_field826 = (source_ml_source1_spread819.clone()).6;
                                                            {
                                                                let source_ml_source1_spread_field827 = (source_ml_source1_spread819.clone()).7;
                                                                {
                                                                    let source_ml_source1_spread_field828 = (source_ml_source1_spread819.clone()).8;
                                                                    {
                                                                        let source_ml_source1_spread_field829 = (source_ml_source1_spread819.clone()).9;
                                                                        {
                                                                            let source_ml_source1_spread_field830 = (source_ml_source1_spread819.clone()).10;
                                                                            {
                                                                                let source_ml_source1_spread_field831 = (source_ml_source1_spread819.clone()).11;
                                                                                {
                                                                                    let source_ml_source1_field832 = true;
                                                                                    {
                                                                                        let source_ml_source1_field833 = value.clone();
                                                                                        (source_ml_source1_spread_field820, source_ml_source1_spread_field821, source_ml_source1_spread_field822, source_ml_source1_spread_field823, source_ml_source1_spread_field824.clone(), source_ml_source1_spread_field825, source_ml_source1_spread_field826.clone(), source_ml_source1_spread_field827, source_ml_source1_field833.clone(), source_ml_source1_field832, source_ml_source1_spread_field830, source_ml_source1_spread_field831)
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
                            if ((match state.clone() {
                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value835, _, _) => {
                                    source_ml_source1_data_property_value835
                                }
                            }) == (3f64)) {
                                {
                                    let source_ml_source1_spread836 = before.clone();
                                    {
                                        let source_ml_source1_spread_field837 = (source_ml_source1_spread836.clone()).0;
                                        {
                                            let source_ml_source1_spread_field838 = (source_ml_source1_spread836.clone()).1;
                                            {
                                                let source_ml_source1_spread_field839 = (source_ml_source1_spread836.clone()).2;
                                                {
                                                    let source_ml_source1_spread_field840 = (source_ml_source1_spread836.clone()).3;
                                                    {
                                                        let source_ml_source1_spread_field841 = (source_ml_source1_spread836.clone()).4;
                                                        {
                                                            let source_ml_source1_spread_field842 = (source_ml_source1_spread836.clone()).5;
                                                            {
                                                                let source_ml_source1_spread_field843 = (source_ml_source1_spread836.clone()).6;
                                                                {
                                                                    let source_ml_source1_spread_field844 = (source_ml_source1_spread836.clone()).7;
                                                                    {
                                                                        let source_ml_source1_spread_field845 = (source_ml_source1_spread836.clone()).8;
                                                                        {
                                                                            let source_ml_source1_spread_field846 = (source_ml_source1_spread836.clone()).9;
                                                                            {
                                                                                let source_ml_source1_spread_field847 = (source_ml_source1_spread836.clone()).10;
                                                                                {
                                                                                    let source_ml_source1_spread_field848 = (source_ml_source1_spread836.clone()).11;
                                                                                    {
                                                                                        let source_ml_source1_field849 = true;
                                                                                        {
                                                                                            let source_ml_source1_field850 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                            (source_ml_source1_spread_field837, source_ml_source1_spread_field838, source_ml_source1_spread_field839, source_ml_source1_spread_field840, source_ml_source1_spread_field841.clone(), source_ml_source1_spread_field842, source_ml_source1_spread_field843.clone(), source_ml_source1_spread_field844, source_ml_source1_spread_field845.clone(), source_ml_source1_spread_field846, source_ml_source1_field850, source_ml_source1_field849)
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
                                if ((match state.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value852, _, _) => {
                                        source_ml_source1_data_property_value852
                                    }
                                }) == (4f64)) {
                                    {
                                        let source_ml_source1_spread853 = before.clone();
                                        {
                                            let source_ml_source1_spread_field854 = (source_ml_source1_spread853.clone()).0;
                                            {
                                                let source_ml_source1_spread_field855 = (source_ml_source1_spread853.clone()).1;
                                                {
                                                    let source_ml_source1_spread_field856 = (source_ml_source1_spread853.clone()).2;
                                                    {
                                                        let source_ml_source1_spread_field857 = (source_ml_source1_spread853.clone()).3;
                                                        {
                                                            let source_ml_source1_spread_field858 = (source_ml_source1_spread853.clone()).4;
                                                            {
                                                                let source_ml_source1_spread_field859 = (source_ml_source1_spread853.clone()).5;
                                                                {
                                                                    let source_ml_source1_spread_field860 = (source_ml_source1_spread853.clone()).6;
                                                                    {
                                                                        let source_ml_source1_spread_field861 = (source_ml_source1_spread853.clone()).7;
                                                                        {
                                                                            let source_ml_source1_spread_field862 = (source_ml_source1_spread853.clone()).8;
                                                                            {
                                                                                let source_ml_source1_spread_field863 = (source_ml_source1_spread853.clone()).9;
                                                                                {
                                                                                    let source_ml_source1_spread_field864 = (source_ml_source1_spread853.clone()).10;
                                                                                    {
                                                                                        let source_ml_source1_spread_field865 = (source_ml_source1_spread853.clone()).11;
                                                                                        {
                                                                                            let source_ml_source1_field866 = true;
                                                                                            {
                                                                                                let source_ml_source1_field867 = value.clone();
                                                                                                (source_ml_source1_spread_field854, source_ml_source1_spread_field855, source_ml_source1_spread_field856, source_ml_source1_spread_field857, source_ml_source1_field867.clone(), source_ml_source1_field866, source_ml_source1_spread_field860.clone(), source_ml_source1_spread_field861, source_ml_source1_spread_field862.clone(), source_ml_source1_spread_field863, source_ml_source1_spread_field864, source_ml_source1_spread_field865)
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
                                        let source_ml_source1_spread868 = before.clone();
                                        {
                                            let source_ml_source1_spread_field869 = (source_ml_source1_spread868.clone()).0;
                                            {
                                                let source_ml_source1_spread_field870 = (source_ml_source1_spread868.clone()).1;
                                                {
                                                    let source_ml_source1_spread_field871 = (source_ml_source1_spread868.clone()).2;
                                                    {
                                                        let source_ml_source1_spread_field872 = (source_ml_source1_spread868.clone()).3;
                                                        {
                                                            let source_ml_source1_spread_field873 = (source_ml_source1_spread868.clone()).4;
                                                            {
                                                                let source_ml_source1_spread_field874 = (source_ml_source1_spread868.clone()).5;
                                                                {
                                                                    let source_ml_source1_spread_field875 = (source_ml_source1_spread868.clone()).6;
                                                                    {
                                                                        let source_ml_source1_spread_field876 = (source_ml_source1_spread868.clone()).7;
                                                                        {
                                                                            let source_ml_source1_spread_field877 = (source_ml_source1_spread868.clone()).8;
                                                                            {
                                                                                let source_ml_source1_spread_field878 = (source_ml_source1_spread868.clone()).9;
                                                                                {
                                                                                    let source_ml_source1_spread_field879 = (source_ml_source1_spread868.clone()).10;
                                                                                    {
                                                                                        let source_ml_source1_spread_field880 = (source_ml_source1_spread868.clone()).11;
                                                                                        {
                                                                                            let source_ml_source1_field881 = true;
                                                                                            {
                                                                                                let source_ml_source1_field882 = value.clone();
                                                                                                (source_ml_source1_spread_field869, source_ml_source1_spread_field870, source_ml_source1_spread_field871, source_ml_source1_spread_field872, source_ml_source1_spread_field873.clone(), source_ml_source1_spread_field874, source_ml_source1_field882.clone(), source_ml_source1_field881, source_ml_source1_spread_field877.clone(), source_ml_source1_spread_field878, source_ml_source1_spread_field879, source_ml_source1_spread_field880)
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
                };
                self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value883, _, _, _) => {
                        let source_ml_source1_data_property_value883 = *source_ml_source1_data_property_value883;
                        source_ml_source1_data_property_value883.clone()
                    }
                }), (match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value884, _, _) => {
                        source_ml_source1_data_property_value884
                    }
                } + 1f64), descriptor.clone(), false)))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error = self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else {
            if match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value885) => {
                    source_ml_source1_data_property_value885
                }
            } {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 71, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 110, 101, 120, 116, 32, 115, 116, 101, 112]))
            } else {
                {
                    let current = state.clone();
                    {
                        let source_ml_source1_s886 = self::source_ml_temporary_step_java_script_descriptor_conversion_loop30(heap.clone(), current.clone());
                        match source_ml_source1_s886.clone() {
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Done(current_2) => {
                                let current_2 = *current_2;
                                if ((self::read_java_script_property_descriptor_error(match current_2.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value888, _) => {
                                        source_ml_source1_data_property_value888.clone()
                                    }
                                })) != (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 98, 111, 116, 104, 32, 100, 97, 116, 97, 32, 97, 110, 100, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115]))
                                } else {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(match current_2.clone() {
                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value889, _) => {
                                            source_ml_source1_data_property_value889.clone()
                                        }
                                    })
                                }
                            }
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(source_ml_temporary_result30) => {
                                let source_ml_temporary_result30 = *source_ml_temporary_result30;
                                source_ml_temporary_result30.clone()
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
pub fn begin_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, receiver: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(heap_error.clone())
        } else {
            {
                let error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
                } else {
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f890) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f891) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f892) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f893) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f894) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f895, source_ml_source1_f896) => {
                            self::step_java_script_descriptor_conversion(heap.clone(), self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), 0f64, self::create_java_script_empty_descriptor(), false))
                        }
                    }
                }
            }
        }
    }
}
